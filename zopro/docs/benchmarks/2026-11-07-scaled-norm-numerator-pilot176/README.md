# Scaled G2 norm and numerator: native qualification and SDK pilot

The matched scaled arithmetic is correct, but this implementation does **not establish a complete-proof speedup**. The paired median is 0.16% faster than fresh OFF and 0.03% slower than accepted; both intervals include parity. Calibration passes. The fixed pilot fails its gain gate, so the candidate is closed without adoption or an unchanged reroll.

## Change

The [mathematical qualification](../2026-11-07-scaled-norm-numerator176/README.md) derives fused sum/difference operations divided by 16. Existing uniform R260 reduction consumes normalized raw R256 digits, and its raw result remains an R256 value representing the scaled operation. There is no representation conversion or runtime inverse-16 multiplication.

For full G2 streaming groups, both the denominator norm and complex numerator receive the same scale. The existing prefix inverse cancels it, yielding the original slope. Owned and signed-borrowed prefix paths change together. Scalar tails, final coordinate operations, G1, generic trees, keys, solver and scheduler retain their original operations. The existing 640-byte prefix record and heap allocation sites remain. No lookup table, key copy or additional input-sized state is introduced.

## Native correctness and emitted code

Two new field tests pass 196,608 output-lane checks over Fq/Fr and four/eight lanes, including inclusive raw boundaries, lazy inputs and changing chains. Independent Arkworks results check the explicit division by 16; 35 original field tests are filtered rather than repeated. OFF and ON each pass all 127 core tests, including activation, signs, ownership, tails and repeated updates. Two prover-library and six cubic integration tests pass.

The field snapshot has 173 exact accepted parents, three reversible edits and one child; the core snapshot has 169 exact parents, seven reversible edits and two children. The native reviewer checks 355 source pins and identical qualified field bytes between snapshots. Coherent eight-library SDK links use native CPU, fat LTO and one codegen unit. [SDK identities](artifacts/sdk-identities.json) bind accepted/OFF/ON binaries.

Final SDK disassembly, decoded once before timing:

|G2 borrowed caller|OFF|ON|
|---|---:|---:|
|Static instructions|8,591|8,522|
|IFMA sites|1,450|1,450|
|Left-4/right-48 shifts|72 / 60|54 / 45|
|Prologue stack subtraction, bytes|5,952|6,336|
|ZMM stack operand/store rows|332 / 156|353 / 174|

The three changed terminal reductions remove 69 instructions, but the stack frame grows by 384 bytes and static stack traffic increases. There are no calls to the new wrappers. G1 static metrics remain unchanged. Static operands are not dynamic spill counts or a complete concurrent stack/RAM bound. The structural gate selects timing qualification, not adoption.

## Complete SDK latency

Ryzen 9 9950X, 32 logical workers pinned to CPUs 0–31. Fifteen fixed rotated/reversed blocks; each process performs three primes and 151 measured complete SDK requests. All latency processes finish before verification, counters or profiles. No other owned CPU workload overlaps recording; every block is retained.

|Arm|Median process mean, ms|Median process maximum RSS, MiB|Largest observed process maximum RSS, MiB|
|---|---:|---:|---:|
|Accepted|7.966927|129.50|134.25|
|Fresh OFF|7.955187|130.25|134.50|
|ON|7.976422|130.75|134.50|

|Paired comparison|Median ratio|Interval|Wins of 15|
|---|---:|---:|---:|
|OFF/accepted|0.999934|0.994789–1.003061|8|
|ON/OFF|0.998423|0.994660–1.010707|8|
|ON/accepted|1.000265|0.993600–1.005610|7|

Ratios pair process means by block and need not equal ratios of displayed medians. Sorted ranks 4–12 give nominal 96.484375% marginal median coverage conditional on independent, exchangeable blocks, without a familywise guarantee. The frozen gate requires ON upper ratios below 1 and at least 10 wins against both controls; OFF/accepted bounds must lie within [1/1.03, 1.03]. Both gain checks fail and calibration passes. There is no gain floor, exclusion, extension or favorable-subset rescue.

## Counters, profiles and limits

After latency, each arm runs one 31-request counter panel and one 251-request profile, with three primes and FIFO warm gates. Profiles use user cycles, a 16-million period and 512 mmap pages. All counters report 100% running.

|User counter per request|OFF|ON|
|---|---:|---:|
|Cycles|813,425,496|812,332,585|
|Instructions|1,388,140,613|1,393,721,945|
|Cache misses|2,694,934|2,764,277|
|Branch misses|900,091|918,659|

These single panels are diagnostic, not replicated estimates or causal evidence. The smaller selected routine does not establish fewer whole-proof instructions: the ON panel has approximately 0.40% more instructions and 0.13% fewer cycles. Arithmetic, register pressure and concurrent scheduling remain possible contributors; this study does not isolate them.

Accepted/OFF/ON profiles contain 12,286/12,317/12,349 samples, zero recorded LOST events and 1.48%/1.79%/1.72% unknown symbols. Headers, events, periods, PID/TID and CPU sets pass review. Sampled user-cycle shares are not exclusive wall time, removable critical-path fractions or a DRAM study. Raw perf data remains private; text decoding receipts and hashes are retained.

Preflight covers all eight supplied shapes plus merge54 in all three arms: 108 requests including primes and 27 fresh exports independently verified by Gnark. Latency adds 6,930 requests/45 exports; diagnostics add 864 requests/six exports. **Total: 7,902 requests and 78 independently verified fresh exports.** The canonical unique-proof ledger advances from 31,543 to 31,621. The independent reader checks 227 pins, capture receipts, proof bindings, raw paired gates and diagnostic totals; it is not a second cryptographic verifier.

Native maximum RSS observations do not establish minimum RAM, mobile, ARM, giant-circuit or memory-pressure behavior. There is no new AWS/Aeglos measurement. The latest qualified all-shape baseline remains 7.915095 ms for 1×2, versus supplied M5 CPU 37 ms and Metal 24 ms; the CPU 10× target is 3.7 ms and remains unmet for all eight supplied shapes.

## Reproducibility

[Artifacts](artifacts/) contain source edits/new test modules, helpers, frozen plans, pins, raw text/JSON receipts and analyses. The receipt archive is compared with original files, and SHA256SUMS covers published files. Actual proofs, public inputs, requests, witnesses, keys, executables, libraries and raw perf data are excluded.

Preparation history is retained: a reviewer launch met the active SDK workflow lock; the first generated-source recovery reviewer removed an original appended implementation and was corrected to remove only the balanced new implementation; formatting-only field-test drift was restored to its exact qualified bytes; benchmark-review compilation initially omitted its demangler library argument and was corrected. Successful tests, proofs, verifiers and decodes were not replayed to repair bookkeeping. No production source was promoted.
