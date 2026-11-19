# Reducing cgroup reads in the native workspace check

The trial parses mount records with borrowed iterators and reads a cgroup's current use only when there is a finite limit. It still walks up the cgroup hierarchy and checks the lower of the hard limit and v2 high threshold. Each admission uses fresh memory values. The revised path caches no cgroup state.

Six paired blocks on Ryzen compare the released build with this change, using fixed 24 main and eight frontend hardware threads. Each process warmed three requests and timed nineteen. Native preparation tests passed, including the parent-limit and namespace-mount checks. All 56 exports independently verified.

| Shape | Input | Released / trial ms | Paired vs released | Peak MiB released / trial |
|---|---|---:|---:|---:|
| 1x2 | catalog | 6.972 / 7.165 | +1.83% (2/6) | 57.8 / 61.0 |
| 1x2 | heldout | 6.856 / 6.972 | +0.77% (3/6) | 61.5 / 64.2 |
| 2x2 | catalog | 9.950 / 9.740 | -1.51% (5/6) | 123.2 / 120.8 |
| 2x2 | heldout | 9.827 / 9.776 | -0.36% (4/6) | 120.5 / 127.2 |

Eight extra captures trace openat on the 1×2 inputs. Estimates subtract counts in the one-request process from counts with eleven timed requests and divide by ten. Each process uses the same three warm-ups. The total column sums the seven cgroup and proc sources in the trace summary. These captures measure syscall counts.

| Input | Released current-file opens per proof | Trial current-file opens | Released total | Trial total |
|---|---:|---:|---:|---:|
| catalog | 30 | 0 | 107 | 77 |
| heldout | 30 | 0 | 107 | 77 |

[Measurements and syscall counts](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
