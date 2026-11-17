# Unshifted borrowed G2 tree on C7a

Question: does the existing unshifted balanced bucket tree's additional independent
256-bucket tasks overcome its locally measured extra CPU cost on C7a's 96 physical
cores? This is a new host test, not a new algorithm or a reopened local success.
The source tree body is reused byte-exact from bucket-round178 staged_tree.rs.
It uses proof-local descriptors and two intermediate point arrays; no persistent
shifted table, key copy, second solver plan, new pool or production edit.

Rebase onto all 179 exact current production files. Only the G2 phi parent gains
explicit experimental dispatch, with the current streaming body preserved. Add
one exact historical child and a test-only caller. Inherit unchanged field and
historical tree/sign/padding qualification; run new complete-query retained
Arkworks oracles plus synthetic generator/exception cases on this integration.
Conservative original tree memory admission remains; verify selection per query.
No all-shape release, portability, pressure or minimum-RAM claim at this stage.

Use one c7a.24xlarge, 96 physical cores/no SMT, fixed workers 32 and 96; shapes
1x2 and grouped 2x2. Native Rust1.98.1/znver4 optimized core tests. Two runtime
arms in one consumer: 0=current streaming, 1=unshifted balanced tree. Three
byte-identical executable backings with separate inodes. Eight functional remote
preflights, then 72 main processes in nine rotated/reversed blocks, three blocks
per backing. Each process: 3 primes, 17 timed queries, one postcheck. Include word
copy, grouping, split, routing, intermediate storage, fills, merges, weighting and
normalization in timing. Oracle hashing outside time but within CPU usage. Load
outside time; process RSS includes load and warmup. All outputs match independent
retained oracles; input points unchanged. No SDK proofs or new proof ledger.

Report nine paired ratios and medians, all copies and CPU/RSS. Progress to exact
full-SDK work only if primary96 improves in >=7/9 blocks and its median ratio<1,
AND best tree latency across32/96 improves over best streaming in >=7/9 blocks
with median ratio<1, AND grouped best latency median ratio<=1.02. This directional
screen has no gain floor and is not release qualification. Retain all samples;
no extension, copy/worker/shape selection or parameter sweep after results.

First preparation/build/remote result allowance:20minutes after protocol creation.
If exceeded record cause before continuing. Reuse Rust/xtask runners. No owned
build/analysis overlaps capture; only poll live handles while measuring. One VM
only, restricted existing network/SSM role, 90min terminate TTL. Validate auth
lifetime, retrieve results then terminate/verify volume deletion; delete temporary
private transfer bucket. No ECR needed. No GitHub push without new authorization.
Current194 SDK remains adopted; all CPU10x goal targets0/8 active. This tests a
complete-query mechanism, not exact accepted SDK latency or an Aeglos comparison.
