The first smoke semantic reader failed at its initial receipt environment check:
xtask's allowlisted command.json omits the harness's SHIFTED_* variables. The
original reader compared that missing field to SHIFTED_ARM="0". No smoke rows
or success marker were written, and the checked exit prevented main dispatch.

All six smoke native query processes had succeeded. Their printed SCREEN records
bind shape/arm/workers/rounds and route-call counts. The frozen plan retains the
full environment; executable/input/argument/stream hashes remain checked.

A new, separately pinned reader skips only unavailable SHIFTED_* receipt fields;
the semantic SCREEN checks remain unchanged. It also checks its own repair pins
before review or guarded main launch. Original reader/source/binary/runtime pins
and the failed diagnostic remain intact. No native source, executable, input,
scientific plan/order, criterion, query capture or successful test changes/repeats.
The independent original reader remains unchanged. This repairs review before
main capture and does not claim the original failed review passed.
