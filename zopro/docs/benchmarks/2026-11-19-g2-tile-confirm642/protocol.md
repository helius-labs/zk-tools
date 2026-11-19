# G2 tile fusion on four 2×2 transfers

Study 640 observed a 1.13% paired improvement from combining tile merge and weighting on the original 2×2 transfer. Repeat the same release, disabled, staged and fused binaries at 32 local hardware threads. Run 12 balanced blocks on the original and eight each on private split 33, deposit 34 and data-bearing transfer 40. Each process warms up three times and times 21 proofs. Collect a separate 15-round CPU profile per arm and one dispatch trace per witness. Verify every exported proof and each expected public output.

Compare fused with staged to isolate the barrier change. Compare with disabled and ordinary release to qualify any deployment claim. Report every witness, paired wins and whole-process maximum RSS. A reversal on the original or inconsistent direction across witnesses leaves this candidate experimental.

[Initial measurement and implementation](../2026-11-19-g2-merge-weight640/README.md)
