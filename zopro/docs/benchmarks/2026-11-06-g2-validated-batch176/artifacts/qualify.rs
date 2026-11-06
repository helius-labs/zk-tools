use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-validated-batch176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/joint-pair52-callers176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g2-validated-batch176";
const F:&str="/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash(p: impl AsRef<Path>) -> String {
    digest(&fs::read(p).unwrap())
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = vec![];
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p))
        } else if p.is_file() {
            v.push(p)
        }
    }
    v.sort();
    v
}
fn main() {
    assert!(!Path::new(R).join("qualification.json").exists());
    assert!(!Path::new(D).exists());
    assert_eq!(read(format!("{R}/build/result.json"))["status"], "complete");
    let mut pins = read(format!("{R}/pins.json")).as_object().unwrap().clone();
    for (p, h) in &pins {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let originals=read(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let audit = read(format!("{R}/source-audit.json"));
    let deltas = read(format!("{R}/source-deltas.json"));
    let mut exact = 0;
    let mut recovered = 0;
    for (name, h) in originals["source_files_sha256"].as_object().unwrap() {
        let p = format!("{R}/source/{name}");
        assert_eq!(hash(format!("{Z}/{name}")), h.as_str().unwrap());
        assert_eq!(hash(&p), audit["hashes"][name]);
        if let Some(d) = deltas.get(name) {
            assert_eq!(
                digest(d["before"].as_str().unwrap().as_bytes()),
                h.as_str().unwrap()
            );
            assert_eq!(
                fs::read_to_string(&p).unwrap(),
                d["after"].as_str().unwrap()
            );
        } else {
            assert_eq!(hash(p), h.as_str().unwrap());
            exact += 1;
        }
        recovered += 1;
    }
    assert_eq!((exact, recovered), (173, 176));
    assert_eq!(audit["hashes"].as_object().unwrap().len(), 178);
    let kernel = fs::read_to_string(format!(
        "{R}/source/crates/zopro-core/src/msm/g2_validated.rs"
    ))
    .unwrap();
    let check = kernel.find("for &(encoded, _) in batch").unwrap();
    assert!(check < kernel.find("let full =").unwrap());
    assert!(check < kernel.find("s.prefix_records.clear()").unwrap());
    assert!(!kernel.contains("batch[k + i]") && !kernel.contains("batch[k+i]"));
    assert_eq!(kernel.matches("unsafe {").count(), 6);
    assert_eq!(kernel.matches("get_unchecked_mut").count(), 2);
    assert_eq!(kernel.matches("get_unchecked(j)").count(), 1);
    let original =
        fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm/g2_borrowed.rs")).unwrap();
    let body = &original[..original
        .find("\nimpl<C: CurveConfig<Base = Fq2>> BatchAdd<C>")
        .unwrap()];
    let changed = &kernel[..kernel.find("#[cfg(test)] mod validation_tests").unwrap()];
    for op in [
        ".mul(",
        ".square(",
        "lanes_norm(",
        "lanes_scaled_numerator(",
        "lanes_sub(",
        "lanes_square(",
        "lanes_mul(",
        "fq2_pack(",
        "fq2_unpack(",
        "direct_chain_inverse(",
        "invert_lanes(",
    ] {
        assert_eq!(
            changed.matches(op).count(),
            body.matches(op).count(),
            "arithmetic sites {op}"
        );
    }
    let plan = read(format!("{R}/build-plan.json"));
    let mut receipts = vec![];
    let mut tests = BTreeMap::new();
    for s in plan["steps"].as_array().unwrap() {
        let name = s["name"].as_str().unwrap();
        let base = Path::new(R).join("build").join(name);
        let exit = read(base.join("exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        let cmd = read(base.join("command.json"));
        assert_eq!(
            &cmd["argv"].as_array().unwrap()[4..],
            s["argv"].as_array().unwrap()
        );
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(base.join(format!("{stream}.log"))),
                exit[format!("{stream}_sha256")]
            );
        }
        if name.starts_with("core-test-") {
            let stdout = fs::read_to_string(base.join("stdout.log")).unwrap();
            assert!(stdout.contains("test result: ok."));
            let summary = stdout
                .lines()
                .find(|s| s.starts_with("test result:"))
                .unwrap();
            tests.insert(name.to_owned(), summary.to_owned());
        }
        receipts.push(json!({"name":name,"command_sha256":hash(base.join("command.json")),"exit_sha256":hash(base.join("exit.json"))}));
    }
    assert_eq!(receipts.len(), 8);
    assert!(tests["core-test-OFF"].contains("1 passed"));
    assert!(tests["core-test-ON"].contains("128 passed"));
    let mut identities = vec![];
    for role in ["OFF", "ON"] {
        let core = format!("{R}/libs/{role}/libzopro_core.rlib");
        let binary = format!("{R}/caller-{role}");
        let corecmd = read(format!("{R}/build/core-{role}/command.json"));
        let drv = read(format!("{R}/build/driver-{role}/command.json"));
        let a = corecmd["argv"].as_array().unwrap();
        assert!(a.contains(&json!(format!("zopro_bn254={F}"))));
        assert_eq!(a.contains(&json!("zopro_validated_g2")), role == "ON");
        let a = drv["argv"].as_array().unwrap();
        assert!(a.contains(&json!(format!("zopro_core={core}"))));
        for f in ["-O", "target-cpu=native", "lto=fat", "codegen-units=1"] {
            assert!(a.contains(&json!(f)));
        }
        assert_eq!(a.contains(&json!("zopro_validated_g2")), role == "ON");
        for p in [&core, &binary] {
            pins.insert(p.clone(), json!(hash(p)));
        }
        identities.push(json!({"role":role,"binary":binary,"sha256":hash(&binary),"core":core,"core_sha256":hash(core),"field":F,"field_sha256":hash(F),"actual_driver_argv":a}));
    }
    let frozen = read(format!("{R}/frozen-reference.json"));
    assert_eq!(
        hash(frozen["binary"].as_str().unwrap()),
        frozen["binary_sha256"]
    );
    for p in [
        format!("{R}/qualify.rs"),
        format!("{R}/qualify"),
        format!("{Q}/codegen.json"),
        format!("{Q}/provenance-note.md"),
        "/usr/bin/nm".into(),
        "/usr/bin/objdump".into(),
    ] {
        pins.insert(p.clone(), json!(hash(p)));
    }
    fs::create_dir(format!("{R}/disassembly")).unwrap();
    let mut rows = vec![];
    for id in &identities {
        let role = id["role"].as_str().unwrap();
        let binary = id["binary"].as_str().unwrap();
        let nm = Command::new("/usr/bin/nm")
            .args(["-S", "--defined-only", binary])
            .output()
            .unwrap();
        assert!(nm.status.success());
        let symbols = String::from_utf8(nm.stdout).unwrap();
        fs::write(format!("{R}/disassembly/{role}-symbols.txt"), &symbols).unwrap();
        for (kind, needle) in [
            ("G1", "msm::stream_g1_impl"),
            (
                "G2",
                if role == "ON" {
                    "msm::g2_validated::apply"
                } else {
                    "msm::g2_borrowed::apply"
                },
            ),
            ("inverse", "inverse_nonzero"),
        ] {
            let matches = symbols
                .lines()
                .filter_map(|l| {
                    let f = l.split_whitespace().collect::<Vec<_>>();
                    if f.len() != 4 {
                        return None;
                    }
                    let name = rustc_demangle::try_demangle(f[3]).ok()?.to_string();
                    if !name.contains(needle)
                        || kind == "G1" && (!name.contains("G1Config") || !name.contains('&'))
                        || kind == "inverse" && !name.contains("FqBatch")
                    {
                        return None;
                    }
                    Some((
                        u64::from_str_radix(f[0], 16).unwrap(),
                        u64::from_str_radix(f[1], 16).unwrap(),
                        f[3].to_owned(),
                        name,
                    ))
                })
                .collect::<Vec<_>>();
            assert_eq!(matches.len(), 1, "{role}/{kind}");
            let (start, size, raw, name) = &matches[0];
            let argv = vec![
                "-d".to_owned(),
                "-Mintel".into(),
                "--no-show-raw-insn".into(),
                format!("--disassemble={raw}"),
                binary.into(),
            ];
            let decode = Command::new("/usr/bin/objdump")
                .args(&argv)
                .output()
                .unwrap();
            assert!(decode.status.success() && decode.stderr.is_empty());
            let text = String::from_utf8(decode.stdout).unwrap();
            let path = format!("{R}/disassembly/{role}-{kind}.txt");
            fs::write(&path, &text).unwrap();
            let inst = text
                .lines()
                .filter_map(|l| {
                    let (a, b) = l.trim().split_once(':')?;
                    u64::from_str_radix(a, 16).ok()?;
                    Some(b.trim())
                })
                .collect::<Vec<_>>();
            let calls = inst
                .iter()
                .filter(|s| s.starts_with("call"))
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let bounds = calls
                .iter()
                .filter(|s| s.contains("panic_bounds_check") || s.contains("slice_index_fail"))
                .count();
            let ifma = inst.iter().filter(|s| s.starts_with("vpmadd52")).count();
            assert_eq!(
                ifma,
                if kind == "G1" {
                    800
                } else if kind == "G2" {
                    1450
                } else {
                    500
                }
            );
            rows.push(json!({"role":role,"kind":kind,"name":name,"raw_symbol":raw,"start":start,"size_bytes":size,"instruction_rows":inst.len(),"ifma":ifma,"bounds_panic_call_sites":bounds,"vector_rsp_rbp_operand_rows":inst.iter().filter(|s|s.starts_with('v')&&(s.contains("[rsp")||s.contains("[rbp"))).count(),"literal_rsp_subtractions":inst.iter().filter(|s|s.starts_with("sub    rsp,")).collect::<Vec<_>>(),"calls":calls,"decode_argv":argv,"disassembly_sha256":hash(path),"new_decode":true}));
        }
    }
    let old = read(format!("{Q}/codegen.json"));
    for row in old["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["role"] == "FROZEN")
    {
        let mut row = row.clone();
        let kind = row["kind"].as_str().unwrap();
        let path = format!("{Q}/disassembly/FROZEN-{kind}.txt");
        assert_eq!(hash(&path), row["disassembly_sha256"]);
        pins.insert(path.clone(), json!(hash(&path)));
        row["inherited_disassembly"] = json!(path);
        row["new_decode"] = json!(false);
        rows.push(row);
    }
    let on = rows
        .iter()
        .find(|r| r["role"] == "ON" && r["kind"] == "G2")
        .unwrap();
    let off = rows
        .iter()
        .find(|r| r["role"] == "OFF" && r["kind"] == "G2")
        .unwrap();
    let removes = on["bounds_panic_call_sites"].as_u64().unwrap()
        < off["bounds_panic_call_sites"].as_u64().unwrap();
    let out = json!({"status":"pass","source_files":178,"accepted_originals_recovered":recovered,"exact_accepted_originals":exact,"field_rebuilt":false,"tests":tests,"commands":receipts,"pins":pins.len(),"six_new_decodes":true,"three_frozen_decodes_inherited":true,"rows":rows,"identities":identities,"static_bounds_sites_reduced":removes,"clock":false,"new_proofs":0,"proof_ledger":30846,"adopted":false,"production_changed":false,"limitations":"Safety checked by source invariants, native boundary/invalid-index tests and unchanged original G2 tests; not a formal machine-code proof. Static sites are not dynamic branch cost, spill cost, lifetime stack or proof latency."});
    save(format!("{R}/qualification-pins.json"), &json!(pins));
    save(format!("{R}/qualification.json"), &out);
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# Once-validated borrowed G2 batch qualification\n\nThe candidate validates encoded bucket indices once before mutation, then uses fixed-size batch groups and justified unchecked bucket/prefix-record accesses in the existing G2 kernel. **Correctness and source/receipt review pass; no latency benchmark has run.** Field arithmetic, retained scratch, input representation and the accepted prover remain unchanged.\n\n## Native qualification\n\nBoth cores and actual callers link the original accepted field library coherently. ON passes128 unfiltered core tests; OFF passes the new shared integration test with125 unchanged tests filtered. The integration covers14 empty/full/tail lengths through511 and1/3/17 reuse counts. Two additional ON tests cover invalid destinations before any write, empty slices and signed bucket-zero indexing. Original G2 sign-mask, exception, tails, reuse and small-stack pool tests also run in ON. Field suites and prior qualified builds are inherited without repetition.\n\nAll eight build/test/link commands succeed. Review checks command argv, exit/output hashes,176 original source recovery and173 byte-exact originals among178 snapshot files. The three changed parents retain reversible deltas; two new children hold the kernel and diagnostic adapters. Arithmetic call-site counts match the original kernel. This source review and testing do not constitute a formal unsafe-code proof.\n\n## Final consumer code\n\nSix newly decoded functions cover fresh OFF/ON G1/G2/vector inverse; three frozen reference decodes are reused with hashes rather than repeated. Counts cover entire static functions, including cold paths. They do not measure executed checks, instruction cost, spills, memory peaks or proof latency.\n\n|Build/function|Instruction rows|IFMA sites|Bytes|Vector stack-operand rows|Bounds-panic call sites (new decodes)|\n|---|---:|---:|---:|---:|---:|\n");
    for row in out["rows"].as_array().unwrap() {
        report.push_str(&format!(
            "|{}/{}|{}|{}|{}|{}|{}|\n",
            row["role"].as_str().unwrap(),
            row["kind"].as_str().unwrap(),
            row["instruction_rows"],
            row["ifma"],
            row["size_bytes"],
            row["vector_rsp_rbp_operand_rows"],
            row.get("bounds_panic_call_sites")
                .map(|v| v.to_string())
                .unwrap_or_else(|| "inherited".into())
        ));
    }
    report.push_str("\n## Safety and scope\n\nThe immutable batch and exact mutable bucket slice are borrowed for the entire call. Every masked destination is validated before any scratch or bucket write. Full chunks partition the original prefix exactly, and the completed forward loop produces one record for each reverse chunk. Points are copied before writes; no simultaneous mutable point aliases are constructed. Tail storage remains checked. Distinct destinations and ordinary nonzero denominators remain existing arithmetic preconditions, not unsafe-memory assumptions. The added validation has a real scan cost, which must be included in any later caller measurement.\n\nNo descriptor array, coordinate validation, hardware gather, point/key copy, lookup table, retained plan, pool or buffer is added. Original field/Fr FFT, arithmetic, inversion, signs, tails, exception guards, queue geometry and production policy remain. This differs from prior gather/descriptor/late-coordinate experiments; no exhaustive novelty claim is made.\n\nBefore the first native compile, a cast comparison was parenthesized in generated child and generator source, and only their source/audit pins were updated. Setup was not repeated. The archived setup binary predates that text correction; building the final setup source emits the corrected child directly. No native compile or test failed in this preparation.\n\nThe frozen389307 control is an audited accepted-source diagnostic caller, distinct from the production SDK. Its recorded field input is described in the inherited [provenance note](../2026-11-06-joint-pair52-callers176/artifacts/provenance-note.md). Fresh OFF/ON explicitly use the original field1200. Paired calibration is still required before performance claims.\n\n## Next step and accepted baseline\n\nOnly if final consumer checks show the intended mechanism is present, freeze a prospective complete-caller comparison with both controls, tails,1/16/32 workers, unchanged G1 controls and automatic PMU/cycle profiles. Keep validation inside the timer. A caller win would select all-eight plus merge54 SDK, resource/portable and independent Gnark proof checks before adoption.\n\nAccepted local32-worker1×2 remains **7.928660 ms**, merge54 **161.883899 ms**. Inherited132.75 MiB RSS is not a new peak or minimum RAM. Ledger30846 is unchanged. All-eight CPU10× remains active and unmet (0/8); inherited [M5 CPU/Metal comparisons](../2026-11-06-joint-pair52-capture176/README.md#accepted-proof-baseline-unchanged) remain current. No new proof, AWS, Aeglos, large-circuit, device, pressure or concurrency measurement is added.\n\n[Qualification](artifacts/qualification.json), [pins](artifacts/qualification-pins.json), [design](artifacts/design.md), [source deltas](artifacts/source-deltas.json), [Rust reviewer/code inspector](artifacts/qualify.rs) and [selected sources/receipts](artifacts/source-and-receipts.json.gz) preserve the implementation. The archive excludes executables, libraries, binary fixtures, keys, witnesses and proofs. Full snapshot and binaries remain local with recorded hashes.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "qualification.json",
        "qualification-pins.json",
        "design.md",
        "source-deltas.json",
        "source-audit.json",
        "build-plan.json",
        "qualify.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let mut members = BTreeMap::new();
    for f in [
        "setup.rs",
        "design.md",
        "driver.rs",
        "perf.rs",
        "source/crates/zopro-core/src/msm/g2_validated.rs",
        "source/crates/zopro-core/src/msm/validated_caller.rs",
        "prebuild-parentheses-correction/setup-initial.rs",
        "prebuild-parentheses-correction/g2_validated-initial.rs",
        "prebuild-parentheses-correction/note.md",
    ] {
        let b = fs::read(format!("{R}/{f}")).unwrap();
        members.insert(
            f.to_owned(),
            json!({"bytes":b.len(),"sha256":digest(&b),"utf8":std::str::from_utf8(&b).unwrap()}),
        );
    }
    for dir in ["build", "disassembly"] {
        for p in walk(&Path::new(R).join(dir)) {
            let b = fs::read(&p).unwrap();
            let text = std::str::from_utf8(&b).unwrap();
            members.insert(
                p.strip_prefix(R).unwrap().display().to_string(),
                json!({"bytes":b.len(),"sha256":digest(&b),"utf8":text}),
            );
        }
    }
    let bytes = serde_json::to_vec(&members).unwrap();
    let path = art.join("source-and-receipts.json.gz");
    let mut gzip = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(&path).unwrap())
        .spawn()
        .unwrap();
    gzip.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(gzip.wait().unwrap().success());
    let expanded = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(&path)
        .output()
        .unwrap();
    assert!(expanded.status.success());
    assert_eq!(expanded.stdout, bytes);
    let v: Value = serde_json::from_slice(&expanded.stdout).unwrap();
    for row in v.as_object().unwrap().values() {
        assert_eq!(
            digest(row["utf8"].as_str().unwrap().as_bytes()),
            row["sha256"]
        );
        assert_eq!(
            row["utf8"].as_str().unwrap().len(),
            row["bytes"].as_u64().unwrap() as usize
        );
    }
    save(
        art.join("publication.json"),
        &json!({"status":"pass","UTF8_archive_members":members.len(),"roundtrip_verified":true,"archive_sha256":hash(path),"new_decodes":6,"inherited_decodes":3,"new_clock":false,"new_proofs":0}),
    );
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(&p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Qualified178sources/all176recovered/eightreceipts/128ONtests/6new+3inheriteddecodes; bounds sites reduced={removes}; published{}UTF8members; no clock.",members.len());
}
