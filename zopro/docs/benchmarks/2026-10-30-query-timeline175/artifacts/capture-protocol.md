# Frozen capture accounting

The workflow contains nine one-request preflights and immediate Gnark checks, then 279 latency processes, then the remaining 279 Gnark checks: 576 steps / 288 processes / 43,002 internally verified requests including primes. There are 288 new distinct exports (expected ledger 25,231 only after actual verification), with 14,046 stage records including the three traced preflight requests.

This experiment's profiling is the in-process stage timeline; it adds no separate PMU or sampled-profile panel. Existing flat profiles remain diagnostic motivation. All capture/analyzer/publisher and source-review helpers must be compiled and pinned before starting capture. No owned independent workload may overlap it. All other prospective conditions remain in design.md, pinned before native qualification.
