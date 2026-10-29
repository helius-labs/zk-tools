use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-preparation175";const OLD:&str="/home/ubuntu/src/zopro-perf-work/service-trace175";
fn main(){let r=Path::new(R);assert!(!r.join("native.rs").exists());
for file in["driver.rs","support/perf.rs","setup.rs"]{fs::copy(Path::new(OLD).join(file),r.join(file)).unwrap();}
let mut trace=fs::read_to_string(Path::new(OLD).join("service_trace.rs")).unwrap();trace.push_str(&fs::read_to_string(r.join("fold-additions.rs.txt")).unwrap());fs::write(r.join("service_trace.rs"),trace).unwrap();
let mut patch=fs::read_to_string(Path::new(OLD).join("patch.rs")).unwrap().replace(OLD,R);
let at=patch.find("    fs::write(\n        Path::new(R).join(\"insertions.json\")").unwrap();patch.insert_str(at,r###"
    // Serial grouping is nonforking; preserve original initializer verbatim.
    let file="msm_affine.rs";let path=Path::new(R).join("source/crates/zopro-core/src").join(file);
    let mut s=fs::read_to_string(&path).unwrap();let a=s.find("pub(super) fn g2_query_exclusive(").unwrap();
    let original="let groups = crate::simd::cpu_option(\"ZOPRO_CPU_DEDUP\")\n        .then(|| scalar_groups(words))\n        .flatten();";
    let offset=a+s[a..].find(original).unwrap();
    let new=format!("let groups = {{\n#[cfg(zopro_service_trace)] let trace_group=crate::service_trace::serial(30,words.len(),0);\n{original}\n#[cfg(zopro_service_trace)] drop(trace_group);\ngroups\n}};");
    s.replace_range(offset..offset+original.len(),&new);fs::write(&path,s).unwrap();p.push(json!({"file":file,"old":original,"new":new}));
    // This boundary can fork; do not charge its elapsed/CPU as serial work.
    add(file,"pub(super) fn g2_query_exclusive(","let points = crate::msm::group_affine(bases, &g.order, &g.starts, &g.counts);","drop(trace_points);",false,&mut p);
    add(file,"pub(super) fn g2_query_exclusive(","let points = crate::msm::group_affine(bases, &g.order, &g.starts, &g.counts);","let trace_points=crate::service_trace::kernel(31,bases.len(),g.values.len());",true,&mut p);
    let file="msm_affine/g2_phi.rs";let path=Path::new(R).join("source/crates/zopro-core/src").join(file);let mut s=fs::read_to_string(&path).unwrap();
    let original="short.par_iter_mut().for_each(|s| *s = phi_split(*s));";
    assert_eq!(s.matches(original).count(),1);
    let new=r#"
    #[cfg(zopro_service_trace)]
    {
        let trace_split=crate::service_trace::scope(trace_kernel_id,32,short.len(),0,false);
        if crate::service_trace::enabled() {
            let trace_split_id=trace_split.id();
            short.par_iter_mut().map_init(||crate::service_trace::Fold::new(trace_split_id),|fold,s|{
                fold.item();*s=phi_split(*s);
            }).for_each(drop);
        } else {
            short.par_iter_mut().for_each(|s| *s = phi_split(*s));
        }
        drop(trace_split);
    }
    #[cfg(not(zopro_service_trace))]
    short.par_iter_mut().for_each(|s| *s = phi_split(*s));
"#;
    s=s.replace(original,new);fs::write(path,s).unwrap();p.push(json!({"file":file,"old":original,"new":new}));
"###);fs::write(r.join("patch.rs"),patch).unwrap();
let mut native=fs::read_to_string(Path::new(OLD).join("native.rs")).unwrap().replace(OLD,R);
let generator="        {\"kind\":\"command\",\"name\":\"generator-check\",\"argv\":[\"python3\",r(\"source/scripts/ifma8.py\"),\"--check\"],\"timeout_seconds\":180},\n";assert!(native.contains(generator));native=native.replace(generator,"");
native=native.replace("Prepared7steps","Prepared6steps").replace("126core8prover","128core8prover").replace("\"service_trace::tests::bounded_capacity_preserves_execution_and_reports_drops\"]","\"service_trace::tests::bounded_capacity_preserves_execution_and_reports_drops\",\"service_trace::fold_tests::folds_count_items_and_restore_parent_before_snapshot\",\"service_trace::fold_tests::map_init_preserves_indexed_producer_bounds_and_fixed_splits\"]");
// Keep the setup source as the actual extracted implementation for provenance.
let a=native.find("fn setup() {").unwrap();let b=native.find("fn inspect() {").unwrap();fs::write(r.join("setup.rs"),&native[a..b]).unwrap();fs::write(r.join("native.rs"),native).unwrap();
// Driver and four-prime/sample semantics are inherited byte-exact.
for name in["map_with.rs","for_each.rs","plumbing/mod.rs"]{let from=Path::new("/home/ubuntu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rayon-1.12.0/src/iter").join(name);let to=r.join("rayon-reference").join(name);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();}
println!("Prepared diagnostic fold extension/8reversible core originals,176private sources; pinned Rayon1.12 source copied");}
