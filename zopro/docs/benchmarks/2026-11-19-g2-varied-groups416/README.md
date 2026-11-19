# Stability of signed G2 scalar groups across transfers

The discovery request supplies groups of row positions with equal or opposite nonzero scalars. Eight held-out 1×2 transfer requests retain the same key and change the private seed and output amounts. A group qualifies only if every row still matches its nominated representative with the original sign. On any mismatch the model keeps all original rows. An arbitrary witness can therefore be checked before using a prepared point sum. The row-by-row reconstruction matched the input scalar vector on every request.

| Request | Valid groups | Fallback groups | Rows removable | Same / opposite sign rows removed | Nonzero window updates avoided |
|---|---:|---:|---:|---:|---:|
| 0 | 926 | 0 | 2350 | 2338 / 12 | 39,575 / 613,891 (6.45%) |
| 1 | 920 | 6 | 1776 | 1766 / 10 | 38,917 / 613,905 (6.34%) |
| 2 | 920 | 6 | 1776 | 1766 / 10 | 38,917 / 613,891 (6.34%) |
| 3 | 920 | 6 | 1776 | 1766 / 10 | 38,917 / 613,919 (6.34%) |
| 4 | 920 | 6 | 1776 | 1766 / 10 | 38,915 / 613,901 (6.34%) |
| 5 | 920 | 6 | 1776 | 1766 / 10 | 38,917 / 613,918 (6.34%) |
| 6 | 920 | 6 | 1776 | 1766 / 10 | 38,917 / 613,896 (6.34%) |
| 7 | 920 | 6 | 1776 | 1766 / 10 | 38,917 / 613,913 (6.34%) |
| 8 | 920 | 6 | 1776 | 1766 / 10 | 38,916 / 613,893 (6.34%) |

The discovery plan has 926 groups and 2,350 nonrepresentative rows. A layout with one 128-byte affine sum per group, one 32-bit signed row index per member, and 32-bit offsets would retain 135,340 bytes before allocator overhead. The estimate excludes any changed query layout, validation scratch and preparation workspace.

The columns count signed nonzero window contributions before bucket collisions, scheduling and reductions. This is a feasibility measurement. All nine requests independently verified through Gnark. A debugger sampled the unmodified executable at the G2 input on all three warmups and the timed proof. The scalar and base digests were identical across the four observations for each request, and all nine point arrays matched. No traced wall times are compared. The captured scalars and individual group membership remain local.

25,246 of 30,089 scalar positions had exactly the same value in all nine requests. Of 920 groups valid throughout, 10 had a representative whose value actually changed. The eight changed seeds and output splits provide limited evidence about arbitrary transfers. A circuit-derived group or a validation on every use would be needed for a correct implementation. No witness-derived cache was installed.

[Aggregate results](artifacts/counts.json) · [Verification](artifacts/verification.json) · [Related digit-block model](../2026-11-19-g2-digit-blocks414/README.md)
