# Fixed fourth root in terminal FFT butterflies

The declared complete-caller gate is **false**. This is a component experiment, with no production change or new proving-time result. All six contexts and required checks are retained.

FIXED is 7.4–7.7% slower than fresh original code at one/16 workers and 15.4–17.9% slower at 32 workers, with zero wins in every fresh-control comparison. All six control calibrations pass. Removing unit-lane work and 54 static IFMA sites did not make this caller faster. The fixed multiplier improves the four-lane ODD diagnostic by 15.1–29.1%, but both remain slower than the original eight-lane butterfly. This exact specialization is closed before full-FFT or proof integration.

## Change

The original paired terminal DIF/DIT multiplies eight lanes by `(1,i,1,i)` twice, where i²=−1. The candidate leaves unit-factor lanes unchanged, gathers the four odd lanes into the existing four-lane Fr/R260 representation, multiplies by one fixed canonical fourth root k and scatters back. Canonical k=−i fits 192 bits. The same positive factor serves both orientations; the forward transform negates four outputs. The mathematical butterfly, root direction, Pair buffers, field representation and outer task structure are unchanged.

Two five-u64 factor/quotient arrays have 80 logical bytes, including two constant zero words. They are globally fixed and computed at compile time, with no per-key or domain-sized table, dynamic preparation, new heap allocation or solver plan. Object-file constants, registers and stack are separate from that logical count. The multiplier uses the [qualified fixed-factor quotient construction](../2026-10-24-fixed-multiplier-component/README.md): for B=2^52, R=B^5 and c=floor(kR/r), the truncated high product and two low products return a normalized residue below 2r. The small factor's leading zero limbs permit constant product terms to disappear. General bounds and arithmetic remain inherited; this specialization gets fresh native checks.

Unlike the [G2 fixed-beta transform](../2026-10-31-beta-fixed176/README.md), this removes unit-lane work and uses a small fourth root in four lanes. Earlier prepared-root tables, Gaussian Fr and whole-FFT layout trials remain separate negative evidence. No outside implementation code is imported.

## Fixed caller measurements

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. FROZEN is the exact accepted field library; OFF is fresh original full-eight-lane arithmetic. ODD uses ordinary four-lane multiplication after unit-lane removal; FIXED adds small-root quotient arithmetic. Fresh modes share one executable. Actual final links use -O, native CPU, fat LTO and one codegen unit. Original butterfly bodies are extracted verbatim from accepted source; only the two candidate product sites differ.

Sixty-four or 1,024 Pair vectors per worker model the current expanded-leaf range. Each Pair holds eight fields; adjacent Pairs hold four independent four-point transforms. Each cycle applies complete in-place DIF then inverse DIT, returning four times the input. All shuffles, loads/stores, sign corrections and arithmetic are timed. Input construction, thread startup, pre-loop barriers and output checks are outside each worker clock. Three checked eight-cycle primes precede 64 timed cycles. Every prime-pass and final timed coordinate must match original scalar input times 4^cycles, with unchanged capacities. Intermediate timed states are not individually checked. Wave latency is maximum worker duration divided by 64; this is an independent terminal-caller wave, not a whole FFT, proof schedule or concurrent-proof throughput.

Workers use 512 KiB stacks and individual affinity: CPU 2 solo, CPUs 0–15 or 0–31. Fifteen fixed paired separate-process blocks rotate/reverse contexts and variants. Every FIXED/control interval must have upper<1 and at least 10/15 wins against both original controls; all OFF/frozen intervals must fit factor 1.03. ODD comparisons are descriptive. Ranks 4–12 of 15 give nominal 96.484375% marginal median bounds under independent/exchangeable blocks, without familywise guarantees. No gain floor, exclusion, extension, size/width/mode sweep or unchanged reroll. A pass selects separate full-FFT/query/proof qualification and never automatic adoption.

|Pair vectors/workers|Frozen µs/cycle|OFF µs/cycle|ODD µs/cycle|FIXED µs/cycle|FIXED/OFF [bounds], wins|FIXED/frozen [bounds], wins|
|---|---:|---:|---:|---:|---|---|
|64/1|2.051672|2.044938|3.084563|2.194922|1.073795 [1.069632, 1.097482], 0/15|1.071113 [1.065262, 1.074474], 0/15|
|64/16|2.182406|2.177078|3.307484|2.347078|1.073812 [1.066302, 1.078261], 0/15|1.069839 [1.063703, 1.075443], 1/15|
|64/32|2.924734|2.910953|3.993313|3.414406|1.178787 [1.147838, 1.186149], 0/15|1.165776 [1.161070, 1.207160], 0/15|
|1024/1|32.492047|32.504266|49.076453|34.941828|1.076770 [1.071026, 1.078942], 0/15|1.072435 [1.071319, 1.079247], 0/15|
|1024/16|35.177891|35.137047|52.875484|37.716281|1.074417 [1.072151, 1.078074], 0/15|1.072859 [1.070068, 1.077432], 0/15|
|1024/32|47.413938|47.559688|66.831969|55.318391|1.153758 [1.144254, 1.171926], 0/15|1.155607 [1.145102, 1.164485], 0/15|

|Pair vectors/workers|OFF/frozen calibration|FIXED/ODD diagnostic|ODD/OFF diagnostic|
|---|---|---|---|
|64/1|0.996805 [0.994917, 0.999246], 12/15|0.710772 [0.707822, 0.725640], 15/15|1.505450 [1.501817, 1.517026], 0/15|
|64/16|0.998084 [0.994514, 1.001229], 10/15|0.709039 [0.677180, 0.713457], 15/15|1.506647 [1.502408, 1.588831], 0/15|
|64/32|1.003885 [0.984886, 1.012361], 7/15|0.849130 [0.826805, 0.865101], 14/15|1.389996 [1.305095, 1.429381], 0/15|
|1024/1|1.000115 [0.996116, 1.001830], 7/15|0.711353 [0.709575, 0.713641], 15/15|1.509214 [1.506824, 1.515341], 0/15|
|1024/16|1.000660 [0.995782, 1.001327], 6/15|0.713264 [0.710670, 0.714414], 15/15|1.508236 [1.501342, 1.516549], 0/15|
|1024/32|1.001701 [0.991949, 1.008923], 6/15|0.824665 [0.814464, 0.838551], 15/15|1.400723 [1.379573, 1.414554], 0/15|

Failed checks: `["n64-w1/fixed/frozen/gain","n64-w1/fixed/off/gain","n64-w16/fixed/frozen/gain","n64-w16/fixed/off/gain","n64-w32/fixed/frozen/gain","n64-w32/fixed/off/gain","n1024-w1/fixed/frozen/gain","n1024-w1/fixed/off/gain","n1024-w16/fixed/frozen/gain","n1024-w16/fixed/off/gain","n1024-w32/fixed/frozen/gain","n1024-w32/fixed/off/gain"]`. Marginal times are medians of process wave means; ratios pair within blocks and need not divide into marginal medians. All positive estimates and uncertain contexts remain in the tables.

## CPU and memory

|Pair vectors/workers|OFF/ODD/FIXED CPU ms/wave|Frozen/OFF/ODD/FIXED max process RSS MiB|
|---|---:|---:|
|64/1|0.002203/0.003250/0.002359|2.00/2.00/2.00/2.00|
|64/16|0.035703/0.053297/0.038187|2.00/2.00/2.00/2.00|
|64/32|0.096344/0.132828/0.112937|2.25/2.25/2.25/2.00|
|1024/1|0.032688/0.049234/0.035109|2.50/2.50/2.50/2.50|
|1024/16|0.546234/0.819984/0.586547|6.50/6.25/6.25/6.50|
|1024/32|1.464406/2.055938/1.690406|10.25/10.25/10.25/10.25|

RSS includes fixture creation, worker-owned modeled buffers and primes. It is not a full-prover peak or minimum RAM. The transform bodies add no heap allocation; capacities are checked but no allocator-call count is measured. Faults and context switches remain in every raw receipt, without a causal RAM or swap claim.

## Linked code and automatic profiles

|Fresh complete-cycle symbol|IFMA sites|Static instructions|RSP-operand instructions|Bytes|
|---|---:|---:|---:|---:|
|driver::odd|212|955|100|5921|
|driver::fixed|146|996|210|6316|
|driver::ordinary|200|746|68|4749|

These are static complete-cycle function counts, not executed spills, cycles or memory bounds. Frozen and fresh original/ODD function sizes and counts match. Removing useful multiplications can still add shuffles, signed corrections and surrounding instructions.

The counter panels record about 500,763 instructions per FIXED buffer cycle versus 368,164 for OFF, a 36.0% increase at both worker counts. Cycles also rise: 179,628→192,561 at 16 workers and 220,549→249,802 at 32. These observations support the measured regression; they do not isolate the cost of spills, cache behavior or a particular instruction port. The original implementation can execute unit-factor lanes without the candidate's gather, scatter and sign-correction work.

After all latency, six FIFO-gated four-event counter panels and six separate fixed-16-million-user-cycle profiles run 8,192 cycles after excluded primes, at 1,024 Pair vectors and 16/32 workers for OFF/ODD/FIXED. Event coverage must be at least 99.9%. PID, registered worker TID/CPU, binary MMAP, worker FORK, period and zero loss are checked. Attribution requires at least 1,000 samples and at most 5% unknown share; small panels remain without resampling. Absolute period coverage, exclusive symbol costs, local CPI and removable proof wall time are unvalidated.

|Panel|Cycles/terminal cycle|Instructions/terminal cycle|Qualified|
|---|---:|---:|---|
|stat-n1024-w16-fixed-b0|192560.678|500763.145|true|
|stat-n1024-w16-odd-b0|270887.068|482837.171|true|
|stat-n1024-w16-off-b0|179627.766|368164.157|true|
|stat-n1024-w32-fixed-b0|249802.257|500763.190|true|
|stat-n1024-w32-odd-b0|328051.421|482837.229|true|
|stat-n1024-w32-off-b0|220548.725|368164.181|true|

|Profile|Samples|Unknown share|Attribution qualified|
|---|---:|---:|---|
|cycles-n1024-w16-fixed-b0|1568|0.0000%|true|
|cycles-n1024-w16-odd-b0|2208|0.0000%|true|
|cycles-n1024-w16-off-b0|1457|0.0000%|true|
|cycles-n1024-w32-fixed-b0|4069|0.0000%|true|
|cycles-n1024-w32-odd-b0|5350|0.0374%|true|
|cycles-n1024-w32-off-b0|3588|0.0836%|true|

## Qualification and evidence

All 37 native field tests pass, including 69,664 new independent Arkworks lazy-lane outputs with changing chains and both root orientations. Two mathematical caller tests check every one-way DIF output against an independent four-point scalar DFT, then DIT restoration; 64 full-width/zero families and all three implementations are covered. Separate buffer tests cover lengths 2/6/64/1024 and seven changing cycles. Both existing FIFO-control tests also pass in the same four-test caller suite. No core, full FFT, SDK proof, portable or pressure qualification is claimed.

The private snapshot has 177 files: 175 byte-exact originals, one append-only field parent and one child. Independent stripping recovers all 176 accepted sources. Every production hash remains unchanged. All native tests, library build, caller links/tests and fixed capture complete once without failure. Display-float checks use relative 1e-12 from the outset; performance thresholds and integer receipts are unchanged.

Independent review checks 432 raw command/exit/stream receipts, 1142 pin entries, 396 caller processes, all 30 paired ratios, CPU/RSS/gates, every PMU value and every profile bin/thread mapping. Recorded terminal cycles including primes: 2893448; profile samples: 18240. A cycle covers the complete modeled buffer, not one butterfly. All latency precedes diagnostics, and all diagnostics precede decoding/hashing/archive work. No other owned build/helper/hash/archive or benchmark overlaps capture; the host is shared. No new proof, solve, oracle execution, Gnark export or verification count is added; ledger 27,353 remains unchanged.

The two field libraries and all caller modes remain private diagnostic artifacts. ODD's descriptive result cannot relabel the FIXED gate or promote another implementation. Further proof-time claims require a separately declared production-caller integration and the original all-shape qualification.

## Accepted proof comparisons — inherited, unchanged

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge 24→1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Accepted merge 54→1 is 161.883899 ms without a supplied M5 reference. Accepted primary process RSS is 132.75 MiB. The [adoption report](../2026-10-30-g2-vector-sign-adoption176/README.md) retains its physical-16 calibration engineering exception and qualification scope. No new memory or platform result follows.

Latest AWS primary remains historical source 175 on c7a.24xlarge: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, each campaign's best tested configuration. That does not compare current source 176. No AWS resources were created here.

All eight CPU 10× targets remain unmet, 0/8. Primary remains 7.928660 ms versus the 3.7 ms target. No new AWS/Aeglos, mobile/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-proof result follows.

[Design](artifacts/design.md), [all observations](artifacts/analysis.json), [independent review](artifacts/review.json), [symbols](artifacts/symbols.json), [static code](artifacts/codegen.json), [field arithmetic](artifacts/quarter.rs), [complete butterfly source](artifacts/kernels.rs), [identities](artifacts/identities.json). Archives: [private source](artifacts/source.tar.gz), [workflow/helpers](artifacts/workflow.tar.gz), [raw receipts](artifacts/receipts.tar.gz), [disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS).

Local root: `/home/ubuntu/src/zopro-perf-work/quarter-root176`. Binaries, libraries and raw perf.data stay local with hashes. The reused field-identities metadata key `scalar_beta_quotient_logical_bytes` denotes the 80-byte quarter-root constants here; no G2 beta method is linked into this private field snapshot.
