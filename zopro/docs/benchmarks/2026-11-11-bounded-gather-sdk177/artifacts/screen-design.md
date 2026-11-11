# Local complete-proof screen design

Status: design prepared before any SDK preflight. Complete builds and freeze
exact artifacts, inputs, runner and independent reader together before running.
No SDK recording has started.

Purpose: measure the opt-in pool-sized grain in canonical A/B gathers against
the accepted G2-grain prover, while checking the fresh disabled build. This is
a small scheduling hypothesis. Earlier gather stage observations do not predict
a twofold whole-proof improvement. Public library synthetic results are not
proof-level qualification. The library dependency is not added to the prover.

Consumers: REF is retained accepted SDK c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe;
OFF/ON are coherent Rust 1.98.1, znver4 builds from this 177-file snapshot.
Both retain all accepted runtime flags, including zopro_phi_grain. Only ON adds
zopro_bounded_gather. Cargo's native selector changes versus the historical
artifact must be reported, not claimed byte-equivalent. Freeze all three
executable hashes and resolved libraries. No artifact substitution after timing.

Local Ryzen: 1x2 at 16 and 32 workers; 2x2 at 32 workers. Fixed affinity 0-15
or 0-31 respectively, one worker per allowed logical CPU, original untraced
driver. Report physical-core/SMT scope and shared-host limitations. Use the
accepted environment. No new production pool, key table or artificial RAM cap.

Functional preflight: all eight supplied shapes and merge54, all three consumers,
32 workers, one measured call after three primes. 27 fresh exports, 108 SDK calls.
Independent Gnark verification and image/public bindings must pass before the
main recording. Begin from the current 51586 verified-export ledger; expect
51613 only after these exports are independently verified.

Main: 31 fixed blocks, three contexts, three consumers. Rotate contexts by
block modulo three, reverse odd blocks; rotate consumers by block modulo three,
reverse odd blocks. 101 measured requests for each 1x2 context, 31 for 2x2,
each after three primes. 279 processes/exports, 21669 timed samples, 22506 SDK
calls including primes. Combined preflight/main: 306 exports, 22614 SDK calls,
21696 measured samples; expected final ledger 51892 if all are verified.
Independent main-export verification runs after all timing completes.

Use each process's arithmetic mean, paired within exact block/context. Report
median ratios and sorted-31 bounds at zero-based ranks 9/21 with their binomial
marginal coverage; no joint confidence or stationarity claim. Freeze the
independent statistical implementations before preflight. No gain floor.

Progression criteria, 18 checks: ON latency upper <1 against REF and OFF for
primary 1x2/32 (2); latency upper <=1.02 against both for the other two contexts
(4); whole-loop CPU/request upper <=1.02 against both for all three contexts (6);
max lifetime RSS <= comparator maximum *1.05 +1MiB for all contexts and both
comparators (6). OFF/REF is a reported control comparison, not an additional
undeclared veto. Report absolute savings, ratios, CPU, RSS, faults and switches.

This is a local progression screen, not full adoption: all-shape performance,
portable behavior, applicable concurrency/resource qualification and an AWS
scaling conclusion remain separate requirements. All8 preflight samples are
functional checks and must not be sold as all8 steady-state performance.
Report existing accepted CPU/Metal comparisons as historical, not new results.

If the screen supports progression, declare a separate diagnostic panel with
automatic CPU profiling and observer checks before its first capture. Do not
conflate sampled scheduler share with removable latency. If it fails, close
the exact screen without reruns, sample extension, threshold changes or selective
contexts. Preserve correctness/resource/performance/adoption conclusions separately.

During SDK preflight or recording, poll only the confirmed live handle until
terminal. No overlapping agent-owned build, helper, analysis or report work.
Local commits are allowed; no push or publication without explicit user approval.
The full verified ALL8 10x M5 CPU objective remains active and unmet.
