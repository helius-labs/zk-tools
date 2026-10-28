# Isolate B2 grouping with existing forecasts

Start from the exact accepted174-source/SDK81033 and the completed global-switch
negative50c046eb5. That switch slowed primary32 by6.93%,16x2 by104% and54 by82.5%,
with lowerRSS; it changed all query grouping AND workspace forecasts. Do not
repeat or adopt it. The earlier caller diagnostic located1.32–1.62ms before raw
inside the G2 grouping attempt, but did not isolate savedCPU or reject reason.

This private source copy adds one runtime probe read at g2_query_exclusive:
ZOPRO_CPU_SKIP_B2_GROUP_PROBE=1 bypasses ONLY its scalar_groups call. OFF=0
retains that call. GLOBAL ZOPRO_CPU_DEDUP=1 stays in every build/capture, as do
all workspace forecasts, G1/fusedKH/grouping/key/solver/field source and native
flags. One original file differs,173originals exact; reverse edit recovers174.
No public API/export/ISA change, table, extra plan, unsafe code or production
source mutation. This selector is experimental and must not become a public
runtime option or production shape whitelist from a pilot.

Build one coherent native candidate with accepted flags and a fresh target,
freeze every transitive rlib/proc macro and audit core/field fingerprints. Run
the unfiltered core122 and raw2/cubic6 tests in BOTH settings (the independent
G2 oracle spans8geometries×4scalarfamilies, including grouped/ungrouped fallback
boundaries); no new field-math test claim. Link ONE fixed SDK. OFF/ON use that
same path/inode, equal-length proof labels/environment values. Accepted uses
the retained SDK, not an arbitrary fast rebuilt copy. No flags/layout rerolls.

Before capture fix7rotated/reversed blocks ×primary16/32,16x2/32,54/32 ×accepted,
OFF,ON.151primary/31other requests after3primes; CPUs0–15distinctcores/0–31all
logical. All12preflights (4cells×3variants) use1request and independentGnark before
latency.84latencycaptures+12preflights=96exports, prospectiveledger15115->15211;
7896+48=7944SDKrequests. All timed requests include witness solving, fresh
blinding, proving, SDKverification and serialization, excluding loading/primes.
No profiling/probes/ownedbuild/helper/hash/archive/verifier/otherbench overlaps
latency. No exclusivehost or externalpreemption claim.

Report every block, CPU and lifetimeRSS and paired extrema-of7 median bounds
(nominal98.4375% under independentexchangeable blocks, no familywise guarantee).
Primary32 advancement: ON/OFF andON/accepted upper<1 with>=5/7wins, and
OFF/accepted bounds withinfactor1.03. Both larger cells need ON/OFF and
ON/accepted upper<=1.03; their calibration is reported separately. Physical16
is descriptive; keep its uncertainty and earlier accepted exceptions explicit.
No gainfloor, exclusion, extension, successfulcapture repeat or gatedata rewrite.
Passing only selects further all8/compat/resource/clean-source qualification;
the complete10xCPU goal and CPU+Metal comparison scope remain unchanged.

No production/default adoption, AWS/Aeglos/mobile/ARM/TEE/giant/swap/minRAM/
pressureclaim from this screen. All8acceptedM5CPU+Metal table remains inherited
explicitly; newtimings coveronlytwo suppliedM5shapes plus54/primary16. Commit
and push every completed experiment, including negatives.
