#!/usr/bin/env bash
# Scores a candidate set of Metal shaders against the CPU: the bench's gpu
# mode runs the dumped shape's G1 MSMs (B, Z; random and skewed scalars)
# with the candidate kernels, asserts their sums equal the CPU's, and
# prints each MSM's device time and its first/tree/reduce split.
#
#   tools/kernel-lab/eval.sh <shader dir> <dump dir>
#
# A candidate directory holds field.metal, curve.metal, buckets.metal and
# quotient.metal (start from a copy of crates/zopro-metal/shaders). Without
# a shader directory ("-") the embedded kernels run: the baseline.
set -euo pipefail
shaders="${1:?shader dir or -}"; dump="${2:?dump dir}"
bench="${ZOPRO_BENCH:-$(dirname "$0")/../../target/release/zopro-bench}"
if [[ "$shaders" != "-" ]]; then export ZOPRO_METAL_SHADERS="$(cd "$shaders" && pwd)"; fi
out="$("$bench" gpu "$dump" 2>&1)" || { echo "$out" | grep -E "panicked|differs|shader:|error" | head -5; echo "FAIL"; exit 1; }
echo "$out" | grep -E '^ZOPRO gpu ' | sed -E 's/.*gpu "([^"]+)".*device_ms=([0-9.]+).*/\1: device \2 ms/' | tr '\n' ' '; echo
echo "$out" | grep -E '^ZOPRO gpu-profile' | sed -E 's/ZOPRO gpu-profile "([^"]+)" first ([0-9.]+) ms.*tree ([0-9.]+) ms.*reduce ([0-9.]+) ms.*/  \1: first \2 tree \3 reduce \4/'
