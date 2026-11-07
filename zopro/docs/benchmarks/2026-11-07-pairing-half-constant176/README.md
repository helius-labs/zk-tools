# Compile-time half constant in G2 preparation

The candidate replaces one runtime inversion of 2 in each nonidentity G2 constructor with its compile-time Fq value. Native and portable pairing checks pass, all nine fresh proof exports independently verify, and the linked SDK constructor loses the inverse call. **No proof-speed gain or source adoption is established by this qualification.** A separate matched build comparison follows before an engineering decision.

## Change and correctness

Only the local `two_inv` binding in `G2Prepared::new` changes. A Rust const block evaluates `(p+1)/2` in the existing Montgomery representation. The identity early return, line formulas/order, vector capacity and API remain original. The logical value occupies 32 bytes independent of keys or circuit size; this is not a whole-binary or process-memory bound. No new allocation, buffer, key table, retained plan, pool or runtime option is added.

An independent four-limb integer calculation checks `2·half=p+1`. The focused field test also checks `2·half=1` in Fq and equality with the original inverse. It compares all prepared coefficients with an unchanged original-constructor reference over 34 public inputs: identity, generator and 32 full-width derived points. The 33 nonidentity inputs produce **2,871 lines**. Both native and portable configurations reapply this same population; it is not 68 distinct inputs.

Each configuration also reruns the existing independent-Arkworks pairing test: eight full-width pairing comparisons plus a pairing-product identity and a negative case. Native runs 2 tests with 34 unrelated tests filtered; portable generic/no_asm/no_ifma runs 2 with 10 filtered. These are fresh checks of the changed preparation path, not fresh full field/core/prover suites or physical-phone qualification. All four native build/test/link workflow steps pass on first execution.

The isolated source has 176 files: 175 accepted originals exact and one parent with a reversible binding replacement plus a test-only suffix. Reversing both edits recovers every accepted hash. Production remains unchanged.

## Actual linked constructor

The candidate SDK coherently links eight libraries with the actual final `-O`, native CPU, fat LTO and one codegen unit. Accepted SDK6d83 is the frozen comparison. Its saved symbol listing is reused; only the new candidate is listed, and each previously unaudited constructor is decoded once.

|Linked constructor|Bytes|Decoded instructions|Direct Fq inverse calls|
|---|---:|---:|---:|
|Accepted|38,297|7,046|1|
|Candidate|38,156|7,025|0|

The original compiler did not fold that inverse call. The candidate removes the call and 21 constructor instructions. This checks emitted work at this boundary, not dynamic proof instructions, cache behavior or latency. Other prover inversions remain; no causal whole-proof speed fraction is inferred.

The first descriptive row counter included byte-continuation rows: 8,236/8,223 addressed rows, including 1,190/1,198 continuations. The independent reader separates actual mnemonic rows and supplies the corrected table above. Original counts and sources remain. The inverse-call counts used for selection were correct and unchanged; no decoder, qualification, capture or verifier is repeated for this classification correction.

## Complete proof correctness

All nine supplied shapes run with 32 workers on CPUs 0–31, each with three primes and one observed complete SDK request: **36 requests and nine fresh exports**. Independent pinned Gnark verification checks 164-byte proofs, 44-byte public inputs, image/public/VK bindings and freshness. Canonical proof ledger advances **31,913→31,922**.

All 18 SDK/verifier commands succeed once. Driver clock/RSS values are correctness-preflight diagnostics, without a matched performance comparison. No speed, RSS saving, minimum RAM, pressure/swap, concurrent-proof, ARM/phone, 250/4999 or new AWS/Aeglos claim follows.

The independent reader checks 312 analysis-input pins, 25 workflow receipts, three read-only listing/decode receipts, all 176 source recoveries, the integer half identity, native/portable results, coherent binary/libraries and all nine fresh proof bindings. It independently classifies instruction versus continuation rows. It is not a second cryptographic verifier or decoder. Its review passes on first execution.

## Accepted comparison and next decision

Accepted production176/SDK6d83 remain unchanged. Inherited local 32-worker 1×2 is **7.915095 ms**, versus supplied M5 CPU 37 ms and Metal 24 ms. Merge54 remains 161.718956 ms without an M5 reference. [All eight accepted CPU/Metal comparisons](../2026-11-07-whole-field-census176/README.md) remain unchanged; the verified CPU 10× goal remains active and unmet, **0/8**.

The next experiment compares the complete candidate build with the exact accepted SDK across all eight shapes plus merge54, retaining every observation. This is a constant-folding cleanup with explicit emitted-work removal, not a proposed twofold MSM improvement. Any engineering adoption must distinguish a nonregression result from a demonstrated latency gain, and keep the full CPU 10× objective unchanged.

[Artifacts](artifacts/) preserve the changed source parent/test, design, helper sources, plans, hashes, exact receipts, two constructor decodes, corrected review and fresh proof inventory. Keys/images/requests/witnesses, proof/public payloads, targets and executable/library blobs are excluded.
