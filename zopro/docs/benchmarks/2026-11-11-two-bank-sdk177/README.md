# Two-bank short SDK screen

**The full-proof screen is inconclusive; no candidate is adopted.** The
[complete K/H caller](../2026-11-11-two-bank-caller177/README.md) improved in all
27 matched pairs, but that benefit does not consistently appear in verified SDK
latency. At32 workers, displayed medians are7.842491ms accepted,
7.870098ms fresh control and7.883080ms two-bank. Peak RSS rises with the candidate.

|Case|Accepted REF ms / RSS MiB|Fresh OFF ms / RSS MiB|Two-bank ON ms / RSS MiB|
|---|---:|---:|---:|
|1×2,16workers|8.930733 /75.00|8.934459 /74.75|8.892308 /76.25|
|1×2,32workers|7.842491 /93.75|7.870098 /96.50|7.883080 /102.00|
|2×2,32workers|11.016577 /238.75|10.831045 /230.00|10.953237 /234.00|

Times are medians of nine process arithmetic means per arm/context, each from
17requests after three primes. RSS is the maximum lifetime peak including
loading/primes; it is not minimum RAM or comparable to a longer campaign's peak
without matching its request count. Primary32 whole-loop CPU ms/request is
177.972REF,177.500OFF,177.937ON. All recorded major-fault counts are zero.

|Case|Paired median ON/OFF|ON/OFF wins|Per-backing ON/OFF medians|Paired median ON/REF|
|---|---:|---:|---|---:|
|1×2,16workers|0.998711|5/9|0.998711,1.010548,0.995462|0.992342|
|1×2,32workers|1.012142|3/9|0.991949,1.020567,1.010699|1.009107|
|2×2,32workers|1.008415|2/9|1.014341,1.008415,1.000396|0.991486|

The prospective selection rule requires all three backing medians to improve
against both OFF and REF for a primary context. Neither primary context meets
it. Mixed signs are inconclusive, not a demonstrated implementation regression
or proof of zero benefit. The unchanged2×2 path also varies between ON/OFF
processes using identical executable bytes; this screen cannot attribute such
variation to two-bank arithmetic. No samples, contexts or executable copies were
dropped, and the fixed sample count was not extended. There is no adoption gain
floor or precise small-gain confidence claim.

## Implementation and validation

The unchanged qualified two-bank Window and noninstrumented balanced feeder are
copied from the caller screen. A diagnostic-only environment switch routes
bits13 fusedK/H to that feeder. OFF retains the same switch lookup but uses the
original caller. No field kernel, key format, blinding, proof semantics or
production source changes. Source routing confirms the primary uses fusedK/H;
this run contains no sampled execution trace or causal profile.

REF is accepted SDK `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`.
OFF and ON share fresh SDK
`6f1312c7071bb3fbb5f3a44b431d2c11de37532b68e3346923f9ce94bdc2d6d0`.
Each binary has three independently written, inode-distinct backings. Rust1.98.1,
znver4, accepted cfg flags, O3, fat LTO, one codegen unit, default SDK stacks and
accepted allocator settings are retained. CPU masks are fixed0–15 or0–31 on the
local Ryzen9950X. Loading, setup, three primes and export are outside timing;
fresh assignment, solving, randomness, proving, SDK verification and serialization
are inside. No helper/build/hash/report runs overlap live preflight/recording.

Nine preflight and81main processes complete **1,656 SDK requests**, including
primes, with1,386 timed samples. All90 exported164-byte proofs have expected
public-input/image bindings, are fresh against the canonical ledger and pass
independent Gnark verification. Native edge/queue/merge qualification is inherited
from the unchanged Window. This is not all-shape, portable, phone, pressure,
concurrent-throughput, minimum-RAM or release qualification.

The reader checks453 pin entries, executable identities, frozen arguments,
stream receipts, sample counts, worker masks and proof-verifier bindings.
The canonical ledger advances **53,917→53,926→54,007**. Its new SHA-256 is
`aab8d06c2d269b1ecbeede37289c505a81a13e17a0e1936c8f32ccff39b4c495`, at
`/home/ubuntu/src/zopro-perf-work/two-bank-sdk177/proof-sha256-after-main.json`.
No prior successful science is replayed.

Preparation from protocol creation to linked SDK takes about185seconds. Cargo
build39.757s, successful link16.429s, main capture18.537s, main verifiers1.387s.
Initial link attempts fail on a duplicate edition flag and a missing host
proc-macro search directory. A plan then encounters an older xtask binary's
unsupported optional field; the compatible rustc-flags form succeeds. Failures
and exact Cargo library/proc-macro pins are retained, and the successful Cargo
build is reused. These are build/harness failures, not proof failures.

## Decision and next question

Preserve the candidate and its positive caller evidence, but defer a larger
all-shape qualification. It still needs reliable integrated benefit and resource
admission/fallback accounting before adoption. Do not tune more bank constants
based on this noisy full-proof screen. The isolated gain at32workers was about
71µs in displayed medians, much smaller than the remaining4.122ms primary goal
gap, and cannot be assumed to survive concurrent proof phases.

Return to the structural parallel-execution question using retained matched C7a
evidence: at32workers, the prior accepted Zopro uses less total CPU time than
Aeglos, while at96workers its CPU cost and coordination residence rise sharply.
The [matched C7a report](../2026-11-10-accepted-current-c7a176/README.md) supports
investigating work availability and amplification; it does not establish that
all sampled coordination time can be removed. Select a distinct intervention
that tests that explanation before another arithmetic or bucket variant. This
screen supplies no new AWS/Aeglos result.

Accepted production remains unchanged. The latest all-eight accepted CPU/M5
CPU/Metal comparison remains in the
[cost map](../2026-11-11-cost-map177/README.md#accepted-full-proof-reference).
The goal remains active and unmet0/8. No push or publication is authorized.

[Protocol](artifacts/protocol.md), [analysis](artifacts/analysis.json),
[main review](artifacts/main-review.json), [reader](artifacts/review.rs),
[plans, source changes, receipts and proof identities](artifacts/evidence.tar.gz),
[checksums](SHA256SUMS). Binaries, keys, witness/proof payloads, unchanged full
source snapshots and full inherited ledger are excluded; reproduction requires
the pinned local inputs and dependencies.
