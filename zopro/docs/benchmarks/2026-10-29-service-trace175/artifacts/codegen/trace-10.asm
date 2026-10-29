
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004c02c0 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>>:
  4c02c0:	push   %rbp
  4c02c1:	push   %r15
  4c02c3:	push   %r14
  4c02c5:	push   %r13
  4c02c7:	push   %r12
  4c02c9:	push   %rbx
  4c02ca:	push   %rax
  4c02cb:	mov    0x10(%rdi),%r13
  4c02cf:	mov    0x8(%rdi),%rbx
  4c02d3:	mov    %rdi,%r14
  4c02d6:	inc    %r13
  4c02d9:	mov    %rbx,%rdi
  4c02dc:	nopl   0x0(%rax)
  4c02e0:	cmp    $0x1,%r13
  4c02e4:	je     4c02f7 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>+0x37>
  4c02e6:	lea    0x30(%rdi),%r12
  4c02ea:	dec    %r13
  4c02ed:	call   4c0090 <core::ptr::drop_glue::<rayon_core::job::JobResult<(alloc::vec::Vec<[u64; 10]>, u64)>>>
  4c02f2:	mov    %r12,%rdi
  4c02f5:	jmp    4c02e0 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>+0x20>
  4c02f7:	mov    (%r14),%rax
  4c02fa:	test   %rax,%rax
  4c02fd:	je     4c0359 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>+0x99>
  4c02ff:	cmp    $0xaaa,%rax
  4c0305:	jbe    4c0342 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>+0x82>
  4c0307:	shl    $0x4,%rax
  4c030b:	movabs $0x7ffffffffffffff8,%rcx
  4c0315:	lea    (%rax,%rax,2),%rax
  4c0319:	mov    %rax,%rsi
  4c031c:	or     $0x1,%rsi
  4c0320:	cmp    %rcx,%rsi
  4c0323:	ja     4c0368 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>+0xa8>
  4c0325:	cmpb   $0x1,(%rbx,%rax,1)
  4c0329:	jne    4c0342 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>+0x82>
  4c032b:	mov    %rbx,%rdi
  4c032e:	add    $0x8,%rsp
  4c0332:	pop    %rbx
  4c0333:	pop    %r12
  4c0335:	pop    %r13
  4c0337:	pop    %r14
  4c0339:	pop    %r15
  4c033b:	pop    %rbp
  4c033c:	jmp    *0x2fa06(%rip)        # 4efd48 <munmap@GLIBC_2.2.5>
  4c0342:	mov    %rbx,%rdi
  4c0345:	add    $0x8,%rsp
  4c0349:	pop    %rbx
  4c034a:	pop    %r12
  4c034c:	pop    %r13
  4c034e:	pop    %r14
  4c0350:	pop    %r15
  4c0352:	pop    %rbp
  4c0353:	jmp    *0x2f9f7(%rip)        # 4efd50 <free@GLIBC_2.2.5>
  4c0359:	add    $0x8,%rsp
  4c035d:	pop    %rbx
  4c035e:	pop    %r12
  4c0360:	pop    %r13
  4c0362:	pop    %r14
  4c0364:	pop    %r15
  4c0366:	pop    %rbp
  4c0367:	ret
  4c0368:	call   114d00 <std::process::abort>
  4c036d:	mov    %rax,%r15
  4c0370:	dec    %r13
  4c0373:	je     4c0387 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>+0xc7>
  4c0375:	mov    %r12,%rdi
  4c0378:	lea    0x30(%r12),%rbp
  4c037d:	call   4c0090 <core::ptr::drop_glue::<rayon_core::job::JobResult<(alloc::vec::Vec<[u64; 10]>, u64)>>>
  4c0382:	mov    %rbp,%r12
  4c0385:	jmp    4c0370 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>+0xb0>
  4c0387:	mov    (%r14),%rax
  4c038a:	test   %rax,%rax
  4c038d:	je     4c03a4 <core::ptr::drop_glue::<alloc::vec::Vec<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>>+0xe4>
  4c038f:	shl    $0x4,%rax
  4c0393:	mov    $0x8,%edx
  4c0398:	mov    %rbx,%rdi
  4c039b:	lea    (%rax,%rax,2),%rsi
  4c039f:	call   eb3e0 <__rustc::__rust_dealloc>
  4c03a4:	mov    %r15,%rdi
  4c03a7:	call   4e5c10 <_Unwind_Resume@plt>
  4c03ac:	call   f5eb3 <core::panicking::panic_in_cleanup>
