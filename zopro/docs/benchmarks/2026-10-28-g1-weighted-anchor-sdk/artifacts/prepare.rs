use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn main() {
    assert!(Command::new("./capture").status().unwrap().success());
    let mut pins: Value = serde_json::from_slice(&fs::read("pins.json").unwrap()).unwrap();
    for (name, argv) in [
        (
            "sdk-host-before",
            vec!["ps", "-eo", "pid,comm,pcpu,pmem,etimes", "--sort=-pcpu"],
        ),
        ("sdk-compiler", vec!["rustc", "-vV"]),
        ("sdk-machine", vec!["lscpu"]),
    ] {
        let o = Command::new(argv[0]).args(&argv[1..]).output().unwrap();
        assert!(o.status.success());
        let p = format!("{name}.txt");
        fs::write(&p, o.stdout).unwrap();
    }
    for p in [
        "prepare.rs",
        "prepare",
        "close.rs",
        "close",
        "sdk-host-before.txt",
        "sdk-compiler.txt",
        "sdk-machine.txt",
    ] {
        let p = std::env::current_dir().unwrap().join(p);
        pins[p.display().to_string()] = json!(hash(&p));
    }
    fs::write("pins.json", serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s: Value = serde_json::from_slice(&fs::read(p).unwrap()).unwrap();
    s["status"]=json!("G1 weighted-anchor SDK native tests/builds complete; prepared fixed full-proof capture; component committed/pushed, SDK result pending");
    s["active_experiment"] = json!({"root":std::env::current_dir().unwrap(),"plan":"run-plan.json","completed_experiment_commit":"daf62e574d8e18a1a85578e4f984963f8ee1932f","proof_ledger_before":12983});
    fs::write(p, serde_json::to_vec_pretty(&s).unwrap()).unwrap();
    println!(
        "Captureprepared;additionalhost/compiler/close-helperpinsbeforetiming;ledger12983pending"
    );
}
