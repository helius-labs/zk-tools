# Bounded preparation of four-point B2 classes on Ryzen

Preparing the B2 sums gave **13.518 / 13.511 ms** full proofs on the original and held-out 4×2 requests. The paired changes against ordinary were **-1.42% / -1.07%** and against the exact frozen SDK were **-0.33% / -0.06%**. The prepared mode fell short of the timing criteria. The report includes the experimental patch. AWS SSO expired, leaving the verified [C7a score](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) at five of eight supplied shapes at 10× M5 CPU.

The plan selects all 17,611 nonzero four-point classes from the [original 918 capture](../2026-11-20-four-by-two-scalar-relations-ryzen918/README.md). On first use, the trial computes each point sum from the loaded B2 bases. During proving, an exact scalar class can use the sum when all four indices match the stored class. The remaining classes use ordinary affine reduction. The stored data contains only indices and point sums.

The source plan is 281,792 bytes. On each request the allocated cache retained **2.60 MiB** of buffers, with peak admission charged before allocation. The two untimed probes observed **38.40 / 37.95 ms** to build the cache the first time the key entered the B2 query. At the paired same-build savings of 0.194 and 0.147 ms per proof, the isolated preparation cost equates to roughly 198 and 259 proofs, respectively. Every proof and prime hit 17,611 classes on the original input and 16,897 on the held-out input, matching the predictions from independent captures. At three pair additions per four-point class, that skips up to 52,833 and 50,691 point additions. A model using the solver's private wire projection found the same exact class counts for K. That observation comes from captured input coefficients. The timed trial runs ordinary K and performs the existing full scalar classification before using the sums.

The native test compared reused and rejected classes with ordinary reduction on points that include identities, duplicates and negatives. Real SDK route probes exercised mismatched dimensions and zero optional workspace. Both took the standard path and produced valid proofs. Gnark independently verified all 56 exports.

Ryzen 9950X uses 24 main and eight frontend hardware threads. Eight counterbalanced blocks per input prime three proofs and time seventeen. Latency is the median of process means. The two ordinary calibrations versus fixed release were **+1.15% / +1.20%**, within the 2% absolute threshold. The prepared mode needs lower paired medians and at least six of eight wins against both controls on both inputs. Peak RSS includes key loading and first use. CPU-ms is process CPU time per timed proof.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | frozen | 13.586 | +0.00% (0/8) | -1.13% (8/8) | 290.6 | 205.8 |
| catalog | ordinary | 13.725 | +1.15% (0/8) | +0.00% (0/8) | 292.2 | 213.5 |
| catalog | quartets | 13.518 | -0.33% (5/8) | -1.42% (8/8) | 287.7 | 213.5 |
| heldout | frozen | 13.551 | +0.00% (0/8) | -1.18% (8/8) | 291.1 | 205.8 |
| heldout | ordinary | 13.698 | +1.20% (0/8) | +0.00% (0/8) | 292.3 | 217.2 |
| heldout | quartets | 13.511 | -0.06% (4/8) | -1.07% (8/8) | 288.6 | 216.8 |

[Reproducible patch](artifacts/source.patch) · [Compressed index plan](artifacts/catalog-quartets.bin.gz) · [Protocol](artifacts/protocol.json) · [Timing and route samples](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
