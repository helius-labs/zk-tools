# G2 prefix record with recomputed reverse norm

The isolated implementation passes **126 core tests in fresh OFF and ON builds**. Each ON streaming record retains one prefix instead of a prefix/norm pair, reducing logical payload **640→320 bytes per eight-point group**. SDK integration is the next stage. **No timing, full-proof qualification or adoption is established.**

## Change and tradeoff

The forward pass computes the same norm and advances the same prefix, then discards that norm. The reverse pass recomputes it from the denominator already reconstructed for slope evaluation. Existing distinct destination indices and immutable incoming points ensure the denominator is unchanged; scalar tails use distinct destinations as well. This preserves the accepted prefix/inverse recurrence.

Both owned and signed-borrowed G2 prefix streaming use the new record. G1, the generic tree's separate records, scalar tails, field arithmetic, solver, queues and scheduler retain their original source. OFF retains the norm field; ON omits it at compile time. The same vector header and allocation count are used, without another buffer, table, copied key, retained plan, pool, unsafe block or fixed RAM cap.

The trade is explicit: one extra fused norm per full group changes variable source work from **7VM+5VF to 7VM+6VF**, nominal IFMA proxy **1,450→1,600 (+10.34%)**. Only record payload and traffic decrease. At 25/44 groups, logical payload falls from 16,000/28,160 to 8,000/14,080 bytes. This may help a working set, but does not predict a twofold or any measured speedup.

The [earlier prefix-record experiment](../2026-10-25-prefix-records-component/README.md) retained a prefix/norm pair for G2. Cached-X and denominator-record alternatives retained more coordinates; norm-heap changed dependencies while retaining pair storage. This construction instead discards and recomputes the norm. The [protocol](artifacts/design.md) records that distinction and the prospective complete-SDK gate.

## Qualification and review

One new test checks the actual 320/640-byte layout and 64-byte alignment, then compares complete signed/owned bucket outputs over 16 lengths from 0 through 511, both canonical settings, both inversion settings and four successive updates. It checks logical record counts and that the old group vector stays unallocated. The unfiltered core suites each pass 126 tests with none failed, ignored or filtered. These are changed-core qualifications; the unchanged field suite is not replayed. Existing tests can contain conditional fixture checks; the count does not prove every optional fixture was exercised.

The first OFF test compilation failed because the new test used a nonexistent generator constant. The repair calls the existing generator method and changes only test code; the runtime helper prefix remains byte-exact. No tests had run in that failed step. The successful OFF/ON suites each execute once. An unused inherited candidate cfg was removed from the unstarted ON command; final OFF flags match accepted flags, and final ON adds only `zopro_g2_prefix_replay`.

The source snapshot has 177 files: 173 accepted parents remain exact, three edited parents recover their original hashes by explicit inverse edits, and one child is added. All 18 field parents, including generated code, remain byte-exact. The reviewer checks those bindings, source pins, actual command argv, recorded build/runtime variables, successful exits, stream hashes and suite counts. Workflow receipts omit `RUSTC` and `RUSTUP_TOOLCHAIN`; those settings remain in the pinned plans rather than being claimed as recorded environment fields.

Two offline reviewer attempts fail before producing a review: one erroneously applies a no-buffer assertion to the test module, and one expects unrecorded environment variables in command receipts. Separate corrected reviewers preserve the original sources, binaries and failures. The final reviewer scopes the buffer assertion to production code and states the receipt boundary above; its build and execution pass. No successful native suite or model is repeated. An empty review-plan preparation was also corrected before any review command ran. [Final review](artifacts/native-review.json).

The memory result is a native type size and logical payload check, not process peak RSS, a complete stack bound or minimum RAM. The old allocation forecast is unchanged and retains its original payload envelope. Actual caller stack, complete-proof memory behavior and portable/mobile/huge-circuit/pressure qualification remain outstanding.

## State

Production remains unchanged. Accepted local 32-worker **1×2 remains 7.915095 ms**, versus supplied M5 CPU 37 ms and Metal 24 ms; [inherited all-shape comparisons](../2026-11-07-g2-real-norm-z176/README.md). The CPU10× goal remains active and unmet on all eight shapes. No proof or timing is created by this qualification; the canonical proof ledger remains **31,465**. No new AWS or Aeglos comparison is made.

Source edits, original/fixed test and reviewer sources, plans, pins and exact native receipts accompany this report. Proof/key/witness payloads, libraries and binaries stay private. Private root: `/home/ubuntu/src/zopro-perf-work/g2-prefix-replay176`. Next: complete SDK builds, independently verified all-shape preflights, actual caller code audit and the frozen primary full-proof comparison before any performance decision.
