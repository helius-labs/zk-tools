# CPU experiment workflow

`xtask` prepares isolated source snapshots, builds and tests candidates, links
SDK drivers from exact Cargo artifacts, and measures separate-process latency
and warm Linux `perf` captures. The Cargo alias uses a release build to hash
large key images efficiently.

```sh
cargo test --manifest-path xtask/Cargo.toml --locked --offline
cargo xtask prepare /absolute/accepted-manifest.json /absolute/source /absolute/candidate /absolute/overlay
cargo xtask workflow /absolute/build-plan.json /absolute/build-results
cargo xtask workflow /absolute/build-plan.json /absolute/build-results --resume
cargo xtask perf /absolute/experiment.json /absolute/results
cargo xtask perf /absolute/experiment.json /absolute/results --resume
cargo xtask instructions /absolute/instructions.json /absolute/instructions-results
```

The task has its own workspace and lockfile. Its dependencies do not enter the
prover's Android, Apple or WebAssembly dependency graph. Prover builds happen
before evaluation; the protocol pins their executables and relevant inputs.
The Linux CI job checks this task's tests, formatting and Clippy. Performance
measurements run separately on a suitable dedicated host.

## Prepare and build

`prepare` reads a manifest's `source_files_sha256` map, verifies every source
file and copies only those files into a new directory. An optional overlay
replaces or adds explicit candidate files. It rejects path escapes, overlay
symlinks, changed inputs and an existing destination. The snapshot records the
base manifest digest, overlay digests and absolute source pins under `.xtask/`.

`workflow` executes declared commands and SDK links in order, with source
checks between steps and retained command, environment, timing and output
receipts. For example:

```json
{
  "version": 1,
  "source_pins": "/absolute/candidate/.xtask/source-pins.json",
  "environment": {
    "RUSTFLAGS": "-C target-cpu=native",
    "CARGO_TARGET_DIR": "/absolute/candidate-target",
    "RAYON_NUM_THREADS": "4"
  },
  "steps": [
    {
      "kind": "command", "name": "core",
      "argv": ["cargo", "test", "--manifest-path", "/absolute/candidate/Cargo.toml", "--locked", "--offline", "--release", "-p", "zopro-core", "--lib", "--", "--test-threads=1"],
      "required_tests": ["msm_affine::grouping::tests::stable_groups_match_independent_ordered_value_index"],
      "unfiltered": true
    },
    {
      "kind": "command", "name": "libraries",
      "argv": ["cargo", "build", "--manifest-path", "/absolute/candidate/Cargo.toml", "--locked", "--offline", "--release", "-p", "zopro-bench", "--message-format=json"]
    },
    {
      "kind": "link-sdk", "name": "candidate",
      "artifacts": "/absolute/build-results/libraries/stdout.log",
      "driver": "/absolute/source/crates/zopro-bench/examples/cpu-sdk.rs",
      "additional_sources": ["/absolute/source/crates/zopro-bench/examples/support/perf.rs"]
    }
  ]
}
```

Required tests are checked by name, so a filtered invocation cannot pass by
matching an old test count. `unfiltered` also requires a full-suite summary.
For named tests with `--nocapture`, use `--test-threads=1`; the parser accepts
diagnostics between a test name and its completion marker.
Use separate plans and target directories for portable qualification and final
native builds; focused builds may use faster code generation without making
latency claims about that configuration.

`link-sdk` selects the eight required libraries from Cargo JSON output and
rejects missing or ambiguous artifacts. It defaults to native CPU, fat LTO and
one codegen unit; override `rustc_flags` explicitly for a portable driver.
The resulting `NAME-sdk` has an `artifacts.json` receipt with its digest and
all direct source/library input digests. Resume checks the original source
pin set, completed command receipts and frozen link inputs before reuse.
Commands default to a 600-second timeout and may set `timeout_seconds`.
Each attempt records the orchestrator digest. Resuming preserves an earlier
failure under an attempt-specific name before revalidating completed steps.

## Protocol

The first variant is the baseline. Three or more blocks rotate/reverse variants
and case order. Every latency process finishes before profiling begins. An
exclusive host lock prevents two instances of this task from overlapping.
Other benchmark launchers must be kept idle separately.
An optional case `cpu_affinity` uses an explicit Linux CPU list such as
`"0-15"` through `taskset`, independently of the Rayon worker count. Confirm
the host's CPU/core mapping before using a list to select physical cores.

The driver is `crates/zopro-bench/examples/cpu-sdk.rs`:

```text
cpu-sdk IMAGE REQUEST_JSON ROUNDS [PROOF_DIRECTORY]
```

It loads the image, makes three priming requests and then measures complete
verified SDK requests. The sample vector is allocated before capture. Optional
`ZOPRO_BENCH_PERF_CTL` and `ZOPRO_BENCH_PERF_ACK` FIFOs enable/disable profiling
around the warm request loop with explicit acknowledgements. Loading, priming,
proof export and result formatting are outside capture. FIFO acknowledgements
have a bounded wait; invalid channels or replies fail the run.

SDK output contains ordered `SAMPLE` records and one `SUMMARY` with the actual
Rayon worker count, warm CPU/wall accounting, context switches, faults and
whole-process peak RSS. RSS includes loading and priming; it is not minimum RAM.
CPU divided by wall time counts occupied logical CPUs, including spinning.

Set `"monotonic_clock": true` when correlating captures with Linux
`CLOCK_MONOTONIC` phase stamps. It passes `--clockid mono` to `perf record`;
the default perf clock can have a different origin. Existing protocols keep
their original clock unless this option is explicit. Check actual sample
coverage of request intervals before drawing conclusions from attribution.

Container profiling also needs a bounded locked-memory allowance for the perf
rings. At 96 CPUs the current buffer setting works with a 256 MiB container
memlock limit. This allowance belongs to the observer; keep its memory scope
separate from the SDK's process RSS and from prover memory-pressure tests.

Use the following structure, replacing paths and hashes with actual artifacts:

```json
{
  "version": 1,
  "blocks": 3,
  "profile_blocks": 3,
  "profile_cases": ["primary32"],
  "command": ["{binary}", "{image}", "{request}", "{rounds}", "{proof_dir}/{stem}"],
  "variants": [
    {"name": "baseline", "binary": "/absolute/baseline-sdk"},
    {"name": "candidate", "binary": "/absolute/candidate-sdk", "environment": {"ZOPRO_EXPERIMENT": "1"}}
  ],
  "cases": [{
    "name": "primary32", "shape": "1x2", "workers": 32,
    "rounds": 31, "profile_rounds": 101,
    "image": "/absolute/transfer_confidential_1_2.image",
    "request": "/absolute/transfer-1x2.json",
    "public_sha256": "ESTABLISHED_PUBLIC_INPUT_SHA256",
    "reference": {"cpu_ms": 37, "metal_ms": 24}
  }],
  "pins": {
    "/absolute/baseline-sdk": "EXECUTABLE_SHA256",
    "/absolute/candidate-sdk": "EXECUTABLE_SHA256",
    "/absolute/verifier": "VERIFIER_SHA256",
    "/absolute/transfer_confidential_1_2.image": "IMAGE_SHA256",
    "/absolute/transfer-1x2.json": "REQUEST_SHA256"
  },
  "checks": [["/absolute/focused-test-binary", "--test-threads=1"]],
  "verifier": ["/absolute/verifier", "/absolute/keys", "{proof_dir}", "/absolute/keys.lock"],
  "environment": {"ZOPRO_BASIS_KEY_DIR": "/absolute/images"},
  "profile_environment": {
    "ZOPRO_BENCH_PERF_CTL": "{control}",
    "ZOPRO_BENCH_PERF_ACK": "{ack}"
  },
  "symbol_categories": [
    {"name": "grouping", "contains": ["scalar_groups", "hashbrown"]},
    {"name": "scheduler", "contains": ["rayon_core", "crossbeam_deque"]}
  ]
}
```

Pin the source snapshot, build receipts, libraries, key lock and expected public
fixture as well when using the output as experiment evidence. The verifier
command must independently verify the final export and fail on invalid proofs.
Every process must export one fresh proof matching the established public-input
digest. `prior_proof_sha256` can include previous exports to extend freshness
checks across experiments.

Files receive full content-hash checks at the beginning and end of evaluation.
Between processes the task checks device, inode, length and nanosecond
modification/change timestamps. It records this policy in `pin-policy.json`.
This avoids rehashing large images before every request process while still
detecting changes to experiment inputs.

Only explicitly declared `ZOPRO_`, `RAYON_`, `MALLOC_` and `CROSS_` variables
and declared Rust build variables are retained from those managed variable
families; unrelated system environment remains available. Profile-control
variables cannot enter latency configuration. Commands are argument arrays
and execute without a shell.

On hosts that require privileged perf access, the protocol can specify:

```json
{
  "perf_prefix": ["sudo", "-n"],
  "child_prefix": ["sudo", "-n", "-u", "ubuntu"]
}
```

These prefixes are explicit protocol inputs. The task does not change kernel
profiling settings. `timeout_seconds` defaults to 90; a timeout wrapper also
bounds privileged descendants. Performance tests require a suitable dedicated
host and should not become timing gates on shared CI runners.

## Results and resume

`report.md` and `result.json` compare paired latency, warm process CPU time,
RSS and sampled symbol categories. `rows.json` preserves all observations.
`profile_blocks` defaults to three, independently of latency block count;
`profile_cases` defaults to every case. An optional case `reference` adds
CPU and Metal ratios to the report. These are supplied reference measurements,
not fresh measurements of those devices.
Every profile retains its raw capture and decoded symbols. Decoding includes
PID/TID/IP, demangles Rust symbols and rejects empty, malformed, mixed-process
or lost-sample data. Categories use ordered substring matches and include all
unclassified weight under `other`. Inlined work can appear under enclosing
symbols; cycle shares are not wall-time fractions or removable latency.

`--resume` requires the same protocol and unchanged artifact pins. It checks
command arguments, benchmark environment and stdout/stderr hashes before
reusing a successful process. It reparses the existing output and captures;
completed proof generation is not repeated. A failed or unfinished benchmark
is preserved and is not silently restarted. Failed decoding can be repaired
against the retained capture. Attempt records retain the orchestrator digest.

The task reports evidence, not an adoption decision. Prospective performance
gates and portable, concurrency, small-stack, pressure and large-circuit
qualification still belong to each optimization's protocol. Keep fast focused
tests separate from final optimized latency builds. Existing full-suite checks
remain required where a change can affect their behavior.

The initial integration exercised two frozen library sets with 744 real SDK
requests, 12 independently verified exports and six warm captures. The first
decoder omitted IP and could not resolve symbols; its original output is
retained, and resume decoded the same capture without repeating successful
SDK measurements.

## Saved instruction profiles

`instructions` reads existing Linux x86-64 captures; it does not run the prover
or collect new samples. Its version-1 configuration declares `binaries` with
`name`, `path`, `disassembly` and `symbols`, and `runs` with `name`, `binary`,
`metadata`, `samples`, `expected_pid`, `expected_samples`, `expected_period`
and `expected_tids`. Every input path must be absolute and present in the
configuration's `pins` map. See the retained [configuration and receipts](../docs/benchmarks/2026-10-26-instruction-census/README.md)
for a complete example.

Generate disassembly with `objdump -d -w --insn-width=16 BINARY` and symbol
ranges with `nm -S -C --defined-only BINARY`. The sample stream uses
`perf script -i PERF_DATA -F pid,tid,period,ip,sym`; metadata uses the same
command with `--show-mmap-events --show-task-events`. These extraction commands
can run through `workflow` for pinned inputs and command receipts.

The reader uses the sampled process's executable MMAP2 records and ELF PT_LOAD
file/virtual offsets. Every disassembled instruction must match the executable
bytes, and every sampled executable IP must match an exact instruction boundary.
External addresses, symbol aliases and unclassified code remain explicit.
Counts and period weights must match the expected capture totals and conserve
across instruction, address-register, symbol and scope bins.

The output contains per-IP JSONL, per-run summaries and a combined result.
Instruction families and memory-operand tags are separate views of the same
samples. Cycle samples may skid; these are not exclusive opcode costs, retired
instruction counts or removable latency. RSP/RBP operands do not establish
spills or actual memory traffic; RBP can be a general register. This offline
command can run inside `workflow` and does not acquire a second host lock.

## Existing scripts

The maintained CPU experiment loop belongs here. The Python files under
`docs/benchmarks/` are historical experiment artifacts; converting them would
change the recorded evidence. `scripts/poseidon_fixtures.py` is an independent
big-integer arithmetic oracle. The IFMA and Metal scripts generate code.
Those have separate purposes from orchestration.

The key preparation and cross-platform quick-benchmark shell scripts still
have documented uses. This task does not yet replace Android/iOS device
control, key conversion or `tools/bench.sh`.
