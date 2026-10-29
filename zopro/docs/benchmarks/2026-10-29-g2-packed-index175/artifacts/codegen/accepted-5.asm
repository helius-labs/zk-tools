
/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001fc160 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>>:
  1fc160:	push   %r14
  1fc162:	push   %rbx
  1fc163:	sub    $0xd8,%rsp
  1fc16a:	vmovups (%rcx),%zmm0
  1fc170:	vmovups 0x38(%rcx),%zmm1
  1fc17a:	mov    %rdx,%r14
  1fc17d:	mov    %rsi,%rax
  1fc180:	mov    0x100(%r14),%rsi
  1fc187:	add    $0x110,%rdx
  1fc18e:	mov    %rdi,%rbx
  1fc191:	mov    %rdx,0xb0(%rsp)
  1fc199:	movq   $0x0,0xb8(%rsp)
  1fc1a5:	mov    %rsi,0xc0(%rsp)
  1fc1ad:	movb   $0x1,0xc8(%rsp)
  1fc1b5:	vmovups %zmm0,(%rsp)
  1fc1bc:	vmovups %zmm1,0x38(%rsp)
  1fc1c7:	movq   $0x0,0x78(%rsp)
  1fc1d0:	lea    0xa9(%rip),%rsi        # 1fc280 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)> as rayon_core::job::Job>::execute>
  1fc1d7:	mov    %rsp,%rdx
  1fc1da:	mov    %rax,%rdi
  1fc1dd:	vzeroupper
  1fc1e0:	call   1046e0 <<rayon_core::registry::Registry>::inject>
  1fc1e5:	mov    0xb8(%rsp),%rax
  1fc1ed:	cmp    $0x3,%rax
  1fc1f1:	jne    1fc23d <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>+0xdd>
  1fc1f3:	vmovdqa 0x80(%rsp),%xmm0
  1fc1fc:	mov    0x78(%rsp),%rax
  1fc201:	cmp    $0x1,%rax
  1fc205:	je     1fc21d <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>+0xbd>
  1fc207:	cmp    $0x2,%rax
  1fc20b:	jne    1fc24f <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>+0xef>
  1fc20d:	vmovq  %xmm0,%rdi
  1fc212:	vpextrq $0x1,%xmm0,%rsi
  1fc218:	call   1044a0 <rayon_core::unwind::resume_unwinding>
  1fc21d:	vmovups 0x90(%rsp),%ymm1
  1fc226:	vmovdqu %xmm0,(%rbx)
  1fc22a:	vmovups %ymm1,0x10(%rbx)
  1fc22f:	add    $0xd8,%rsp
  1fc236:	pop    %rbx
  1fc237:	pop    %r14
  1fc239:	vzeroupper
  1fc23c:	ret
  1fc23d:	lea    0xb8(%rsp),%rsi
  1fc245:	mov    %r14,%rdi
  1fc248:	call   fe1b0 <<rayon_core::registry::WorkerThread>::wait_until_cold>
  1fc24d:	jmp    1fc1f3 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>+0x93>
  1fc24f:	lea    -0x1dd10a(%rip),%rdi        # 1f14c <zmij::STATIC_DATA+0x4d4c>
  1fc256:	lea    0x2e8a53(%rip),%rdx        # 4e4cb0 <__frame_dummy_init_array_entry+0x6690>
  1fc25d:	mov    $0x28,%esi
  1fc262:	call   f03d0 <core::panicking::panic>
  1fc267:	mov    %rax,%rbx
  1fc26a:	mov    %rsp,%rdi
  1fc26d:	call   1fc590 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>>>
  1fc272:	mov    %rbx,%rdi
  1fc275:	call   4dd5d0 <_Unwind_Resume@plt>
  1fc27a:	call   f1863 <core::panicking::panic_in_cleanup>
