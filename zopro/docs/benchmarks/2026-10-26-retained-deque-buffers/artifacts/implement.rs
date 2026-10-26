use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap(); let mut h = Sha256::new(); let mut b = [0; 65536];
    loop { let n = f.read(&mut b).unwrap(); if n == 0 { break; } h.update(&b[..n]); }
    format!("{:x}", h.finalize())
}
fn replace(s: &mut String, old: &str, new: &str, count: usize) {
    assert_eq!(s.matches(old).count(), count, "unexpected patch anchor {old}");
    *s = s.replace(old, new);
}
fn main() {
    let root = Path::new("/tmp/zopro-retained-deque-buffers");
    let overlay = root.join("overlay"); assert!(!overlay.exists()); fs::create_dir_all(overlay.join("vendor")).unwrap();
    let manifest: Value = serde_json::from_slice(&fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap()).unwrap();
    let repo = Path::new("/home/ubuntu/src/zk-tools/zopro");
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() { assert_eq!(hash(repo.join(p)), h.as_str().unwrap()); }
    fs::copy("/tmp/zopro-grouping-module/production-source-manifest.json", root.join("accepted-manifest.json")).unwrap();
    let cache = Path::new("/home/ubuntu/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f");
    let mut pins = BTreeMap::new();
    for (name, dst) in [("crossbeam-deque-0.8.8", overlay.join("vendor")), ("rayon-core-1.13.0", root.join("upstream"))] {
        fs::create_dir_all(&dst).unwrap(); let archive = cache.join(format!("{name}.crate"));
        assert!(Command::new("tar").arg("-xf").arg(&archive).arg("-C").arg(&dst).status().unwrap().success());
        pins.insert(archive.to_str().unwrap().to_owned(), hash(&archive));
    }
    let vendor = overlay.join("vendor/crossbeam-deque-0.8.8"); let path = vendor.join("src/deque.rs");
    let original = fs::read_to_string(&path).unwrap(); fs::write(root.join("upstream-deque.rs"), &original).unwrap();
    let mut source = original.clone();
    replace(&mut source, "    buffer: CachePadded<Atomic<Buffer<T>>>,\n}", r#"    buffer: CachePadded<Atomic<Buffer<T>>>,

    // Immutable mode. Only the !Sync Worker owner changes retired_buffers.
    // Stealers never access the vector; their Arc keeps its allocations alive.
    #[cfg(zopro_retained_deque_buffers)]
    retained: bool,
    #[cfg(zopro_retained_deque_buffers)]
    retired_buffers: UnsafeCell<Vec<Atomic<Buffer<T>>>>,
    #[cfg(all(zopro_retained_deque_buffers, zopro_deque_trace))]
    trace_id: usize,
}"#, 1);
    replace(&mut source, "            buffer: CachePadded::new(Atomic::new(buffer)),", r#"            buffer: CachePadded::new(Atomic::new(buffer)),
            #[cfg(zopro_retained_deque_buffers)]
            retained: retained_buffers_enabled(),
            #[cfg(zopro_retained_deque_buffers)]
            retired_buffers: UnsafeCell::new(Vec::new()),
            #[cfg(all(zopro_retained_deque_buffers, zopro_deque_trace))]
            trace_id: TRACE_QUEUE_ID.fetch_add(1, Ordering::Relaxed),"#, 2);
    replace(&mut source, "        Worker {\n            inner,", "        #[cfg(all(zopro_retained_deque_buffers, zopro_deque_trace))]\n        inner.trace(\"new\", buffer.cap);\n\n        Worker {\n            inner,", 2);
    replace(&mut source, "            // Free the memory allocated by the buffer.\n            buffer.into_owned().into_box().dealloc();", r#"            #[cfg(all(zopro_retained_deque_buffers, zopro_deque_trace))]
            self.trace("drop", buffer.deref().cap);

            // Arc has no remaining owner/stealer: all descriptor references are gone.
            // Pending payloads were dropped above, only in the current buffer.
            #[cfg(zopro_retained_deque_buffers)]
            for retired in self.retired_buffers.get_mut().drain(..) {
                // Atomic -> Owned is deferred until here, never during a live steal.
                // Old arrays contain stale copies, so deallocate MaybeUninit storage
                // without running T::drop on any copied entry.
                retired.into_owned().into_box().dealloc();
            }

            // Free the memory allocated by the buffer.
            buffer.into_owned().into_box().dealloc();"#, 1);
    replace(&mut source, "        // Destroy the old buffer later.\n        guard.defer_unchecked", r#"        #[cfg(zopro_retained_deque_buffers)]
        if self.inner.retained {
            // SAFETY: Worker is !Sync, so only this owner mutates the vector.
            // Atomic::from preserves a shared pointer, without claiming exclusive
            // access while a stealer might still reference the old descriptor.
            (*self.inner.retired_buffers.get()).push(Atomic::from(old));
            #[cfg(zopro_deque_trace)]
            self.inner.trace("resize", new_cap);
            return;
        }
        #[cfg(all(zopro_retained_deque_buffers, zopro_deque_trace))]
        self.inner.trace("resize", new_cap);

        // Destroy the old buffer later.
        guard.defer_unchecked"#, 1);
    replace(&mut source, "if buffer.cap > MIN_CAP && len <= buffer.cap as WorkerIndex / 4 {", "if buffer.cap > MIN_CAP && len <= buffer.cap as WorkerIndex / 4\n                        && self.shrink_allowed()\n                    {", 1);
    replace(&mut source, "if buffer.cap > MIN_CAP && len < buffer.cap as WorkerIndex / 4 {", "if buffer.cap > MIN_CAP && len < buffer.cap as WorkerIndex / 4\n                            && self.shrink_allowed()\n                        {", 1);
    // The original body is compiled unchanged when the private cfg is absent.
    let old_guard = r#"        // A SeqCst fence is needed here.
        //
        // If the current thread is already pinned (reentrantly), we must manually issue the
        // fence. Otherwise, the following pinning will issue the fence anyway, so we don't
        // have to.
        if epoch::is_pinned() {
            atomic::fence(Ordering::SeqCst);
        }

        let guard = &epoch::pin();"#;
    assert_eq!(source.matches(old_guard).count(), 3);
    let mut replacements = Vec::new();
    for condition in ["self.inner.retained", "self.inner.retained && dest.inner.retained", "self.inner.retained && dest.inner.retained"] {
        let new_guard = format!(r#"        #[cfg(zopro_retained_deque_buffers)]
        let pinned;
        #[cfg(zopro_retained_deque_buffers)]
        let guard = if {condition} {{
            // Pinning normally supplies this required fence. Retention removes
            // reclamation bookkeeping, never the queue's ordering requirement.
            atomic::fence(Ordering::SeqCst);
            // SAFETY: every shared worker buffer accessed by this operation
            // remains owned until the final Inner Arc is gone. &self (and dest)
            // keep that Arc alive throughout this operation. No pointer escapes.
            unsafe {{ epoch::unprotected() }}
        }} else {{
            if epoch::is_pinned() {{
                atomic::fence(Ordering::SeqCst);
            }}
            pinned = epoch::pin();
            &pinned
        }};

        #[cfg(not(zopro_retained_deque_buffers))]
        let guard = {{
{old_guard}
            guard
        }};"#);
        replacements.push(new_guard);
    }
    let mut position = 0;
    for replacement in &replacements {
        let i = source[position..].find(old_guard).unwrap() + position;
        source.replace_range(i..i + old_guard.len(), replacement); position = i + replacement.len();
    }
    source.push_str(r#"

#[cfg(zopro_retained_deque_buffers)]
fn retained_buffers_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("ZOPRO_CPU_RETAINED_DEQUE_BUFFERS").is_ok_and(|v| v == "1"))
}

impl<T> Worker<T> {
    #[inline]
    fn shrink_allowed(&self) -> bool {
        #[cfg(zopro_retained_deque_buffers)]
        { !self.inner.retained }
        #[cfg(not(zopro_retained_deque_buffers))]
        { true }
    }
}

#[cfg(all(zopro_retained_deque_buffers, zopro_deque_trace))]
static TRACE_QUEUE_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(all(zopro_retained_deque_buffers, zopro_deque_trace))]
impl<T> Inner<T> {
    // Only constructor/owner-resize/final-drop call this diagnostic. It is never
    // compiled into the latency executable and adds no per-probe writes.
    fn trace(&self, event: &str, current: usize) {
        let retired = unsafe { &*self.retired_buffers.get() };
        let retired_capacity: usize = retired.iter().map(|p| unsafe {
            p.load(Ordering::Relaxed, epoch::unprotected()).deref().cap
        }).sum();
        eprintln!("ZOPRO_DEQUE_BUFFER queue={} event={} retained={} current={} retired={} generations={} vector_capacity={} payload_size={} descriptor_size={} handle_size={} inner_size={}",
            self.trace_id, event, self.retained as u8, current, retired_capacity,
            retired.len(), retired.capacity(), mem::size_of::<T>(),
            mem::size_of::<Buffer<T>>(), mem::size_of::<Atomic<Buffer<T>>>(),
            mem::size_of::<CachePadded<Inner<T>>>());
    }
}

#[cfg(all(test, zopro_retained_deque_buffers))]
mod retained_tests;
"#);
    // Allocation accounting is compiled only into the unit-test executable.
    replace(&mut source, "        Buffer { ptr, cap }", "        #[cfg(all(test, zopro_retained_deque_buffers))]\n        retained_tests::allocation(ptr as usize, true);\n        Buffer { ptr, cap }", 1);
    replace(&mut source, "    unsafe fn dealloc(self) {\n", "    unsafe fn dealloc(self) {\n        #[cfg(all(test, zopro_retained_deque_buffers))]\n        retained_tests::allocation(self.ptr as usize, false);\n", 1);
    fs::write(&path, &source).unwrap();
    fs::write(root.join("patch-anchors.json"), serde_json::to_vec_pretty(&json!({"original_sha256":hash(root.join("upstream-deque.rs")),"patched_sha256":hash(&path),"guard_original":old_guard,"guard_replacements":replacements})).unwrap()).unwrap();
    let cargo = vendor.join("Cargo.toml"); let mut text = fs::read_to_string(&cargo).unwrap();
    text.push_str("\n# Private test workspace; runtime versions match the frozen prover lock.\n[workspace]\n\n[dev-dependencies.crossbeam-epoch]\nversion = \"=0.9.21\"\ndefault-features = false\n\n[dev-dependencies.crossbeam-utils]\nversion = \"=0.8.23\"\ndefault-features = false\n");
    fs::write(&cargo, text).unwrap();
    let mut top = fs::read_to_string(repo.join("Cargo.toml")).unwrap();
    top.push_str("\n[patch.crates-io]\ncrossbeam-deque = { path = \"vendor/crossbeam-deque-0.8.8\" }\n");
    fs::write(overlay.join("Cargo.toml"), top).unwrap();
    let lock = fs::read_to_string(repo.join("Cargo.lock")).unwrap(); let mut blocks = lock.split("[[package]]").map(str::to_owned).collect::<Vec<_>>(); assert_eq!(blocks.len(),220);
    let deque = blocks.iter_mut().find(|s| s.starts_with("\nname = \"crossbeam-deque\"\n")).unwrap();
    let before = deque.clone(); *deque = deque.lines().filter(|l| !l.starts_with("source = ") && !l.starts_with("checksum = ")).collect::<Vec<_>>().join("\n"); deque.push('\n');
    let after = deque.clone();
    let patched = blocks.join("[[package]]"); fs::write(overlay.join("Cargo.lock"), &patched).unwrap();
    for (a,b) in lock.split("[[package]]").zip(patched.split("[[package]]")) { if a != b { assert_eq!(a,before); assert_eq!(b,after); } }
    fs::write(root.join("lock-audit.json"), serde_json::to_vec_pretty(&json!({"packages":219,"unchanged_package_entries":218,"changed_entry":"crossbeam-deque0.8.8 only registry source/checksum removed for private path","before":before,"after":after,"baseline_sha256":hash(repo.join("Cargo.lock")),"candidate_sha256":hash(overlay.join("Cargo.lock"))})).unwrap()).unwrap();
    for p in ["crates/zopro-bench/examples/cpu-sdk.rs", "crates/zopro-bench/examples/support/perf.rs"] { fs::create_dir_all(overlay.join(p).parent().unwrap()).unwrap(); fs::copy(Path::new("/tmp/zopro-serial-fft-leaves/candidate-v2").join(p),overlay.join(p)).unwrap(); }
    fs::write(root.join("upstream-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
    println!("Declared private overlay: original219 runtime packages, one source path substitution, original steal/CAS operations and fences preserved");
}
