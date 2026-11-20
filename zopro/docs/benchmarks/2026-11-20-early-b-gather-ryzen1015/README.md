# Gathering exclusive G2 inputs before A and K

The existing armed scope launches G2 after the fused witness gather has converted the wires and copied both A and B. The trial gathers and converts B directly from the solved Montgomery wires. Once G2 starts, the main pool completes in-place conversion and copies A. The parallel arm also runs the quotient alongside that remaining gather.

Both trials use the same loaded keys and the same fixed 16+16 Ryzen placement as the control. Each case has six paired timing blocks of thirteen proofs after three primes. Separate two-proof route runs clock B readiness and the point just before the G1 calls.

| Shape | Request | Profile | Proof ms | Paired vs fused gather | Peak MiB |
|---|---|---|---:|---:|---:|
| 1×2 | catalog | Fused gather | 6.804 | +0.00% (0/6) | 57.0 |
| 1×2 | catalog | B first | 6.895 | +1.08% (1/6) | 60.8 |
| 1×2 | catalog | B first with parallel quotient | 6.840 | +0.80% (1/6) | 72.8 |
| 1×2 | heldout | Fused gather | 6.616 | +0.00% (0/6) | 57.2 |
| 1×2 | heldout | B first | 6.589 | +0.13% (3/6) | 58.0 |
| 1×2 | heldout | B first with parallel quotient | 6.657 | +1.40% (2/6) | 67.0 |
| 1×4 | catalog | Fused gather | 8.187 | +0.00% (0/6) | 122.0 |
| 1×4 | catalog | B first | 8.229 | +0.42% (1/6) | 123.8 |
| 1×4 | catalog | B first with parallel quotient | 8.279 | +0.64% (2/6) | 112.0 |
| 1×4 | heldout | Fused gather | 8.111 | +0.00% (0/6) | 101.0 |
| 1×4 | heldout | B first | 8.158 | +0.14% (3/6) | 100.8 |
| 1×4 | heldout | B first with parallel quotient | 8.278 | +2.43% (0/6) | 127.8 |
| 1×8 | catalog | Fused gather | 9.628 | +0.00% (0/6) | 119.8 |
| 1×8 | catalog | B first | 9.661 | +0.55% (2/6) | 119.2 |
| 1×8 | catalog | B first with parallel quotient | 9.753 | +1.44% (0/6) | 135.0 |
| 1×8 | heldout | Fused gather | 9.671 | +0.00% (0/6) | 123.8 |
| 1×8 | heldout | B first | 9.701 | +0.39% (2/6) | 151.8 |
| 1×8 | heldout | B first with parallel quotient | 9.781 | +1.16% (0/6) | 134.5 |
| 1×16 | catalog | Fused gather | 12.499 | +0.00% (0/6) | 173.1 |
| 1×16 | catalog | B first | 12.422 | -0.09% (3/6) | 163.2 |
| 1×16 | catalog | B first with parallel quotient | 12.601 | +0.64% (3/6) | 170.5 |
| 1×16 | heldout | Fused gather | 12.576 | +0.00% (0/6) | 151.0 |
| 1×16 | heldout | B first | 12.496 | -0.33% (4/6) | 150.4 |
| 1×16 | heldout | B first with parallel quotient | 12.640 | +0.66% (2/6) | 157.1 |
| 2×2 | catalog | Fused gather | 9.911 | +0.00% (0/6) | 91.8 |
| 2×2 | catalog | B first | 9.991 | +0.07% (1/6) | 88.0 |
| 2×2 | catalog | B first with parallel quotient | 10.079 | +1.62% (0/6) | 122.5 |
| 2×2 | heldout | Fused gather | 10.104 | +0.00% (0/6) | 125.0 |
| 2×2 | heldout | B first | 10.014 | +0.98% (1/6) | 132.8 |
| 2×2 | heldout | B first with parallel quotient | 10.197 | +0.37% (3/6) | 116.8 |

The small 1×16 gain with the B-first arm was inconsistent across blocks. The parallel-quotient arm regressed on both requests for all five shapes. The SDK keeps the fused gather. All 210 exported proofs were verified independently.

[Summary and route intervals](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
