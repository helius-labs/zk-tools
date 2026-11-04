#!/usr/bin/env bash
# A zopro key from an Xark circuit's artifacts: zopro-xark writes the rows
# its proving key was made for, `zopro-bench pack-xark` packs them with the
# proving and verifying keys.
#
#   tools/xark/pack.sh <circuit dir> <name> <out.zopro> [<metadata file>]
#
# The directory holds circuit.xbc, pk.bin and vk.bin (`xark build` and
# `xark setup` write them to target/xark/<circuit>/), and r1cs.stream when
# `xark setup` wrote one beside a minimized key.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
zopro="$(cd "$here/../.." && pwd)"
abs() { echo "$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"; }
dir="$(cd "${1:?circuit dir}" && pwd)"
name="${2:?key name}"; out="$(abs "${3:?output file}")"
work="$(mktemp -d "${TMPDIR:-/tmp}/zopro-xark.XXXXXX")"
trap 'rm -rf "$work"' EXIT
metadata=()
[[ -n "${4:-}" ]] && metadata=("$(abs "$4")")
(cd "$here" && cargo run --release -q -- rows "$dir/circuit.xbc" "$dir/pk.bin" "$work/r1cs.stream")
(cd "$zopro" && cargo run --release -q -p zopro-bench -- pack-xark "$dir/pk.bin" "$dir/vk.bin" \
  "$work/r1cs.stream" "$name" "$out" ${metadata[@]+"${metadata[@]}"})
