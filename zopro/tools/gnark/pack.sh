#!/usr/bin/env bash
# A zopro key from a gnark Groth16 BN254 constraint system and its keys:
# zopro-dump writes their parts, `zopro-bench pack` packs them.
#
#   tools/gnark/pack.sh <r1cs> <pk> <vk> <name> <out.zopro> [<metadata file>]
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
zopro="$(cd "$here/../.." && pwd)"
abs() { echo "$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"; }
r1cs="$(abs "${1:?r1cs}")"; pk="$(abs "${2:?pk}")"; vk="$(abs "${3:?vk}")"
name="${4:?key name}"; out="$(abs "${5:?output file}")"
work="$(mktemp -d "${TMPDIR:-/tmp}/zopro-pack.XXXXXX")"
trap 'rm -rf "$work"' EXIT
metadata=()
[[ -n "${6:-}" ]] && metadata=(-metadata "$(abs "$6")")
(cd "$here" && go run ./cmd/zopro-dump -r1cs "$r1cs" -pk "$pk" -vk "$vk" -out "$work/dump" ${metadata[@]+"${metadata[@]}"})
(cd "$zopro" && cargo run --release -q -p zopro-bench -- pack "$work/dump" "$name" "$out")
