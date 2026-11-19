# Proof latency at fixed core count across two L3 cache domains

Each engine used eight physical Ryzen 9950X cores with one thread per core. Compact placement selected one entire eight-core L3 domain. Spread placement selected four cores from each of the two domains. Over four blocks, the masks in each policy cover all sixteen physical cores equally often. Within each engine and shape, both placement order and plain versus instrumented order alternate. Each process primed three proofs, measured nine and exported one independently verified proof against the original key. All 96 exports verified. The hardware exposes one NUMA node. Spreading changes available L3 capacity, communication distance and power distribution together.

The measured executables are the ordinary reference from the staged AWS pilot and the prepared Aeglos binary. All keys, requests, native libraries and binaries are pinned in the configuration. Latency and CPU time come from the plain processes. Linux user-mode cycles and retired instructions come from separate processes with hardware counters enabled around the warm loop by each SDK. At least 98% counter running time was required for both simultaneous events. The paired counter effect column compares those instrumented processes with their corresponding plain controls. Aeglos records a native cache hit on each timed request. Zopro also reports the pinned CPU of each main worker.

| Shape | Engine | Compact / spread ms | Spread paired change (wins) | Compact / spread CPU-ms | Retired instructions M/proof | Instructions per cycle | Counter effect compact / spread | Peak MiB compact / spread |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1x16 | zopro | 22.616 / 22.890 | +1.36% (2/4) | 165.2 / 166.7 | 2010.5 / 2012.1 | 2.457 / 2.407 | +0.6% / +0.6% | 131.5 / 135.0 |
| 1x16 | aeglos | 34.740 / 36.104 | +6.20% (0/4) | 275.4 / 277.6 | 3042.1 / 3042.6 | 2.197 / 2.154 | +1.0% / -1.7% | 781.1 / 767.9 |
| 5x16 | zopro | 40.824 / 41.329 | +2.14% (1/4) | 306.6 / 309.3 | 3400.4 / 3402.5 | 2.225 / 2.175 | +0.7% / +0.4% | 269.0 / 270.0 |
| 5x16 | aeglos | 118.135 / 105.679 | -11.00% (4/4) | 938.4 / 804.6 | 8183.4 / 8192.4 | 1.715 / 1.996 | -0.3% / -0.7% | 1920.1 / 1905.6 |
| 6x2 | zopro | 35.173 / 35.661 | +1.59% (1/4) | 263.0 / 266.9 | 2883.8 / 2885.4 | 2.197 / 2.148 | +0.3% / +0.1% | 245.5 / 238.0 |
| 6x2 | aeglos | 115.555 / 102.299 | -11.49% (3/4) | 917.3 / 784.3 | 8168.8 / 8175.5 | 1.748 / 2.019 | -0.2% / -1.7% | 1888.6 / 1879.8 |

Aeglos on 5×16 and 6×2 gained 11.00% and 11.49% in paired latency from spreading the same eight cores across both cache domains. Retired instructions per proof varied by less than 0.1% between those placements on both keys. CPU-ms fell and instructions per cycle increased. On 1×16, Aeglos instead took 6.20% longer when spread and lost all four pairs. Zopro had higher paired latency when spread on all three shapes. These results favor testing a locality-aware worker policy for Zopro and show why giving both engines the same thread count alone does not characterize the hardware comparison.

Both domains reside in the same NUMA node. This intervention cannot separate the extra L3 capacity, inter-domain communication, boost clocks or memory-channel behavior. The 96-physical-core C7a measurements require a separate same-host test.

[Topology and masks](artifacts/topology.json) · [Exact xtask workflow](artifacts/workflow.json.gz) · [Input pins](artifacts/pins.json.gz) · [Per-process samples and counters](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
