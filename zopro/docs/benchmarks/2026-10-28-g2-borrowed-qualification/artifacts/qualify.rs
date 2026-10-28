use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-qualification";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-sdk";
const I: &str = "/home/ubuntu/src/zopro-perf-work/sdk-control-copies";
const V: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
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
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn base() -> Value {
    let mut e =
        read("/home/ubuntu/src/zopro-perf-work/center-census/run-plan.json")["environment"].clone();
    e["ZOPRO_BASIS_KEY_DIR"] = json!(format!("{I}/images"));
    e["ZOPRO_CPU_LAGRANGE_DIR"] = json!(format!("{I}/lagrange-cache"));
    e
}
fn sdk(v: &str) -> PathBuf {
    if v == "portable" {
        r("build/portable-sdk")
    } else {
        Path::new(P).join(if v == "accepted" {
            "accept-sdk"
        } else {
            "borrow-sdk"
        })
    }
}
fn verifier(image: &Path, ih: &str, proof: &Path, public: &Path, ph: &str) -> Vec<String> {
    vec![
        V.into(),
        image.display().to_string(),
        ih.into(),
        proof.display().to_string(),
        public.display().to_string(),
        ph.into(),
    ]
}
fn prepare() {
    assert!(!r("functional-plan.json").exists());
    assert_eq!(read(r("build/result.json"))["status"], "complete");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h)
    }
    let pa = read(r("portable-artifacts.json"));
    for (p, h) in pa["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    for label in [
        "portable",
        "accepted-warm",
        "accepted-concurrent",
        "on-warm",
        "on-concurrent",
    ] {
        let p = r(format!("build/{label}/artifacts.json"));
        let a = read(&p);
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        pin(&mut pins, a["binary"].as_str().unwrap());
        pin(&mut pins, p);
    }
    for v in ["accepted", "on"] {
        pin(&mut pins, sdk(v));
    }
    let cases = read(format!("{I}/cases.json"));
    let cases = cases.as_array().unwrap();
    let primary = &cases[0];
    let big = &cases[8];
    let mut functional = vec![];
    for c in cases {
        functional.push(json!({"name":format!("portable-basis-{}",c["shape"].as_str().unwrap()),"variant":"portable","kind":"basis","case":c,"workers":4}));
    }
    for c in [primary, big] {
        for kind in ["raw", "denied"] {
            functional.push(json!({"name":format!("portable-{kind}-{}",c["shape"].as_str().unwrap()),"variant":"portable","kind":kind,"case":c,"workers":4}));
        }
    }
    for w in [1, 4] {
        for stack in [524288, 1048576] {
            for c in [primary, big] {
                for kind in ["basis", "raw", "denied"] {
                    functional.push(json!({"name":format!("native-w{w}-s{stack}-{kind}-{}",c["shape"].as_str().unwrap()),"variant":"on","kind":kind,"case":c,"workers":w,"stack":stack}));
                }
            }
        }
    }
    for v in ["accepted", "on"] {
        for option in ["PACKED", "DIRECT", "CANONICAL_BATCH", "VECTOR_INVERSE"] {
            functional.push(json!({"name":format!("optout-{v}-{}",option.to_lowercase()),"variant":v,"kind":"basis","case":primary,"workers":4,"optout":format!("ZOPRO_CPU_{option}")}));
        }
    }
    assert_eq!(functional.len(), 45);
    let mut steps = vec![];
    let mut jobs = vec![];
    let mut verify = vec![];
    for f in functional {
        let c = &f["case"];
        let name = f["name"].as_str().unwrap();
        let v = f["variant"].as_str().unwrap();
        let kind = f["kind"].as_str().unwrap();
        let mut env = base();
        env["RAYON_NUM_THREADS"] = json!(f["workers"].as_u64().unwrap().to_string());
        if let Some(n) = f["stack"].as_u64() {
            env["RUST_MIN_STACK"] = json!(n.to_string());
        }
        if let Some(o) = f["optout"].as_str() {
            env[o] = json!("0");
        }
        if kind != "basis" {
            env["ZOPRO_CPU_PREPARE_MAX_BYTES"] = json!("0");
        }
        let image = PathBuf::from(
            c[if kind == "raw" { "raw" } else { "image" }]
                .as_str()
                .unwrap(),
        );
        let ih = c[if kind == "raw" {
            "raw_sha256"
        } else {
            "image_sha256"
        }]
        .as_str()
        .unwrap();
        assert_eq!(hash(&image), ih);
        pin(&mut pins, &image);
        pin(&mut pins, c["request"].as_str().unwrap());
        let dir = r("proofs").join(name);
        steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c",if f["workers"]==1{"0"}else{"0-3"},sdk(v),image,c["request"],"1",dir],"environment":env,"timeout_seconds":1800}));
        let verification_image = Path::new(c["image"].as_str().unwrap());
        pin(&mut pins, verification_image);
        verify.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":verifier(verification_image,c["image_sha256"].as_str().unwrap(),&dir.join("0.proof"),&dir.join("0.public"),c["public_sha256"].as_str().unwrap()),"timeout_seconds":180}));
        jobs.push(json!({"name":name,"kind":"functional","variant":v,"configuration":f,"image":image,"image_sha256":ih,"verification_image":verification_image,"verification_image_sha256":c["image_sha256"],"directory":dir,"exports":1,"SDK_requests":4,"public_sha256":c["public_sha256"]}));
    }
    for v in ["accepted", "on"] {
        for c in [primary, big] {
            let name = format!("concurrent-{v}-{}", c["shape"].as_str().unwrap());
            let dir = r("proofs").join(&name);
            let mut env = base();
            env["RAYON_NUM_THREADS"] = json!("32");
            steps.push(json!({"kind":"command","name":name,"argv":["/usr/bin/time","-v",r(format!("build/{v}-concurrent-sdk")),c["shape"],"4","2",dir],"environment":env,"timeout_seconds":1800}));
            let image = Path::new(c["image"].as_str().unwrap());
            pin(&mut pins, image);
            let stem = image.file_stem().unwrap();
            for caller in 0..4 {
                for round in 0..2 {
                    let out = dir.join(stem);
                    verify.push(json!({"kind":"command","name":format!("verify-{name}-t{caller}-r{round}"),"argv":verifier(image,c["image_sha256"].as_str().unwrap(),&out.join(format!("t{caller}-r{round}.proof")),&out.join(format!("t{caller}-r{round}.public")),c["public_sha256"].as_str().unwrap()),"timeout_seconds":180}));
                }
            }
            jobs.push(json!({"name":name,"kind":"concurrent","variant":v,"image":image,"image_sha256":c["image_sha256"],"directory":dir,"exports":8,"SDK_requests":8,"public_sha256":c["public_sha256"]}));
        }
    }
    steps.extend(verify);
    let mut resources = vec![];
    for kind in ["cold", "warm"] {
        for v in ["accepted", "on"] {
            for policy in ["default", "retained"] {
                let name = format!("{kind}-{v}-{policy}");
                let bin = if kind == "warm" {
                    r(format!("build/{v}-warm-sdk"))
                } else {
                    sdk(v)
                };
                resources.push(json!({"name":name,"kind":kind,"variant":v,"allocator":policy,"binary":bin,"image":primary["image"],"image_sha256":primary["image_sha256"],"request":primary["request"],"verifier":[r("qualify"),"verify-resource",primary["image"],primary["image_sha256"],"{proofs}",primary["public_sha256"],if kind=="warm"{"2"}else{"1"}]}));
                jobs.push(json!({"name":name,"kind":"resource","variant":v,"configuration":kind,"image":primary["image"],"image_sha256":primary["image_sha256"],"directory":r("runs").join(&name).join("proofs"),"exports":if kind=="warm"{2}else{1},"SDK_requests":if kind=="warm"{5}else{4},"public_sha256":primary["public_sha256"]}));
            }
        }
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{P}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 13712);
    for (p, h) in read(format!("{P}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        if p.ends_with(".proof") {
            assert_eq!(hash(p), h.as_str().unwrap());
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 14462);
    save(r("prior-proof-sha256.json"), &json!(prior));
    save(r("jobs.json"), &json!(jobs));
    save(r("resource-cases.json"), &json!(resources));
    save(r("inputs.json"), &json!({"environment":base()}));
    for p in [
        "design.md",
        "prepare.rs",
        "prepare",
        "qualify.rs",
        "qualify",
        "resources.rs",
        "resources",
        "warm.rs",
        "concurrent.rs",
        "finish.rs",
        "finish",
        "close.rs",
        "close",
        "build/result.json",
        "portable-artifacts.json",
        "prior-proof-sha256.json",
        "jobs.json",
        "inputs.json",
        "resource-cases.json",
    ] {
        pin(&mut pins, r(p));
    }
    pin(&mut pins, V);
    save(r("pins.json"), &json!(pins));
    save(
        r("functional-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared45functional+4concurrent,77exports; resources8cases/12exports; total248SDK89exports ledger14551 after validation");
}
fn files(root: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(root).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            files(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path());
        }
    }
}
fn verify_resource(a: &[String]) {
    assert_eq!(a.len(), 7);
    let mut f = vec![];
    files(Path::new(&a[4]), &mut f);
    let mut proofs: Vec<_> = f
        .into_iter()
        .filter(|p| p.extension().is_some_and(|e| e == "proof"))
        .collect();
    proofs.sort();
    assert_eq!(proofs.len(), a[6].parse::<usize>().unwrap());
    for p in proofs {
        let argv = verifier(
            Path::new(&a[2]),
            &a[3],
            &p,
            &p.with_extension("public"),
            &a[5],
        );
        let o = Command::new(&argv[0]).args(&argv[1..]).output().unwrap();
        print!("{}", String::from_utf8(o.stdout).unwrap());
        eprint!("{}", String::from_utf8(o.stderr).unwrap());
        assert!(o.status.success());
    }
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("prepare") => prepare(),
        Some("verify-resource") => verify_resource(&a),
        _ => panic!("prepare/verify-resource"),
    }
}
