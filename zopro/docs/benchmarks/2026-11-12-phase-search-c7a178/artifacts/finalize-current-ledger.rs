//! Join the frozen C7a proof branch with subsequent local verified exports.
//! This does not generate proofs, execute verifiers, or change the frozen image.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, io::Write, path::{Path, PathBuf}};
const TARGET: &str = "/home/ubuntu/src/zopro-perf-work/phase-search-c7a178";
const SEED_HASH: &str = "22edc7fdac8573e74fd2d476a7af32bc694f4c1ecb5c3ae3b24bde6940a0ff1b";
type Ledger = BTreeSet<String>;
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).expect("read input")).expect("JSON input") }
fn hash(p: impl AsRef<Path>) -> String { format!("{:x}", Sha256::digest(fs::read(p).expect("hash input"))) }
fn parse(v: &Value) -> Result<Ledger, &'static str> {
    let values = v.as_array().ok_or("ledger must be an array")?;
    let mut out = Ledger::new();
    for value in values {
        let s = value.as_str().ok_or("ledger item must be text")?;
        if s.len()!=64 || !s.bytes().all(|b|b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) { return Err("invalid SHA256"); }
        if !out.insert(s.to_owned()) { return Err("duplicate ledger entry"); }
    }
    Ok(out)
}
fn merge(seed: &Ledger, current: &Ledger, target: &Ledger, transported: &Ledger, expected: usize) -> Result<Ledger, &'static str> {
    if !seed.is_subset(current) || !seed.is_subset(target) { return Err("missing common ancestor"); }
    let added = target.difference(seed).cloned().collect::<Ledger>();
    if added.len()!=expected || &added!=transported { return Err("target delta differs from verified transport"); }
    if !added.is_disjoint(current) { return Err("target export already appears in canonical ledger"); }
    Ok(current.union(&added).cloned().collect())
}
fn check_pins(path: &Path) {
    for (p,h) in read(path).as_object().expect("pin map") { assert_eq!(hash(p), h.as_str().unwrap(), "pin {p}"); }
}
fn verify_transport(root: &Path, run: &Path) -> Ledger {
    check_pins(&root.join("transport-pins.json"));
    let plan = read(root.join("transport-plan.json"));
    assert_eq!(plan["source_pins"], root.join("transport-pins.json").display().to_string());
    let steps = plan["steps"].as_array().unwrap();
    assert_eq!(steps.len(),124);
    let result = read(run.join("result.json"));
    assert_eq!(result["status"],"complete");
    assert_eq!(result["steps"],json!(steps.iter().map(|s|s["name"].as_str().unwrap()).collect::<Vec<_>>()));
    let mut proofs = Ledger::new();
    for step in steps {
        let name = step["name"].as_str().unwrap();
        assert!(!name.contains('/') && name!="." && name!="..");
        let dir = run.join(name);
        let exit = read(dir.join("exit.json"));
        assert_eq!(exit["success"],true); assert_eq!(exit["returncode"],0);
        for stream in ["stdout","stderr"] { assert_eq!(hash(dir.join(format!("{stream}.log"))),exit[format!("{stream}_sha256")]); }
        let command = read(dir.join("command.json"));
        assert_eq!(&command["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());
        let argv = step["argv"].as_array().unwrap(); assert_eq!(argv.len(),6);
        assert_eq!(argv[0],root.join("context/interval-verify").display().to_string());
        let image = argv[1].as_str().unwrap(); let proof = argv[3].as_str().unwrap(); let public = argv[4].as_str().unwrap();
        assert!(Path::new(image).starts_with(root.join("context/basis")));
        assert!(Path::new(proof).starts_with(root.join("remote-results/output")));
        assert_eq!(Path::new(proof).parent(),Path::new(public).parent());
        assert_eq!(hash(image),argv[2]); assert_eq!(hash(public),argv[5]);
        assert_eq!(fs::metadata(proof).unwrap().len(),164);
        let ph = hash(proof); assert!(proofs.insert(ph.clone()),"duplicate transported proof");
        let stdout = fs::read_to_string(dir.join("stdout.log")).unwrap();
        let verified = stdout.lines().filter(|l|l.starts_with("VERIFIED ")).collect::<Vec<_>>();
        assert_eq!(verified.len(),1);
        for field in [format!("proof_sha256={ph}"),format!("public_sha256={}",argv[5].as_str().unwrap()),format!("image_sha256={}",argv[2].as_str().unwrap())] { assert!(verified[0].split_whitespace().any(|s|s==field)); }
    }
    proofs
}
fn write_new(path: &Path, value: &Value) {
    let bytes=serde_json::to_vec_pretty(value).unwrap();
    let mut file=fs::OpenOptions::new().write(true).create_new(true).open(path).expect("new output only");
    file.write_all(&bytes).unwrap(); file.sync_all().unwrap();
}
fn main() {
    let args=std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(),5,"usage: finalize CURRENT_LEDGER CURRENT_SHA256 LOCAL_TRANSPORT_RUN NEW_OUTPUT_DIRECTORY");
    let root=Path::new(TARGET); let current_path=Path::new(&args[1]); let run=Path::new(&args[3]); let output=PathBuf::from(&args[4]);
    assert!(!output.exists(),"preserve prior outputs");
    check_pins(&root.join("prepared-pins.json"));
    let seed_path=root.join("proof-sha256-after-preflight.json");
    assert_eq!(hash(&seed_path),SEED_HASH); assert_eq!(hash(current_path),args[2]);
    let seed=parse(&read(&seed_path)).unwrap(); assert_eq!(seed.len(),55028);
    let current=parse(&read(current_path)).unwrap();
    let floor=Path::new("/home/ubuntu/src/zopro-perf-work/accepted-scaling179/proof-sha256-after-stat.json");
    assert_eq!(hash(floor),"946f2b40315004bed6c4fe61f84b8d5f70fff27bb0c206924e58db0a055207f2");
    assert!(parse(&read(floor)).unwrap().is_subset(&current),"canonical input omits already verified local exports");
    let measure=read(root.join("remote-review.json")); assert_eq!(measure["passed"],true); assert_eq!(measure["canonical_count"],55148);
    assert_eq!(hash(root.join("proof-sha256-after-capture.json")),measure["canonical_sha256"]);
    let profile_root=root.join("remote-results/output/profile");
    let diagnostic=read(profile_root.join("diagnostic-analysis.json")); assert_eq!(diagnostic["canonical_count"],55152);
    let target_path=profile_root.join("proof-sha256-after-diagnostic.json"); assert_eq!(hash(&target_path),diagnostic["canonical_sha256"]);
    let target=parse(&read(&target_path)).unwrap(); assert_eq!(target.len(),55152);
    let transported=verify_transport(root,run);
    let joined=merge(&seed,&current,&target,&transported,124).unwrap();
    // Check the mutable input again before writing any output.
    assert_eq!(hash(current_path),args[2]);
    fs::create_dir(&output).unwrap();
    let ledger_path=output.join("proof-sha256.json");write_new(&ledger_path,&json!(joined));
    let receipt=json!({"passed":true,"common_seed_count":seed.len(),"current_count":current.len(),"current_path":current_path,"current_sha256":args[2],"target_branch_count":target.len(),"target_branch_sha256":hash(&target_path),"transported_verified_exports":transported.len(),"transport_plan_sha256":hash(root.join("transport-plan.json")),"transport_result_sha256":hash(run.join("result.json")),"canonical_count":joined.len(),"canonical_sha256":hash(&ledger_path),"profile_quality_passed":diagnostic["quality_passed"],"target_performance_passed":read(root.join("target-analysis.json"))["passed"],"adoption_authorized":false});
    write_new(&output.join("review.json"),&receipt);println!("{receipt}");
}
#[cfg(test)] mod tests {
    use super::*;
    fn set(xs:&[u64])->Ledger{xs.iter().map(|x|format!("{x:064x}")).collect()}
    #[test] fn divergent_history_preserves_both_branches(){assert_eq!(merge(&set(&[1,2]),&set(&[1,2,3,4]),&set(&[1,2,5,6]),&set(&[5,6]),2).unwrap(),set(&[1,2,3,4,5,6]));}
    #[test] fn target_may_not_reuse_a_later_local_export(){assert_eq!(merge(&set(&[1]),&set(&[1,2]),&set(&[1,2,3]),&set(&[2,3]),2),Err("target export already appears in canonical ledger"));}
    #[test] fn missing_ancestor_rejected_in_either_branch(){for(current,target)in[(set(&[2]),set(&[1,3])),(set(&[1,2]),set(&[3]))]{assert_eq!(merge(&set(&[1]),&current,&target,&set(&[3]),1),Err("missing common ancestor"));}}
    #[test] fn verifier_set_and_count_must_equal_delta(){for(exports,n)in[(set(&[3]),2),(set(&[2]),1),(set(&[3,4]),1)]{assert_eq!(merge(&set(&[1]),&set(&[1,2]),&set(&[1,3]),&exports,n),Err("target delta differs from verified transport"));}}
    #[test] fn malformed_or_duplicate_input_cannot_be_silently_deduplicated(){let h=format!("{:064x}",1);assert!(parse(&json!([h,h])).is_err());for v in[json!(["bad"]),json!(["G".repeat(64)]),json!([3]),json!({})]{assert!(parse(&v).is_err());}assert_eq!(parse(&json!([format!("{:064x}",1)])).unwrap(),set(&[1]));}
}
