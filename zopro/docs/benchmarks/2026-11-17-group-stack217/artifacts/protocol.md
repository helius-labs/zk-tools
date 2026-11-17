# G2 stack-address audit217

Frozen before any217SDK execution,2026-11-17.216scoped profiles locate most
extra OFF samples immediately following a32B scalar-to-stack copy. Stack
placement is a hypothesis, not established latency cause.215heap-offset test
does not test stack placement. No optimization or unchanged timing retry.

Reuse exact216SDK SHA6d9ae8f89e09b4bec8ac7c6a6f795bfbf04594623e4abf39802ada7b4d5d254c,
no relink/source/input change. Two32worker1x2 processes BooleanOFF/ON,3primes+
1timed call each, native trace/internalPMU ON/root as216. GDB breaks at actual
scalar_groups entry+17 just AFTER sixpushes and sub0x1a8, conditionalrdx30089.
Record frameRSP,entryRSP,spill(f0)address,scalarpointer,returnPC,functionbase and
GDBthreadID for all4calls/arm. Compare stack/spill mod32/64 and32Bcacheline
crossing. Verify exact prologue+hash beforelaunch. Do not read scalar bytes.

Question, not gate: does the actualstackplacement differ consistently across
these four calls/arm? If not, simple32B/64Bspillalignment is not an explanation
in this scope. Never infer timings/observer quality from debugger-paused SDK
runs, nor hardware causality from a correlated address. Only then consider
bounded Rust intervention; no parameter sweep/borrowed-key rediscovery.

Pin inputs,actualSDK,protocol,debugger,helpers/scripts+receipts. AllSDK captures
finish before lateGnark/analysis; no owned CPU cojobs during capture. Require
four hits/arm,validABI/prologue,addressrelationships,unchanged source pins,
Booleanroutes,placement,8SDKcalls/2Gnarkfreshproofs. Seed21659032 SHA
3d8d234da2237304a40b1658a1b5a2d47a06a2c6245a464a3ec87ca9a39756ad.
Fullgoalall8CPU10xactive0/8. No sourceadoption/AWS/portable/pressure claims.
Firstresultallowance15min. Localcommit/no push. Rawstackaddresses private;
public report boundedmodulos andreturnPCrelativeoffset only.
