#!/usr/bin/env bash
# Builds a zopro key for every confidential transfer and merge key of a
# gnark key set, and the zopro-keys.lock that pins them: downloads each
# gnark key from the key set's CDN folder and checks it against the gnark
# lockfile, writes its parts and Zolana metadata with TestZoproDump, packs
# them with `zopro-bench pack`, and deletes the key and the parts (one key
# at a time, for the disk). Keys already in <out> are kept.
#
#   tools/zolana/build-keys.sh <gnark proving-keys.lock> <out> [<CDN base>]
#
# The CDN base defaults to the Zolana key CDN; the lockfile's prefix names
# the folder. Needs ./fetch-zolana.sh first.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
zopro="$(cd "$here/../.." && pwd)"
gnark_lock="$(cd "$(dirname "${1:?gnark proving-keys.lock}")" && pwd)/$(basename "$1")"
out="$(mkdir -p "${2:?output directory}" && cd "$2" && pwd)"
cdn="${3:-https://d3gbdb0egjwcw9.cloudfront.net}"
prefix="$(python3 -c 'import json, sys; print(json.load(open(sys.argv[1]))["prefix"])' "$gnark_lock")"
work="$(mktemp -d "${TMPDIR:-/tmp}/zopro-keys.XXXXXX")"
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/keys" "$work/dump"

(cd "$zopro" && cargo build --release -q -p zopro-bench)
bench="${CARGO_TARGET_DIR:-$zopro/target}/release/zopro-bench"

python3 - "$gnark_lock" <<'PY' > "$work/list"
import json, re, sys
for name, entry in sorted(json.load(open(sys.argv[1]))["keys"].items()):
    if re.fullmatch(r"(transfer_confidential_\d+_\d+|merge_\d+_1)\.key", name):
        print(name[:-4], entry["sha256"])
PY
while read -r name sha256; do
  if [[ -f "$out/$name.zopro" ]]; then continue; fi
  case "$name" in
    merge_*) n="${name#merge_}"; shape="merge-${n%_1}x1" ;;
    *) rest="${name#transfer_confidential_}"; shape="${rest%_*}x${rest##*_}" ;;
  esac
  curl -sSf -o "$work/keys/$name.key" "$cdn/$prefix/$name.key"
  got="$(shasum -a 256 "$work/keys/$name.key" | cut -d' ' -f1)"
  [[ "$got" == "$sha256" ]] || { echo "$name.key does not match the gnark lockfile" >&2; exit 1; }
  (cd "$here" && ZOPRO_DUMP="$work/dump" ZOPRO_KEYS="$work/keys" ZOPRO_SHAPES="$shape" \
    go test -count=1 -timeout 60m -run TestZoproDump . > "$work/dump.log" 2>&1) ||
    { tail -20 "$work/dump.log" >&2; exit 1; }
  "$bench" pack "$work/dump/$shape" "$name" "$out/$name.zopro" | grep "ZOPRO pack"
  rm -rf "$work/keys/$name.key" "$work/dump/$shape"
done < "$work/list"

python3 - "$gnark_lock" "$out" "$cdn" <<'PY'
import hashlib, json, os, re, sys
gnark_lock, out, cdn = sys.argv[1:]
gnark = json.load(open(gnark_lock))
keys = {}
for name, entry in sorted(gnark["keys"].items()):
    if not re.fullmatch(r"(transfer_confidential_\d+_\d+|merge_\d+_1)\.key", name):
        continue
    file = name[:-4] + ".zopro"
    data = open(os.path.join(out, file), "rb").read()
    # The metadata blob, after the 564-byte fixed part and the name's blob.
    at = 564
    at += 4 + int.from_bytes(data[at:at + 4], "little")
    size = int.from_bytes(data[at:at + 4], "little")
    if json.loads(data[at + 4:at + 4 + size]).get("keySha256") != entry["sha256"]:
        sys.exit(f"{file} was not built from the pinned {name}")
    keys[name] = {"file": file, "size": len(data), "sha256": hashlib.sha256(data).hexdigest(), "key_sha256": entry["sha256"]}
lock = {"prefix": gnark["prefix"], "baseUrl": f"{cdn}/{gnark['prefix']}/", "keys": keys}
with open(os.path.join(out, "zopro-keys.lock"), "w") as f:
    f.write(json.dumps(lock, indent=2) + "\n")
print(f"{len(keys)} zopro keys and zopro-keys.lock in {out}")
PY
