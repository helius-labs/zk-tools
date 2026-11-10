# Research and prior-result check

Primary pages read on 2026-11-10:

- [Zhang, Fu, Franchetti, MICRO 2025](https://arxiv.org/html/2509.12494v1).
  The paper's arithmetic uses moduli at most124bits in its128bit FHE setting.
  MQX is a proposed instruction extension with projected performance. Neither
  its hardware projection nor its baseline speedup establishes an improvement
  for existing BN254 IFMA arithmetic. No unsupported instruction is used here.
- [SimdMSM author repository](https://github.com/JR-account/SimdMSM).
  Its paired/projective mechanisms already have the local2026-11-06 source
  audit. No repository performance claim or C implementation is imported.
- [Hrabar thesis primary record](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403).
  Scalar factor/clustering methods are already represented by local factor
  recoding and partial-window studies; they are not rediscovered here.

Relevant local negative results remain closed: direct signed difference175,
signed G2 caller175, two-chain G2 streaming176, four-chain phased inverse174,
descriptor chunks176, and unfinished-class scanning176. The signed G2 caller
already regressed with32logicalworkers, so the earlier affine-expression gain
is not promoted. This new implementation does not use its signed REDC helper.

The new source schedule has one prefix chain and computes exactly the same
factor and inverse updates in the same j order, moving next-group preparation
before current point finishing. It does not add3root-combination products,
materialize inverse norms, add scratch passes, or alter field arithmetic. The
possible compiler/hardware overlap is a hypothesis, not a mathematical product
count saving or a prediction that proof latency can halve. New native and real
complete-caller evidence is required. All-eight proof targets remain the goal.
