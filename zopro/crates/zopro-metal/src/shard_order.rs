use metal::{
    Buffer, ComputeCommandEncoderRef, ComputePipelineState, Device, MTLResourceOptions, MTLSize,
};

use crate::{discard, pipeline};

pub(crate) const BAND_SHIFT: u32 = 14;

pub(crate) struct ShardOrder {
    bands: u32,
    kernels: [ComputePipelineState; 4],
    pub(crate) first: ComputePipelineState,
    counts: Buffer,
    tags: Buffer,
    indices: Buffer,
}

impl ShardOrder {
    pub(crate) fn new(device: &Device, points: usize, capacity: usize) -> Result<Self, String> {
        // One tag combines the band with a within-band shard ordinal.
        if points > (1 << 21) || capacity > (1 << 21) {
            return Err("large shard order index".into());
        }
        let library = |name| pipeline(device, false, name);
        let kernels = [
            library("clear_shard_bands")?,
            library("count_shard_bands")?,
            library("prefix_shard_bands")?,
            library("scatter_shard_bands")?,
        ];
        let first = library("shards_first_g1_radix29_ordered")?;
        let bands = points.div_ceil(1 << BAND_SHIFT) as u32;
        let private = |n: usize| {
            device.new_buffer(
                (n * 4).max(16) as u64,
                MTLResourceOptions::StorageModePrivate,
            )
        };
        Ok(Self {
            bands,
            kernels,
            first,
            counts: private(bands as usize),
            tags: private(capacity),
            indices: private(capacity),
        })
    }

    pub(crate) fn encode(
        &self,
        encoder: &ComputeCommandEncoderRef,
        sorted: &Buffer,
        starts: &Buffer,
        shards: usize,
    ) -> &Buffer {
        debug_assert!(shards as u64 * 4 <= self.tags.length());
        let dispatch = |kernel: &ComputePipelineState, n: u64| {
            let group = n
                .clamp(1, 128)
                .min(kernel.max_total_threads_per_threadgroup());
            encoder.dispatch_threads(MTLSize::new(n, 1, 1), MTLSize::new(group, 1, 1));
        };
        encoder.set_compute_pipeline_state(&self.kernels[0]);
        encoder.set_buffer(0, Some(&self.counts), 0);
        dispatch(&self.kernels[0], self.bands.into());

        encoder.set_compute_pipeline_state(&self.kernels[1]);
        for (i, b) in [sorted, starts, &self.counts, &self.tags]
            .into_iter()
            .enumerate()
        {
            encoder.set_buffer(i as u64, Some(b), 0);
        }
        encoder.set_bytes(4, 4, (&BAND_SHIFT as *const u32).cast());
        dispatch(&self.kernels[1], shards as u64);

        encoder.set_compute_pipeline_state(&self.kernels[2]);
        encoder.set_buffer(0, Some(&self.counts), 0);
        encoder.set_bytes(1, 4, (&self.bands as *const u32).cast());
        dispatch(&self.kernels[2], 1);

        encoder.set_compute_pipeline_state(&self.kernels[3]);
        for (i, b) in [&self.counts, &self.tags, &self.indices]
            .into_iter()
            .enumerate()
        {
            encoder.set_buffer(i as u64, Some(b), 0);
        }
        dispatch(&self.kernels[3], shards as u64);
        &self.indices
    }
}

impl Drop for ShardOrder {
    fn drop(&mut self) {
        [&self.counts, &self.tags, &self.indices]
            .into_iter()
            .for_each(discard);
    }
}
