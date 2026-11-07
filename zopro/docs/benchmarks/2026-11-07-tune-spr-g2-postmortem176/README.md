# G2 scheduling regression: saved-profile postmortem

The rejected SPR-tuned build's extra G2 samples are spread across register IFMAs, vector operations and instructions with stack operands. **The static function has the same 8,591 instructions and 1,450 IFMAs in both builds.** Both have 11 memory-form IFMAs, all without stack operands. The data rule out a simple increase in IFMA count or stack-folded IFMAs in this function; they do not establish a causal stall or select another runtime implementation. Production remains unchanged.

This reuses the [completed full-proof campaign](../2026-11-07-tune-spr176/README.md). No benchmark, native operation, recording, external decoder or cryptographic verifier is run again. All nine profiles remain: accepted, accepted-source control and rejected candidate at primary with 16/32 workers and 24×2 with 32 workers.

## Sample locations

Every one of the 13,413 samples inside the selected signed borrowed G2 function maps to an exact instruction start. The analysis uses each process's executable MMAP2 file offset and the actual ELF executable LOAD segment; file and virtual offsets are not assumed equal. Function name, start and size are checked against the ELF symbol table. The nine whole profiles retain 92,285 samples. The large-shape profiles have zero samples in this selected function; they exercise a different G2 consumer and are retained as zeros.

The operand classes below are disjoint labels, not retired dynamic instruction counts or memory accesses. In particular, stack-address syntax can occur in address calculation. IFMAs are classified first.

|Context/arm|G2 samples|Register IFMA|Memory IFMA|Vector register|Vector stack operand|Largest 256B window / G2|
|---|---:|---:|---:|---:|---:|---:|
|1x2-w16-accepted|1601|621|2|626|151|3.12%|
|1x2-w16-control|1541|621|3|571|136|3.05%|
|1x2-w16-candidate|3048|1350|5|1147|291|2.76%|
|1x2-w32-accepted|1976|794|3|780|180|2.28%|
|1x2-w32-control|1941|796|7|739|180|2.16%|
|1x2-w32-candidate|3306|1357|9|1319|330|2.39%|
|24x2-w32-accepted|0|0|0|0|0|0.00%|
|24x2-w32-control|0|0|0|0|0|0.00%|
|24x2-w32-candidate|0|0|0|0|0|0.00%|

The candidate's largest single sampled instruction has 26/3,048 samples at 16 workers and 20/3,306 at 32 workers, below 1% of the selected function. Its largest fixed 256-byte windows contain 84/3,048 and 79/3,306 samples. Those windows are address bins, not necessarily basic blocks. No single sampled address or small window explains the distributed difference. Sampling skid and the location of dependent work prevent treating these counts as exclusive cycle costs or removable proof time.

The actual G2 function shrinks 51,862→51,720 bytes. Vector instructions with explicit stack operands change 603→607; broad scalar/vector/control classes move slightly while total instruction and IFMA counts stay equal. These are static locations, not spill traffic, live registers or whole-stack bounds. This does not prove memory is cheap, the code is optimal, or arithmetic alone causes the regression.

## Cached decode compatibility

The accepted decode uses AT&T syntax and the candidate decode Intel syntax. Both are already captured and are not rerun. The control uses the accepted decode only after checking its complete function bytes: 8,385 instructions match exactly, 195 differ only in a permitted relative-address tail and 11 need the explicit EVEX RIP-relative compare form with a trailing immediate. Opcode/ModRM prefix and immediate stay exact; only the four displacement bytes differ.

The original conservative pass leaves control instruction classes unassigned because its simple last-four-byte rule cannot recognize those 11 forms. That successful analysis is preserved unchanged. A separate refinement recognizes the specific 11-byte EVEX form, checks all 11 directly against the ELF bytes and performs 22 opcode/immediate mutation rejection checks. It then enriches the already recorded control offset histograms; it does not reparse or rerun the original accepted/candidate analysis. This establishes instruction-boundary/class compatibility, not equivalence of branch destinations, constants or field semantics. Control operand text in the refinement is an explicitly labeled accepted template.

## Review and decision

Both new xtask commands succeed on their first execution. An independent Rust reader checks 251 pins, the two command receipts, all 176 unchanged production identities, all nine saved raw sample streams and separate file-offset/ELF translations. It independently reconstructs all 13,413 selected instruction locations, operand classes and every fixed 256-byte window. It checks the 11 refined compare forms against actual bytes; it does not independently reimplement the complete compatibility algorithm or constitute a new disassembler.

The global SPR-tuning rejection remains. This follow-up selects no compiler-model sweep, threshold/shape rescue, assembly rewrite or native candidate. A next implementation needs a distinct dominant-work or dependency reduction with complete caller accounting; a few selected hot addresses are insufficient justification.

The [earlier operation/fetch IBS study](../2026-10-29-stream-ibs175/README.md) already reports arithmetic completion waits and limited demand-load DRAM service for a previous accepted build. We reviewed that evidence without repeating its captures. It keeps arithmetic dependencies as a research question; it is not a fresh measurement or an explanation of this particular scheduling regression.

Accepted local primary remains 8.008090 ms, versus supplied M5 CPU 37 ms and Metal 24 ms (4.620× and 2.997×). The [full all-eight CPU/Metal comparisons](../2026-11-07-tune-spr176/README.md#m5-comparisons) remain current; the verified all-eight CPU 10× goal stays active and unmet, 0/8. Accepted source 176/SDK `6d83` and canonical 34,439 are unchanged. No new proof, RAM peak/minimum, AWS/Aeglos, mobile, 250/4999 or pressure/swap result is added.

[Analysis](artifacts/analysis.json), [control refinement](artifacts/control-refinement.json), [review](artifacts/review.json), [source/plans/pins/receipts and saved input streams](artifacts/evidence.tar.gz), [checksums](SHA256SUMS). Binaries, proving keys, witnesses, requests and proof/public payloads are excluded.
