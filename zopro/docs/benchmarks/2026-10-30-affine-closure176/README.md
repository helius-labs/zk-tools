# Recursive affine closure of witness assignments

**No removable query terms found under this simplification.** Exact sparse affine propagation through running sums, stored slots, hints and assignments finds zero new affine or constant assignments in the retained primary and 2×2 programs. A separate reader certifies the same negative result. No prover implementation or timing trial follows.

This strengthens the earlier [direct linear-assignment census](artifacts/prior-linear-assignment-summary.md), which checked assignment expression syntax. That audit explicitly left recursive stored-slot and constant propagation open. The new reader evaluates cancellations and previously derived constants over Fr, with only input wire 0 fixed to one. Every nonlinear assignment or unknown hint introduces an opaque atom. This is an incomplete affine abstraction: it does not solve general polynomial relations or infer facts from constraints without assignments. The earlier coefficient-matrix rank census addresses a different transformation and supplies no witness-folding bound here.

|Circuit|Wires|Assignments|Stored slots|Sum records|New affine assignments|New constant assignments|Constant hint outputs|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|30521|28501|9928|247530|0|0|0|
|2x2|53064|49479|17439|429744|0|0|0|

A/B query lengths are 21,006/30,089 for primary and 36,637/52,274 for 2×2. All four populations contain zero candidate rows beyond the inputs. No stored slot becomes constant, and no additional expression side becomes constant beyond the original syntax. The largest affine expression contains 254 atoms in both programs; no limit truncates the analysis. The maps retain 269,589/468,298 atom terms offline. These counts describe analysis objects, not runtime plans or prover memory.

Division is only expanded with a provably nonzero constant divisor. A known zero divisor preserves the solver's zero-output branch for satisfiable inputs. With an unknown divisor, proportional numerator/divisor is not cancelled; zero changes solver semantics. The exact all-zero assigned-side/output special case is recognized. General multiplication expands only when an operand is constant. Unknown hint outputs remain opaque, including facts about bit ranges that this abstraction does not exploit.

## Validation and provenance

The five-step xtask workflow completes once: link three structural/algebra tests, execute them, link the reader, then analyze each cached solver program. All three tests pass. They cover 128 field valuations, cancellation and constant scaling, assignment expressions, unknown/zero divisors, a synthetic stored-sum image with a hidden linear multiplier, and malformed input/read/write rejection. No successful test or census is repeated.

Both readers consume every image byte, execute every sum record exactly once, account for every row/write, and check assignment coefficients and inverses. The original reader uses normalized BTreeMaps. The separate checker uses sorted coefficient vectors and certifies each assignment nonlinear before treating its output as a new atom. They agree on dimensions, all 77,980 assignments, 677,274 sum records and constant-side counts. They share the accepted Fr arithmetic and documented image layout; this is not an independent field-arithmetic oracle.

All 84,796 constraints and 85,144 operations are covered. The two solver exports are reused byte for byte from [wire readiness](../2026-10-30-wire-readiness175/README.md). Program hashes, accepted-library links, source pins and every command/stdout/stderr/exit receipt are retained. All seven workflow commands complete on their first attempts. Helper binaries named `*-sdk` are readers, not proof SDK artifacts.

Production remains the 176-file accepted source, SDK SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. All source, helper, input and library pins match. No witness solve, solver export, proof, Gnark verification, profile or latency sample is added; the distinct-proof ledger stays 27,353. Private solver images, libraries and executables remain local. Production suites are inherited unchanged.

## Accepted proof comparisons, unchanged

These are inherited local 32-worker full SDK observations and supplied M5 reference timings, not measurements of this census. All eight CPU 10× targets remain unmet.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2x2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4x2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8x2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12x2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16x2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24x2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge-24x1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Accepted merge 54→1 is 161.883899 ms without a supplied M5 reference. Accepted primary process RSS is 132.75 MiB. This census measures no prover memory. The physical-16 calibration engineering exception remains documented in [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md).

The latest retained AWS comparison uses historical source 175 on c7a.24xlarge: Zopro 11.915591 ms at 32 workers and Aeglos 7.959184 ms at 96, each campaign's best tested configuration. It does not measure current source 176. No AWS resources or other platform/resource measurements are added. The target still requires roughly 2.14× further primary latency reduction and improvements on every supplied shape.

This exact affine-closure route is closed on the two retained programs. Stronger nonlinear algebra or other circuits are outside its scope. No scalar/key table, key transformation, persistent duplicate solver plan or arbitrary runtime RAM cap is proposed.

[Design](artifacts/design.md), [all counts](artifacts/analysis.json), [primary](artifacts/1x2.json), [2×2](artifacts/2x2.json), [model](artifacts/model.rs), [driver](artifacts/census.rs), [workflow helper](artifacts/workflow.rs), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [input provenance](artifacts/inputs.json), [separate checker](artifacts/review.rs), [certificate](artifacts/review.json), [review plan](artifacts/review-plan.json), [review pins](artifacts/review-pins.json), [all raw receipts](artifacts/receipts.json.gz), [accepted manifest](artifacts/accepted-manifest.json), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS).
