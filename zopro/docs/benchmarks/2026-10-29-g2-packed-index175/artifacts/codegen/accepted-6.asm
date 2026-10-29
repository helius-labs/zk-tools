
/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000028ded0 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>>:
  28ded0:	push   %r14
  28ded2:	push   %rbx
  28ded3:	sub    $0xd8,%rsp
  28deda:	vmovups (%rcx),%zmm0
  28dee0:	vmovups 0x38(%rcx),%zmm1
  28deea:	mov    %rdx,%r14
  28deed:	mov    %rsi,%rax
  28def0:	mov    0x100(%r14),%rsi
  28def7:	add    $0x110,%rdx
  28defe:	mov    %rdi,%rbx
  28df01:	mov    %rdx,0xb0(%rsp)
  28df09:	movq   $0x0,0xb8(%rsp)
  28df15:	mov    %rsi,0xc0(%rsp)
  28df1d:	movb   $0x1,0xc8(%rsp)
  28df25:	vmovups %zmm0,(%rsp)
  28df2c:	vmovups %zmm1,0x38(%rsp)
  28df37:	movq   $0x0,0x78(%rsp)
  28df40:	lea    0xa9(%rip),%rsi        # 28dff0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)> as rayon_core::job::Job>::execute>
  28df47:	mov    %rsp,%rdx
  28df4a:	mov    %rax,%rdi
  28df4d:	vzeroupper
  28df50:	call   1046e0 <<rayon_core::registry::Registry>::inject>
  28df55:	mov    0xb8(%rsp),%rax
  28df5d:	cmp    $0x3,%rax
  28df61:	jne    28dfad <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>+0xdd>
  28df63:	vmovdqa 0x80(%rsp),%xmm0
  28df6c:	mov    0x78(%rsp),%rax
  28df71:	cmp    $0x1,%rax
  28df75:	je     28df8d <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>+0xbd>
  28df77:	cmp    $0x2,%rax
  28df7b:	jne    28dfbf <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>+0xef>
  28df7d:	vmovq  %xmm0,%rdi
  28df82:	vpextrq $0x1,%xmm0,%rsi
  28df88:	call   1044a0 <rayon_core::unwind::resume_unwinding>
  28df8d:	vmovups 0x90(%rsp),%ymm1
  28df96:	vmovdqu %xmm0,(%rbx)
  28df9a:	vmovups %ymm1,0x10(%rbx)
  28df9f:	add    $0xd8,%rsp
  28dfa6:	pop    %rbx
  28dfa7:	pop    %r14
  28dfa9:	vzeroupper
  28dfac:	ret
  28dfad:	lea    0xb8(%rsp),%rsi
  28dfb5:	mov    %r14,%rdi
  28dfb8:	call   fe1b0 <<rayon_core::registry::WorkerThread>::wait_until_cold>
  28dfbd:	jmp    28df63 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>+0x93>
  28dfbf:	lea    -0x26ee7a(%rip),%rdi        # 1f14c <zmij::STATIC_DATA+0x4d4c>
  28dfc6:	lea    0x256ce3(%rip),%rdx        # 4e4cb0 <__frame_dummy_init_array_entry+0x6690>
  28dfcd:	mov    $0x28,%esi
  28dfd2:	call   f03d0 <core::panicking::panic>
  28dfd7:	mov    %rax,%rbx
  28dfda:	mov    %rsp,%rdi
  28dfdd:	call   1fc590 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>>>
  28dfe2:	mov    %rbx,%rdi
  28dfe5:	call   4dd5d0 <_Unwind_Resume@plt>
  28dfea:	call   f1863 <core::panicking::panic_in_cleanup>
