# Vectorized G2 signs: compatibility and resource checks

The exact measured candidate passes these additional compatibility and resource checks. Production remains unchanged; the candidate is not adopted. The [full SDK performance gate](../2026-10-30-g2-vector-sign-sdk175/README.md) remains **false** solely because the fresh control's calibration against the accepted build at 16 physical cores exceeded the declared range. The candidate's prior 32-worker primary result remains 7.887963 ms, a paired 1.92% improvement over the accepted build with 31/31 wins. This study adds no new latency comparison and changes no performance threshold.

The candidate's 176 source files match the measured snapshot byte for byte. Native core tests (124 control / 125 candidate), eight prover tests per variant, field tests (33 control / 35 candidate), and portable tests (101 core / eight prover) are inherited after exact source, receipt and flag checks. The unfiltered portable field suite passes 11 fresh tests. Portable baseline-x86 libraries and the standard SDK are built with IFMA disabled; native consumers link only the frozen accepted or measured candidate libraries.

- 13 portable processes cover all eight reference shapes plus 54→1, and primary/54→1 raw and preparation-denied paths.
- 24 native candidate processes cover 1/4 workers, 512 KiB/1 MiB configured stacks, primary/54→1 shapes and basis/raw/denied paths.
- Eight opt-out processes independently disable packed/direct/canonical-batch/vector-inverse paths for the accepted build and candidate.
- Four concurrent processes cover accepted/candidate × primary/54→1 with four actual SDK callers, two proofs each, one global 32-worker pool and a shared mapped key. Thread counts are checked: 37 loaded and 33 idle.
- Eight owned cgroup cases cover accepted/candidate×cold/warm×default/retained allocator settings, with swap disabled.

All 248 SDK requests verify internally; 89 fresh 164-byte exports independently pass Gnark with exact image/key/public bindings and uniqueness against 26,203 prior identities. The verified ledger advances to 26,292. All 178 proof/public members are retained. Raw loader checks use the matching Euler verifying image; public bindings remain unchanged.

## Memory observations

Limits derive from the accepted build's measured 134 MiB primary peak RSS: 1.25 times that value rounded upward to 64 MiB gives a 192 MiB maximum; 0.9 times rounded upward to 8 MiB gives a 128 MiB warm high limit. Warm cases prime at 512 MiB, touch 32 MiB of ballast, then lower the high/maximum limits to 128/192 MiB before two fresh proofs. Each case charges a private key-image copy within its owned service. These are test controls, not an architectural cap or minimum-RAM estimate. No larger-limit retry is permitted.

|Case|Cgroup peak MiB|High events|Max events|OOM/kill|Swap limit|
|---|---:|---:|---:|---|---:|
|cold-accepted-default|60.54|0|0|0/0|0|
|cold-accepted-retained|61.80|0|0|0/0|0|
|cold-on-default|61.32|0|0|0/0|0|
|cold-on-retained|62.50|0|0|0/0|0|
|warm-accepted-default|102.91|0|0|0/0|0|
|warm-accepted-retained|103.77|0|0|0/0|0|
|warm-on-default|104.82|0|0|0/0|0|
|warm-on-retained|102.96|0|0|0/0|0|

All cases exit normally with zero max/OOM/kill events. Actual high-event counts are reported above; zero high events establish operation under the configured limits, without establishing active-throttling behavior. Only uniquely prefixed owned units and their control directories are cleaned up. No global memory or kernel settings are changed. Peak includes the private image, controller, SDK and any ballast. Concurrent/functionality receipts are not throughput or latency qualifications.

## Supplied CPU and Metal references

These are the prior measured candidate's 32-worker full-SDK timings and supplied historical M5 references. All accepted local/AWS timing/CPU/RSS rows remain in [accepted rows](accepted-rows.json).

|Shape|Candidate ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.887963|37 / 4.691×|24 / 3.043×|3.7|
|2x2|11.157129|59 / 5.288×|30 / 2.689×|5.9|
|4x2|16.055974|109 / 6.789×|48 / 2.990×|10.9|
|8x2|24.206045|189 / 7.808×|74 / 3.057×|18.9|
|12x2|33.770306|284 / 8.410×|101 / 2.991×|28.4|
|16x2|40.063214|335 / 8.362×|124 / 3.095×|33.5|
|24x2|66.218548|528 / 7.974×|176 / 2.658×|52.8|
|merge-24x1|65.295067|507 / 7.765×|175 / 2.680×|50.7|
|merge-54x1|161.880641|—|—|—|

The original objective of reaching 10× the supplied M5 CPU performance on all eight shapes remains active and unmet (0/8). Primary requires 3.7 ms; 54→1 has no supplied M5 reference. This study adds no AWS/Aeglos, phone/ARM/TEE, large-circuit 250/4999, swap, minimum-RAM or concurrent-throughput result. The candidate adds no per-key lookup table, point copy or second execution plan. Existing full-SDK profiles are inherited; no profile is rerun for this source-identical functional study.

Passing these checks does not override the failed performance calibration or authorize automatic adoption. Generator-consistent/default-source integration, final-artifact qualification and a separate explicit engineering review remain. A possible handwritten field extension has not been integrated or measured here.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [source reuse](artifacts/source-reuse.json), [resource results](artifacts/resource-result.json), [qualification review](artifacts/qualification-review.json), [archive review](artifacts/post-publication-review.json), [review](artifacts/review.json), [source](artifacts/source.json.gz), [helpers](artifacts/helpers.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [checksums](SHA256SUMS).
