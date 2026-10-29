
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004bf570 <zopro_core::service_trace::snapshot>:
  4bf570:	push   %rbp
  4bf571:	push   %r15
  4bf573:	push   %r14
  4bf575:	push   %r13
  4bf577:	push   %r12
  4bf579:	push   %rbx
  4bf57a:	sub    $0xc8,%rsp
  4bf581:	xor    %eax,%eax
  4bf583:	xchg   %al,0x315ef(%rip)        # 4f0b78 <zopro_core::service_trace::ENABLED>
  4bf589:	mov    %rdi,%r14
  4bf58c:	mov    %fs:0xffffffffffffffb0,%rax
  4bf595:	test   %rax,%rax
  4bf598:	je     4bf5ae <zopro_core::service_trace::snapshot+0x3e>
  4bf59a:	add    $0x110,%rax
  4bf5a0:	mov    (%rax),%rcx
  4bf5a3:	lock incq (%rcx)
  4bf5a7:	jg     4bf5c0 <zopro_core::service_trace::snapshot+0x50>
  4bf5a9:	jmp    4bfc8c <zopro_core::service_trace::snapshot+0x71c>
  4bf5ae:	call   108b20 <rayon_core::registry::global_registry>
  4bf5b3:	mov    (%rax),%rcx
  4bf5b6:	lock incq (%rcx)
  4bf5ba:	jle    4bfc8c <zopro_core::service_trace::snapshot+0x71c>
  4bf5c0:	mov    (%rax),%rbx
  4bf5c3:	lea    0xf(%rsp),%rax
  4bf5c8:	mov    %fs:0xffffffffffffffb0,%rdx
  4bf5d1:	mov    %rax,0x60(%rsp)
  4bf5d6:	mov    0x208(%rbx),%rbp
  4bf5dd:	mov    %rdx,0xa8(%rsp)
  4bf5e5:	test   %rdx,%rdx
  4bf5e8:	je     4bf60b <zopro_core::service_trace::snapshot+0x9b>
  4bf5ea:	mov    0x110(%rdx),%rax
  4bf5f1:	lock incq (%rax)
  4bf5f5:	jle    4bfc8c <zopro_core::service_trace::snapshot+0x71c>
  4bf5fb:	mov    0x100(%rdx),%rcx
  4bf602:	mov    0x110(%rdx),%rax
  4bf609:	jmp    4bf60f <zopro_core::service_trace::snapshot+0x9f>
  4bf60b:	xor    %eax,%eax
  4bf60d:	xor    %ecx,%ecx
  4bf60f:	mov    %rbp,0x68(%rsp)
  4bf614:	mov    %rax,0x70(%rsp)
  4bf619:	mov    %rbp,%rax
  4bf61c:	shl    $0x4,%rax
  4bf620:	movabs $0x8000000000000000,%r12
  4bf62a:	movq   $0x0,0x78(%rsp)
  4bf633:	mov    %rcx,0x80(%rsp)
  4bf63b:	mov    %rbx,(%rsp)
  4bf63f:	lea    (%rax,%rax,2),%r13
  4bf643:	test   %rbp,%rbp
  4bf646:	je     4bf670 <zopro_core::service_trace::snapshot+0x100>
  4bf648:	mov    $0x8,%edi
  4bf64d:	mov    %r13,%rsi
  4bf650:	xor    %edx,%edx
  4bf652:	call   4c0830 <<zopro_core::alloc::linux::Allocator>::allocate>
  4bf657:	test   %rax,%rax
  4bf65a:	je     4bfc7f <zopro_core::service_trace::snapshot+0x70f>
  4bf660:	mov    %rax,%r15
  4bf663:	cmp    $0x8,%rbp
  4bf667:	jae    4bf67b <zopro_core::service_trace::snapshot+0x10b>
  4bf669:	xor    %eax,%eax
  4bf66b:	jmp    4bf70a <zopro_core::service_trace::snapshot+0x19a>
  4bf670:	mov    $0x8,%r15d
  4bf676:	jmp    4bf744 <zopro_core::service_trace::snapshot+0x1d4>
  4bf67b:	vmovdqa64 -0x491f45(%rip),%zmm2        # 2d740 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5a10>
  4bf685:	vpbroadcastq -0x498a67(%rip),%zmm3        # 26c28 <GCC_except_table3180+0x1e8>
  4bf68f:	vpbroadcastq -0x498ad9(%rip),%zmm4        # 26bc0 <GCC_except_table3180+0x180>
  4bf699:	vpbroadcastq -0x498c1b(%rip),%zmm5        # 26a88 <GCC_except_table3180+0x48>
  4bf6a3:	lea    0x68(%rsp),%rcx
  4bf6a8:	movabs $0x3fffffffffffff8,%rax
  4bf6b2:	vpbroadcastq %rcx,%zmm0
  4bf6b8:	lea    0x60(%rsp),%rcx
  4bf6bd:	and    %rbp,%rax
  4bf6c0:	vpbroadcastq %rcx,%zmm1
  4bf6c6:	mov    %rax,%rcx
  4bf6c9:	nopl   0x0(%rax)
  4bf6d0:	vpmullq %zmm3,%zmm2,%zmm6
  4bf6d6:	kxnorb %k0,%k0,%k1
  4bf6da:	vpaddq %zmm5,%zmm2,%zmm2
  4bf6e0:	add    $0xfffffffffffffff8,%rcx
  4bf6e4:	vpscatterqq %zmm4,(%r15,%zmm6,1){%k1}
  4bf6eb:	kxnorb %k0,%k0,%k1
  4bf6ef:	vpscatterqq %zmm0,0x20(%r15,%zmm6,1){%k1}
  4bf6f7:	kxnorb %k0,%k0,%k1
  4bf6fb:	vpscatterqq %zmm1,0x28(%r15,%zmm6,1){%k1}
  4bf703:	jne    4bf6d0 <zopro_core::service_trace::snapshot+0x160>
  4bf705:	cmp    %rax,%rbp
  4bf708:	je     4bf744 <zopro_core::service_trace::snapshot+0x1d4>
  4bf70a:	lea    (%rax,%rax,2),%rcx
  4bf70e:	mov    %rbp,%rdx
  4bf711:	sub    %rax,%rdx
  4bf714:	lea    0x68(%rsp),%rax
  4bf719:	lea    0x60(%rsp),%rsi
  4bf71e:	shl    $0x4,%rcx
  4bf722:	lea    0x28(%rcx,%r15,1),%rcx
  4bf727:	nopw   0x0(%rax,%rax,1)
  4bf730:	mov    %r12,-0x28(%rcx)
  4bf734:	mov    %rax,-0x8(%rcx)
  4bf738:	mov    %rsi,(%rcx)
  4bf73b:	add    $0x30,%rcx
  4bf73f:	dec    %rdx
  4bf742:	jne    4bf730 <zopro_core::service_trace::snapshot+0x1c0>
  4bf744:	mov    %rbp,0xb0(%rsp)
  4bf74c:	mov    %r15,0xb8(%rsp)
  4bf754:	mov    %rbp,0xc0(%rsp)
  4bf75c:	mov    0x208(%rbx),%rax
  4bf763:	mov    %rbp,0x18(%rsp)
  4bf768:	mov    %rax,0x88(%rsp)
  4bf770:	cmp    %rbp,%rax
  4bf773:	jne    4bfb53 <zopro_core::service_trace::snapshot+0x5e3>
  4bf779:	lea    0x180(%rbx),%rax
  4bf780:	mov    $0x1,%ecx
  4bf785:	mov    %rax,0x58(%rsp)
  4bf78a:	xor    %eax,%eax
  4bf78c:	lock cmpxchg %ecx,0x180(%rbx)
  4bf794:	jne    4bfb76 <zopro_core::service_trace::snapshot+0x606>
  4bf79a:	mov    0x313a7(%rip),%rax        # 4f0b48 <std::panicking::panic_count::GLOBAL_PANIC_COUNT>
  4bf7a1:	shl    $1,%rax
  4bf7a4:	test   %rax,%rax
  4bf7a7:	jne    4bfb98 <zopro_core::service_trace::snapshot+0x628>
  4bf7ad:	movl   $0x0,0x8(%rsp)
  4bf7b5:	movzbl 0x184(%rbx),%eax
  4bf7bc:	test   %al,%al
  4bf7be:	jne    4bfbb5 <zopro_core::service_trace::snapshot+0x645>
  4bf7c4:	mov    0x198(%rbx),%rax
  4bf7cb:	mov    %rbp,0x18(%rsp)
  4bf7d0:	mov    %rax,0x88(%rsp)
  4bf7d8:	cmp    %rbp,%rax
  4bf7db:	jne    4bfbf5 <zopro_core::service_trace::snapshot+0x685>
  4bf7e1:	mov    %r13,0x38(%rsp)
  4bf7e6:	mov    %r14,0xa0(%rsp)
  4bf7ee:	mov    %rbp,0x10(%rsp)
  4bf7f3:	test   %rbp,%rbp
  4bf7f6:	je     4bf890 <zopro_core::service_trace::snapshot+0x320>
  4bf7fc:	mov    (%rsp),%rax
  4bf800:	mov    0x10(%rsp),%rbx
  4bf805:	lea    0x634(%rip),%rdx        # 4bfe40 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute>
  4bf80c:	mov    %r15,%rbp
  4bf80f:	mov    0x190(%rax),%r13
  4bf816:	cs nopw 0x0(%rax,%rax,1)
  4bf820:	mov    0x0(%r13),%rax
  4bf824:	mov    0x108(%rax),%r14
  4bf82b:	mov    0x100(%rax),%rax
  4bf832:	mov    0x10(%r13),%rsi
  4bf836:	mov    %r14,%rcx
  4bf839:	sub    %rax,%rcx
  4bf83c:	cmp    %rsi,%rcx
  4bf83f:	jge    4bf875 <zopro_core::service_trace::snapshot+0x305>
  4bf841:	mov    0x8(%r13),%rax
  4bf845:	dec    %rsi
  4bf848:	and    %r14,%rsi
  4bf84b:	inc    %r14
  4bf84e:	shl    $0x4,%rsi
  4bf852:	mov    %rdx,(%rax,%rsi,1)
  4bf856:	mov    %rbp,0x8(%rax,%rsi,1)
  4bf85b:	mov    0x0(%r13),%rcx
  4bf85f:	add    $0x30,%rbp
  4bf863:	add    $0x20,%r13
  4bf867:	dec    %rbx
  4bf86a:	mov    %r14,0x108(%rcx)
  4bf871:	jne    4bf820 <zopro_core::service_trace::snapshot+0x2b0>
  4bf873:	jmp    4bf890 <zopro_core::service_trace::snapshot+0x320>
  4bf875:	add    %rsi,%rsi
  4bf878:	mov    %r13,%rdi
  4bf87b:	vzeroupper
  4bf87e:	call   1c6c30 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::resize>
  4bf883:	mov    0x10(%r13),%rsi
  4bf887:	lea    0x5b2(%rip),%rdx        # 4bfe40 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute>
  4bf88e:	jmp    4bf841 <zopro_core::service_trace::snapshot+0x2d1>
  4bf890:	cmpb   $0x0,0x8(%rsp)
  4bf895:	mov    (%rsp),%rbx
  4bf899:	jne    4bf8ae <zopro_core::service_trace::snapshot+0x33e>
  4bf89b:	mov    0x312a6(%rip),%rax        # 4f0b48 <std::panicking::panic_count::GLOBAL_PANIC_COUNT>
  4bf8a2:	shl    $1,%rax
  4bf8a5:	test   %rax,%rax
  4bf8a8:	jne    4bfc63 <zopro_core::service_trace::snapshot+0x6f3>
  4bf8ae:	mov    0x58(%rsp),%rsi
  4bf8b3:	xor    %eax,%eax
  4bf8b5:	xchg   %eax,(%rsi)
  4bf8b7:	mov    0x38(%rsp),%rbp
  4bf8bc:	cmp    $0x2,%eax
  4bf8bf:	je     4bfc15 <zopro_core::service_trace::snapshot+0x6a5>
  4bf8c5:	mov    0x208(%rbx),%rbx
  4bf8cc:	test   %rbx,%rbx
  4bf8cf:	je     4bf8f6 <zopro_core::service_trace::snapshot+0x386>
  4bf8d1:	mov    (%rsp),%rax
  4bf8d5:	xor    %r13d,%r13d
  4bf8d8:	lea    0x1d8(%rax),%r14
  4bf8df:	nop
  4bf8e0:	mov    %r14,%rdi
  4bf8e3:	mov    %r13,%rsi
  4bf8e6:	vzeroupper
  4bf8e9:	call   1038b0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  4bf8ee:	inc    %r13
  4bf8f1:	cmp    %r13,%rbx
  4bf8f4:	jne    4bf8e0 <zopro_core::service_trace::snapshot+0x370>
  4bf8f6:	mov    0xa8(%rsp),%rsi
  4bf8fe:	mov    (%rsp),%rbx
  4bf902:	lea    0x68(%rsp),%rdi
  4bf907:	vzeroupper
  4bf90a:	call   109340 <<rayon_core::latch::CountLatch>::wait>
  4bf90f:	mov    0x10(%rsp),%rax
  4bf914:	lea    (%r15,%rbp,1),%r13
  4bf918:	mov    %r15,0x18(%rsp)
  4bf91d:	mov    %rax,0x28(%rsp)
  4bf922:	mov    %r13,0x30(%rsp)
  4bf927:	test   %rax,%rax
  4bf92a:	je     4bf99b <zopro_core::service_trace::snapshot+0x42b>
  4bf92c:	mov    $0x1,%ecx
  4bf931:	mov    %r15,%rbp
  4bf934:	mov    %r15,%rdx
  4bf937:	mov    (%rdx),%r8
  4bf93a:	mov    0x8(%rdx),%rax
  4bf93e:	mov    0x10(%rdx),%rsi
  4bf942:	lea    0x30(%rdx),%rdi
  4bf946:	mov    %r8,%r9
  4bf949:	xor    %r12,%r9
  4bf94c:	test   %r8,%r8
  4bf94f:	cmovns %rcx,%r9
  4bf953:	cmp    $0x1,%r9
  4bf957:	jne    4bf97f <zopro_core::service_trace::snapshot+0x40f>
  4bf959:	mov    0x18(%rdx),%r9
  4bf95d:	mov    %r8,0x0(%rbp)
  4bf961:	mov    %rax,0x8(%rbp)
  4bf965:	mov    %rsi,0x10(%rbp)
  4bf969:	add    $0x30,%rdx
  4bf96d:	mov    %r9,0x18(%rbp)
  4bf971:	add    $0x20,%rbp
  4bf975:	cmp    %r13,%rdx
  4bf978:	jne    4bf937 <zopro_core::service_trace::snapshot+0x3c7>
  4bf97a:	mov    %r13,%rax
  4bf97d:	jmp    4bf9a4 <zopro_core::service_trace::snapshot+0x434>
  4bf97f:	cmp    $0x2,%r9
  4bf983:	jne    4bfc44 <zopro_core::service_trace::snapshot+0x6d4>
  4bf989:	mov    %rdi,0x20(%rsp)
  4bf98e:	mov    %rax,%rdi
  4bf991:	call   108e90 <rayon_core::unwind::resume_unwinding>
  4bf996:	jmp    4bfc8c <zopro_core::service_trace::snapshot+0x71c>
  4bf99b:	mov    %r15,%rax
  4bf99e:	mov    %r15,%rbp
  4bf9a1:	mov    %r15,%rdi
  4bf9a4:	vmovdqa -0x4985cc(%rip),%ymm0        # 273e0 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x240>
  4bf9ac:	mov    %r13,%rdx
  4bf9af:	sub    %rax,%rdx
  4bf9b2:	mov    0x10(%rsp),%rcx
  4bf9b7:	movabs $0xaaaaaaaaaaaaaaab,%rbx
  4bf9c1:	sub    %r15,%rbp
  4bf9c4:	mov    %r15,0x88(%rsp)
  4bf9cc:	shr    $0x4,%rdx
  4bf9d0:	shr    $0x5,%rbp
  4bf9d4:	imul   %rdx,%rbx
  4bf9d8:	mov    %rbp,0x90(%rsp)
  4bf9e0:	inc    %rbx
  4bf9e3:	mov    %rcx,0x98(%rsp)
  4bf9eb:	vmovdqu %ymm0,0x18(%rsp)
  4bf9f1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4bfa00:	cmp    $0x1,%rbx
  4bfa04:	je     4bfa1a <zopro_core::service_trace::snapshot+0x4aa>
  4bfa06:	lea    0x30(%rdi),%r14
  4bfa0a:	dec    %rbx
  4bfa0d:	vzeroupper
  4bfa10:	call   4c0090 <core::ptr::drop_glue::<rayon_core::job::JobResult<(alloc::vec::Vec<[u64; 10]>, u64)>>>
  4bfa15:	mov    %r14,%rdi
  4bfa18:	jmp    4bfa00 <zopro_core::service_trace::snapshot+0x490>
  4bfa1a:	mov    0x38(%rsp),%rdx
  4bfa1f:	test   $0x10,%dl
  4bfa22:	jne    4bfa2a <zopro_core::service_trace::snapshot+0x4ba>
  4bfa24:	mov    (%rsp),%rbx
  4bfa28:	jmp    4bfa9b <zopro_core::service_trace::snapshot+0x52b>
  4bfa2a:	mov    (%rsp),%rbx
  4bfa2e:	add    $0xffffffffffffffe0,%r12
  4bfa32:	and    %rdx,%r12
  4bfa35:	je     4bfa64 <zopro_core::service_trace::snapshot+0x4f4>
  4bfa37:	mov    $0x8,%esi
  4bfa3c:	mov    %r15,%rdi
  4bfa3f:	mov    %r12,%rcx
  4bfa42:	vzeroupper
  4bfa45:	call   4c0ab0 <<zopro_core::alloc::linux::Allocator as core::alloc::global::GlobalAlloc>::realloc>
  4bfa4a:	mov    %rax,%r15
  4bfa4d:	test   %rax,%rax
  4bfa50:	jne    4bfa9b <zopro_core::service_trace::snapshot+0x52b>
  4bfa52:	mov    $0x8,%edi
  4bfa57:	mov    %r12,%rsi
  4bfa5a:	call   f1ccd <alloc::alloc::handle_alloc_error>
  4bfa5f:	jmp    4bfc8c <zopro_core::service_trace::snapshot+0x71c>
  4bfa64:	cmpq   $0xaaa,0x10(%rsp)
  4bfa6d:	jbe    4bfa89 <zopro_core::service_trace::snapshot+0x519>
  4bfa6f:	cmpb   $0x1,0x0(%r13)
  4bfa74:	jne    4bfa89 <zopro_core::service_trace::snapshot+0x519>
  4bfa76:	mov    $0x11,%esi
  4bfa7b:	mov    %r15,%rdi
  4bfa7e:	vzeroupper
  4bfa81:	call   *0x302c1(%rip)        # 4efd48 <munmap@GLIBC_2.2.5>
  4bfa87:	jmp    4bfa95 <zopro_core::service_trace::snapshot+0x525>
  4bfa89:	mov    %r15,%rdi
  4bfa8c:	vzeroupper
  4bfa8f:	call   *0x302bb(%rip)        # 4efd50 <free@GLIBC_2.2.5>
  4bfa95:	mov    $0x8,%r15d
  4bfa9b:	mov    0x38(%rsp),%rax
  4bfaa0:	shr    $0x5,%rax
  4bfaa4:	mov    %rax,0x40(%rsp)
  4bfaa9:	mov    %r15,0x48(%rsp)
  4bfaae:	mov    %rbp,0x50(%rsp)
  4bfab3:	lea    0x18(%rsp),%rdi
  4bfab8:	vzeroupper
  4bfabb:	call   4bff90 <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>>
  4bfac0:	mov    0x70(%rsp),%rax
  4bfac5:	mov    0xa0(%rsp),%r14
  4bfacd:	test   %rax,%rax
  4bfad0:	je     4bfae2 <zopro_core::service_trace::snapshot+0x572>
  4bfad2:	lock decq (%rax)
  4bfad6:	jne    4bfae2 <zopro_core::service_trace::snapshot+0x572>
  4bfad8:	mov    0x70(%rsp),%rdi
  4bfadd:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4bfae2:	lock decq (%rbx)
  4bfae6:	jne    4bfaf0 <zopro_core::service_trace::snapshot+0x580>
  4bfae8:	mov    %rbx,%rdi
  4bfaeb:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4bfaf0:	lea    0x18(%rsp),%rdi
  4bfaf5:	call   4c03c0 <zopro_core::service_trace::snapshot::{closure#0}>
  4bfafa:	mov    0x50(%rsp),%rbx
  4bfaff:	cmp    0x40(%rsp),%rbx
  4bfb04:	jne    4bfb10 <zopro_core::service_trace::snapshot+0x5a0>
  4bfb06:	lea    0x40(%rsp),%rdi
  4bfb0b:	call   3fe0f0 <<alloc::raw_vec::RawVec<[usize; 4]>>::grow_one>
  4bfb10:	vmovups 0x18(%rsp),%ymm0
  4bfb16:	mov    0x48(%rsp),%rax
  4bfb1b:	mov    %rbx,%rcx
  4bfb1e:	shl    $0x5,%rcx
  4bfb22:	inc    %rbx
  4bfb25:	vmovups %ymm0,(%rax,%rcx,1)
  4bfb2a:	mov    %rbx,0x50(%rsp)
  4bfb2f:	mov    %rbx,0x10(%r14)
  4bfb33:	vmovups 0x40(%rsp),%xmm0
  4bfb39:	vmovups %xmm0,(%r14)
  4bfb3e:	add    $0xc8,%rsp
  4bfb45:	pop    %rbx
  4bfb46:	pop    %r12
  4bfb48:	pop    %r13
  4bfb4a:	pop    %r14
  4bfb4c:	pop    %r15
  4bfb4e:	pop    %rbp
  4bfb4f:	vzeroupper
  4bfb52:	ret
  4bfb53:	lea    0x29aa6(%rip),%r8        # 4e9600 <__frame_dummy_init_array_entry+0x2978>
  4bfb5a:	lea    0x88(%rsp),%rdi
  4bfb62:	lea    0x18(%rsp),%rsi
  4bfb67:	xor    %edx,%edx
  4bfb69:	vzeroupper
  4bfb6c:	call   f3d52 <core::panicking::assert_failed::<usize, usize>>
  4bfb71:	jmp    4bfc8c <zopro_core::service_trace::snapshot+0x71c>
  4bfb76:	lea    0x180(%rbx),%rdi
  4bfb7d:	vzeroupper
  4bfb80:	call   115050 <<std::sys::sync::mutex::futex::Mutex>::lock_contended>
  4bfb85:	mov    0x30fbc(%rip),%rax        # 4f0b48 <std::panicking::panic_count::GLOBAL_PANIC_COUNT>
  4bfb8c:	shl    $1,%rax
  4bfb8f:	test   %rax,%rax
  4bfb92:	je     4bf7ad <zopro_core::service_trace::snapshot+0x23d>
  4bfb98:	vzeroupper
  4bfb9b:	call   114f10 <std::panicking::panic_count::is_zero_slow_path>
  4bfba0:	xor    $0x1,%al
  4bfba2:	mov    %eax,0x8(%rsp)
  4bfba6:	movzbl 0x184(%rbx),%eax
  4bfbad:	test   %al,%al
  4bfbaf:	je     4bf7c4 <zopro_core::service_trace::snapshot+0x254>
  4bfbb5:	lea    0x180(%rbx),%rax
  4bfbbc:	mov    %rax,0x18(%rsp)
  4bfbc1:	mov    0x8(%rsp),%eax
  4bfbc5:	mov    %al,0x20(%rsp)
  4bfbc9:	lea    -0x41987f(%rip),%rdi        # a6351 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x5b9>
  4bfbd0:	lea    0x2cf99(%rip),%rcx        # 4ecb70 <__frame_dummy_init_array_entry+0x5ee8>
  4bfbd7:	lea    0x29a3a(%rip),%r8        # 4e9618 <__frame_dummy_init_array_entry+0x2990>
  4bfbde:	lea    0x18(%rsp),%rdx
  4bfbe3:	mov    $0x2b,%esi
  4bfbe8:	vzeroupper
  4bfbeb:	call   f5df0 <core::result::unwrap_failed>
  4bfbf0:	jmp    4bfc8c <zopro_core::service_trace::snapshot+0x71c>
  4bfbf5:	lea    0x29a34(%rip),%r8        # 4e9630 <__frame_dummy_init_array_entry+0x29a8>
  4bfbfc:	lea    0x88(%rsp),%rdi
  4bfc04:	lea    0x18(%rsp),%rsi
  4bfc09:	xor    %edx,%edx
  4bfc0b:	vzeroupper
  4bfc0e:	call   f3d52 <core::panicking::assert_failed::<usize, usize>>
  4bfc13:	jmp    4bfc8c <zopro_core::service_trace::snapshot+0x71c>
  4bfc15:	mov    $0xca,%edi
  4bfc1a:	mov    $0x81,%edx
  4bfc1f:	mov    $0x1,%ecx
  4bfc24:	xor    %eax,%eax
  4bfc26:	vzeroupper
  4bfc29:	call   *0x301a1(%rip)        # 4efdd0 <syscall@GLIBC_2.2.5>
  4bfc2f:	mov    0x208(%rbx),%rbx
  4bfc36:	test   %rbx,%rbx
  4bfc39:	jne    4bf8d1 <zopro_core::service_trace::snapshot+0x361>
  4bfc3f:	jmp    4bf8f6 <zopro_core::service_trace::snapshot+0x386>
  4bfc44:	mov    %rdi,0x20(%rsp)
  4bfc49:	lea    -0x484fda(%rip),%rdi        # 3ac76 <zmij::STATIC_DATA+0x4d76>
  4bfc50:	lea    0x2da61(%rip),%rdx        # 4ed6b8 <__frame_dummy_init_array_entry+0x6a30>
  4bfc57:	mov    $0x28,%esi
  4bfc5c:	call   f49d0 <core::panicking::panic>
  4bfc61:	jmp    4bfc8c <zopro_core::service_trace::snapshot+0x71c>
  4bfc63:	vzeroupper
  4bfc66:	call   114f10 <std::panicking::panic_count::is_zero_slow_path>
  4bfc6b:	test   %al,%al
  4bfc6d:	jne    4bf8ae <zopro_core::service_trace::snapshot+0x33e>
  4bfc73:	movb   $0x1,0x184(%rbx)
  4bfc7a:	jmp    4bf8ae <zopro_core::service_trace::snapshot+0x33e>
  4bfc7f:	mov    $0x8,%edi
  4bfc84:	mov    %r13,%rsi
  4bfc87:	call   f1bb1 <alloc::raw_vec::handle_error>
  4bfc8c:	ud2
  4bfc8e:	mov    (%rsp),%rbx
  4bfc92:	mov    0x58(%rsp),%rdi
  4bfc97:	jmp    4bfd44 <zopro_core::service_trace::snapshot+0x7d4>
  4bfc9c:	mov    %rax,%r13
  4bfc9f:	jmp    4bfda0 <zopro_core::service_trace::snapshot+0x830>
  4bfca4:	mov    %rax,%r13
  4bfca7:	mov    0x18(%rsp),%rax
  4bfcac:	test   %rax,%rax
  4bfcaf:	je     4bfcdd <zopro_core::service_trace::snapshot+0x76d>
  4bfcb1:	mov    0x20(%rsp),%rdi
  4bfcb6:	shl    $0x4,%rax
  4bfcba:	mov    $0x8,%edx
  4bfcbf:	lea    (%rax,%rax,4),%rsi
  4bfcc3:	call   eb3e0 <__rustc::__rust_dealloc>
  4bfcc8:	lea    0x40(%rsp),%rdi
  4bfccd:	call   4c0550 <core::ptr::drop_glue::<alloc::vec::Vec<(alloc::vec::Vec<[u64; 10]>, u64)>>>
  4bfcd2:	mov    %r13,%rdi
  4bfcd5:	call   4e5c10 <_Unwind_Resume@plt>
  4bfcda:	mov    %rax,%r13
  4bfcdd:	lea    0x40(%rsp),%rdi
  4bfce2:	call   4c0550 <core::ptr::drop_glue::<alloc::vec::Vec<(alloc::vec::Vec<[u64; 10]>, u64)>>>
  4bfce7:	mov    %r13,%rdi
  4bfcea:	call   4e5c10 <_Unwind_Resume@plt>
  4bfcef:	mov    %rax,%r13
  4bfcf2:	jmp    4bfd86 <zopro_core::service_trace::snapshot+0x816>
  4bfcf7:	mov    (%rsp),%rbx
  4bfcfb:	mov    %rax,%r13
  4bfcfe:	jmp    4bfd86 <zopro_core::service_trace::snapshot+0x816>
  4bfd03:	mov    %rax,%r13
  4bfd06:	dec    %rbx
  4bfd09:	je     4bfd2a <zopro_core::service_trace::snapshot+0x7ba>
  4bfd0b:	mov    %r14,%rdi
  4bfd0e:	lea    0x30(%r14),%r15
  4bfd12:	call   4c0090 <core::ptr::drop_glue::<rayon_core::job::JobResult<(alloc::vec::Vec<[u64; 10]>, u64)>>>
  4bfd17:	mov    %r15,%r14
  4bfd1a:	jmp    4bfd06 <zopro_core::service_trace::snapshot+0x796>
  4bfd1c:	call   f5eb3 <core::panicking::panic_in_cleanup>
  4bfd21:	mov    (%rsp),%rbx
  4bfd25:	jmp    4bfd62 <zopro_core::service_trace::snapshot+0x7f2>
  4bfd27:	mov    %rax,%r13
  4bfd2a:	lea    0x88(%rsp),%rdi
  4bfd32:	call   4c01b0 <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>>
  4bfd37:	mov    (%rsp),%rbx
  4bfd3b:	jmp    4bfd7c <zopro_core::service_trace::snapshot+0x80c>
  4bfd3d:	lea    0x180(%rbx),%rdi
  4bfd44:	mov    %rax,%r13
  4bfd47:	movzbl 0x8(%rsp),%esi
  4bfd4c:	call   4bee30 <core::ptr::drop_glue::<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>>>>
  4bfd51:	jmp    4bfd65 <zopro_core::service_trace::snapshot+0x7f5>
  4bfd53:	lea    0x18(%rsp),%rdi
  4bfd58:	mov    %rax,%r13
  4bfd5b:	call   4bede0 <core::ptr::drop_glue::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>>>>>
  4bfd60:	jmp    4bfd65 <zopro_core::service_trace::snapshot+0x7f5>
  4bfd62:	mov    %rax,%r13
  4bfd65:	lea    0xb0(%rsp),%rdi
  4bfd6d:	call   4c02c0 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>>
  4bfd72:	jmp    4bfd86 <zopro_core::service_trace::snapshot+0x816>
  4bfd74:	sub    %r15,%rbp
  4bfd77:	mov    %rax,%r13
  4bfd7a:	jne    4bfdb6 <zopro_core::service_trace::snapshot+0x846>
  4bfd7c:	lea    0x18(%rsp),%rdi
  4bfd81:	call   4bff90 <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>>
  4bfd86:	mov    0x70(%rsp),%rax
  4bfd8b:	test   %rax,%rax
  4bfd8e:	je     4bfda0 <zopro_core::service_trace::snapshot+0x830>
  4bfd90:	lock decq (%rax)
  4bfd94:	jne    4bfda0 <zopro_core::service_trace::snapshot+0x830>
  4bfd96:	mov    0x70(%rsp),%rdi
  4bfd9b:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4bfda0:	lock decq (%rbx)
  4bfda4:	jne    4bfdae <zopro_core::service_trace::snapshot+0x83e>
  4bfda6:	mov    %rbx,%rdi
  4bfda9:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4bfdae:	mov    %r13,%rdi
  4bfdb1:	call   4e5c10 <_Unwind_Resume@plt>
  4bfdb6:	mov    0x2ff8b(%rip),%rbx        # 4efd48 <munmap@GLIBC_2.2.5>
  4bfdbd:	mov    0x2ff8c(%rip),%r14        # 4efd50 <free@GLIBC_2.2.5>
  4bfdc4:	shr    $0x5,%rbp
  4bfdc8:	add    $0x8,%r15
  4bfdcc:	add    $0xfffffffffffffff9,%r12
  4bfdd0:	jmp    4bfdf0 <zopro_core::service_trace::snapshot+0x880>
  4bfdd2:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4bfde0:	call   *%r14
  4bfde3:	add    $0x20,%r15
  4bfde7:	dec    %rbp
  4bfdea:	je     4bfd37 <zopro_core::service_trace::snapshot+0x7c7>
  4bfdf0:	mov    -0x8(%r15),%rax
  4bfdf4:	test   %rax,%rax
  4bfdf7:	je     4bfde3 <zopro_core::service_trace::snapshot+0x873>
  4bfdf9:	mov    (%r15),%rdi
  4bfdfc:	cmp    $0x666,%rax
  4bfe02:	jbe    4bfde0 <zopro_core::service_trace::snapshot+0x870>
  4bfe04:	shl    $0x4,%rax
  4bfe08:	lea    (%rax,%rax,4),%rax
  4bfe0c:	mov    %rax,%rsi
  4bfe0f:	or     $0x1,%rsi
  4bfe13:	cmp    %r12,%rsi
  4bfe16:	jae    4bfe22 <zopro_core::service_trace::snapshot+0x8b2>
  4bfe18:	cmpb   $0x1,(%rdi,%rax,1)
  4bfe1c:	jne    4bfde0 <zopro_core::service_trace::snapshot+0x870>
  4bfe1e:	call   *%rbx
  4bfe20:	jmp    4bfde3 <zopro_core::service_trace::snapshot+0x873>
  4bfe22:	call   114d00 <std::process::abort>
  4bfe27:	call   f5eb3 <core::panicking::panic_in_cleanup>
  4bfe2c:	call   f5eb3 <core::panicking::panic_in_cleanup>
  4bfe31:	call   f5eb3 <core::panicking::panic_in_cleanup>
