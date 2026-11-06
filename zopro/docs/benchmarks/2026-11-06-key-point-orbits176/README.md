# Exact key-point endomorphism orbit census

**No additional cube-root endomorphism aliases occur in the 14 inspected views.** The extension is closed without a runtime prototype. This is a work census, not a latency or memory benchmark; production and accepted proof times remain unchanged.

Equal or negated bases allow combining their scalar coefficients. The same principle extends to bases related by the cube-root map `phi(x,y) = (beta*x,y)`, using the appropriate scalar eigenvalue. The question was whether these relationships remove meaningful extra MSM terms without key lookup tables or duplicated plans.

An [earlier primary-key census](../2026-10-24-point-io-component/README.md) already found 27 duplicate A entries, 77 duplicate B2 entries, and none in K/H. Those ordinary duplicate counts are inherited evidence, reconciled here against the same image hash. This study extends the relation to all six signed cube-root transforms, adds 2×2, and examines exact-scalar group sums for four fixed captured inputs.

## Counts

Every row has zero identities. Equal and signed excess counts agree, so negation adds no aliases in these populations. Additional phi excess is the number of signed classes merged by adding the cube-root relationship; it is zero everywhere.

| Key / query | Points | Exact or signed duplicate excess | Additional phi excess |
|---|---:|---:|---:|
| 1×2 A | 21,006 | 27 | 0 |
| 1×2 B2 | 30,089 | 77 | 0 |
| 1×2 K | 30,529 | 0 | 0 |
| 1×2 H | 32,768 | 0 | 0 |
| 1×2 K/H combined | 63,297 | 0 | 0 |
| 2×2 A | 36,637 | 50 | 0 |
| 2×2 B2 | 52,274 | 155 | 0 |
| 2×2 K | 53,079 | 0 | 0 |
| 2×2 H | 65,536 | 0 | 0 |
| 2×2 K/H combined | 118,615 | 0 | 0 |

K/H is included both separately and combined because it contributes to the same proof output. These overlapping views must not be summed as a unique point total. A and K/H are not combined across distinct proof outputs.

For captured A/B inputs, points with equal entire nonzero scalars are also summed with complete curve operations before counting aliases:

| Fixed captured input | Scalar classes | Exact or signed duplicate excess after grouping | Additional phi excess | Current data gate |
|---|---:|---:|---:|---|
| 1×2 A | 17,454 | 0 | 0 | Declines |
| 1×2 B2 | 26,142 | 0 | 0 | Declines |
| 2×2 A | 19,268 | 0 | 0 | Allows |
| 2×2 B2 | 28,852 | 1 | 0 | Allows |

These four rows deliberately compute full mathematical grouping even when the existing sample/cardinality gate declines it. Resource admission and prepared-key routing are not replayed. They are fixed-input mathematical observations, not claims about the live SDK route, every witness, or every circuit. The scalar class counts themselves were already observed in earlier grouping studies; the new measurement is the point relationship after those sums.

The one ordinary duplicate in grouped 2×2 B2 does not establish a useful runtime optimization. No native candidate or speed prediction follows from these counts.

## Validation and scope

Both images and all four fixtures are hash pinned. Decoded A/B point arrays match between the native Euler image format and independent fixture encoding, point by point. Fixture hashes retain prior independent Arkworks on-curve, subgroup, scalar and complete-MSM qualification; those reference MSMs are not rerun. All decoded key points receive fresh on-curve checks. K/H subgroup validity is inherited from key construction, without a fresh all-point subgroup run.

Three new tests pass for cube roots and both curve eigenvalues, synthetic orbit multiplicities with identity/sign handling, and scalar-group/orbit coefficient conservation. The tool links the retained accepted field/curve library `1200e635…` with its original helper dependencies; no new core library is linked. All 176 production source files remain exact.

The study chooses each orbit's minimum canonical point key and sums scalar classes sequentially. Review instead uses sorted exact keys, expanded-transform lookups and disjoint-set unions, and forms scalar-class sums through pairwise trees. Every count and ordered point/scalar digest matches. The two algorithms share the input codec and accepted field arithmetic; this is not independent image-header parsing or a second field implementation.

Review checks 257 source/input pins, four successful study commands, all 14 views and the earlier primary counts. Two initial failed compilations are preserved: a missing collection type annotation, then helper dependencies without embedded LTO bitcode. Explicit types and the original coherent frozen dependency set repair those failures before any successful test or census; no successful stage is repeated.

No key points, scalars, witnesses, private keys, executables or libraries are published. Offline counting maps are analysis scratch, not proposed retained prover tables or a mobile RAM requirement. There is no fresh SDK/Gnark proof, timing, profile, AWS/Aeglos, device, pressure, swap, minimum-RAM or concurrent-throughput result. The distinct-proof ledger remains 30,846.

## Accepted CPU and Metal comparison, inherited

These accepted local timings and supplied historical M5 references are unchanged, not measured by this census. The all-eight CPU10× goal remains active and unmet, 0/8. Cross-machine timings do not establish equal hardware efficiency.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| merge24 | 65.060937 | 507 | 175 | 50.7 |

Merge54 remains 161.883899 ms without a supplied M5 reference. Accepted SDK SHA-256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`.

[All counts](artifacts/analysis.json), [review](artifacts/review.json), [design](artifacts/design.md), [input identities](artifacts/inputs.json), [pins](artifacts/pins.json), [plan](artifacts/plan.json), [study](artifacts/census.rs), [shared codec](artifacts/codec.rs), [reviewer](artifacts/review.rs), [receipts and helpers](artifacts/receipts-and-helpers.json.gz), [checksums](SHA256SUMS). The archive round-trips exact UTF-8 bytes, lengths and hashes. Reproduction requires the pinned toolchain, dependencies and private inputs. Private root: `/home/ubuntu/src/zopro-perf-work/key-point-orbits176`.
