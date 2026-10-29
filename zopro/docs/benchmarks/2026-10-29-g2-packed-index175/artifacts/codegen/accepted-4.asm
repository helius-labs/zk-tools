
/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000028ddd0 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>>:
  28ddd0:	push   %rbx
  28ddd1:	sub    $0xc0,%rsp
  28ddd8:	vmovups (%rdx),%zmm0
  28ddde:	vmovups 0x38(%rdx),%zmm1
  28dde8:	mov    %fs:0x0,%rcx
  28ddf1:	mov    %rsi,%rax
  28ddf4:	mov    %rdi,%rbx
  28ddf7:	lea    -0x28(%rcx),%rcx
  28ddfe:	vmovups %zmm1,0x48(%rsp)
  28de09:	vmovups %zmm0,0x10(%rsp)
  28de14:	mov    %rcx,0x8(%rsp)
  28de19:	movq   $0x0,0x88(%rsp)
  28de25:	lea    0x4d4(%rip),%rsi        # 28e300 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)> as rayon_core::job::Job>::execute>
  28de2c:	lea    0x8(%rsp),%rdx
  28de31:	mov    %rax,%rdi
  28de34:	vzeroupper
  28de37:	call   1046e0 <<rayon_core::registry::Registry>::inject>
  28de3c:	mov    0x8(%rsp),%rdi
  28de41:	call   1044b0 <<rayon_core::latch::LockLatch>::wait_and_reset>
  28de46:	vmovdqu 0x90(%rsp),%xmm0
  28de4f:	mov    0x88(%rsp),%rax
  28de57:	cmp    $0x1,%rax
  28de5b:	je     28de73 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>+0xa3>
  28de5d:	cmp    $0x2,%rax
  28de61:	jne    28de91 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>+0xc1>
  28de63:	vmovq  %xmm0,%rdi
  28de68:	vpextrq $0x1,%xmm0,%rsi
  28de6e:	call   1044a0 <rayon_core::unwind::resume_unwinding>
  28de73:	vmovups 0xa0(%rsp),%ymm1
  28de7c:	vmovups %ymm1,0x10(%rbx)
  28de81:	vmovdqu %xmm0,(%rbx)
  28de85:	add    $0xc0,%rsp
  28de8c:	pop    %rbx
  28de8d:	vzeroupper
  28de90:	ret
  28de91:	lea    -0x26ed4c(%rip),%rdi        # 1f14c <zmij::STATIC_DATA+0x4d4c>
  28de98:	lea    0x256e11(%rip),%rdx        # 4e4cb0 <__frame_dummy_init_array_entry+0x6690>
  28de9f:	mov    $0x28,%esi
  28dea4:	call   f03d0 <core::panicking::panic>
  28dea9:	mov    %rax,%rbx
  28deac:	lea    0x8(%rsp),%rdi
  28deb1:	call   1fc910 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>>>
  28deb6:	mov    %rbx,%rdi
  28deb9:	call   4dd5d0 <_Unwind_Resume@plt>
  28debe:	call   f1863 <core::panicking::panic_in_cleanup>
