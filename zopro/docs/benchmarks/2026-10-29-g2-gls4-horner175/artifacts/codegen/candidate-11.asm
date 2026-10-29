
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000485b50 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute>:
  485b50:	push   %rbp
  485b51:	push   %r15
  485b53:	push   %r14
  485b55:	push   %r13
  485b57:	push   %r12
  485b59:	push   %rbx
  485b5a:	sub    $0x68,%rsp
  485b5e:	mov    (%rdi),%rdx
  485b61:	mov    0x8(%rdi),%rcx
  485b65:	mov    0x10(%rdi),%rax
  485b69:	movq   $0x0,(%rdi)
  485b70:	test   %rdx,%rdx
  485b73:	je     485ceb <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x19b>
  485b79:	mov    0x38(%rdi),%r11
  485b7d:	mov    %rdi,%rbx
  485b80:	lea    0x40(%rsp),%r10
  485b85:	mov    %r11,0x60(%rsp)
  485b8a:	vmovups 0x18(%rdi),%ymm0
  485b8f:	vmovups %ymm0,0x40(%rsp)
  485b95:	mov    %rdx,0x28(%rsp)
  485b9a:	mov    %rcx,0x30(%rsp)
  485b9f:	mov    %rax,0x38(%rsp)
  485ba4:	mov    (%rdx),%rsi
  485ba7:	mov    0x58(%rsp),%r9
  485bac:	mov    0x8(%rax),%r8
  485bb0:	sub    (%rcx),%rsi
  485bb3:	mov    (%rax),%rcx
  485bb6:	mov    $0x1,%edx
  485bbb:	lea    0x10(%rsp),%rdi
  485bc0:	push   %r10
  485bc2:	push   %r11
  485bc4:	vzeroupper
  485bc7:	call   467ab0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>>
  485bcc:	add    $0x10,%rsp
  485bd0:	mov    0x10(%rsp),%rax
  485bd5:	mov    0x18(%rsp),%r15
  485bda:	mov    0x20(%rsp),%r13
  485bdf:	mov    $0x1,%ebp
  485be4:	cmpl   $0x2,0x40(%rbx)
  485be8:	jb     485c77 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x127>
  485bee:	mov    0x50(%rbx),%r14
  485bf2:	mov    %rax,0x8(%rsp)
  485bf7:	mov    0x48(%rbx),%r12
  485bfb:	mov    (%r14),%rax
  485bfe:	test   %rax,%rax
  485c01:	je     485c08 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0xb8>
  485c03:	mov    %r12,%rdi
  485c06:	call   *%rax
  485c08:	mov    0x8(%r14),%rsi
  485c0c:	mov    0x8(%rsp),%rax
  485c11:	test   %rsi,%rsi
  485c14:	je     485c77 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x127>
  485c16:	mov    0x10(%r14),%rax
  485c1a:	cmp    $0x20000,%rsi
  485c21:	setae  %cl
  485c24:	cmp    $0x1001,%rax
  485c2a:	setb   %dl
  485c2d:	test   %cl,%dl
  485c2f:	je     485c67 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x117>
  485c31:	blsr   %eax,%ecx
  485c36:	jne    485cf9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x1a9>
  485c3c:	movabs $0x8000000000000000,%rcx
  485c46:	sub    %rax,%rcx
  485c49:	cmp    %rcx,%rsi
  485c4c:	jae    485cf9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x1a9>
  485c52:	cmpb   $0x1,(%r12,%rsi,1)
  485c57:	jne    485c69 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x119>
  485c59:	inc    %rsi
  485c5c:	mov    %r12,%rdi
  485c5f:	call   *0x6c573(%rip)        # 4f21d8 <munmap@GLIBC_2.2.5>
  485c65:	jmp    485c72 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x122>
  485c67:	jmp    485c69 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x119>
  485c69:	mov    %r12,%rdi
  485c6c:	call   *0x6c56e(%rip)        # 4f21e0 <free@GLIBC_2.2.5>
  485c72:	mov    0x8(%rsp),%rax
  485c77:	mov    %rbp,0x40(%rbx)
  485c7b:	mov    %rax,0x48(%rbx)
  485c7f:	mov    %r15,0x50(%rbx)
  485c83:	mov    %r13,0x58(%rbx)
  485c87:	mov    0x60(%rbx),%rax
  485c8b:	movzbl 0x78(%rbx),%ebp
  485c8f:	mov    (%rax),%rdi
  485c92:	cmp    $0x1,%bpl
  485c96:	jne    485caa <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x15a>
  485c98:	lock incq (%rdi)
  485c9c:	jle    485d68 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x218>
  485ca2:	mov    (%rax),%r14
  485ca5:	mov    %r14,%rdi
  485ca8:	jmp    485caa <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x15a>
  485caa:	mov    $0x3,%eax
  485caf:	mov    0x70(%rbx),%rsi
  485cb3:	xchg   %rax,0x68(%rbx)
  485cb7:	cmp    $0x2,%rax
  485cbb:	jne    485cc9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x179>
  485cbd:	add    $0x1d8,%rdi
  485cc4:	call   100060 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  485cc9:	test   %bpl,%bpl
  485ccc:	je     485cdc <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x18c>
  485cce:	lock decq (%r14)
  485cd2:	jne    485cdc <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x18c>
  485cd4:	mov    %r14,%rdi
  485cd7:	call   100e20 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  485cdc:	add    $0x68,%rsp
  485ce0:	pop    %rbx
  485ce1:	pop    %r12
  485ce3:	pop    %r13
  485ce5:	pop    %r14
  485ce7:	pop    %r15
  485ce9:	pop    %rbp
  485cea:	ret
  485ceb:	lea    0x6b25e(%rip),%rdi        # 4f0f50 <__frame_dummy_init_array_entry+0x7aa0>
  485cf2:	call   f28a0 <core::option::unwrap_failed>
  485cf7:	jmp    485d68 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x218>
  485cf9:	call   111210 <std::process::abort>
  485cfe:	mov    0x8(%r14),%rsi
  485d02:	test   %rsi,%rsi
  485d05:	je     485d13 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x1c3>
  485d07:	mov    0x10(%r14),%rdx
  485d0b:	mov    %r12,%rdi
  485d0e:	call   e99d0 <__rustc::__rust_dealloc>
  485d13:	mov    0x8(%rsp),%rax
  485d18:	mov    %rbp,0x40(%rbx)
  485d1c:	mov    %rax,0x48(%rbx)
  485d20:	mov    %r15,0x50(%rbx)
  485d24:	mov    %r13,0x58(%rbx)
  485d28:	jmp    485d63 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x213>
  485d2a:	test   %bpl,%bpl
  485d2d:	je     485d63 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x213>
  485d2f:	lock decq (%r14)
  485d33:	jne    485d63 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x213>
  485d35:	mov    %r14,%rdi
  485d38:	call   100e20 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  485d3d:	jmp    485d63 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x213>
  485d3f:	call   f2a13 <core::panicking::panic_in_cleanup>
  485d44:	mov    %rax,%rdi
  485d47:	call   1450fa <std::panicking::catch_unwind::cleanup>
  485d4c:	mov    $0x2,%ebp
  485d51:	mov    %rdx,%r15
  485d54:	cmpl   $0x2,0x40(%rbx)
  485d58:	jae    485bee <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x9e>
  485d5e:	jmp    485c77 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x127>
  485d63:	call   105b70 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  485d68:	ud2
  485d6a:	call   f2a13 <core::panicking::panic_in_cleanup>
