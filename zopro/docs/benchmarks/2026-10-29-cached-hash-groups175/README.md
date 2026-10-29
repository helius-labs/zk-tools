# Temporary cached hashes for scalar grouping

29 October 2026. The new component gate **passes**. Production remains unchanged. This tests a new implementation motivated by the [frozen 54→1 CPU diagnostic](../2026-10-29-representative-54-diagnostic175/README.md), where representative-table growth has substantially more sampled residence than the original inline-key dictionary. The previous [constructor](../2026-10-29-representative-groups175/README.md) and [all-nine SDK](../2026-10-29-representative-sdk175/README.md) gates remain failed.

The all-18-input aggregate improves **5.17% versus original** and **5.06% versus the uncached parent**, with 31/31 wins against each. All 18 original-relative nonregression checks pass. Primary A/B improve 21.27%/25.26% versus original; 54→1 A/B improve 6.11%/5.56% versus parent. These are constructor results, not proof-speed gains.

## Implementation and memory

A proof-local Vec<u64> retains the hash already computed for each distinct nonzero class. HashTable<u32> growth reads hashes[class] instead of following a representative row and hashing its four input words again. Representative row/count records stay eight bytes; equality still compares all four original words, including under hash collisions. First-occurrence order, class counts, stable row lists, zeros, exact cutoff, sample policy and initial reservation remain. The original input-length bound below u32::MAX remains.

Hashes and the dictionary are dropped before admitted scalar materialization. The first phase adds eight geometric temporary bytes per class and fits the source-extracted original map/value/count/class forecast. After hash destruction, the earlier representative materialization bounds are unchanged, including independent 32/64-bit payload checks. Checked overflow paths are retained. These are typed allocation bounds, not measured minimum RAM or allocator overhead. No persistent key table, execution plan, point copy, pool, cross-query barrier, unsafe code or field operation is added.

## Measurement method

Ryzen 9 9950X, main thread pinned to CPU 2. All 18 real A/B inputs from nine fixtures are retained, including policy-declined contexts. Frozen accepted raw-key and freshly serialized solver-image scalar outputs agree. The first two shapes use canonical words and the remainder Montgomery words, following the accepted entry geometry. No witness/scalar arrays are persisted.

The source-extracted original HashMap, exact previous uncached representative constructor and new cached-hash constructor share one new native fat-LTO executable and deterministic AHash seeds per block/call. Frozen accepted libraries and hashbrown 0.16.1 native artifact remain unchanged. This is one compiled-context comparison, not independent-build calibration.

Thirty-one fixed blocks rotate/reverse context and variant order. Each cell averages eight separately timed constructors after three checked primes/context/algorithm. The 13,392 timed calls and 162 primes all match a whole-output independent BTreeMap oracle. Timers include hashing, growth, allocation, hash destruction, admitted output materialization and stable packing; sampling/reservation and returned output checking/destruction are outside.

## Constructor results

Times are milliseconds, medians of block means. Paired ratios need not equal ratios of displayed medians; lower is faster.

|Input|Original ms|Parent ms|Cached ms|Cached/original|Bounds|Wins / 31|Cached/parent|Bounds|Wins / 31|
|---|---:|---:|---:|---:|---|---:|---:|---|---:|
|1x2/A|0.130632|0.104488|0.102606|0.787289038|[0.708031155, 0.791001979]|31|0.980674113|[0.974524330, 0.986135257]|26|
|1x2/B|0.207465|0.159524|0.154975|0.747385806|[0.746528512, 0.749347175]|31|0.972655290|[0.967749127, 0.975715691]|31|
|2x2/A|0.231829|0.232430|0.226464|0.977727470|[0.974877434, 0.980545865]|31|0.975534395|[0.968121080, 0.978388308]|31|
|2x2/B|0.345518|0.341153|0.334929|0.971105927|[0.965303060, 0.973684822]|31|0.980817752|[0.977430190, 0.985912548]|31|
|4x2/A|0.380028|0.397683|0.380010|1.000821503|[0.998269163, 1.002525001]|13|0.956055129|[0.949721683, 0.962261187]|31|
|4x2/B|0.599570|0.595928|0.571894|0.953611491|[0.952745322, 0.956735050]|31|0.957930987|[0.950174319, 0.962915270]|31|
|8x2/A|0.727594|0.769948|0.728805|1.002444709|[0.995993356, 1.005667137]|11|0.947067933|[0.938734895, 0.954770632]|31|
|8x2/B|1.110213|1.127449|1.071975|0.963740096|[0.961152367, 0.970709542]|31|0.949284999|[0.943297242, 0.957887030]|31|
|12x2/A|1.196341|1.275954|1.194480|0.997705738|[0.995750789, 1.005812526]|18|0.937677879|[0.933309683, 0.945015645]|31|
|12x2/B|1.810107|1.881237|1.769181|0.975534111|[0.974296800, 0.982710797]|31|0.941736359|[0.930153137, 0.948222889]|31|
|16x2/A|1.552600|1.646662|1.541613|0.992347536|[0.987081471, 0.995292714]|22|0.934771067|[0.923398358, 0.937660136]|31|
|16x2/B|2.172208|2.216500|2.097650|0.963150340|[0.961046881, 0.973591669]|31|0.944845572|[0.931720593, 0.956246724]|31|
|24x2/A|2.335096|2.469233|2.309892|0.988389159|[0.984373057, 1.010187739]|21|0.935111844|[0.913261524, 0.955679687]|31|
|24x2/B|3.739860|3.763906|3.526403|0.943665176|[0.938464281, 0.945563198]|31|0.937930499|[0.926587504, 0.944369182]|31|
|merge-24x1/A|2.194524|2.309759|2.163430|0.985585369|[0.979163865, 1.015169440]|20|0.937309741|[0.914087926, 0.964919233]|31|
|merge-24x1/B|3.476199|3.569274|3.301970|0.948847049|[0.945481441, 0.958078967]|31|0.928727194|[0.914513259, 0.940378562]|31|
|merge-54x1/A|5.490706|5.709014|5.349131|0.974300407|[0.969211645, 0.980926636]|31|0.938931516|[0.931499927, 0.944598579]|31|
|merge-54x1/B|9.309407|9.768386|9.232281|0.993399920|[0.987489745, 0.996187553]|26|0.944444648|[0.939419187, 0.946655194]|31|

|All 18 equal log aggregate versus|Ratio|Bounds|Wins / 31|
|---|---:|---|---:|
|original|0.948305731|[0.946040180, 0.952979918]|31|
|parent|0.949390753|[0.947929195, 0.951318569]|31|

The new prospective gate requires primary A/B upper ratios <1 with at least 21 wins against original, every candidate/original upper ≤1.03, all-18 aggregate upper <1 with at least 21 wins against original and parent, and both 54→1 A/B upper <1 with at least 21 wins against parent. Other individual parent comparisons remain descriptive because parent is not adopted. Paired ranks 10–22/31 give nominal 97.055% individual median coverage under independent/exchangeable blocks, not a familywise guarantee. No gain floor, exclusion, extension, seed/layout selection, post-result subset promotion or threshold change is applied.

Failed checks:

None.

## Correctness and profiling

Nine unfiltered native component tests pass. The new counting-BuildHasher test proves exactly one hash per nonzero input with forced table growth, multiple initial capacities and zero values. Existing component collision, all-four-word equality, immutable input, exact cutoff/occupied-limit/later-new-value, stable whole-output, resource/overflow and FIFO checks are reused. Field/core/SDK libraries and their qualification are inherited unchanged.

All latency finishes before three separate FIFO-gated 199 Hz cycles:u profiles of primary B, original/parent/candidate, 4,096 constructors each after primes. CPU IDs and timestamps are explicit. All captures precede ownership transfer, owner-user decoding, event and lost-record audits. No owned compiler/helper/hash/archive or second CPU workload overlaps capture. This stage has no new 54→1 profile or PMU panel.

|Profile|Samples|Lost records|
|---|---:|---:|
|candidate|117|0|
|original|165|0|
|parent|127|0|

Each profile has one sampled process and zero lost records. All 16 capture/offline workflow steps complete. Linked code, native receipts, full observations, failed checks, exact source/dependency pins and archives are retained. Raw profiles and binaries stay local with hashes. Source and receipt archives round-trip byte-exactly. Flat samples do not isolate exclusive cache or allocator costs.

## Decision and prover comparisons

This component result selects a separate all-nine accepted/OFF/ON SDK qualification. It does not adopt the constructor or establish a proof-speed gain.

No SDK request, proof export or Gnark verification is added. The distinct-proof ledger stays **21,703**. The original all-eight verified CPU AVX single-proof 10× objective remains active and unmet, 0/8. Accepted local 1×2 remains 8.022165 ms and 134 MiB peak RSS; latest AWS remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96. The accepted physical-16 calibration engineering exception and separately passed final-artifact gate remain unchanged. [All nine local/AWS rows and original eight supplied CPU/Metal references](../2026-10-29-current-cpu-summary/README.md).

This stage adds no AWS, mobile/ARM/TEE, forester 250/4999, minimum-RAM, swap/pressure or concurrent-throughput result.

[Protocol](artifacts/design.md), [all observations](artifacts/analysis.json), [cached-hash implementation](artifacts/deferred.rs), [exact parent constructor](artifacts/parent.rs), [source extraction audit](artifacts/extraction-audit.json), [source archive](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [publication review](artifacts/publication-review.json).
