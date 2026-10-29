
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047f300 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut>:
  47f300:	push   %rbp
  47f301:	push   %r15
  47f303:	push   %r14
  47f305:	push   %r13
  47f307:	push   %r12
  47f309:	push   %rbx
  47f30a:	sub    $0xf8,%rsp
  47f311:	mov    %rdx,0xe8(%rsp)
  47f319:	mov    %rdi,%rbx
  47f31c:	vmovups (%rsi),%xmm0
  47f320:	mov    0x10(%rsi),%r12
  47f324:	mov    0x18(%rsi),%r13
  47f328:	mov    0x20(%rsi),%rbp
  47f32c:	mov    0x28(%rsi),%r14
  47f330:	movq   $0x0,0x38(%rsp)
  47f339:	movq   $0x8,0x40(%rsp)
  47f342:	movq   $0x0,0x48(%rsp)
  47f34b:	movq   $0x4,0xf0(%rsp)
  47f357:	vmovaps %xmm0,0x20(%rsp)
  47f35d:	lea    0x38(%rsp),%rdi
  47f362:	mov    $0x4,%edx
  47f367:	mov    $0x8,%ecx
  47f36c:	mov    $0x18,%r8d
  47f372:	xor    %esi,%esi
  47f374:	call   1b71c0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  47f379:	mov    0x38(%rsp),%rax
  47f37e:	mov    0x48(%rsp),%r15
  47f383:	sub    %r15,%rax
  47f386:	cmp    $0x3,%rax
  47f38a:	ja     47f3a9 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0xa9>
  47f38c:	lea    -0x45f7e3(%rip),%rdi        # 1fbb0 <zmij::STATIC_DATA+0x5670>
  47f393:	lea    0x736c6(%rip),%rdx        # 4f2a60 <__frame_dummy_init_array_entry+0x6240>
  47f39a:	mov    $0x2f,%esi
  47f39f:	call   f1740 <core::panicking::panic>
  47f3a4:	jmp    47f835 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x535>
  47f3a9:	lea    (%r15,%r15,2),%rax
  47f3ad:	vmovaps 0x20(%rsp),%xmm0
  47f3b3:	lea    0x90(%rsp),%rdx
  47f3bb:	movq   $0x4,0x68(%rsp)
  47f3c4:	lea    0xe8(%rsp),%rcx
  47f3cc:	mov    %rdx,0x50(%rsp)
  47f3d1:	shl    $0x3,%rax
  47f3d5:	add    0x40(%rsp),%rax
  47f3da:	mov    %rax,0x58(%rsp)
  47f3df:	mov    %fs:0xffffffffffffffc0,%rax
  47f3e8:	movq   $0x4,0x60(%rsp)
  47f3f1:	vmovaps %xmm0,0x90(%rsp)
  47f3fa:	mov    %r12,0xa0(%rsp)
  47f402:	mov    %r13,0xa8(%rsp)
  47f40a:	mov    %rbp,0xb0(%rsp)
  47f412:	mov    %rcx,0xb8(%rsp)
  47f41a:	mov    %r14,0xc0(%rsp)
  47f422:	movq   $0x0,0xc8(%rsp)
  47f42e:	movq   $0x4,0xd0(%rsp)
  47f43a:	test   %rax,%rax
  47f43d:	je     47f447 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x147>
  47f43f:	add    $0x110,%rax
  47f445:	jmp    47f44c <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x14c>
  47f447:	call   105490 <rayon_core::registry::global_registry>
  47f44c:	mov    (%rax),%rax
  47f44f:	mov    0x208(%rax),%rcx
  47f456:	lea    0x50(%rsp),%r12
  47f45b:	xor    %edx,%edx
  47f45d:	lea    0x70(%rsp),%rdi
  47f462:	mov    $0x4,%esi
  47f467:	mov    $0x1,%r8d
  47f46d:	xor    %r9d,%r9d
  47f470:	push   %r12
  47f472:	push   $0x4
  47f474:	call   47f900 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>>
  47f479:	add    $0x10,%rsp
  47f47d:	mov    0x80(%rsp),%r14
  47f485:	mov    %r14,0x50(%rsp)
  47f48a:	cmp    $0x4,%r14
  47f48e:	jne    47f7e4 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4e4>
  47f494:	vmovups 0x38(%rsp),%xmm0
  47f49a:	add    $0x4,%r15
  47f49e:	mov    %r15,0x80(%rsp)
  47f4a6:	vmovaps %xmm0,0x70(%rsp)
  47f4ac:	je     47f830 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x530>
  47f4b2:	mov    0x78(%rsp),%r13
  47f4b7:	dec    %r15
  47f4ba:	mov    0x0(%r13),%rax
  47f4be:	mov    0x8(%r13),%rcx
  47f4c2:	mov    0x10(%r13),%r12
  47f4c6:	mov    %r13,0x10(%rsp)
  47f4cb:	mov    %rax,0x18(%rsp)
  47f4d0:	lea    (%r15,%r15,2),%rax
  47f4d4:	mov    %rcx,0x20(%rsp)
  47f4d9:	vmovups 0x0(%r13,%rax,8),%xmm0
  47f4e0:	mov    0x10(%r13,%rax,8),%rcx
  47f4e5:	lea    0x0(%r13,%rax,8),%rdx
  47f4ea:	mov    %rdx,0x88(%rsp)
  47f4f2:	mov    %rcx,0x10(%r13)
  47f4f6:	vmovups %xmm0,0x0(%r13)
  47f4fc:	mov    %r15,0x80(%rsp)
  47f504:	mov    %r13,0x50(%rsp)
  47f509:	mov    0x70(%rsp),%rax
  47f50e:	mov    %rax,0x60(%rsp)
  47f513:	mov    %rax,0x30(%rsp)
  47f518:	mov    %rdx,0x68(%rsp)
  47f51d:	je     47f662 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x362>
  47f523:	mov    0x10(%rsp),%rcx
  47f528:	lea    0x1ff(%r12),%rax
  47f530:	shr    $0x9,%rax
  47f534:	mov    %rax,0xe0(%rsp)
  47f53c:	lea    0x18(%rcx),%r13
  47f540:	jmp    47f561 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x261>
  47f542:	mov    %r15,%rdi
  47f545:	call   *0x76065(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  47f54b:	add    $0x18,%r13
  47f54f:	add    $0x18,%rbp
  47f553:	cmp    0x88(%rsp),%rbp
  47f55b:	je     47f6e6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e6>
  47f561:	mov    -0x18(%r13),%r14
  47f565:	cmp    $0xffffffffffffffff,%r14
  47f569:	je     47f662 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x362>
  47f56f:	lea    -0x18(%r13),%rbp
  47f573:	mov    -0x10(%r13),%r15
  47f577:	mov    0x10(%rbp),%rax
  47f57b:	mov    %r12,0x38(%rsp)
  47f580:	mov    %rax,0x90(%rsp)
  47f588:	cmp    %rax,%r12
  47f58b:	jne    47f7c2 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4c2>
  47f591:	mov    0x20(%rsp),%rax
  47f596:	mov    %rax,0x90(%rsp)
  47f59e:	mov    %fs:0xffffffffffffffc0,%rax
  47f5a7:	mov    %r12,0x98(%rsp)
  47f5af:	movq   $0x200,0xa0(%rsp)
  47f5bb:	mov    %r15,0xa8(%rsp)
  47f5c3:	mov    %r12,0xb0(%rsp)
  47f5cb:	movq   $0x200,0xb8(%rsp)
  47f5d7:	test   %rax,%rax
  47f5da:	je     47f5e4 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x2e4>
  47f5dc:	add    $0x110,%rax
  47f5e2:	jmp    47f5e9 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x2e9>
  47f5e4:	call   105490 <rayon_core::registry::global_registry>
  47f5e9:	mov    (%rax),%rax
  47f5ec:	mov    0x208(%rax),%rdx
  47f5f3:	mov    0xe0(%rsp),%rdi
  47f5fb:	mov    $0x1,%ecx
  47f600:	lea    0x90(%rsp),%r8
  47f608:	lea    0xf(%rsp),%r9
  47f60d:	xor    %esi,%esi
  47f60f:	call   250820 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::zip::ZipProducer<rayon::slice::chunks::ChunksMutProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::slice::chunks::ChunksProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::merge_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>
  47f614:	test   %r14,%r14
  47f617:	je     47f54b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x24b>
  47f61d:	cmp    $0x3ff,%r14
  47f624:	jbe    47f542 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x242>
  47f62a:	shl    $0x7,%r14
  47f62e:	movabs $0x7ffffffffffffff9,%rax
  47f638:	mov    %r14,%rsi
  47f63b:	or     $0x1,%rsi
  47f63f:	cmp    %rax,%rsi
  47f642:	jae    47f83c <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x53c>
  47f648:	cmpb   $0x1,(%r15,%r14,1)
  47f64d:	jne    47f65d <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x35d>
  47f64f:	mov    %r15,%rdi
  47f652:	call   *0x75f50(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  47f658:	jmp    47f54b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x24b>
  47f65d:	jmp    47f542 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x242>
  47f662:	mov    0x88(%rsp),%rdx
  47f66a:	movabs $0xaaaaaaaaaaaaaaab,%rax
  47f674:	sub    %r13,%rdx
  47f677:	mulx   %rax,%r14,%r14
  47f67c:	mov    %r13,0x58(%rsp)
  47f681:	je     47f6e6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e6>
  47f683:	mov    0x75f1e(%rip),%r15        # 4f55a8 <munmap@GLIBC_2.2.5>
  47f68a:	mov    0x75f1f(%rip),%rbp        # 4f55b0 <free@GLIBC_2.2.5>
  47f691:	shr    $0x4,%r14
  47f695:	add    $0x8,%r13
  47f699:	jmp    47f6a6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3a6>
  47f69b:	call   *%rbp
  47f69d:	add    $0x18,%r13
  47f6a1:	dec    %r14
  47f6a4:	je     47f6e6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e6>
  47f6a6:	mov    -0x8(%r13),%rax
  47f6aa:	test   %rax,%rax
  47f6ad:	je     47f69d <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39d>
  47f6af:	mov    0x0(%r13),%rdi
  47f6b3:	cmp    $0x3ff,%rax
  47f6b9:	jbe    47f69b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39b>
  47f6bb:	shl    $0x7,%rax
  47f6bf:	movabs $0x7ffffffffffffff9,%rcx
  47f6c9:	mov    %rax,%rsi
  47f6cc:	or     $0x1,%rsi
  47f6d0:	cmp    %rcx,%rsi
  47f6d3:	jae    47f837 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x537>
  47f6d9:	cmpb   $0x1,(%rdi,%rax,1)
  47f6dd:	jne    47f6e4 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e4>
  47f6df:	call   *%r15
  47f6e2:	jmp    47f69d <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39d>
  47f6e4:	jmp    47f69b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39b>
  47f6e6:	cmpq   $0x0,0x30(%rsp)
  47f6ec:	je     47f745 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x445>
  47f6ee:	cmpq   $0x1555,0x30(%rsp)
  47f6f7:	jbe    47f738 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x438>
  47f6f9:	mov    0x30(%rsp),%rax
  47f6fe:	movabs $0x7ffffffffffffff9,%rcx
  47f708:	shl    $0x3,%rax
  47f70c:	lea    (%rax,%rax,2),%rax
  47f710:	mov    %rax,%rsi
  47f713:	or     $0x1,%rsi
  47f717:	cmp    %rcx,%rsi
  47f71a:	jae    47f841 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x541>
  47f720:	mov    0x10(%rsp),%rcx
  47f725:	cmpb   $0x1,(%rcx,%rax,1)
  47f729:	jne    47f73a <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x43a>
  47f72b:	mov    0x10(%rsp),%rdi
  47f730:	call   *0x75e72(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  47f736:	jmp    47f745 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x445>
  47f738:	jmp    47f73a <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x43a>
  47f73a:	mov    0x10(%rsp),%rdi
  47f73f:	call   *0x75e6b(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  47f745:	mov    0x20(%rsp),%rsi
  47f74a:	mov    %rbx,%rdi
  47f74d:	mov    %r12,%rdx
  47f750:	call   247df0 <zopro_core::msm::weighted_affine::<zopro_bn254::curve::G2Config>>
  47f755:	cmpq   $0x0,0x18(%rsp)
  47f75b:	je     47f7b0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4b0>
  47f75d:	cmpq   $0x3ff,0x18(%rsp)
  47f766:	jbe    47f7a3 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4a3>
  47f768:	mov    0x18(%rsp),%rsi
  47f76d:	movabs $0x7ffffffffffffff9,%rax
  47f777:	shl    $0x7,%rsi
  47f77b:	mov    %rsi,%rcx
  47f77e:	or     $0x1,%rsi
  47f782:	cmp    %rax,%rsi
  47f785:	jae    47f846 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x546>
  47f78b:	mov    0x20(%rsp),%rax
  47f790:	cmpb   $0x1,(%rax,%rcx,1)
  47f794:	jne    47f7a5 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4a5>
  47f796:	mov    0x20(%rsp),%rdi
  47f79b:	call   *0x75e07(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  47f7a1:	jmp    47f7b0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4b0>
  47f7a3:	jmp    47f7a5 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4a5>
  47f7a5:	mov    0x20(%rsp),%rdi
  47f7aa:	call   *0x75e00(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  47f7b0:	add    $0xf8,%rsp
  47f7b7:	pop    %rbx
  47f7b8:	pop    %r12
  47f7ba:	pop    %r13
  47f7bc:	pop    %r14
  47f7be:	pop    %r15
  47f7c0:	pop    %rbp
  47f7c1:	ret
  47f7c2:	mov    %r13,0x58(%rsp)
  47f7c7:	lea    0x6fa8a(%rip),%r8        # 4ef258 <__frame_dummy_init_array_entry+0x2a38>
  47f7ce:	lea    0x38(%rsp),%rdi
  47f7d3:	lea    0x90(%rsp),%rsi
  47f7db:	xor    %edx,%edx
  47f7dd:	call   f0ac2 <core::panicking::assert_failed::<usize, usize>>
  47f7e2:	jmp    47f835 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x535>
  47f7e4:	lea    0xf0(%rsp),%rax
  47f7ec:	lea    -0x38bb33(%rip),%rcx        # f3cc0 <<u64 as core::fmt::Display>::fmt>
  47f7f3:	mov    %rax,0x90(%rsp)
  47f7fb:	mov    %rcx,0x98(%rsp)
  47f803:	mov    %r12,0xa0(%rsp)
  47f80b:	mov    %rcx,0xa8(%rsp)
  47f813:	lea    -0x4689db(%rip),%rdi        # 16e3f <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x533f>
  47f81a:	lea    0x712b7(%rip),%rdx        # 4f0ad8 <__frame_dummy_init_array_entry+0x42b8>
  47f821:	lea    0x90(%rsp),%rsi
  47f829:	call   ef670 <core::panicking::panic_fmt>
  47f82e:	jmp    47f835 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x535>
  47f830:	call   eee17 <<alloc::vec::Vec<_, _>>::swap_remove::assert_failed>
  47f835:	ud2
  47f837:	call   1113d0 <std::process::abort>
  47f83c:	call   1113d0 <std::process::abort>
  47f841:	call   1113d0 <std::process::abort>
  47f846:	call   1113d0 <std::process::abort>
  47f84b:	mov    %rax,%rbx
  47f84e:	jmp    47f8b9 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x5b9>
  47f850:	jmp    47f852 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x552>
  47f852:	mov    %rax,%rbx
  47f855:	mov    %r13,0x58(%rsp)
  47f85a:	jmp    47f896 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x596>
  47f85c:	mov    %rax,%rbx
  47f85f:	lea    0x70(%rsp),%rdi
  47f864:	call   2504b0 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f869:	mov    %rbx,%rdi
  47f86c:	call   4eb7d0 <_Unwind_Resume@plt>
  47f871:	mov    0x70(%rsp),%rdi
  47f876:	mov    %rax,%rbx
  47f879:	mov    %r14,%rsi
  47f87c:	call   294870 <core::ptr::drop_glue::<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f881:	lea    0x38(%rsp),%rdi
  47f886:	call   2504b0 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f88b:	mov    %rbx,%rdi
  47f88e:	call   4eb7d0 <_Unwind_Resume@plt>
  47f893:	mov    %rax,%rbx
  47f896:	test   %r14,%r14
  47f899:	je     47f8af <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x5af>
  47f89b:	shl    $0x7,%r14
  47f89f:	mov    $0x8,%edx
  47f8a4:	mov    %r15,%rdi
  47f8a7:	mov    %r14,%rsi
  47f8aa:	call   e9b90 <__rustc::__rust_dealloc>
  47f8af:	lea    0x50(%rsp),%rdi
  47f8b4:	call   250720 <core::ptr::drop_glue::<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f8b9:	cmpq   $0x0,0x18(%rsp)
  47f8bf:	je     47f8d9 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x5d9>
  47f8c1:	mov    0x18(%rsp),%rsi
  47f8c6:	shl    $0x7,%rsi
  47f8ca:	mov    0x20(%rsp),%rdi
  47f8cf:	mov    $0x8,%edx
  47f8d4:	call   e9b90 <__rustc::__rust_dealloc>
  47f8d9:	mov    %rbx,%rdi
  47f8dc:	call   4eb7d0 <_Unwind_Resume@plt>
  47f8e1:	mov    %rax,%rbx
  47f8e4:	lea    0x38(%rsp),%rdi
  47f8e9:	call   2504b0 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f8ee:	mov    %rbx,%rdi
  47f8f1:	call   4eb7d0 <_Unwind_Resume@plt>
