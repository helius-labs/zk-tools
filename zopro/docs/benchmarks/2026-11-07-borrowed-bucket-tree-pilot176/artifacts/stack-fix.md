# Small-stack qualification repair

First core-OFF workflow is terminal101: compilation succeeds, four new tests pass, then window_routing_on_small_stack_pools aborts with stack overflow on a configured512KiB Rayon worker. Core-ON never starts. Preserve source/178files, original plan/pins/target/binary and failed receipts unchanged.

Inspection of the failed native test binary finds the candidate recursive Rayon producer bridge at0x3b0da0 reserving0x1b000+0x88=110728bytes per frame. Its inlined leaf includes two256-point arrays returned by value. Repeated bridge frames can compound that size. The original hb_window inspected at0x51fc90 reserves0x1378bytes. This is static evidence, not an exclusive runtime stack trace or latency attribution.

The source-fixed snapshot replaces large returned point arrays with256 usize slot indices. High roots are written directly to the already allocated output slice; low leaves borrow those roots. The low stage returns indices into retained compact storage, and outputs are copied only after that borrow ends. No extra heap buffer, inline attribute, worker/stack-size change, arithmetic or scheduling geometry change. Existing conservative root allowance in the admission bound is retained as a floor.

This runtime change justifies fresh unfiltered OFF/ON core qualification. Earlier mathematical model, descriptor microcheck and unchanged field qualification are not rerun. Four successful tests from the failed suite are not a completed qualification, and their changed stage/stack implementation must be rechecked. New snapshots/targets retain every failed-stage artifact.
