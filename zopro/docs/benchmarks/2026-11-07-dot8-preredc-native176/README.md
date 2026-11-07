# Horizontal dot8 native screen

The proposed dot product is **9.39% faster on dependent-result calls but31.53% slower on independent calls**. It passes native correctness, yet fails the fixed screen's throughput requirement. This implementation is closed without production adoption or a favorable-only retry. No full-proof improvement is established.

The [integer model](../2026-11-07-dot8-preredc-math176/README.md) qualified summing eight canonical R256 products before one scalar Montgomery reduction. The [current census](../2026-11-07-dot8-dispatch176/README.md) finds11,422primary interpreter calls. This screen implements that operation in one guarded BN254 child module. Only an append-only module declaration changes an existing snapshot parent:175original files remain exact, all176are recoverable, and all176production hashes remain unchanged. Generated field sources and their generator are byte-identical.

## Correctness and emitted work

The new native consumer passes63,122comparisons across Fr/Fq against both independent BigUint modular calculations and the accepted Ifma8 product/horizontal-sum control. Each prime covers146canonical boundary values and their cross-product, explicit zero/max vectors,8,192deterministic random vectors,2,048dependent vectors and all three retained integer-model correction examples. Both canonical output and exact value are checked. This exercises the new native operator; the old integer model and successful old field/SDK suites are not replayed.

The first consumer build failed because a BigUint shift needed an explicit integer type. Only that driver type annotation was repaired; its original template, failed receipts and logs remain. The successful lock stage was not repeated. The repaired build and new native correctness stage succeeded. Actual linked-consumer compiler arguments establish O3, native targeting, fatLTO, one codegen unit and the accepted production flags plus the isolated diagnostic cfg. Four direct dependency identities are retained.

Linked Fr wrapper inspection finds:

| Static property | Control | Candidate |
|---|---:|---:|
| Raw/reduction IFMA instructions |100|50|
| Total listed instructions |342|444|
| Conditional/unconditional jumps |1|0|
| Calls |1bounds-panic path|0|

The candidate adds16scalar `mulx`, four `imul`, ten horizontal vector reductions, packing and scalar carry/correction work. Its wrapper saves six registers and uses stack references down to`rsp-0x11`; the control saves two registers and subtracts0x28from its stack pointer. These are static wrapper observations, not total prover stack/RAM bounds. The candidate contains no allocation call or runtime table lookup. Removing half the IFMAs does not remove half the complete operation's work.

## Matched complete arithmetic calls

One fixed campaign ran31blocks×two modes×two arms=124processes on CPU2. All calls include packing, product generation, horizontal accumulation and final correction; both arms have equivalent noinline wrapper boundaries. Every process prepares the same1,024canonical vectors outside the clock, performs3×1,000warm calls, then200,000observed calls. Dependent mode feeds each result into the next input. Independent mode retains outputs without feeding them back. All24,800,000observed calls are retained. Input hashes, output checksums and final values agree between arms in every pair.

| Mode | Control median ns/call | Candidate median ns/call | Paired candidate/control geometric mean | Paired bootstrap95% interval | Wins |
|---|---:|---:|---:|---:|---:|
| Independent |21.508945|28.200860|1.315304|[1.311342,1.319574]|0/31|
| Dependent |39.551905|35.776835|0.906112|[0.904168,0.908048]|31/31|

The positive dependent-call result is retained; it does not cancel the independent-call regression. There is no minimum positive-gain threshold, favorable mode selection, extension or reroll. This is a same-consumer kernel comparison, not the actual inlined interpreter, a fresh accepted-SDK calibration, a solver benchmark or a proof. The measurements do not establish how these instruction tradeoffs affect the real solver. Under the declared screen this fixed recipe closes; the broad idea of delayed reduction is not disproved for other independently justified callers.

All recordings completed before analysis, review or publication. Offline review checks128successful command receipts, the retained failed build,378current source pins,124paired observations, output bindings, ratio/win calculations, emitted-work identities and all176production/inverse-source hashes. It does not rerun native tests, disassemble again, redo bootstrap populations or verify cryptographic proofs.

## Accepted prover state

Production remains SDK SHA256`6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. Primary [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md) measured**7.928660ms**, a later baseline**7.915095ms**, and the latest matched comparison**8.041470ms**for that same binary. No slower code version was adopted.

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

Merge54adoption161.883899ms has no supplied M5 reference. All-eight CPU10×status remains0/8. Cross-machine references do not establish equal hardware efficiency. This stage creates zero proofs, solves or profiles; canonical verified-proof ledger remains32,196. No new AWS/Aeglos, mobile, huge-circuit, pressure/swap, minimumRAM or throughput-of-concurrent-proofs result is supplied.

[Evidence archive](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), [checksum](artifacts/SHA256SUMS). Evidence retains the design, new module, original/repaired drivers, changed parent, native/build receipts, disassembly, complete measurements, analysis and review. Targets, executable/library blobs, proving fixtures and witness/request/public/proof payloads are excluded. Absolute paths identify the working environment and need reconstruction elsewhere.
