# Retained failure

The native workflow completed the field build and both probe links, then the
independent oracle build exited1 with E0282: BigUint's shift needed an explicit
index type. Changing the first boundary-list literal from1 to1usize resolves
type inference. Original source is retained as qualify-before-index-type.rs;
the exact one-token reversal is checked. No arithmetic, boundary value, random
seed, test population or production source changes. Only the failed build and
unexecuted steps continue in native-plan-fixed.json; three successful builds
are reused. The original workflow's stderr/command/exit remain archived.
