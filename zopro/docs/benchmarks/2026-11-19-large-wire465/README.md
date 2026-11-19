# Public wire coefficient coverage on 12×2

Eight public training transactions shared a nonzero coefficient at 20,488 of the 270,757 B2 positions (7.57%). Of those wires, 14,047 also feed A and 20,487 feed K. The resulting key-bound coefficient file occupies 737,616 bytes. Selection compared training vectors in chunks of 4,096 rows.

Eight further transactions supplied held-out inputs for a full-proof timing trial. Both sets included private splits, deposits, withdrawals and data-bearing outputs. All 16 baseline exports passed independent Gnark verification. This experiment measured coverage. A warm latency result requires running the general solver with the file enabled.

[Counts and file digest](artifacts/screen.json) · [Request, public input and proof digests](artifacts/verification.json)
