#!/usr/bin/env bash
# Checks out zolana's prover at the release the keys come from into
# tools/zolana/zolana (gitignored), which go.mod builds against in place of
# its Go module, github.com/helius-labs/zolana/prover/server.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository="https://github.com/helius-labs/zolana.git"
revision="88854ef0429d318f0b27d72e17bbce0fdc67fa70" # v0.5.0-alpha
dir="$here/zolana"
if [[ ! -d "$dir/.git" ]]; then
  git init -q "$dir"
  git -C "$dir" remote add origin "$repository"
fi
git -C "$dir" fetch -q --depth=1 origin "$revision"
git -C "$dir" checkout -q --detach FETCH_HEAD
echo "zolana $(git -C "$dir" rev-parse --short HEAD) in $dir"
