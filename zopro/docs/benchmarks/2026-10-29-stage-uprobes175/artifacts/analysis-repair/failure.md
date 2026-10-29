# Offline perf-header parser repair

All captures and independent verifications completed before this repair.
The original `driver finish` stopped on a `# cmdline` header containing the
owned event-group name, treating it as an event with a wrong column count.
No analysis or proof-ledger result was written by that stopped attempt.

The fixed reader skips comment headers. Original pinned driver/analyzer source
and executables remain unchanged. The separate repaired analyzer was compiled
after captures, solely for offline decoding/analysis. One added regression test
checks this exact header shape. All earlier six parser tests remain qualified
under their original sources; no SDK/test/capture/Gnark work is repeated.
