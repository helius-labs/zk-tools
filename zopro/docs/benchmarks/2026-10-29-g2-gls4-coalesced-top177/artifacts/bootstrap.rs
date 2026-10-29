use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177";
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    for f in [
        "lattice.rs",
        "historical-gls4.rs",
        "historical-integer-oracle.json",
        "code-audit.rs",
        "top.rs",
        "top-tests.rs",
    ] {
        fs::copy(Path::new(P).join(f), r.join(f)).unwrap();
    }
    let mut g = fs::read_to_string(Path::new(P).join("gls4.rs")).unwrap();
    once(
        &mut g,
        "    for count in counts {",
        "    for &count in &counts {",
    );
    once(
        &mut g,
        "let mut reduced = crate::msm::reduce_indexed_buckets(bases, &order, &starts);",
        "let mut reduced = crate::msm::group_affine(bases, &order, &starts, &counts);",
    );
    g = g.replace("GLS4_HYBRID entered", "GLS4_COALESCED entered");
    let tests = fs::read_to_string(r.join("coalesced-tests.rs")).unwrap();
    let marker = "    #[test]\n    fn checked_admission_and_fallback_preserve_words()";
    once(&mut g, marker, &format!("{tests}\n{marker}"));
    fs::write(r.join("gls4.rs"), g).unwrap();
    let mut setup = fs::read_to_string(Path::new(P).join("setup.rs")).unwrap();
    let a = "    add(add(vector::<[u64;5]>(n,false)?,held)?,mul(p.leaf.max(t.leaf),workers)?)";
    let b = r#"    // Four parts x eight signed 256-bucket groups have disjoint point ranges.
    // Keep the old conservative routing/output held bound, but charge the
    // collective point/pair/prefix scratch once across all 32 possible groups.
    let scratch=crate::msm::workspace_grouping::<G2Config>(n,32)?;
    let local_stack=mul(32,mul(512,size_of::<usize>())?)?;
    add(add(add(add(vector::<[u64;5]>(n,false)?,held)?,scratch)?,local_stack)?,mul(p.leaf,workers)?)"#;
    once(&mut setup, a, b);
    setup=setup.replace("\"msm_affine::g2_borrowed::gls4::tests::hybrid_msm_custom_rayon_pools_match_generator_sum\"", "\"msm_affine::g2_borrowed::gls4::tests::hybrid_msm_custom_rayon_pools_match_generator_sum\",\"msm_affine::g2_borrowed::gls4::tests::coalesced_top_skewed_complete_curve_groups\"");
    setup = setup
        .replace("OFF124/ON130", "OFF124/ON131")
        .replace("newhybridindexedtopGLS4", "newcoalescedtopGLS4");
    setup = setup.replace(
        "\"top-tests.rs\",\"setup.rs\"",
        "\"top-tests.rs\",\"coalesced-tests.rs\",\"setup.rs\"",
    );
    fs::write(r.join("setup.rs"), &setup).unwrap();
    let old = fs::read_to_string(Path::new(P).join("native.rs")).unwrap();
    let a = old.find("fn setup() {").unwrap();
    let b = old.find("fn freeze(label:").unwrap();
    let source = format!(
        "{}{}\n{}\n{}",
        &old[..a],
        setup,
        fs::read_to_string(r.join("code-audit.rs")).unwrap(),
        &old[b..]
    )
    .replace(P, R);
    fs::write(r.join("native.rs"), source).unwrap();
}
