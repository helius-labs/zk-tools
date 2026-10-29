# Partition eligible G2 queries by component instead of point range

Declared before builds/capture, 29 October 2026. Previous goal turn is progress:
simultaneous counters established sampling-period undercoverage. Do not rerun
that diagnostic or interpret its period sums as work counts. Accepted source175
and SDK20fde stay unchanged; all8 CPU10x targets remain active/unmet.

Current eligible phi2 G2 queries have eleven 12-bit windows and two point-range
partitions. Each partition sequentially feeds high component, transforms its
buckets, then feeds low. This candidate instead runs high/low components over
the full borrowed range as the two tasks of a window, transforms only high's
finished affine buckets, merges the two sets and uses original weighted/Horner
recombination. Total ordinary bucket sets stays two/window. No transformed-point
cache, extra decomposition dimensions, new field formula, shifted bases, point
array copy, new pool, digit array or persistent plan. Different input ordering,
collision behavior, exceptions and scheduling remain real costs, not assumed
gains. Full-range narrow-top routing can hold more indices; native workspace
forecast conservatively adds both complete full-range top-parent envelopes and
their leaf maximum, retaining the old forecast as a floor. Admission/fallback
remain existing checked resource policy, not a synthetic global RAM cap.

Private native cfg zopro_phi_component_partition; original eligible range
1024<=N<65536, scalar split, fixed12-bit width and two-set count unchanged.
K/H,A,FFT,solver,field and Rayon code exact. OFF retains production path. Tests
cover independent known-generator complete queries, per-component buckets,
compiled dispatch and conservative top envelope. Run complete OFF/ON native
core/prover suites; inherit unchanged field qualification. Freeze coherent
libraries immediately after each build and link original SDK consumer.

Primary32 and primary16 physical0-15 pilot:15 fixed rotated/reversed blocks,
accepted/freshOFF/ON,151 requests+3primes each. All9 shapes x3variants plus
primary16 x3variants get1+3 preflight with independent Gnark before latency.
All90 latency processes precede6 four-event stat captures31+3 and6 fixed-period
cycles:u profiles251+3 at16/32workers x3variants. Profiles use period16M, explicit
CPU/TIME/PERIOD and FIFO only request loop, zeroLost/affinity/SDKMMAP checks;
their known accounting limitations remain qualitative only. Stat events require
>=99.9%runningcoverage. All12diagnostics precede their decoding/verifiers and
all remaining90latency verifiers. No owned compiler/hash/archive/analysis or
second CPU workload overlaps captures. All helpers including publisher and
analyzer compile BEFORE captures. No successful rerun or conditional extension.

Gate: BOTH primary32 and primary16 ON/accepted and ON/OFF rank4..12 ratio
interval upper<1 and>=10/15wins. OFF/accepted intervals for both within
[1/1.03,1.03]. Observed lifetime maximum RSS ON/control<=1.05 for both controls
and both cases. Conditional nominal96.484375%individual median coverage under
exchangeable independent blocks; no familywise guarantee. No gain floor,
exclusion, gate waiver or subset adoption. A passing pilot needs separate
all9 performance/resource/portable qualification; it is not goal completion.

30preflight+90latency+12diagnostic exports=132 fresh164B distinct Gnark proofs,
ledger22234->22366. Completed SDK requests120+13860+204+1524=15708. Exact fixture
public/image identities. Preserve all raw observations and count failed work
separately. Publish/commit/push positive/negative. No AWS/mobile/TEE/large-circuit,
minimum-RAM/pressure/swap/concurrent-throughput claims. CPU/Metal original8
comparisons remain visible with accepted/adoption exceptions unchanged.
