#!/usr/bin/env bash
set -u
root=/tmp/zopro-instruction-census
"$root/target/release/xtask" instructions "$root/invalid-hash-config.json" "$root/invalid-hash-output" > "$root/invalid-hash.stdout.log" 2> "$root/invalid-hash.stderr.log"
check_status=$?
jq -n --argjson returncode "$check_status" '{returncode:$returncode,expected:"hash mismatch, before output directory creation"}' > "$root/invalid-hash.exit.json"
test "$check_status" -ne 0 && test ! -e "$root/invalid-hash-output" && rg -q 'artifact changed while hashing' "$root/invalid-hash.stderr.log"
