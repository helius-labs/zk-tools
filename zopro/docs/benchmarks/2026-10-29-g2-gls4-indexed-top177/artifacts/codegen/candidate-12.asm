
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000489c40 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute>:
  489c40:	push   %rbp
  489c41:	push   %r15
  489c43:	push   %r14
  489c45:	push   %r13
  489c47:	push   %r12
  489c49:	push   %rbx
  489c4a:	sub    $0x68,%rsp
  489c4e:	mov    (%rdi),%rdx
  489c51:	mov    0x8(%rdi),%rcx
  489c55:	mov    0x10(%rdi),%rax
  489c59:	movq   $0x0,(%rdi)
  489c60:	test   %rdx,%rdx
  489c63:	je     489ddd <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x19d>
  489c69:	mov    0x38(%rdi),%rsi
  489c6d:	mov    %rdi,%rbx
  489c70:	lea    0x50(%rsp),%r10
  489c75:	mov    %rsi,0x60(%rsp)
  489c7a:	vmovups 0x18(%rdi),%ymm0
  489c7f:	vmovups %ymm0,0x40(%rsp)
  489c85:	mov    %rdx,0x28(%rsp)
  489c8a:	mov    %rcx,0x30(%rsp)
  489c8f:	mov    %rax,0x38(%rsp)
  489c94:	mov    (%rdx),%rsi
  489c97:	mov    0x40(%rsp),%r9
  489c9c:	mov    0x8(%rax),%r8
  489ca0:	sub    (%rcx),%rsi
  489ca3:	mov    (%rax),%rcx
  489ca6:	mov    $0x1,%edx
  489cab:	lea    0x10(%rsp),%rdi
  489cb0:	push   %r10
  489cb2:	push   0x50(%rsp)
  489cb6:	vzeroupper
  489cb9:	call   467500 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  489cbe:	add    $0x10,%rsp
  489cc2:	mov    0x10(%rsp),%rax
  489cc7:	mov    0x18(%rsp),%r15
  489ccc:	mov    0x20(%rsp),%r13
  489cd1:	mov    $0x1,%ebp
  489cd6:	cmpl   $0x2,0x40(%rbx)
  489cda:	jb     489d69 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x129>
  489ce0:	mov    0x50(%rbx),%r14
  489ce4:	mov    %rax,0x8(%rsp)
  489ce9:	mov    0x48(%rbx),%r12
  489ced:	mov    (%r14),%rax
  489cf0:	test   %rax,%rax
  489cf3:	je     489cfa <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0xba>
  489cf5:	mov    %r12,%rdi
  489cf8:	call   *%rax
  489cfa:	mov    0x8(%r14),%rsi
  489cfe:	mov    0x8(%rsp),%rax
  489d03:	test   %rsi,%rsi
  489d06:	je     489d69 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x129>
  489d08:	mov    0x10(%r14),%rax
  489d0c:	cmp    $0x20000,%rsi
  489d13:	setae  %cl
  489d16:	cmp    $0x1001,%rax
  489d1c:	setb   %dl
  489d1f:	test   %cl,%dl
  489d21:	je     489d59 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x119>
  489d23:	blsr   %eax,%ecx
  489d28:	jne    489deb <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x1ab>
  489d2e:	movabs $0x8000000000000000,%rcx
  489d38:	sub    %rax,%rcx
  489d3b:	cmp    %rcx,%rsi
  489d3e:	jae    489deb <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x1ab>
  489d44:	cmpb   $0x1,(%r12,%rsi,1)
  489d49:	jne    489d5b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x11b>
  489d4b:	inc    %rsi
  489d4e:	mov    %r12,%rdi
  489d51:	call   *0x6b851(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  489d57:	jmp    489d64 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x124>
  489d59:	jmp    489d5b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x11b>
  489d5b:	mov    %r12,%rdi
  489d5e:	call   *0x6b84c(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  489d64:	mov    0x8(%rsp),%rax
  489d69:	mov    %rbp,0x40(%rbx)
  489d6d:	mov    %rax,0x48(%rbx)
  489d71:	mov    %r15,0x50(%rbx)
  489d75:	mov    %r13,0x58(%rbx)
  489d79:	mov    0x60(%rbx),%rax
  489d7d:	movzbl 0x78(%rbx),%ebp
  489d81:	mov    (%rax),%rdi
  489d84:	cmp    $0x1,%bpl
  489d88:	jne    489d9c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x15c>
  489d8a:	lock incq (%rdi)
  489d8e:	jle    489e5a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x21a>
  489d94:	mov    (%rax),%r14
  489d97:	mov    %r14,%rdi
  489d9a:	jmp    489d9c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x15c>
  489d9c:	mov    $0x3,%eax
  489da1:	mov    0x70(%rbx),%rsi
  489da5:	xchg   %rax,0x68(%rbx)
  489da9:	cmp    $0x2,%rax
  489dad:	jne    489dbb <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x17b>
  489daf:	add    $0x1d8,%rdi
  489db6:	call   100220 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  489dbb:	test   %bpl,%bpl
  489dbe:	je     489dce <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x18e>
  489dc0:	lock decq (%r14)
  489dc4:	jne    489dce <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x18e>
  489dc6:	mov    %r14,%rdi
  489dc9:	call   100fe0 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  489dce:	add    $0x68,%rsp
  489dd2:	pop    %rbx
  489dd3:	pop    %r12
  489dd5:	pop    %r13
  489dd7:	pop    %r14
  489dd9:	pop    %r15
  489ddb:	pop    %rbp
  489ddc:	ret
  489ddd:	lea    0x6a53c(%rip),%rdi        # 4f4320 <__frame_dummy_init_array_entry+0x7b00>
  489de4:	call   f2a60 <core::option::unwrap_failed>
  489de9:	jmp    489e5a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x21a>
  489deb:	call   1113d0 <std::process::abort>
  489df0:	mov    0x8(%r14),%rsi
  489df4:	test   %rsi,%rsi
  489df7:	je     489e05 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x1c5>
  489df9:	mov    0x10(%r14),%rdx
  489dfd:	mov    %r12,%rdi
  489e00:	call   e9b90 <__rustc::__rust_dealloc>
  489e05:	mov    0x8(%rsp),%rax
  489e0a:	mov    %rbp,0x40(%rbx)
  489e0e:	mov    %rax,0x48(%rbx)
  489e12:	mov    %r15,0x50(%rbx)
  489e16:	mov    %r13,0x58(%rbx)
  489e1a:	jmp    489e55 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  489e1c:	test   %bpl,%bpl
  489e1f:	je     489e55 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  489e21:	lock decq (%r14)
  489e25:	jne    489e55 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  489e27:	mov    %r14,%rdi
  489e2a:	call   100fe0 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  489e2f:	jmp    489e55 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  489e31:	call   f2bd3 <core::panicking::panic_in_cleanup>
  489e36:	mov    %rax,%rdi
  489e39:	call   1452ba <std::panicking::catch_unwind::cleanup>
  489e3e:	mov    $0x2,%ebp
  489e43:	mov    %rdx,%r15
  489e46:	cmpl   $0x2,0x40(%rbx)
  489e4a:	jae    489ce0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0xa0>
  489e50:	jmp    489d69 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x129>
  489e55:	call   105d30 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  489e5a:	ud2
  489e5c:	call   f2bd3 <core::panicking::panic_in_cleanup>
