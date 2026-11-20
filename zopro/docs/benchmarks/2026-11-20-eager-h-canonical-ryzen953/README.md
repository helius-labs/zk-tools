# Canonical H coefficients at quotient emission

An eager encoding passed the paired full-proof requirement.

The FFT writes canonical limbs directly into the existing H coefficient buffer. The scalar mode converts each final output before its store. Packed mode removes the Montgomery radix in eight lanes using the raw integer one, then exports the canonical words. The H query consumes the resulting word buffer without the usual separate conversion. The proof preserves its existing fields and uses the ordinary path when the packed emitter is unavailable.

The 4×2 test uses 24 main and eight frontend Ryzen threads, with a separate four-thread solver pool. Each input uses eight counterbalanced blocks with three primes and seventeen measured proofs per process. The same-build ordinary path and exact prior solver executable are the controls.

| Input | Frozen ms | Ordinary ms | Eager scalar ms | Eager packed ms | Scalar vs ordinary | Packed vs ordinary |
|---|---:|---:|---:|---:|---:|---:|
| catalog | 14.483 | 14.498 | 14.500 | 14.414 | +0.11% (4/8) | -0.62% (6/8) |
| heldout | 14.514 | 14.481 | 14.443 | 14.393 | -0.66% (6/8) | -0.64% (6/8) |

| Input | Path | Quotient span ms | Peak MiB | Clock effect |
|---|---|---:|---:|---:|
| catalog | ordinary | 2.274 | 228.2 | +0.97% (0/2) |
| catalog | scalar | 2.232 | 211.2 | +2.84% (0/2) |
| catalog | packed | 2.296 | 217.2 | +2.67% (0/2) |
| heldout | ordinary | 2.252 | 239.8 | +0.91% (0/2) |
| heldout | scalar | 2.256 | 271.2 | -0.16% (1/2) |
| heldout | packed | 2.249 | 216.5 | +0.80% (0/2) |

The two clock blocks are separate from the eight proof-timing blocks. Seven native FFT tests compared complete encoded outputs, including padding and a short final block. The local route tests confirmed both canonical paths and the disabled-emitter fallback. All 84 exports independently verified. [C7a H readiness](../2026-11-20-h-readiness-c7a952/README.md) · [Eight-shape scorecard](../2026-11-20-quartet-placement-factorial-c7a950/README.md#eight-supplied-m5-references)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)
