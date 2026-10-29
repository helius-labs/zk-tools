
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001fd2f0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute>:
  1fd2f0:	push   %rbp
  1fd2f1:	push   %r15
  1fd2f3:	push   %r14
  1fd2f5:	push   %r13
  1fd2f7:	push   %r12
  1fd2f9:	push   %rbx
  1fd2fa:	sub    $0x68,%rsp
  1fd2fe:	mov    (%rdi),%rdx
  1fd301:	mov    0x8(%rdi),%rcx
  1fd305:	mov    0x10(%rdi),%rax
  1fd309:	movq   $0x0,(%rdi)
  1fd310:	test   %rdx,%rdx
  1fd313:	je     1fd48d <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x19d>
  1fd319:	mov    0x38(%rdi),%rsi
  1fd31d:	mov    %rdi,%rbx
  1fd320:	lea    0x50(%rsp),%r10
  1fd325:	mov    %rsi,0x60(%rsp)
  1fd32a:	vmovups 0x18(%rdi),%ymm0
  1fd32f:	vmovups %ymm0,0x40(%rsp)
  1fd335:	mov    %rdx,0x28(%rsp)
  1fd33a:	mov    %rcx,0x30(%rsp)
  1fd33f:	mov    %rax,0x38(%rsp)
  1fd344:	mov    (%rdx),%rsi
  1fd347:	mov    0x40(%rsp),%r9
  1fd34c:	mov    0x8(%rax),%r8
  1fd350:	sub    (%rcx),%rsi
  1fd353:	mov    (%rax),%rcx
  1fd356:	mov    $0x1,%edx
  1fd35b:	lea    0x10(%rsp),%rdi
  1fd360:	push   %r10
  1fd362:	push   0x50(%rsp)
  1fd366:	vzeroupper
  1fd369:	call   1f9da0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>>
  1fd36e:	add    $0x10,%rsp
  1fd372:	mov    0x10(%rsp),%rax
  1fd377:	mov    0x18(%rsp),%r15
  1fd37c:	mov    0x20(%rsp),%r13
  1fd381:	mov    $0x1,%ebp
  1fd386:	cmpl   $0x2,0x40(%rbx)
  1fd38a:	jb     1fd419 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x129>
  1fd390:	mov    0x50(%rbx),%r14
  1fd394:	mov    %rax,0x8(%rsp)
  1fd399:	mov    0x48(%rbx),%r12
  1fd39d:	mov    (%r14),%rax
  1fd3a0:	test   %rax,%rax
  1fd3a3:	je     1fd3aa <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0xba>
  1fd3a5:	mov    %r12,%rdi
  1fd3a8:	call   *%rax
  1fd3aa:	mov    0x8(%r14),%rsi
  1fd3ae:	mov    0x8(%rsp),%rax
  1fd3b3:	test   %rsi,%rsi
  1fd3b6:	je     1fd419 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x129>
  1fd3b8:	mov    0x10(%r14),%rax
  1fd3bc:	cmp    $0x20000,%rsi
  1fd3c3:	setae  %cl
  1fd3c6:	cmp    $0x1001,%rax
  1fd3cc:	setb   %dl
  1fd3cf:	test   %cl,%dl
  1fd3d1:	je     1fd409 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x119>
  1fd3d3:	blsr   %eax,%ecx
  1fd3d8:	jne    1fd49b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x1ab>
  1fd3de:	movabs $0x8000000000000000,%rcx
  1fd3e8:	sub    %rax,%rcx
  1fd3eb:	cmp    %rcx,%rsi
  1fd3ee:	jae    1fd49b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x1ab>
  1fd3f4:	cmpb   $0x1,(%r12,%rsi,1)
  1fd3f9:	jne    1fd40b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x11b>
  1fd3fb:	inc    %rsi
  1fd3fe:	mov    %r12,%rdi
  1fd401:	call   *0x2ed841(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1fd407:	jmp    1fd414 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x124>
  1fd409:	jmp    1fd40b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x11b>
  1fd40b:	mov    %r12,%rdi
  1fd40e:	call   *0x2ed83c(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1fd414:	mov    0x8(%rsp),%rax
  1fd419:	mov    %rbp,0x40(%rbx)
  1fd41d:	mov    %rax,0x48(%rbx)
  1fd421:	mov    %r15,0x50(%rbx)
  1fd425:	mov    %r13,0x58(%rbx)
  1fd429:	mov    0x60(%rbx),%rax
  1fd42d:	movzbl 0x78(%rbx),%ebp
  1fd431:	mov    (%rax),%rdi
  1fd434:	cmp    $0x1,%bpl
  1fd438:	jne    1fd44c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x15c>
  1fd43a:	lock incq (%rdi)
  1fd43e:	jle    1fd50a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x21a>
  1fd444:	mov    (%rax),%r14
  1fd447:	mov    %r14,%rdi
  1fd44a:	jmp    1fd44c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x15c>
  1fd44c:	mov    $0x3,%eax
  1fd451:	mov    0x70(%rbx),%rsi
  1fd455:	xchg   %rax,0x68(%rbx)
  1fd459:	cmp    $0x2,%rax
  1fd45d:	jne    1fd46b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x17b>
  1fd45f:	add    $0x1d8,%rdi
  1fd466:	call   ff7c0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  1fd46b:	test   %bpl,%bpl
  1fd46e:	je     1fd47e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x18e>
  1fd470:	lock decq (%r14)
  1fd474:	jne    1fd47e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x18e>
  1fd476:	mov    %r14,%rdi
  1fd479:	call   100580 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  1fd47e:	add    $0x68,%rsp
  1fd482:	pop    %rbx
  1fd483:	pop    %r12
  1fd485:	pop    %r13
  1fd487:	pop    %r14
  1fd489:	pop    %r15
  1fd48b:	pop    %rbp
  1fd48c:	ret
  1fd48d:	lea    0x2ec52c(%rip),%rdi        # 4e99c0 <__frame_dummy_init_array_entry+0x7b60>
  1fd494:	call   f1ff0 <core::option::unwrap_failed>
  1fd499:	jmp    1fd50a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x21a>
  1fd49b:	call   110970 <std::process::abort>
  1fd4a0:	mov    0x8(%r14),%rsi
  1fd4a4:	test   %rsi,%rsi
  1fd4a7:	je     1fd4b5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x1c5>
  1fd4a9:	mov    0x10(%r14),%rdx
  1fd4ad:	mov    %r12,%rdi
  1fd4b0:	call   e9120 <__rustc::__rust_dealloc>
  1fd4b5:	mov    0x8(%rsp),%rax
  1fd4ba:	mov    %rbp,0x40(%rbx)
  1fd4be:	mov    %rax,0x48(%rbx)
  1fd4c2:	mov    %r15,0x50(%rbx)
  1fd4c6:	mov    %r13,0x58(%rbx)
  1fd4ca:	jmp    1fd505 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x215>
  1fd4cc:	test   %bpl,%bpl
  1fd4cf:	je     1fd505 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x215>
  1fd4d1:	lock decq (%r14)
  1fd4d5:	jne    1fd505 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x215>
  1fd4d7:	mov    %r14,%rdi
  1fd4da:	call   100580 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  1fd4df:	jmp    1fd505 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x215>
  1fd4e1:	call   f2163 <core::panicking::panic_in_cleanup>
  1fd4e6:	mov    %rax,%rdi
  1fd4e9:	call   14485a <std::panicking::catch_unwind::cleanup>
  1fd4ee:	mov    $0x2,%ebp
  1fd4f3:	mov    %rdx,%r15
  1fd4f6:	cmpl   $0x2,0x40(%rbx)
  1fd4fa:	jae    1fd390 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0xa0>
  1fd500:	jmp    1fd419 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x129>
  1fd505:	call   1052d0 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  1fd50a:	ud2
  1fd50c:	call   f2163 <core::panicking::panic_in_cleanup>
