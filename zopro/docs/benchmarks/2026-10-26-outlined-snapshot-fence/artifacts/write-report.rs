use serde_json::Value;
use std::{fmt::Write,fs,path::Path};
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn n(v:&Value,k:&str)->f64{v[k].as_f64().unwrap()}
fn pct(v:&Value,k:&str)->f64{assert!(v.is_object());v.get(k).map(|n|n.as_f64().unwrap()).unwrap_or(0.)}
fn main(){let root=Path::new("/tmp/zopro-outlined-snapshot-fence");let data=read(root.join("pilot-result.json"));let closed=read(root.join("closeout.json"));let gate=read(root.join("codegen-gate.json"));let report=Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-outlined-snapshot-fence");fs::create_dir_all(report).unwrap();let passed=data["performance_gate_passed"].as_bool().unwrap();let mut out=String::from("# Outline the bounded victim-snapshot scan\n\n");out.push_str(if passed{"The fixed four-variant local speed screen passes. The candidate is preserved for resource, portable, concurrent SDK, very-large-circuit and target-host qualification before production integration.\n\n"}else{"The fixed four-variant local speed screen fails. The candidate is not adopted and no unchanged timing rerun follows. All observations and failed checks are preserved.\n\n"});
let primary=&data["cases"]["1x2-w32"]["metrics"];writeln!(out,"Primary accepted/new OFF/outlined ON/old inlined ON is **{:.6}/{:.6}/{:.6}/{:.6} ms**. Original-eight paired geometric outlined/accepted, outlined/OFF and outlined/inlined ratios are **{:.6}/{:.6}/{:.6}**. Production remains unchanged; the all-eight tenfold CPU-reference goal remains unmet.\n",n(&primary["accepted"],"median_ms"),n(&primary["off"],"median_ms"),n(&primary["on"],"median_ms"),n(&primary["inlined"],"median_ms"),n(&data["original8_aggregate"]["accepted"],"paired_median"),n(&data["original8_aggregate"]["off"],"paired_median"),n(&data["original8_aggregate"]["inlined"],"paired_median")).unwrap();
out.push_str(r#"## Source and code-generation gate

The prepared source differs from the [previous inlined prototype](../2026-10-26-front-snapshot-fence/README.md)
by exactly one `#[inline(never)]` on Rayon's `steal_front_snapshot_tiles` method.
231 of 232 files are byte-identical; removing the attribute recovers the remaining
file byte-for-byte. The scan body, tile of eight, first scalar probe, saved-front
ordering, explicit common SC fence, normal epoch guard and retry semantics stay
identical. Kernels, dependency versions, queue layout, shrinking, reclamation
policy and proof representation are unchanged.

The source and linked-code gate ran before any SDK measurement. It requires a
separate scan symbol, a direct call from the scheduler, and a caller smaller
than the old inlined routine and no larger than twice the accepted caller.
Failure would close the experiment without SDK timing or another attribute.

"#);
writeln!(out,"The gate **passes**. `wait_until_cold` is {} bytes accepted, {} bytes inlined, and {} bytes outlined. The separate outlined scan is {} bytes. These sizes establish the intended boundary, not cache misses or a performance cause. [Exact linked disassembly](artifacts/codegen.txt) and [gate checks](artifacts/codegen-gate.json) are retained.\n",gate["accepted_parent_bytes"],gate["inlined_parent_bytes"],gate["outlined_parent_bytes"],gate["outlined_helper_bytes"]).unwrap();
out.push_str(r#"Scratch remains bounded by the same eight references/eight WorkerIndex values
plus ordinary call frames. No key table, retained buffer, copied point array,
queue field or per-proof scan heap allocation is added. Collection policy is
unchanged; changed scheduling can still change observed collection activity and
memory lifetime. Static footprint does not establish a whole-process RAM bound.

## Complete SDK comparison

Ryzen 9950X; seven rotated/reversed fresh-process blocks compare frozen accepted,
the new outlined executable OFF/ON, and the exact previous inlined executable ON.
The old prototype is a direct control for this source change; its original
negative result remains unchanged. All nine shapes use 32 workers on CPUs 0–31,
31 measured requests after three primes. Primary with 16 workers uses CPUs 0–15,
one logical CPU per physical core, and 101 measured requests.

Fresh solving, randomness, proving, internal verification and serialization are
timed. Loading and priming are excluded. All latency completes before sampling,
with no other owned build, benchmark, profiler or bulk audit overlapping.
Displayed times are medians of process medians; paired ratios are medians of
within-block ratios and can differ from ratios of displayed medians.

| Case | Accepted ms | New OFF ms | Outlined ON ms | Inlined ON ms | ON/accepted paired | ON/OFF paired | ON/inlined paired |
|---|---:|---:|---:|---:|---:|---:|---:|
"#);
for name in ["1x2-w32","2x2-w32","4x2-w32","8x2-w32","12x2-w32","16x2-w32","24x2-w32","merge-24x1-w32","merge-54x1-w32","1x2-w16"]{let c=&data["cases"][name];writeln!(out,"| {name} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} |",n(&c["metrics"]["accepted"],"median_ms"),n(&c["metrics"]["off"],"median_ms"),n(&c["metrics"]["on"],"median_ms"),n(&c["metrics"]["inlined"],"median_ms"),n(&c["on_over_control"]["accepted"],"paired_median"),n(&c["on_over_control"]["off"],"paired_median"),n(&c["on_over_control"]["inlined"],"paired_median")).unwrap();}
out.push_str("\nThe declared gate requires primary and original-eight aggregate improvement against all three controls, primary wins in at least four of seven blocks, every case paired ratio ≤1.02, every block ratio ≤1.05, and OFF/accepted calibration within a factor of 1.03. There is no minimum gain floor or post-measurement exclusion.\n\n");let cal=data["cases"].as_object().unwrap().values().filter(|c|n(c,"off_calibration_factor")>1.03).count();writeln!(out,"Speed gate: **{}**; {} failed checks, {} OFF-calibration failures. [All paired checks](artifacts/pilot-result.json) are retained.\n",if passed{"pass"}else{"fail"},data["failed_checks"].as_array().unwrap().len(),cal).unwrap();
out.push_str("## CPU work and memory\n\nCPU milliseconds are summed user+system time per measured request, median across processes. RSS is maximum whole-process high-water observation, including loading/primes; it is not minimum RAM or query-only storage.\n\n| Case | Variant | CPU ms/request | Maximum RSS MiB | Voluntary switches/request |\n|---|---|---:|---:|---:|\n");for name in ["1x2-w32","12x2-w32","merge-54x1-w32","1x2-w16"]{for variant in ["accepted","off","on","inlined"]{let m=&data["cases"][name]["metrics"][variant];writeln!(out,"| {name} | {variant} | {:.3} | {:.2} | {:.1} |",n(m,"cpu_ms_per_request"),n(m,"maxrss_mib"),n(m,"voluntary_switches_per_request")).unwrap();}}
out.push_str("\n## Separate profiles\n\nTwelve profiles at 999 Hz follow all latency runs using the monotonic clock. Shares are exclusive sampled CPU-period categories with symbol/inlining limits, not removable wall-time fractions. Zero means no attributed samples. Outlining can move attribution between symbols; CPU and latency remain separate observations.\n\n| Case | Variant | CPU ms/request | Epoch % | Scheduler % | FFT % | Voluntary switches/request |\n|---|---|---:|---:|---:|---:|---:|\n");let rows=read(root.join("perf/rows.json"));for r in rows.as_array().unwrap().iter().filter(|r|r["phase"]=="profile"){let s=&r["summary"];let p=&r["profile"]["category_percent"];let rounds=n(s,"rounds");writeln!(out,"| {} | {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.1} |",r["case"].as_str().unwrap(),r["variant"].as_str().unwrap(),(n(s,"user_cpu_ms")+n(s,"system_cpu_ms"))/rounds,pct(p,"epoch"),pct(p,"scheduler"),pct(p,"fft"),n(s,"voluntary_switches")/rounds).unwrap();}writeln!(out,"\n{} samples; zero lost records and exactly one sampled PID per capture. All twelve declared case/variant profile pairs are checked independently on closeout.\n",closed["profile_samples"]).unwrap();
out.push_str("## Supplied M5 references\n\nThese are supplied historical CPU/Metal timings, not fresh matched M5 measurements. Merge54 has no supplied reference.\n\n| Shape | CPU reference ms | Metal reference ms | Outlined ON ms | CPU/ON | Metal/ON |\n|---|---:|---:|---:|---:|---:|\n");for c in data["m5_supplied_references"].as_array().unwrap(){writeln!(out,"| {} | {:.0} | {:.0} | {:.6} | {:.2}× | {:.2}× |",c["shape"].as_str().unwrap(),n(c,"cpu_reference_ms"),n(c,"metal_reference_ms"),n(c,"on_ms"),n(c,"on_cpu_speedup"),n(c,"on_metal_speedup")).unwrap();}
out.push_str(r#"
## Validation and provenance

Prepared source identity is checked across all 232 files before building.
The inherited five instrumented Loom models (three-preemption bound) are rerun,
including an expected-message missing-fence negative control. Both runtime modes
pass 44 deque, 113 Rayon (two upstream tests ignored), and 109 core tests. ON
raw/cubic tests pass 2/6. Field kernels are unchanged; inherited 33-test field
qualification is not claimed as a new execution. No new test mirrors the attribute.
The bounded models do not verify the full generic slot protocol, epoch collector
or scheduler; the previous allocation/drop/guard-free-job tests are retained.

The 219 runtime packages retain the same versions and source choices as the
inlined prototype. Source, tool, compiler, actual linked libraries, all three
executables, original keys, images, requests and verifier are pinned. The prior
registry-source audit is inherited, with actual file/archive pins checked again.
All nine shapes pass new OFF/ON preflight with four workers. Every exported proof
is independently Gnark-verified, has the expected 164-byte compressed format,
matches fixture public inputs, and is fresh against all prior proof identities.

An initial protocol-builder compile error from an ambiguous string conversion
was repaired before SDK preflight. The original helper and error log remain;
runtime sources and measurement counts were unchanged. All command exit and
stdout/stderr hash receipts, samples and source/binary guards are preserved.

No cloud resource or new Aeglos/C7a comparison is created here. Portable,
whole-SDK concurrency, pressure and very-large-circuit qualification remain
outstanding. Production remains unchanged.

"#);
writeln!(out,"Total: **{} complete SDK requests, {} distinct independently Gnark-verified exports, {} proof/public members**, plus two fixed source-fixture members. Performance uses {} input pins and rejects {} prior identities before preflight.\n",closed["sdk_requests"],closed["distinct_gnark_exports"],closed["proof_public_members"],closed["performance_pins"],closed["prior_identities_before_preflight"]).unwrap();out.push_str("See [declaration](artifacts/experiment.json), [source identity](artifacts/outline-audit.json), [code-generation gate](artifacts/codegen-gate.json), [speed checks](artifacts/pilot-result.json), [closeout](artifacts/closeout.json), and [artifact index](artifacts/artifact-index.json). [Every request sample](artifacts/perf/rows.json) is retained. Twelve raw captures and twelve expanded streams are kept locally with hashes and omitted from Git. Executables/compiler caches are represented by build/link hashes; the old inlined source is preserved in its linked prior report.\n");fs::write(report.join("README.md"),out).unwrap();println!("REPORT_WRITTEN {}",report.join("README.md").display());}
