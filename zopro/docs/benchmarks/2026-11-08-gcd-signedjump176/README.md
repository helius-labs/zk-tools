# Signed small-quotient GCD matrix screen

**The mathematical checks pass; this fixed recipe does not select a production-backend implementation or a timing trial.** Cancelling several low bits per small-matrix jump reduces those jumps by about 31%, but increases full-width integer-row updates by about 39% and modular coefficient updates by 33–34%, while adding quotient multiplications. This operation tradeoff is unattractive for the next native experiment. It is not a measured latency regression or a proof-speed result. Production remains unchanged.

The accepted inversion uses Pornin's batched binary-GCD structure ([primary paper](https://eprint.iacr.org/2020/972)) with 31 low bits and 33 leading approximation bits. The [upstream libsecp256k1 explanation](https://github.com/bitcoin-core/secp256k1/blob/master/doc/safegcd_implementation.md) describes low-bit inverse cancellation for variable-time divsteps. This model applies that general idea to the current positive approximate GCD and its encoded matrix rows. No upstream implementation or performance number is copied; no novelty claim is made. Earlier SIMD full-integer rows, packed rows, fused coefficient carry, Euclid finish quotients, cross-worker root sharing and LogJumps retain their recorded outcomes.

For odd b, b⁻¹=b mod 8. After selecting the larger approximate operand as a, choose q=ab mod 8 in {−3,−1,1,3}. Then a−qb is divisible by eight. Normalize its sign and divide out its trailing zero run, updating row0−=q·row1, optional row0 sign, and the row1 power-of-two scale. The last two remaining bits use signed mod-4 q in {−1,1}; the last bit uses the original +1 step. First-even-run stripping stays. Row encoding E=f+2^32g mod 2^64 remains.

A signed step can temporarily reach four times the larger approximate operand. Two headroom bits are required, so this fixed recipe uses 30 low plus 32 leading bits, rather than the accepted 31+33. This is the largest block satisfying 2k+2≤62. The block row L1 bound is 2^30 and the paired composition bound is 2^60. A production port would consequently need 30/60-bit modular coefficient updates, while retaining the original 64-bit Euclid finish and final scale. No table, key, solver-plan or pool allocation is proposed.

The checked Rust/BigInt model validates 1,048,576 toy matrix executions and 120 additional low/top-word boundary cases. For each prime, 3,327 canonical nonzero raw inputs cover six boundaries, 761 power-of-two/neighbor cases, 2,048 fixed full-width random values and 512 nearby values. Both algorithms produce 13,308 modeled inverse outputs in total. Every output matches independent BigUint Fermat inversion and the exact accepted field-library reference; those references are semantic checks, not performance controls. Every tested complete inverse finishes within the original 32-round cap, with maximum fifteen observed rounds in each algorithm/field.

Checks cover encoded rows versus independently tracked signed integer rows, determinant magnitude, exact full-row divisibility, actual full-integer sign normalization, nonincreasing integer maxima and modular coefficient invariants. The accepted matrix helper is copied byte-exactly from the pinned production source and checked against the model baseline on the same inputs. Signed BigInt arithmetic models the prospective coefficient logic; this does not qualify a new fixed-limb instruction implementation or prove untested inputs.

The whole-inversion counts below use each prime's 2,048 full-width random inputs. Counts are semantic operations, not equal-cost instructions or cycles. In particular, Euclid quotient counts are not emitted DIV-site counts, and additional source multiply expressions are not measured multiplication instructions.

|Field / operation|Accepted scheme|Signed-jump scheme|Change|
|---|---:|---:|---:|
|Fq / small-matrix jumps|288832|199840|-30.81%|
|Fq / full-width integer rows|37236|51922|+39.44%|
|Fq / modular coefficient rows|20460|27302|+33.44%|
|Fq / finish Euclid quotients|66121|68144|+3.06%|
|Fq / added quotient multiply expressions|0|580071|283.24 per input|
|Fr / small-matrix jumps|288763|200153|-30.69%|
|Fr / full-width integer rows|37270|52064|+39.69%|
|Fr / modular coefficient rows|20448|27436|+34.17%|
|Fr / finish Euclid quotients|65701|68481|+4.23%|
|Fr / added quotient multiply expressions|0|580893|283.64 per input|

The fewer small-loop jumps require more expensive-width updates and new quotient products. **We decline native work on this recipe after the required complete-tradeoff review.** No fixed instruction-cost weighting, hardware speed estimate, quotient-set/block-size/swap-policy sweep or parameter rescue is used. Other input cohorts and every per-input counter row remain in the raw archive. This decision does not disprove all multiple-bit GCD methods.

Two interpretation details remain explicit. First, the raw model's `selects_native=true` label only negates its narrow initial four-counter domination predicate. That predicate is false because small jumps decrease. The original design and raw output both require a separate tradeoff review before native selection; that review now records `selected_native_backend=false`. The raw label is preserved, not changed into a failed mathematical gate or used as adoption authorization. The public summary adds that decision explicitly.

Second, `max_observed_row_L1` records post-jump maxima and omits an initial-only even strip. Its original values remain visible as partial observed maxima. The initial strip has row (0,2^first), with first≤BITS by construction; this satisfies the bound independently. Post-jump bounds, determinants, exact divisibility and complete inverse checks pass. No selection judgment depends on that incomplete maximum counter, and no finished model run is repeated to change a metadata label.

The first model compile failed on a counter field/method typo and a BigUint shift annotation. Original source and failure logs are retained; only those two compilation fixes precede the first model execution. The successful two-step continuation builds with overflow checks enabled and runs the fixed study once. An unused-import warning is retained without a cleanup rebuild. The independent reader checks source/helper binding, input pins and command/stream hashes, all 6,654 per-input counter rows, cohort and histogram conservation, maxima scope, and both selection interpretations. It does not rerun mathematical references or claim a second independent execution of every model case.

No production-backend candidate, point caller, SDK request, proof export, CPU profile, latency recording or cloud resource is produced. Production176, accepted SDK6d83 and canonical proof inventory35,171 remain unchanged. The observed proof comparisons below are entirely **inherited** from the [preceding report](../2026-11-08-pbqp-codegen176/README.md): latest matched 1×2 plus seven earlier full-campaign rows. Supplied M5 references use different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.002770|37.0|24.0|4.623×|2.999×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

The original verified CPU 10× goal remains active and unmet, **0/8**. The same accepted binary previously measured 7.915–7.929 ms locally; variation versus its latest 8.002770 ms remains unexplained, with no slower replacement adopted. Merge54 remains inherited at 162.240109 ms without an M5 reference. No new AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, RAM peak/minimum or concurrent-throughput result is added.

The [evidence archive](artifacts/evidence.json.gz) retains exact plans/pins, original/fixed model sources, helpers, the raw complete reviewed model result, per-input counters, receipt streams, preserved failure, design and decision. Decompression yields relative paths mapped to {utf8,bytes,sha256}; every extracted payload is checked. The public model summary removes only per-input arrays and adds the explicit native-review decision/selector explanation. Original raw result bytes remain archived and are distinguished from the public view by hashes. Binary/library blobs, papers, full accepted sources, keys/images/proof payloads and private ledgers are excluded; relevant identities and the small accepted matrix fragment remain.
