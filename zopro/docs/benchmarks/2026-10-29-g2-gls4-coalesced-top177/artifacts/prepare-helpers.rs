use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("capture.rs").exists());
    let mut c=fs::read_to_string(Path::new(P).join("capture.rs")).unwrap().replace(P,R)
        .replace("/home/ubuntu/src/zopro-perf-work/g2-gls4-census177",P)
        .replace("17630","17708").replace("17552","17630").replace("17551","17552")
        .replace("(\"candidate\", 130)","(\"candidate\", 131)").replace("\"ON\":130","\"ON\":131")
        .replace("GLS4_HYBRID entered","GLS4_COALESCED entered")
        .replace("completed hybrid indexed-top GLS4 primary pilot;not adopted","completed coalesced-top GLS4 primary pilot;not adopted")
        .replace("G2 five ordinary11bit plus one indexed-top/fourpart GLS4;typed full admission/40Nscalar vector","G2 five ordinary11bit plus cross-bucket grouped-top/fourpart GLS4;typed collective scratch/40Nscalar vector");
    let marker="        \"msm_affine::g2_borrowed::gls4::tests::hybrid_msm_custom_rayon_pools_match_generator_sum\",";
    assert_eq!(c.matches(marker).count(), 1);
    c=c.replace(marker,&format!("{marker}\n        \"msm_affine::g2_borrowed::gls4::tests::coalesced_top_skewed_complete_curve_groups\","));
    c = c.replace(
        "        \"top-tests.rs\",",
        "        \"top-tests.rs\",\n        \"coalesced-tests.rs\",",
    );
    fs::write(r.join("capture.rs"), &c).unwrap();
    let a = c.find("fn prepare() {").unwrap();
    let b = c.find("\nfn push(").unwrap();
    fs::write(r.join("capture-setup.rs"), &c[a..b]).unwrap();
    let close = fs::read_to_string(Path::new(P).join("close.rs"))
        .unwrap()
        .replace(P, R)
        .replace(
            "2026-10-29-g2-gls4-indexed-top177",
            "2026-10-29-g2-gls4-coalesced-top177",
        )
        .replace("17630", "17708");
    fs::write(r.join("close.rs"), close).unwrap();
    let mut report = fs::read_to_string(Path::new(P).join("report.rs"))
        .unwrap()
        .replace(P, R)
        .replace(
            "2026-10-29-g2-gls4-indexed-top177",
            "2026-10-29-g2-gls4-coalesced-top177",
        )
        .replace("17630", "17708")
        .replace("17,630", "17,708")
        .replace("17,552", "17,630")
        .replace(
            "# Hybrid indexed-top GLS4: primary SDK pilot",
            "# Coalesced GLS4 top reduction: primary SDK pilot",
        );
    let a = report.find("    s.push_str(\"## Change and scope").unwrap();
    let b = report[a..].find("    let code =").unwrap() + a;
    report.replace_range(a..b,r####"    s.push_str("## Change and scope\n\nThe candidate changes only the GLS4 highest window from per-bucket tiled trees to the existing group_affine reduction. Each tree level batches ordinary pairs across 256 adjacent sign-class buckets, amortizing inversion work. Lower five eleven-bit Horner windows, four partitions and sequential components, 1,024 magnitude bins, 2,048 sign classes, phi/compound/phi maps and merges remain the parent algorithm. group_affine, complete-curve exceptions, generic/prefix arithmetic, Window and all field code remain exact. This changes pair grouping, point copies and the task graph, with no width or flag sweep.\n\nThe [tiled-top parent](../2026-10-29-g2-gls4-indexed-top177/README.md) remains rejected at 10.325850 ms versus 8.036072 ms accepted in its own run. Its profiles motivated testing inversion amortization, without proving an exclusive top cost. Earlier full ordinary-window tree/prefix/gather trials also remain rejected; they explicitly preserved narrow-top handling. This experiment does not repeat their ordinary-window routing.\n\nAll seven GLS4 tests pass: six parent tests rerun, including signed 65-bit extrema and full MSMs in custom 1/4/16/32-thread pools. One added skew/complete-curve test covers empty/odd inputs, many equal/opposite/identity points and distant sign bins under 1/4/32-thread pools. Fresh OFF/ON pass all 124/131 core tests and eight prover tests each. Exact 17 field sources inherit 33 field tests. Historical 100,767 lattice integer cases remain inherited. The official generator check passes.\n\nOptional admission conservatively retains the old six-output shape and four top parents' routing/output/sign/component arrays. It adds workers times the original nonforking leaf scratch, collective workspace_grouping(N,32), 32 local group stack arrays and the 40-byte-per-scalar vector. Across four parts and eight groups each, point ranges are disjoint, so retained per-init maximum point capacities sum to at most N. Existing typed grouping bounds cover pair capacity growth, generic/prefix records and allocator margins. Point limb copies are temporary, charged memory. Dynamic Reservation::analysis and scalar try_reserve_exact preserve original words and fall back on denial/failure. No retained key setup, lookup table, new runtime pool, field formula or synthetic global RAM cap.\n\nSource 177 retains 172 of 175 original files exact, three cfg-only parent edits and two new modules; inverse edits recover 175. Relative to the frozen tiled-top parent only gls4.rs and its workspace estimate change. Lattice, field, solver, FFT, key and original MSM/Window methods remain exact. [Design](artifacts/design.md), [history](artifacts/history-audit.md), [implementation](artifacts/source/crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs), [cost](artifacts/source/crates/zopro-core/src/msm_affine/workspace.rs).\n\n## Linked route and activation\n\nNew GLS4 symbols occur only in ON. The primary ON preflight requires four COALESCED entry trace markers before the pinned Gnark check and timing; those are the standard three primes and one preflight request. Timing/profiles have no trace. Selected disassemblies establish linked structure, not dynamic operation counts or speed.\n\n| Variant | Selected GLS4 functions | Selected bytes |\n|---|---:|---:|\n");
"####);
    fs::write(r.join("report.rs"), report).unwrap();
}
