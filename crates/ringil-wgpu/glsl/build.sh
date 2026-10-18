#!/bin/sh
# Compiles the Vulkan kernels to SPIR-V with the NDK's glslc into out/.
set -e
cd "$(dirname "$0")"
GLSLC=${GLSLC:-$HOME/Library/Android/sdk/ndk/28.2.13676358/shader-tools/darwin-x86_64/glslc}
mkdir -p out
for k in shards_first shards_tree reduce_blocks shards_first_g2 shards_tree_g2 reduce_pairs_g2 fr_dif_stage fr_dit_stage fr_scale fr_pointwise; do
  "$GLSLC" -O -fshader-stage=compute --target-env=vulkan1.1 "$k.comp" -o "out/$k.spv"
  echo "$k.spv $(stat -f %z "out/$k.spv") bytes"
done
