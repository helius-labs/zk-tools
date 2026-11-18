# Sampled work during the solver call on C7a

Each of 33 measured solver calls per setting was split into five equal elapsed-time intervals. The original perf event samples userspace at 500 Hz per running thread. The interpreter column counts instruction pointers within the named general or Poseidon interpreter functions. Time in callees can fall in other categories. Worker activity elsewhere in the process can contribute to the same interval.

Across the complete 16×2 solver interval, total sampled userspace time increased from 231.5 to 432.6 CPU-ms per proof at 96 cores. Crossbeam increased from 148.7 to 337.9 CPU-ms, accounting for 94% of the increase. The largest absolute increase was in the middle fifth. These samples identify where workers spent CPU time. A wall-time comparison is needed to establish whether changing worker behavior shortens the proof.

| Shape | Main cores | Fifth | Wall ms | Interpreter CPU-ms | Crossbeam CPU-ms | Other Rayon CPU-ms | Other CPU-ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | 1 | 1.059 | 16.7 | 20.3 | 2.5 | 2.6 |
| 16x2 | 64 | 2 | 1.059 | 7.5 | 30.8 | 1.6 | 1.7 |
| 16x2 | 64 | 3 | 1.059 | 11.1 | 45.9 | 1.9 | 2.1 |
| 16x2 | 64 | 4 | 1.059 | 19.3 | 24.5 | 1.1 | 0.8 |
| 16x2 | 64 | 5 | 1.059 | 11.6 | 27.2 | 0.7 | 1.6 |
| 16x2 | 96 | 1 | 1.259 | 17.4 | 45.8 | 3.7 | 3.5 |
| 16x2 | 96 | 2 | 1.259 | 9.6 | 73.1 | 2.5 | 1.9 |
| 16x2 | 96 | 3 | 1.259 | 13.6 | 90.4 | 3.0 | 1.6 |
| 16x2 | 96 | 4 | 1.259 | 16.7 | 59.2 | 1.3 | 2.1 |
| 16x2 | 96 | 5 | 1.259 | 13.9 | 69.5 | 1.9 | 2.1 |
| merge-24x1 | 64 | 1 | 2.022 | 12.2 | 8.0 | 1.5 | 2.2 |
| merge-24x1 | 64 | 2 | 2.022 | 16.9 | 7.8 | 1.6 | 1.9 |
| merge-24x1 | 64 | 3 | 2.022 | 4.2 | 2.5 | 0.1 | 0.6 |
| merge-24x1 | 64 | 4 | 2.022 | 19.6 | 7.8 | 1.5 | 0.6 |
| merge-24x1 | 64 | 5 | 2.022 | 33.8 | 12.1 | 1.3 | 2.6 |
| merge-24x1 | 96 | 1 | 2.164 | 12.8 | 21.1 | 1.9 | 3.6 |
| merge-24x1 | 96 | 2 | 2.164 | 16.3 | 24.5 | 2.8 | 2.9 |
| merge-24x1 | 96 | 3 | 2.164 | 9.8 | 7.2 | 0.7 | 0.8 |
| merge-24x1 | 96 | 4 | 2.164 | 13.1 | 18.2 | 1.8 | 1.2 |
| merge-24x1 | 96 | 5 | 2.164 | 43.7 | 27.6 | 2.5 | 3.8 |
