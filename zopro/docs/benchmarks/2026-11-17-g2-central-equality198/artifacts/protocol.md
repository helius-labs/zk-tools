# Central G2 denominator validation198

Assumption challenged: removing scalar equal-X checks requires a new vector
classification pass. Original ordinary G2 filling reads finite bucket and input
X before admission; the norm pass reads both again. Use zero of the existing
denominator product to reject an entire exceptional batch before any bucket write.
The norm of dx is zero iff dx is zero in Fq2. The existing scalar field inverse
already returns Option; preserve that result instead of unwrapping it.

Prediction: remove per-arrival scalar X equality without adding per-vector masks,
status arrays, field products or point copies. The October31G1 central-check and
November10G2 packed-filter failures remain closed. This G2 experiment uses the
same existing prefix record/kernel operations, differs from G1's forward
numerator records and from G2's added vector filtering, and charges the fallible
control flow. The latter filter added instructions/cycles despite removing scalar
checks; this implementation directly removes its classification work.

Cheapest discriminating test: original mathematics and focused exceptional-batch
tests, then a complete G2 query screen, not an arithmetic microbenchmark. Current
179-source production baseline; one shared OFF/ON native executable. Two retained
real queries at1/16/32workers. Three independent byte-identical backings,three
rotated/reversed blocks,3primes+17timed+1postcheck/process;12one-round preflights
precede36mainprocesses. Scalar copy,grouping,all allocation/filling/reduction and
affine normalization timed; input loading,pools,oracle hashes excluded. All outputs
must match retained independent Arkworks hashes and points stay immutable.

Memory/setup: no extra vector/table/point buffer/pool; one Window mode bool and
fallible inverse/branch. Zero batches pay speculative prefix work plus complete
scalar additions to every distinct destination. Pathological valid inputs can
therefore cost more CPU. No constant-time or adversarial throughput claim.

Directional advance if primary32 improves in all3blocks and every other context
paired median<=1.02. No minimum gain floor,confidence claim,sample extension,
discard,geometry/layout sweep or favorable subset promotion. A survivor needs
exact full-SDK screen,profiles and all-shape/resource/portable qualification before
adoption. A negative closes this exact G2 recipe. No new AWS resource or push.
Preparation+capture allowance30min from protocol creation; state and explain an
overrun without changing samples or result. Existing successful tests/fixtures
are reused; no new proof export or ledger increment is claimed for query checks.
