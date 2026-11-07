# Universal transposed bucket-weight circuit

**Close this proposal without a native benchmark.** At the production chunk size of 256 buckets, both the current reducer and the complete transposed circuit require **502 generic affine joins and eight dependent batch phases**. The G1 vector-product proxy is unchanged; G2 adds one vector multiplication. The new circuit has larger peak batches and logical live state, without demonstrated work removal. This is a mathematical/source-work screen, not a timing result or a lower bound on every possible circuit.

## Required outputs and construction

The current `weighted_affine` returns both `sum(i*P_i)` and `sum(P_i)` for each 256-bucket chunk. The second result supplies chunk-offset corrections. A weighted-only circuit is therefore incomplete. The [source audit](artifacts/source-audit.json) binds that requirement, the current `weighted_sum` recurrence, eight-lane generic arithmetic and retained scratch types to the unchanged accepted source.

For `n=2^k`, start from a universal subset-sum circuit and transpose its linear map by reverse accumulation. An output adjoint is an arbitrary input point `P_i`. Each original addition propagates its adjoint to both operands; contribution sums are balanced by ready depth while preserving shared nodes. The complete original map is `F_i=C+sum(bit_j(i)*X_j)` for `i=1..n`; its transpose yields the unweighted total at `C` and every coefficient bit plane at the `X_j`. Horner combination of those planes gives the weighted result. The circuit depends only on n, with no key-specific point precomputation or retained proving plan.

Without C, the lower-bit subset map yields only the weighted result. In the dense model, it appears to save k affine joins and one batch phase. Restoring the required total exactly removes that saving. Dense counts are `2n-k-2` joins for current/complete versus `2n-2k-2` for the incomplete variant. The current phase populations are `l*n/2^l`, `l=1..k`; the complete transpose uses `(l+2)*n/2^(l+1)` for `l<k`, followed by one final join. These formulas describe this construction and schedule, not an optimality theorem for elliptic-curve arithmetic.

## Production-size work and memory

All values below assume generic nonexceptional point additions. VM is an eight-lane base-field multiply; F is a fused sum/difference of two products; M/S/Fs/I are scalar multiply/square/fused2/inverse. The nominal source proxy is 100 per VM and 150 per F, excluding carries, linear operations, packing, guards and metadata. Scalar work remains separate; no synthetic cycle score prices an inverse as a chosen number of multiplies.

|Quantity, n=256|Current|Complete transpose|Weighted-only, invalid alone|
|---|---:|---:|---:|
|Generic affine joins|502|502|494|
|Dependent affine batches|8|8|7|
|Batch populations|128,128,96,64,40,24,14,8|192,128,80,48,28,16,9,1|191,127,79,47,27,15,8|
|G1 VM / scalar M / S / I|412 /38 /6 /8|412 /38 /6 /8|377 /197 /38 /7|
|G2 VM / F / scalar M / Fs / I|479 /310 /50 /30 /8|480 /310 /50 /30 /8|439 /285 /273 /190 /7|
|G1 vector IFMA proxy|41,200|41,200|37,700|
|G2 vector IFMA proxy|94,400|94,500|86,650|
|Scalar-tail pairs|6|6|38|
|G1 retained group payload B|25,600|38,400|36,800|
|G2 retained group+prefix payload B|18,880|24,000|23,360|
|Logical level-schedule peak live point values|384|448|447|
|Pair-index logical payload B|1,024|1,536|1,528|

Both complete algorithms retain k final projective doublings and k projective mixed additions/subtractions in this model. The source uses subtraction corrections; the transposed assembly uses positive bit-plane additions. Their exact field costs are not measured or equated here. The G2 vector difference follows the existing prefix threshold at 32 generic pairs, not a new threshold sweep.

The current implemented point array has **511 slots**, while384 is its DAG's logical liveness under the modeled append-before-free schedule; those numbers are not interchangeable. The proposed 448 live slots could be fewer than the existing implemented array, but its record/index requirements rise, and a native slot allocator is absent. Pair-only universal instruction metadata would add 4,016 bytes; destination mapping, reference counts, free lists or a compiled slot schedule are additional. Immutable caller inputs, SIMD/scalar arithmetic scratch, vector headers/capacity rounding, stack, task results and process allocations are outside these point-liveness counts. No smaller total RAM, minimum-RAM or pressure advantage is established.

## All dense sizes

|Buckets|Current / complete affine joins|Incomplete affine joins|Current / complete phases|Current / complete G1 proxy|Current / complete G2 proxy|
|---:|---:|---:|---:|---:|---:|
|1|0 / 0|0|0 / 0|0 / 0|0 / 0|
|2|1 / 1|0|1 / 1|500 / 500|500 / 500|
|4|4 / 4|2|2 / 2|1000 / 1000|1000 / 1000|
|8|11 / 11|8|3 / 3|1500 / 1500|1500 / 1500|
|16|26 / 26|22|4 / 4|3200 / 3200|5100 / 5100|
|32|57 / 57|52|5 / 5|6100 / 6100|11800 / 11800|
|64|120 / 120|114|6 / 6|11400 / 10800|23900 / 22150|
|128|247 / 247|240|7 / 7|20900 / 21500|45850 / 47500|
|256|502 / 502|494|8 / 8|41200 / 41200|94400 / 94500|
|512|1013 / 1013|1004|9 / 9|80100 / 80100|187800 / 187500|
|1024|2036 / 2036|2026|10 / 10|157400 / 156800|373600 / 372150|
|2048|4083 / 4083|4072|11 / 11|310900 / 310900|743850 / 743850|
|4096|8178 / 8178|8166|12 / 12|618600 / 618600|1486750 / 1486850|

Other sizes redistribute scalar tails and vector work. No favorable size or identity mask is promoted after the fixed production-size continuation condition fails. Dense, all-zero, alternating, first-quarter, last-quarter and deterministic sparse masks for all 13 sizes remain in the 78-row analysis. Formal identities do not model equal/opposite points, cancellations or actual bucket occupancy.

## Validation and decision

Two new tests pass once: exact support/coefficient checks over all 278 identity masks through eight points, and independent dense formulas/full-output requirements through 64 points. Every one of the 234 variant rows checks exact formal supports, final INTEGER coefficients and three separately evaluated deterministic integer populations. The support-bitset model verifies disjoint operands at each affine node and every output bit plane against its direct coefficient matrix; no finite-field or native curve suite is rerun. These linear identities hold over any abelian group, but their operation counts assume the stated generic point path.

Independent review checks 187 pins, both successful workflow commands and stream hashes, all 176 unchanged accepted sources, the required chunk total,13 dense closed-form phase populations,234 scalar/vector ledgers and logical level-liveness conservation. An initial draft was compiled but not executed. A pre-execution source audit corrected its G2 scalar tail to 7M+5Fs, with zero scalar squares; the old source is retained. The final source is rebuilt before its first tests/model run. There is no failed arithmetic run, test/model replay, new proof, SDK request, profile or timing. Canonical proof ledger remains **33,949**.

The complete construction does not remove credible dense 256-point work and requires more scratch/metadata. No native implementation, threshold/shape rescue or unchanged retry is selected. This closes this exact positive-add transpose and scheduling recipe, not every fixed-coefficient circuit or MSM algorithm. The original verified CPU 10× objective remains active and unmet, **0/8 shapes**.

The [latest full SDK CPU/Metal comparison](../2026-11-07-fft-column-prep-sdk176/README.md) remains authoritative, with accepted primary 8.013551 ms, merge54 161.737128ms and accepted SDK 6d83 unchanged. These are inherited proof observations, not model timings. No new AWS/Aeglos, mobile, huge-circuit, swap or memory-pressure execution is claimed.

Inherited comparisons against the supplied M5 references follow. They are cross-machine elapsed-time ratios, not equal-hardware efficiency or results of this model.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|8.013551|37 / 4.617×|24 / 2.995×|3.7|
|2x2|11.254041|59 / 5.243×|30 / 2.666×|5.9|
|4x2|15.963789|109 / 6.828×|48 / 3.007×|10.9|
|8x2|24.269224|189 / 7.788×|74 / 3.049×|18.9|
|12x2|33.884727|284 / 8.381×|101 / 2.981×|28.4|
|16x2|40.199590|335 / 8.333×|124 / 3.085×|33.5|
|24x2|65.690345|528 / 8.038×|176 / 2.679×|52.8|
|merge-24x1|65.555031|507 / 7.734×|175 / 2.670×|50.7|

[Full model results](artifacts/analysis.json), [review](artifacts/review.json), [decision](artifacts/decision.json), [source audit](artifacts/source-audit.json), [model sources, plans, pins and receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
