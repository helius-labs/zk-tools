# Coefficient pre-add prover integration and proof preflight

The native candidate now passes prover integration and **27 fresh independent Gnark proof checks**: accepted, fresh OFF and ON across all eight supplied shapes plus merge54. This qualifies a matched primary performance pilot. **No controlled timing comparison or production adoption is established.**

## Integration and identities

The [native field qualification](../2026-11-07-cios-preadd-native176/README.md) is inherited without replaying its successful tests, models, builds or decodes. The changed field then passes125core lib tests, two prover lib tests and six cubic integration tests, all unfiltered with no failures or ignored tests. Existing tests can contain conditional fixture checks; these counts do not establish every optional fixture scenario. New integration runs do not replay the earlier field suite.

Coherent OFF/ON production libraries and SDK consumers were built using the unchanged accepted SDK driver and perf support. Actual consumer receipts contain O3, native CPU, fatLTO and one codegen unit. The source differs from accepted production only in the three qualified field/generator parents; no point-kernel shim or altered prover schedule is added. Both G1 and G2 can use the changed Fq8 arithmetic.

|SDK arm|Binary SHA-256|
|---|---|
|Accepted|`6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`|
|Fresh OFF|`e8e50d3724624f48f1ada8808efda681771d951b4d8b1c9265407f027c116124`|
|ON|`358673f246a3341e517615f8bea532fb60ff943b5911b9f18f96e95838a72e9c`|

## Independent proof preflight

Each arm completes one measured diagnostic request after three primes on32workers, pinned to CPUs0–31. All nine shapes pass in all three arms:1×2,2×2,4×2,8×2,12×2,16×2,24×2,merge24andmerge54. This is108SDKrequests including primes and27exports, not27performance trials. Diagnostic clocks are retained but excluded from performance inference.

Each exported164-byte proof passes the pinned external Gnark verifier. Image hashes, expected public-input hashes, proof hashes and verifying-key hashes are bound to the exact command receipts. The separate reviewer confirms all27exports are distinct and absent from the prior31,387-entry ledger. The new canonical ledger contains**31,414**identities; the old ledger remains unchanged.

An initial reviewer attempted to read its containing workflow's completion file before its own command had completed. All54proof/verifier commands had already passed. The preserved offline-only repair checks the55completed preparation/proof/verifier steps and their exact argv/exits/stream hashes, then reviews proof bindings and freshness. The fixed review's compile and execution pass. No successful SDK request, proof, verifier or build is repeated for this repair. A missing copied-driver path was also corrected before the SDK build plan executed; its byte-exact sources are unchanged.

The graph/field qualification supplies no new key table/copy, heap workspace, retained plan, pool or representation. This preflight does not measure a live-memory peak, minimumRAM, portable/mobile, memory-pressure or huge-circuit feasibility result.

## Next performance stage

Freeze the primary32-worker pilot with accepted/OFF/ON,15rotated/reversed blocks, three primes and151measured requests per process. Require ON's upper paired ratio below1and at least10/15wins against both controls, with fresh OFF/accepted calibration within factor1.03; no gain floor, exclusions, extension or unchanged reroll. Verify every process's exported proof independently and check freshness. Record focused FIFO-gated counters/profiles for all three arms after latency, with no other owned CPU work during recording. A positive pilot selects separate all-eight plus merge54 performance/resource qualification; it is not adoption or goal completion.

Accepted local32-worker **1×2 remains 7.915095ms**, versus supplied M5 CPU37ms and Metal24ms. The all-eight verified CPU10× goal remains active and unmet, **0/8**; [full inherited comparisons](../2026-11-07-g2-real-norm-z176/README.md). Accepted merge54 remains161.718956ms without a supplied M5 reference. No new AWS/Aeglos comparison is added.

[Preflight review and proof identities](artifacts/preflight-review.json), [SDK identities](artifacts/sdk-identities.json), [declared stage](artifacts/design.md), original/fixed reviewer sources, workflow plans, pins and exact streams retain the evidence. Actual proof/public payloads, keys, requests, binaries and libraries stay private. The canonical hash ledger stays local at the path recorded in the review.
