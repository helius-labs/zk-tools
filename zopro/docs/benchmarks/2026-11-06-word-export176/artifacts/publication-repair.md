# Source archive encoding repair

The initial publisher completed the report, direct artifact copies and two analysis/profile-symbol compression round trips. It then rejected a checked-in binary test-vector source file while constructing the candidate source archive, before writing that archive or any receipt/profile archive/checksums. Original publisher source/binary are retained in initial-caller-preparation and the original stderr remains publication.stderr.log. The error is a UTF-8 assumption in source packaging, not a native/test/measurement/review failure.

The corrected publisher uses lossless hex with byte count and SHA-256 for each source member, verifying every decode round trip. This includes the accepted repository's binary test vectors; no private key, image, witness, proof or executable/library blob is added. For a fresh destination use corrected publish.rs. For this partial destination, make-publish-repair generates publish-repair.rs to resume only the failed source archive and remaining receipt/profile/checksum work. Earlier successful report/artifact/compression operations are reused; the report gains this encoding/repair note and the corrected publication sources.

No native compilation, test, caller, capture, decoding, statistical finish or successful independent review is repeated. Failed full performance gates remain false.
