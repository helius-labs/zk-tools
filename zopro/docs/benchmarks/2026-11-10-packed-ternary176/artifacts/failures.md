# Retained workflow failures

1. capture-tools-runs/freeze-caller-protocol-and-readers exits101 before any
   caller or recording. The preparer tries to save arm identities to a path
   already holding candidate build identities. Four reader/tool builds pass and
   are retained. fix-freeze.rs moves the candidate identity to a distinct path,
   validates the already-created empty diagnostic directories/FIFOs and emits
   a separate campaign-fixed.rs. Original source/plan/pins/stdout/stderr remain.
   The three-step continuation succeeds; no scientific step is replayed.

2. post-runs/independent-raw-reader-all-decisions exits101 after both producers
   succeed. It expects RUSTC among recorded managed environment values, while
   xtask intentionally records only its managed configuration subset. Its old
   failed-tools reader also expects argv without xtask's timeout prefix.
   fix-reader.rs emits raw-reader-v2.rs: check the actual managed-key schema,
   retain the planned effective compiler check against Cargo's actual rustc
   command, and recognize the timeout argv prefix. Original raw-reader-fixed.rs
   remains frozen and archived. No data, statistic, threshold, producer,
   oracle, test, clock, profile or decoder changes. The reader-only continuation
   succeeds on the same data and reconstructs all28 candidate decisions.

Both failures are bookkeeping/reader failures, not failed point arithmetic or
lost scientific data. Candidate speed/resource gates fail13/28 and remain
failed independently of these corrections. Total825 workflow commands,
823 successes and2 retained failures through the successful raw reader.
