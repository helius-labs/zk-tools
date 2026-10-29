
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000028f6f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>:
  28f6f0:	push   %rbp
  28f6f1:	push   %r15
  28f6f3:	push   %r14
  28f6f5:	push   %r13
  28f6f7:	push   %r12
  28f6f9:	push   %rbx
  28f6fa:	sub    $0x6d8,%rsp
  28f701:	mov    0x718(%rsp),%rbx
  28f709:	mov    0x710(%rsp),%rbp
  28f711:	mov    %rsi,0x28(%rsp)
  28f716:	shr    $1,%rsi
  28f719:	mov    %rcx,0x30(%rsp)
  28f71e:	mov    %r9,%r12
  28f721:	mov    %rdi,0x50(%rsp)
  28f726:	mov    %r8,0x38(%rsp)
  28f72b:	cmp    %r8,%rsi
  28f72e:	jae    28fe62 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x772>
  28f734:	mov    (%rbx),%rcx
  28f737:	mov    0x8(%rbx),%rsi
  28f73b:	mov    0x10(%rbx),%rdx
  28f73f:	movabs $0x7fffffffffffffc1,%rax
  28f749:	shl    $0x4,%rbp
  28f74d:	xor    %ebx,%ebx
  28f74f:	add    %r12,%rbp
  28f752:	add    $0x37,%rax
  28f756:	mov    %rax,0x78(%rsp)
  28f75b:	mov    %rbp,0x18(%rsp)
  28f760:	mov    %rcx,0x58(%rsp)
  28f765:	movabs $0x7fffffffffffffd,%rcx
  28f76f:	mov    %rsi,0x40(%rsp)
  28f774:	mov    %rdx,0x48(%rsp)
  28f779:	cmp    %rbp,%r12
  28f77c:	je     2900dd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9ed>
  28f782:	mov    0x58(%rsp),%rax
  28f787:	mov    (%r12),%r14
  28f78b:	mov    0x8(%r12),%r13
  28f790:	mov    0x8(%rax),%rdx
  28f794:	cmp    %r14,%r13
  28f797:	jb     2900f9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa09>
  28f79d:	cmp    %rdx,%r13
  28f7a0:	ja     2900f9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa09>
  28f7a6:	shl    $0x2,%r13
  28f7aa:	shl    $0x2,%r14
  28f7ae:	mov    %rbx,0x70(%rsp)
  28f7b3:	mov    %r13,%r15
  28f7b6:	sub    %r14,%r15
  28f7b9:	mov    %r15,%rbp
  28f7bc:	shl    $0x5,%rbp
  28f7c0:	cmp    %rcx,%r15
  28f7c3:	setb   %al
  28f7c6:	cmp    0x78(%rsp),%rbp
  28f7cb:	setbe  %cl
  28f7ce:	test   %cl,%al
  28f7d0:	je     28fe5a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x76a>
  28f7d6:	mov    0x58(%rsp),%rax
  28f7db:	mov    %r12,0x8(%rsp)
  28f7e0:	shr    $0x2,%r15
  28f7e4:	mov    (%rax),%r12
  28f7e7:	mov    0x10(%rax),%rbx
  28f7eb:	mov    0x18(%rax),%rax
  28f7ef:	add    %r14,%r12
  28f7f2:	mov    %rax,0x68(%rsp)
  28f7f7:	test   %rbp,%rbp
  28f7fa:	je     28f8d3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x1e3>
  28f800:	mov    $0x8,%edi
  28f805:	mov    %rbp,%rsi
  28f808:	xor    %edx,%edx
  28f80a:	vzeroupper
  28f80d:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  28f812:	mov    %r15,0x10(%rsp)
  28f817:	mov    %rax,0x20(%rsp)
  28f81c:	test   %rax,%rax
  28f81f:	je     2902e3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbf3>
  28f825:	cmp    %r14,%r13
  28f828:	jne    28f8ef <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x1ff>
  28f82e:	xor    %r15d,%r15d
  28f831:	lea    0x160(%rsp),%r13
  28f839:	lea    0x120(%rsp),%r14
  28f841:	mov    %r15,0x60(%rsp)
  28f846:	mov    $0x1d,%edx
  28f84b:	lea    0x80(%rsp),%rdi
  28f853:	lea    -0x205223(%rip),%rsi        # 8a637 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x22f>
  28f85a:	vzeroupper
  28f85d:	call   143720 <std::sys::env::unix::getenv>
  28f862:	lea    0x200(%rsp),%rbx
  28f86a:	mov    0x80(%rsp),%r12
  28f872:	cmp    $0xffffffffffffffff,%r12
  28f876:	je     28fa29 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x339>
  28f87c:	mov    0x88(%rsp),%rbx
  28f884:	mov    0x90(%rsp),%r14
  28f88c:	lea    0x80(%rsp),%rdi
  28f894:	mov    %rbx,%rsi
  28f897:	mov    %r14,%rdx
  28f89a:	call   f0a00 <core::str::converts::from_utf8>
  28f89f:	cmpb   $0x0,0x80(%rsp)
  28f8a7:	je     28fa33 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x343>
  28f8ad:	mov    $0x1,%bpl
  28f8b0:	test   %r12,%r12
  28f8b3:	jle    28fe19 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x729>
  28f8b9:	lea    0x120(%rsp),%r14
  28f8c1:	cmp    $0x1ffff,%r12
  28f8c8:	ja     28fa59 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x369>
  28f8ce:	jmp    28fa77 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x387>
  28f8d3:	mov    $0x8,%eax
  28f8d8:	movq   $0x0,0x10(%rsp)
  28f8e1:	mov    %rax,0x20(%rsp)
  28f8e6:	cmp    %r14,%r13
  28f8e9:	je     28f82e <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x13e>
  28f8ef:	mov    0x20(%rsp),%rax
  28f8f4:	mov    0x68(%rsp),%rsi
  28f8f9:	lea    0x160(%rsp),%r13
  28f901:	lea    0x120(%rsp),%r14
  28f909:	lea    0xc0(%rsp),%rdx
  28f911:	xor    %ecx,%ecx
  28f913:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  28f920:	mov    (%r12,%rcx,4),%edi
  28f924:	cmp    %rdi,%rsi
  28f927:	jbe    2902b8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbc8>
  28f92d:	shl    $0x7,%rdi
  28f931:	vmovups (%rbx,%rdi,1),%zmm0
  28f938:	vmovups 0x40(%rbx,%rdi,1),%zmm1
  28f940:	vmovups %zmm1,0x240(%rsp)
  28f948:	vmovups %zmm0,0x200(%rsp)
  28f950:	vmovdqu (%rbx,%rdi,1),%ymm0
  28f955:	vptest %ymm0,%ymm0
  28f95a:	jne    28f990 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x2a0>
  28f95c:	vmovdqu 0x220(%rsp),%ymm0
  28f965:	vptest %ymm0,%ymm0
  28f96a:	jne    28f990 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x2a0>
  28f96c:	vmovdqu 0x240(%rsp),%ymm0
  28f975:	vptest %ymm0,%ymm0
  28f97a:	jne    28f990 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x2a0>
  28f97c:	vmovdqu 0x260(%rsp),%ymm0
  28f985:	vptest %ymm0,%ymm0
  28f98a:	je     28fa13 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x323>
  28f990:	add    %rbx,%rdi
  28f993:	vmovups 0x20(%rdi),%ymm0
  28f998:	vmovups %ymm0,0x0(%r13)
  28f99e:	vmovups (%rdi),%ymm0
  28f9a2:	vmovups %ymm0,0x140(%rsp)
  28f9ab:	vmovups 0x60(%rdi),%ymm0
  28f9b0:	vmovups 0x140(%rsp),%zmm2
  28f9b8:	vmovups %ymm0,(%r14)
  28f9bd:	vmovdqu 0x40(%rdi),%ymm0
  28f9c2:	vmovups %zmm2,0x80(%rsp)
  28f9ca:	vmovdqu %ymm0,0x100(%rsp)
  28f9d3:	vmovups 0x100(%rsp),%zmm1
  28f9db:	vmovups %zmm1,(%rdx)
  28f9e1:	vmovdqu64 0x80(%rsp),%zmm0
  28f9e9:	vmovups 0xc0(%rsp),%zmm1
  28f9f1:	inc    %rcx
  28f9f4:	vmovups %zmm1,0x40(%rax)
  28f9fb:	vmovdqu64 %zmm0,(%rax)
  28fa01:	sub    $0xffffffffffffff80,%rax
  28fa05:	cmp    %rcx,%r15
  28fa08:	jne    28f920 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x230>
  28fa0e:	jmp    28f841 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x151>
  28fa13:	vpxor  %xmm0,%xmm0,%xmm0
  28fa17:	vmovdqu64 %zmm0,0xc0(%rsp)
  28fa1f:	vmovdqu64 %zmm0,0x80(%rsp)
  28fa27:	jmp    28f9e1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x2f1>
  28fa29:	mov    0x8(%rsp),%r12
  28fa2e:	mov    $0x1,%bpl
  28fa31:	jmp    28fa8d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x39d>
  28fa33:	mov    $0x1,%bpl
  28fa36:	cmp    $0x1,%r14
  28fa3a:	jne    28fa43 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x353>
  28fa3c:	cmpb   $0x30,(%rbx)
  28fa3f:	setne  %bpl
  28fa43:	lea    0x120(%rsp),%r14
  28fa4b:	test   %r12,%r12
  28fa4e:	je     28fa80 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x390>
  28fa50:	cmp    $0x1ffff,%r12
  28fa57:	jbe    28fa77 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x387>
  28fa59:	mov    %r12,%rsi
  28fa5c:	inc    %rsi
  28fa5f:	js     2902c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28fa65:	cmpb   $0x1,(%rbx,%r12,1)
  28fa6a:	jne    28fa77 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x387>
  28fa6c:	mov    %rbx,%rdi
  28fa6f:	call   *0x25b1d3(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28fa75:	jmp    28fa80 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x390>
  28fa77:	mov    %rbx,%rdi
  28fa7a:	call   *0x25b1d0(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28fa80:	mov    0x8(%rsp),%r12
  28fa85:	lea    0x200(%rsp),%rbx
  28fa8d:	lea    0x210(%rsp),%rax
  28fa95:	mov    $0x408,%edx
  28fa9a:	lea    0x240(%rsp),%rdi
  28faa2:	mov    %bpl,0x648(%rsp)
  28faaa:	movq   $0x0,0x200(%rsp)
  28fab6:	xor    %esi,%esi
  28fab8:	add    $0x10,%r12
  28fabc:	movq   $0x8,0x208(%rsp)
  28fac8:	vpxor  %xmm0,%xmm0,%xmm0
  28facc:	vmovdqu %xmm0,(%rax)
  28fad0:	movq   $0x40,0x220(%rsp)
  28fadc:	vmovdqu %xmm0,0x18(%rax)
  28fae1:	movq   $0x40,0x238(%rsp)
  28faed:	call   *0x25b1dd(%rip)        # 4eacd0 <memset@GLIBC_2.2.5>
  28faf3:	mov    0x18(%rsp),%rbp
  28faf8:	nopl   0x0(%rax,%rax,1)
  28fb00:	cmpq   $0x1,0x60(%rsp)
  28fb06:	jbe    28fb40 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x450>
  28fb08:	mov    0x20(%rsp),%rdi
  28fb0d:	mov    $0x1,%ecx
  28fb12:	mov    $0x1,%r9d
  28fb18:	lea    -0x284807(%rip),%rdx        # b318 <__abi_tag+0xb01c>
  28fb1f:	lea    0x60(%rsp),%r8
  28fb24:	mov    %r15,%rsi
  28fb27:	mov    %rbx,(%rsp)
  28fb2b:	call   22cb80 <zopro_core::msm::reduce_layer::<zopro_bn254::curve::G2Config>>
  28fb30:	jmp    28fb00 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x410>
  28fb32:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  28fb40:	test   %r15,%r15
  28fb43:	je     2902d1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbe1>
  28fb49:	mov    0x20(%rsp),%r15
  28fb4e:	mov    0x70(%rsp),%rbx
  28fb53:	vmovups (%r15),%zmm0
  28fb59:	vmovups 0x40(%r15),%zmm1
  28fb60:	vmovups %zmm1,0xc0(%rsp)
  28fb68:	vmovups %zmm0,0x80(%rsp)
  28fb70:	vmovdqu (%r15),%ymm0
  28fb75:	vptest %ymm0,%ymm0
  28fb7a:	jne    28fbb0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x4c0>
  28fb7c:	vmovdqu 0xa0(%rsp),%ymm0
  28fb85:	vptest %ymm0,%ymm0
  28fb8a:	jne    28fbb0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x4c0>
  28fb8c:	vmovdqu 0xc0(%rsp),%ymm0
  28fb95:	vptest %ymm0,%ymm0
  28fb9a:	jne    28fbb0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x4c0>
  28fb9c:	vmovdqu 0xe0(%rsp),%ymm0
  28fba5:	vptest %ymm0,%ymm0
  28fbaa:	je     28fe2b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x73b>
  28fbb0:	vmovups 0x20(%r15),%ymm0
  28fbb6:	lea    0x1c0(%rsp),%rax
  28fbbe:	vmovups %ymm0,0x0(%r13)
  28fbc4:	vmovups (%r15),%ymm0
  28fbc9:	vmovups %ymm0,0x140(%rsp)
  28fbd2:	vmovups 0x60(%r15),%ymm0
  28fbd8:	vmovups 0x140(%rsp),%zmm2
  28fbe0:	vmovups %ymm0,(%r14)
  28fbe5:	vmovdqu 0x40(%r15),%ymm0
  28fbeb:	vmovups %zmm2,0x180(%rsp)
  28fbf3:	vmovdqu %ymm0,0x100(%rsp)
  28fbfc:	vmovups 0x100(%rsp),%zmm1
  28fc04:	vmovups %zmm1,(%rax)
  28fc0a:	mov    0x10(%rsp),%r14
  28fc0f:	mov    0x200(%rsp),%rax
  28fc17:	test   %rax,%rax
  28fc1a:	je     28fc69 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x579>
  28fc1c:	mov    0x208(%rsp),%rdi
  28fc24:	cmp    $0x3fff,%rax
  28fc2a:	jbe    28fc60 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x570>
  28fc2c:	movabs $0x7fffffffffffffc1,%rcx
  28fc36:	shl    $0x3,%rax
  28fc3a:	mov    %rax,%rsi
  28fc3d:	or     $0x1,%rsi
  28fc41:	add    $0x38,%rcx
  28fc45:	cmp    %rcx,%rsi
  28fc48:	jae    2902c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28fc4e:	cmpb   $0x1,(%rdi,%rax,1)
  28fc52:	jne    28fc60 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x570>
  28fc54:	vzeroupper
  28fc57:	call   *0x25afeb(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28fc5d:	jmp    28fc69 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x579>
  28fc5f:	nop
  28fc60:	vzeroupper
  28fc63:	call   *0x25afe7(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28fc69:	mov    0x218(%rsp),%rax
  28fc71:	test   %rax,%rax
  28fc74:	je     28fcc0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x5d0>
  28fc76:	mov    0x220(%rsp),%rdi
  28fc7e:	cmp    $0x2d,%rax
  28fc82:	jbe    28fcd0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x5e0>
  28fc84:	imul   $0xb40,%rax,%rax
  28fc8b:	movabs $0x7fffffffffffffc1,%rcx
  28fc95:	mov    %rax,%rsi
  28fc98:	or     $0x1,%rsi
  28fc9c:	cmp    %rcx,%rsi
  28fc9f:	jae    2902c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28fca5:	cmpb   $0x1,(%rdi,%rax,1)
  28fca9:	jne    28fcd0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x5e0>
  28fcab:	vzeroupper
  28fcae:	call   *0x25af94(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28fcb4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  28fcc0:	mov    0x230(%rsp),%rax
  28fcc8:	test   %rax,%rax
  28fccb:	jne    28fce6 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x5f6>
  28fccd:	jmp    28fd30 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x640>
  28fccf:	nop
  28fcd0:	vzeroupper
  28fcd3:	call   *0x25af77(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28fcd9:	mov    0x230(%rsp),%rax
  28fce1:	test   %rax,%rax
  28fce4:	je     28fd30 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x640>
  28fce6:	mov    0x238(%rsp),%rdi
  28fcee:	cmp    $0xcc,%rax
  28fcf4:	jbe    28fd40 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x650>
  28fcf6:	shl    $0x7,%rax
  28fcfa:	movabs $0x7fffffffffffffc1,%rcx
  28fd04:	lea    (%rax,%rax,4),%rax
  28fd08:	mov    %rax,%rsi
  28fd0b:	or     $0x1,%rsi
  28fd0f:	cmp    %rcx,%rsi
  28fd12:	jae    2902c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28fd18:	cmpb   $0x1,(%rdi,%rax,1)
  28fd1c:	jne    28fd40 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x650>
  28fd1e:	vzeroupper
  28fd21:	call   *0x25af21(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28fd27:	nopw   0x0(%rax,%rax,1)
  28fd30:	test   %r14,%r14
  28fd33:	jne    28fd4e <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x65e>
  28fd35:	jmp    28fd9c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6ac>
  28fd37:	nopw   0x0(%rax,%rax,1)
  28fd40:	vzeroupper
  28fd43:	call   *0x25af07(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28fd49:	test   %r14,%r14
  28fd4c:	je     28fd9c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6ac>
  28fd4e:	cmp    $0x3ff,%r14
  28fd55:	jbe    28fd90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6a0>
  28fd57:	movabs $0x7fffffffffffffc1,%rax
  28fd61:	shl    $0x7,%r14
  28fd65:	mov    %r14,%rsi
  28fd68:	or     $0x1,%rsi
  28fd6c:	add    $0x38,%rax
  28fd70:	cmp    %rax,%rsi
  28fd73:	jae    2902c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28fd79:	cmpb   $0x1,(%r15,%r14,1)
  28fd7e:	jne    28fd90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6a0>
  28fd80:	mov    %r15,%rdi
  28fd83:	vzeroupper
  28fd86:	call   *0x25aebc(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28fd8c:	jmp    28fd9c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6ac>
  28fd8e:	xchg   %ax,%ax
  28fd90:	mov    %r15,%rdi
  28fd93:	vzeroupper
  28fd96:	call   *0x25aeb4(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28fd9c:	vmovups 0x180(%rsp),%zmm0
  28fda4:	vmovups 0x1c0(%rsp),%zmm1
  28fdac:	vmovups %zmm1,0x690(%rsp)
  28fdb7:	vmovups %zmm0,0x650(%rsp)
  28fdc2:	cmp    0x48(%rsp),%rbx
  28fdc7:	je     29010e <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa1e>
  28fdcd:	vmovdqu64 0x650(%rsp),%zmm0
  28fdd8:	vmovups 0x690(%rsp),%zmm1
  28fde3:	mov    0x40(%rsp),%rcx
  28fde8:	mov    %rbx,%rax
  28fdeb:	shl    $0x7,%rax
  28fdef:	inc    %rbx
  28fdf2:	vmovups %zmm1,0x40(%rcx,%rax,1)
  28fdfa:	vmovdqu64 %zmm0,(%rcx,%rax,1)
  28fe01:	movabs $0x7fffffffffffffd,%rcx
  28fe0b:	cmp    %rbp,%r12
  28fe0e:	jne    28f782 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x92>
  28fe14:	jmp    2900dd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9ed>
  28fe19:	mov    0x8(%rsp),%r12
  28fe1e:	lea    0x120(%rsp),%r14
  28fe26:	jmp    28fa85 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x395>
  28fe2b:	vpxor  %xmm0,%xmm0,%xmm0
  28fe2f:	vmovdqu64 %zmm0,0x1c0(%rsp)
  28fe37:	vmovdqu64 %zmm0,0x180(%rsp)
  28fe3f:	mov    0x10(%rsp),%r14
  28fe44:	mov    0x200(%rsp),%rax
  28fe4c:	test   %rax,%rax
  28fe4f:	jne    28fc1c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x52c>
  28fe55:	jmp    28fc69 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x579>
  28fe5a:	vzeroupper
  28fe5d:	call   edf50 <alloc::raw_vec::capacity_overflow>
  28fe62:	test   %dl,%dl
  28fe64:	je     28fe7c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x78c>
  28fe66:	mov    %fs:0xffffffffffffffc0,%rax
  28fe6f:	test   %rax,%rax
  28fe72:	je     28fe8a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x79a>
  28fe74:	add    $0x110,%rax
  28fe7a:	jmp    28fe9b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x7ab>
  28fe7c:	test   %rcx,%rcx
  28fe7f:	je     28f734 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x44>
  28fe85:	shr    $1,%rcx
  28fe88:	jmp    28feaf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x7bf>
  28fe8a:	mov    %rsi,%r14
  28fe8d:	mov    %rcx,%r15
  28fe90:	call   104a30 <rayon_core::registry::global_registry>
  28fe95:	mov    %r15,%rcx
  28fe98:	mov    %r14,%rsi
  28fe9b:	mov    (%rax),%rax
  28fe9e:	shr    $1,%rcx
  28fea1:	mov    0x208(%rax),%rax
  28fea8:	cmp    %rax,%rcx
  28feab:	cmovbe %rax,%rcx
  28feaf:	sub    %rsi,%rbp
  28feb2:	mov    %rcx,0x30(%rsp)
  28feb7:	mov    %rsi,0x180(%rsp)
  28febf:	jb     290129 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa39>
  28fec5:	mov    0x10(%rbx),%r15
  28fec9:	sub    %rsi,%r15
  28fecc:	jb     290141 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa51>
  28fed2:	mov    (%rbx),%r14
  28fed5:	mov    0x8(%rbx),%rbx
  28fed9:	mov    %fs:0xffffffffffffffc0,%rdx
  28fee2:	mov    %rsi,%r13
  28fee5:	shl    $0x4,%r13
  28fee9:	mov    %r12,%rdi
  28feec:	add    %r12,%r13
  28feef:	mov    %rsi,%r12
  28fef2:	shl    $0x7,%r12
  28fef6:	add    %rbx,%r12
  28fef9:	test   %rdx,%rdx
  28fefc:	je     28ff91 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x8a1>
  28ff02:	lea    0x28(%rsp),%rax
  28ff07:	lea    0x180(%rsp),%r8
  28ff0f:	lea    0x30(%rsp),%rcx
  28ff14:	mov    %rax,0x200(%rsp)
  28ff1c:	mov    %r8,0x208(%rsp)
  28ff24:	mov    %rcx,0x210(%rsp)
  28ff2c:	mov    %r13,0x218(%rsp)
  28ff34:	mov    %rbp,0x220(%rsp)
  28ff3c:	mov    %r14,0x228(%rsp)
  28ff44:	mov    %r12,0x230(%rsp)
  28ff4c:	mov    %r15,0x238(%rsp)
  28ff54:	mov    %r8,0x240(%rsp)
  28ff5c:	mov    %rcx,0x248(%rsp)
  28ff64:	mov    %rdi,0x250(%rsp)
  28ff6c:	mov    %rsi,0x258(%rsp)
  28ff74:	mov    %r14,0x260(%rsp)
  28ff7c:	mov    %rbx,0x268(%rsp)
  28ff84:	mov    %rsi,0x270(%rsp)
  28ff8c:	jmp    29005c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x96c>
  28ff91:	mov    %rbp,0x18(%rsp)
  28ff96:	mov    %rdi,0x8(%rsp)
  28ff9b:	mov    %rsi,%rbp
  28ff9e:	call   104a30 <rayon_core::registry::global_registry>
  28ffa3:	mov    (%rax),%rsi
  28ffa6:	mov    %fs:0xffffffffffffffc0,%rdx
  28ffaf:	test   %rdx,%rdx
  28ffb2:	je     290159 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa69>
  28ffb8:	lea    0x28(%rsp),%rcx
  28ffbd:	mov    %rbp,%rax
  28ffc0:	cmp    %rsi,0x110(%rdx)
  28ffc7:	jne    29020b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xb1b>
  28ffcd:	mov    0x18(%rsp),%rdi
  28ffd2:	mov    %rcx,0x200(%rsp)
  28ffda:	lea    0x180(%rsp),%rcx
  28ffe2:	lea    0x30(%rsp),%rsi
  28ffe7:	mov    %rcx,0x208(%rsp)
  28ffef:	mov    %rsi,0x210(%rsp)
  28fff7:	mov    %r13,0x218(%rsp)
  28ffff:	mov    %rdi,0x220(%rsp)
  290007:	mov    %r14,0x228(%rsp)
  29000f:	mov    %r12,0x230(%rsp)
  290017:	mov    %r15,0x238(%rsp)
  29001f:	mov    %rcx,0x240(%rsp)
  290027:	mov    %rsi,0x248(%rsp)
  29002f:	mov    0x8(%rsp),%rsi
  290034:	mov    %rsi,0x250(%rsp)
  29003c:	mov    %rax,0x258(%rsp)
  290044:	mov    %r14,0x260(%rsp)
  29004c:	mov    %rbx,0x268(%rsp)
  290054:	mov    %rax,0x270(%rsp)
  29005c:	lea    0x80(%rsp),%rdi
  290064:	lea    0x200(%rsp),%rsi
  29006c:	xor    %ecx,%ecx
  29006e:	call   290340 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}>
  290073:	mov    0x90(%rsp),%rcx
  29007b:	mov    0x80(%rsp),%rax
  290083:	mov    %rcx,%rdx
  290086:	shl    $0x7,%rdx
  29008a:	add    %rax,%rdx
  29008d:	cmp    0x98(%rsp),%rdx
  290095:	jne    2900a9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9b9>
  290097:	mov    0xa0(%rsp),%rdx
  29009f:	mov    0xa8(%rsp),%rsi
  2900a7:	jmp    2900ad <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9bd>
  2900a9:	xor    %edx,%edx
  2900ab:	xor    %esi,%esi
  2900ad:	mov    0x50(%rsp),%rdi
  2900b2:	add    0x88(%rsp),%rdx
  2900ba:	add    %rcx,%rsi
  2900bd:	mov    %rax,(%rdi)
  2900c0:	mov    %rdx,0x8(%rdi)
  2900c4:	mov    %rsi,0x10(%rdi)
  2900c8:	add    $0x6d8,%rsp
  2900cf:	pop    %rbx
  2900d0:	pop    %r12
  2900d2:	pop    %r13
  2900d4:	pop    %r14
  2900d6:	pop    %r15
  2900d8:	pop    %rbp
  2900d9:	vzeroupper
  2900dc:	ret
  2900dd:	mov    0x50(%rsp),%rax
  2900e2:	mov    0x40(%rsp),%rcx
  2900e7:	mov    0x48(%rsp),%rdx
  2900ec:	mov    %rcx,(%rax)
  2900ef:	mov    %rdx,0x8(%rax)
  2900f3:	mov    %rbx,0x10(%rax)
  2900f7:	jmp    2900c8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9d8>
  2900f9:	lea    0x256408(%rip),%rcx        # 4e6508 <__frame_dummy_init_array_entry+0x46a8>
  290100:	mov    %r14,%rdi
  290103:	mov    %r13,%rsi
  290106:	vzeroupper
  290109:	call   f0c00 <core::slice::index::slice_index_fail>
  29010e:	lea    -0x205b1b(%rip),%rdi        # 8a5fa <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1f2>
  290115:	lea    0x25991c(%rip),%rdx        # 4e9a38 <__frame_dummy_init_array_entry+0x7bd8>
  29011c:	mov    $0x45,%esi
  290121:	vzeroupper
  290124:	call   eec00 <core::panicking::panic_fmt>
  290129:	lea    -0x2705a6(%rip),%rdi        # 1fb8a <zmij::STATIC_DATA+0x558a>
  290130:	lea    0x259951(%rip),%rdx        # 4e9a88 <__frame_dummy_init_array_entry+0x7c28>
  290137:	mov    $0x13,%esi
  29013c:	call   eec00 <core::panicking::panic_fmt>
  290141:	lea    -0x205b6c(%rip),%rdi        # 8a5dc <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1d4>
  290148:	lea    0x2598b9(%rip),%rdx        # 4e9a08 <__frame_dummy_init_array_entry+0x7ba8>
  29014f:	mov    $0x1e,%esi
  290154:	call   f0cd0 <core::panicking::panic>
  290159:	mov    0x18(%rsp),%rdx
  29015e:	lea    0x28(%rsp),%rax
  290163:	lea    0x30(%rsp),%rcx
  290168:	sub    $0xffffffffffffff80,%rsi
  29016c:	lea    0x80(%rsp),%rdi
  290174:	mov    %rax,0x200(%rsp)
  29017c:	lea    0x180(%rsp),%rax
  290184:	mov    %rax,0x208(%rsp)
  29018c:	mov    %rcx,0x210(%rsp)
  290194:	mov    %r13,0x218(%rsp)
  29019c:	mov    %rdx,0x220(%rsp)
  2901a4:	mov    %r14,0x228(%rsp)
  2901ac:	mov    %r12,0x230(%rsp)
  2901b4:	mov    %r15,0x238(%rsp)
  2901bc:	mov    %rax,0x240(%rsp)
  2901c4:	mov    %rcx,0x248(%rsp)
  2901cc:	mov    0x8(%rsp),%rcx
  2901d1:	lea    0x200(%rsp),%rdx
  2901d9:	mov    %rcx,0x250(%rsp)
  2901e1:	mov    %rbp,0x258(%rsp)
  2901e9:	mov    %r14,0x260(%rsp)
  2901f1:	mov    %rbx,0x268(%rsp)
  2901f9:	mov    %rbp,0x270(%rsp)
  290201:	call   2907d0 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>>
  290206:	jmp    290073 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x983>
  29020b:	mov    0x18(%rsp),%r8
  290210:	mov    %rcx,0x200(%rsp)
  290218:	lea    0x180(%rsp),%rcx
  290220:	lea    0x30(%rsp),%rdi
  290225:	sub    $0xffffffffffffff80,%rsi
  290229:	mov    %rcx,0x208(%rsp)
  290231:	mov    %rdi,0x210(%rsp)
  290239:	mov    %r13,0x218(%rsp)
  290241:	mov    %r8,0x220(%rsp)
  290249:	mov    %r14,0x228(%rsp)
  290251:	mov    %r12,0x230(%rsp)
  290259:	mov    %r15,0x238(%rsp)
  290261:	mov    %rcx,0x240(%rsp)
  290269:	mov    %rdi,0x248(%rsp)
  290271:	mov    0x8(%rsp),%rdi
  290276:	lea    0x200(%rsp),%rcx
  29027e:	mov    %rdi,0x250(%rsp)
  290286:	lea    0x80(%rsp),%rdi
  29028e:	mov    %rax,0x258(%rsp)
  290296:	mov    %r14,0x260(%rsp)
  29029e:	mov    %rbx,0x268(%rsp)
  2902a6:	mov    %rax,0x270(%rsp)
  2902ae:	call   2908d0 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>>
  2902b3:	jmp    290073 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x983>
  2902b8:	lea    0x256e31(%rip),%rdx        # 4e70f0 <__frame_dummy_init_array_entry+0x5290>
  2902bf:	vzeroupper
  2902c2:	call   efcab <core::panicking::panic_bounds_check>
  2902c7:	jmp    2902e1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbf1>
  2902c9:	vzeroupper
  2902cc:	call   110970 <std::process::abort>
  2902d1:	lea    0x256218(%rip),%rdx        # 4e64f0 <__frame_dummy_init_array_entry+0x4690>
  2902d8:	xor    %edi,%edi
  2902da:	xor    %esi,%esi
  2902dc:	call   efcab <core::panicking::panic_bounds_check>
  2902e1:	ud2
  2902e3:	mov    $0x8,%edi
  2902e8:	mov    %rbp,%rsi
  2902eb:	call   ede21 <alloc::raw_vec::handle_error>
  2902f0:	jmp    2902f4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xc04>
  2902f2:	jmp    2902f9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xc09>
  2902f4:	mov    %rax,%rbx
  2902f7:	jmp    290309 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xc19>
  2902f9:	lea    0x200(%rsp),%rdi
  290301:	mov    %rax,%rbx
  290304:	call   2853b0 <core::ptr::drop_glue::<zopro_core::msm::Scratch<zopro_bn254::curve::G2Config>>>
  290309:	cmpq   $0x0,0x10(%rsp)
  29030f:	je     290329 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xc39>
  290311:	mov    0x10(%rsp),%rsi
  290316:	mov    0x20(%rsp),%rdi
  29031b:	mov    $0x8,%edx
  290320:	shl    $0x7,%rsi
  290324:	call   e9120 <__rustc::__rust_dealloc>
  290329:	mov    %rbx,%rdi
  29032c:	call   4e0e10 <_Unwind_Resume@plt>
