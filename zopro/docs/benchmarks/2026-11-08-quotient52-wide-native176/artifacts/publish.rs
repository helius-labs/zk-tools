use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/quotient52-wide176";
const P: &str = "/home/ubuntu/src/zopro-perf-work/quotient-mullo176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-08-quotient52-wide-native176";
fn digest(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn hash(p: impl AsRef<Path>) -> String { digest(&fs::read(p).unwrap()) }
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn opcode_count(s: &str, op: &str) -> usize { s.lines().filter(|l| l.contains(&format!("\t{op} "))).count() }
fn main() {
    let r=Path::new(R); let d=Path::new(D); assert!(!d.exists());
    let q=read(r.join("qualification.json")); assert_eq!(q["passed"],true);
    assert_eq!(q["new_proofs"],0); assert_eq!(q["new_timings"],0); assert_eq!(q["goal_achieved"],false);
    let mut pin_count=0;
    for n in ["build-pins.json","codegen-pins.json","decode-pins.json"] {
        for (p,h) in read(r.join(n)).as_object().unwrap() { assert_eq!(hash(p),h.as_str().unwrap()); pin_count+=1; }
    }
    let a=read(r.join("source-audit.json")); let rel=a["changed_member"].as_str().unwrap();
    let new=fs::read_to_string(r.join("source").join(rel)).unwrap();
    let old=fs::read_to_string(format!("{P}/source/{rel}")).unwrap();
    assert_eq!(new.replacen(a["after"].as_str().unwrap(),a["before"].as_str().unwrap(),1),old);
    let pa=read(format!("{P}/source-audit.json"));
    let original=old.strip_suffix(pa["diagnostic_suffix"].as_str().unwrap()).unwrap()
        .replacen(pa["changed_cios"].as_str().unwrap(),pa["original_cios"].as_str().unwrap(),1);
    assert_eq!(original,fs::read_to_string(format!("/home/ubuntu/src/zk-tools/zopro/{rel}")).unwrap());
    let np=read(r.join("build-plan.json")); let pp=read(format!("{P}/build-plan.json"));
    for (ni,pi) in [(0,0),(3,4)] {
        let av=np["steps"][ni]["argv"].as_array().unwrap();
        let mut recovered=av.iter().map(|s|s.as_str().unwrap().replace(R,P)).collect::<Vec<_>>();
        assert_eq!(recovered.pop().unwrap(),"zopro_quotient_wide"); assert_eq!(recovered.pop().unwrap(),"--cfg");
        let parent=pp["steps"][pi]["argv"].as_array().unwrap().iter().map(|s|s.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        assert_eq!(recovered,parent);
    }
    let manifest=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let mut field_unchanged=0;
    for (n,h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(format!("/home/ubuntu/src/zk-tools/zopro/{n}")),h.as_str().unwrap());
        if n.starts_with("crates/zopro-bn254/") && n!=rel {
            assert_eq!(hash(r.join("source").join(n)),h.as_str().unwrap()); field_unchanged+=1;
        }
    }
    assert_eq!(field_unchanged,17);
    let old_test=fs::read_to_string(format!("{P}/native-tests.rs")).unwrap();
    let prefix=old_test.split_once("\n#[test]\nfn full_width_native_quotient_products_match_ifma_and_wrapping_integer()").unwrap().0;
    assert_eq!(fs::read_to_string(r.join("native-tests.rs")).unwrap(),prefix.replace("0x71756f746d756c6c","0x7769646535327175")+"\n"+&fs::read_to_string(r.join("consumer-test.rs")).unwrap());
    for (file,key) in [("libzopro_bn254.rlib","ON"),("libzopro_bn254-generic.rlib","generic")] {
        assert_eq!(hash(r.join(file)),q["libraries"][key].as_str().unwrap());
    }
    let parentq=read(format!("{P}/qualification.json"));
    for (key,parentkey,file) in [("reused_original_OFF","OFF","libzopro_bn254-OFF.rlib"),("reused_masked_parent","ON","libzopro_bn254.rlib")] {
        assert_eq!(hash(format!("{P}/{file}")),q["libraries"][key].as_str().unwrap());
        assert_eq!(q["libraries"][key],parentq["libraries"][parentkey]);
    }
    let symbols=read(r.join("symbols.json")); let parent_symbols=read(format!("{P}/symbols.json"));
    let mut rows=vec![];
    for (i,s) in symbols.as_array().unwrap().iter().enumerate() {
        let name=s["name"].as_str().unwrap();
        let pi=parent_symbols.as_array().unwrap().iter().position(|p|p["name"]==s["name"]).unwrap();
        let nt=fs::read_to_string(r.join(format!("decode/decode-wide-wrapper-{i:02}/stdout.log"))).unwrap();
        let pt=fs::read_to_string(format!("{P}/decode/decode-new-quotient-wrapper-{pi:02}/stdout.log")).unwrap();
        let ifmas=opcode_count(&nt,"vpmadd52luq")+opcode_count(&nt,"vpmadd52huq");
        let mullo=opcode_count(&nt,"vpmullq"); let calls=opcode_count(&nt,"call");
        let ands=opcode_count(&nt,"vpandq"); let parent_ands=opcode_count(&pt,"vpandq");
        assert_eq!(ifmas,if name.ends_with("_mul"){95}else{145}); assert_eq!(mullo,5); assert_eq!(calls,0);
        assert_eq!(parent_ands-ands,if name.contains("_256_"){4}else{5});
        let qr=&q["actual_new_consumer_symbols"][i];
        assert_eq!(qr["symbol"]["name"],s["name"]); assert_eq!(qr["ifma_sites"],ifmas);
        assert_eq!(qr["vpandq_sites"],ands); assert_eq!(qr["parent_masked_vpandq_sites"],parent_ands);
        rows.push(json!({"name":name,"ifma":ifmas,"mullo":mullo,"calls":calls,"ands":ands,"parent_ands":parent_ands,"ANDs_removed":parent_ands-ands}));
    }
    assert_eq!(rows.len(),12);
    let t=fs::read_to_string(r.join("build/run-wide-native-oracles/stdout.log")).unwrap();
    for required in ["2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out", "NATIVE_FIELD_TOTAL outputs=554496 exports=554496", "WIDE_CONSUMERS lanes=164280 assertions=1478520 wrapped_consumers=49 mixed_counterexamples=8"] { assert!(t.contains(required)); }
    let mut members=BTreeMap::<String,String>::new();
    for n in ["design.md","prepare.rs","prepare-initial.rs","consumer-test.rs","native-tests.rs","wrappers.rs","codegen.rs","publish.rs","source-audit.json","qualification.json","build-plan.json","build-pins.json","codegen-plan.json","codegen-pins.json","decode-plan.json","decode-pins.json","symbols.json","report.md"] {
        members.insert(n.into(),fs::read_to_string(r.join(n)).unwrap());
    }
    // Include only the new source expression, never an unchanged private source tree.
    members.insert("changed-normal-quotient.txt".into(),a["after"].as_str().unwrap().to_owned());
    let mut receipts=0;
    for (root,plan) in [("build","build-plan.json"),("codegen-build","codegen-plan.json"),("decode","decode-plan.json")] {
        assert_eq!(read(r.join(root).join("result.json"))["status"],"complete");
        for n in ["protocol.json","result.json"] {
            let p=format!("{root}/{n}"); members.insert(p.clone(),fs::read_to_string(r.join(p)).unwrap());
        }
        for s in read(r.join(plan))["steps"].as_array().unwrap() {
            let name=s["name"].as_str().unwrap(); let dir=r.join(root).join(name);
            let exit=read(dir.join("exit.json")); assert_eq!(exit["success"],true); assert_eq!(exit["returncode"],0);
            for (f,k) in [("stdout.log","stdout_sha256"),("stderr.log","stderr_sha256")] { assert_eq!(hash(dir.join(f)),exit[k].as_str().unwrap()); }
            for f in ["command.json","exit.json","stdout.log","stderr.log"] {
                let p=format!("{root}/{name}/{f}"); members.insert(p.clone(),fs::read_to_string(r.join(p)).unwrap());
            }
            receipts+=1;
        }
    }
    assert_eq!(receipts,19);
    let review=json!({"passed":true,"same_agent_separate_parser":true,"pin_occurrences":pin_count,"successful_command_receipts":receipts,"new_native_tests":2,"producer_counts_match_separate_opcode_parser":rows,"all176_original_hashes_exact":true,"parent_and_accepted_source_recovery_exact":true,"other_field_files_exact":17,"compiler_flags_parent_exact_plus_wide_cfg":true,"test_oracle_prefix_unchanged_except_seed":true,"native_tests_or_decoders_replayed":false,"new_proofs":0,"new_timings":0,"goal_achieved":false});
    members.insert("publication-review.json".into(),serde_json::to_string_pretty(&review).unwrap());
    let archive=members.iter().map(|(n,s)|(n,json!({"utf8":s,"bytes":s.len(),"sha256":digest(s.as_bytes())}))).collect::<BTreeMap<_,_>>();
    let raw=serde_json::to_vec(&archive).unwrap(); fs::write(r.join("evidence.json"),&raw).unwrap();
    let gz=Command::new("/usr/bin/gzip").args(["-n","-c"]).arg(r.join("evidence.json")).output().unwrap(); assert!(gz.status.success());
    fs::write(r.join("evidence.json.gz"),&gz.stdout).unwrap();
    let un=Command::new("/usr/bin/gzip").arg("-cd").arg(r.join("evidence.json.gz")).output().unwrap(); assert!(un.status.success()); assert_eq!(un.stdout,raw);
    let parsed:Value=serde_json::from_slice(&un.stdout).unwrap();
    for (n,s) in &members { assert_eq!(parsed[n]["utf8"],*s); assert_eq!(parsed[n]["sha256"],digest(s.as_bytes())); }
    let index=json!({"members":members.iter().map(|(n,s)|(n,json!({"bytes":s.len(),"sha256":digest(s.as_bytes())}))).collect::<BTreeMap<_,_>>(),"gzip_bytes":gz.stdout.len(),"uncompressed_bytes":raw.len(),"roundtrip_verified":true});
    fs::create_dir_all(d.join("artifacts")).unwrap();
    fs::copy(r.join("report.md"),d.join("README.md")).unwrap(); let mut files=vec!["README.md".to_owned()];
    for n in ["design.md","source-audit.json","qualification.json","publish.rs","evidence.json.gz"] {
        let to=format!("artifacts/{n}"); fs::copy(r.join(n),d.join(&to)).unwrap(); assert_eq!(hash(r.join(n)),hash(d.join(&to))); files.push(to);
    }
    for (n,v) in [("publication-review.json",review),("archive-index.json",index)] {
        let to=format!("artifacts/{n}"); fs::write(d.join(&to),serde_json::to_vec_pretty(&v).unwrap()).unwrap(); files.push(to);
    }
    files.sort(); fs::write(d.join("SHA256SUMS"),files.iter().map(|n|format!("{}  {n}\n",hash(d.join(n)))).collect::<String>()).unwrap();
    println!("Publication review PASS: {receipts} commands, {} members, {} gzip bytes, {pin_count} pin occurrences; twelve exact mask reductions; no timing or adoption",members.len(),gz.stdout.len());
}
