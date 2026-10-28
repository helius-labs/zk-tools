# Coordinate scaling for wide R260 arithmetic

The exact arithmetic screen passes. Native performance remains unmeasured; production and the accepted SDK are unchanged. This permits a native prototype after accounting for conversion, packing and inversion costs.

## Mechanism

Use the curve isomorphism (x,y) → (x/4,y/8) with Montgomery radix R=2^260=16H, where H=2^256 is the host radix. A canonical host raw coordinate a=xH mod p imports numerically as 4a for X and 2a for Y. Export uses modular division by 4 or 2 followed by canonicalization. The isomorphic curve has coefficient b/64; ordinary affine addition commutes with the scaling. Original compact host key and bucket coordinates remain the proposed storage format, without a persistent recoded point copy or lookup table.

Keep the original two-fused-product complex formulas. Let wider linear intermediates survive several operations: imported X<4p and Y<2p; dx<8p, dy<4p, norm<3p, slope<2p, square real<2p and imaginary<4p, X output<12p, finishing difference<16p, Y output<4p. Since 64p<R, product bounds permit the remaining reductions. Divide the wide X before packing into four words. The export result is below 4p and needs conditional subtraction of 2p then p. Wider values must use explicit bounded operations; ordinary compact/get/difference2 contracts are not widened.

The source work ledger retains 1450 IFMA sites per eight-point forward/reverse group. Potential savings concern eleven general linear add/sub operations and five mixed fused corrections. Shift/packing imports, modular half/quarter exports, extra final corrections, root-inversion codecs and register pressure remain costs. Counts are source-graph estimates, not emitted instructions or predicted speed. See [work ledger](artifacts/work-ledger.md).

The rejected lazy-linear experiment normalized every linear result below 2p using a quotient/table method. Earlier raw-R260 bucket experiments persistently recoded points. Their negative results remain closed; this screen uses coordinate scaling and wider intermediate bounds. No novelty claim is made.

## Exact checks

Three new focused model tests pass, followed by the fixed exact study once. A BigUint full-radix REDC oracle is compared with separate scalar five-by-52-bit CIOS across wider domains. These are scalar integer models, not actual SIMD instruction execution.

| Domain | Operand bounds in p | Output bound | Reductions |
|---|---|---:|---:|
| ordinary | [8,8,0,0] | 2p | 8217 |
| norm | [8,8,8,8] | 3p | 8817 |
| numerator | [4,8,4,8] | 2p | 8817 |
| finish | [2,16,2,16] | 2p | 8817 |
| prefix | [2,3,0,0] | 2p | 8217 |

There are 42,885 reduction comparisons, 20,810 codec/alias checks, and 2,560 ordinary BN254 G2 outputs at batch sizes 8/16/32/64/200 over eight changing steps, checked against independent Arkworks generator scalar sums. Eight boundary ordinary pairs are each repeated across eight lanes; 17 boundary attempts take whole-call guards.

The toy F49 curve has 38 finite points. All 1,368 ordinary pairs agree, each repeated across eight lanes; 76 equal-x pairs take guards. Three tail lengths take guards, and the empty case accepts without arithmetic. Identity/equal-x/non-multiple-of-eight cases use typed whole-call fallback before arithmetic. This does not implement or qualify a new native exceptional path.

The first compilation failed at an ambiguous integer shift in a boundary fixture. The repair adds only an explicit BigUint type and usize shift. Original source, pins, plan and failed build receipts are preserved; the repaired three tests and fixed study then pass. No successful test or study was repeated for bookkeeping.

All 172 production files and 396 run pins are verified after execution. Source, plans, compiler, dependency hashes, failures and receipts are archived; executables remain local with hashes. The model's arbitrary-precision allocations are diagnostic storage. There is no native speed, CPU profile, proof timing, memory peak, minimum RAM, mobile/TEE/pressure/giant-circuit or new AWS/Aeglos result. Proof ledger remains 13,181.

## Inherited CPU and Metal comparison

The all-eight CPU10x objective remains active and unmet, 0/8. The accepted FFT adoption retains its engineering exception and false full-performance and qualified-scope gates. These accepted local timings and supplied M5 references are inherited, not measurements of this model.

| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted | CPU10x target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37.0 | 24.0 | 4.445× | 2.883× | 3.7 |
| 2x2 | 11.460675 | 59.0 | 30.0 | 5.148× | 2.618× | 5.9 |
| 4x2 | 15.854365 | 109.0 | 48.0 | 6.875× | 3.028× | 10.9 |
| 8x2 | 24.084626 | 189.0 | 74.0 | 7.847× | 3.072× | 18.9 |
| 12x2 | 33.693397 | 284.0 | 101.0 | 8.429× | 2.998× | 28.4 |
| 16x2 | 39.981636 | 335.0 | 124.0 | 8.379× | 3.101× | 33.5 |
| 24x2 | 66.010670 | 528.0 | 176.0 | 7.999× | 2.666× | 52.8 |
| merge-24x1 | 65.086624 | 507.0 | 175.0 | 7.790× | 2.689× | 50.7 |

Historical primary peak is 134.50 MiB. Merge54 remains 161.341835 ms / 1910.1875 MiB, without a supplied M5 reference. No production change or full-proof improvement is established.

[Design](artifacts/design.md), [model](artifacts/model.rs), [analysis](artifacts/analysis.json), [tests](artifacts/native/tests/stdout.log), [fixed study](artifacts/runs/result.json), [pin closeout](artifacts/closeout.json), [repair](artifacts/repair.json).

Publication check preserves two captured logs with trailing blank lines byte-for-byte; authored sources are checked separately. See [receipt](artifacts/publication-check.json).
