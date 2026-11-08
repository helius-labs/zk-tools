# Retained helper repairs

The native field build, new test link, both new tests and generic field compile each succeeded on the first execution. The symbol listing and one full disassembly also succeeded once. No arithmetic/test/decoder change or replay followed.

Two offline helper compiles needed ordinary Rust type fixes before execution: qualify initially compared &String to serde_json::Value, repaired to a string comparison; review initially inferred BTreeMap<&String,...>, repaired to owned String keys. Initial helper sources are preserved. Compiler diagnostics are in the tool transcript rather than separately archived raw stderr files.

The first compiled offline reader execution found three rather than four coverage rows because libtest prints the first LINEAR_NATIVE record after its test-name prefix on the same line. Only the reader changed from strip_prefix to split_once for that marker. The pre-repair source is retained. Its assertion diagnostic is in the tool transcript rather than an independently archived stream; this note is a summary, not the raw receipt. The repaired reader passes without rerunning any test, build, oracle or decoder. No statistical or performance gate exists in this correctness-only stage.
