
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004899d0 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>:
  4899d0:	push   %r14
  4899d2:	push   %rbx
  4899d3:	sub    $0xd8,%rsp
  4899da:	vmovups (%rcx),%zmm0
  4899e0:	vmovups 0x38(%rcx),%zmm1
  4899ea:	mov    %rdx,%r14
  4899ed:	mov    %rsi,%rax
  4899f0:	mov    0x100(%r14),%rsi
  4899f7:	add    $0x110,%rdx
  4899fe:	mov    %rdi,%rbx
  489a01:	mov    %rdx,0xb0(%rsp)
  489a09:	movq   $0x0,0xb8(%rsp)
  489a15:	mov    %rsi,0xc0(%rsp)
  489a1d:	movb   $0x1,0xc8(%rsp)
  489a25:	vmovups %zmm0,(%rsp)
  489a2c:	vmovups %zmm1,0x38(%rsp)
  489a37:	movq   $0x0,0x78(%rsp)
  489a40:	lea    0xa9(%rip),%rsi        # 489af0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)> as rayon_core::job::Job>::execute>
  489a47:	mov    %rsp,%rdx
  489a4a:	mov    %rax,%rdi
  489a4d:	vzeroupper
  489a50:	call   105b00 <<rayon_core::registry::Registry>::inject>
  489a55:	mov    0xb8(%rsp),%rax
  489a5d:	cmp    $0x3,%rax
  489a61:	jne    489aad <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xdd>
  489a63:	vmovdqa 0x80(%rsp),%xmm0
  489a6c:	mov    0x78(%rsp),%rax
  489a71:	cmp    $0x1,%rax
  489a75:	je     489a8d <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xbd>
  489a77:	cmp    $0x2,%rax
  489a7b:	jne    489abf <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xef>
  489a7d:	vmovq  %xmm0,%rdi
  489a82:	vpextrq $0x1,%xmm0,%rsi
  489a88:	call   1058c0 <rayon_core::unwind::resume_unwinding>
  489a8d:	vmovups 0x90(%rsp),%ymm1
  489a96:	vmovdqu %xmm0,(%rbx)
  489a9a:	vmovups %ymm1,0x10(%rbx)
  489a9f:	add    $0xd8,%rsp
  489aa6:	pop    %rbx
  489aa7:	pop    %r14
  489aa9:	vzeroupper
  489aac:	ret
  489aad:	lea    0xb8(%rsp),%rsi
  489ab5:	mov    %r14,%rdi
  489ab8:	call   ff5d0 <<rayon_core::registry::WorkerThread>::wait_until_cold>
  489abd:	jmp    489a63 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0x93>
  489abf:	lea    -0x46a83a(%rip),%rdi        # 1f28c <zmij::STATIC_DATA+0x4d4c>
  489ac6:	lea    0x69553(%rip),%rdx        # 4f3020 <__frame_dummy_init_array_entry+0x66c0>
  489acd:	mov    $0x28,%esi
  489ad2:	call   f1800 <core::panicking::panic>
  489ad7:	mov    %rax,%rbx
  489ada:	mov    %rsp,%rdi
  489add:	call   1fea80 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>>>
  489ae2:	mov    %rbx,%rdi
  489ae5:	call   4eb910 <_Unwind_Resume@plt>
  489aea:	call   f2c93 <core::panicking::panic_in_cleanup>
