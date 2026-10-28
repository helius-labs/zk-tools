#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
deps=/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps
args=(--edition=2021 -O -L "dependency=$deps"
  --extern "ark_bn254=$deps/libark_bn254-d9ceb28748a3bfd8.rlib"
  --extern "ark_ec=$deps/libark_ec-e4a9bfa1662bf7f1.rlib"
  --extern "ark_ff=$deps/libark_ff-9216c709978bd3d2.rlib"
  --extern "num_bigint=$deps/libnum_bigint-158c502562b8bd18.rlib"
  --extern "serde_json=$deps/libserde_json-9c41682dbe4e057f.rlib"
  --extern "sha2=$deps/libsha2-b36128d04ec91f93.rlib")
rustc "${args[@]}" --test screen.rs -o tests > test-build.log 2>&1
./tests --nocapture > tests.log 2>&1
rustc "${args[@]}" screen.rs -o screen > model-build.log 2>&1
./screen /home/ubuntu/src/zk-tools > result.json 2> model-run.log
