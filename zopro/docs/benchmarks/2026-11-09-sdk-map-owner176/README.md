# Accepted SDK large mapping ownership

Live debugger stacks identify the accepted merge54 prover’s two64MiB+4KiB mappings per request as constraint-row allocations in `solver::Plan::solve_in_with_slots`. The previous8KiB sampled unwinds stopped at the allocator. This audit makes no source change and establishes no latency improvement.

Two processes use the same accepted SDK at1/32workers, with3priming and1observed request each. Every process has8successful matching mmap calls and16entry/exit stops; every immediate Rust owner is the shared row-allocation closure. It does not identify A versus B individually. All4workflow commands complete on their first execution. Both exported randomized proofs pass independent Gnark verification and uniqueness checks against the previous40,160exports; canonical count40,162.

The solver reserves the padded domain for every row buffer. Source review shows that the Euler path returns actual constraint rows, packing its full domain into separate FFT scratch. A distinct Euler-only capacity change is now justified for testing; the coset path must retain its padded capacity. Requested capacity is not committed RSS or minimum RAM, and a smaller request can alter allocator behavior. No speed, pressure, mobile, AWS or Aeglos comparison follows from these debugger runs.

[Analysis and selected owner stacks](artifacts/analysis.json), [capture design](artifacts/design.md), [proof bindings](artifacts/proof-inventory.json), [tool provenance](artifacts/tools.json), [continuation](artifacts/next.md).

Accepted local full-proof latency is unchanged. These inherited results compare the32worker Ryzen9950X SDK against the supplied M5 CPU and Metal references; the10×CPU objective remains unmet for all8shapes.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10×target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

[Sources and command receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Debugger binaries/packages, proof payloads and the full hash ledger remain private.
