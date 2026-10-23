#!/usr/bin/env bash
# Builds, signs and installs the iOS bench app on a cabled device:
#
#   tools/ios-bench/build.sh <device UDID> <provisioning profile> [cargo features]
#
# The profile is a development profile that lists the device (an Xcode-
# managed one works); the app takes its bundle identifier, and the signing
# identity is the first Apple Development one in the keychain. Features
# default to `metal,zopro-core/trace`.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
zopro="$(cd "$here/../.." && pwd)"
device="${1:?device UDID}"; profile="${2:?provisioning profile}"
features="${3:-metal,zopro-core/trace}"
target="${CARGO_TARGET_DIR:-$zopro/target}"
work="$(mktemp -d "${TMPDIR:-/tmp}/zopro-ios.XXXXXX")"
trap 'rm -rf "$work"' EXIT

security cms -D -i "$profile" > "$work/profile.plist"
app_id="$(/usr/libexec/PlistBuddy -c 'Print :Entitlements:application-identifier' "$work/profile.plist")"
team="$(/usr/libexec/PlistBuddy -c 'Print :TeamIdentifier:0' "$work/profile.plist")"
bundle_id="${app_id#"$team".}"
identity="$(security find-identity -v -p codesigning | grep -m1 'Apple Development' | awk '{ print $2 }')"

(cd "$zopro" && CARGO_TARGET_DIR="$target" cargo build --release -q -p zopro-bench --lib \
  --target aarch64-apple-ios --features "$features")
app="$work/ZoproBench.app"
mkdir -p "$app"
sdk="$(xcrun --sdk iphoneos --show-sdk-path)"
xcrun --sdk iphoneos clang -target arm64-apple-ios16.0 -isysroot "$sdk" -fobjc-arc -O2 \
  "$here/main.m" "$target/aarch64-apple-ios/release/libzopro_bench.a" \
  -framework UIKit -framework Foundation -framework Metal -framework QuartzCore \
  -framework CoreGraphics -framework IOSurface -lc++ -lresolv -o "$app/ZoproBench"
sed "s/BUNDLE_ID/$bundle_id/" "$here/Info.plist" > "$app/Info.plist"
cp "$profile" "$app/embedded.mobileprovision"
cat > "$work/entitlements.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>application-identifier</key><string>$app_id</string>
  <key>com.apple.developer.team-identifier</key><string>$team</string>
  <key>get-task-allow</key><true/>
</dict></plist>
PLIST
codesign -f -s "$identity" --entitlements "$work/entitlements.plist" "$app"
xcrun devicectl device install app --device "$device" "$app" | tail -2
echo "$bundle_id" > "$here/.bundle-id"
