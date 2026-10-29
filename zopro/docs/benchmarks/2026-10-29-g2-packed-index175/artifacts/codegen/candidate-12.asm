
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000291d60 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute>:
  291d60:	push   %rbp
  291d61:	push   %r15
  291d63:	push   %r14
  291d65:	push   %r13
  291d67:	push   %r12
  291d69:	push   %rbx
  291d6a:	sub    $0xb8,%rsp
  291d71:	vmovups (%rdi),%xmm0
  291d75:	cmpq   $0x0,(%rdi)
  291d79:	movq   $0x0,(%rdi)
  291d80:	je     291ebe <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x15e>
  291d86:	mov    %fs:0xffffffffffffffc0,%rsi
  291d8f:	mov    %rdi,%rbx
  291d92:	test   %rsi,%rsi
  291d95:	je     291ea4 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x144>
  291d9b:	lea    0x10(%rbx),%rax
  291d9f:	vmovaps %xmm0,(%rsp)
  291da4:	vmovups (%rax),%zmm0
  291daa:	vmovups 0x40(%rax),%zmm1
  291db1:	vmovups 0x60(%rax),%zmm2
  291dbb:	vmovups %zmm1,0x50(%rsp)
  291dc6:	vmovups %zmm0,0x10(%rsp)
  291dd1:	vmovups %zmm2,0x70(%rsp)
  291ddc:	mov    %rsp,%rdi
  291ddf:	vzeroupper
  291de2:	call   2912b0 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}>
  291de7:	mov    $0x1,%r13d
  291ded:	cmpl   $0x2,0xb8(%rbx)
  291df4:	jb     291e71 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x111>
  291df6:	mov    0xc8(%rbx),%rbp
  291dfd:	mov    0xc0(%rbx),%r12
  291e04:	mov    0x0(%rbp),%rax
  291e08:	test   %rax,%rax
  291e0b:	je     291e12 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0xb2>
  291e0d:	mov    %r12,%rdi
  291e10:	call   *%rax
  291e12:	mov    0x8(%rbp),%rsi
  291e16:	test   %rsi,%rsi
  291e19:	je     291e71 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x111>
  291e1b:	mov    0x10(%rbp),%rax
  291e1f:	cmp    $0x20000,%rsi
  291e26:	setae  %cl
  291e29:	cmp    $0x1001,%rax
  291e2f:	setb   %dl
  291e32:	test   %cl,%dl
  291e34:	je     291e68 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x108>
  291e36:	blsr   %eax,%ecx
  291e3b:	jne    291ecc <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x16c>
  291e41:	movabs $0x8000000000000000,%rcx
  291e4b:	sub    %rax,%rcx
  291e4e:	cmp    %rcx,%rsi
  291e51:	jae    291ecc <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x16c>
  291e53:	cmpb   $0x1,(%r12,%rsi,1)
  291e58:	jne    291e68 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x108>
  291e5a:	inc    %rsi
  291e5d:	mov    %r12,%rdi
  291e60:	call   *0x258de2(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  291e66:	jmp    291e71 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x111>
  291e68:	mov    %r12,%rdi
  291e6b:	call   *0x258ddf(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  291e71:	mov    %r13,0xb8(%rbx)
  291e78:	mov    %r15,0xc0(%rbx)
  291e7f:	mov    %r14,0xc8(%rbx)
  291e86:	mov    0xb0(%rbx),%rdi
  291e8d:	call   1c36a0 <<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch> as rayon_core::latch::Latch>::set>
  291e92:	add    $0xb8,%rsp
  291e99:	pop    %rbx
  291e9a:	pop    %r12
  291e9c:	pop    %r13
  291e9e:	pop    %r14
  291ea0:	pop    %r15
  291ea2:	pop    %rbp
  291ea3:	ret
  291ea4:	lea    -0x272547(%rip),%rdi        # 1f964 <zmij::STATIC_DATA+0x5364>
  291eab:	lea    0x2551f6(%rip),%rdx        # 4e70a8 <__frame_dummy_init_array_entry+0x5248>
  291eb2:	mov    $0x36,%esi
  291eb7:	call   f0cd0 <core::panicking::panic>
  291ebc:	jmp    291f02 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x1a2>
  291ebe:	lea    0x257afb(%rip),%rdi        # 4e99c0 <__frame_dummy_init_array_entry+0x7b60>
  291ec5:	call   f1ff0 <core::option::unwrap_failed>
  291eca:	jmp    291f02 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x1a2>
  291ecc:	call   110970 <std::process::abort>
  291ed1:	mov    0x8(%rbp),%rsi
  291ed5:	test   %rsi,%rsi
  291ed8:	je     291ee6 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x186>
  291eda:	mov    0x10(%rbp),%rdx
  291ede:	mov    %r12,%rdi
  291ee1:	call   e9120 <__rustc::__rust_dealloc>
  291ee6:	mov    %r13,0xb8(%rbx)
  291eed:	mov    %r15,0xc0(%rbx)
  291ef4:	mov    %r14,0xc8(%rbx)
  291efb:	jmp    291efd <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x19d>
  291efd:	call   1052d0 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  291f02:	ud2
  291f04:	call   f2163 <core::panicking::panic_in_cleanup>
  291f09:	mov    %rax,%rdi
  291f0c:	call   14485a <std::panicking::catch_unwind::cleanup>
  291f11:	mov    $0x2,%r13d
  291f17:	mov    %rax,%r15
  291f1a:	mov    %rdx,%r14
  291f1d:	cmpl   $0x2,0xb8(%rbx)
  291f24:	jae    291df6 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x96>
  291f2a:	jmp    291e71 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute+0x111>
