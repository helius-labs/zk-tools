# Follow-up reading during the fixed run

These sources were reviewed after the experiment declaration and did not
change its source, thresholds, ordering or gate. No new performance result
is inferred from them.

- [Heartbeat Scheduling: Provable Efficiency for Nested Parallelism](https://www.andrew.cmu.edu/user/mrainey/papers/heartbeat.pdf)
  promotes an older eligible frame after useful work has accumulated. Its
  efficiency argument depends on that choice and on representing promotable
  frames. A counter that simply skips most Rayon forks would not inherit
  that argument. Application to this prover would require a separate design
  and correctness evaluation; the reference is not a request for a C++ rewrite.
- [A NUMA-Aware Provably-Efficient Task-Parallel Platform Based on the Work-First Principle](https://arxiv.org/html/1806.11128v1)
  biases victim selection and performs bounded lazy work pushing on selected
  scheduling paths. For Zopro, locality and task placement are hypotheses to
  investigate after topology and existing experiments are checked. The paper
  does not establish that Zopro's current bottleneck is NUMA or memory bandwidth.
- [Computing multi-scalar multiplication on memory-constrained devices](https://eprint.iacr.org/2026/1316)
  describes adapting the bucket count to available memory. The
  [publisher abstract](https://link.springer.com/article/10.1007/s13389-026-00393-z)
  reports BLS12-381 experiments with 256–16,384 points. This may inform a
  memory policy, but supplies no measured BN254 or Zopro speedup. The authors'
  page links to this ePrint; its PDF returned HTTP403, so only the public
  abstracts were reviewed and no algorithm implementation is claimed.
