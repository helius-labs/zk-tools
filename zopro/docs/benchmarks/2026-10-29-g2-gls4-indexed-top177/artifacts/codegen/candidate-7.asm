
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000488270 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute>:
  488270:	push   %rbp
  488271:	push   %r15
  488273:	push   %r14
  488275:	push   %r13
  488277:	push   %r12
  488279:	push   %rbx
  48827a:	sub    $0x58,%rsp
  48827e:	mov    (%rdi),%rdx
  488281:	mov    0x8(%rdi),%rcx
  488285:	mov    0x10(%rdi),%rax
  488289:	movq   $0x0,(%rdi)
  488290:	test   %rdx,%rdx
  488293:	je     48837f <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x10f>
  488299:	mov    0x38(%rdi),%r11
  48829d:	mov    %rdi,%rbx
  4882a0:	lea    0x30(%rsp),%r10
  4882a5:	mov    %r11,0x50(%rsp)
  4882aa:	vmovups 0x18(%rdi),%ymm0
  4882af:	vmovups %ymm0,0x30(%rsp)
  4882b5:	mov    %rdx,0x18(%rsp)
  4882ba:	mov    %rcx,0x20(%rsp)
  4882bf:	mov    %rax,0x28(%rsp)
  4882c4:	mov    (%rdx),%rsi
  4882c7:	mov    0x48(%rsp),%r9
  4882cc:	mov    0x8(%rax),%r8
  4882d0:	sub    (%rcx),%rsi
  4882d3:	mov    (%rax),%rcx
  4882d6:	mov    $0x1,%edx
  4882db:	mov    %rsp,%rdi
  4882de:	push   %r10
  4882e0:	push   %r11
  4882e2:	vzeroupper
  4882e5:	call   47f900 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>>
  4882ea:	add    $0x10,%rsp
  4882ee:	mov    (%rsp),%r14
  4882f2:	mov    0x8(%rsp),%r15
  4882f7:	mov    0x10(%rsp),%r12
  4882fc:	mov    $0x1,%r13d
  488302:	lea    0x40(%rbx),%rdi
  488306:	call   296450 <core::ptr::drop_glue::<rayon_core::job::JobResult<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>>
  48830b:	mov    %r13,0x40(%rbx)
  48830f:	mov    %r14,0x48(%rbx)
  488313:	mov    %r15,0x50(%rbx)
  488317:	mov    %r12,0x58(%rbx)
  48831b:	mov    0x60(%rbx),%rax
  48831f:	movzbl 0x78(%rbx),%ebp
  488323:	mov    (%rax),%rdi
  488326:	cmp    $0x1,%bpl
  48832a:	jne    48833e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0xce>
  48832c:	lock incq (%rdi)
  488330:	jle    4883d7 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x167>
  488336:	mov    (%rax),%r14
  488339:	mov    %r14,%rdi
  48833c:	jmp    48833e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0xce>
  48833e:	mov    $0x3,%eax
  488343:	mov    0x70(%rbx),%rsi
  488347:	xchg   %rax,0x68(%rbx)
  48834b:	cmp    $0x2,%rax
  48834f:	jne    48835d <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0xed>
  488351:	add    $0x1d8,%rdi
  488358:	call   100220 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  48835d:	test   %bpl,%bpl
  488360:	je     488370 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x100>
  488362:	lock decq (%r14)
  488366:	jne    488370 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x100>
  488368:	mov    %r14,%rdi
  48836b:	call   100fe0 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  488370:	add    $0x58,%rsp
  488374:	pop    %rbx
  488375:	pop    %r12
  488377:	pop    %r13
  488379:	pop    %r14
  48837b:	pop    %r15
  48837d:	pop    %rbp
  48837e:	ret
  48837f:	lea    0x6bf9a(%rip),%rdi        # 4f4320 <__frame_dummy_init_array_entry+0x7b00>
  488386:	call   f2a60 <core::option::unwrap_failed>
  48838b:	jmp    4883d7 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x167>
  48838d:	test   %bpl,%bpl
  488390:	je     4883d2 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x162>
  488392:	lock decq (%r14)
  488396:	jne    4883d2 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x162>
  488398:	mov    %r14,%rdi
  48839b:	call   100fe0 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4883a0:	jmp    4883d2 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x162>
  4883a2:	call   f2bd3 <core::panicking::panic_in_cleanup>
  4883a7:	mov    %r13,0x40(%rbx)
  4883ab:	mov    %r14,0x48(%rbx)
  4883af:	mov    %r15,0x50(%rbx)
  4883b3:	mov    %r12,0x58(%rbx)
  4883b7:	jmp    4883d2 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x162>
  4883b9:	mov    %rax,%rdi
  4883bc:	call   1452ba <std::panicking::catch_unwind::cleanup>
  4883c1:	mov    $0x2,%r13d
  4883c7:	mov    %rax,%r14
  4883ca:	mov    %rdx,%r15
  4883cd:	jmp    488302 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>> as rayon_core::job::Job>::execute+0x92>
  4883d2:	call   105d30 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  4883d7:	ud2
  4883d9:	call   f2bd3 <core::panicking::panic_in_cleanup>
