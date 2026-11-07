# Actual SDK structure review

Freeze before disassembly or performance capture. Verify actual final SDK links use O/native/fatLTO/CGU1 and coherent pinned libraries. Decode signed G1 and G2 kernels once per SDK. Also decode ON direct borrowed-tree functions and its recursive Rayon producer bridge; OFF must contain no live tree text symbols. Record instruction/IFMA/stack metrics without requiring a field-arithmetic saving: field expressions are unchanged.

Necessary stack screen: every selected tree consumer's individual prologue reservation is below the existing512KiB test stack. Parse initial stack-probe loops using their r11 reservation, not one4096-byte loop iteration. Report raw prologues and probes. This is not a formal whole-call-chain bound, dynamic spill count, performance result or adoption gate. Actual native512KiB pool tests already cover the corrected Cargo consumer; final SDK code may differ. Full-proof timings decide performance after all9 Gnark preflight.
