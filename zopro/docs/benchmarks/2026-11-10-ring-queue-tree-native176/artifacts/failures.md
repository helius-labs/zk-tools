# Preserved failures and repairs

- The first preparer parses only 19 of 20 old resource rows: the first marker
  follows a test-name prefix on the same stdout line. It stops before writing
  the native plan or launching a build/test. Parsing from the marker fixes it.
  Initial preparer source and its partial private source snapshot remain local.
  Preparation tool calls have transcript evidence, not xtask command receipts.
- The first offline reader expects the inserted fragment to begin at impl;
  the actual source includes its two leading comments. It fails after all nine
  native commands have succeeded. The corrected reader compares the complete
  inserted fragment, including comments. This changes only the reader, not
  candidate code, resource arithmetic, tests, compiler flags or any gate.
  Old source, binary, frozen plan/pins and failed stdout/stderr/exit are retained.
- The first publisher validates its checksum link before creating that file.
  All native/reader results and archive data already exist. Its failed source
  and partial publication are retained; a placeholder orders link validation
  before the final real checksum write. Only publication is regenerated.

Nine native workflow commands and two repaired-reader commands pass. The
initial reader workflow adds one successful helper build and one failed reader
execution, for 13 workflow commands: 12 successes and one retained failure.
No successful native build, test, model, capture or cryptographic verifier is
repeated for either repair. This stage creates no timing or proof export.
