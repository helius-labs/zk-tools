# Gather SDK: executable backing diagnostic

The large 16-worker gap in the original gather candidate disappears in both
predetermined pairs of newly written executable files. Each copy is byte-exact
with its original. This demonstrates a backing-associated effect in this
full-SDK comparison, extending the [earlier component finding](../2026-11-10-backing-control176/README.md).
It does not establish a hardware mechanism or a gather-policy speedup.

The [original 31-block screen](../2026-11-11-bounded-gather-sdk177/README.md)
remains failed. This is a separate diagnostic of the exact retained OFF/ON
binaries, not a replacement qualification. The candidate remains disabled.

## Measurements

Ryzen 9950X, primary 1x2, 16 fixed workers on CPUs 0–15. Fifteen fixed blocks
rotate/reverse all six equal-length executable paths in one directory. Two paths
hardlink the original OFF/ON inodes; four are independently written copies,
two per consumer. Complete hashes and device/inode identities are checked
before and after. No native rebuild, ELF modification, ASLR change or fastest-copy
selection occurs. Physical page identities and virtual mappings were not measured.

Each process measures 151 complete SDK requests after three primes. Table times
are medians of process means. Paired ratios need not equal ratios of medians.

| Backing | OFF ms | ON ms | Paired ON/OFF [marginal bounds] | OFF / ON CPU ms per proof |
| --- | ---: | ---: | --- | ---: |
| Original inodes | 9.20496 | 11.06881 | 1.20553 [1.19762, 1.21445] | 117.627 / 138.032 |
| Copy pair 0 | 9.20307 | 9.22226 | 1.00143 [0.99916, 1.00557] | 117.599 / 117.346 |
| Copy pair 1 | 9.16743 | 9.20917 | 1.00712 [1.00395, 1.00908] | 117.668 / 117.384 |

Candidate copy/original paired latency is 0.83255 [0.82824, 0.83699] for copy0
and 0.83181 [0.83009, 0.83401] for copy1. Control copy/original is 1.00196
[0.99916, 1.00415] and 0.99592 [0.99289, 1.00184]. Both candidate copies remove
most of the original excess CPU use. Their ON/OFF CPU ratios are 0.99864
[0.99780, 0.99984] and 0.99974 [0.99820, 1.00093].

Intervals use sorted-15 ranks 3/11, zero-based, with 96.484375% nominal marginal
coverage under the declared median model. Two integer binomial constructions
agree. These are not simultaneous bounds or proof of stationarity. All copies
and observations remain included. The copied candidate has no demonstrated
latency gain: one pair includes parity and the other has a small regression.

The association follows this candidate's backing file under the tested
intervention. The experiment does not isolate physical cache coloring, fetch
latency, page placement, branch prediction, runtime data addresses or any
other specific mechanism. It adds no new 32-worker, 2x2, AWS or Aeglos result.
The broader CPU10x goal is unchanged and unmet.

## Static audit and verification

Before capture, the retained G2 apply functions were decoded again using the
previously tested ELF/literal reader. All 8,591 relative instruction positions
and normalized instructions match, and all 102 checked explicit-width read-only
literal loads have equal payloads. This extends the previous normalized-text
comparison; it does not prove whole-program or runtime-pointer equivalence.

All 90 fresh exported proofs independently pass Gnark verification, image/public
binding and freshness checks. Command/stream receipts, worker masks and all
input/executable pins pass. There are 13,860 SDK calls including primes and
13,590 timed samples. The canonical ledger advances 51,910→52,000. No owned
work overlaps recording; all proof verification and analysis follow recording.

The evidence root is `/home/ubuntu/src/zopro-perf-work/gather-backing177/`.
It retains originals through hardlinks, all copies, identities, frozen protocol,
plans, Rust reader/generator, SDK/verifier receipts, every sample/resource summary,
literal audit and final ledger. No capture or successful verifier was repeated.

Future small-change protocols on this host should use predetermined independent
backings for every consumer and retain all outcomes. This is a measurement
control, not a production speed optimization or a promise that copying always
normalizes performance. Prior failed gates remain closed; a future qualification
must declare its backing model and comparisons before recording. The current
candidate still lacks evidence of a useful proof-latency gain.

Report and playbook updates are local only; no push or publication is approved.
