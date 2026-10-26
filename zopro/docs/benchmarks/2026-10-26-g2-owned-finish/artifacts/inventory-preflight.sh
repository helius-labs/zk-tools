#!/usr/bin/env bash
set -euo pipefail
rg --files /tmp/zopro-g2-owned-finish/preflight-proofs | sort | xargs -r sha256sum | jq -Rn '[inputs|capture("^(?<hash>[a-f0-9]{64})  (?<path>.+)$")|{key:.path,value:.hash}]|from_entries' > /tmp/zopro-g2-owned-finish/preflight-proof-inventory.json
