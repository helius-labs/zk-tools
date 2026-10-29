use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/fq-fips175";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g2-fused-outline175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    let old = fs::read_to_string(format!("{OLD}/native.rs")).unwrap();
    let start = old.find("    let new=format!").unwrap();
    let end = old[start..]
        .find("    let mut text = original.clone();")
        .unwrap()
        + start;
    let mut source = old[..start].to_string();
    source.push_str(r##"    let new=format!("{old} #[cfg(zopro_fq_fips)] if LANES==8 && MIXED && C::P==crate::FqConfig::P {{return self.product_fips::<FUSED>(b,c,d);}}");
    let helper=fs::read_to_string(r("kernel.txt")).unwrap();
    let marker = "#[cfg(test)]\nmod tests {";
"##);
    source.push_str(&old[end..]);
    source = source.replace(
        "once(&mut back, helper, \"\");",
        "once(&mut back, &helper, \"\");",
    );
    let before = "    fs::write(&p, &text).unwrap();";
    let extra = r##"    let test_marker="                verify(v[0].difference2(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]-c[i]*d[i]));";
    let extra=r#"                #[cfg(zopro_fq_fips)]
                if LANES==8 && MIXED && C::P==crate::FqConfig::P {
                    verify(v[0].product_fips::<false>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]));
                    verify(v[0].product_fips::<true>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]+c[i]*d[i]));
                    let mut fips_chain=v[0];let mut fips_want=a;
                    for step in 0..16 {
                        if step%2==0 {
                            fips_chain=fips_chain.product_fips::<false>(&v[1],&v[2],&v[3]);
                            for i in 0..LANES {fips_want[i]*=b[i];}
                        } else {
                            fips_chain=fips_chain.product_fips::<true>(&v[1],&v[2],&v[3]);
                            for i in 0..LANES {fips_want[i]=fips_want[i]*b[i]+c[i]*d[i];}
                        }
                        verify(fips_chain,fips_want);
                    }
                }
"#;
    once(&mut text,test_marker,&format!("{test_marker}\n{extra}"));
    fs::write(&p, &text).unwrap();"##;
    assert_eq!(source.matches(before).count(), 1);
    source = source.replace(before, extra);
    source=source.replace("    assert_eq!(back, original);","    once(&mut back,&format!(\"{test_marker}\\n{extra}\"),test_marker);\n    assert_eq!(back, original);");
    source = source.replace("zopro_g2_fused_outline", "zopro_fq_fips");
    source = source.replace(
        "\"no_new_arithmetic_or_representation\":true",
        "\"new_diagonal_arithmetic\":true,\"representation_unchanged\":true",
    );
    source = source.replace(
        "\"helper\":helper,",
        "\"helper\":helper,\"candidate_test_extra\":extra,",
    );
    source=source.replace("        \"design.md\",","        \"design.md\",\n        \"algebra.md\",\n        \"kernel.txt\",\n        \"model.rs\",\n        \"model.json\",\n        \"build-model.sh\",\n        \"bootstrap.rs\",");
    source = source.replace(
        "onefusedFq8MIXEDoutline",
        "ordinaryandfusedFq8MIXEDdiagonal",
    );
    let a = source.find("fn inspect() {").unwrap();
    let b = source.find("fn freeze(label:").unwrap();
    source.replace_range(a..b, &fs::read_to_string(r.join("code-audit.rs")).unwrap());
    source = source.replace(OLD, R);
    fs::write(r.join("native.rs"), &source).unwrap();
    let a = source.find("fn setup() {").unwrap();
    let b = source.find("fn inspect() {").unwrap();
    fs::write(r.join("setup.rs"), &source[a..b]).unwrap();
}
