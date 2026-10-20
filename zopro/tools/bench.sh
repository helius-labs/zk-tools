#!/usr/bin/env bash
# Benches zopro on this computer with Zolana's keys and request fixtures,
# shape by shape, as the mobile SDK proves: each key's image mapped, written
# from the key file first (a key's first load), every proof verified.
#
#   tools/bench.sh [shapes] [rounds]
#
# Shapes are NxM transfers and merge-Nx1 merges with a request in
# tools/zolana/fixtures (default 1x2,2x2,4x2,8x2,12x2,16x2,24x2); rounds,
# proofs per shape (default 5). The keys download once into
# $ZOPRO_KEYS_DIR (default ~/.cache/zopro-keys) from $ZOPRO_KEYS_URL (the
# key set's CDN) and are checked against tools/zolana/zopro-keys.lock. On
# macOS the MSMs and the quotient run on Metal. KEEP_IMAGES=1 keeps the
# images of an earlier run, so no shape measures a first load.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(dirname "$here")"
shapes="${1:-1x2,2x2,4x2,8x2,12x2,16x2,24x2}"
rounds="${2:-5}"
lock="$here/zolana/zopro-keys.lock"
fixtures="$here/zolana/fixtures"
keys="${ZOPRO_KEYS_DIR:-$HOME/.cache/zopro-keys}"
cdn="${ZOPRO_KEYS_URL:-https://d3gbdb0egjwcw9.cloudfront.net}"

features=()
[[ "$(uname)" == Darwin ]] && features=(--features metal)
log="$(mktemp "${TMPDIR:-/tmp}/zopro-bench.XXXXXX")"
trap 'rm -f "$log"' EXIT
# The build's and the bench's own output only when they fail.
(cd "$root" && cargo build --quiet --release -p zopro-bench "${features[@]}" 2>"$log") || { cat "$log" >&2; exit 1; }
bench="${CARGO_TARGET_DIR:-$root/target}/release/zopro-bench"

sha256() { if command -v sha256sum >/dev/null; then sha256sum "$1"; else shasum -a 256 "$1"; fi | cut -d' ' -f1; }
# The lockfile's prefix, and a key's file and sha256.
pin() { python3 -I -c 'import json, sys
lock = json.load(open(sys.argv[1]))
if len(sys.argv) == 2:
    print(lock["prefix"])
else:
    entry = lock["keys"][sys.argv[2]]
    print(entry["file"], entry["sha256"])' "$lock" "$@"; }

mkdir -p "$keys"
prefix="$(pin)"
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
  read -r file sha <<<"$(pin "$key")"
  path="$keys/$file"
  if [[ ! -f "$path" || "$(sha256 "$path")" != "$sha" ]]; then
    echo "downloading $file" >&2
    curl -fsSL --retry 3 -o "$path.partial" "$cdn/$prefix/$file"
    [[ "$(sha256 "$path.partial")" == "$sha" ]] || { echo "$file: sha256 mismatch" >&2; exit 1; }
    mv "$path.partial" "$path"
  fi
  [[ "${KEEP_IMAGES:-}" == 1 ]] || rm -f "${path%.zopro}.image"
  line="$("$bench" zolana "$path" "$request" "$rounds" 2>"$log" | grep '^ZOPRO zolana')" || { cat "$log" >&2; exit 1; }
  field() { sed -E "s/.* $1=([^ ]+).*/\1/" <<<"$line"; }
  row="| $shape | $(field write_ms) | $(field write_peak_mb) | $(field load_ms) | $(field held_mb) | $(field peak_mb) | $(field witness_ms) | $(field prove_ms_median) |"
  rows+=("$row")
  echo "$row" >&2
done

echo
echo "zopro on $(uname -sm), $rounds proofs a shape, memory the process's own (MB), times in ms"
echo
echo "| shape | first load | first load peak | load | key held | proof peak | witness | proof (median) |"
echo "|---|---|---|---|---|---|---|---|"
printf '%s\n' "${rows[@]}"
