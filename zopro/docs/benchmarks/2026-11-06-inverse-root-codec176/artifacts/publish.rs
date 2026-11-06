use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-root-codec176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-inverse-root-codec176";
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
    let audit = read(format!("{R}/source-audit.json"));
    for arm in ["OFF", "ON"] {
        let inv: Vec<_> = codegen[arm]["functions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v["symbol"].as_str().unwrap().contains("inverse_nonzero"))
            .collect();
        assert_eq!(inv.len(), 1);
        assert_eq!(inv[0]["IFMA_sites"], 500);
        assert_eq!(
            inv[0]["vector_compare_rows"],
            if arm == "OFF" { 1 } else { 0 }
        );
        let id = &review["identities"][arm];
        assert_eq!(
            hash_file(Path::new(id["binary"].as_str().unwrap())),
            id["binary_sha256"]
        );
        assert_eq!(
            hash_file(Path::new(id["core"].as_str().unwrap())),
            id["core_sha256"]
        );
    }
    assert_eq!(
        hash_file(Path::new(
            review["new_shared_field_library"]["path"].as_str().unwrap()
        )),
        review["new_shared_field_library"]["sha256"]
    );
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# Native lazy inverse-root product codec\n\n**Correctness and emitted-work qualification passed; performance is not measured.** The codec defers canonicalization until after the scalar root product. In the actual optimized inverse consumer it removes the vector canonical comparison and reduces static instruction rows by 24. This selects a separate matched complete-caller experiment, not production adoption.\n\n");
    report.push_str(&fs::read_to_string(format!("{R}/decision.md")).unwrap());
    report.push_str("\n\n## Implementation and qualification\n\nThe [raw scalar range proof](../2026-11-06-inverse-lazy-root-product176/README.md) establishes that the existing scalar backend accepts the proposed normalized lazy input range and returns a canonical field. Its 22,616 cases are inherited without rerun. The new `Ifma8::root_product` packs raw lanes 0 and 4, calls that unchanged backend and constructs Fp only after the canonical result. R260 uses the original conversion/product fallback. The [half-root export experiment](../2026-11-06-inverse-half-roots176/README.md) remains closed: it did not defer a reduction across multiplication and added work.\n\nOne changed field library is shared by the fresh OFF/ON core and test consumers. OFF retains the accepted midpoint root conversion/product; ON selects the new method through a compile-time flag. Both use the same caller source. The pre-existing frozen control is source176 with the matching complete G1/G2 adapters and optimized native link, not the accepted proof SDK. All final links explicitly use native targeting, O3, fat LTO and one CGU. Every immutable identity and field link is retained.\n\nTen new Rust/xtask commands pass once: the changed generator check; field-codec test build/run; changed field library; OFF/ON cores; focused shared-inverse test build/run; OFF/ON final caller links. Two new Arkworks populations cover mixed R256 and uniform R260 with Fq/Fr, 17,168 root-product configurations. The field test process reports two passed and 35 original tests filtered. The existing shared-inverse population is reapplied to the changed codec, one passed and 125 originals filtered; its 8,200 lane checks are not a new independent population. No original successful suite, scalar range harness, performance capture or decoder is rerun.\n\nThe source snapshot has 179 files: 171 originals exact, five reversible/append-only original-file changes and three children. All 176 production source bytes recover exactly. Generator and generated source match. Existing field-operation bodies, scalar inverse, point arithmetic and workspace forecasts remain unchanged. The independent review checks 434 source/support pins and command/exit/stream hashes before decoding the two actual final consumers once.\n\n## Actual optimized final consumers\n\nStatic counts are instruction/operand rows, not dynamic execution cost or measured spills.\n\n|Function|Arm|Instruction rows|IFMA sites|Vector compare rows|RSP operand rows|YMM operand rows|ZMM operand rows|\n|---|---|---:|---:|---:|---:|---:|---:|\n");
    for arm in ["OFF", "ON"] {
        for f in codegen[arm]["functions"].as_array().unwrap() {
            let symbol = f["symbol"].as_str().unwrap();
            let label = if symbol.contains("inverse_nonzero") {
                "FqBatch::inverse_nonzero"
            } else if symbol.contains("stream_g1_impl") {
                "G1 stream_g1_impl"
            } else {
                "G2 borrowed apply"
            };
            report.push_str(&format!(
                "|`{label}`|{arm}|{}|{}|{}|{}|{}|{}|\n",
                f["instruction_rows"],
                f["IFMA_sites"],
                f["vector_compare_rows"],
                f["RSP_operand_rows"],
                f["YMM_operand_rows"],
                f["ZMM_operand_rows"]
            ));
        }
    }
    report.push_str("\nBoth inverse functions reserve 0x438 bytes explicitly and push six registers. This is not a total stack peak, and neither stack operands nor instruction counts establish latency. Actual allocation/RSS/fault/CPU and speed evidence is deferred to complete callers. There is no new architectural table, key/plan copy, pool, heap buffer or RAM cap.\n\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let section = &old[old
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    report.push_str(&section[..section.find("\n[Analysis]").unwrap()]);
    report.push_str("\n\n## Continuation and artifacts\n\nThe [first unfinished stage](artifacts/next.md) is a separately frozen complete G1/G2 caller protocol with actual batch sizes and tails, solo/physical16/SMT32, both controls, resources and automatic profiles. No candidate clock observation, SDK/Gnark proof, AWS/Aeglos, huge-circuit, phone/ARM/TEE, pressure/swap, minimum-RAM or proof-throughput result is added here. The full all-eight CPU10× goal remains active and unmet.\n\nThe publication includes eight exact changed/new UTF8 source members relative to accepted176, all command/test receipts, selected complete function bodies, new field/core/caller identities and hash/roundtrip verification. Full disassembly remains local with a byte/hash index; it is not decoded again for packaging. No private input/key/image/witness/proof bytes or executable/library blobs are embedded.\n\n[Design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [new field method/tests](artifacts/root_product.rs), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [identities](artifacts/identities.json), [codegen](artifacts/codegen.json), [OFF bodies](artifacts/selected-codegen-OFF.json), [ON bodies](artifacts/selected-codegen-ON.json), [native plan](artifacts/build-plan.json), [pins](artifacts/build-pins.json), [source deltas](artifacts/source-deltas.json.gz), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [full-decode index](artifacts/full-disassembly-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS).\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "design.md",
        "decision.md",
        "next.md",
        "root_product.rs",
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
    let mut delta = BTreeMap::new();
    for (p, h) in audit["source_files"].as_object().unwrap() {
        let path = Path::new(R).join("source").join(p);
        assert_eq!(hash_file(&path), h.as_str().unwrap());
        if audit["accepted_source_files"]
            .get(p)
            .is_some_and(|old| old == h)
        {
            continue;
        }
        delta.insert(p.clone(), member(&path));
    }
    assert_eq!(delta.len(), 8);
    archive(&art.join("source-deltas.json.gz"), &delta);
    let mut helpers = BTreeMap::new();
    let mut full = BTreeMap::new();
    for e in fs::read_dir(R).unwrap() {
        let p = e.unwrap().path();
        let name = p.file_name().unwrap().to_string_lossy();
        if name.starts_with("codegen-") && name.ends_with(".stdout.log") {
            full.insert(name.to_string(),json!({"local_path":p,"bytes":fs::metadata(&p).unwrap().len(),"sha256":hash_file(&p),"storage":"local original full decode; selected bodies embedded"}));
            continue;
        }
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log"].iter().any(|s| e == *s))
        {
            helpers.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    assert_eq!(full.len(), 2);
    fs::write(
        art.join("full-disassembly-index.json"),
        serde_json::to_vec_pretty(&full).unwrap(),
    )
    .unwrap();
    for p in walk(&Path::new(R).join("build-runs")) {
        helpers.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
    }
    archive(&art.join("helpers-and-receipts.json.gz"), &helpers);
    fs::write(art.join("publication.json"),serde_json::to_vec_pretty(&json!({"passed":true,"new_commands":10,"source_files":179,"original_sources_exact":171,"original_sources_recovered":176,"source_delta_UTF8_members":delta.len(),"helper_receipt_UTF8_members":helpers.len(),"new_Arkworks_field_populations":2,"field_root_product_configurations":17168,"reapplied_core_inverse_population":1,"final_consumers_decoded_once":2,"actual_vector_root_canonical_compare_removed":true,"full_disassemblies_indexed_locally":full.len(),"archive_hashes_and_roundtrip_verified":true,"new_timing_observations":0,"new_proofs":0,"adopted":false,"production_changed":false,"goal_status":"active/unmet0/8"})).unwrap()).unwrap();
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash_file(&p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published{}files/{}UTF8receipts/8source deltas/2full-decodeindexes; archive/hash/identity PASS; nativecodecqualified/no timing",walk(Path::new(D)).len(),helpers.len());
}
