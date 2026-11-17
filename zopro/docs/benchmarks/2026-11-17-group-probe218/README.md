# Borrowed hash probe with owned entries218

Hashing the input scalar before creating an owned lookup key improves the
Hashbrown control by **2.17%** in complete SDK latency. It remains **0.56%
slower** than the retained standard-map path and loses all three blocks against
that path. The fixed screen fails and this implementation is not adopted.
The positive result against its immediate control is retained in full.

|Block|STD ms|HB ordinary entry ms|RAW prehashed entry ms|RAW/STD|RAW/HB|
|---:|---:|---:|---:|---:|---:|
|0|7.522904|7.691454|7.524900|1.000265|0.978346|
|1|7.534667|7.769138|7.594309|1.007916|0.977497|
|2|7.573924|7.761483|7.616071|1.005565|0.981265|

Entries are process means over251 timed complete SDK requests, following three
primes. Paired medians are1.005565 against STD and0.978346 against HB. The
corresponding CPU ratios are1.004160 and0.990367. Peak RSS is129.25MiB RAW,
126.25MiB STD and126.75MiB HB; both declared memory guards pass. These are
process high-water observations, not minimum RAM requirements.

The fresh HB/STD calibration median is1.024764, within the frozen3% limit. The
screen has no minimum gain floor: it requires a positive median and at least
two of three wins against **both** controls, plus CPU and RSS guards. Failure
against STD is a performance result; correctness and resource checks pass.
There is no unchanged timing extension or alternate control selection.

## Intervention and generated code

The earlier216 profile concentrated extra running CPU time immediately after
a32-byte scalar-to-stack copy before hashing. The217 debugger audit found the
same spill alignment in both inspected call paths. This experiment removes
the copy from the hashing dependence without changing the owned key/value
representation, equality, hash algorithm, seed policy, admission, cutoff or
stable output order.

Both new builders use Hashbrown0.15.5 with AHash and40-byte key/value entries.
HB calls ordinary `entry(*scalar)`. RAW computes the hash from the existing
scalar reference, searches with that hash and only copies an owned key into a
vacant entry. Hashbrown documents the requirement that supplied hashes match
stored keys, including during growth; the tests exercise that invariant.
[Hashbrown0.15.5 raw-entry source](https://github.com/rust-lang/hashbrown/blob/v0.15.5/src/raw_entry.rs).

Emitted-code inspection before SDK capture confirms the intended change. HB
stores the32-byte key at `[rsp+0x40]` and reloads both halves before hashing;
RAW feeds hashing from direct128-bit loads at `[r12]` and `[r12+0x10]`.
Both loops and their complete builders are retained in the source archive.
Removing this dependence helps the HB control here; it does not prove a
specific store-forwarding, cache or predictor cause for the earlier216 gap.

|Block|STD G2 group cycles|HB cycles|RAW cycles|STD/HB/RAW retired instructions|
|---:|---:|---:|---:|:---|
|0|4,555,207|6,121,183|5,064,612|3,615,032 / 4,155,985 / 3,842,721|
|1|4,500,351|6,249,424|4,786,837|3,614,771 / 4,156,316 / 3,842,443|
|2|4,468,829|5,880,412|4,925,601|3,615,158 / 4,155,956 / 3,842,568|

These are per-process group medians. RAW improves group CPU and cycles over HB
but still executes more instructions than STD. The new STD context has CPI
about1.24–1.26, so the older slow Boolean-OFF context is not reproduced by this
artifact. Code generation and call context changed. This is not a controlled
speedup over216 or accepted194, and the old group gap cannot be treated as
guaranteed recoverable proof time.

The default standard-map construction remains byte-for-byte intact after
removing the new dispatch and module declaration. Relative to216,179 source
files remain exact; changes are confined to that parent, the core dependency
manifest, one lockfile dependency-list line, and a new child module. No existing
dependency version changes. The old Oct25 borrowed-key map stored references
in16-byte entries; that failed experiment is distinct and remains closed.

## Validation and fixed execution

All144 native core tests pass: the prior141 plus three tests covering420
independent BTreeMap class/order constructions, forced collisions and hash-table
growth, and G1/G2 weighted-generator oracles with concurrent callers in
1/4/16/32-worker pools using512KiB stacks. RAW insertions are also checked through
ordinary map lookups after every operation. No unsafe code, persistent table,
proving-key duplication, new pool or larger memory forecast is added. This is native coverage;
portable, ARM and physical memory-pressure qualification are not claimed.

A postcapture API audit finds an additional portability condition: AHash's
`hash_one` may differ from manually building a hasher on other configurations.
This exact native build does not enable AHash specialization; its trait method
uses the same manual hash sequence as Hashbrown, consistent with the growth
tests. Any future portable implementation must explicitly match the table's
hash construction rather than assume that API equivalence. The captured
prototype remains unchanged and closed for performance.
[AHash0.8.12 source](https://github.com/tkaitchuck/aHash/blob/v0.8.12/src/random_state.rs).

Three4-worker smoke processes precede nine32-worker main processes. All use
the same executable through three independent byte-identical backings, three
rotated/reversed blocks, Boolean peeling disabled, native tracing and internal
PMU counters enabled. Both captures and all native build/test/link steps pass
first execution. All main capture finishes before late verification and
analysis, with no owned CPU cojobs during capture. The SDK digest is
`4f209a5cb04525bc5bba26b63d7a41ea37e22a09bb0fbf30233b49c978f4fbe2`.

All12 exports pass Gnark;2,298 SDK calls complete. All2,262 recorded requests
pass interval, route, placement, source and CPU/PMU checks, with100% PMU coverage.
Two separately written readers agree on the raw sample means, paired decisions
and proof inventory;24 receipts and48 stream hashes pass. An initial setup
helper delimiter error and an initially unresolved lockfile were repaired
before native work; the earlier prepared snapshot is retained privately.
Protocol19:36:44UTC to result19:43:28UTC is6m44s, within30minutes.

Canonical distinct exports are **59,046**:
`/home/ubuntu/src/zopro-perf-work/group-probe218/proof-sha256-after-main.json`,
SHA256 `e8e6566bb7066b23a6f43dd73757873b2e2354e5037f9b4d6d1adf139ed05d73`.
Complete evidence remains in that private root. Raw witnesses, keys, proof
bytes, full ledgers and binaries are excluded here. No AWS was used. Production
194/196 and its [CPU/Metal comparisons](../2026-11-17-certified-all196/README.md)
remain accepted; the original all-eight
CPU10× objective remains unmet0/8. This failed screen does not justify wider
qualification or a dependency/backend sweep to rescue the fixed recipe.
