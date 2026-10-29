
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000291f30 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>>:
  291f30:	push   %r15
  291f32:	push   %r14
  291f34:	push   %rbx
  291f35:	cmpl   $0x2,0xb8(%rdi)
  291f3c:	jb     291fad <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>+0x7d>
  291f3e:	mov    0xc8(%rdi),%r15
  291f45:	mov    0xc0(%rdi),%rbx
  291f4c:	mov    (%r15),%rax
  291f4f:	test   %rax,%rax
  291f52:	je     291f59 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>+0x29>
  291f54:	mov    %rbx,%rdi
  291f57:	call   *%rax
  291f59:	mov    0x8(%r15),%rsi
  291f5d:	test   %rsi,%rsi
  291f60:	je     291fad <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>+0x7d>
  291f62:	mov    0x10(%r15),%rax
  291f66:	cmp    $0x20000,%rsi
  291f6d:	setae  %cl
  291f70:	cmp    $0x1001,%rax
  291f76:	setb   %dl
  291f79:	test   %cl,%dl
  291f7b:	je     291fb3 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>+0x83>
  291f7d:	blsr   %eax,%ecx
  291f82:	jne    291fc1 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>+0x91>
  291f84:	movabs $0x8000000000000000,%rcx
  291f8e:	sub    %rax,%rcx
  291f91:	cmp    %rcx,%rsi
  291f94:	jae    291fc1 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>+0x91>
  291f96:	cmpb   $0x1,(%rbx,%rsi,1)
  291f9a:	jne    291fb3 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>+0x83>
  291f9c:	inc    %rsi
  291f9f:	mov    %rbx,%rdi
  291fa2:	pop    %rbx
  291fa3:	pop    %r14
  291fa5:	pop    %r15
  291fa7:	jmp    *0x258c9b(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  291fad:	pop    %rbx
  291fae:	pop    %r14
  291fb0:	pop    %r15
  291fb2:	ret
  291fb3:	mov    %rbx,%rdi
  291fb6:	pop    %rbx
  291fb7:	pop    %r14
  291fb9:	pop    %r15
  291fbb:	jmp    *0x258c8f(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  291fc1:	call   110970 <std::process::abort>
  291fc6:	mov    0x8(%r15),%rsi
  291fca:	mov    %rax,%r14
  291fcd:	test   %rsi,%rsi
  291fd0:	je     291fde <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>+0xae>
  291fd2:	mov    0x10(%r15),%rdx
  291fd6:	mov    %rbx,%rdi
  291fd9:	call   e9120 <__rustc::__rust_dealloc>
  291fde:	mov    %r14,%rdi
  291fe1:	call   4e0e10 <_Unwind_Resume@plt>
