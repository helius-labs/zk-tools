# Compact lazy-Y complete G2 Window screen

**Rejected; not adopted.** Retaining unreduced Y in compact buckets makes the complete Window about 4% slower across all six contexts, with zero winning blocks against either control. All six original/OFF calibrations pass. The two-size 32-worker aggregate ON/OFF ratio is **1.042390 [1.032445, 1.046172]**, zero wins/15. Fourteen declared checks fail; no SDK or AWS trial follows.

This tests the qualified [lazy-Y arithmetic](../2026-11-07-g2-lazy-y176/README.md) inside the actual ordinary Window: fresh construction, queue/busy handling, both Phi feeds, prefix inversions, scalar tails, exceptional routing, projective merge and final canonical conversion. X remains canonical, Y stays in a separate raw-word type, and each bucket remains 128 bytes. No key table, duplicate plan, new pool or arbitrary memory cap is introduced. The original query selector and accepted prover remain unchanged.

Times are median milliseconds per parallel wave across independent per-worker Windows, **not complete proof latency or throughput**. Descriptors/valid points are synthetic, with geometry from saved accepted 1×2/2×2 ordinary tasks:15,044/14,426 points, 2,048 buckets, batch 200, column 0.

|Points/workers|Original ms|OFF ms|ON ms|Paired ON/OFF [low, high]|Wins/15|
|---|---:|---:|---:|---|---:|
|G2-15044-w1|1.243004|1.240089|1.295110|1.043647 [1.034222, 1.048489]|0|
|G2-15044-w16|1.452645|1.445886|1.504978|1.040724 [1.037986, 1.042577]|0|
|G2-15044-w32|2.357775|2.361221|2.456976|1.043757 [1.029633, 1.052233]|0|
|G2-14426-w1|1.190605|1.190509|1.238873|1.041884 [1.035432, 1.049127]|0|
|G2-14426-w16|1.418623|1.408761|1.465425|1.039752 [1.038156, 1.041922]|0|
|G2-14426-w32|2.276565|2.268996|2.354166|1.040146 [1.033363, 1.044891]|0|

The protocol fixes 15 rotated/reversed blocks and three arms: a newly built original-source control frozen before capture, original Window OFF in the candidate executable, and lazy-Y ON in that same executable. Driver/support bytes are identical, Cargo dependencies are coherent, and final consumers use O3/native/fatLTO/oneCGU. `Original` is this prepared Window control, not the accepted SDK binary. Three one-Window checked primes precede 64 observed Windows per worker. Timing includes 63 intermediate result destructions; last equality/hash/destruction, pool/fixture/oracle setup and primes are excluded. Fixed descriptors repeat within a process; intermediate timed outputs are consumed but only the last is checked.

Paired ordered ranks 4–12/15 give nominal 96.484375% marginal median coverage under independent exchangeable blocks, without simultaneous coverage. Every control interval must fit factor 1.03; every ON/control upper bound must be<=1.03; both 32-worker aggregate upper bounds must be<1 with >=10 wins. All twelve ON nonregressions and both aggregate gain tests fail. There is no exclusion, extension, positive gain floor, subset promotion or unchanged reroll.

## CPU and memory observations

The complete Window does more counted work despite skipping two vector export reductions. Separate single PMU panels show approximately 0.4% more instructions and 3.1–3.7% more user cycles. These are diagnostic observations without replicated counter uncertainty or an exclusive critical-path/cause claim. Generic cache-miss counts rise at 32 workers but fall in the 16-worker panel; they identify neither RAM-capacity pressure nor a unique cache/DRAM bottleneck.

|Panel|ON/OFF instructions|ON/OFF cycles|
|---|---:|---:|
|G2-15044-w16|1.004006|1.037395|
|G2-15044-w32|1.003996|1.031208|
|G2-14426-w32|1.004111|1.031433|

All six fixed 16M-cycle/512-page profiles qualify:45,824 samples, at least 3,401 each, maximum unknown share 0.170%, zero LOST, owned PID/TIDs, worker CPUs, mappings/task events and period/header checks. Original profiles sample the original signed borrowed kernel; ON profiles sample `msm::lazy_y::apply`. Flat function bins include inlined work and are not exclusive cost. All 24 PMU events run >=99.9%. Profiles and panels follow all latency; ownership/decoding/loss checks follow all recording. No owned build, analyzer, hashing, archive or publication overlaps capture.

All six allocation preflights have exactly equal allocation, reallocation and cumulative requested-byte totals across the three arms. At 15,044 points/32 workers, each arm requests 9,809,824 bytes across 415 allocations and 139 reallocations for 32 complete Windows; at 14,426 points/32 workers, 9,902,880 bytes across 415 allocations and 144 reallocations. Counters are enabled only for separate one-Window preflights, disabled for latency/profiles. These are cumulative requests, not live memory. Safe terminal conversion introduces no observed extra allocation here. The earlier standalone reuse result is not a portable allocation guarantee. Eventual integration would still need conservative admission accounting or the original memory-constrained fallback.

Maximum lifetimeRSS at 15,044 points/32 workers is 39.00/39.00/38.75 MiB original/OFF/ON; at 14,426 points/32 workers all three are 39.00 MiB. Benchmark fixture/reference/runtime storage and priming are included. Immutable base points are shared; per-worker scalar descriptors and independent reference/result vectors are validation overhead, not proposed production key duplication. These observations do not establish minimum RAM, whole-proof peak, mobile/ARM/TEE behavior, swap or pressure performance.

## Validation and disposition

Three new targeted tests pass, with 125 old core tests filtered/inherited. They compare original and lazy complete bucket vectors against projective sums over sizes 0–1,025, both Phi phases, multiple columns, duplicate/opposite/identity keys and custom batch sizes 8/9/31/32/33/200, plus repeated 15,045-point native Windows at 1/16/32 workers on 512 KiB stacks. Original points/descriptors remain unchanged. Prior field arithmetic populations are inherited rather than rerun. A whole-prover generic/no_asm/no_ifma compile also passes. No runtime branch-hit census or fresh proof verification is claimed.

The candidate snapshot recovers all 176 accepted sources by removing six explicit parent suffixes; 170 originals remain exact. The original control recovers the same 176 sources from three diagnostic suffixes; 173 are exact. The new kernel and Window reverse to their original bodies, and the field child is byte-exact from the qualified feasibility stage. The query selector is unchanged.339 workflow commands succeed, including 331 capture/offline steps and 300 caller processes. Benchmark loops execute405,414 observed plus 15,072 priming Windows (420,486 total), not proofs. All final/prime outputs match independent projective references. No prior successful test, model, recording, decoder or verifier is replayed for bookkeeping.

Two preparation failures are retained: the generator initially counted two canonical-subtraction sites rather than three, then its inverse audit treated an inherited binary asset as UTF-8. Both occur before native builds or timings. Fixed preparation validates source bytes and reversible edits. Native tests/builds/fallback/capture/analysis/review subsequently pass on first execution. Close this exact lazy-Y mechanism; no compiler/layout/field/geometry rescue or whole-SDK trial follows the failed gate.

## Original all-shape goal

Accepted SDK 6d83 is unchanged. These are inherited adoption results, not new measurements. The latest matched primary observation remains 8.027583 ms for the same binary previously measured below 8 ms.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 /4.667×|24 /3.027×|3.7|
|2×2|11.151932|59 /5.291×|30 /2.690×|5.9|
|4×2|16.086386|109 /6.776×|48 /2.984×|10.9|
|8×2|24.179453|189 /7.817×|74 /3.060×|18.9|
|12×2|33.884455|284 /8.381×|101 /2.981×|28.4|
|16×2|40.077381|335 /8.359×|124 /3.094×|33.5|
|24×2|65.850777|528 /8.018×|176 /2.673×|52.8|
|merge24×1|65.060937|507 /7.793×|175 /2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. The original all-eight verified CPU 10× goal remains active and unmet, 0/8. No new SDK/Gnark/AWS/Aeglos timing is established; the independent-proof ledger remains32,857.

[Protocol](artifacts/protocol.md), [review](artifacts/review.json), [source audit](artifacts/source-audit.json), [analysis](artifacts/analysis.json), [source/receipts/decoded profiles](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
