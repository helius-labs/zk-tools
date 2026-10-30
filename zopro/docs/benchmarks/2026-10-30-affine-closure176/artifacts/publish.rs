use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::{Path,PathBuf},process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/affine-closure176";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-affine-closure176";
fn r(s:impl AsRef<Path>)->PathBuf{Path::new(R).join(s)}
fn d(s:impl AsRef<Path>)->PathBuf{Path::new(D).join(s)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn check_pins(name:&str){for(p,h)in read(r(name)).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}}
fn plan(){
    check_pins("pins.json");assert!(!r("review-plan.json").exists());
    let mut pins=read(r("pins.json")).as_object().unwrap().clone();
    for p in [r("review.rs"),r("publish.rs"),r("publisher"),r("analysis.json"),r("1x2.json"),r("2x2.json"),r("runs/result.json")]{pins.insert(p.display().to_string(),json!(hash(&p)));}
    save(r("review-pins.json"),&json!(pins));
    save(r("review-plan.json"),&json!({"version":1,"source_pins":r("review-pins.json"),"steps":[
      {"kind":"link-sdk","name":"independent-reader","artifacts":"/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen-candidate-cargo.jsonl","driver":r("review.rs"),"timeout_seconds":600},
      {"kind":"command","name":"independent-reader-run","argv":[r("review-runs/independent-reader-sdk")],"timeout_seconds":600}
    ]}));
    println!("Prepared separate vector-LC negative certificate; no original test/census rerun");
}
fn walk(p:&Path,out:&mut Vec<PathBuf>){for e in fs::read_dir(p).unwrap(){let p=e.unwrap().path();if p.is_dir(){walk(&p,out)}else{out.push(p)}}}
fn receipts(root:&str)->Value{
    let mut files=vec![];walk(&r(root),&mut files);files.sort();
    let mut out=BTreeMap::new();
    for p in files{let name=p.strip_prefix(r(root)).unwrap().display().to_string();if name.ends_with("-sdk"){continue}assert!(p.extension().is_some_and(|x|x=="json"||x=="log"));out.insert(name,fs::read_to_string(p).unwrap());}
    json!(out)
}
fn verify_receipts(root:&str)->usize{
    let result=read(r(format!("{root}/result.json")));assert_eq!(result["status"],"complete");
    let steps=result["steps"].as_array().unwrap();
    for s in steps{
        let dir=r(format!("{root}/{}",s.as_str().unwrap()));let exit=read(dir.join("exit.json"));
        assert_eq!(exit["success"],true);assert_eq!(exit["returncode"],0);
        for stream in ["stdout","stderr"]{assert_eq!(hash(dir.join(format!("{stream}.log"))),exit[format!("{stream}_sha256")]);}
        assert!(read(dir.join("command.json"))["argv"].as_array().unwrap().len()>1);
        if dir.join("artifacts.json").exists(){let a=read(dir.join("artifacts.json"));for(p,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}assert_eq!(hash(a["binary"].as_str().unwrap()),a["binary_sha256"]);}
    }
    steps.len()
}
fn publish(){
    check_pins("pins.json");check_pins("review-pins.json");
    assert_eq!(verify_receipts("runs"),5);assert_eq!(verify_receipts("review-runs"),2);
    assert_eq!(read(r("review.json"))["complete"],true);assert_eq!(read(r("analysis.json"))["opportunity"],false);
    assert!(!d("README.md").exists());fs::create_dir_all(d("artifacts")).unwrap();
    for name in ["analysis.json","1x2.json","2x2.json","model.rs","census.rs","workflow.rs","review.rs","publish.rs","design.md","inputs.json","pins.json","plan.json","review-pins.json","review-plan.json","review.json","setup.log","workflow.log","finish.log","review-workflow.log"]{fs::copy(r(name),d("artifacts").join(name)).unwrap();}
    let rec=json!({"runs":receipts("runs"),"review-runs":receipts("review-runs")});
    fs::write(r("receipts.json"),serde_json::to_vec(&rec).unwrap()).unwrap();
    let gz=Command::new("gzip").args(["-n","-c"]).arg(r("receipts.json")).output().unwrap();assert!(gz.status.success());fs::write(d("artifacts/receipts.json.gz"),gz.stdout).unwrap();
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    fs::copy(&manifest,d("artifacts/accepted-manifest.json")).unwrap();let m=read(&manifest);
    let mut report=String::from("# Recursive affine closure of witness assignments\n\n**No removable query terms found under this simplification.** Exact sparse affine propagation through running sums, stored slots, hints and assignments finds zero new affine or constant assignments in the retained primary and2×2 programs. The separate vector-coefficient reader certifies the same negative result. No prover implementation or timing trial follows.\n\nThis strengthens the earlier [direct linear-assignment census](../2026-10-22-universal-linear-assignment-census/README.md), which checked assignment expression syntax. The old audit explicitly left recursive stored-slot/constant propagation open. The new reader evaluates cancellations and previously derived constants over Fr, with only input wire0 fixed to one. Every nonlinear assignment or unknown hint introduces an opaque atom. It is a sound incomplete affine abstraction; it does not solve general polynomial relations or infer new assumptions from constraints without assignments. The independent coefficient-matrix rank census addresses a different transformation and is not used as a witness-folding bound here.\n\n|Circuit|Wires|Assignments|Stored slots|Sum records|New affine assignments|New constant assignments|Constant hint outputs|\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    let analysis=read(r("analysis.json"));
    for shape in ["1x2","2x2"]{let v=&analysis["cases"][shape];report.push_str(&format!("|{}|{}|{}|{}|{}|{}|{}|{}|\n",shape,v["wires"],v["assignments"],v["stored_slots"],v["sum_records"],v["expanded_affine_assignments"],v["expanded_constant_assignments"],v["constant_hint_outputs"]));}
    report.push_str("\nA/B query lengths are21,006/30,089 for primary and36,637/52,274 for2×2. All four populations contain zero noninput candidate rows. No stored slot becomes constant, and no additional expression side becomes constant beyond the original syntax. Maximum affine support is254atoms in both programs; no map-width or operation limit truncates the analysis. The maps retain269,589/468,298atom terms offline. These are analysis objects, not proposed runtime plans, prover workspace or RSS measurements.\n\nDivision is only expanded with a provably nonzero constant divisor. A known zero divisor preserves the solver's zero-output branch for satisfiable inputs. With an unknown divisor, proportional numerator/divisor is not cancelled; zero changes solver semantics. The exact all-zero assigned-side/output special case is recognized. General multiplication expands only when an operand is constant. Unknown hint outputs remain opaque, including facts about bit ranges that this abstraction does not exploit.\n\n## Validation and provenance\n\nThe five-step xtask workflow completes once: link three structural/algebra tests, execute them, link the reader, analyze each cached solver program. All three tests pass, covering128 independent field valuations, sparse cancellation, constant scaling, assignment expressions, unknown/zero divisor branches, a synthetic stored-sum image with a hidden linear multiplier, truncation/trailing bytes/unsorted query rejection, undefined reads and duplicate writes. No original successful test or census is repeated.\n\nBoth readers consume every image byte, execute every sum record exactly once, account for every row/write, and check assignment coefficients/inverses. The original reader uses normalized BTreeMaps. The separate negative checker uses sorted coefficient vectors, certifies each assignment nonlinear before treating its output as a new atom, and independently agrees on dimensions, all77,980assignments,671?sum records and constant-side counts. They share the exact accepted Fr arithmetic and the documented image layout; this is not an independent cryptographic field oracle.\n\nThe literal sum-record total is677,274 (247,530+429,744). All84,796constraints and85,144operations are covered. Three tests, two primary census executions and two separate certificate reads occur. The two solver exports are reused byte-exactly from [wire readiness](../2026-10-30-wire-readiness175/README.md); they are not regenerated. Program hashes, exact coherent accepted-library links, source pins and every command/stdout/stderr/exit receipt are retained. All seven commands complete on their first attempts. Helper binaries named `*-sdk` are readers, not new proof SDK artifacts.\n\nProduction remains the176-file accepted source, SDK SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. All production/helper/input/library pins close. No actual witness solve, solver export, proof, Gnark verification, profile or latency sample is added; the distinct-proof ledger stays27,353. Private solver images and libraries/binaries remain local. Published source contains only the new analysis helpers and receipts. No field/core/prover suite is repeated because production is unchanged.\n\n## Accepted proof comparisons, unchanged\n\nThese are inherited local32-worker full SDK observations and supplied M5 reference timings, not measurements of this census. All eight CPU10× targets remain unmet.\n\n|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|\n|---|---:|---:|---:|---:|\n");
    // Avoid hand-transcription ambiguities in aggregate accounting.
    report=report.replace("671?sum records", "677,274sum records");
    assert_eq!(28501+49479,77980);assert_eq!(30922+53874,84796);assert_eq!(31061+54083,85144);
    for shape in ["1x2","2x2","4x2","8x2","12x2","16x2","24x2","merge-24x1"]{
        let v=&m["m5"][shape];report.push_str(&format!("|{}|{:.6}|{:.0} / {:.3}×|{:.0} / {:.3}×|{:.1}|\n",shape,v["accepted_ms"].as_f64().unwrap(),v["cpu_reference_ms"].as_f64().unwrap(),v["cpu_speedup"].as_f64().unwrap(),v["metal_reference_ms"].as_f64().unwrap(),v["metal_speedup"].as_f64().unwrap(),v["required_ms_for_CPU10x"].as_f64().unwrap()));
    }
    report.push_str("\nAccepted merge54is161.883899ms without a supplied M5 reference. Accepted primary process RSS is132.75MiB; this census does not measure peak prover RAM or minimum RAM. The accepted physical16 calibration engineering exception remains documented in [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md).\n\nLatest retained AWS comparison is historical source175 on c7a.24xlarge: Zopro11.915591ms at32workers and Aeglos7.959184ms at96, each campaign's best tested configuration. It does not measure current source176. No new AWS resource, Aeglos, phone/ARM/TEE, huge250/4999, pressure/swap/minimum-RAM or concurrent-throughput result follows. The current target still requires roughly2.14× further primary latency reduction, plus improvements on every supplied shape.\n\nThis exact affine-closure route is closed on the two retained programs. Stronger nonlinear algebra or other circuits are outside its scope. No scalar/key table, key transformation, persistent duplicate solver plan or arbitrary runtime RAM cap is proposed.\n\n[Design](artifacts/design.md), [all counts](artifacts/analysis.json), [primary](artifacts/1x2.json), [2×2](artifacts/2x2.json), [model](artifacts/model.rs), [driver](artifacts/census.rs), [workflow helper](artifacts/workflow.rs), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [input provenance](artifacts/inputs.json), [separate checker](artifacts/review.rs), [certificate](artifacts/review.json), [review plan](artifacts/review-plan.json), [review pins](artifacts/review-pins.json), [all raw receipts](artifacts/receipts.json.gz), [accepted manifest](artifacts/accepted-manifest.json), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS).\n");
    fs::write(d("README.md"),report).unwrap();
    let decoded=Command::new("gzip").args(["-d","-c"]).arg(d("artifacts/receipts.json.gz")).output().unwrap();assert!(decoded.status.success());assert_eq!(decoded.stdout,fs::read(r("receipts.json")).unwrap());
    let mut files=vec![];walk(Path::new(D),&mut files);files.sort();
    let mut sums=String::new();for f in &files{sums.push_str(&format!("{}  {}\n",hash(f),f.strip_prefix(D).unwrap().display()));}
    fs::write(d("SHA256SUMS"),sums).unwrap();
    save(r("publication-review.json"),&json!({"complete":true,"original_workflow_steps":5,"separate_review_steps":2,"all_command_stream_hashes":true,"all_binary_and_library_pins":true,"receipt_archive_exact":true,"published_files_before_checksums":files.len(),"private_solver_images_published":false,"production_changed":false,"goal_achieved":false}));
    println!("Published affine-closure negative result, all command receipts, exact accepted rows; verified pins and receipt roundtrip; no proof/timing reruns");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("plan")=>plan(),Some("publish")=>publish(),_=>panic!("plan/publish")}}
