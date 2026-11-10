# Preserved workflow failure

tools/build-frozen-campaign returns1 before any capture. RustE0382: a PathBuf
is moved into hash(p) while also being used for an index. Replace exactly
pins[p.display().to_string()]=json!(hash(p)); with a borrowed hash(&p).
campaign-before-borrow-repair.rs, original tools-plan/pins, failed command,
stderr and repair-tools.rs retained. tools-fixed builds corrected producer
and unchanged raw-reader source. No successful build/fixture/capture replay.
No numerical formula, gate, order, worker count or capture source changed
after freezing capture. Producer/raw-reader analysis both pass first execution.

A later read-only jq inspection projection errors on a string and is corrected;
it writes no artifact, changes no data and is not a workflow recording attempt.

Scientific screen FAIL6/36 is a retained result, not repaired away. All316
capturecommands succeed, all614432Window outputs checked. Total326workflow
commands325success1compilefailure. No SDK trial/adoption/unchanged reroll.
