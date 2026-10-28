# Borrowed G2 pending queue in SDK proofs

The candidate **fails the declared SDK screen and is not adopted**. Primary32 is **8.226135 ms**, ON/accepted paired median **0.978700145**, bounds[0.973916815,0.984719850], 15/15wins. ON/OFF is **0.982414773**, bounds[0.980352054,0.985605957], 14/15wins. All positive estimates and failed checks are retained. Production remains unchanged.

The exact private candidate from the [complete G2 MSM study](../2026-10-28-g2-borrowed-queue/README.md) is linked into the real SDK for the first time. Signed 16-byte references replace 136-byte copied G2 points in proof-local batch/collision queues. Signs are materialized at bucket insertion/projective fallback or Y packing; two unused scalar buffers are omitted. Field arithmetic, phi split, bucket representation, width/parts, immutable key, top windows, exception handling and unsupported-path fallback are unchanged. There is no persistent point/key table, recoded-key copy or expanded execution plan. The component's positive16-worker estimate, measured extra-live reduction and failed gate are retained within their original scopes.

## Complete verified request latency

Ryzen9950X,15 fixed rotated/reversed process blocks, accepted/frozen/freshOFF/ON at32 workers for all eight supplied shapes and merge54, plus primary16 physical workers. Each process primes3 times, then151 primary or31 other requests. Witness solving, fresh randomized proving, SDK verification and serialization are timed; loading is excluded. CPU includes coordination; RSS includes load/primes. These are warm single-request times, not parallel-proof throughput. Values are medians of block medians; every block is retained.

| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 1x2-w32 | accepted | 8.379021 | 191.661 | 130.50 | 134.00 |
| 1x2-w32 | frozen | 8.392573 | 190.912 | 129.50 | 136.75 |
| 1x2-w32 | off | 8.356026 | 190.647 | 130.50 | 137.75 |
| 1x2-w32 | on | 8.226135 | 187.395 | 130.75 | 134.50 |
| 2x2-w32 | accepted | 11.502104 | 265.738 | 256.50 | 279.50 |
| 2x2-w32 | frozen | 11.525586 | 265.325 | 256.50 | 264.75 |
| 2x2-w32 | off | 11.526017 | 264.197 | 261.00 | 272.75 |
| 2x2-w32 | on | 11.434172 | 262.165 | 262.75 | 274.75 |
| 4x2-w32 | accepted | 16.233889 | 420.974 | 299.25 | 312.75 |
| 4x2-w32 | frozen | 16.248096 | 424.441 | 292.50 | 308.75 |
| 4x2-w32 | off | 16.184640 | 418.622 | 304.25 | 326.00 |
| 4x2-w32 | on | 16.228639 | 420.764 | 297.00 | 314.25 |
| 8x2-w32 | accepted | 24.535797 | 669.547 | 487.75 | 509.25 |
| 8x2-w32 | frozen | 24.570710 | 668.954 | 490.25 | 520.00 |
| 8x2-w32 | off | 24.634247 | 669.883 | 493.00 | 510.25 |
| 8x2-w32 | on | 24.481497 | 669.294 | 489.00 | 505.75 |
| 12x2-w32 | accepted | 34.317529 | 938.100 | 695.00 | 724.75 |
| 12x2-w32 | frozen | 34.325817 | 939.644 | 691.75 | 703.00 |
| 12x2-w32 | off | 34.327040 | 941.621 | 689.00 | 723.50 |
| 12x2-w32 | on | 34.148377 | 940.277 | 686.50 | 715.75 |
| 16x2-w32 | accepted | 40.360763 | 1120.510 | 795.50 | 828.25 |
| 16x2-w32 | frozen | 40.477126 | 1119.213 | 797.75 | 832.25 |
| 16x2-w32 | off | 40.583476 | 1123.459 | 790.75 | 824.50 |
| 16x2-w32 | on | 40.625234 | 1121.466 | 800.00 | 829.00 |
| 24x2-w32 | accepted | 66.617867 | 1890.641 | 1240.76 | 1303.88 |
| 24x2-w32 | frozen | 66.417999 | 1886.702 | 1264.99 | 1293.75 |
| 24x2-w32 | off | 66.587840 | 1889.648 | 1243.75 | 1311.39 |
| 24x2-w32 | on | 66.614080 | 1885.534 | 1222.00 | 1359.04 |
| merge-24x1-w32 | accepted | 66.145121 | 1806.291 | 1142.49 | 1167.47 |
| merge-24x1-w32 | frozen | 65.623023 | 1808.122 | 1126.84 | 1160.74 |
| merge-24x1-w32 | off | 66.371647 | 1814.867 | 1096.75 | 1154.00 |
| merge-24x1-w32 | on | 65.775218 | 1809.321 | 1128.78 | 1174.75 |
| merge-54x1-w32 | accepted | 163.442909 | 4419.680 | 1763.59 | 1860.42 |
| merge-54x1-w32 | frozen | 163.266446 | 4402.630 | 1757.47 | 1883.58 |
| merge-54x1-w32 | off | 163.109106 | 4413.857 | 1776.21 | 1855.16 |
| merge-54x1-w32 | on | 162.707139 | 4404.743 | 1760.89 | 1855.37 |
| 1x2-w16 | accepted | 10.497078 | 124.565 | 85.75 | 88.50 |
| 1x2-w16 | frozen | 10.349034 | 125.999 | 85.50 | 89.50 |
| 1x2-w16 | off | 10.407708 | 124.781 | 86.25 | 89.25 |
| 1x2-w16 | on | 10.083143 | 123.052 | 86.50 | 91.25 |

## Paired checks

Bounds use ranks4..12 of15, nominal96.484375% under independent exchangeable blocks, without familywise guarantees. Frozen/accepted and OFF/frozen bounds must fit[1/1.03,1.03] in all10cells. Primary32 ON requires upper<1 and at least11wins against all three controls. Every other ON/control cell requires upper<=1.02. No minimum gain floor, exclusion, extension, fastest-file selection or successful-capture repeat is applied. Failed bounds do not by themselves establish an exclusive causal slowdown.

| Cell/ratio | Paired median | Lower | Upper | Wins | Required check |
|---|---:|---:|---:|---:|---|
| 12x2-w32/frozen/accepted | 1.000480088 | 0.992108684 | 1.007659557 | 7/15 | pass |
| 12x2-w32/off/accepted | 1.006110006 | 0.994317552 | 1.011194104 | 4/15 | descriptive |
| 12x2-w32/off/frozen | 1.001516243 | 0.993814326 | 1.015088333 | 6/15 | pass |
| 12x2-w32/on/accepted | 0.995261929 | 0.990588440 | 1.003772405 | 10/15 | pass |
| 12x2-w32/on/frozen | 0.995100999 | 0.992699314 | 1.007557442 | 9/15 | pass |
| 12x2-w32/on/off | 0.993594469 | 0.991218049 | 1.000734862 | 11/15 | pass |
| 16x2-w32/frozen/accepted | 1.001029323 | 0.997639716 | 1.006186503 | 7/15 | pass |
| 16x2-w32/off/accepted | 1.005318680 | 0.991803674 | 1.012991266 | 6/15 | descriptive |
| 16x2-w32/off/frozen | 1.002231985 | 0.989084970 | 1.011097511 | 7/15 | pass |
| 16x2-w32/on/accepted | 1.005760545 | 0.998379794 | 1.017452639 | 5/15 | pass |
| 16x2-w32/on/frozen | 1.004882586 | 0.991430333 | 1.015558839 | 6/15 | pass |
| 16x2-w32/on/off | 1.006554341 | 0.992218000 | 1.015126008 | 5/15 | pass |
| 1x2-w16/frozen/accepted | 0.995899827 | 0.976159922 | 1.007466420 | 8/15 | pass |
| 1x2-w16/off/accepted | 0.993127481 | 0.949795799 | 1.036462839 | 8/15 | descriptive |
| 1x2-w16/off/frozen | 1.006541241 | 0.972992005 | 1.050659496 | 7/15 | fail |
| 1x2-w16/on/accepted | 0.966170059 | 0.935346573 | 0.998746840 | 12/15 | pass |
| 1x2-w16/on/frozen | 0.974520018 | 0.948877278 | 1.028285067 | 10/15 | fail |
| 1x2-w16/on/off | 0.968457791 | 0.934478908 | 0.995455995 | 12/15 | pass |
| 1x2-w32/frozen/accepted | 1.000746088 | 0.992067073 | 1.006087978 | 7/15 | pass |
| 1x2-w32/off/accepted | 0.993613812 | 0.983193646 | 1.000930252 | 11/15 | descriptive |
| 1x2-w32/off/frozen | 0.996215198 | 0.990049566 | 1.001002001 | 10/15 | pass |
| 1x2-w32/on/accepted | 0.978700145 | 0.973916815 | 0.984719850 | 15/15 | pass |
| 1x2-w32/on/frozen | 0.979446622 | 0.975018724 | 0.984055568 | 15/15 | pass |
| 1x2-w32/on/off | 0.982414773 | 0.980352054 | 0.985605957 | 14/15 | pass |
| 24x2-w32/frozen/accepted | 0.994274691 | 0.981951086 | 1.002101459 | 10/15 | pass |
| 24x2-w32/off/accepted | 0.999194510 | 0.987549746 | 1.005584085 | 8/15 | descriptive |
| 24x2-w32/off/frozen | 1.003290513 | 0.998710236 | 1.010473660 | 5/15 | pass |
| 24x2-w32/on/accepted | 0.995691765 | 0.987110464 | 1.006555894 | 9/15 | pass |
| 24x2-w32/on/frozen | 1.004675341 | 0.988279879 | 1.011839807 | 6/15 | pass |
| 24x2-w32/on/off | 1.001113778 | 0.991364539 | 1.003122468 | 6/15 | pass |
| 2x2-w32/frozen/accepted | 1.000658828 | 0.995703459 | 1.002764627 | 6/15 | pass |
| 2x2-w32/off/accepted | 1.002004164 | 0.997000072 | 1.007449553 | 5/15 | descriptive |
| 2x2-w32/off/frozen | 1.001302207 | 0.992732843 | 1.008507994 | 7/15 | pass |
| 2x2-w32/on/accepted | 0.995195918 | 0.989548724 | 0.998541616 | 13/15 | pass |
| 2x2-w32/on/frozen | 0.990572412 | 0.986218025 | 1.000992696 | 10/15 | pass |
| 2x2-w32/on/off | 0.992957820 | 0.990423662 | 0.997347440 | 13/15 | pass |
| 4x2-w32/frozen/accepted | 0.999780886 | 0.992531103 | 1.011365257 | 9/15 | pass |
| 4x2-w32/off/accepted | 0.994627950 | 0.990066784 | 1.006718158 | 9/15 | descriptive |
| 4x2-w32/off/frozen | 0.994630662 | 0.984492985 | 1.010260920 | 8/15 | pass |
| 4x2-w32/on/accepted | 1.000563626 | 0.987723025 | 1.014711125 | 7/15 | pass |
| 4x2-w32/on/frozen | 0.992057268 | 0.987162365 | 1.005580209 | 11/15 | pass |
| 4x2-w32/on/off | 0.998407809 | 0.985951474 | 1.016217523 | 8/15 | pass |
| 8x2-w32/frozen/accepted | 1.004869079 | 0.990650155 | 1.010901584 | 6/15 | pass |
| 8x2-w32/off/accepted | 1.004608113 | 0.991986455 | 1.024005461 | 5/15 | descriptive |
| 8x2-w32/off/frozen | 1.010273225 | 0.992162205 | 1.026970556 | 6/15 | pass |
| 8x2-w32/on/accepted | 1.004618374 | 0.995220065 | 1.018866461 | 6/15 | pass |
| 8x2-w32/on/frozen | 1.009186933 | 0.987142570 | 1.017082811 | 6/15 | pass |
| 8x2-w32/on/off | 0.996005981 | 0.986697596 | 1.010773663 | 9/15 | pass |
| merge-24x1-w32/frozen/accepted | 0.996367321 | 0.981311081 | 1.005827821 | 8/15 | pass |
| merge-24x1-w32/off/accepted | 1.001016399 | 0.991323572 | 1.010637462 | 7/15 | descriptive |
| merge-24x1-w32/off/frozen | 1.005692216 | 0.996200260 | 1.015975300 | 5/15 | pass |
| merge-24x1-w32/on/accepted | 0.997107753 | 0.992871617 | 1.001458890 | 11/15 | pass |
| merge-24x1-w32/on/frozen | 1.000584509 | 0.992956321 | 1.007569496 | 7/15 | pass |
| merge-24x1-w32/on/off | 0.998408958 | 0.982704764 | 1.001561595 | 10/15 | pass |
| merge-54x1-w32/frozen/accepted | 0.996305898 | 0.990359577 | 1.003024306 | 10/15 | pass |
| merge-54x1-w32/off/accepted | 0.996582498 | 0.987575581 | 1.003954697 | 9/15 | descriptive |
| merge-54x1-w32/off/frozen | 1.000513685 | 0.993155543 | 1.010302733 | 7/15 | pass |
| merge-54x1-w32/on/accepted | 0.993589144 | 0.986715773 | 0.999474471 | 12/15 | pass |
| merge-54x1-w32/on/frozen | 0.999554268 | 0.989971914 | 1.005250941 | 8/15 | pass |
| merge-54x1-w32/on/off | 0.996833567 | 0.987315260 | 1.008361048 | 9/15 | pass |

Failed checks: `["1x2-w16/off/frozen","1x2-w16/on/frozen"]`. Passing this screen would not establish the original10x goal or qualify every device/resource regime.

## Separate requested-memory observations

Three rotated observations per cell/variant, one complete SDK request after3primes, using a separate instrumented executable. The exact parent allocation wrapper delegates to the same native allocator and snapshots counters before JSON formatting. Its accounting test is inherited, not newly run. Allocation requests/bytes count requested payloads, including realloc requests; extra live is peak requested payload above the pre-request live level. These are whole-process request observations, not per-worker capacities, actual resident bytes or minimum RAM. Instrumented times are excluded from latency tables. All three observations are retained.

| Cell/variant | Allocation requests | Requested MiB | Extra live peak MiB | Live before MiB | Live after MiB |
|---|---:|---:|---:|---:|---:|
| 1x2-w16/accepted | 13310 | 105.966 | 10.745 | 5.865 | 5.866 |
| 1x2-w16/frozen | 13310 | 105.966 | 12.208 | 5.865 | 5.866 |
| 1x2-w16/off | 13310 | 105.966 | 12.872 | 5.865 | 5.866 |
| 1x2-w16/on | 13270 | 104.562 | 11.238 | 5.865 | 5.866 |
| 1x2-w32/accepted | 13310 | 105.966 | 16.122 | 5.974 | 5.976 |
| 1x2-w32/frozen | 13310 | 105.966 | 16.374 | 5.974 | 5.976 |
| 1x2-w32/off | 13310 | 105.966 | 16.367 | 5.974 | 5.976 |
| 1x2-w32/on | 13270 | 104.562 | 16.205 | 5.974 | 5.976 |
| merge-54x1-w32/accepted | 42699 | 1613.732 | 367.658 | 267.045 | 267.047 |
| merge-54x1-w32/frozen | 42784 | 1615.931 | 367.502 | 267.045 | 267.047 |
| merge-54x1-w32/off | 42763 | 1611.410 | 367.658 | 267.045 | 267.047 |
| merge-54x1-w32/on | 42498 | 1600.641 | 367.658 | 267.045 | 267.047 |

## Counters and execution profiles

Counters follow all latency captures:3 blocks × primary16/32 and merge54/32 ×4variants ×2 grouped userland panels,31 requests after3primes. All four counters per panel must count with at least99.9% running. One shared FIFO pair gates the measured request loop. Values below are median per-request counts; cycles/instructions/L1I/iTLB come from the fetch panel, frontend/L1D from separate stall-panel runs. This does not create a combined cycle decomposition or removable wall-time estimate.

| Cell/variant | Cycles M | Instructions M | L1I misses M | iTLB misses | Frontend stall M | L1D misses M |
|---|---:|---:|---:|---:|---:|---:|
| 1x2-w16/accepted | 604.869 | 1396.946 | 0.615185 | 102.5 | 28.117 | 30.330 |
| 1x2-w16/frozen | 608.638 | 1398.298 | 0.790682 | 149.7 | 28.670 | 29.815 |
| 1x2-w16/off | 594.496 | 1394.750 | 0.588611 | 103.5 | 26.997 | 29.858 |
| 1x2-w16/on | 593.265 | 1412.729 | 0.565301 | 134.6 | 27.921 | 26.734 |
| 1x2-w32/accepted | 870.478 | 1440.535 | 0.622571 | 730.5 | 60.926 | 38.371 |
| 1x2-w32/frozen | 864.661 | 1438.387 | 0.905172 | 611.8 | 59.119 | 38.094 |
| 1x2-w32/off | 864.266 | 1438.510 | 0.644779 | 558.6 | 56.792 | 38.318 |
| 1x2-w32/on | 846.228 | 1453.194 | 0.645819 | 663.6 | 60.560 | 34.398 |
| merge-54x1-w32/accepted | 20553.123 | 18016.351 | 3.828407 | 12159.7 | 544.662 | 798.203 |
| merge-54x1-w32/frozen | 20611.588 | 18018.181 | 3.769188 | 12133.6 | 554.582 | 793.297 |
| merge-54x1-w32/off | 20408.328 | 18014.810 | 4.597405 | 12446.6 | 557.452 | 791.912 |
| merge-54x1-w32/on | 20617.942 | 18020.584 | 3.265317 | 12004.1 | 611.341 | 791.451 |

Six separate FIFO-gated199Hz cycles profiles cover primary16/32 accepted/OFF/ON,251 requests after3primes. Decode runs after all proof/resource captures. All profiles must have one sampled SDK PID, no lost records and sampled TID coverage at least the intended worker count. TID counts are observed SDK threads, not a census assigning every TID to a particular Rayon task. ON must show the selected borrowed-module path; controls must not. Symbol period shares include called/inlined work and are not removable proof latency.

| Profile | Samples | Lost | Sampled TIDs | Borrowed symbol period |
|---|---:|---:|---:|---:|
| profile-1x2-w16-accepted | 5671 | 0 | 17 | 0.000% |
| profile-1x2-w16-off | 5616 | 0 | 17 | 0.000% |
| profile-1x2-w16-on | 5647 | 0 | 17 | 22.032% |
| profile-1x2-w32-accepted | 9440 | 0 | 33 | 0.000% |
| profile-1x2-w32-off | 9304 | 0 | 33 | 0.000% |
| profile-1x2-w32-on | 9092 | 0 | 33 | 19.873% |

Raw profiles and executable payloads remain local with hashes. All six complete loss dumps have verified deterministic-gzip round trips. Demangled maps, TID sets, counts, summaries and every counter observation are retained. Aggregate event changes do not establish a sole cause or equate frontend events to all CPU cost.

## Supplied M5 CPU and Metal references

All eight reference shapes are newly measured here. M5 numbers are supplied historical cross-machine references, not newly remeasured M5 performance. Accepted and candidate times are shown separately; the candidate is not adopted. Merge54 has no supplied reference.

| Shape | Variant | Local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | accepted | 8.379021 | 37 | 24 | 4.416× | 2.864× |
| 1x2 | on | 8.226135 | 37 | 24 | 4.498× | 2.918× |
| 2x2 | accepted | 11.502104 | 59 | 30 | 5.129× | 2.608× |
| 2x2 | on | 11.434172 | 59 | 30 | 5.160× | 2.624× |
| 4x2 | accepted | 16.233889 | 109 | 48 | 6.714× | 2.957× |
| 4x2 | on | 16.228639 | 109 | 48 | 6.717× | 2.958× |
| 8x2 | accepted | 24.535797 | 189 | 74 | 7.703× | 3.016× |
| 8x2 | on | 24.481497 | 189 | 74 | 7.720× | 3.023× |
| 12x2 | accepted | 34.317529 | 284 | 101 | 8.276× | 2.943× |
| 12x2 | on | 34.148377 | 284 | 101 | 8.317× | 2.958× |
| 16x2 | accepted | 40.360763 | 335 | 124 | 8.300× | 3.072× |
| 16x2 | on | 40.625234 | 335 | 124 | 8.246× | 3.052× |
| 24x2 | accepted | 66.617867 | 528 | 176 | 7.926× | 2.642× |
| 24x2 | on | 66.614080 | 528 | 176 | 7.926× | 2.642× |
| merge-24x1 | accepted | 66.145121 | 507 | 175 | 7.665× | 2.646× |
| merge-24x1 | on | 65.775218 | 507 | 175 | 7.708× | 2.661× |

The original eight-shape CPU10x objective remains **active and unmet**, 0/8 candidate point estimates at the target in this capture. Primary requires3.7ms. Component improvements and screen outcomes do not redefine completion. The prior accepted FFT engineering exception and its false full-performance/qualified-scope gates remain unchanged.

## Provenance and validation

Accepted production has172 original files and SDK SHA256 `14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a`.168 private original files remain byte-exact; reversing the four suffix/dispatch edits recovers all172. Two new modules implement the borrowed queue/prefix. Field source and arithmetic are unchanged. The original accepted executable is hardlinked; the three ordinary SDK links and four resource SDK links use exact coherent previously frozen native rlibs/proc macros. Actual native flags/features and transitive library hashes are checked; no native library or component test is rebuilt/repeated.

Parent native OFF119 / ON122 core tests are inherited, including signed borrowed-prefix tails/reuse, full-width phi MSMs and adversarial identity/equal/opposite/scalar-edge cases. Prior field qualification is inherited. Optional missing-fixture scenarios are not claimed. These are not newly executed test counts. Original parent diagnostic exports are unused by the SDK; ordinary latency executables have the unchanged standard SDK driver/global allocator and no heap observer.

Before capture, frozen/OFF complete .text/.rodata and every other runtime section byte are identical except individually decoded g2_phi Rust panic source-line fields+2 and build ID. Executable identities and names of equal length are fixed before timing; all outcomes are kept. The preceding same-byte SDK study did not show a large full-proof backing effect, but does not establish universal equivalence; measured control calibration remains required here.

The9 images are the preceding diagnostic's restored historical Euler images, each checked against expected image/public hashes and requests.36 preflights,600 latency,72 counter,6 profile and36 separate resource processes complete. All **39,060 complete SDK requests verify internally**; all **750 fresh164-byte exports independently pass Gnark**, expected public/image binding, member hashing and uniqueness against13,712 prior identities. The independent ledger is **14,462**. The1,500 proof/public members are not counted as1,500 proofs. All independent verification and decoding occur after capture.

No owned build/helper/hash/archive or second benchmark overlaps capture. Unrelated host work remains untouched; host snapshots are not an exclusive-use claim. Initial metadata preparation corrected the heap-test receipt path before SDK linking; original helper and failure are retained, with no successful test/link/capture repeat. ELF audit templates are retargeted to the actual native-links directory before execution. No global ASLR/cache/kernel change occurs. Broader portable/resource/device/250/4999 qualification remains necessary before adoption; this screen does not claim AWS/Aeglos, phone/TEE, pressure/swap, minimum RAM or giant-circuit performance. Resource forecasts retain their conservative existing capacity estimates.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native links](artifacts/native-links/result.json), [qualification inheritance](artifacts/qualification-inheritance.json), [control audit](artifacts/control-locations.json), [all750 captures/verifiers](artifacts/runs/result.json), [proof inventory](artifacts/proof-inventory.json).

## Findings and next action

**Retain the clear primary32 improvement.** ON is8.226135ms versus accepted8.379021, frozen8.392573 and OFF8.356026. The paired ON/accepted ratio is0.978700145 [0.973916815,0.984719850],15/15wins: a2.13% paired reduction, bounds1.53–2.61%. ON/frozen is0.979446622 [0.975018724,0.984055568],15/15; ON/OFF is0.982414773 [0.980352054,0.985605957],14/15. A ratio of marginal medians is a different estimator and is not substituted for these paired results. All primary32 improvement checks pass, and every other all-eight/54 calibration/nonregression check passes apart from the two physical16 checks below.

**The full declared screen still fails two physical16 bounds; no actual16-worker slowdown is established.** ON16 is10.083143ms versus accepted10.497078, frozen10.349034 and OFF10.407708. Its paired ratios improve against each control: ON/accepted0.966170059 [0.935346573,0.998746840],12/15wins; ON/OFF0.968457791 [0.934478908,0.995455995],12/15; ON/frozen0.974520018 [0.948877278,1.028285067],10/15. The last upper bound does not exclude a regression exceeding2%, so it misses nonregression. The same-code OFF/frozen calibration is1.006541241 [0.972992005,1.050659496],7/15wins, missing its required3% upper bound. No block is excluded, rerun or blamed on a proved exclusive host/kernel/backing cause. Failed checks remain false; positive estimates remain positive within their measured scope.

Primary16 and32 instrumented requests each remove exactly40 allocation requests and1,472,000 requested bytes (1.403808594MiB) in the reported medians:13,310/111,112,976bytes for all controls versus13,270/109,640,976bytes for ON. This is less allocation traffic, not proof of1.4MiB lower RSS. Primary32 requested extra-live medians are16,991,717bytes ON,17,161,809 OFF and16,904,997 accepted; timing-run median RSS is130.75MiB ON versus130.50 accepted, with maxima134.50/134.00. Peak overlap/retention depends on execution, and a stable whole-process resident-peak reduction is not established. Merge54 resource observations vary and stay descriptive; its latency ON/accepted is0.993589144 [0.986715773,0.999474471],12/15, while ON/OFF0.996833567 [0.987315260,1.008361048] spans parity.

Primary32 fetch-panel ON/OFF is846.23/864.27Mcycles, while retired instructions increase to1453.19 from1438.51M. Separate stall-panel L1D misses fall to34.40 from38.32M, while frontend stalls are60.56 versus56.79M. These three-block event associations do not prove a sole mechanism. The real improvement is not explained by fewer total retired instructions or uniformly fewer frontend stalls. All six profiles have one SDK PID, zero lost records and17/33 sampled TIDs for16/32workers. Borrowed-module period is22.03%/19.87% in ON and zero in controls, confirming the intended path is used; those shares are not removable elapsed proof time.

The candidate is not adopted by this screen, and the original all-eight10x goal remains unmet. The next action is **compatibility/resource qualification and an explicit adoption assessment of this positive candidate**, retaining the two failed physical16 bounds and their uncertainty. This includes native opt-outs and the unchanged portable fallback, plus relevant memory-pressure behavior before any source promotion. Do not discard the primary gain as zero, silently promote a failed gate, choose a favorable executable or rerun unchanged latency in search of a passing result. Any later engineering exception must be stated plainly and supported by the additional qualification. Further CPU-work reductions remain necessary for the original target.
