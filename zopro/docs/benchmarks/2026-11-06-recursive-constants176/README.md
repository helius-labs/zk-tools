# Recursive structural constants: no additional candidates

**All nine mapped-image plans contain zero additional linear assignments or constant wires under the fixed Known/Unknown analysis.** This closes this constant-propagation approach without a key transformation or production change. Accepted proof times remain unchanged. The all-eight CPU10× goal remains unmet (0/8).

## Question and result

The [earlier direct-linear census](../2026-10-22-universal-linear-assignment-census/README.md) recognized constants written directly in expressions. This screen additionally propagates constants through stored sum prefixes, derived wires and Bits/InvZero hints, using the current accepted solver semantics. Actual input values remain unknown. It also recognizes unconditional zero assignments in the solver's zero-divisor branches, while keeping guarded cases separate.

Every plan has zero direct or newly derived linear assignments, zero constant assignments, zero known hint inputs/outputs, zero known stored-slot writes, zero guarded zero-divisor cases and zero additional zero-RHS assignments. A and B each contain only the existing wire-zero constant one; private K and saved C slots contain no structural constants. The analyzer validates read/write order and stored assignment coefficient inverses. No point rewrite, lookup table, second plan or serialized-plan copy is made.

| Shape | Constraints | Assignments | New linear | Constant assignments |
|---|---:|---:|---:|---:|
| 1x2 | 30922 | 28501 | 0 | 0 |
| 2x2 | 53874 | 49479 | 0 | 0 |
| 4x2 | 98674 | 90319 | 0 | 0 |
| 8x2 | 189618 | 173295 | 0 | 0 |
| 12x2 | 280242 | 255887 | 0 | 0 |
| 16x2 | 371122 | 338671 | 0 | 0 |
| 24x2 | 554226 | 505391 | 0 | 0 |
| merge-24x1 | 525759 | 479818 | 0 | 0 |
| merge-54x1 | 1178199 | 1074928 | 0 | 0 |

This domain does not identify arbitrary cancellation of unknown expressions, affine aliases, nonlinear identities, hint-bit correlations or simplifications that depend on actual inputs. Zero candidates close this specific analysis, not every possible circuit simplification. Counts of constants would not themselves predict MSM savings because accepted execution already skips or groups some cheap scalars. No broader domain or post-result parameter sweep is selected here.

## Qualification and execution

The private source copy adds one analysis child and three append-only diagnostic exports. Independent source recovery checks all 176 accepted originals: 173 are byte-identical and three recover exactly after removing recorded suffixes. Production files are unchanged. Diagnostic core and prover libraries reuse the accepted field and dependencies with Rust 1.98.1, without the closed experimental compiler model. The driver borrows the solver plan already loaded by Prover.

Two new focused tests pass; the 125 existing core tests are filtered rather than repeated. Source inventory totals 550 synthetic concrete-solver attempts, including expected unsatisfiable cases: 70 inputs across pools of 1/4/16/32 workers with configured 512 KiB stacks (280 attempts), 70 dynamic-hint inputs, and 200 combinations covering both division sides, unknown divisors and contradictions. Predicted constants are compared with the unchanged solver. The positive stored-sum fixture covers None and Slot dependencies; it has no dedicated known-valued Chain fixture. These are synthetic solves, not SDK proofs.

Each of the nine real images is loaded once in a separate process. No raw-format repeat or real-witness solve is performed. Fourteen native commands succeed: five build/test commands and nine audits. The initial core compilation failed because its plan omitted Cargo macro environment and the new serde_json extern. A continuation repairs only the invocation, preserving source/rules and the failed logs; no successful stage is repeated.

The first reviewer incorrectly assumed historical stored-slot counts were identical. Merge54 currently reports 387,544 slots versus 1,429,945 in the October 22 census, which explicitly used a copied f5c720 solver. The corrected reviewer retains this difference and checks logical dimensions, exact current source, source pins, command/stream receipts and query/count invariants. The older result is not an identical-plan control, and this difference is not claimed as a new resource improvement. The failed reviewer and its logs remain archived. Independent review does not replay the analyzer or establish arbitrary symbolic completeness.

## Analysis resources

These values describe the diagnostic process only. Modeled auxiliary bytes sum actual vector capacities, including 40-byte Option<Fr> entries, readiness/eligibility flags and temporary query lists. They exclude the borrowed plan, JSON, allocator overhead and stacks. Whole-process maximum RSS also includes the image loader and mapped pages. Neither column is a proposed production working set, proof peak or minimum RAM.

| Shape | Modeled auxiliary bytes | Whole-process max RSS MiB |
|---|---:|---:|
| 1x2 | 1871375 | 17.750 |
| 2x2 | 3261444 | 28.250 |
| 4x2 | 5972978 | 49.250 |
| 8x2 | 11476590 | 111.750 |
| 12x2 | 16957546 | 144.000 |
| 16x2 | 22450982 | 173.500 |
| 24x2 | 33512094 | 282.250 |
| merge-24x1 | 31862923 | 272.500 |
| merge-54x1 | 71412437 | 585.500 |

The implementation is a temporary circuit screen, not persistent per-key metadata. Its largest modeled allocation is about 68.1 MiB at merge54; its maximum observed process RSS is 585.5 MiB. The negative result gives no reason to add that state to the universal prover.

## Accepted end-to-end baseline, inherited

These are retained accepted local 32-worker measurements, not new measurements from this screen. M5 values are supplied CPU and Metal references in milliseconds. The target is one tenth of M5 CPU.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU speedup vs M5 CPU | CPU10× target ms |
|---|---:|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 4.667× | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.291× | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 6.776× | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 7.817× | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 8.381× | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 8.359× | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 8.018× | 52.8 |
| merge 24→1 | 65.060937 | 507 | 175 | 7.793× | 50.7 |

Merge54's inherited accepted local result is 161.883899 ms; no supplied M5 reference exists for it. The canonical proof ledger remains 30,846, with zero new SDK or independently verified proofs. The inherited accepted 1×2 RSS is 132.75 MiB, not a new measurement or minimum-RAM claim. No fresh AWS, Aeglos, Gnark, mobile-device, huge-circuit, pressure/swap or concurrent-proof measurement is made. No cloud resources are created.

## Evidence and decision

[Review and all nine count/query rows](artifacts/review.json), [source inverse audit](artifacts/source-audit.json), and [archive index](artifacts/archive-index.json) preserve the observations and their scope. The compressed UTF-8 archive contains analysis/test source, invocation plans, pins, failed/successful receipts and review/publication helpers. Every archived member and the gzip roundtrip are checked exactly. Images, points, keys, witnesses, dependencies and native executables/libraries are excluded; pinned hashes identify local inputs.

Local root: `/home/ubuntu/src/zopro-perf-work/recursive-constants176`. Accepted source manifest: [176-file manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). Accepted SDK SHA256: `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`.

Close this fixed-domain screen. Select a distinct way to remove substantial MSM work using retained profiles and a whole-proof impact/memory argument; no speed adoption follows from this result.
