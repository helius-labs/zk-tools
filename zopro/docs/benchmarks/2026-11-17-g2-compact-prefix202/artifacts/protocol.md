# Compact G2 prefix records

Challenge: saved prefix/norm values in borrowed G2 use five-vector FqBatch values,
but existing PackedIfma8 preserves their same normalized numerical Montgomery
values in four vectors. Store exactly two packed values per group, expand only
when multiplying in reverse. Record payload falls640->512bytes (20%):200-pair
batches16000->12800bytes,352-pair28160->22528. Point/scalar formats and field
arithmetic remain exact; no key table, copied points, new pool or RAM cap.

This changes prefix scratch representation, not cached denominator/X records,
point-array compression, exception classification, prefix chain topology or the
closed compact tree. Existing packing/unpacking field API/qualification is reused.
No novel field algorithm or2x prediction:1450static kernel IFMAs remain expected;
extra packing instructions can outweigh lower record traffic. C7a has32KiB L1D
versus local48KiB; whole working-set residency/cache cause is NOT established.

Use current179-source manifest196 and ledger58257 from201. Private mode only,
separate compact Vec in Fq2 scratch; ordinary math body and original OFF remain.
Own/borrow/default and configured fallback retain existing behavior. Charge the
new24B Vec header for every BatchAdd workspace; payload bound conservatively sums
original allowance plus compact Vec, so reuse does not rely on dropping original
scratch. Reserve compact records exactly; do not grow a second point-sized buffer.
No production adoption until full SDK/resource/portable qualification.

First run focused curve/tail/sign/reuse/capacity tests, complete independent
retained MSM oracles and1/4/16/32pools512KiB stacks. Inherit unchanged field and
PackedIfma8 tests; new whole-caller coverage verifies this placement. Check all179
production hashes and reverse original source edits. Inspect exact linked consumer
if the result leaves a mechanism question; no generic outline/layout rescue.

Local directional screen:1x2/grouped2x2 at1/16/32workers,3primes+17timed+1postcheck,
three independent byte-identical executable backings,three rotated/reversed blocks.
12preflights before36main processes. Include scalar copy/grouping/split/fills/
merges/weighting/normalization and allocations. All outputs match retained Arkworks
hashes; immutable points rehashed outside time. Report CPU/lifetimeRSS/requested
compact capacities. No sample exclusions/extensions or parameter sweep.

Local progression: primary32 wins3/3 and other5 medians<=1.02 selects profile and
full-SDK screen. For the separately stated C7a data-cache question, a local-neutral
result (all six median ratios<=1.02) also selects one same-consumer32/96 C7a screen;
otherwise close without cloud. This is a research spending rule,not latency proof.
C7a protocol must be frozen before its first target request; no target claim from
local data. Preserve local failure independently. No gain floor for adoption.

First preparation/build/local-result allowance15minutes; if overrun document
cause before continuing. Use existing Rust/xtask runners and inherited fixtures,
not a new framework. Only poll confirmed live handles during capture. No unrelated
CPU workload alongside measurement. No new SDK proofs in query screen. Fullall8
CPU10x remains0/8; reportCPU/Metal through201/196 rather than invent new proof times.
Commit locally after resolved experiment. No push without fresh authorization.
