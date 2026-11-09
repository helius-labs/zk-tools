# Live ownership of the accepted SDK's large mappings

The prior goal turn made progress: accepted VM traces and scope audit pushed
b769d04c47. Sixteen syscall panels were clean; three fault panels were incomplete.
Two64MiB+4KiBmappings/request in real54SDK go through libc malloc but their Rust
owners were unresolved in8192byte captured stacks. Goal all8CPU10x remains unmet.

Use live GDB memory unwinding against the same accepted SDK6d83, without source
changes or rebuilding it. Catch mmap only when its x86-64 length argument equals
67112960. Print stack names/locations, never argument values, registers holding
field data or stack payloads. Include loading and three primes plus one observed
SDK request. Two processes: merge54 at1/32workers. Eight internally verified
proofs total; independently verify both final fresh exports after both debugger
recordings, checking uniqueness against the40160 ledger. These are diagnostic
requests and their timings are not performance results. GDB perturbs execution.

The helper is an unmodified Ubuntu GDB package extracted into this private
directory with dependencies, not installed system-wide. Disable debuginfod,
Python auto-load, and ASLR disabling; clear its private library path for the
inferior. Record tool/package identities. Keep all results, failed unwinds or
unmatched events; no debugger-recording reroll. All recordings precede analysis
or verification. Only poll the confirmed live workflow during recording.

Before attribution, distinguish solver wire storage, constraint row storage,
FFT and preparation-only allocations by actual call stack. A static size match
is not sufficient. If source/lifetimes prove Euler rows do not need domain tail
capacity, consider a distinct allocation-sizing change with all original field
initialization, legacy fallback and pressure ownership preserved. Do not add
persistent workspaces, new pools, tables, limits or change allocator policy.
