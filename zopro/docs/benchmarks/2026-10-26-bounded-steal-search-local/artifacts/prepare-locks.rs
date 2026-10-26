use std::{collections::BTreeMap, fs, path::Path};
fn name(block: &str) -> &str {
    block.lines().find_map(|line|line.strip_prefix("name = \"")).unwrap().trim_end_matches('"')
}
fn packages(text: &str) -> BTreeMap<(&str, &str), &str> {
    text.split("[[package]]").skip(1).map(|block| {
        let version=block.lines().find_map(|line|line.strip_prefix("version = \"")).unwrap().trim_end_matches('"');
        ((name(block),version),block)
    }).collect()
}
fn main() {
    let base=Path::new("/home/ubuntu/src/zk-tools/zopro");
    let root=Path::new("/tmp/zopro-bounded-steal-search");
    let original=fs::read_to_string(base.join("Cargo.lock")).unwrap();
    let mut output=String::new();
    let mut changed=0;
    for (i,block) in original.split("[[package]]").enumerate() {
        if i==0 {output.push_str(block);continue;}
        output.push_str("[[package]]");
        if name(block)=="rayon-core" {
            changed+=1;
            for line in block.split_inclusive('\n') {
                if !line.starts_with("source = ") && !line.starts_with("checksum = ") {output.push_str(line);}
            }
        } else {output.push_str(block);}
    }
    assert_eq!(changed,1);
    let before=packages(&original);let after=packages(&output);
    assert_eq!(before.len(),after.len());
    for (key,block) in &before {if key.0!="rayon-core" {assert_eq!(*block,after[key]);}}
    fs::write(root.join("overlay/Cargo.lock"),output).unwrap();
    let upstream=fs::read_to_string("/tmp/zopro-single-steal-sdk/workspace/vendor/rayon-core/Cargo.lock").unwrap();
    let mut dev=String::new();
    for (i,block) in upstream.split("[[package]]").enumerate() {
        if i==0 {dev.push_str(block);continue;}
        if name(block)=="st3" {continue;}
        dev.push_str("[[package]]");
        for line in block.split_inclusive('\n') {if line!=" \"st3\",\n" {dev.push_str(line);}}
    }
    let dev_packages=packages(&dev);
    for key in [("crossbeam-deque","0.8.8"),("crossbeam-epoch","0.9.21"),("crossbeam-utils","0.8.23")] {
        assert_eq!(before[&key],dev_packages[&key]);
    }
    fs::write(root.join("overlay/vendor/rayon-core/Cargo.lock"),dev).unwrap();
    println!("Main lock: {} entries, only Rayon path source/checksum changed. Dev lock: exact three Crossbeam runtime entries match.",before.len());
}
