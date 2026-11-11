use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/ibs-backing177";
fn main(){
    assert!(!Path::new(&format!("{R}/common.rs")).exists());
    let old="/home/ubuntu/src/zopro-perf-work/g1-offset-profile177";
    let c=fs::read_to_string(format!("{old}/common.rs")).unwrap();
    let c=c.split("include!(\"sampling.rs\");").next().unwrap().replace(old,R).replace("/home/ubuntu/src/zopro-perf-work/g1-offset177/proof-sha256-after-main.json",&format!("{old}/proof-sha256-after-profile.json"));
    fs::write(format!("{R}/common.rs"),c).unwrap();
    let old="/home/ubuntu/src/zopro-perf-work/stream-ibs175";
    fs::copy(format!("{old}/legacy-decoder.rs"),format!("{R}/legacy-decoder.rs")).unwrap();
    let d=fs::read_to_string(format!("{old}/decode.rs")).unwrap();
    let path="/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk";
    assert_eq!(d.matches(path).count(),1);
    for i in 0..3{fs::write(format!("{R}/decode-{i}.rs"),d.replace(path,&format!("/home/ubuntu/src/zopro-perf-work/g1-offset177/bin/sdk-f{i}"))).unwrap();}
    let census=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/page-census177/census.rs").unwrap();
    for phase in ["before","after"] {let p=format!("{R}/pages-{phase}");fs::write(format!("{p}/census.rs"),census.replace("/home/ubuntu/src/zopro-perf-work/page-census177",&p)).unwrap();}
    println!("Inherited SDK receipts/proof checks, exact IBS parser with DSO-path changes, exact census with destination changes; no SDK executed");
}
