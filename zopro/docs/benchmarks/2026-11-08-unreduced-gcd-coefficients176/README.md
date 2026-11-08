# Unit-seeded unreduced GCD coefficients

**The arithmetic model passes and selects a fixed-limb qualification. No speed improvement is measured yet.** Keeping exact growing integer coefficients until the end of inversion removes about 53% of the modeled coefficient/finish word-product expressions on the random inputs. It retains the accepted integer GCD matrices and Euclid finish. Wider signed values, carries, active-length handling and correction costs still need native measurement; this is not a twofold proof-speed claim or a production change.

The current inverse reduces two modular coefficients after every pair of 31-bit GCD rounds. This model starts the corresponding exact integer coefficients at `(1,0)`, applies the same composed matrices without those reductions, then performs one complete dyadic cancellation and restores the Montgomery scale. It adds no lookup table, proving-key copy, solver plan or worker pool. The dyadic-coefficient viewpoint also appears in [this primary implementation article](https://purplesyringa.moe/blog/faster-practical-modular-inversion/); no upstream code or performance figure is copied. The whole-coefficient representation differs from the earlier local seed-scale, signed-jump, active integer-word, packed-matrix and fused-coefficient trials.

For canonical raw input `x`, prime `p`, and `R=2^256`, after `k` rounds the exact coefficients satisfy `c*x+d*p=a*2^(31k)`. The terminal Euclid combination selects `a=1`. Cancelling that power of two therefore produces raw `x^-1`; one Montgomery multiplication by raw `R^3 mod p` produces the required raw inverse `x^-1*R^2 mod p`. There is no table of inverse powers. The full derivation and conservative storage bounds are in the [design](artifacts/design.md).

## Complete coefficient tradeoff

Each field has 2,048 newly generated deterministic random inputs in the table. Counts are scalar word-product expressions, including multiplication by the cancellation inverse and modulus words. They are not equally priced machine instructions or cycle counts. Original integer GCD updates, small matrices and tail Euclid divisions remain unchanged and are outside this coefficient-only table.

|Category|Fq accepted|Fq exact coefficients|Fr accepted|Fr exact coefficients|
|---|---:|---:|---:|---:|
|Coefficient and finish linear word products|179,968|87,376|180,048|87,247|
|Cancellation word products|112,480|51,120|112,530|51,145|
|Combined word-product expressions|292,448|138,496|292,578|138,392|
|Final full field multiplications, counted separately|2,048|2,048|2,048|2,048|
|Combined expression ratio|1|0.473575|1|0.473009|

The exact-coefficient linear column includes both coefficient propagation and the widened final Euclid combination. All late cancellation steps are included. The first exact matrix initializes the unit-seeded coefficients directly. The accepted expression count does not account for compiler folding of its known initial seed; actual code may remove some operations. No fixed instruction-cost weighting or hardware speed forecast is applied.

The random cases reach at most 218 coefficient bits, 253 final coefficient bits and 318 transient cancellation bits. Observed widths are not a general bound. The original 32-round cap gives a conservative 17-word coefficient envelope; a prospective fixed implementation must also reserve correction headroom and preserve early exits. Active-length bookkeeping, sign conversion, carry propagation, normalization, packing and final field multiplication remain real costs. The 256 early-`b=1` cases per field also add a final field multiplication that the accepted early exit avoids; all cohort results are retained in [review](artifacts/review.json).

## Checks and scope

The new model builds and executes once through Rust xtask; both commands succeed on their first executions. Installed rustc 1.98.1, the accepted matrix fragment, all 176 production source hashes and existing field/BigInt dependencies are pinned. Production is unchanged. No field/core rebuild, old suite, historical model, fixture generation, decoder or recording is replayed.

There are 2,810 checked case rows per field: three boundary rows, 759 power/neighbor rows and 2,048 random rows. Each field has 2,808 distinct input values; the two duplicated boundary values are retained and are not counted as distinct coverage. Every result agrees with independent BigUint Fermat inversion and the unchanged accepted native field library. The model checks matrix norms and determinants, exact integer divisibility/sign normalization, composed dyadic identities, original modular coefficient ranges, terminal Euclid identities and complete signed correction/output ranges. A separate small-prime binary implementation checks 2,048 dyadic inverses against an ordinary extended-Euclid oracle.

The offline reader passes on its first execution. It checks 185 source/dependency pins, both completed command receipts and stdout/stderr hashes, all 5,620 case rows, cohort sums, distinct-input counts, shift totals and conservative widths. It reconstructs the table without replaying inverse oracles. It does not independently reimplement the BN254 algebra, qualify fixed-limb instructions, prove the unchanged GCD termination cap for all inputs, or turn active-width product counts into latency evidence.

**Next: implement and qualify one fixed-limb inverse with the exact original integer path and complete correction.** It needs independent boundary/random/early-exit checks, actual optimized consumer inspection, then controlled complete-inverse and G1/G2 caller measurements before any SDK integration. Existing negative GCD recipes remain closed. The favorable count screen alone authorizes no production adoption.

## Accepted CPU and Metal comparison, inherited

These are earlier local full-request observations, not this model's timings. The same accepted binary measured local 1×2 at **7.915, 7.929 and 7.996 ms**, with its latest observation **8.003 ms**. Current performance is roughly **7.9–8.0 ms**; variation remains unexplained and no adopted regression is established. Supplied M5 references use different hardware.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 / 4.623×|24 / 2.999×|3.7|
|2×2|11.241463|59 / 5.248×|30 / 2.669×|5.9|
|4×2|15.933816|109 / 6.841×|48 / 3.012×|10.9|
|8×2|24.234311|189 / 7.799×|74 / 3.054×|18.9|
|12×2|33.855565|284 / 8.389×|101 / 2.983×|28.4|
|16×2|40.071563|335 / 8.360×|124 / 3.094×|33.5|
|24×2|65.917462|528 / 8.010×|176 / 2.670×|52.8|
|merge-24×1|65.305023|507 / 7.764×|175 / 2.680×|50.7|

Source: [previous accepted comparison](../2026-11-08-config-lookup-census176/README.md). Seven nonprimary rows remain inherited from the earlier full campaign. Merge54 remains 162.240109 ms without a supplied M5 reference. The original verified all-eight CPU 10× goal remains active and unmet, **0/8**. Accepted production176/SDK6d83 and the canonical 35,181-proof inventory remain unchanged. No fresh proof, SDK request, latency, RSS, CPU profile, AWS/Aeglos, huge-circuit, device/TEE, RAM-pressure/swap or throughput result is added.

[Model source](artifacts/model.rs), [reviewer](artifacts/review.rs), [review](artifacts/review.json), [design](artifacts/design.md), [source/receipt/raw-model archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). The archive preserves all synthetic input/count rows and exact command streams. Executables, libraries, full unchanged production source, full papers, private keys/images/requests/witnesses, proof/public payloads and private proof inventories are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/unreduced-gcd-coefficients176`.
