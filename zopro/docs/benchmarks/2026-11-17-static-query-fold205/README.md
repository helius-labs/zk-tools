# Static proportional query folding

**The candidate removes work and passes native correctness, but its latency gain
remains inconclusive under the frozen qualification rule. It is not adopted.**
Primary32 paired ON/REF is **0.992798 [0.988917,1.000003],20/30wins**. Its CPU
ratio is **0.991708 [0.986890,0.998341],23/30wins**. All ten latency/CPU
non-regression and RSS guards pass, as do all three primary backing medians.
Only the strict primary latency bound `<1` fails. This is not an implementation
failure or a rejection for having too small a percentage gain.

The latency upper bound is1.00000291358562: the interval permits about0.000291%
slowdown. The paired point estimate is0.720%lower; displayed process-mean medians
are7.681454→7.650088ms,31.366µs apart. These summarize different distributions.
The result neither proves zero benefit nor qualifies a precise latency gain.
No samples, confidence rule, rounding threshold or favorable subset is changed.

## Change and correctness

An offline converter uses the existing semantic-cse176 analysis to certify
proportional scalar expressions across arbitrary valid solver inputs. If
`s_j=c*s_i`, their point contributions become `s_i*(P_i+c*P_j)`. The earliest
query row represents its class. The converter combines A/G1, B/G1 and B2/G2
points, removes redundant extraction indices and writes a smaller Euler-v2
image. Original constraint operations, InvZero/zero-divisor semantics, C-basis
metadata, verifying key and public input binding stay intact. Affected G2 inputs
are checked on-curve/in-subgroup before scalar multiplication.

The **exact adopted SDK194** consumes both images; no prover relink or runtime
source change is involved. SDK SHA256:
`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.
Certificate, fused-preparation and indexed-tile selectors are enabled in both
arms. REF uses the original image; ON uses its transformed replacement. The
timed prover maps one image, with no retained analysis DAG, additional point
table, runtime key copy, new pool or per-point alias indirection.

|Shape|A rows original /folded|B rows original /folded|Image bytes saved|
|---|---:|---:|---:|
|1x2|21,006 /20,896|30,089 /29,904|43,740|
|2x2|36,637 /36,442|52,274 /51,971|72,648|
|4x2|67,154 /66,786|95,537 /94,996|131,060|
|8x2|129,064 /128,338|183,347 /182,306|253,404|
|12x2|190,734 /189,634|270,757 /269,184|383,108|
|16x2|252,548 /251,058|358,343 /356,206|520,172|
|24x2|376,992 /374,674|534,619 /531,258|816,380|
|merge24x1|359,220 /357,378|507,895 /505,934|509,612|
|merge54x1|804,765 /800,613|1,137,190 /1,132,769|1,148,852|

These are image/term counts, not proportional latency or peak-RAM reductions.
The converter maps its source and streams output with sparse point patches;
temporary symbolic analysis remains unqualified for huge-circuit memory budgets.
No claim is made that this prototype is a universally bounded production
compiler or that a phone should run its analysis.

Ten native model/converter tests pass, including existing semantic zero/corruption
cases, new sparse certificates under independent field valuations and G1/G2
folds checked against generator-coefficient sums. Field/core/prover runtime
qualification is inherited by exact source identity. A separate sorted-vector
symbolic reader checks all nine sparse certificates by cross multiplication,
along with every parser/semantic/query summary. It imports neither primary
model module. This is a separate same-agent implementation, not an external audit.

## Full-proof results

Local Ryzen9950X,16physical/32logical CPUs, shared host. The selected204screen
uses three blocks and shows primary32 ON/REF0.984695,3/3wins, with all guards
passing. Its1×2 means are7.610506/7.491210ms. That result selects qualification;
it is not pooled with the larger campaign or treated as a precise speedup.

Qualification205 has30paired process blocks in ten contexts,17timed fresh SDK
calls after3excluded primes per process. Contexts rotate/reverse; arm order
alternates; three byte-identical independently copied executable backings rotate
by block. Workers pin individually to CPUs0..N−1. Key/pool loading, preparation
and final export are outside latency; solving, quotient, fresh blinding,
construction, SDK verification and serialization are inside.

|Shape/workers|REF ms|ON ms|Paired ON/REF [bounds]|Wins/30|Peak RSS REF /ON MiB|
|---|---:|---:|---|---:|---:|
|1x2/16|8.880131|8.870779|0.997283 [0.996632,1.000098]|20|75.75 /77.50|
|1x2/32|7.681454|7.650088|0.992798 [0.988917,1.000003]|20|93.75 /95.00|
|2x2/32|10.789926|10.854270|1.006588 [0.998965,1.009228]|10|233.75 /235.50|
|4x2/32|16.013520|15.971199|0.999226 [0.995605,1.004255]|15|287.75 /289.00|
|8x2/32|24.365193|24.325267|0.996455 [0.985739,1.009450]|19|470.75 /477.50|
|12x2/32|34.103661|34.163440|1.000425 [0.989982,1.011333]|15|669.00 /644.75|
|16x2/32|40.263299|40.505949|1.003594 [0.999461,1.009396]|11|786.00 /778.75|
|24x2/32|66.931423|66.817638|0.999382 [0.991748,1.004861]|17|1200.75 /1214.15|
|merge24/32|66.070555|65.798699|0.998276 [0.992963,1.004803]|15|1100.33 /1114.28|
|merge54/32|163.516808|163.943491|0.999904 [0.996420,1.004279]|16|1817.93 /1805.74|

Values are medians of process means. Paired summaries need not equal quotients
of displayed medians. Sorted zero-based ranks9/20 provide nominal95.722605%
marginal median coverage under independent continuous-block assumptions, with
no joint guarantee. Primary32 backing medians are0.996278/0.989935/0.991020.
Its displayed CPU medians are171.252/170.397ms per request. Other CPU intervals
and all raw samples are retained in the artifacts. RSS is lifetime process peak,
including loading/primes, not minimum RAM or physical-pressure qualification.

Four204smoke exports precede its18main processes. Qualification adds18verified
native preflights, both arms across all nine shapes, before600main processes.
Each phase completes timing before its late independent Gnark checks. All
**640 exports/12,448 SDK requests** across204/205 pass their bindings and freshness
checks, including SDK self-verification on every prime/timed call. All179
production source hashes remain exact. No successful capture is replayed.

The204protocol starts16:18:13UTC and its first screen result is recorded by
16:23:10, within five minutes. Qualification's16:31protocol reaches its complete
assessment at16:46:38, within its30-minute allowance. Its1,236 SDK/verification
commands sum736.710seconds; the600main captures plus600late verifier commands
sum719.571seconds. These are summed command durations, not exclusive CPU or
total research time. Subsequent profiles/readers/reporting extend the work
through16:57UTC; earlier source/history research is outside these intervals.

## Diagnostic profiles206

The post-qualification diagnostic is explicitly separate from the failed latency
gate. It adds no qualification samples. Exact primary32 images/SDK, three blocks
of plain/stat/cycles panels in both arms,251timed calls after3primes:18processes,
4,572requests and18new Gnark-verified exports. All captures precede decoding and
verification. No owned build, analysis or archive overlaps a capture.

All four factor1.05 observer checks and twelve PMU/profile quality checks pass.
Grouped counters have at least99.9%scheduled coverage. Six fixed16M user-cycle
profiles contain **72,116 samples**, all assigned PID/TID/CPU/MMAP checks pass,
zeroLOST, unknown shares1.59–1.99%. FIFO acknowledgement gates recording after
primes;512mmap pages. Absolute period coverage remains unvalidated.

|Counter|ON/REF block0|Block1|Block2|
|---|---:|---:|---:|
|Instructions|0.999741|0.999363|0.998970|
|Cycles|1.000994|0.999785|0.997522|
|Cache misses|0.988195|0.996737|0.983876|
|Branch misses|0.989899|0.990478|0.984174|

Instructions fall0.026–0.103%; cycles are mixed. These are descriptive panels,
not causal uncertainty bounds or proof that a cache change caused latency.
REF/ON profiles have36,107/36,009 samples: G1 stream7,707/7,703, G2 borrowed
5,934/5,924 and epoch pin3,178/3,270. Flat cycle samples are not exclusive
critical-path time; these counts do not justify another epoch/scheduler variant.
The dominant kernels remain present after this small work removal.

The first manual profile-helper link omitted its existing demangler extern.
The dependent prepare/workflow invocations therefore stopped before any capture.
Supplying the extern repairs only that link; all scientific profile steps then
succeed first execution. The initial workflow failure log and repair note are retained.
Model tests, conversion, native capture, verification and statistical reader all
succeed on their first scientific executions. Unused-helper warnings remain.

The separate205statistical reader reconstructs1,236 SDK/Gnark receipts and2,472
stream hashes, all618 qualification outputs, every paired statistic/decision,
three backing identities and179 source hashes. It also checks206profile symbol
count conservation and counter ratios, inheriting206's pinned decoder/quality
reader rather than claiming a second complete perf decoder.

Final inventory is **58,915 distinct Gnark-verified exports**, SHA256
`8e40735a1253e777437b8a1d06ca29a61642129fea8fcf5b68d7cebe8fbd35ee`.
The658new exports across204–206 account for17,020 completed SDK requests;
reverification is not counted as additional distinct proofs.

## References and decision

The candidate's32-worker observations versus supplied historical M5 references:

|Shape|Candidate ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.650088|37 /4.84×|24 /3.14×|3.7|
|2x2|10.854270|59 /5.44×|30 /2.76×|5.9|
|4x2|15.971199|109 /6.82×|48 /3.01×|10.9|
|8x2|24.325267|189 /7.77×|74 /3.04×|18.9|
|12x2|34.163440|284 /8.31×|101 /2.96×|28.4|
|16x2|40.505949|335 /8.27×|124 /3.06×|33.5|
|24x2|66.817638|528 /7.90×|176 /2.63×|52.8|
|merge24|65.798699|507 /7.71×|175 /2.66×|50.7|

These are cross-machine descriptive ratios, not new Apple measurements or an
adopted baseline. Merge54 has no supplied M5 reference. The verified all-eight
CPU10× goal remains active and unmet, **0/8**. Accepted production remains194/196,
with local primary7.628251ms at32 workers. Exact accepted C7a/Aeglos comparisons
remain [201](../2026-11-17-accepted-c7a201/README.md); this candidate has no AWS run.

Close this fixed image recipe under its declared rule, preserving its positive
pilot, CPU reduction and borderline latency result. Do not relabel it a slowdown
or erase the useful work reduction. No portable-image/phone/ARM, huge-circuit,
pressure/swap or concurrent-throughput qualification is claimed. Production
converter integration is not undertaken after this performance decision.
No source adoption, changed confidence rule, unchanged retry or public push.

[Protocols and results](artifacts/), [source/receipts](evidence.tar.gz),
[checksums](SHA256SUMS). Archives exclude raw keys/points/scalars/witnesses,
private folding-certificate records, proof/public bytes, full ledgers, executable
libraries/binaries and raw perf.data. Converter/source helpers and decoded
diagnostics remain reproducible with the pinned private inputs.

Private roots: `/home/ubuntu/src/zopro-perf-work/static-query-fold204`,
`/home/ubuntu/src/zopro-perf-work/static-query-fold205`,
`/home/ubuntu/src/zopro-perf-work/static-query-profile206`.
