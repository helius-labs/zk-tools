use serde_json::{json, Value};
use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-preparation175";
fn add(
    file: &str,
    function: &str,
    anchor: &str,
    body: &str,
    before: bool,
    patches: &mut Vec<Value>,
) {
    let p = Path::new(R).join("source/crates/zopro-core/src").join(file);
    let mut s = fs::read_to_string(&p).unwrap();
    let begin = s
        .find(function)
        .unwrap_or_else(|| panic!("function {file}:{function}"));
    let at = begin
        + s[begin..]
            .find(anchor)
            .unwrap_or_else(|| panic!("anchor {file}:{function}:{anchor}"))
        + if before { 0 } else { anchor.len() };
    let text = format!("\n#[cfg(zopro_service_trace)] {body}\n");
    s.insert_str(at, &text);
    fs::write(p, s).unwrap();
    patches.push(json!({"file":file,"function":function,"inserted":text}));
}
fn main() {
    let mut p = vec![];
    add(
        "lib.rs",
        "mod msm;",
        "mod msm;",
        "pub mod service_trace;",
        false,
        &mut p,
    );
    for (file, function, call) in [
        (
            "msm_affine.rs",
            "pub(super) fn msm_canonical<C:",
            "root(6,bases.len(),core::mem::size_of::<C::Base>())",
        ),
        (
            "msm_affine.rs",
            "pub(super) fn g2_query_exclusive(",
            "root(5,bases.len(),0)",
        ),
        (
            "msm_affine/fused_kh.rs",
            "pub(crate)fn fused(",
            "root(2,k.len(),h.len())",
        ),
        (
            "msm_affine.rs",
            "fn merge_split_width<",
            "kernel(1,bases.len(),width)",
        ),
        (
            "msm_affine/borrowed_pending.rs",
            "pub(crate) fn msm<",
            "kernel(7,bases.len(),width)",
        ),
        (
            "msm_affine/borrowed_pending/balanced.rs",
            "pub(super) fn fused<",
            "kernel(8,k.len(),h.len())",
        ),
        (
            "msm_affine/g2_phi.rs",
            "fn hb_raw(",
            "kernel(3,bases.len(),bits)",
        ),
    ] {
        add(
            file,
            function,
            "{",
            &format!("let _trace_boundary=crate::service_trace::{call};"),
            false,
            &mut p,
        );
    }
    for (file, function, anchor, kind, args) in [
        (
            "msm_affine/borrowed_pending.rs",
            "pub(crate) fn msm<",
            ".map(|w| {",
            10,
            "w,width",
        ),
        (
            "msm_affine/borrowed_pending/balanced.rs",
            "pub(super) fn fused<",
            ".map(|&span| {",
            11,
            "span.start,span.width",
        ),
        (
            "msm_affine/g2_phi.rs",
            "fn hb_raw(",
            ".map(|w| {",
            12,
            "w,bits",
        ),
    ] {
        let boundary_end = if kind == 10 {
            "let _trace_boundary=crate::service_trace::kernel(7,bases.len(),width);"
        } else if kind == 11 {
            "let _trace_boundary=crate::service_trace::kernel(8,k.len(),h.len());"
        } else {
            "let _trace_boundary=crate::service_trace::kernel(3,bases.len(),bits);"
        };
        add(
            file,
            function,
            boundary_end,
            "let trace_kernel_id=_trace_boundary.id();",
            false,
            &mut p,
        );
        add(file,function,anchor,&format!("let _trace_window=crate::service_trace::scope(trace_kernel_id,{kind},{args},false);"),false,&mut p);
        if kind != 11 {
            let scope=format!("let _trace_window=crate::service_trace::scope(trace_kernel_id,{kind},{args},false);");
            add(
                file,
                function,
                &scope,
                "let trace_window_id=_trace_window.id();",
                false,
                &mut p,
            );
            let (part_anchor, part_kind, part) = if kind == 10 {
                (".map(|p| {", 13, "p")
            } else {
                (".map(|part| {", 14, "part")
            };
            add(file,function,part_anchor,&format!("let _trace_part=crate::service_trace::scope(trace_window_id,{part_kind},{part},0,false);"),false,&mut p);
        }
    }
    add(
        "msm_affine/g2_phi.rs",
        "fn hb_raw(",
        "short.par_iter_mut().for_each(|s| *s = phi_split(*s));",
        "{let ready=crate::service_trace::scope(trace_kernel_id,4,0,0,false);drop(ready);}",
        false,
        &mut p,
    );
    for (file, function, args, kind) in [
        (
            "msm_affine/borrowed_pending.rs",
            "    fn feed(",
            "points.len(),w",
            20,
        ),
        (
            "msm_affine/borrowed_pending/balanced.rs",
            "fn feed<'a,",
            "points.len(),span.start",
            21,
        ),
        (
            "msm_affine/g2_borrowed.rs",
            "    fn feed(",
            "bases.len(),w",
            22,
        ),
    ] {
        add(
            file,
            function,
            "{",
            &format!("let _trace_service=crate::service_trace::serial({kind},{args});"),
            false,
            &mut p,
        );
    }
    for (file, curve, kind) in [
        ("msm_affine/borrowed_pending.rs", "Projective::<C>", 27),
        ("msm_affine/g2_borrowed.rs", "G2Projective", 29),
    ] {
        add(file,"    fn finish(mut self)","{",&format!("let _trace_tail=crate::service_trace::serial({kind},self.affine.len(),self.projective.len());"),false,&mut p);
        add(
            file,
            "    fn finish(mut self)",
            &format!("        for (p, &i) in {curve}::batch_to_affine"),
            "drop(_trace_tail);",
            true,
            &mut p,
        );
    }
    // The affine/projective phi loops are nonforking and preserve their exact bytes.
    let file = "msm_affine/g2_borrowed.rs";
    let path = Path::new(R).join("source/crates/zopro-core/src").join(file);
    let mut s = fs::read_to_string(&path).unwrap();
    let a = s.find("    for p in &mut s.affine {").unwrap();
    let b = a + s[a..].find("    s.feed(bases, short, false").unwrap();
    let old = s[a..b].to_string();
    let new=format!("{{\n#[cfg(zopro_service_trace)] let _trace_maps=crate::service_trace::serial(28,s.affine.len(),s.projective.len());\n{old}}}\n");
    s.replace_range(a..b, &new);
    fs::write(path, s).unwrap();
    p.push(json!({"file":file,"old":old,"new":new}));
    for (file, function, anchor, kind, args) in [
        (
            "msm.rs",
            "fn group_affine_impl<",
            "|(points, scratch), (i, output)| {",
            23,
            "output.len(),i",
        ),
        (
            "msm.rs",
            "pub(super) fn reduce_indexed_buckets<",
            ".map(|(first, end)| {",
            24,
            "end-first,first",
        ),
        (
            "msm.rs",
            "pub(super) fn weighted_affine<",
            "|(tree, scratch), buckets| {",
            25,
            "buckets.len(),0",
        ),
        (
            "msm_affine.rs",
            "fn merge_buckets<",
            ".for_each(|(left, right)| {",
            26,
            "left.len(),0",
        ),
    ] {
        add(
            file,
            function,
            "{",
            "let trace_parent=crate::service_trace::parent();",
            false,
            &mut p,
        );
        add(
            file,
            function,
            anchor,
            &format!(
                "let _trace_leaf=crate::service_trace::scope(trace_parent,{kind},{args},true);"
            ),
            false,
            &mut p,
        );
    }

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
            short.par_iter_mut().map_init(||crate::service_trace::Fold::new(trace_split.id()),|fold,s|{
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
    fs::write(
        Path::new(R).join("insertions.json"),
        serde_json::to_vec_pretty(&p).unwrap(),
    )
    .unwrap();
    println!("Inserted sparse current query/kernel/window/part/serial-leaf scopes");
}
