# Distinct grouping builder

Current accepted grouping performs a sampled admission, builds an owned-key
HashMap, then either declines or returns stable scalar classes. The recent
accepted175 preparation observer measured1.307ms median G2 grouping CPU and
full-map decline. That is combined service, not removable proof latency.

Existing outcomes remain closed:

-25October borrowed-group-keys replaces40-byte map entries by16-byte references,
  retaining hashing/probing and loses the complete SDK gate.
-26October bitmap lower bound/isolated grouping uses a separate exact occupied
  bitmap precheck; fails its declared gate/calibration. It is not included.
-27October prefix cutoff stops the existing map on a proven unique excess;
  its full gate fails. No standalone old cutoff recipe is repeated.
-27October parallel scalar groups shards eight exact maps, reconstructs stable
  output, and regresses larger proofs. No shard or parallel grouping exists here.
-25October bulk sorted Bos-Coster repeatedly sorts/subtracts mutable full
  scalars to replace MSM itself. This candidate instead performs one four-byte
  fingerprint radix sort solely to produce unchanged grouping metadata. It
  performs no Euclidean scalar subtraction or curve-kernel substitution.
-28October B2/global grouping bypass regress. Current sample/decision/resource
  cutoffs are retained, and no circuit/shape bypass is introduced.

Retained README/design search found no equivalent compact-fingerprint radix
grouping builder. No novelty claim. This is local Rust code without an outside
implementation or new crate. Fingerprint collisions use exact full-word sorting,
so random hashing affects work only, never equality or admission correctness.
The previous direct inverse-finish SDK pilot (901bfe191e) remains rejected and
production source175/SDK20fde remains unchanged.
