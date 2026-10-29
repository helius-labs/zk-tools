
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004c01b0 <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>>:
  4c01b0:	push   %rbp
  4c01b1:	push   %r15
  4c01b3:	push   %r14
  4c01b5:	push   %r13
  4c01b7:	push   %r12
  4c01b9:	push   %rbx
  4c01ba:	push   %rax
  4c01bb:	mov    (%rdi),%rax
  4c01be:	mov    0x8(%rdi),%r15
  4c01c2:	mov    0x10(%rdi),%r14
  4c01c6:	mov    %rax,(%rsp)
  4c01ca:	test   %r15,%r15
  4c01cd:	je     4c023e <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0x8e>
  4c01cf:	mov    (%rsp),%rax
  4c01d3:	mov    0x2fb6e(%rip),%rbp        # 4efd48 <munmap@GLIBC_2.2.5>
  4c01da:	mov    0x2fb6f(%rip),%rbx        # 4efd50 <free@GLIBC_2.2.5>
  4c01e1:	movabs $0x7ffffffffffffff9,%r12
  4c01eb:	lea    0x8(%rax),%r13
  4c01ef:	jmp    4c020b <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0x5b>
  4c01f1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4c0200:	call   *%rbx
  4c0202:	add    $0x20,%r13
  4c0206:	dec    %r15
  4c0209:	je     4c023e <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0x8e>
  4c020b:	mov    -0x8(%r13),%rax
  4c020f:	test   %rax,%rax
  4c0212:	je     4c0202 <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0x52>
  4c0214:	mov    0x0(%r13),%rdi
  4c0218:	cmp    $0x666,%rax
  4c021e:	jbe    4c0200 <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0x50>
  4c0220:	shl    $0x4,%rax
  4c0224:	lea    (%rax,%rax,4),%rax
  4c0228:	mov    %rax,%rsi
  4c022b:	or     $0x1,%rsi
  4c022f:	cmp    %r12,%rsi
  4c0232:	jae    4c02af <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0xff>
  4c0234:	cmpb   $0x1,(%rdi,%rax,1)
  4c0238:	jne    4c0200 <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0x50>
  4c023a:	call   *%rbp
  4c023c:	jmp    4c0202 <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0x52>
  4c023e:	test   %r14,%r14
  4c0241:	je     4c0288 <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0xd8>
  4c0243:	cmp    $0xaaa,%r14
  4c024a:	jbe    4c0297 <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0xe7>
  4c024c:	shl    $0x4,%r14
  4c0250:	movabs $0x7ffffffffffffff9,%rcx
  4c025a:	lea    (%r14,%r14,2),%rax
  4c025e:	mov    %rax,%rsi
  4c0261:	or     $0x1,%rsi
  4c0265:	cmp    %rcx,%rsi
  4c0268:	jae    4c02af <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0xff>
  4c026a:	mov    (%rsp),%rdi
  4c026e:	cmpb   $0x1,(%rdi,%rax,1)
  4c0272:	jne    4c029b <core::ptr::drop_glue::<alloc::vec::in_place_drop::InPlaceDstDataSrcBufDrop<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>, (alloc::vec::Vec<[u64; 10]>, u64)>>+0xeb>
  4c0274:	add    $0x8,%rsp
  4c0278:	pop    %rbx
  4c0279:	pop    %r12
  4c027b:	pop    %r13
  4c027d:	pop    %r14
  4c027f:	pop    %r15
  4c0281:	pop    %rbp
  4c0282:	jmp    *0x2fac0(%rip)        # 4efd48 <munmap@GLIBC_2.2.5>
  4c0288:	add    $0x8,%rsp
  4c028c:	pop    %rbx
  4c028d:	pop    %r12
  4c028f:	pop    %r13
  4c0291:	pop    %r14
  4c0293:	pop    %r15
  4c0295:	pop    %rbp
  4c0296:	ret
  4c0297:	mov    (%rsp),%rdi
  4c029b:	add    $0x8,%rsp
  4c029f:	pop    %rbx
  4c02a0:	pop    %r12
  4c02a2:	pop    %r13
  4c02a4:	pop    %r14
  4c02a6:	pop    %r15
  4c02a8:	pop    %rbp
  4c02a9:	jmp    *0x2faa1(%rip)        # 4efd50 <free@GLIBC_2.2.5>
  4c02af:	call   114d00 <std::process::abort>
