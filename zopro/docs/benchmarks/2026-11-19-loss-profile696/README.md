# Warm CPU profiles on the four highest Zopro-to-Aeglos latency ratios

The eight-block confirmation selected the four largest ratios of Zopro to Aeglos median time. Both provers then ran plain, under user-cycle sampling and under separate minor-fault sampling in three new blocks at the same 96 physical cores. FIFO control enabled collection after three primes and stopped it after 31 measured requests. The prover ran at its ordinary unprivileged UID. Every profile reported zero lost records. All 72 exported proofs independently verified against their original key and expected public inputs. CPU-ms and peak RSS are from the plain runs. The profile effect is a paired latency difference. Period share across threads measures CPU activity and can include inlined work. The symbols were decoded from the retained recordings with the instruction-pointer field explicitly included in perf script. The first text export lacked symbol names. The separate fault profiles sample every eleventh user-mode minor page fault. The instruction address identifies the access that touched the page. The count does not measure time spent handling faults.

| Shape | Configuration | Plain ms | Profile effect | CPU-ms | Peak MiB | User-cycle samples | Plain minor faults per proof |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x16 | aeglos | 12.290 | +4.50% | 877.1 | 821.0 | 70,083 | 135 |
| 1x16 | runtime-off | 16.847 | +7.25% | 729.6 | 114.1 | 58,414 | 279 |
| 2x16 | aeglos | 16.962 | +3.51% | 1252.8 | 1231.8 | 97,198 | 347 |
| 2x16 | coeff-packed | 21.806 | +6.13% | 1180.1 | 335.3 | 92,735 | 1,528 |
| 1x8 | aeglos | 10.608 | +5.25% | 726.5 | 690.3 | 57,760 | 210 |
| 1x8 | wide32 | 13.385 | +7.76% | 648.5 | 88.0 | 53,721 | 261 |
| 1x4 | aeglos | 9.430 | +4.19% | 654.3 | 584.5 | 52,943 | 161 |
| 1x4 | bands-tree2 | 11.789 | +8.98% | 602.5 | 125.3 | 49,380 | 302 |

### 1x16, aeglos

| Symbol and image | Sampled user-cycle period |
|---|---:|
| aeglos / aeglos::cpu::Pool::work(unsigned int) | 22.36% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(unsigned long const*, unsigned char*) | 10.28% |
| aeglos / aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::invert_all(aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E*, unsigned long) | 9.38% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::load(unsigned long const*, unsigned long const*, unsigned long) | 8.80% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(unsigned long const*, unsigned char*) | 7.95% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::load(unsigned long const*, unsigned long const*, unsigned long) | 6.00% |

Minor-fault sample locations in separate processes.

| Symbol and image | Share of sampled minor faults |
|---|---:|
| aeglos / std::vector<unsigned int, std::allocator<unsigned int> >::_M_fill_assign(unsigned long, unsigned int const&) | 44.64% |
| libc.so.6 / [unresolved] | 41.05% |
| aeglos / std::vector<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E, std::allocator<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E> >::_M_default_append(unsigned long) | 5.90% |
| aeglos / std::vector<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E, std::allocator<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E> >::_M_default_append(unsigned long) | 5.54% |
| aeglos / aeglos::cpu::ifma512::Projective<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E const&, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E const&, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E const&, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E) | 0.50% |

### 1x16, runtime-off

| Symbol and image | Sampled user-cycle period |
|---|---:|
| runtime-sdk / crossbeam_epoch::default::with_handle::<crossbeam_epoch::default::pin::{closure#0}, crossbeam_epoch::guard::Guard> | 20.63% |
| runtime-sdk / <crossbeam_epoch::internal::Global>::try_advance | 15.83% |
| runtime-sdk / zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>> | 14.56% |
| runtime-sdk / zopro_core::msm::g2_borrowed::apply::<zopro_bn254::curve::G2Config, true> | 11.69% |
| runtime-sdk / <&zopro_core::msm_affine::borrowed_pending::msm<zopro_bn254::curve::G1Config>::{closure#0}::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut | 3.57% |
| runtime-sdk / zopro_bn254::inverse::inverse::<zopro_bn254::FqConfig> | 3.28% |

Minor-fault sample locations in separate processes.

| Symbol and image | Share of sampled minor faults |
|---|---:|
| libc.so.6 / [unresolved] | 40.66% |
| runtime-sdk / zopro_core::msm::reduce_layer::<zopro_bn254::curve::G1Config> | 18.68% |
| runtime-sdk / rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::enumerate::EnumerateProducer<rayon::slice::chunks::ChunksMutProducer<core::mem::maybe_uninit::MaybeUninit<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>, rayon::iter::map_with::MapInitConsumer<rayon::iter::noop::NoopConsumer, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G1Config, zopro_core::msm::group_affine<zopro_bn254::curve::G1Config>::{closure#0}>::{closure#0}, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G1Config, zopro_core::msm::group_affine<zopro_bn254::curve::G1Config>::{closure#0}>::{closure#1}>> | 13.42% |
| runtime-sdk / rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::enumerate::EnumerateProducer<rayon::slice::chunks::ChunksMutProducer<core::mem::maybe_uninit::MaybeUninit<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>, rayon::iter::map_with::MapInitConsumer<rayon::iter::noop::NoopConsumer, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G2Config, zopro_core::msm::group_affine<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G2Config, zopro_core::msm::group_affine<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#1}>> | 8.74% |
| runtime-sdk / zopro_core::msm_affine::grouping::classify_nonzero::<ahash::random_state::RandomState> | 7.58% |

### 2x16, aeglos

| Symbol and image | Sampled user-cycle period |
|---|---:|
| aeglos / aeglos::cpu::Pool::work(unsigned int) | 15.14% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(unsigned long const*, unsigned char*) | 11.68% |
| aeglos / aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::invert_all(aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E*, unsigned long) | 9.84% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::load(unsigned long const*, unsigned long const*, unsigned long) | 9.78% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(unsigned long const*, unsigned char*) | 7.99% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::load(unsigned long const*, unsigned long const*, unsigned long) | 5.81% |

Minor-fault sample locations in separate processes.

| Symbol and image | Share of sampled minor faults |
|---|---:|
| libc.so.6 / [unresolved] | 76.49% |
| aeglos / std::vector<unsigned int, std::allocator<unsigned int> >::_M_fill_assign(unsigned long, unsigned int const&) | 16.19% |
| aeglos / std::vector<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E, std::allocator<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E> >::_M_default_append(unsigned long) | 3.11% |
| aeglos / std::vector<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E, std::allocator<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E> >::_M_default_append(unsigned long) | 2.06% |
| aeglos / std::vector<unsigned long, std::allocator<unsigned long> >::_M_fill_assign(unsigned long, unsigned long const&) | 0.71% |

### 2x16, coeff-packed

| Symbol and image | Sampled user-cycle period |
|---|---:|
| release / crossbeam_epoch::default::with_handle::<crossbeam_epoch::default::pin::{closure#0}, crossbeam_epoch::guard::Guard> | 21.29% |
| release / <crossbeam_epoch::internal::Global>::try_advance | 16.58% |
| release / zopro_core::msm::g2_prefix::apply::<zopro_bn254::curve::G2Config, true> | 8.15% |
| release / zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, false, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>> | 5.64% |
| release / zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>> | 5.09% |
| release / rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterMutProducer<[u64; 4]>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::grouping::group_witness::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterMutProducer<[u64; 4]>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::grouping::group_witness::{closure#0}>>::{closure#1}, | 3.63% |

Minor-fault sample locations in separate processes.

| Symbol and image | Share of sampled minor faults |
|---|---:|
| release / rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::enumerate::EnumerateProducer<rayon::slice::chunks::ChunksMutProducer<core::mem::maybe_uninit::MaybeUninit<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>, rayon::iter::map_with::MapInitConsumer<rayon::iter::noop::NoopConsumer, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G2Config, zopro_core::msm::group_affine<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G2Config, zopro_core::msm::group_affine<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#1}>> | 43.28% |
| release / rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::enumerate::EnumerateProducer<rayon::slice::chunks::ChunksMutProducer<core::mem::maybe_uninit::MaybeUninit<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>, rayon::iter::map_with::MapInitConsumer<rayon::iter::noop::NoopConsumer, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G1Config, zopro_core::msm::group_affine<zopro_bn254::curve::G1Config>::{closure#0}>::{closure#0}, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G1Config, zopro_core::msm::group_affine<zopro_bn254::curve::G1Config>::{closure#0}>::{closure#1}>> | 20.67% |
| libc.so.6 / [unresolved] | 15.70% |
| release / zopro_core::msm_affine::grouping::classify_nonzero::<ahash::random_state::RandomState> | 11.55% |
| release / zopro_core::fft::pair::euler_center::transform | 1.78% |

### 1x8, aeglos

| Symbol and image | Sampled user-cycle period |
|---|---:|
| aeglos / aeglos::cpu::Pool::work(unsigned int) | 23.94% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(unsigned long const*, unsigned char*) | 9.73% |
| aeglos / aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::invert_all(aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E*, unsigned long) | 9.09% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::load(unsigned long const*, unsigned long const*, unsigned long) | 8.93% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(unsigned long const*, unsigned char*) | 6.78% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::load(unsigned long const*, unsigned long const*, unsigned long) | 5.05% |

Minor-fault sample locations in separate processes.

| Symbol and image | Share of sampled minor faults |
|---|---:|
| libc.so.6 / [unresolved] | 40.54% |
| aeglos / std::vector<unsigned int, std::allocator<unsigned int> >::_M_fill_assign(unsigned long, unsigned int const&) | 40.22% |
| aeglos / std::vector<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E, std::allocator<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E> >::_M_default_append(unsigned long) | 8.49% |
| aeglos / std::vector<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E, std::allocator<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E> >::_M_default_append(unsigned long) | 4.06% |
| aeglos / runtime.memclrNoHeapPointers | 1.56% |

### 1x8, wide32

| Symbol and image | Sampled user-cycle period |
|---|---:|
| owned / crossbeam_epoch::default::with_handle::<crossbeam_epoch::default::pin::{closure#0}, crossbeam_epoch::guard::Guard> | 21.28% |
| owned / <crossbeam_epoch::internal::Global>::try_advance | 18.80% |
| owned / zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>> | 12.60% |
| owned / zopro_core::msm::g2_borrowed::apply::<zopro_bn254::curve::G2Config, true> | 10.45% |
| owned / <&zopro_core::msm_affine::borrowed_pending::msm<zopro_bn254::curve::G1Config>::{closure#0}::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut | 3.03% |
| owned / zopro_core::msm::reduce_layer::<zopro_bn254::curve::G1Config> | 3.00% |

Minor-fault sample locations in separate processes.

| Symbol and image | Share of sampled minor faults |
|---|---:|
| libc.so.6 / [unresolved] | 40.96% |
| owned / zopro_core::msm::reduce_layer::<zopro_bn254::curve::G1Config> | 17.78% |
| owned / rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::enumerate::EnumerateProducer<rayon::slice::chunks::ChunksMutProducer<core::mem::maybe_uninit::MaybeUninit<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>, rayon::iter::map_with::MapInitConsumer<rayon::iter::noop::NoopConsumer, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G1Config, zopro_core::msm::group_affine<zopro_bn254::curve::G1Config>::{closure#0}>::{closure#0}, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G1Config, zopro_core::msm::group_affine<zopro_bn254::curve::G1Config>::{closure#0}>::{closure#1}>> | 12.07% |
| owned / rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::enumerate::EnumerateProducer<rayon::slice::chunks::ChunksMutProducer<core::mem::maybe_uninit::MaybeUninit<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>, rayon::iter::map_with::MapInitConsumer<rayon::iter::noop::NoopConsumer, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G2Config, zopro_core::msm::group_affine<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G2Config, zopro_core::msm::group_affine<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#1}>> | 9.73% |
| owned / zopro_core::msm_affine::grouping::classify_nonzero::<ahash::random_state::RandomState> | 5.16% |

### 1x4, aeglos

| Symbol and image | Sampled user-cycle period |
|---|---:|
| aeglos / aeglos::cpu::Pool::work(unsigned int) | 26.26% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(unsigned long const*, unsigned char*) | 9.56% |
| aeglos / aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::invert_all(aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E*, unsigned long) | 8.49% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane1<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::load(unsigned long const*, unsigned long const*, unsigned long) | 8.46% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(unsigned long const*, unsigned char*) | 5.95% |
| aeglos / aeglos::cpu::ifma512::PairBatch<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::load(unsigned long const*, unsigned long const*, unsigned long) | 4.89% |

Minor-fault sample locations in separate processes.

| Symbol and image | Share of sampled minor faults |
|---|---:|
| libc.so.6 / [unresolved] | 52.14% |
| aeglos / std::vector<unsigned int, std::allocator<unsigned int> >::_M_fill_assign(unsigned long, unsigned int const&) | 30.40% |
| aeglos / std::vector<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E, std::allocator<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E> >::_M_default_append(unsigned long) | 9.10% |
| aeglos / std::vector<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E, std::allocator<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams>::E> >::_M_default_append(unsigned long) | 4.88% |
| aeglos / aeglos::cpu::ifma512::Projective<aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> > >::finish(aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E const&, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E const&, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E const&, aeglos::cpu::ifma512::Lane2<aeglos::cpu::ifma512::VMont<aeglos::cpu::ifma512::Ifma<aeglos::cpu::ifma512::Zmm>, aeglos::cpu::ifma512::FpParams> >::E) | 0.96% |

### 1x4, bands-tree2

| Symbol and image | Sampled user-cycle period |
|---|---:|
| band-sdk / crossbeam_epoch::default::with_handle::<crossbeam_epoch::default::pin::{closure#0}, crossbeam_epoch::guard::Guard> | 26.01% |
| band-sdk / <crossbeam_epoch::internal::Global>::try_advance | 16.11% |
| band-sdk / zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>> | 10.54% |
| band-sdk / zopro_core::msm::g2_borrowed::apply::<zopro_bn254::curve::G2Config, true> | 8.44% |
| band-sdk / zopro_core::msm::reduce_layer::<zopro_bn254::curve::G1Config> | 3.24% |
| band-sdk / <&zopro_core::msm_affine::borrowed_pending::msm<zopro_bn254::curve::G1Config>::{closure#0}::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut | 2.78% |

Minor-fault sample locations in separate processes.

| Symbol and image | Share of sampled minor faults |
|---|---:|
| libc.so.6 / [unresolved] | 44.83% |
| band-sdk / zopro_core::msm_affine::g2_phi::staged_tree::append | 16.55% |
| band-sdk / rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::enumerate::EnumerateProducer<rayon::slice::chunks::ChunksMutProducer<core::mem::maybe_uninit::MaybeUninit<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>, rayon::iter::map_with::MapInitConsumer<rayon::iter::noop::NoopConsumer, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G2Config, zopro_core::msm::group_affine<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, zopro_core::msm::group_affine_impl<zopro_bn254::curve::G2Config, zopro_core::msm::group_affine<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#1}>> | 8.77% |
| band-sdk / rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::enumerate::EnumerateProducer<rayon::slice::chunks::ChunksMutProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::map_with::MapInitConsumer<rayon::iter::noop::NoopConsumer, <zopro_core::msm_affine::g2_phi::staged_tree::State as core::default::Default>::default, zopro_core::msm_affine::g2_phi::staged_tree::window<12>::{closure#0}>> | 7.20% |
| band-sdk / rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::ifma8::PackedIfma8<zopro_bn254::FrConfig>>, zopro_core::fft::pair::euler_product_study::{closure#1}>> | 6.87% |

[Selection and full confirmation](../2026-11-19-loss-routes689/README.md) · [Warm sampling](artifacts/sample.sh) · [All observed symbols and samples](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
