
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047f3f0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut>:
  47f3f0:	push   %rbp
  47f3f1:	push   %r15
  47f3f3:	push   %r14
  47f3f5:	push   %r13
  47f3f7:	push   %r12
  47f3f9:	push   %rbx
  47f3fa:	sub    $0xf8,%rsp
  47f401:	mov    %rdx,0xe8(%rsp)
  47f409:	mov    %rdi,%rbx
  47f40c:	vmovups (%rsi),%xmm0
  47f410:	mov    0x10(%rsi),%r12
  47f414:	mov    0x18(%rsi),%r13
  47f418:	mov    0x20(%rsi),%rbp
  47f41c:	mov    0x28(%rsi),%r14
  47f420:	movq   $0x0,0x38(%rsp)
  47f429:	movq   $0x8,0x40(%rsp)
  47f432:	movq   $0x0,0x48(%rsp)
  47f43b:	movq   $0x4,0xf0(%rsp)
  47f447:	vmovaps %xmm0,0x20(%rsp)
  47f44d:	lea    0x38(%rsp),%rdi
  47f452:	mov    $0x4,%edx
  47f457:	mov    $0x8,%ecx
  47f45c:	mov    $0x18,%r8d
  47f462:	xor    %esi,%esi
  47f464:	call   1b7280 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  47f469:	mov    0x38(%rsp),%rax
  47f46e:	mov    0x48(%rsp),%r15
  47f473:	sub    %r15,%rax
  47f476:	cmp    $0x3,%rax
  47f47a:	ja     47f499 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0xa9>
  47f47c:	lea    -0x45f8d3(%rip),%rdi        # 1fbb0 <zmij::STATIC_DATA+0x5670>
  47f483:	lea    0x73716(%rip),%rdx        # 4f2ba0 <__frame_dummy_init_array_entry+0x6240>
  47f48a:	mov    $0x2f,%esi
  47f48f:	call   f1800 <core::panicking::panic>
  47f494:	jmp    47f925 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x535>
  47f499:	lea    (%r15,%r15,2),%rax
  47f49d:	vmovaps 0x20(%rsp),%xmm0
  47f4a3:	lea    0x90(%rsp),%rdx
  47f4ab:	movq   $0x4,0x68(%rsp)
  47f4b4:	lea    0xe8(%rsp),%rcx
  47f4bc:	mov    %rdx,0x50(%rsp)
  47f4c1:	shl    $0x3,%rax
  47f4c5:	add    0x40(%rsp),%rax
  47f4ca:	mov    %rax,0x58(%rsp)
  47f4cf:	mov    %fs:0xffffffffffffffc0,%rax
  47f4d8:	movq   $0x4,0x60(%rsp)
  47f4e1:	vmovaps %xmm0,0x90(%rsp)
  47f4ea:	mov    %r12,0xa0(%rsp)
  47f4f2:	mov    %r13,0xa8(%rsp)
  47f4fa:	mov    %rbp,0xb0(%rsp)
  47f502:	mov    %rcx,0xb8(%rsp)
  47f50a:	mov    %r14,0xc0(%rsp)
  47f512:	movq   $0x0,0xc8(%rsp)
  47f51e:	movq   $0x4,0xd0(%rsp)
  47f52a:	test   %rax,%rax
  47f52d:	je     47f537 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x147>
  47f52f:	add    $0x110,%rax
  47f535:	jmp    47f53c <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x14c>
  47f537:	call   105550 <rayon_core::registry::global_registry>
  47f53c:	mov    (%rax),%rax
  47f53f:	mov    0x208(%rax),%rcx
  47f546:	lea    0x50(%rsp),%r12
  47f54b:	xor    %edx,%edx
  47f54d:	lea    0x70(%rsp),%rdi
  47f552:	mov    $0x4,%esi
  47f557:	mov    $0x1,%r8d
  47f55d:	xor    %r9d,%r9d
  47f560:	push   %r12
  47f562:	push   $0x4
  47f564:	call   47f9f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>>
  47f569:	add    $0x10,%rsp
  47f56d:	mov    0x80(%rsp),%r14
  47f575:	mov    %r14,0x50(%rsp)
  47f57a:	cmp    $0x4,%r14
  47f57e:	jne    47f8d4 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4e4>
  47f584:	vmovups 0x38(%rsp),%xmm0
  47f58a:	add    $0x4,%r15
  47f58e:	mov    %r15,0x80(%rsp)
  47f596:	vmovaps %xmm0,0x70(%rsp)
  47f59c:	je     47f920 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x530>
  47f5a2:	mov    0x78(%rsp),%r13
  47f5a7:	dec    %r15
  47f5aa:	mov    0x0(%r13),%rax
  47f5ae:	mov    0x8(%r13),%rcx
  47f5b2:	mov    0x10(%r13),%r12
  47f5b6:	mov    %r13,0x10(%rsp)
  47f5bb:	mov    %rax,0x18(%rsp)
  47f5c0:	lea    (%r15,%r15,2),%rax
  47f5c4:	mov    %rcx,0x20(%rsp)
  47f5c9:	vmovups 0x0(%r13,%rax,8),%xmm0
  47f5d0:	mov    0x10(%r13,%rax,8),%rcx
  47f5d5:	lea    0x0(%r13,%rax,8),%rdx
  47f5da:	mov    %rdx,0x88(%rsp)
  47f5e2:	mov    %rcx,0x10(%r13)
  47f5e6:	vmovups %xmm0,0x0(%r13)
  47f5ec:	mov    %r15,0x80(%rsp)
  47f5f4:	mov    %r13,0x50(%rsp)
  47f5f9:	mov    0x70(%rsp),%rax
  47f5fe:	mov    %rax,0x60(%rsp)
  47f603:	mov    %rax,0x30(%rsp)
  47f608:	mov    %rdx,0x68(%rsp)
  47f60d:	je     47f752 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x362>
  47f613:	mov    0x10(%rsp),%rcx
  47f618:	lea    0x1ff(%r12),%rax
  47f620:	shr    $0x9,%rax
  47f624:	mov    %rax,0xe0(%rsp)
  47f62c:	lea    0x18(%rcx),%r13
  47f630:	jmp    47f651 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x261>
  47f632:	mov    %r15,%rdi
  47f635:	call   *0x760b5(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  47f63b:	add    $0x18,%r13
  47f63f:	add    $0x18,%rbp
  47f643:	cmp    0x88(%rsp),%rbp
  47f64b:	je     47f7d6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e6>
  47f651:	mov    -0x18(%r13),%r14
  47f655:	cmp    $0xffffffffffffffff,%r14
  47f659:	je     47f752 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x362>
  47f65f:	lea    -0x18(%r13),%rbp
  47f663:	mov    -0x10(%r13),%r15
  47f667:	mov    0x10(%rbp),%rax
  47f66b:	mov    %r12,0x38(%rsp)
  47f670:	mov    %rax,0x90(%rsp)
  47f678:	cmp    %rax,%r12
  47f67b:	jne    47f8b2 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4c2>
  47f681:	mov    0x20(%rsp),%rax
  47f686:	mov    %rax,0x90(%rsp)
  47f68e:	mov    %fs:0xffffffffffffffc0,%rax
  47f697:	mov    %r12,0x98(%rsp)
  47f69f:	movq   $0x200,0xa0(%rsp)
  47f6ab:	mov    %r15,0xa8(%rsp)
  47f6b3:	mov    %r12,0xb0(%rsp)
  47f6bb:	movq   $0x200,0xb8(%rsp)
  47f6c7:	test   %rax,%rax
  47f6ca:	je     47f6d4 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x2e4>
  47f6cc:	add    $0x110,%rax
  47f6d2:	jmp    47f6d9 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x2e9>
  47f6d4:	call   105550 <rayon_core::registry::global_registry>
  47f6d9:	mov    (%rax),%rax
  47f6dc:	mov    0x208(%rax),%rdx
  47f6e3:	mov    0xe0(%rsp),%rdi
  47f6eb:	mov    $0x1,%ecx
  47f6f0:	lea    0x90(%rsp),%r8
  47f6f8:	lea    0xf(%rsp),%r9
  47f6fd:	xor    %esi,%esi
  47f6ff:	call   2508e0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::zip::ZipProducer<rayon::slice::chunks::ChunksMutProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::slice::chunks::ChunksProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::merge_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>
  47f704:	test   %r14,%r14
  47f707:	je     47f63b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x24b>
  47f70d:	cmp    $0x3ff,%r14
  47f714:	jbe    47f632 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x242>
  47f71a:	shl    $0x7,%r14
  47f71e:	movabs $0x7ffffffffffffff9,%rax
  47f728:	mov    %r14,%rsi
  47f72b:	or     $0x1,%rsi
  47f72f:	cmp    %rax,%rsi
  47f732:	jae    47f92c <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x53c>
  47f738:	cmpb   $0x1,(%r15,%r14,1)
  47f73d:	jne    47f74d <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x35d>
  47f73f:	mov    %r15,%rdi
  47f742:	call   *0x75fa0(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  47f748:	jmp    47f63b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x24b>
  47f74d:	jmp    47f632 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x242>
  47f752:	mov    0x88(%rsp),%rdx
  47f75a:	movabs $0xaaaaaaaaaaaaaaab,%rax
  47f764:	sub    %r13,%rdx
  47f767:	mulx   %rax,%r14,%r14
  47f76c:	mov    %r13,0x58(%rsp)
  47f771:	je     47f7d6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e6>
  47f773:	mov    0x75f6e(%rip),%r15        # 4f56e8 <munmap@GLIBC_2.2.5>
  47f77a:	mov    0x75f6f(%rip),%rbp        # 4f56f0 <free@GLIBC_2.2.5>
  47f781:	shr    $0x4,%r14
  47f785:	add    $0x8,%r13
  47f789:	jmp    47f796 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3a6>
  47f78b:	call   *%rbp
  47f78d:	add    $0x18,%r13
  47f791:	dec    %r14
  47f794:	je     47f7d6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e6>
  47f796:	mov    -0x8(%r13),%rax
  47f79a:	test   %rax,%rax
  47f79d:	je     47f78d <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39d>
  47f79f:	mov    0x0(%r13),%rdi
  47f7a3:	cmp    $0x3ff,%rax
  47f7a9:	jbe    47f78b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39b>
  47f7ab:	shl    $0x7,%rax
  47f7af:	movabs $0x7ffffffffffffff9,%rcx
  47f7b9:	mov    %rax,%rsi
  47f7bc:	or     $0x1,%rsi
  47f7c0:	cmp    %rcx,%rsi
  47f7c3:	jae    47f927 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x537>
  47f7c9:	cmpb   $0x1,(%rdi,%rax,1)
  47f7cd:	jne    47f7d4 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e4>
  47f7cf:	call   *%r15
  47f7d2:	jmp    47f78d <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39d>
  47f7d4:	jmp    47f78b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39b>
  47f7d6:	cmpq   $0x0,0x30(%rsp)
  47f7dc:	je     47f835 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x445>
  47f7de:	cmpq   $0x1555,0x30(%rsp)
  47f7e7:	jbe    47f828 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x438>
  47f7e9:	mov    0x30(%rsp),%rax
  47f7ee:	movabs $0x7ffffffffffffff9,%rcx
  47f7f8:	shl    $0x3,%rax
  47f7fc:	lea    (%rax,%rax,2),%rax
  47f800:	mov    %rax,%rsi
  47f803:	or     $0x1,%rsi
  47f807:	cmp    %rcx,%rsi
  47f80a:	jae    47f931 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x541>
  47f810:	mov    0x10(%rsp),%rcx
  47f815:	cmpb   $0x1,(%rcx,%rax,1)
  47f819:	jne    47f82a <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x43a>
  47f81b:	mov    0x10(%rsp),%rdi
  47f820:	call   *0x75ec2(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  47f826:	jmp    47f835 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x445>
  47f828:	jmp    47f82a <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x43a>
  47f82a:	mov    0x10(%rsp),%rdi
  47f82f:	call   *0x75ebb(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  47f835:	mov    0x20(%rsp),%rsi
  47f83a:	mov    %rbx,%rdi
  47f83d:	mov    %r12,%rdx
  47f840:	call   247eb0 <zopro_core::msm::weighted_affine::<zopro_bn254::curve::G2Config>>
  47f845:	cmpq   $0x0,0x18(%rsp)
  47f84b:	je     47f8a0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4b0>
  47f84d:	cmpq   $0x3ff,0x18(%rsp)
  47f856:	jbe    47f893 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4a3>
  47f858:	mov    0x18(%rsp),%rsi
  47f85d:	movabs $0x7ffffffffffffff9,%rax
  47f867:	shl    $0x7,%rsi
  47f86b:	mov    %rsi,%rcx
  47f86e:	or     $0x1,%rsi
  47f872:	cmp    %rax,%rsi
  47f875:	jae    47f936 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x546>
  47f87b:	mov    0x20(%rsp),%rax
  47f880:	cmpb   $0x1,(%rax,%rcx,1)
  47f884:	jne    47f895 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4a5>
  47f886:	mov    0x20(%rsp),%rdi
  47f88b:	call   *0x75e57(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  47f891:	jmp    47f8a0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4b0>
  47f893:	jmp    47f895 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4a5>
  47f895:	mov    0x20(%rsp),%rdi
  47f89a:	call   *0x75e50(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  47f8a0:	add    $0xf8,%rsp
  47f8a7:	pop    %rbx
  47f8a8:	pop    %r12
  47f8aa:	pop    %r13
  47f8ac:	pop    %r14
  47f8ae:	pop    %r15
  47f8b0:	pop    %rbp
  47f8b1:	ret
  47f8b2:	mov    %r13,0x58(%rsp)
  47f8b7:	lea    0x6fada(%rip),%r8        # 4ef398 <__frame_dummy_init_array_entry+0x2a38>
  47f8be:	lea    0x38(%rsp),%rdi
  47f8c3:	lea    0x90(%rsp),%rsi
  47f8cb:	xor    %edx,%edx
  47f8cd:	call   f0b82 <core::panicking::assert_failed::<usize, usize>>
  47f8d2:	jmp    47f925 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x535>
  47f8d4:	lea    0xf0(%rsp),%rax
  47f8dc:	lea    -0x38bb63(%rip),%rcx        # f3d80 <<u64 as core::fmt::Display>::fmt>
  47f8e3:	mov    %rax,0x90(%rsp)
  47f8eb:	mov    %rcx,0x98(%rsp)
  47f8f3:	mov    %r12,0xa0(%rsp)
  47f8fb:	mov    %rcx,0xa8(%rsp)
  47f903:	lea    -0x468afb(%rip),%rdi        # 16e0f <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x530f>
  47f90a:	lea    0x71307(%rip),%rdx        # 4f0c18 <__frame_dummy_init_array_entry+0x42b8>
  47f911:	lea    0x90(%rsp),%rsi
  47f919:	call   ef730 <core::panicking::panic_fmt>
  47f91e:	jmp    47f925 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x535>
  47f920:	call   eeed7 <<alloc::vec::Vec<_, _>>::swap_remove::assert_failed>
  47f925:	ud2
  47f927:	call   111490 <std::process::abort>
  47f92c:	call   111490 <std::process::abort>
  47f931:	call   111490 <std::process::abort>
  47f936:	call   111490 <std::process::abort>
  47f93b:	mov    %rax,%rbx
  47f93e:	jmp    47f9a9 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x5b9>
  47f940:	jmp    47f942 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x552>
  47f942:	mov    %rax,%rbx
  47f945:	mov    %r13,0x58(%rsp)
  47f94a:	jmp    47f986 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x596>
  47f94c:	mov    %rax,%rbx
  47f94f:	lea    0x70(%rsp),%rdi
  47f954:	call   250570 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f959:	mov    %rbx,%rdi
  47f95c:	call   4eb910 <_Unwind_Resume@plt>
  47f961:	mov    0x70(%rsp),%rdi
  47f966:	mov    %rax,%rbx
  47f969:	mov    %r14,%rsi
  47f96c:	call   294930 <core::ptr::drop_glue::<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f971:	lea    0x38(%rsp),%rdi
  47f976:	call   250570 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f97b:	mov    %rbx,%rdi
  47f97e:	call   4eb910 <_Unwind_Resume@plt>
  47f983:	mov    %rax,%rbx
  47f986:	test   %r14,%r14
  47f989:	je     47f99f <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x5af>
  47f98b:	shl    $0x7,%r14
  47f98f:	mov    $0x8,%edx
  47f994:	mov    %r15,%rdi
  47f997:	mov    %r14,%rsi
  47f99a:	call   e9c50 <__rustc::__rust_dealloc>
  47f99f:	lea    0x50(%rsp),%rdi
  47f9a4:	call   2507e0 <core::ptr::drop_glue::<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f9a9:	cmpq   $0x0,0x18(%rsp)
  47f9af:	je     47f9c9 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x5d9>
  47f9b1:	mov    0x18(%rsp),%rsi
  47f9b6:	shl    $0x7,%rsi
  47f9ba:	mov    0x20(%rsp),%rdi
  47f9bf:	mov    $0x8,%edx
  47f9c4:	call   e9c50 <__rustc::__rust_dealloc>
  47f9c9:	mov    %rbx,%rdi
  47f9cc:	call   4eb910 <_Unwind_Resume@plt>
  47f9d1:	mov    %rax,%rbx
  47f9d4:	lea    0x38(%rsp),%rdi
  47f9d9:	call   250570 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f9de:	mov    %rbx,%rdi
  47f9e1:	call   4eb910 <_Unwind_Resume@plt>
