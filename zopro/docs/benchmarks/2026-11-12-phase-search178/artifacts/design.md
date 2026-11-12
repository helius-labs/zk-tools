# Solver-phase search participation

Hypothesis: excess simultaneous searching interferes with sparse witness-solving
work; restrict private-deque search to the first ceil(N/3) existing workers only
during solve_in_with_slots, then restore all workers before canonical conversion.
Original arithmetic, task creation, local pops, external injection, broadcasts,
latches, finite idle rounds and sleep protocol remain. This is not a CPU-worker
execution cap: already active work and injected/broadcast work can still run.

New per-registry atomic limit and mutex-protected scope count; each scope owns an
Arc and restores via Drop. Overlapping/nested participating scopes lift the limit
until all end. Restoration reuses injected-job SeqCst fence, sleepy event counter
and wake-all-current-sleepers request. No new pool/table/key plan/unsafe code.
Private experimental API only; not an upstream Rayon contract or formal proof.

First qualify nested work, panic cleanup, concurrent participating callers,
foreign pools and all-worker broadcast after restoration, plus upstream scheduler
tests. Then a short complete SDK screen includes scope and wake-transition costs,
fixed worker affinity, OFF/ON same binary, exact accepted REF, 3 fixed executable
backings, 1x2 at16/32 and2x2 at32. Each export verifies independently with Gnark.
Freeze sample/order/reader/input pins before first SDK preflight. No performance
claim before measurement. Local rejection does not answer96physical-core behavior.

This differs from closed pause/yield replacement, victim bitmap, coarse proof
graph and fixed separate-query pools. Search restriction may simply trade scanning
for parking/waking or limit useful parallel work. Those costs belong in timing.
Current source includes adopted phi grain and G2 tile batching; old perturbed
C7a search traces motivate this diagnostic but do not predict its gain.
