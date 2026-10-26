# Fixed BN254 exponent chain in the full SDK

The declared local gate **fails**. Production remains unchanged; no candidate is adopted by this experiment.

Primary32 improves by a paired 0.389% against fresh OFF and 0.376% against accepted, with five and six wins of seven respectively. The original-eight aggregate improves by 0.248% and 0.414%. Five control-calibration checks fail, and physical16 exceeds the 2% ON/OFF nonregression guard. The full failure is retained; the primary result does not replace it. [Interpretation and limits](artifacts/interpretation.md) records the decision.

The exact candidate from the [actual-library component](../2026-10-26-pairing-x-chain/README.md) replaces binary parameter powers with the fixed addition/subtraction schedule in [Gnark Crypto v0.21.0](https://github.com/Consensys-Incorporated/gnark-crypto/blob/v0.21.0/ecc/bn254/internal/fptower/e12_pairing.go). Each power uses60 cyclotomic squares,17 full Fq12 products and5 conjugations instead of63 squares/28products. Three powers per final exponentiation remove9 squares and33 full extension products at source level. The component passed, but that does not establish a whole-proof improvement.

The general exponent, Miller loop, base-field kernels, hard-part formula, MSMs, solver and SDK driver remain unchanged. A tiny policy-only OnceLock flag selects the complete helper once; no key-specific eligibility, retained input-dependent state, point table, key copy or new heap workspace is added. Key loading latches the flag before warm proof timers. Stack use is not measured as a RAM minimum. Only pairing.rs plus one new module change;169 original files and all170 production files remain unchanged.

## Complete verified requests

Ryzen9950X16physical/32logical CPUs. All9shapes use32workers/CPUs0–31. Primary also uses16physical workers/CPUs0–15. Accepted/freshOFF/ON run7 rotated/reversed fresh-process blocks. Nine32-worker cases use31timed+3primes; physical16 uses151timed+3primes, declared before the first build to increase steady samples after previous short-run control instability. Earlier failures are preserved; no previous experiment is reclassified.

OFF/ON share an executable and unchanged SDK driver. Timer includes witness,fresh blinded proving,standard verification and serialization; loading/setup excluded. Times are medians of process medians, ratios are within-block geometric means. CPU includes scheduling and is not an arithmetic count; RSS includes loading/primes and is not minimum RAM.

|Case|Accepted ms|OFF ms|ON ms|ON/OFF|ON/accepted|ON/accepted wins|ON RSS MiB|ON CPU ms/request|
|---|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2-w32|8.642453|8.619118|8.618067|0.996112|0.996237|6/7|113.50|199.734|
|1x2-w16|10.929908|10.379018|10.576250|1.026702|0.958801|5/7|88.50|131.101|
|2x2-w32|11.756497|11.642822|11.636600|0.997301|0.989757|6/7|282.00|269.535|
|4x2-w32|16.230727|16.171866|16.027312|0.992061|0.990872|6/7|318.00|422.249|
|8x2-w32|24.740679|24.556572|24.665167|1.004758|0.996369|2/7|531.00|681.645|
|12x2-w32|34.551946|34.643139|34.542959|0.999893|0.999239|3/7|721.25|965.968|
|16x2-w32|40.814660|40.502662|40.649779|0.999431|0.992373|5/7|844.25|1138.022|
|24x2-w32|68.811062|68.892877|68.535851|0.992819|1.000677|5/7|1294.50|1974.224|
|merge-24x1-w32|67.064555|67.295080|67.312555|0.997874|1.001387|3/7|1192.98|1874.536|
|merge-54x1-w32|177.082271|174.691230|175.679550|1.008410|0.995956|5/7|1900.46|4864.853|

|Selected case|Accepted RSS MiB|OFF RSS MiB|ON RSS MiB|
|---|---:|---:|---:|
|1x2-w32|114.25|111.75|113.50|
|1x2-w16|91.00|90.00|88.50|
|merge-54x1-w32|1907.95|1901.25|1900.46|

These maxima do not establish a memory saving. [Native resource audit](artifacts/resource-audit.json) and the three archived assembly listings record function text sizes and explicit frame decrements. The candidate's public frame grows from 9048 to 11512 explicit bytes and calls a 2888-byte power frame, excluding pushes, return addresses and callees. This is a small local stack cost, with no new key table or heap workspace; full stack high-water and minimum RAM are unmeasured.

## Supplied M5 comparisons

Historical supplied references. New ON numbers include the complete verified request. Merge54 has no supplied reference.

|Shape|ON ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|
|---|---:|---:|---:|---:|---:|
|1x2|8.618067|37|24|4.29×|2.78×|
|2x2|11.636600|59|30|5.07×|2.58×|
|4x2|16.027312|109|48|6.80×|2.99×|
|8x2|24.665167|189|74|7.66×|3.00×|
|12x2|34.542959|284|101|8.22×|2.92×|
|16x2|40.649779|335|124|8.24×|3.05×|
|24x2|68.535851|528|176|7.70×|2.57×|
|merge-24x1|67.312555|507|175|7.53×|2.60×|
|merge-54x1|175.679550|—|—|—|—|

0/8 reach supplied CPU10×. The original all-eight goal remains active and unproven.

## Frozen gate

OFF/accepted calibration requires factor1.05eachblock and1.03pairedmedian ALL10cases. Primary32 must improve against BOTHcontrols with4/7wins, no minimumgainfloor. Everycaseincludingphysical16 must stay within1.02pairedgeomean both. Original8aggregate must be<=1.00both. No posthoc selection, exclusions or successful repeats.

calibration_failures:

- 8x2-w32 b5 OFF/accepted=0.933144017997509
- 1x2-w16 b1 OFF/accepted=0.9250461266451896
- 1x2-w16 b2 OFF/accepted=0.9293794614211037
- 1x2-w16 b6 OFF/accepted=0.7692962521943781
- 1x2-w16 paired OFF/accepted=0.9567670651939615

primary_failures:

- None.

regressions:

- 1x2-w16 on/off=1.0267015662670549

Original8ON/OFF geometric ratio 0.997523944;ON/accepted 0.995855084.

## Validation and profiles

Fresh35native field tests,109core,raw2/cubic6,generator checks,portable field13/native-noIFMA field16,and portable/noIFMA core/prover/zolana consumer checks pass. New field tests independently check symbolic60/17/5 counts, general/binary power agreement, complete final exponentiation and existing Arkworks references. No phone/portable whole-SDK latency is claimed.

All9OFF/ON4worker preflight completes72SDK18distinctGnark exports. The complete study totals10,494SDK requests,231distinct independently Gnark-verified exports and462proof/public members. Public fixture hashes,164byte lengths,fresh identity against6,562prior and all source/input/library/binary/verifier pins are checked. Nextprior6,793. Rechecks are not new exports. Selection is deterministic from the process-latched opt-in flag, with no per-key geometry guard; no per-shape selection trace is claimed.

All210latency processes finish before primary32profiles,3variants199Hz251timed+3primes each, warmFIFO gated, --no-buildid-cache. Aggregate user-cycle categories are descriptive; inlining and scheduling prevent interpreting them as removable stage wall fractions.

|Variant|Samples|Lost|PIDs|Fixed-chain category %|
|---|---:|---:|---:|---:|
|accepted|9743|0|1|0.000|
|off|9672|0|1|0.000|
|on|9292|0|1|0.000|

The absent fixed-chain category does not imply zero execution. Shared and inlined callees can receive its samples. The profile is insufficient to attribute the small request difference exclusively to the new chain. [Physical16 resource observations](artifacts/physical16-observations.json) record CPU and context switches for all 21 physical16 processes; these counters alone do not identify the cause of their control instability.

All helper builds and hashes complete before capture or after all latency/profiles. No owned build or benchmark overlaps timings. Binaries/rawprofiles/largekeys stay local with hashes. No AWS,Aeglos,phone,TEE,giant,swap,pressure or throughput measurement is added. Local pass alone does not promote the source; remaining resource/compiler/consumer evidence must be assessed. Latest [C7a comparison](../2026-10-26-g2-tree-sdk/README.md) is unchanged.

[Design](artifacts/design.md),[implementation audit](artifacts/implementation-audit.json),[analysis](artifacts/analysis.json),[rows](artifacts/performance/rows.json),[proof inventory](artifacts/proof-inventory.json),[artifact index](artifacts/artifact-index.json).
