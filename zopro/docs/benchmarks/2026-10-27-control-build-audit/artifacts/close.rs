use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-control-build-audit";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-control-build-audit";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let ids = read(r.join("identities.json"));
    let code = read(r.join("code.json"));
    let sections = read(r.join("sections.json"));
    let manifest = read("/tmp/zopro-composed-adoption/production-source-manifest.json");
    for (p, h) in read(r.join("input-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    for (name, v) in ids["variants"].as_object().unwrap() {
        assert_eq!(
            hash(v["binary"].as_str().unwrap()),
            v["binary_sha256"].as_str().unwrap(),
            "{name}"
        );
    }
    assert_eq!(
        ids["variants"]["fresh"]["binary_sha256"],
        ids["variants"]["previous_fresh"]["binary_sha256"]
    );
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    assert_eq!(code["instructions"], 826692);
    assert_eq!(code["functions"], 3095);
    assert_eq!(code["shape_difference_count"], 0);
    assert_eq!(code["readonly_rodata_loads"], 5344);
    assert_eq!(code["different_rodata_load_bytes"], 0);
    assert!(fs::read_to_string(r.join("code-tests.log"))
        .unwrap()
        .contains("4 passed; 0 failed"));
    for (name, key) in [
        ("accepted", "accepted-local-only.asm"),
        ("fresh", "fresh-local-only.asm"),
    ] {
        assert_eq!(
            hash(r.join(key)),
            code["full_assembly_local_only_sha256"][name]
                .as_str()
                .unwrap()
        );
    }
    let close = json!({"status":"complete offline control-build audit","source_files_unchanged":171,"fresh_builds_exact_same_executable":true,"instructions_compared":826692,"text_functions_same_names_addresses_sizes":3095,"non_RIP_instruction_shape_differences":0,"RIP_references":22674,"changed_RIP_addresses":794,"readonly_literal_loads_compared":5344,"literal_payload_differences":0,"observer_tests":4,"performance_cause_established":false,"new_SDK_calls":0,"new_proofs":0,"new_native_builds":0,"next_prior_count":9904,"adopted":false,"goal_achieved":false,"original8_CPU10x_rows":0});
    fs::write(
        r.join("closeout.json"),
        serde_json::to_vec_pretty(&close).unwrap(),
    )
    .unwrap();
    let md = r#"# Source-identical SDK build audit

Two independent fresh builds produce the **same SDK executable**, SHA256 `6740f320700f954d1e9c2fa46883ea1aeaae5c7c551834c996833b8da3c3b430`. Compared with accepted `8241648f…`, all **826,692 decoded text instructions** match after normalizing only RIP displacements. All **5,344 identified readonly literal loads** read equal file-image payloads. The audit finds no changed arithmetic operation, register, stack offset, immediate or branch destination in this comparison.

This narrows the build discrepancy; it **does not explain the latency difference** or resolve the five failed control-calibration bounds in the preceding study. Production, the accepted SDK and all earlier measurements/verdicts remain unchanged. No new proof, native build, timing or profile capture is run.

## Inputs and comparison

All 171 production source hashes match the accepted manifest. Both fresh snapshots match those hashes. Standard driver/support bytes, recorded direct-rustc optimization flags and build runtime environment agree. Cargo core fingerprints agree on compiler, features, profile and Rust flags; build-script fingerprint values differ, but their retained directive outputs are identical. The direct link uses the same optimization defaults when flags are omitted from the plan. Command receipts, rather than omissions in plan JSON, establish this.

The selected libc, memmap2 and Rayon archive bytes match accepted. Selected serde_json, BN254, core, prover and SDK archives differ. Intermediate archive differences therefore cannot alone establish different executable work: the two fresh builds have differing archives but identical final executable hashes. This audit checks the recorded direct libraries and core fingerprint; it is not a reconstruction of every historical toolchain input.

Accepted and fresh ELF files are both 8,089,056 bytes with identical section layout. All 3,095 named text-function entries have the same names, virtual addresses and sizes. Static comparison covers the entire `.text`, including padding, and preserves instruction addresses, opcodes, registers, stack offsets, immediates and branch targets. Only RIP displacement values and disassembler comments are normalized.

|Section|Bytes|Different file bytes|
|---|---:|---:|
|.text|4,070,726|822|
|.rodata|527,344|21,341|
|.rela.dyn|39,648|1,199|
|.data.rel.ro|34,720|4|
|.symtab|126,768|15|
|.note.gnu.build-id|36|20|

There are 22,674 decoded RIP references; 794 refer to different virtual addresses. For 5,344 explicit-width reads into `.rodata`, the referenced file bytes agree at the decoded widths. Address-only LEAs, indirect calls, stores and mutable/relocated pointer tables require different reasoning and are not called literal-load equivalence. The full binary is not claimed byte-identical or proven semantically equivalent. Runtime ASLR, dynamic relocation state, cache behavior, scheduling and frequency were not measured by this audit.

## Limits and next action

The previous prefix-cutoff result remains: primary paired saving **3.998 µs (0.048%)** versus accepted, interval including parity; candidate/fresh gain about 1.12%, with failed control calibration. No small gain is reclassified as zero, and no failed bound is waived. The candidate remains unadopted.

Four observer tests pass, including distinct file/virtual offsets, preservation of non-RIP operands and exclusion of overlapping nonallocated debug sections from address lookup. Original failed fixtures and superseded observer outputs are retained with repair notes. No executable, source, latency capture or profile was changed or repeated to repair the observer. Complete disassemblies remain local with checked hashes; JSON summaries, reference metadata and observer sources are archived.

The user requested a fresh AWS comparison and confirmed login. Next, compare the exact adopted source with pinned Aeglos on one owned c7a.24xlarge, including all nine available circuit shapes and the 54→1 merge case. Declare build, timing, independent verification and cleanup before provisioning; report current CPU/Metal references without inferring AWS gains from local results. The old C7a primary measurements (12.327 ms Zopro, 8.121 ms Aeglos) belong to an earlier Zopro source. This audit adds no AWS result.

The global independent-proof ledger stays **9,904**. The original all-eight supplied M5 CPU 10× objective remains active, achieved 0/8. Historical CPU/Metal values and all shape results are in the [preceding SDK report](../2026-10-27-grouping-prefix-cutoff/README.md); no speed or memory qualification is newly claimed here.

[Design](artifacts/design.md), [identities and commands](artifacts/identities.json), [section comparison](artifacts/sections.json), [instruction comparison](artifacts/code.json), [RIP references](artifacts/RIP-references.json), [observer repairs](artifacts/observer-repairs.json), [closeout](artifacts/closeout.json), [artifact hashes](artifact-index.json).
"#;
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    for e in fs::read_dir(r).unwrap() {
        let p = e.unwrap().path();
        if !p.is_file() {
            continue;
        }
        if !["rs", "md", "json", "log"]
            .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
        {
            continue;
        }
        let name = p.file_name().unwrap();
        let to = d.join("artifacts").join(name);
        fs::copy(&p, &to).unwrap();
        assert_eq!(hash(&p), hash(&to));
        index.insert(name.to_string_lossy().to_string(), hash(&to));
    }
    fs::write(
        d.join("artifact-index.json"),
        serde_json::to_vec_pretty(&index).unwrap(),
    )
    .unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    println!(
        "Closed offline audit: {} artifact hashes;0newSDK/0newproof;ledger9904;productionunchanged",
        index.len()
    );
    let _ = sections;
}
