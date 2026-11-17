# Generated-proof subgroup certificate: short screen

The private candidate passes its declared short-screen advancement rule. It saves
about0.61% against its own disabled mode at16workers, but the accepted-production
comparison is nearly flat and includes parity. Retain the exact build and finish
mechanism profiling before all-shape qualification. It is not adopted.

| Local Ryzen9950X case | Accepted REF ms | Candidate OFF ms | Candidate ON ms | REF / ON peak MiB |
|---|---:|---:|---:|---:|
| 1x2,16workers | 8.794146 | 8.808613 | 8.745414 | 77.50 / 79.25 |
| 1x2,32workers | 7.543668 | 7.576178 | 7.527096 | 104.75 / 107.25 |
| 2x2,32workers | 10.576818 | 10.621662 | 10.585946 | 268.50 / 269.00 |

Values are medians of nine process medians. Paired ratios need not equal ratios
of displayed medians. Primary16 ON/OFF is0.993901 [0.987332,0.995510],8/9wins;
ON/REF0.999347 [0.991539,1.003574],6/9. Primary32 ON/OFF is0.991843
[0.990263,1.000493],7/9;ON/REF0.999957 [0.992931,1.000568],5/9. The three
primary16 ON/REF backing medians are1.003574,0.996099,0.999347. These are
exploratory bounds, not an established production speedup. Raw contrasts remain
in [review.json](artifacts/review.json), including the2x2 control and OFF/REF.

Advancement requires either primary paired median below1 against both controls,
at least6/9wins against each, all three latency medians<=1.03,CPU<=1.05,and
maximum RSS<=eachcontrol*1.05+8MiB. All guards pass; primary16 satisfies advancement.
There is no minimum gain floor, excluded backing, sample extension or retry.
Bounds are zero-based sorted ranks1/7 of nine paired process-median ratios.

The generated G2 output is a linear combination of immutable B2, beta2 and delta2
inputs. The candidate deterministically checks those inputs on-curve and in the
subgroup once, then privately certifies the generated output's subgroup provenance.
The opaque output wrapper cannot be constructed by external callers. External
`Proof::from_raw` retains every check. Generated decoding retains coordinate bounds
and all curve checks; only the certified subgroup multiplication is omitted.
The pairing equation and fresh rerandomization remain unchanged. Invalid input
certificates retain original parsing rather than changing key acceptance.

This first prototype keeps the existing proof byte encodings and intermediate
MSM normalization. It does not implement typed projective outlets or serialization
removal. Its distinct question is moving a validation dependency to immutable
input preparation, rather than optimizing the subgroup multiplication itself.

Two small OnceLocks retain decisions on the Key and PreparedFixedPoints owners.
There is no point/scalar lookup table, witness cache, duplicate key, new pool or
architectural RAM cap. `ZOPRO_BENCH_CERTIFIED_OUTPUT=1` enables the private native
Linux IFMA route for B2 lengths below65536; unsupported/large/disabled paths skip
the scan and keep ordinary validation. The existing trusted mapped-image contract
requires the key bytes to remain unchanged while used.

Cold certification is real added work, outside warmed request timing:

| Case | Median certificate ms | Observed range ms |
|---|---:|---:|
| 1x2,16workers | 63.160 | 62.071–77.881 |
| 1x2,32workers | 52.056 | 50.829–56.159 |
| 2x2,32workers | 88.676 | 87.708–95.900 |

These are measured scan spans, not full cold-proof latency or CPU service.
Each serial SDK process records one key scan and one fixed-input scan during
priming, zero scans during timed requests, and one certified parse per enabled
timed proof. Concurrent first callers may duplicate cold scans; every actual scan
is counted. Later calls use the published immutable Boolean. A small warm saving
does not imply a one-shot benefit, so this remains explicitly opt-in.

The first native run exposed a Rayon/OnceLock dependency cycle in the new test.
Parallel work ran inside the initializer and could yield to another caller waiting
for that initializer. The hung test process was terminated; its partial output,
thread observation and exit101 are retained. The repaired snapshot scans before
publishing a completed Boolean, allowing cold duplication without holding the
initializer across cooperative work. No SDK ran before the repair.

The repaired source passes134 unfiltered native core tests,2prover unit tests
and6cubic integration tests, with no failures or ignored tests. New tests cover
valid/identity/off-curve/non-subgroup inputs, distinct fixed owners, ordinary versus
generated parsing, coordinate and curve rejection, and four concurrent OS callers
on1/4/16/32worker pools with512KiB requested stacks. Generic/ARM execution remains
unqualified for this candidate; unchanged field arithmetic qualification is reused.

The frozen screen uses three independent backings per binary, nine rotated and
reversed blocks,31timed fresh SDK requests after3primes, and two functional4worker
preflights. All83fresh exports pass independent Gnark verification and global
freshness/image/public-input bindings; the SDK verifies all2762calls including
primes. Timed work includes witness solving, fresh randomness, proving, SDK
verification and serialization. Loading, primes, certification and exports are
outside warm latency. No owned build or analysis overlapped capture.

Closing review rechecks179 source files, all unchanged178 production hashes,
256successful command receipts and stream hashes. Four private parents differ;
174 originals remain exact. A first closing reader failed by treating a structured
SDK-link step as a plain argv step; its repaired reader validates the link flags
separately. No scientific measurement was repeated. First-design-to-capture setup
was536.0seconds; capture workflow32.7seconds. Elapsed timing excludes earlier
history/research review and is not a research productivity or speedup metric.

Exact SDK SHA256:
`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.
Canonical57309->57392,SHA256
`f6425169716f853ffa71b4731b8dde5734c37b77d6aaf1dbe9f1107dc35cefb0`.
Private root:`/home/ubuntu/src/zopro-perf-work/certified-proof194`.

Next: profile this exact build, then finish all-shape performance, portable and
resource qualification if the mechanism survives. Production remains fusion182.
The [latest all-eight CPU/Metal comparisons](../2026-11-17-finite-all193/README.md)
retain their scope; this screen adds no all-eight performance claim. The all-eight
CPU10xgoal remains unmet,0/8. No AWS, physical phone, huge-circuit, pressure/swap,
minimum-RAM or throughput result is added. RSS is whole-process peak.
This report is committed locally; nothing is pushed.

[Closing review](artifacts/close.json),[complete raw evidence](artifacts/evidence.tar.gz),
[checksums](SHA256SUMS).
