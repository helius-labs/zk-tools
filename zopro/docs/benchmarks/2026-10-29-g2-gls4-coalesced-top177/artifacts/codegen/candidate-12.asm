
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000489d80 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute>:
  489d80:	push   %rbp
  489d81:	push   %r15
  489d83:	push   %r14
  489d85:	push   %r13
  489d87:	push   %r12
  489d89:	push   %rbx
  489d8a:	sub    $0x68,%rsp
  489d8e:	mov    (%rdi),%rdx
  489d91:	mov    0x8(%rdi),%rcx
  489d95:	mov    0x10(%rdi),%rax
  489d99:	movq   $0x0,(%rdi)
  489da0:	test   %rdx,%rdx
  489da3:	je     489f1d <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x19d>
  489da9:	mov    0x38(%rdi),%rsi
  489dad:	mov    %rdi,%rbx
  489db0:	lea    0x50(%rsp),%r10
  489db5:	mov    %rsi,0x60(%rsp)
  489dba:	vmovups 0x18(%rdi),%ymm0
  489dbf:	vmovups %ymm0,0x40(%rsp)
  489dc5:	mov    %rdx,0x28(%rsp)
  489dca:	mov    %rcx,0x30(%rsp)
  489dcf:	mov    %rax,0x38(%rsp)
  489dd4:	mov    (%rdx),%rsi
  489dd7:	mov    0x40(%rsp),%r9
  489ddc:	mov    0x8(%rax),%r8
  489de0:	sub    (%rcx),%rsi
  489de3:	mov    (%rax),%rcx
  489de6:	mov    $0x1,%edx
  489deb:	lea    0x10(%rsp),%rdi
  489df0:	push   %r10
  489df2:	push   0x50(%rsp)
  489df6:	vzeroupper
  489df9:	call   4675f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  489dfe:	add    $0x10,%rsp
  489e02:	mov    0x10(%rsp),%rax
  489e07:	mov    0x18(%rsp),%r15
  489e0c:	mov    0x20(%rsp),%r13
  489e11:	mov    $0x1,%ebp
  489e16:	cmpl   $0x2,0x40(%rbx)
  489e1a:	jb     489ea9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x129>
  489e20:	mov    0x50(%rbx),%r14
  489e24:	mov    %rax,0x8(%rsp)
  489e29:	mov    0x48(%rbx),%r12
  489e2d:	mov    (%r14),%rax
  489e30:	test   %rax,%rax
  489e33:	je     489e3a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0xba>
  489e35:	mov    %r12,%rdi
  489e38:	call   *%rax
  489e3a:	mov    0x8(%r14),%rsi
  489e3e:	mov    0x8(%rsp),%rax
  489e43:	test   %rsi,%rsi
  489e46:	je     489ea9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x129>
  489e48:	mov    0x10(%r14),%rax
  489e4c:	cmp    $0x20000,%rsi
  489e53:	setae  %cl
  489e56:	cmp    $0x1001,%rax
  489e5c:	setb   %dl
  489e5f:	test   %cl,%dl
  489e61:	je     489e99 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x119>
  489e63:	blsr   %eax,%ecx
  489e68:	jne    489f2b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x1ab>
  489e6e:	movabs $0x8000000000000000,%rcx
  489e78:	sub    %rax,%rcx
  489e7b:	cmp    %rcx,%rsi
  489e7e:	jae    489f2b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x1ab>
  489e84:	cmpb   $0x1,(%r12,%rsi,1)
  489e89:	jne    489e9b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x11b>
  489e8b:	inc    %rsi
  489e8e:	mov    %r12,%rdi
  489e91:	call   *0x6b851(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  489e97:	jmp    489ea4 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x124>
  489e99:	jmp    489e9b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x11b>
  489e9b:	mov    %r12,%rdi
  489e9e:	call   *0x6b84c(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  489ea4:	mov    0x8(%rsp),%rax
  489ea9:	mov    %rbp,0x40(%rbx)
  489ead:	mov    %rax,0x48(%rbx)
  489eb1:	mov    %r15,0x50(%rbx)
  489eb5:	mov    %r13,0x58(%rbx)
  489eb9:	mov    0x60(%rbx),%rax
  489ebd:	movzbl 0x78(%rbx),%ebp
  489ec1:	mov    (%rax),%rdi
  489ec4:	cmp    $0x1,%bpl
  489ec8:	jne    489edc <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x15c>
  489eca:	lock incq (%rdi)
  489ece:	jle    489f9a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x21a>
  489ed4:	mov    (%rax),%r14
  489ed7:	mov    %r14,%rdi
  489eda:	jmp    489edc <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x15c>
  489edc:	mov    $0x3,%eax
  489ee1:	mov    0x70(%rbx),%rsi
  489ee5:	xchg   %rax,0x68(%rbx)
  489ee9:	cmp    $0x2,%rax
  489eed:	jne    489efb <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x17b>
  489eef:	add    $0x1d8,%rdi
  489ef6:	call   1002e0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  489efb:	test   %bpl,%bpl
  489efe:	je     489f0e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x18e>
  489f00:	lock decq (%r14)
  489f04:	jne    489f0e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x18e>
  489f06:	mov    %r14,%rdi
  489f09:	call   1010a0 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  489f0e:	add    $0x68,%rsp
  489f12:	pop    %rbx
  489f13:	pop    %r12
  489f15:	pop    %r13
  489f17:	pop    %r14
  489f19:	pop    %r15
  489f1b:	pop    %rbp
  489f1c:	ret
  489f1d:	lea    0x6a53c(%rip),%rdi        # 4f4460 <__frame_dummy_init_array_entry+0x7b00>
  489f24:	call   f2b20 <core::option::unwrap_failed>
  489f29:	jmp    489f9a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x21a>
  489f2b:	call   111490 <std::process::abort>
  489f30:	mov    0x8(%r14),%rsi
  489f34:	test   %rsi,%rsi
  489f37:	je     489f45 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x1c5>
  489f39:	mov    0x10(%r14),%rdx
  489f3d:	mov    %r12,%rdi
  489f40:	call   e9c50 <__rustc::__rust_dealloc>
  489f45:	mov    0x8(%rsp),%rax
  489f4a:	mov    %rbp,0x40(%rbx)
  489f4e:	mov    %rax,0x48(%rbx)
  489f52:	mov    %r15,0x50(%rbx)
  489f56:	mov    %r13,0x58(%rbx)
  489f5a:	jmp    489f95 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  489f5c:	test   %bpl,%bpl
  489f5f:	je     489f95 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  489f61:	lock decq (%r14)
  489f65:	jne    489f95 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  489f67:	mov    %r14,%rdi
  489f6a:	call   1010a0 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  489f6f:	jmp    489f95 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  489f71:	call   f2c93 <core::panicking::panic_in_cleanup>
  489f76:	mov    %rax,%rdi
  489f79:	call   14537a <std::panicking::catch_unwind::cleanup>
  489f7e:	mov    $0x2,%ebp
  489f83:	mov    %rdx,%r15
  489f86:	cmpl   $0x2,0x40(%rbx)
  489f8a:	jae    489e20 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0xa0>
  489f90:	jmp    489ea9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x129>
  489f95:	call   105df0 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  489f9a:	ud2
  489f9c:	call   f2c93 <core::panicking::panic_in_cleanup>
