
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004bee90 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute>:
  4bee90:	push   %rbp
  4bee91:	push   %r15
  4bee93:	push   %r14
  4bee95:	push   %r13
  4bee97:	push   %r12
  4bee99:	push   %rbx
  4bee9a:	push   %rax
  4bee9b:	cmpq   $0x0,0x8(%rdi)
  4beea0:	movq   $0x0,0x8(%rdi)
  4beea8:	je     4befbe <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x12e>
  4beeae:	cmpq   $0x0,%fs:0xffffffffffffffb0
  4beeb8:	mov    %rdi,%rbx
  4beebb:	je     4befcc <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x13c>
  4beec1:	call   4bf220 <zopro_core::service_trace::clear>
  4beec6:	mov    $0x1,%r13d
  4beecc:	cmpl   $0x2,0x10(%rbx)
  4beed0:	jb     4bef4b <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xbb>
  4beed2:	mov    0x20(%rbx),%rbp
  4beed6:	mov    0x18(%rbx),%r12
  4beeda:	mov    0x0(%rbp),%rax
  4beede:	test   %rax,%rax
  4beee1:	je     4beee8 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x58>
  4beee3:	mov    %r12,%rdi
  4beee6:	call   *%rax
  4beee8:	mov    0x8(%rbp),%rsi
  4beeec:	test   %rsi,%rsi
  4beeef:	je     4bef4b <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xbb>
  4beef1:	mov    0x10(%rbp),%rax
  4beef5:	cmp    $0x20000,%rsi
  4beefc:	setae  %cl
  4beeff:	cmp    $0x1001,%rax
  4bef05:	setb   %dl
  4bef08:	test   %cl,%dl
  4bef0a:	je     4bef42 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xb2>
  4bef0c:	blsr   %eax,%ecx
  4bef11:	jne    4befe6 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x156>
  4bef17:	movabs $0x8000000000000000,%rcx
  4bef21:	sub    %rax,%rcx
  4bef24:	cmp    %rcx,%rsi
  4bef27:	jae    4befe6 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x156>
  4bef2d:	cmpb   $0x1,(%r12,%rsi,1)
  4bef32:	jne    4bef42 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xb2>
  4bef34:	inc    %rsi
  4bef37:	mov    %r12,%rdi
  4bef3a:	call   *0x30e08(%rip)        # 4efd48 <munmap@GLIBC_2.2.5>
  4bef40:	jmp    4bef4b <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xbb>
  4bef42:	mov    %r12,%rdi
  4bef45:	call   *0x30e05(%rip)        # 4efd50 <free@GLIBC_2.2.5>
  4bef4b:	mov    %r13,0x10(%rbx)
  4bef4f:	mov    %r15,0x18(%rbx)
  4bef53:	mov    %r14,0x20(%rbx)
  4bef57:	mov    (%rbx),%rdi
  4bef5a:	lock decq (%rdi)
  4bef5e:	jne    4befaf <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x11f>
  4bef60:	mov    0x8(%rdi),%rax
  4bef64:	test   %rax,%rax
  4bef67:	je     4befa6 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x116>
  4bef69:	mov    0x18(%rdi),%rsi
  4bef6d:	lock incq (%rax)
  4bef71:	jle    4bf028 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x198>
  4bef77:	mov    $0x3,%eax
  4bef7c:	mov    0x8(%rdi),%rbx
  4bef80:	xchg   %rax,0x10(%rdi)
  4bef84:	cmp    $0x2,%rax
  4bef88:	jne    4bef96 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x106>
  4bef8a:	lea    0x1d8(%rbx),%rdi
  4bef91:	call   1038b0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  4bef96:	lock decq (%rbx)
  4bef9a:	jne    4befaf <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x11f>
  4bef9c:	mov    %rbx,%rdi
  4bef9f:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4befa4:	jmp    4befaf <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x11f>
  4befa6:	add    $0x10,%rdi
  4befaa:	call   1c78c0 <<rayon_core::latch::LockLatch as rayon_core::latch::Latch>::set>
  4befaf:	add    $0x8,%rsp
  4befb3:	pop    %rbx
  4befb4:	pop    %r12
  4befb6:	pop    %r13
  4befb8:	pop    %r14
  4befba:	pop    %r15
  4befbc:	pop    %rbp
  4befbd:	ret
  4befbe:	lea    0x2fb33(%rip),%rdi        # 4eeaf8 <__frame_dummy_init_array_entry+0x7e70>
  4befc5:	call   f5cf0 <core::option::unwrap_failed>
  4befca:	jmp    4bf028 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x198>
  4befcc:	lea    -0x4843a2(%rip),%rdi        # 3ac31 <zmij::STATIC_DATA+0x4d31>
  4befd3:	lea    0x2a4ae(%rip),%rdx        # 4e9488 <__frame_dummy_init_array_entry+0x2800>
  4befda:	mov    $0x2a,%esi
  4befdf:	call   f49d0 <core::panicking::panic>
  4befe4:	jmp    4bf028 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x198>
  4befe6:	call   114d00 <std::process::abort>
  4befeb:	lock decq (%rbx)
  4befef:	jne    4bf023 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x193>
  4beff1:	mov    %rbx,%rdi
  4beff4:	call   104670 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  4beff9:	jmp    4bf023 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x193>
  4beffb:	call   f5eb3 <core::panicking::panic_in_cleanup>
  4bf000:	mov    0x8(%rbp),%rsi
  4bf004:	test   %rsi,%rsi
  4bf007:	je     4bf015 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x185>
  4bf009:	mov    0x10(%rbp),%rdx
  4bf00d:	mov    %r12,%rdi
  4bf010:	call   eb3e0 <__rustc::__rust_dealloc>
  4bf015:	mov    %r13,0x10(%rbx)
  4bf019:	mov    %r15,0x18(%rbx)
  4bf01d:	mov    %r14,0x20(%rbx)
  4bf021:	jmp    4bf023 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x193>
  4bf023:	call   109660 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  4bf028:	ud2
  4bf02a:	mov    %rax,%rdi
  4bf02d:	call   14883a <std::panicking::catch_unwind::cleanup>
  4bf032:	mov    $0x2,%r13d
  4bf038:	mov    %rax,%r15
  4bf03b:	mov    %rdx,%r14
  4bf03e:	cmpl   $0x2,0x10(%rbx)
  4bf042:	jae    4beed2 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0x42>
  4bf048:	jmp    4bef4b <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::reset::{closure#0}, ()>::{closure#0}, ()> as rayon_core::job::Job>::execute+0xbb>
  4bf04d:	call   f5eb3 <core::panicking::panic_in_cleanup>
