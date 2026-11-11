# Retained executable page census

Read-only diagnostic following the closed G1-only screen. Its purpose is to
replace unmeasured physical-page speculation with an actual present-time census.
It does not recover physical addresses from the previous timing campaign and
does not qualify a candidate or choose a faster executable copy.

Inspect all nine retained REF/OFF/ON copies in runtime-consumers.json. Check
device/inode/mode/length and full SHA-256 against that record. Map ELF64 little
endian PT_LOAD file regions with PF_W clear, PROT_READ/MAP_SHARED, in a helper
process. Never execute an SDK or map a key, image, witness, heap or other process.
Read mincore before touching each mapping, then touch one byte in each file page.
Read this helper's pagemap and kpageflags/kpagecount with CAP_SYS_ADMIN. Retain
raw PFNs privately. Map each region a second time and require matching frame
identities; this is a same-inode alias check, not another timing run.

Record present/file/swapped flags, page-size/compound flags, contiguous physical
runs, cross-file sharing and descriptive PFN residues modulo 16/32/64. Residues
are not validated cache-set mappings. Also report pages covering each G2 apply
symbol from nm. Hash source inputs and binaries after capture and recheck inode
identities. Symbol listing and hashing can populate the page cache; residency
counts describe this diagnostic only. sdk-f0 was inspected with nm/readelf before
this protocol, so no historical cold-cache inference is permitted.

No eviction, cache drop, mlock, page migration, ASLR change, ELF rewrite, relink
or new executable copy. Current mapping may warm file pages. No proof generation,
no latency gate and no timing rerun. Canonical ledger remains 52,945. Production
and failed gates remain unchanged. Execute through retained xtask. Keep results
and report local; no push/publication approval exists.

Interpretation source:
https://cdn.kernel.org/doc/html/latest/admin-guide/mm/pagemap.html

If PFNs are unavailable, report that limitation without attempting to bypass it.
Otherwise use the census to select a distinct follow-up capable of distinguishing
physical placement from runtime scheduling, data alignment or instruction-fetch
effects. A correlation with previous timings cannot establish causality.
