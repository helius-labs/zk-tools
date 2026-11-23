//! Preserve each shard's destination while grouping its first point loads.
use super::{Gpu, Slot, WORKGROUP, profile, resident, storage, uniform};
use std::path::Path;

pub(super) struct ShardOrder {
    pub(super) first: wgpu::ComputePipeline,
    pub(super) indices: wgpu::Buffer,
    heads: wgpu::ComputePipeline,
    prefix: wgpu::ComputePipeline,
    scatter: wgpu::ComputePipeline,
    counts: wgpu::Buffer,
    tags: wgpu::Buffer,
    geometry: wgpu::Buffer,
}

impl ShardOrder {
    pub(super) fn new(gpu: &Gpu, path: &Path, n: usize, capacity: usize) -> Result<Self, String> {
        use Slot::{Read, Uniform, Write};
        if capacity > 1 << 21 {
            return Err("Vulkan shard order tags cannot hold the workspace capacity".into());
        }
        let shift = std::env::var("ZOPRO_ANDROID_SHARD_ORDER_SHIFT").ok()
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|shift| (11..=17).contains(shift))
            .unwrap_or(14);
        let bands = n.div_ceil(1 << shift);
        let pipeline = |name, slots: &[Slot]| resident::load_pipeline(gpu, path, name, slots);
        let heads = pipeline("msm_shard_heads", &[Read, Read, Write, Write, Uniform, Uniform])?;
        let prefix = pipeline("msm_shard_head_prefix", &[Write, Uniform])?;
        let scatter = pipeline("msm_shard_head_scatter", &[Read, Read, Write, Uniform])?;
        let first = pipeline("shards_first_ordered", &[Read, Read, Read, Read, Read, Write, Uniform, Read])?;

        log::info!("ZOPRO_TRACE vulkan-euler-h shard read order shift={shift}");
        Ok(Self {
            first,
            indices: storage(&gpu.device, "MSM shard order", capacity * 4),
            heads,
            prefix,
            scatter,
            counts: storage(&gpu.device, "MSM shard head counts", bands * 4),
            tags: storage(&gpu.device, "MSM shard head tags", capacity * 4),
            geometry: uniform(gpu, [shift, bands as u32, 0, 0]),
        })
    }

    pub(super) fn encode(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        [sorted, starts, shards]: [&wgpu::Buffer; 3],
        count: usize,
        timer: &mut Option<profile::Timer>,
    ) {
        encoder.clear_buffer(&self.counts, 0, None);
        let groups = [(count as u32).div_ceil(WORKGROUP), 1];
        resident::dispatch(
            &gpu.device,
            encoder,
            &self.heads,
            &[sorted, starts, &self.counts, &self.tags, shards, &self.geometry],
            groups,
            timer,
            "msm_shard_heads",
        );
        resident::dispatch(
            &gpu.device,
            encoder,
            &self.prefix,
            &[&self.counts, &self.geometry],
            [1, 1],
            timer,
            "msm_shard_head_prefix",
        );
        resident::dispatch(
            &gpu.device,
            encoder,
            &self.scatter,
            &[&self.counts, &self.tags, &self.indices, shards],
            groups,
            timer,
            "msm_shard_head_scatter",
        );
    }
}
