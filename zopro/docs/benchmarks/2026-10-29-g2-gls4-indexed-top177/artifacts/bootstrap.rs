use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175";
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
    ] {
        fs::copy(Path::new(P).join(f), r.join(f)).unwrap();
    }
    let mut g = fs::read_to_string(Path::new(P).join("gls4.rs")).unwrap();
    let top = fs::read_to_string(r.join("top.rs")).unwrap();
    let marker="fn window(bases: &[G2Affine], short: &[[u64; 5]], w: usize, x_factor: &Fq2) -> Vec<G2Affine> {";
    once(
        &mut g,
        marker,
        &format!(
            "{top}\n{marker}\n    if w + 1 == WINDOWS {{return top_window(bases,short,x_factor);}}"
        ),
    );
    g = g.replace("GLS4_HORNER entered", "GLS4_HYBRID entered");
    let tests = fs::read_to_string(r.join("top-tests.rs")).unwrap();
    let marker = "    #[test]\n    fn checked_admission_and_fallback_preserve_words()";
    once(&mut g, marker, &format!("{tests}\n{marker}"));
    fs::write(r.join("gls4.rs"), g).unwrap();
    let mut setup = fs::read_to_string(Path::new(P).join("setup.rs")).unwrap();
    let a="    let p=shape::<G2Config>(1<<10,6,4,batch_size(11).max(32))?;\n    add(add(vector::<[u64;5]>(n,false)?,p.held)?,mul(p.leaf,workers)?)";
    let b = r#"    // Six-output shape conservatively includes returned top pieces and all merge scratch.
    let p=shape::<G2Config>(1<<10,6,4,batch_size(11).max(32))?;
    let t=top::<G2Config>(n.div_ceil(4),11)?;
    // Two sign classes, split-off negative vector, and retained earlier component.
    let extra=mul(vector::<Affine<G2Config>>(1<<10,false)?,3)?;
    let held=add(p.held,mul(add(t.held,extra)?,4)?)?;
    add(add(vector::<[u64;5]>(n,false)?,held)?,mul(p.leaf.max(t.leaf),workers)?)"#;
    once(&mut setup, a, b);
    setup=setup.replace("\"msm_affine::g2_borrowed::gls4::tests::checked_admission_and_fallback_preserve_words\"", "\"msm_affine::g2_borrowed::gls4::tests::checked_admission_and_fallback_preserve_words\",\"msm_affine::g2_borrowed::gls4::tests::indexed_top_signed_edges_match_generator_sum\",\"msm_affine::g2_borrowed::gls4::tests::hybrid_msm_custom_rayon_pools_match_generator_sum\"");
    setup = setup
        .replace("OFF124/ON128", "OFF124/ON130")
        .replace("newonebucketGLS4", "newhybridindexedtopGLS4");
    setup = setup.replace(
        "\"bootstrap.rs\",\"setup.rs\"",
        "\"bootstrap.rs\",\"top.rs\",\"top-tests.rs\",\"setup.rs\"",
    );
    fs::write(r.join("setup.rs"), &setup).unwrap();
    let old = fs::read_to_string(Path::new(P).join("native.rs")).unwrap();
    let a = old.find("fn setup() {").unwrap();
    let b = old.find("fn freeze(label:").unwrap();
    let native = format!(
        "{}{}\n{}\n{}",
        &old[..a],
        setup,
        fs::read_to_string(r.join("code-audit.rs")).unwrap(),
        &old[b..]
    )
    .replace(P, R);
    fs::write(r.join("native.rs"), native).unwrap();
}
