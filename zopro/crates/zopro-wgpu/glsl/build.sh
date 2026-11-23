#!/bin/sh
# Compiles the Vulkan kernels to SPIR-V with the NDK's glslc into out/.
set -e
cd "$(dirname "$0")"
GLSLC=${GLSLC:-$HOME/Library/Android/sdk/ndk/28.2.13676358/shader-tools/darwin-x86_64/glslc}
mkdir -p out
for k in shards_first shards_tree reduce_blocks shards_first_g2 shards_tree_g2 reduce_pairs_g2 fr_dif_stage fr_dit_stage fr_fft_tile fr_scale fr_pointwise fr_euler_degree fr_euler_product msm_count msm_scatter msm_canonicalize msm_count_booth msm_scatter_booth; do
  "$GLSLC" -O -fshader-stage=compute --target-env=vulkan1.1 "$k.comp" -o "out/$k.spv"
  echo "$k.spv $(stat -f %z "out/$k.spv") bytes"
done
"$GLSLC" -O -DZOPRO_EULER_TILE=1 -fshader-stage=compute --target-env=vulkan1.1 fr_fft_tile.comp -o out/fr_fft_tile_euler.spv
for k in msm_count_booth msm_scatter_booth; do
  "$GLSLC" -O -DZOPRO_BOOTH_GROUPED=1 -fshader-stage=compute --target-env=vulkan1.1 "$k.comp" -o "out/${k}_grouped.spv"
done
for k in fr_fft_tile fr_euler_degree fr_euler_product; do
  "$GLSLC" -O -DZOPRO_VULKAN_WIDE_FR=1 -fshader-stage=compute --target-env=vulkan1.1 "$k.comp" -o "out/${k}_wide.spv"
done
