# Prequalification width repair

The original OFF field library builds successfully. ON fails before tests because
the mechanically converted four-lane insertion retained the `_si512` suffix in
one AND intrinsic. Correct it to `_mm256_and_si256`; the eight-lane arithmetic
is unchanged. The four-lane body retains its old path under LANES==4. Save the
old four-lane source and source audit. Only this cfg-excluded insertion changes
relative to the already-built OFF library; no successful OFF build is repeated.

Before its first compilation, add an explicit eight-lane array type to the new
test vector collection. Save that prior test source. No mathematical recipe,
test population, performance scope or threshold changes. The remaining eleven
build/test/link steps run in a separate source-pinned repair workflow.
