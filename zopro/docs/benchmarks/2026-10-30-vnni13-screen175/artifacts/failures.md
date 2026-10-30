# Preparation failures

The initial XML extractor required literal `<t>` and panicked on an attributed text element (expected one literal match, found zero). Its source is retained as extract-initial.rs; the terminal observation was not redirected to a raw log. The repaired reader accepts text attributes and rich runs. Its one successful extraction is reused; subsequent rustfmt changes formatting only.

The initial workflow plan lacked version:1 and xtask rejected it before creating an output directory or executing the command. Original screen source, plan, pins and raw workflow.log are retained in receipts.json.gz under initial-workflow/. The corresponding original executable remains local, with its hash in the archived pins. Final pins bind the corrected executable. No successful native semantic check, field operation or proof was rerun.
