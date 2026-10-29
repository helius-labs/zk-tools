use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g2-norm-heap175";
const REPORT_OLD: &str = "/home/ubuntu/src/zopro-perf-work/fr-dot3-175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("capture.rs").exists());
    let mut c = fs::read_to_string(format!("{OLD}/capture.rs"))
        .unwrap()
        .replace(OLD, R)
        .replace(
            "/home/ubuntu/src/zopro-perf-work/g2-fused-outline175",
            REPORT_OLD,
        );
    c = c
        .replace("17239", "17551")
        .replace("17161", "17473")
        .replace("17083", "17395");
    c = c
        .replace(
            "audit[\"source_files\"], 176",
            "audit[\"source_files\"], 177",
        )
        .replace(
            "audit[\"originals_exact\"], 174",
            "audit[\"originals_exact\"], 172",
        )
        .replace(
            "accepted_balanced13_byte_exact",
            "solver_key_field_fft_exact",
        );
    let a = c.find("    let required=").unwrap();
    let b = c[a..].find("    for (label, count)").unwrap() + a;
    c.replace_range(
        a..b,
        r#"    let required=[
        "msm_affine::g2_borrowed::gls4::tests::decomposition_and_digits_reconstruct_fullwidth",
        "msm_affine::g2_borrowed::gls4::tests::affine_and_projective_horner_maps_match_scalars",
        "msm_affine::g2_borrowed::gls4::tests::full_msm_matches_independent_generator_sums",
        "msm_affine::g2_borrowed::gls4::tests::checked_admission_and_fallback_preserve_words"
    ];
"#,
    );
    c = c
        .replace("(\"candidate\", 126)", "(\"candidate\", 128)")
        .replace("\"ON\":126", "\"ON\":128")
        .replace("caller_route_verified", "linked_GLS4_present_only_ON")
        .replace("g2_norm_heap_cfg", "gls4_horner_cfg");
    c = c
        .replace(
            "completed G2 norm-product heap primary pilot;not adopted",
            "completed one-bucket four-component G2 Horner primary pilot;not adopted",
        )
        .replace(
            "G2 full8lane norm-product heap in existing2N-vector storage",
            "G2 six11bit/fourpart single-state GLS4 Horner;typed admission/40Nscalar vector",
        );
    c=c.replace("        \"norm_heap.rs\",","        \"gls4.rs\",\n        \"lattice.rs\",\n        \"historical-gls4.rs\",\n        \"historical-integer-oracle.json\",\n        \"bootstrap.rs\",\n        \"prepare-helpers.rs\",");
    let marker = "        let dir = Path::new(j[\"directory\"].as_str().unwrap());";
    let branch = r#"        if j["preflight"]==true && j["variant"]=="ON" && c["name"]=="1x2-w32" {
            return json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[r("capture-helper"),"verify-traced",r("runs/pre-1x2-w32-ON/stderr.log"),VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":120});
        }
"#;
    assert_eq!(c.matches(marker).count(), 1);
    c = c.replace(marker, &format!("{marker}\n{branch}"));
    let marker = "            let j = push(c, v, 0, true, &base, &ids, &mut jobs, &mut steps);";
    let branch="\n            if c[\"name\"]==\"1x2-w32\" && v==\"ON\" {steps.last_mut().unwrap()[\"environment\"][\"ZOPRO_GLS4_HORNER_TRACE\"]=json!(\"1\");}";
    assert_eq!(c.matches(marker).count(), 1);
    c = c.replace(marker, &format!("{marker}{branch}"));
    let marker = "        Some(\"prepare\") => prepare(),";
    let branch = r#"        Some("verify-traced") => {
            let a: Vec<_> = std::env::args().collect();
            let log=fs::read_to_string(&a[2]).unwrap();
            assert_eq!(log.lines().filter(|line|line.contains("GLS4_HORNER entered bits=11 parts=4")).count(),4,"new route not active in all four primary preflight requests");
            assert!(Command::new(&a[3]).args(&a[4..]).status().unwrap().success());
        },
"#;
    assert_eq!(c.matches(marker).count(), 1);
    c = c.replace(marker, &format!("{branch}{marker}"));
    fs::write(r.join("capture.rs"), &c).unwrap();
    let a = c.find("fn prepare() {").unwrap();
    let b = c.find("\nfn push(").unwrap();
    fs::write(r.join("capture-setup.rs"), &c[a..b]).unwrap();
    let close = fs::read_to_string(format!("{REPORT_OLD}/close.rs"))
        .unwrap()
        .replace(REPORT_OLD, R)
        .replace("2026-10-29-fr-dot3-175", "2026-10-29-g2-gls4-horner175")
        .replace("17473", "17551")
        .replace(
            "assert_eq!(source.len(), 176)",
            "assert_eq!(source.len(), 177)",
        );
    fs::write(r.join("close.rs"), close).unwrap();
    let mut report = fs::read_to_string(format!("{REPORT_OLD}/report.rs"))
        .unwrap()
        .replace(REPORT_OLD, R)
        .replace("2026-10-29-fr-dot3-175", "2026-10-29-g2-gls4-horner175")
        .replace("17473", "17551")
        .replace("17,473", "17,551")
        .replace("17,395", "17,473");
    report = report.replace(
        "# Pure Rust three-product Poseidon dot: primary SDK pilot",
        "# Single-state four-component G2 Horner: primary SDK pilot",
    );
    let a = report.find("    s.push_str(\"## Change and scope").unwrap();
    let b = report[a..]
        .find("    s.push_str(\"\\n## Complete verified requests")
        .unwrap()
        + a;
    report.replace_range(a..b,r####"    s.push_str("## Change and scope\n\nThe candidate reuses one accepted borrowed Window through four scalar-component passes: S3, phi, S2, phi²psi, S1, phi, S0. Since phi³=1 and the maps commute, the result is S0+phi*S1+psi*S2+phi*psi*S3. Both affine and deferred projective state are transformed after pending queues drain. The old four-component study used separate component fills and three component merges per partition/window; it remains rejected. This implementation removes those merges and separate component bucket states.\n\nOne prospective geometry uses six 11-bit windows and four point partitions, with 24 fill jobs and 1,024 buckets per state. Accepted phi2 uses eleven 12-bit windows with two partitions, and a narrower indexed top. The candidate has no narrow top window. Four input traversals, scalar decomposition, collision costs and scheduling can offset fewer bucket reductions; counts do not establish CPU gains.\n\nThe old local GLS4 lattice/quotient/digit bodies are reused with source and arithmetic-oracle provenance. Historical 100,767 integer cases are inherited, not newly executed. Four fresh candidate tests recheck full-width decomposition/digits, affine/projective map composition, complete MSM generator sums including identity/opposite/zero/one/full-width cases and tails, and denied admission/checked-cost/input preservation. Native OFF/ON pass all 124/128 core tests and eight prover tests each. The 17 field files are exact, so 33 native field tests are inherited. The unchanged official generator check passes.\n\nThe 40-byte-per-scalar temporary and the entire new typed workspace geometry are conservatively charged through resource-aware analysis admission, in addition to existing future-proof forecasts. Admission/allocation failure preserves input words and uses the original phi2 path. No transformed/copied key, point lookup table, duplicate plan, new field arithmetic, pool, synthetic universal memory cap or retained per-key state is added. The temporary scalar vector is additional memory; fewer bucket sets alone do not prove a process-RAM saving. Unsupported sizes/targets retain the original implementation.\n\nSource has 177 files: 172 of 175 originals exact, three cfg-only parent edits and two new modules. Inverse edits recover all 175. Original Window methods, solver, FFT, key and field sources remain exact. [Design](artifacts/design.md), [history](artifacts/history-audit.md), [historical lattice oracle](artifacts/historical-integer-oracle.json), [new implementation](artifacts/source/crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs).\n\n## Linked route and activation\n\nThe new GLS4 namespace is present only in ON. The primary ON preflight enables a diagnostic trace; its verifier wrapper requires four entry markers before executing the pinned Gnark verifier. These are the same three primes plus one preflight request, not additional exports. Timing and profiler jobs have no trace. Selected linked disassemblies are archived; code size is not dynamic work or a memory bound.\n\n| Variant | Selected GLS4 namespace functions | Selected bytes |\n|---|---:|---:|\n");
    let code=read(r.join("code-audit.json"));
    for v in code["variants"].as_array().unwrap(){
        let functions:Vec<_>=v["selected_functions"].as_array().unwrap().iter().filter(|x|x["new_GLS4"]==true).collect();
        let bytes=functions.iter().map(|x|x["bytes"].as_u64().unwrap()).sum::<u64>();
        writeln!(s,"| {} | {} | {bytes} |",v["variant"].as_str().unwrap(),functions.len()).unwrap();
    }
"####);
    report = report
        .replace("helper/driver, source176", "helper/driver, source177")
        .replace("helper/driver, source 176", "helper/driver, source 177");
    report = report.replace(
        "        (\"source176\", \"source 176\"),",
        "        (\"source176\", \"source 176\"),\n        (\"source177\", \"source 177\"),",
    );
    fs::write(r.join("report.rs"), report).unwrap();
}
