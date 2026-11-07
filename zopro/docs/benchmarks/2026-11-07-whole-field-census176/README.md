# Complete-request field-operation census

The accepted prover's complete 1×2 request evaluates **21,231,136 Fq vector lane products**, **1,070,736 Fr vector lane products** and **15,784 scalar Fq inverse calls** in this diagnostic. This fills the scalar, inversion and FFT exclusions of the earlier MSM-only census. It changes no production algorithm and establishes no new latency or memory result.

## Counts

Each shape runs three primes and two observed complete SDK requests, including proving, verification and serialization. All nine shapes use 32 workers on CPUs 0–31; primary also uses one worker on CPU 2. The table shows the first observed request. Both repetitions and every per-thread counter remain in the artifacts.

|Shape/workers|Scalar Fq backend multiplications|Scalar Fq fused sum2 calls|Fq SIMD lane products|Scalar Fq inverses|Scalar Fr backend multiplications|Fr SIMD lane products|
|---|---:|---:|---:|---:|---:|---:|
|1×2/32|570,133|108,396|21,231,136|15,784|113,692|1,070,736|
|1×2/1|570,161|105,346|21,231,136|15,784|113,692|1,070,736|
|2×2/32|363,862|95,085|26,364,640|21,769|194,432|2,233,072|
|4×2/32|295,615|77,854|35,604,488|21,945|337,109|4,678,480|
|8×2/32|315,951|73,969|54,097,912|30,063|589,376|9,836,736|
|12×2/32|336,182|79,670|73,078,632|38,374|562,359|20,236,800|
|16×2/32|351,661|78,801|90,450,368|45,765|718,611|20,668,952|
|24×2/32|434,364|78,621|127,610,304|59,984|1,042,653|42,517,888|
|merge24/32|384,558|78,250|120,838,680|58,709|984,982|42,429,424|
|merge54/32|588,697|84,391|253,779,144|109,377|2,203,735|88,891,360|

Scalar backend multiplications include API multiplication, square and conversion to canonical form. Scalar sum2 is separate: two elementary products share one reduction. Vector lane products weight an unfused product by its lane count and a fused product by twice that count; those weights are arithmetic counts, not interchangeable cost units. All observed vector products use the eight-lane backend; four-lane product counts are zero. API, backend, signed-operation, constructor and internal product counters overlap and must not be added indiscriminately.

For primary at 32 workers, 1,875,822 unfused and 389,035 fused Fq vector calls yield the reported lane products. Scalar Fq calls include 446,860 multiplications, 123,250 squares and 23 canonical conversions. Inversion records 143,468 31-bit GCD rounds, 28,052 31-bit coefficient updates, 129,442 62-bit updates and 509,240 one-word Euclid divisions. These integer operations are not additional field multiplications. Fq vector new/get counts are 1,781,866/750,904; they are not allocation counts.

The vector product counts and primary inverse-call counts match between the two repetitions and worker topologies. Scalar arithmetic and GCD details vary; raw differences are retained. The primary at 32 workers scalar sum2 count is 108,396/108,472, versus 105,346/105,372 at one worker. The accepted verifier permits three separate Miller loops at multiple workers and a shared recurrence at one worker, which is one source of different work; this census does not attribute every difference to that branch or prove a throughput or latency effect.

Fr/Fq vector lane-product ratios range from 5.04% for primary to 35.03% for merge54. Together with the existing sampled profiles, this supports retaining MSM/base-field work as a primary research target while keeping FFT work relevant for larger shapes. It does not establish equal operation costs, removable wall fractions, a memory bandwidth limit or a twofold optimization. Earlier packing, inversion, grouping, G1 finishing-loop fission and queue variants remain closed; this diagnostic does not reopen them.

## Fidelity and verification

The isolated snapshot adds 56 reversible observer insertions to six accepted parent files and one new counter module. Reversing those insertions recovers all 176 accepted hashes; 170 original files remain byte-identical. Arithmetic expressions, layout, keys, solver, resource admission, scheduling and pools stay unchanged. Production176 and SDK6d83 remain unchanged.

Each diagnostic thread has 960 u64 slots, 7,680 bytes plus TLS overhead. Reset/snapshot occurs outside each request on the caller and every current Rayon worker. No per-key cache, additional pool or retained duplicate execution plan is introduced. This storage is diagnostic-only. Counting side effects can retain operations otherwise eliminated by the compiler and perturb inlining/scheduling. These are evaluated method entries in the instrumented program, not accepted-machine instruction retirement. Portable integer helpers and constant-expression constructors are outside the backend-product counters.

One new focused native test passes on its first run; 35 existing tests are filtered and inherited. It checks API/backend nesting, thread isolation, disabling/reset, and both-prime/both-tag four/eight-lane products against Arkworks. The coherent eight-library SDK link is checked for `-O`, native CPU, fat LTO and one CGU before capture. An initial invalid xtask CLI invocation fails before running a workflow; its logs remain. The corrected native workflow and all preparation/capture/analysis/review stages succeed once. No successful qualification, decoder, profile, capture or verifier is replayed.

Ten processes make **50 SDK requests**,20 observed requests and **10 fresh independently Gnark-verified exports**. Proof size 164 bytes, public size 44 bytes, public/image/VK bindings and freshness pass. The canonical proof ledger advances **31,903→31,913**. No latency or RSS rows are emitted. No other owned CPU workload overlaps capture.

An independent reader validates 323 analysis-input pins, 25 completed command receipts, all 176 source reversals, coherent library/binary hashes,20raw per-thread sums, source conservation equations,10 fresh proof bindings and the resulting ledger. Its workflow has 402 pins including its own source/executable and completed analyses. It is not a second cryptographic verifier, disassembler or machine-instruction counter.

For each modulus, scalar backend mul=API mul+square+canonical conversion, add=API add+double, sub=API sub+neg; sum2/neg and GCD/API inverse entries match. In each vector configuration, unfused product entries equal mul and fused entries equal sum2+difference2. Signed scaling nests in mul. Every equation passes; other-modulus and unused slots are zero. Repetition differences are reported without forcing randomness-dependent work to be identical.

## Accepted CPU and Metal comparisons, inherited

These accepted local 32-worker complete-SDK observations and supplied M5 references are unchanged. They are not timings of the instrumented executable. All eight CPU 10× requirements remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.915095|37 /4.675×|24 /3.032×|3.7|
|2×2|11.169812|59 /5.282×|30 /2.686×|5.9|
|4×2|16.085882|109 /6.776×|48 /2.984×|10.9|
|8×2|24.517520|189 /7.709×|74 /3.018×|18.9|
|12×2|33.838626|284 /8.393×|101 /2.985×|28.4|
|16×2|40.083672|335 /8.358×|124 /3.094×|33.5|
|24×2|65.506198|528 /8.060×|176 /2.687×|52.8|
|merge24|65.200982|507 /7.776×|175 /2.684×|50.7|

Merge54 remains 161.718956 ms without an M5 reference. No fresh AWS/Aeglos, 250/4999, ARM/phone, concurrent-proof, pressure/swap, minimum-RAM or peak-memory result is added. No implementation is adopted. Goal remains active and unmet, 0/8.

[Artifacts](artifacts/) include frozen design, diagnostic/helper sources, changed source parents, plans, raw counts, receipts, hashes, analyses and independent review. Actual requests/images/keys/witnesses, proof/public-input payloads, executable/library blobs and build targets are excluded.
