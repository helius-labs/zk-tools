
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000291b60 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute>:
  291b60:	push   %rbp
  291b61:	push   %r15
  291b63:	push   %r14
  291b65:	push   %r13
  291b67:	push   %r12
  291b69:	push   %rbx
  291b6a:	push   %rax
  291b6b:	mov    %rdi,%rbx
  291b6e:	mov    (%rdi),%rdi
  291b71:	mov    0x8(%rbx),%rsi
  291b75:	mov    0x10(%rbx),%rdx
  291b79:	mov    0x18(%rbx),%rcx
  291b7d:	mov    0x20(%rbx),%r8
  291b81:	mov    0x28(%rbx),%r9
  291b85:	mov    0x30(%rbx),%r12
  291b89:	mov    0x38(%rbx),%rax
  291b8d:	mov    0x40(%rbx),%r10
  291b91:	mov    0x48(%rbx),%r11
  291b95:	mov    0x50(%rbx),%r14
  291b99:	movq   $0x0,(%rbx)
  291ba0:	test   %rdi,%rdi
  291ba3:	je     291cc9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x169>
  291ba9:	mov    (%r10),%r10
  291bac:	mov    %rax,%r15
  291baf:	sub    %r10,%r15
  291bb2:	jb     291cd7 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x177>
  291bb8:	lea    (%r12,%r10,8),%rax
  291bbc:	push   %r14
  291bbe:	push   %r11
  291bc0:	push   %r15
  291bc2:	push   %rax
  291bc3:	call   28e7e0 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>>
  291bc8:	add    $0x20,%rsp
  291bcc:	mov    $0x1,%r13d
  291bd2:	cmpl   $0x2,0x58(%rbx)
  291bd6:	jb     291c53 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xf3>
  291bd8:	mov    0x68(%rbx),%rbp
  291bdc:	mov    0x60(%rbx),%r12
  291be0:	mov    0x0(%rbp),%rax
  291be4:	test   %rax,%rax
  291be7:	je     291bee <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x8e>
  291be9:	mov    %r12,%rdi
  291bec:	call   *%rax
  291bee:	mov    0x8(%rbp),%rsi
  291bf2:	test   %rsi,%rsi
  291bf5:	je     291c53 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xf3>
  291bf7:	mov    0x10(%rbp),%rax
  291bfb:	cmp    $0x20000,%rsi
  291c02:	setae  %cl
  291c05:	cmp    $0x1001,%rax
  291c0b:	setb   %dl
  291c0e:	test   %cl,%dl
  291c10:	je     291c48 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xe8>
  291c12:	blsr   %eax,%ecx
  291c17:	jne    291cee <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x18e>
  291c1d:	movabs $0x8000000000000000,%rcx
  291c27:	sub    %rax,%rcx
  291c2a:	cmp    %rcx,%rsi
  291c2d:	jae    291cee <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x18e>
  291c33:	cmpb   $0x1,(%r12,%rsi,1)
  291c38:	jne    291c4a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xea>
  291c3a:	inc    %rsi
  291c3d:	mov    %r12,%rdi
  291c40:	call   *0x259002(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  291c46:	jmp    291c53 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xf3>
  291c48:	jmp    291c4a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xea>
  291c4a:	mov    %r12,%rdi
  291c4d:	call   *0x258ffd(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  291c53:	mov    %r13,0x58(%rbx)
  291c57:	mov    %r15,0x60(%rbx)
  291c5b:	mov    %r14,0x68(%rbx)
  291c5f:	mov    0x70(%rbx),%rax
  291c63:	movzbl 0x88(%rbx),%ebp
  291c6a:	mov    (%rax),%rdi
  291c6d:	cmp    $0x1,%bpl
  291c71:	jne    291c85 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x125>
  291c73:	lock incq (%rdi)
  291c77:	jle    291d35 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x1d5>
  291c7d:	mov    (%rax),%r14
  291c80:	mov    %r14,%rdi
  291c83:	jmp    291c85 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x125>
  291c85:	mov    $0x3,%eax
  291c8a:	mov    0x80(%rbx),%rsi
  291c91:	xchg   %rax,0x78(%rbx)
  291c95:	cmp    $0x2,%rax
  291c99:	jne    291ca7 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x147>
  291c9b:	add    $0x1d8,%rdi
  291ca2:	call   ff7c0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  291ca7:	test   %bpl,%bpl
  291caa:	je     291cba <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x15a>
  291cac:	lock decq (%r14)
  291cb0:	jne    291cba <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x15a>
  291cb2:	mov    %r14,%rdi
  291cb5:	call   100580 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  291cba:	add    $0x8,%rsp
  291cbe:	pop    %rbx
  291cbf:	pop    %r12
  291cc1:	pop    %r13
  291cc3:	pop    %r14
  291cc5:	pop    %r15
  291cc7:	pop    %rbp
  291cc8:	ret
  291cc9:	lea    0x257cf0(%rip),%rdi        # 4e99c0 <__frame_dummy_init_array_entry+0x7b60>
  291cd0:	call   f1ff0 <core::option::unwrap_failed>
  291cd5:	jmp    291d35 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x1d5>
  291cd7:	lea    0x255352(%rip),%rcx        # 4e7030 <__frame_dummy_init_array_entry+0x51d0>
  291cde:	mov    %r10,%rdi
  291ce1:	mov    %rax,%rsi
  291ce4:	mov    %rax,%rdx
  291ce7:	call   f0c00 <core::slice::index::slice_index_fail>
  291cec:	jmp    291d35 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x1d5>
  291cee:	call   110970 <std::process::abort>
  291cf3:	mov    0x8(%rbp),%rsi
  291cf7:	test   %rsi,%rsi
  291cfa:	je     291d08 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x1a8>
  291cfc:	mov    0x10(%rbp),%rdx
  291d00:	mov    %r12,%rdi
  291d03:	call   e9120 <__rustc::__rust_dealloc>
  291d08:	mov    %r13,0x58(%rbx)
  291d0c:	mov    %r15,0x60(%rbx)
  291d10:	mov    %r14,0x68(%rbx)
  291d14:	jmp    291d30 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x1d0>
  291d16:	test   %bpl,%bpl
  291d19:	je     291d30 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x1d0>
  291d1b:	lock decq (%r14)
  291d1f:	jne    291d30 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x1d0>
  291d21:	mov    %r14,%rdi
  291d24:	call   100580 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  291d29:	jmp    291d30 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x1d0>
  291d2b:	call   f2163 <core::panicking::panic_in_cleanup>
  291d30:	call   1052d0 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  291d35:	ud2
  291d37:	mov    %rax,%rdi
  291d3a:	call   14485a <std::panicking::catch_unwind::cleanup>
  291d3f:	mov    $0x2,%r13d
  291d45:	mov    %rax,%r15
  291d48:	mov    %rdx,%r14
  291d4b:	cmpl   $0x2,0x58(%rbx)
  291d4f:	jae    291bd8 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x78>
  291d55:	jmp    291c53 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<(), rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xf3>
  291d5a:	call   f2163 <core::panicking::panic_in_cleanup>
