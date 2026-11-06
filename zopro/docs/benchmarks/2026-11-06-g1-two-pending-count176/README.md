# Two-arrival G1 streaming count screen

This fixed state-machine model replays retained primary K/H inputs with one pending point per bucket, making a triple ready only after the next arrival. **No native candidate is selected.** The model reduces projective routing but increases generic multiplication work, inverse stages and pending storage. It is a count screen, not a prover timing result.

No native candidate is selected from this fixed count screen. The two-arrival state machine greatly reduces modeled projective routing but adds arithmetic, inverse stages and storage. It does not establish a credible path to a primary-proof breakthrough.

Across all 20 original balanced K/H windows, both models consume the same 1,170,733 nonzero/nonidentity signed arrivals and make the same 70,681 first writes. The original finite model has 1,088,726 ordinary affine updates and 11,326 projective arrivals. Two-arrival produces 532,086 triple updates, 35,482 ordinary updates and 398 projective arrivals. Queue arrivals decrease 68,404→28,317; spill flushes 52→2; modeled projective initializations 4,415→1 and accumulator adds 6,911→397. These are generic state counts, not native exceptional-branch observations.

The fixed ready capacity preserves roughly the original logical-update batch budget: 96 triples for batch 200 and 176 for batch 352. Therefore two logical updates per triple do not halve the number of inversion stages. Modeled stages increase 3,713→3,809 (+2.59%), including 120 ordinary tail batches. Triple batches have 78 scalar-tail triples and residual ordinary batches have 74 scalar-tail updates; scalar-tail service is not priced in the simple multiplication proxy.

The explicit 6M-per-ordinary/13M-per-triple source proxy rises 6,532,356→7,130,010 (+9.15%). It excludes both the removed projective arithmetic/normalization and extra inverse service, as well as packing, signs, memory, exceptions and scheduling. It is not a latency forecast or proof that every two-arrival implementation must lose. Additional pending storage alone is forecast at 1,146,880 bytes across 20 windows (16 bytes per bucket); ready records, allocator capacity and simultaneous old/new scratch must also be charged. This is not measured RSS or a mobile memory bound.

All 1,229,020 digits match an independent bit reader, all 61,451 scalar rows reconstruct exactly, and 60 crafted state-machine cases conserve signed integer-group outputs and work totals. Every real-window integer-group sum matches the independent input sum. No coordinates are present, so equal-X/H-zero guards, inverse-point cancellations and complete curve correctness are untested. Native curve arithmetic is not implemented. The successful input extraction and old native tests were not repeated; no solver/FFT, SDK proof, profile or timing call was added.

The older mixed-triple SDK regression, current H-only tree failure and binary-prefix component failures remain unchanged. The proposed coalescer is not a flag rerun of those implementations, but this count result does not justify carrying it into a new native experiment. Keep all results and move to a different useful-work hypothesis. Accepted source 176, SDK6d83, all-eight comparisons and proof ledger 30,846 remain unchanged; the CPU 10× objective stays active and unmet, 0/8.


## Policy and complete counts

The original balanced 255-bit cover has 20 windows, five of width 12 and fifteen of width 13. Original batches remain 200/352; new ready limits are 96/176 triples. The same signed digit reader, input identities and original bounded newest-first queue geometry are used. After ready triples and queue draining finish, unmatched pending points run through ordinary-size batches. The source/model/design were frozen before one Rust/xtask replay; no parameter search or model extension followed.

This differs from the [earlier mixed SDK](../2026-10-28-mixed-sdk/README.md), which consumed selected queue collisions from already-full binary batches and remains rejected. The [H-only native algebra](../2026-11-06-triple-h-native176/README.md) supplies the 13M generic formula count, but is not integrated here.

|Modeled work|Original|Two-arrival|
|---|---:|---:|
|arrivals|1170733|1170733|
|first_writes|70681|70681|
|ordinary_affine_updates|1088726|35482|
|triple_updates|0|532086|
|logical_affine_updates|1088726|1099654|
|queue_arrivals|68404|28317|
|queue_drained|57078|27919|
|projective_arrivals|11326|398|
|projective_initializations|4415|1|
|projective_accumulator_adds|6911|397|
|queue_flushes|52|2|
|ordinary_batches|3713|120|
|triple_batches|0|3689|
|inverse_stages_model|3713|3809|
|ordinary_scalar_tail_updates|78|74|
|triple_scalar_tail_updates|0|78|
|generic_base_field_M_site_proxy|6532356|7130010|
|additional_pending_payload_forecast_bytes|0|1146880|

All 40 window rows, histograms, pending/queue/ready maxima and conservation values are retained in the analysis. The baseline arrival/occupied/generic-join totals exactly match the independently reviewed earlier [K/H work model](../2026-11-06-g1-tagged-phi-review176/README.md). This does not make the new baseline replay a native branch census; no input coordinates or exceptional-case evidence were added.

## Validation and limitations

The replay checks all 1,229,020 digits against a separate bit reader, reconstructs all 61,451 canonical scalar words and checks 60 crafted state machines using independent signed integer-group sums. Each actual-window sum also matches its independent per-destination input sum. The publication reader independently reconstructs aggregate totals, 40 window conservation identities, batch/tail histograms, input/hash/command receipts and the old generic-join counts. It does not rerun the successful replay or certify native curve behavior.

The first helper compile failed on ambiguous integer sum types in assert_eq; explicit sum::<u64> annotations fixed it before freezing/execution. Original source and the preparation note are retained. No successful solver extraction, native test, profile, SDK proof or count replay was repeated. The private cache stays local and is represented only by size/hash; no scalar, key, image, witness, proof or executable bytes are included.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.




## Continuation and artifacts

The [next research step](artifacts/next.md) seeks a different reduction in dominant streamed field work or inverse service, without sweeping this fixed coalescer or reopening closed captures. Production, accepted latency/memory and the 30,846-proof ledger remain unchanged. All eight CPU10× targets remain active and unmet. No current AWS/Aeglos, phone/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or proof-throughput result is added.

[Analysis](artifacts/analysis.json), [design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [model](artifacts/model.rs), [preparation](artifacts/preparation.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS).
