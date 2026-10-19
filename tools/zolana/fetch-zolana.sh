#!/usr/bin/env bash
# Checks out zolana's prover at the release the keys come from into
# tools/gnark/zolana (gitignored), which go.mod builds against: at this
# release its Go module still calls itself zolana/prover.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository="https://github.com/helius-labs/zolana.git"
revision="0fde5f1e1e209b85896362c9293ec0b60bbf7563" # v0.4.0-alpha
dir="$here/zolana"
if [[ ! -d "$dir/.git" ]]; then
  git init -q "$dir"
  git -C "$dir" remote add origin "$repository"
fi
git -C "$dir" fetch -q --depth=1 origin "$revision"
git -C "$dir" checkout -q --detach FETCH_HEAD
echo "zolana $(git -C "$dir" rev-parse --short HEAD) in $dir"
