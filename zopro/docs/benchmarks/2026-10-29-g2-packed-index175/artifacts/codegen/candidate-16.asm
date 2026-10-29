
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000291850 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute>:
  291850:	push   %rbp
  291851:	push   %r15
  291853:	push   %r14
  291855:	push   %r13
  291857:	push   %r12
  291859:	push   %rbx
  29185a:	sub    $0xb8,%rsp
  291861:	vmovups (%rdi),%xmm0
  291865:	cmpq   $0x0,(%rdi)
  291869:	movq   $0x0,(%rdi)
  291870:	je     291a07 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x1b7>
  291876:	mov    %fs:0xffffffffffffffc0,%rsi
  29187f:	mov    %rdi,%rbx
  291882:	test   %rsi,%rsi
  291885:	je     2919ed <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x19d>
  29188b:	lea    0x10(%rbx),%rax
  29188f:	vmovaps %xmm0,(%rsp)
  291894:	vmovups (%rax),%zmm0
  29189a:	vmovups 0x40(%rax),%zmm1
  2918a1:	vmovups 0x60(%rax),%zmm2
  2918ab:	vmovups %zmm1,0x50(%rsp)
  2918b6:	vmovups %zmm0,0x10(%rsp)
  2918c1:	vmovups %zmm2,0x70(%rsp)
  2918cc:	mov    %rsp,%rdi
  2918cf:	vzeroupper
  2918d2:	call   2912b0 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}>
  2918d7:	mov    $0x1,%r13d
  2918dd:	cmpl   $0x2,0xb0(%rbx)
  2918e4:	jb     291965 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x115>
  2918e6:	mov    0xc0(%rbx),%rbp
  2918ed:	mov    0xb8(%rbx),%r12
  2918f4:	mov    0x0(%rbp),%rax
  2918f8:	test   %rax,%rax
  2918fb:	je     291902 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0xb2>
  2918fd:	mov    %r12,%rdi
  291900:	call   *%rax
  291902:	mov    0x8(%rbp),%rsi
  291906:	test   %rsi,%rsi
  291909:	je     291965 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x115>
  29190b:	mov    0x10(%rbp),%rax
  29190f:	cmp    $0x20000,%rsi
  291916:	setae  %cl
  291919:	cmp    $0x1001,%rax
  29191f:	setb   %dl
  291922:	test   %cl,%dl
  291924:	je     29195c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x10c>
  291926:	blsr   %eax,%ecx
  29192b:	jne    291a15 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x1c5>
  291931:	movabs $0x8000000000000000,%rcx
  29193b:	sub    %rax,%rcx
  29193e:	cmp    %rcx,%rsi
  291941:	jae    291a15 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x1c5>
  291947:	cmpb   $0x1,(%r12,%rsi,1)
  29194c:	jne    29195c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x10c>
  29194e:	inc    %rsi
  291951:	mov    %r12,%rdi
  291954:	call   *0x2592ee(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  29195a:	jmp    291965 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x115>
  29195c:	mov    %r12,%rdi
  29195f:	call   *0x2592eb(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  291965:	mov    %r13,0xb0(%rbx)
  29196c:	mov    %r15,0xb8(%rbx)
  291973:	mov    %r14,0xc0(%rbx)
  29197a:	mov    0xc8(%rbx),%rax
  291981:	movzbl 0xe0(%rbx),%ebp
  291988:	mov    (%rax),%rdi
  29198b:	cmp    $0x1,%bpl
  29198f:	jne    2919a3 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x153>
  291991:	lock incq (%rdi)
  291995:	jle    291a65 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x215>
  29199b:	mov    (%rax),%r14
  29199e:	mov    %r14,%rdi
  2919a1:	jmp    2919a3 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x153>
  2919a3:	mov    $0x3,%eax
  2919a8:	mov    0xd8(%rbx),%rsi
  2919af:	xchg   %rax,0xd0(%rbx)
  2919b6:	cmp    $0x2,%rax
  2919ba:	jne    2919c8 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x178>
  2919bc:	add    $0x1d8,%rdi
  2919c3:	call   ff7c0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  2919c8:	test   %bpl,%bpl
  2919cb:	je     2919db <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x18b>
  2919cd:	lock decq (%r14)
  2919d1:	jne    2919db <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x18b>
  2919d3:	mov    %r14,%rdi
  2919d6:	call   100580 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  2919db:	add    $0xb8,%rsp
  2919e2:	pop    %rbx
  2919e3:	pop    %r12
  2919e5:	pop    %r13
  2919e7:	pop    %r14
  2919e9:	pop    %r15
  2919eb:	pop    %rbp
  2919ec:	ret
  2919ed:	lea    -0x272090(%rip),%rdi        # 1f964 <zmij::STATIC_DATA+0x5364>
  2919f4:	lea    0x25480d(%rip),%rdx        # 4e6208 <__frame_dummy_init_array_entry+0x43a8>
  2919fb:	mov    $0x36,%esi
  291a00:	call   f0cd0 <core::panicking::panic>
  291a05:	jmp    291a65 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x215>
  291a07:	lea    0x257fb2(%rip),%rdi        # 4e99c0 <__frame_dummy_init_array_entry+0x7b60>
  291a0e:	call   f1ff0 <core::option::unwrap_failed>
  291a13:	jmp    291a65 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x215>
  291a15:	call   110970 <std::process::abort>
  291a1a:	mov    0x8(%rbp),%rsi
  291a1e:	test   %rsi,%rsi
  291a21:	je     291a2f <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x1df>
  291a23:	mov    0x10(%rbp),%rdx
  291a27:	mov    %r12,%rdi
  291a2a:	call   e9120 <__rustc::__rust_dealloc>
  291a2f:	mov    %r13,0xb0(%rbx)
  291a36:	mov    %r15,0xb8(%rbx)
  291a3d:	mov    %r14,0xc0(%rbx)
  291a44:	jmp    291a60 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x210>
  291a46:	test   %bpl,%bpl
  291a49:	je     291a60 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x210>
  291a4b:	lock decq (%r14)
  291a4f:	jne    291a60 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x210>
  291a51:	mov    %r14,%rdi
  291a54:	call   100580 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  291a59:	jmp    291a60 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x210>
  291a5b:	call   f2163 <core::panicking::panic_in_cleanup>
  291a60:	call   1052d0 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  291a65:	ud2
  291a67:	call   f2163 <core::panicking::panic_in_cleanup>
  291a6c:	mov    %rax,%rdi
  291a6f:	call   14485a <std::panicking::catch_unwind::cleanup>
  291a74:	mov    $0x2,%r13d
  291a7a:	mov    %rax,%r15
  291a7d:	mov    %rdx,%r14
  291a80:	cmpl   $0x2,0xb0(%rbx)
  291a87:	jae    2918e6 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x96>
  291a8d:	jmp    291965 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x115>
