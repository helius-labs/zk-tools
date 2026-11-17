# Clean production grouping-frame comparison222

The 32-byte frame does **not establish a production full-proof gain** and is
not adopted. The preceding221 diagnostic screen remains a preliminary positive
observation; this longer comparison uses the clean production source and the
exact accepted194 SDK. Small gain size is not a rejection criterion. Both
prospectively named primary contexts fail the gain requirement against both
controls, while all70 latency/CPU/RSS/calibration guard checks pass.

|Context|ON/accepted paired median|Nominal interval|ON/fresh OFF paired median|Nominal interval|
|:---|---:|:---|---:|:---|
|1x2-w16|1.002553|1.000007–1.004626|0.998532|0.995026–1.003699|
|1x2-w32|1.001592|0.999444–1.002860|1.000977|0.995411–1.002484|
|2x2-w32|0.999021|0.992367–1.003496|0.996395|0.988431–1.001755|
|4x2-w32|1.003544|0.995436–1.011280|1.003553|0.994673–1.007448|
|8x2-w32|1.007218|0.996673–1.013030|1.002452|0.991749–1.011758|
|12x2-w32|0.994384|0.989826–1.000593|0.989340|0.985456–0.996540|
|16x2-w32|1.000732|0.997384–1.006689|0.998678|0.993233–1.003186|
|24x2-w32|1.002360|0.996121–1.008182|1.001310|0.997207–1.004046|
|merge-24x1-w32|1.004743|0.998263–1.012201|1.004112|0.998896–1.008297|
|merge-54x1-w32|1.004778|1.000553–1.007991|1.003213|0.997272–1.010525|

Ratios below1 favor ON. At16 workers ON is0.255% slower than accepted REF
and0.147% faster than fresh OFF, with the OFF comparison inconclusive. At32
workers ON is0.159% slower than REF and0.098% slower than OFF; both intervals
include1. The primary16 REF lower bound barely exceeds1 (1.0000073), so the
full-precision result is retained. No context, backing or block is removed.
The12x2 gain against fresh OFF does not establish improvement against accepted
REF, and was not declared as a primary adoption context. No post-hoc shape
dispatch is introduced.

|Context|REF ms|OFF ms|ON ms|Peak RSS REF/OFF/ON MiB|
|:---|---:|---:|---:|:---|
|1x2-w16|8.944481|8.978982|8.966948|84.75 / 84.25 / 82.00|
|1x2-w32|7.662432|7.678531|7.677843|128.25 / 130.75 / 130.75|
|2x2-w32|10.783563|10.799800|10.786392|239.25 / 232.00 / 238.75|
|4x2-w32|15.803102|15.859653|15.899299|290.50 / 287.25 / 290.25|
|8x2-w32|24.146844|24.168842|24.267523|473.00 / 479.25 / 464.00|
|12x2-w32|34.017525|34.212940|33.901423|657.00 / 671.50 / 667.00|
|16x2-w32|40.068259|40.087932|40.060445|772.00 / 802.50 / 783.25|
|24x2-w32|66.465760|66.412940|66.609492|1222.92 / 1233.00 / 1231.79|
|merge-24x1-w32|66.019283|66.108733|66.311232|1103.25 / 1111.50 / 1112.00|
|merge-54x1-w32|163.245173|163.400309|164.088337|1822.31 / 1845.62 / 1860.59|

Displayed times are medians of process means. Decisions use medians of paired
within-block ratios, which can differ in direction from a quotient of the
displayed medians. Peak RSS includes setup and priming; it is neither minimum
RAM nor memory-pressure qualification.

REF is the exact accepted194 consumer
`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.
Fresh unchanged OFF is
`fa101d27e74c6d7f9ca4b701da83f7e1a75effdecfc076fa4edf26de3b6e1565`.
ON is `81d565e0b7439216ed8a529535500d3b32c6ac0127e7253a3c758db5d1aaf975`.
Each arm uses three independent byte-identical executable backings.

OFF matches all179 accepted production source hashes. ON changes only the
grouping parent: the native32-byte-aligned one-byte marker, a bounded
`black_box` borrow and its grouping/worker oracle. The experimental selector
is removed for this proposed native default. The exact194 driver/support
and compiler/link flags are retained. No209–216 Boolean, PMU or phase
diagnostics are linked. No field arithmetic, dependency, map entry, heap/table
allocation, proving-key copy, pool or default stack reservation changes.

The emitted ON function at0x22d5e0 has size0x1e08, six saved registers,
`and rsp,-32` and480bytes reserved. Maximum activation is560bytes including
the return slot, within the frozen2048-byte bound. Its32-byte key spill at
0x22e676 targets `[rsp+0x100]`, with two16-byte reloads in that aligned span.
That span cannot cross a4KiB boundary. This confirms the static mechanism;
it does not establish a unique hardware cause or a production cycle saving.
There is no new live-SP or clean-build grouping PMU measurement in this run.
The large cycle benefit in the diagnostic build cannot be treated as a
production latency benefit.

Native OFF passes all134 core and2 prover tests. Native ON passes all135 core
and2 prover tests. The same ON source passes all103 scalar-fallback core and
2 prover tests with noIFMA/noASM. The worker oracle checks all53 actual workers
in1/4/16/32-worker pools with512KiB stacks, plus four OS callers sharing an
eight-worker pool. Unchanged arithmetic qualification is inherited by source
identity. These are x86 scalar checks, not physical ARM/mobile results.

Thirty preflight exports are checked before the main campaign. Main capture
uses30 matched blocks ×10 contexts ×3 arms:151 timed requests per primary
process at16/32workers and17 per other process, each after three primes.
Contexts and arms rotate/reverse; backing=block%3. All main timing precedes
late Gnark verification and analysis, with no owned CPU cojobs during capture.
All SDK, verifier and reader steps pass first execution. There are930 fresh
Gnark exports,42,240 SDK calls including primes,39,450 timed records,1,860
receipts and3,720 checked stream hashes. Two separate same-agent raw readers
agree; this is not an external audit.

The frozen interval uses30 paired process means and one-based ranks10/21,
nominal95.7226% marginal median coverage under independent exchangeable
blocks. Gain requires an upper bound below1 against both controls at at least
one of16/32workers, plus all three backing-stratum medians below1 there.
The prospectively declared two-primary one-sided union bound is4.2774% under
the same assumptions. Other checks are marginal guardrails without a joint
confidence claim: latency/CPU uppers<=1.02, peak RSS<=1.05×control+1MiB,
OFF/REF paired medians within factor1.03. There is no gain floor, extension,
dropped backing or rescue replay.

The warm SDK boundary includes witness solving, fresh randomness, proving,
SDK verification and serialization. Key loading, three primes, cold certificate
setup and export remain outside timing. All arms enable the accepted
certified-output/fused/indexed opt-ins. The existing cold certificate setup
cost remains visible and is not a first-proof latency improvement. In this run,
cold certification ranges from 61–86 ms for 1x2 at 16 workers, 51–61 ms at
32 workers, and 88–103 ms for 2x2. All recorded major page-fault counts are zero;
this does not establish behavior under memory pressure.

Candidate comparison against the supplied M5 references follows. These are
reference ratios across different hardware, not same-machine CPU/Metal tests.
All eight CPU10× targets remain unmet. Merge54 has no supplied M5 reference.

|Shape|Candidate ms|M5 CPU speedup|M5 Metal speedup|CPU10× limit ms|
|:---|---:|---:|---:|---:|
|1x2|7.677843|4.82×|3.13×|3.7|
|2x2|10.786392|5.47×|2.78×|5.9|
|4x2|15.899299|6.86×|3.02×|10.9|
|8x2|24.267523|7.79×|3.05×|18.9|
|12x2|33.901423|8.38×|2.98×|28.4|
|16x2|40.060445|8.36×|3.10×|33.5|
|24x2|66.609492|7.93×|2.64×|52.8|
|merge-24x1|66.311232|7.65×|2.64×|50.7|

Production194/196 remains accepted. The fixed compact-frame recipe is closed
as a production candidate; no unchanged retry is planned. The next useful
question is whether clean production even exhibits the diagnostic grouping
cycle deficit. Any further diagnosis must use the exact clean consumers and
keep profiled observations separate from unprofiled latency qualification.

Canonical proof ledger is now **60,025**,
`/home/ubuntu/src/zopro-perf-work/store-frame-clean222/proof-sha256-after-main.json`,
SHA256 `ac50e8c1c48858afcc1d7438cc88443aab322e88408a6cf3d07fed1bb2e63a83`.
Complete raw evidence remains private under that experiment root. Keys, raw
witnesses/proofs, full ledgers and binaries are excluded from this report.
No AWS resources were used. This report is committed locally; no push.
