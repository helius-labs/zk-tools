
## Profile-directed follow-up

The selected G2 profile attributes **8.596% of sampled period** to the out-of-line `zopro_bn254::ifma8::scaled::modulus` function. The source calls `modulus(1)` and `modulus(P)` from inlined export/linear operations without an explicit const context. Although its arguments are fixed, this implementation reconstructs five-limb constants at runtime. The checked bound helper also calls `modulus(B)`. This work is unnecessary; it is separate from the intended arithmetic trade.

The grouped G2 observation records about27.7% more instructions and21.9% more cycles per call. The constant builder's sampled weight does not quantify all consequences of the calls, conversion costs or spills. No exclusive cause or recoverable full-proof latency is inferred.

The next distinct implementation change is to require const evaluation at those three source sites. The five-limb constants are fixed by the field modulus and small bounds, without a per-key cache or point table. Keep this measured negative and all failed gates. Revalidate the changed native code and compare complete concurrent callers; neither const syntax nor disappearance of the symbol establishes speed. See [numeric insight](artifacts/constant-insight.json).
