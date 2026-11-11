# Bounded canonical query gathers: local SDK screen

The opt-in gather candidate is slower than both the accepted prover and its
fresh disabled control. It passed correctness and all six memory guards, but
failed all twelve latency/CPU checks. It remains disabled and is not adopted.
The 31-block screen is closed without sample extension or threshold changes.

The implementation applies `with_min_len(ceil(indices / (2 * workers)))` to
the existing canonical A/B indexed map/collect. It retains the original joins,
output order, allocation path and arithmetic. There is no new pool, key table,
unsafe writer or initialization pass. This tests the task-grain policy explored
in [zopro-parallel](../2026-11-11-parallel-chunks/README.md), without adding that
crate as a prover dependency. The synthetic component gains did not transfer
to this caller. The result does not establish the cause of the regression or
rule out every use of coarser task splitting.

## Results

Ryzen 9 9950X, fixed worker affinity, 16 physical cores / 32 logical CPUs.
Rust 1.98.1, znver4, original untraced SDK driver. REF is the retained accepted
G2-grain executable; OFF and ON are freshly built disabled/enabled consumers.
The comparison uses 31 rotating/reversing blocks. Each 1x2 process measures 101
requests and each 2x2 process 31, after three primes. Values below are medians
of process arithmetic means, including witness assignment, solving, proving,
internal verification and serialization; loading and priming are excluded.

| Context | REF ms | OFF ms | ON ms | Paired ON/REF median [bounds] |
| --- | ---: | ---: | ---: | --- |
| 1x2, 16 workers | 9.2543 | 9.2194 | 11.0779 | 1.1981 [1.1946, 1.2040] |
| 1x2, 32 workers | 7.8686 | 7.8214 | 8.7336 | 1.1102 [1.1065, 1.1118] |
| 2x2, 32 workers | 11.0662 | 10.9779 | 11.7793 | 1.0639 [1.0619, 1.0699] |

Bounds are sorted paired-ratio ranks 9/21, zero-based, with 97.055% marginal
coverage under the declared median model. They are not simultaneous confidence
bounds or a guarantee against shared-host drift. The primary candidate's
median process time increased about 0.865 ms against REF and 0.912 ms against
OFF. OFF was slightly faster than REF; it was a descriptive build control,
with no additional calibration veto.

| Context | REF CPU ms/proof | OFF CPU ms/proof | ON CPU ms/proof | REF / OFF / ON peak MiB |
| --- | ---: | ---: | ---: | --- |
| 1x2, 16 workers | 118.401 | 117.811 | 138.278 | 87.50 / 88.00 / 86.75 |
| 1x2, 32 workers | 179.638 | 178.205 | 195.510 | 130.25 / 129.25 / 129.25 |
| 2x2, 32 workers | 259.929 | 258.120 | 275.201 | 271.75 / 277.00 / 277.75 |

CPU includes user and system time across the request loop. RSS is the maximum
process lifetime peak, including load and priming; it is not minimum RAM.
Fault and context-switch counters are retained with every process summary.
No new profile or causal attribution to synchronization, memory or code layout
is claimed. The previously observed roughly 0.1 ms gather stage is historical;
it cannot explain this new whole-proof regression by itself.

## Verification and decision

Native correctness passed: 129 core tests, two prover tests and six cubic tests.
Preflight covered all eight supplied shapes plus merge54 for all three consumers:
27 independently verified exports. The main screen added 279 independently
verified exports and 21,669 timed samples. Combined: 22,614 SDK calls including
primes, 21,696 checked samples and 306 fresh exports. Proof, image and public
input bindings, stream hashes, command receipts and worker affinity were checked.
The canonical verified-export ledger advanced from 51,586 to 51,892.

All 18 progression checks were declared before preflight: primary latency gain
against both controls, latency nonregression in the other contexts, CPU
nonregression everywhere, and lifetime RSS guards. Only the six RSS checks
passed. Correctness passed; performance qualification failed; adoption is
rejected for this candidate. Portable/concurrent application qualification was
not performed. Default behavior remains OFF.

The accepted local 1x2/32 measurement is 4.70x the supplied M5 CPU reference
(37 ms) and 3.05x its Metal reference (24 ms). Accepted 2x2/32 is 5.33x CPU
(59 ms) and 2.71x Metal (30 ms). Neither meets the 10x CPU goal. Other shapes
received functional preflight only; their single samples are not steady-state
comparisons. The complete eight-shape accepted C7a CPU/Metal table remains in
the [earlier report](../2026-11-11-phi-grain-parent176/README.md), and is historical
relative to this local campaign. No AWS or Aeglos measurement was added.

## Identities and evidence

- REF: `c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe`.
- OFF: `51f2fa0611c60bf13b06aafad028469a8221cc46fd54960a83503cedfb8cf295`.
- ON: `cbd87b66fc132259b41118c541315aff9045e5c609de119b29891d93c8a19d87`.
- Source: local commit `0f49e87348`; 174 accepted files unchanged, two explicit
  existing-file changes and one added helper. Fresh Cargo builds are distinct
  artifacts from the retained accepted SDK.

Local evidence root: `/home/ubuntu/src/zopro-perf-work/bounded-gather-sdk177/`.
It retains source and library snapshots, build plans/receipts, frozen runtime
pins, `screen-design.md`, `screen.rs`, statistical cross-checks, all SDK/verifier
command logs, raw samples, exported proofs, rows and `assessment.json`.
The independent receipt reader was corrected before preflight to match xtask's
environment allowlist; no scientific run was replayed for that correction.

The public library remains an unpublished local experiment. Push and publication
require explicit user approval. The next useful work is a separate diagnostic
of the unexpected full-proof regression, rather than promoting the synthetic gain.
