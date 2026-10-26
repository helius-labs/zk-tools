#!/usr/bin/env bash
set -euo pipefail
rg --files /tmp/zopro-serial-fft-leaves/preflight-proofs | sort | xargs -r sha256sum | jq -Rn '[inputs|capture("^(?<hash>[a-f0-9]{64})  (?<path>.+)$")|{key:.path,value:.hash}]|from_entries' > /tmp/zopro-serial-fft-leaves/preflight-proof-inventory.json
