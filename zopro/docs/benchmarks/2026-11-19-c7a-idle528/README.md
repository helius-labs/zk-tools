# Matched C7a bench for idle scans and Aeglos

The runner packages the integrated production executable, the patched Rayon executable, Aeglos and the original verifier for all eight supplied circuits. On a 96-physical-core host it runs five randomized blocks at 32, 64 and 96 cores. Every shape uses the same CPU mask and request for all engines. Both provers warm three calls. The timed sample contains 31 calls for 1×2 and 2×2 and eleven calls for the other six shapes. Each process exports an independently verified proof. Zopro's candidate runs with ordinary sleep and two idle-search limits. The 1×2 profile uses its 16-worker frontend and four serial G2 ranges. The manifest records all per-shape settings.

The Ryzen smoke run used 16 and 32 workers, one timed call per process and all eight shapes. All 80 exports verified, including both engines. Physical-core scaling results require execution on the C7a. The standalone local bundle is located at `/home/ubuntu/src/zopro-perf-work/c7a-idle528/bundle`. Run `python3 run.py OUTPUT_DIRECTORY` from the copied bundle on the target. It writes the topology, per-process samples, CPU time, peak RSS, verification results and an eight-shape summary with both M5 comparisons.

[Runner](artifacts/run.py) · [Hashes and profiles](artifacts/config.json) · [Smoke verification](artifacts/verification.json)
