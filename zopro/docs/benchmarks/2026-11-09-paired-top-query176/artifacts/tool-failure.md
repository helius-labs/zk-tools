# Preserved observer-test compile failure

The first command in build-workflow failed compilation before any new test or performance caller executed. The new observer test captured Rayon BroadcastContext in nested join closures; that context is not Send/Sync. observer-initial.rs, build-plan.json, build-pins.json and build-workflow/build-new-observer-test retain the source/plan/pins/compiler receipt. repair-test.rs/test-only-repair.json copy ctx.index() to an ordinary usize before constructing the closures and bind the corrected source in build-plan-fixed.json/build-pins-fixed.json.

Only test code changes. The runtime observer, paired scheduler, arithmetic, field, route policy and prospective measurement protocol are unchanged. The four corrected native steps, two caller links, three tool builds and all665 capture/offline steps pass on their first execution. The arithmetic qualification is inherited rather than replayed. No observation or result is discarded.
