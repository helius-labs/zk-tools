
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004bf060 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>>:
  4bf060:	push   %rbp
  4bf061:	push   %r15
  4bf063:	push   %r14
  4bf065:	push   %r13
  4bf067:	push   %r12
  4bf069:	push   %rbx
  4bf06a:	push   %rax
  4bf06b:	test   %rsi,%rsi
  4bf06e:	je     4bf116 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0xb6>
  4bf074:	mov    0x30cd5(%rip),%rbp        # 4efd50 <free@GLIBC_2.2.5>
  4bf07b:	mov    %rdi,%r14
  4bf07e:	add    $0x28,%r14
  4bf082:	movabs $0x8000000000000000,%r12
  4bf08c:	mov    %rsi,%rbx
  4bf08f:	jmp    4bf0ae <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0x4e>
  4bf091:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4bf0a0:	mov    %r15,%rdi
  4bf0a3:	call   *%rbp
  4bf0a5:	add    $0x28,%r14
  4bf0a9:	dec    %rbx
  4bf0ac:	je     4bf116 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0xb6>
  4bf0ae:	cmpl   $0x2,-0x18(%r14)
  4bf0b3:	jb     4bf0a5 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0x45>
  4bf0b5:	mov    -0x8(%r14),%r13
  4bf0b9:	mov    -0x10(%r14),%r15
  4bf0bd:	mov    0x0(%r13),%rax
  4bf0c1:	test   %rax,%rax
  4bf0c4:	je     4bf0cb <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0x6b>
  4bf0c6:	mov    %r15,%rdi
  4bf0c9:	call   *%rax
  4bf0cb:	mov    0x8(%r13),%rsi
  4bf0cf:	test   %rsi,%rsi
  4bf0d2:	je     4bf0a5 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0x45>
  4bf0d4:	mov    0x10(%r13),%rax
  4bf0d8:	cmp    $0x20000,%rsi
  4bf0df:	setae  %cl
  4bf0e2:	cmp    $0x1001,%rax
  4bf0e8:	setb   %dl
  4bf0eb:	test   %cl,%dl
  4bf0ed:	je     4bf0a0 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0x40>
  4bf0ef:	blsr   %eax,%ecx
  4bf0f4:	jne    4bf125 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0xc5>
  4bf0f6:	mov    %r12,%rcx
  4bf0f9:	sub    %rax,%rcx
  4bf0fc:	cmp    %rcx,%rsi
  4bf0ff:	jae    4bf125 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0xc5>
  4bf101:	cmpb   $0x1,(%r15,%rsi,1)
  4bf106:	jne    4bf0a0 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0x40>
  4bf108:	inc    %rsi
  4bf10b:	mov    %r15,%rdi
  4bf10e:	call   *0x30c34(%rip)        # 4efd48 <munmap@GLIBC_2.2.5>
  4bf114:	jmp    4bf0a5 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0x45>
  4bf116:	add    $0x8,%rsp
  4bf11a:	pop    %rbx
  4bf11b:	pop    %r12
  4bf11d:	pop    %r13
  4bf11f:	pop    %r14
  4bf121:	pop    %r15
  4bf123:	pop    %rbp
  4bf124:	ret
  4bf125:	call   114d00 <std::process::abort>
  4bf12a:	mov    0x8(%r13),%rsi
  4bf12e:	mov    %rax,%r12
  4bf131:	test   %rsi,%rsi
  4bf134:	je     4bf150 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0xf0>
  4bf136:	mov    0x10(%r13),%rdx
  4bf13a:	mov    %r15,%rdi
  4bf13d:	call   eb3e0 <__rustc::__rust_dealloc>
  4bf142:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4bf150:	dec    %rbx
  4bf153:	je     4bf166 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0x106>
  4bf155:	mov    %r14,%rdi
  4bf158:	lea    0x28(%r14),%r15
  4bf15c:	call   4bf410 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>>>
  4bf161:	mov    %r15,%r14
  4bf164:	jmp    4bf150 <core::ptr::drop_glue::<[rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()>]>+0xf0>
  4bf166:	mov    %r12,%rdi
  4bf169:	call   4e5c10 <_Unwind_Resume@plt>
  4bf16e:	call   f5eb3 <core::panicking::panic_in_cleanup>
