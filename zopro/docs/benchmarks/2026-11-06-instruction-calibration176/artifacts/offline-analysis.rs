use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/instruction-calibration-native176";
const A: &str = "/home/ubuntu/src/zopro-perf-work/instruction-calibration176";
const N: u64 = 1048576;
fn p(s: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(s)
}
fn hash(s: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(s).unwrap()))
}
fn read(s: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(s).unwrap()).unwrap()
}
fn save(s: impl AsRef<Path>, v: &Value) {
    fs::write(s, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn u16at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes(b[o..o + 2].try_into().unwrap())
}
fn u32at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn u64at(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}
fn cstr(b: &[u8], o: usize) -> &str {
    let n = b[o..].iter().position(|v| *v == 0).unwrap();
    std::str::from_utf8(&b[o..o + n]).unwrap()
}
fn text(id: &str) -> (Vec<u8>, Value) {
    let file = format!("{A}/objects/{id}.o");
    let b = fs::read(&file).unwrap();
    assert_eq!(&b[..7], b"\x7fELF\x02\x01\x01");
    assert_eq!(u16at(&b, 16), 1);
    assert_eq!(u16at(&b, 18), 62);
    let base = u64at(&b, 40) as usize;
    let count = u16at(&b, 60) as usize;
    assert_eq!(u16at(&b, 58), 64);
    let ns = base + 64 * u16at(&b, 62) as usize;
    let names = &b[u64at(&b, ns + 24) as usize..][..u64at(&b, ns + 32) as usize];
    let mut ix = None;
    let mut off = 0;
    let mut len = 0;
    for i in 0..count {
        let sh = base + 64 * i;
        let name = cstr(names, u32at(&b, sh) as usize);
        if u64at(&b, sh + 8) & 4 != 0 {
            assert_eq!(name, ".text");
            assert!(ix.is_none());
            ix = Some(i);
            assert_eq!(u32at(&b, sh + 4), 1);
            off = u64at(&b, sh + 24) as usize;
            len = u64at(&b, sh + 32) as usize;
        }
    }
    let ix = ix.unwrap();
    assert!(len > 0);
    let mut found = false;
    for i in 0..count {
        let sh = base + 64 * i;
        let typ = u32at(&b, sh + 4);
        let start = u64at(&b, sh + 24) as usize;
        let size = u64at(&b, sh + 32) as usize;
        if (typ == 4 || typ == 9) && u32at(&b, sh + 44) as usize == ix {
            assert_eq!(size, 0, "text relocation");
        }
        if typ == 2 {
            assert_eq!(u64at(&b, sh + 56), 24);
            let st = base + 64 * u32at(&b, sh + 40) as usize;
            let strings = &b[u64at(&b, st + 24) as usize..][..u64at(&b, st + 32) as usize];
            for j in (start..start + size).step_by(24) {
                let name = cstr(strings, u32at(&b, j) as usize);
                let ndx = u16at(&b, j + 6);
                assert!(!(ndx == 0 && !name.is_empty()), "undefined symbol {name}");
                if b[j + 4] & 15 == 2 {
                    assert_eq!(name, "foo");
                    assert!(!found);
                    assert_eq!(ndx as usize, ix);
                    assert_eq!(u64at(&b, j + 8), 0);
                    assert_eq!(u64at(&b, j + 16) as usize, len);
                    found = true;
                }
            }
        }
    }
    assert!(found);
    let code = b[off..off + len].to_vec();
    assert_eq!(code.last(), Some(&0xc3));
    let meta = json!({"object":file,"object_sha256":hash(&file),"text_file_offset":off,"text_bytes":len,"text_sha256":format!("{:x}",Sha256::digest(&code)),"entry":0,"text_relocations":0});
    (code, meta)
}
fn step(name: String, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":120})
}
fn prepare() {
    assert!(!p("plan.json").exists());
    let cs = read(format!("{A}/cases.json"));
    assert_eq!(cs.as_array().unwrap().len(), 16);
    let prep = read(format!("{A}/prepared.json"));
    assert_eq!(prep["status"], "pass");
    for (s, h) in prep["output_pins"].as_object().unwrap() {
        assert_eq!(h, &json!(hash(s)));
    }
    let mut elf = vec![];
    let mut pins = BTreeMap::new();
    for c in cs.as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let (_, meta) = text(id);
        pins.insert(
            meta["object"].as_str().unwrap().to_string(),
            meta["object_sha256"].as_str().unwrap().to_string(),
        );
        pins.insert(
            c["snippet"].as_str().unwrap().to_string(),
            hash(c["snippet"].as_str().unwrap()),
        );
        elf.push(meta);
    }
    for f in ["campaign.rs", "campaign", "protocol.md"] {
        pins.insert(p(f).display().to_string(), hash(p(f)));
    }
    for f in [
        "cases.json",
        "prepared.json",
        "host.json",
        "plan.json",
        "capture-plan.json",
        "pins.json",
        "capture-pins.json",
        "affinity-premeasurement-failure/campaign.rs",
        "affinity-premeasurement-failure/campaign",
        "affinity-premeasurement-failure/protocol.md",
        "affinity-premeasurement-failure/pins.json",
        "runs/b00-ifma-lo-acc-lat/stderr.log",
        "runs/b00-ifma-lo-acc-lat/exit.json",
        "runs-affinity-fixed/b00-ifma-lo-acc-lat/stderr.log",
        "runs-affinity-fixed/b00-ifma-lo-acc-lat/exit.json",
    ] {
        pins.insert(format!("{A}/{f}"), hash(format!("{A}/{f}")));
    }
    pins.insert(
        "/usr/include/linux/perf_event.h".into(),
        hash("/usr/include/linux/perf_event.h"),
    );
    let xt = "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
    pins.insert(xt.into(), hash(xt));
    save(p("pins.json"), &json!(pins));
    save(p("elf.json"), &json!(elf));
    let mut steps = vec![];
    for b in 0..15 {
        let mut ix = (0..16).map(|j| (j + b) % 16).collect::<Vec<_>>();
        if b % 2 == 1 {
            ix.reverse();
        }
        for i in ix {
            let c = &cs[i];
            let id = c["id"].as_str().unwrap();
            steps.push(step(
                format!("b{b:02}-{id}"),
                vec![
                    "sudo".into(),
                    "-n".into(),
                    "taskset".into(),
                    "-c".into(),
                    "2".into(),
                    p("campaign").display().to_string(),
                    "measure".into(),
                    id.into(),
                    b.to_string(),
                ],
            ));
        }
    }
    steps.push(step(
        "analyze".into(),
        vec![p("campaign").display().to_string(), "analyze".into()],
    ));
    save(
        p("plan.json"),
        &json!({"version":1,"environment":{},"source_pins":p("pins.json"),"steps":steps}),
    );
    let h = Command::new("lscpu").arg("-J").output().unwrap();
    assert!(h.status.success());
    save(
        p("host.json"),
        &json!({"lscpu":serde_json::from_slice::<Value>(&h.stdout).unwrap(),"cpuinfo":fs::read_to_string("/proc/cpuinfo").unwrap(),"cpu2_siblings":fs::read_to_string("/sys/devices/system/cpu/cpu2/topology/thread_siblings_list").unwrap(),"nonexclusive":true}),
    );
    println!("16 existing ELF objects checked without execution; 240 processes + analyzer frozen");
}
struct Group {
    cycle: i32,
    inst: i32,
    cycle_id: u64,
    inst_id: u64,
}
impl Group {
    fn open(config: u64, group: i32) -> i32 {
        let mut attr = [0u64; 16];
        attr[0] = 128u64 << 32;
        attr[1] = config;
        attr[4] = 15;
        attr[5] = (1 << 5) | (1 << 6) | if group < 0 { 1 | (1 << 2) } else { 0 };
        let fd = unsafe { libc::syscall(libc::SYS_perf_event_open, attr.as_ptr(), 0, -1, group, 8) }
            as i32;
        assert!(
            fd >= 0,
            "perf_event_open: {}",
            std::io::Error::last_os_error()
        );
        fd
    }
    fn new() -> Self {
        let cycle = Self::open(0, -1);
        let inst = Self::open(1, cycle);
        let mut cycle_id = 0;
        let mut inst_id = 0;
        assert_eq!(
            unsafe { libc::ioctl(cycle, 0x80082407u64, &mut cycle_id) },
            0
        );
        assert_eq!(unsafe { libc::ioctl(inst, 0x80082407u64, &mut inst_id) }, 0);
        assert_ne!(cycle_id, inst_id);
        Self {
            cycle,
            inst,
            cycle_id,
            inst_id,
        }
    }
    fn ioctl(&self, code: u64) {
        assert_eq!(
            unsafe { libc::ioctl(self.cycle, code, 1) },
            0,
            "ioctl {code}"
        );
    }
    fn read(&self) -> [u64; 7] {
        let mut buf = [0u64; 7];
        assert_eq!(
            unsafe { libc::read(self.cycle, buf.as_mut_ptr() as *mut _, 56) },
            56
        );
        assert_eq!(buf[0], 2);
        assert_eq!(buf[4], self.cycle_id);
        assert_eq!(buf[6], self.inst_id);
        buf
    }
}
impl Drop for Group {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.inst);
            libc::close(self.cycle);
        }
    }
}
fn cpu() {
    unsafe {
        let mut mask: libc::cpu_set_t = std::mem::zeroed();
        assert_eq!(
            libc::sched_getaffinity(0, std::mem::size_of_val(&mask), &mut mask),
            0
        );
        assert_eq!(libc::CPU_COUNT(&mask), 1);
        assert!(libc::CPU_ISSET(2, &mask));
        assert_eq!(libc::sched_getcpu(), 2);
    }
}
fn measure(id: &str, b: u64) {
    let cs = read(format!("{A}/cases.json"));
    assert!(cs.as_array().unwrap().iter().any(|c| c["id"] == id));
    assert!(b < 15);
    cpu();
    assert!(
        std::arch::is_x86_feature_detected!("avx512f")
            && std::arch::is_x86_feature_detected!("avx512ifma")
    );
    let (code, elf) = text(id);
    let mapped = (code.len() + 4095) & !4095;
    let code_ptr = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            mapped,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    assert_ne!(code_ptr, libc::MAP_FAILED);
    assert_eq!(code_ptr as usize % 4096, 0);
    unsafe {
        std::ptr::copy_nonoverlapping(code.as_ptr(), code_ptr as *mut u8, code.len());
    }
    assert_eq!(
        unsafe { libc::mprotect(code_ptr, mapped, libc::PROT_READ | libc::PROT_EXEC) },
        0
    );
    let size = 1 << 20;
    let scratch = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    assert_ne!(scratch, libc::MAP_FAILED);
    unsafe {
        std::ptr::write_bytes(scratch as *mut u8, 0x5a, size);
    }
    let f: unsafe extern "C" fn(*mut u8) = unsafe { std::mem::transmute(code_ptr) };
    let group = Group::new();
    let mut before = group.read();
    let mut rows = vec![];
    for k in 0..7 {
        cpu();
        group.ioctl(0x2403);
        group.ioctl(0x2400);
        unsafe {
            std::hint::black_box(f)(std::hint::black_box(scratch as *mut u8));
        }
        group.ioctl(0x2401);
        cpu();
        let r = group.read();
        let enabled = r[1] - before[1];
        let running = r[2] - before[2];
        assert!(enabled > 0 && running > 0 && running <= enabled);
        let coverage = running as f64 / enabled as f64;
        assert!(coverage >= 0.99, "counter coverage {coverage}");
        assert!(
            (0.98 * N as f64..=1.03 * N as f64).contains(&(r[5] as f64)),
            "retired {}",
            r[5]
        );
        assert!(r[3] > 0);
        rows.push(json!({"repeat":k,"cycles":r[3],"instructions":r[5],"enabled_ns":enabled,"running_ns":running,"running_fraction":coverage,"cpu_before":2,"cpu_after":2,"cycle_event_id":r[4],"instruction_event_id":r[6]}));
        before = r;
    }
    assert_eq!(unsafe { libc::munmap(code_ptr, mapped) }, 0);
    assert_eq!(unsafe { libc::munmap(scratch, size) }, 0);
    let mean = rows
        .iter()
        .map(|r| r["cycles"].as_u64().unwrap() as f64 / N as f64)
        .sum::<f64>()
        / 7.0;
    println!(
        "{}",
        json!({"id":id,"block":b,"status":"pass","target_instructions":N,"mean_cycles_per_target_instruction":mean,"elf":elf,"repetitions":rows,"cpu":2,"scratch_bytes":size,"scratch_fill":90,"event_type":0,"cycle_config":0,"instruction_config":1,"read_format":15,"flags":{"leader":101,"follower":96}})
    );
}
fn median(x: &[f64]) -> f64 {
    let mut x = x.to_vec();
    x.sort_by(f64::total_cmp);
    x[x.len() / 2]
}
fn summary(x: &[f64]) -> Value {
    let mut seed = 0x2545f4914f6cdd1du64;
    let mut boots = vec![];
    for _ in 0..10000 {
        let mut row = vec![];
        for _ in x {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            row.push(x[seed as usize % x.len()]);
        }
        boots.push(median(&row));
    }
    boots.sort_by(f64::total_cmp);
    json!({"median":median(x),"bootstrap_95":[boots[250],boots[9749]],"min":x.iter().copied().fold(f64::INFINITY,f64::min),"max":x.iter().copied().fold(f64::NEG_INFINITY,f64::max),"all_blocks":x})
}
fn analyze() {
    assert!(!p("analysis.json").exists());
    for (s, h) in read(p("pins.json")).as_object().unwrap() {
        assert_eq!(h, &json!(hash(s)), "{s}");
    }
    let cs = read(format!("{A}/cases.json"));
    let mut rows = vec![];
    let mut receipts = vec![];
    let mut mincov = 1.0f64;
    let mut minins = u64::MAX;
    let mut maxins = 0;
    for c in cs.as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let mut xs = vec![];
        for b in 0..15 {
            let d = p(format!("runs/b{b:02}-{id}"));
            let e = read(d.join("exit.json"));
            assert_eq!(e["success"], true);
            assert_eq!(e["returncode"], 0);
            assert_eq!(e["stdout_sha256"], hash(d.join("stdout.log")));
            assert_eq!(e["stderr_sha256"], hash(d.join("stderr.log")));
            assert!(fs::read(d.join("stderr.log")).unwrap().is_empty());
            let v = read(d.join("stdout.log"));
            assert_eq!(v["id"], id);
            assert_eq!(v["block"], b);
            assert_eq!(v["status"], "pass");
            assert_eq!(v["target_instructions"], N);
            assert_eq!(v["cpu"], 2);
            assert_eq!(
                v["elf"]["object_sha256"],
                hash(format!("{A}/objects/{id}.o"))
            );
            let reps = v["repetitions"].as_array().unwrap();
            assert_eq!(reps.len(), 7);
            let mut total = 0.0;
            for (k, r) in reps.iter().enumerate() {
                assert_eq!(r["repeat"], k);
                assert_eq!(r["cpu_before"], 2);
                assert_eq!(r["cpu_after"], 2);
                let enabled = r["enabled_ns"].as_u64().unwrap();
                let running = r["running_ns"].as_u64().unwrap();
                assert!(enabled > 0 && running > 0 && running <= enabled);
                let cov = running as f64 / enabled as f64;
                assert!(cov >= 0.99);
                assert_eq!(cov, r["running_fraction"].as_f64().unwrap());
                mincov = mincov.min(cov);
                let ins = r["instructions"].as_u64().unwrap();
                assert!((0.98 * N as f64..=1.03 * N as f64).contains(&(ins as f64)));
                minins = minins.min(ins);
                maxins = maxins.max(ins);
                let cycles = r["cycles"].as_u64().unwrap();
                assert!(cycles > 0);
                total += cycles as f64 / N as f64;
            }
            let mean = total / 7.0;
            let serialized = v["mean_cycles_per_target_instruction"].as_f64().unwrap();
            assert!((mean - serialized).abs() <= 1e-12 * mean.abs().max(1.0));
            xs.push(mean);
            receipts.push(json!({"id":id,"block":b,"stdout_sha256":hash(d.join("stdout.log")),"exit_sha256":hash(d.join("exit.json"))}));
        }
        rows.push(json!({"id":id,"scope":c["scope"],"cycles_per_target_instruction":summary(&xs)}));
    }
    let mut ratios = vec![];
    for op in ["lo", "hi"] {
        for (a, b, name) in [
            (
                format!("ifma-{op}-acc-source-lat"),
                format!("ifma-{op}-acc-lat"),
                "combined_alias_over_accumulator",
            ),
            (
                format!("ifma-{op}-mem-tp"),
                format!("ifma-{op}-reg-tp"),
                "folded_hot_memory_over_register",
            ),
        ] {
            let x = rows.iter().find(|v| v["id"] == a).unwrap();
            let y = rows.iter().find(|v| v["id"] == b).unwrap();
            let xs = x["cycles_per_target_instruction"]["all_blocks"]
                .as_array()
                .unwrap()
                .iter()
                .zip(
                    y["cycles_per_target_instruction"]["all_blocks"]
                        .as_array()
                        .unwrap(),
                )
                .map(|(a, b)| a.as_f64().unwrap() / b.as_f64().unwrap())
                .collect::<Vec<_>>();
            ratios.push(json!({"operation":op,"comparison":name,"ratio":summary(&xs)}));
        }
    }
    let out = json!({"status":"pass","calibration_only":true,"production_changed":false,"cases":rows,"paired_ratios":ratios,"timed_processes":240,"total_repetitions":1680,"minimum_running_fraction":mincov,"retired_instruction_range":[minins,maxins],"receipts":receipts,"new_proofs":0,"proof_ledger":30846,"plan_sha256":hash(p("plan.json"))});
    save(p("analysis.json"), &out);
    for r in out["cases"].as_array().unwrap() {
        println!(
            "{}: {} cycles/target instruction",
            r["id"], r["cycles_per_target_instruction"]["median"]
        );
    }
}
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    match a.get(1).map(String::as_str) {
        Some("prepare") => prepare(),
        Some("measure") => measure(&a[2], a[3].parse().unwrap()),
        Some("analyze") => analyze(),
        _ => panic!("prepare|measure|analyze"),
    }
}
