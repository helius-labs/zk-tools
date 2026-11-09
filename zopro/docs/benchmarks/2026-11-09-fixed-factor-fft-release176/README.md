# Release fixed-factor FFT consumers and preparation order

The bounded FFT candidate passes independent output checks with diagnostics
disabled in normal release builds. Release preparation also revealed and fixed
an admission-order problem: the small fixed-root reservation could prevent an
existing column table from being admitted during twisted-to-Euler conversion.
No timing or adoption result is added.

The same new consumer harness was linked to OFF, ON and the frozen accepted
reference. Each executable passed nine full coset/Euler cases at domains
32,768, 65,536 and 262,144 with 1/16/32 workers and 512 KiB stacks. Independent
Arkworks sparse-polynomial oracles checked 6,488,064 outputs across these arms.
Assertions and overflow checks were disabled; fixed-FFT diagnostic counters
were omitted. The actual OFF/ON executables that passed were disassembled.

An additional four admission probes confirmed the setup problem at domain
262,144. After twisted-to-Euler conversion, OFF admitted the ordinary column
table while ON retained neither direction's column table. The fixed-root
reservation occupied part of the shared legacy allowance before the existing
builder requested its full peak reservation. With ordinary initial preparation,
both arms retained the same inverse column table.

The revised private snapshot changes only the Euler preparation lifecycle:
release compact-root reservations before column admission, then rebuild compact
factors through the existing resource policy. Arithmetic, cache layout, column
builder and admission policy are unchanged. Five focused release cases check
ordinary/twisted initialization, 16/32 workers, two release/rebuild rounds, and
the real zero configured preparation budget. They verify the same column
configuration as OFF and 5,242,880 additional product/Euler outputs. All 27
commands in this stage passed on their first execution. The parent qualification
retains its earlier oracle repairs in its own report.

The inherited maximum compact-cache heap charge remains 32,016 bytes across
three directions, plus 24 bytes of optional pointers per plan. Euler retains
two directions. This excludes allocator overhead and temporary stack use;
neither process RSS nor minimum RAM was measured here. The existing column
table is separate from this bounded cache and follows its original policy.

Static code grew: the first release ON DIF/DIT leaves occupy 12,381/15,173 bytes,
versus 4,753/5,605 bytes in OFF. ON contains both fixed-factor and fallback
branches. Its static IFMA counts therefore include both paths; these are not
dynamic operation counts. Stack operands and call sites are also static
observations, not measured spills or peak stack. Factor expansion and additional
loads remain inside the actual consumer and may offset the arithmetic saving.

See the [release review](artifacts/review.json),
[lifecycle review](artifacts/admission-fixed-review.json),
[source audit](artifacts/admission-fixed-source-audit.json) and
[continuation notes](artifacts/next.md). The
[parent FFT qualification](../2026-11-09-fixed-factor-fft176/README.md) covers
separate inverse/forward transforms, bounded storage and diagnostic route proof.
The production sources and accepted SDK remain unchanged.

Next: link the measurement harness to the corrected ON-v2 core, retain the
qualified OFF/reference controls, and freeze a matched complete-consumer trial
with automatic CPU profiles. No candidate speed is established. Full verified
proofs, all supplied shapes, merge-54, resources and independent Gnark
validation remain required before adoption.

These are inherited local 9950X accepted-build results with 32 workers; this
stage adds no benchmark rows. The supplied M5 CPU/Metal figures are reference
measurements from another machine. The CPU 10× objective remains unmet, 0/8.

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

No new timing, profile, verified proof, SDK request, Gnark export or AWS/Aeglos comparison. The canonical ledger remains 40,154. Goal active, unmet 0/8. No owned job or cloud resource remains live.

[Source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
