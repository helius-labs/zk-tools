use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, os::unix::fs::{MetadataExt, PermissionsExt}, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf177";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g1-offset177";
fn hash(p: impl AsRef<Path>) -> String { format!("{:x}", Sha256::digest(fs::read(p).unwrap())) }
fn save(name: &str, bytes: impl AsRef<[u8]>) {
    let p = format!("{R}/{name}"); assert!(!Path::new(&p).exists()); fs::write(p, bytes).unwrap();
}
fn main() {
    let check: Value = serde_json::from_slice(&fs::read(format!("{R}/checkpoint.json")).unwrap()).unwrap();
    assert_eq!(check["passed"], true);
    assert_eq!(check["SDK_requests"], 0);
    assert_eq!(hash("/home/ubuntu/src/zopro-perf-work/ibs-backing177/proof-sha256-after-profile.json"), "4f86193860e35d6f16a765ab2aac7cbff561b747928a73f56a153d86a3cc202d");
    let mut ids = json!({"REF":{"path":"/home/ubuntu/src/zopro-perf-work/phi-grain-c7a176/sdk-candidate-znver4","sha256":"c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe"}});
    for (label,arm) in [("control","OFF"),("candidate","ON")] {
        ids[arm] = check["consumers"].as_array().unwrap().iter().find(|c| c["label"] == label).unwrap().clone();
    }
    fs::create_dir(format!("{R}/bin")).unwrap();
    let mut inodes = BTreeSet::new();
    for (arm,letter) in [("REF","r"),("OFF","f"),("ON","n")] {
        let source = ids[arm]["path"].as_str().unwrap();
        assert_eq!(hash(source), ids[arm]["sha256"]);
        let bytes = fs::read(source).unwrap(); let original = fs::metadata(source).unwrap(); let mut copies = vec![];
        for i in 0..3 {
            let p = format!("{R}/bin/sdk-{letter}{i}");
            fs::write(&p, &bytes).unwrap(); fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).unwrap();
            assert_eq!(hash(&p), ids[arm]["sha256"]); let m = fs::metadata(&p).unwrap();
            assert!(inodes.insert((m.dev(),m.ino()))); assert_ne!((m.dev(),m.ino()),(original.dev(),original.ino()));
            copies.push(json!({"path":p,"sha256":hash(&p),"device":m.dev(),"inode":m.ino(),"mode":m.permissions().mode()}));
        }
        ids[arm]["copies"] = json!(copies);
    }
    save("runtime-consumers.json", serde_json::to_vec_pretty(&ids).unwrap());
    for name in ["screen.rs","independent.rs"] {
        let s = fs::read_to_string(format!("{OLD}/{name}")).unwrap().replace(OLD,R)
            .replace("/home/ubuntu/src/zopro-perf-work/gather-backing177/proof-sha256-after-capture.json", "/home/ubuntu/src/zopro-perf-work/ibs-backing177/proof-sha256-after-profile.json")
            .replace("52000","52972").replace("52027","52999").replace("52927","53899");
        assert!(!s.contains(OLD)); save(name,s);
    }
    let mut design = fs::read_to_string(format!("{OLD}/screen-design.md")).unwrap()
        .replace("# G1-only full-proof screen", "# G2 coordinate leaf full-proof screen")
        .replace("Earlier\ncombined G1/G2 and gather campaigns remain closed with their original outcomes.",
                 "Earlier\ncampaigns remain closed with their original outcomes. The candidate introduces\none coordinate helper call per full eight-point borrowed G2 reverse group.\nInstruction delivery is the hypothesis; code size and stack reservations grew,\nso no gain is presumed. See design.md and checkpoint.json for source/test scope.")
        .replace("90f1d8fbd8bd533daa4fcd572f66764a71266de378c17112f4074d5814ace4c3", ids["OFF"]["sha256"].as_str().unwrap())
        .replace("G1-only ON 0db7b2530991766c7d5b6436b89d759a9f258690aa60f625bc70ea5bc265108e", &format!("G2 coordinate leaf ON {}", ids["ON"]["sha256"].as_str().unwrap()))
        .replace("52000","52972").replace("52027","52999").replace("52927","53899")
        .replace("gather-backing177 ledger", "ibs-backing177/proof-sha256-after-profile.json ledger");
    design.push_str("\nPreparation reuses the completed g1-offset177 screen and independent reader\nwith only root/ledger/count identities adapted; comparisons and order are\nunchanged. This is a new source candidate, not a rerun of an earlier failed gate.\n");
    save("screen-design.md", design);
    println!("Prepared nine fixed independent executable backings and full ALL8 screen; no SDK calls.");
}
