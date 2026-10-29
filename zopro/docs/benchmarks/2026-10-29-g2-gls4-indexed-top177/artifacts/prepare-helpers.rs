use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("capture.rs").exists());
    let mut c=fs::read_to_string(Path::new(P).join("capture.rs")).unwrap().replace(P,R)
        .replace("/home/ubuntu/src/zopro-perf-work/fr-dot3-175","/home/ubuntu/src/zopro-perf-work/g2-gls4-census177")
        .replace("17551","17630").replace("17473","17552").replace("17395","17551")
        .replace("(\"candidate\", 128)","(\"candidate\", 130)").replace("\"ON\":128","\"ON\":130")
        .replace("GLS4_HORNER entered","GLS4_HYBRID entered")
        .replace("completed one-bucket four-component G2 Horner primary pilot;not adopted","completed hybrid indexed-top GLS4 primary pilot;not adopted")
        .replace("G2 six11bit/fourpart single-state GLS4 Horner;typed admission/40Nscalar vector","G2 five ordinary11bit plus one indexed-top/fourpart GLS4;typed full admission/40Nscalar vector");
    let marker="        \"msm_affine::g2_borrowed::gls4::tests::checked_admission_and_fallback_preserve_words\",";
    assert_eq!(c.matches(marker).count(), 1);
    c=c.replace(marker,&format!("{marker}\n        \"msm_affine::g2_borrowed::gls4::tests::indexed_top_signed_edges_match_generator_sum\",\n        \"msm_affine::g2_borrowed::gls4::tests::hybrid_msm_custom_rayon_pools_match_generator_sum\","));
    c = c.replace(
        "        \"bootstrap.rs\",",
        "        \"bootstrap.rs\",\n        \"top.rs\",\n        \"top-tests.rs\",",
    );
    fs::write(r.join("capture.rs"), &c).unwrap();
    let a = c.find("fn prepare() {").unwrap();
    let b = c.find("\nfn push(").unwrap();
    fs::write(r.join("capture-setup.rs"), &c[a..b]).unwrap();
    let close = fs::read_to_string(Path::new(P).join("close.rs"))
        .unwrap()
        .replace(P, R)
        .replace(
            "2026-10-29-g2-gls4-horner175",
            "2026-10-29-g2-gls4-indexed-top177",
        )
        .replace("17551", "17630");
    fs::write(r.join("close.rs"), close).unwrap();
    let mut report = fs::read_to_string(Path::new(P).join("report.rs"))
        .unwrap()
        .replace(P, R)
        .replace(
            "2026-10-29-g2-gls4-horner175",
            "2026-10-29-g2-gls4-indexed-top177",
        )
        .replace("17551", "17630")
        .replace("17,551", "17,630")
        .replace("17,473", "17,552")
        .replace(
            "# Single-state four-component G2 Horner: primary SDK pilot",
            "# Hybrid indexed-top GLS4: primary SDK pilot",
        );
    let a = report.find("    s.push_str(\"## Change and scope").unwrap();
    let b = report[a..].find("    let code =").unwrap() + a;
    report.replace_range(a..b,r####"    s.push_str("## Change and scope\n\nThe candidate keeps the parent GLS4 lower five eleven-bit Horner windows. Its highest window instead reduces borrowed points using the existing tiled indexed affine tree, separately for positive/negative digit classes and sequentially for four components. Affine bucket outputs follow the same S3,phi,S2,phi²psi,S1,phi,S0 composition, now with three component merges in the top window. Four point partitions and1,024 magnitude bins remain fixed. The full65-bit magnitude plus Booth carry bound is used, not the observed544 maximum. The old six-ordinary-window candidate remains rejected.\n\nThe preceding [diagnostic](../2026-10-29-g2-gls4-census177/README.md) counted38,842/45,725 (84.947%) existing projective feed updates in the highest window. This motivated the new reduction algorithm; it did not measure its speed or establish cycle cost. The top path removes that window's projective queue, but index construction, four tree passes and extra merges may outweigh it. Accepted phi2 already uses an indexed narrow top.\n\nSix candidate GLS4 tests pass: the parent's decomposition/digit, map, full-MSM and admission tests rerun; two additional tests cover signed top digits including65-bit extrema and generator sums, and full MSMs on custom Rayon1/4/16/32-thread pools. OFF/ON pass all124/130 core tests and eight prover tests each. All17 field sources are exact;33 native field tests and100,767 historical lattice integer cases are inherited. The official generator check passes.\n\nThe40-byte-per-scalar temporary and entire candidate workspace are charged via dynamic Reservation::analysis in addition to existing forecasts. The conservative six-window output shape, four simultaneous indexed-top parent buffers and retained sign/component arrays, plus workers times the maximum applicable leaf scratch are included. Checked overflow/admission failure or scalar try_reserve_exact failure preserves original words and falls back to accepted phi2. No transformed key copy, point lookup table, retained setup, new runtime pool, field arithmetic or synthetic global cap. Temporary indices/scalars are additional memory; a lower RSS sample would not establish minimum RAM.\n\nSource177 retains172 of175 original files exact, three cfg-only parent edits and two new modules; inverse edits recover175. Relative to the frozen GLS4 parent, only gls4.rs and its workspace estimate change. Original Window methods, lattice, field, solver, FFT and key sources remain exact. [Design](artifacts/design.md), [history](artifacts/history-audit.md), [implementation](artifacts/source/crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs), [cost](artifacts/source/crates/zopro-core/src/msm_affine/workspace.rs).\n\n## Linked route and activation\n\nNew GLS4 symbols occur only in ON. The primary ON preflight must produce four hybrid-entry trace markers before the pinned Gnark verification and timing. These are the normal three primes and one preflight request. Timing and profiling have no trace. Selected disassemblies establish linked route structure, not speed or stack bounds.\n\n| Variant | Selected GLS4 functions | Selected bytes |\n|---|---:|---:|\n");
"####);
    fs::write(r.join("report.rs"), report).unwrap();
}
