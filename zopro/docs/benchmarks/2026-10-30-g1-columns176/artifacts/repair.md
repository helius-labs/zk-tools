# Native preparation repair

Initial workflow completed control125core tests/libraries/freeze and35base field tests, then candidate core compilation failed. The new field child incorrectly used core's `ifma8` feature guard (the field crate has no such feature), and the core FqBatch wrapper lacked forwarding methods. No new helper tests, candidate point test or timing had run. The35field results cover only unchanged arithmetic and remain inherited; do not label them helper qualification.

Preserve original source178, plan/pins, native receipts and failed compiler log. Prepare a separate source-fixed snapshot that removes that field-only feature condition and appends three guarded core forwarders. Field and filler helper bodies, algorithm and geometry stay byte-identical. No existing generated arithmetic is edited. The corrected snapshot has172accepted originals exact, four changed parents and two children.

New workflow runs only the two previously excluded field helper tests, then the unfinished unfiltered candidate core suite/libraries/freeze. The35passed base field cases are filtered out and inherited. The successful original control125core/libraries/freeze remain the baseline; its candidate cfg is absent under both guards and all original arithmetic bodies are exact. No successful test, build or capture is repeated. Production source176 is untouched.
