# G2 explicit stack-lifetime diagnostic

Offline analysis only: reuse the terminal, qualified factor-lookahead profiles and
their exact OFF/ON binaries. No performance recording or native build is needed.
Earlier primary instruction and consumer-work audits already counted stack
operand samples. This diagnostic must add control-flow-aware direct stack-byte
liveness, per-slot long store/read spans, and source-line ownership in the actual
reverse group loop. A repeated mnemonic census alone is insufficient progress.

Two final consumer symbols are decoded with GNU objdump, including debug line
annotations. Derive PIE bias from each ELF executable LOAD plus its saved MMAP2,
check every selected sample is exactly an instruction start, retain all six G2
profile panels (both shapes/topologies and both arms), and preserve all samples.

Find backward direct-branch lexical intervals and report their IFMA sites; select
the unique maximal interval containing 1,200 IFMA sites as the reverse full-group
loop. Fail rather than silently choose another interval if that expectation is
not met. These are lexical loop intervals, not dynamic trip counts or dominance
proofs. Store/reads paired within one straight-line basic block have no intervening
direct write to the same bytes; this is not full alias/provenance analysis.

Stack-byte liveness uses direct constant [rsp+offset] operands only, per-byte use
and kill, CFG branch/fallthrough edges, with demands outside the selected interval
excluded. This is loop-internal explicit demand, not full function liveness.
Address-taking and calls are explicitly counted;
unknown memory effects and stack escapes prevent a whole-stack/spill claim.
Loads of constants and deliberate local records are included. RBP is not assumed
to be a frame pointer. Memory widths, masks and read/modify/write destinations
must be explicit; unsupported direct-RSP instructions fail analysis. LEA and NOP
are address calculations, not memory accesses. Aligned/masked moves are not all
assumed to kill every byte. Tail/general/fallback and panic blocks remain distinct.

No observed sample share is a causal stall decomposition, removable wall fraction,
retired instruction count, or native speed prediction. A specific native change
requires distinct source/dataflow evidence and cannot reopen closed late-packing,
cached-X/denominator, phase/staged, lazy-Y or lookahead experiments unchanged.
Full ALL8+54 SDK/Gnark/resource verification remains necessary before adoption.

Use Rust 1.98.1 and serialized xtask; preserve failures and exact command receipts.
Separate reader checks instruction/sample/interval conservation and selected loop
work, source/binary/raw-profile identities, explicit stack classifications and
liveness fixed-point equations. No successful native timing/proof rerun.
