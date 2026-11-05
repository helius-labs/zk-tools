# Integer boundary arithmetic for the auxiliary-prime Euler operator

**Exact integer projection, direct CRT correction and Montgomery import compensation pass the new algebra checks.** A separate 32,768-point operator call matches every independent reference output. There is no new performance result or production adoption. The SIMD boundary helpers and complete comparison against the accepted native FFT remain to be implemented.

This follows the [qualified convolution prototype](../2026-11-05-crt-euler176/README.md). It retains the same Euler kernel, Fr twist,17 auxiliary primes and NTT library, while replacing BigUint input projection and Garner output reconstruction with scalar references for different integer formulas. The three successful parent tests are filtered out; three new tests run. The complete caller explicitly links the exact accepted field library, with native CPU, optimization, fat LTO and one codegen unit checked in its command receipt.

For CRT product M and M_i=M/q_i, set `t_i=(residue_i*inverse(M_i mod q_i)) mod q_i`. The signed coefficient satisfies

```text
c = sum(t_i*M_i) - k*M
k = round(sum(t_i/q_i))
```

The parent bound M>16C leaves7/16 of a unit of rounding margin. With `mu_i=floor(2^63/q_i)`, `f_i=(t_i*mu_i)>>31` underestimates `2^32*t_i/q_i` by less than2, and every product fits below2^63. The sum error is below34, so `k=(sum(f_i)+2^31)>>32` is exact. Output modulo Fr is an18-term fixed-coefficient dot product. **No floating point is used.** This supersedes the parent's unimplemented floating-point selector idea, while preserving its exact results.

Input projection uses eight32-bit chunks and fixed weights modulo each prime. Shared quotient estimation leaves a small nonnegative residual below16q even though the full dot product intentionally wraps its low64 bits. A final48-bit reciprocal estimate and one conditional subtraction normalize it. The import uses raw Montgomery words for the varying operand while the kernel remains canonically lifted; one inverse-R factor in Fr compensates the resulting scale. [Design and bounds](artifacts/design.md), [corrected implementation](artifacts/direct-fixed.rs).

|New qualification|Scope|
|---|---:|
|Integer projection versus BigUint|78,336 cases across all17 primes|
|Integer CRT versus exact signed BigInt coefficients|1,031 cases|
|Small complete operators versus parent reconstruction and coefficient oracles|8 cases at32/64 points|
|New full-domain operator versus independent polynomial derivatives|32,768 outputs|

Projection inputs include full256-bit words, word/radix/modulus boundaries and4,096 deterministic words per prime. CRT inputs include both signed coefficient extremes, near-order values and1,024 deterministic bounded values. Operator cases include zero, signs, full-width fields, partial and empty arrays. The large-domain reference uses known low-degree polynomial coefficients, not a captured proof witness. These are algebra/operator checks, not curve/proof oracles or measured speedups. [Full-domain result](artifacts/algebra-direct.json), [independent review](artifacts/review.json).

The reviewer checks1,452 pins, all176 unchanged production files, exact accepted field identity, all four successful workflow commands and both retained failures, every stream hash, the fixed-point error margin and product ranges, and source recovery to the parent. Initial test compilation failed on a test-only BigInt ownership conversion; the corrected source changes only that expression. After successful tests, the full driver link failed because the tooling serde library lacked LTO bitcode. Only the remaining link/call steps were repaired with the coherent parent LTO serde library. No successful test, library build or operator call was repeated. Raw streams are encoded as UTF-8 JSON with original SHA256 and checked byte-for-byte on publication.

The next unimplemented SIMD recipe shares modulus subtraction across18 fixed small-factor products, rather than doing18 general field multiplications. Its proposed source count is201 IFMA sites before carries, linear operations, loads and packing; this is not compiled code or a timing prediction. The reviewer checks the necessary top-limb reducer inequality, but no native helper, overflow oracle or caller gain is qualified by that check. [Native follow-up notes](artifacts/native-next.md).

The parent domain context still carries12.625MiB of known array payload, plus headers, alignment, BigUint state and query scratch. This scalar prototype also allocates per-output row storage and uses general Fr multiplication in its CRT dot product; it makes no allocation-free or minimum-RAM claim. Full resource accounting, optional context admission, unsupported-domain and pressure fallback, native timing, profiling and eventual SDK qualification remain required. No giant point table, copied proving key, second retained solver plan or production pool was added.

No SDK requests, proof exports, Gnark checks, AWS/Aeglos comparison, pressure/phone qualification or latency measurement occurs. The canonical proof ledger remains29,953. The latest full-SDK evidence remains the [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), with its measured primary gain and failed full gate retained.

Unchanged accepted local32-worker observations versus supplied historical M5 references follow; these are not measured here, and hardware differs.

|Shape|Accepted CPU ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37|4.667×|24|3.027×|3.7|
|2x2|11.151932|59|5.291×|30|2.690×|5.9|
|4x2|16.086386|109|6.776×|48|2.984×|10.9|
|8x2|24.179453|189|7.817×|74|3.060×|18.9|
|12x2|33.884455|284|8.381×|101|2.981×|28.4|
|16x2|40.077381|335|8.359×|124|3.094×|33.5|
|24x2|65.850777|528|8.018×|176|2.673×|52.8|
|merge24x1|65.060937|507|7.793×|175|2.690×|50.7|

Accepted merge54x1 remains161.883899ms without a supplied M5 reference. The verified all-eight CPU10× goal remains active and unmet,0/8. Continue to native boundary arithmetic and complete caller qualification; do not substitute an isolated NTT benchmark or these algebra passes for a proof-latency result.
