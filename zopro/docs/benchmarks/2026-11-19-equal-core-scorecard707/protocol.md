# Scoring each shape on the 96-core C7a

Compare warm verified single-proof latency with prepared Aeglos on the same key, transaction and 96 physical cores. Score each of the 41 shapes individually on the catalog transaction and again on a second transaction with a different public input.

Freeze one Zopro configuration for each shape before reading the timings used to score it.

1. Experiment 689 selects a configuration from its catalog screening blocks. An original release configuration that swept its earlier comparison carries forward.
2. For shapes selected for experiment 703, compare the incumbent with its strongest new eligible candidate using the four 703 catalog screening blocks. Use the lower median of process means. An exact tie retains the incumbent. Both arms run in the fresh confirmation alongside prepared Aeglos regardless of which wins the screen. The split-pool configurations must fit both worker pools within the same 96-core process mask.
3. Score each 703 shape solely in its six fresh paired blocks per transaction. For every other shape use the eight 689 catalog confirmation blocks and the four 700 second-transaction blocks. Both transactions must use the same configuration. The earlier timings that determined which shapes enter 703 are excluded from their final score.

A shape passes the latency comparison when Zopro's median process mean is strictly lower than Aeglos's on **both** transactions. Report the two latency pairs, within-block win counts, and maximum whole-process RSS across both transactions for each engine. Also count shapes that won every pair on both inputs. These counts describe the tested transactions and blocks.

List all unresolved shapes in descending order of their worse Zopro-to-Aeglos median ratio. Report the corresponding millisecond deficit. For the original eight Apple reference shapes, preserve the supplied M5 CPU and Metal comparisons using the catalog transaction. Identify experiment-specific executables and profiles independently of any production default.
