# Final artifact result and engineering review

The exact clean candidate confirms a useful primary32 improvement:8.366931ms
versus accepted8.563320 and byte-identical-runtime rebuiltcontrol8.526204.
Paired candidate/accepted median0.9786269598351882, bounds
[0.9724897117061744,0.9812218078085982],15/15wins. Versus rebuiltcontrol:
0.9830914420157155,[0.9724010318213789,0.987757656436995],15/15wins.
These are the final artifact's timings; the earlier8.226135 experimental timing
is retained in its original cohort and is not substituted here.

Final artifact gate remains FALSE, with26of27default32 checks passing. The only
failure is16x2 candidate/accepted upper1.025268463655265>1.02. Its paired
median is1.0000167976106455,[0.9837754299646532,1.025268463655265],7wins.
Against rebuiltcontrol it is0.9963636692024107,
[0.9732867316070689,1.0011493963351172],11wins. All nine calibration checks
and every other declared nonregression/gain check pass. The failed bound is
uncertainty about a small shape effect, not evidence of a2.53% causal slowdown;
its converse also does not prove no regression. All observations remain.

The descriptive primary16 estimate is slightly slower:11.031550ms versus
accepted10.935407; paired1.0062775416089293,
[0.9267755199303257,1.0595788830678912],7wins. This is not a confirmed16-worker
improvement. The original experimental screen's two failed16 bounds also
remain FALSE. No extra block, executable, topology selector or fallback is
introduced to make either screen pass.

Engineering decision: adopt the exact clean source/artifact for its independently
confirmed primary32 gain and lower requested-allocation traffic, while explicitly
accepting the16x2 uncertainty and unresolved physical16 behavior. This is a
reviewed exception, not a passing performance gate. The architecture adds no
persistent table, extra plan, pool or arithmetic rewrite, and retains the original
conservative forecasts. Source/native tests and earlier exact-module fallback,
small-stack, concurrent and bounded-resource evidence support the change within
their recorded scopes. The accepted-only forced-high timeout remains a resource
limitation; candidate under that condition is untested. No new pressure retry.

RSS is mixed. Primary candidate median/max131.25/135.00MiB versus accepted
130.25/132.75MiB; merge54 candidate1802.20/1885.21MiB versus accepted
1804.90/1858.91MiB. Earlier40fewer allocation requests/1472000fewer requested
bytes concern traffic and do not establish a peak-RAM reduction or minimum RAM.
No new giant/phone/ARM/TEE/swap/AWS qualification is claimed.

All954workflow steps completed exactly once:477SDKcaptures and477independent
Gnark verifications.26208complete requests verified internally;954proof/public
members,477distinct fresh164-byte exports, ledger14551->15028. Source/library/
executable/input pins pass after capture. No owned build/helper/hash/archive/
other benchmark overlapped captures. No exclusive-host claim. Session47937 is
terminal/reaped exit0; finish succeeded without repeating captures/verifications.

The complete all-eight CPU/Metal table is in analysis/report. Original tenfold
CPU goal remains active and unmet0/8; primary still needs3.7ms. New AWS comparison
is absent. This adoption is a concrete incremental improvement, not a breakthrough
or goal completion. A source-promotion receipt and current accepted manifest must
be created from these exact bytes before claiming production has changed.
