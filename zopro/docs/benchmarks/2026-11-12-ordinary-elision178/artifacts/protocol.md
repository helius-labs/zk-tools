# Ordinary G2 stage elision diagnostic

Question: how do ordinary bucket fill and subsequent part-merge/weighted-reduction
work interact in complete query latency? The previous top-only elision left most
threaded query time. This four-arm intervention removes the two ordinary stages
separately and together while preserving the query's outer task geometry. It does
not isolate arithmetic from memory or prove a C7a cause. No AWS claim is intended.

Start from the adopted 177-file manifest, b646 SDK baseline. In a private snapshot,
wrap the twenty ordinary part/window fills and ten ordinary window reductions.
Mode 0 executes both; 1 replays fills; 2 replays reductions after still executing
fills; 3 replays both. All modes save identical first-prime outputs. Replay fills
clone 2048 affine buckets per slot; reduction replay returns the saved projective
sum. Fill buffers are still produced and dropped when reduction is elided. The
indexed top, grouping, scalar split, final fold and normalization remain active.
Use fixed mode/process and positional slots only for one immutable input. Exact
call/execute/slot/byte counts and every complete query's independent Arkworks
oracle must pass. This is intentionally NOT a production cache or valid speedup.

Reuse top-elision177's caller, perf support, fixtures and independent oracles.
Current sources, build recipe, helper, protocol and driver are pinned before build;
the linked executable, three independently copied backings, inputs and capture
environment are pinned before any caller. Rust 1.98.1, znver4, O3/fat LTO/one CGU,
current native selector including bounded G2 batching. No production edits.

Six contexts: 1x2 and grouped 2x2, at 1/16/32 workers with fixed CPU2 or 0..N-1
affinity and 512 KiB worker stacks. All 24 one-timed-call preflights must pass first.
Then three blocks, each using a distinct byte-identical executable, rotate/reverse
contexts and mode order. Each of 72 main processes uses three excluded primes,
17 timed queries and one postcheck. All samples are retained; no extension or
favorable backing selection. During capture only poll its live workflow handle.

Report all four times, paired effects of removing each stage, both-stage residual,
CPU service and process peak RSS. A direction disagreed across blocks is
inconclusive. Three blocks are descriptive; no confidence or adoption gate.
Graph/cache/allocation effects and replay-copy/observer costs remain; do not add
the individual savings into a supposed proof bound. Both-stage residual retains
the indexed top and all other query work, so it is not pure scheduler overhead.
Prefer a direction with consistent threaded headroom for the next structural
prototype; if removing a stage changes mainly CPU, deprioritize it for latency.
Review already rejected implementations before selecting that prototype.

Preparation allowance: 45 minutes to a frozen runnable screen, then simplify or
justify more preparation. Reuse the existing xtask workflow/readers; no all-shape
SDK qualification here. Current full-proof all-eight goal stays unmet; no proofs
are exported and canonical ledger stays 54927. This diagnostic's retained cache
is about 5 MiB in every arm, not a production memory policy or phone qualification.
