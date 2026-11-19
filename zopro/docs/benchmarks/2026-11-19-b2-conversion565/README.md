# Serial conversion of distinct B2 scalars after grouping

On the B2 branch the candidate converts the distinct grouped scalars out of Montgomery representation using a serial loop. Ordinary execution uses the existing parallel iterator. Other curve queries retain parallel conversion. Both settings share a binary, with B1 omission scheduling and hash-based class construction held fixed. The production executable supplies an independent control. A native test checks both paths against the field oracle.

Five randomized blocks on each shape and worker count include the production executable, ordinary and serial conversion, and a traced process for each conversion policy. Each process warms three complete proofs and times eleven. The diagnostic timestamps bracket class construction, distinct-scalar conversion and B2 query execution. All 100 exported proofs verified independently.

| Shape | Ryzen workers | Release / ordinary / serial ms | Serial paired vs ordinary | Serial paired vs release | Serial vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 16x2 | 16 | 43.864 / 43.692 / 43.434 | -0.59% (3/5) | -1.05% | 7.71× / 2.85× |
| 16x2 | 32 | 39.830 / 39.904 / 39.162 | -1.30% (4/5) | -2.72% | 8.55× / 3.17× |
| merge-24x1 | 16 | 68.563 / 68.277 / 68.346 | -0.20% (4/5) | -0.32% | 7.42× / 2.56× |
| merge-24x1 | 32 | 63.306 / 62.714 / 63.343 | +1.00% (0/5) | +0.15% | 8.00× / 2.76× |

| Shape | Ryzen workers | Schedule | Paired trace overhead | Median ms to B entry / preparing scalars / ready to B2 entry / inside B2 | Distinct scalars | Class construction / canonical conversion ms |
|---|---:|---|---:|---:|---:|---:|
| 16x2 | 16 | control | +1.26% | 0.0013 / 5.8604 / 0.0009 / 21.1556 | 46,795 | 5.1621 / 0.4414 |
| 16x2 | 16 | serial | +0.94% | 0.0012 / 5.9286 / 0.0011 / 20.6199 | 46,795 | 5.2205 / 0.5293 |
| 16x2 | 32 | control | -0.83% | 0.0012 / 10.0981 / 0.0011 / 11.7401 | 46,795 | 9.6712 / 0.4324 |
| 16x2 | 32 | serial | -0.09% | 0.0012 / 10.6373 / 0.0014 / 11.6791 | 46,795 | 9.6681 / 0.8919 |
| merge-24x1 | 16 | control | -0.70% | 0.0037 / 8.3460 / 0.0010 / 27.9338 | 62,005 | 7.3560 / 0.5816 |
| merge-24x1 | 16 | serial | -0.12% | 0.0028 / 8.4929 / 0.0012 / 28.1169 | 62,005 | 7.2367 / 0.8288 |
| merge-24x1 | 32 | control | +1.55% | 0.0019 / 16.0466 / 0.0021 / 14.9791 | 62,005 | 15.7793 / 0.3227 |
| merge-24x1 | 32 | serial | +0.40% | 0.0021 / 16.7096 / 0.0018 / 13.6996 | 62,005 | 15.2824 / 1.2203 |

The timestamps bracket the B2 call, which can prepare additional data internally before reaching any sampled arithmetic symbol. The archived C7a observation counts samples in one arithmetic function. Both measurements may depend on the order in which workers acquire tasks. The bundled runner uses 64 and 96 physical cores by default and checks the topology. The results above use the local Ryzen mode.

[Samples](artifacts/samples.json.gz) · [Source change](artifacts/source.patch) · [Host runner](artifacts/run.py) · [Binary and input hashes](artifacts/config.json) · [Proof verification](artifacts/verification.json)
