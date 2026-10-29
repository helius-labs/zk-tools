
/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000464560 <zopro_core::msm_affine::g2_phi::raw_exclusive>:
  464560:	push   %rbp
  464561:	push   %r15
  464563:	push   %r14
  464565:	push   %r13
  464567:	push   %r12
  464569:	push   %rbx
  46456a:	sub    $0x1f8,%rsp
  464571:	mov    %rdx,0x40(%rsp)
  464576:	mov    %r8,0x60(%rsp)
  46457b:	cmp    %r8,%rdx
  46457e:	jne    46469c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x13c>
  464584:	lea    -0x400(%rdx),%rax
  46458b:	mov    %rcx,%r15
  46458e:	mov    %rdx,%r14
  464591:	cmp    $0xfc00,%rax
  464597:	jae    4645c7 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x67>
  464599:	mov    %fs:0xffffffffffffffc0,%rax
  4645a2:	mov    %rsi,%rbx
  4645a5:	mov    %rdi,%r13
  4645a8:	movq   $0xc,0x18(%rsp)
  4645b1:	movq   $0x2,0x48(%rsp)
  4645ba:	test   %rax,%rax
  4645bd:	je     4645e6 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x86>
  4645bf:	add    $0x110,%rax
  4645c5:	jmp    4645eb <zopro_core::msm_affine::g2_phi::raw_exclusive+0x8b>
  4645c7:	mov    %r14,%rdx
  4645ca:	mov    %r15,%rcx
  4645cd:	mov    %r14,%r8
  4645d0:	add    $0x1f8,%rsp
  4645d7:	pop    %rbx
  4645d8:	pop    %r12
  4645da:	pop    %r13
  4645dc:	pop    %r14
  4645de:	pop    %r15
  4645e0:	pop    %rbp
  4645e1:	jmp    459540 <zopro_core::msm_affine::g2_horner::raw>
  4645e6:	call   104130 <rayon_core::registry::global_registry>
  4645eb:	mov    (%rax),%rax
  4645ee:	mov    0x208(%rax),%rdx
  4645f5:	sub    $0x8,%rsp
  4645f9:	lea    0x1f(%rsp),%rax
  4645fe:	mov    $0x1,%ecx
  464603:	mov    %r14,%rdi
  464606:	xor    %esi,%esi
  464608:	mov    %r15,%r8
  46460b:	mov    %r14,%r9
  46460e:	push   %rax
  46460f:	call   464a80 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterMutProducer<[u64; 4]>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::g2_phi::hb_raw::{closure#0}>>>
  464614:	add    $0x10,%rsp
  464618:	mov    %r14,%rax
  46461b:	shr    $1,%rax
  46461e:	mov    %r14,%rcx
  464621:	movq   $0x0,0x20(%rsp)
  46462a:	movq   $0x8,0x28(%rsp)
  464633:	movq   $0x0,0x30(%rsp)
  46463c:	movq   $0xb,0x58(%rsp)
  464645:	sub    %rax,%rcx
  464648:	mov    %rcx,0x50(%rsp)
  46464d:	lea    0x20(%rsp),%rdi
  464652:	mov    $0xb,%edx
  464657:	mov    $0x8,%ecx
  46465c:	mov    $0xc0,%r8d
  464662:	xor    %esi,%esi
  464664:	call   1b5e60 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  464669:	mov    0x20(%rsp),%rbp
  46466e:	mov    0x30(%rsp),%r12
  464673:	mov    %rbp,%rax
  464676:	sub    %r12,%rax
  464679:	cmp    $0xb,%rax
  46467d:	jae    4646b4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x154>
  46467f:	lea    -0x444cdf(%rip),%rdi        # 1f9a7 <zmij::STATIC_DATA+0x55a7>
  464686:	lea    0x801a3(%rip),%rdx        # 4e4830 <__frame_dummy_init_array_entry+0x6210>
  46468d:	mov    $0x2f,%esi
  464692:	call   f03d0 <core::panicking::panic>
  464697:	jmp    464a41 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x4e1>
  46469c:	lea    0x812cd(%rip),%r8        # 4e5970 <__frame_dummy_init_array_entry+0x7350>
  4646a3:	lea    0x40(%rsp),%rdi
  4646a8:	lea    0x60(%rsp),%rsi
  4646ad:	xor    %edx,%edx
  4646af:	call   ef752 <core::panicking::assert_failed::<usize, usize>>
  4646b4:	mov    0x28(%rsp),%rcx
  4646b9:	lea    0x48(%rsp),%rax
  4646be:	lea    0x50(%rsp),%rsi
  4646c3:	lea    0x18(%rsp),%rdx
  4646c8:	movq   $0xb,0x138(%rsp)
  4646d4:	mov    %rax,0x60(%rsp)
  4646d9:	mov    %fs:0xffffffffffffffc0,%rax
  4646e2:	mov    %rsi,0x68(%rsp)
  4646e7:	lea    (%r12,%r12,2),%rsi
  4646eb:	mov    %rbx,0x70(%rsp)
  4646f0:	mov    %r14,0x78(%rsp)
  4646f5:	mov    %r15,0x80(%rsp)
  4646fd:	mov    %r14,0x88(%rsp)
  464705:	mov    %rdx,0x90(%rsp)
  46470d:	lea    0x60(%rsp),%rdx
  464712:	movq   $0x0,0x98(%rsp)
  46471e:	movq   $0xb,0xa0(%rsp)
  46472a:	mov    %rdx,0x120(%rsp)
  464732:	shl    $0x6,%rsi
  464736:	add    %rcx,%rsi
  464739:	mov    %rcx,0x8(%rsp)
  46473e:	mov    %rsi,0x128(%rsp)
  464746:	movq   $0xb,0x130(%rsp)
  464752:	test   %rax,%rax
  464755:	je     46475f <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1ff>
  464757:	add    $0x110,%rax
  46475d:	jmp    464764 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x204>
  46475f:	call   104130 <rayon_core::registry::global_registry>
  464764:	mov    (%rax),%rax
  464767:	mov    0x208(%rax),%rcx
  46476e:	lea    0x120(%rsp),%rbx
  464776:	xor    %edx,%edx
  464778:	lea    0x1e0(%rsp),%rdi
  464780:	mov    $0xb,%esi
  464785:	mov    $0x1,%r8d
  46478b:	xor    %r9d,%r9d
  46478e:	push   %rbx
  46478f:	push   $0xb
  464791:	call   464ef0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_phi::hb_raw::{closure#1}>>>
  464796:	add    $0x10,%rsp
  46479a:	mov    0x1f0(%rsp),%rax
  4647a2:	mov    %rax,0x120(%rsp)
  4647aa:	cmp    $0xb,%rax
  4647ae:	jne    464a09 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x4a9>
  4647b4:	vmovups -0x44536e(%rip),%zmm0        # 1f450 <zmij::STATIC_DATA+0x5050>
  4647be:	vmovups -0x4453b8(%rip),%zmm2        # 1f410 <zmij::STATIC_DATA+0x5010>
  4647c8:	vmovups -0x445402(%rip),%zmm1        # 1f3d0 <zmij::STATIC_DATA+0x4fd0>
  4647d2:	mov    0x18(%rsp),%rax
  4647d7:	shl    $0x6,%r12
  4647db:	lea    (%r12,%r12,2),%r12
  4647df:	mov    %rax,0x38(%rsp)
  4647e4:	vmovups %zmm0,0x1a0(%rsp)
  4647ef:	vmovups %zmm2,0x160(%rsp)
  4647fa:	vmovups %zmm1,0x120(%rsp)
  464805:	test   %rax,%rax
  464808:	je     4648e9 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x389>
  46480e:	mov    0x8(%rsp),%rax
  464813:	lea    0x60(%rsp),%r15
  464818:	lea    0x840(%rax,%r12,1),%r14
  464820:	lea    0x120(%rsp),%r12
  464828:	mov    0x38(%rsp),%rbx
  46482d:	mov    %r15,%rdi
  464830:	mov    %r12,%rsi
  464833:	vzeroupper
  464836:	call   190460 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  46483b:	vmovups 0x60(%rsp),%zmm0
  464846:	vmovups 0xa0(%rsp),%zmm1
  464851:	vmovups 0xe0(%rsp),%zmm2
  46485c:	dec    %rbx
  46485f:	vmovups %zmm2,0x1a0(%rsp)
  46486a:	vmovups %zmm1,0x160(%rsp)
  464875:	vmovups %zmm0,0x120(%rsp)
  464880:	jne    46482d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x2cd>
  464882:	add    $0xffffffffffffff40,%r14
  464889:	mov    %r15,%rdi
  46488c:	mov    %r12,%rsi
  46488f:	mov    %r14,%rdx
  464892:	vzeroupper
  464895:	call   183e90 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  46489a:	vmovups 0x60(%rsp),%zmm0
  4648a5:	vmovups 0xa0(%rsp),%zmm1
  4648b0:	vmovups 0xe0(%rsp),%zmm2
  4648bb:	vmovups %zmm2,0x1a0(%rsp)
  4648c6:	vmovups %zmm1,0x160(%rsp)
  4648d1:	vmovups %zmm0,0x120(%rsp)
  4648dc:	cmp    %r14,0x8(%rsp)
  4648e1:	jne    464828 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x2c8>
  4648e7:	jmp    464966 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x406>
  4648e9:	mov    0x8(%rsp),%rax
  4648ee:	lea    0x60(%rsp),%r14
  4648f3:	lea    0x120(%rsp),%r15
  4648fb:	lea    0x780(%rax),%rbx
  464902:	lea    (%rbx,%r12,1),%rdx
  464906:	mov    %r14,%rdi
  464909:	mov    %r15,%rsi
  46490c:	vzeroupper
  46490f:	call   183e90 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  464914:	vmovups 0x60(%rsp),%zmm0
  46491f:	vmovups 0xa0(%rsp),%zmm1
  46492a:	vmovups 0xe0(%rsp),%zmm2
  464935:	add    $0xffffffffffffff40,%r12
  46493c:	vmovups %zmm2,0x1a0(%rsp)
  464947:	vmovups %zmm1,0x160(%rsp)
  464952:	vmovups %zmm0,0x120(%rsp)
  46495d:	cmp    $0xfffffffffffff7c0,%r12
  464964:	jne    464902 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x3a2>
  464966:	vmovups 0x120(%rsp),%zmm0
  464971:	vmovups 0x160(%rsp),%zmm1
  46497c:	vmovups 0x1a0(%rsp),%zmm2
  464987:	vmovups %zmm2,0x80(%r13)
  46498e:	vmovups %zmm1,0x40(%r13)
  464995:	vmovups %zmm0,0x0(%r13)
  46499c:	test   %rbp,%rbp
  46499f:	je     4649f4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x494>
  4649a1:	cmp    $0x2aa,%rbp
  4649a8:	jbe    4649e4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x484>
  4649aa:	shl    $0x6,%rbp
  4649ae:	movabs $0x7ffffffffffffff8,%rcx
  4649b8:	lea    0x0(%rbp,%rbp,2),%rax
  4649bd:	mov    %rax,%rsi
  4649c0:	or     $0x1,%rsi
  4649c4:	cmp    %rcx,%rsi
  4649c7:	ja     464a43 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x4e3>
  4649c9:	mov    0x8(%rsp),%rcx
  4649ce:	cmpb   $0x1,(%rcx,%rax,1)
  4649d2:	jne    4649e6 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x486>
  4649d4:	mov    0x8(%rsp),%rdi
  4649d9:	vzeroupper
  4649dc:	call   *0x8291e(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  4649e2:	jmp    4649f4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x494>
  4649e4:	jmp    4649e6 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x486>
  4649e6:	mov    0x8(%rsp),%rdi
  4649eb:	vzeroupper
  4649ee:	call   *0x82914(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  4649f4:	add    $0x1f8,%rsp
  4649fb:	pop    %rbx
  4649fc:	pop    %r12
  4649fe:	pop    %r13
  464a00:	pop    %r14
  464a02:	pop    %r15
  464a04:	pop    %rbp
  464a05:	vzeroupper
  464a08:	ret
  464a09:	lea    0x58(%rsp),%rax
  464a0e:	lea    -0x3720c5(%rip),%rcx        # f2950 <<u64 as core::fmt::Display>::fmt>
  464a15:	mov    %rax,0x60(%rsp)
  464a1a:	mov    %rcx,0x68(%rsp)
  464a1f:	mov    %rbx,0x70(%rsp)
  464a24:	mov    %rcx,0x78(%rsp)
  464a29:	lea    -0x44dcde(%rip),%rdi        # 16d52 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x52d2>
  464a30:	lea    0x7dea1(%rip),%rdx        # 4e28d8 <__frame_dummy_init_array_entry+0x42b8>
  464a37:	lea    0x60(%rsp),%rsi
  464a3c:	call   ee300 <core::panicking::panic_fmt>
  464a41:	ud2
  464a43:	vzeroupper
  464a46:	call   110070 <std::process::abort>
  464a4b:	mov    %rax,%rbx
  464a4e:	mov    0x20(%rsp),%rax
  464a53:	test   %rax,%rax
  464a56:	je     464a6f <zopro_core::msm_affine::g2_phi::raw_exclusive+0x50f>
  464a58:	mov    0x28(%rsp),%rdi
  464a5d:	shl    $0x6,%rax
  464a61:	lea    (%rax,%rax,2),%rsi
  464a65:	mov    $0x8,%edx
  464a6a:	call   e8820 <__rustc::__rust_dealloc>
  464a6f:	mov    %rbx,%rdi
  464a72:	call   4dd5d0 <_Unwind_Resume@plt>
