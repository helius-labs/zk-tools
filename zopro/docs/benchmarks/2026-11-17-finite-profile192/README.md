# Exact finite-base SDK profile

The exact191SDK passes all four observer checks and all twelve PMU quality checks.
ON executes about0.20–0.22% fewer instructions per proof than OFF,consistent with
removing raw finite-feed checks. Cycles and latency remain mixed;this profile does
not establish a robust speedup or replace full qualification.

Three balanced blocks reuse all three191candidate backings. Primary1x2 runs16
fixed workers;plain/stat/cycles panels each execute251 fresh verified requests
after3primes. FIFOcapture excludes loading/certification/primes/exports. Counters
run100%;all six cycle captures have>=1000samples,unknown<=5%,zeroLOST andverified
MMAP/PID/TID/workerCPUownership. Observer medians OFFstat/cycles are1.005140/
1.004329;ON1.007516/1.009683,within the predeclared factor1.05 guard. This is coarse
calibration,not equivalence or proof of unbiased attribution.

OFF instructions/query are1280.8–1281.1million,ON1277.9–1278.5million. OFF cycles
are537.0–538.3million,ON537.2–539.0million. Allplain blocks and raw measurements
are retained;no unfavorable panel was discarded. Symbol shares are not exclusive
or removable proof time. The screen's small positive median remains a candidate,
with no arbitrary minimum gain threshold added here.

All18fresh exports independently Gnark-verify,4572SDKcalls includingprimes.
EveryONtimed request records known/raw=1 andgrouped=0;OFFallzero. The exact SDK
`b655c4c68483d54e2343a0593818b49be05e140c949fe83cd6216d332ecb1897`
is unchanged. Canonical56668->56686,SHA256
`ad8832cbd7619e0ccd682da64e348fda1b1e849fdaed370382ff78d112384d69`.

Next finish all-shape performance,portable,concurrent and resource checks against
accepted production on this retained build. No production adoption,AWSresource
or publicpush. Fullall-eight CPU10xgoal remains unmet. Private root:
`/home/ubuntu/src/zopro-perf-work/finite-profile192`.
