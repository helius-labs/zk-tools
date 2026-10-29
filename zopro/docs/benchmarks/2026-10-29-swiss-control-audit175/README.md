# Offline review of the grouped dictionary control gap

The accepted and fresh OFF executables contain **identical machine-code bytes for the selected G2 stream kernel, G1 stream kernels and G1/G2 reduction trees**. The G2 stream kernel is 51,950 bytes in each. The earlier 10.04% OFF latency regression therefore cannot be explained by a change inside those function bodies alone. Other code/data, runtime state and execution paths remain possible influences; the root cause is unresolved.

This is a bounded, exploratory review of existing artifacts from the [SDK trial](../2026-10-29-swiss-groups-sdk175/README.md), with no new captures, proofs, candidate changes or acceptance decisions. It preserves that trial's failed calibration and uncertain 22.313 µs ON/accepted saving. The 23.22% lower observed ON peak RSS remains a separate positive observation.

## Exact ELF comparison

The reader checks 64-bit little-endian x86 ELF headers and maps each unique demangled code symbol into its executable section. Comparisons include all bytes, including relative addresses; no relocation normalization is used. Names are matched after removing Rust crate disambiguators through the demangler. Of 2,985 common uniquely matched code symbols, 2,918 have identical bytes. The other 67 all retain the same size; 24 move address. Unmatched and ambiguously named symbols are excluded, so this is not whole-program equivalence. Every differing matched function and differing byte offset is retained.

|Selected function|Accepted bytes|OFF exact bytes equal|
|---|---:|---|
|`zopro_core::msm::g2_borrowed::apply::<zopro_bn254::curve::G2Config, true>`|51950|yes|
|`zopro_core::msm::reduce_layer::<zopro_bn254::curve::G1Config>`|23599|yes|
|`zopro_core::msm::reduce_layer::<zopro_bn254::curve::G2Config>`|81580|yes|
|`zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, false, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|26407|yes|
|`zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|26912|yes|
|`zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, true, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|27094|yes|
|`zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|26230|yes|
|`zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>`|26282|yes|

Independent objcopy section dumps reproduce both G2 function byte hashes. The grouping function has 7,624 bytes at the same address in both executables. Its eight differing bytes lie in RIP-relative operands selecting two adjacent vector constants. Their 32-byte payloads are identical, relocated from 0xb870 to 0xb880. This does not prove equal runtime cache behavior, or equivalence of every external call/data dependency. [All 67 differences](artifacts/differences.json), [grouping instruction locations](artifacts/grouping-byte-differences.json), [constant check](artifacts/constant-check.json).

The first objcopy dump omitted an output ELF filename and reserialized both input executable containers. The closing binary-hash check detected this. Both executables were restored by relinking the exact original driver with the original pinned frozen libraries and command; both original hashes match exactly. No benchmark, proof or test ran after the rewrite. A subsequent dump uses an explicit throwaway output and confirms that the entire text section matches the first dump while input hashes remain unchanged. The reproduced decoder source uses that safe form. These repairs repeat links to recover artifacts, not performance captures or tests. [Restoration receipt](artifacts/restoration.json), [safe cross-check](artifacts/safe-objcopy.json).

## Existing CPU profiles

The table reuses the trial's separate 199 Hz cycles:u primary 32 profiles, 251 requests after three primes each. Periods are weighted samples, not dynamic instruction counts or exclusive wall-time fractions. The captures are separate processes and do not supply paired uncertainty for these profile differences.

|Profile|Total sampled period, billion|G2 stream period, billion|G2 share|Grouping share|
|---|---:|---:|---:|---:|
|accepted|165.807|29.085|17.54%|0.94%|
|OFF|190.552|51.115|26.82%|0.83%|
|ON|165.415|28.107|16.99%|0.49%|

G2 contributes 22.029 billion of the 24.745 billion net increase in flat sampled period, approximately 89.0%. That descriptive difference is not a fraction of the proof latency regression and does not establish cache, allocator, frequency or scheduler causality.

A second offline decode adds timestamps and symbol offsets. All sample counts and period sums exactly match the earlier decode. Every G2 offset is retained in 256-byte bins, rather than selecting only favorable instructions. All three primary 32 profiles sample 32 TIDs in G2; this does not show their simultaneous placement. The existing recordings lack the CPU attribute. Requesting CPU fields is rejected before decoding; the successful fallback uses only fields actually present. No SMT/core-placement claim or new profiling capture follows.

## Decision and next action

Changing generated code in the unchanged G2 function is not supported as an explanation of this control gap. The exact failed SDK trial stays failed; no timing is repeated or threshold changed. The lower RSS result remains useful for a separately declared memory/speed tradeoff, without relabeling the old speed gate.

The next concrete optimization screen is to count overlap between A/B witness-wire queries and test whether shared proof-local classification can remove duplicate hashing. It must charge the extra global/local metadata and any front-loaded critical path. It may fail on operation count, memory or scheduling cost; no speed gain is predicted. Prior shared/equality/grouping experiments must be checked before implementation. This is distinct from another dictionary-layout variation.

## Prover state and supplied references

Production source 175/SDK 20fde remains unchanged. There are zero new SDK requests or independent proofs; the verified ledger remains 21,007. The original all-eight CPU 10× objective is active and unmet 0/8. The complete accepted timings and supplied historical M5 CPU/Metal references remain:

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. Accepted primary peak RSS remains 134 MiB from its own full qualification. The [current local/AWS summary](../2026-10-29-current-cpu-summary/README.md) retains the accepted physical 16 calibration exception and separately true final-artifact gate. No new AWS/Aeglos, mobile/TEE, pressure, swap, minimum-RAM or concurrency result is claimed.
