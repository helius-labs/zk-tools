use serde_json::Value;
use std::{fmt::Write,fs,path::Path};
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn n(v:&Value,k:&str)->f64{v[k].as_f64().unwrap()}
fn pct(v:&Value,k:&str)->f64{assert!(v.is_object());v.get(k).map(|n|n.as_f64().unwrap()).unwrap_or(0.)}
fn main(){let root=Path::new("/tmp/zopro-front-snapshot-fence");let data=read(root.join("pilot-result.json"));let closed=read(root.join("closeout.json"));let report=Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-front-snapshot-fence");fs::create_dir_all(report).unwrap();let passed=data["performance_gate_passed"].as_bool().unwrap();let mut out=String::from("# One common fence after bounded victim-front snapshots\n\n");
out.push_str(if passed{"The fixed local speed screen passes. The candidate is preserved for resource, portable, concurrent SDK, very-large-circuit and target-host qualification before production integration.\n\n"}else{"The fixed local speed screen fails. The candidate is not adopted and no unchanged timing rerun follows. All samples and failures are preserved.\n\n"});
let primary=&data["cases"]["1x2-w32"]["metrics"];writeln!(out,"Primary accepted/OFF/ON is **{:.6}/{:.6}/{:.6} ms**. Summed CPU time is **{:.3}/{:.3}/{:.3} ms/request**. Original-eight paired geometric ON/accepted is **{:.6}**, ON/OFF **{:.6}**. Production remains unchanged; the all-eight tenfold CPU-reference goal remains unmet.\n",n(&primary["accepted"],"median_ms"),n(&primary["off"],"median_ms"),n(&primary["on"],"median_ms"),n(&primary["accepted"],"cpu_ms_per_request"),n(&primary["off"],"cpu_ms_per_request"),n(&primary["on"],"cpu_ms_per_request"),n(&data["original8_aggregate"]["accepted"],"paired_median"),n(&data["original8_aggregate"]["off"],"paired_median")).unwrap();
out.push_str(r#"## Change and ordering

The private Crossbeam API pins one ordinary epoch guard, saves up to eight
victim-front indices with Acquire, issues one explicit SeqCst fence after all
snapshots, then tries the original back-load/buffer-read/pointer-check/front-CAS
body per victim. Every victim retains front -> fence -> back ordering. It returns
the first task, otherwise Retry if any probe raced, otherwise Empty. Guard and
snapshots end before returning; no task execution or parking occurs inside them.

Rayon preserves the first scalar victim probe, then groups later victims into
ordered tiles of up to eight references. The random circular victim order, self
skipping, aggregate Retry and complete-pass rescan on a race are retained. The
original OFF steal body is unchanged after removing the experimental dispatch.
The complete upstream deque source is a byte-identical prefix of the candidate;
new methods are appended. Source audit compares the extraction body byte-for-byte.
Injector, wakeup, arithmetic, proving-key preparation and job graph are unchanged.

This differs from [the earlier guard-reuse trial](../2026-10-25-batched-steal/README.md),
which retained a separate fence for every victim. It does not compose
[retained worker buffers](../2026-10-26-retained-deque-buffers/README.md).
Normal queue shrinking and deferred reclamation remain. The guard's own
publication barrier is retained before snapshots; the common explicit fence
after snapshots is also retained. No SC access is assumed to replace that fence.

Private cfg `zopro_front_snapshot_fence` and explicit
`ZOPRO_CPU_FRONT_SNAPSHOT_FENCE=1`, read once per Rayon registry, select ON.
Absent/zero select OFF. Scratch uses at most eight references and eight
WorkerIndex values on stack, plus ordinary call frames. One immutable registry
boolean is added. There is no new queue field, retained array, proving-key lookup
table, copied point buffer or per-proof heap allocation in this path. Earlier
snapshots and fewer pins can change retries, scheduling and collection cadence;
bounded scratch does not establish a whole-process memory bound.

The [ordering review](artifacts/ordering-review.md) links the primary paper and
Rust fence contract. Applying one common fence across these victims is our
inference; the cited paper does not prove this API. Native and bounded model
tests support the screen, without establishing full scheduler verification.

## Complete SDK latency

Ryzen 9950X; seven rotated/reversed fresh-process blocks compare frozen
accepted, the new executable OFF, and that same executable ON. All nine shapes
use 32 workers on CPUs 0–31 and 31 measured requests after 3 primes. Primary with
16 workers uses CPUs 0–15, one logical CPU per physical core, and 101 measured requests. Loading
and primes are excluded; fresh solving, randomness, proving, internal verification
and serialization are included. All latency finishes before CPU sampling. No
owned build, second benchmark, profiler or bulk audit overlaps latency.

Times are medians of process medians. Paired ratios are medians of within-block
ON/control ratios and need not equal the ratio of displayed medians.

| Case | Accepted ms | OFF ms | ON ms | ON/accepted paired | ON/OFF paired |
|---|---:|---:|---:|---:|---:|
"#);
for name in ["1x2-w32","2x2-w32","4x2-w32","8x2-w32","12x2-w32","16x2-w32","24x2-w32","merge-24x1-w32","merge-54x1-w32","1x2-w16"]{let c=&data["cases"][name];writeln!(out,"| {name} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} |",n(&c["metrics"]["accepted"],"median_ms"),n(&c["metrics"]["off"],"median_ms"),n(&c["metrics"]["on"],"median_ms"),n(&c["on_over_control"]["accepted"],"paired_median"),n(&c["on_over_control"]["off"],"paired_median")).unwrap();}
out.push_str("\nThe declared gate requires primary and original-eight aggregate improvement against both controls, primary wins in at least four of seven blocks, every case paired ratio ≤1.02, every individual block ratio ≤1.05, and OFF/accepted calibration within a factor of 1.03. There is no minimum gain floor, exclusion or unchanged rerun.\n\n");
let cal=data["cases"].as_object().unwrap().values().filter(|c|n(c,"off_calibration_factor")>1.03).count();writeln!(out,"Speed gate: **{}**. Failed checks: **{}**; OFF calibration fails for **{}** cases. [Exact checks and every block ratio](artifacts/pilot-result.json) are retained.\n",if passed{"pass"}else{"fail"},data["failed_checks"].as_array().unwrap().len(),cal).unwrap();
out.push_str("## CPU work and process memory\n\nUnprofiled CPU time is user+system time per measured request, median across processes. RSS is maximum whole-process high-water observation, including loading/primes; it is not minimum RAM or query-only memory.\n\n| Case | Variant | CPU ms/request | Maximum RSS MiB | Voluntary switches/request |\n|---|---|---:|---:|---:|\n");for name in ["1x2-w32","12x2-w32","merge-54x1-w32","1x2-w16"]{for variant in ["accepted","off","on"]{let m=&data["cases"][name]["metrics"][variant];writeln!(out,"| {name} | {variant} | {:.3} | {:.2} | {:.1} |",n(m,"cpu_ms_per_request"),n(m,"maxrss_mib"),n(m,"voluntary_switches_per_request")).unwrap();}}
out.push_str("\n## Separate profiles\n\nNine profiles at 999 Hz follow all latency runs and use the monotonic clock. Shares are exclusive sampled CPU-period categories with symbol/inlining limits; they are not removable wall-time fractions. Zero means no attributed samples. Barrier work may move between epoch and inlined scheduler frames. Profiled CPU time is a separate observation.\n\n| Case | Variant | CPU ms/request | Epoch % | Scheduler % | FFT % | Voluntary switches/request |\n|---|---|---:|---:|---:|---:|---:|\n");let rows=read(root.join("perf/rows.json"));for r in rows.as_array().unwrap().iter().filter(|r|r["phase"]=="profile"){let s=&r["summary"];let share=&r["profile"]["category_percent"];let rounds=n(s,"rounds");writeln!(out,"| {} | {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.1} |",r["case"].as_str().unwrap(),r["variant"].as_str().unwrap(),(n(s,"user_cpu_ms")+n(s,"system_cpu_ms"))/rounds,pct(share,"epoch"),pct(share,"scheduler"),pct(share,"fft"),n(s,"voluntary_switches")/rounds).unwrap();}writeln!(out,"\n{} samples, zero lost records and one sampled PID per capture. Raw captures and expanded symbols are retained locally with hashes.\n",closed["profile_samples"]).unwrap();
out.push_str("## Supplied M5 references\n\nHistorical supplied CPU/Metal timings, not fresh matched M5 measurements. Merge54 has no supplied reference.\n\n| Shape | CPU reference ms | Metal reference ms | ON ms | CPU/ON | Metal/ON |\n|---|---:|---:|---:|---:|---:|\n");for c in data["m5_supplied_references"].as_array().unwrap(){writeln!(out,"| {} | {:.0} | {:.0} | {:.6} | {:.2}× | {:.2}× |",c["shape"].as_str().unwrap(),n(c,"cpu_reference_ms"),n(c,"metal_reference_ms"),n(c,"on_ms"),n(c,"on_cpu_speedup"),n(c,"on_metal_speedup")).unwrap();}
out.push_str(r#"
## Validation and provenance

The runtime lock preserves 219 packages: 217 entries are byte-identical; deque and
Rayon-core lose only registry source/checksum for exact private path copies.
Their original licenses and upstream revisions are retained. Development locks
are separate and retain matching epoch 0.9.21 and utils 0.8.23. The four relevant
registry src/build/manifest trees are checked against pristine crate archives.
Actual scheduler/collector libraries and eight directly linked libraries are
pinned with the executable, source, compiler, helpers, inputs and verifier.

Each mode passes 44 deque tests (38 upstream plus 6 added), 113 Rayon tests
(two upstream tests remain ignored), and 109 core tests. Raw/cubic tests pass 2/6
with ON enabled.
Field kernels are unchanged; inherited 33-test field qualification is not claimed
as a new execution. Added checks cover ordered/mixed-flavor tiles, duplicate
references and bounds, stale front/retry aggregation, owner/stealer destruction,
concurrent growth/pop/stealing, nested guards, and actual deferred array release.
Five retired arrays remain allocated under an externally held guard and reclaim
after it drops. The allocation hook exists only in the unit-test executable.
Overloaded nested FIFO/cross-pool/panic/reuse tests also check jobs execute with
no steal guard held. First runtime compilation/tests pass; an unused test import
warning is preserved without changing frozen source.

The linked containing `wait_until_cold` routine grows from 1,661 to 5,757 bytes;
the scalar steal routine remains 356 bytes in both executables. The snapshot
helper is inlined into the scheduler routine. Exact linked disassembly is in
[codegen-review.json](artifacts/codegen-review.json) and [codegen.txt](artifacts/codegen.txt).
This static code-size difference does not establish cache misses or explain the
latency regression; no outlining or tile-size timing search follows this result.

Five instrumented Loom tests with a three-preemption bound check two-pair
store-buffering, a missing-common-fence negative control with the expected
failure message, owner-last-pop/two-thief uniqueness, publication and generation
identity. Atomic slots and retained model generations bound their scope. They
do not verify the original generic slot protocol, wrapping indices, epoch
collector or complete scheduler. The original queue protocol remains unchanged.

All nine shapes pass OFF/ON preflight with four workers before measurement. Every export
is independently Gnark-verified; fixture public bindings, uniqueness, 164-byte
compressed proof format and freshness against previous exports are audited.
All exit/stdout/stderr receipts, successful timing samples and source guards
are preserved.

No AWS resource is created and no new C7a/Aeglos comparison is claimed. Portable,
whole-SDK concurrency, pressure and very-large-circuit qualification remain
outstanding. Neither small stack arrays nor observed GC progress proves a
universal memory bound. Production is unchanged.

"#);
writeln!(out,"Total: **{} complete SDK requests, {} distinct independently Gnark-verified exports, {} proof/public members**, plus two separately counted fixed source-fixture members. {} input pins protect performance; {} prior identities are rejected before preflight.\n",closed["sdk_requests"],closed["distinct_gnark_exports"],closed["proof_public_members"],closed["performance_pins"],closed["prior_identities_before_preflight"]).unwrap();out.push_str("See [declaration](artifacts/experiment.json), [source audit](artifacts/source-audit.json), [speed checks](artifacts/pilot-result.json), [closeout](artifacts/closeout.json), and [artifact index](artifacts/artifact-index.json). Nine raw perf captures and nine expanded streams are local with hashes and omitted from Git. The research PDF/full HTML remain local with links/hashes; executable/compiler caches are represented by exact build/link receipts.\n");fs::write(report.join("README.md"),out).unwrap();println!("REPORT_WRITTEN {}",report.join("README.md").display());}
