# Mixed G1 flush in verified SDK proofs

The mixed flush fails the full SDK gate. At 32 workers, enabling it is **6.58% slower than the fresh accepted-source control and 6.90% slower than OFF**, losing all seven paired blocks. **The candidate is rejected; production remains unchanged.** The earlier modeled caller gains do not transfer to this production integration.

The candidate combines ordinary and three-point signed additions in full 200-entry borrowed G1 batches, with 1–32 matching queued destinations. Other batches retain the original path. Its arithmetic comes from the [qualified mixed caller](../2026-10-28-mixed-callers/README.md). The experiment includes actual pending/queue handling, optional resource admission and complete exceptional fallback; it is not another uniform small-triple benchmark.

## Complete SDK requests

Ryzen 9950X. Seven rotated/reversed fresh-process blocks compare a fresh exact-172-source control with runtime OFF/ON in one candidate executable. Primary32 uses CPUs 0–31; secondary16 uses CPUs 0–15, one logical CPU per physical core. Each process runs 151 measured calls after three primes. Loading/preparation precedes measurement; solving, fresh randomness, proving, internal verification and serialization are inside each request.

| Workers | Variant | Median SDK ms | CPU ms/request | Median high-water RSS MiB |
|---|---|---:|---:|---:|
| 32 | Fresh control | 8.476779 | 190.779 | 131.50 |
| 32 | OFF | 8.490616 | 190.050 | 131.25 |
| 32 | ON | 9.015185 | 198.797 | 132.00 |
| 16 | Fresh control | 10.823085 | 124.565 | 88.00 |
| 16 | OFF | 11.925083 | 122.088 | 89.50 |
| 16 | ON | 11.047378 | 128.792 | 88.25 |

CPU sums user and system time over the measured loop. RSS is the median of whole-process high-water observations across seven processes, including loading/primes; it is not minimum RAM or a replacement for the accepted historical RSS figure.

| Workers / comparison | Paired median ratio | Individual median interval | Wins/7 |
|---|---:|---:|---:|
| 32 OFF/control | 1.000779 | 0.977753–1.010086 | 2 |
| 32 ON/control | 1.065825 | 1.041339–1.088852 | 0 |
| 32 ON/OFF | 1.069016 | 1.056830–1.079229 | 0 |
| 16 OFF/control | 1.127189 | 0.811046–1.150567 | 2 |
| 16 ON/control | 1.017150 | 0.921568–1.222831 | 3 |
| 16 ON/OFF | 0.902264 | 0.876054–1.232519 | 5 |

Intervals are the min/max of all seven paired block ratios, 98.4375% nominal coverage under independent/exchangeable blocks, without a familywise guarantee. No samples are removed, extended or rerun. Both primary gain conditions fail. The 16-worker calibration also fails the factor-1.03 requirement, and both secondary upper bounds exceed 1.02. Its favorable ON/OFF median cannot establish a gain. There is no minimum gain floor.

## Actual queue behavior

Separate OFF/ON diagnostic SDK processes observe one proof after three unobserved primes. Their arithmetic module bytes match the timed candidate; counters are absent from the timed executable. Signed input identities, pending additions, projective updates, inverse-point cancellations and consumed queued entries are included in the logical-update conservation check.

| G1 200-entry stream, per proof | OFF | ON |
|---|---:|---:|
| Windows | 21 | 21 |
| Affine flushes | 1722 | 1617 |
| Pending entries executed | 343214 | 322204 |
| Mixed flushes | 0 | 1558 |
| Queued entries consumed by triples | 0 | 17264 |
| Projective queue updates | 5331 | 9077 |
| Projective queue flushes | 46 | 65 |
| Queue-take blocked events | 921 | 141 |
| Tracked logical updates | 391544 | 391544 |

Affine flushes decrease by 105, about 6.1%, but projective queue updates increase by 3746, about 70.3%. Pending updates fall by 21010; 17264 move into triple operations and 3746 into projective operations. The complete logical count is conserved. The other G1 352-entry stream and G2 stream retain identical aggregate counts.

This demonstrates why an explicit two-layer caller was insufficient: changing which queued entries are consumed also changes later queue and fallback behavior. Fewer inversion batches do not alone imply lower total CPU time. These counters identify an unfavorable interaction with queue routing; they do not isolate the entire latency regression as one causal cost.

## Memory accounting, tests and profiles

One shared query-level reservation covers all potential worker windows, including split parts. Existing resource-aware admission uses current headroom, active workspace forecasts and an explicit user ceiling when configured. There is no new universal 64 MiB cap. Checked typed forecasts cover six vectors, singleton state storage, shared ownership and allocation margins. Buffer construction uses `try_reserve_exact`; admission denial or a buffer-reservation error selects the original algorithm. Over-limit collision counts are rejected before growing the reserved triple buffer. Old workspace may coexist, so the charge does not assume a replacement buffer has been freed. These implementation checks are not phone, TEE, pressure or minimum-memory qualification.

All 121 core tests pass with the option OFF and again ON as separate runtime configurations. Two new test cases cover checked forecasts/admission denial and every collision count from 0 through 40, including unchanged fallback state outside the selected range. The prover's two raw and six cubic tests pass. Unchanged field qualification is inherited. The prior independent Arkworks formula/queue tests remain separate evidence; no new claim that all these counts represent independent test cases is made.

The private candidate changes two original source files and adds one arithmetic/admission module. Inverse edits recover both originals exactly; 170 other private originals and all 172 production files remain unchanged. The fresh control has all 172 accepted hashes. The diagnostic build adds the census hooks and a cancellation counter while preserving the candidate arithmetic module bytes. An extended diagnostic snapshot leaves the original pinned snapshot untouched.

Two initial parallel build launches were rejected by xtask's exclusive host lock before compilation. Their logs remain; builds then completed sequentially. No test, SDK capture or proof verification failed or was repeated. The completed diagnostic extension occurred before any SDK capture.

Six FIFO-gated user-cycle profiles at 199 Hz follow all latency captures. Counters enable after loading/three primes and disable before export. Sample counts at 32 workers are 6546/6672/6782 for control/OFF/ON; at 16 they are 3270/3356/3440, 30066 total, one user PID each and zero lost records. Raw recordings remain local with hashes; decoded symbols are archived. The new mixed body appears among ON hotspots; scalar inversion, projective addition, existing streams and coordination remain visible. Sample shares are not removable wall-time fractions. No owned build/helper/hash/archive or second workload overlaps SDK capture.

All 53 captures finish before independent verification and profile decoding. **7412 complete internally verified SDK requests, 53 distinct fresh independently Gnark-verified exports, 106 proof/public members; ledger 12527 to 12580.** Public bindings and freshness against all prior/new hashes pass. All 924 source, executable, library, helper and input pins match before and after the study.

No adoption, AWS/Aeglos result, other-shape performance, portable-device, concurrency/pressure or large-circuit qualification is claimed. This specified integration is closed. A follow-up must change the actual work or routing mechanism rather than rerolling its flags, inlining, register allocation or kernel layout. The remaining queued free entries and the original newest-first stop rule warrant source/history review before any distinct queue algorithm is proposed.

## Inherited CPU and Metal comparison

The [accepted report](../2026-10-27-clean-fft-adoption/README.md), including its full-performance-gate failure and engineering exception, remains authoritative. M5 figures are supplied historical references. Accepted local 1x2 remains **8.324853 ms at 134.50 MiB peak RSS**. The original all-eight CPU 10x goal remains active and unmet, 0/8.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445x | 24 | 2.883x |
| 2x2 | 11.460675 | 59 | 5.148x | 30 | 2.618x |
| 4x2 | 15.854365 | 109 | 6.875x | 48 | 3.028x |
| 8x2 | 24.084626 | 189 | 7.847x | 74 | 3.072x |
| 12x2 | 33.693397 | 284 | 8.429x | 101 | 2.998x |
| 16x2 | 39.981636 | 335 | 8.379x | 124 | 3.101x |
| 24x2 | 66.010670 | 528 | 7.999x | 176 | 2.666x |
| Merge24x1 | 65.086624 | 507 | 7.790x | 175 | 2.689x |

Accepted merge54 remains 161.341835 ms without a supplied M5 reference.

[Protocol](artifacts/design.md), [complete results and counters](artifacts/analysis.json), [candidate source audit](artifacts/source-audit.json), [native OFF tests](artifacts/core-tests.log), [native ON tests](artifacts/core-on-tests.log), [proof inventory](artifacts/proof-inventory.json), [pins](artifacts/pins.json), [workflow completion](artifacts/runs/result.json), [artifact hashes](artifact-index.json).
