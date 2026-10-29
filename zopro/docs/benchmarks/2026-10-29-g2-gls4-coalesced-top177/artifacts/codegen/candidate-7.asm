
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004883b0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute>:
  4883b0:	push   %rbp
  4883b1:	push   %r15
  4883b3:	push   %r14
  4883b5:	push   %r13
  4883b7:	push   %r12
  4883b9:	push   %rbx
  4883ba:	sub    $0x58,%rsp
  4883be:	mov    (%rdi),%rdx
  4883c1:	mov    0x8(%rdi),%rcx
  4883c5:	mov    0x10(%rdi),%rax
  4883c9:	movq   $0x0,(%rdi)
  4883d0:	test   %rdx,%rdx
  4883d3:	je     4884bf <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x10f>
  4883d9:	mov    0x38(%rdi),%r11
  4883dd:	mov    %rdi,%rbx
  4883e0:	lea    0x30(%rsp),%r10
  4883e5:	mov    %r11,0x50(%rsp)
  4883ea:	vmovups 0x18(%rdi),%ymm0
  4883ef:	vmovups %ymm0,0x30(%rsp)
  4883f5:	mov    %rdx,0x18(%rsp)
  4883fa:	mov    %rcx,0x20(%rsp)
  4883ff:	mov    %rax,0x28(%rsp)
  488404:	mov    (%rdx),%rsi
  488407:	mov    0x48(%rsp),%r9
  48840c:	mov    0x8(%rax),%r8
  488410:	sub    (%rcx),%rsi
  488413:	mov    (%rax),%rcx
  488416:	mov    $0x1,%edx
  48841b:	mov    %rsp,%rdi
  48841e:	push   %r10
  488420:	push   %r11
  488422:	vzeroupper
  488425:	call   47f9f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>>
  48842a:	add    $0x10,%rsp
  48842e:	mov    (%rsp),%r14
  488432:	mov    0x8(%rsp),%r15
  488437:	mov    0x10(%rsp),%r12
  48843c:	mov    $0x1,%r13d
  488442:	lea    0x40(%rbx),%rdi
  488446:	call   296510 <core::ptr::drop_glue::<rayon_core::job::JobResult<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>>
  48844b:	mov    %r13,0x40(%rbx)
  48844f:	mov    %r14,0x48(%rbx)
  488453:	mov    %r15,0x50(%rbx)
  488457:	mov    %r12,0x58(%rbx)
  48845b:	mov    0x60(%rbx),%rax
  48845f:	movzbl 0x78(%rbx),%ebp
  488463:	mov    (%rax),%rdi
  488466:	cmp    $0x1,%bpl
  48846a:	jne    48847e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0xce>
  48846c:	lock incq (%rdi)
  488470:	jle    488517 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x167>
  488476:	mov    (%rax),%r14
  488479:	mov    %r14,%rdi
  48847c:	jmp    48847e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0xce>
  48847e:	mov    $0x3,%eax
  488483:	mov    0x70(%rbx),%rsi
  488487:	xchg   %rax,0x68(%rbx)
  48848b:	cmp    $0x2,%rax
  48848f:	jne    48849d <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0xed>
  488491:	add    $0x1d8,%rdi
  488498:	call   1002e0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  48849d:	test   %bpl,%bpl
  4884a0:	je     4884b0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x100>
  4884a2:	lock decq (%r14)
  4884a6:	jne    4884b0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x100>
  4884a8:	mov    %r14,%rdi
  4884ab:	call   1010a0 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4884b0:	add    $0x58,%rsp
  4884b4:	pop    %rbx
  4884b5:	pop    %r12
  4884b7:	pop    %r13
  4884b9:	pop    %r14
  4884bb:	pop    %r15
  4884bd:	pop    %rbp
  4884be:	ret
  4884bf:	lea    0x6bf9a(%rip),%rdi        # 4f4460 <__frame_dummy_init_array_entry+0x7b00>
  4884c6:	call   f2b20 <core::option::unwrap_failed>
  4884cb:	jmp    488517 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x167>
  4884cd:	test   %bpl,%bpl
  4884d0:	je     488512 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x162>
  4884d2:	lock decq (%r14)
  4884d6:	jne    488512 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x162>
  4884d8:	mov    %r14,%rdi
  4884db:	call   1010a0 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4884e0:	jmp    488512 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x162>
  4884e2:	call   f2c93 <core::panicking::panic_in_cleanup>
  4884e7:	mov    %r13,0x40(%rbx)
  4884eb:	mov    %r14,0x48(%rbx)
  4884ef:	mov    %r15,0x50(%rbx)
  4884f3:	mov    %r12,0x58(%rbx)
  4884f7:	jmp    488512 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x162>
  4884f9:	mov    %rax,%rdi
  4884fc:	call   14537a <std::panicking::catch_unwind::cleanup>
  488501:	mov    $0x2,%r13d
  488507:	mov    %rax,%r14
  48850a:	mov    %rdx,%r15
  48850d:	jmp    488442 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x92>
  488512:	call   105df0 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  488517:	ud2
  488519:	call   f2c93 <core::panicking::panic_in_cleanup>
