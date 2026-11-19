# Sampled instructions in the Ryzen G1 and G2 bucket kernels

The Rust `xtask instructions` reader matched instruction pointers to the exact prover executable for all twelve existing hardware-counter recordings. The recordings covered 89,695 cycle samples from verified 1×2 and 16×2 proofs. Shares below weight each sample by its recorded event period. Each row pools three processes and includes samples outside the prover executable in its denominator.

| Shape | Encoding | G1 stream | G2 borrowed | G2 prefix | Poseidon emit | Outside executable |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | off | 23.84% | 19.46% | 0.86% | 0.31% | 2.06% |
| 1x2 | on | 22.96% | 19.06% | 0.79% | 0.23% | 1.69% |
| 16x2 | off | 21.99% | 0.00% | 6.72% | 3.93% | 1.05% |
| 16x2 | on | 22.75% | 0.00% | 6.96% | 1.88% | 0.95% |

## Instruction families inside the bucket kernels

The denominator in this table is the sampled period inside the named kernel with encoding disabled. IFMA denotes AVX-512 integer fused multiply-add instructions.

| Shape | Kernel | IFMA | Vector add/subtract | Vector logic | Vector moves | Vector permutes |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | G1 stream | 40.6% | 12.7% | 13.0% | 10.5% | 6.7% |
| 1x2 | G2 borrowed | 37.8% | 11.0% | 10.0% | 11.9% | 8.4% |
| 16x2 | G1 stream | 40.4% | 13.8% | 15.6% | 10.5% | 5.3% |
| 16x2 | G2 prefix | 44.2% | 12.9% | 12.8% | 11.2% | 5.1% |

On 1×2, `stream_g1_impl` and `g2_borrowed::apply` together accounted for over 40% of the sampled period. The 16×2 run used the owned G2 prefix kernel. A change to borrowed G2 alone would therefore reach a different workload from a change to G1 streaming. The profiles support comparing arithmetic changes in the complete bucket kernels before running full proofs. They also provide a baseline for the same measurements on C7a.

Instruction samples can land past the operation that caused a stall. These figures measure on-CPU activity across workers. Critical-path savings require a separate timed proof comparison. The recordings were collected on the Ryzen 9950X.

[Counter study and verified proof records](../2026-11-19-poseidon-emit-perf389/README.md) · [Per-process measurements](artifacts/runs.json) · [Executable and capture hashes](artifacts/binding.json)
