
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047f6d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>>:
  47f6d0:	push   %rbp
  47f6d1:	push   %r15
  47f6d3:	push   %r14
  47f6d5:	push   %r13
  47f6d7:	push   %r12
  47f6d9:	push   %rbx
  47f6da:	sub    $0xe8,%rsp
  47f6e1:	mov    0x128(%rsp),%r15
  47f6e9:	mov    0x120(%rsp),%rbx
  47f6f1:	mov    %rsi,0x50(%rsp)
  47f6f6:	shr    $1,%rsi
  47f6f9:	mov    %rcx,0x58(%rsp)
  47f6fe:	mov    %r9,0x8(%rsp)
  47f703:	mov    %rdi,%r13
  47f706:	mov    %r8,0x60(%rsp)
  47f70b:	cmp    %r8,%rsi
  47f70e:	jae    47f792 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xc2>
  47f714:	mov    0x8(%rsp),%r14
  47f719:	mov    0x8(%r15),%rax
  47f71d:	mov    0x10(%r15),%rcx
  47f721:	mov    %r13,0x48(%rsp)
  47f726:	sub    %rbx,%r14
  47f729:	mov    %rcx,(%rsp)
  47f72d:	mov    %rax,0x10(%rsp)
  47f732:	jae    47f7ac <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xdc>
  47f734:	mov    (%r15),%r13
  47f737:	lea    0x10(%rax),%rbp
  47f73b:	xor    %r12d,%r12d
  47f73e:	xchg   %ax,%ax
  47f740:	mov    0x8(%rsp),%rax
  47f745:	lea    (%rax,%r12,1),%rdx
  47f749:	lea    0x18(%rsp),%rdi
  47f74e:	mov    %r13,%rsi
  47f751:	call   47fa90 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut>
  47f756:	mov    0x18(%rsp),%r15
  47f75b:	cmp    $0xffffffffffffffff,%r15
  47f75f:	je     47f7af <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xdf>
  47f761:	mov    0x20(%rsp),%rbx
  47f766:	cmp    %r12,(%rsp)
  47f76a:	je     47f9d1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x301>
  47f770:	mov    0x28(%rsp),%rax
  47f775:	inc    %r12
  47f778:	mov    %r15,-0x10(%rbp)
  47f77c:	mov    %rbx,-0x8(%rbp)
  47f780:	mov    %rax,0x0(%rbp)
  47f784:	add    $0x18,%rbp
  47f788:	mov    %r14,%rax
  47f78b:	add    %r12,%rax
  47f78e:	jne    47f740 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x70>
  47f790:	jmp    47f7af <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xdf>
  47f792:	test   %dl,%dl
  47f794:	je     47f7cd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xfd>
  47f796:	mov    %fs:0xffffffffffffffc0,%rax
  47f79f:	test   %rax,%rax
  47f7a2:	je     47f7db <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x10b>
  47f7a4:	add    $0x110,%rax
  47f7aa:	jmp    47f7ec <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x11c>
  47f7ac:	xor    %r12d,%r12d
  47f7af:	mov    0x48(%rsp),%rax
  47f7b4:	mov    0x10(%rsp),%rcx
  47f7b9:	mov    (%rsp),%rdx
  47f7bd:	mov    %rcx,(%rax)
  47f7c0:	mov    %rdx,0x8(%rax)
  47f7c4:	mov    %r12,0x10(%rax)
  47f7c8:	jmp    47f9bf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ef>
  47f7cd:	test   %rcx,%rcx
  47f7d0:	je     47f714 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x44>
  47f7d6:	shr    $1,%rcx
  47f7d9:	jmp    47f800 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x130>
  47f7db:	mov    %rsi,%r14
  47f7de:	mov    %rcx,%r12
  47f7e1:	call   1052d0 <rayon_core::registry::global_registry>
  47f7e6:	mov    %r12,%rcx
  47f7e9:	mov    %r14,%rsi
  47f7ec:	mov    (%rax),%rax
  47f7ef:	shr    $1,%rcx
  47f7f2:	mov    0x208(%rax),%rax
  47f7f9:	cmp    %rax,%rcx
  47f7fc:	cmovbe %rax,%rcx
  47f800:	mov    %rcx,0x58(%rsp)
  47f805:	xor    %eax,%eax
  47f807:	mov    %rbx,%rcx
  47f80a:	sub    0x8(%rsp),%rcx
  47f80f:	mov    %rsi,0x68(%rsp)
  47f814:	cmovae %rcx,%rax
  47f818:	cmp    %rax,%rsi
  47f81b:	ja     47f9eb <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x31b>
  47f821:	mov    0x10(%r15),%rax
  47f825:	sub    %rsi,%rax
  47f828:	jb     47fa03 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x333>
  47f82e:	mov    0x8(%r15),%rdi
  47f832:	mov    0x8(%rsp),%r11
  47f837:	mov    (%r15),%rdx
  47f83a:	lea    (%rsi,%rsi,2),%r8
  47f83e:	lea    0x50(%rsp),%r9
  47f843:	lea    0x58(%rsp),%r10
  47f848:	mov    %r9,0x70(%rsp)
  47f84d:	lea    0x68(%rsp),%r9
  47f852:	mov    %r9,0x78(%rsp)
  47f857:	mov    %r10,0x80(%rsp)
  47f85f:	lea    (%rdi,%r8,8),%r8
  47f863:	lea    (%rsi,%r11,1),%rcx
  47f867:	mov    %rdx,0x88(%rsp)
  47f86f:	mov    %r8,0x90(%rsp)
  47f877:	mov    %rax,0x98(%rsp)
  47f87f:	mov    %rcx,0xa0(%rsp)
  47f887:	mov    %rbx,0xa8(%rsp)
  47f88f:	mov    %r9,0xb0(%rsp)
  47f897:	mov    %r10,0xb8(%rsp)
  47f89f:	mov    %rdx,0xc0(%rsp)
  47f8a7:	mov    %fs:0xffffffffffffffc0,%rdx
  47f8b0:	mov    %rdi,0xc8(%rsp)
  47f8b8:	mov    %rsi,0xd0(%rsp)
  47f8c0:	mov    %r11,0xd8(%rsp)
  47f8c8:	mov    %rcx,0xe0(%rsp)
  47f8d0:	test   %rdx,%rdx
  47f8d3:	jne    47f8fc <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x22c>
  47f8d5:	call   1052d0 <rayon_core::registry::global_registry>
  47f8da:	mov    (%rax),%rsi
  47f8dd:	mov    %fs:0xffffffffffffffc0,%rdx
  47f8e6:	test   %rdx,%rdx
  47f8e9:	je     47fa1b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x34b>
  47f8ef:	cmp    %rsi,0x110(%rdx)
  47f8f6:	jne    47fa33 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x363>
  47f8fc:	lea    0x18(%rsp),%rdi
  47f901:	lea    0x70(%rsp),%rsi
  47f906:	xor    %ecx,%ecx
  47f908:	call   484750 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}>
  47f90d:	mov    0x28(%rsp),%rcx
  47f912:	vmovdqu 0x20(%rsp),%xmm0
  47f918:	mov    0x18(%rsp),%rax
  47f91d:	mov    0x30(%rsp),%rbx
  47f922:	lea    (%rcx,%rcx,2),%rcx
  47f926:	lea    (%rax,%rcx,8),%rcx
  47f92a:	cmp    %rbx,%rcx
  47f92d:	je     47f9af <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2df>
  47f933:	mov    0x40(%rsp),%r14
  47f938:	mov    %rax,0x0(%r13)
  47f93c:	vmovdqu %xmm0,0x8(%r13)
  47f942:	test   %r14,%r14
  47f945:	je     47f9bf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ef>
  47f947:	mov    0x7288a(%rip),%r12        # 4f21d8 <munmap@GLIBC_2.2.5>
  47f94e:	mov    0x7288b(%rip),%r13        # 4f21e0 <free@GLIBC_2.2.5>
  47f955:	add    $0x8,%rbx
  47f959:	movabs $0x7ffffffffffffff8,%r15
  47f963:	jmp    47f97c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ac>
  47f965:	data16 cs nopw 0x0(%rax,%rax,1)
  47f970:	call   *%r13
  47f973:	add    $0x18,%rbx
  47f977:	dec    %r14
  47f97a:	je     47f9bf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ef>
  47f97c:	mov    -0x8(%rbx),%rax
  47f980:	test   %rax,%rax
  47f983:	je     47f973 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a3>
  47f985:	mov    (%rbx),%rdi
  47f988:	cmp    $0x3ff,%rax
  47f98e:	jbe    47f970 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a0>
  47f990:	shl    $0x7,%rax
  47f994:	mov    %rax,%rsi
  47f997:	or     $0x1,%rsi
  47f99b:	cmp    %r15,%rsi
  47f99e:	ja     47fa4b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x37b>
  47f9a4:	cmpb   $0x1,(%rdi,%rax,1)
  47f9a8:	jne    47f970 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a0>
  47f9aa:	call   *%r12
  47f9ad:	jmp    47f973 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a3>
  47f9af:	vpaddq 0x38(%rsp),%xmm0,%xmm0
  47f9b5:	mov    %rax,0x0(%r13)
  47f9b9:	vmovdqu %xmm0,0x8(%r13)
  47f9bf:	add    $0xe8,%rsp
  47f9c6:	pop    %rbx
  47f9c7:	pop    %r12
  47f9c9:	pop    %r13
  47f9cb:	pop    %r14
  47f9cd:	pop    %r15
  47f9cf:	pop    %rbp
  47f9d0:	ret
  47f9d1:	lea    -0x3f542d(%rip),%rdi        # 8a5ab <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x21b>
  47f9d8:	lea    0x715e9(%rip),%rdx        # 4f0fc8 <__frame_dummy_init_array_entry+0x7b18>
  47f9df:	mov    $0x45,%esi
  47f9e4:	call   ef4b0 <core::panicking::panic_fmt>
  47f9e9:	ud2
  47f9eb:	lea    -0x466de9(%rip),%rdi        # 18c09 <miniz_oxide::inflate::core::reverse_bits::REVERSED_BITS_LOOKUP+0x611>
  47f9f2:	lea    0x6a9ef(%rip),%rdx        # 4ea3e8 <__frame_dummy_init_array_entry+0xf38>
  47f9f9:	mov    $0x2b,%esi
  47f9fe:	call   f1580 <core::panicking::panic>
  47fa03:	lea    -0x3f547d(%rip),%rdi        # 8a58d <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1fd>
  47fa0a:	lea    0x71587(%rip),%rdx        # 4f0f98 <__frame_dummy_init_array_entry+0x7ae8>
  47fa11:	mov    $0x1e,%esi
  47fa16:	call   f1580 <core::panicking::panic>
  47fa1b:	sub    $0xffffffffffffff80,%rsi
  47fa1f:	lea    0x18(%rsp),%rdi
  47fa24:	lea    0x70(%rsp),%rdx
  47fa29:	call   484650 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>
  47fa2e:	jmp    47f90d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x23d>
  47fa33:	sub    $0xffffffffffffff80,%rsi
  47fa37:	lea    0x18(%rsp),%rdi
  47fa3c:	lea    0x70(%rsp),%rcx
  47fa41:	call   484bf0 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>
  47fa46:	jmp    47f90d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x23d>
  47fa4b:	call   111210 <std::process::abort>
  47fa50:	mov    %rax,%r14
  47fa53:	jmp    47fa75 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x3a5>
  47fa55:	mov    %rax,%r14
  47fa58:	test   %r15,%r15
  47fa5b:	je     47fa71 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x3a1>
  47fa5d:	shl    $0x7,%r15
  47fa61:	mov    $0x8,%edx
  47fa66:	mov    %rbx,%rdi
  47fa69:	mov    %r15,%rsi
  47fa6c:	call   e99d0 <__rustc::__rust_dealloc>
  47fa71:	mov    (%rsp),%r12
  47fa75:	mov    0x10(%rsp),%rdi
  47fa7a:	mov    %r12,%rsi
  47fa7d:	call   2946b0 <core::ptr::drop_glue::<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47fa82:	mov    %r14,%rdi
  47fa85:	call   4e8460 <_Unwind_Resume@plt>
