use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/interleaved-complex175";
fn rep(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"expected unique {a}");*s=s.replace(a,b);}
fn main(){
    let root=Path::new(R);let mut s=fs::read_to_string(root.join("capture.rs")).unwrap();
    assert_eq!(s.matches("native/result.json").count(),2);s=s.replace("native/result.json","native-remaining/result.json");
    rep(&mut s,"read(r(\"build-pins.json\"))","read(r(\"remaining-pins.json\"))");
    rep(&mut s,"r(format!(\"native/{s}-{v}/tests.json\"))","r(format!(\"{}/{s}-{v}/tests.json\",if v==\"on\"{\"native-remaining\"}else{\"native\"}))");
    rep(&mut s,"r(format!(\"native/{s}-{v}/{f}\"))","r(format!(\"{}/{s}-{v}/{f}\",if v==\"on\"{\"native-remaining\"}else{\"native\"}))");
    rep(&mut s,"\"target/release/model\"]","\"target/release/model\",\"native-inheritance.json\",\"remaining-plan.json\",\"prepare-closing.rs\",\"prepare-closing\"]");
    fs::write(root.join("capture.rs"),s).unwrap();
    let mut s=fs::read_to_string(root.join("publication.rs")).unwrap();
    rep(&mut s,"\"parent-unsigned-cios.rs\"]","\"parent-unsigned-cios.rs\",\"repair-native.rs\",\"prepare-closing.rs\",\"native-inheritance.json\",\"remaining-pins.json\",\"remaining-plan.json\",\"source-audit-initial.json\",\"initial-simd.rs\"]");
    rep(&mut s,"receipts.extend(files(&r(\"runs\")));","receipts.extend(files(&r(\"runs\")));receipts.extend(files(&r(\"native-remaining\")));");
    rep(&mut s,"\"private172_originals_exact\":true","\"private171_originals_exact\":true");
    rep(&mut s,"with172of175 originals exact","with 171 of 175 originals exact");
    rep(&mut s,"Removing the field include and core adapter suffixes and reversing two dispatch edits recovers all 175 originals;", "Removing the field include, ON-only SIMD forwarding bridge and core adapter suffixes and reversing two dispatch edits recovers all 175 originals;");
    rep(&mut s,"Existing field qualification for exact OFF arithmetic is inherited.", "The initial core ON compilation found a missing FqBatch forwarding method. An ON-only wrapper fixes it without changing field arithmetic or compiled OFF tokens. The 35 successful field tests and all six completed initial OFF/field/build steps are retained, with seven remaining workflow steps separately recorded. Existing field qualification for exact OFF arithmetic is inherited.");
    fs::write(root.join("publication.rs"),s).unwrap();println!("Prepared closing receipt resolution; original build/repair sources and successful receipts preserved");
}
