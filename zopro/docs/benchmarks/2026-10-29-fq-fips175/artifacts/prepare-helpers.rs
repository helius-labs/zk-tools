use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/fq-fips175";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g2-fused-outline175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("capture.rs").exists());
    let mut c = fs::read_to_string(format!("{OLD}/capture.rs"))
        .unwrap()
        .replace(OLD, R);
    c = c.replace(
        "/home/ubuntu/src/zopro-perf-work/balanced-ordinary175",
        "/home/ubuntu/src/zopro-perf-work/fft-eight-columns175",
    );
    c = c
        .replace("17005", "17239")
        .replace("17083", "17317")
        .replace("17161", "17395");
    c = c.replace(
        "code[\"caller_route_verified\"]",
        "code[\"actual_G1_G2_IFMA_callers_changed\"]",
    );
    c = c.replace("g2_fused_outline_cfg", "fq_fips_cfg");
    c = c.replace(
        "completed selective fusedFq outlining primary pilot;not adopted",
        "completed Fq diagonal FIPS primary pilot;not adopted",
    );
    c=c.replace("\"new_field_test_cases\":0","\"new_field_test_cases\":\"Fq8/R256 ordinary and fused plus16-product chains per existing fixture\"");
    c = c.replace(
        "native8FqR256fusedCIOSoutlinedonly;exactarithmeticbody",
        "native8FqR256ordinaryandfuseddiagonalFIPS;newdataflow/sameintegerresult",
    );
    c=c.replace("        \"design.md\",","        \"design.md\",\n        \"algebra.md\",\n        \"model.rs\",\n        \"model.json\",\n        \"model.stderr.log\",\n        \"model-build.stderr.log\",\n        \"model-build.stdout.log\",\n        \"build-model.sh\",\n        \"bootstrap.rs\",\n        \"prepare-helpers.rs\",\n        \"kernel.txt\",");
    let marker = "    let code = read(r(\"code-audit.json\"));";
    let deps = r#"    for path in [
        "/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/target/release/deps/libnum_bigint-6bc1a1b0ca7a2b87.rlib",
        "/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/target/release/deps/libnum_traits-62d94157db5bd7b9.rlib",
        "/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/target/release/deps/libserde_json-23322b031c2a5860.rlib",
        "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/frozen/candidate/libzopro_bn254-5778e0f87c7d66ba5e17a5f97bf5992c699ca4d2f3df952e07aca8ff03a40370.rlib"
    ] { pin(&mut pins,path); }
    assert_eq!(read(r("model.json"))["passed"],true);
    assert_eq!(read(r("model.json"))["cases"],38176);
"#;
    assert_eq!(c.matches(marker).count(), 1);
    c = c.replace(marker, &format!("{deps}{marker}"));
    fs::write(r.join("capture.rs"), &c).unwrap();
    let a = c.find("fn prepare() {").unwrap();
    let b = c.find("\nfn push(").unwrap();
    fs::write(r.join("capture-setup.rs"), &c[a..b]).unwrap();
    let close = fs::read_to_string(format!("{OLD}/close.rs"))
        .unwrap()
        .replace(OLD, R)
        .replace("2026-10-29-g2-fused-outline175", "2026-10-29-fq-fips175")
        .replace("17161", "17395");
    fs::write(r.join("close.rs"), close).unwrap();
    let mut report = fs::read_to_string(format!("{OLD}/report.rs"))
        .unwrap()
        .replace(OLD, R)
        .replace("2026-10-29-g2-fused-outline175", "2026-10-29-fq-fips175");
    report = report
        .replace("17161", "17395")
        .replace("17,083", "17,317")
        .replace("17,161", "17,395");
    report = report.replace(
        "# Selective fused-Fq outlining: primary SDK pilot",
        "# Diagonal Fq Montgomery multiplication: primary SDK pilot",
    );
    let a = report.find("    s.push_str(\"## Change and scope").unwrap();
    let b = report
        .find("    s.push_str(\"\\nThe candidate caller plus helper")
        .unwrap();
    report.replace_range(a..b,r####"    s.push_str("## Change and scope\n\nEight-lane Fq/R256 ordinary and fused products use anti-diagonal product scanning interleaved with Montgomery cancellation. Fr, R260, four-lane, corrections, field representation, core consumers and resource policies retain their accepted paths. Only the IFMA template and its officially generated eight-lane file differ among175 originals;173 are exact and the reverse check restores all175. The original CIOS body is unchanged.\n\nThere are no new heap allocations, tables or copied keys/points. The kernel uses fixed vector locals, with the existing inline policy. Nominal IFMA counts remain100 ordinary/150 fused; new dependency order and register pressure can lose. No speed claim follows from operation counts.\n\nAn independently derived mixed-radix interpreter passes38176 exact BigUint comparisons and32 cancellation identities, including both primes/radices as mathematical controls. Native dispatch remains only Fq/R256/eight lanes. Both native builds freshly pass33 field,124 core and8 prover tests. The candidate adds Arkworks ordinary/fused comparisons and16-product chains to the shared carry test; the generator check passes. This qualification is native only.\n\n[ZKPU](https://arxiv.org/html/2610.27530v1), sections2/3.4, suggested reconsidering FIPS dataflow. Its results describe hardware. The mixed-radix CPU implementation and tests here are independently derived; no implementation was copied and no hardware speedup is transferred to CPUs. See [algebra and bounds](artifacts/algebra.md) and [model receipt](artifacts/model.json).\n\n## Linked code before timing\n\nSource, cfg fingerprints, native test qualification and selected actual G1/G2 IFMA caller disassembly are recorded. Both caller implementations change in the linked SDK. This is a structural observation; no isolated arithmetic cost, dynamic instruction count or stack bound is inferred.\n\n| Variant | Selected G1 bytes | Selected G2 bytes | Static IFMA sites |\n|---|---:|---:|---:|\n");
    let code=read(r.join("code-audit.json"));
    for v in code["variants"].as_array().unwrap() {
        let selected=v["selected_functions"].as_array().unwrap();
        let bytes=|key:&str|selected.iter().filter(|x|x[key]==true).map(|x|x["bytes"].as_u64().unwrap()).sum::<u64>();
        let sites=selected.iter().map(|x|x["ifma_sites"].as_u64().unwrap()).sum::<u64>();
        writeln!(s,"| {} | {} | {} | {sites} |",v["variant"].as_str().unwrap(),bytes("G1"),bytes("G2")).unwrap();
    }
"####);
    let a = report
        .find("    s.push_str(\"\\nThe candidate caller plus helper")
        .unwrap();
    let b = report[a..].find("## Complete verified requests").unwrap() + a;
    report.replace_range(a..b, "    s.push_str(\"\\n");
    let a = report
        .find("    s.push_str(\"\\n| Profile variant")
        .unwrap();
    let b = report[a..].find("## CPU and Metal scope").unwrap() + a;
    report.replace_range(a..b,r####"    s.push_str("\n| Profile variant | Samples | Lost records |\n|---|---:|---:|\n");
    for (k,v) in a["diagnostics"]["profiles"].as_object().unwrap() {
        writeln!(s,"| {k} | {} | {} |",v["samples"],v["lost"]["lost_records"].as_array().unwrap().len()).unwrap();
    }
    s.push_str("\nAll stat events run at least99.9%; each profile contains one SDK PID and zero lost records. Symbol aggregates include surrounding work and inlining; cycle-period shares are not removable wall time or an isolated cost. No exclusive memory/cache/scheduler cause is established.\n\n"####);
    fs::write(r.join("report.rs"), report).unwrap();
}
