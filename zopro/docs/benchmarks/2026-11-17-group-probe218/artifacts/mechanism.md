# Emitted mechanism check before SDK capture

SDK SHA2564f209a5cb04525bc5bba26b63d7a41ea37e22a09bb0fbf30233b49c978f4fbe2.
Native144coretests PASS,0failed/ignored/filtered;original141+3neworacles.

HB builder symbol0x2379f0,size0xea1. Lookup loop loads32B scalar at0x237e59,
then storesYMM to0x40(%rsp) at0x237e75 and reloads128-bit halves at0x237e7b/81
before scalar-dependentAES0x237eaa/0x237ebc. Thus ordinaryownedentry retains
stack temporary in hashing dependence.

RAW builder symbol0x2388a0,size0xe7d. Lookup loop directly loads128-bit scalar
halves from(%r12) and0x10(%r12) at0x238d0f/15, thenAES0x238d44/4d. No32B
key-to-stack store precedes hashing in that loop. Key ownedcopies on insertion
remain. MechanismcheckPASS, not a latency result/hardware-cause proof.

Source unchanged exceptparentdispatch,newchild,coredependencyCargo.toml and
oneCargo.lockline. OriginalSTDconstruction remains. No unsafe added. Direct
hash uses same mapBuildHasher and Hash/Eq types;growth/collision oracle checks
raw insertions withordinarymap.get aftereveryoperation inclforcedcollisions.
Hashbrown0.15.5officialrawentrysource reviewed at
https://github.com/rust-lang/hashbrown/blob/v0.15.5/src/raw_entry.rs ;cached
registry source used. Two docs.rs URLs unavailable; primaryGitHubsourceworks.
No external source copied; standardpublicAPI only.

Preparation repairs before any SDK:extra delimiter in setuphelper;no-deps
metadata didnotresolve lock;cargo update--offline--workspace changesONLY
zopro-core's dependencylist addinghashbrown0.15.5. First prepared snapshot
preserved beforecorrectoverlaylock reprepare. No nativecompile/test failures;
first actual fullnative/build/link allPASS. All runtime/buildsources pinned.
