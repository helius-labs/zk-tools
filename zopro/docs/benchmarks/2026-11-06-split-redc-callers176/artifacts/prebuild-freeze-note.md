# Pre-build frozen filename correction

Before any caller workflow command ran, the freezer's core filename was corrected to libzopro_core-HASH.rlib, so it is also a valid Rust --extern input. The copied sources, build/test/driver arguments, native field libraries and all gates are unchanged. Recompile only the modified freezer/preparation helper and refresh its two pins. Do not repeat the already successful preparation/source copy, any field stage or an earlier model. This is not a timing or native failure.
