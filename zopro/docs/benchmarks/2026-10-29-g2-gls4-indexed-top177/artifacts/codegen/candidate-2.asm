
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004879c0 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>:
  4879c0:	push   %r14
  4879c2:	push   %rbx
  4879c3:	sub    $0xb8,%rsp
  4879ca:	vmovups (%rdx),%zmm0
  4879d0:	vmovups 0x38(%rdx),%zmm1
  4879da:	mov    %fs:0x0,%rcx
  4879e3:	lea    0x80(%rsp),%r14
  4879eb:	mov    %rsi,%rax
  4879ee:	mov    %rdi,%rbx
  4879f1:	lea    -0x28(%rcx),%rcx
  4879f8:	mov    %rcx,(%rsp)
  4879fc:	vmovups %zmm0,0x8(%rsp)
  487a07:	vmovups %zmm1,0x40(%rsp)
  487a0f:	movq   $0x0,0x80(%rsp)
  487a1b:	lea    0x9be(%rip),%rsi        # 4883e0 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)> as rayon_core::job::Job>::execute>
  487a22:	mov    %rsp,%rdx
  487a25:	mov    %rax,%rdi
  487a28:	vzeroupper
  487a2b:	call   105a40 <<rayon_core::registry::Registry>::inject>
  487a30:	mov    (%rsp),%rdi
  487a34:	call   105810 <<rayon_core::latch::LockLatch>::wait_and_reset>
  487a39:	vmovdqu 0x88(%rsp),%xmm0
  487a42:	mov    0x80(%rsp),%rax
  487a4a:	cmp    $0x1,%rax
  487a4e:	je     487a66 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>+0xa6>
  487a50:	cmp    $0x2,%rax
  487a54:	jne    487a86 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>+0xc6>
  487a56:	vmovq  %xmm0,%rdi
  487a5b:	vpextrq $0x1,%xmm0,%rsi
  487a61:	call   105800 <rayon_core::unwind::resume_unwinding>
  487a66:	vmovups 0x98(%rsp),%ymm1
  487a6f:	vmovups %ymm1,0x10(%rbx)
  487a74:	vmovdqu %xmm0,(%rbx)
  487a78:	add    $0xb8,%rsp
  487a7f:	pop    %rbx
  487a80:	pop    %r14
  487a82:	vzeroupper
  487a85:	ret
  487a86:	lea    -0x468801(%rip),%rdi        # 1f28c <zmij::STATIC_DATA+0x4d4c>
  487a8d:	lea    0x6b44c(%rip),%rdx        # 4f2ee0 <__frame_dummy_init_array_entry+0x66c0>
  487a94:	mov    $0x28,%esi
  487a99:	call   f1740 <core::panicking::panic>
  487a9e:	mov    %rax,%rbx
  487aa1:	mov    %r14,%rdi
  487aa4:	call   296110 <core::ptr::drop_glue::<rayon_core::job::JobResult<(rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>>
  487aa9:	mov    %rbx,%rdi
  487aac:	call   4eb7d0 <_Unwind_Resume@plt>
  487ab1:	call   f2bd3 <core::panicking::panic_in_cleanup>
