# Starting B2 directly when the B1 branch returns the identity

On the instrumented large-query path, the original schedule joins a B1 closure with B2 after preparing the scalars. When B1 is omitted its closure returns the identity. The candidate calls B2 on the current worker in this case. Its grouped and ordinary scalar paths retain the same curve arithmetic. The decision is per process, with both settings in one experimental executable. The production release supplies a third control.

Every randomized block runs all five modes for both circuit shapes and pool widths. Each process warms three calls and times eleven complete proofs. The two trace arms also timestamp entry and return for A, K and H, entry into the B branch, availability of B2 scalars, entry and return of the B2 query, and the enclosing join. Timings include final verification. All 100 process exports passed independent verification.

| Shape | Ryzen workers | Release / ordinary / direct ms | Direct paired vs ordinary | Direct paired vs release | Direct vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 16x2 | 16 | 43.805 / 44.454 / 43.621 | -0.60% (5/5) | -0.70% | 7.68× / 2.84× |
| 16x2 | 32 | 39.393 / 39.080 / 38.733 | -0.13% (3/5) | -1.02% | 8.65× / 3.20× |
| merge-24x1 | 16 | 68.377 / 68.662 / 68.631 | -0.27% (3/5) | +0.37% | 7.39× / 2.55× |
| merge-24x1 | 32 | 63.582 / 63.565 / 63.339 | -0.74% (3/5) | -0.66% | 8.00× / 2.76× |

| Shape | Ryzen workers | Schedule | Paired trace overhead | Median ms to B entry / preparing scalars / ready to B2 entry / inside B2 | B2 entry after H return |
|---|---:|---|---:|---:|---:|
| 16x2 | 16 | control | -1.61% | 0.0012 / 5.2661 / 0.0008 / 21.2036 | 0/55 |
| 16x2 | 16 | direct | -0.20% | 0.0010 / 5.6373 / 0.0000 / 20.7278 | 0/55 |
| 16x2 | 32 | control | -0.21% | 0.0012 / 10.1555 / 0.0014 / 11.9113 | 0/55 |
| 16x2 | 32 | direct | +1.69% | 0.0012 / 10.2450 / 0.0002 / 12.0514 | 0/55 |
| merge-24x1 | 16 | control | -1.09% | 0.0035 / 7.9750 / 0.0008 / 28.3109 | 0/55 |
| merge-24x1 | 16 | direct | -0.21% | 0.0032 / 8.5997 / 0.0000 / 27.7434 | 0/55 |
| merge-24x1 | 32 | control | -0.43% | 0.0021 / 16.0660 / 0.0012 / 14.7958 | 0/55 |
| merge-24x1 | 32 | direct | +0.17% | 0.0013 / 15.7806 / 0.0001 / 14.5373 | 0/55 |

The timestamps bracket the B2 call, which can prepare additional data internally before reaching any sampled arithmetic symbol. The archived C7a observation counts samples in one arithmetic function. Both measurements may depend on the order in which workers acquire tasks. The bundled runner uses 64 and 96 physical cores by default and checks the topology. The results above use the local Ryzen mode.

[Samples](artifacts/samples.json.gz) · [Source change](artifacts/source.patch) · [Host runner](artifacts/run.py) · [Binary and input hashes](artifacts/config.json) · [Proof verification](artifacts/verification.json)
