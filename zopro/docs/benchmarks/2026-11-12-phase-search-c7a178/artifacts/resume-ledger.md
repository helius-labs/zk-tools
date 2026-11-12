# Host ledger finalization after C7a retrieval

Subsequent local scaling measurements advanced the canonical ledger to 55,085.
The target package remains frozen at its original 55,028 seed. Its existing
postprocessors intentionally produce a target-local branch ending at 55,152.
Preserve those files and image pins; do not rebuild or replay local preflights.

After transport verification and both existing postprocessors complete, run the
new host finalizer. It checks original prepared pins, postprocessor ledger hashes,
all 124 local Gnark command receipts and proof/public/image bindings. Its target
delta must equal the independently verified transport set and be disjoint from
the current canonical ledger. The canonical input must include the known 55,085
local ledger, preventing accidental reuse of the package seed as current state.
Performance/profile outcomes are reported separately and cannot authorize adoption.

With no intervening exports, the invocation is:

```sh
/home/ubuntu/src/zopro-perf-work/c7a-ledger-merge179/finalize \
  /home/ubuntu/src/zopro-perf-work/accepted-scaling179/proof-sha256-after-stat.json \
  946f2b40315004bed6c4fe61f84b8d5f70fff27bb0c206924e58db0a055207f2 \
  /home/ubuntu/src/zopro-perf-work/phase-search-c7a178/transport-run \
  /home/ubuntu/src/zopro-perf-work/phase-search-c7a178/canonical-final
```

Use the actual transport workflow output directory if different. If local work
creates later exports, supply its latest canonical path and hash instead. The
output directory must be new; existing ledgers are never overwritten. Expected
merged count without intervening exports: 55,209. Promote the produced path/hash
only after `review.json` passes. This finalizer does not generate proofs or rerun
verifiers.

Five unit tests pass for divergent histories, reuse of later local exports,
missing common ancestors, mismatched verifier sets/counts and malformed or
duplicate ledger entries. The optimized host executable compiles. Actual target
receipt integration is untested because AWS has not run. The first test binary
was compiled before adding the canonical-floor check but never executed; only
the final source's five tests ran. No benchmark or cryptographic verification
was repeated. Source: [finalizer](finalize-current-ledger.rs),
[test output](finalize-ledger-tests.txt).
