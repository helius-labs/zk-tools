# Linear constraint rows and G2 query wires

A key-only scan tests whether direct linear substitution could reduce the number of general G2 scalars. The scanner reads each pinned solver image, includes the unknown assignment term on its correct constraint side, and propagates purely constant sums through stored slots. For rows with a provably constant A or B operand, it expands the resulting linear equation over original wires modulo the scalar field, including slot prefixes and repeated terms. The count excludes zero identities. The Boolean column counts only wires produced by Bits hints. Other circuit constraints can also enforce Boolean values.

| Shape | B query terms | B wires produced by Bits hints | Nontrivial linear rows | Rows touching B wires outside hint outputs | Distinct such B wires | Direct pivot upper bound | Full-width hints on one such input |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 30089 | 1753 | 83 | 73 | 76 | 73 | 2 |
| 2x2 | 52274 | 3166 | 121 | 101 | 110 | 101 | 4 |
| 4x2 | 95537 | 5992 | 197 | 157 | 186 | 157 | 8 |
| 8x2 | 183347 | 11644 | 349 | 269 | 362 | 269 | 16 |
| 12x2 | 270757 | 17296 | 501 | 381 | 570 | 381 | 24 |
| 16x2 | 358343 | 22948 | 653 | 493 | 810 | 493 | 32 |
| 24x2 | 534619 | 34252 | 957 | 717 | 1386 | 717 | 48 |
| merge-24x1 | 507895 | 33904 | 513 | 273 | 402 | 273 | 48 |

On 1×2 the matching equations can directly eliminate at most 73 of 30,089 B-query scalar positions outside the hint-bit outputs, or 0.24% of all B-query positions. No shape exceeded this fraction. The much smaller set of full-width bit hints also leaves little room for this substitution. These counts support keeping the existing key basis for this family of relations.

The pivot bound counts the smaller of the direct matching row count and distinct touched wires. Exact rank, costs of introduced scalars, and point transformation costs can reduce the achievable saving. Rows whose A and B values depend on original witness wires are outside the constant-factor scan, including values that become constant only through cancellation or other constraints. Bits hints shorter than the field’s 254-bit representation describe only low bits. This census adds no proof or latency measurement.

[Image hashes and counts](artifacts/counts.json) · [Reader](artifacts/reader.py)
