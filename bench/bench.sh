#!/usr/bin/env bash
# Benches gnark, through the bridge in cgo/, on this computer with Zolana's
# keys and request fixtures, shape by shape, as zopro's tools/bench.sh does
# for zopro (on zk-tools' sergei/zopro branch), so the two tables compare:
# each key read onto the heap as the bridge reads it, every proof verified.
#
#   bench/bench.sh [shapes] [rounds]
#
# Shapes are NxM transfers and merge-Nx1 merges with a request in
# bench/fixtures (default 1x2,2x2,4x2,8x2,12x2,16x2,24x2); rounds, proofs
# per shape (default 5). The gnark keys download once into
# $GNARK_KEYS_DIR (default ~/.cache/gnark-keys) from $ZOLANA_KEYS_URL
# (the key set's folder on its CDN) and are checked against
# bench/proving-keys.lock.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(dirname "$here")"
shapes="${1:-1x2,2x2,4x2,8x2,12x2,16x2,24x2}"
rounds="${2:-5}"
lock="$here/proving-keys.lock"
fixtures="$here/fixtures"
keys="${GNARK_KEYS_DIR:-$HOME/.cache/gnark-keys}"
url="${ZOLANA_KEYS_URL:-https://d3gbdb0egjwcw9.cloudfront.net/proving-keys/3a1c88f90f609eb1}"
log="$(mktemp "${TMPDIR:-/tmp}/gnark-bench.XXXXXX")"
trap 'rm -f "$log"' EXIT

sha256() { if command -v sha256sum >/dev/null; then sha256sum "$1"; else shasum -a 256 "$1"; fi | cut -d' ' -f1; }
pin() { python3 -I -c 'import json, sys; print(json.load(open(sys.argv[1]))["keys"][sys.argv[2]]["sha256"])' "$lock" "$1"; }

# The test binary once: each shape then runs in a process of its own.
bench="$keys/gnark-bench.test"
mkdir -p "$keys"
(cd "$root/cgo" && go test -c -o "$bench" . 2>"$log") || { cat "$log" >&2; exit 1; }

header="| shape | load | load peak | key held | proof peak | witness | proof (median) |"
echo "$header" >&2
rows=()
for shape in ${shapes//,/ }; do
  if [[ "$shape" == merge-* ]]; then
    n="${shape#merge-}"
    key="merge_${n/x/_}.key"
    request="$fixtures/$shape.json"
  else
    key="transfer_confidential_${shape/x/_}.key"
    request="$fixtures/transfer-$shape.json"
  fi
  [[ -f "$request" ]] || { echo "no request for $shape in $fixtures" >&2; exit 1; }
  sha="$(pin "$key")"
  path="$keys/$key"
  if [[ ! -f "$path" || "$(sha256 "$path")" != "$sha" ]]; then
    echo "downloading $key" >&2
    curl -fsSL --retry 3 -o "$path.partial" "$url/$key"
    [[ "$(sha256 "$path.partial")" == "$sha" ]] || { echo "$key: sha256 mismatch" >&2; exit 1; }
    mv "$path.partial" "$path"
  fi
  line="$(GNARK_BENCH_KEY="$path" GNARK_BENCH_REQUEST="$request" GNARK_BENCH_ROUNDS="$rounds" \
    "$bench" -test.run '^TestBench$' -test.count=1 2>"$log" | grep '^GNARK zolana')" || { cat "$log" >&2; exit 1; }
  field() { sed -E "s/.* $1=([^ ]+).*/\1/" <<<"$line"; }
  row="| $shape | $(field load_ms) | $(field load_peak_mb) | $(field held_mb) | $(field peak_mb) | $(field witness_ms) | $(field prove_ms_median) |"
  rows+=("$row")
  echo "$row" >&2
done

echo
echo "gnark on $(uname -sm), $rounds proofs a shape, memory the process's own (MB), times in ms"
echo
echo "$header"
echo "|---|---|---|---|---|---|---|"
printf '%s\n' "${rows[@]}"
