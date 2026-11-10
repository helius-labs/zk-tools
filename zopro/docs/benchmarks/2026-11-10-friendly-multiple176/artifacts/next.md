# Continue full CPU AVX optimization

Goal remains verified single-proof CPUAVX>=10x suppliedM5CPU for ALL8, with CPU
and Metal comparisons. Active/unmet0/8. No questions, agents or Python. Previous
turn progressed with qualified correction-mask native/query regression, pushed
cf90cfc662. This turn adds distinct friendly-multiple exact arithmetic, explicit
complete-recipe work accounting and an independent small-radix reader, closes
the fixed recipes and pushes the report. No speedup/native/proof result follows.

Root /home/ubuntu/src/zopro-perf-work/friendly-multiple176.
Read analysis.json,review.json,design.md and the report. Four serialized commands
compile-model/model/compile-review/review all succeed first execution; one unused
Field import warning retained.187pins/176production hashes unchanged. No live
owned jobs/cloud; no previous recording/model/proof is replayed.

Fixed recipes CLOSED: M=k*p with low52bit digit+1or-1; both BN254 primes and both
smallest positive residues k. kFq+=3932390912793719,Fq-=571208714576777;
kFr+=3952108915130369,Fr-=551490712240127. Mbits306/303 respectively. A multiplier
fitting five radix52 limbs is at most84, so neither sign can stay within five
limbs. All larger congruent multipliers are larger still. This is not an RNS basis.

Cheap quotientdigit=-low for+1,low for-1, with exact shift/cancellation. ModulusM
is composite; all field zero tests/inversion bridges must still use projection
modulo p. R260canonical-state MUST project after each product because M>R260
and the wide result cannot simply reenter a five-limb product. Charge10IFMAs for
one52bit quotient timesp; quotient estimation and tag conversion are priced ZERO.
Recipe110/160vsbaseline100/150 ordinary/fused. R312wide-state uses six-input-
limb products and six cancellation rows,132/204, with stable exact output<2M.
It adds one vector word per internal value, before unpriced conversions/bridges.
Neither recipe removes modeled work. No subset/radix/quotient/table rescue.
These are fixed schoolbook-tape IFMA counts, not universal lower bounds, a full
dependency schedule or a theorem that a higher-count implementation cannot win.

123392new BigUint cheap-row/full-quotient/Arkworks-projection output checks and
678656row cancellations pass across both fields/signs/recipes, inclusive lazy
edges, all8^4fused corner tuples,2048randomtuples*3ops and128eight-stepchains per
context. Counts include repeated values, not unique field tuples. Separate u128
reader checks122140reduced-radix instances/305350rows with brute-force inverses,
odd moduli3..31,radix16,2/3rows,both signs; recomputes8workscreen decisions.
It does not replay the large Arkworks model or verify native digit headroom.

Current accepted176source/SDK unchanged:
/home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON
SHA7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01.
Local32fullproofCPUms7.829285503/10.979334806/15.803968871/23.944016903/
33.703579161/39.858478065/66.186709387/65.489902226;primaryRSS133.50MiB.
16workerprimary9.192236934ms;merge54=162.541995323ms(noM5reference).
Earlier7.803403940ms SAME executable. No new AWS observation.
Canonical43730 MUST seed future verified exports:
/home/ubuntu/src/zopro-perf-work/large-class-sdk176/proof-sha256-after-capture.json.

Targeted checks this turn also found paired/triple/full-quotient Montgomery,
parallel carry normalization and first48 recipes already covered. SolverExprSum
producer emits at most two direct terms; coefficient-2 specialization already
has a tiny12record-primary census. Across-hash Poseidon, scalar fused-dot/direct
dot/three-product and linear-eight variants have existing results. Do not repeat
those or use isolated throughput as proof-latency evidence. RNS/MQX literature
was only surveyed; no external implementation or performance claim is adopted.

Autoresearch loop:
1.Revalidate actual current state and exact live handle; never restart terminal
  captures/old extraction or replay successful work for bookkeeping.
2.Choose distinct bulk work/dependency reduction, targeted relevant history only.
  Price all work required to obtain fewer products/shorter scalars/new radix,
  including canonical projection/import/export/zero checks/inversion bridges.
  Establish actual use before another cold-path optimization. Avoid another broad
  history inventory, repeated mnemonic census or known closed candidate.
3.Private reversible Rust1.98.1 prototype; independent exact references, native+
  generic, checked admission/originalfallback,small-stack/shared-pool callers.
  No giant LUT,expanded keysolver plan,newpool,architecturalRAMcap,C/ASM/Python.
4.Freeze sources/compiler/deps/binaries/inputs/order/gates/analysis/reader before
  any preflight. During ANYpreflight/recording ONLYpoll confirmedlivehandle to
  terminal; no helper/build/hash/report/push overlap. Timeout isn'tterminal.
5.Keep every failure/positive/uncertain estimate. No minimum gain floor, but actual
  calibrated complete callers must establish improvement. No posthoc scope,
  threshold, size/worker subset or successful rerun to rescue a fixed failure.
6.Positive callers permit ALL8+54 fullSDK/Gnark/resource/portable/concurrency
  qualification; only final-artifact proofs permit adoption/completion. Goal
  never becomes one component, favorable shapes, correctness or throughput.
7.CommitANDpush completed experiments, verify exactremoteHEAD, updatehandoff/
  latest-status and continue autonomously. No complete/block status merely for
  difficulty, time spent or negative results. Goal stays active.
