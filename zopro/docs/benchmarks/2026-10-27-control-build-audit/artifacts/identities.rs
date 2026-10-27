use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
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
    let roots = [
        ("accepted", "/tmp/zopro-composed-cpu/native/composed"),
        ("fresh", "/tmp/zopro-grouping-prefix-cutoff/native-v2/fresh"),
        (
            "previous_fresh",
            "/tmp/zopro-g1-reference-staging/native-v2/fresh",
        ),
    ];
    let mut variants = BTreeMap::new();
    let mut pins = BTreeMap::new();
    for (name, root) in roots {
        let p = Path::new(root);
        let a = read(p.join("artifacts.json"));
        let c = read(p.join("command.json"));
        for (f, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
            pins.insert(f.clone(), h.as_str().unwrap().to_owned());
        }
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        let libs: BTreeMap<_, _> = a["libraries"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, p)| {
                (
                    k.clone(),
                    json!({"path":p,"sha256":hash(p.as_str().unwrap())}),
                )
            })
            .collect();
        variants.insert(name,json!({"binary":a["binary"],"binary_sha256":a["binary_sha256"],"libraries":libs,"command":c}));
        for f in [p.join("artifacts.json"), p.join("command.json")] {
            pins.insert(f.display().to_string(), hash(&f));
        }
    }
    let mut comparisons = Vec::new();
    for name in ["fresh", "previous_fresh"] {
        for lib in [
            "libc",
            "memmap2",
            "rayon",
            "serde_json",
            "zopro_bn254",
            "zopro_core",
            "zopro_prover",
            "zopro_zolana",
        ] {
            comparisons.push(json!({"variant":name,"library":lib,"bytes_equal_to_accepted":variants[name]["libraries"][lib]["sha256"]==variants["accepted"]["libraries"][lib]["sha256"]}));
        }
    }
    let m = read("/tmp/zopro-composed-adoption/production-source-manifest.json");
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(f)),
            h.as_str().unwrap()
        );
        for tree in [
            "/tmp/zopro-grouping-prefix-cutoff/fresh",
            "/tmp/zopro-g1-reference-staging/fresh",
        ] {
            assert_eq!(hash(Path::new(tree).join(f)), h.as_str().unwrap());
        }
    }
    fs::write("/tmp/zopro-control-build-audit/identities.json",serde_json::to_vec_pretty(&json!({"variants":variants,"library_comparisons":comparisons,"production_and_both_fresh_sources_exact":171,"new_proofs":0,"source_changed":false,"goal_achieved":false})).unwrap()).unwrap();
    fs::write(
        "/tmp/zopro-control-build-audit/input-pins.json",
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    for c in comparisons {
        println!(
            "{} {} bytes_equal={}",
            c["variant"], c["library"], c["bytes_equal_to_accepted"]
        );
    }
}
