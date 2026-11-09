# Euler row capacity experiment

The accepted SDK live unwind (sdk-map-owner176, pushed a076636158) identifies
two64MiB+4KiB mappings/request as solver constraint rows. The canonical Euler
consumer packs a separate full domain and returns only actual constraint rows.
This candidate reserves the actual row count, using the existing zero-domain
argument to the solver. Non-Euler and all other callers retain their original
domain capacity. Arithmetic, initialization/definite assignment, wire/slot
ownership, allocator tags, release policy and forecasts are unchanged.

Private OFF is the exact accepted176source files, ON differs only in canonical
call-site capacity and a test insertion. Both rebuild coherent core/prover/SDK
against the same frozen accepted field dependencies. No retained workspace,
lookup table, pool, memory limit or changed allocator setting is introduced.
This may change glibc thresholds; requested bytes do not imply physical RAM.

Before any latency, qualify row values/slots/C on original and reloaded plans,
Euler output equality, invalid witnesses and incomplete initialization at1/16/32
workers with both initialization policies. No unchanged field test recounting.
Then run fixed SDK preflights/proofs and freeze a matched latency protocol with
frozen accepted REF, fresh OFF, ON. Keep worker placement fixed. Gate the primary
and54case first; all8shape correctness/latency/resource qualification is required
before adopting any positive result. Instrumented runs are never new latency.
All recordings precede analysis; only poll the exact live workflow handle.
