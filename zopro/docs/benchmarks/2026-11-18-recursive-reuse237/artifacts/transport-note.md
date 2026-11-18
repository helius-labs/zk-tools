The first continuation launch was rejected by xtask's strict plan schema because
plan-v2 contained an unsupported top-level continuation metadata field. It
created no run-v2 directory and executed no command. plan-v3 removes only that
metadata field and binds an extended pin manifest. All five step argv/env/timeouts
are unchanged; saved plan-v2 remains the reader's scientific step reference.
The original schema diagnostic is retained in the tool transcript. No successful
compile, test or count is repeated for this transport-only correction.
