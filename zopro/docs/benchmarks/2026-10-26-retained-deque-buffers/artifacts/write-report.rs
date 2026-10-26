use serde_json::Value;
use std::{fs,path::Path,fmt::Write};
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn n(v:&Value,k:&str)->f64{v[k].as_f64().unwrap()}
fn pct(v:&Value,k:&str)->f64{assert!(v.is_object());v.get(k).map(|n|n.as_f64().unwrap()).unwrap_or(0.)}
fn main(){let root=Path::new("/tmp/zopro-retained-deque-buffers");let data=read(root.join("pilot-result.json"));let closed=read(root.join("closeout.json"));let report=Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-retained-deque-buffers");fs::create_dir_all(report).unwrap();let mut out=String::from("# Retained worker-deque buffers\n\n");
if data["performance_gate_passed"]==true{out.push_str("The fixed local speed screen passes. The candidate is preserved for resource, portable, concurrent SDK, large-circuit and target-host qualification before integration. Production is unchanged. The all-eight tenfold CPU-reference goal remains unmet.\n\n");}else{out.push_str("The fixed local speed screen fails. The candidate is not adopted and no unchanged timing rerun follows. All samples and failures are retained; production is unchanged. The all-eight tenfold CPU-reference goal remains unmet.\n\n");}
let primary=&data["cases"]["1x2-w32"]["metrics"];writeln!(out,"Primary ON is {:.3} ms versus accepted {:.3} ms. Summed CPU work falls from {:.3} to {:.3} ms/request, while voluntary switches rise from {:.1} to {:.1}/request. The original-eight paired geometric result is {:.2}% slower against accepted. Reducing the epoch category did not improve complete-SDK latency.\n",n(&primary["on"],"median_ms"),n(&primary["accepted"],"median_ms"),n(&primary["accepted"],"cpu_ms_per_request"),n(&primary["on"],"cpu_ms_per_request"),n(&primary["accepted"],"voluntary_switches_per_request"),n(&primary["on"],"voluntary_switches_per_request"),(n(&data["original8_aggregate"]["accepted"],"paired_median")-1.)*100.).unwrap();
out.push_str(r#"The private Crossbeam-deque 0.8.8 change keeps superseded worker-buffer
descriptors and arrays alive until the final worker/stealer `Arc` is dropped.
Retained queues only grow: the sum of old array capacities is below the current
capacity, so all arrays together occupy less than twice the high-water current
array storage. Descriptor and vector metadata are additional. The pool retains
that high-water allocation until destruction, a real memory-policy tradeoff.
There is no proving-key lookup table or additional copied point array.

Private cfg `zopro_retained_deque_buffers` and explicit
`ZOPRO_CPU_RETAINED_DEQUE_BUFFERS=1`, sampled once per process, select ON.
Absent/zero select OFF. Mode is immutable for each queue. Retained probes use
an unprotected guard while the shared queue handle keeps every old buffer alive.
The required SeqCst fence remains before the back-index load. Batch operations
pin normally if either source or destination uses ordinary epoch retirement.
Resizing still pins; only final shared-owner destruction converts retired
pointer handles to exclusive ownership and deallocates their arrays.

The source audit recovers byte-identical upstream single/batch extraction,
CAS and ordering bodies after removing the guard-selection block. Push and
reserve are unchanged; pop differs only at its two shrink-admission conditions.
The original generic slot-access protocol is preserved. Rayon scheduling,
injector behavior, arithmetic and proving-key preparation are unchanged.
This differs from the previously tested empty-victim precheck, bounded victim
search, batched epoch guards and St3 queue replacements.

## Fixed complete-SDK comparison

Ryzen 9950X; seven rotated/reversed process blocks compare frozen accepted,
the new executable OFF, and the same executable ON. All nine shapes use
32 workers on CPUs 0–31, 31 measured requests after three primes. The primary
16-worker diagnostic uses CPUs 0–15, one logical CPU per physical core, with
101 measured requests. Counts, order and gates were frozen before measurement.

Timed calls include fresh solving, randomness, proving, internal verification
and serialization. Loading and priming are excluded. All unprofiled latency
finishes before sampling. No owned build, other benchmark, profiler or bulk
audit overlaps latency. Times are medians of process medians; paired ratios
are medians of within-block ON/control ratios, and need not equal the ratio
of the displayed medians.

| Case | Accepted ms | OFF ms | ON ms | ON/accepted paired | ON/OFF paired |
|---|---:|---:|---:|---:|---:|
"#);
for name in ["1x2-w32","2x2-w32","4x2-w32","8x2-w32","12x2-w32","16x2-w32","24x2-w32","merge-24x1-w32","merge-54x1-w32","1x2-w16"]{let c=&data["cases"][name];writeln!(out,"| {name} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} |",n(&c["metrics"]["accepted"],"median_ms"),n(&c["metrics"]["off"],"median_ms"),n(&c["metrics"]["on"],"median_ms"),n(&c["on_over_control"]["accepted"],"paired_median"),n(&c["on_over_control"]["off"],"paired_median")).unwrap();}
writeln!(out,"\nOriginal-eight paired geometric aggregate: ON/accepted **{:.6}**, ON/OFF **{:.6}**.",n(&data["original8_aggregate"]["accepted"],"paired_median"),n(&data["original8_aggregate"]["off"],"paired_median")).unwrap();
out.push_str("\nThe declared gate requires primary and original-eight aggregate improvement against both controls, at least four of seven primary block wins, every case paired ratio ≤1.02, every block ratio ≤1.05, and OFF/accepted calibration within a factor of 1.03. There is no minimum improvement floor.\n\n");
if data["performance_gate_passed"]==true{out.push_str("All declared local speed checks pass.\n\n");}else{let calibration_failures=data["cases"].as_object().unwrap().values().filter(|c|n(c,"off_calibration_factor")>1.03).count();writeln!(out,"There are {} failed primary, aggregate, case or block checks; the complete list is in [pilot-result.json](artifacts/pilot-result.json). OFF calibration fails for {} cases. The observed speed failure is retained without exclusions or reruns.\n",data["failed_checks"].as_array().unwrap().len(),calibration_failures).unwrap();}
out.push_str("## CPU work and process memory\n\nCPU milliseconds are summed user+system time per measured request from unprofiled runs, shown as medians across processes. RSS is the maximum whole-process high-water observation, including loading and primes; it is not a minimum RAM requirement or a proof-buffer-only peak.\n\n| Case | Variant | CPU ms/request | Maximum RSS MiB | Voluntary switches/request |\n|---|---|---:|---:|---:|\n");
for name in ["1x2-w32","12x2-w32","merge-54x1-w32","1x2-w16"]{for variant in ["accepted","off","on"]{let m=&data["cases"][name]["metrics"][variant];writeln!(out,"| {name} | {variant} | {:.3} | {:.2} | {:.1} |",n(m,"cpu_ms_per_request"),n(m,"maxrss_mib"),n(m,"voluntary_switches_per_request")).unwrap();}}
out.push_str("\n## Separate CPU profiles\n\nThe nine profiles follow all latency runs, use the monotonic clock and 999 Hz sampling, and cover primary, 12x2 and merge54 for all three variants. Percentages are exclusive sampled CPU-period shares with symbol/inlining limits, not removable wall-time fractions. Profiled CPU times are separate observations.\n\n| Case | Variant | CPU ms/request | Epoch % | Scheduler % | FFT % | Voluntary switches/request |\n|---|---|---:|---:|---:|---:|---:|\n");
let rows=read(root.join("perf/rows.json"));for r in rows.as_array().unwrap().iter().filter(|r|r["phase"]=="profile"){let summary=&r["summary"];let rounds=n(summary,"rounds");let share=&r["profile"]["category_percent"];writeln!(out,"| {} | {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.1} |",r["case"].as_str().unwrap(),r["variant"].as_str().unwrap(),(n(summary,"user_cpu_ms")+n(summary,"system_cpu_ms"))/rounds,pct(share,"epoch"),pct(share,"scheduler"),pct(share,"fft"),n(summary,"voluntary_switches")/rounds).unwrap();}
writeln!(out,"\n{} samples; zero lost records; exactly one sampled PID per capture. Direct and expanded symbol streams and their hashes are preserved. Zero category percentage means no attributed samples. The required fence remains in the steal body: the linked native ON branch lowers it to a locked OR on stack storage before loading the back index, as preserved in [disassembly](artifacts/steal-disassembly.txt). Some attribution can therefore move from epoch helpers into scheduler frames; sample-category removal does not establish eliminated synchronization cost.\n",closed["profile_samples"]).unwrap();
out.push_str("## Worker-buffer capacity observations\n\nAfter performance measurement, the same frozen source is rebuilt with a separate trace cfg. Constructor/resize/final-drop events record actual generic payload size and queue capacity; no trace code is compiled into the latency executable. All nine shapes run OFF and ON at 32 workers, four complete SDK requests each including primes, with independent exported-proof verification.\n\nThe table shows ON only. Bytes sum per-queue observed maxima, which may occur at different times. Descriptor bytes include current/retired descriptors and retired-vector capacity. They exclude pointed-to jobs, fixed Inner/Arc/thread storage and all prover allocations. OFF traces omit deferred epoch allocations and cannot provide a physical-memory baseline. Tracing changes scheduling, so these observations do not establish a universal or uninstrumented peak.\n\n| Shape | Queues | Largest final capacity | Retired capacity sum | Observed array bytes | Descriptor/vector bytes | Growth events |\n|---|---:|---:|---:|---:|---:|---:|\n");
let trace=read(root.join("trace-result.json"));for c in trace["cases"].as_array().unwrap().iter().filter(|c|c["mode"]=="on"){writeln!(out,"| {} | {} | {} | {} | {} | {} | {} |",c["shape"].as_str().unwrap(),c["queues"],c["max_last_capacity"],c["retired_capacity_sum"],c["summed_per_queue_max_array_bytes"],c["summed_per_queue_max_descriptor_and_vector_bytes"],c["growth_events"]).unwrap();}
out.push_str("\nEvery traced run constructs 64 worker deques: 32 ordinary queues and 32 broadcast queues in the pinned Rayon registry. None grows beyond its initial 64 entries. Observed array storage is 64 KiB in total, with no retired generations. This is runtime queue storage rather than a proving-key cache.\n\nThe geometric bound is checked at every retained event. Unit tests additionally force repeated 1,024-entry bursts and multiple retained generations, verify array deallocation after the last stealer drops, and check that pending payloads are dropped once. These observations do not replace long-lived-pool, low-memory, concurrency or very-large-circuit qualification.\n\n## Supplied M5 references\n\nThese are the supplied historical CPU/Metal timings, not fresh matched M5 runs. Merge54 has no supplied reference.\n\n| Shape | CPU reference ms | Metal reference ms | ON ms | CPU/ON | Metal/ON |\n|---|---:|---:|---:|---:|---:|\n");
for c in data["m5_supplied_references"].as_array().unwrap(){writeln!(out,"| {} | {:.0} | {:.0} | {:.6} | {:.2}× | {:.2}× |",c["shape"].as_str().unwrap(),n(c,"cpu_reference_ms"),n(c,"metal_reference_ms"),n(c,"on_ms"),n(c,"on_cpu_speedup"),n(c,"on_metal_speedup")).unwrap();}
out.push_str(r#"
## Validation and provenance

The private runtime lock retains all 219 package entries, changing only
Crossbeam-deque to a private path. The other 218 entries remain byte-identical.
Upstream tests use separate development locks with matching deque/epoch/utils
runtime versions. Actual registry source trees, build scripts and manifests
for deque/epoch/utils/Rayon-core are byte-checked against pristine crate archives.
The directly linked libraries and actual scheduler/collector library files
are pinned along with the complete SDK executable.

Both modes pass 45 Crossbeam tests (38 upstream plus seven added lifetime,
growth, destruction, mixed-batch and concurrency tests), 112 Rayon tests
(two upstream tests remain ignored), and 109 native core tests. Raw/cubic
prover tests pass 2/6. Field kernels are unchanged; inherited 33-test field
qualification is not represented as a new run. The two instrumented Loom
tests cover retention/publication/final shared-handle destruction, including
a premature-reclamation negative control. They do not verify extraction,
the original generic slot protocol or the complete scheduler.

Initial compilation failures are retained: the private deque additions needed
explicit std Vec/macro imports in the upstream no_std crate, and the model
needed an explicit borrow through a raw pointer. Separate v2 snapshots fix
those imports/borrows and register private cfg names without changing the
queue algorithm. No SDK timing preceded those fixes.

The initial closeout helper expected a 256-byte raw-coordinate proof. The SDK
exports the 164-byte compressed Gnark representation documented by
`Proof::to_gnark`; the audit was corrected and rerun without rerunning any SDK
measurement. The initial audit source and failing log are retained.

Preflight checks all nine shapes OFF/ON at four workers before performance.
Fixture public-input hashes, proof uniqueness and freshness are mandatory
before timing and independently audited again on closeout. Every exported
proof is checked with the pinned Gnark verifier. Exact commands, source and
binary hashes, test/exit/stdout/stderr receipts, all samples, proof bytes and
profiles are preserved. No AWS resource is created by this local experiment;
there is no new Aeglos or C7a measurement here.

No portable, whole-SDK concurrent, memory-pressure or very-large-circuit
qualification is claimed. Production remains unchanged pending qualification.

"#);
writeln!(out,"Total: **{} complete SDK requests, {} distinct independently Gnark-verified exports, {} proof/public members**; {} captures. These counts include preflight, performance and separate capacity tracing, and exclude two fixed source-fixture members. Performance uses {} pinned inputs and rejects {} prior proof identities before preflight.\n",closed["sdk_requests"],closed["distinct_gnark_exports"],closed["proof_public_members"],closed["profiles"],closed["performance_pins"],closed["prior_identities_before_preflight"]).unwrap();
out.push_str("Start with [declaration](artifacts/experiment.json), [source audit](artifacts/source-audit.json), [full speed checks](artifacts/pilot-result.json), [capacity observations](artifacts/trace-result.json), [independent closeout](artifacts/closeout.json), and [artifact index](artifacts/artifact-index.json). Nine raw perf captures and nine expanded streams are kept locally with hashes and omitted from Git; executable/compiler caches are represented by exact build/link hashes.\n");fs::write(report.join("README.md"),out).unwrap();println!("REPORT_WRITTEN {}",report.join("README.md").display());}
