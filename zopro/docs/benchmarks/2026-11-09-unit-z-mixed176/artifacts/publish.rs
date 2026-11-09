use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::{Path,PathBuf},process::Command,fmt::Write as _};
const R:&str="/home/ubuntu/src/zopro-perf-work/unit-z-mixed176";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-09-unit-z-mixed176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn walk(p:&Path,out:&mut Vec<PathBuf>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();if e.file_type().unwrap().is_dir(){walk(&e.path(),out);}else if e.file_type().unwrap().is_file(){out.push(e.path());}}}
fn main(){
    assert!(!Path::new(D).exists());
    let review=read(format!("{R}/review-result.json"));assert_eq!(review["passed"],true);assert_eq!(review["speed_measured"],false);
    for(p,h)in read(format!("{R}/pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let manifest="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-09-busy-reset-adoption176/artifacts/production-source-manifest.json";
    let m=read(manifest);assert_eq!(m["adopted"],true);assert_eq!(m["proof_ledger"],41792);
    let ledger=m["proof_ledger_path"].as_str().unwrap();assert_eq!(hash(ledger),m["proof_ledger_sha256"]);
    let next=r#"# Continue unit-Z specialization with actual-proof branch census

Goal ACTIVE/UNMET: verified single-proof CPU AVX >=10x supplied M5 CPU ALL8, report CPU+Metal. Previous turn PROGRESS: a new safe Rust standard unit-Z mixed-add implementation, native/generic independent curve qualification, actual linked codegen review, factual report/commit/push. No speed/SDK/core integration claim yet. Keep full goal; don't redefine success around this component.

Root /home/ubuntu/src/zopro-perf-work/unit-z-mixed176. Read design.md,kernel.rs,qualification.rs,review-result.json,publication.json. Native5-command workflow complete terminal0;session88758. Native2tests passed and generic1passed. Exactnative19320/Gnark0 andgeneric11520independentArkworks pointoutputs;native2576/generic1536changing-chaintransitions;genericmainargumentpopulationreusesnative;53actualbroadcastworkers1/4/16/32 requested512KiB;4OS callers share8workerpool(8requested,notall8provenactive). All176productionfiles unchanged. Originalfield1200/native+originalgeneric reused;no old field/core/nativeWindow/proof/decoder replay. No failed build/test/model/reader. One unusedFieldimport warning retained,no cleanup rebuild.

Kernel afteroriginalidentityguards testsleft.z==BaseONE. OtherZ callsacceptedadd_affine unchanged. UnitZ uses standardEFDmmadd2007bl4M2S instead of7M4S;equalXdouble/cancel usesoriginalguards. Allnative/generic checkedoutputs matchaccepted rawJacobiancoordinates and independentArkworks affine points;inputimmutable. SafeRust,noheapexpression/unsafeblock/table/keycopy/plan/pool/forecastchange. This is private standaloneprototype,not integratedproduction. Emittedactualkernel G1/G2 bodies have192/512mulxsites vs352/928acceptedadd_affine;firsttwoexportwrapperscallthekernel. Equalpointdoubleandnonunitfallback remainoutofline;nonunitfallback repeatsidentityguards andpaysnewprologue/branch. Static countsnotretiredinstructions/cycles/stackpeak/speed;actualproofeligibilityunknown. No IFMA stream rewrite. Sourcefunction knownunit mightalreadybeoptimized atsomeothercallsites;don'ttransferglobal savings.

NEXT required: establish actualunit-Z mixed-addbranch frequencies in complete SDK proof workload beforeperformance selection. A diagnostic-only instrumented originalfield can count add_affine calls afteridentityguards,unit/nonunit,curve,ordinary/equal/opposite;no clocks or native kernel replay. Buildcoherentdiagnostic core/prover/zolana/SDK againstnewfieldwithcurrentadoptedbitmapreset flags/source;don'treuseoldcoreswithwrongfieldidentity. ReusebaselineRustbuildrecipes carefully;acceptednativeREF isbusy-reset-sdk176/sdk-ON,source manifestbelow. Countmultiple originalshapes+merge54,notonlyfavorableprimary. Preserve originaloperation bodies,diagnosticonly counters,andplan beforefreshproofs;canonical41792ledger. Useonerequestpercaseafter3primeswithcountersresetafterprimes;counterboundarycouldbedriverorpublicdiagnosticfieldAPIbutnoglobalatomicsintimedcandidate. No productmemorycap/newpool/table. Ifeligiblecountis negligible, closeasuseful-workscreenwithoutmeaninglessbenchmark. Otherwise integratespecialization privately intoactualProjective::add_affine(defaultprivatecfg),eliminatewrapper-inducedduplicateguards byusingthealreadyexistingidentityguards,qualifytheACTUALintegratedartifact/ranges/generic/core/concurrency before prospective coherent SDK all8+54controls/automaticprofiles. This is integration into the existing method,not a post-clock rescue: no clock yet. Do not claim component staticcount as proofgain.

Acceptedproduction176manifest /home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-09-busy-reset-adoption176/artifacts/production-source-manifest.json. AcceptedSDK /home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON;corebusy-reset176/libs/ON;originalfield1200. Baseline manualrustflags include --cfg zopro_dense_busy_reset andoriginalg2prefix/carry flags. Cargo emitsbitmapcfgautomaticallyfornativeIFMA. OldG2vector-sign176manifest sourcehasheshistorical after3filebitmapadoption. Do not assertoldhashesequalcurrent. Do not buildfuturecontrolfromold6d83orworker-placement-controlsdk.

Canonical41792 unchanged /home/ubuntu/src/zopro-perf-work/busy-reset-compat176/proof-sha256-after.json. Thisstage0SDKcalls/0newproofs/0newlatencyprofiles. No livejobs/cloud. Acceptedall8CPUms7.809033907/10.940974419/15.750369548/23.951790484/33.857141452/39.726525548/65.984807806/65.660587968;M5CPU37/59/109/189/284/335/528/507;Metal24/30/48/74/101/124/176/175;CPU10xtargets3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7,0/8. Primarywholeprocesspeak132.50MiB,merge54CPU163.167792ms. Already sub8before;historical7.699867differentcampaign/statistic. No AWS/Aeglos/mobile/ARM/huge/swap/pressure/minRAM/concurrentthroughputclaim.

Autoresearch:concretecurrentcode/profile/closedhistory->newboundedusefulwork/dependencyhypothesis->Rustimplementation->independentcurve/resource/generic/concurrencyqualification->freezecompletecoherentSDKall8+54controls/automaticprofiles/readersBEFOREANYpreflight->serializedRust1.98.1xtask->ONLYpoll exactlivehandle throughoutANYpreflight/recordinguntilterminal(timeout/truncationnotterminal)->allrecordingsend->verifyallobservations->politefactualreport,commitANDpush,exactremoteHEADverify/checkpoint->continuefullgoal. Noquestions/Python/subagents/largepersistentLUT/keyplan/newpool/artificialRAMcap/CASM. Do not repeatunchangedclosedendpoint/queue/tree/fieldrecipes;smallpositivegainsmatterbutdon'tchangefullgoal. No successfuloldtest/build/recording/decoder rerunforbookkeeping.
"#;
    fs::write(format!("{R}/next.md"),next).unwrap();
    fs::create_dir_all(format!("{D}/artifacts")).unwrap();
    for f in["design.md","kernel.rs","qualification.rs","prepare.rs","review.rs","publish.rs","plan.json","pins.json","review-result.json","next.md"]{fs::copy(format!("{R}/{f}"),format!("{D}/artifacts/{f}")).unwrap();}
    let mut report=r#"# Unit-Z mixed-add qualification

**Correctness and emitted-code checks pass; performance is unmeasured and production is unchanged.** A private Rust prototype specializes mixed addition when the left Jacobian point has Z=1. It uses the standard [Bernstein–Lange mmadd-2007-bl formula](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html), costing 4 base-field multiplications and 2 squares instead of the general method's 7 multiplications and 4 squares. Equal points, opposite points, identity and nonunit-Z inputs retain the accepted complete behavior. This is a standard formula specialization, not a mathematical novelty or a proof-speed result.

Both curves pass the new independent Arkworks oracles. Outputs also match the accepted operation's **exact Jacobian coordinates**, and arguments remain unchanged. Inputs include rejection-sampled full-width scalars and field scales, signed points, identity, doubling, cancellation, unit/nonunit scales, unusual identity representations and successive mixed-add transitions.

|Execution|Independent affine output checks|Additional changing-chain transitions|Tests|
|---|---:|---:|---:|
|Native, main + workers/concurrent callers|19,320|2,576|2|
|Generic x86, no ASM / no IFMA|11,520|1,536|1|

The generic main population repeats the native main arguments through a different original field backend; these are execution counts, not wholly distinct mathematical fixtures. Native worker coverage broadcasts to all 53 workers across 1/4/16/32 pools with requested 512KiB stacks. Four OS callers also share one eight-worker Rayon pool. Eight is the pool configuration, not evidence that every worker executed a concurrent call. These are component correctness checks, not SDK throughput or physical ARM/phone qualification.

The actual native qualification executable is decoded once. Both private entrypoints call their retained generic kernels; the nonunit fallback and equal-point double remain out of line. The inspected kernel bodies contain the specialized arithmetic:

|Curve|Unit-Z kernel MULX sites|Accepted general mixed-add MULX sites|
|---|---:|---:|
|G1|192|352|
|G2|512|928|

These are static instruction sites in different function bodies, not executed instructions, cycles, stack peaks or proof speed. The unit-Z wrapper adds a prologue and checks, and its nonunit fallback repeats the accepted identity guards. A production integration would use the guards already inside the existing method and must qualify its actual artifact. The present decode does not establish a general improvement at every callsite or eliminate compiler/inlining effects. No IFMA stream arithmetic is changed.

Five serialized Rust/xtask commands pass on first execution: native build/test, generic build/test and actual native decode. The review verifies 190 pins, all five command arguments/exits/stream hashes, test summaries, unchanged 176 accepted production files, kernel guards and the selected symbols/counts. Helper compiler/review/publisher invocations have tool-transcript evidence rather than complete xtask receipts. One unused-import warning is retained; no successful build or test is repeated for cleanup. The review is a separate reader by the same agent, not an external audit.

There is no heap allocation expression or unsafe block in the kernel, and no new table, key/point copy, solver plan, pool, workspace forecast or memory cap. That source observation does not prove stack, RSS or minimum-RAM bounds. The accepted field libraries and existing field/core/Window/proof qualification are reused; no old suite is replayed. No coherent core or SDK integration exists yet.

Next, count unit-Z eligibility in actual complete proofs across shapes before deciding whether a timing campaign is useful. Main SIMD affine streams bypass this scalar operation, so this specialization cannot be projected onto their profile shares. Earlier endpoint, queue-drain, bucket-tree and normalization experiments remain separate results. If eligible work is material, integrate into the original method and qualify that artifact before freezing matched complete-proof controls and automatic profiles.

Zero new SDK requests, Gnark exports, profiles or latency observations are added. The canonical independently verified proof ledger remains **41,792**. No AWS/Aeglos, huge-circuit, ARM/mobile/TEE, pressure/swap/minimum-RAM or concurrent-throughput comparison is added. There is no measured speedup from this prototype.

The following observations are inherited from the [adopted bitmap-reset campaign](../2026-11-09-busy-reset-adoption176/README.md), not results of this kernel. M5 CPU and Metal timings are supplied historical references on different hardware.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10x target ms|
|---|---:|---:|---:|---:|
"#.to_string();
    for row in m["original8_CPU10x_rows"].as_array().unwrap(){let f=|k:&str|row[k].as_f64().unwrap();writeln!(report,"|{}|{:.6}|{:.0} / {:.3}x|{:.0} / {:.3}x|{:.1}|",row["shape"].as_str().unwrap(),f("CPU_ms"),f("M5_CPU_ms"),f("CPU_speedup"),f("M5_Metal_ms"),f("Metal_speedup"),f("CPU_10x_target_ms")).unwrap();}
    report+="\nThe full verified CPU10x objective remains active and unmet, **0/8**. Accepted primary peak process RSS is 132.50MiB; merge54 remains 163.167792ms without a supplied M5 reference. The baseline was already below 8ms; historical 7.699867ms used a different campaign/statistic.\n\nThe first publisher stopped while validating its checksum link before creating the checksum file. The initial publisher source and partial publication are retained; adding the missing placeholder fixes publication ordering. All qualification, test, review and decode results are reused unchanged.\n\n[Design](artifacts/design.md), [kernel](artifacts/kernel.rs), [independent curve tests](artifacts/qualification.rs), [review and symbol counts](artifacts/review-result.json), [continuation](artifacts/next.md), [source and command receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Executables/libraries, unchanged source snapshots, full raw decode and all key/witness/proof payloads are omitted. Selected disassembly is in the archive; the full local decode has a hash and size in the index.\n";
    fs::write(format!("{D}/README.md"),&report).unwrap();
    let pack=PathBuf::from(format!("{R}/publication-source"));fs::create_dir(&pack).unwrap();
    let mut files=Vec::new();for f in["design.md","kernel.rs","qualification.rs","prepare.rs","review.rs","publish.rs","publish-before-checksum-placeholder.rs","plan.json","pins.json","review-result.json","selected-disassembly.txt","next.md"]{files.push(PathBuf::from(format!("{R}/{f}")));}walk(Path::new(&format!("{R}/run")),&mut files);files.sort();
    let mut members=Vec::new();let mut local=Vec::new();
    for p in files{let rel=p.strip_prefix(R).unwrap();let size=fs::metadata(&p).unwrap().len();if p==Path::new(&format!("{R}/run/decode-native-actual-entrypoints/stdout.log")){local.push(json!({"path":p,"bytes":size,"sha256":hash(&p)}));continue}let bytes=fs::read(&p).unwrap();assert!(std::str::from_utf8(&bytes).is_ok());let dest=pack.join(rel);fs::create_dir_all(dest.parent().unwrap()).unwrap();fs::write(&dest,&bytes).unwrap();members.push(json!({"path":rel,"bytes":size,"sha256":hash(&dest)}));}
    let archive=format!("{D}/sources-receipts.tar.gz");assert!(Command::new("tar").args(["-czf",&archive,"-C",pack.to_str().unwrap(),"."]).status().unwrap().success());
    let unpack=PathBuf::from(format!("{R}/publication-roundtrip"));fs::create_dir(&unpack).unwrap();assert!(Command::new("tar").args(["-xzf",&archive,"-C",unpack.to_str().unwrap()]).status().unwrap().success());
    for v in &members{assert_eq!(hash(unpack.join(v["path"].as_str().unwrap())),v["sha256"]);}
    fs::write(format!("{D}/artifacts/archive-index.json"),serde_json::to_vec_pretty(&json!({"archive_sha256":hash(archive),"members":members,"local_only":local,"canonical_proofs":41792,"proof_ledger_sha256":m["proof_ledger_sha256"]})).unwrap()).unwrap();
    fs::write(format!("{D}/SHA256SUMS"),"").unwrap();
    let mut links=0;for p in report.split("](").skip(1){let link=p.split(')').next().unwrap();if !link.starts_with("http"){assert!(Path::new(D).join(link).exists(),"{link}");links+=1;}}
    let publication=json!({"passed":true,"archive_members":members.len(),"archive_roundtrip":true,"local_links":links,"production_unchanged":true,"new_curve_kernel_qualified":true,"performance_unmeasured":true,"adopted":false,"SDK_requests":0,"new_Gnark_exports":0,"canonical_proofs":41792,"goal_active":true,"goal_complete":false,"goal_targets_met":0,"all8_CPU_Metal_rows":8});
    fs::write(format!("{D}/artifacts/publication.json"),serde_json::to_vec_pretty(&publication).unwrap()).unwrap();
    fs::write(format!("{R}/publication.json"),serde_json::to_vec_pretty(&publication).unwrap()).unwrap();
    let mut files=Vec::new();walk(Path::new(D),&mut files);files.sort();let mut sums=String::new();for p in files{if p.file_name().unwrap()=="SHA256SUMS"{continue}writeln!(sums,"{}  {}",hash(&p),p.strip_prefix(D).unwrap().display()).unwrap();}fs::write(format!("{D}/SHA256SUMS"),&sums).unwrap();
    for l in sums.lines(){let(h,p)=l.split_once("  ").unwrap();assert_eq!(hash(Path::new(D).join(p)),h);}
    println!("UNIT_Z_PUBLICATION_PASS{}members/{}links;newnative+genericqualification/codegen;productionunchanged;canonical41792",members.len(),links);
}
