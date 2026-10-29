# Direct Euclidean-finish reduction: first complete SDK pilot

Accepted production is source 175 / SDK SHA256
20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f.
The all-eight CPU 10x objective remains active and unmet; 1x2 target is 3.7 ms.

The existing direct-finish reducer removes a scalar Montgomery cancellation
and subsequent scale-restoring field multiplication after the unchanged
one-word Euclidean finish. Reuse its exact arithmetic, supported-modulus guard,
tests and bounds. Current production inverse.rs must be byte-identical to its
original input. No changes to the loop, matrix updates, flags, point/field
representations, heaps, tables, plans, scheduling or query decomposition.
Only one of 175 originals changes; a single helper file is added privately.

The previous primitive comparison accidentally linked ON twice and established
no qualified timing effect. Do not rerun that primitive schedule. This is the
first complete SDK integration, with immutable separately frozen hashed OFF/ON
libraries, distinct library and executable identities, exact Cargo cfg checks,
and disassembly of the actual linked Fq/Fr inverse functions before capture.
Both fresh builds run all 34 field, 124 core and 8 prover tests; the 68,672
signed-residue cases are inherited inputs, not newly distinct field cases.
An accepted SDK binary is reused exactly, without rebuilding or repeating its
qualification. Complete proofs and fresh independent Gnark verification are
mandatory. No performance claim follows from the prior primitive estimates.

Fixed necessary pilot: all nine shapes preflight in accepted/OFF/ON at 32
workers, then 15 rotated/reversed blocks of primary 1x2, three primes and
151 measured requests per process. Loading excluded; fresh witness solving,
blinded proving, SDK verification and serialization included. Upper paired
ON/accepted and ON/OFF ratios must be below 1 and at least 10/15 blocks win
against each. OFF/accepted rank interval within [1/1.03,1.03]. Sorted ranks
4..12/15 have nominal 96.484375% individual coverage under independent
exchangeable blocks, not a familywise guarantee. No minimum gain floor,
exclusion, extension, successful rerun, threshold/layout/flag sweep or partial
promotion. A failure closes this exact SDK integration. Passing permits
separate full all-eight latency and compatibility/resource qualification;
it does not establish adoption, universal speed or the full 10x goal.

All latency captures precede latency proof verifications. Then three separate
FIFO-gated four-event perf-stat runs (31 requests), and three 199 Hz cycles:u
profiles (251 requests), each with three primes. All six diagnostic captures
precede their verification/decoding. Helpers, source, libraries, executable,
fixtures and tool pins freeze before capture. No owned build/helper/hash/
archive/other CPU job overlaps capture. Poll the exact live workflow handle;
an observation timeout never authorizes a restart.

Expected pilot totals: 7,902 SDK requests, 78 fresh Gnark verifications,
156 proof/public members; independent ledger 18,076 -> 18,154 only when actually
verified and unique. Original accepted all-eight CPU/Metal rows are inherited
explicitly; fresh primary comparisons must include both supplied references.
Accepted full-performance gate remains false with its recorded physical16
calibration engineering exception, final artifact gate true. No new AWS,
Aeglos, mobile, huge-circuit, pressure, minimum-RAM or throughput conclusion
from this pilot. Commit and push the completed result, including negatives.
