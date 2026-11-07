# Integer quotient/remainder call boundary

Complete Fq inversion improves **3.59%** and Fr **1.63%** against fresh controls, with31/31winning blocks for both fields. Accepted-library comparisons also improve and both calibration checks pass. The scalar screen selects measurement in the complete prover's real G1/G2 callers. **Production is not adopted and no proof-time gain is established.**

## Change and correctness

The private Rust helper is `#[inline(never)] fn divrem(a,b)->(u64,u64) {(a/b,a%b)}`. Both results remain live across its return boundary before the inverse's zero-remainder branch. This lets one DIV supply both values, at the cost of a call/return. The GCD matrices, termination, Bezout updates and final field correction remain original. No floating arithmetic, new assembly, unsafe operation, allocation, table, key copy, retained plan or pool is introduced in the changed inverse.

The checked native prototype validates146,788pair outputs through widened integer identities and8,192coprime Bezout outputs against an independent i128Euclid recurrence. Its initial prediction of one static helper call site was false: the compiler peels the first iteration, producing one preheader call and one repeated-loop call. Retained CFG review confirms one call per Euclid step. A separate backward edge joins the shared return epilogue; the initial reviewer incorrectly counted that as a loop. Original design, failed reviewer source/summary, correction and retained decodes remain. No successful test or decoder is repeated to alter the count, and no performance gate changes.

That prototype uses checked arithmetic and no final LTO; it does not qualify production codegen. The complete integration separately builds atO3/native/fatLTO/oneCGU and passes16,384inverse outputs,8,192canonical boundary/full-width cases per prime, against independent BigUint inverse references and x*inverse=1. Canonical output and zero handling pass. Qualified pure input/reference helpers are reused; the old floating model and SAE test population are not rerun.

The isolated snapshot has175byte-identical originals, one reversibly changed inverse parent and one new helper child. All176originals are recoverable and all176production hashes remain unchanged. Generated field sources/generators and existing portable module selection are unchanged. Phone/ARM execution and memory pressure are not qualified here.

## Actual consumers and matched screen

FROZEN and OFF are exact previously qualified component consumers: original accepted field1200e635 and fresh original arithmetic. The new ON consumer uses byte-exact previous benchmark driver source and compatible LTO dependencies. Its old cold SAE-correctness entry is never invoked in this trial; the changed inverse paths contain no FP instructions. A separate new correctness executable checks this integration.

Only the new consumer is listed/decoded; previous control decodes are inherited. Actual Fq/Fr inverse bodies contain zero DIV sites and call one shared46-byte/17-instruction helper. That helper has two width-alternative DIV sites, returning both results. Each inverse body has two helper call sites from peeling, rather than two calls for the same step. Fq/Fr body sizes are7,667/7,603bytes versus7,723/7,786in the controls. Stack/call instructions are retained. These are static observations, not dynamic retirement or prover-memory bounds.

The fixed screen runs31blocks×two fields×three arms=186processes on CPU2. Every process prepares the same4,096nonzero canonical inputs outside the clock, warms3×256calls, then makes65,536observed complete inversions. All12,189,696observations remain. Input hashes, checksums and final values match in every paired block; case order rotates/reverses. Medians below summarize31process means. Ratios and bootstrap intervals use paired block observations.

| Field | Accepted-library ns/inverse | Fresh ns/inverse | Candidate ns/inverse | Candidate/fresh ratio [bootstrap95%] | Wins |
|---|---:|---:|---:|---:|---:|
| Fq |531.398209|532.182907|513.201218|0.964127[0.963363,0.964982]|31/31|
| Fr |521.314041|521.692413|513.319092|0.983692[0.981687,0.985323]|31/31|

Candidate/accepted-library ratios are0.965629[0.964868,0.966368] forFq and0.985012[0.983792,0.986266] forFr, both31/31wins. Fresh/accepted-library calibration is1.001558[1.000970,1.002161] and1.001342[0.999746,1.003567], within the declared factor1.03range. Both fields' candidate intervals have upper<1against both controls. No minimum positive-gain floor, favorable-field selection, extension or reroll is used.

All new build, link, correctness, codegen, measurement and analysis workflow stages succeed on their first executions. The earlier prototype reviewer failure remains separate. All clocks finish before analysis, hashes, review or publication. Offline review checks197successful workflow receipts, six new listing/decode receipts,382capture pins, all176source reversals, native identities and all186paired observations/ratio calculations. It does not rerun tests, bootstrap populations, decoders or cryptographic verification.

Next: measure the real inverse call paths within the complete SDK, with automatic profiles and fresh independent Gnark verification. This gives end-to-end caller evidence without adding a new production adapter. All-nine performance/resource/fallback qualification remains required before adoption. A3.59%inverse gain is not a3.59%proof gain, and neither satisfies the original tenfold target. No SDK call, proof, solve, profile or AWSresource is created in this component stage.

## Accepted prover comparison, unchanged

Accepted SDK SHA256 is still`6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. Primary [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md) measured7.928660ms; later7.915095ms and latest matched8.041470ms are separate observations of the same binary. No slower build was adopted.

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

Merge54adoption161.883899ms has no supplied M5 reference. Original all-eightCPU10×goal stays active/unmet0/8. Cross-machine references do not establish equal hardware efficiency. Canonical verified-proof ledger stays32,196. No new AWS/Aeglos, full-proof RAM/minimumRAM, large-circuit, mobile, pressure/swap or concurrent-proof result is supplied.

[Evidence](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), [checksum](artifacts/SHA256SUMS). Includes prototype, source/CFG corrections, changed parent/child, inherited benchmark-source helpers, declarations, receipts, new disassembly, all measurements and review. Targets, executable/library blobs, private continuation notes and proving/input/proof payloads are excluded. Previous control identities/decodes are in the linked [parent evidence](../2026-11-07-euclid-float-quotient176/README.md); absolute paths require reconstruction elsewhere.
