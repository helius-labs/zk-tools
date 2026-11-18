#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
mapfile -t envs < <(jq -r 'to_entries[]|.key+"="+.value' ../general-path265/config/env.json)
cfg=../ccd-placement280/bundle/config/cases.json; fixture=../full-c7a244/bundle
shapes=(merge-24x1 16x2 24x2); modes=(accepted off 13 14)
for b in {0..4};do
 cp ../general-width279/bundle/bin/accepted "output/sdk$b"
 cp build/candidate-sdk "output/candidate$b"
 for j in {0..2};do
  sh=${shapes[(b+j)%3]}
  read -r im req imh reqh pub < <(jq -r --arg s "$sh" '.[$s]|[.image,.request,.image_sha256,.request_sha256,.public_sha256]|@tsv' "$cfg")
  [[ "$(sha256sum "$fixture/$im"|cut -c1-64)" == "$imh" && "$(sha256sum "$fixture/$req"|cut -c1-64)" == "$reqh" ]]
  for i in {0..3};do
   k=$(((b+j+i)%4)); if ((b%2)); then k=$((3-k));fi; mode=${modes[k]}
   bin=candidate$b; bits=0; if [[ $mode == accepted ]];then bin=sdk$b; elif [[ $mode != off ]];then bits=$mode;fi
   extra=("ZOPRO_BENCH_DENSE_H_BITS=$bits" ZOPRO_BENCH_EARLY_DIVERSITY=0 ZOPRO_BENCH_FRONTEND_WORKERS=0)
   n="$sh-b$b-$mode"
   env -u ZOPRO_CPU_LARGE_WIDTH -u ZOPRO_CPU_UNSIGNED_WIDTH "${envs[@]}" RAYON_NUM_THREADS=16 "${extra[@]}" taskset -c 0-15 "output/$bin" "$fixture/$im" "$fixture/$req" 11 "$PWD/output/$n" >"output/$n.log"
   ../general-path265/bin/verify "$fixture/$im" "$imh" "output/$n/0.proof" "output/$n/0.public" "$pub" >"output/$n.verify"
  done
 done
done
