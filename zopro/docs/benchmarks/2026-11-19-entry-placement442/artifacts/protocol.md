Build and test the public CPU SDK example using Cargo release mode and the previously measured native flags. Verify the inherited default and a requested invalid CPU. Seven matched blocks compare the current public example and the prior experimental driver, each with an eight-worker or twelve-worker frontend on a single L3. The 32 main workers stay fixed. Each paired block uses the same L3, alternating between the two. Time 51 verified SDK calls after three warmups. Check the reported affinity on all configured processes, Gnark verification on every exported proof and both drivers’ paired effects.

Rust 1.98.1, Linux x86-64. Build from the zopro workspace with the following flags.

```sh
export RUSTFLAGS='-C target-cpu=znver4 --cfg zopro_horizontal_limb_sum --cfg zopro_g2_horner_buckets --cfg zopro_booth_digits --cfg zopro_live_preparation_charge --cfg zopro_folded_query_reclaim --cfg zopro_bounded_schedule --cfg zopro_resource_schedule --cfg zopro_real_kh --cfg zopro_euler_leading_weights --cfg zopro_cios_cancel_carry --cfg zopro_euler_pointwise8 --cfg zopro_direct_chain_inverse --cfg zopro_gcd_balanced_runs --cfg zopro_canonical_task_grain --cfg zopro_small_core_entry --cfg zopro_g2_prefix_records --cfg zopro_dense_busy_reset --check-cfg=cfg(zopro_phi_grain) --cfg zopro_phi_grain --check-cfg=cfg(zopro_g2_coordinate_leaf) --cfg zopro_indexed_tile_bundles --cfg zopro_coalesce_probe --check-cfg=cfg(zopro_indexed_tile_bundles) --check-cfg=cfg(zopro_coalesce_probe) --cfg zopro_paired_weighted --check-cfg=cfg(zopro_paired_weighted)'
cargo build --locked --release --target x86_64-unknown-linux-gnu -p zopro-bench --example cpu-sdk
```

The first L3 used logical CPU IDs 0 through 7 and SMT siblings 16,18,20,22. The second used 8 through 15 and siblings 24,26,28,30. The original request used the same prover settings as the [varied-input comparison](../../2026-11-19-varied-smt440/README.md).
