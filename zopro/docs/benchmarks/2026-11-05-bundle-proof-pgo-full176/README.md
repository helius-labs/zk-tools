# Composed proof-only PGO: full SDK comparison

The candidate records a paired median **3.495% reduction** on local 1×2 at 32 workers versus the accepted SDK, and a **0.953% reduction** in the original-eight geometric aggregate. Primary and aggregate gain checks pass against all three controls. The full gate fails one original-link calibration check on 8×2; the fixed candidate is **not adopted**. No observations are removed and no unchanged rerun is scheduled.

This is a local AMD Ryzen 9 9950X experiment, not a new AWS or Aeglos comparison. The accepted implementation remains unchanged. The all-eight CPU10× objective remains active and unmet, **0/8**.

## Candidate and protocol

The candidate combines the frozen parent GLV final assembly and guarded affine queue draining with freshly trained proof-only PGO. Its 179 source files are byte exact to the qualified composition; all 176 original files are recoverable and 170 are unchanged. Field arithmetic and generated constants remain unchanged. The prior [qualification report](../2026-11-05-bundle-proof-pgo176/README.md) retains source, raw/merged compiler profiles, tests, compiler warnings and coverage limits. Its 99 GEN proof exports are inherited and are not counted among the new exports here.

Four distinct binaries run in 15 matched, rotated/reversed blocks across ten contexts: the original eight shapes and merge54 with 32 logical workers on CPUs 0–31, plus primary with 16 distinct physical cores on CPUs 0–15. Each process primes three proofs, then measures 151 primary32, 301 primary16, or 31 other requests. SDK calls include internal verification. Original-driver fresh linking uses original coherent libraries and matching native/O/fat-LTO/CGU1 options; it is a link control, not a fresh source compilation. The plain composition changes behavior and is not calibration-equivalent to accepted.

| Arm | Meaning | SDK SHA-256 |
|---|---|---|
| accepted | Original accepted SDK | `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` |
| OFF | Fresh original-library link | `60539c41c247e49dcba339e3a2fbe3fcbbc97b9f8b3d2996376915e9c4921d47` |
| plain | Frozen composition without this PGO | `1a1ecbd680c5f5794859d5b225647e6b34e8e80d325cebd47b356b70b6fcde34` |
| ON | Qualified PGO composition | `817d07cd7aadc5586d0a8472dcce18ee7fe12f5f8802e0a283d6212070c95203` |

All 40 preflight exports pass independent Gnark verification before latency recording. All 600 latency processes precede eight warm PMU panels and eight fixed-period cycle profiles; all recordings precede the remaining 616 independent export verifications and profile decoding. No owned compiler, analyzer, archive, verifier or second workload overlaps recording. The 1,353-command capture completes on its first attempt with no resume.

## Full-proof cells

Values are medians of the 15 process medians. Paired ratio medians below are calculated within blocks, so they need not equal ratios of these marginal medians. CPU and RSS are per-process SDK diagnostics. RSS is a maximum observed lifetime high-water mark, including setup, not an isolated proving peak, minimum RAM or memory-pressure qualification.

| Context | Accepted ms | Original-link ms | Plain ms | PGO ms | Accepted CPU ms | PGO CPU ms | Accepted RSS MiB | PGO RSS MiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2-w32 | 7.944809 | 7.930953 | 7.719835 | 7.655704 | 179.769 | 172.043 | 134.500 | 133.000 |
| 2x2-w32 | 11.124654 | 11.187405 | 11.282853 | 11.084839 | 259.296 | 255.792 | 268.000 | 270.500 |
| 4x2-w32 | 15.962982 | 15.967175 | 16.091759 | 16.124231 | 419.567 | 423.862 | 311.500 | 313.500 |
| 8x2-w32 | 24.309470 | 24.037700 | 24.292412 | 24.239252 | 669.713 | 663.518 | 503.250 | 514.500 |
| 12x2-w32 | 33.779147 | 33.847005 | 33.980647 | 33.613352 | 937.713 | 929.860 | 738.000 | 726.000 |
| 16x2-w32 | 40.188313 | 39.986487 | 40.123229 | 39.749694 | 1127.496 | 1112.676 | 847.250 | 833.500 |
| 24x2-w32 | 65.820317 | 66.371731 | 66.374771 | 65.609733 | 1882.561 | 1875.950 | 1277.750 | 1332.500 |
| merge-24x1-w32 | 65.188836 | 65.375832 | 65.597620 | 64.705525 | 1812.214 | 1797.197 | 1200.914 | 1175.000 |
| merge-54x1-w32 | 162.343469 | 161.943295 | 162.389646 | 161.784063 | 4436.840 | 4432.032 | 1921.211 | 1882.617 |
| 1x2-w16 | 9.439689 | 9.396646 | 9.354536 | 9.079436 | 116.632 | 112.352 | 92.500 | 92.250 |

The one RSS warning is 24×2 PGO/original-link: 1,332.500 / 1,265.117 MiB (about +5.33%). Accepted is 1,277.750 MiB. This exceeds the diagnostic 1.05 ratio threshold and remains retained; it is not evidence of a hard RAM requirement or a passed resource gate. Primary32 observes 133.000 MiB for PGO and 134.500 MiB for accepted.

## Fixed gates and decision

Before recording, original-link/accepted calibration required each paired interval within [1/1.03, 1.03]. Candidate non-regression requires upper ≤1.03 in all ten contexts against each of accepted, original-link and plain. Primary32 and the original-eight geometric aggregate additionally require upper <1 and at least 10/15 winning blocks against each of those three controls. No gain floor, exception transfer, observation removal, extension, profile choice, favorable subset or unchanged reroll is allowed. Bounds use sorted paired block ratios at ranks 4 and 12; nominal 96.484375% coverage is conditional on exchangeability and is not a simultaneous guarantee.

All 30 candidate non-regression checks and all six primary/aggregate gain checks pass. Nine of ten calibration checks pass. The single failed check is **8×2 original-link/accepted**: median 0.994316208, interval **[0.969122199, 1.004207915]**. The lower calibration bound is **0.970873786**. This is a control-calibration failure, not a measured candidate slowdown. The fixed full gate therefore fails and production does not change.

| Gain check | Paired ratio median | Interval | Wins / 15 | Pass |
|---|---:|---|---:|---|
| Primary32 / accepted | 0.965048252 | [0.959115116, 0.968969328] | 15 | true |
| Original-eight / accepted | 0.990474918 | [0.989249534, 0.993021797] | 15 | true |
| Primary32 / OFF | 0.969030614 | [0.961623110, 0.971844720] | 15 | true |
| Original-eight / OFF | 0.992159589 | [0.987861724, 0.993901540] | 14 | true |
| Primary32 / plain | 0.989988582 | [0.988141303, 0.995603517] | 13 | true |
| Original-eight / plain | 0.990932446 | [0.986081338, 0.993657979] | 15 | true |

All 40 individual paired checks and their 15 raw ratios, including uncertainty and small regressions, remain in [analysis](artifacts/analysis.json). For example, 4×2 PGO/accepted has median 1.002073 and upper 1.022021: it passes the fixed non-regression allowance, but does not establish a speedup for that shape.

## Counters and profile scope

All eight four-event PMU panels record 100% running time. Profiles use user cycles, fixed period 16,000,000 and 251 measured requests after priming. All eight pass owner PID/TID membership, CPU range, fixed-period, ≥1,000 samples, ≤5% unknown and zero-loss checks; total samples are 82,550. Single diagnostic panels and flat profiles do not establish exclusive critical paths, whole-proof absolute-period coverage, or causation.

| Context / arm | Cycles/request | Instructions/request | Branch misses/request | Cache misses/request |
|---|---:|---:|---:|---:|
| 1x2-w16 / accepted | 551051638 | 1345658593 | 787732 | 1930066 |
| 1x2-w16 / OFF | 559932399 | 1347248069 | 787725 | 2090281 |
| 1x2-w16 / plain | 552629909 | 1301694342 | 783734 | 1979061 |
| 1x2-w16 / ON | 543730524 | 1265594137 | 809226 | 2107847 |
| 1x2-w32 / accepted | 812657681 | 1387547148 | 899452 | 2866683 |
| 1x2-w32 / OFF | 820325335 | 1390549245 | 904404 | 2978066 |
| 1x2-w32 / plain | 799363668 | 1339491879 | 889590 | 2933163 |
| 1x2-w32 / ON | 786426189 | 1295568436 | 925808 | 3049223 |

| Profile | Samples | Unknown % | MSM/curve % | Inverse % | Crossbeam epoch % | Runtime or inlined payload % |
|---|---:|---:|---:|---:|---:|---:|
| cycles-1x2-w16-OFF | 8685 | 1.313 | 74.427 | 8.923 | 3.397 | 1.670 |
| cycles-1x2-w16-ON | 8415 | 0.963 | 73.345 | 9.138 | 0.345 | 5.645 |
| cycles-1x2-w16-accepted | 8605 | 1.360 | 75.503 | 8.600 | 3.149 | 1.604 |
| cycles-1x2-w16-plain | 8567 | 1.587 | 73.830 | 9.280 | 3.969 | 1.856 |
| cycles-1x2-w32-OFF | 12349 | 1.887 | 67.301 | 7.952 | 9.466 | 3.385 |
| cycles-1x2-w32-ON | 11754 | 0.893 | 67.007 | 7.376 | 1.412 | 11.817 |
| cycles-1x2-w32-accepted | 12209 | 1.638 | 67.377 | 7.126 | 10.099 | 3.366 |
| cycles-1x2-w32-plain | 11966 | 1.588 | 67.015 | 7.655 | 9.836 | 3.368 |

The PGO profiles move attribution away from named Crossbeam epoch functions toward runtime or inlined payload. Inlining and code layout can change symbol attribution, so this does not demonstrate elimination of scheduling work. MSM/curve arithmetic remains the largest category. Counter differences are diagnostic evidence for this compiler artifact, not an allocation or memory-bandwidth explanation.

## Correctness and independent review

All **46,264 SDK calls** verify internally, including primes and diagnostic requests. All **656 new 164-byte exports** independently pass Gnark and expected public/image binding, common verification-key and freshness checks. The cumulative proof-hash ledger moves from **30,052 to 30,708**. The independent offline reviewer checks 1,201 source/input pins, 1,354 successful workflow commands including the fresh link, all raw samples and cell medians, all 40 paired checks, all six gain checks, all eight CPU/Metal comparisons, all export identities and all PMU/profile ownership and loss checks. It agrees with the single failed gate. No successful capture, test or verifier is repeated.

The first bootstrap looked for SDK-driver pins in the prover-source manifest; the accepted link artifact holds them separately. It stopped before driver copying or link preparation. The repair uses those exact accepted input pins and binds the artifact to the accepted SDK hash. Original source and failed output remain in the artifacts. No arithmetic, compiler flag, gate, measurement or successful stage changes. [Repair record](artifacts/initial-bootstrap/repair.md).

## Supplied M5 references

These are current local PGO measurements compared with supplied historical M5 CPU/Metal references; they are not simultaneous hardware measurements. The PGO candidate is not adopted.

| Shape | PGO local ms | M5 CPU ms | CPU/local | M5 Metal ms | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2-w32 | 7.655704 | 37 | 4.833× | 24 | 3.135× | 3.7 |
| 2x2-w32 | 11.084839 | 59 | 5.323× | 30 | 2.706× | 5.9 |
| 4x2-w32 | 16.124231 | 109 | 6.760× | 48 | 2.977× | 10.9 |
| 8x2-w32 | 24.239252 | 189 | 7.797× | 74 | 3.053× | 18.9 |
| 12x2-w32 | 33.613352 | 284 | 8.449× | 101 | 3.005× | 28.4 |
| 16x2-w32 | 39.749694 | 335 | 8.428× | 124 | 3.120× | 33.5 |
| 24x2-w32 | 65.609733 | 528 | 8.048× | 176 | 2.683× | 52.8 |
| merge-24x1-w32 | 64.705525 | 507 | 7.835× | 175 | 2.705× | 50.7 |

Merge54 records 161.784063 ms for PGO and 162.343469 ms for accepted in this run; no M5 reference is supplied. There is no new AWS/Aeglos result, portable or mobile qualification, TEE result, memory-pressure/swap or minimum-RAM measurement, concurrent-throughputput result, or 250/4999 circuit comparison. Passing this timing protocol would only select further compatibility/resource/final-artifact qualification. The present failed calibration closes this fixed candidate without that adoption step.

## Reproduction and artifacts

Private experiment root: `/home/ubuntu/src/zopro-perf-work/bundle-proof-pgo176/performance`. The sealed [capture plan](artifacts/plan.json), [link plan](artifacts/link-plan.json), [identities](artifacts/identities.json), [design](artifacts/design.md), [independent review](artifacts/review.json), [proof inventory](artifacts/proof-inventory.json) and [harness source](artifacts/harness-source.tar.gz) bind this run to exact inputs and commands. Prior qualification retains the unchanged prover source and compiler profiles. Exact command/output receipts, including decoded profiles and the initial bootstrap failure, are in [receipts](artifacts/receipts.json.gz). All verified proof/public outputs are in [exports](artifacts/proofs.tar.gz). [Checksums](SHA256SUMS) and [publication review](publication-review.json) cover these files.

Artifact extraction checks member names, lengths and hashes; receipt compression round trips exact UTF-8 bytes. Executables, frozen libraries, raw perfdata, proving keys, image/witness/scalar/point arrays are excluded. Private inputs are required for reproduction.
