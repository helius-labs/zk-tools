# Bound parallel work inside expanded FFT leaves

The accepted CPU source now uses the exact tested bounded FFT leaves. The original-eight paired aggregate improves **0.856% against accepted and 0.419% against fresh**, with intervals below parity. Primary latency is **8.324853 ms**; a reliable primary improvement is unproven. Merge 54 measures **161.341835 ms**, saving a paired 2.142 / 2.572 ms against accepted / fresh.

This separate engineering decision accepts primary uncertainty and the 8×2 candidate/fresh nonregression interval [0.983854, 1.024137], whose upper bound exceeds the frozen 1.02 requirement. All calibration checks and all other nonregression bounds pass. The [original performance gate](../2026-10-27-clean-fft-leaves/README.md) remains **false**. No observation is excluded, rerun or relabeled. See the [decision](artifacts/decision.md).

One source file changes and one private module is added. All 172 production source files exactly match the measured candidate. Existing leaf storage, field kernels and outer transform geometry remain; no new table, buffer, solver plan, pool or runtime option is added. Native arithmetic is retained without rebuilding or changing tested bytes.

[Resource qualification](../2026-10-27-clean-fft-qualification/README.md) passes portable, fallback, small-stack, concurrent and memory cases. Forester 4999 completes under 95 GiB with swap disabled at 160.741 / 149.120 seconds and 94.692 GiB peak RSS, with reclaim and no OOM/kill. These are capacity observations, not minimum RAM or matched speed comparisons. RSS across shapes remains mixed.

The following values reuse the candidate column of the completed fifteen-block local study. M5 CPU and Metal figures are supplied historical references; no 54 reference was supplied.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU 10×|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.324853|37|4.445×|24|2.883×|3.7|
|2x2|11.460675|59|5.148×|30|2.618×|5.9|
|4x2|15.854365|109|6.875×|48|3.028×|10.9|
|8x2|24.084626|189|7.847×|74|3.072×|18.9|
|12x2|33.693397|284|8.429×|101|2.998×|28.4|
|16x2|39.981636|335|8.379×|124|3.101×|33.5|
|24x2|66.010670|528|7.999×|176|2.666×|52.8|
|merge-24x1|65.086624|507|7.790×|175|2.689×|50.7|
|54→1|161.341835|—|—|—|—|—|

Exact SDK: `/tmp/zopro-clean-fft-leaves/native/candidate-sdk`, SHA-256 `14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a`. The [manifest](artifacts/production-source-manifest.json) records adoption/resource review as passed and full/qualified-scope performance gates as false. The latest C7a comparison belongs to the previous 171-file source; no new AWS or Aeglos result is claimed.

Promotion adds no benchmark, build or proof export. The proof ledger stays **11,660**. Physical phones, ARM, TEE and universal/minimum memory remain unqualified. The original all-eight CPU 10× goal remains **active and unmet, 0/8**.

[Source audit](artifacts/source-promotion.json), [review](artifacts/review.json), [artifact hashes](artifact-index.json).
