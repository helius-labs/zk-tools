
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004bf180 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>>>:
  4bf180:	push   %r15
  4bf182:	push   %r14
  4bf184:	push   %rbx
  4bf185:	mov    0x8(%rdi),%rbx
  4bf189:	mov    0x10(%rdi),%rsi
  4bf18d:	mov    %rdi,%r15
  4bf190:	mov    %rbx,%rdi
  4bf193:	call   4bf060 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>>
  4bf198:	mov    (%r15),%rax
  4bf19b:	test   %rax,%rax
  4bf19e:	je     4bf1e8 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>>+0x68>
  4bf1a0:	cmp    $0xccc,%rax
  4bf1a6:	jbe    4bf1da <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>>+0x5a>
  4bf1a8:	shl    $0x3,%rax
  4bf1ac:	movabs $0x7ffffffffffffff8,%rcx
  4bf1b6:	lea    (%rax,%rax,4),%rax
  4bf1ba:	mov    %rax,%rsi
  4bf1bd:	or     $0x1,%rsi
  4bf1c1:	cmp    %rcx,%rsi
  4bf1c4:	ja     4bf1ee <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>>+0x6e>
  4bf1c6:	cmpb   $0x1,(%rbx,%rax,1)
  4bf1ca:	jne    4bf1da <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>>+0x5a>
  4bf1cc:	mov    %rbx,%rdi
  4bf1cf:	pop    %rbx
  4bf1d0:	pop    %r14
  4bf1d2:	pop    %r15
  4bf1d4:	jmp    *0x30b6e(%rip)        # 4efd48 <munmap@GLIBC_2.2.5>
  4bf1da:	mov    %rbx,%rdi
  4bf1dd:	pop    %rbx
  4bf1de:	pop    %r14
  4bf1e0:	pop    %r15
  4bf1e2:	jmp    *0x30b68(%rip)        # 4efd50 <free@GLIBC_2.2.5>
  4bf1e8:	pop    %rbx
  4bf1e9:	pop    %r14
  4bf1eb:	pop    %r15
  4bf1ed:	ret
  4bf1ee:	call   114d00 <std::process::abort>
  4bf1f3:	mov    %rax,%r14
  4bf1f6:	mov    (%r15),%rax
  4bf1f9:	test   %rax,%rax
  4bf1fc:	je     4bf213 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>>+0x93>
  4bf1fe:	shl    $0x3,%rax
  4bf202:	mov    $0x8,%edx
  4bf207:	mov    %rbx,%rdi
  4bf20a:	lea    (%rax,%rax,4),%rsi
  4bf20e:	call   eb3e0 <__rustc::__rust_dealloc>
  4bf213:	mov    %r14,%rdi
  4bf216:	call   4e5c10 <_Unwind_Resume@plt>
