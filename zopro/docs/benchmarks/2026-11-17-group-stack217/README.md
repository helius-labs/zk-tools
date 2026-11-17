# G2 grouping stack addresses217

Both call paths have the same32/64-byte stack-spill alignment in this debugger
audit. The simple explanation that one path's32-byte temporary crosses a cache
line and the other's does not is unsupported here. This is an address audit;
it does not measure or explain the prior216 CPU-time gap.

|Property|Boolean OFF|Boolean ON|
|:---|:---|:---|
|Calls observed|4|4|
|Frame RSP modulo64|0 in4/4|0 in4/4|
|Scalar spill address modulo64|48 in4/4|48 in4/4|
|Scalar spill address modulo32|16 in4/4|16 in4/4|
|32-byte spill crosses64-byte line|4/4|4/4|
|Return PC, executable virtual offset|0x4773ae|0x4ba8ed|

The exact216 executable is reused without relinking:
`6d9ae8f89e09b4bec8ac7c6a6f795bfbf04594623e4abf39802ada7b4d5d254c`.
GDB stops immediately after the verified six-register push and0x1a8-byte stack
reservation in `scalar_groups`, conditional on30,089 scalar words. The recorded
RSP is the actual post-prologue register, and the spill address adds the0xf0
offset seen in the hot loop. No scalar contents are read or dumped.

Each32-worker process makes three primes and one timed SDK call. GDB pauses
execution and disables address randomization by default, so no timing or
observer conclusion is taken from these calls. The original unpaused216 runs
did not record stack addresses. Different allocation addresses, predictor state
and concurrent worker activity remain possible factors; this audit does not
rule out all memory or execution-context effects. No aligned-stack variant or
offset sweep is justified by this result.

Both captures, both late Gnark verifications and the receipt/address/source
reader pass first execution: eight SDK calls, two fresh exports, four receipts
and eight stream hashes. An initial helper compile error from an ambiguous
JSON conversion was repaired before pins or any SDK execution. No runtime
source, installed library, input or production configuration changes. Protocol
19:29:53UTC to result19:31:22UTC is about89seconds, within15minutes.

Canonical distinct exports are **59,034**:
`/home/ubuntu/src/zopro-perf-work/group-stack217/proof-sha256-after-audit.json`,
SHA256 `0f55f0927fcb0afff0f9b21e23966a1b34679157215a7c947955b7a1a2b83505`.
Raw addresses and complete evidence remain private under that root. The report
contains only alignment classes and executable-relative return offsets. No AWS
was used. Production194/196 stays accepted; the original all-eight CPU10× goal
remains unmet0/8. The separate216 profiles still identify a useful stack-copy/
hashing region, without establishing a precise hardware cause.
