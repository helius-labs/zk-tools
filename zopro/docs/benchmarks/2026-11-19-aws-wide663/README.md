# C7a pilot with the grouped G2 candidate

This extension adds one engine configuration to the four-shape pilot. On 5×16 it pairs the new executable, packed FFT and grouped G2 option with the prior executable using only packed FFT. The pair alternates order across blocks. The ordinary current reference, historical reference and prepared Aeglos also run in every 5×16 block. The other shape configurations match the earlier protocol.

The full workflow uses 32 and 96 physical cores on one C7a.24xlarge. Each of 256 timed processes primes three requests, measures eleven and exports a proof for independent verification against the original key and public inputs. Compute the 2× target from the current ordinary reference measured on that instance. Aggregate CPU time, wall time, warm request samples and peak RSS are available in the process logs. The executable payload, workflow and pin file are staged for installation at `/opt/zopro-wide663`. From that directory, `bin/xtask workflow workflow.json results` runs the pilot.

[Original pilot and smoke checks](../2026-11-19-aws-wide660/README.md) · [Local release check](../2026-11-19-owned-phi661/README.md) · [Exact cloud workflow](artifacts/workflow.json.gz) · [Payload manifest](artifacts/manifest.json) · [Input pins](artifacts/pins.json)

The new executable and environment passed an incremental Ryzen smoke at 32 workers. Its exported proof verified against the same 5×16 key and public input used by every other arm. The prior 15 smoke configurations cover the rest of the payload. Execution on C7a awaits a valid AWS session. [Verification receipt](artifacts/verification.json)
