use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/fr-dot3-175";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/fq-fips175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("capture.rs").exists());
    let mut c = fs::read_to_string(format!("{OLD}/capture.rs"))
        .unwrap()
        .replace(OLD, R);
    c = c.replace("/home/ubuntu/src/zopro-perf-work/fft-eight-columns175", OLD);
    c = c
        .replace("17395", "17473")
        .replace("17317", "17395")
        .replace("17239", "17317");
    c = c
        .replace(
            "audit[\"source_files\"], 175",
            "audit[\"source_files\"], 176",
        )
        .replace(
            "audit[\"originals_exact\"], 173",
            "audit[\"originals_exact\"], 172",
        );
    c = c
        .replace(
            "reverse175_by_unedit_and_official_generator",
            "reverse175_by_inverse_edits",
        )
        .replace(
            "core_solver_key_scheduler_forecast_exact",
            "key_scheduler_forecast_exact",
        );
    c = c.replace(
        "field[\"passed_tests\"].as_array().unwrap().len(), 33",
        "field[\"passed_tests\"].as_array().unwrap().len(), if label==\"candidate\" {35}else{33}",
    );
    let start = c.find("    for path in [").unwrap();
    let end = c[start..].find("    let code = read(").unwrap() + start;
    c.replace_range(start..end, "");
    c = c.replace(
        "actual_G1_G2_IFMA_callers_changed",
        "solver_dense_changed_and_called",
    );
    let start = c.find("    fn pin_tree(").unwrap();
    let end = c[start..].find("    let mut ids =").unwrap() + start;
    c.replace_range(start..end, "");
    c = c.replace("fq_fips_cfg", "fr_dot3_cfg");
    c = c.replace(
        "completed Fq diagonal FIPS primary pilot;not adopted",
        "completed pureRust three-product Fr dense Poseidon pilot;not adopted",
    );
    c = c.replace(
        "\"native_field_tests_each_new_execution\":33",
        "\"native_field_tests_new_execution\":{\"OFF\":33,\"ON\":35}",
    );
    c=c.replace("Fq8/R256 ordinary and fused plus16-product chains per existing fixture","two new tests: smallradix28561 exact cases and bothprime canonical results plus16-product chains");
    c = c.replace(
        "native8FqR256ordinaryandfuseddiagonalFIPS;newdataflow/sameintegerresult",
        "native scalarFr width3dense only sum3/oneREDC;partialrounds exact",
    );
    // The later finish assertion is a multiline count, not the early field one.
    c = c.replace(
        "            33\n        );",
        "            if label==\"candidate\" {35}else{33}\n        );",
    );
    for name in [
        "algebra.md",
        "model.rs",
        "model.json",
        "model.stderr.log",
        "model-build.stderr.log",
        "model-build.stdout.log",
        "build-model.sh",
        "kernel.txt",
    ] {
        c = c.replace(&format!("        \"{name}\",\n"), "");
    }
    c = c.replace(
        "        \"design.md\",",
        "        \"design.md\",\n        \"bounds.md\",\n        \"sop3.rs\",",
    );
    fs::write(r.join("capture.rs"), &c).unwrap();
    let start = c.find("fn prepare() {").unwrap();
    let end = c.find("\nfn push(").unwrap();
    fs::write(r.join("capture-setup.rs"), &c[start..end]).unwrap();
    let close = fs::read_to_string(format!("{OLD}/close.rs"))
        .unwrap()
        .replace(OLD, R)
        .replace("2026-10-29-fq-fips175", "2026-10-29-fr-dot3-175")
        .replace("17395", "17473")
        .replace(
            "assert_eq!(source.len(), 175)",
            "assert_eq!(source.len(), 176)",
        );
    fs::write(r.join("close.rs"), close).unwrap();
    let mut report = fs::read_to_string(format!("{OLD}/report.rs"))
        .unwrap()
        .replace(OLD, R)
        .replace("2026-10-29-fq-fips175", "2026-10-29-fr-dot3-175")
        .replace("17395", "17473")
        .replace("17,317", "17,395")
        .replace("17,395**,", "17,473**,");
    // Existing report contains the ledger literal in bold, independently.
    report = report.replace("**17,395**", "**17,473**");
    report = report.replace(
        "# Diagonal Fq Montgomery multiplication: primary SDK pilot",
        "# Pure Rust three-product Poseidon dot: primary SDK pilot",
    );
    let start = report.find("    s.push_str(\"## Change and scope").unwrap();
    let end = report[start..]
        .find("    s.push_str(\"\\n## Complete verified requests")
        .unwrap()
        + start;
    report.replace_range(start..end,r####"    s.push_str("## Change and scope\n\nOnly width-three dense Poseidon rows select a new scalar Fr helper that computes three products with one Montgomery reduction. The accepted row used sum2 plus an ordinary product, with two reductions. Partial rounds, S-boxes, constants, recognition, all witness/constraint writes and scheduling are unchanged. Four CIOS rows use safe Rust u128 multiply-adds and five u64 accumulators, followed by the existing canonical correction.\n\nThe template has 48 full raw products, 16 modulus products and four low quotient products, versus 48/32/eight for the original sum2+mul expression. These source arithmetic counts are not emitted instructions or speed predictions. No assembly, unsafe block, heap allocation, table, copied key/plan or resource-policy change is added.\n\nThree original parent files differ and one private field module is added: 176 total source files, 172 original files exact, and all 175 originals recovered by reversing the edits. IFMA kernels and representation stay unchanged. Native control passes all 33 field tests; candidate passes all 35, including two new independent tests. Both pass 124 core and eight prover tests. Small-radix coverage checks 28,561 exact integer cancellation cases. Actual-word tests compare both primes with Arkworks and accepted arithmetic, covering modulus/limb boundaries, 4,096 random six-tuples per prime and 16-step dependent chains, with canonical output and carry bounds checked. These are native checks, not a portable or giant-circuit qualification.\n\n[Bounds](artifacts/bounds.md), [new field kernel/tests](artifacts/source/crates/zopro-bn254/src/sop3.rs), [history audit](artifacts/history-audit.md).\n\n## Linked solver code\n\nThe actual SDK dense function changes and the original task caller still calls it. Source/cfg/fingerprint qualification and assembly are retained. Selected code sizes do not establish instruction-cache pressure or performance.\n\n| Variant | Dense bytes | Task caller bytes | Dense call sites |\n|---|---:|---:|---:|\n");
    let code=read(r.join("code-audit.json"));
    for v in code["variants"].as_array().unwrap() {
        let functions=v["selected_functions"].as_array().unwrap();
        let bytes=|key:&str|functions.iter().find(|x|x[key]==true).unwrap()["bytes"].as_u64().unwrap();
        let calls=functions.iter().map(|x|x["dense_call_sites"].as_u64().unwrap()).sum::<u64>();
        writeln!(s,"| {} | {} | {} | {calls} |",v["variant"].as_str().unwrap(),bytes("dense"),bytes("caller")).unwrap();
    }
"####);
    report = report.replace("helper/driver, source175", "helper/driver, source176");
    report = report.replace("helper/driver, source 175", "helper/driver, source 176");
    fs::write(r.join("report.rs"), report).unwrap();
}
