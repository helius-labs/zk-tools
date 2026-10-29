
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004be6c0 <zopro_core::service_trace::reset>:
  4be6c0:	push   %rbp
  4be6c1:	push   %r15
  4be6c3:	push   %r14
  4be6c5:	push   %r13
  4be6c7:	push   %r12
  4be6c9:	push   %rbx
  4be6ca:	sub    $0x98,%rsp
  4be6d1:	mov    %edi,0x44(%rsp)
  4be6d5:	xor    %eax,%eax
  4be6d7:	xchg   %al,0x3249b(%rip)        # 4f0b78 <zopro_core::service_trace::ENABLED>
  4be6dd:	mov    0x3233d(%rip),%eax        # 4f0a20 <zopro_core::service_trace::CLOCK+0x10>
  4be6e3:	test   %eax,%eax
  4be6e5:	jne    4be709 <zopro_core::service_trace::reset+0x49>
  4be6e7:	mov    %fs:0xffffffffffffffb0,%rax
  4be6f0:	test   %rax,%rax
  4be6f3:	je     4be71c <zopro_core::service_trace::reset+0x5c>
  4be6f5:	add    $0x110,%rax
  4be6fb:	mov    (%rax),%rcx
  4be6fe:	lock incq (%rcx)
  4be702:	jg     4be72e <zopro_core::service_trace::reset+0x6e>
  4be704:	jmp    4bec6c <zopro_core::service_trace::reset+0x5ac>
  4be709:	call   4bed84 <<std::sync::once_lock::OnceLock<std::time::Instant>>::initialize::<<std::sync::once_lock::OnceLock<std::time::Instant>>::get_or_init<<std::time::Instant>::now>::{closure#0}, !>>
  4be70e:	mov    %fs:0xffffffffffffffb0,%rax
  4be717:	test   %rax,%rax
  4be71a:	jne    4be6f5 <zopro_core::service_trace::reset+0x35>
  4be71c:	call   108b20 <rayon_core::registry::global_registry>
  4be721:	mov    (%rax),%rcx
  4be724:	lock incq (%rcx)
  4be728:	jle    4bec6c <zopro_core::service_trace::reset+0x5ac>
  4be72e:	mov    (%rax),%r15
  4be731:	mov    %fs:0xffffffffffffffb0,%r12
  4be73a:	mov    0x208(%r15),%r14
  4be741:	test   %r12,%r12
  4be744:	je     4be76a <zopro_core::service_trace::reset+0xaa>
  4be746:	mov    0x110(%r12),%rax
  4be74e:	lock incq (%rax)
  4be752:	jle    4bec6c <zopro_core::service_trace::reset+0x5ac>
  4be758:	mov    0x100(%r12),%rcx
  4be760:	mov    0x110(%r12),%rax
  4be768:	jmp    4be76e <zopro_core::service_trace::reset+0xae>
  4be76a:	xor    %eax,%eax
  4be76c:	xor    %ecx,%ecx
  4be76e:	mov    %r14,0x60(%rsp)
  4be773:	mov    %rax,0x68(%rsp)
  4be778:	lea    0x0(,%r14,8),%rax
  4be780:	movq   $0x0,0x70(%rsp)
  4be789:	mov    %rcx,0x78(%rsp)
  4be78e:	lea    (%rax,%rax,4),%rsi
  4be792:	mov    %rsi,0x48(%rsp)
  4be797:	test   %r14,%r14
  4be79a:	je     4be7c1 <zopro_core::service_trace::reset+0x101>
  4be79c:	mov    $0x8,%edi
  4be7a1:	xor    %edx,%edx
  4be7a3:	call   4c0830 <<zopro_core::alloc::linux::Allocator>::allocate>
  4be7a8:	test   %rax,%rax
  4be7ab:	je     4bec5d <zopro_core::service_trace::reset+0x59d>
  4be7b1:	mov    %rax,%rbx
  4be7b4:	cmp    $0x8,%r14
  4be7b8:	jae    4be7cb <zopro_core::service_trace::reset+0x10b>
  4be7ba:	xor    %eax,%eax
  4be7bc:	jmp    4be85a <zopro_core::service_trace::reset+0x19a>
  4be7c1:	mov    $0x8,%ebx
  4be7c6:	jmp    4be898 <zopro_core::service_trace::reset+0x1d8>
  4be7cb:	vmovdqa64 -0x4910d5(%rip),%zmm2        # 2d700 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x59d0>
  4be7d5:	vpbroadcastq -0x497d07(%rip),%zmm3        # 26ad8 <GCC_except_table3180+0x98>
  4be7df:	vpbroadcastq -0x497d61(%rip),%zmm5        # 26a88 <GCC_except_table3180+0x48>
  4be7e9:	lea    0x60(%rsp),%rcx
  4be7ee:	movabs $0x3fffffffffffff8,%rax
  4be7f8:	vpxor  %xmm4,%xmm4,%xmm4
  4be7fc:	vpbroadcastq %rcx,%zmm0
  4be802:	lea    0xf(%rsp),%rcx
  4be807:	and    %r14,%rax
  4be80a:	vpbroadcastq %rcx,%zmm1
  4be810:	mov    %rax,%rcx
  4be813:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4be820:	vpmullq %zmm3,%zmm2,%zmm6
  4be826:	kxnorb %k0,%k0,%k1
  4be82a:	vpaddq %zmm5,%zmm2,%zmm2
  4be830:	add    $0xfffffffffffffff8,%rcx
  4be834:	vpscatterqq %zmm0,(%rbx,%zmm6,1){%k1}
  4be83b:	kxnorb %k0,%k0,%k1
  4be83f:	vpscatterqq %zmm1,0x8(%rbx,%zmm6,1){%k1}
  4be847:	kxnorb %k0,%k0,%k1
  4be84b:	vpscatterqq %zmm4,0x10(%rbx,%zmm6,1){%k1}
  4be853:	jne    4be820 <zopro_core::service_trace::reset+0x160>
  4be855:	cmp    %rax,%r14
  4be858:	je     4be898 <zopro_core::service_trace::reset+0x1d8>
  4be85a:	lea    (%rax,%rax,4),%rcx
  4be85e:	mov    %r14,%rdx
  4be861:	sub    %rax,%rdx
  4be864:	lea    0x60(%rsp),%rax
  4be869:	lea    0xf(%rsp),%rsi
  4be86e:	lea    0x10(%rbx,%rcx,8),%rcx
  4be873:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4be880:	mov    %rax,-0x10(%rcx)
  4be884:	mov    %rsi,-0x8(%rcx)
  4be888:	movq   $0x0,(%rcx)
  4be88f:	add    $0x28,%rcx
  4be893:	dec    %rdx
  4be896:	jne    4be880 <zopro_core::service_trace::reset+0x1c0>
  4be898:	mov    %r14,0x80(%rsp)
  4be8a0:	mov    %rbx,0x88(%rsp)
  4be8a8:	mov    %r14,0x90(%rsp)
  4be8b0:	mov    0x208(%r15),%rax
  4be8b7:	mov    %r14,0x30(%rsp)
  4be8bc:	mov    %rax,0x28(%rsp)
  4be8c1:	cmp    %r14,%rax
  4be8c4:	jne    4beb34 <zopro_core::service_trace::reset+0x474>
  4be8ca:	mov    $0x1,%ecx
  4be8cf:	xor    %eax,%eax
  4be8d1:	lea    0x180(%r15),%r13
  4be8d8:	lock cmpxchg %ecx,0x180(%r15)
  4be8e1:	jne    4beb54 <zopro_core::service_trace::reset+0x494>
  4be8e7:	mov    0x3225a(%rip),%rax        # 4f0b48 <std::panicking::panic_count::GLOBAL_PANIC_COUNT>
  4be8ee:	shl    $1,%rax
  4be8f1:	test   %rax,%rax
  4be8f4:	jne    4beb72 <zopro_core::service_trace::reset+0x4b2>
  4be8fa:	xor    %ebp,%ebp
  4be8fc:	movzbl 0x184(%r15),%eax
  4be904:	test   %al,%al
  4be906:	jne    4beb90 <zopro_core::service_trace::reset+0x4d0>
  4be90c:	mov    0x198(%r15),%rax
  4be913:	mov    %r14,0x30(%rsp)
  4be918:	mov    %rax,0x28(%rsp)
  4be91d:	cmp    %r14,%rax
  4be920:	jne    4bebc6 <zopro_core::service_trace::reset+0x506>
  4be926:	mov    %ebp,0x1c(%rsp)
  4be92a:	mov    %r13,0x50(%rsp)
  4be92f:	mov    %r12,0x10(%rsp)
  4be934:	mov    %r15,0x20(%rsp)
  4be939:	mov    %r14,0x58(%rsp)
  4be93e:	test   %r14,%r14
  4be941:	je     4be9d9 <zopro_core::service_trace::reset+0x319>
  4be947:	mov    0x20(%rsp),%rax
  4be94c:	mov    0x58(%rsp),%r14
  4be951:	lea    0x538(%rip),%rbp        # 4bee90 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute>
  4be958:	mov    %rbx,%r15
  4be95b:	mov    0x190(%rax),%r13
  4be962:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4be970:	mov    0x0(%r13),%rax
  4be974:	mov    0x108(%rax),%r12
  4be97b:	mov    0x100(%rax),%rax
  4be982:	mov    0x10(%r13),%rsi
  4be986:	mov    %r12,%rcx
  4be989:	sub    %rax,%rcx
  4be98c:	cmp    %rsi,%rcx
  4be98f:	jge    4be9c5 <zopro_core::service_trace::reset+0x305>
  4be991:	mov    0x8(%r13),%rax
  4be995:	dec    %rsi
  4be998:	and    %r12,%rsi
  4be99b:	inc    %r12
  4be99e:	shl    $0x4,%rsi
  4be9a2:	mov    %rbp,(%rax,%rsi,1)
  4be9a6:	mov    %r15,0x8(%rax,%rsi,1)
  4be9ab:	mov    0x0(%r13),%rcx
  4be9af:	add    $0x28,%r15
  4be9b3:	add    $0x20,%r13
  4be9b7:	dec    %r14
  4be9ba:	mov    %r12,0x108(%rcx)
  4be9c1:	jne    4be970 <zopro_core::service_trace::reset+0x2b0>
  4be9c3:	jmp    4be9d9 <zopro_core::service_trace::reset+0x319>
  4be9c5:	add    %rsi,%rsi
  4be9c8:	mov    %r13,%rdi
  4be9cb:	vzeroupper
  4be9ce:	call   1c6c30 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::resize>
  4be9d3:	mov    0x10(%r13),%rsi
  4be9d7:	jmp    4be991 <zopro_core::service_trace::reset+0x2d1>
  4be9d9:	cmpb   $0x0,0x1c(%rsp)
  4be9de:	mov    0x20(%rsp),%r15
  4be9e3:	jne    4be9f8 <zopro_core::service_trace::reset+0x338>
  4be9e5:	mov    0x3215c(%rip),%rax        # 4f0b48 <std::panicking::panic_count::GLOBAL_PANIC_COUNT>
  4be9ec:	shl    $1,%rax
  4be9ef:	test   %rax,%rax
  4be9f2:	jne    4bec34 <zopro_core::service_trace::reset+0x574>
  4be9f8:	mov    0x50(%rsp),%rsi
  4be9fd:	xor    %eax,%eax
  4be9ff:	xchg   %eax,(%rsi)
  4bea01:	cmp    $0x2,%eax
  4bea04:	je     4bebe6 <zopro_core::service_trace::reset+0x526>
  4bea0a:	mov    0x208(%r15),%r14
  4bea11:	mov    0x58(%rsp),%r12
  4bea16:	test   %r14,%r14
  4bea19:	je     4bea46 <zopro_core::service_trace::reset+0x386>
  4bea1b:	lea    0x1d8(%r15),%r13
  4bea22:	xor    %ebp,%ebp
  4bea24:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  4bea30:	mov    %r13,%rdi
  4bea33:	mov    %rbp,%rsi
  4bea36:	vzeroupper
  4bea39:	call   1038b0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  4bea3e:	inc    %rbp
  4bea41:	cmp    %rbp,%r14
  4bea44:	jne    4bea30 <zopro_core::service_trace::reset+0x370>
  4bea46:	mov    0x10(%rsp),%rsi
  4bea4b:	lea    0x60(%rsp),%rdi
  4bea50:	vzeroupper
  4bea53:	call   109340 <<rayon_core::latch::CountLatch>::wait>
  4bea58:	test   %r12,%r12
  4bea5b:	je     4bea97 <zopro_core::service_trace::reset+0x3d7>
  4bea5d:	mov    0x48(%rsp),%rcx
  4bea62:	mov    %rbx,%rax
  4bea65:	lea    (%rbx,%rcx,1),%rbp
  4bea69:	mov    %rcx,%r14
  4bea6c:	mov    0x10(%rax),%rdx
  4bea70:	lea    0x28(%rax),%r13
  4bea74:	cmp    $0x1,%rdx
  4bea78:	jne    4beb18 <zopro_core::service_trace::reset+0x458>
  4bea7e:	add    $0xffffffffffffffd8,%rcx
  4bea82:	mov    %r13,%rax
  4bea85:	jne    4bea6c <zopro_core::service_trace::reset+0x3ac>
  4bea87:	mov    $0x8,%edx
  4bea8c:	mov    %rbx,%rdi
  4bea8f:	mov    %r14,%rsi
  4bea92:	call   eb3e0 <__rustc::__rust_dealloc>
  4bea97:	mov    0x68(%rsp),%rax
  4bea9c:	test   %rax,%rax
  4bea9f:	je     4beab1 <zopro_core::service_trace::reset+0x3f1>
  4beaa1:	lock decq (%rax)
  4beaa5:	jne    4beab1 <zopro_core::service_trace::reset+0x3f1>
  4beaa7:	mov    0x68(%rsp),%rdi
  4beaac:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4beab1:	lock decq (%r15)
  4beab5:	jne    4beabf <zopro_core::service_trace::reset+0x3ff>
  4beab7:	mov    %r15,%rdi
  4beaba:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4beabf:	call   4bf220 <zopro_core::service_trace::clear>
  4beac4:	movq   $0x0,0x32099(%rip)        # 4f0b68 <zopro_core::service_trace::API>
  4beacf:	mov    0x31f4b(%rip),%eax        # 4f0a20 <zopro_core::service_trace::CLOCK+0x10>
  4bead5:	test   %eax,%eax
  4bead7:	jne    4bec51 <zopro_core::service_trace::reset+0x591>
  4beadd:	lea    0x31f2c(%rip),%rdi        # 4f0a10 <zopro_core::service_trace::CLOCK>
  4beae4:	call   13d820 <<std::time::Instant>::elapsed>
  4beae9:	imul   $0x3b9aca00,%rax,%rax
  4beaf0:	mov    %edx,%ecx
  4beaf2:	add    %rax,%rcx
  4beaf5:	mov    %rcx,0x32074(%rip)        # 4f0b70 <zopro_core::service_trace::ORIGIN>
  4beafc:	mov    0x44(%rsp),%eax
  4beb00:	xchg   %al,0x32072(%rip)        # 4f0b78 <zopro_core::service_trace::ENABLED>
  4beb06:	add    $0x98,%rsp
  4beb0d:	pop    %rbx
  4beb0e:	pop    %r12
  4beb10:	pop    %r13
  4beb12:	pop    %r14
  4beb14:	pop    %r15
  4beb16:	pop    %rbp
  4beb17:	ret
  4beb18:	cmp    $0x2,%rdx
  4beb1c:	jne    4bec1a <zopro_core::service_trace::reset+0x55a>
  4beb22:	mov    0x18(%rax),%rdi
  4beb26:	mov    0x20(%rax),%rsi
  4beb2a:	call   108e90 <rayon_core::unwind::resume_unwinding>
  4beb2f:	jmp    4bec6c <zopro_core::service_trace::reset+0x5ac>
  4beb34:	lea    0x2aac5(%rip),%r8        # 4e9600 <__frame_dummy_init_array_entry+0x2978>
  4beb3b:	lea    0x28(%rsp),%rdi
  4beb40:	lea    0x30(%rsp),%rsi
  4beb45:	xor    %edx,%edx
  4beb47:	vzeroupper
  4beb4a:	call   f3d52 <core::panicking::assert_failed::<usize, usize>>
  4beb4f:	jmp    4bec6c <zopro_core::service_trace::reset+0x5ac>
  4beb54:	mov    %r13,%rdi
  4beb57:	vzeroupper
  4beb5a:	call   115050 <<std::sys::sync::mutex::futex::Mutex>::lock_contended>
  4beb5f:	mov    0x31fe2(%rip),%rax        # 4f0b48 <std::panicking::panic_count::GLOBAL_PANIC_COUNT>
  4beb66:	shl    $1,%rax
  4beb69:	test   %rax,%rax
  4beb6c:	je     4be8fa <zopro_core::service_trace::reset+0x23a>
  4beb72:	vzeroupper
  4beb75:	call   114f10 <std::panicking::panic_count::is_zero_slow_path>
  4beb7a:	mov    %eax,%ebp
  4beb7c:	xor    $0x1,%bpl
  4beb80:	movzbl 0x184(%r15),%eax
  4beb88:	test   %al,%al
  4beb8a:	je     4be90c <zopro_core::service_trace::reset+0x24c>
  4beb90:	mov    %r13,0x30(%rsp)
  4beb95:	mov    %bpl,0x38(%rsp)
  4beb9a:	lea    -0x418850(%rip),%rdi        # a6351 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x5b9>
  4beba1:	lea    0x2dfc8(%rip),%rcx        # 4ecb70 <__frame_dummy_init_array_entry+0x5ee8>
  4beba8:	lea    0x2aa69(%rip),%r8        # 4e9618 <__frame_dummy_init_array_entry+0x2990>
  4bebaf:	lea    0x30(%rsp),%rdx
  4bebb4:	mov    $0x2b,%esi
  4bebb9:	vzeroupper
  4bebbc:	call   f5df0 <core::result::unwrap_failed>
  4bebc1:	jmp    4bec6c <zopro_core::service_trace::reset+0x5ac>
  4bebc6:	lea    0x2aa63(%rip),%r8        # 4e9630 <__frame_dummy_init_array_entry+0x29a8>
  4bebcd:	lea    0x28(%rsp),%rdi
  4bebd2:	lea    0x30(%rsp),%rsi
  4bebd7:	xor    %edx,%edx
  4bebd9:	vzeroupper
  4bebdc:	call   f3d52 <core::panicking::assert_failed::<usize, usize>>
  4bebe1:	jmp    4bec6c <zopro_core::service_trace::reset+0x5ac>
  4bebe6:	mov    $0xca,%edi
  4bebeb:	mov    $0x81,%edx
  4bebf0:	mov    $0x1,%ecx
  4bebf5:	xor    %eax,%eax
  4bebf7:	vzeroupper
  4bebfa:	call   *0x311d0(%rip)        # 4efdd0 <syscall@GLIBC_2.2.5>
  4bec00:	mov    0x208(%r15),%r14
  4bec07:	mov    0x58(%rsp),%r12
  4bec0c:	test   %r14,%r14
  4bec0f:	jne    4bea1b <zopro_core::service_trace::reset+0x35b>
  4bec15:	jmp    4bea46 <zopro_core::service_trace::reset+0x386>
  4bec1a:	lea    -0x483fab(%rip),%rdi        # 3ac76 <zmij::STATIC_DATA+0x4d76>
  4bec21:	lea    0x2ea90(%rip),%rdx        # 4ed6b8 <__frame_dummy_init_array_entry+0x6a30>
  4bec28:	mov    $0x28,%esi
  4bec2d:	call   f49d0 <core::panicking::panic>
  4bec32:	jmp    4bec6c <zopro_core::service_trace::reset+0x5ac>
  4bec34:	vzeroupper
  4bec37:	call   114f10 <std::panicking::panic_count::is_zero_slow_path>
  4bec3c:	test   %al,%al
  4bec3e:	jne    4be9f8 <zopro_core::service_trace::reset+0x338>
  4bec44:	movb   $0x1,0x184(%r15)
  4bec4c:	jmp    4be9f8 <zopro_core::service_trace::reset+0x338>
  4bec51:	lea    0x2f498(%rip),%rdi        # 4ee0f0 <__frame_dummy_init_array_entry+0x7468>
  4bec58:	call   f5cf0 <core::option::unwrap_failed>
  4bec5d:	mov    0x48(%rsp),%rsi
  4bec62:	mov    $0x8,%edi
  4bec67:	call   f1bb1 <alloc::raw_vec::handle_error>
  4bec6c:	ud2
  4bec6e:	mov    0x20(%rsp),%r15
  4bec73:	mov    0x50(%rsp),%r13
  4bec78:	mov    0x1c(%rsp),%ebp
  4bec7c:	jmp    4beca2 <zopro_core::service_trace::reset+0x5e2>
  4bec7e:	mov    %rax,0x10(%rsp)
  4bec83:	jmp    4bed4d <zopro_core::service_trace::reset+0x68d>
  4bec88:	mov    %rax,0x10(%rsp)
  4bec8d:	mov    0x68(%rsp),%rax
  4bec92:	test   %rax,%rax
  4bec95:	jne    4bed3d <zopro_core::service_trace::reset+0x67d>
  4bec9b:	jmp    4bed4d <zopro_core::service_trace::reset+0x68d>
  4beca0:	jmp    4becc6 <zopro_core::service_trace::reset+0x606>
  4beca2:	mov    %rax,0x10(%rsp)
  4beca7:	movzbl %bpl,%esi
  4becab:	mov    %r13,%rdi
  4becae:	call   4bee30 <core::ptr::drop_glue::<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>>>>
  4becb3:	jmp    4beccb <zopro_core::service_trace::reset+0x60b>
  4becb5:	lea    0x30(%rsp),%rdi
  4becba:	mov    %rax,0x10(%rsp)
  4becbf:	call   4bede0 <core::ptr::drop_glue::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>>>>>
  4becc4:	jmp    4beccb <zopro_core::service_trace::reset+0x60b>
  4becc6:	mov    %rax,0x10(%rsp)
  4beccb:	lea    0x80(%rsp),%rdi
  4becd3:	call   4bf180 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>>>
  4becd8:	jmp    4bed1e <zopro_core::service_trace::reset+0x65e>
  4becda:	mov    %rax,0x10(%rsp)
  4becdf:	mov    %rbp,%rax
  4bece2:	sub    %r13,%rax
  4bece5:	movabs $0xcccccccccccccccd,%rsi
  4becef:	shr    $0x3,%rax
  4becf3:	imul   %rax,%rsi
  4becf7:	mov    %r13,%rdi
  4becfa:	call   4bf060 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>>
  4becff:	cmp    $0xccc,%r12
  4bed06:	jbe    4bed2a <zopro_core::service_trace::reset+0x66a>
  4bed08:	cmpb   $0x1,0x0(%rbp)
  4bed0c:	jne    4bed2a <zopro_core::service_trace::reset+0x66a>
  4bed0e:	or     $0x1,%r14
  4bed12:	mov    %rbx,%rdi
  4bed15:	mov    %r14,%rsi
  4bed18:	call   *0x3102a(%rip)        # 4efd48 <munmap@GLIBC_2.2.5>
  4bed1e:	mov    0x68(%rsp),%rax
  4bed23:	test   %rax,%rax
  4bed26:	jne    4bed3d <zopro_core::service_trace::reset+0x67d>
  4bed28:	jmp    4bed4d <zopro_core::service_trace::reset+0x68d>
  4bed2a:	mov    %rbx,%rdi
  4bed2d:	call   *0x3101d(%rip)        # 4efd50 <free@GLIBC_2.2.5>
  4bed33:	mov    0x68(%rsp),%rax
  4bed38:	test   %rax,%rax
  4bed3b:	je     4bed4d <zopro_core::service_trace::reset+0x68d>
  4bed3d:	lock decq (%rax)
  4bed41:	jne    4bed4d <zopro_core::service_trace::reset+0x68d>
  4bed43:	mov    0x68(%rsp),%rdi
  4bed48:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4bed4d:	lock decq (%r15)
  4bed51:	jne    4bed5b <zopro_core::service_trace::reset+0x69b>
  4bed53:	mov    %r15,%rdi
  4bed56:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4bed5b:	mov    0x10(%rsp),%rdi
  4bed60:	call   4e5c10 <_Unwind_Resume@plt>
  4bed65:	call   f5eb3 <core::panicking::panic_in_cleanup>
  4bed6a:	call   f5eb3 <core::panicking::panic_in_cleanup>
  4bed6f:	mov    $0x8,%edx
  4bed74:	mov    %rbx,%rdi
  4bed77:	mov    %r14,%rsi
  4bed7a:	call   eb3e0 <__rustc::__rust_dealloc>
  4bed7f:	call   f5eb3 <core::panicking::panic_in_cleanup>
