use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-half-roots176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-inverse-half-roots176";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash_file(p: &Path) -> String {
    let mut file = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        h.update(&buffer[..n]);
    }
    format!("{:x}", h.finalize())
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = Vec::new();
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p))
        } else {
            v.push(p)
        }
    }
    v.sort();
    v
}
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let text = std::str::from_utf8(&b).expect("only explicit UTF8 helper/delta/receipt members");
    json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text})
}
fn archive(path: &Path, members: &BTreeMap<String, Value>) {
    let bytes = serde_json::to_vec(members).unwrap();
    let mut p = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(path).unwrap())
        .spawn()
        .unwrap();
    p.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(p.wait().unwrap().success());
    let out = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(path)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&out.stdout).unwrap();
    for v in decoded.as_object().unwrap().values() {
        let b = v["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(b.len(), v["bytes"].as_u64().unwrap() as usize);
        assert_eq!(hash(b), v["sha256"]);
    }
}
fn main() {
    assert!(!Path::new(D).exists());
    let review = read(format!("{R}/review.json"));
    assert_eq!(review["review_passed"], true);
    let codegen = read(format!("{R}/codegen.json"));
    let mut observations = BTreeMap::new();
    for arm in ["OFF", "ON"] {
        let selected = read(format!("{R}/selected-codegen-{arm}.json"));
        let inverse: Vec<_> = selected
            .as_object()
            .unwrap()
            .iter()
            .filter(|(k, _)| k.contains("inverse_nonzero"))
            .collect();
        assert_eq!(inverse.len(), 1);
        let lines: Vec<_> = inverse[0].1.as_str().unwrap().lines().collect();
        let first_scalar = lines.iter().position(|l| l.contains("\tmulx ")).unwrap();
        let call = lines
            .iter()
            .position(|l| l.contains("\tcall ") && l.contains("inverse::inverse"))
            .unwrap();
        let last_ifma = (0..first_scalar)
            .filter(|&i| lines[i].contains("vpmadd52"))
            .last()
            .unwrap();
        assert!(lines.iter().any(|l| l.contains("sub    $0x438,%rsp")));
        let mut stores = Vec::new();
        let mut bytes = 0;
        for l in &lines[last_ifma + 1..first_scalar] {
            if !l.contains("\tvmov") {
                continue;
            }
            if ["0x30(%rsp)", "0x40(%rsp)", "0x50(%rsp)", "0x60(%rsp)"]
                .iter()
                .any(|dest| l.trim().ends_with(&format!(",{dest}")))
            {
                let width = if l.contains("%ymm") {
                    32
                } else if l.contains("%xmm") {
                    16
                } else {
                    panic!("unexpected root store {l}")
                };
                bytes += width;
                stores.push(json!({"line":l,"bytes":width}));
            }
        }
        assert_eq!(bytes, 64);
        assert_eq!(stores.len(), if arm == "OFF" { 2 } else { 4 });
        observations.insert(arm,json!({"root_payload_bytes":bytes,"root_stores":stores,"first_scalar_product_row":first_scalar,"scalar_inverse_call_row":call,"explicit_stack_adjustment_bytes":0x438,"not_total_stack_or_dynamic_cost":true,"consumer_sha256":codegen[arm]["consumer_sha256"]}));
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    fs::write(
        art.join("root-export-observation.json"),
        serde_json::to_vec_pretty(&observations).unwrap(),
    )
    .unwrap();
    let mut report=String::from("# Shared inverse half-root export\n\n**Closed before timing; not adopted.** The new inverse output checks pass, but generated code shows that the original already exports only the two required roots. Splitting the export through the four-lane halves repeats reduction/packing work rather than removing six materialized host fields. No proving-time gain or measured slowdown is claimed.\n\n");
    report.push_str(&fs::read_to_string(format!("{R}/decision.md")).unwrap());
    report.push_str("\n\n## Exact change and preparation\n\nNative FqBatch::inverse_nonzero keeps its existing pair/quad product tree. OFF calls quads.get and uses lanes 0/4. ON uses the existing four-lane low/high halves, calls get()[0] for each, and performs the same scalar root product/inverse and reverse multiplications. The accepted field library and both export implementations are reused unchanged. The portable/nonIFMA body is unchanged and is not newly phone-qualified. No table, key/image/plan copy, pool, runtime memory cap or new production allocation is introduced.\n\nThis is distinct from the [direct inverse finish](../2026-10-28-inverse-finish-sdk/README.md), [whole-array word export](../2026-11-06-word-export176/README.md) and [LogJumps](../2026-10-31-ifma-logjumps176/README.md) studies; their failed or unqualified observations remain unchanged. The prototype tries to remove unused root materialization, not to change field reduction or inverse arithmetic.\n\nSix Rust/xtask commands pass once: OFF/ON core builds, focused test build/run and OFF/ON complete-caller links. Test execution is one new test with 125 originals filtered, not a fresh full-core suite. It checks 1,025 lazy/full-width/pattern cases, 8,200 inverse lanes and 2,050 root exports against the unchanged scalar and representation oracles. These are not new independent Arkworks populations. The previous field qualification is inherited without rebuilding the field. Complete G1/G2 adapter bodies are copied exactly from their earlier checked callers; the new drivers are linked but not executed here.\n\nThe source/receipt/identity reviewer checks 428 pins, six command/exit/hash receipts, all 176 accepted source recovery, immutable library/consumer identities and actual native/O3/fatLTO/oneCGU flags. The two actual final consumers are decoded once after preparation. No clock observation, profiling, SDK proof or additional capture is selected.\n\n## Actual final-consumer code\n\nCounts include the saved instruction rows and operands; they are not dynamic execution counts, measured spills or cycle costs.\n\n|Function|Arm|Instruction rows|IFMA sites|RSP operand rows|YMM operand rows|ZMM operand rows|Unpack rows|\n|---|---|---:|---:|---:|---:|---:|---:|\n");
    for arm in ["OFF", "ON"] {
        for f in codegen[arm]["functions"].as_array().unwrap() {
            report.push_str(&format!(
                "|{}|{}|{}|{}|{}|{}|{}|{}|\n",
                f["symbol"].as_str().unwrap().trim_end_matches(':'),
                arm,
                f["instruction_rows"],
                f["IFMA_sites"],
                f["RSP_operand_rows"],
                f["YMM_operand_rows"],
                f["ZMM_operand_rows"],
                f["unpack_rows"]
            ));
        }
    }
    report.push_str("\nThe root-export reader inspects the saved code between the final forward IFMA and the first scalar root-product multiply. OFF uses two 32-byte root stores; ON uses four 16-byte stores. Both materialize 64 bytes, not 256. The exact instructions, consumer hashes, scalar product/call positions and 0x438-byte explicit frame adjustment are retained in the observation artifact. This is a source-premise rejection, not a timing gate or proof that instruction count alone predicts speed. No alignment/attribute/layout/width sweep follows.\n\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let section = &old[old
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    report.push_str(&section[..section.find("\n[Analysis]").unwrap()]);
    report.push_str("\n\n## Continuation and artifacts\n\nThe [next question](artifacts/next.md) is whether canonicalization can be deferred across the root product itself, subject to an exact range/implementation/history audit. It is not yet an implemented or qualified candidate. The current half-root route stays closed.\n\nPublisher checks all embedded archive hashes and lossless roundtrips, source deltas, preserved command and stream bindings, and root-store payload observations without repeating native builds/tests or decodes. Full snapshots/binaries and unselected full disassembly stay local with their saved hashes. Git contains all selected inverse/G1/G2 code, source bindings/deltas, tests and receipts. No private key, image, witness, point/scalar cache, proof or executable/library bytes are embedded.\n\n[Design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [test](artifacts/tests.rs), [identities](artifacts/identities.json), [codegen](artifacts/codegen.json), [root export observation](artifacts/root-export-observation.json), [OFF bodies](artifacts/selected-codegen-OFF.json), [ON bodies](artifacts/selected-codegen-ON.json), [native plan](artifacts/build-plan.json), [pins](artifacts/build-pins.json), [source deltas](artifacts/source-deltas.json.gz), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS).\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "design.md",
        "decision.md",
        "next.md",
        "source-audit.json",
        "review.json",
        "review.rs",
        "tests.rs",
        "driver.rs",
        "perf.rs",
        "identities.json",
        "frozen-reference.json",
        "codegen.json",
        "selected-codegen-OFF.json",
        "selected-codegen-ON.json",
        "build-plan.json",
        "build-pins.json",
        "setup.rs",
        "publish.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let source = read(format!("{R}/source-audit.json"));
    let mut deltas = BTreeMap::new();
    for (p, h) in source["source_files"].as_object().unwrap() {
        let path = Path::new(R).join("source").join(p);
        assert_eq!(hash_file(&path), h.as_str().unwrap());
        if source["accepted_source_files"]
            .get(p)
            .is_some_and(|old| old == h)
        {
            continue;
        }
        deltas.insert(p.clone(), member(&path));
    }
    assert_eq!(deltas.len(), 5);
    archive(&art.join("source-deltas.json.gz"), &deltas);
    let mut helpers = BTreeMap::new();
    let mut full_decodes = BTreeMap::new();
    for e in fs::read_dir(R).unwrap() {
        let p = e.unwrap().path();
        let name = p.file_name().unwrap().to_string_lossy();
        if name.starts_with("codegen-") && name.ends_with(".stdout.log") {
            full_decodes.insert(name.to_string(), json!({"local_path":p,"bytes":fs::metadata(&p).unwrap().len(),"sha256":hash_file(&p),"storage":"local original full decode; selected bodies embedded separately"}));
            continue;
        }
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log"].iter().any(|s| e == *s))
        {
            helpers.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    assert_eq!(full_decodes.len(), 2);
    fs::write(
        art.join("full-disassembly-index.json"),
        serde_json::to_vec_pretty(&full_decodes).unwrap(),
    )
    .unwrap();
    for p in walk(&Path::new(R).join("build-runs")) {
        helpers.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
    }
    archive(&art.join("helpers-and-receipts.json.gz"), &helpers);
    fs::write(art.join("publication.json"),serde_json::to_vec_pretty(&json!({"passed":true,"new_native_commands":6,"new_focused_tests":1,"original_tests_filtered":125,"new_final_consumers_decoded":2,"source_originals_recovered":176,"source_delta_UTF8_members":deltas.len(),"helper_receipt_UTF8_members":helpers.len(),"archive_hashes_and_roundtrip_verified":true,"root_payload_bytes_both":64,"root_export_premise_disproved_in_final_consumer":true,"performance_capture_selected":false,"new_clock_observations":0,"new_proofs":0,"adopted":false,"production_changed":false,"goal_status":"active/unmet0/8"})).unwrap()).unwrap();
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash_file(&p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published {}files/{}exactUTF8receipts/5source deltas; rootpayload64Bboth/hash+roundtripPASS; closed before timing",walk(Path::new(D)).len(),helpers.len());
}
