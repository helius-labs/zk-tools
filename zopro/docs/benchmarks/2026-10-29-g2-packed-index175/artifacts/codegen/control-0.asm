
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/control-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001fcc20 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>>:
  1fcc20:	push   %r15
  1fcc22:	push   %r14
  1fcc24:	push   %rbx
  1fcc25:	cmpq   $0x0,(%rdi)
  1fcc29:	je     1fcc3b <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>+0x1b>
  1fcc2b:	movq   $0x8,0x18(%rdi)
  1fcc33:	movq   $0x0,0x20(%rdi)
  1fcc3b:	cmpl   $0x2,0x40(%rdi)
  1fcc3f:	jb     1fccaa <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>+0x8a>
  1fcc41:	mov    0x50(%rdi),%r15
  1fcc45:	mov    0x48(%rdi),%rbx
  1fcc49:	mov    (%r15),%rax
  1fcc4c:	test   %rax,%rax
  1fcc4f:	je     1fcc56 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>+0x36>
  1fcc51:	mov    %rbx,%rdi
  1fcc54:	call   *%rax
  1fcc56:	mov    0x8(%r15),%rsi
  1fcc5a:	test   %rsi,%rsi
  1fcc5d:	je     1fccaa <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>+0x8a>
  1fcc5f:	mov    0x10(%r15),%rax
  1fcc63:	cmp    $0x20000,%rsi
  1fcc6a:	setae  %cl
  1fcc6d:	cmp    $0x1001,%rax
  1fcc73:	setb   %dl
  1fcc76:	test   %cl,%dl
  1fcc78:	je     1fccb0 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>+0x90>
  1fcc7a:	blsr   %eax,%ecx
  1fcc7f:	jne    1fccbe <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>+0x9e>
  1fcc81:	movabs $0x8000000000000000,%rcx
  1fcc8b:	sub    %rax,%rcx
  1fcc8e:	cmp    %rcx,%rsi
  1fcc91:	jae    1fccbe <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>+0x9e>
  1fcc93:	cmpb   $0x1,(%rbx,%rsi,1)
  1fcc97:	jne    1fccb0 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>+0x90>
  1fcc99:	inc    %rsi
  1fcc9c:	mov    %rbx,%rdi
  1fcc9f:	pop    %rbx
  1fcca0:	pop    %r14
  1fcca2:	pop    %r15
  1fcca4:	jmp    *0x2ea656(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1fccaa:	pop    %rbx
  1fccab:	pop    %r14
  1fccad:	pop    %r15
  1fccaf:	ret
  1fccb0:	mov    %rbx,%rdi
  1fccb3:	pop    %rbx
  1fccb4:	pop    %r14
  1fccb6:	pop    %r15
  1fccb8:	jmp    *0x2ea64a(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1fccbe:	call   110070 <std::process::abort>
  1fccc3:	mov    0x8(%r15),%rsi
  1fccc7:	mov    %rax,%r14
  1fccca:	test   %rsi,%rsi
  1fcccd:	je     1fccdb <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>>+0xbb>
  1fcccf:	mov    0x10(%r15),%rdx
  1fccd3:	mov    %rbx,%rdi
  1fccd6:	call   e8820 <__rustc::__rust_dealloc>
  1fccdb:	mov    %r14,%rdi
  1fccde:	call   4dd5d0 <_Unwind_Resume@plt>
