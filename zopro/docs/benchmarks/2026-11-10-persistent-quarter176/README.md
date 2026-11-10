# Persistent quarter-scaled G1/G2 coordinates

**The native arithmetic passes correctness checks; no speedup has been measured.** The prototype retains scaled coordinates between updates while keeping G1/G2 buckets at 64/128 bytes. Accepted production and its latest local 1×2 full-proof result, **7.783594 ms at 32 workers**, are unchanged.

The curve scaling is `(X,Y)=(x/16,y/64)` with uniform Montgomery radix `R=2²⁶⁰=16H`. Numerical X words therefore equal the existing `xH` representation. Numerical Y words represent `yH/4 mod p` and remain in a separate opaque type below `2p`. Incoming key points keep their original format; their Y components are quartered when consumed. No converted key, lookup table, second solver plan or production pool is retained.

Earlier experiments used per-batch scaled codecs, unscaled persistent R260 coordinates with preconverted keys, or lazy Y in the original H representation. This experiment combines a different scaling with a persistent storage boundary. No algorithmic novelty or performance advantage follows from that distinction alone.

The private module reuses the accepted uniform CIOS kernel and exact raw packing. Canonical X requires subtraction of `2p` and then `p`; retained Y requires subtraction of `2p`. Signed differences include the `3p` endpoint. The uniform inverse tree calls its original radix-converting `get/new` boundary. Those conversions are required work. Scalar tails and exceptional groups decode Y, use complete original curve arithmetic and encode Y again.

|New qualification|Checked population|
|---|---:|
|Signed scalar tuples|288 boundary + 4,096 deterministic random|
|Exact native arithmetic lane comparisons against BigUint REDC|26,312|
|Incoming quarter / retained-Y codec comparisons|13,152|
|Output-fed groups / exact lane comparisons|512 / 8,192|
|Ordinary G1/G2 point comparisons against independent Arkworks results|16,384|
|Tail and exceptional point comparisons|3,648|
|Complete scalar whole-group fallbacks|112|
|Native groups on requested 512 KiB thread stacks|98, across 1/16/32 threads|

Ordinary point checks cover all 256 sign masks over four changing updates for both groups, including 3,072 deliberately lifted Y components. Boundary scalar tuples exercise coupled complex operands; they are not an exhaustive Cartesian population of every Fq2 component. Point expectations use full-width independent Arkworks generator/scalar results. Exact integer checks compare numerical Montgomery representatives as well as bounds. All three native workflow commands and two offline review commands succeed on first execution. Review binds 373 pins, checks all 176 unchanged production sources, reverses the one append-only snapshot declaration and verifies the original packing body. The numerical oracle is part of the native qualifier; the separate same-agent reviewer checks sources, receipts and totals without rerunning it.

The qualifier uses one inverse tree per eight-point group. **It is not a complete 200-point prefix or Window implementation.** Its scalar fallback and identity guard are correctness scaffolding. A performance implementation must retain the original long-prefix inverse amortization and charge all bucket operations, tails, exceptions, final conversion, allocation and drop. Thread tests establish neither Rayon integration nor concurrent throughput. Equal bucket sizes establish neither equal peak RSS nor minimum RAM.

The prospective source cost for an ordinary 200-point prefix is unchanged in IFMA products:

|Group|Accepted and proposed ordinary vector products|Fused two-product events|IFMA events|Existing prefix record bytes per eight points|
|---|---:|---:|---:|---:|
|G1|155|0|15,500|1,600|
|G2|180|125|36,750|640|

These source counts include five vector products in the central inverse tree; its two scalar products and one inverse remain. They are not measured emitted instructions or runtime. Any gain must come from simpler linear corrections and representation boundaries. Incoming quarter conversion, uniform inverse conversion, scalar paths and final Y decoding may offset it. This stage establishes no 2× or 10× improvement. The next experiment must measure the complete Window lifetime before any SDK claim.

There are no new proof exports, SDK requests, profiles, timing, RSS, AWS/Aeglos, phone/ARM, huge-circuit, memory-pressure or swap observations. Canonical independently verified proof inventory remains 44,265. The following accepted local 32-worker full-proof observations and supplied M5 references are inherited from the [latest full SDK campaign](../2026-11-10-frontend-overlap176/README.md).

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 / 4.754×|24 / 3.083×|3.7|
|2×2|10.932731|59 / 5.397×|30 / 2.744×|5.9|
|4×2|15.757088|109 / 6.918×|48 / 3.046×|10.9|
|8×2|23.973295|189 / 7.884×|74 / 3.087×|18.9|
|12×2|33.827269|284 / 8.396×|101 / 2.986×|28.4|
|16×2|39.900917|335 / 8.396×|124 / 3.108×|33.5|
|24×2|66.041083|528 / 7.995×|176 / 2.665×|52.8|
|merge-24×1|65.717994|507 / 7.715×|175 / 2.663×|50.7|

The full goal remains active and unmet, **0/8**. Accepted 16-worker 1×2 is 9.016864 ms; merge54 is 162.170158 ms without a supplied M5 reference. Prior 111.25 MiB and 133.50 MiB primary full-proof RSS observations belong to different campaigns; this experiment establishes no memory saving.

[Protocol and bounds](artifacts/design.md), [qualification](artifacts/qualification.json), [source cost](artifacts/work-cost.json), [source audit](artifacts/source-audit.json), [review](artifacts/review.json), [five command receipts](artifacts/receipts.json), [native child](artifacts/quarter.rs), [oracle](artifacts/qualify.rs), [continuation and autoresearch loop](artifacts/next.md), [source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Executables, libraries, unchanged source snapshots, private fixture/key/witness/proof payloads and full ledgers are excluded.
