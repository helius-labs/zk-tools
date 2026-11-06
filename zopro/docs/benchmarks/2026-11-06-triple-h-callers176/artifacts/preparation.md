# Preparation and publication notes

All three new native/control/driver builds and all 1,107 capture/offline workflow commands completed on their first attempt. Qualified native core/field libraries and prior tests were reused without rebuild/rerun. Campaign analysis and independent review both passed on their first execution; the candidate's performance gate is false with 27 retained failures. Those are measured negative checks, not workflow errors.

Before protocol freeze the initial campaign compilation warned about a parenthesized Vec type. The changed helper was rebuilt once after removing that warning and adding the finish.rs source pin. No capture had started. A few exploratory rg/read requests found absent paths; those were source discovery, not failed experiment commands.

The first publication-helper rustfmt and compile failed on one extra closing parenthesis in a formatting closure. The independent reviewer compiled successfully in that same preparation batch and was not rebuilt. The parenthesis was removed and publication helper compiled. No publisher execution had started. Actual captures, builds and decodes were not repeated.

Observed artifact size then changed the publication design: raw perf data total552MiB, workflow output3.2GiB, and independent review covers10,321,095samples. The helper now streams hashes for raw files and the24large decoded stdout streams, preserving these exact originals locally and publishing indexed byte sizes/SHA256. It embeds all other command/exit/source/timing/preflight/counter receipts and source delta with verified archive round trips. It does not load the entire multi-GiB decode collection into a JSON archive or push recordings into Git. This is a packaging change, not a capture, algorithm, gate or data change.

No successful build/test/capture/decode/analysis/reviewer was repeated for bookkeeping. No new proofs, production change, SDK trial, AWS resource or external message was added.
