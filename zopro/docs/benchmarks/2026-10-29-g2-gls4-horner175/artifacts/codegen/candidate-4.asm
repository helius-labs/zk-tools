
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000486520 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>:
  486520:	push   %r14
  486522:	push   %rbx
  486523:	sub    $0xd8,%rsp
  48652a:	vmovups (%rcx),%zmm0
  486530:	vmovups 0x38(%rcx),%zmm1
  48653a:	mov    %rdx,%r14
  48653d:	mov    %rsi,%rax
  486540:	mov    0x100(%r14),%rsi
  486547:	add    $0x110,%rdx
  48654e:	mov    %rdi,%rbx
  486551:	mov    %rdx,0xb0(%rsp)
  486559:	movq   $0x0,0xb8(%rsp)
  486565:	mov    %rsi,0xc0(%rsp)
  48656d:	movb   $0x1,0xc8(%rsp)
  486575:	vmovups %zmm0,(%rsp)
  48657c:	vmovups %zmm1,0x38(%rsp)
  486587:	movq   $0x0,0x78(%rsp)
  486590:	lea    0xa9(%rip),%rsi        # 486640 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)> as rayon_core::job::Job>::execute>
  486597:	mov    %rsp,%rdx
  48659a:	mov    %rax,%rdi
  48659d:	vzeroupper
  4865a0:	call   105880 <<rayon_core::registry::Registry>::inject>
  4865a5:	mov    0xb8(%rsp),%rax
  4865ad:	cmp    $0x3,%rax
  4865b1:	jne    4865fd <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xdd>
  4865b3:	vmovdqa 0x80(%rsp),%xmm0
  4865bc:	mov    0x78(%rsp),%rax
  4865c1:	cmp    $0x1,%rax
  4865c5:	je     4865dd <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xbd>
  4865c7:	cmp    $0x2,%rax
  4865cb:	jne    48660f <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xef>
  4865cd:	vmovq  %xmm0,%rdi
  4865d2:	vpextrq $0x1,%xmm0,%rsi
  4865d8:	call   105640 <rayon_core::unwind::resume_unwinding>
  4865dd:	vmovups 0x90(%rsp),%ymm1
  4865e6:	vmovdqu %xmm0,(%rbx)
  4865ea:	vmovups %ymm1,0x10(%rbx)
  4865ef:	add    $0xd8,%rsp
  4865f6:	pop    %rbx
  4865f7:	pop    %r14
  4865f9:	vzeroupper
  4865fc:	ret
  4865fd:	lea    0xb8(%rsp),%rsi
  486605:	mov    %r14,%rdi
  486608:	call   ff350 <<rayon_core::registry::WorkerThread>::wait_until_cold>
  48660d:	jmp    4865b3 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0x93>
  48660f:	lea    -0x4673ca(%rip),%rdi        # 1f24c <zmij::STATIC_DATA+0x4d4c>
  486616:	lea    0x69553(%rip),%rdx        # 4efb70 <__frame_dummy_init_array_entry+0x66c0>
  48661d:	mov    $0x28,%esi
  486622:	call   f1580 <core::panicking::panic>
  486627:	mov    %rax,%rbx
  48662a:	mov    %rsp,%rdi
  48662d:	call   1fe800 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>>>
  486632:	mov    %rbx,%rdi
  486635:	call   4e8460 <_Unwind_Resume@plt>
  48663a:	call   f2a13 <core::panicking::panic_in_cleanup>
