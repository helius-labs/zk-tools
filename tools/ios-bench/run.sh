#!/usr/bin/env bash
# Runs the installed bench app on a cabled device with the given arguments
# and prints its output:
#
#   tools/ios-bench/run.sh <device UDID> dump/2x2 5
#   tools/ios-bench/run.sh <device UDID> --push <dump dir> dump/<name>
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
device="${1:?device UDID}"; shift
bundle_id="$(cat "$here/.bundle-id")"
if [[ "${1:-}" == "--push" ]]; then
  for file in "$2"/*; do
    xcrun devicectl device copy to --device "$device" --domain-type appDataContainer \
      --domain-identifier "$bundle_id" --source "$file" \
      --destination "Documents/$3/$(basename "$file")" >/dev/null
  done
  echo "pushed $2 to Documents/$3"
  exit 0
fi
xcrun devicectl device process launch --device "$device" --console --terminate-existing \
  "$bundle_id" "$@" 2>&1 | grep -v -E '^(Launched application|Waiting for the application|Press Control-C)'
