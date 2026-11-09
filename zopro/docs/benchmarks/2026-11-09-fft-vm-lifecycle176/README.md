# Accepted FFT and SDK memory-lifecycle diagnostic

The large consumer's repeated arena churn does **not** describe the primary
SDK proof. Clean syscall traces show no mmap or munmap during three warm 1×2
requests at either one or 32 workers. The 54→1 SDK does repeatedly allocate and
release two mappings of 64 MiB + 4 KiB per request through libc malloc. Their
precise Rust buffer owners remain unresolved. No optimization or speedup is
selected; production and the accepted SDK are unchanged.

The previous consumer harness used System. The SDK installs Zopro's
[pressure-aware allocator](../../../crates/zopro-core/src/alloc/linux.rs) and
enters proof scopes. This diagnostic compares the unchanged System consumer,
a baseline consumer installing that same allocator, and the unchanged accepted
SDK. The new consumer passes 327,680 independent output checks before tracing.
No fixed-factor candidate is executed and its prior rejection stays closed.

At domain 262,144 with 16/32 workers, both consumer allocators show the same
pattern per call: one 128 MiB PROT_NONE arena reservation, two protection changes
covering about 24 MiB, and two 64 MiB unmaps. The reservation originates in a
driver input clone; a later protection change and release occur through the
Euler call. Virtual reservation and unmapping volume are not physical RAM.
Single-worker consumers show no selected VM syscalls in the observed interval.

|Observed context|Workers|Minor faults/call|Successful mmap calls/call|Successful munmap calls/call|
|---|---:|---:|---:|---:|
|System consumer, domain 262144|16|6221.7|1|2|
|System consumer, domain 262144|32|6275.7|1|2|
|SDK-allocator consumer, domain 262144|16|6211.3|1|2|
|SDK-allocator consumer, domain 262144|32|6342.0|1|2|
|Accepted SDK 1×2|1|0.0|0|0|
|Accepted SDK 1×2|32|1079.3|0|0|
|Accepted SDK 54→1|1|18412.0|2|2|
|Accepted SDK 54→1|32|25437.3|2.67|4|

The table uses complete syscall panels and getrusage totals over three traced
calls. Instrumentation perturbs execution; no traced wall time is a replacement
latency measurement. Primary32 has protection changes as malloc grows committed
storage, with some stacks reaching MSM code. Large SDK mappings have malloc
frames above the core allocator, rather than the allocator's direct pressure
mmap path. Some 8 KiB stack captures end before the Rust buffer owner, so naming
those mappings as particular solver arrays would go beyond the evidence.

A separate unchanged System-consumer process has only 14 faults/call in the
large32 fault panel. Low-fault behavior therefore does not require the rejected
fixed-factor optimization. It varies with process/allocation state; it is not
a selected performance result.

All 16 syscall traces have zero LOST records, matched entry/exit pairs and no
failed observed VM calls. Three of six fault-stack recordings lose records:
large32 SDK-allocator consumer, primary32 SDK, and 54→1 SDK. Retained frames show
FFT/clone activity in the consumer and MSM/solver activity in the SDK, but these
incomplete recordings cannot provide unbiased attribution percentages. Although
512 was requested for the software event, retained fault records serialize
period 1; the analysis reports actual records and never scales them by 512.
No recording was repeated to repair these limitations.

All 138 capture/offline/verifier commands complete on first execution. The 22
recordings use acknowledged region gates after three primes, followed by three
observed calls. Every recording finishes before offline work. The SDK completes
36 internally verified proofs including primes; six distinct final exports also
pass Gnark and extend the private canonical ledger from 40,154 to **40,160**.
No proof/public payload, input, raw stack record or full ledger is published.

An initial preparation-helper type error and infrastructure-only perf format/
buffer failures are retained. The final separate VM/fault panels were frozen
before any prover recording. Event control and DWARF capture follow the
[kernel perf manual](https://man7.org/linux/man-pages/man1/perf-record.1.html);
the installed perf6.8.12 attributes and trace formats determine interpretation.
Independent receipt and raw-header readers check all saved counts and byte sums.

[Analysis and stack examples](artifacts/analysis.json),
[review](artifacts/review.json), [raw-header audit](artifacts/raw-audit.json),
[proof inventory](artifacts/proof-inventory.json),
[final diagnostic design](artifacts/capture-design.md) and
[continuation notes](artifacts/next.md) retain the evidence.

Next: identify the actual large SDK allocation owners before proposing any
proof-local buffer reuse. The primary needs separate compute/MSM work; its
traces do not support treating the consumer's whole-arena churn as its main
bottleneck. Persistent large workspaces, disabled reclamation and the earlier
prefault/stack/scheduler variants are not selected by this evidence.

The following full-proof results are inherited local 9950X/32-worker accepted
measurements, compared with the supplied M5 CPU/Metal references. No new latency
row is added. The CPU 10× objective remains active and unmet for all eight shapes.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

The canonical proof ledger is now40,160. No candidate is adopted; no owned job or cloud resource remains live.

[Source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Large decode logs and raw stack recordings remain local; omissions and hashes are indexed.
