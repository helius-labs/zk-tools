#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/src/zopro-perf-work/fq-fips175
rustfmt --edition 2021 model.rs
rustc --edition=2021 -C opt-level=3 -C target-cpu=native model.rs \
  -L dependency=/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/target/release/deps \
  -L dependency=/home/ubuntu/src/zopro-perf-work/balanced-kh-production/frozen/candidate \
  --extern num_bigint=/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/target/release/deps/libnum_bigint-6bc1a1b0ca7a2b87.rlib \
  --extern num_traits=/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/target/release/deps/libnum_traits-62d94157db5bd7b9.rlib \
  --extern serde_json=/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/target/release/deps/libserde_json-23322b031c2a5860.rlib \
  --extern zopro_bn254=/home/ubuntu/src/zopro-perf-work/balanced-kh-production/frozen/candidate/libzopro_bn254-5778e0f87c7d66ba5e17a5f97bf5992c699ca4d2f3df952e07aca8ff03a40370.rlib \
  -o model > model-build.stdout.log 2> model-build.stderr.log
./model > model.json 2> model.stderr.log
