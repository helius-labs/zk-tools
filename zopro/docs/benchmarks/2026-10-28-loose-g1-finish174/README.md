# Bounded loose G1 coordinate finish

The candidate fails the declared primary SDK latency gate and is not adopted. On this Ryzen 9950X host, 1x2 with 32 workers takes **8.175023 ms** versus fresh OFF **8.086036 ms** and accepted **8.085906 ms**. The paired ON/OFF ratio is 1.011330, with bounds [1.008204, 1.014614] and zero wins in 15 blocks. Production source174 and its accepted executable remain unchanged.

## Arithmetic change

The candidate finishes ordinary G1 affine coordinates using local loose Montgomery integers. With slope s and S=Ax+Bx, it computes U=s², X0=2p+U−S, D0=2p+Ax+S−U, V=REDC(s·D0), and Y0=p+V−Ay. It canonicalizes X0/Y0 directly at host export. This removes some intermediate carry/correction passes in the source, but does not remove a field multiplication.

For both supported primes, 5p<2²⁵⁶. Given s<2p, Ax/Ay<p and S<2p, U<2p, X0<4p, D0<5p, V<3p and Y0<4p. The normalized wider operand reaches only the unchanged nonfused CIOS recurrence and stays local. One conditional subtraction of 2p followed by one of p canonicalizes each output. [Range argument](artifacts/bounds.md). Finite oracle tests supplement this argument; no formal machine-code proof is claimed.

Only the native eight-lane FAST G1 stream finish changes. Prefixes, signed numerators, inversions, record layout, scalar tails, general non-FAST streams, G2, trees, widths/parts, solver, scheduler and production memory forecasts are unchanged. No heap buffer, point/key table, execution plan, pool or assembly is added. Compiler frames are not claimed identical.

This differs from the [weighted-Y anchor](../2026-10-28-g1-weighted-anchor-sdk/README.md), [lazy-linear quotient arithmetic](../2026-10-26-lazy-linear/README.md), and [post-REDC signed subtraction](../2026-10-28-signed-final-sub-sdk174/README.md). Their failures remain closed. Current performance is screened on actual complete requests first; no isolated arithmetic gain is used to predict proof speed.

The private snapshot contains 175 files: 171 byte-exact originals, three reversibly changed parents, and one new eight-lane field child. Original nonfused CIOS, the four-lane parent and the generation script remain byte-exact. This experimental child is not a cleaned generator/portable production change.

## Complete requests

Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. Each latency process runs three primes followed by 151 complete SDK requests. Solving, fresh blinding, proving, internal verification and serialization are included; loading is excluded. Affinity is CPUs0–31 with 32 Rayon workers on 16 physical cores. No exclusive-host claim is made.

| Variant | Median block ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |
|---|---:|---:|---:|---:|
| Accepted | 8.085906 | 186.699 | 131.25 | 134.75 |
| Fresh OFF | 8.086036 | 186.264 | 130.75 | 134.00 |
| ON | 8.175023 | 188.593 | 131.50 | 134.75 |

| Paired comparison | Median ratio | Bounds | Wins/15 | Check |
|---|---:|---|---:|---|
| OFF/accepted | 1.001673 | [0.995879, 1.005094] | 5 | Calibration passes |
| ON/OFF | 1.011330 | [1.008204, 1.014614] | 0 | Fails |
| ON/accepted | 1.013094 | [1.007878, 1.016271] | 0 | Fails |

Bounds are sorted paired-ratio ranks4–12 of15, nominal96.484375% coverage per comparison under exchangeable independent blocks, without a familywise guarantee. Both candidate upper bounds must be below1 with at least10wins; calibration must fit [1/1.03,1.03]. There is no gain floor, exclusion, threshold change, extension or successful-capture repeat. Paired savings are negative: −91.612 µs versus OFF and −105.137 µs versus accepted. They differ from differences of unpaired medians.

The failed primary gate closes this exact candidate. No physical16/larger/all-eight/clean production qualification follows. Observed RSS does not establish a memory saving, minimum RAM or pressure behavior.

## Counters, profiles and generated code

After all latency, three FIFO-gated four-event stat runs and three separate199Hz cycles:u profiles cover all variants. Counters measure31requests and profiles251, after three primes. Loading/primes are outside the FIFO gate. All six diagnostic captures precede their verification and decode. Diagnostic timings do not enter latency decisions.

| Per-request event | OFF | ON |
|---|---:|---:|
| cycles:u | 852,381,490 | 857,642,569 |
| instructions:u | 1,453,386,711 | 1,473,504,253 |
| cache-misses:u | 2,926,810 | 2,977,748 |
| branch-misses:u | 1,243,765 | 1,244,835 |

All events run100%. Profiles contain9,252accepted,9,172OFF and9,244ON samples, **27,668 total**, one PID each and zero lost records. Both ON and OFF profiles identify the native FAST borrowed-signed G1 stream. The new finish is selected unconditionally in that compiled FAST body; it has no resource-admission branch. Shares are descriptive sampled CPU periods, not removable proof wall-time fractions.

After captures, offline disassembly of that exact sampled symbol shows:

| Variant | Static instruction sites | IFMA sites | RSP-address sites | Call sites |
|---|---:|---:|---:|---:|
| Accepted | 4,428 | 800 | 588 | 31 |
| OFF | 4,428 | 800 | 588 | 31 |
| ON | 4,551 | 800 | 612 | 31 |

Static counts cover the complete symbol, not dynamic executions or spill counts. Literal stack subtractions are0xfc0+0x100 in accepted/OFF and0xfc0+0xc0 in ON; these are not full nested stack peaks. The [audit](artifacts/codegen.json) binds disassembly to pinned executable hashes and demangled symbols.

The algebra removes source-level normalization work but adds offsets, a larger expression and canonical export corrections. The actual code has123more instruction sites, and measured retired instructions rise1.38%; measured cycles rise0.62%, while the paired latency regression is1.13%. These observations reject this implementation. They do not isolate an exclusive instruction, cache, SMT or RAM-capacity cause. A smaller count of source carry loops did not produce less executed work. The mathematical range argument remains valid within its scope.

## Validation and preparation repair

Current OFF passes33field/122core tests; ON passes34field/122core tests. Each passes eight prover/raw-cubic tests. The new field test independently checks133,120output coordinates against Arkworks over both primes, with lazy slopes/sums, canonical anchors, boundaries, zero and full-width inputs, and also checks exact canonical exports against the ordinary formula. Existing core tests cover complete point addition, signed/unsigned streams, scalar tails, scratch reuse and generator MSMs. Optional fixture cases absent from the local checkout are not claimed as new external coverage.

The initial candidate compilation failed before tests because two const assertions compared a boolean carry flag with integer0. Those assertions change to comparison with false; runtime arithmetic is unchanged. All six successful control steps are reused. A separate workflow completes the six remaining candidate steps. The full initial175-file source, native failure and preparation receipts are preserved under `before-const-bool-repair`. The merged completion index explicitly references both authoritative workflow receipts. Capture preparation checks exactly the two permitted pinned-file changes and reconstructs their old bytes. No successful test or capture is repeated. [Repair record](artifacts/repair-note.json), [completion index](artifacts/native-completion.json).

Before capture, a helper's stale final-ledger assertion was corrected and explicit field-suite receipt checks were added. The original generation script remains archived. A publication-helper Path-call typo was also corrected before capture and before that helper executed. These preparation corrections change no measured source, executable or protocol after capture begins.

All117capture steps complete. All **7,806 SDK requests** verify internally. All **54 fresh164-byte exports** independently pass Gnark, have correct public-input bindings and are distinct from the complete prior inventory. All108proof/public members are archived; ledger15,811→15,865. All closing source/library/helper/compiler/input pins match. OFF is separately rebuilt and differs in binary hash from accepted; their calibration is measured.

Full current and pre-repair source, manifests, licenses, tests, commands, raw observations and decoded diagnostics are included. Three raw perf recordings remain local with checked SHA256 receipts. No owned build/helper/hash/archive/second benchmark overlaps capture. There is no new AWS/Aeglos, phone/ARM/TEE, giant-circuit, portable, concurrent-proof, swap or forced-pressure qualification.

## CPU and Metal references

The supplied historical M5 CPU/Metal1x2 references are37/24ms. Fresh accepted is4.576×/2.968× faster, OFF4.576×/2.968× and ON4.526×/2.936×. These are reference ratios, not fresh M5 captures.

The complete accepted table is inherited from the [source174 adoption](../2026-10-28-g2-borrowed-adoption/README.md), including its failed performance gate and explicit engineering exception. This primary-only screen does not replace that all-eight qualification.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.366931 | 37 | 4.422× | 24 | 2.868× |
| 2x2 | 11.539335 | 59 | 5.113× | 30 | 2.600× |
| 4x2 | 16.540233 | 109 | 6.590× | 48 | 2.902× |
| 8x2 | 25.136722 | 189 | 7.519× | 74 | 2.944× |
| 12x2 | 34.858408 | 284 | 8.147× | 101 | 2.897× |
| 16x2 | 41.377043 | 335 | 8.096× | 124 | 2.997× |
| 24x2 | 67.764001 | 528 | 7.792× | 176 | 2.597× |
| merge-24x1 | 67.096516 | 507 | 7.556× | 175 | 2.608× |

Qualified primary remains8.366931ms; its CPU10×target is3.7ms. The original all-eight CPU10×goal remains **active and unmet,0/8**. No new accepted performance result is claimed.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [closing pins](artifacts/pins.json), [capture completion](artifacts/runs/result.json), [research/history](artifacts/research.md).
