#!/bin/bash
set -euo pipefail
d=$PWD
fixture="$d/../full-c7a244/bundle";cases="$d/../early-diversity257/source-config.json"
mapfile -t envs < <(jq -r 'to_entries[]|.key+"="+.value' ../general-path265/config/env.json)
mkdir output
for b in {0..2}; do cp ../general-path265/bin/sdk "output/ref$b"; cp build/candidate-sdk "output/candidate$b"; done
shapes=(16x2 merge-24x1 24x2);arms=(ref off on)
for b in {0..2}; do
 for j in {0..2}; do
  shape=${shapes[(j+b)%3]}
  read -r image request imgh reqh pub < <(jq -r --arg s "$shape" '.cases[$s]|[.image,.request,.image_sha256,.request_sha256,.public_sha256]|@tsv' "$cases")
  [[ "$(sha256sum "$fixture/$image"|cut -c1-64)" == "$imgh" && "$(sha256sum "$fixture/$request"|cut -c1-64)" == "$reqh" ]]
  for t in {0..2}; do
   k=$(((t+b+j)%3)); if ((b==1)); then k=$((2-k)); fi
   arm=${arms[k]}; flag=0;bin=candidate$b; if [[ $arm == on ]]; then flag=1; elif [[ $arm == ref ]]; then bin=ref$b; fi
   n="$shape-b$b-$arm"
   env "${envs[@]}" RAYON_NUM_THREADS=16 ZOPRO_BENCH_EARLY_DIVERSITY=0 ZOPRO_BENCH_FRONTEND_WORKERS=0 "ZOPRO_BENCH_EULER_CENTER=$flag" taskset -c 0-15 "output/$bin" "$fixture/$image" "$fixture/$request" 11 "$d/output/$n" > "output/$n.log"
   ../general-path265/bin/verify "$fixture/$image" "$imgh" "output/$n/0.proof" "output/$n/0.public" "$pub" > "output/$n.verify"
  done
 done
done
