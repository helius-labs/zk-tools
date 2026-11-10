# Complete-query screen: 62-bit approximate GCD

ON replaces the field inversion's positive 31-bit approximate blocks and paired
composition with positive 62-bit blocks over u128 approximations. All point
formulas, field products, grouping, queues, scratch and pool policies stay fixed.
The change affects G1 and G2, including A queries; A is NOT an unchanged control.
REF is the accepted-source complete-query executable, OFF is freshly built from
the reversible snapshot with the selector absent, ON enables the new inversion.

Eight real contexts: 1x2-B and 2x2-B each at 1/16/32 workers, plus 1x2-A32 and
2x2-A32. Existing Rayon pool, 512 KiB requested stacks, CPU2 for solo and CPUi
for worker i otherwise. Timed work includes scalar reset, grouping/admission,
recoding/windows/merges/weighting, allocations/drop and affine normalization.
Loading, pool construction, three primes, postcheck and oracle comparisons are
outside timing. Every timed output must match its retained independent Arkworks
full-query oracle; input points must remain unchanged.

Fixed 31 rotated/reversed matched blocks, 51 queries/process. Per-process median
51; paired ratios across blocks; ranks9/15/21 for lower/median/upper. Conditional
sign coverage assumes independent exchangeable blocks, without familywise
coverage. No minimum positive gain floor or optional sample extension.

52 checks: eight OFF/REF calibration intervals in[1/1.03,1.03]; sixteen ON/control
nonregression upper bounds<=1.02; primary B32 and equal-log two-B-shape32 gains
against BOTH controls, upper<1 and>=21/31wins; eight ON maxRSS guards at
max(controls)*1.05+1MiB; eight PMU and eight profile quality checks. There is no
unchanged-A check because the inversion changes A too. RSS is an experimental
comparison guard, not an architectural cap or minimum-RAM claim.

24 preflights, 744 latency processes, then eight PMU panels and eight profiles
with400queries each. Events: grouped cycles/instructions/cache-misses/branch-
misses:u with>=99.9%running. Profiles: fixed16Mcycles,512pages, CPU/time/period,
PID/TID/executableMMAP,>=1000samples,<=5%unknown,zeroLOST. All40offline decode
steps follow every recording. Total824commands/784callers/47504checked queries
including primes and postchecks. Raw counts and all decisions frozen beforehand.

During ANY preflight/recording ONLY poll its confirmed live handle until terminal.
No overlapping helper/build/hash/decode/analyze/report/push. Retain all outcomes,
failures and uncertain positives; no posthoc worker/fixture/threshold/width rescue.
Passing selects ALL8+54 SDK/Gnark/resource/portable/concurrency qualification,
not adoption. Goal remains verified CPUAVX >=10x supplied M5CPU on ALL8, with
CPU and Metal comparisons; canonical43730 and accepted SDK remain unchanged.
