use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::{BTreeMap,BTreeSet},fs,io::Read,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-cached-x175";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}


fn freeze(label: &str) {
    assert!(["frozen", "off", "on"].contains(&label));
    let flags = std::env::var("RUSTFLAGS").unwrap();
    let base = fs::read_to_string(r("native-flags.txt")).unwrap();
    assert_eq!(
        flags,
        if label == "on" {
            format!("{base} --cfg zopro_g2_cached_x")
        } else {
            base
        }
    );
    let log = fs::read_to_string(r(format!("native/libraries-{label}/stdout.log"))).unwrap();
    let root = r(format!("frozen/{label}"));
    fs::create_dir_all(&root).unwrap();
    let mut libs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut pins = BTreeMap::new();
    for line in log.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        if v["reason"] != "compiler-artifact" {
            continue;
        }
        let name = v["target"]["name"].as_str().unwrap();
        let files = v["filenames"].as_array().unwrap();
        let metas: Vec<_> = files
            .iter()
            .filter_map(Value::as_str)
            .filter(|p| p.ends_with(".rmeta") && p.contains("/deps/"))
            .collect();
        for file in files
            .iter()
            .filter_map(Value::as_str)
            .filter(|p| p.ends_with(".rlib") || p.ends_with(".so"))
        {
            let p = if file.ends_with(".rlib") && !file.contains("/deps/") && metas.len() == 1 {
                Path::new(metas[0]).with_extension("rlib")
            } else {
                PathBuf::from(file)
            };
            let h = hash(&p);
            let ext = p.extension().unwrap().to_str().unwrap();
            let dest = root.join(format!("lib{name}-{h}.{ext}"));
            if dest.exists() {
                assert_eq!(hash(&dest), h)
            } else {
                fs::copy(&p, &dest).unwrap();
            }
            assert_eq!(hash(&dest), h);
            pins.insert(dest.display().to_string(), h);
            if ext == "rlib" {
                libs.entry(name.into())
                    .or_default()
                    .insert(dest.display().to_string());
            }
        }
    }
    let mut selected = BTreeMap::new();
    for name in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "memmap2",
        "serde_json",
        "libc",
        "rayon",
        "sha2",
    ] {
        let x = libs.get(name).unwrap();
        assert_eq!(x.len(), 1);
        selected.insert(name, x.first().unwrap());
    }
    let mut fingerprints = vec![];
    for e in fs::read_dir(r(format!("target-{label}/release/.fingerprint"))).unwrap() {
        let e = e.unwrap();
        for name in ["zopro-core", "zopro-bn254"] {
            if e.file_name()
                .to_string_lossy()
                .starts_with(&format!("{name}-"))
            {
                let p = e
                    .path()
                    .join(format!("lib-{}.json", name.replace('-', "_")));
                if p.exists() {
                    let f = read(&p);
                    assert_eq!(
                        f["rustflags"],
                        json!(flags.split_whitespace().collect::<Vec<_>>())
                    );
                    if name == "zopro-core" {
                        assert!(f["features"].as_str().unwrap().contains("ifma8"));
                    }
                    fingerprints.push(json!({"path":p,"receipt":f}));
                }
            }
        }
    }
    for name in ["zopro-core", "zopro-bn254"] {
        assert!(fingerprints
            .iter()
            .any(|v| v["path"].as_str().unwrap().contains(name)));
    }
    save(
        r(format!("artifacts-{label}.json")),
        &json!({"label":label,"rustflags":flags,"libraries":selected,"all_libraries":libs,"input_pins":pins,"fingerprints":fingerprints,"proc_macro_shared_objects_frozen":true}),
    );
    println!(
        "Froze{label}{}rlib/sofiles;core/fieldflagschecked",
        pins.len()
    );
}



fn link(label:&str){let info=read(r(format!("artifacts-{label}.json")));let root=Path::new(info["libraries"]["zopro_core"].as_str().unwrap()).parent().unwrap();let mut cmd=std::process::Command::new("rustc");cmd.args(["--edition=2021","-O","-C","target-cpu=native","-C","lto=fat","-C","codegen-units=1","-L"]).arg(format!("dependency={}",root.display()));for(n,p)in info["libraries"].as_object().unwrap(){cmd.arg("--extern").arg(format!("{n}={}",p.as_str().unwrap()));}cmd.arg(r("probe.rs")).arg("-o").arg(r(format!("{label}-probe")));assert!(cmd.status().unwrap().success());let mut pins:BTreeMap<String,String>=serde_json::from_value(info["input_pins"].clone()).unwrap();for f in["probe.rs","perf.rs"]{pins.insert(r(f).display().to_string(),hash(r(f)));}save(r(format!("probe-{label}.json")),&json!({"binary":r(format!("{label}-probe")),"sha256":hash(r(format!("{label}-probe"))),"input_pins":pins,"native_flags":info["rustflags"],"lto":"fat","library_manifest":info}));}
fn main(){let a:Vec<_>=std::env::args().collect();match a[1].as_str(){"freeze"=>freeze(&a[2]),"link"=>link(&a[2]),_=>panic!()}}
