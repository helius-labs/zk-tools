# Offline tool notes

Before this audit, one exploratory ON reduce_layer annotation was displayed with output truncation. It made no recording. A later command tried perf annotate --show-nr-samples together with --show-total-period; perf rejected these mutually exclusive display modes with exit129 before decoding. The command was corrected to sample counts only. indexed-reduce-ON.txt was then saved successfully and is reused by audit.rs. No binary/source/recording changed. Remaining annotations are new offline views of retained data. No scientific timing is repeated.

Mnemonic classes label emitted instructions only. Shifts also implement field carry/radix work; memory moves include many purposes; branch samples can skid. The audit must not interpret their sample shares as packing-only costs, exclusive CPU removal potential, retired instruction frequencies or a causal critical path.
