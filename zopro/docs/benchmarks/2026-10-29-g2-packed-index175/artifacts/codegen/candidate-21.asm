
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000002912b0 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}>:
  2912b0:	push   %rbp
  2912b1:	push   %r15
  2912b3:	push   %r14
  2912b5:	push   %r13
  2912b7:	push   %r12
  2912b9:	push   %rbx
  2912ba:	sub    $0x128,%rsp
  2912c1:	vmovups (%rdi),%zmm0
  2912c7:	vmovups 0x18(%rdi),%zmm1
  2912d1:	mov    0x100(%rsi),%rcx
  2912d8:	lea    0x110(%rsi),%rax
  2912df:	mov    %rsi,%rbx
  2912e2:	mov    %rdi,%r15
  2912e5:	mov    %rax,0x70(%rsp)
  2912ea:	movq   $0x0,0x78(%rsp)
  2912f3:	mov    %rcx,0x80(%rsp)
  2912fb:	movb   $0x0,0x88(%rsp)
  291303:	vmovups %zmm0,(%rsp)
  29130a:	vmovups %zmm1,0x18(%rsp)
  291315:	movq   $0x0,0x58(%rsp)
  29131e:	mov    0x118(%rsi),%rax
  291325:	mov    0x108(%rax),%r14
  29132c:	mov    0x100(%rax),%r13
  291333:	mov    0x118(%rsi),%rcx
  29133a:	mov    0x108(%rcx),%r12
  291341:	mov    0x100(%rcx),%rax
  291348:	mov    0x128(%rsi),%rsi
  29134f:	mov    %r12,%rcx
  291352:	sub    %rax,%rcx
  291355:	cmp    %rsi,%rcx
  291358:	jge    291636 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x386>
  29135e:	mov    0x120(%rbx),%rax
  291365:	dec    %rsi
  291368:	sub    %r13,%r14
  29136b:	lea    0x7ee(%rip),%r13        # 291b60 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute>
  291372:	mov    %rsp,%rbp
  291375:	movabs $0x100000000,%rcx
  29137f:	and    %r12,%rsi
  291382:	inc    %r12
  291385:	shl    $0x4,%rsi
  291389:	mov    %r13,(%rax,%rsi,1)
  29138d:	mov    %rbp,0x8(%rax,%rsi,1)
  291392:	mov    0x118(%rbx),%rdx
  291399:	mov    %r12,0x108(%rdx)
  2913a0:	mov    0x110(%rbx),%rsi
  2913a7:	lea    0x1d8(%rsi),%rdi
  2913ae:	xchg   %ax,%ax
  2913b0:	mov    0x1f0(%rsi),%rax
  2913b7:	test   %rcx,%rax
  2913ba:	jne    2913da <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x12a>
  2913bc:	mov    %rax,%rdx
  2913bf:	or     %rcx,%rdx
  2913c2:	lock cmpxchg %rdx,0x1f0(%rsi)
  2913cb:	jne    2913b0 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x100>
  2913cd:	mov    %rdx,%rax
  2913d0:	and    $0xffff,%rax
  2913d6:	jne    2913e8 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x138>
  2913d8:	jmp    2913fc <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x14c>
  2913da:	mov    %rax,%rdx
  2913dd:	mov    %rdx,%rax
  2913e0:	and    $0xffff,%rax
  2913e6:	je     2913fc <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x14c>
  2913e8:	test   %r14,%r14
  2913eb:	jg     291624 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x374>
  2913f1:	shr    $0x10,%edx
  2913f4:	cmp    %eax,%edx
  2913f6:	je     291624 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x374>
  2913fc:	mov    0x98(%r15),%rax
  291403:	mov    0x90(%r15),%rdx
  29140a:	mov    (%rax),%rax
  29140d:	cmp    %rdx,%rax
  291410:	jae    291654 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x3a4>
  291416:	mov    0x68(%r15),%rdx
  29141a:	mov    0x58(%r15),%rdi
  29141e:	mov    0x60(%r15),%rsi
  291422:	mov    0x70(%r15),%rcx
  291426:	mov    0x78(%r15),%r8
  29142a:	mov    0x80(%r15),%r9
  291431:	inc    %rax
  291434:	push   0xa8(%r15)
  29143b:	push   0xa0(%r15)
  291442:	push   %rax
  291443:	push   0x88(%r15)
  29144a:	vzeroupper
  29144d:	call   28e7e0 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>>
  291452:	add    $0x20,%rsp
  291456:	cs nopw 0x0(%rax,%rax,1)
  291460:	mov    0x78(%rsp),%rax
  291465:	cmp    $0x3,%rax
  291469:	je     29149f <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x1ef>
  29146b:	mov    %rbx,%rdi
  29146e:	call   1c2f60 <<rayon_core::registry::WorkerThread>::take_local_job>
  291473:	test   %rax,%rax
  291476:	je     291490 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x1e0>
  291478:	mov    %rbp,%rcx
  29147b:	mov    %rax,%rsi
  29147e:	xor    %rdx,%rcx
  291481:	xor    %r13,%rsi
  291484:	or     %rcx,%rsi
  291487:	je     2914c7 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x217>
  291489:	mov    %rdx,%rdi
  29148c:	call   *%rax
  29148e:	jmp    291460 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x1b0>
  291490:	mov    0x78(%rsp),%rax
  291495:	cmp    $0x3,%rax
  291499:	jne    291685 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x3d5>
  29149f:	mov    0x58(%rsp),%rax
  2914a4:	cmp    $0x1,%rax
  2914a8:	je     291612 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x362>
  2914ae:	cmp    $0x2,%rax
  2914b2:	jne    29166d <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x3bd>
  2914b8:	mov    0x60(%rsp),%rdi
  2914bd:	mov    0x68(%rsp),%rsi
  2914c2:	call   104da0 <rayon_core::unwind::resume_unwinding>
  2914c7:	vmovups (%rsp),%zmm0
  2914ce:	vmovups 0x40(%rsp),%zmm1
  2914d6:	vmovups 0x50(%rsp),%zmm2
  2914e1:	vmovups %zmm0,0x90(%rsp)
  2914ec:	vmovups %zmm1,0xd0(%rsp)
  2914f7:	vmovups %zmm2,0xe0(%rsp)
  291502:	mov    0x90(%rsp),%rdi
  29150a:	test   %rdi,%rdi
  29150d:	je     291697 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x3e7>
  291513:	mov    0xd0(%rsp),%rax
  29151b:	mov    0xc8(%rsp),%rdx
  291523:	mov    (%rax),%rax
  291526:	mov    %rdx,%r10
  291529:	sub    %rax,%r10
  29152c:	jb     2916a8 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x3f8>
  291532:	mov    0xc0(%rsp),%r11
  29153a:	mov    0xa0(%rsp),%rdx
  291542:	mov    0x98(%rsp),%rsi
  29154a:	mov    0xa8(%rsp),%rcx
  291552:	mov    0xb0(%rsp),%r8
  29155a:	mov    0xb8(%rsp),%r9
  291562:	lea    (%r11,%rax,8),%rax
  291566:	push   0xe0(%rsp)
  29156d:	push   0xe0(%rsp)
  291574:	push   %r10
  291576:	push   %rax
  291577:	vzeroupper
  29157a:	call   28e7e0 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>>
  29157f:	add    $0x20,%rsp
  291583:	cmpl   $0x2,0xe8(%rsp)
  29158b:	jb     291612 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x362>
  291591:	mov    0xf8(%rsp),%r15
  291599:	mov    0xf0(%rsp),%rbx
  2915a1:	mov    (%r15),%rax
  2915a4:	test   %rax,%rax
  2915a7:	je     2915ae <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x2fe>
  2915a9:	mov    %rbx,%rdi
  2915ac:	call   *%rax
  2915ae:	mov    0x8(%r15),%rsi
  2915b2:	test   %rsi,%rsi
  2915b5:	je     291612 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x362>
  2915b7:	mov    0x10(%r15),%rax
  2915bb:	cmp    $0x20000,%rsi
  2915c2:	setae  %cl
  2915c5:	cmp    $0x1001,%rax
  2915cb:	setb   %dl
  2915ce:	test   %cl,%dl
  2915d0:	je     291607 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x357>
  2915d2:	blsr   %eax,%ecx
  2915d7:	jne    2916bf <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x40f>
  2915dd:	movabs $0x8000000000000000,%rcx
  2915e7:	sub    %rax,%rcx
  2915ea:	cmp    %rcx,%rsi
  2915ed:	jae    2916bf <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x40f>
  2915f3:	cmpb   $0x1,(%rbx,%rsi,1)
  2915f7:	jne    291609 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x359>
  2915f9:	inc    %rsi
  2915fc:	mov    %rbx,%rdi
  2915ff:	call   *0x259643(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  291605:	jmp    291612 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x362>
  291607:	jmp    291609 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x359>
  291609:	mov    %rbx,%rdi
  29160c:	call   *0x25963e(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  291612:	add    $0x128,%rsp
  291619:	pop    %rbx
  29161a:	pop    %r12
  29161c:	pop    %r13
  29161e:	pop    %r14
  291620:	pop    %r15
  291622:	pop    %rbp
  291623:	ret
  291624:	mov    $0x1,%esi
  291629:	vzeroupper
  29162c:	call   ff2f0 <<rayon_core::sleep::Sleep>::wake_any_threads>
  291631:	jmp    2913fc <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x14c>
  291636:	lea    0x118(%rbx),%rdi
  29163d:	add    %rsi,%rsi
  291640:	vzeroupper
  291643:	call   1c2a10 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::resize>
  291648:	mov    0x128(%rbx),%rsi
  29164f:	jmp    29135e <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0xae>
  291654:	lea    0x2559a5(%rip),%rcx        # 4e7000 <__frame_dummy_init_array_entry+0x51a0>
  29165b:	xor    %edi,%edi
  29165d:	mov    %rax,%rsi
  291660:	vzeroupper
  291663:	call   f0c00 <core::slice::index::slice_index_fail>
  291668:	jmp    29171a <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x46a>
  29166d:	lea    -0x272328(%rip),%rdi        # 1f34c <zmij::STATIC_DATA+0x4d4c>
  291674:	lea    0x256f7d(%rip),%rdx        # 4e85f8 <__frame_dummy_init_array_entry+0x6798>
  29167b:	mov    $0x28,%esi
  291680:	call   f0cd0 <core::panicking::panic>
  291685:	lea    0x78(%rsp),%rsi
  29168a:	mov    %rbx,%rdi
  29168d:	call   feab0 <<rayon_core::registry::WorkerThread>::wait_until_cold>
  291692:	jmp    29149f <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x1ef>
  291697:	lea    0x256d1a(%rip),%rdi        # 4e83b8 <__frame_dummy_init_array_entry+0x6558>
  29169e:	vzeroupper
  2916a1:	call   f1ff0 <core::option::unwrap_failed>
  2916a6:	jmp    29171a <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x46a>
  2916a8:	lea    0x255981(%rip),%rcx        # 4e7030 <__frame_dummy_init_array_entry+0x51d0>
  2916af:	mov    %rax,%rdi
  2916b2:	mov    %rdx,%rsi
  2916b5:	vzeroupper
  2916b8:	call   f0c00 <core::slice::index::slice_index_fail>
  2916bd:	jmp    29171a <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x46a>
  2916bf:	call   110970 <std::process::abort>
  2916c4:	jmp    29171c <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x46c>
  2916c6:	mov    0x8(%r15),%rsi
  2916ca:	mov    %rax,%r14
  2916cd:	test   %rsi,%rsi
  2916d0:	je     291727 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x477>
  2916d2:	mov    0x10(%r15),%rdx
  2916d6:	mov    %rbx,%rdi
  2916d9:	call   e9120 <__rustc::__rust_dealloc>
  2916de:	mov    %r14,%rdi
  2916e1:	call   4e0e10 <_Unwind_Resume@plt>
  2916e6:	jmp    29171c <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x46c>
  2916e8:	mov    %rax,%r14
  2916eb:	lea    0xe8(%rsp),%rdi
  2916f3:	call   2aab60 <core::ptr::drop_glue::<core::cell::UnsafeCell<rayon_core::job::JobResult<core::result::Result<(), &str>>>>>
  2916f8:	jmp    291727 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}+0x477>
  2916fa:	call   f2163 <core::panicking::panic_in_cleanup>
  2916ff:	mov    %rax,%rdi
  291702:	call   14485a <std::panicking::catch_unwind::cleanup>
  291707:	mov    %rdx,%rcx
  29170a:	lea    0x70(%rsp),%rsi
  29170f:	mov    %rbx,%rdi
  291712:	mov    %rax,%rdx
  291715:	call   105250 <rayon_core::join::join_recover_from_panic>
  29171a:	ud2
  29171c:	mov    %rax,%r14
  29171f:	mov    %rsp,%rdi
  291722:	call   47f210 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterMutProducer<[u64; 4]>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::g2_phi::hb_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterMutProducer<[u64; 4]>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::g2_phi::hb_raw::{closure#0}>>::{closure#1}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>>
  291727:	mov    %r14,%rdi
  29172a:	call   4e0e10 <_Unwind_Resume@plt>
  29172f:	call   f2163 <core::panicking::panic_in_cleanup>
