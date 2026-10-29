
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004bf410 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>>:
  4bf410:	push   %r15
  4bf412:	push   %r14
  4bf414:	push   %rbx
  4bf415:	cmpl   $0x2,0x10(%rdi)
  4bf419:	jb     4bf484 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>+0x74>
  4bf41b:	mov    0x20(%rdi),%r15
  4bf41f:	mov    0x18(%rdi),%rbx
  4bf423:	mov    (%r15),%rax
  4bf426:	test   %rax,%rax
  4bf429:	je     4bf430 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>+0x20>
  4bf42b:	mov    %rbx,%rdi
  4bf42e:	call   *%rax
  4bf430:	mov    0x8(%r15),%rsi
  4bf434:	test   %rsi,%rsi
  4bf437:	je     4bf484 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>+0x74>
  4bf439:	mov    0x10(%r15),%rax
  4bf43d:	cmp    $0x20000,%rsi
  4bf444:	setae  %cl
  4bf447:	cmp    $0x1001,%rax
  4bf44d:	setb   %dl
  4bf450:	test   %cl,%dl
  4bf452:	je     4bf48a <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>+0x7a>
  4bf454:	blsr   %eax,%ecx
  4bf459:	jne    4bf498 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>+0x88>
  4bf45b:	movabs $0x8000000000000000,%rcx
  4bf465:	sub    %rax,%rcx
  4bf468:	cmp    %rcx,%rsi
  4bf46b:	jae    4bf498 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>+0x88>
  4bf46d:	cmpb   $0x1,(%rbx,%rsi,1)
  4bf471:	jne    4bf48a <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>+0x7a>
  4bf473:	inc    %rsi
  4bf476:	mov    %rbx,%rdi
  4bf479:	pop    %rbx
  4bf47a:	pop    %r14
  4bf47c:	pop    %r15
  4bf47e:	jmp    *0x308c4(%rip)        # 4efd48 <munmap@GLIBC_2.2.5>
  4bf484:	pop    %rbx
  4bf485:	pop    %r14
  4bf487:	pop    %r15
  4bf489:	ret
  4bf48a:	mov    %rbx,%rdi
  4bf48d:	pop    %rbx
  4bf48e:	pop    %r14
  4bf490:	pop    %r15
  4bf492:	jmp    *0x308b8(%rip)        # 4efd50 <free@GLIBC_2.2.5>
  4bf498:	call   114d00 <std::process::abort>
  4bf49d:	mov    0x8(%r15),%rsi
  4bf4a1:	mov    %rax,%r14
  4bf4a4:	test   %rsi,%rsi
  4bf4a7:	je     4bf4b5 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>+0xa5>
  4bf4a9:	mov    0x10(%r15),%rdx
  4bf4ad:	mov    %rbx,%rdi
  4bf4b0:	call   eb3e0 <__rustc::__rust_dealloc>
  4bf4b5:	mov    %r14,%rdi
  4bf4b8:	call   4e5c10 <_Unwind_Resume@plt>
