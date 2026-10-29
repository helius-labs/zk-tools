
/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001f94a0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>>:
  1f94a0:	push   %rbp
  1f94a1:	push   %r15
  1f94a3:	push   %r14
  1f94a5:	push   %r13
  1f94a7:	push   %r12
  1f94a9:	push   %rbx
  1f94aa:	sub    $0x3a8,%rsp
  1f94b1:	mov    0x3e8(%rsp),%rbx
  1f94b9:	mov    0x3e0(%rsp),%r13
  1f94c1:	mov    %rsi,0x28(%rsp)
  1f94c6:	shr    $1,%rsi
  1f94c9:	mov    %rcx,0x30(%rsp)
  1f94ce:	mov    %r9,%r15
  1f94d1:	mov    %rdi,0x58(%rsp)
  1f94d6:	mov    %r8,0x38(%rsp)
  1f94db:	cmp    %r8,%rsi
  1f94de:	jae    1f9ae3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x643>
  1f94e4:	mov    (%rbx),%rcx
  1f94e7:	mov    0x8(%rbx),%rsi
  1f94eb:	mov    0x10(%rbx),%rdx
  1f94ef:	movabs $0x7fffffffffffffc1,%rax
  1f94f9:	shl    $0x4,%r13
  1f94fd:	xor    %ebx,%ebx
  1f94ff:	add    %r15,%r13
  1f9502:	add    $0x37,%rax
  1f9506:	mov    %rax,0xc8(%rsp)
  1f950e:	mov    %r13,0x40(%rsp)
  1f9513:	mov    %rcx,0x60(%rsp)
  1f9518:	movabs $0xffffffffffffffd,%rcx
  1f9522:	mov    %rsi,0x48(%rsp)
  1f9527:	mov    %rdx,0x50(%rsp)
  1f952c:	cmp    %r13,%r15
  1f952f:	je     1f9d54 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8b4>
  1f9535:	mov    0x60(%rsp),%rax
  1f953a:	mov    (%r15),%r14
  1f953d:	mov    0x8(%r15),%r12
  1f9541:	mov    0x8(%rax),%rdx
  1f9545:	cmp    %r14,%r12
  1f9548:	jb     1f9d70 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8d0>
  1f954e:	cmp    %rdx,%r12
  1f9551:	ja     1f9d70 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8d0>
  1f9557:	shl    $0x2,%r12
  1f955b:	shl    $0x2,%r14
  1f955f:	mov    %rbx,0xc0(%rsp)
  1f9567:	mov    %r12,%rbp
  1f956a:	sub    %r14,%rbp
  1f956d:	mov    %rbp,%r13
  1f9570:	shl    $0x4,%r13
  1f9574:	cmp    %rcx,%rbp
  1f9577:	setb   %al
  1f957a:	cmp    0xc8(%rsp),%r13
  1f9582:	setbe  %cl
  1f9585:	test   %cl,%al
  1f9587:	je     1f9adb <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x63b>
  1f958d:	mov    0x60(%rsp),%rax
  1f9592:	mov    %r15,0x10(%rsp)
  1f9597:	shr    $0x2,%rbp
  1f959b:	mov    (%rax),%r15
  1f959e:	mov    0x10(%rax),%rbx
  1f95a2:	mov    0x18(%rax),%rax
  1f95a6:	add    %r14,%r15
  1f95a9:	mov    %rax,0xb8(%rsp)
  1f95b1:	test   %r13,%r13
  1f95b4:	je     1f9655 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x1b5>
  1f95ba:	mov    $0x8,%edi
  1f95bf:	mov    %r13,%rsi
  1f95c2:	xor    %edx,%edx
  1f95c4:	vzeroupper
  1f95c7:	call   4b8940 <<zopro_core::alloc::linux::Allocator>::allocate>
  1f95cc:	mov    %rbp,0x20(%rsp)
  1f95d1:	mov    %rax,0x18(%rsp)
  1f95d6:	test   %rax,%rax
  1f95d9:	je     1f9f52 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xab2>
  1f95df:	cmp    %r14,%r12
  1f95e2:	jne    1f9671 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x1d1>
  1f95e8:	mov    0x40(%rsp),%r13
  1f95ed:	xor    %ebp,%ebp
  1f95ef:	mov    %rbp,0x68(%rsp)
  1f95f4:	mov    $0x1d,%edx
  1f95f9:	lea    0x70(%rsp),%rdi
  1f95fe:	lea    -0x16f1e6(%rip),%rsi        # 8a41f <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x22f>
  1f9605:	vzeroupper
  1f9608:	call   142e20 <std::sys::env::unix::getenv>
  1f960d:	mov    0x70(%rsp),%r15
  1f9612:	cmp    $0xffffffffffffffff,%r15
  1f9616:	je     1f9717 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x277>
  1f961c:	mov    0x78(%rsp),%rbx
  1f9621:	mov    0x80(%rsp),%r14
  1f9629:	lea    0x70(%rsp),%rdi
  1f962e:	mov    %rbx,%rsi
  1f9631:	mov    %r14,%rdx
  1f9634:	call   f0100 <core::str::converts::from_utf8>
  1f9639:	cmpb   $0x0,0x70(%rsp)
  1f963e:	je     1f971c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x27c>
  1f9644:	mov    $0x1,%r12b
  1f9647:	test   %r15,%r15
  1f964a:	jg     1f9731 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x291>
  1f9650:	jmp    1f9761 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2c1>
  1f9655:	mov    $0x8,%eax
  1f965a:	movq   $0x0,0x20(%rsp)
  1f9663:	mov    %rax,0x18(%rsp)
  1f9668:	cmp    %r14,%r12
  1f966b:	je     1f95e8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x148>
  1f9671:	mov    0x18(%rsp),%rax
  1f9676:	mov    0x40(%rsp),%r13
  1f967b:	mov    0xb8(%rsp),%rsi
  1f9683:	lea    0x90(%rsp),%rdx
  1f968b:	xor    %ecx,%ecx
  1f968d:	jmp    1f96c7 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x227>
  1f968f:	nop
  1f9690:	add    %rbx,%rdi
  1f9693:	vmovups (%rdi),%ymm0
  1f9697:	vmovups %ymm0,0x70(%rsp)
  1f969d:	vmovdqu 0x20(%rdi),%ymm0
  1f96a2:	vmovdqu %ymm0,(%rdx)
  1f96a6:	vmovdqu64 0x70(%rsp),%zmm0
  1f96b1:	inc    %rcx
  1f96b4:	vmovdqu64 %zmm0,(%rax)
  1f96ba:	add    $0x40,%rax
  1f96be:	cmp    %rcx,%rbp
  1f96c1:	je     1f95ef <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x14f>
  1f96c7:	mov    (%r15,%rcx,4),%edi
  1f96cb:	cmp    %rdi,%rsi
  1f96ce:	jbe    1f9f24 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa84>
  1f96d4:	shl    $0x6,%rdi
  1f96d8:	vmovups (%rbx,%rdi,1),%zmm0
  1f96df:	vmovups %zmm0,0x110(%rsp)
  1f96ea:	vmovdqu (%rbx,%rdi,1),%ymm0
  1f96ef:	vptest %ymm0,%ymm0
  1f96f4:	jne    1f9690 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x1f0>
  1f96f6:	vmovdqu 0x130(%rsp),%ymm0
  1f96ff:	vptest %ymm0,%ymm0
  1f9704:	jne    1f9690 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x1f0>
  1f9706:	vpxor  %xmm0,%xmm0,%xmm0
  1f970a:	vmovdqu64 %zmm0,0x70(%rsp)
  1f9715:	jmp    1f96a6 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x206>
  1f9717:	mov    $0x1,%r12b
  1f971a:	jmp    1f9761 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2c1>
  1f971c:	mov    $0x1,%r12b
  1f971f:	cmp    $0x1,%r14
  1f9723:	jne    1f972c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x28c>
  1f9725:	cmpb   $0x30,(%rbx)
  1f9728:	setne  %r12b
  1f972c:	test   %r15,%r15
  1f972f:	je     1f9761 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2c1>
  1f9731:	cmp    $0x1ffff,%r15
  1f9738:	jbe    1f9758 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2b8>
  1f973a:	mov    %r15,%rsi
  1f973d:	inc    %rsi
  1f9740:	js     1f9f35 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1f9746:	cmpb   $0x1,(%rbx,%r15,1)
  1f974b:	jne    1f9758 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2b8>
  1f974d:	mov    %rbx,%rdi
  1f9750:	call   *0x2edbaa(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1f9756:	jmp    1f9761 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2c1>
  1f9758:	mov    %rbx,%rdi
  1f975b:	call   *0x2edba7(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1f9761:	mov    0x10(%rsp),%r15
  1f9766:	mov    %r12b,0x358(%rsp)
  1f976e:	lea    0x120(%rsp),%rax
  1f9776:	lea    0x110(%rsp),%rbx
  1f977e:	lea    -0x1ee56d(%rip),%r14        # b218 <__abi_tag+0xaf1c>
  1f9785:	lea    0x68(%rsp),%r12
  1f978a:	movq   $0x0,0x110(%rsp)
  1f9796:	movq   $0x8,0x118(%rsp)
  1f97a2:	vpxor  %xmm0,%xmm0,%xmm0
  1f97a6:	vmovdqu %xmm0,(%rax)
  1f97aa:	movq   $0x40,0x130(%rsp)
  1f97b6:	vmovdqu %xmm0,0x18(%rax)
  1f97bb:	movq   $0x40,0x148(%rsp)
  1f97c7:	vpxor  %xmm0,%xmm0,%xmm0
  1f97cb:	vmovdqu64 %zmm0,0x1f0(%rax)
  1f97d5:	vmovdqu64 %zmm0,0x1b0(%rax)
  1f97df:	vmovdqu64 %zmm0,0x170(%rax)
  1f97e9:	vmovdqu64 %zmm0,0x130(%rax)
  1f97f3:	vmovdqu64 %zmm0,0xf0(%rax)
  1f97fd:	vmovdqu64 %zmm0,0xb0(%rax)
  1f9807:	vmovdqu64 %zmm0,0x70(%rax)
  1f9811:	vmovdqu64 %zmm0,0x30(%rax)
  1f981b:	movq   $0x0,0x230(%rax)
  1f9826:	add    $0x10,%r15
  1f982a:	nopw   0x0(%rax,%rax,1)
  1f9830:	cmpq   $0x1,0x68(%rsp)
  1f9836:	jbe    1f9860 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x3c0>
  1f9838:	mov    0x18(%rsp),%rdi
  1f983d:	mov    $0x1,%ecx
  1f9842:	mov    $0x1,%r9d
  1f9848:	mov    %rbx,(%rsp)
  1f984c:	mov    %rbp,%rsi
  1f984f:	mov    %r14,%rdx
  1f9852:	mov    %r12,%r8
  1f9855:	vzeroupper
  1f9858:	call   1e0be0 <zopro_core::msm::reduce_layer::<zopro_bn254::curve::G1Config>>
  1f985d:	jmp    1f9830 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x390>
  1f985f:	nop
  1f9860:	test   %rbp,%rbp
  1f9863:	je     1f9f3d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa9d>
  1f9869:	mov    0x18(%rsp),%r14
  1f986e:	mov    0xc0(%rsp),%rbx
  1f9876:	mov    0x20(%rsp),%r12
  1f987b:	vmovups (%r14),%zmm0
  1f9881:	vmovups %zmm0,0x70(%rsp)
  1f988c:	vmovdqu (%r14),%ymm0
  1f9891:	vptest %ymm0,%ymm0
  1f9896:	jne    1f98ac <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x40c>
  1f9898:	vmovdqu 0x90(%rsp),%ymm0
  1f98a1:	vptest %ymm0,%ymm0
  1f98a6:	je     1f9ab6 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x616>
  1f98ac:	vmovups (%r14),%ymm0
  1f98b1:	lea    0xf0(%rsp),%rax
  1f98b9:	vmovups %ymm0,0xd0(%rsp)
  1f98c2:	vmovdqu 0x20(%r14),%ymm0
  1f98c8:	vmovdqu %ymm0,(%rax)
  1f98cc:	mov    0x110(%rsp),%rax
  1f98d4:	test   %rax,%rax
  1f98d7:	je     1f9929 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x489>
  1f98d9:	mov    0x118(%rsp),%rdi
  1f98e1:	cmp    $0x3fff,%rax
  1f98e7:	jbe    1f9920 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x480>
  1f98e9:	movabs $0x7fffffffffffffc1,%rcx
  1f98f3:	shl    $0x3,%rax
  1f98f7:	mov    %rax,%rsi
  1f98fa:	or     $0x1,%rsi
  1f98fe:	add    $0x38,%rcx
  1f9902:	cmp    %rcx,%rsi
  1f9905:	jae    1f9f35 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1f990b:	cmpb   $0x1,(%rdi,%rax,1)
  1f990f:	jne    1f9920 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x480>
  1f9911:	vzeroupper
  1f9914:	call   *0x2ed9e6(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1f991a:	jmp    1f9929 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x489>
  1f991c:	nopl   0x0(%rax)
  1f9920:	vzeroupper
  1f9923:	call   *0x2ed9df(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1f9929:	mov    0x128(%rsp),%rax
  1f9931:	test   %rax,%rax
  1f9934:	je     1f9980 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x4e0>
  1f9936:	mov    0x130(%rsp),%rdi
  1f993e:	cmp    $0x51,%rax
  1f9942:	jbe    1f9990 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x4f0>
  1f9944:	imul   $0x640,%rax,%rax
  1f994b:	movabs $0x7fffffffffffffc1,%rcx
  1f9955:	mov    %rax,%rsi
  1f9958:	or     $0x1,%rsi
  1f995c:	cmp    %rcx,%rsi
  1f995f:	jae    1f9f35 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1f9965:	cmpb   $0x1,(%rdi,%rax,1)
  1f9969:	jne    1f9990 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x4f0>
  1f996b:	vzeroupper
  1f996e:	call   *0x2ed98c(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1f9974:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  1f9980:	mov    0x140(%rsp),%rax
  1f9988:	test   %rax,%rax
  1f998b:	jne    1f99a6 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x506>
  1f998d:	jmp    1f99f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x550>
  1f998f:	nop
  1f9990:	vzeroupper
  1f9993:	call   *0x2ed96f(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1f9999:	mov    0x140(%rsp),%rax
  1f99a1:	test   %rax,%rax
  1f99a4:	je     1f99f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x550>
  1f99a6:	mov    0x148(%rsp),%rdi
  1f99ae:	cmp    $0xcc,%rax
  1f99b4:	jbe    1f9a00 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x560>
  1f99b6:	shl    $0x7,%rax
  1f99ba:	movabs $0x7fffffffffffffc1,%rcx
  1f99c4:	lea    (%rax,%rax,4),%rax
  1f99c8:	mov    %rax,%rsi
  1f99cb:	or     $0x1,%rsi
  1f99cf:	cmp    %rcx,%rsi
  1f99d2:	jae    1f9f35 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1f99d8:	cmpb   $0x1,(%rdi,%rax,1)
  1f99dc:	jne    1f9a00 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x560>
  1f99de:	vzeroupper
  1f99e1:	call   *0x2ed919(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1f99e7:	nopw   0x0(%rax,%rax,1)
  1f99f0:	test   %r12,%r12
  1f99f3:	jne    1f9a0e <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x56e>
  1f99f5:	jmp    1f9a5c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5bc>
  1f99f7:	nopw   0x0(%rax,%rax,1)
  1f9a00:	vzeroupper
  1f9a03:	call   *0x2ed8ff(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1f9a09:	test   %r12,%r12
  1f9a0c:	je     1f9a5c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5bc>
  1f9a0e:	cmp    $0x7ff,%r12
  1f9a15:	jbe    1f9a50 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5b0>
  1f9a17:	movabs $0x7fffffffffffffc1,%rax
  1f9a21:	shl    $0x6,%r12
  1f9a25:	mov    %r12,%rsi
  1f9a28:	or     $0x1,%rsi
  1f9a2c:	add    $0x38,%rax
  1f9a30:	cmp    %rax,%rsi
  1f9a33:	jae    1f9f35 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1f9a39:	cmpb   $0x1,(%r14,%r12,1)
  1f9a3e:	jne    1f9a50 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5b0>
  1f9a40:	mov    %r14,%rdi
  1f9a43:	vzeroupper
  1f9a46:	call   *0x2ed8b4(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1f9a4c:	jmp    1f9a5c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5bc>
  1f9a4e:	xchg   %ax,%ax
  1f9a50:	mov    %r14,%rdi
  1f9a53:	vzeroupper
  1f9a56:	call   *0x2ed8ac(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1f9a5c:	vmovups 0xd0(%rsp),%zmm0
  1f9a67:	vmovups %zmm0,0x360(%rsp)
  1f9a72:	cmp    0x50(%rsp),%rbx
  1f9a77:	je     1f9d85 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8e5>
  1f9a7d:	vmovdqu64 0x360(%rsp),%zmm0
  1f9a88:	mov    0x48(%rsp),%rcx
  1f9a8d:	mov    %rbx,%rax
  1f9a90:	shl    $0x6,%rax
  1f9a94:	inc    %rbx
  1f9a97:	vmovdqu64 %zmm0,(%rcx,%rax,1)
  1f9a9e:	movabs $0xffffffffffffffd,%rcx
  1f9aa8:	cmp    %r13,%r15
  1f9aab:	jne    1f9535 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x95>
  1f9ab1:	jmp    1f9d54 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8b4>
  1f9ab6:	vpxor  %xmm0,%xmm0,%xmm0
  1f9aba:	vmovdqu64 %zmm0,0xd0(%rsp)
  1f9ac5:	mov    0x110(%rsp),%rax
  1f9acd:	test   %rax,%rax
  1f9ad0:	jne    1f98d9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x439>
  1f9ad6:	jmp    1f9929 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x489>
  1f9adb:	vzeroupper
  1f9ade:	call   ed650 <alloc::raw_vec::capacity_overflow>
  1f9ae3:	mov    %r15,0x10(%rsp)
  1f9ae8:	test   %dl,%dl
  1f9aea:	je     1f9b02 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x662>
  1f9aec:	mov    %fs:0xffffffffffffffc0,%rax
  1f9af5:	test   %rax,%rax
  1f9af8:	je     1f9b10 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x670>
  1f9afa:	add    $0x110,%rax
  1f9b00:	jmp    1f9b21 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x681>
  1f9b02:	test   %rcx,%rcx
  1f9b05:	je     1f94e4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x44>
  1f9b0b:	shr    $1,%rcx
  1f9b0e:	jmp    1f9b35 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x695>
  1f9b10:	mov    %rsi,%r14
  1f9b13:	mov    %rcx,%r15
  1f9b16:	call   104130 <rayon_core::registry::global_registry>
  1f9b1b:	mov    %r15,%rcx
  1f9b1e:	mov    %r14,%rsi
  1f9b21:	mov    (%rax),%rax
  1f9b24:	shr    $1,%rcx
  1f9b27:	mov    0x208(%rax),%rax
  1f9b2e:	cmp    %rax,%rcx
  1f9b31:	cmovbe %rax,%rcx
  1f9b35:	sub    %rsi,%r13
  1f9b38:	mov    %rcx,0x30(%rsp)
  1f9b3d:	mov    %rsi,0xd0(%rsp)
  1f9b45:	jb     1f9da0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x900>
  1f9b4b:	mov    0x10(%rbx),%r15
  1f9b4f:	sub    %rsi,%r15
  1f9b52:	jb     1f9db8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x918>
  1f9b58:	mov    (%rbx),%r14
  1f9b5b:	mov    0x10(%rsp),%rdi
  1f9b60:	mov    0x8(%rbx),%rbx
  1f9b64:	mov    %fs:0xffffffffffffffc0,%rdx
  1f9b6d:	mov    %rsi,%r12
  1f9b70:	shl    $0x6,%r12
  1f9b74:	mov    %r13,%rbp
  1f9b77:	mov    %rsi,%r13
  1f9b7a:	shl    $0x4,%r13
  1f9b7e:	add    %rdi,%r13
  1f9b81:	add    %rbx,%r12
  1f9b84:	test   %rdx,%rdx
  1f9b87:	je     1f9c1c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x77c>
  1f9b8d:	lea    0x28(%rsp),%rax
  1f9b92:	lea    0xd0(%rsp),%r8
  1f9b9a:	lea    0x30(%rsp),%rcx
  1f9b9f:	mov    %rax,0x110(%rsp)
  1f9ba7:	mov    %r8,0x118(%rsp)
  1f9baf:	mov    %rcx,0x120(%rsp)
  1f9bb7:	mov    %r13,0x128(%rsp)
  1f9bbf:	mov    %rbp,0x130(%rsp)
  1f9bc7:	mov    %r14,0x138(%rsp)
  1f9bcf:	mov    %r12,0x140(%rsp)
  1f9bd7:	mov    %r15,0x148(%rsp)
  1f9bdf:	mov    %r8,0x150(%rsp)
  1f9be7:	mov    %rcx,0x158(%rsp)
  1f9bef:	mov    %rdi,0x160(%rsp)
  1f9bf7:	mov    %rsi,0x168(%rsp)
  1f9bff:	mov    %r14,0x170(%rsp)
  1f9c07:	mov    %rbx,0x178(%rsp)
  1f9c0f:	mov    %rsi,0x180(%rsp)
  1f9c17:	jmp    1f9cdc <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x83c>
  1f9c1c:	mov    %rsi,0x18(%rsp)
  1f9c21:	call   104130 <rayon_core::registry::global_registry>
  1f9c26:	mov    (%rax),%rsi
  1f9c29:	mov    %fs:0xffffffffffffffc0,%rdx
  1f9c32:	test   %rdx,%rdx
  1f9c35:	je     1f9dd0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x930>
  1f9c3b:	mov    0x18(%rsp),%rax
  1f9c40:	lea    0x28(%rsp),%rcx
  1f9c45:	cmp    %rsi,0x110(%rdx)
  1f9c4c:	jne    1f9e7f <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x9df>
  1f9c52:	lea    0xd0(%rsp),%rdi
  1f9c5a:	lea    0x30(%rsp),%rsi
  1f9c5f:	mov    %rcx,0x110(%rsp)
  1f9c67:	mov    %rdi,0x118(%rsp)
  1f9c6f:	mov    %rsi,0x120(%rsp)
  1f9c77:	mov    %r13,0x128(%rsp)
  1f9c7f:	mov    %rbp,0x130(%rsp)
  1f9c87:	mov    %r14,0x138(%rsp)
  1f9c8f:	mov    %r12,0x140(%rsp)
  1f9c97:	mov    %r15,0x148(%rsp)
  1f9c9f:	mov    %rdi,0x150(%rsp)
  1f9ca7:	mov    %rsi,0x158(%rsp)
  1f9caf:	mov    0x10(%rsp),%rsi
  1f9cb4:	mov    %rsi,0x160(%rsp)
  1f9cbc:	mov    %rax,0x168(%rsp)
  1f9cc4:	mov    %r14,0x170(%rsp)
  1f9ccc:	mov    %rbx,0x178(%rsp)
  1f9cd4:	mov    %rax,0x180(%rsp)
  1f9cdc:	lea    0x70(%rsp),%rdi
  1f9ce1:	lea    0x110(%rsp),%rsi
  1f9ce9:	xor    %ecx,%ecx
  1f9ceb:	call   1fbbd0 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}>
  1f9cf0:	mov    0x80(%rsp),%rcx
  1f9cf8:	mov    0x70(%rsp),%rax
  1f9cfd:	mov    %rcx,%rdx
  1f9d00:	shl    $0x6,%rdx
  1f9d04:	add    %rax,%rdx
  1f9d07:	cmp    0x88(%rsp),%rdx
  1f9d0f:	jne    1f9d23 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x883>
  1f9d11:	mov    0x90(%rsp),%rdx
  1f9d19:	mov    0x98(%rsp),%rsi
  1f9d21:	jmp    1f9d27 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x887>
  1f9d23:	xor    %edx,%edx
  1f9d25:	xor    %esi,%esi
  1f9d27:	mov    0x58(%rsp),%rdi
  1f9d2c:	add    0x78(%rsp),%rdx
  1f9d31:	add    %rcx,%rsi
  1f9d34:	mov    %rax,(%rdi)
  1f9d37:	mov    %rdx,0x8(%rdi)
  1f9d3b:	mov    %rsi,0x10(%rdi)
  1f9d3f:	add    $0x3a8,%rsp
  1f9d46:	pop    %rbx
  1f9d47:	pop    %r12
  1f9d49:	pop    %r13
  1f9d4b:	pop    %r14
  1f9d4d:	pop    %r15
  1f9d4f:	pop    %rbp
  1f9d50:	vzeroupper
  1f9d53:	ret
  1f9d54:	mov    0x58(%rsp),%rax
  1f9d59:	mov    0x48(%rsp),%rcx
  1f9d5e:	mov    0x50(%rsp),%rdx
  1f9d63:	mov    %rcx,(%rax)
  1f9d66:	mov    %rdx,0x8(%rax)
  1f9d6a:	mov    %rbx,0x10(%rax)
  1f9d6e:	jmp    1f9d3f <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x89f>
  1f9d70:	lea    0x2e8e91(%rip),%rcx        # 4e2c08 <__frame_dummy_init_array_entry+0x45e8>
  1f9d77:	mov    %r14,%rdi
  1f9d7a:	mov    %r12,%rsi
  1f9d7d:	vzeroupper
  1f9d80:	call   f0300 <core::slice::index::slice_index_fail>
  1f9d85:	lea    -0x16f9aa(%rip),%rdi        # 8a3e2 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1f2>
  1f9d8c:	lea    0x2ec35d(%rip),%rdx        # 4e60f0 <__frame_dummy_init_array_entry+0x7ad0>
  1f9d93:	mov    $0x45,%esi
  1f9d98:	vzeroupper
  1f9d9b:	call   ee300 <core::panicking::panic_fmt>
  1f9da0:	lea    -0x1da433(%rip),%rdi        # 1f974 <zmij::STATIC_DATA+0x5574>
  1f9da7:	lea    0x2ec392(%rip),%rdx        # 4e6140 <__frame_dummy_init_array_entry+0x7b20>
  1f9dae:	mov    $0x13,%esi
  1f9db3:	call   ee300 <core::panicking::panic_fmt>
  1f9db8:	lea    -0x16f9fb(%rip),%rdi        # 8a3c4 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1d4>
  1f9dbf:	lea    0x2ec2fa(%rip),%rdx        # 4e60c0 <__frame_dummy_init_array_entry+0x7aa0>
  1f9dc6:	mov    $0x1e,%esi
  1f9dcb:	call   f03d0 <core::panicking::panic>
  1f9dd0:	lea    0x28(%rsp),%rax
  1f9dd5:	lea    0xd0(%rsp),%rdx
  1f9ddd:	lea    0x30(%rsp),%rcx
  1f9de2:	sub    $0xffffffffffffff80,%rsi
  1f9de6:	lea    0x70(%rsp),%rdi
  1f9deb:	mov    %rax,0x110(%rsp)
  1f9df3:	mov    %rdx,0x118(%rsp)
  1f9dfb:	mov    %rcx,0x120(%rsp)
  1f9e03:	mov    %r13,0x128(%rsp)
  1f9e0b:	mov    %rbp,0x130(%rsp)
  1f9e13:	mov    %r14,0x138(%rsp)
  1f9e1b:	mov    %r12,0x140(%rsp)
  1f9e23:	mov    %r15,0x148(%rsp)
  1f9e2b:	mov    %rdx,0x150(%rsp)
  1f9e33:	mov    %rcx,0x158(%rsp)
  1f9e3b:	mov    0x10(%rsp),%rcx
  1f9e40:	mov    0x18(%rsp),%rbp
  1f9e45:	lea    0x110(%rsp),%rdx
  1f9e4d:	mov    %rcx,0x160(%rsp)
  1f9e55:	mov    %rbp,0x168(%rsp)
  1f9e5d:	mov    %r14,0x170(%rsp)
  1f9e65:	mov    %rbx,0x178(%rsp)
  1f9e6d:	mov    %rbp,0x180(%rsp)
  1f9e75:	call   1fc060 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>>
  1f9e7a:	jmp    1f9cf0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x850>
  1f9e7f:	lea    0xd0(%rsp),%r8
  1f9e87:	lea    0x30(%rsp),%rdi
  1f9e8c:	mov    %rcx,0x110(%rsp)
  1f9e94:	sub    $0xffffffffffffff80,%rsi
  1f9e98:	lea    0x110(%rsp),%rcx
  1f9ea0:	mov    %r8,0x118(%rsp)
  1f9ea8:	mov    %rdi,0x120(%rsp)
  1f9eb0:	mov    %r13,0x128(%rsp)
  1f9eb8:	mov    %rbp,0x130(%rsp)
  1f9ec0:	mov    %r14,0x138(%rsp)
  1f9ec8:	mov    %r12,0x140(%rsp)
  1f9ed0:	mov    %r15,0x148(%rsp)
  1f9ed8:	mov    %r8,0x150(%rsp)
  1f9ee0:	mov    %rdi,0x158(%rsp)
  1f9ee8:	mov    0x10(%rsp),%rdi
  1f9eed:	mov    %rdi,0x160(%rsp)
  1f9ef5:	lea    0x70(%rsp),%rdi
  1f9efa:	mov    %rax,0x168(%rsp)
  1f9f02:	mov    %r14,0x170(%rsp)
  1f9f0a:	mov    %rbx,0x178(%rsp)
  1f9f12:	mov    %rax,0x180(%rsp)
  1f9f1a:	call   1fc160 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>>
  1f9f1f:	jmp    1f9cf0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x850>
  1f9f24:	lea    0x2e987d(%rip),%rdx        # 4e37a8 <__frame_dummy_init_array_entry+0x5188>
  1f9f2b:	vzeroupper
  1f9f2e:	call   ef3ab <core::panicking::panic_bounds_check>
  1f9f33:	jmp    1f9f50 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xab0>
  1f9f35:	vzeroupper
  1f9f38:	call   110070 <std::process::abort>
  1f9f3d:	lea    0x2e8cac(%rip),%rdx        # 4e2bf0 <__frame_dummy_init_array_entry+0x45d0>
  1f9f44:	xor    %edi,%edi
  1f9f46:	xor    %esi,%esi
  1f9f48:	vzeroupper
  1f9f4b:	call   ef3ab <core::panicking::panic_bounds_check>
  1f9f50:	ud2
  1f9f52:	mov    $0x8,%edi
  1f9f57:	mov    %r13,%rsi
  1f9f5a:	call   ed521 <alloc::raw_vec::handle_error>
  1f9f5f:	jmp    1f9f63 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xac3>
  1f9f61:	jmp    1f9f68 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xac8>
  1f9f63:	mov    %rax,%rbx
  1f9f66:	jmp    1f9f78 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xad8>
  1f9f68:	lea    0x110(%rsp),%rdi
  1f9f70:	mov    %rax,%rbx
  1f9f73:	call   1e68f0 <core::ptr::drop_glue::<zopro_core::msm::Scratch<zopro_bn254::curve::G1Config>>>
  1f9f78:	cmpq   $0x0,0x20(%rsp)
  1f9f7e:	je     1f9f98 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xaf8>
  1f9f80:	mov    0x20(%rsp),%rsi
  1f9f85:	mov    0x18(%rsp),%rdi
  1f9f8a:	mov    $0x8,%edx
  1f9f8f:	shl    $0x6,%rsi
  1f9f93:	call   e8820 <__rustc::__rust_dealloc>
  1f9f98:	mov    %rbx,%rdi
  1f9f9b:	call   4dd5d0 <_Unwind_Resume@plt>
