# Block-local G2 gap factoring: native caller results

**The fixed candidate is not adopted.** It reduces modeled complete G2 additions by6.18% for1×2 and0.67% for2×2, and passes actual curve checks, but is substantially slower at16/32workers. Primary32 complete-query latency is **3.811480ms OFF versus16.783548ms ON**; paired ratio4.411321 [4.359218,4.501369],0/15wins. Primary32 query-process peak RSS rises **32.00→125.75MiB**. All12 diagnostic-quality checks pass;13/22 total caller checks pass. No new full-proof latency is measured, and accepted production remains unchanged at about **7.77ms for1×2**.

After qualified scalar grouping and signed Phi splitting, sort each component's nonzero/nonidentity terms by magnitude, with original-index ties, and partition into fixed256-record blocks. Within[b,e], let Uⱼ=Σᵢ₌ⱼᵉQᵢ. Then Σᵢ₌ᵦᵉmᵢQᵢ=mᵦUᵦ+Σⱼ₌ᵦ₊₁ᵉ(mⱼ−mⱼ₋₁)Uⱼ. Local suffixes replace global suffixes; full-width block-base correction terms account for every omitted cross-block offset. This changes the representation rather than retesting the closed global Brent–Kung scan. The existing weighted reducer's256-point geometry was fixed before counting; no geometry sweep is selected.

|Generic G2 work ledger|1×2 original|1×2 factored|2×2 original|2×2 factored|
|---|---:|---:|---:|---:|
|Complete modeled point-add calls|634486|595291|674949|670420|
|Nonzero digit arrivals|613891|520031|633695|573193|
|Local suffix joins|0|56180|0|57461|
|Windows|11|11|11|11|

All222/226 high-bit corrections are included. Gap coefficients reach115bits, but corrections reach126bits, so eleven width12 windows remain. Both variants retain846 active weighted/Horner doublings. Counts include grouping, generic noncancelling fill/part merges, actual occupied-bucket weighting, Horner and actual nonidentity local scan joins. They do not price affine/projective/exceptional operations equally in native code, or include sorting, inverse-stage latency, allocation, zero-word scans and scheduling. All six necessary algebra/work/payload checks pass; this permitted the native experiment, not adoption.

The native candidate computes each component's local suffixes with255 dependent stages using the existing affine AVX G2 stream and prefix-record kernel, complete-formula exceptional handling, and the original field. Components run through rayon::join in the existing pool. Compacted low/high point and word arrays are concatenated, with unused component words zero, then consumed by the original Phi bucket-Horner fill/top/merge/weighted/Horner path. Width12, two partitions and eleven windows remain fixed. All preprocessing and layout costs are inside the caller clock. No persistent key table, expanded solver plan or new production pool is added.

|Complete G2 query context|OFF ms|ON ms|Paired ON/OFF [bounds], wins|OFF / ON peak RSS MiB|
|---|---:|---:|---|---:|
|1x2-w1|55.119762|51.324220|0.930288 [0.926256,0.932000],15/15|15.25 / 29.50|
|1x2-w16|5.277048|16.315575|3.099457 [3.057927,3.118316],0/15|26.25 / 94.75|
|1x2-w32|3.811480|16.783548|4.411321 [4.359218,4.501369],0/15|32.00 / 125.75|
|2x2-w32|4.290684|18.276107|4.237289 [4.207210,4.364248],0/15|59.75 / 158.25|

Solo is about7% faster, but the sixteen-worker ratio is3.099457 and both32-worker shapes exceed4×. Equal-log two-shape32 ratio is4.330676 [4.280051,4.400502],0/15wins. The fixed rule fails all four RSS guards, three parallel nonregression checks, primary32 gain and aggregate gain. Solo nonregression and all twelve diagnostic checks pass. No favorable-worker adoption, block-size/scan-family rescue, threshold change or unchanged rerun follows. The result does not establish that every possible gap representation is ineffective.

These are complete G2 query timings, not full proofs. The driver calls the real g2_query_exclusive consumer, including scalar reset copy, grouping, Phi split, sorting, local scans/corrections, allocation/drop, complete MSM and final normalization. Setup, oracle checks, three checked primes and final output validation are outside the timer. Each reported latency is the median of fifteen arithmetic means of seventeen integer-nanosecond samples. Paired bounds are zero-based ranks3/11, nominal96.484375% marginal median coverage conditional on independent exchangeable blocks; no familywise/stationarity guarantee. Every block is retained.

Ryzen9950X,16physical/32logical CPUs, shared host. The process is taskset-constrained; worker i pins to CPUi, solo CPU2. Query drivers use default worker stacks with no explicit stack_size and no RUST_MIN_STACK. Both coherent SDK-style driver links use native/O3/fat-LTO/oneCGU and original field1200, dense busy reset, the same allocator/environment and original176 sources plus three reversible private parents and two child modules. OFF excludes the transform at compile time. This is a two-arm caller screen; passing would still require fresh accepted-control/full-SDK ALL8+54/Gnark and admission/fallback qualification.

|Diagnostic context|OFF / ON user cycles per query|OFF / ON instructions per query|OFF / ON median CPU ms/query|
|---|---:|---:|---:|
|1x2-w16|321383429 / 311037915|545724918 / 804847059|62.916 / 60.677|
|1x2-w32|405874939 / 402271774|565313754 / 832030602|83.826 / 82.682|
|2x2-w32|440509191 / 465194746|593014955 / 918558324|90.607 / 95.672|

The primary32 panel has slightly fewer cycles but about47% more instructions; latency still rises sharply. Reduced addition counts and solo CPU do not imply preserved parallel critical path. The implementation exposes255 dependent affine stages per component; low/high construction initially uses only two tasks. This is a plausible explanation for the lost parallelism, not a causal isolation of every effect. Concatenated zero-word walks, extra arrays, exceptional checks, allocator history and worker waiting also remain in the measured consumer. Flat samples cannot assign all wall time to one function, and aggregate user cycles are summed across threads.

All six PMU panels have100% scheduled coverage. Six fixed16M cycles:u profiles retain36,647 samples, zeroLOST, correct sampled PID/TID/CPU/period, executable mapping, at least1000samples and below5%unknown each. Automatic captures occur after latency, with setup/primes and final checks outside FIFO gates. All captures finish before ownership transfer/decoding/analysis. Absolute period coverage is unvalidated. Categories use raw symbol substrings and are descriptive: the common block_gap_probe API name also appears in OFF, so the block_gap category is not an exclusive preprocessing cost. Raw top symbols are retained.

Memory is materially worse. The modeled logical new payload was12,182,832/12,460,392bytes plus unchanged scalar words, not a bound on native capacity/RSS. Actual scratch, concatenation capacity, temporary overlap and allocator retention exceed that model, especially across workers. Observed RSS includes fixture setup and primes; it is not minimum RAM or full-proof peak. The private candidate has no new checked workspace admission/fallback integration and cannot be promoted as a universal low-memory implementation. No architectural RAM cap is introduced.

Correctness includes544 integer block identities,32 curve exceptional cases, four new component Arkworks MSMs matching two retained independent complete-query hashes, and128 unfiltered native core tests with16,695 new suffix comparisons. The separate scalar/recursive-work reader checks2,506,295 bitwise digits,341,934 coefficient reconstructions and44window-work rows. All4,160 native queries including primes match retained independent Arkworks sums, and input points remain unchanged. Old solver/FFT/fixture/oracle production is not repeated. No new proof is exported.

The producer and separately compiled rank reader are frozen before preflight. The reader uses u128 integer folds and rank selection rather than the producer's u64 sum and sorting, and reconstructs all120 raw means and22 decisions. They share source, receipt parsing and detailed profile/PMU parsers; this is a same-agent cross-check, not an external independent audit. Both post-capture executions pass first time. All176 production files remain exact; private source recovery checks173 unchanged originals, three reversible parents and the two new children.

There are188 attempted serialized xtask commands through both analyses:187pass and one test-only compile failure; one subsequent step in that failed plan was unattempted. The test initially called Affine.double, corrected to to_projective().double. The two successful OFF/ON builds are retained; non-test source bytes are unchanged. Corrected test compilation/all128tests pass in a new two-step plan. All failed/successful receipts are preserved. Direct preparers/compiler invocations and publication/checkpoint are separate. During functional preflight and recording only the exact live handle is polled; no owned build/helper/hash/analysis/report overlaps capture. Shared-host exclusivity is not claimed.

The following full-proof observations are inherited from the accepted retained-tail SDK campaign, not this caller trial. Supplied M5 CPU/Metal references are historical and on different hardware, not newly measured or equal-hardware efficiency comparisons.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.772288|37 / 4.761×|24 / 3.088×|3.7|
|2x2|10.929386|59 / 5.398×|30 / 2.745×|5.9|
|4x2|15.731643|109 / 6.929×|48 / 3.051×|10.9|
|8x2|23.965774|189 / 7.886×|74 / 3.088×|18.9|
|12x2|33.742953|284 / 8.417×|101 / 2.993×|28.4|
|16x2|39.704242|335 / 8.437×|124 / 3.123×|33.5|
|24x2|66.034077|528 / 7.996×|176 / 2.665×|52.8|
|merge-24x1|65.387417|507 / 7.754×|175 / 2.676×|50.7|

Accepted primary remains7.772288ms/112.75MiB at32workers and9.049269ms at16. Same accepted executable was already below8ms. Merge54 remains162.096312ms/1989.32MiB without a supplied M5 reference. Canonical45,335 verified-proof inventory is unchanged; every future export must seed retained-tail-sdk176/proof-sha256-after-capture.json. No new AWS/Aeglos, ARM/mobile/TEE, huge250/4999, pressure/swap, minimum-RAM or concurrent-throughput result is added. Full goal remains active and unmet,0/8.

Next hypotheses must reduce main work while pricing dependency depth, native live arrays and complete-consumer parallelism before promotion. This fixed recipe is closed.

[Model](artifacts/analysis.json), [model/scalar reader](artifacts/review.json), [complete-query results and diagnostics](artifacts/caller-analysis.json), [rank reader](artifacts/caller-review.json), [decision](artifacts/decision.json), [protocol](artifacts/caller-protocol.md), [algebra design](artifacts/design.md), [native design](artifacts/native-design.md), [source recovery](artifacts/native-source-edits.json), [test repair](artifacts/native-test-repair.json), [receipts](artifacts/receipts.json), [inherited CPU/Metal comparisons](artifacts/inherited-comparison.json), [continuation](artifacts/next.md), [evidence archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
