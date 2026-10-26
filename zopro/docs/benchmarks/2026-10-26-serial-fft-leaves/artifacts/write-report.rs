use serde_json::Value;
use std::{fs,path::Path,fmt::Write};
fn n(v:&Value,k:&str)->f64{v[k].as_f64().unwrap()}
fn main(){let root=Path::new("/tmp/zopro-serial-fft-leaves");let data:Value=serde_json::from_slice(&fs::read(root.join("pilot-result.json")).unwrap()).unwrap();let report=Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-serial-fft-leaves");fs::create_dir_all(report).unwrap();let mut out=String::from(r#"# Bounded serial work inside expanded FFT leaves

Every 32-worker speed and regression condition passes, but the overall
screen remains unqualified: the physical-16 OFF control differs from
accepted by a factor of 1.040054, beyond the fixed 1.03 calibration bound.
That failure is retained. The candidate is preserved without adoption or an
unchanged timing rerun. The original all-eight tenfold CPU goal remains unmet.

## Change

The private module retains the existing 32-way packed FFT passes and the
expanded `Vec<Pair>` leaves of at most 1,024 vectors. Inside each expanded
leaf, binary forks stop at a power-of-two cutoff computed once per transform:
`floor_power_of_two(clamp(vectors / (2 * workers), 64, 1024))`.
The cutoff is carried through recursion. When there are at least 128 vectors
per worker, this exposes at least two terminal chunks per worker.

Terminal leaves call the existing DIF/DIT quad and within-pair arithmetic
in serial loops, without Rayon operations or heap allocation. They bypass
the nested multi-stage fused leaf execution; the outer packed 32-way passes
retain their kernels. An audit recovers the original OFF function body and
the 32-way geometry, expanded-vector allocation/load/store/drop after removing
only the experimental dispatches. This is a different boundary from the
[previous cheap-pass grain trial](../2026-10-26-fft-pass-grain/README.md).

Private cfg `zopro_serial_fft_leaves` and explicit
`ZOPRO_CPU_SERIAL_FFT_LEAVES=1` select ON. A process-local `OnceLock` reads
the option once; absence/zero select OFF. No new key table, buffer, copied
point array, plan, dependency or per-proof heap allocation is introduced.
Additional binary-fork frames hold references and scalar parameters, with no
large stack array. Scheduling can still change allocation lifetimes and peaks.

## Full SDK matrix

Ryzen 9950X; seven rotated/reversed fresh-process blocks, accepted versus the
same new executable OFF and ON. All nine shapes use 32 workers on CPUs 0–31,
31 measured requests after three primes. The 16-worker primary uses CPUs 0–15, one
logical CPU per physical core, and 101 measured requests. The larger 16-worker
sample was declared before implementation because earlier observations
varied across controls; its regression and calibration bounds were retained.

Loading and preparation are excluded. Fresh solving, randomness, proving,
internal verification and serialization are timed. All latency completes
before separate profiles. No owned build, profiler, CPU benchmark or bulk
audit overlaps latency. Times below are medians of process medians; ratios
pair corresponding blocks, so ratios of displayed medians differ.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF | M5 CPU / ON | M5 Metal / ON |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
"#);
let names=["1x2-w32","2x2-w32","4x2-w32","8x2-w32","12x2-w32","16x2-w32","24x2-w32","merge-24x1-w32","merge-54x1-w32","1x2-w16"];
for name in names {let row=&data["cases"][name];let m=&row["metrics"];let reference=data["m5_supplied_references"].as_array().unwrap().iter().find(|r|format!("{}-w32",r["shape"].as_str().unwrap())==name);let(cpu,metal)=reference.map(|r|(format!("{:.2}x",n(r,"on_cpu_speedup")),format!("{:.2}x",n(r,"on_metal_speedup")))).unwrap_or(("—".into(),"—".into()));writeln!(out,"| {name} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {cpu} | {metal} |",n(&m["accepted"],"median_ms"),n(&m["off"],"median_ms"),n(&m["on"],"median_ms"),n(&row["on_over_control"]["accepted"],"paired_median"),n(&row["on_over_control"]["off"],"paired_median")).unwrap();}
out.push_str(r#"
Primary at 32 workers wins 6/7 against both controls. The original-eight paired geometric
ratios are 0.990246 against accepted and 0.995147 against OFF. Merge-54 wins 7/7
against accepted and 5/7 against OFF. All 32-worker cases satisfy the fixed
bounds. The physical-16 case's ON/accepted and ON/OFF paired ratios also improve, but
its OFF calibration fails; more requests did not resolve that mismatch.
These observations do not establish its cause or qualify universal adoption.

The declaration requires positive primary 32 and original-eight geometric
results against both controls, at least four primary wins, every case within
2% paired-median and 5% per-block regression, and OFF calibration within 1.03.
No minimum gain floor or later exclusions are used.
[All failed checks and observations](artifacts/pilot-result.json).

The M5 CPU references are 37/59/109/189/284/335/528/507 ms and Metal references
24/30/48/74/101/124/176/175 ms in the supplied shape order. They are historical
references, not fresh matched-machine M5 runs. None of the eight candidate
results reaches tenfold CPU speedup. No M5 reference was supplied for merge-54.

## CPU time, memory and profiles

The following CPU values are medians of process-loop CPU per request; RSS
is the maximum ordinary latency-process peak and includes loading/priming.

| Case | CPU ms accepted | CPU ms OFF | CPU ms ON | Peak MiB accepted | Peak MiB OFF | Peak MiB ON |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
"#);
for name in names{let m=&data["cases"][name]["metrics"];writeln!(out,"| {name} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",n(&m["accepted"],"cpu_ms_per_request"),n(&m["off"],"cpu_ms_per_request"),n(&m["on"],"cpu_ms_per_request"),n(&m["accepted"],"maxrss_mib"),n(&m["off"],"maxrss_mib"),n(&m["on"],"maxrss_mib")).unwrap();}
out.push_str(r#"
Nine separate 999 Hz monotonic-clock captures contain 300,972 samples, with
one process per capture and no lost records. Primary profiled CPU is
198.95 ms OFF and 198.02 ms ON; merge-54 is 4951.84 ms OFF and 4800.42 ms ON.
Primary epoch share falls from 9.41% to 8.42%, while the scheduler category
rises from 3.52% to 3.95%. Merge-54 epoch share falls from 2.21% to 1.75% and
scheduler from 2.18% to 2.04%; FFT remains about 42% of its sampled work.
The 12x2 capture does not show a CPU reduction:969.17 ms OFF versus 969.97 ms ON.
These are modest changes, not a broad CPU breakthrough. Inlining limits
attribution; sampled user-cycle shares are not removable wall time.
[Complete profile observations](artifacts/perf/report.md).

No peak establishes minimum RAM or pressure feasibility. The trial adds no
buffer, but measured peaks can still vary with scheduling and allocator
state. Portable, real memory-pressure, concurrent whole-SDK, large-circuit
and clean-integration qualification remain outstanding. There is no new
C7a, Aeglos, TEE or phone measurement here.

## Verification and retained failure

Native OFF and ON each pass all 112 core tests. Three new tests check cutoff
bounds, independent scalar-polynomial inverse coefficients before forward
identities, ordinary/twisted roots and admitted cached directions, concurrent
callers sharing roots, and 512 KiB worker stacks. Raw-image tests pass two and
cubic tests pass six. Field kernels are unchanged; the prior 33-test field
qualification is inherited, with no new field-suite run claimed.

The first OFF suite passes 111 tests and fails a test that assumes both
optional root caches were admitted. Clearing column vectors retained their
reservation tokens, and two peak builders need not fit in the existing
preparation budget. The correction changes only test fixtures: replace the
whole roots object to release storage and credit, exercise cached forward,
release it, then exercise cached inverse. No budget or runtime algorithm
changes. The failed source, logs and executable hash remain; the original
executable is retained locally. The final build uses a new source snapshot
and target directory. No SDK or proof timing preceded that correction.
[Failure review](artifacts/failure-v1/review.json).

All-nine OFF/ON preflight at four workers completes 72 SDK requests and 18
independent Gnark exports. Fixture/freshness validation runs inside that
workflow and again as a required perf check before timing. Latency adds
8,610 requests and 210 exports; profiles add 510 requests and nine exports.
Total: **9,192 SDK requests, 237 distinct verified exports and 474 proof/public
members**. Public fixtures and fresh proof identities pass against 3,045
prior exports; the 18 preflight identities also guard timed/profiled exports.
The final review rehashes every member and checks run/verifier exit codes
and stdout/stderr receipts. All 460 performance input pins pass.

The final candidate has 173 source pins:170 accepted files, the frozen
CLI/helper, and the private module. Source, actual linked libraries, binaries,
images, original verification-key inputs, fixtures, verifier and orchestrator
are pinned. Accepted production's170-file manifest remains unchanged.
No AWS resource is created by this experiment.

[Protocol](artifacts/experiment.json), [closeout](artifacts/closeout.json),
[source audit](artifacts/source-audit.json), [patch](artifacts/source.patch),
[build](artifacts/native-v2/result.json),
[proof inventory](artifacts/proof-inventory.json),
[next design to audit](artifacts/next-design.md).

Full local experiment: `/tmp/zopro-serial-fft-leaves/`.
Raw perf captures and expanded symbol streams are retained locally, omitted
from Git, and included in the artifact hash index. Two existing source test
fixtures are hashed separately from the 474 new export members. Executables
and compiler caches are represented by source/build/link and failure receipts.
"#);
fs::write(report.join("README.md"),out).unwrap();println!("REPORT_WRITTEN_ALL8_CPU_AND_METAL");}
