
## Positive estimates and unresolved control discrepancy

The large16worker case improves37.73% against OFF (31/31wins) and6.81% against frozen (29/31wins, interval0.91334–0.93500). The corresponding solo case improves33.89% against OFF. At32workers, ON/OFF improves10.06% (22/31wins, interval0.74309–0.99587), while ON/frozen improves3.55% with interval0.77934–1.04900 and19/31wins. These positive observations remain in scope; the declared complete gate fails and no proof gain is established.

Frozen/OFF code and constants are byte-identical, yet the30,089point solo OFF/frozen ratio is1.45650 with a narrow1.45536–1.45848 interval; physical16 is1.48039 with interval1.46011–1.50713. This cannot be dismissed as ordinary zero-mean timing noise from these observations. Whole-MSM requested allocation counts/bytes agree between those controls. Source audit identifies only panic-line/build-ID byte differences, and the mode argument labels output only. Executable backing/addresses, process/allocator layout and runtime scheduling remain unresolved possibilities, not established causes. The large fresh-control gain is therefore not a reliable estimate of the optimization's isolated benefit.

The primary resource-only probe reports7,826,504 versus6,408,372peak extra requested bytes: about1.35MiB less with borrowing. Both OFF/ON processes reach23.5MiB RSS in that observation. Requested bytes perMSM drop from33,031,031 to31,559,031 and allocation requests5475→5435, consistent with removing unused buffers and smaller entries. These are single diagnostic resource observations, not minimumRAM or actual proof peaks.

The16worker counter observations show584.39M→598.08M instructions/MSM and363.04M→242.92M cycles/MSM. At32workers they show606.28M→618.95M instructions and454.25M→341.68M cycles. Thus the sampled improvement does not consist of fewer retired instructions. Kernel profiles locate the borrowed path and retain full symbol weights; no exclusive memory-bandwidth or spill cause is inferred.

The next distinct experiment will diagnose the byte-identical control discrepancy using unchanged binaries, mode labels and equal-length executable aliases. This is a control-identity diagnostic, not a rerun of the optimization gate, and it does not promote this candidate. Resolve the control problem before deciding whether a separately declared full-proof diagnostic is warranted.
