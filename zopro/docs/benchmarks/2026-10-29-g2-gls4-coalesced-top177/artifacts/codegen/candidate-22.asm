
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004857e0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component>:
  4857e0:	push   %rbp
  4857e1:	push   %r15
  4857e3:	push   %r14
  4857e5:	push   %r13
  4857e7:	push   %r12
  4857e9:	push   %rbx
  4857ea:	sub    $0x148,%rsp
  4857f1:	mov    %r9,%r14
  4857f4:	mov    %r8,%r13
  4857f7:	mov    %rcx,%r15
  4857fa:	mov    %rdx,%rbx
  4857fd:	mov    %rdi,%r12
  485800:	mov    %rsi,0x8(%rsp)
  485805:	mov    $0x4000,%edi
  48580a:	mov    $0x1,%esi
  48580f:	call   *0x6ff73(%rip)        # 4f5788 <calloc@GLIBC_2.2.5>
  485815:	mov    %rax,0x30(%rsp)
  48581a:	test   %rax,%rax
  48581d:	je     4860e0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x900>
  485823:	cmp    %rbx,%r13
  485826:	cmovae %rbx,%r13
  48582a:	mov    %r13,0x28(%rsp)
  48582f:	test   %r13,%r13
  485832:	je     48590c <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x12c>
  485838:	mov    0x28(%rsp),%rdx
  48583d:	mov    0x8(%rsp),%rsi
  485842:	mov    $0x10,%eax
  485847:	lea    0x20(%r15),%rcx
  48584b:	shlx   %r14,%rax,%rax
  485850:	jmp    485871 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x91>
  485852:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  485860:	add    $0x28,%rcx
  485864:	sub    $0xffffffffffffff80,%rsi
  485868:	dec    %rdx
  48586b:	je     48590c <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x12c>
  485871:	mov    (%rcx),%r9
  485874:	mov    -0x20(%rcx,%r14,8),%rdi
  485879:	shrx   %r14,%r9,%r8
  48587e:	shr    $0x36,%rdi
  485882:	and    $0x1,%r8d
  485886:	shl    $0xa,%r8d
  48588a:	lea    0x1(%rdi,%r8,1),%edi
  48588f:	shr    $1,%edi
  485891:	mov    %edi,%r8d
  485894:	neg    %r8d
  485897:	test   %rax,%r9
  48589a:	cmove  %edi,%r8d
  48589e:	test   %r8w,%r8w
  4858a2:	je     485860 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x80>
  4858a4:	vmovdqu (%rsi),%ymm0
  4858a8:	vptest %ymm0,%ymm0
  4858ad:	jne    4858e0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x100>
  4858af:	vmovdqu 0x20(%rsi),%ymm0
  4858b4:	vptest %ymm0,%ymm0
  4858b9:	jne    4858e0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x100>
  4858bb:	vmovdqu 0x40(%rsi),%ymm0
  4858c0:	vptest %ymm0,%ymm0
  4858c5:	jne    4858e0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x100>
  4858c7:	vmovdqu 0x60(%rsi),%ymm0
  4858cc:	vptest %ymm0,%ymm0
  4858d1:	je     485860 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x80>
  4858d3:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4858e0:	movzwl %r8w,%r8d
  4858e4:	shr    $0xf,%r8d
  4858e8:	shl    $0xa,%r8d
  4858ec:	lea    -0x1(%rdi,%r8,1),%rdi
  4858f1:	cmp    $0x7ff,%rdi
  4858f8:	ja     4860a3 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x8c3>
  4858fe:	mov    0x30(%rsp),%r8
  485903:	incq   (%r8,%rdi,8)
  485907:	jmp    485860 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x80>
  48590c:	mov    %rbx,0x20(%rsp)
  485911:	mov    $0x4008,%edi
  485916:	vzeroupper
  485919:	call   *0x6fde1(%rip)        # 4f5700 <malloc@GLIBC_2.2.5>
  48591f:	test   %rax,%rax
  485922:	je     4860ef <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x90f>
  485928:	mov    $0x1,%ebp
  48592d:	mov    $0x4000,%ebx
  485932:	mov    %rax,%rcx
  485935:	movq   $0x801,0x48(%rsp)
  48593e:	mov    %rax,0x50(%rsp)
  485943:	movq   $0x0,(%rax)
  48594a:	movq   $0x1,0x58(%rsp)
  485953:	jmp    485972 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x192>
  485955:	data16 cs nopw 0x0(%rax,%rax,1)
  485960:	mov    %r13,(%rcx,%rbp,8)
  485964:	inc    %rbp
  485967:	add    $0xfffffffffffffff8,%rbx
  48596b:	mov    %rbp,0x58(%rsp)
  485970:	je     485999 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x1b9>
  485972:	mov    0x30(%rsp),%rax
  485977:	mov    -0x8(%rcx,%rbp,8),%r13
  48597c:	add    -0x8(%rax,%rbp,8),%r13
  485981:	cmp    0x48(%rsp),%rbp
  485986:	jne    485960 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x180>
  485988:	lea    0x48(%rsp),%rdi
  48598d:	call   f53c0 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  485992:	mov    0x50(%rsp),%rcx
  485997:	jmp    485960 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x180>
  485999:	mov    0x4000(%rcx),%r13
  4859a0:	mov    %rcx,0x38(%rsp)
  4859a5:	test   %r13,%r13
  4859a8:	je     485a27 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x247>
  4859aa:	mov    %r12,0x98(%rsp)
  4859b2:	mov    $0x4000,%edi
  4859b7:	mov    $0x4000,%r12d
  4859bd:	call   *0x6fd3d(%rip)        # 4f5700 <malloc@GLIBC_2.2.5>
  4859c3:	test   %rax,%rax
  4859c6:	je     485f4f <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x76f>
  4859cc:	mov    %rax,%rbx
  4859cf:	movabs $0x7ffffffffffffff9,%r12
  4859d9:	mov    0x38(%rsp),%rsi
  4859de:	mov    $0x4000,%edx
  4859e3:	mov    %rax,%rdi
  4859e6:	call   *0x6fce4(%rip)        # 4f56d0 <memcpy@GLIBC_2.14>
  4859ec:	lea    0x0(,%r13,4),%rsi
  4859f4:	mov    %r13,%rax
  4859f7:	shr    $0x3e,%rax
  4859fb:	lea    0x3(%r12),%rcx
  485a00:	setne  %al
  485a03:	cmp    %rcx,%rsi
  485a06:	seta   %cl
  485a09:	or     %al,%cl
  485a0b:	je     485b5a <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x37a>
  485a11:	mov    %rsi,%rbp
  485a14:	xor    %r12d,%r12d
  485a17:	mov    %r12,%rdi
  485a1a:	mov    %rbp,%rsi
  485a1d:	call   ee951 <alloc::raw_vec::handle_error>
  485a22:	jmp    486170 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x990>
  485a27:	mov    0x71a9a(%rip),%rbx        # 4f74c8 <zopro_core::alloc::linux::RECLAIM_USERS>
  485a2e:	test   %rbx,%rbx
  485a31:	je     485f35 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x755>
  485a37:	mov    $0x20001,%esi
  485a3c:	mov    $0x3,%edx
  485a41:	mov    $0x22,%ecx
  485a46:	mov    $0xffffffff,%r8d
  485a4c:	xor    %edi,%edi
  485a4e:	xor    %r9d,%r9d
  485a51:	call   *0x6fed9(%rip)        # 4f5930 <mmap@GLIBC_2.2.5>
  485a57:	cmp    $0xffffffffffffffff,%rax
  485a5b:	je     485f49 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x769>
  485a61:	test   %rax,%rax
  485a64:	je     485f49 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x769>
  485a6a:	test   %rbx,%rbx
  485a6d:	lea    0x100(%rsp),%rcx
  485a75:	vpxor  %xmm0,%xmm0,%xmm0
  485a79:	setne  0x20000(%rax)
  485a80:	xor    %esi,%esi
  485a82:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  485a90:	vmovdqu64 %zmm0,(%rcx)
  485a96:	vmovdqu64 %zmm0,0xc0(%rsp)
  485a9e:	mov    %rsi,%rdx
  485aa1:	vmovups 0x100(%rsp),%zmm1
  485aa9:	vmovdqu64 %zmm0,(%rax,%rsi,1)
  485ab0:	vmovups %zmm1,0x40(%rax,%rsi,1)
  485ab8:	vmovdqu64 %zmm0,(%rcx)
  485abe:	vmovdqu64 %zmm0,0xc0(%rsp)
  485ac6:	vmovups 0x100(%rsp),%zmm1
  485ace:	vmovdqu64 %zmm0,0x80(%rax,%rsi,1)
  485ad6:	vmovups %zmm1,0xc0(%rax,%rsi,1)
  485ade:	vmovdqu64 %zmm0,(%rcx)
  485ae4:	vmovdqu64 %zmm0,0xc0(%rsp)
  485aec:	vmovups 0x100(%rsp),%zmm1
  485af4:	vmovdqu64 %zmm0,0x100(%rax,%rsi,1)
  485afc:	vmovups %zmm1,0x140(%rax,%rsi,1)
  485b04:	add    $0x180,%rsi
  485b0b:	cmp    $0x1ff80,%rsi
  485b12:	jne    485a90 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x2b0>
  485b18:	lea    0x180(%rax,%rdx,1),%rcx
  485b20:	vpxor  %xmm0,%xmm0,%xmm0
  485b24:	vmovdqu64 %zmm0,0x40(%rcx)
  485b2b:	vmovdqu64 %zmm0,(%rcx)
  485b31:	movq   $0x400,(%r12)
  485b39:	mov    %rax,0x8(%r12)
  485b3e:	movq   $0x400,0x10(%r12)
  485b47:	mov    0x48(%rsp),%rax
  485b4c:	test   %rax,%rax
  485b4f:	jne    486021 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x841>
  485b55:	jmp    48606d <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x88d>
  485b5a:	mov    $0x4,%edi
  485b5f:	mov    $0x1,%edx
  485b64:	mov    $0x4,%r12d
  485b6a:	mov    %rsi,%rbp
  485b6d:	call   4c6c80 <<zopro_core::alloc::linux::Allocator>::allocate>
  485b72:	test   %rax,%rax
  485b75:	je     485a17 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x237>
  485b7b:	mov    0x28(%rsp),%r12
  485b80:	mov    %rax,0x60(%rsp)
  485b85:	test   %r12,%r12
  485b88:	je     485ca3 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x4c3>
  485b8e:	mov    $0x10,%ecx
  485b93:	add    $0x20,%r15
  485b97:	shl    $0x7,%r12
  485b9b:	lea    0x6e606(%rip),%rdx        # 4f41a8 <__frame_dummy_init_array_entry+0x7848>
  485ba2:	lea    0x6e617(%rip),%rsi        # 4f41c0 <__frame_dummy_init_array_entry+0x7860>
  485ba9:	xor    %r8d,%r8d
  485bac:	shlx   %r14,%rcx,%r11
  485bb1:	xor    %ecx,%ecx
  485bb3:	jmp    485bdf <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x3ff>
  485bb5:	data16 cs nopw 0x0(%rax,%rax,1)
  485bc0:	mov    %r8d,(%rax,%rdi,4)
  485bc4:	inc    %rdi
  485bc7:	mov    %rdi,(%rbx,%r9,8)
  485bcb:	add    $0x28,%r15
  485bcf:	sub    $0xffffffffffffff80,%rcx
  485bd3:	inc    %r8d
  485bd6:	cmp    %rcx,%r12
  485bd9:	je     485ca3 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x4c3>
  485bdf:	mov    (%r15),%r10
  485be2:	mov    -0x20(%r15,%r14,8),%rdi
  485be7:	shrx   %r14,%r10,%r9
  485bec:	shr    $0x36,%rdi
  485bf0:	and    $0x1,%r9d
  485bf4:	shl    $0xa,%r9d
  485bf8:	lea    0x1(%rdi,%r9,1),%edi
  485bfd:	shr    $1,%edi
  485bff:	mov    %edi,%r9d
  485c02:	neg    %r9d
  485c05:	test   %r11,%r10
  485c08:	cmove  %edi,%r9d
  485c0c:	test   %r9w,%r9w
  485c10:	je     485bcb <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x3eb>
  485c12:	mov    0x8(%rsp),%r10
  485c17:	vmovdqu (%r10,%rcx,1),%ymm0
  485c1d:	vptest %ymm0,%ymm0
  485c22:	jne    485c70 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x490>
  485c24:	mov    0x8(%rsp),%r10
  485c29:	vmovdqu 0x20(%r10,%rcx,1),%ymm0
  485c30:	vptest %ymm0,%ymm0
  485c35:	jne    485c70 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x490>
  485c37:	mov    0x8(%rsp),%r10
  485c3c:	vmovdqu 0x40(%r10,%rcx,1),%ymm0
  485c43:	vptest %ymm0,%ymm0
  485c48:	jne    485c70 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x490>
  485c4a:	mov    0x8(%rsp),%r10
  485c4f:	vmovdqu 0x60(%r10,%rcx,1),%ymm0
  485c56:	vptest %ymm0,%ymm0
  485c5b:	je     485bcb <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x3eb>
  485c61:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  485c70:	movzwl %r9w,%r9d
  485c74:	shr    $0xf,%r9d
  485c78:	shl    $0xa,%r9d
  485c7c:	lea    -0x1(%rdi,%r9,1),%r9
  485c81:	cmp    $0x7ff,%r9
  485c88:	ja     4860c3 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x8e3>
  485c8e:	mov    (%rbx,%r9,8),%rdi
  485c92:	cmp    %r13,%rdi
  485c95:	jb     485bc0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x3e0>
  485c9b:	mov    %rsi,%rdx
  485c9e:	jmp    4860d0 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x8f0>
  485ca3:	sub    $0x8,%rsp
  485ca7:	mov    0x28(%rsp),%rdx
  485cac:	mov    0x10(%rsp),%rsi
  485cb1:	mov    0x40(%rsp),%r9
  485cb6:	lea    0x70(%rsp),%rdi
  485cbb:	mov    %rax,%rcx
  485cbe:	mov    %r13,%r8
  485cc1:	push   $0x800
  485cc6:	push   0x40(%rsp)
  485cca:	push   $0x801
  485ccf:	vzeroupper
  485cd2:	call   22c510 <zopro_core::msm::group_affine::<zopro_bn254::curve::G2Config>>
  485cd7:	add    $0x20,%rsp
  485cdb:	mov    0x78(%rsp),%rsi
  485ce0:	mov    %rsi,%r14
  485ce3:	sub    $0x400,%r14
  485cea:	jb     48608d <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x8ad>
  485cf0:	shl    $0x7,%r14
  485cf4:	mov    %rsi,%r12
  485cf7:	sub    $0x400,%r12
  485cfe:	mov    %rsi,0x80(%rsp)
  485d06:	mov    %rbx,0x18(%rsp)
  485d0b:	mov    %rbp,0x40(%rsp)
  485d10:	mov    %r14,0x28(%rsp)
  485d15:	je     486100 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x920>
  485d1b:	mov    %r12,0x90(%rsp)
  485d23:	mov    $0x8,%edi
  485d28:	mov    %r14,%rsi
  485d2b:	xor    %edx,%edx
  485d2d:	call   4c6c80 <<zopro_core::alloc::linux::Allocator>::allocate>
  485d32:	test   %rax,%rax
  485d35:	je     486159 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x979>
  485d3b:	movq   $0x400,0x78(%rsp)
  485d44:	mov    %rax,%r15
  485d47:	mov    0x70(%rsp),%rax
  485d4c:	mov    %rax,0x88(%rsp)
  485d54:	lea    0x20000(%rax),%rsi
  485d5b:	mov    %r15,%rdi
  485d5e:	mov    %r14,%rdx
  485d61:	call   *0x6f969(%rip)        # 4f56d0 <memcpy@GLIBC_2.14>
  485d67:	vmovdqu -0x47418f(%rip),%ymm0        # 11be0 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0xe0>
  485d6f:	xor    %eax,%eax
  485d71:	mov    %r15,0x20(%rsp)
  485d76:	cs nopw 0x0(%rax,%rax,1)
  485d80:	vmovups 0x40(%r15,%rax,1),%ymm1
  485d87:	vmovdqu %ymm0,0xc0(%rsp)
  485d90:	xor    %esi,%esi
  485d92:	xor    %edi,%edi
  485d94:	xor    %r8d,%r8d
  485d97:	xor    %r9d,%r9d
  485d9a:	lea    0xa0(%rsp),%rdx
  485da2:	lea    0xc0(%rsp),%rcx
  485daa:	vmovups %ymm1,0xa0(%rsp)
  485db3:	sub    (%rdx),%rsi
  485db6:	sbb    0x8(%rdx),%rdi
  485dba:	sbb    0x10(%rdx),%r8
  485dbe:	sbb    0x18(%rdx),%r9
  485dc2:	sbb    %rbp,%rbp
  485dc5:	mov    (%rcx),%r10
  485dc8:	mov    0x8(%rcx),%r11
  485dcc:	mov    0x10(%rcx),%rbx
  485dd0:	mov    0x18(%rcx),%r13
  485dd4:	and    %rbp,%r10
  485dd7:	and    %rbp,%r11
  485dda:	and    %rbp,%rbx
  485ddd:	and    %rbp,%r13
  485de0:	add    %r10,%rsi
  485de3:	adc    %r11,%rdi
  485de6:	adc    %rbx,%r8
  485de9:	adc    %r13,%r9
  485dec:	mov    %rsi,0x8(%rsp)
  485df1:	vmovdqu %ymm0,0xc0(%rsp)
  485dfa:	xor    %r10d,%r10d
  485dfd:	xor    %r11d,%r11d
  485e00:	xor    %ebx,%ebx
  485e02:	xor    %r13d,%r13d
  485e05:	lea    0xc0(%rsp),%rsi
  485e0d:	vmovups 0x60(%r15,%rax,1),%ymm1
  485e14:	lea    0xa0(%rsp),%r15
  485e1c:	vmovups %ymm1,0xa0(%rsp)
  485e25:	sub    (%r15),%r11
  485e28:	sbb    0x8(%r15),%rbx
  485e2c:	sbb    0x10(%r15),%r13
  485e30:	sbb    0x18(%r15),%r10
  485e34:	sbb    %r12,%r12
  485e37:	mov    (%rsi),%rbp
  485e3a:	mov    0x8(%rsi),%r14
  485e3e:	mov    0x10(%rsi),%rcx
  485e42:	mov    0x18(%rsi),%rdx
  485e46:	and    %r12,%rbp
  485e49:	and    %r12,%r14
  485e4c:	and    %r12,%rcx
  485e4f:	and    %r12,%rdx
  485e52:	add    %rbp,%r11
  485e55:	adc    %r14,%rbx
  485e58:	adc    %rcx,%r13
  485e5b:	adc    %rdx,%r10
  485e5e:	mov    0x20(%rsp),%r15
  485e63:	mov    0x8(%rsp),%rcx
  485e68:	mov    0x28(%rsp),%r14
  485e6d:	mov    %rcx,0x40(%r15,%rax,1)
  485e72:	mov    %rdi,0x48(%r15,%rax,1)
  485e77:	mov    %r8,0x50(%r15,%rax,1)
  485e7c:	mov    %r9,0x58(%r15,%rax,1)
  485e81:	mov    %r11,0x60(%r15,%rax,1)
  485e86:	mov    %rbx,0x68(%r15,%rax,1)
  485e8b:	mov    %r13,0x70(%r15,%rax,1)
  485e90:	mov    %r10,0x78(%r15,%rax,1)
  485e95:	sub    $0xffffffffffffff80,%rax
  485e99:	cmp    %rax,%r14
  485e9c:	jne    485d80 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x5a0>
  485ea2:	mov    0x90(%rsp),%rax
  485eaa:	movq   $0x400,0xa0(%rsp)
  485eb6:	mov    %rax,0xc0(%rsp)
  485ebe:	cmp    $0x400,%rax
  485ec4:	jne    486136 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x956>
  485eca:	mov    0x88(%rsp),%rax
  485ed2:	mov    0x98(%rsp),%rbx
  485eda:	mov    0x40(%rsp),%rbp
  485edf:	mov    %rax,0xc0(%rsp)
  485ee7:	mov    %fs:0xffffffffffffffc0,%rax
  485ef0:	movq   $0x400,0xc8(%rsp)
  485efc:	movq   $0x200,0xd0(%rsp)
  485f08:	mov    %r15,0xd8(%rsp)
  485f10:	movq   $0x400,0xe0(%rsp)
  485f1c:	movq   $0x200,0xe8(%rsp)
  485f28:	test   %rax,%rax
  485f2b:	je     485f61 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x781>
  485f2d:	add    $0x110,%rax
  485f33:	jmp    485f69 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x789>
  485f35:	mov    $0x20001,%edi
  485f3a:	call   *0x6f7c0(%rip)        # 4f5700 <malloc@GLIBC_2.2.5>
  485f40:	test   %rax,%rax
  485f43:	jne    485a6a <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x28a>
  485f49:	mov    $0x20000,%r12d
  485f4f:	mov    $0x8,%edi
  485f54:	mov    %r12,%rsi
  485f57:	call   ee951 <alloc::raw_vec::handle_error>
  485f5c:	jmp    486170 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x990>
  485f61:	vzeroupper
  485f64:	call   105550 <rayon_core::registry::global_registry>
  485f69:	mov    (%rax),%rax
  485f6c:	mov    0x208(%rax),%rdx
  485f73:	lea    0xc0(%rsp),%r8
  485f7b:	lea    0x17(%rsp),%r9
  485f80:	mov    $0x2,%edi
  485f85:	mov    $0x1,%ecx
  485f8a:	xor    %esi,%esi
  485f8c:	vzeroupper
  485f8f:	call   2508e0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::zip::ZipProducer<rayon::slice::chunks::ChunksMutProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::slice::chunks::ChunksProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::merge_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>
  485f94:	vmovdqu 0x68(%rsp),%xmm0
  485f9a:	mov    0x78(%rsp),%rax
  485f9f:	cmpb   $0x1,(%r15,%r14,1)
  485fa4:	mov    %rax,0x10(%rbx)
  485fa8:	vmovdqu %xmm0,(%rbx)
  485fac:	jne    485fbe <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x7de>
  485fae:	mov    $0x20001,%esi
  485fb3:	mov    %r15,%rdi
  485fb6:	call   *0x6f72c(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  485fbc:	jmp    485fc7 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x7e7>
  485fbe:	mov    %r15,%rdi
  485fc1:	call   *0x6f729(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  485fc7:	mov    0x18(%rsp),%rbx
  485fcc:	mov    0x60(%rsp),%rdi
  485fd1:	movabs $0x7ffffffffffffff9,%rax
  485fdb:	cmp    $0x1ffff,%rbp
  485fe2:	jbe    486006 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x826>
  485fe4:	mov    %rbp,%rsi
  485fe7:	or     $0x1,%rsi
  485feb:	add    $0x4,%rax
  485fef:	cmp    %rax,%rsi
  485ff2:	jae    48617a <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x99a>
  485ff8:	cmpb   $0x1,(%rdi,%rbp,1)
  485ffc:	jne    486008 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x828>
  485ffe:	call   *0x6f6e4(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  486004:	jmp    48600e <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x82e>
  486006:	jmp    486008 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x828>
  486008:	call   *0x6f6e2(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  48600e:	mov    %rbx,%rdi
  486011:	call   *0x6f6d9(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  486017:	mov    0x48(%rsp),%rax
  48601c:	test   %rax,%rax
  48601f:	je     48606d <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x88d>
  486021:	cmp    $0x3fff,%rax
  486027:	jbe    48605d <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x87d>
  486029:	shl    $0x3,%rax
  48602d:	movabs $0x7ffffffffffffff9,%rcx
  486037:	mov    %rax,%rsi
  48603a:	or     $0x1,%rsi
  48603e:	cmp    %rcx,%rsi
  486041:	jae    486172 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x992>
  486047:	mov    0x38(%rsp),%rdi
  48604c:	cmpb   $0x1,(%rdi,%rax,1)
  486050:	jne    486064 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x884>
  486052:	vzeroupper
  486055:	call   *0x6f68d(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  48605b:	jmp    48606d <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x88d>
  48605d:	mov    0x38(%rsp),%rdi
  486062:	jmp    486064 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x884>
  486064:	vzeroupper
  486067:	call   *0x6f683(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  48606d:	mov    0x30(%rsp),%rdi
  486072:	vzeroupper
  486075:	call   *0x6f675(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  48607b:	add    $0x148,%rsp
  486082:	pop    %rbx
  486083:	pop    %r12
  486085:	pop    %r13
  486087:	pop    %r14
  486089:	pop    %r15
  48608b:	pop    %rbp
  48608c:	ret
  48608d:	lea    0x6e0fc(%rip),%rdx        # 4f4190 <__frame_dummy_init_array_entry+0x7830>
  486094:	mov    $0x400,%edi
  486099:	call   eef1c <<alloc::vec::Vec<_, _>>::split_off::assert_failed>
  48609e:	jmp    486170 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x990>
  4860a3:	lea    0x6e12e(%rip),%rdx        # 4f41d8 <__frame_dummy_init_array_entry+0x7878>
  4860aa:	mov    $0x800,%esi
  4860af:	mov    $0xffffffffffffffff,%rdi
  4860b6:	vzeroupper
  4860b9:	call   f07db <core::panicking::panic_bounds_check>
  4860be:	jmp    486170 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x990>
  4860c3:	mov    $0x800,%r13d
  4860c9:	mov    $0xffffffffffffffff,%rdi
  4860d0:	mov    %r13,%rsi
  4860d3:	vzeroupper
  4860d6:	call   f07db <core::panicking::panic_bounds_check>
  4860db:	jmp    486170 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x990>
  4860e0:	mov    $0x8,%edi
  4860e5:	mov    $0x4000,%esi
  4860ea:	call   ee951 <alloc::raw_vec::handle_error>
  4860ef:	mov    $0x8,%edi
  4860f4:	mov    $0x4008,%esi
  4860f9:	call   ee951 <alloc::raw_vec::handle_error>
  4860fe:	jmp    486170 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x990>
  486100:	mov    $0x20000,%esi
  486105:	add    0x70(%rsp),%rsi
  48610a:	mov    $0x8,%eax
  48610f:	mov    $0x8,%edi
  486114:	mov    %r14,%rdx
  486117:	mov    %rax,0x20(%rsp)
  48611c:	call   *0x6f5ae(%rip)        # 4f56d0 <memcpy@GLIBC_2.14>
  486122:	movq   $0x400,0xa0(%rsp)
  48612e:	mov    %r12,0xc0(%rsp)
  486136:	lea    0x6925b(%rip),%r8        # 4ef398 <__frame_dummy_init_array_entry+0x2a38>
  48613d:	lea    0xa0(%rsp),%rdi
  486145:	lea    0xc0(%rsp),%rsi
  48614d:	xor    %edx,%edx
  48614f:	vzeroupper
  486152:	call   f0b82 <core::panicking::assert_failed::<usize, usize>>
  486157:	jmp    486170 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x990>
  486159:	mov    0x18(%rsp),%rbx
  48615e:	mov    0x40(%rsp),%rbp
  486163:	mov    $0x8,%edi
  486168:	mov    %r14,%rsi
  48616b:	call   ee951 <alloc::raw_vec::handle_error>
  486170:	ud2
  486172:	vzeroupper
  486175:	call   111490 <std::process::abort>
  48617a:	call   111490 <std::process::abort>
  48617f:	cmpq   $0x400,0x80(%rsp)
  48618b:	mov    0x18(%rsp),%rbx
  486190:	mov    0x40(%rsp),%rbp
  486195:	mov    %rax,%r12
  486198:	jne    4861a4 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x9c4>
  48619a:	jmp    4861df <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x9ff>
  48619c:	mov    0x18(%rsp),%rbx
  4861a1:	mov    %rax,%r12
  4861a4:	mov    0x20(%rsp),%rdi
  4861a9:	mov    0x28(%rsp),%rsi
  4861ae:	mov    $0x8,%edx
  4861b3:	call   e9c50 <__rustc::__rust_dealloc>
  4861b8:	jmp    4861df <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x9ff>
  4861ba:	jmp    4861d7 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0x9f7>
  4861bc:	mov    %rax,%r12
  4861bf:	jmp    4861fc <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa1c>
  4861c1:	mov    %rax,%r12
  4861c4:	mov    0x30(%rsp),%rdi
  4861c9:	call   *0x6f521(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  4861cf:	mov    %r12,%rdi
  4861d2:	call   4eb910 <_Unwind_Resume@plt>
  4861d7:	mov    %rax,%r12
  4861da:	jmp    48621c <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa3c>
  4861dc:	mov    %rax,%r12
  4861df:	mov    0x68(%rsp),%rsi
  4861e4:	test   %rsi,%rsi
  4861e7:	je     4861fc <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa1c>
  4861e9:	mov    0x70(%rsp),%rdi
  4861ee:	shl    $0x7,%rsi
  4861f2:	mov    $0x8,%edx
  4861f7:	call   e9c50 <__rustc::__rust_dealloc>
  4861fc:	mov    0x60(%rsp),%rdi
  486201:	mov    $0x4,%edx
  486206:	mov    %rbp,%rsi
  486209:	call   e9c50 <__rustc::__rust_dealloc>
  48620e:	jmp    486213 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa33>
  486210:	mov    %rax,%r12
  486213:	mov    %rbx,%rdi
  486216:	call   *0x6f4d4(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  48621c:	mov    0x48(%rsp),%rax
  486221:	test   %rax,%rax
  486224:	je     486272 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa92>
  486226:	mov    0x50(%rsp),%rdi
  48622b:	cmp    $0x3fff,%rax
  486231:	jbe    48626c <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xa8c>
  486233:	shl    $0x3,%rax
  486237:	movabs $0x7ffffffffffffff9,%rcx
  486241:	mov    %rax,%rsi
  486244:	or     $0x1,%rsi
  486248:	cmp    %rcx,%rsi
  48624b:	jae    486285 <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xaa5>
  48624d:	cmpb   $0x1,(%rdi,%rax,1)
  486251:	jne    48628a <zopro_core::msm_affine::g2_borrowed::gls4::top_component+0xaaa>
  486253:	call   *0x6f48f(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  486259:	mov    0x30(%rsp),%rdi
  48625e:	call   *0x6f48c(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  486264:	mov    %r12,%rdi
  486267:	call   4eb910 <_Unwind_Resume@plt>
  48626c:	call   *0x6f47e(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  486272:	mov    0x30(%rsp),%rdi
  486277:	call   *0x6f473(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  48627d:	mov    %r12,%rdi
  486280:	call   4eb910 <_Unwind_Resume@plt>
  486285:	call   111490 <std::process::abort>
  48628a:	call   *0x6f460(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  486290:	mov    0x30(%rsp),%rdi
  486295:	call   *0x6f455(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  48629b:	mov    %r12,%rdi
  48629e:	call   4eb910 <_Unwind_Resume@plt>
