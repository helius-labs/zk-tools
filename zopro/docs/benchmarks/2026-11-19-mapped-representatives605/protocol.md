# Upper G2 group inputs on the centered 12×2 prover

The existing upper G2 query can materialize one field value for each lower-level representative. A previously tested view reads each representative through the selected member. The new view reads the current witness through a single index slice. It borrows the key's source indices if every selected representative is the default. When a choice changes, it reserves and copies four bytes per index. If the reservation fails, it uses the prior view. The scalar selection and residual correction still execute for the current witness.

Seven shuffled blocks compare the previous binary and three settings in one candidate executable on 12×2 at 30 pinned Ryzen workers. Each block rotates its two omitted SMT siblings. The original, one adjacent request and two scattered requests each receive three warmups and 21 timed calls per process. Four separate diagnostic processes record the borrowed, copied or skipped route. A native unit test checks the selected indices and lookup values when choices are empty, unchanged or changed. The same new executable also compares ordinary and sampled centered H routing on the 2×2 reference at 32 workers against its prior sampled build, using 31 timed calls per process. All exports require the ordinary verification key and the expected public inputs.

[Process order](artifacts/schedule.json)
