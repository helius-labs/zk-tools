# Preparation observations

The source archive download and extraction each succeeded once. The three source files fetched through the GitHub connector match the extracted pinned tree (full checks are retained by the next stage). Two initially guessed model filenames returned404; the actual include is X86ScheduleZnver4.td, located by reading X86.td and the directory metadata. The public upstream main source still aliases znver5 to Znver4Model; it is context only, not an input to either backend.

Before any patch compilation, source review found that VPSUBQZrr already has a zero-idiom scheduling override. The generator removes that one opcode from its former list and adds a calibrated variant that preserves the original zero-latency branch. A plain duplicate InstRW was corrected in preparation; neither version had executed. The patch retains all masked/memory/shift forms and all other processors.

The initial IR-workflow launch stopped at the xtask host lock while the original backend was building. No IR helper or Rust consumer command ran, and no workflow output directory was created. The error log is retained; do not bypass the lock. Run the already prepared workflow only after the confirmed compiler build is terminal, preserving the launch log.

The original CMake configuration succeeds with a warning that LLVM_ENABLE_TERMINFO is unused. No configuration is repeated merely to remove that warning. Host clang18 builds the owned LLVM backend; production source and system compiler installations are unchanged.

An exploratory jq read named a nonexistent field-frozen metadata file and returned before creating a plan. The actual field/library paths are pinned by the inherited caller repair plan; no field/library build or qualification was repeated.
