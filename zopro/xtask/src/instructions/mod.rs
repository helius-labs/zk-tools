//! Offline attribution of retained cycle samples to byte-bound instructions.
//! Sampling skid and inlining prevent interpreting these bins as exclusive costs.
mod decode;
mod elf;
mod samples;

use crate::{process, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    version: u32,
    pins: BTreeMap<String, String>,
    binaries: Vec<Binary>,
    runs: Vec<Run>,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Binary {
    name: String,
    path: PathBuf,
    disassembly: PathBuf,
    symbols: PathBuf,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Run {
    name: String,
    binary: String,
    metadata: PathBuf,
    samples: PathBuf,
    expected_pid: u32,
    expected_samples: u64,
    expected_period: u64,
    expected_tids: usize,
}
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
struct Tally {
    samples: u64,
    period: u64,
}
impl Tally {
    fn add(&mut self, other: Self) -> Result<()> {
        self.samples = self
            .samples
            .checked_add(other.samples)
            .ok_or("sample-count overflow")?;
        self.period = self
            .period
            .checked_add(other.period)
            .ok_or("sample-period overflow")?;
        Ok(())
    }
}
#[derive(Default, Serialize)]
struct Bins {
    total: Tally,
    families: BTreeMap<&'static str, Tally>,
    memory: BTreeMap<&'static str, Tally>,
}
impl Bins {
    fn add(&mut self, family: &'static str, memory: &'static str, value: Tally) -> Result<()> {
        self.total.add(value)?;
        self.families.entry(family).or_default().add(value)?;
        self.memory.entry(memory).or_default().add(value)?;
        Ok(())
    }
    fn verify(&self) -> Result<()> {
        for bins in [&self.families, &self.memory] {
            let mut sum = Tally::default();
            for v in bins.values() {
                sum.add(*v)?;
            }
            if sum != self.total {
                return Err("instruction-bin conservation failed".into());
            }
        }
        Ok(())
    }
}
#[derive(Serialize)]
struct Function {
    names: Vec<String>,
    scope: &'static str,
    bins: Bins,
}
#[derive(Serialize)]
struct Ip<'a> {
    runtime_ip: String,
    elf_ip: Option<String>,
    tally: Tally,
    function: Option<usize>,
    assembly: Option<&'a str>,
    family: &'static str,
    memory: &'static str,
    scope: &'static str,
}
#[derive(Serialize)]
struct RunSummary {
    name: String,
    binary: String,
    total: Tally,
    executable: Tally,
    outside_executable: Tally,
    sampled_instruction_sites: usize,
    sampled_tids: usize,
    load_bias: String,
}
#[derive(Serialize)]
pub struct Summary {
    version: u32,
    status: &'static str,
    total: Tally,
    runs: Vec<RunSummary>,
    interpretation: &'static str,
}

fn name_valid(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
fn pinned(path: &Path, pins: &BTreeMap<String, String>) -> Result<()> {
    if !path.is_absolute() || !pins.contains_key(path.to_str().ok_or("non-UTF8 input path")?) {
        return Err(format!(
            "input needs an absolute, hash-pinned path: {}",
            path.display()
        )
        .into());
    }
    Ok(())
}

fn analyze(
    run: &Run,
    binary: &Binary,
    elf: &elf::Elf<'_>,
    instructions: &BTreeMap<u64, decode::Instruction>,
    symbols: &decode::Symbols,
    output: &Path,
) -> Result<RunSummary> {
    let metadata = fs::read_to_string(&run.metadata)?;
    let mapping = samples::Mapping::parse(
        &metadata,
        run.expected_pid,
        binary.path.to_str().ok_or("non-UTF8 binary path")?,
        elf,
    )?;
    let mut ips = BTreeMap::<u64, Tally>::new();
    let mut total = Tally::default();
    let mut tids = BTreeSet::new();
    for line in BufReader::new(File::open(&run.samples)?).lines() {
        if let Some(s) = samples::sample(&line?)? {
            if s.pid != run.expected_pid {
                return Err("sample PID differs from configured capture PID".into());
            }
            let value = Tally {
                samples: 1,
                period: s.period,
            };
            total.add(value)?;
            ips.entry(s.ip).or_default().add(value)?;
            tids.insert(s.tid);
        }
    }
    if total
        != (Tally {
            samples: run.expected_samples,
            period: run.expected_period,
        })
        || tids.len() != run.expected_tids
    {
        return Err(format!("sample count/period/thread mismatch in {}", run.name).into());
    }
    let mut output_ips = BufWriter::new(File::create(
        output.join(format!("{}-ips.jsonl", run.name)),
    )?);
    let mut all = Bins::default();
    let mut executable = Tally::default();
    let mut outside = Tally::default();
    let mut scopes = BTreeMap::<&'static str, Bins>::new();
    let mut function_ids = BTreeMap::<Vec<String>, usize>::new();
    let mut functions = Vec::<Function>::new();
    let mut sites = 0;
    for (runtime_ip, tally) in ips {
        let (elf_ip, function, assembly, family, memory, scope) = if let Some(address) =
            mapping.address(runtime_ip)?
        {
            let instruction=instructions.get(&address).ok_or_else(||format!(
                "no exact instruction boundary for recorded executable IP {runtime_ip:x} (ELF {address:x})"))?;
            if instruction.mnemonic == "(bad)" || instruction.mnemonic.starts_with('.') {
                return Err("sampled executable IP was not decoded as an instruction".into());
            }
            let names = symbols.names(address);
            let scope = decode::scope(&names);
            let names = names.into_iter().map(str::to_owned).collect::<Vec<_>>();
            let index = if let Some(&i) = function_ids.get(&names) {
                i
            } else {
                let i = functions.len();
                function_ids.insert(names.clone(), i);
                functions.push(Function {
                    names,
                    scope,
                    bins: Bins::default(),
                });
                i
            };
            functions[index]
                .bins
                .add(instruction.family, instruction.memory, tally)?;
            executable.add(tally)?;
            sites += 1;
            (
                Some(format!("{address:x}")),
                Some(index),
                Some(instruction.assembly.as_str()),
                instruction.family,
                instruction.memory,
                scope,
            )
        } else {
            outside.add(tally)?;
            (
                None,
                None,
                None,
                "outside_executable",
                "unknown",
                "outside_executable",
            )
        };
        all.add(family, memory, tally)?;
        scopes
            .entry(scope)
            .or_default()
            .add(family, memory, tally)?;
        serde_json::to_writer(
            &mut output_ips,
            &Ip {
                runtime_ip: format!("{runtime_ip:x}"),
                elf_ip,
                tally,
                function,
                assembly,
                family,
                memory,
                scope,
            },
        )?;
        output_ips.write_all(b"\n")?;
    }
    output_ips.flush()?;
    all.verify()?;
    let mut scope_sum = Tally::default();
    for bins in scopes.values() {
        bins.verify()?;
        scope_sum.add(bins.total)?;
    }
    let mut function_sum = Tally::default();
    for f in &functions {
        f.bins.verify()?;
        function_sum.add(f.bins.total)?;
    }
    let mut mapped_sum = executable;
    mapped_sum.add(outside)?;
    if all.total != total
        || scope_sum != total
        || function_sum != executable
        || mapped_sum != total
        || executable.period == 0
    {
        return Err("sample conservation or executable coverage failed".into());
    }
    let summary = RunSummary {
        name: run.name.clone(),
        binary: run.binary.clone(),
        total,
        executable,
        outside_executable: outside,
        sampled_instruction_sites: sites,
        sampled_tids: tids.len(),
        load_bias: format!("{:x}", mapping.bias),
    };
    process::json(
        output.join(format!("{}.json", run.name)),
        &serde_json::json!({
            "summary":summary,"all":all,"scopes":scopes,"functions":functions,
            "executable_intervals":mapping.intervals,"sampled_ips":format!("{}-ips.jsonl",run.name),
            "interpretation":"Cycle samples may skid. Instruction families and address-register tags are observations, not exclusive opcode costs, retired instruction counts or removable latency. RSP/RBP operands are not proof of spills or stack traffic; RBP can be a general register. Aliases and external samples remain explicit."
        }),
    )?;
    Ok(summary)
}

pub fn evaluate(config: Config, output: &Path) -> Result<Summary> {
    if config.version != 1 || config.binaries.is_empty() || config.runs.is_empty() {
        return Err("instructions config needs version1, binaries and runs".into());
    }
    let mut binaries = BTreeSet::new();
    for b in &config.binaries {
        if !name_valid(&b.name) || !binaries.insert(&b.name) {
            return Err("invalid/duplicate binary name".into());
        }
        for p in [&b.path, &b.disassembly, &b.symbols] {
            pinned(p, &config.pins)?;
        }
    }
    let mut runs = BTreeSet::new();
    for r in &config.runs {
        if !name_valid(&r.name)
            || !runs.insert(&r.name)
            || !binaries.contains(&r.binary)
            || r.expected_pid == 0
            || r.expected_samples == 0
            || r.expected_period == 0
            || r.expected_tids == 0
        {
            return Err(
                "invalid/duplicate run, binary reference or expected capture totals".into(),
            );
        }
        for p in [&r.metadata, &r.samples] {
            pinned(p, &config.pins)?;
        }
    }
    if output.exists() {
        return Err("instruction output already exists".into());
    }
    // This offline command can run inside `workflow`, whose host lock is held.
    // Input identities and content are checked independently of that lock.
    let guard = process::PinGuard::new(&config.pins)?;
    fs::create_dir_all(output)?;
    process::json(output.join("inputs.json"), &config)?;
    let mut summaries = Vec::new();
    let mut total = Tally::default();
    for binary in &config.binaries {
        let bytes = fs::read(&binary.path)?;
        let elf = elf::Elf::parse(&bytes)?;
        let instructions = decode::instructions(&fs::read_to_string(&binary.disassembly)?, &elf)?;
        let symbols = decode::Symbols::parse(&fs::read_to_string(&binary.symbols)?)?;
        for run in config.runs.iter().filter(|r| r.binary == binary.name) {
            guard.check_identity()?;
            let result = analyze(run, binary, &elf, &instructions, &symbols, output)?;
            total.add(result.total)?;
            summaries.push(result);
        }
    }
    guard.verify()?;
    let result=Summary{version:1,status:"complete; diagnostic only",total,runs:summaries,
        interpretation:"No new proofs or measurements. Cycle-IP association is not a causal cost model or an optimization qualification."};
    process::json(output.join("result.json"), &result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    #[test]
    fn end_to_end_pie_binding_external_samples_and_conservation() {
        let root = std::env::temp_dir().join(format!(
            "zopro-instruction-reader-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let mut bytes = vec![0u8; 0x4000];
        bytes[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
        for (p, v, n) in [
            (16, 3u64, 2),
            (18, 62, 2),
            (32, 64, 8),
            (54, 56, 2),
            (56, 1, 2),
            (64, 1, 4),
            (68, 5, 4),
            (72, 0x1200, 8),
            (80, 0x2200, 8),
            (96, 0x1000, 8),
            (104, 0x1000, 8),
        ] {
            bytes[p..p + n].copy_from_slice(&v.to_le_bytes()[..n]);
        }
        bytes[0x1345..0x134d].copy_from_slice(&[0x48, 0x89, 0xd8, 0x48, 0x8d, 0x44, 0x24, 0x10]);
        let binary = root.join("binary with spaces");
        fs::write(&binary, bytes).unwrap();
        let disassembly = root.join("objdump.txt");
        fs::write(
            &disassembly,
            "2345:\t48 89 d8\tmov %rbx,%rax\n2348:\t48 8d 44 24 10\tlea 0x10(%rsp),%rax\n",
        )
        .unwrap();
        let symbols = root.join("nm.txt");
        fs::write(
            &symbols,
            "2345 8 t zopro_core::msm::stream_g1_impl::<G1Config, true>\n",
        )
        .unwrap();
        let metadata = root.join("mmap.txt");
        let map = format!(
            "17/17 PERF_RECORD_MMAP2 17/17: [0x70002000(0x3000) @ 0x1000 00:00 1 0]: r-xp {}\n",
            binary.display()
        );
        fs::write(&metadata, &map).unwrap();
        let samples = root.join("samples.txt");
        fs::write(
            &samples,
            "17/17 15 70002345 a\n17/18 20 70002348 b\n17/18 7 80001000 external\n",
        )
        .unwrap();
        let mut pins = BTreeMap::new();
        for p in [&binary, &disassembly, &symbols, &metadata, &samples] {
            pins.insert(p.to_str().unwrap().into(), process::sha256(p).unwrap());
        }
        let config = Config {
            version: 1,
            pins,
            binaries: vec![Binary {
                name: "test".into(),
                path: binary.clone(),
                disassembly,
                symbols,
            }],
            runs: vec![Run {
                name: "case".into(),
                binary: "test".into(),
                metadata: metadata.clone(),
                samples,
                expected_pid: 17,
                expected_samples: 3,
                expected_period: 42,
                expected_tids: 2,
            }],
        };
        let result = evaluate(config, &root.join("output")).unwrap();
        assert_eq!(
            result.total,
            Tally {
                samples: 3,
                period: 42
            }
        );
        assert_eq!(
            result.runs[0].executable,
            Tally {
                samples: 2,
                period: 35
            }
        );
        let v: serde_json::Value =
            serde_json::from_slice(&fs::read(root.join("output/case.json")).unwrap()).unwrap();
        assert_eq!(
            v["scopes"]["g1_stream"]["families"]["address_generation"]["period"],
            20
        );
        assert_eq!(
            v["scopes"]["g1_stream"]["memory"]["address_only"]["period"],
            20
        );
        assert_eq!(v["scopes"]["outside_executable"]["total"]["period"], 7);
        let elf_bytes = fs::read(&binary).unwrap();
        let elf = elf::Elf::parse(&elf_bytes).unwrap();
        let conflict = format!("{map}{}", map.replace("0x70002000", "0x71002000"));
        assert!(samples::Mapping::parse(&conflict, 17, binary.to_str().unwrap(), &elf).is_err());
        assert!(samples::Mapping::parse(&map, 18, binary.to_str().unwrap(), &elf).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn bin_overflow_is_an_error_and_names_cannot_escape_output() {
        let mut t = Tally {
            samples: u64::MAX,
            period: 1,
        };
        assert!(t
            .add(Tally {
                samples: 1,
                period: 0
            })
            .is_err());
        for name in ["", "../escape", "a/b", ".", "two words"] {
            assert!(!name_valid(name));
        }
        assert!(name_valid("merge-54x1-w32-accepted"));
    }
}
