# C7a comparison for the grouped 24×2 candidate

The runner compares the release, ordinary and outer-only worker policies, the two policies with fixed A/K/B2 point groups, and Aeglos on the same original 24×2 request and two held-out requests. The held-out IDs are 99 and 103. Each uses the same key, affinity, and external verification. At 32, 64 and 96 physical cores, five randomized blocks warm three calls and measure eleven per process. Only the original request has M5 CPU and Metal references. The process samples include CPU time, latency and whole-process peak RSS.

The local smoke test executed all six configurations for all three requests at 16 and 32 Ryzen workers. All 36 exports verified. Cloud results are pending. The standalone bundle is at `/home/ubuntu/src/zopro-perf-work/grouped-c7a537/bundle`. On the 96-core host run `python3 run.py OUTPUT_DIRECTORY` from the copied bundle.

[Runner](artifacts/run.py) · [Binary, plan and request hashes](artifacts/config.json) · [Local verification](artifacts/verification.json)
