//! Build the Android Euler MSM's ordered bucket runs on the device.
use super::{Curve, Gpu, GpuSort, Msm, Slot, WORKGROUP, profile, spirv_pipeline, storage, uniform};
use std::path::Path;

pub(super) struct ResidentSort {
    prefix: wgpu::ComputePipeline,
    first: wgpu::ComputePipeline,
    tree: wgpu::ComputePipeline,
    offsets: wgpu::Buffer,
    totals: wgpu::Buffer,
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
        let prefix = pipeline("msm_resident_prefix", &[Read, Write, Write, Write, Uniform])?;
        let first = pipeline("msm_resident_first", &[Read, Read, Read, Read, Read, Write, Uniform])?;
        let tree = pipeline("msm_resident_tree", &[Read, Read, Read, Read, Write, Uniform])?;
        let stride = n.div_ceil(32) + half_range;
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
            offsets: storage(&gpu.device, "resident MSM bucket offsets", windows * half_range * 16),
            totals: storage(&gpu.device, "resident MSM window totals", windows * 16),
            prefix_arguments: uniform(gpu, [n as u32, half_range as u32, windows as u32, 0]),
            chunk_arguments,
        })
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
                &sort.counts, &self.offsets, &sort.cursors, &self.totals, &self.prefix_arguments,
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
        dispatch(
            device,
            encoder,
            &self.first,
            &[&msm.bases, &msm.sorted, &sort.counts, &self.offsets, &self.totals, &msm.tree[0], first],
            groups(msm.n.div_ceil(32) + msm.half),
            timer,
            "msm_resident_first",
        );
        for (input, output, arguments, count) in [
            (&msm.tree[0], &msm.tree[1], second, msm.n.div_ceil(1024) + msm.half),
            (&msm.tree[1], &msm.tree[0], third, msm.n.div_ceil(32768) + msm.half),
            (&msm.tree[0], &msm.buckets, final_level, msm.half),
        ] {
            dispatch(
                device,
                encoder,
                &self.tree,
                &[input, &sort.counts, &self.offsets, &self.totals, output, arguments],
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
