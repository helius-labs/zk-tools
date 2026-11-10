# Operational amendment before publication and launch

Local preflight completed: 44 SDK requests, 11 fresh independently Gnark-verified
exports; independent trace reader passed. Canonical ledger is now 50148 at
task-interval-c7a176/proof-sha256-after-preflight.json.

The first container attempt failed creating its output directory, before any
SDK request. The UID was incorrectly used as its group; the directory now uses
the explicitly supplied supplemental group 1000. Original failure receipts are
retained in preflight-runs-access-failure; successful proofs were not replayed.

The existing SSO role expires at 2026-11-11T00:01:44Z. Attempted normal role
renewal found an expired SSO token whose refresh failed; the working role cache
was restored. No credential contents are included in the experiment artifacts.
No image was published or VM launched before this amendment.

Shorten the owned instance automatic shutdown from 90 to 25 minutes, require
at least 35 minutes of remaining role lifetime before publication and launch,
and limit the measurement SSM command to 900 seconds. The extra ten minutes
allow cleanup with the existing valid credentials. Preserve the original 90m
helper and user-data sources. No benchmark inputs, order, counts, recorder
capacity, statistical gates, binary or scientific reader changes.

The VM shutdown behavior remains terminate with encrypted delete-on-termination
storage. If interrupted, retain partial results without extending/replaying the
measurement to replace an unfavorable outcome.
