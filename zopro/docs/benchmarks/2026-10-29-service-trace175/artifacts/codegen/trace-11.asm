
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004bff90 <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>>:
  4bff90:	push   %r15
  4bff92:	push   %r14
  4bff94:	push   %r13
  4bff96:	push   %r12
  4bff98:	push   %rbx
  4bff99:	mov    %rdi,%rbx
  4bff9c:	mov    0x8(%rdi),%rdi
  4bffa0:	mov    0x18(%rbx),%rax
  4bffa4:	movabs $0xaaaaaaaaaaaaaaab,%r12
  4bffae:	sub    %rdi,%rax
  4bffb1:	shr    $0x4,%rax
  4bffb5:	imul   %rax,%r12
  4bffb9:	inc    %r12
  4bffbc:	nopl   0x0(%rax)
  4bffc0:	cmp    $0x1,%r12
  4bffc4:	je     4bffd7 <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>+0x47>
  4bffc6:	lea    0x30(%rdi),%r15
  4bffca:	dec    %r12
  4bffcd:	call   4c0090 <core::ptr::drop_glue::<rayon_core::job::JobResult<(alloc::vec::Vec<[u64; 10]>, u64)>>>
  4bffd2:	mov    %r15,%rdi
  4bffd5:	jmp    4bffc0 <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>+0x30>
  4bffd7:	mov    0x10(%rbx),%rax
  4bffdb:	test   %rax,%rax
  4bffde:	je     4c002d <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>+0x9d>
  4bffe0:	mov    (%rbx),%rdi
  4bffe3:	cmp    $0xaaa,%rax
  4bffe9:	jbe    4c001e <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>+0x8e>
  4bffeb:	shl    $0x4,%rax
  4bffef:	movabs $0x7ffffffffffffff8,%rcx
  4bfff9:	lea    (%rax,%rax,2),%rax
  4bfffd:	mov    %rax,%rsi
  4c0000:	or     $0x1,%rsi
  4c0004:	cmp    %rcx,%rsi
  4c0007:	ja     4c0037 <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>+0xa7>
  4c0009:	cmpb   $0x1,(%rdi,%rax,1)
  4c000d:	jne    4c001e <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>+0x8e>
  4c000f:	pop    %rbx
  4c0010:	pop    %r12
  4c0012:	pop    %r13
  4c0014:	pop    %r14
  4c0016:	pop    %r15
  4c0018:	jmp    *0x2fd2a(%rip)        # 4efd48 <munmap@GLIBC_2.2.5>
  4c001e:	pop    %rbx
  4c001f:	pop    %r12
  4c0021:	pop    %r13
  4c0023:	pop    %r14
  4c0025:	pop    %r15
  4c0027:	jmp    *0x2fd23(%rip)        # 4efd50 <free@GLIBC_2.2.5>
  4c002d:	pop    %rbx
  4c002e:	pop    %r12
  4c0030:	pop    %r13
  4c0032:	pop    %r14
  4c0034:	pop    %r15
  4c0036:	ret
  4c0037:	call   114d00 <std::process::abort>
  4c003c:	mov    %rax,%r14
  4c003f:	nop
  4c0040:	dec    %r12
  4c0043:	je     4c0056 <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>+0xc6>
  4c0045:	mov    %r15,%rdi
  4c0048:	lea    0x30(%r15),%r13
  4c004c:	call   4c0090 <core::ptr::drop_glue::<rayon_core::job::JobResult<(alloc::vec::Vec<[u64; 10]>, u64)>>>
  4c0051:	mov    %r13,%r15
  4c0054:	jmp    4c0040 <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>+0xb0>
  4c0056:	mov    0x10(%rbx),%rax
  4c005a:	test   %rax,%rax
  4c005d:	je     4c0074 <core::ptr::drop_glue::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::CountLatch>, &rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#0}, (alloc::vec::Vec<[u64; 10]>, u64)>>, rayon_core::broadcast::broadcast_in<zopro_core::service_trace::snapshot::{closure#1}, (alloc::vec::Vec<[u64; 10]>, u64)>::{closure#3}>>+0xe4>
  4c005f:	mov    (%rbx),%rdi
  4c0062:	shl    $0x4,%rax
  4c0066:	mov    $0x8,%edx
  4c006b:	lea    (%rax,%rax,2),%rsi
  4c006f:	call   eb3e0 <__rustc::__rust_dealloc>
  4c0074:	mov    %r14,%rdi
  4c0077:	call   4e5c10 <_Unwind_Resume@plt>
  4c007c:	call   f5eb3 <core::panicking::panic_in_cleanup>
