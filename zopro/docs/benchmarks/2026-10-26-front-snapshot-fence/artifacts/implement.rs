use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::Path,process::Command};
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn replace(s:&mut String,old:&str,new:&str,count:usize){assert_eq!(s.matches(old).count(),count,"patch anchor {old}");*s=s.replace(old,new);}
fn main(){let root=Path::new("/tmp/zopro-front-snapshot-fence");let overlay=root.join("overlay");assert!(!overlay.exists());fs::create_dir_all(overlay.join("vendor")).unwrap();let repo=Path::new("/home/ubuntu/src/zk-tools/zopro");let manifest:Value=serde_json::from_slice(&fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap()).unwrap();assert_eq!(manifest["source_files_sha256"].as_object().unwrap().len(),170);for(p,h)in manifest["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(repo.join(p)),h.as_str().unwrap());}fs::copy("/tmp/zopro-grouping-module/production-source-manifest.json",root.join("accepted-manifest.json")).unwrap();
let lock=fs::read_to_string(repo.join("Cargo.lock")).unwrap();let cache=Path::new("/home/ubuntu/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f");let mut pins=BTreeMap::new();for name in ["crossbeam-deque-0.8.8","rayon-core-1.13.0"]{let p=cache.join(format!("{name}.crate"));let digest=hash(&p);assert!(lock.contains(&format!("checksum = \"{digest}\"")));assert!(Command::new("tar").arg("-xf").arg(&p).arg("-C").arg(overlay.join("vendor")).status().unwrap().success());pins.insert(p.to_str().unwrap().to_owned(),digest);}
let deque=overlay.join("vendor/crossbeam-deque-0.8.8");let path=deque.join("src/deque.rs");let original=fs::read_to_string(&path).unwrap();fs::write(root.join("upstream-deque.rs"),&original).unwrap();let start=original.find("    pub fn steal(&self) -> Steal<T> {").unwrap();let end=original[start..].find("\n    }\n").unwrap()+start+7;let single=&original[start..end];let extract=&single[single.find("        // Load the back index.").unwrap()..];
let added=r#"

#[cfg(zopro_front_snapshot_fence)]
const FRONT_SNAPSHOT_LIMIT: usize = 8;

#[cfg(zopro_front_snapshot_fence)]
impl<T> Stealer<T> {
    /// Tries at most eight queues in order, returning at most one task.
    ///
    /// A normal epoch guard protects every buffer. All front snapshots precede
    /// one explicit SeqCst fence; each back load and extraction follows it.
    /// A raced probe contributes Retry only if no later queue supplies work.
    /// No task, queue pointer, snapshot or guard is retained after return.
    pub fn steal_one_from_front_snapshots(stealers: &[&Self]) -> Steal<T> {
        assert!(stealers.len() <= FRONT_SNAPSHOT_LIMIT);
        if stealers.is_empty() { return Steal::Empty; }
        let guard = &epoch::pin();
        let mut fronts: [WorkerIndex; FRONT_SNAPSHOT_LIMIT] = [0; FRONT_SNAPSHOT_LIMIT];
        for (index, stealer) in stealers.iter().enumerate() {
            fronts[index] = stealer.inner.front.load(Ordering::Acquire);
        }
        // The guard's pin barrier precedes snapshots. This explicit common
        // fence is required AFTER every front load and BEFORE every back load.
        atomic::fence(Ordering::SeqCst);
        Self::snapshot_from_fronts(stealers, &fronts[..stealers.len()], guard)
    }

    #[inline]
    fn snapshot_from_fronts(stealers: &[&Self], fronts: &[WorkerIndex], guard: &epoch::Guard) -> Steal<T> {
        debug_assert_eq!(stealers.len(), fronts.len());
        let mut retry = false;
        for (stealer, front) in stealers.iter().zip(fronts) {
            match stealer.snapshot_extract(*front, guard) {
                Steal::Success(task) => return Steal::Success(task),
                Steal::Retry => retry = true,
                Steal::Empty => {}
            }
        }
        if retry { Steal::Retry } else { Steal::Empty }
    }

    // The only runtime caller has pinned and fenced after every saved front.
    // The back/read/pointer-check/CAS body is copied byte-for-byte from steal.
    #[inline]
    fn snapshot_extract(&self, f: WorkerIndex, guard: &epoch::Guard) -> Steal<T> {
"#;
fs::write(&path,format!("{original}{added}{extract}}}\n\n#[cfg(all(test, zopro_front_snapshot_fence))]\nmod front_snapshot_tests;\n")).unwrap();
let rayon=overlay.join("vendor/rayon-core-1.13.0");let registry=rayon.join("src/registry.rs");let before=fs::read_to_string(&registry).unwrap();fs::write(root.join("upstream-registry.rs"),&before).unwrap();let mut text=before.clone();replace(&mut text,"    terminate_count: AtomicUsize,\n}","    terminate_count: AtomicUsize,\n    #[cfg(zopro_front_snapshot_fence)]\n    front_snapshot_fence: bool,\n}",1);replace(&mut text,"            terminate_count: AtomicUsize::new(1),","            terminate_count: AtomicUsize::new(1),\n            #[cfg(zopro_front_snapshot_fence)]\n            front_snapshot_fence: std::env::var(\"ZOPRO_CPU_FRONT_SNAPSHOT_FENCE\").is_ok_and(|v| v == \"1\"),",1);replace(&mut text,"    fn steal(&self) -> Option<JobRef> {\n","    fn steal(&self) -> Option<JobRef> {\n        #[cfg(zopro_front_snapshot_fence)]\n        if self.registry.front_snapshot_fence { return self.steal_front_snapshot_tiles(); }\n",1);text.push_str("\n#[cfg(zopro_front_snapshot_fence)]\nmod front_snapshot;\n");fs::write(&registry,text).unwrap();fs::create_dir_all(rayon.join("src/registry")).unwrap();
fs::write(rayon.join("src/registry/front_snapshot.rs"),r#"//! Preserve the first scalar probe, then share a fence across bounded tiles.
use super::*;

impl WorkerThread {
    pub(super) fn steal_front_snapshot_tiles(&self) -> Option<JobRef> {
        debug_assert!(self.local_deque_is_empty());
        let thread_infos = &self.registry.thread_infos.as_slice();
        let num_threads = thread_infos.len();
        if num_threads <= 1 { return None; }
        loop {
            let mut retry = false;
            let start = self.rng.next_usize(num_threads);
            let mut victims = (start..num_threads).chain(0..start).filter(|&i| i != self.index);
            // Keep the common successful-first-victim path as a scalar probe.
            if let Some(index) = victims.next() {
                match thread_infos[index].stealer.steal() {
                    Steal::Success(job) => return Some(job),
                    Steal::Retry => retry = true,
                    Steal::Empty => {}
                }
            }
            while let Some(index) = victims.next() {
                let mut tile = [&thread_infos[index].stealer; 8];
                let mut len = 1;
                while len < tile.len() {
                    match victims.next() {
                        Some(next) => { tile[len] = &thread_infos[next].stealer; len += 1; }
                        None => break,
                    }
                }
                match Stealer::steal_one_from_front_snapshots(&tile[..len]) {
                    Steal::Success(job) => return Some(job),
                    Steal::Retry => retry = true,
                    Steal::Empty => {}
                }
            }
            // Exactly as the original scan: a race causes another complete
            // pass with a fresh random start, rather than declaring idle.
            if !retry { return None; }
        }
    }
}
"#).unwrap();
let p=deque.join("Cargo.toml");let mut text=fs::read_to_string(&p).unwrap();replace(&mut text,"    \"cfg(crossbeam_loom)\",","    \"cfg(crossbeam_loom)\",\n    \"cfg(zopro_front_snapshot_fence)\",",1);text.push_str("\n[workspace]\n[dev-dependencies.crossbeam-epoch]\nversion = \"=0.9.21\"\ndefault-features = false\n[dev-dependencies.crossbeam-utils]\nversion = \"=0.8.23\"\ndefault-features = false\n");fs::write(p,text).unwrap();
let p=rayon.join("Cargo.toml");let mut text=fs::read_to_string(&p).unwrap();assert!(!text.contains("[lints.rust.unexpected_cfgs]"));text.push_str("\n[workspace]\n[patch.crates-io]\ncrossbeam-deque = { path = \"../crossbeam-deque-0.8.8\" }\n[dev-dependencies.crossbeam-epoch]\nversion = \"=0.9.21\"\ndefault-features = false\n[dev-dependencies.crossbeam-utils]\nversion = \"=0.8.23\"\ndefault-features = false\n[lints.rust.unexpected_cfgs]\nlevel = \"warn\"\ncheck-cfg = [\"cfg(zopro_front_snapshot_fence)\"]\n[[test]]\nname = \"front_snapshot\"\npath = \"tests/front_snapshot.rs\"\n");fs::write(p,text).unwrap();
let mut cargo=fs::read_to_string(repo.join("Cargo.toml")).unwrap();cargo.push_str("\n[patch.crates-io]\ncrossbeam-deque = { path = \"vendor/crossbeam-deque-0.8.8\" }\nrayon-core = { path = \"vendor/rayon-core-1.13.0\" }\n");fs::write(overlay.join("Cargo.toml"),cargo).unwrap();
let mut blocks=lock.split("[[package]]").map(str::to_owned).collect::<Vec<_>>();assert_eq!(blocks.len(),220);let mut changed=BTreeMap::new();for name in ["crossbeam-deque","rayon-core"]{let entry=blocks.iter_mut().find(|s|s.starts_with(&format!("\nname = \"{name}\"\n"))).unwrap();let before=entry.clone();*entry=entry.lines().filter(|l|!l.starts_with("source = ")&&!l.starts_with("checksum = ")).collect::<Vec<_>>().join("\n");entry.push('\n');changed.insert(name,json!({"before":before,"after":entry}));}let patched=blocks.join("[[package]]");for(a,b)in lock.split("[[package]]").zip(patched.split("[[package]]")){if a!=b{assert!(changed.values().any(|v|v["before"]==a&&v["after"]==b));}}fs::write(overlay.join("Cargo.lock"),&patched).unwrap();fs::write(root.join("lock-audit.json"),serde_json::to_vec_pretty(&json!({"packages":219,"unchanged_entries":217,"changed":changed,"baseline_sha256":hash(repo.join("Cargo.lock")),"candidate_sha256":hash(overlay.join("Cargo.lock"))})).unwrap()).unwrap();
for p in ["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{fs::create_dir_all(overlay.join(p).parent().unwrap()).unwrap();fs::copy(Path::new("/tmp/zopro-retained-deque-buffers/candidate-v2").join(p),overlay.join(p)).unwrap();}fs::write(root.join("upstream-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();println!("Private overlay prepared: unmodified original deque prefix; common-fence API; original OFF scheduler body and first scalar probe retained;217/219 runtime lock entries identical");}
