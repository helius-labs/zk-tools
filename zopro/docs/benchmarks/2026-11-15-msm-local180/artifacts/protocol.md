# Local phase scaling and observer calibration

Prospective diagnostic, primary 1x2 only. No optimization or adoption decision.
Exact accepted Zopro REF, diagnostic same-binary OFF/ON; retained Aeglos REF,
fresh unmodified-source FRESH, same-binary diagnostic OFF/ON, both Sorted and
Windowed. All executables have already passed the 4-worker full-SDK preflight.
Original key/request/public bindings and default preparation policies retained.

This host has 16 physical cores, two SMT threads each. Compare mask0-15 with
mask0-31: 16 versus32 workers. This is not the C7a32/96 physical-core comparison.
Zopro workers retain fixed affinity; Aeglos retains its native pool and inherited
process mask, GOMAXPROCS equals native workers. No parallel proof requests.

First one functional request after three primes for all11arms at both counts;
22 processes,88SDKcalls. Require independent Gnark verification and complete
phase records before the timed matrix. Then nine blocks, all11arms at both
counts,51 requests after three primes per process.198processes10692SDKcalls.
Rotate engine and arm order deterministically by block; reverse order on odd
blocks and alternate worker-count order. No adaptive repeats. All sources,
inputs, sequence and readers pinned before capture. Only poll the live workflow.
Fresh exports220 total; canonical55220 ->55440 if all qualify. Only the final
request is exported by each process; all requests include SDK verification.

Primary observer gates use paired ratios of process medians across nine blocks:
fresh/retained calibration (Z OFF/REF; A FRESH/REF), A OFF/FRESH build effect,
and ON/OFF observer effect. Require the entire order-statistic interval from
sorted ratios[1] through[7] inside[0.95,1.05]. Nominal median coverage96.09375%,
assuming independent continuous block ratios; no simultaneous-coverage claim.
These are diagnostic-equivalence gates, not speedup/adoption thresholds. Report
individual failures. A failed gate forbids treating that trace as representative
of the corresponding uninstrumented retained implementation. No tuned rerun.

Record full-request latency, CPU, faults, context switches and whole-process RSS.
RSS is not incremental scratch or minimum RAM. Phase means exclude three primes.
Zopro query spans overlap and may include helping/waiting; don't add A/KH/G2 as
exclusive elapsed or CPU time. Report solver, prepare, FFT, complete query span,
output and residual; separately report query completion tails. Aeglos serial
phases include every merge round, and per-pass item counts conserve per-job sums.
Keep unassigned phase and full-request residuals explicit. Stage counts are
scheduled items, not equal-cost operations. Full SDK includes observer emission.

No CPU PMU sampling in this panel: bounded phase records are the profiling
instrument under qualification. Capture pre/post host state and CPU topology.
Report scaling in all nine blocks, not just the fastest block. AWS login was
checked before this panel and failed expired SSO; no AWS resources are created.
Full all-eight CPU10x and CPU/Metal goal remains unchanged,0/8.
