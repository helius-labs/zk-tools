# Earlier smoke results

These are the original CSV files previously saved in `/tmp`, copied unchanged.
They validate successful proofs and matrix coverage. Each case measured **one
proof with zero warmups**, so these are not the full warmed performance run.

| File | Scope |
| --- | --- |
| [zopro-bench-all-circuits-18.csv](zopro-bench-all-circuits-18.csv) | All 41 supported shapes at 18 threads |
| [zopro-bench-threads-smoke.csv](zopro-bench-threads-smoke.csv) | Transfer 1x2 at 1, 2, 4, 8 and 18 threads |

Observed environment: Apple M5 Pro, 18 available CPU threads, macOS arm64,
Node v26.0.0. All 46 recorded cases generated and verified their proof.
The full 205-case run with warmups and repeated samples is stored separately.

Commands, run from `zopro/packages/spr-prover`:

```sh
node bench/prove.mjs --threads 18 --rounds 1 --warmup 0 --output /tmp/zopro-bench-all-circuits-18.csv
node bench/prove.mjs --circuits 1x2 --threads 1,2,4,8,18 --rounds 1 --warmup 0 --output /tmp/zopro-bench-threads-smoke.csv
```

The full-shape run used source revision `5abf02af465eaa1a016dc39734d3c9b1f3de793e`.
The five-thread smoke was produced while the benchmark was being developed;
its harness was first committed as `45aa4ab8bf`. An exact clean source snapshot
and OS/power/load metadata were not captured for that earlier run.
See [metadata.json](metadata.json) for file hashes and recorded settings.
