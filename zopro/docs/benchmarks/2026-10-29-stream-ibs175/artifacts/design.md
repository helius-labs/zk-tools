# Current streaming-kernel IBS diagnostic

Accepted source175 and SDK20fde remain unchanged. The previous sparse service graphs show both K/H and G2 finishing last, with substantial serial feeding service. Existing 23Oct and27Oct IBS reports measure demand-load sources; this study adds operation tag-to-completion/completion-to-retirement by exact instruction and separate fetch latency/status. It does not repeat a candidate or adoption screen.

Six fixed fresh processes: three blocks, op/fetch, fetch/op, op/fetch. Each32workers, CPUs0–31, three priming proofs plus61 sampled complete internally verified SDK requests. Both event periods524288; op uses cnt_ctl=1, fetch rand_en=1. FIFO acknowledgments start after priming and stop before export. No user/kernel hardware exclusion; split actual recorded IP afterward. No extension, exclusions, successful rerun, global perf changes, simultaneous owned build/helper/hash/archive/other workload. All captures precede all independent verifiers and decoding. Expected384SDK,6fresh Gnark exports,12members; prior17805→17811. Diagnostic timings do not qualify latency or RSS improvements.

Raw registers and perf script join one-to-one by PID/TID/exact ns/recorded IP, op weights/source also agree. Pretty-decoded latency/flags cross-check raw bit positions. RipInvalid or substituted IP retained in sample counts but excluded from precise operation latency and mnemonic attribution. Exact mangled symbol+symoff maps into objdump instruction VMA without guessing PIE relocation. Only exact accepted DSO resolves. Stack-addressed means RSP/ESP operand, not necessarily compiler spill. Opcode instruction may contain multiple uops; load/store attribution uses raw IBS flags. Keep unsaturated valid latency pairs only; report invalid/saturated counts.

Tag-to-completion equals tag-to-retirement minus completion-to-retirement. It includes operand/resource wait and execution; it is not intrinsic opcode latency. Retirement waits are not exclusive stalls; samples/sums cannot be converted to removable wall time, bandwidth or a causal arithmetic/memory/frontend split. Op selection sees retired ops; fetch sampling is separate and can see nonretired fetches. Fetch completed latency and L2 miss only count complete fetches. Preserve incomplete counts and opcache/icache/TLB statuses. Three blocks are descriptive, with no causal or adoption confidence claim.

Source semantics: current AMD uProf5.3 IBS derived events (Zen4/Zen5), installed6.8.0-147 amd-ibs.h, upstream Linuxv6.8 perf IBS implementation. Header register layout documented for Fam19; Zen4 IBS extensions are explicitly exposed by this Zen5 runtime, not assumed from an older erratum. No generic frontend alias interpretation.

Four new decoder tests pass. A new nonSDK32MiB SHA256 fetch capability yielded31records. Its first decode exposed perf's omission of L2Miss on incomplete fetch; corrected before SDK captures with a focused regression test. No capability workload repeated. Existing239op capability records reused and decoded with new metrics, avoiding another unchanged capture. Original raw/perf fixtures and initial tests retained, inherited9 tests not rerun. No prover source/build/math test repeats: executable unchanged and qualified native tests inherited.

References:
- https://docs.amd.com/r/en-US/57368-uProf-user-guide/7.9.3.2.4.-IBS-Derived-Events
- https://raw.githubusercontent.com/torvalds/linux/v6.8/tools/perf/util/amd-sample-raw.c
- https://raw.githubusercontent.com/torvalds/linux/v6.8/arch/x86/events/amd/ibs.c
