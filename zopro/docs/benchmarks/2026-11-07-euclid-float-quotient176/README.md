# Corrected floating quotient in the complete inverse

The candidate is correct but slower: complete Fq inversion regresses **4.62%** and Fr **7.44%** against fresh controls. Both fields have0/31winning blocks. Accepted-library/fresh-source calibration passes. **This fixed recipe is rejected; production is unchanged.** There is no full-proof speed result.

## Arithmetic and floating environment

For eligible64bit operands, b>(a>>16) implies a/b<2^16. The candidate estimates the quotient through f32 division, then uses an exact u128 product and at most one integer correction to recover q,r with a=q*b+r and0<=r<b. Small32bit operands and large quotients retain integer division. No GCD matrix, Bezout update, termination condition, field representation or final modular correction changes.

With directed-rounding relative error u=2^-23, the combined conversion/division error is bounded by (1+u)^2/(1-u)-1 above and1-(1-u)^2/(1+u) below. Multiplying either by2^16 gives less than1; the integer model checks these inequalities as exact rational comparisons. Positive inputs and eligible quotients are normal f32 values; zero dividend is exact. Thus truncation differs from the exact quotient by at most1. Wider products are explicitly retained in u128 rather than overflowing64bits.

The new normal-f32 model checks146,788pairs across16rounding/DAZ/FTZ settings, totaling2,348,608pair/mode checks. Inputs include4,545boundary pairs,65,536deterministic random pairs and76,707dependent Euclid pairs from2,048chains. Both correction directions, integer fallback classes and products above u64::MAX occur. Quotient, remainder and the widened integer identity all agree with integer references. Model eligibility counts are not a measured workload distribution.

Ordinary floating instructions could change exception flags. Before native timing, the candidate therefore uses AVX512F Rust intrinsics with explicit nearest rounding and exception suppression for both conversions, division and integer truncation. The documented [conversion](https://doc.rust-lang.org/beta/core/arch/x86_64/fn._mm_cvt_roundu64_ss.html) and [division](https://doc.rust-lang.org/stable/core/arch/x86_64/fn._mm_div_round_ss.html) support those controls. No external implementation or speed figure is imported.

The native SAE helper passes9,394,432pair/mode checks across64settings: four rounding modes, DAZ/FTZ, all-masked/all-unmasked exceptions and clear/set sticky flags. Every call preserves MXCSR. Complete inverses add131,072output checks over1,024nonzero boundary/full-width inputs per prime against independent BigUint inverse references and x*inverse(x)=1; zero handling also passes. These are new native/operator checks, not repetitions of the old model or field suites. Tests restore the original floating environment.

## Source and actual consumers

One existing inverse parent receives a reversible two-binding replacement and module suffix; one new helper child is added.175of176snapshot originals remain byte-identical, all176are recoverable and all176production hashes are unchanged. Generated field sources/generators are unchanged. The helper adds no heap allocation, lookup table, key copy, retained plan or pool. Non-AVX512 and portable paths retain the original source; physical phone/ARM execution is not qualified here.

Three consumers share identical driver source and O3/native/fatLTO/one-CGUflags. FROZEN links the original accepted field library1200e635; OFF links a fresh original path; ON selects the helper. FROZEN is a newly linked component consumer, not the accepted SDK binary. Reference/hash dependencies are built with compatible LTO bitcode. The two successful field builds are retained throughout preparation repairs.

Saved accepted-SDK inverse disassembly and its successful receipt are reused. New component consumers are listed/decoded once each. Both original/fresh inverse bodies contain six integer DIV sites, including32/64bit alternatives and separate quotient/remainder loop work. Each candidate body has two integer DIV sites and four explicit SAE instruction sites: two conversions, division and truncation. Static sites are not dynamic instruction retirement or removable wall cost. Candidate Fq/Frbody sizes are7,771/7,952bytes versus7,723/7,786in both controls.

Three preparation failures remain in the evidence: a driver JSON format string, reference dependencies without LTO bitcode, and an offline reviewer assuming every manifest member was UTF8. Repairs change only the failed harness/reader stage; the last failure occurs before any new listing/decode. No successful field build, native correctness population, model, listing or decoder is rerun for those repairs.

## Fixed complete-inverse screen

The campaign uses31blocks×two fields×three arms=186processes on CPU2. Each prepares the same4,096nonzero canonical fields outside the clock, warms3×256calls, then observes16passes over all inputs:65,536complete inversions per process,12,189,696total. Input hashes, output checksums and final outputs match in every paired block. Order rotates/reverses; no subset, extension or reroll is selected.

| Field | Accepted-library median ns | Fresh median ns | Candidate median ns | Candidate/fresh paired ratio [bootstrap95%] | Wins |
|---|---:|---:|---:|---:|---:|
| Fq |529.673569|530.331711|555.102036|1.046153[1.043746,1.047676]|0/31|
| Fr |519.542938|518.985703|557.746323|1.074388[1.073785,1.074998]|0/31|

Candidate/accepted-library ratios are1.048612[1.047952,1.049233] forFq and1.073342[1.072209,1.074408] forFr, also0/31wins. Fresh/accepted-library calibration ratios are1.002350[1.000730,1.004905] and0.999027[0.997889,1.000079], within the declared factor1.03range. Both fields needed interval upper<1against both controls; all four gain checks fail. There is no minimum positive-gain floor. Fewer integer DIV sites did not improve the complete operation; these data do not isolate a precise cause among conversions, correction, scalar division, branches and compiler context.

All clocks finish before analysis, review or publication. Offline review checks198successful workflow commands plus15read-only listing/decode commands, three retained failures, source pins, all176source reversals, model bounds/counts, native identities, all186paired observations and ratio/win arithmetic. It does not replay tests, bootstrap populations, decoders or a cryptographic verifier. No batch caller, SDK, proof, solve, profile, AWSresource, pressure/swap, minimum-RAM or concurrent-proof result is produced. Further investigation requires a distinct mechanism, rather than a favorable-only repeat of this one.

## Accepted prover comparison, unchanged

Accepted SDK SHA256 remains`6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. Primary [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md) measured7.928660ms; a later baseline7.915095ms and the latest matched comparison8.041470ms used that same binary. No slower version was adopted.

| Shape | Local adoption ms | Supplied M5 CPU ms | Supplied M5 Metal ms | CPU10×target ms |
|---|---:|---:|---:|---:|
|1×2|7.928660|37|24|3.7|
|2×2|11.151932|59|30|5.9|
|4×2|16.086386|109|48|10.9|
|8×2|24.179453|189|74|18.9|
|12×2|33.884455|284|101|28.4|
|16×2|40.077381|335|124|33.5|
|24×2|65.850777|528|176|52.8|
|Merge24|65.060937|507|175|50.7|

Merge54adoption161.883899ms has no supplied M5 reference. Original all-eightCPU10×goal stays active/unmet0/8. Cross-machine references do not establish equal hardware efficiency. Canonical verified-proof ledger stays32,196; no new AWS/Aeglos comparison is supplied.

[Evidence](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), [archive checksum](artifacts/SHA256SUMS). Models, helper/driver source, changed parent, declarations, receipts, disassembly, measurements and review are retained. Targets, executable/library blobs, private continuation notes, proving fixtures and witness/request/public/proof payloads are excluded. Absolute paths require reconstruction elsewhere.
