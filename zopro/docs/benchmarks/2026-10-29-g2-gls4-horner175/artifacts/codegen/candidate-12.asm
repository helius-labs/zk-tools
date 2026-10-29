
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004868d0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute>:
  4868d0:	push   %rbp
  4868d1:	push   %r15
  4868d3:	push   %r14
  4868d5:	push   %r13
  4868d7:	push   %r12
  4868d9:	push   %rbx
  4868da:	sub    $0x68,%rsp
  4868de:	mov    (%rdi),%rdx
  4868e1:	mov    0x8(%rdi),%rcx
  4868e5:	mov    0x10(%rdi),%rax
  4868e9:	movq   $0x0,(%rdi)
  4868f0:	test   %rdx,%rdx
  4868f3:	je     486a6d <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x19d>
  4868f9:	mov    0x38(%rdi),%rsi
  4868fd:	mov    %rdi,%rbx
  486900:	lea    0x50(%rsp),%r10
  486905:	mov    %rsi,0x60(%rsp)
  48690a:	vmovups 0x18(%rdi),%ymm0
  48690f:	vmovups %ymm0,0x40(%rsp)
  486915:	mov    %rdx,0x28(%rsp)
  48691a:	mov    %rcx,0x30(%rsp)
  48691f:	mov    %rax,0x38(%rsp)
  486924:	mov    (%rdx),%rsi
  486927:	mov    0x40(%rsp),%r9
  48692c:	mov    0x8(%rax),%r8
  486930:	sub    (%rcx),%rsi
  486933:	mov    (%rax),%rcx
  486936:	mov    $0x1,%edx
  48693b:	lea    0x10(%rsp),%rdi
  486940:	push   %r10
  486942:	push   0x50(%rsp)
  486946:	vzeroupper
  486949:	call   4672d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  48694e:	add    $0x10,%rsp
  486952:	mov    0x10(%rsp),%rax
  486957:	mov    0x18(%rsp),%r15
  48695c:	mov    0x20(%rsp),%r13
  486961:	mov    $0x1,%ebp
  486966:	cmpl   $0x2,0x40(%rbx)
  48696a:	jb     4869f9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x129>
  486970:	mov    0x50(%rbx),%r14
  486974:	mov    %rax,0x8(%rsp)
  486979:	mov    0x48(%rbx),%r12
  48697d:	mov    (%r14),%rax
  486980:	test   %rax,%rax
  486983:	je     48698a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0xba>
  486985:	mov    %r12,%rdi
  486988:	call   *%rax
  48698a:	mov    0x8(%r14),%rsi
  48698e:	mov    0x8(%rsp),%rax
  486993:	test   %rsi,%rsi
  486996:	je     4869f9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x129>
  486998:	mov    0x10(%r14),%rax
  48699c:	cmp    $0x20000,%rsi
  4869a3:	setae  %cl
  4869a6:	cmp    $0x1001,%rax
  4869ac:	setb   %dl
  4869af:	test   %cl,%dl
  4869b1:	je     4869e9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x119>
  4869b3:	blsr   %eax,%ecx
  4869b8:	jne    486a7b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x1ab>
  4869be:	movabs $0x8000000000000000,%rcx
  4869c8:	sub    %rax,%rcx
  4869cb:	cmp    %rcx,%rsi
  4869ce:	jae    486a7b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x1ab>
  4869d4:	cmpb   $0x1,(%r12,%rsi,1)
  4869d9:	jne    4869eb <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x11b>
  4869db:	inc    %rsi
  4869de:	mov    %r12,%rdi
  4869e1:	call   *0x6b7f1(%rip)        # 4f21d8 <munmap@GLIBC_2.2.5>
  4869e7:	jmp    4869f4 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x124>
  4869e9:	jmp    4869eb <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x11b>
  4869eb:	mov    %r12,%rdi
  4869ee:	call   *0x6b7ec(%rip)        # 4f21e0 <free@GLIBC_2.2.5>
  4869f4:	mov    0x8(%rsp),%rax
  4869f9:	mov    %rbp,0x40(%rbx)
  4869fd:	mov    %rax,0x48(%rbx)
  486a01:	mov    %r15,0x50(%rbx)
  486a05:	mov    %r13,0x58(%rbx)
  486a09:	mov    0x60(%rbx),%rax
  486a0d:	movzbl 0x78(%rbx),%ebp
  486a11:	mov    (%rax),%rdi
  486a14:	cmp    $0x1,%bpl
  486a18:	jne    486a2c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x15c>
  486a1a:	lock incq (%rdi)
  486a1e:	jle    486aea <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x21a>
  486a24:	mov    (%rax),%r14
  486a27:	mov    %r14,%rdi
  486a2a:	jmp    486a2c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x15c>
  486a2c:	mov    $0x3,%eax
  486a31:	mov    0x70(%rbx),%rsi
  486a35:	xchg   %rax,0x68(%rbx)
  486a39:	cmp    $0x2,%rax
  486a3d:	jne    486a4b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x17b>
  486a3f:	add    $0x1d8,%rdi
  486a46:	call   100060 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  486a4b:	test   %bpl,%bpl
  486a4e:	je     486a5e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x18e>
  486a50:	lock decq (%r14)
  486a54:	jne    486a5e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x18e>
  486a56:	mov    %r14,%rdi
  486a59:	call   100e20 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  486a5e:	add    $0x68,%rsp
  486a62:	pop    %rbx
  486a63:	pop    %r12
  486a65:	pop    %r13
  486a67:	pop    %r14
  486a69:	pop    %r15
  486a6b:	pop    %rbp
  486a6c:	ret
  486a6d:	lea    0x6a4dc(%rip),%rdi        # 4f0f50 <__frame_dummy_init_array_entry+0x7aa0>
  486a74:	call   f28a0 <core::option::unwrap_failed>
  486a79:	jmp    486aea <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x21a>
  486a7b:	call   111210 <std::process::abort>
  486a80:	mov    0x8(%r14),%rsi
  486a84:	test   %rsi,%rsi
  486a87:	je     486a95 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x1c5>
  486a89:	mov    0x10(%r14),%rdx
  486a8d:	mov    %r12,%rdi
  486a90:	call   e99d0 <__rustc::__rust_dealloc>
  486a95:	mov    0x8(%rsp),%rax
  486a9a:	mov    %rbp,0x40(%rbx)
  486a9e:	mov    %rax,0x48(%rbx)
  486aa2:	mov    %r15,0x50(%rbx)
  486aa6:	mov    %r13,0x58(%rbx)
  486aaa:	jmp    486ae5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  486aac:	test   %bpl,%bpl
  486aaf:	je     486ae5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  486ab1:	lock decq (%r14)
  486ab5:	jne    486ae5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  486ab7:	mov    %r14,%rdi
  486aba:	call   100e20 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  486abf:	jmp    486ae5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x215>
  486ac1:	call   f2a13 <core::panicking::panic_in_cleanup>
  486ac6:	mov    %rax,%rdi
  486ac9:	call   1450fa <std::panicking::catch_unwind::cleanup>
  486ace:	mov    $0x2,%ebp
  486ad3:	mov    %rdx,%r15
  486ad6:	cmpl   $0x2,0x40(%rbx)
  486ada:	jae    486970 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0xa0>
  486ae0:	jmp    4869f9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute+0x129>
  486ae5:	call   105b70 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  486aea:	ud2
  486aec:	call   f2a13 <core::panicking::panic_in_cleanup>
