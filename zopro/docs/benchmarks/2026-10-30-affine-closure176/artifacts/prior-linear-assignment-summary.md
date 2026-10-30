# Direct linear-assignment census: no candidates

The read-only census loads all nine actual SDK key plans in raw and mapped-image formats. It recognizes zero, constant, wire-zero, constant terms and sums of constant terms, then checks whether the multiplier in an assignment constraint makes that assignment linear. All 18 plans contain zero candidates under this syntactic test. All stored assignment coefficient/inverse pairs multiply to one.

This rejects implementing direct linear-wire folding from this census as the next performance experiment. It does not recursively expand stored slots, discover arbitrary constant propagation or prove that every possible circuit simplification is absent. No query basis transformation, new key table, execution plan or production change is made; no timing or proof verification is claimed.

The initial protocol inherited pre-integration baseline metadata. Its copied solver hash is the retained f5c720 solver, and its linked SDK is the current retained build. The corrected protocol supplies that explicit provenance and preserves the initial protocol. The census is not rerun.
