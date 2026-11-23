//! Build the Android Euler MSM's ordered bucket runs on the device.
use super::{Curve, Gpu, GpuSort, Msm, Slot, WORKGROUP, profile, spirv_pipeline, storage, uniform};
use std::path::Path;

const PAIRS_PER_SHARD: usize = 16;
const AFFINE_BATCH: usize = 128;
const AFFINE_BYTES: usize = 64;

struct AffineBatch {
    pipeline: wgpu::ComputePipeline,
    pairs: wgpu::Buffer,
}


pub(super) struct ResidentSort {
    prefix: wgpu::ComputePipeline,
    first: wgpu::ComputePipeline,
    tree: wgpu::ComputePipeline,
    affine_batch: Option<AffineBatch>,
    offsets: wgpu::Buffer,
    totals: wgpu::Buffer,
    shard_buckets: wgpu::Buffer,
    prefix_arguments: wgpu::Buffer,
    chunk_arguments: Vec<[wgpu::Buffer; 4]>,
}

impl ResidentSort {
    pub(super) fn new(
        gpu: &Gpu,
        path: &Path,
        [n, half_range, windows, chunk_windows]: [usize; 4],
    ) -> Result<Self, String> {
        use Slot::{Read, Uniform, Write};
        let pipeline = |name, slots: &[Slot]| load_pipeline(gpu, path, name, slots);
        let prefix = pipeline("msm_resident_prefix", &[Read, Write, Write, Write, Uniform, Write])?;
        let first = pipeline("msm_resident_first", &[Read, Read, Read, Read, Read, Write, Uniform, Read])?;
        let tree = pipeline("msm_resident_tree", &[Read, Read, Read, Read, Write, Uniform, Write])?;
        let stride = n.div_ceil(32) + half_range;
        let affine_batch = if std::env::var("ZOPRO_ANDROID_VULKAN_RESIDENT_AFFINE_BATCH")
            .is_ok_and(|value| value == "1")
        {
            let bytes = chunk_windows * stride * PAIRS_PER_SHARD * AFFINE_BYTES;
            let limits = gpu.device.limits();
            if bytes as u64 > limits.max_buffer_size
                || bytes > limits.max_storage_buffer_binding_size as usize
            {
                return Err(format!("affine pair buffer exceeds Vulkan limits: {bytes} bytes"));
            }
            let pipeline = pipeline(
                "msm_resident_affine_pairs",
                &[Read, Read, Read, Read, Read, Write, Uniform, Read],
            )?;
            log::info!("ZOPRO_TRACE vulkan-euler-h affine pair batching");
            Some(AffineBatch {
                pipeline,
                pairs: storage(&gpu.device, "resident MSM affine pairs", bytes),
            })
        } else {
            None
        };

        let chunk_arguments = (0..windows.div_ceil(chunk_windows))
            .map(|chunk| {
                [0, 1, 2, 3].map(|level| {
                    uniform(
                        gpu,
                        [(chunk * chunk_windows) as u32, half_range as u32, stride as u32, level],
                    )
                })
            })
            .collect();
        Ok(Self {
            prefix,
            first,
            tree,
            affine_batch,
            offsets: storage(&gpu.device, "resident MSM bucket offsets", windows * half_range * 16),
            totals: storage(&gpu.device, "resident MSM window totals", windows * 16),
            shard_buckets: storage(&gpu.device, "resident MSM first-stage bucket IDs", windows * stride * 4),
            prefix_arguments: uniform(gpu, [n as u32, half_range as u32, windows as u32, stride as u32]),
            chunk_arguments,
        })
    }

    pub(super) fn passes_per_chunk(&self) -> usize {
        4 + usize::from(self.affine_batch.is_some())
    }

    pub(super) fn encode_prefix(
        &self,
        msm: &Msm<impl Curve>,
        sort: &GpuSort,
        encoder: &mut wgpu::CommandEncoder,
        timer: &mut Option<profile::Timer>,
    ) {
        dispatch(
            &msm.gpu.device,
            encoder,
            &self.prefix,
            &[
                &sort.counts, &self.offsets, &sort.cursors, &self.totals,
                &self.prefix_arguments, &self.shard_buckets,
            ],
            [msm.windows as u32, 1],
            timer,
            "msm_resident_prefix",
        );
    }

    pub(super) fn encode_chunk(
        &self,
        msm: &Msm<impl Curve>,
        sort: &GpuSort,
        chunk: usize,
        encoder: &mut wgpu::CommandEncoder,
        timer: &mut Option<profile::Timer>,
    ) {
        let window_count = (msm.windows - chunk * msm.chunk_windows).min(msm.chunk_windows) as u32;
        let groups = |points: usize| [points.div_ceil(WORKGROUP as usize) as u32, window_count];
        let device = &msm.gpu.device;
        let [first, second, third, final_level] = &self.chunk_arguments[chunk];
        let points = msm.n.div_ceil(32) + msm.half;
        let bases = if let Some(batch) = &self.affine_batch {
            dispatch(
                device,
                encoder,
                &batch.pipeline,
                &[
                    &msm.bases, &msm.sorted, &sort.counts, &self.offsets,
                    &self.totals, &batch.pairs, first, &self.shard_buckets,
                ],
                groups((points * PAIRS_PER_SHARD).div_ceil(AFFINE_BATCH)),
                timer,
                "msm_resident_affine_pairs",
            );
            &batch.pairs
        } else {
            &msm.bases
        };

        dispatch(
            device,
            encoder,
            &self.first,
            &[
                bases, &msm.sorted, &sort.counts, &self.offsets,
                &self.totals, &msm.tree[0], first, &self.shard_buckets,
            ],
            groups(points),
            timer,
            "msm_resident_first",
        );
        for (input, output, arguments, count) in [
            (&msm.tree[0], &msm.tree[1], second, msm.n.div_ceil(1024) + msm.half),
            (&msm.tree[1], &msm.tree[0], third, msm.n.div_ceil(32768) + msm.half),
            (&msm.tree[0], &msm.tree[1], final_level, msm.half),
        ] {
            dispatch(
                device,
                encoder,
                &self.tree,
                &[input, &sort.counts, &self.offsets, &self.totals, output, arguments, &msm.buckets],
                groups(count),
                timer,
                "msm_resident_tree",
            );
        }
    }
}

pub(super) fn load_pipeline(
    gpu: &Gpu,
    path: &Path,
    name: &'static str,
    slots: &[Slot],
) -> Result<wgpu::ComputePipeline, String> {
    let kernel = std::fs::read(path.join(format!("{name}.spv")))
        .map_err(|error| format!("Android MSM shader {name}: {error}"))?;
    let pipeline = gpu.kernel(name, || spirv_pipeline(&gpu.device, gpu.cache(), name, &kernel, slots));
    let _ = gpu.device.poll(wgpu::PollType::Poll);
    if let Some(failure) = gpu.failure() {
        return Err(failure.to_owned());
    }
    Ok(pipeline)
}

pub(super) fn dispatch(
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    pipeline: &wgpu::ComputePipeline,
    buffers: &[&wgpu::Buffer],
    groups: [u32; 2],
    timer: &mut Option<profile::Timer>,
    label: &'static str,
) {
    let entries: Vec<_> = buffers.iter().enumerate().map(|(binding, buffer)| {
        wgpu::BindGroupEntry { binding: binding as u32, resource: buffer.as_entire_binding() }
    }).collect();
    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &entries,
    });
    let mut pass = match timer.as_mut() {
        Some(timer) => encoder.begin_compute_pass(&timer.pass(label)),
        None => encoder.begin_compute_pass(&Default::default()),
    };
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &group, &[]);
    pass.dispatch_workgroups(groups[0], groups[1], 1);
}
