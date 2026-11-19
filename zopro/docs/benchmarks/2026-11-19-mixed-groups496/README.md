# Refining point groups with scattered and adjacent transfer witnesses

Eight public adjacent-leaf 12×2 training witnesses defined the original equality groups. Four public scattered-leaf witnesses (137–140) further split each group wherever the B2 scalar observations disagreed. Only subgroups of at least three members were retained and mapped to A and private K. The runtime validates every group it uses and accounts for scalar differences exactly. The executable, fixed wire offsets, sampling policy, and key are identical across configurations. A disabled control prepares no point groups. Training and evaluation requests are disjoint.

- B2 has 8,327 groups and 180,493 compact terms out of 270,757 original terms.
- A has 5,535 groups and 130,693 compact terms out of 190,734 original terms.
- K has 8,327 groups and 185,163 compact terms out of 275,427 original terms.

Four randomized blocks on the Ryzen 9950X used 32 logical workers and three primes followed by eleven timed verified SDK calls. The original and requests 57 and 64 have adjacent leaves. The other four have scattered leaves. Each process mean is summarized by its median across blocks. Paired changes and win counts compare matching processes. Peak RSS includes setup.

| Request | Off / adjacent plan / mixed plan, ms | Mixed vs off | Mixed vs adjacent | Peak MiB off / adjacent / mixed |
|---|---:|---:|---:|---:|
| Original | 31.507 / 30.068 / 30.904 | -1.00% (3/4) | +2.68% (0/4) | 577.3 / 641.0 / 726.8 |
| 57 | 31.574 / 29.821 / 30.890 | -2.21% (4/4) | +3.35% (0/4) | 570.5 / 637.5 / 709.4 |
| 64 | 31.168 / 30.206 / 30.771 | -1.37% (3/4) | +1.79% (1/4) | 578.8 / 611.8 / 690.6 |
| 141 | 47.652 / 47.221 / 47.396 | +0.15% (2/4) | -0.09% (2/4) | 1033.3 / 1003.9 / 1231.6 |
| 142 | 47.169 / 47.926 / 47.230 | -0.38% (3/4) | -1.45% (4/4) | 955.8 / 1000.0 / 1142.2 |
| 143 | 47.200 / 47.627 / 46.547 | -1.39% (4/4) | -2.33% (4/4) | 978.3 / 976.8 / 1112.2 |
| 144 | 47.161 / 47.532 / 46.227 | -2.28% (4/4) | -2.98% (4/4) | 965.9 / 996.3 / 1129.1 |

The mixed plan exercised all three compact queries on the four scattered held-out requests. Its median paired change across those requests was -0.89% versus disabled. The original request took 30.904 ms with the mixed plan and 30.068 ms with the adjacent plan. The mixed plan also had the highest process peak on every measured request.

All 94 exported proofs passed independent Gnark verification, including the four training exports and six route probes. [Measurements](artifacts/processes.json.gz) · [Compact query execution and fallbacks](artifacts/probes.json) · [Training fixture identities](artifacts/training.json) · [Plan dimensions](artifacts/layouts.json) · [Verification](artifacts/verification.json)
