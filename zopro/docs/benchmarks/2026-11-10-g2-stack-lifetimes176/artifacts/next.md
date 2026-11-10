# Continue the full CPU AVX goal

Goal active/unmet: verified single-proof CPU AVX >=10x the supplied M5 CPU for
ALL8 shapes, reporting CPU and Metal. No targets met. No user questions or agents.
Previous goal turn progressed by closing/pushing sorted-gap scan ea926ca463.
This turn adds exact consumer CFG/lifetime evidence from retained profiles; it
does not improve speed, implement a kernel or produce a new proof.

Current root /home/ubuntu/src/zopro-perf-work/g2-stack-lifetimes176.
Read README in report2026-11-10-g2-stack-lifetimes176, review.json,
tooling-notes.md and repair.md. Both disassemblies are saved once. The initial
lexical interval assertion and first CFG analysis fail; repaired analysis passes.
Four parser tests pass, including cold reentry/noreturn/broadcast syntax. One
direct reviewer assumption about total ON IFMA sites fails, then final separate
review passes. Preserve all receipts; no successful analysis/profile replay.
No current owned live job or cloud resource.

17,588 exact instruction rows /18,326 selected-symbol samples /15,271 reverse-SCC
samples. Two reverse SCCs have1,200IFMA sites each,3,590/3,637instruction rows.
Whole OFF has1,450IFMA sites; ON1,650 because its two-product initial preparation
is peeled outside the loop. Static duplication is not extra products per group.
Peak loop-internal direct-RSP byte demand4200OFF/3624ON(-13.714%).51same-basic-
block store/read pairs per arm; longest775/780instructions. No calls/address-
taking in these SCCs. Six retained profiles have9.97..12.51%reverse samples on
direct stack operands. No sampled IFMA in either reverse SCC uses a stack operand.
These are explicit syntactic demands and cycle sample locations; not whole stack
liveness, coordinate provenance, spill stalls, causal costs or a RAM bound.
Named noreturn call contracts are part of the CFG. DWARF mostly points to intrinsic
definitions and cannot name these long-lived values as Ax/Ay/Bx/By.

Critical implication: lower explicit stack demand already coexists with a
rejected implementation and ~0.5% more G2 query instructions. Do not select a
blanket spill/packing/inline rewrite, a compiler-flag sweep, or another identical
diagnostic from this evidence. Earlier source40coordinatevector arithmetic does
not prove40simultaneously live hardware registers. Native candidate still needs
a distinct arithmetic/dependency removal, complete consumer dataflow accounting,
new correctness/resource qualification and measured callers. Current profiles
cannot identify an exclusive G2 wall critical path or a twofold gain mechanism.

Targeted follow-up during this audit found symmetric-norm specialization already
closed in2026-10-28-symmetric-norm174,2026-10-24-g2-norm-squares and2026-11-05-
absolute-norm176. Do not implement it again. Existing phase/staged/layout,
lazyY, gather/pairedpacking, cachedX/denominator, independentchain, GLV/KH,
rank4/5, RNS/PMNS and sorted-gap recipes remain closed. Read only directly relevant
history for a distinct proposal; another broad history inventory is not progress.

Accepted current production176 and SDK unchanged:
/home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON
SHA7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01.
Fullproof local32 CPUms7.829285503/10.979334806/15.803968871/23.944016903/
33.703579161/39.858478065/66.186709387/65.489902226;primaryRSS133.50MiB,
16workers9.192236934ms,merge54=162.541995323ms(no suppliedM5reference).
Earlier7.803403940ms SAME executable. Not a new AWS observation.
Canonical43730 ledger MUST seed any new verified exports:
/home/ubuntu/src/zopro-perf-work/large-class-sdk176/proof-sha256-after-capture.json.

Autoresearch loop:
1. Revalidate current state and exact live handle; don't restart terminal work.
2. Choose a distinct bulk work/dependency reduction. Account for all work needed
   to obtain reduced mathematical products/length, sorting, conversion, scheduling
   and scratch. Source or isolated speed alone is not full-proof improvement.
3. Prototype privately/reversibly using Rust1.98.1. Independent arithmetic/oracle
   checks, checked admission/original fallback and small-stack shared callers.
   No giant LUT/expanded solverkey plan/productionpool/artificialRAMcap/C/ASM/Python.
4. Freeze source/compiler/libs/binaries/order/gates/oracles/environment/reader before
   preflight. During ANY preflight or recording ONLYpoll exact livehandle to
   terminal. No competing helpers/build/hash/report/push. Timeout not terminal.
5. Preserve failures and gains. No changed thresholds/favorable subsets/repeated
   successful measurements to rescue a fixed failed recipe. No positive gain floor;
   actual complete-caller evidence must distinguish gain from calibrated controls.
6. Positive callers permit ALL8+54 full SDK/Gnark/resource/portable/concurrency
   validation; only final-artifact proof evidence permits adoption. Full goal
   remains all8 CPU10x and CPU/Metal reports, never a component/subset/throughput.
7. Commit AND push each completed experiment, verify exact remote HEAD, update
   handoff/latest-status. Continue autonomously. Never complete/block merely for
   difficulty, negative evidence or time spent. Goal remains active.
