# Retained SDK executable page census

All nine retained SDK copies occupy distinct physical pages at inspection time.
The slow OFF copy has no observed huge-page, residency or contiguity difference
that explains its earlier excess G2 cycles. This diagnostic narrows possible
explanations; it does not establish a hardware cause or a prover improvement.

The [G1-only screen](../2026-11-11-g1-offset177/README.md) remains closed and its
candidate unadopted. No SDK request, new proof, source change or timing rerun was
performed. The verified-export ledger remains 52,945.

## Physical pages

A standalone Rust helper maps only non-writable ELF load segments from the
three predetermined copies of each REF/OFF/ON executable. It reads its own
pagemap, kpageflags and kpagecount with the available administrative capability.
Each segment is mapped twice; all corresponding page frames agree between the
two aliases. Complete executable hashes and device/inode/mode identities agree
with the original runtime manifest. The helper ran through the retained xtask.

| Consumer | Copies | Read-only file pages per copy | Resident before touch | Huge/THP pages | Longest physical run, copies 0/1/2 |
| --- | ---: | ---: | ---: | ---: | --- |
| REF | 3 | 1,265 | All | 0 | 2 / 2 / 3 pages |
| OFF | 3 | 1,265 | All | 0 | 1 / 1 / 1 pages |
| ON | 3 | 1,264 | All | 0 | 1 / 2 / 1 pages |

The census covers 11,382 distinct file-offset pages across the nine files.
All 36 pairwise comparisons find zero shared physical frames. G2 borrowed apply
occupies 14 file pages in every OFF copy, with no adjacent physical frames.
No copy has a different page size or compound-page flag in the inspected regions.

Page interpretations follow the [Linux pagemap documentation](https://cdn.kernel.org/doc/html/latest/admin-guide/mm/pagemap.html).
PFN residue histograms are descriptive integers, not a validated cache-index
model. They do not establish or exclude cache conflicts.

These observations describe the current census, not the previous measurement
intervals. Mappings are inspected sequentially rather than held as one atomic
snapshot. Reading headers, symbols and hashes can warm file pages; sdk-f0 had
also been inspected with nm/readelf before this census. No claim about historical
cold-cache behavior, live SDK data mappings, NUMA locality or fetch latency follows.
No page eviction, migration, locking, cache drop or executable rewrite was used.

## Retained G2 sample offsets

A separate offline reader resolves the existing OFF profiles through each
recorded executable MMAP and the exact ELF segment/symbol addresses. It reproduces
the previously reported G2 borrowed-apply sample counts, then groups instruction
pointers into 1 KiB offsets within the same 51,862-byte function.

| OFF copy | G2 cycle samples | Logical CPUs represented | Minimum / maximum samples per CPU |
| --- | ---: | ---: | ---: |
| 0 | 2,007 | 32 | 45 / 86 |
| 1 | 3,564 | 32 | 87 / 137 |
| 2 | 2,017 | 32 | 47 / 76 |

The slow copy's excess is distributed across all workers and multiple arithmetic
regions. Relative to the mean count of copies 0/2, 27 of 39 occupied offset bins
have higher counts in copy 1. The four largest differences occur at offsets
39,936, 48,128, 38,912 and 47,104 bytes and total 831 of the net 1,552 extra samples.
Those bins were selected retrospectively; they are leads, not prespecified tests.

Cycle samples can skid. These offsets do not identify precise instruction
latencies, intrinsic arithmetic costs or removable wall time. Each copy has only
one profile in this panel. ASLR produced different load addresses; the prior
[fixed-address component study](../2026-11-10-backing-control176/README.md) is
separate evidence, not proof that virtual placement is irrelevant here.

## Next decision

Do not implement a page-layout workaround or repeat the rejected arithmetic
candidate from this evidence. A distinct next diagnostic would compare operation
and instruction-fetch IBS service observations for the same retained OFF copies,
with page identities checked around capture and observer effects measured.
It should distinguish frontend service differences from operand/resource waits
and data access, rather than repeat aggregate miss counters. The earlier
[streaming IBS study](../2026-10-29-stream-ibs175/README.md) examined one accepted
binary, not these differing byte-identical backings. Such a follow-up has not
yet been prepared or run.

Accepted local 1x2 remains **7.800 ms**, with **131.75 MiB** lifetime peak RSS in
the latest full screen; the rejected candidate was **7.856 ms / 129.75 MiB**.
The [complete CPU/Metal table](../2026-11-11-g1-offset177/README.md) remains the
current eight-shape comparison. The CPU 10× objective is unmet for all eight
shapes. No new AWS, Aeglos, mobile, pressure or minimum-RAM conclusion is added.

## Evidence

[Census summary](artifacts/result.json), [offset counts](artifacts/offsets.json),
[protocol](artifacts/protocol.md), [census source](artifacts/census.rs) and
[offset reader](artifacts/offsets.rs) are retained with command receipts and
hashes. Raw physical addresses, complete symbol lists and original profile
files remain at `/home/ubuntu/src/zopro-perf-work/page-census177/` and the linked
prior study roots; raw page data has a digest in the summary.

One helper compile initially failed on integer inference in a flag-count closure;
explicit integer types fixed it before any census. Capture completed once with
all identity/alias checks passing. The offline reader reproduced all three
original symbol totals. No successful capture was repeated. Production is
unchanged. Report and source are local only; nothing is pushed or published.
