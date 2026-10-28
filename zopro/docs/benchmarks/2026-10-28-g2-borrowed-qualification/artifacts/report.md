# Borrowed G2 compatibility and resource qualification

The exact candidate passes the added qualification:11 portable field tests,99 core tests,8 prover tests,248 complete SDK requests and89 independent Gnark exports. Eight cold/warmed memory cases finish under their declared limits without OOM/max events. Production remains unchanged pending an explicit adoption assessment. The original SDK performance screen remains **failed** at two physical16 bounds; its clear primary32 improvement is retained.

The [full SDK experiment](../2026-10-28-g2-borrowed-sdk/README.md) measured primary32 at8.226135ms versus accepted8.379021ms, paired2.13% faster with bounds1.53–2.61% and15/15wins. It removes40requests/1472000requestedbytes per primary proof, without a stable whole-process RSS reduction. Physical16 point estimates improve, but OFF/frozen calibration and ON/frozen nonregression missed required bounds. This qualification does not repeat latency, select another executable, exclude blocks or turn those failed checks into passes.

## Coverage

- The174-file private candidate snapshot matches the measured source. Portable x86-64/no-ASM/no-IFMA builds keep the private borrowed flag enabled while its native guards exclude the new path. All unfiltered suites pass, and native119/122 tests are inherited rather than rerun.

- 13 portable real-proof cases cover all8 reference shapes plus54 with basis images, and primary/54 with raw and denied optional-preparation paths.

- 24 native cases cover1/4workers,512KiB/1MiB worker stacks, primary/54, and basis/raw/denied paths.8 additional cases disable packed/direct/canonical-batch/vector-inverse on accepted and candidate SDKs.

- 4 concurrent cases cover accepted/candidate × primary/54,4callers ×2requests. They check37 loaded TIDs and33 idle TIDs: one32worker global pool plus callers, without a replacement pool. All32 fresh exports pass independent verification.

- 8 pressure cases cover cold/warm × accepted/candidate × default/retained allocator settings. The controller creates and charges a private image copy inside each owned transient service. Cold limits are192MiB; warmed cases prime at512MiB, add32MiB ballast, then lower high/maximum limits to128/192MiB before two fresh proofs. Swap is disabled for these cases. These are inherited validation conditions, not an architectural cap, minimum RAM or proof of swap behavior.

Raw proving images are checked against historical hashes. The unchanged independent Gnark verifier obtains the matching verifying key from the corresponding historical Euler image; it is the same circuit/key in a different query representation. Both image identities and expected public hashes are retained. This checks raw-loader proof correctness without changing the verifier parser.

## Resource observations

| Case | Passed | Peak MiB | High events | Max events | OOM/kill | Swap limit |
|---|---|---:|---:|---:|---|---|
| cold-accepted-default | true | 62.25 | 0 | 0 | 0/0 | 0 |
| cold-accepted-retained | true | 61.89 | 0 | 0 | 0/0 | 0 |
| cold-on-default | true | 61.29 | 0 | 0 | 0/0 | 0 |
| cold-on-retained | true | 62.12 | 0 | 0 | 0/0 | 0 |
| warm-accepted-default | true | 101.40 | 0 | 0 | 0/0 | 0 |
| warm-accepted-retained | true | 101.14 | 0 | 0 | 0/0 | 0 |
| warm-on-default | true | 105.45 | 0 | 0 | 0/0 | 0 |
| warm-on-retained | true | 102.32 | 0 | 0 | 0/0 | 0 |

High events distinguish observed throttling from merely running under configured limits; complete current/peak/pressure/event/stat snapshots are retained. No larger-limit retry is permitted on failure. The service controller stops and resets only its uniquely prefixed units and removes their owned control directories. No global memory/kernel setting or unrelated process changes. Actual resource peaks include the private charged image, controller, SDK and any ballast; they are not a minimum-RAM guarantee.

## Verification and adoption assessment

All248 SDK requests verify internally. All89 fresh164byte exports independently pass Gnark, expected public/key binding, member hashing and uniqueness against14462 prior identities. The independent ledger is14551;178 proof/public members are not178 proofs. There are45 functional processes,4 concurrent processes and8 resource cases. Builds finish before proof capture, and no owned build/hash/helper/archive/second benchmark overlaps capture. Unrelated host work is not excluded.

The candidate is eligible for an explicit engineering adoption assessment using the clear primary32 improvement and this added coverage, while accepting the documented physical16 uncertainty. The old performance gates remain false. Source promotion must still be concrete and reviewed; this report alone does not change production. No new mobile/TEE, AWS/Aeglos, swap,250/4999 or minimum-RAM result is claimed. Giant images from earlier qualification are absent. The original all8 CPU10x objective remains active/unmet0/8, with primary requiring3.7ms.

## Inherited CPU and Metal comparisons

The following are the preceding full SDK measurements and supplied historical M5 references, not timings from these functional/resource checks.

| Shape | Accepted ms | Candidate ms | M5 CPU ms | M5 Metal ms | CPU/candidate | Metal/candidate |
|---|---:|---:|---:|---:|---:|---:|
| 12x2 | 34.317529 | 34.148377 | 284 | 101 | 8.317× | 2.958× |
| 16x2 | 40.360763 | 40.625234 | 335 | 124 | 8.246× | 3.052× |
| 1x2 | 8.379021 | 8.226135 | 37 | 24 | 4.498× | 2.918× |
| 24x2 | 66.617867 | 66.614080 | 528 | 176 | 7.926× | 2.642× |
| 2x2 | 11.502104 | 11.434172 | 59 | 30 | 5.160× | 2.624× |
| 4x2 | 16.233889 | 16.228639 | 109 | 48 | 6.717× | 2.958× |
| 8x2 | 24.535797 | 24.481497 | 189 | 74 | 7.720× | 3.023× |
| merge-24x1 | 66.145121 | 65.775218 | 507 | 175 | 7.708× | 2.661× |

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [build/test receipts](artifacts/build/result.json), [functional receipts](artifacts/functional/result.json), [resource receipts](artifacts/resource-result.json), [proof inventory](artifacts/proof-inventory.json). Executable/rlib payloads remain local with hashes and exact library receipts.

## Evidence limits and next action

The added scope passes:11 field/99 core/8 prover portable tests,45 functional configurations,4 concurrent cases,8 bounded-memory cases,248 complete SDK requests and89 independent fresh Gnark exports. Native119/122 tests are inherited. All new proof identities are unique against the14462 earlier identities; the ledger is14551. Source and native measured artifacts remain unchanged, and no latency screen is repeated.

**The bounded-memory cases did not exercise kernel high-limit throttling.** Every `memory.events.high` value is0. Cold charged peaks are roughly61–62MiB; warmed peaks, including32MiB held ballast, are roughly101–105MiB. All cases have no OOM/kill/max events, `memory.max=192MiB` and swap disabled. The native allocator may act proactively on its forecast/headroom, but these counters do not establish which internal branch caused the observed footprint. Passing below a configured limit is not evidence of observed throttling, minimum RAM, or phone/swap performance.

Before source promotion, run a distinct declared pressure scenario with held ballast alone above `memory.high`, while keeping the inherited192MiB hard limit. Require positive high events, successful fresh independently verified proofs, no OOM/max events and owned-service cleanup. This changes the operating condition rather than rerunning the failed latency screen or enlarging a failed budget. The current89 exports and all original successes remain retained; pressure-phase results must be reported separately.

The prior primary32 gain remains real within its measured scope: paired2.13%,15/15wins, against accepted; all three primary gain checks pass. The two physical16 bounds remain failed, with positive point estimates and uncertainty. These results justify an explicit engineering adoption assessment after the missing pressure evidence; they do not justify silently marking the old screen passed. The10x objective remains active/unmet0/8.

## CPU follow-up

Retained profile IPs resolve into the small epoch-pinning bodies; sampled offset`+0x61` is immediately after a locked compare/exchange. Sampling skid and parallel execution prevent equating that association with removable wall time. Local pinned Crossbeam code pins before checking an empty queue, and its public `is_empty` also requires a sequentially consistent fence. Thus an empty-queue precheck may avoid TLS/epoch-publication work but does not remove every synchronization cost; long-lived pin caching would delay reclamation and still needs fences. Any optimization must preserve the queue's ordering/liveness guarantees. The [Crossbeam implementation](https://github.com/crossbeam-rs/crossbeam/blob/main/crossbeam-deque/src/deque.rs) supports these source-level constraints; no queue change is adopted here.

For a more substantial alternative, [BWoS (OSDI2023)](https://www.usenix.org/conference/osdi23/presentation/wang-jiawei) separates owner/thief activity into blocks to reduce synchronization and provides a weak-memory verification framework. Its runtime results are not Zopro proof-speed evidence. It is a research direction to compare with the measured scheduler cost, not a claim of a future10x or2x gain.
