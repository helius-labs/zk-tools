
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000291aa0 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>>:
  291aa0:	push   %r15
  291aa2:	push   %r14
  291aa4:	push   %rbx
  291aa5:	cmpl   $0x2,0xb0(%rdi)
  291aac:	jb     291b1d <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>+0x7d>
  291aae:	mov    0xc0(%rdi),%r15
  291ab5:	mov    0xb8(%rdi),%rbx
  291abc:	mov    (%r15),%rax
  291abf:	test   %rax,%rax
  291ac2:	je     291ac9 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>+0x29>
  291ac4:	mov    %rbx,%rdi
  291ac7:	call   *%rax
  291ac9:	mov    0x8(%r15),%rsi
  291acd:	test   %rsi,%rsi
  291ad0:	je     291b1d <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>+0x7d>
  291ad2:	mov    0x10(%r15),%rax
  291ad6:	cmp    $0x20000,%rsi
  291add:	setae  %cl
  291ae0:	cmp    $0x1001,%rax
  291ae6:	setb   %dl
  291ae9:	test   %cl,%dl
  291aeb:	je     291b23 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>+0x83>
  291aed:	blsr   %eax,%ecx
  291af2:	jne    291b31 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>+0x91>
  291af4:	movabs $0x8000000000000000,%rcx
  291afe:	sub    %rax,%rcx
  291b01:	cmp    %rcx,%rsi
  291b04:	jae    291b31 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>+0x91>
  291b06:	cmpb   $0x1,(%rbx,%rsi,1)
  291b0a:	jne    291b23 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>+0x83>
  291b0c:	inc    %rsi
  291b0f:	mov    %rbx,%rdi
  291b12:	pop    %rbx
  291b13:	pop    %r14
  291b15:	pop    %r15
  291b17:	jmp    *0x25912b(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  291b1d:	pop    %rbx
  291b1e:	pop    %r14
  291b20:	pop    %r15
  291b22:	ret
  291b23:	mov    %rbx,%rdi
  291b26:	pop    %rbx
  291b27:	pop    %r14
  291b29:	pop    %r15
  291b2b:	jmp    *0x25911f(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  291b31:	call   110970 <std::process::abort>
  291b36:	mov    0x8(%r15),%rsi
  291b3a:	mov    %rax,%r14
  291b3d:	test   %rsi,%rsi
  291b40:	je     291b4e <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>+0xae>
  291b42:	mov    0x10(%r15),%rdx
  291b46:	mov    %rbx,%rdi
  291b49:	call   e9120 <__rustc::__rust_dealloc>
  291b4e:	mov    %r14,%rdi
  291b51:	call   4e0e10 <_Unwind_Resume@plt>
