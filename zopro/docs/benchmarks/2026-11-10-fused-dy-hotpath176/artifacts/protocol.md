# Offline comparison of the existing G2 profiles

The goal remains verified CPU AVX single-proof latency >=10x supplied M5 CPU
for all eight shapes, with CPU and Metal comparisons. No shape currently meets
its CPU target. Accepted local32-worker 1x2 remains7.783593581ms. Previous goal
turn PROGRESS: distinct incoming-quarter fusion qualified, measured, rejected
FAIL7/48 and pushed17c33c001d. No inherited live job or cloud.

This stage adds no recording, timing, native candidate or proof. Reuse exactly
the OFF and ON32-worker G2 cycle profiles from fused-incoming-dy176. The profile
and grouped-counter observations are separate panels. Every input, executable,
library and existing profile payload must match the published retained hashes.
Keep all original latency/counter/quality decisions; do not reinterpret the
closed recipe as accepted or rerun it unchanged.

Generate exact G2Scratch::apply disassembly, ELF segment listing, nm bounds and
perf annotate for both executables using a serialized xtask workflow. Preserve
full commands/output/status/hash receipts. A Rust analyzer independently maps
sample IPs: parse ELF64 program headers; find executable PT_LOAD matching the
recorded MMAP2 page offset; derive runtime bias from its virtual address (not
file offset); check PID, mapped binary, executable range, symbol bounds and
instruction boundaries. Cross-check nm/readelf metadata and perf annotate
function sample totals and every annotated instruction count if supported.

Report static function size, prologue stack reservation, mnemonic counts,
syntactic rsp-relative accesses and sampled instruction classes/hot addresses.
An rsp reference does not necessarily imply a compiler spill. Non-rsp references
are not assumed to be key/heap accesses. PMU cycle samples may skid and do not
measure exclusive instruction latency, cache misses, memory stalls or execution
port utilization. No causal conclusion from a sampled address or static count.
Do not claim statistical certainty for the difference between two profile panels.

Choose a next falsifiable mechanism only after inspecting this evidence. A
potential hypothesis is removing the general vpmullq used for the four-valued
quarter correction by selecting tiny compile-time field constants with masks.
This is not an indexed LUT, new allocation, changed key/solver plan or a claim
that integer multiply is the bottleneck. Avoid an instruction tweak unsupported
by the full caller and profiling. Any implementation must receive fresh numeric
qualification and prospective full-caller evaluation; new SDK/full-query and
ALL8+54 fresh Gnark verification/resources still required before adoption.

No Python, questions, subagents, C/ASM rewrite, giant LUT, key/solver expansion,
new production pool or arbitrary RAM cap. Rust1.98.1 and serialized xtask.
Publish a concise polite factual report, preserving CPU+Metal inherited ALL8
comparisons, and commit AND push with exact remote verification. Checkpoint
handoff/latest-status. Canonical proof inventory44265 unchanged; seed any future
proof export from frontend-overlap176/proof-sha256-after-capture.json.
