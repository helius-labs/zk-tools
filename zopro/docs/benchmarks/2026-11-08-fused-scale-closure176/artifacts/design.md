# Fused numerator followed by narrow scaling

One fixed mechanism: omit only the intermediate2p correction in R256 fused
sum/difference followed immediately by multiplication by a normalized factor
<=2p. Preserve all raw products, Montgomery rows, packing and normalization.
Do not expose a wide value through public Ifma8/get/compact/FqBatch APIs.

For inputs a,b,c,d,f in[0,2p], T=ab+cd<=8p². A difference uses existing2p-c
complement, including2p for zero. RawU=REDC(T)<p+8p²/R. Then
V=REDC(Uf)<p+2p²/R+16p³/R²<2p if16p²+2pR<R². Both actual BN254 primes meet
this condition at R256 andR260. The sufficient compile-time bound p<199*2^246
atR256 implies it because16*199²+2*199*1024<1024². Original normalized
five52bit digits fit256bits even in the wide transient, since U<3p<2^256.
Inclusive2p input corners deliberately exercise negative zero and raw outputs
that can differ by p from the original sequence. Compare canonical values,
but check exact new raw REDC independently too.

The literal scalar52/48CIOS observer uses original cancellation-carry recipe,
adds the second input product per row, and checks every row against BigUint
whole integers. A separate closed full-radixREDC checks both stages. Per-digit
safety remains governed by normalized52bit inputs: each fused row adds at
most two raw halves and one modulus half per coefficient. Across five rows
the coefficients stay within32B before mixed48 packing; the four-bit mixed
packing stays below2^61. The model's maximum is an observation, not a proof.

New finite qualification: five raw edges crossed in all five operands,
radixbit boundaries,2048 changing random tuples and256 output-fed tuples,
both primes/radices and both sum/difference signs. Save mixed-radix native
oracles privately; do not publish input/output arrays. No old model/test run.

History: late-centered trials changed signed arithmetic through the full
point circuit and failed. Affine-offset fusion removed later coordinate
subtractions and failed. Scaled-norm/numerator and uniform-prefix changed
radix with a cancelling factor; this candidate keeps R256 throughout. Loose
complex-square/delta changed linear operand preparation; do not reopen them.
No novelty claim for Montgomery bounds/deferred reduction. Native original
reduce_fused has a fast-path compare/branch; removing these checks is a
modest hypothesis, not a product reduction or twofold proof-speed forecast.

If exact algebra passes, build an isolated native field extension from the
accepted176 source; copy original CIOS only with a renamed helper, replace
private helper references by byte-equivalent free helpers, omit final fused
correction. Leave original generated files/generator/runtime bodies exact.
Expose only fused_sum_scaled/fused_difference_scaled returning below2p.
Native test against new private exact oracles and original sequence, plus
generic gating. Inspect actual optimized consumers before selecting a
complete G2 caller trial. This stage alone selects no performance capture.

No new runtime heap buffer, key/point table/copy, retainedplan, productionpool,
C/ASM backend or RAMcap. Goal verifiedsingleproofCPUAVX>=10xM5CPUall8+Metal
remains active0/8. Canonical37023proofledger unchanged. CommitANDpush each
completed stage with polite factual report. No CPUhelperduringANYrecording.
