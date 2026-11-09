# Final VM diagnostic recipe, before any prover recording

This supersedes the provisional16-trace/4-export design for event-format reasons,
before actual consumer or SDK recording. An infrastructure-only /bin/true probe
showed perf6.8 rejects period terms on tracepoints. A mixed-event infrastructure
record used default small buffers, lost records, and did not provide a trustworthy
software-period interpretation. Preserve infra.data and this finding; it is not
prover evidence. Use separate panels instead of interpreting mixed periods.

Final:16 period1 syscall traces (System and routed consumers x6contexts, plus
SDK1x2/merge54 x1/32workers);6 separate period512 minor-fault profiles (System
and routed consumers xsmall/large32, plusSDK1x2/merge54 x32). All use8192byte DWARF
stacks,512mmap pages and acknowledged region gates. All22 record first, followed
by110 ownership/decode/metadata/event/loss steps and6Gnark verifications.
138 frozen commands. Three primes plus3observed calls/process. SDK totals36
internally verified proofs including18primes,6distinct final exports,priorledger
40154 ->40160 only after each fresh export independently verifies.

No source optimization or speed selector. New routed consumer changes only the
global allocator, uses accepted core/field, and passes327680newindependent output
checks. Its standalone Euler calls do not enter an SDK proof pressure scope.
Use unchanged acceptedSDK for real-pressure-policy observations. No candidate
fixed-factor binary is executed. Prior lookup rejection remains closed.

Keep all outcomes including zero events, lost/truncated records or unknown
frames; no recording reroll. Loss prevents treating counts as complete. Raw
stack data and proof/public payloads stay private. Tracepoint counts and software
fault samples/getrusage are distinct; tracing perturbs execution, so no observed
ms is a new accepted latency. Actual event attributes, mappings and saved
receipts are the authority, not assumed event names or guessed page counts.
