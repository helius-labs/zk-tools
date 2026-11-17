# Exact retained-SDK grouping input audit212

Frozen before212debugger runs2026-11-17.211finds a~0.74ms runningCPU difference
in groupdecision before Boolean peeling, same scalar_groups function. No new
optimization, timing retry or causal PMU inference. Check exact runtime scalar
inputs first. Goal remains all8CPU10x,active0/8;209recipeclosed/notadopted.

Use unchanged exact211SDK84a472c6cd1a1eca40c9010d330aeea870ceb8e54381384bd163c2b626b3a6fe.
Two1x2/32worker SDKprocesses BooleanOFF/ON,traceON, three primes+one measuredSDK
call each. GDB at shared scalar_groups ENTRY, conditional scalar length30089,
reads 32*n bytes through raw ABI scalars argument on everyhit. Fourhits expected
perprocess. Export8rawscalar arrays PRIVATEONLY, retain bytecomparison/hashes/
classcounts/alignment records; no scalar bytes in report. Debugger stops alter
runtime heavily: no performance number or observer qualification is inferred.
New GDB15.1 installed(no existing packages upgraded); no runtime source/relink,
field,solver,grouping,threadpool,memorypolicy,keydatachange. Disabledebugdownloads
andautoload; pins include exactSDK,inputs,debugger,script andhelper.

Both exported freshproofs mustpassGnark.8SDKcalls,2fresh exports, seed21158980
SHA25f3486018e2702859911f4e047fb105d57020648a30aa46d2579e6262ad6e2b.
All8dump lengths962848, expectedcanonical scalars, recordexactnonzero/zero/one
anddistinctclass counts, byteequal acrossarms ANDcalls asquestionnotassumption.
If unequal, inspect actualdifferences before furtherCPU-cause work. Ifequal,
inputmismatch isexcluded in this auditable4-callscope; allocation/hashstate/
cache/parallelcontext stillunproven. Do notclaim allhistorical210/211inputs were
captured or infer fewerinstructions. Ifdebuggerentry/ABI/callcount fails, stop,
retainfailure, repairtoolingonly; nooraclechange. Firstresultallowance15min.
NoAWS, no publicrawkey/witness/proof/ledger/binary,localcommit/no push.
