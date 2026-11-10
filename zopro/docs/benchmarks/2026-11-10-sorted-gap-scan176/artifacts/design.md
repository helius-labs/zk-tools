# Sorted G2 component gaps and suffix-point scans

Full goal remains verified single-proof CPU AVX >=10x supplied M5 CPU ALL8,
with CPU and Metal comparisons. Accepted primary7.829285503ms/133.50MiB,
targets0/8. A passing algebra/work screen is not a speedup or adoption.

For each existing signed Phi component, discard zero/identity terms, sort by
unsigned magnitude with original-index ties, and form adjacent gaps d_j=m_j-m_(j-1)
(m_-1=0). Let S_j=sum_(i>=j) sign_i P_i. Then sum m_i sign_i P_i=sum d_j S_j.
Drop zero gaps and identity S_j. High-component Phi still acts linearly after
its MSM. Input scalar grouping and the exact current Phi split are inherited.
Widths stay12, parts2; windows derive from the actual maximum gap bitlength
plus carry. No window/batch/partition or scan-family sweep is selected.

This differs from the closed adjacent-input difference basis (which changes
neighbor scalars in input order) and disjoint-neighbor Bos-Coster subtraction
(which updates one paired point per round). Here every coefficient is a sorted
adjacent gap and its point is an inclusive signed suffix. No heap-priority
iteration, permanent key rewrite, lookup table, solver plan or new pool.

Count a fixed in-place Brent-Kung inclusive scan of the reversed signed points:
upsweep at power-of-two strides and downsweep correction. Charge every real
nonidentity/nonidentity scan join. Planned native scan uses batched affine
updates per dependency level and existing pool/scratch; it is not implemented.
Arkworks projective scan here is an offline correctness oracle, not a native
timing or the proposed memory layout. Compare every scan output with independent
sequential suffix sums before collecting gap points. Check the new complete
Arkworks MSM against retained independent full-query oracle hashes, without
replaying the old oracle producer or solver/FFT extraction.

Reproduce original generic noncancelling point-work ledger and candidate ledger
including scan and common grouping joins, ordinary fill/part merges, actual
bucket occupancy, original weighted reduction and Horner. Native queue spill,
exception/doubling/cancellation, inversion/batching, sorting time, normalization,
packing, allocation, scheduling and cache costs remain unpriced. Known identity
inputs/derived scan outputs are filtered exactly. This is a generic work screen,
not a lower bound or prediction of complete latency. Original weighted counter
is reused; a separate recursive reader must reproduce its counts.

Two fixed retained actual B fixtures:1x2 and2x2. Necessary selection requires
all algebra/reference checks, strictly fewer modeled complete point-add calls
in BOTH shapes, no greater active weighted/Horner doublings, and checked bounded
proof-local payload accounting. No minimum gain floor. Failure closes this fixed
recipe before native/timing; no favorable shape, altered scan, threshold, width,
parts or sorting family rescue. Success only permits native resource/correctness
qualification, followed by frozen complete caller controls and ALL8+54 SDK/Gnark
validation. The full objective is not reduced to these two screen fixtures.

Model new proof-local native payload, not RSS/minRAM:two affine component scan
arrays128B/row, signed sort records32B/row, gaps16B/row, indices8B/row and a full
second sort-record plane32B/row, plus original mutable words32B/input. Explicitly
separate unchanged input words and new arrays; headers, scratch, suspended actor
frames, sorter allocation and actual overlap remain for native qualification.
Any production integration needs admission/fallback before allocation. No
architectural RAM cap or persistent expanded key is introduced by this model.

Rust1.98.1 and serializedxtask. Freeze sources/inputs/helpers/reviewer before
execution; preserve all failures. No new CPU timing, performance profile, native
AVX kernel, SDK request, proof export or cloud resource. Canonical43730 unchanged.
CommitANDpush a factual report and next-agent loop. No questions or delegation.
