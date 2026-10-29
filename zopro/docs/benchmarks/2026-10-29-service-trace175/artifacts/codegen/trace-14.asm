
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004bfe40 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute>:
  4bfe40:	push   %r15
  4bfe42:	push   %r14
  4bfe44:	push   %r13
  4bfe46:	push   %r12
  4bfe48:	push   %rbx
  4bfe49:	sub    $0x20,%rsp
  4bfe4d:	cmpq   $0x0,0x28(%rdi)
  4bfe52:	movq   $0x0,0x28(%rdi)
  4bfe5a:	je     4bff0c <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0xcc>
  4bfe60:	cmpq   $0x0,%fs:0xffffffffffffffb0
  4bfe6a:	mov    %rdi,%rbx
  4bfe6d:	je     4bff1a <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0xda>
  4bfe73:	mov    %rsp,%rdi
  4bfe76:	call   4c03c0 <zopro_core::service_trace::snapshot::{closure#0}>
  4bfe7b:	mov    (%rsp),%r13
  4bfe7f:	mov    0x8(%rsp),%r15
  4bfe84:	mov    0x10(%rsp),%r14
  4bfe89:	mov    0x18(%rsp),%r12
  4bfe8e:	mov    %rbx,%rdi
  4bfe91:	call   4c0090 <core::ptr::drop_glue::<rayon_core::job::JobResult<(alloc::vec::Vec<[u64; 10]>, u64)>>>
  4bfe96:	mov    %r13,(%rbx)
  4bfe99:	mov    %r15,0x8(%rbx)
  4bfe9d:	mov    %r14,0x10(%rbx)
  4bfea1:	mov    %r12,0x18(%rbx)
  4bfea5:	mov    0x20(%rbx),%rdi
  4bfea9:	lock decq (%rdi)
  4bfead:	jne    4bfefe <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0xbe>
  4bfeaf:	mov    0x8(%rdi),%rax
  4bfeb3:	test   %rax,%rax
  4bfeb6:	je     4bfef5 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0xb5>
  4bfeb8:	mov    0x18(%rdi),%rsi
  4bfebc:	lock incq (%rax)
  4bfec0:	jle    4bff5f <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0x11f>
  4bfec6:	mov    $0x3,%eax
  4bfecb:	mov    0x8(%rdi),%rbx
  4bfecf:	xchg   %rax,0x10(%rdi)
  4bfed3:	cmp    $0x2,%rax
  4bfed7:	jne    4bfee5 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0xa5>
  4bfed9:	lea    0x1d8(%rbx),%rdi
  4bfee0:	call   1038b0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  4bfee5:	lock decq (%rbx)
  4bfee9:	jne    4bfefe <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0xbe>
  4bfeeb:	mov    %rbx,%rdi
  4bfeee:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4bfef3:	jmp    4bfefe <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0xbe>
  4bfef5:	add    $0x10,%rdi
  4bfef9:	call   1c78c0 <<rayon_core::latch::LockLatch as rayon_core::latch::Latch>::set>
  4bfefe:	add    $0x20,%rsp
  4bff02:	pop    %rbx
  4bff03:	pop    %r12
  4bff05:	pop    %r13
  4bff07:	pop    %r14
  4bff09:	pop    %r15
  4bff0b:	ret
  4bff0c:	lea    0x2ebe5(%rip),%rdi        # 4eeaf8 <__frame_dummy_init_array_entry+0x7e70>
  4bff13:	call   f5cf0 <core::option::unwrap_failed>
  4bff18:	jmp    4bff5f <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0x11f>
  4bff1a:	lea    -0x4852f0(%rip),%rdi        # 3ac31 <zmij::STATIC_DATA+0x4d31>
  4bff21:	lea    0x29560(%rip),%rdx        # 4e9488 <__frame_dummy_init_array_entry+0x2800>
  4bff28:	mov    $0x2a,%esi
  4bff2d:	call   f49d0 <core::panicking::panic>
  4bff32:	jmp    4bff5f <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0x11f>
  4bff34:	lock decq (%rbx)
  4bff38:	jne    4bff5a <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0x11a>
  4bff3a:	mov    %rbx,%rdi
  4bff3d:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4bff42:	jmp    4bff5a <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0x11a>
  4bff44:	call   f5eb3 <core::panicking::panic_in_cleanup>
  4bff49:	mov    %r13,(%rbx)
  4bff4c:	mov    %r15,0x8(%rbx)
  4bff50:	mov    %r14,0x10(%rbx)
  4bff54:	mov    %r12,0x18(%rbx)
  4bff58:	jmp    4bff5a <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0x11a>
  4bff5a:	call   109660 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  4bff5f:	ud2
  4bff61:	mov    %rax,%rdi
  4bff64:	call   14883a <std::panicking::catch_unwind::cleanup>
  4bff69:	movabs $0x8000000000000002,%r13
  4bff73:	mov    %rax,%r15
  4bff76:	mov    %rdx,%r14
  4bff79:	jmp    4bfe8e <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)> as rayon_core::job::Job>::execute+0x4e>
  4bff7e:	call   f5eb3 <core::panicking::panic_in_cleanup>
