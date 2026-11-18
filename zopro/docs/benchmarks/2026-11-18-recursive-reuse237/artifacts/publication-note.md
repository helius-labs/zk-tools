The first publication attempt correctly validated counts/receipts, saved three
metadata files and copied public artifacts, then tar failed on run/result.json.
That original workflow failed compilation and has failure.json instead of a
success result. No report/register/commit was produced in the failed attempt.

The partial archive is retained privately. A publication-only continuation
checks/reuses identical metadata, archives the actual failure receipt and
completes the report/checksums/register. No model, scientific protocol, DAG,
test/count/reviewer execution or decision changes or is replayed. The initial
publication source and tool diagnostic remain; no private DAG is archived.
