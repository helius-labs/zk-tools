# Block-local G2 gap factoring: fixed algebra and work screen

The original goal remains verified CPU AVX single-proof latency at least 10x
the supplied M5 CPU timings for ALL8, with CPU and Metal comparisons. Current
accepted local primary is 7.772288 ms at 32 workers; targets met remain 0/8.

For each original signed Phi component, retain the qualified grouping and split,
omit zero/identity terms, and sort by magnitude with original-index ties. Partition
this ordered sequence into fixed blocks of 256 records. This geometry is fixed
before new counting, matching the existing weighted reducer's 256-point group;
no block-size, width, partition, scan-family or shape sweep is authorized here.

Within a block [b,e], let U_j = sum(i=j..e) Q_i for signed input points Q_i.
Use coefficient m_b for U_b and m_j-m_(j-1) for U_j, j>b. Thus
sum(i=b..e) m_i Q_i = m_b U_b + sum(j=b+1..e) (m_j-m_(j-1)) U_j.
The high-magnitude block-base terms are compulsory corrections. Feed corrections
and gap terms into the same ordinary width-12, two-part buckets. Preserve the
full maximum coefficient bit bound and Booth carry; never claim all terms have
short gaps or omit the extra window for corrections.

This is a distinct representation from the closed global suffix-scan recipe:
its U_j stop at block boundaries, and high-bit correction terms replace every
cross-block suffix offset. The closed global scan remains closed. Construct local
suffixes with a linear-work right-to-left chain independently per block. Charge
all nonidentity scan joins, common grouping, fill/part merge, actual occupancy,
weighted reduction and Horner. Also expose scan dependency stages, vector widths
and at least one inverse per nonempty affine stage. A per-stage affine native
implementation is hypothetical; Arkworks projective execution is a correctness
oracle, not timing, field-cost equivalence or a memory measurement.

Necessary screening gates: exact algebra plus retained complete-query curve
oracles; fewer complete generic point-add calls in BOTH retained 1x2 and 2x2
fixtures; no increase in active weighted/Horner doublings; checked proof-local
payload arithmetic. No minimum positive gain floor. Failure closes this fixed
representation before native timing; no unchanged rerun or block-size rescue.
Success permits further native inversion/dependency/resource qualification only,
not adoption or a latency claim. Full SDK ALL8+54 and fresh Gnark qualification
would still be required for any production change.

Model native logical payload: per active component row 128-byte affine suffix
point, 32-byte signed sorting record, 16-byte coefficient, 8-byte index and a
second 32-byte sorting plane, plus unchanged 32-byte scalar words per original
grouped input. Reuse each block's U_b for its correction (no extra point).
This checked payload is not an allocation/RSS/minimum-RAM bound: capacities,
allocator metadata, scratch, original input arrays and overlap are additional.
Any production integration needs checked admission and original fallback before
allocation. No persistent key/LUT/solver expansion, new pool or artificial RAM cap.

Freeze helpers, fixed design, inputs and separate reader before execution. Reuse
retained fixtures, exact Phi/grouping/Booth/weighted helpers and complete-query
oracle hashes; do not replay old solver/FFT/scan/oracle production for bookkeeping.
New work comprises the local factored transform, four component Arkworks MSMs,
integer identities and independent scalar/weighted-work reconstruction. No
native speed, new SDK proof, Aeglos/AWS, mobile or pressure claim. Canonical
inventory remains 45,335; any future export must seed retained-tail-sdk176's
proof-sha256-after-capture.json. Rust 1.98.1 and serialized xtask. Commit and push
the completed experiment, preserve all failures, leave the full goal active.
