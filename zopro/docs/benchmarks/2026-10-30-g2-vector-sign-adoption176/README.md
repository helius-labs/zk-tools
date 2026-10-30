# Default G2 vector-sign adoption

Adopt the measured signed canonical G2 coordinate difference under the existing native guards. Primary at 32 workers measures **7.928660 ms versus 8.041215 ms**, a paired **1.54% improvement** with 30/31 wins. Peak RSS is **132.75 versus 135 MiB**. The all-eight equal-log paired improvement is 0.377%; memory observations are mixed across shapes.

This is an explicit engineering exception to the original private-candidate 16-core fresh-control calibration failure. That full gate remains **false**. The distinct, prospective final-artifact gate passes every check, and the exact default artifact passes the functional/resource qualification. The interval against the private implementation includes parity; no improvement over that implementation is established. Earlier adoption exceptions remain in the previous manifest.

The production source is byte-exact to the tested 176-file snapshot: three existing files change and one handwritten field module is added. Generated arithmetic and generator scripts stay unchanged. The experiment-only flag is removed. Arithmetic, strict normalized output below 2p, negative-zero correction, native guards and fallback behavior are preserved. No heap allocation, per-key table, persistent point copy, second execution plan or pool is added. Accepted native flags and runtime settings remain unchanged.

Fresh actual-artifact suites pass: native field/core/prover 35/125/8; baseline-x86 with IFMA disabled 11/101/8. The same default artifact passes 45 functional processes, four concurrent processes and eight configured-memory cases. Across performance and compatibility qualification, 56,036 SDK requests verify internally and 1,061 fresh exports independently pass Gnark. The verified ledger is 27,353. Promotion adds no test, proof, profile or timing capture; all 176 production source hashes match the qualified artifact.

Memory cases use 32 workers, swap disabled, a 192 MiB maximum and a 128 MiB warm high limit derived from the accepted 135 MiB peak RSS. All eight exit normally with zero high/max/OOM events. Candidate cgroup peaks are 61.62–62.96 MiB cold and 104.49–104.59 MiB warm, including charged private image/controller/SDK/ballast. Shared mappings can be charged elsewhere. These are not standalone or minimum-RAM estimates, and zero high events do not establish active throttling. No new AWS/Aeglos, phone/ARM/TEE, huge250/4999, swap or concurrent-throughput result is claimed. Separate CLI build is unverified; the standard SDK engine is qualified.

## Supplied M5 comparisons

|Shape|Adopted ms|Peak RSS MiB|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|7.928660|132.75|37 / 4.667×|24 / 3.027×|3.7|
|2x2|11.151932|275.00|59 / 5.291×|30 / 2.690×|5.9|
|4x2|16.086386|312.25|109 / 6.776×|48 / 2.984×|10.9|
|8x2|24.179453|517.25|189 / 7.817×|74 / 3.060×|18.9|
|12x2|33.884455|706.50|284 / 8.381×|101 / 2.981×|28.4|
|16x2|40.077381|842.00|335 / 8.359×|124 / 3.094×|33.5|
|24x2|65.850777|1300.50|528 / 8.018×|176 / 2.673×|52.8|
|merge-24x1|65.060937|1168.19|507 / 7.793×|175 / 2.690×|50.7|
|merge-54x1|161.883899|1876.96|—|—|—|

The original all-eight CPU 10× objective remains **active and unmet, 0/8**. Primary requires 3.7 ms. No M5 reference was supplied for 54→1. Historical AWS rows describe the earlier source and are not measurements of this adoption.

[Performance](../2026-10-30-g2-vector-sign-default175/README.md), [functional/resources](../2026-10-30-g2-vector-sign-default-compat175/README.md), [decision](artifacts/decision.json), [production manifest](artifacts/production-source-manifest.json), [post-adoption review](artifacts/post-adoption-review.json), [local rows](artifacts/local-rows.json), [historical accepted rows](historical-accepted-rows.json).
