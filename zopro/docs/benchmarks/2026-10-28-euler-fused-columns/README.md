# Consume Euler output in the final FFT columns

The candidate **failed the primary SDK screen** and was not adopted. At 32 workers, ON takes 8.509658 ms versus 8.519286 ms accepted and 8.486726 ms fresh OFF. Paired ON/accepted and ON/OFF median ratios are 1.000533 and 1.003273, with both intervals crossing parity and only 7/15 and 6/15 wins. The physical-16 OFF/accepted calibration interval also exceeds its declared bounds. All observations remain in the tables.

Eliminating final derivative writeback and a separate output pass did not establish a full-proof gain. Total CPU time at 32 workers is also effectively unchanged in this run. The implementation trades those stores and dispatch for two-column scratch, strided A/B access and different scheduling; the profiles do not isolate which cost offsets the removed work. This closes the tested fusion without an unchanged layout, grain or register reroll. The next investigation should return to useful MSM work on the critical path, rather than assume another output-pass rearrangement can deliver the 10× target.

The [compaction trial](../2026-10-28-euler-compact/README.md) rejected checked lane gathers. Its faster old-arithmetic tile control prompted a history audit: the [earlier FFT-pass-grain trial](../2026-10-26-fft-pass-grain/README.md) already tested a related 128-row splitting floor and failed its full gate. This experiment changes dataflow rather than rerunning a grain parameter.

The existing Euler forward transform completes its same 32 child transforms. The new terminal helper expands two adjacent final columns, runs the unchanged column butterflies, and computes the accepted eight-row `DA*B+A*DB` directly from those results. It writes A without packing final derivative values back into the temporary vector and reading them in a separate output pass. Safe disjoint output stripes preserve prefixes and zero-padded B tails without extending A. There is no new unsafe code, field formula, key table, execution plan or input-sized buffer. Two 32-Pair columns require roughly 20 KiB of explicit arithmetic scratch per active terminal call, versus the old single-column 10 KiB; compiler frames are additional. The never-inline terminal helper keeps scratch outside recursive iterator frames. Domains at most 4096 fields, unsupported output mode and OFF preserve the old path. The private process flag defaults OFF.

Only one of 172 original source files changes reversibly, plus one new module. Both OFF and ON pass the complete 122-test native core suite serially, avoiding the previously observed global allocator-test interference. Three new cases check full-width scalar-product oracles after the retained forward transform across four domains, cached/uncached roots, short prefixes/B extents; padded output row conservation; and concurrent shared-root callers with one/two/four-worker pools and 512 KiB stacks. Existing independent polynomial derivative tests also pass. Raw-image two and cubic six prover tests pass with explicit required names. Field source and its qualification are inherited; no field-suite rerun is claimed.

Fifteen fixed rotated/reversed blocks compare the accepted SDK and the same fresh binary OFF/ON at 32 logical and 16 physical workers. Each latency process performs three primes then 151 measured verified SDK requests. Three four-worker preflights precede latency; ON confirms four actual `n=32768 rows=30922` fusion entries. Key loading is excluded; fresh solving, randomness, proving, SDK verification and serialization are included.

| Workers | Variant | Median block ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |
|---|---|---:|---:|---:|---:|
| 32 | control | 8.519286 | 191.242974 | 130.25 | 133.75 |
| 32 | off | 8.486726 | 191.263212 | 131.25 | 134.50 |
| 32 | on | 8.509658 | 191.403146 | 133.75 | 139.50 |
| 16 | control | 10.857978 | 125.190907 | 88.50 | 91.25 |
| 16 | off | 10.805778 | 124.476305 | 88.00 | 93.50 |
| 16 | on | 10.811630 | 125.628119 | 90.50 | 97.25 |

| Workers | Paired ratio | Median | Conditional interval | Wins | Check |
|---|---|---:|---:|---:|---|
| 32 | off/control | 0.999547180 | [0.991996237, 1.005023696] | 8/15 | pass |
| 32 | on/control | 1.000533290 | [0.994850273, 1.005121811] | 7/15 | fail |
| 32 | on/off | 1.003273089 | [0.995901991, 1.007713297] | 6/15 | fail |
| 16 | off/control | 0.997758901 | [0.961625856, 1.039613110] | 8/15 | fail |
| 16 | on/control | 0.996797599 | [0.953883047, 1.013640569] | 8/15 | pass |
| 16 | on/off | 1.000541562 | [0.969477999, 1.009981869] | 6/15 | pass |

Ratios pair the 15 process medians. The 4th–12th ordered bounds have nominal 96.484375% coverage under independent exchangeable blocks, without a familywise guarantee. OFF/accepted calibration must fit [1/1.03,1.03]. Primary ON requires upper ratio below one and at least 11 wins against both controls; physical-16 upper ratios must be at most 1.02. There is no gain floor, exclusion, extension or unchanged reroll.

All 90 latency processes precede six separate FIFO-gated 199 Hz cycles:u profiles with 251 requests after three primes. All 99 SDK captures precede independent Gnark verification, profile decoding, hashes and archive work. No owned helper, build or other CPU benchmark overlaps capture. Every profile has one sampled SDK PID and zero lost records; demangled symbols and raw profile hashes are retained. Fused-symbol share includes butterfly and output work and is not a removable wall-time fraction.

| Workers | Variant | Samples | Lost | Fused-column symbol period share |
|---|---|---:|---:|---:|
| 32 | control | 8907 | 0 | 0.000% |
| 32 | off | 8897 | 0 | 0.000% |
| 32 | on | 8829 | 0 | 0.990% |
| 16 | control | 5732 | 0 | 0.000% |
| 16 | off | 5426 | 0 | 0.000% |
| 16 | on | 5622 | 0 | 1.304% |

All 15,396 SDK calls verify internally. The 99 exported fresh 164-byte proofs independently pass Gnark, public binding and disjointness against 12,686 prior identities; ledger 12,785. Full pins and every exported member are checked. RSS includes loading and primes and is not minimum RAM or memory-pressure qualification. Production and accepted SDK remain unchanged. No new all-eight/54, portable/mobile/TEE, concurrent whole-proof, pressure, large-circuit or AWS result is claimed. A positive primary screen still requires those broader applicable checks before adoption.

The original all-eight 10× M5 CPU objective remains active and unmet (0/8). The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its engineering exception and false full-performance/qualified-scope gates. These accepted local comparisons are inherited, not measurements from this primary-only experiment.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Accepted primary peak RSS remains historically 134.50 MiB. Merge 54→1 remains 161.341835 ms / 1,910.1875 MiB, with no supplied M5 reference. The latest AWS/Aeglos comparison predates the current accepted FFT source; no new cloud claim follows from this experiment.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [build](artifacts/native/result.json), [source audit](artifacts/source-audit.json).
