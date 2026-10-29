
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004856f0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component>:
  4856f0:	push   %rbp
  4856f1:	push   %r15
  4856f3:	push   %r14
  4856f5:	push   %r13
  4856f7:	push   %r12
  4856f9:	push   %rbx
  4856fa:	sub    $0x148,%rsp
  485701:	mov    %rsi,0x30(%rsp)
  485706:	mov    %rdi,%r13
  485709:	mov    $0x4000,%edi
  48570e:	mov    $0x1,%esi
  485713:	mov    %r9,%r12
  485716:	mov    %r8,%rbp
  485719:	mov    %rcx,0x28(%rsp)
  48571e:	mov    %rdx,%rbx
  485721:	call   *0x6ff21(%rip)        # 4f5648 <calloc@GLIBC_2.2.5>
  485727:	test   %rax,%rax
  48572a:	je     485fee <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x8fe>
  485730:	cmp    %rbx,%rbp
  485733:	mov    %rax,%r14
  485736:	cmovae %rbx,%rbp
  48573a:	mov    %rbp,%r15
  48573d:	test   %rbp,%rbp
  485740:	je     485817 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x127>
  485746:	mov    0x28(%rsp),%rcx
  48574b:	mov    0x30(%rsp),%rsi
  485750:	mov    $0x10,%eax
  485755:	mov    %r15,%rdx
  485758:	shlx   %r12,%rax,%rax
  48575d:	add    $0x20,%rcx
  485761:	jmp    485781 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x91>
  485763:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  485770:	add    $0x28,%rcx
  485774:	sub    $0xffffffffffffff80,%rsi
  485778:	dec    %rdx
  48577b:	je     485817 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x127>
  485781:	mov    (%rcx),%r9
  485784:	mov    -0x20(%rcx,%r12,8),%rdi
  485789:	shrx   %r12,%r9,%r8
  48578e:	shr    $0x36,%rdi
  485792:	and    $0x1,%r8d
  485796:	shl    $0xa,%r8d
  48579a:	lea    0x1(%rdi,%r8,1),%edi
  48579f:	shr    $1,%edi
  4857a1:	mov    %edi,%r8d
  4857a4:	neg    %r8d
  4857a7:	test   %rax,%r9
  4857aa:	cmove  %edi,%r8d
  4857ae:	test   %r8w,%r8w
  4857b2:	je     485770 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x80>
  4857b4:	vmovdqu (%rsi),%ymm0
  4857b8:	vptest %ymm0,%ymm0
  4857bd:	jne    4857f0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x100>
  4857bf:	vmovdqu 0x20(%rsi),%ymm0
  4857c4:	vptest %ymm0,%ymm0
  4857c9:	jne    4857f0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x100>
  4857cb:	vmovdqu 0x40(%rsi),%ymm0
  4857d0:	vptest %ymm0,%ymm0
  4857d5:	jne    4857f0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x100>
  4857d7:	vmovdqu 0x60(%rsi),%ymm0
  4857dc:	vptest %ymm0,%ymm0
  4857e1:	je     485770 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x80>
  4857e3:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4857f0:	movzwl %r8w,%r8d
  4857f4:	shr    $0xf,%r8d
  4857f8:	shl    $0xa,%r8d
  4857fc:	lea    -0x1(%rdi,%r8,1),%rdi
  485801:	cmp    $0x7ff,%rdi
  485808:	ja     485fb1 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x8c1>
  48580e:	incq   (%r14,%rdi,8)
  485812:	jmp    485770 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x80>
  485817:	mov    $0x4008,%edi
  48581c:	mov    %r13,0x48(%rsp)
  485821:	mov    %rbx,0x20(%rsp)
  485826:	vzeroupper
  485829:	call   *0x6fd91(%rip)        # 4f55c0 <malloc@GLIBC_2.2.5>
  48582f:	test   %rax,%rax
  485832:	je     485ffd <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x90d>
  485838:	mov    $0x1,%ebp
  48583d:	mov    $0x4000,%ebx
  485842:	movq   $0x801,0x50(%rsp)
  48584b:	mov    %rax,0x58(%rsp)
  485850:	movq   $0x0,(%rax)
  485857:	movq   $0x1,0x60(%rsp)
  485860:	jmp    485882 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x192>
  485862:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  485870:	mov    %r13,(%rax,%rbp,8)
  485874:	inc    %rbp
  485877:	add    $0xfffffffffffffff8,%rbx
  48587b:	mov    %rbp,0x60(%rsp)
  485880:	je     4858a4 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x1b4>
  485882:	mov    -0x8(%rax,%rbp,8),%r13
  485887:	add    -0x8(%r14,%rbp,8),%r13
  48588c:	cmp    0x50(%rsp),%rbp
  485891:	jne    485870 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x180>
  485893:	lea    0x50(%rsp),%rdi
  485898:	call   f5300 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  48589d:	mov    0x58(%rsp),%rax
  4858a2:	jmp    485870 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x180>
  4858a4:	mov    %r14,%rdi
  4858a7:	mov    %rax,0x18(%rsp)
  4858ac:	call   *0x6fcfe(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  4858b2:	mov    0x18(%rsp),%rax
  4858b7:	mov    0x4000(%rax),%r14
  4858be:	test   %r14,%r14
  4858c1:	je     485932 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x242>
  4858c3:	mov    $0x4000,%edi
  4858c8:	mov    $0x4000,%ebx
  4858cd:	call   *0x6fced(%rip)        # 4f55c0 <malloc@GLIBC_2.2.5>
  4858d3:	test   %rax,%rax
  4858d6:	je     485e56 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x766>
  4858dc:	mov    0x18(%rsp),%rsi
  4858e1:	mov    $0x4000,%edx
  4858e6:	mov    %rax,%rdi
  4858e9:	mov    %rax,%r13
  4858ec:	movabs $0x7ffffffffffffff9,%rbx
  4858f6:	call   *0x6fc94(%rip)        # 4f5590 <memcpy@GLIBC_2.14>
  4858fc:	lea    0x0(,%r14,4),%rbp
  485904:	mov    %r14,%rax
  485907:	shr    $0x3e,%rax
  48590b:	lea    0x3(%rbx),%rcx
  48590f:	setne  %al
  485912:	cmp    %rcx,%rbp
  485915:	seta   %cl
  485918:	or     %al,%cl
  48591a:	je     485a67 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x377>
  485920:	xor    %ebx,%ebx
  485922:	mov    %rbx,%rdi
  485925:	mov    %rbp,%rsi
  485928:	call   ee891 <alloc::raw_vec::handle_error>
  48592d:	jmp    48607e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x98e>
  485932:	mov    0x71a4f(%rip),%rbx        # 4f7388 <zopro_core::alloc::linux::RECLAIM_USERS>
  485939:	test   %rbx,%rbx
  48593c:	je     485e38 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x748>
  485942:	mov    $0x20001,%esi
  485947:	mov    $0x3,%edx
  48594c:	mov    $0x22,%ecx
  485951:	mov    $0xffffffff,%r8d
  485957:	xor    %edi,%edi
  485959:	xor    %r9d,%r9d
  48595c:	call   *0x6fe8e(%rip)        # 4f57f0 <mmap@GLIBC_2.2.5>
  485962:	mov    0x48(%rsp),%rdi
  485967:	cmp    $0xffffffffffffffff,%rax
  48596b:	je     485e51 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x761>
  485971:	test   %rax,%rax
  485974:	je     485e51 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x761>
  48597a:	test   %rbx,%rbx
  48597d:	lea    0x100(%rsp),%rcx
  485985:	vpxor  %xmm0,%xmm0,%xmm0
  485989:	setne  0x20000(%rax)
  485990:	xor    %esi,%esi
  485992:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4859a0:	vmovdqu64 %zmm0,(%rcx)
  4859a6:	vmovdqu64 %zmm0,0xc0(%rsp)
  4859ae:	mov    %rsi,%rdx
  4859b1:	vmovups 0x100(%rsp),%zmm1
  4859b9:	vmovdqu64 %zmm0,(%rax,%rsi,1)
  4859c0:	vmovups %zmm1,0x40(%rax,%rsi,1)
  4859c8:	vmovdqu64 %zmm0,(%rcx)
  4859ce:	vmovdqu64 %zmm0,0xc0(%rsp)
  4859d6:	vmovups 0x100(%rsp),%zmm1
  4859de:	vmovdqu64 %zmm0,0x80(%rax,%rsi,1)
  4859e6:	vmovups %zmm1,0xc0(%rax,%rsi,1)
  4859ee:	vmovdqu64 %zmm0,(%rcx)
  4859f4:	vmovdqu64 %zmm0,0xc0(%rsp)
  4859fc:	vmovups 0x100(%rsp),%zmm1
  485a04:	vmovdqu64 %zmm0,0x100(%rax,%rsi,1)
  485a0c:	vmovups %zmm1,0x140(%rax,%rsi,1)
  485a14:	add    $0x180,%rsi
  485a1b:	cmp    $0x1ff80,%rsi
  485a22:	jne    4859a0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x2b0>
  485a28:	lea    0x180(%rax,%rdx,1),%rcx
  485a30:	vpxor  %xmm0,%xmm0,%xmm0
  485a34:	vmovdqu64 %zmm0,0x40(%rcx)
  485a3b:	vmovdqu64 %zmm0,(%rcx)
  485a41:	movq   $0x400,(%rdi)
  485a48:	mov    %rax,0x8(%rdi)
  485a4c:	movq   $0x400,0x10(%rdi)
  485a54:	mov    0x50(%rsp),%rax
  485a59:	test   %rax,%rax
  485a5c:	jne    485f3c <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x84c>
  485a62:	jmp    485f86 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x896>
  485a67:	mov    $0x4,%edi
  485a6c:	mov    $0x1,%edx
  485a71:	mov    %rbp,%rsi
  485a74:	mov    $0x4,%ebx
  485a79:	call   4c6b40 <<zopro_core::alloc::linux::Allocator>::allocate>
  485a7e:	test   %rax,%rax
  485a81:	je     485922 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x232>
  485a87:	mov    %r15,%rbx
  485a8a:	mov    0x28(%rsp),%r9
  485a8f:	mov    %rax,0x68(%rsp)
  485a94:	test   %r15,%r15
  485a97:	mov    0x30(%rsp),%r15
  485a9c:	je     485ba8 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x4b8>
  485aa2:	mov    $0x10,%ecx
  485aa7:	add    $0x20,%r9
  485aab:	shl    $0x7,%rbx
  485aaf:	lea    0x6e5b2(%rip),%rdx        # 4f4068 <__frame_dummy_init_array_entry+0x7848>
  485ab6:	xor    %r8d,%r8d
  485ab9:	shlx   %r12,%rcx,%r11
  485abe:	xor    %ecx,%ecx
  485ac0:	jmp    485af3 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x403>
  485ac2:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  485ad0:	mov    %r8d,(%rax,%rdi,4)
  485ad4:	inc    %rdi
  485ad7:	mov    %rdi,0x0(%r13,%r9,8)
  485adc:	mov    %rsi,%r9
  485adf:	add    $0x28,%r9
  485ae3:	sub    $0xffffffffffffff80,%rcx
  485ae7:	inc    %r8d
  485aea:	cmp    %rcx,%rbx
  485aed:	je     485ba8 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x4b8>
  485af3:	mov    (%r9),%r10
  485af6:	mov    -0x20(%r9,%r12,8),%rdi
  485afb:	mov    %r9,%rsi
  485afe:	shrx   %r12,%r10,%r9
  485b03:	shr    $0x36,%rdi
  485b07:	and    $0x1,%r9d
  485b0b:	shl    $0xa,%r9d
  485b0f:	lea    0x1(%rdi,%r9,1),%edi
  485b14:	shr    $1,%edi
  485b16:	mov    %edi,%r9d
  485b19:	neg    %r9d
  485b1c:	test   %r11,%r10
  485b1f:	cmove  %edi,%r9d
  485b23:	test   %r9w,%r9w
  485b27:	je     485adc <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x3ec>
  485b29:	vmovdqu (%r15,%rcx,1),%ymm0
  485b2f:	vptest %ymm0,%ymm0
  485b34:	jne    485b70 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x480>
  485b36:	vmovdqu 0x20(%r15,%rcx,1),%ymm0
  485b3d:	vptest %ymm0,%ymm0
  485b42:	jne    485b70 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x480>
  485b44:	vmovdqu 0x40(%r15,%rcx,1),%ymm0
  485b4b:	vptest %ymm0,%ymm0
  485b50:	jne    485b70 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x480>
  485b52:	vmovdqu 0x60(%r15,%rcx,1),%ymm0
  485b59:	vptest %ymm0,%ymm0
  485b5e:	je     485adc <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x3ec>
  485b64:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  485b70:	movzwl %r9w,%r9d
  485b74:	shr    $0xf,%r9d
  485b78:	shl    $0xa,%r9d
  485b7c:	lea    -0x1(%rdi,%r9,1),%r9
  485b81:	cmp    $0x7ff,%r9
  485b88:	ja     485fd1 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x8e1>
  485b8e:	mov    0x0(%r13,%r9,8),%rdi
  485b93:	cmp    %r14,%rdi
  485b96:	jb     485ad0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x3e0>
  485b9c:	lea    0x6e4dd(%rip),%rdx        # 4f4080 <__frame_dummy_init_array_entry+0x7860>
  485ba3:	jmp    485fde <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x8ee>
  485ba8:	mov    0x20(%rsp),%rdx
  485bad:	mov    0x18(%rsp),%r9
  485bb2:	lea    0x70(%rsp),%rdi
  485bb7:	movq   $0x801,(%rsp)
  485bbf:	mov    %r15,%rsi
  485bc2:	mov    %rax,%rcx
  485bc5:	mov    %r14,%r8
  485bc8:	vzeroupper
  485bcb:	call   2471c0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>>
  485bd0:	mov    0x80(%rsp),%rsi
  485bd8:	mov    %rsi,%rbx
  485bdb:	sub    $0x400,%rbx
  485be2:	jb     485f9b <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x8ab>
  485be8:	shl    $0x7,%rbx
  485bec:	mov    %rsi,%r15
  485bef:	sub    $0x400,%r15
  485bf6:	mov    %rsi,0x88(%rsp)
  485bfe:	mov    %r13,0x40(%rsp)
  485c03:	mov    %rbp,0x38(%rsp)
  485c08:	mov    %rbx,0x28(%rsp)
  485c0d:	je     48600e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x91e>
  485c13:	mov    $0x8,%edi
  485c18:	mov    %r15,0x98(%rsp)
  485c20:	mov    %rbx,%rsi
  485c23:	xor    %edx,%edx
  485c25:	call   4c6b40 <<zopro_core::alloc::linux::Allocator>::allocate>
  485c2a:	test   %rax,%rax
  485c2d:	je     486067 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x977>
  485c33:	movq   $0x400,0x80(%rsp)
  485c3f:	mov    %rax,%r14
  485c42:	mov    %r14,%rdi
  485c45:	mov    %rbx,%rdx
  485c48:	mov    0x78(%rsp),%rax
  485c4d:	lea    0x20000(%rax),%rsi
  485c54:	mov    %rax,0x90(%rsp)
  485c5c:	call   *0x6f92e(%rip)        # 4f5590 <memcpy@GLIBC_2.14>
  485c62:	vmovdqu -0x47408a(%rip),%ymm0        # 11be0 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0xe0>
  485c6a:	xor    %eax,%eax
  485c6c:	mov    %r14,0x20(%rsp)
  485c71:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  485c80:	vmovups 0x40(%r14,%rax,1),%ymm1
  485c87:	vmovdqu %ymm0,0xc0(%rsp)
  485c90:	xor    %esi,%esi
  485c92:	xor    %edi,%edi
  485c94:	xor    %r8d,%r8d
  485c97:	xor    %r9d,%r9d
  485c9a:	lea    0xa0(%rsp),%rdx
  485ca2:	lea    0xc0(%rsp),%rcx
  485caa:	vmovups %ymm1,0xa0(%rsp)
  485cb3:	sub    (%rdx),%rsi
  485cb6:	sbb    0x8(%rdx),%rdi
  485cba:	sbb    0x10(%rdx),%r8
  485cbe:	sbb    0x18(%rdx),%r9
  485cc2:	sbb    %r13,%r13
  485cc5:	mov    (%rcx),%r10
  485cc8:	mov    0x8(%rcx),%r11
  485ccc:	mov    0x10(%rcx),%r12
  485cd0:	mov    0x18(%rcx),%rbp
  485cd4:	and    %r13,%r10
  485cd7:	and    %r13,%r11
  485cda:	and    %r13,%r12
  485cdd:	and    %r13,%rbp
  485ce0:	add    %r10,%rsi
  485ce3:	adc    %r11,%rdi
  485ce6:	adc    %r12,%r8
  485ce9:	adc    %rbp,%r9
  485cec:	mov    %rsi,0x30(%rsp)
  485cf1:	vmovdqu %ymm0,0xc0(%rsp)
  485cfa:	xor    %r10d,%r10d
  485cfd:	xor    %r11d,%r11d
  485d00:	xor    %r12d,%r12d
  485d03:	xor    %r13d,%r13d
  485d06:	lea    0xa0(%rsp),%rsi
  485d0e:	vmovups 0x60(%r14,%rax,1),%ymm1
  485d15:	lea    0xc0(%rsp),%r14
  485d1d:	vmovups %ymm1,0xa0(%rsp)
  485d26:	sub    (%rsi),%r11
  485d29:	sbb    0x8(%rsi),%r12
  485d2d:	sbb    0x10(%rsi),%r13
  485d31:	sbb    0x18(%rsi),%r10
  485d35:	sbb    %r15,%r15
  485d38:	mov    (%r14),%rbp
  485d3b:	mov    0x8(%r14),%rbx
  485d3f:	mov    0x10(%r14),%rcx
  485d43:	mov    0x18(%r14),%rdx
  485d47:	and    %r15,%rbp
  485d4a:	and    %r15,%rbx
  485d4d:	and    %r15,%rcx
  485d50:	and    %r15,%rdx
  485d53:	add    %rbp,%r11
  485d56:	adc    %rbx,%r12
  485d59:	adc    %rcx,%r13
  485d5c:	adc    %rdx,%r10
  485d5f:	mov    0x20(%rsp),%r14
  485d64:	mov    0x30(%rsp),%rcx
  485d69:	mov    0x28(%rsp),%r15
  485d6e:	mov    %rcx,0x40(%r14,%rax,1)
  485d73:	mov    %rdi,0x48(%r14,%rax,1)
  485d78:	mov    %r8,0x50(%r14,%rax,1)
  485d7d:	mov    %r9,0x58(%r14,%rax,1)
  485d82:	mov    %r11,0x60(%r14,%rax,1)
  485d87:	mov    %r12,0x68(%r14,%rax,1)
  485d8c:	mov    %r13,0x70(%r14,%rax,1)
  485d91:	mov    %r10,0x78(%r14,%rax,1)
  485d96:	sub    $0xffffffffffffff80,%rax
  485d9a:	cmp    %rax,%r15
  485d9d:	jne    485c80 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x590>
  485da3:	mov    0x98(%rsp),%rax
  485dab:	movq   $0x400,0xa0(%rsp)
  485db7:	mov    %rax,0xc0(%rsp)
  485dbf:	cmp    $0x400,%rax
  485dc5:	jne    486044 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x954>
  485dcb:	mov    0x90(%rsp),%rax
  485dd3:	mov    0x48(%rsp),%rbx
  485dd8:	mov    0x40(%rsp),%r13
  485ddd:	mov    0x38(%rsp),%rbp
  485de2:	mov    %rax,0xc0(%rsp)
  485dea:	mov    %fs:0xffffffffffffffc0,%rax
  485df3:	movq   $0x400,0xc8(%rsp)
  485dff:	movq   $0x200,0xd0(%rsp)
  485e0b:	mov    %r14,0xd8(%rsp)
  485e13:	movq   $0x400,0xe0(%rsp)
  485e1f:	movq   $0x200,0xe8(%rsp)
  485e2b:	test   %rax,%rax
  485e2e:	je     485e68 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x778>
  485e30:	add    $0x110,%rax
  485e36:	jmp    485e70 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x780>
  485e38:	mov    $0x20001,%edi
  485e3d:	call   *0x6f77d(%rip)        # 4f55c0 <malloc@GLIBC_2.2.5>
  485e43:	mov    0x48(%rsp),%rdi
  485e48:	test   %rax,%rax
  485e4b:	jne    48597a <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x28a>
  485e51:	mov    $0x20000,%ebx
  485e56:	mov    $0x8,%edi
  485e5b:	mov    %rbx,%rsi
  485e5e:	call   ee891 <alloc::raw_vec::handle_error>
  485e63:	jmp    48607e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x98e>
  485e68:	vzeroupper
  485e6b:	call   105490 <rayon_core::registry::global_registry>
  485e70:	mov    (%rax),%rax
  485e73:	mov    0x208(%rax),%rdx
  485e7a:	lea    0xc0(%rsp),%r8
  485e82:	lea    0x17(%rsp),%r9
  485e87:	mov    $0x2,%edi
  485e8c:	mov    $0x1,%ecx
  485e91:	xor    %esi,%esi
  485e93:	vzeroupper
  485e96:	call   250820 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::zip::ZipProducer<rayon::slice::chunks::ChunksMutProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::slice::chunks::ChunksProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::merge_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>
  485e9b:	vmovdqu 0x70(%rsp),%xmm0
  485ea1:	mov    0x80(%rsp),%rax
  485ea9:	cmpb   $0x1,(%r14,%r15,1)
  485eae:	mov    %rax,0x10(%rbx)
  485eb2:	vmovdqu %xmm0,(%rbx)
  485eb6:	jne    485ee0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x7f0>
  485eb8:	mov    $0x20001,%esi
  485ebd:	mov    %r14,%rdi
  485ec0:	call   *0x6f6e2(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  485ec6:	mov    0x68(%rsp),%rdi
  485ecb:	movabs $0x7ffffffffffffff9,%rax
  485ed5:	cmp    $0x1ffff,%rbp
  485edc:	ja     485f01 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x811>
  485ede:	jmp    485f23 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x833>
  485ee0:	mov    %r14,%rdi
  485ee3:	call   *0x6f6c7(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  485ee9:	mov    0x68(%rsp),%rdi
  485eee:	movabs $0x7ffffffffffffff9,%rax
  485ef8:	cmp    $0x1ffff,%rbp
  485eff:	jbe    485f23 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x833>
  485f01:	mov    %rbp,%rsi
  485f04:	or     $0x1,%rsi
  485f08:	add    $0x4,%rax
  485f0c:	cmp    %rax,%rsi
  485f0f:	jae    486166 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa76>
  485f15:	cmpb   $0x1,(%rdi,%rbp,1)
  485f19:	jne    485f23 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x833>
  485f1b:	call   *0x6f687(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  485f21:	jmp    485f29 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x839>
  485f23:	call   *0x6f687(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  485f29:	mov    %r13,%rdi
  485f2c:	call   *0x6f67e(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  485f32:	mov    0x50(%rsp),%rax
  485f37:	test   %rax,%rax
  485f3a:	je     485f86 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x896>
  485f3c:	cmp    $0x3fff,%rax
  485f42:	jbe    485f78 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x888>
  485f44:	shl    $0x3,%rax
  485f48:	movabs $0x7ffffffffffffff9,%rcx
  485f52:	mov    %rax,%rsi
  485f55:	or     $0x1,%rsi
  485f59:	cmp    %rcx,%rsi
  485f5c:	jae    486166 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa76>
  485f62:	mov    0x18(%rsp),%rdi
  485f67:	cmpb   $0x1,(%rdi,%rax,1)
  485f6b:	jne    485f7d <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x88d>
  485f6d:	vzeroupper
  485f70:	call   *0x6f632(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  485f76:	jmp    485f86 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x896>
  485f78:	mov    0x18(%rsp),%rdi
  485f7d:	vzeroupper
  485f80:	call   *0x6f62a(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  485f86:	add    $0x148,%rsp
  485f8d:	pop    %rbx
  485f8e:	pop    %r12
  485f90:	pop    %r13
  485f92:	pop    %r14
  485f94:	pop    %r15
  485f96:	pop    %rbp
  485f97:	vzeroupper
  485f9a:	ret
  485f9b:	lea    0x6e0ae(%rip),%rdx        # 4f4050 <__frame_dummy_init_array_entry+0x7830>
  485fa2:	mov    $0x400,%edi
  485fa7:	call   eee5c <<alloc::vec::Vec<_, _>>::split_off::assert_failed>
  485fac:	jmp    48607e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x98e>
  485fb1:	lea    0x6e0e0(%rip),%rdx        # 4f4098 <__frame_dummy_init_array_entry+0x7878>
  485fb8:	mov    $0x800,%esi
  485fbd:	mov    $0xffffffffffffffff,%rdi
  485fc4:	vzeroupper
  485fc7:	call   f071b <core::panicking::panic_bounds_check>
  485fcc:	jmp    48607e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x98e>
  485fd1:	mov    $0x800,%r14d
  485fd7:	mov    $0xffffffffffffffff,%rdi
  485fde:	mov    %r14,%rsi
  485fe1:	vzeroupper
  485fe4:	call   f071b <core::panicking::panic_bounds_check>
  485fe9:	jmp    48607e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x98e>
  485fee:	mov    $0x8,%edi
  485ff3:	mov    $0x4000,%esi
  485ff8:	call   ee891 <alloc::raw_vec::handle_error>
  485ffd:	mov    $0x8,%edi
  486002:	mov    $0x4008,%esi
  486007:	call   ee891 <alloc::raw_vec::handle_error>
  48600c:	jmp    48607e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x98e>
  48600e:	mov    $0x20000,%esi
  486013:	add    0x78(%rsp),%rsi
  486018:	mov    $0x8,%eax
  48601d:	mov    $0x8,%edi
  486022:	mov    %rbx,%rdx
  486025:	mov    %rax,0x20(%rsp)
  48602a:	call   *0x6f560(%rip)        # 4f5590 <memcpy@GLIBC_2.14>
  486030:	movq   $0x400,0xa0(%rsp)
  48603c:	mov    %r15,0xc0(%rsp)
  486044:	lea    0x6920d(%rip),%r8        # 4ef258 <__frame_dummy_init_array_entry+0x2a38>
  48604b:	lea    0xa0(%rsp),%rdi
  486053:	lea    0xc0(%rsp),%rsi
  48605b:	xor    %edx,%edx
  48605d:	vzeroupper
  486060:	call   f0ac2 <core::panicking::assert_failed::<usize, usize>>
  486065:	jmp    48607e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x98e>
  486067:	mov    0x40(%rsp),%r13
  48606c:	mov    0x38(%rsp),%rbp
  486071:	mov    $0x8,%edi
  486076:	mov    %rbx,%rsi
  486079:	call   ee891 <alloc::raw_vec::handle_error>
  48607e:	ud2
  486080:	cmpq   $0x400,0x88(%rsp)
  48608c:	mov    0x40(%rsp),%r13
  486091:	mov    0x38(%rsp),%rbp
  486096:	mov    %rax,%r15
  486099:	jne    4860a0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x9b0>
  48609b:	jmp    4860d6 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x9e6>
  48609d:	mov    %rax,%r15
  4860a0:	mov    0x20(%rsp),%rdi
  4860a5:	mov    0x28(%rsp),%rsi
  4860aa:	mov    $0x8,%edx
  4860af:	call   e9b90 <__rustc::__rust_dealloc>
  4860b4:	jmp    4860d6 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x9e6>
  4860b6:	mov    %rax,%r15
  4860b9:	jmp    486113 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa23>
  4860bb:	mov    %rax,%r15
  4860be:	jmp    4860f3 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa03>
  4860c0:	mov    %rax,%r15
  4860c3:	mov    %r14,%rdi
  4860c6:	jmp    486158 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa68>
  4860cb:	mov    %r14,%rdi
  4860ce:	mov    %rax,%r15
  4860d1:	jmp    48610d <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa1d>
  4860d3:	mov    %rax,%r15
  4860d6:	mov    0x70(%rsp),%rsi
  4860db:	test   %rsi,%rsi
  4860de:	je     4860f3 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa03>
  4860e0:	mov    0x78(%rsp),%rdi
  4860e5:	shl    $0x7,%rsi
  4860e9:	mov    $0x8,%edx
  4860ee:	call   e9b90 <__rustc::__rust_dealloc>
  4860f3:	mov    0x68(%rsp),%rdi
  4860f8:	mov    $0x4,%edx
  4860fd:	mov    %rbp,%rsi
  486100:	call   e9b90 <__rustc::__rust_dealloc>
  486105:	jmp    48610a <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa1a>
  486107:	mov    %rax,%r15
  48610a:	mov    %r13,%rdi
  48610d:	call   *0x6f49d(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  486113:	mov    0x50(%rsp),%rax
  486118:	test   %rax,%rax
  48611b:	je     48615e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa6e>
  48611d:	mov    0x58(%rsp),%rdi
  486122:	cmp    $0x3fff,%rax
  486128:	jbe    486158 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa68>
  48612a:	shl    $0x3,%rax
  48612e:	movabs $0x7ffffffffffffff9,%rcx
  486138:	mov    %rax,%rsi
  48613b:	or     $0x1,%rsi
  48613f:	cmp    %rcx,%rsi
  486142:	jae    486166 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa76>
  486144:	cmpb   $0x1,(%rdi,%rax,1)
  486148:	jne    486158 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa68>
  48614a:	call   *0x6f458(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  486150:	mov    %r15,%rdi
  486153:	call   4eb7d0 <_Unwind_Resume@plt>
  486158:	call   *0x6f452(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  48615e:	mov    %r15,%rdi
  486161:	call   4eb7d0 <_Unwind_Resume@plt>
  486166:	vzeroupper
  486169:	call   1113d0 <std::process::abort>
