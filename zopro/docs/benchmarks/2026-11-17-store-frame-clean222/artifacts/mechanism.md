# Clean binary alignment check

Clean OFF SDK fa101d27e74c6d7f9ca4b701da83f7e1a75effdecfc076fa4edf26de3b6e1565.
Its scalar_groups function is at0x22d600 with size0x1dc8, six pushes and
sub rsp,0x1a8. Its32-byte key store uses the unaligned offset0xf0.

Clean ON SDK81d565e0b7439216ed8a529535500d3b32c6ac0127e7253a3c758db5d1aaf975.
scalar_groups at0x22d5e0, size0x1e08. Like221 it pushes48bytes, aligns RSP down
to32 and reserves480bytes. ABI entry RSPmod16=8 bounds the adjustment by24.
Total maximum activation560bytes including return slot, passing <=2048.
The main32-byte stack store uses [rsp+0x100]; its two16-byte reloads are within
that aligned span. A32-byte aligned transfer cannot cross a4KiB boundary.
Static mechanism passes; no actual liveSP/hardware-cause or latency claim.

ON changes one grouping parent plus a BTreeMap oracle/worker test. OFF matches
all179 production manifest hashes. No field/dependency change, heap allocation,
map-entry increase, proving-key/table copy, pool or default stack reservation.
All native tests finish before any SDK. Scalar fallback uses the same source
with noIFMA/noASM, excluding only the native marker. Exact194 driver/support.
No private Boolean/PMU/phase instrumentation is present in this clean build.
