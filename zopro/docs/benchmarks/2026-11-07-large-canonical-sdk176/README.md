# Canonical-wire admission for larger queries

On the retained 4×2 circuit at 32 logical workers, the candidate improves paired proof latency **1.58% against the accepted SDK** and **2.30% against a fresh control**. It measures **15.736 ms**, versus 15.949 ms accepted and 16.094 ms fresh. The complete pilot gate nevertheless fails: the 16-physical-core control calibration and both nonregression checks fail. The exact build is closed without adoption or further blocks. The positive 32-worker result remains evidence, rather than being discarded with the failed overall gate.

## Change and admission audit

The accepted constructor enables canonical-wire solving only when the largest query has fewer than 65,536 entries. A private `zopro_large_canonical_wires` cfg removes only that size condition. Runtime opt-in, supported native target, no-GPU, affine, no-prepared-equal-witness and folded-query checks remain. Specialized MSM and fused K/H size guards, resource reservations and portable fallbacks remain unchanged. This does not change 1×2 or 2×2 eligibility.

The canonical-wire path converts the existing solved wire buffer in place before gathering A/B canonical limbs. The public correction consumes Montgomery values first; separate a/b product arrays remain field values. Its indices and lengths use u32/usize rather than a 16-bit encoding. The cutoff is an admission restriction, not a solver-image format limit in this pipeline. Earlier qualification explicitly covered small keys; the targeted history search does not establish the original author's complete rationale or an earlier larger-admission timing result.

Broader admission is not automatically less work: the general path groups repeated Montgomery scalars before converting only distinct values. An inherited [population diagnostic](../2026-11-05-chunk-grouping176/README.md) records 4×2 A/B lengths 67,154/95,537 and admitted distinct nonzero counts 20,386/30,544. Canonicalizing the full wire array can therefore add conversions or frontload work. No instruction or speed prediction is inferred from those counts.

The existing capacity forecast charges wires/slots, A/B extraction, witness/product FFT buffers and grouped-query scratch. In-place conversion keeps the same initialized four-word storage and A/B element width; the change introduces no buffer, table, retained plan, point/key copy or production pool. Existing grouping admission still reserves metadata and retains raw-kernel fallback. This is a structural capacity argument, not measured minimum RAM or pressure qualification. The observed RSS increase below remains material.

## Native and proof qualification

The isolated snapshot has 175 byte-identical original files and one reversibly edited constructor boolean. All 176 originals are recoverable, production hashes remain unchanged, and no child module is added. Fresh OFF/ON SDKs each link coherent eight-library sets at actual O3/native/fat-LTO/one-CGU settings with the byte-identical qualified SDK driver/support. The portable generic/no-ASM/no-IFMA check with the candidate cfg passes. Unchanged field/curve/model tests are inherited rather than rerun.

Two new symbol listings and two constructor decodes qualify the actual consumers. Control contains the maximum-length comparison `cmp rcx,0x10000` and branch before reading the runtime flag. Candidate removes that maximum-length comparison while retaining the runtime flag read. Constructor size changes from 4,201 to 4,154 bytes; these static sizes are not a performance claim. No accepted SDK is decoded again.

Before timing, OFF and ON each run all nine shapes, with three primes and one observed request per shape. All 18 exported proofs independently pass Gnark with image, public-input and verification-key bindings. This includes Merge54; it does not establish all-nine candidate performance, phone execution or large-circuit memory behavior.

Accepted SHA256: `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`.
Fresh control: `80239da70f9ae35f0ce320d4a816c58ae43db5acd684951c4ddd8a19934869ff`.
Candidate: `c0c6937514567d4f68f803144e0d82b4ae05fcac6f4fae91b4743292def3c4d8`.

## Fixed 4×2 pilot

The Ryzen 9950X uses CPUs 0–31 with 32 workers or distinct physical cores 0–15 with 16 workers. All 31 blocks retain accepted/fresh/candidate at both topologies, rotating/reversing the six-case order. Each process primes three requests, then observes 151 at 32 workers or 51 at 16. All 186 captures finish before their 186 independent Gnark verifications. No owned build, hash, analysis or publication overlaps recording.

Times are medians of 31 process means. Ratios use paired block geometric means and the prospectively declared deterministic 20,000-resample bootstrap; ratios of medians are different statistics.

| Workers | Accepted ms | Fresh ms | Candidate ms | Candidate/accepted [95%] | Candidate/fresh [95%] |
|---|---:|---:|---:|---:|---:|
| 32 logical | 15.948818 | 16.094029 | 15.736392 | 0.984187 [0.979472, 0.988744] | 0.976977 [0.972025, 0.982092] |
| 16 physical | 18.609599 | 18.402561 | 18.404696 | 1.041675 [0.995098, 1.094008] | 1.033485 [0.984121, 1.085160] |

At 32 workers candidate wins 29/31 blocks against accepted and 28/31 against fresh. Fresh/accepted is 1.007380 [1.001370, 1.013248], within the declared factor-1.03 calibration bound. Both candidate intervals have upper bounds below parity, meeting the two gain checks without a minimum gain floor.

At 16 workers fresh/accepted is 1.007925 [0.979334, 1.041160], failing calibration. Both candidate intervals exceed the declared 1.02 nonregression ceiling, so three checks fail overall. These data do not establish a significant candidate regression; neither interval lies entirely above parity. The candidate wins 18/31 against accepted and 15/31 against fresh. No favorable topology subset, excluded block, extension, reroll or all-nine adoption stage follows this failed pilot.

| Workers | Accepted peak MiB | Fresh peak MiB | Candidate peak MiB |
|---|---:|---:|---:|
| 32 logical | 365.50 | 370.75 | 382.75 |
| 16 physical | 247.25 | 249.00 | 246.75 |

Candidate peak at 32 workers is 17.25 MiB above accepted and 12 MiB above fresh. These are maximum whole-process RSS observations across the corresponding process population, including loading, priming and allocator history. They are not minimum RAM, concurrent-proof capacity, pressure/swap or huge-circuit requirements. Median process CPU time per observed request is 417.613/419.518/411.453 ms at 32 workers and 243.192/243.198/242.928 ms at 16, accepted/fresh/candidate. CPU usage is sampled around the request loop, before export; see the [preceding report's CPU-scope correction](../2026-11-07-euclid-pair-sdk176/ERRATA.md).

## Automatic profiles and variation diagnosis

Six grouped counter panels and six `cycles:u` profiles cover each arm/topology after latency recording. Panels observe 31 requests and profiles 251, after three primes, using period 16,000,000, 512 mmap pages and FIFO gating. Every panel reports 100% running. All profiles qualify ownership/MMAP/PID/TID/CPU/gates, at least 1,000 samples, at most 5% unknown and zero LOST: total **144,009 samples**, maximum unknown **2.719%**. Real G1/G2 MSM symbols occur in every profile. Candidate canonical-entry attribution has **308 samples at 32 workers and 180 at 16**; both controls have zero. The changed larger-query route actually runs. Symbol samples are not exclusive wall-time fractions.

The single 32-worker candidate/fresh panel ratios are 0.990512 cycles, 0.998721 instructions, 0.984643 cache misses and 0.995194 branch misses. At 16 they are 0.999345, 1.000885, 1.006177 and 1.007801. These single panels do not provide repeated-panel uncertainty or a causal decomposition of the latency gain.

A separately labeled post-hoc audit retains all observations and leaves the gate unchanged. None of the 18,786 observed requests in any arm/topology exceeds twice its own process median. Yet 16-core process means span 17.646–22.290 ms accepted, 17.416–25.202 ms fresh and 17.517–25.621 ms candidate. This is not explained by a few isolated >2× request outliers. Pearson associations between process mean and involuntary context switches per request are 0.9889/0.9887/0.9945, while minor-fault associations are 0.0719/0.1113/0.0571. CPU-time associations are negative, −0.9252/−0.9545/−0.9822. These are descriptive associations across 31 processes per arm, not proof of a scheduler cause, an adjustment model or grounds to discard slow runs. They motivate studying large-query pool entry and scheduling before further leaf arithmetic changes.

All stages total **21,144 SDK requests and 216 distinct independently Gnark-verified exports**. The canonical export ledger advances 32,412 → 32,430 → 32,616 → **32,628**. Internal requests and copied hash inventories are not additional independent exports.

Offline review passes 467 workflow receipts, four new listing/decode receipts, 424 pilot pins, all original-source recovery/production checks, all proof bindings, raw means, paired arithmetic and profile/counter summaries. Preparatory reader generation had two text-substitution failures before capture; three successful generated readers were retained while only the missing reader was repaired. The first offline reviewer used the final ledger count at its initial-count assertion; a separate reviewer corrects only that initial assertion. The post-hoc helper initially failed to compile its JSON boolean and was repaired before its first execution. Original sources and failure summaries/logs remain. No successful build, test, clock, verifier, bootstrap or decoder is repeated to repair these helper errors.

## Original target and unchanged accepted comparisons

The table is inherited from [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md), not newly measured or mixed with this 4×2 pilot. Supplied M5 CPU/Metal references are historical cross-machine comparisons.

| Shape | Local adoption ms | M5 CPU ms | M5 Metal ms | CPU 10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| Merge24 | 65.060937 | 507 | 175 | 50.7 |

Accepted primary adoption 7.928660 ms, later 7.915095 ms and latest primary observation 7.991694 ms use the same accepted binary; this experiment adds no primary timing. Merge54 adoption is 161.883899 ms with no supplied M5 reference. Original all-eight CPU 10× goal remains active/unmet **0/8**; even the positive 4×2 pilot remains above its 10.9 ms target. No new AWS/Aeglos, mobile, large-circuit, minimum-RAM, swap or concurrent-throughput claim is made.

[Evidence](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), [checksum](artifacts/SHA256SUMS). Includes the reversible parent, driver/support, fixed plans, receipts, all timing observations, constructor decodes, analyses and review. Raw perf data/large decoded streams remain local with hash/receipt references. Targets, binaries/libraries, private continuation notes and proving/input/proof payloads are excluded. Absolute paths require reconstruction elsewhere.
