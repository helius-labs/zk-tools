# Diversity certificate capacity on three 1×2 inputs

The single-pass fingerprint layout certified 16,915 distinct nonzero scalars on the original canonical query. The grouping cutoff was 16,804. The two-pass bitmap certified 16,932 and the release bitmap certified 16,631. Each uses 32 KiB of scratch in this size range. Both alternative counts also cleared the cutoff on the two other canonical inputs. All three layouts cleared the G2 cutoff on all three inputs. The debugger measured counts and independently verified one exported proof per input. A full-proof timing is needed to compare the cost of checking the byte fingerprints.

| Witness | Query | Cutoff | Exact distinct | Release bitmap | Two-pass bitmap | Single-pass tags |
|---|---|---:|---:|---:|---:|---:|
| Reference | Canonical | 16,804 | 17,234 | 16,631 | 16,932 | 16,915 |
| Reference | G2 | 24,071 | 25,816 | 24,584 | 25,203 | 24,642 |
| 29 | Canonical | 16,804 | 17,795 | 17,210 | 17,496 | 17,474 |
| 29 | G2 | 24,071 | 26,641 | 25,373 | 25,981 | 25,277 |
| 32 | Canonical | 16,804 | 17,785 | 17,182 | 17,466 | 17,442 |
| 32 | G2 | 24,071 | 26,630 | 25,311 | 25,947 | 25,319 |

[Protocol](protocol.md) · [Observed streams and proof digests](artifacts/observations.json) · [Verification](artifacts/verification.json)
