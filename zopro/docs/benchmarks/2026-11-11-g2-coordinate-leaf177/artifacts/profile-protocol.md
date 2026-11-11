# G2 coordinate leaf SDK profile panel

The completed full-proof screen passed correctness and 29/31 progression
checks. Both primary32 gain checks failed, with paired latency about0.9% slower
than accepted and1.2% slower than the fresh disabled control. Candidate remains
unadopted. This panel is diagnostic, not a rerun or adoption rescue.

Question: does the coordinate helper reduce G2 sampled residence while adding
enough instructions/cycles elsewhere to offset it, or does G2 itself become more
expensive? Report G2 caller and helper separately and combined, alongside total
instructions/cycles. Sample residence is not exclusive or removable wall time.
This panel does not directly measure op-cache misses or establish their cause.

Use exact OFF/ON copies from g2-coordinate-leaf177/runtime-consumers.json.
Primary1x2,32fixed workers on0–31, original driver and accepted runtime
environment. Three blocks, one predetermined copy index per block(0,1,2), same
pair across plain/stat/cycles within that block. Rotate panels by block, reverse
panel and arm order on odd blocks. No new executable copies or relinks.
Check SHA/device/inode/mode identities of all nine retained copies before/after.

18processes,251requests after3primes:4518samples,4572SDKcalls,18fresh exports.
Seed53899 from g2-coordinate-leaf177/proof-sha256-after-main.json, SHA256
6ce122fa00eea0b0ca93c305a835b266ce33ea3f53f51d5b821423efb3f6630b.
Expected final53917 only after independent Gnark verification, bindings,
freshness and receipt checks. All recording precedes decoding/verification.
During capture only poll the live handle; no other owned work overlaps it.

FIFO-gated stat: grouped user cycles,instructions,cache-misses,branch-misses,
all four present and each>=99.9%running. FIFO-gated cycles samples use fixed
16million period, actual MMAP/PID/TID/CPU attribution,>=1000samples,<=5%unknown,
zeroLOST. Each arm/instrumented-panel median of three paired latency ratios
against plain must be within factor1.05 to treat it as broadly representative.
This descriptive guard does not prove observer equivalence.

Report each copy and aggregates; one block per copy has no per-copy confidence
claim. Reuse the completed g1-offset-profile177 implementation, adapting only
root/consumer/ledger identities. Do not change guards after observing results.
The full screen remains closed. CPU10x for all eight shapes remains unmet.
No AWS resources, production change, push or publication.
