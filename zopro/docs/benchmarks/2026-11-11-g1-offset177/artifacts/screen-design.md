# G1-only full-proof screen

Freeze this protocol, executable copies, input identities, plans and readers
before the first SDK preflight. This is a new candidate and campaign. Earlier
combined G1/G2 and gather campaigns remain closed with their original outcomes.

Consumers: retained accepted REF c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe;
fresh OFF 90f1d8fbd8bd533daa4fcd572f66764a71266de378c17112f4074d5814ace4c3;
G1-only ON 0db7b2530991766c7d5b6436b89d759a9f258690aa60f625bc70ea5bc265108e.
OFF/ON use the completed coherent Rust1.98.1 znver4 builds. No relink, source,
worker-count or flag adjustment after capture starts. The original untraced
SDK driver includes witness assignment/solving, fresh blinding, proving, SDK
verification and serialization. Loading, three primes and export are excluded
from per-request latency. Whole-process lifetime RSS includes loading/primes.

Create three independently written byte-exact executable files for every arm,
in one directory with equal-length names, before capture. No reflink/hardlink or
fastest-copy selection. Record SHA256, device, inode and mode for all nine files;
require distinct identities and check them before/after both phases. Use copy
block modulo3 for every arm of a paired block. Each copy receives ten blocks
per context. This balances file backings but does not guarantee absence of
placement effects. Report per-copy medians and all aggregate results.

Host: local Ryzen9950X, primary1x2 at16 fixed workers (CPUs0–15), and all eight
supplied shapes plus merge54 at32 workers (CPUs0–31), ten contexts total. Retain
accepted runtime settings and original stack/allocator policy. Shared host,
frequency unlocked; SMT32 and physical16 results are distinct contexts. No
AWS/Aeglos/mobile/huge-circuit/minimum-RAM/throughput result is implied.

Functional preflight: nine shapes at32 workers, all three consumers, copy0,
one request after three primes. 27 processes/exports,108SDK calls,27samples.
Independent Gnark checks, expected public/image bindings, sample accounting,
affinity and freshness must pass before the main run. Seed52000 from the exact
gather-backing177 ledger; expected post-preflight count52027.

Main:30 fixed blocks. Rotate ten contexts by block modulo10, reverse odd blocks.
Rotate the REF/OFF/ON arm list by floor(block/3) modulo3, reverse it when
floor(block/3) is odd. Thus every copy sees the same arm-order sequence. Use101
requests per primary context and31 per other context, after three primes each.
900processes/exports,40500samples,43200SDK calls. Combined preflight/main:
927fresh exports,40527samples,43308SDK calls; expected final count52927 after
all independent verifications. All recording precedes main proof verification.
No result-dependent extensions, exclusions, copy choices or timing reruns.

Pair process arithmetic means within block/context. For30 paired ratios use
sorted zero-based ranks9/20 as bounds, average ranks14/15 as median. Check
nominal binomial marginal coverage using two integer constructions before
execution. No simultaneous-confidence, independent physical-page or stationarity
claim. Publish absolute latency, CPU/request, RSS, faults, switches and supplied
M5 CPU/Metal comparisons for every shape; merge54 has no supplied M5 reference.

Thirty-one prospective progression checks: primary32 latency upper<1 against
both REF and OFF (2); other nine contexts latency upper<=1.02 versus REF (9);
all ten CPU/request uppers<=1.02 versus REF (10); all ten maximum lifetime RSS
ON<=REF*1.05+1MiB (10). There is no gain floor. Other OFF comparisons are
diagnostic, not added vetoes. Per-copy ratio summaries are descriptive. Flag a
primary32 between-copy median-ratio spread greater than factor1.02 for diagnosis
before making a source-gain claim, rather than selecting a favorable copy.

This is a progression screen, not automatic adoption. A separate prospective
automatic profiling panel follows the latency screen, regardless of its outcome,
with controlled observer overhead, event coverage, attribution and lost-sample
checks. Its exact inputs/plans/readers must be frozen before its own preflight.
Generator integration and applicable resource/concurrency qualification remain
required before promotion. A small measured gain does not satisfy the full
all-eight CPU10x objective. Report correct/resource/performance/adoption status
separately. No push or publication without the user's approval.

During every SDK preflight/recording poll only the confirmed live handle until
terminal. No owned build, hash, helper, analysis or reporting overlaps capture.
