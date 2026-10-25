# Shared raw products for quadratic-extension multiplication

The native field component passed its prospective speed gate. It reuses three
raw Karatsuba products across both complex outputs and performs two Montgomery
reductions. The retained implementation computes four raw products through
two fused field expressions. This report establishes a field-component gain;
SDK integration is a separate experiment.

For inputs `(a0,a1)` and `(b0,b1)`, set `A=a0*b0`, `B=a1*b1`, and
`C=(a0+a1)*(b0+b1)` using exact integer sums. The outputs before reduction are
`A-B` and `C-A-B`. Both accumulators use signed radix-52 coefficients, including
arithmetic carry propagation. No table, heap allocation or key preparation is
added.

## Range argument

Inputs are normalized and at most `2p`, with `p < 2^254`. Exact input sums are
below `4p < 2^256`. For Montgomery radix `R=2^256` or `2^260`, the real raw
product lies between `-4p²` and `4p²`; its REDC result lies strictly between
`-p` and `2p`. Adding `p` makes it positive and below `3p`. The imaginary raw
product equals `a0*b1+a1*b0`, between zero and `8p²`; its REDC result is
nonnegative and below `3p`. One conditional subtraction of `2p` leaves each
output below `2p`, preserving the existing lazy-field contract.

The low cancellation digit is computed modulo its radix, including for a
negative accumulator. Arithmetic shifts propagate exact signed carries.
Before mixed-radix packing, a conservative coefficient bound is `32*2^52` in
absolute value. The final four-bit packing expansion and modulus bias remain
below `2^62`, within signed 64-bit coefficients. All IFMA multiplicands remain
normalized nonnegative limbs; the last 48-bit cancellation uses `m<<4` only
when `m<2^48`.

## Component results

AMD Ryzen 9 9950X, one thread pinned to CPU 2, 31 rotating/reversing blocks.
Each fresh process performs three 2,000-iteration primes and 100,000 measured
iterations. Accepted and newly compiled old arithmetic are separate controls.
The new and old paths share the candidate executable. Setup, field packing,
canonical output conversion and result formatting are outside timing; a black
box consumes state before the end timestamp.

Numbers are ns per eight-lane complex element per iteration. Independent-stream
values also divide by four. The affine expression is an arithmetic workload,
not a complete curve operation or a proof.

| Pattern | Accepted | Fresh old | Shared | Paired ratio/fresh | Winning blocks |
| --- | ---: | ---: | ---: | ---: | ---: |
| Dependent chain | 5.638700 | 5.636910 | 4.524986 | 0.804438 | 31/31 |
| Four independent streams | 5.474700 | 5.479572 | 4.313372 | 0.786840 | 31/31 |
| Complex product then real scaling | 9.071490 | 9.072780 | 8.070500 | 0.889034 | 31/31 |
| Affine arithmetic expression | 21.463980 | 21.467976 | 20.627278 | 0.961072 | 31/31 |

The geometric mean of paired pattern medians is `0.857725` against accepted
and `0.857558` against fresh old, with 31/31 improving block aggregates for
both. The declared gate requires positive aggregate improvement and 21/31
wins against both controls, every pattern within 3% regression, and control
calibration within a factor of 1.03. All checks passed. No minimum gain floor
or unchanged timing retry was used.

There are 372 timed observations and 1,116 priming calls. Every final canonical
output word matches across all variants and blocks, rather than only a reduced
checksum. No other owned build, profile or benchmark overlaps timing.

## Correctness and code generation

All 35 native field tests pass unfiltered. Two new named tests check 729,088
result lanes against independent Arkworks field algebra: Fq and Fr, radices
256/260, four/eight SIMD lanes, lazy/boundary inputs and eight-step chains.
Coverage includes zero, `p`, `2p`, values around limb boundaries and random
inputs offset by `p`. Every raw result has normalized limbs and is below
`2p`. The existing IFMA generator check passes.

The first orchestration attempt did not recognize required test names because
`--nocapture` split their completion lines. The tests themselves succeeded.
The Rust parser was corrected and the same successful command receipt reused;
the tests were not repeated. The original failure is retained. Before timing,
the harness gained an explicit optimizer barrier; its earlier source/build
receipts remain separate.

| Standalone wrapper | IFMAs | Symbol bytes | Explicit stack frame |
| --- | ---: | ---: | ---: |
| Retained complex product | 300 | 3646 | 248 bytes |
| Shared raw products | 250 | 6717 | 6008 bytes |

The lower IFMA count does not imply fewer total instructions: signed carry
handling and register pressure increase code and stack use. These are
standalone wrapper observations. Inlined prover functions require separate
inspection, and no whole-process RAM reduction is claimed.

The SDK prototype changes two native IFMA8 Fq2 call sites: multiplication and
the conjugate numerator. It uses `--cfg zopro_shared_complex_products`; omission
keeps the old path. There is no per-call environment lookup. Portable, ARM,
WebAssembly and scalar arithmetic retain their paths. Native SDK OFF and ON
each pass 109 core tests and two raw/six cubic prover tests. Their all-nine
four-worker preflight completes 72 SDK requests and 18 independently verified
Gnark exports with fixture bindings and fresh proof identities. The separate
fixed full-SDK comparison finished and rejected this integration: primary latency
regressed 5.17% against accepted and 4.16% against fresh OFF. The component result
remains positive; no production arithmetic was changed. See the parent README
and `sdk-result.json` for the complete decision and retained observations.

The original all-eight 10x M5 CPU goal remains unmet. Supplied CPU and Metal
comparisons belong to full SDK measurements, not this primitive benchmark.
