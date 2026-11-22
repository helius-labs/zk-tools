use metal::{Buffer, ComputeCommandEncoderRef, ComputePipelineState, Device, MTLSize};
use zopro_bn254::Fr;

const TILE: usize = 512;

pub(crate) struct TensorFft {
    pipelines: [ComputePipelineState; 2],
    roots: [Buffer; 2],
    weights: [Buffer; 2],
}

impl TensorFft {
    pub(crate) fn new(device: &Device, n: usize, roots: [&[Fr]; 2]) -> Option<Self> {
        #[cfg(not(tensor_metallib))]
        {
            let _ = (device, n, roots);
            None
        }
        #[cfg(tensor_metallib)]
        {
            use metal::MTLResourceOptions;
            use objc::{msg_send, sel, sel_impl};

            if n != 1 << 21
                || crate::smaller_code()
                || device.max_threadgroup_memory_length() < 32 * 1024
            {
                return None;
            }
            // metal-rs 0.33 predates MTLGPUFamilyApple10 (1010 in the SDK).
            // SAFETY: this is MTLDevice's supportsFamily: selector, whose
            // argument is a GPU family number and whose result is BOOL.
            let apple10: objc::runtime::BOOL =
                unsafe { msg_send![&**device, supportsFamily: 1010u64] };
            if apple10 == objc::runtime::NO {
                return None;
            }
            let pipelines = tensor_pipelines(device)?;
            let upload = |bytes: &[u8]| {
                device.new_buffer_with_data(
                    bytes.as_ptr().cast(),
                    bytes.len() as u64,
                    MTLResourceOptions::StorageModeShared,
                )
            };
            let twiddles = roots.map(|powers| {
                powers
                    .iter()
                    .copied()
                    .step_by(n / TILE)
                    .take(TILE / 2)
                    .collect::<Vec<_>>()
            });
            // Each matrix contains the 32 base-256 multiples of its twiddle,
            // reduced in Fr. All rows in a local tensor multiply share it.
            let weights = twiddles.each_ref().map(|powers| {
                let radix = Fr::from(256u64);
                let mut data = Vec::with_capacity(8 * 1024);
                for mut factor in powers.iter().copied().step_by(TILE / 16).take(8) {
                    for _ in 0..32 {
                        data.extend(factor.to_canonical().into_iter().flat_map(u64::to_le_bytes));
                        factor *= radix;
                    }
                }
                upload(&data)
            });
            let roots = twiddles.map(|powers| {
                let data: Vec<_> = powers
                    .into_iter()
                    .flat_map(|x| x.0)
                    .flat_map(u64::to_le_bytes)
                    .collect();
                upload(&data)
            });
            Some(Self {
                pipelines,
                roots,
                weights,
            })
        }
    }

    pub(crate) fn encode(
        &self,
        encoder: &ComputeCommandEncoderRef,
        values: &Buffer,
        offset: u64,
        n: usize,
        dit: bool,
    ) {
        let index = usize::from(dit);
        encoder.set_compute_pipeline_state(&self.pipelines[index]);
        encoder.set_buffer(0, Some(values), offset);
        encoder.set_buffer(1, Some(&self.roots[index]), 0);
        encoder.set_buffer(2, Some(&self.weights[index]), 0);
        encoder.dispatch_thread_groups(
            MTLSize::new((n / TILE) as u64, 1, 1),
            MTLSize::new((TILE / 2) as u64, 1, 1),
        );
    }
}

#[cfg(tensor_metallib)]
fn tensor_pipelines(device: &Device) -> Option<[ComputePipelineState; 2]> {
    use std::sync::OnceLock;
    static PIPELINES: OnceLock<Option<crate::Shared<[ComputePipelineState; 2]>>> = OnceLock::new();
    PIPELINES
        .get_or_init(|| {
            let data = include_bytes!(concat!(env!("OUT_DIR"), "/zopro_tensor.metallib"));
            let library = device.new_library_with_data(data).ok()?;
            let pipeline = |name| {
                let function = library.get_function(name, None).ok()?;
                let p = device
                    .new_compute_pipeline_state_with_function(&function)
                    .ok()?;
                (p.max_total_threads_per_threadgroup() >= (TILE / 2) as u64).then_some(p)
            };
            Some(crate::Shared([
                pipeline("fr_dif_tensor9")?,
                pipeline("fr_dit_tensor9")?,
            ]))
        })
        .as_ref()
        .map(|p| p.0.clone())
}

impl Drop for TensorFft {
    fn drop(&mut self) {
        objc::rc::autoreleasepool(|| {
            self.roots
                .iter()
                .chain(&self.weights)
                .for_each(crate::discard)
        });
    }
}
