# Research and history audit,28October2026

Read the abstract/introduction and sections6.2/6.3 of
[Didier et al.,arXiv2410.18129v1](https://arxiv.org/html/2410.18129v1).
The work's sub-quadratic multiplication results concern larger operands, and
its small-size discussion favors schoolbook arithmetic. It does not establish
a BN254/prover gain or justify replacing the accepted field kernel. The older
radix51/PMNS/shared-product trials already cover related arithmetic leads.

Read the multiple-guard contract in the official
[Crossbeam guard documentation](https://docs.rs/crossbeam-epoch/latest/src/crossbeam_epoch/guard.rs.html)
and the actual locked0.9.21/0.8.8/1.13.0 epoch/deque/Rayon sources locally.
The reentrant guard mechanism is real, but bounded shared pinning was already
measured in25October's batched-steal report and failed its SDK gate.26October's
shared-front fence and27October's single epoch advancer are also closed. Do not
repeat them or interpret sampled epoch CPU as removable wall latency.

Read the current g2_phi and g2_borrowed source,22OctoberG1Horner,24OctoberG2
transformed-X interleaving/guarded interleaving and26OctoberG2level-tree reports.
The chosen independent adaptation combines borrowed pending work across
separate high/low BUCKET planes, preserves original points, and applies phi
after accumulation. It does not copy a transformed-key cache or level tree.
No paper speed percentage or external implementation is transferred.

The first source draft admitted each leaf separately. Source review found
Reservation::analysis reads OS headroom under the shared admission lock, so
the final candidate reserves all potential leaves once per query. This change
was made BEFORE ANY performance capture; native source validation is repeated
because two parent files/resource lifetime changed. The first draft's full
source/build receipts are retained in pre-query-admission-v1, with zero SDK
performance captures or proof exports. Qualified measurements use only the
query-admission revision. No claim of new independent test fixtures follows
from running existing oracle fixtures against revised code.
