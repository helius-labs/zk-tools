The original compile-study command failed E0432 because inherited compiler argv
did not include sha2. The successful compile-engine-tests step is retained and
not replayed. The original source and command/stream/exit receipts remain.

Continuation adds the existing sha2 extern argument to the two uncompleted
helper builds. The study switches only its pin-manifest filename. The reader
switches only pin/plan/receipt paths to combine the inherited successful compile
with the remaining steps and records the failed original compile. Pre-repair
source copies remain. No optimizer, recoding, formal oracle, input, order,
criterion or timeout changes. No count or test execution occurred before repair.
