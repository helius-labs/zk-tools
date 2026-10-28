# Fresh control backing and frontend counters

This diagnostic compares original control backing with two new byte-exact files from each source. All six executable names have equal lengths in one directory; mode0,seed17,CPU2,arguments and shared FIFO environment values are identical. Original aliases retain parent inodes; fresh files have new inodes and exact source hashes. Original sources and executables are unchanged.

Three fixed rotated/reversed blocks cover six files and two grouped panels:36processes. Each has3checkedprimes+7counted completeMSMs at30,089points. All outputs agree with the generator oracle and each other. Capability-only true commands demonstrate both panels can count; their logs are retained. Cache event names omit:u in perf output despite explicit userland requests, so labels are normalized.

| Image/panel | ms/MSM | CPU ms/MSM | Cycles/MSM | Instructions/MSM | L1I misses/MSM | iTLB misses/MSM | Frontend stalled cycles/MSM | L1D misses/MSM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| copy-001-f/fetch | 41.844119 | 41.843 | 227572794.143 | 574561684.286 | 397037.143 | 0.143 | — | — |
| copy-001-f/stall | 41.855138 | 41.855 | 227604013.286 | 574561966.857 | — | — | 10520048.571 | 10791737.571 |
| copy-001-o/fetch | 41.766923 | 41.765 | 227284042.000 | 574562060.714 | 427038.714 | 0.286 | — | — |
| copy-001-o/stall | 41.763664 | 41.762 | 227277054.143 | 574561863.000 | — | — | 10648740.286 | 10784664.143 |
| copy-002-f/fetch | 42.199319 | 42.198 | 229532122.000 | 574562303.286 | 290442.286 | 0.286 | — | — |
| copy-002-f/stall | 42.143751 | 42.142 | 229596786.429 | 574561987.571 | — | — | 10746537.857 | 10782126.714 |
| copy-002-o/fetch | 41.880474 | 41.880 | 227669389.000 | 574561554.429 | 303743.429 | 0.143 | — | — |
| copy-002-o/stall | 41.790496 | 41.789 | 227482419.714 | 574562454.000 | — | — | 10173378.571 | 10763401.143 |
| original-f/fetch | 42.058301 | 42.057 | 228735580.857 | 574561969.857 | 502995.000 | 0.429 | — | — |
| original-f/stall | 42.004372 | 42.003 | 228625542.714 | 574562401.000 | — | — | 11267746.857 | 10756893.143 |
| original-o/fetch | 61.482439 | 61.482 | 336752264.857 | 574562171.000 | 15325977.143 | 0.000 | — | — |
| original-o/stall | 61.432621 | 61.431 | 336407181.571 | 574561991.714 | — | — | 115681395.286 | 10699928.714 |

All groups100%running: **true**. All3observations are retained. These descriptive panels do not supply confidence/adoption gates or isolate a unique microarchitectural cause. Different inodes do not prove independent physical placement. Generic cache/stall events are not DRAM bandwidth, exclusive instruction costs or removable proof latency.

There are252countedMSMs+108primes, no new native builds, SDK/Gnark/proofs or optimization acceptance measurement. No ASLR/globalkernel/cache settings change. All pinned sources/images/compiler/helper inputs are rechecked after capture. No owned build/helper/hash/archive/otherbenchmark overlaps counters; host snapshots do not prove exclusive use. Goal remainsACTIVE0/8, proofledger13,181, all172production/SDK14da unchanged.

The inherited accepted all-eight CPU/Metal figures remain in the [borrowed-queue report](../2026-10-28-g2-borrowed-queue/README.md):primary8.324853ms versus37ms M5CPU/24ms M5Metal,4.445×/2.883×. Target3.7ms remains unmet. Primaryhistoricalpeak134.50MiB; merge54 161.341835ms/1910.1875MiB without a suppliedM5reference. No new full-proof speed, mobile/TEE/minRAM/pressure/giant/AWS/Aeglos claim. The parent borrowed-queue gate remains failed and its positivephysical16/memory observations retained.

[Design](artifacts/design.md), [file identities](artifacts/identities.json), [all observations](artifacts/analysis.json), [workflow](artifacts/runs/result.json), [prior identity diagnostic](../2026-10-28-g2-control-identity/README.md). Captured logs retain bytes including trailing blank lines; authored source/report whitespace checked separately.

## What changed with fresh files

Both fresh copies of OFF's complete ELF run in41.77–41.88ms/MSM in the fetch panel, while the same-inode original alias remains61.48ms. The fresh frozen copies run41.84–42.20ms and its original alias42.06ms. Stall-panel times agree closely. All six files retire approximately574.56million instructions perMSM. Original OFF therefore remains slow after relabeling/hardlinking, but its exact bytes do not require that slow execution rate when written to fresh files.

Original OFF records15.326million L1instruction-cache misses/MSM, versus0.304–0.427million for its fresh copies. Its frontend-stalled event is115.681million cycles/MSM, versus10.173–10.649million for fresh copies. Total cycles fall from336.75million to227.28–227.67million in the fetch observations. L1data misses remain around10.7–10.8million and iTLB misses near zero across files. Event definitions and nonexclusive sampling/aggregation limits still apply; these counters are not an additive causal decomposition.

The intervention supports a backing-associated instruction-fetch/frontend effect in this workload. It rules out ELF contents or output label alone as a sufficient explanation of the earlier45–48% control gap under these conditions. It does not identify a physical page, cachecolor, virtual alias or kernel mechanism: physical placement was not measured. Fresh writes may affect mapping/backing state, and four fresh files on one host do not guarantee that every future copy will run at the same rate. Equal-length names and shared FIFO environment remove the earlier name-length confound without changing the program's arithmetic.

This is a benchmark-comparability finding, not a prover breakthrough or a correction of every historical result. The borrowed-queue component's original failed gate, all positive estimates and separate1.35MiB requested-memory saving remain intact. In particular, do not reinterpret its37.7% fresh-control gain as an isolated algorithm benefit; the original-frozen comparison was6.81% at16workers and uncertain at32workers.

The next distinct measurement should use the actual accepted SDK, original and prospectively fixed fresh copies, with real verified proofs and CPU/Metal comparisons across all supplied shapes. Keep every copy and outcome rather than selecting a favorable file. This will establish whether the observed backing sensitivity matters to real proof latency before choosing a new optimization or qualification protocol. No source or proof-time improvement is claimed by this diagnostic.
