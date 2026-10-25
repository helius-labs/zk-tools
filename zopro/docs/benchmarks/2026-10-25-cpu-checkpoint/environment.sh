# Source from the zopro directory on the CPU used for the build.
export RUSTFLAGS='-C target-cpu=native --cfg zopro_horizontal_limb_sum --cfg zopro_g2_horner_buckets --cfg zopro_booth_digits --cfg zopro_live_preparation_charge --cfg zopro_folded_query_reclaim --cfg zopro_bounded_schedule --cfg zopro_resource_schedule --cfg zopro_real_kh --cfg zopro_euler_leading_weights --cfg zopro_cios_cancel_carry --cfg zopro_euler_pointwise8 --cfg zopro_direct_chain_inverse --cfg zopro_gcd_balanced_runs'
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-32}"
export ZOPRO_CPU_LAGRANGE_DIR="${ZOPRO_CPU_LAGRANGE_DIR:-${TMPDIR:-/tmp}/zopro-lagrange-cache}"
export ZOPRO_CPU_FFT_COLUMN_ROOTS='1'
export ZOPRO_CPU_CANONICAL_WIRES='1'
export MALLOC_TRIM_THRESHOLD_='-1'
export MALLOC_MMAP_THRESHOLD_='1073741824'
export ZOPRO_CPU_LAGRANGE='1'
export ZOPRO_CPU_LAGRANGE_FOLD='1'
export MALLOC_ARENA_MAX='32'
export ZOPRO_CPU_DEFER_SIGNED_Y='1'
export ZOPRO_CPU_DEFER_SIGN_WIDTH='12'
export ZOPRO_CPU_DEFER_SIGN_PARTS='1'
export ZOPRO_CPU_BOUNDED_SCHEDULE='1'
export ZOPRO_CPU_RESOURCE_SCHEDULE='1'
export ZOPRO_CPU_PROOF_CONCURRENCY='1'
export ZOPRO_CPU_ZERO_AWARE_GROUPS='1'
