# Continue the full CPU AVX optimization goal

Goal remains verified single-proof CPU AVX >=10x supplied M5 CPU for ALL8,
reporting both CPU and Metal. Active/unmet0/8. No questions, subagents, Python,
giant lookup/expanded key/solver plans, new production pool, architectural RAM
cap or C/assembly rewrite. Rust1.98.1 and serialized xtask. Commit AND push each
completed experiment and verify exact remote HEAD.

Fixed canonical-front-overlap candidate CLOSED at complete-SDK measurement.
It overlaps public correction, in-place canonical conversion and A/B scalar
gather with the independent quotient product in the existing Rayon pool. The
main A/K/H/B2 query phase still follows the join. Important scope clarification:
public_term itself does existing point multiplication and can fall back to
msm_g1; that correction work/temporary lifetime DOES overlap the FFT. Initial
design's "allMSMs" language means the main query phase, not public correction.
No new owned vector or changed point formula. One original lib.rs reverses;
other175 originals and field1200 exact. Native127tests and all9 SDK proofs pass.

Primary32 real positive: REF/OFF/ON=7.783593581/7.776514935/7.747712323ms.
ON/OFF0.993080991[0.989961540,0.997861556],12/15wins; ON/REF0.992859331
[0.989282098,0.999881178],12/15. Both primary gain checks PASS.2x2 also improves
against both controls: ON/OFF0.989169486[0.980046558,0.995873627],12/15;
ON/REF0.982817768[0.981509625,0.994661724],14/15. Preserve these positives.

Full gate FAIL4of77:8x2 ON/control nonregression and bothALL8gain checks.
ALL8ON/OFF1.000327460[0.996617963,1.002950951],7/15wins;ON/REF0.997632232
[0.993294681,1.000106512],10/15.8x2 medians nearparity but upper bounds1.033588/
1.038292 fail2%nonregression; lower bounds<1, so not a demonstrated3.8%slowdown.
All11 calibrations,22RSSguards,18diagnosticquality checks PASS. No gain floor,
extra blocks, exclusions, shape/worker rescue or same-candidate recapture.

Capture1071commands=495latency+18diagnostics+45offline+513Gnark, allsuccess.
15matchedblocks*31queries/process onALL8+54w32+primary1/16.19,422capture SDK
requests plus88functionalpreflight =19,510internallyverifiedrequests.535fresh
unique exports independently pass Gnark,22preflight+513capture. Canonical
43730→43752→44265. All9profiles qualify,118916samples. Final423pins unchanged;
frozen analyzer and separate raw reader pass firsttry. Reader independently
reconstructs raw integer means/rank ratios/all77gates; SDK/proof/diagnostic
parsers shared. One helper compilation failure for duplicate median/missing
demangler retained;3repairedhelperbuildsPASS.10native/downstreambuildsPASS.
No native, preflight, recording, proof or successful scientific stage repeated.

CANONICAL NOW44265 MUST seed new exports:
/home/ubuntu/src/zopro-perf-work/frontend-overlap176/proof-sha256-after-capture.json.
Accepted SDK remains busy-reset-sdk176/sdk-ON,
SHA7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01.
Fresh accepted local32 fullproof ms:
7.783593581/10.932730677/15.757088065/23.973294548/33.827268968/
39.900916645/66.041082710/65.717993516. Latestprimarypeak111.25MiB in this
31request/process campaign; earlier7.829285503ms/133.50MiB and7.803403940ms
are SAME accepted executable in other campaigns, not replaced regressions or
proven memory savings. Current accepted16=9.016864226ms;solo94.302385516ms;
merge54=162.170157871ms. Candidateprimary111MiB. No new AWS/Aeglos result.

Next needs a distinct bulk work/dependency reduction, not favorable-size
selection of this fixed overlap. This result preserves a real small-primary
benefit but establishes no overallALL8gain or2xjump. Review actual source and
dominant complete-MSM profiles before choosing a new arithmetic/topology.
Old four/five-query overlap and solver-A actors are different, closed schedules.
Wide approximate62, affine offsets, numerator fusion, ring-affine, scalar reuse,
polarized FFT, GLV/GLS, common-Z and lane-stable buckets have recorded results;
do not rebrand/replay them without a materially different complete mechanism.

Autoresearch loop:
1.Revalidate source/accepted artifacts/live handles; classify previous turn.
  Timeout/truncation is not terminal. Never restart a successful capture.
2.Select a distinct reduction in relevant complete work or dependencies; price
  packing, coordinate conversion, normalization, inverses, memory and scheduling.
3.Private reversible Rust prototype; independent exact references, native and
  generic admission/fallback, small-stack/shared-pool/concurrency qualification.
4.Freeze source/compiler/deps/binaries/inputs/order/gates/analysis/raw reader
  BEFOREpreflight. During ANYpreflight/recording ONLYpoll its exact confirmed
  live handle to terminal; no overlapping helper/build/hash/analyze/report/push.
5.Retain every failed/positive/uncertain result. No minimum percentage floor,
  but complete calibrated final callers must establish improvement. No posthoc
  sample extension, subset, threshold, layout/compiler/width rescue or reroll.
6.Positive callers permit ALL8+54 SDK/Gnark/resource/portable/concurrency work;
  final-artifact verified proof latency alone supports adoption/completion.
  Preserve fullALL8>=10xCPU objective and CPU/Metal comparisons.
7.Publish factual polite results; commit AND push; exactremoteverify; update
  latest-status/handoff. Continue despite negative results; no complete/block
  status from difficulty or time spent. No owned jobs/cloud remain at close.
