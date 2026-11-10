# Complete-query screen: fused correction mask

Only eight-lane fused correction changes. Existing needs==0 path is exact;
unequal top limbs reuse the mask in one subtraction; ties use original reduction.
No field multiply/square count, representation, core algorithm, grouping, queue,
inversion, source point, scratch, pool or memory policy changes.

Eight real contexts:1x2-B and2x2-B each1/16/32workers;1x2-A32 and2x2-A32
unchanged controls. Complete queries use their existing Rayon pool internally,
512KiB stacks and individual fixed affinity (soloCPU2,others0..workers-1).
Timed B scalar reset, grouping/admission, windows, merges, weight/Horner,
arithmetic, allocations/drop and canonical affine normalization. Loading/pool,
three primes, postcheck and oracle comparisons outside timing. Every timed output
must match retained independent Arkworks full-query oracles; points unchanged.

Fixed31rotated/reversed matched blocks,51queries/process, frozen before any new
preflight. More observations than prior15x17 screens to resolve small changes;
no optional extension. Per-process median51, paired ratios over31blocks. Sorted
zero-index ranks9/15/21 give bounds/median; sign coverage is conditional on
independent exchangeable blocks and not familywise. No minimum gain floor.

54checks:8OFF/REF calibration intervals within[1/1.03,1.03];16ON/control
nonregression upper<=1.02;2primary-A32 unchanged intervals withinfactor1.03;
2primary-B32 and2equal-log two-B-shape32 gains upper<1 and>=21of31wins
against BOTH controls;8ONmaxRSS<=maxcontrols*1.05+1MiB;8PMU+8profile quality.
The RSS comparison is experimental guard, not architecture or minimum-RAM cap.

24preflights precede744latencyprocesses; then8PMU+8profiles,400queries each.
Grouped cycles/instructions/cache-misses/branch-misses:u >=99.9%running.
Profiles fixed16Mcycles,512pages,CPU/time/period/PID/TID/executableMMAP,
>=1000samples,unknown<=5%,zeroLOST. All40offline decode steps AFTER every
recording.824steps/784callers/47504checked queries including primes/postchecks.

No helper/build/hash/decode/analysis/report/push overlaps ANY preflight/recording:
ONLYpoll exact livehandle until terminal. Keep every result, failure and uncertain
positive. No changed threshold, favorable subset or unchanged rerun. No goal
completion/adoption from this component: success permits ALL8+54 SDK/Gnark/
resource/portable/concurrency qualification. Canonical43730/currentSDK unchanged.
No new AWS or Aeglos measurement. CPU10xALL8 active/unmet0/8 with CPU+Metal refs.
