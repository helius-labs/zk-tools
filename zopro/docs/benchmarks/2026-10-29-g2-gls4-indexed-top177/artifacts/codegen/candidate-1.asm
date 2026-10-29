
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000489300 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>:
  489300:	push   %rbx
  489301:	sub    $0xc0,%rsp
  489308:	vmovups (%rdx),%zmm0
  48930e:	vmovups 0x38(%rdx),%zmm1
  489318:	mov    %fs:0x0,%rcx
  489321:	mov    %rsi,%rax
  489324:	mov    %rdi,%rbx
  489327:	lea    -0x28(%rcx),%rcx
  48932e:	mov    %rcx,0x8(%rsp)
  489333:	vmovups %zmm0,0x10(%rsp)
  48933e:	vmovups %zmm1,0x48(%rsp)
  489349:	movq   $0x0,0x88(%rsp)
  489355:	lea    0xb14(%rip),%rsi        # 489e70 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)> as rayon_core::job::Job>::execute>
  48935c:	lea    0x8(%rsp),%rdx
  489361:	mov    %rax,%rdi
  489364:	vzeroupper
  489367:	call   105a40 <<rayon_core::registry::Registry>::inject>
  48936c:	mov    0x8(%rsp),%rdi
  489371:	call   105810 <<rayon_core::latch::LockLatch>::wait_and_reset>
  489376:	vmovdqu 0x90(%rsp),%xmm0
  48937f:	mov    0x88(%rsp),%rax
  489387:	cmp    $0x1,%rax
  48938b:	je     4893a3 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xa3>
  48938d:	cmp    $0x2,%rax
  489391:	jne    4893c1 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xc1>
  489393:	vmovq  %xmm0,%rdi
  489398:	vpextrq $0x1,%xmm0,%rsi
  48939e:	call   105800 <rayon_core::unwind::resume_unwinding>
  4893a3:	vmovups 0xa0(%rsp),%ymm1
  4893ac:	vmovups %ymm1,0x10(%rbx)
  4893b1:	vmovdqu %xmm0,(%rbx)
  4893b5:	add    $0xc0,%rsp
  4893bc:	pop    %rbx
  4893bd:	vzeroupper
  4893c0:	ret
  4893c1:	lea    -0x46a13c(%rip),%rdi        # 1f28c <zmij::STATIC_DATA+0x4d4c>
  4893c8:	lea    0x69b11(%rip),%rdx        # 4f2ee0 <__frame_dummy_init_array_entry+0x66c0>
  4893cf:	mov    $0x28,%esi
  4893d4:	call   f1740 <core::panicking::panic>
  4893d9:	mov    %rax,%rbx
  4893dc:	lea    0x8(%rsp),%rdi
  4893e1:	call   1feec0 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>>>
  4893e6:	mov    %rbx,%rdi
  4893e9:	call   4eb7d0 <_Unwind_Resume@plt>
  4893ee:	call   f2bd3 <core::panicking::panic_in_cleanup>
