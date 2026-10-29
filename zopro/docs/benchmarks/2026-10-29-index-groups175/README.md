# Compact index dictionary for scalar grouping

The component gate fails. For primary 1×2, A grouping is **3.094% slower** and B grouping **29.421% slower** than the source-extracted original. The equal-log aggregate across 18 real A/B input vectors is **6.344% slower**, with 0/31 winning blocks. Production remains unchanged; no SDK integration is selected.

## Mechanism and scope

The temporary dictionary stores eight-byte fingerprint/class-index entries and checks every matching fingerprint against all four words in the existing output scalar array. It replaces standard HashMap probing with scalar linear probing, a maximum 7/8 load and geometric growth. The existing value/class/count/order arrays, stable first-occurrence ordering, zero handling and final admission cutoff remain. There is no new persistent key table, point copy, scalar arithmetic, unsafe code or thread pool.

This differs from the earlier [borrowed-key map](../2026-10-25-borrowed-group-keys/README.md), which retained standard HashMap probing with 16-byte reference/index entries, and the [radix grouping trial](../2026-10-29-radix-groups175/README.md), which sorted fingerprint/index records. Their earlier outcomes remain unchanged. The initial research scan also found prior SRE, GLV, FFT and inversion experiments, so those mechanisms were not rerun. No external implementation is copied.

The original builder is extracted from accepted source175, adapting only its constructor signature, result/reservation type and supplied AHash state. Candidate edits replace dictionary allocation and insertion. Sampling and resource admission precede both builders and are outside their timers. There are no production-source edits. [Extraction audit](artifacts/extraction-audit.json).

## Real scalar constructor measurements

Ryzen 9950X, main thread pinned to CPU2. The frozen accepted solver derives A and B vectors for all nine fixtures; raw-key and freshly serialized solver-image solutions agree for every shape. Solver images are constructed in process from pinned keys. The first two shapes use canonical words and the remaining shapes Montgomery words, following their production entry geometry. All 18 samples admit the builder; both primary populations subsequently decline at the original full-cardinality cutoff. No witness/scalar arrays are exported.

Thirty-one fixed blocks rotate/reverse contexts and alternate variant order. Each cell averages eight separately timed constructors after three primes per context/variant. Matching variants share deterministic AHash seeds for each block/call, with no seed selection. Timers include dictionary allocation, hashing, insertion, growth and temporary-map destruction, plus class/output construction when grouping succeeds. Returned metadata is checked and destroyed outside the constructor timer. These are constructor measurements, not proof latency, query-stage CPU costs or a concurrency qualification.

Every prime and timed output matches an independent BTreeMap oracle, including all output fields and stable input lists. Ratios pair block averages and need not equal ratios of displayed marginal medians.

|Input|Original ms|Candidate ms|Paired ratio|Bounds|Wins / 31|
|---|---:|---:|---:|---|---:|
|1x2/A|0.166538|0.172043|1.030937|[1.026317, 1.036687]|2|
|1x2/B|0.257059|0.332823|1.294211|[1.291687, 1.296271]|0|
|2x2/A|0.269155|0.315455|1.172647|[1.164374, 1.177799]|0|
|2x2/B|0.402423|0.407452|1.011790|[1.007038, 1.013390]|0|
|4x2/A|0.420869|0.491436|1.167314|[1.165611, 1.170698]|0|
|4x2/B|0.648606|0.643864|0.994371|[0.992560, 0.996450]|29|
|8x2/A|0.770060|0.882297|1.144126|[1.141038, 1.149436]|0|
|8x2/B|1.180348|1.193540|1.010961|[1.007403, 1.012521]|1|
|12x2/A|1.261061|1.378550|1.093984|[1.088846, 1.099621]|0|
|12x2/B|1.883098|2.649406|1.407921|[1.405413, 1.411551]|0|
|16x2/A|1.616264|1.728919|1.069668|[1.064198, 1.077382]|0|
|16x2/B|2.293433|2.244354|0.977756|[0.974760, 0.982709]|31|
|24x2/A|2.380471|2.579676|1.076009|[1.071964, 1.096249]|0|
|24x2/B|3.879669|3.823125|0.984828|[0.983357, 0.986879]|31|
|merge-24x1/A|2.272495|2.288955|1.005212|[0.999131, 1.028931]|12|
|merge-24x1/B|3.670107|3.464031|0.943820|[0.938806, 0.947004]|31|
|merge-54x1/A|5.704424|5.421523|0.949695|[0.947156, 0.953199]|31|
|merge-54x1/B|9.980101|9.085352|0.911521|[0.905886, 0.913343]|31|

The 18-context aggregate ratio is 1.063441 [1.061641, 1.064841]. Six individual inputs improve, including both 54→1 vectors; all positive results remain above. They do not justify selecting a favorable subset after capture. The prospective gate requires both primary upper bounds below parity with at least 21/31 wins, every context upper bound≤1.03, and an improving aggregate upper bound with at least 21/31 wins. Both primary gain checks, nine nonregression checks and the aggregate gain check fail.

Paired order ranks 10–22 of 31 give nominal 97.055% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No gain floor, exclusion, extension, threshold change or unchanged reroll is used. The smaller entry representation does not establish a faster dictionary. A possible next mechanism would preserve grouped control-byte probing while storing class indices; it needs a separate design and qualification, rather than changing this completed trial.

## Validation and CPU profiles

Three new tests cover forced identical hashes, wraparound/growth, equality across all four words, stable whole outputs against the ordered oracle at zero/duplicate/distinct/cutoff edges, and dictionary-buffer bounds. Two inherited FIFO tests also pass, five unfiltered tests total. Existing accepted field/core/SDK libraries are frozen and unchanged; no new full-prover suite is claimed. Test and benchmark executables both use native targeting, fat LTO and one codegen unit.

The first test compilation fails on an ambiguous integer in a test closure before any test or measurement. Adding its usize annotation fixes the harness. The first linked-code audit expects separate builder symbols, but the compiler inlines both into the common emitted build function. The audit is corrected to retain that actual function; the measured binary is unchanged and no successful build or test repeats. Both initial sources/observations and repairs are retained. No inline/layout/flag tuning follows.

All 8,928 timed constructors and 108 primes finish before two separate FIFO-gated 199Hz cycles:u profiles, each 8,192 primary-B builders. All profile captures precede owner-user decoding and loss checks. Each has one sampled PID and zero lost records. The profile loop checks primes and a post-profile output; individual profile outputs are dropped, rather than separately retained/verified.

|Profile|Samples|Lost records|
|---|---:|---:|
|original|332|0|
|candidate|524|0|

The original/candidate loops share an emitted build symbol. Flat symbol periods are retained and cannot isolate a probing/cache cause or represent removable proof wall time. No owned build/helper/hash/archive or second benchmark overlaps captures.

The dictionary uses eight bytes per bucket, versus 40-byte original key/value payload plus control storage on this host. Old and new candidate buffers at growth fit the existing conservative map forecast in tested integer-bound calculations; other output buffers remain. This is an allocation-payload bound, not allocator overhead or process-RSS savings. The shared latency process peaks at 3,181.98 MiB over loading/solving all nine keys and benchmarking; it is not per-variant prover RSS or minimum RAM.

## Accepted prover and reference comparisons

No complete SDK proof or independent Gnark export is created. The distinct-proof ledger remains **20,881**. Accepted production source175 and SDK `20fde…` remain unchanged. The following complete-proof measurements and supplied historical M5 references are inherited, rather than measurements of this dictionary. The full accepted performance gate retains its documented physical-16 calibration exception; its final-artifact gate remains true.

|Shape|Accepted local ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|134.00|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|275.00|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|315.00|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|512.50|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|707.25|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|826.25|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|1328.75|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|1175.50|507|7.767×|175|2.681×|50.7|
|54→1|162.250464|1831.87|—|—|—|—|—|

The original all-eight CPU 10× objective remains active and unmet, **0/8**. Latest AWS results remain [11.916 ms Zopro versus 7.959 ms Aeglos for 1×2](../2026-10-29-current-cpu-summary/README.md), at each engine's fastest tested worker count. No new AWS, phone/ARM/TEE, forester 250/4999, memory-pressure, swap, minimum-RAM or concurrent-throughput result is claimed. This exact dictionary is closed without SDK integration or production adoption.

[Full blocks and profile weights](artifacts/analysis.json), [declared design](artifacts/design.md), [measurement plan](artifacts/run-plan.json), [closing pins](artifacts/pins.json), [compressed source and receipt archive](artifacts/source-and-receipts.json.gz).
