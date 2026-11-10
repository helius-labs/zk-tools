# Additional offline repair notes

The first repair preparation expected result.json after a failed workflow, which
instead writes failure.json. That preparation exited101; a subsequent workflow
invocation exited1 before running any step because its plan was absent. Correct
the receipt filename; neither native work nor objdump was repeated.

The first CFG analysis then stopped because slice_index_fail is noreturn, like
panic_bounds_check. Treating it as returning connected a cold bounds-error block
to an allocation-reentry block and falsely joined both arithmetic loops. Add that
named noreturn case, terminate indirect jumps, and recognize QWORD BCST memory
width. Save the pre-repair source and add an explicit parser/CFG regression test.
The second repair runs four tests and completes the analysis successfully.

The first separate review assumed both complete symbols have 1,450 IFMA sites.
OFF does; ON has 1,650, including its peeled two-product factor preparation outside
the 1,200-site reverse SCC. This is static code duplication/peeling, not 200 extra
IFMAs per group. Save that reviewer version and correct its observation to the
actual two binaries. This changes no producer data, performance result or gate.
The first review was invoked directly: rustc succeeded, review exited101 at the
static-count assertion (left1650/right1450). Its failure message is recorded here;
no xtask receipt or original stdout file is claimed for that direct attempt.

The final separate reviewer uses a workflow with exact command/stream receipts.
Preparation helper compilations and the exploratory truncated disassembly display
were direct tool invocations; they are distinct from the numbered workflow steps.
Only two saved disassemblies are used. The successful analysis is not repeated.
