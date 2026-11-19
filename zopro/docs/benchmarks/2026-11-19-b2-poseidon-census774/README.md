# Exact Poseidon origins for B2 scalar grouping

The solver already recognizes Poseidon spans and verifies their coefficients. This probe compares the complete state after the first ordinary round of each recognized span. Identical states of the same width produce identical later outputs at the same round and coordinate. Every recognized span in the measured inputs exposes its signature wires in B2 itself. The probe assigns the later outputs such an origin and checks every resulting class against the solved witness.

For the original 8×2 request, the origin labels cover 115,691 of 183,347 B2 terms. Of those, 95,130 share a label with an earlier term. The model treats unlabeled terms with the ordinary exact dictionary. It keeps separate origin classes when they happen to have the same scalar as another class. This raises the modeled distinct-class count from 37,850 to 38,315 on the 8×2 original. The witness showed no contradictory scalar within any origin class. Both Poseidon output encodings gave the same counts on the two primary 8×2 inputs.

The probe runs once after the solver, before any optional public-wire adjustments. It records counts only. All 14 proof exports independently verified. This is a coverage measurement. The model does not establish a speedup or change the [current M5 and Aeglos results](../2026-11-19-b2-serial-sdk-c7a773/README.md).

| Shape | Input | B2 terms | Labeled terms | Repeated labeled terms | Exact dictionary classes | Modeled classes |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | 30,089 | 19,045 (63.3%) | 222 | 26,142 | 26,142 |
| 2×2 | catalog | 52,274 | 34,181 (65.4%) | 15,285 | 28,852 | 28,852 |
| 4×2 | catalog | 95,537 | 56,768 (59.4%) | 39,345 | 30,544 | 30,769 |
| 8×2 | catalog | 183,347 | 115,691 (63.1%) | 95,130 | 37,850 | 38,315 |
| 12×2 | catalog | 270,757 | 172,452 (63.7%) | 149,553 | 46,062 | 46,287 |
| 16×2 | catalog | 358,343 | 238,570 (66.6%) | 213,837 | 51,597 | 51,822 |
| 24×2 | catalog | 534,619 | 343,245 (64.2%) | 312,384 | 66,221 | 66,686 |
| merge-24×1 | catalog | 507,895 | 326,183 (64.2%) | 303,456 | 62,005 | 62,005 |
| 4×2 | heldout | 95,537 | 56,768 (59.4%) | 39,345 | 32,159 | 32,384 |
| 8×2 | heldout | 183,347 | 115,691 (63.1%) | 94,680 | 39,666 | 39,891 |
| 8×2 | v153 | 183,347 | 115,691 (63.1%) | 94,680 | 39,653 | 39,878 |
| 8×2 | v156 | 183,347 | 115,691 (63.1%) | 94,680 | 39,656 | 39,881 |
| 8×2 | catalog-emit1 | 183,347 | 115,691 (63.1%) | 95,130 | 37,850 | 38,315 |
| 8×2 | heldout-emit1 | 183,347 | 115,691 (63.1%) | 94,680 | 39,666 | 39,891 |

[Counts](artifacts/counts.json) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
