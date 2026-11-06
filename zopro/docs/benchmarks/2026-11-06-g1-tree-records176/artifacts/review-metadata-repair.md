# Portable test-count metadata correction

The main independent review passed once. Its summary string already contained the actual portable result:4passed and101filtered, but a separate JSON metadata field mistakenly hardcoded125filtered from the native original-test count. There are24native-only originals absent from the portable executable. These tests were not executed or reported as portable passes.

The original reviewer source/JSON/stdout/stderr are retained byte-exact inreview-before-filter-count/. A narrow helper verifies the actual portable stdout hash against its successful receipt, parses101filtered and corrects only this derived metadata. It preserves all original review results. The published reviewer source is updated to derive this field from the summary for reproducibility, but the already successful main review/build/test populations are not reexecuted or rebuilt. No runtime, arithmetic, protocol, test count or gate changes.
