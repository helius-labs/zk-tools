# Clean borrowed G2 production artifact

Final artifact screen passed: **false**. Production adoption: **pending**. All 477 exported proofs verified independently with Gnark; 26208 complete SDK requests also verified internally. The original tenfold CPU goal remains active: **0/8 shapes**.

The clean candidate replaces copied G2 point queues with proof-local borrowed references and omits two unused scalar buffers. It removes experimental flags and public probe exports, using the existing supported-native guard. No persistent key table or expanded plan is introduced. The source audit retains 169 of 172 original files exactly; reversing three edits and removing two modules recovers the original source. Native tests pass: 122 core and 8 raw/cubic prover tests.

The rebuilt control has identical allocated ELF sections to the accepted SDK. Clean candidate code differs from the earlier experimental build, so these are fresh fixed-artifact measurements, not inherited timings. See `artifacts/final-protocol.md`, executable identities, all raw logs and independent verifier records. Fifteen interleaved rotated/reversed blocks, 32 workers on CPUs 0–31, plus a descriptive physical16 primary cell. Each process primes three times, then measures151 primary or31 other requests. Loading is excluded; solving, randomized proving, SDK verification and serialization are included. No exclusive host or parallel-proof throughput claim. RSS includes load/primes.

| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 1x2-w32 | accepted | 8.563320 | 192.020 | 130.25 | 132.75 |
| 1x2-w32 | control | 8.526204 | 191.143 | 131.00 | 137.00 |
| 1x2-w32 | candidate | 8.366931 | 187.775 | 131.25 | 135.00 |
| 2x2-w32 | accepted | 11.715533 | 265.442 | 260.75 | 269.50 |
| 2x2-w32 | control | 11.681242 | 265.743 | 257.75 | 280.50 |
| 2x2-w32 | candidate | 11.539335 | 263.331 | 265.75 | 276.50 |
| 4x2-w32 | accepted | 16.521704 | 419.558 | 303.75 | 314.50 |
| 4x2-w32 | control | 16.447584 | 417.040 | 297.25 | 316.25 |
| 4x2-w32 | candidate | 16.540233 | 420.261 | 299.00 | 310.75 |
| 8x2-w32 | accepted | 24.989183 | 665.952 | 486.75 | 510.50 |
| 8x2-w32 | control | 24.905854 | 664.708 | 486.00 | 508.00 |
| 8x2-w32 | candidate | 25.136722 | 672.256 | 491.25 | 518.50 |
| 12x2-w32 | accepted | 35.340175 | 927.706 | 676.50 | 695.25 |
| 12x2-w32 | control | 34.606472 | 928.505 | 691.00 | 727.75 |
| 12x2-w32 | candidate | 34.858408 | 929.848 | 691.25 | 716.50 |
| 16x2-w32 | accepted | 41.690357 | 1118.411 | 794.25 | 817.50 |
| 16x2-w32 | control | 42.148081 | 1125.723 | 800.25 | 825.54 |
| 16x2-w32 | candidate | 41.377043 | 1121.647 | 788.50 | 824.00 |
| 24x2-w32 | accepted | 68.298581 | 1889.610 | 1236.58 | 1298.75 |
| 24x2-w32 | control | 68.114221 | 1883.880 | 1248.31 | 1343.12 |
| 24x2-w32 | candidate | 67.764001 | 1883.911 | 1254.09 | 1308.06 |
| merge-24x1-w32 | accepted | 66.847135 | 1788.647 | 1123.62 | 1158.76 |
| merge-24x1-w32 | control | 67.078231 | 1792.593 | 1126.73 | 1180.87 |
| merge-24x1-w32 | candidate | 67.096516 | 1793.683 | 1119.72 | 1148.91 |
| merge-54x1-w32 | accepted | 164.819989 | 4370.163 | 1804.90 | 1858.91 |
| merge-54x1-w32 | control | 165.418710 | 4370.475 | 1824.36 | 1921.86 |
| merge-54x1-w32 | candidate | 164.388184 | 4355.140 | 1802.20 | 1885.21 |
| 1x2-w16 | accepted | 10.935407 | 124.391 | 87.00 | 94.50 |
| 1x2-w16 | control | 10.863711 | 124.446 | 89.75 | 93.50 |
| 1x2-w16 | candidate | 11.031550 | 122.112 | 89.50 | 92.75 |

| Shape | M5 CPU ms | M5 Metal ms | Candidate ms | CPU speedup | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 12x2 | 284.0 | 101.0 | 34.858408 | 8.147× | 2.897× |
| 16x2 | 335.0 | 124.0 | 41.377043 | 8.096× | 2.997× |
| 1x2 | 37.0 | 24.0 | 8.366931 | 4.422× | 2.868× |
| 24x2 | 528.0 | 176.0 | 67.764001 | 7.792× | 2.597× |
| 2x2 | 59.0 | 30.0 | 11.539335 | 5.113× | 2.600× |
| 4x2 | 109.0 | 48.0 | 16.540233 | 6.590× | 2.902× |
| 8x2 | 189.0 | 74.0 | 25.136722 | 7.519× | 2.944× |
| merge-24x1 | 507.0 | 175.0 | 67.096516 | 7.556× | 2.608× |

Paired median ratio bounds use ranks4 and12 of15, nominal96.484375% without familywise guarantees. Default32 control/accepted calibration must fit[1/1.03,1.03]; candidate primary upper<1 and at least11wins against both controls; other candidate shapes upper<=1.02. Physical16 is descriptive here. All blocks are retained; no minimum gain floor or successful-capture reroll.

| Pair | Median ratio | Lower | Upper | Wins | Check |
|---|---:|---:|---:|---:|---|
| 12x2-w32/candidate/accepted | 0.986583428 | 0.980601637 | 1.009102505 | 10/15 | pass |
| 12x2-w32/candidate/control | 1.004837160 | 0.996027659 | 1.009168872 | 5/15 | pass |
| 12x2-w32/control/accepted | 0.988413920 | 0.975993340 | 1.013126991 | 10/15 | pass |
| 16x2-w32/candidate/accepted | 1.000016798 | 0.983775430 | 1.025268464 | 7/15 | fail |
| 16x2-w32/candidate/control | 0.996363669 | 0.973286732 | 1.001149396 | 11/15 | pass |
| 16x2-w32/control/accepted | 1.001650727 | 0.990260324 | 1.027089995 | 7/15 | pass |
| 1x2-w16/candidate/accepted | 1.006277542 | 0.926775520 | 1.059578883 | 7/15 | descriptive |
| 1x2-w16/candidate/control | 1.008219997 | 0.972333191 | 1.054156766 | 6/15 | descriptive |
| 1x2-w16/control/accepted | 0.998073382 | 0.945699557 | 1.015797600 | 8/15 | descriptive |
| 1x2-w32/candidate/accepted | 0.978626960 | 0.972489712 | 0.981221808 | 15/15 | pass |
| 1x2-w32/candidate/control | 0.983091442 | 0.972401032 | 0.987757656 | 15/15 | pass |
| 1x2-w32/control/accepted | 0.995355364 | 0.989431007 | 1.003981231 | 10/15 | pass |
| 24x2-w32/candidate/accepted | 0.997932218 | 0.994651230 | 1.002773856 | 9/15 | pass |
| 24x2-w32/candidate/control | 0.998489490 | 0.991314060 | 1.004685928 | 8/15 | pass |
| 24x2-w32/control/accepted | 0.998096846 | 0.986709615 | 1.008981448 | 8/15 | pass |
| 2x2-w32/candidate/accepted | 0.987214504 | 0.981769424 | 0.995380552 | 15/15 | pass |
| 2x2-w32/candidate/control | 0.989680641 | 0.985122297 | 0.996493544 | 13/15 | pass |
| 2x2-w32/control/accepted | 0.999846158 | 0.995316640 | 1.002913375 | 9/15 | pass |
| 4x2-w32/candidate/accepted | 0.998526464 | 0.986650510 | 1.006146348 | 9/15 | pass |
| 4x2-w32/candidate/control | 1.001503118 | 0.990536273 | 1.013042113 | 6/15 | pass |
| 4x2-w32/control/accepted | 0.995513780 | 0.985169684 | 1.009862747 | 9/15 | pass |
| 8x2-w32/candidate/accepted | 1.001878376 | 0.992088027 | 1.010148986 | 6/15 | pass |
| 8x2-w32/candidate/control | 1.005697395 | 0.992069010 | 1.015065601 | 5/15 | pass |
| 8x2-w32/control/accepted | 0.998172690 | 0.986467730 | 1.001168123 | 10/15 | pass |
| merge-24x1-w32/candidate/accepted | 1.009090831 | 1.002017501 | 1.013509105 | 3/15 | pass |
| merge-24x1-w32/candidate/control | 0.998602773 | 0.993384965 | 1.009852511 | 8/15 | pass |
| merge-24x1-w32/control/accepted | 1.005366819 | 0.995454360 | 1.019950420 | 5/15 | pass |
| merge-54x1-w32/candidate/accepted | 1.001992108 | 0.994367294 | 1.007132808 | 6/15 | pass |
| merge-54x1-w32/candidate/control | 0.999255932 | 0.992197944 | 1.009629938 | 8/15 | pass |
| merge-54x1-w32/control/accepted | 0.998618432 | 0.995841746 | 1.008037381 | 8/15 | pass |

Failed final-artifact checks: `["16x2-w32/candidate/accepted"]`.

The earlier full experimental screen remains failed at two physical16 uncertainty bounds; its positive32 estimate is retained. Compatibility/resource evidence is in [the qualification report](../2026-10-28-g2-borrowed-qualification/README.md). Forced-high stress [timed out on the accepted baseline](../2026-10-28-g2-borrowed-forced-pressure/README.md); candidate was not run under that condition. No new pressure/minimum-RAM/mobile/swap/giant/AWS claim follows from this study. Earlier allocation-traffic savings are not a resident-memory guarantee. Any adoption must explicitly retain these limitations. Binary/rlib identities are hashed locally; large executable artifacts are not committed.
