# Preserved preparation and review failures

1. native-runs/build-quarter-window-native-tests returns1. Two generated child
 files put inner module docs after imports, the new G2 constructor no longer
 inferred a BatchAdd type, and the Arkworks oracle attempted Fq2*=Fq. Fixes:
 comment style, explicitG2Config and componentwise scalar multiplication.
 Original field build retained; no numerical test had run. Initial source
 bytes are in before-compile-repair/. repair.json lists exact text changes.
 Original prepare.rs and native-plan/pins remain unchanged.
2. release-runs/link-quarter-window-allocation-oracle returns1. A local counts
 array shadows fn counts. Rename only the function and four invocations.
 allocations-before-name-repair.rs retained. Both successful release libraries
 reused; no allocation recording had run before this repair.
3. review-runs/review-window-native-and-allocation-evidence returns101. The
 reader treats every pinned source as UTF8; the accepted manifest includes a
 binary source asset. review-fixed.rs changes parent recovery to byte slices.
 Original review source/binary/failed receipt retained. No scientific workflow,
 test, output or library rebuild is replayed for this offline repair.

All18 workflow attempts are indexed:15success/3failure. Four new native tests,
24conversion rows and four fullWindow allocation rows pass. No numerical
counterexample or latency observation is concealed by these compile/reader
repairs. No test population, threshold, worker layout or arithmetic changes.
