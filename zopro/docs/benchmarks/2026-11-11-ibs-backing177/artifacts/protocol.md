# Same-byte SDK backing IBS diagnostic

The page census and retrospective cycle offsets found no simple page-size,
residency or single-worker explanation for the slow OFF copy. Compare exact
retained OFF copies 0/1/2 using operation and fetch IBS, with plain controls.
Unlike stream-ibs175, this compares different backings of identical executable
bytes. It does not qualify the rejected G1 candidate or select a fastest copy.

Three blocks, each containing plain/op/fetch panels for all three copies:
27 processes, each 61 measured requests after three primes, 1,728 SDK calls,
1,647 timed samples and 27 fresh independent Gnark exports. Canonical ledger
52,945 -> 52,972 only after receipt, binding and freshness checks. Primary 1x2,
32 workers pinned to CPUs 0-31; retain the previous full-screen environment.
For block b, rotate panels by b; copy order rotates by b+panel index modulo 3;
reverse panels and copy order on odd blocks. Keep all copies and observations.

IBS period 524288 for both ibs_op/cnt_ctl=1/ and ibs_fetch/rand_en=1/.
Use FIFO-gated warm intervals, raw samples, operation weight/data source, and
fixed original SDK binaries. No hardware kernel exclusion; decode user/kernel
IP separately. Retain every raw and script record. Reuse the qualified
stream-ibs175 decoder byte-for-byte except the exact SDK DSO path, separately
for each copy. Resolve exact symbol plus offset in one byte-identical OFF
disassembly. Raw/script joins, register/pretty checks, zero lost and all original
decoder invariants must pass. Record actual PID and fixed worker placement.

Before the first SDK and after the last, run isolated read-only page censuses
of all nine retained files. Compare PFNs at identical file offsets. Census
operations must not overlap any SDK recording. Do not evict, migrate, pin or
rewrite pages. Equality at the two endpoints is not proof of no interim changes;
helper file mappings do not inspect SDK private data or copy-on-write pages.

All 27 SDK processes complete before verification, disassembly or decoding.
During capture only poll the confirmed xtask handle. No helper/build/hash/archive
from the agent may overlap a recording. Source and identity checks by xtask
occur between sequential steps. No rerun of successful capture or extra blocks.

Observer guard per copy and event: median of three instrumented/plain mean
latency ratios within factor 1.05. Quality guard per profile: >=1000 user G2
op samples or >=300 user G2 fetch samples; zero lost; exact raw/script joins;
single user PID agrees with SDK main task; no unresolved exact SDK instruction
lookups for operation samples. Report imprecise/saturated samples separately.
If guards fail, retain findings but do not treat service observations as
representative. These are descriptive guards, not equivalence tests.

Report G2 and G1 counts, operation tag-to-completion/completion-to-retirement,
instruction classes, demand load sources/latencies, completed fetch latency,
incomplete fetch fraction and I-cache/op-cache/TLB misses, each by copy/block.
Do not turn sample fractions or mean delays into exclusive wall-time savings.
No causal cache-index model, no adopted optimization, no AWS or mobile result.
Current ALL8 CPU/Metal timing report remains g1-offset177. Goal active, 0/8.
All work local; no push or publication without approval.
