# Prepared G2 reducer on C7a

**The prepared reducer improves 96-worker scaling but does not improve the best
query latency. It does not advance.** Primary G2 at96workers improves7.35%, with
9/9paired wins, but its best tested result is5.868ms versus streaming4.875ms.
At32workers the prepared primary and grouped2×2 queries regress20.58%/32.85%.
Two of four progression checks fail. Production is unchanged.

This tests whether the [masked prepared reducer](../2026-11-17-shifted-identity188/README.md)
has a server advantage hidden by its local regression. An older
[C7a tree experiment](../2026-10-26-g2-tree-c7a/README.md) supplied direct evidence
that a local regression can coexist with better high-core scaling. The new
measurement supports that distinction but does not make this fixed design useful
for best single-query latency. More independent tasks do not establish less work
or shorter latency at the best worker count.

## Complete-query screen

One owned c7a.24xlarge, AMD EPYC9R14,96physical cores without SMT. Current179-file
production snapshot, with one test-only parent append and the unchanged188
prepared algorithm plus a new harness. Same executable selects streaming or
prepared mode. Rust1.98.1,znver4,release; executable SHA256
`06092266e64152d3d5ea5714fcb6443f67e419423c77fe706d20807cef7215b5`.
This is an experimental complete-G2 consumer, **not the accepted SDK executable**.

Eight one-round preflight processes precede72main processes: two retained real
queries,32/96workers,two arms,nine paired process blocks. Each main process has
3excluded primes,17timed queries and1postcheck. Order rotates/reverses; all three
predeclared separately copied,byte-identical executable backings have distinct
inodes. Workers pin to CPUs0–N−1 with512KiB stacks. All observations are retained.
No owned CPU work overlaps capture. No extensions,layout/width sweep or favorable
subset selection follows.

|Query/workers|Streaming ms|Prepared ms|Paired ratio|Prepared wins|
|---|---:|---:|---:|---:|
|1×2/32|4.875482|5.868280|1.205832|0/9|
|1×2/96|6.781612|6.263302|0.926450|9/9|
|2×2/32|6.506039|8.727235|1.328504|0/9|
|2×2/96|8.461589|7.731104|0.913670|9/9|

Times are medians of process means. Ratios pair corresponding blocks and need
not equal ratios of displayed medians. This is a directional screen without
confidence intervals or an adoption claim. All three backing medians agree in
direction for each context. The frozen gate requires primary96 and best tested
primary latency to improve with≥7/9wins, and grouped2×2 ratios≤1.02at both counts.
Primary96 and grouped96 pass; best primary and grouped32 fail.

Scalar copying,grouping,routing,allocation/drop,reduction and affine normalization
remain timed. Fixture loading,pool creation and prepared-table construction are
excluded. Oracle hashing is outside each wall timer. CPU service spans the timed
loop including those checks; it is not an arithmetic-operation count.

|Query/workers|Stream/prepared CPU ms per query|Stream/prepared peak RSS MiB|Prepared setup ms|
|---|---:|---:|---:|
|1×2/32|89.54/135.57|17.50/145.17|174.40|
|1×2/96|258.24/360.51|66.89/178.26|109.45|
|2×2/32|102.53/211.24|28.09/229.70|289.60|
|2×2/96|302.55/456.50|159.12/291.33|182.34|

RSS is the largest lifetime process peak across blocks,including setup and primes,
not minimum RAM. Prepared coordinate payload is84,730,624/147,203,584bytes,plus
661,958/1,150,028identity bytes for primary/2×2. Allocator/workspace/stacks add to
that. The benchmark service has a30GiB memory ceiling and swap disabled; those
are harness settings,not architectural requirements. No PMU,cache-locality,
memory-pressure,phone,huge-circuit or concurrent-throughput claim follows.

## Checks and decision

All1,552remote queries,including primes and postchecks,match the retained
independent Arkworks complete-MSM hashes; input points remain unchanged. Three
focused native tests and two local32-worker smoke processes pass before launch.
No new full-proof or Gnark export runs; canonical verified-proof ledger58033 is
unchanged. All179production hashes still match. The prepared body recovers188
exactly after removing the appended harness.

A separate same-agent offline reader checks80process receipts,160stream hashes,
runtime/plan bindings,paired ratios and remote executable hashes/inodes. This is
not an external audit or a new cryptographic implementation. The first remote
runner stopped before any query because its generated xtask plan omitted the
required source-pins field. Failure archive/source are retained; only the runner
is repaired,with native binary and protocol unchanged. Earlier preparation had a
manifest-field error and a moved-PathBuf compile error,both before execution.
Cleanup's first empty-list check expected a paginated KeyCount; using the explicit
nonpaginated response repairs observation without changing the measurement.

Native correctness takes61.50s; local smoke commands total0.21s; remote service
takes22.853s. Protocol creation to remote terminal result is about8m30s,including
the preserved pre-query orchestration failure,within the declared30min preparation
allowance. Reporting/retrieval/cleanup follow. One instance is terminated,its
volume is absent,and the private transfer objects/bucket are removed. No ECR
image is created and nothing is pushed to GitHub.

Close this exact prepared recipe on both tested hosts. The useful change in
understanding is that high-core relative gains can survive while best latency
gets worse,and extra preparation does not by itself close the server gap.
Further work must target lower complete-caller cost; no unchanged table,worker
count,layout or routing sweep is queued. The accepted194 full-proof SDK has
**still not been measured on AWS**. There is no fresh Aeglos comparison here.

## Accepted full-proof comparisons, unchanged

Inherited [local196 qualification](../2026-11-17-certified-all196/README.md),32workers;
M5 references were supplied,not remeasured. These are not AWS or prepared-query
proof timings. All-eight CPU10× goal remains active and unmet,0/8.

|Shape|Accepted ms|M5 CPU ms/speedup|M5 Metal ms/speedup|
|---|---:|---:|---:|
|1×2|7.628251|37/4.85×|24/3.15×|
|2×2|10.708637|59/5.51×|30/2.80×|
|4×2|15.851327|109/6.88×|48/3.03×|
|8×2|24.008343|189/7.87×|74/3.08×|
|12×2|34.020761|284/8.35×|101/2.97×|
|16×2|40.083109|335/8.36×|124/3.09×|
|24×2|66.332847|528/7.96×|176/2.65×|
|Merge24×1|66.233652|507/7.65×|175/2.64×|

Accepted merge54 remains163.384864ms/1881.32MiBpeak,without an M5 reference.

[Frozen protocol](artifacts/protocol.md),[all results](artifacts/analysis.json),
[offline review](artifacts/review.json),[artifact hashes](artifacts/package-hashes.json),
[sources and receipts](evidence.tar.gz),[checksums](SHA256SUMS).
Private root:`/home/ubuntu/src/zopro-perf-work/shifted-c7a197`.
