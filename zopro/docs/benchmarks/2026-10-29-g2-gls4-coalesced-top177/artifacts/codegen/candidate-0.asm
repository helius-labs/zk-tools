
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000488690 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>>:
  488690:	push   %rbx
  488691:	sub    $0xc0,%rsp
  488698:	vmovups (%rdx),%zmm0
  48869e:	vmovups 0x38(%rdx),%zmm1
  4886a8:	mov    %fs:0x0,%rcx
  4886b1:	mov    %rsi,%rax
  4886b4:	mov    %rdi,%rbx
  4886b7:	lea    -0x28(%rcx),%rcx
  4886be:	mov    %rcx,0x8(%rsp)
  4886c3:	vmovups %zmm0,0x10(%rsp)
  4886ce:	vmovups %zmm1,0x48(%rsp)
  4886d9:	movq   $0x0,0x88(%rsp)
  4886e5:	lea    0xb34(%rip),%rsi        # 489220 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)> as rayon_core::job::Job>::execute>
  4886ec:	lea    0x8(%rsp),%rdx
  4886f1:	mov    %rax,%rdi
  4886f4:	vzeroupper
  4886f7:	call   105b00 <<rayon_core::registry::Registry>::inject>
  4886fc:	mov    0x8(%rsp),%rdi
  488701:	call   1058d0 <<rayon_core::latch::LockLatch>::wait_and_reset>
  488706:	vmovdqu 0x90(%rsp),%xmm0
  48870f:	mov    0x88(%rsp),%rax
  488717:	cmp    $0x1,%rax
  48871b:	je     488733 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>+0xa3>
  48871d:	cmp    $0x2,%rax
  488721:	jne    488751 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>+0xc1>
  488723:	vmovq  %xmm0,%rdi
  488728:	vpextrq $0x1,%xmm0,%rsi
  48872e:	call   1058c0 <rayon_core::unwind::resume_unwinding>
  488733:	vmovups 0xa0(%rsp),%ymm1
  48873c:	vmovups %ymm1,0x10(%rbx)
  488741:	vmovdqu %xmm0,(%rbx)
  488745:	add    $0xc0,%rsp
  48874c:	pop    %rbx
  48874d:	vzeroupper
  488750:	ret
  488751:	lea    -0x4694cc(%rip),%rdi        # 1f28c <zmij::STATIC_DATA+0x4d4c>
  488758:	lea    0x6a8c1(%rip),%rdx        # 4f3020 <__frame_dummy_init_array_entry+0x66c0>
  48875f:	mov    $0x28,%esi
  488764:	call   f1800 <core::panicking::panic>
  488769:	mov    %rax,%rbx
  48876c:	lea    0x8(%rsp),%rdi
  488771:	call   1fef80 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>>>
  488776:	mov    %rbx,%rdi
  488779:	call   4eb910 <_Unwind_Resume@plt>
  48877e:	call   f2c93 <core::panicking::panic_in_cleanup>
