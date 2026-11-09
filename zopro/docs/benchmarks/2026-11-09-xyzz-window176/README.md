# Sparse XYZZ Windows: integration qualification

**The isolated G1/G2 sparse Window implementation passes native and generic qualification. It has not been timed or adopted.** Production, the accepted SDK and recorded proof latency remain unchanged.

The previously qualified XYZZ helper now replaces only sparse residual accumulators in borrowed G1 ordinary/fused/balanced owners and the G2 Phi owner. Ordinary affine batches, queue order, window geometry and weighted reduction stay unchanged. A static `SparsePoint` type selects the implementation once per operation. G2 retains its specialized base-field Phi multiplication on X; cached Z²/Z³ do not change. There is no additional populated key/point array, table, pool or assembly.

The operation owner uses the existing process-wide resource policy before constructing changed Windows and retains the reservation through all parallel work, high/Phi/low feeds and normalization. Checked extra charges cover both added-coordinate capacities, a complete previous allocation during vector growth, bookkeeping and all possible windows/parts. Denial, overflow, disabled configuration and generic execution retain the original Jacobian path. The original required-workspace forecast is unchanged.

|Memory quantity|G1|G2|
|---|---:|---:|
|Original sparse point payload|96 bytes|192 bytes|
|XYZZ sparse point payload|128 bytes|256 bytes|
|Conservative optional reservation for tested owner geometry|15,733,888 bytes /20 balanced windows|17,307,264 bytes /22 Phi parts|

Reservation bounds are not allocated bytes, process RSS, minimum RAM or whole-proof memory increases. This stage adds no measured candidate memory peak. The mixed-add arithmetic may improve while doubling costs more; complete caller timing must include both. This small residual path is not a standalone twofold primary improvement.

Twelve new focused test functions cover actual queues and complete owners. Independent Arkworks points and carry-based recoding check 66 Window cases per curve with both Jacobian and XYZZ; generic execution repeats the same G1 fixtures. Cases span zero/short inputs, batch boundaries, full-width scalars, signed tails, identity/equal/opposite points, doubling/cancellation, two feeds, reuse, final affine merge and normalization. Each explicit queue fixture/type/curve records 589 queued operands, 17 flushes and 14 packed batches; G2 also maps Phi between reused rounds. Complete owner checks include 20 G1 configurations and 4 G2 scalar patterns, admitted and denied.

Real 1/4/16/32-worker pools submit 53 Window tasks per curve with 512 KiB stacks. Four OS callers share eight workers, including eight complete G1 operations and four complete G2 Phi operations. The existing reservation token is checked for denial, drop/readmission, panic unwinding and overflow. Nine invalid fused inputs per backend preserve original width12/13/14 preconditions. Generic x86 checks do not establish physical ARM or phone behavior.

A separate same-agent Rust reader validates all 176 unchanged production hashes, 171 exact private originals, five reversibly changed parents, the exact qualified XYZZ helper, source/binary bindings and 28 command receipts: 26 successes and two retained failures. A source-copy preparation failure is also documented. The failures involved an unsupported Arkworks test-oracle operator and a missing inherited G2 runtime flag. Source review then found and repaired missing width13 fused input assertions in the new dispatcher; changed native/generic owners and their preconditions were rebuilt and rechecked. Successful unaffected checks were retained. See the [preparation record](artifacts/preparation-note.md). Unused-import warnings remain without warning-only rebuilds.

Next is a prospectively frozen complete caller comparison with accepted-source REF, fresh OFF and ON, including allocation/destruction, admission, doubling, ordinary batches, Phi, merge, normalization, actual RSS and automatic CPU profiles. Only a useful complete caller result would justify a full SDK/all-eight-plus-merge54 proof campaign. There is no performance selection or proof export in this qualification stage; canonical verified inventory stays 43,730.

The following timings are inherited from the latest accepted local 32-worker full-proof campaign. M5 CPU and Metal references were supplied on different hardware and were not remeasured. The full CPU10× objective remains active and unmet, **0/8**.

|Shape|Accepted CPU ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37.0 /4.726×|24.0 /3.065×|3.7|
|2x2|10.979335|59.0 /5.374×|30.0 /2.732×|5.9|
|4x2|15.803969|109.0 /6.897×|48.0 /3.037×|10.9|
|8x2|23.944017|189.0 /7.893×|74.0 /3.091×|18.9|
|12x2|33.703579|284.0 /8.426×|101.0 /2.997×|28.4|
|16x2|39.858478|335.0 /8.405×|124.0 /3.111×|33.5|
|24x2|66.186709|528.0 /7.977×|176.0 /2.659×|52.8|
|merge-24x1|65.489902|507.0 /7.742×|175.0 /2.672×|50.7|

Accepted primary RSS remains 133.50 MiB, and merge54 remains 162.541995 ms without a supplied M5 reference. Earlier 7.803404 ms and the latest 7.829286 ms came from the same accepted SDK executable; no slower replacement was adopted. No new AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap, minimum-RAM or throughput evidence is added.

[Review](artifacts/review-result.json), [source audit](artifacts/source-audit.json), [design](artifacts/design.md), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private root: /home/ubuntu/src/zopro-perf-work/xyzz-window176.
