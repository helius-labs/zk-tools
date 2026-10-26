use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::Path};
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn add(p:impl AsRef<Path>,pins:&mut BTreeMap<String,String>){let p=p.as_ref();pins.insert(p.to_str().unwrap().into(),hash(p));}
fn tree(p:&Path,pins:&mut BTreeMap<String,String>){for e in fs::read_dir(p).unwrap(){let p=e.unwrap().path();if p.file_name().unwrap()=="target"{continue}if p.is_dir(){tree(&p,pins)}else{add(&p,pins)}}}
fn save(p:impl AsRef<Path>,v:Value){fs::write(p,format!("{}\n",serde_json::to_string_pretty(&v).unwrap())).unwrap()}
fn main(){let root=Path::new("/tmp/zopro-retained-deque-buffers");
let old=fs::read_to_string("/tmp/zopro-serial-fft-leaves/native-plan-v2.json").unwrap();let replaced=old.replace("/tmp/zopro-serial-fft-leaves/candidate-v2","/tmp/zopro-retained-deque-buffers/candidate").replace("/tmp/zopro-serial-fft-leaves/target-native-v2","/tmp/zopro-retained-deque-buffers/target-native").replace("/tmp/zopro-serial-fft-leaves/native-v2","/tmp/zopro-retained-deque-buffers/native").replace("zopro_serial_fft_leaves","zopro_retained_deque_buffers").replace("ZOPRO_CPU_SERIAL_FFT_LEAVES","ZOPRO_CPU_RETAINED_DEQUE_BUFFERS");
let mut plan:Value=serde_json::from_str(&replaced).unwrap();plan["source_pins"]=json!(root.join("build-input-pins.json"));
for s in plan["steps"].as_array_mut().unwrap(){if s["name"]=="core-off"||s["name"]=="core-on"{s.as_object_mut().unwrap().remove("required_tests");}}
let mut steps=vec![plan["steps"][0].clone()];
let new_tests=["deque::retained_tests::constructors_sample_explicit_process_mode","deque::retained_tests::geometric_storage_bound_and_no_shrink_across_reused_bursts","deque::retained_tests::owner_can_drop_before_stealers_without_reclaiming_any_generation","deque::retained_tests::final_drop_drops_only_live_current_payloads_once","deque::retained_tests::every_mixed_mode_flavor_batch_and_limit_preserves_unique_payloads","deque::retained_tests::concurrent_growth_owner_pop_and_steal_are_exactly_once","deque::retained_tests::retained_probes_preserve_nested_epoch_state_and_batch_destination_growth"];
for mode in ["off","on"] {for (name,path,required) in [("deque",root.join("candidate/vendor/crossbeam-deque-0.8.8/Cargo.toml"),json!(new_tests)),("rayon",root.join("upstream/rayon-core-1.13.0/Cargo.toml"),json!(["nested_bursts_fifo_reuse_and_pool_drop_are_exactly_once","cross_pool_entry_panic_cleanup_and_reuse_complete"]))]{steps.push(json!({"kind":"command","name":format!("{name}-{mode}"),"argv":["cargo","test","--manifest-path",path.to_str().unwrap(),"--offline","--locked","--release","--lib","--tests","--","--test-threads=1"],"environment":{"ZOPRO_CPU_RETAINED_DEQUE_BUFFERS":if mode=="on"{"1"}else{"0"},"CARGO_TARGET_DIR":root.join("target-upstream")},"timeout_seconds":1200,"unfiltered":true,"required_tests":required}));}}
steps.extend(plan["steps"].as_array().unwrap().iter().skip(1).cloned());plan["steps"]=json!(steps);
let mut pins:BTreeMap<String,String>=serde_json::from_value(read(root.join("candidate/.xtask/source-pins.json"))).unwrap();
tree(&root.join("upstream"),&mut pins);for name in ["model/Cargo.toml","model/Cargo.lock","model/src/lib.rs","experiment.json","implement.rs","prepare-build.rs","prepare.json","upstream-deque.rs","upstream-pins.json","lock-audit.json","patch-anchors.json"]{add(root.join(name),&mut pins)}
for(p,h)in read(root.join("upstream-pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pins.insert(p.into(),h.as_str().unwrap().into());}
add("/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask",&mut pins);
save(root.join("build-input-pins.json"),json!(pins));save(root.join("native-plan.json"),plan);println!("FROZEN native/upstream build inputs {}",pins.len());
}
