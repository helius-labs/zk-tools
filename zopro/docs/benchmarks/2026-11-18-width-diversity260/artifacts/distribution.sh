#!/usr/bin/env bash
set -euo pipefail
root=${1:?provide the directory containing the b0 through b6 workflow captures}
for block in {0..6}; do
    for arm in accepted parent neither width cert both both32 aeglos; do
        jq -Rn --arg arm "$arm" --argjson block "$block" '
          [inputs | select(startswith("SAMPLE ")) | ltrimstr("SAMPLE ") | fromjson] as $samples
          | if ($samples|map(.round)) != [range(151)] then error("incomplete timed series") else
             {arm:$arm,block:$block,wall:[$samples[] | .verified_ms // .ms]} end
        ' "$root/b$block/b$block-$arm/stdout.log"
    done
done | jq -s '
  group_by(.arm) | map(
    . as $series | (map(.wall[])|sort) as $x | ($x|length) as $n |
    if $n!=1057 then error("incomplete arm") else
      {arm:.[0].arm,n:$n,p50:$x[($n*0.50|ceil)-1],p95:$x[($n*0.95|ceil)-1],
       p99:$x[($n*0.99|ceil)-1],half_ms_histogram:($x|group_by(. * 2 | floor)|map({lower_ms:((.[0]*2|floor)/2),count:length})),
       within_9ms:([$x[]|select(.<=9)]|length)} end
  )'
