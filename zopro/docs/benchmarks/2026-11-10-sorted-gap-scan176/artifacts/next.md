# Continue CPU AVX prover research

Goal stays active/unmet: verified single-proof CPU AVX >=10x supplied M5 CPU
for ALL8, with CPU and Metal comparisons.0/8 targets. No budget limit or genuine
external blocker is asserted. Previous goal turn was progress: qualified reverse
factor lookahead and a complete real-query/profile comparison, rejected and pushed.
This goal turn is progress: a distinct sorted-gap/suffix algorithm, actual curve
correctness, complete work/memory accounting, independent scalar/work readers,
negative selection and report/push. Do not reclassify these as speedups.

Current root: /home/ubuntu/src/zopro-perf-work/sorted-gap-scan176.
Read analysis.json, review.json, scalar-audit.json, design.md, decision.json.
Original6-command workflow26651terminal0; extra2-command scalar audit terminal0.
No failed builds, repairs, old solver/FFT extraction, successful model replay,
native kernel, timing, new performance profiles, SDK calls, proof exports or cloud.

Fixed recipe CLOSED: after original grouping/Phi split, sort each signed component
by magnitude, replace coefficients by adjacent gaps and points by signed suffix
sums. Fixed Brent-Kung affine scan, width12,parts2,window count from maxgap+carry.
Not input-order difference basis or disjoint-neighbor Bos-Coster. Do not rescue
by switching scan family, width, partitions, sort thresholds or favorable shape.

Algebra correct:114089actual scan outputs match independent sequential suffix
references;4new component Arkworks MSMs combine to the2retained full-query
oracle hashes.117882signed components/58941original grouped-or-raw scalars reuse
the qualified Phi split and check reconstruction in independent Arkworks Fr.
Two new unit tests cover175ragged signed scan cases and288telescoping identities.
Separate recursive weighted reader reproduces42window work rows andall6gates;
additional bitwise scalar reader reproduces2396202digits/227832coefficient
reconstructions, sort/sign hashes, gaps, bit bounds andall42occupancies. It reuses
the Phi helper and inherits zero-identity facts from the curve execution; no new
curve scan/MSM replay. The model is not a universal native cost lower bound.

Both components' largest gaps are115bits,vsoriginal126;11windows become10.
Primary arrivals613891->519691, but scan adds112758joins:
generic total634486->651020 (+2.6059%).2x2arrivals633695->572960,
scan115330;generic total674949->727518 (+7.7886%).Weighted/Horner active doubles
846->763 inboth.2of6necessarychecksfail(feweradds,bothshapes). No native trial.
New logical native payload12182832/12460392bytes plus unchanged scalar words
962848/923264. These are model payloads, not allocation/RSS/minRAM bounds.
Sorting, queue/exception/cancellation/inversion/batching/packing/normalization/
scheduling/cache costs remain unpriced. Offline Ark projectives are oracle storage,
not the proposed native affine layout. No persistent key table or newpool.

Accepted source176/field1200/SDK remain unchanged. SDK path:
/home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON
SHA7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01.
Latest local32 full-proof CPUms ALL8:
7.829285503/10.979334806/15.803968871/23.944016903/33.703579161/
39.858478065/66.186709387/65.489902226. PrimaryRSS133.50MiB;
primary16=9.192236934ms;merge54=162.541995323ms(noM5reference).
Earlier7.803403940ms is the same accepted executable in another campaign.
Canonical43730 ledger (seed this exact file for future new exports):
/home/ubuntu/src/zopro-perf-work/large-class-sdk176/proof-sha256-after-capture.json.

Autoresearch loop:
1. Revalidate current authoritative files and exact live handle if any. Noneowned
   after this checkpoint. Do not restart completed captures or old extraction.
2. Choose a distinct bulk arithmetic/dependency removal; check only its relevant
   prior result. Avoid another broad history survey. Direct signed REDC, two-/four-
   chain inversion, lookahead, compact lazyY, paired/gather packing, active descriptor
   scans, indexedwide, sparseXYZZ, class splitting, GLV/KH and this scan remain closed.
3. One useful diagnostic direction is the actual hot G2 consumer's register/stack
   traffic using already retained full-query profiles and exact final binaries,
   rather than another standalone-field count. Inspect source coordinate lifetimes
   and actual emitted instructions before proposing a layout or inline boundary.
   Earlier phase-split/staged-operator/layout negatives still require a distinct
   mechanism; do not merely repeat them. Flat samples are not stall causality.
4. Prototype privately with reversible source, independent mathematical/output
   checks, checked memory admission/fallback, generic execution and real512KiB
   workers/concurrent callers. No giant LUT, expanded keyplan, architecturalRAMcap,
   productionpool, C/ASM rewrite, Python or unasked delegated agent.
5. Freeze compiler/source/libs/binary/oracles/environment/order/gates/fullanalysis/
   separate reader BEFORE preflight. Use Rust1.98.1/serializedxtask. During ANY
   preflight/recording ONLYpoll confirmedlivehandle through terminal; no helper,
   build, hash, decode, analysis, report or push overlap. Timeout/truncation is not
   terminal. Preserve all positive/adverse results and failures. No minimum gain
   floor, but require real complete-caller evidence against calibrated controls.
6. Failure closes fixed recipe. Success only permits ALL8+54 full SDK/Gnark/resource
   validation and final-artifact verification before adoption. Full goal is not a
   component, favorable subset, throughput or correctness result.
7. CommitANDpush each completed experiment, verify exactremoteHEAD, updatehandoff/
   latest-status, continue without questions. Goal staysactive until all8 CPU10x
   targets are actually verified and both CPU/Metal comparisons reported.
