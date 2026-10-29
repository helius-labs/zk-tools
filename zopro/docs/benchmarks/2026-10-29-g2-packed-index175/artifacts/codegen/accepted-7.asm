
/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001fc9f0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute>:
  1fc9f0:	push   %rbp
  1fc9f1:	push   %r15
  1fc9f3:	push   %r14
  1fc9f5:	push   %r13
  1fc9f7:	push   %r12
  1fc9f9:	push   %rbx
  1fc9fa:	sub    $0x68,%rsp
  1fc9fe:	mov    (%rdi),%rdx
  1fca01:	mov    0x8(%rdi),%rcx
  1fca05:	mov    0x10(%rdi),%rax
  1fca09:	movq   $0x0,(%rdi)
  1fca10:	test   %rdx,%rdx
  1fca13:	je     1fcb8d <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x19d>
  1fca19:	mov    0x38(%rdi),%rsi
  1fca1d:	mov    %rdi,%rbx
  1fca20:	lea    0x50(%rsp),%r10
  1fca25:	mov    %rsi,0x60(%rsp)
  1fca2a:	vmovups 0x18(%rdi),%ymm0
  1fca2f:	vmovups %ymm0,0x40(%rsp)
  1fca35:	mov    %rdx,0x28(%rsp)
  1fca3a:	mov    %rcx,0x30(%rsp)
  1fca3f:	mov    %rax,0x38(%rsp)
  1fca44:	mov    (%rdx),%rsi
  1fca47:	mov    0x40(%rsp),%r9
  1fca4c:	mov    0x8(%rax),%r8
  1fca50:	sub    (%rcx),%rsi
  1fca53:	mov    (%rax),%rcx
  1fca56:	mov    $0x1,%edx
  1fca5b:	lea    0x10(%rsp),%rdi
  1fca60:	push   %r10
  1fca62:	push   0x50(%rsp)
  1fca66:	vzeroupper
  1fca69:	call   1f94a0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>>
  1fca6e:	add    $0x10,%rsp
  1fca72:	mov    0x10(%rsp),%rax
  1fca77:	mov    0x18(%rsp),%r15
  1fca7c:	mov    0x20(%rsp),%r13
  1fca81:	mov    $0x1,%ebp
  1fca86:	cmpl   $0x2,0x40(%rbx)
  1fca8a:	jb     1fcb19 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x129>
  1fca90:	mov    0x50(%rbx),%r14
  1fca94:	mov    %rax,0x8(%rsp)
  1fca99:	mov    0x48(%rbx),%r12
  1fca9d:	mov    (%r14),%rax
  1fcaa0:	test   %rax,%rax
  1fcaa3:	je     1fcaaa <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0xba>
  1fcaa5:	mov    %r12,%rdi
  1fcaa8:	call   *%rax
  1fcaaa:	mov    0x8(%r14),%rsi
  1fcaae:	mov    0x8(%rsp),%rax
  1fcab3:	test   %rsi,%rsi
  1fcab6:	je     1fcb19 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x129>
  1fcab8:	mov    0x10(%r14),%rax
  1fcabc:	cmp    $0x20000,%rsi
  1fcac3:	setae  %cl
  1fcac6:	cmp    $0x1001,%rax
  1fcacc:	setb   %dl
  1fcacf:	test   %cl,%dl
  1fcad1:	je     1fcb09 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x119>
  1fcad3:	blsr   %eax,%ecx
  1fcad8:	jne    1fcb9b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x1ab>
  1fcade:	movabs $0x8000000000000000,%rcx
  1fcae8:	sub    %rax,%rcx
  1fcaeb:	cmp    %rcx,%rsi
  1fcaee:	jae    1fcb9b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x1ab>
  1fcaf4:	cmpb   $0x1,(%r12,%rsi,1)
  1fcaf9:	jne    1fcb0b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x11b>
  1fcafb:	inc    %rsi
  1fcafe:	mov    %r12,%rdi
  1fcb01:	call   *0x2ea7f9(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1fcb07:	jmp    1fcb14 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x124>
  1fcb09:	jmp    1fcb0b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x11b>
  1fcb0b:	mov    %r12,%rdi
  1fcb0e:	call   *0x2ea7f4(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1fcb14:	mov    0x8(%rsp),%rax
  1fcb19:	mov    %rbp,0x40(%rbx)
  1fcb1d:	mov    %rax,0x48(%rbx)
  1fcb21:	mov    %r15,0x50(%rbx)
  1fcb25:	mov    %r13,0x58(%rbx)
  1fcb29:	mov    0x60(%rbx),%rax
  1fcb2d:	movzbl 0x78(%rbx),%ebp
  1fcb31:	mov    (%rax),%rdi
  1fcb34:	cmp    $0x1,%bpl
  1fcb38:	jne    1fcb4c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x15c>
  1fcb3a:	lock incq (%rdi)
  1fcb3e:	jle    1fcc0a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x21a>
  1fcb44:	mov    (%rax),%r14
  1fcb47:	mov    %r14,%rdi
  1fcb4a:	jmp    1fcb4c <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x15c>
  1fcb4c:	mov    $0x3,%eax
  1fcb51:	mov    0x70(%rbx),%rsi
  1fcb55:	xchg   %rax,0x68(%rbx)
  1fcb59:	cmp    $0x2,%rax
  1fcb5d:	jne    1fcb6b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x17b>
  1fcb5f:	add    $0x1d8,%rdi
  1fcb66:	call   feec0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  1fcb6b:	test   %bpl,%bpl
  1fcb6e:	je     1fcb7e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x18e>
  1fcb70:	lock decq (%r14)
  1fcb74:	jne    1fcb7e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x18e>
  1fcb76:	mov    %r14,%rdi
  1fcb79:	call   ffc80 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  1fcb7e:	add    $0x68,%rsp
  1fcb82:	pop    %rbx
  1fcb83:	pop    %r12
  1fcb85:	pop    %r13
  1fcb87:	pop    %r14
  1fcb89:	pop    %r15
  1fcb8b:	pop    %rbp
  1fcb8c:	ret
  1fcb8d:	lea    0x2e94e4(%rip),%rdi        # 4e6078 <__frame_dummy_init_array_entry+0x7a58>
  1fcb94:	call   f16f0 <core::option::unwrap_failed>
  1fcb99:	jmp    1fcc0a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x21a>
  1fcb9b:	call   110070 <std::process::abort>
  1fcba0:	mov    0x8(%r14),%rsi
  1fcba4:	test   %rsi,%rsi
  1fcba7:	je     1fcbb5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x1c5>
  1fcba9:	mov    0x10(%r14),%rdx
  1fcbad:	mov    %r12,%rdi
  1fcbb0:	call   e8820 <__rustc::__rust_dealloc>
  1fcbb5:	mov    0x8(%rsp),%rax
  1fcbba:	mov    %rbp,0x40(%rbx)
  1fcbbe:	mov    %rax,0x48(%rbx)
  1fcbc2:	mov    %r15,0x50(%rbx)
  1fcbc6:	mov    %r13,0x58(%rbx)
  1fcbca:	jmp    1fcc05 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x215>
  1fcbcc:	test   %bpl,%bpl
  1fcbcf:	je     1fcc05 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x215>
  1fcbd1:	lock decq (%r14)
  1fcbd5:	jne    1fcc05 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x215>
  1fcbd7:	mov    %r14,%rdi
  1fcbda:	call   ffc80 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  1fcbdf:	jmp    1fcc05 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x215>
  1fcbe1:	call   f1863 <core::panicking::panic_in_cleanup>
  1fcbe6:	mov    %rax,%rdi
  1fcbe9:	call   143f5a <std::panicking::catch_unwind::cleanup>
  1fcbee:	mov    $0x2,%ebp
  1fcbf3:	mov    %rdx,%r15
  1fcbf6:	cmpl   $0x2,0x40(%rbx)
  1fcbfa:	jae    1fca90 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0xa0>
  1fcc00:	jmp    1fcb19 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>> as rayon_core::job::Job>::execute+0x129>
  1fcc05:	call   1049d0 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  1fcc0a:	ud2
  1fcc0c:	call   f1863 <core::panicking::panic_in_cleanup>
