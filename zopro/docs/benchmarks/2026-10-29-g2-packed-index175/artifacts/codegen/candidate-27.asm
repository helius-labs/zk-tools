
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000028e7e0 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>>:
  28e7e0:	push   %rbp
  28e7e1:	push   %r15
  28e7e3:	push   %r14
  28e7e5:	push   %r13
  28e7e7:	push   %r12
  28e7e9:	push   %rbx
  28e7ea:	sub    $0x6c8,%rsp
  28e7f1:	mov    0x708(%rsp),%rax
  28e7f9:	mov    0x718(%rsp),%r15
  28e801:	mov    0x710(%rsp),%r14
  28e809:	mov    0x700(%rsp),%r10
  28e811:	lea    -0x1(%rax),%r11
  28e815:	cmp    $0x2,%r11
  28e819:	jb     28e93e <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x15e>
  28e81f:	shr    $1,%r11
  28e822:	mov    %r11,0x270(%rsp)
  28e82a:	cmp    %rax,%r11
  28e82d:	jae    28f519 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd39>
  28e833:	mov    (%r10,%r11,8),%r12
  28e837:	sub    (%r10),%r12
  28e83a:	sub    %r12,%r15
  28e83d:	jb     28f4ac <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xccc>
  28e843:	mov    %r12,%r11
  28e846:	shl    $0x7,%r11
  28e84a:	lea    0x270(%rsp),%rbx
  28e852:	mov    %rdi,0x40(%rsp)
  28e857:	mov    %rsi,0x48(%rsp)
  28e85c:	mov    %rdx,0x50(%rsp)
  28e861:	mov    %rcx,0x58(%rsp)
  28e866:	mov    %r8,0x60(%rsp)
  28e86b:	mov    %r9,0x68(%rsp)
  28e870:	mov    %r10,0x70(%rsp)
  28e875:	mov    %rax,0x78(%rsp)
  28e87a:	add    %r14,%r11
  28e87d:	mov    %rbx,0x80(%rsp)
  28e885:	mov    %r11,0x88(%rsp)
  28e88d:	mov    %r15,0x90(%rsp)
  28e895:	mov    %rdi,0x98(%rsp)
  28e89d:	mov    %rsi,0xa0(%rsp)
  28e8a5:	mov    %fs:0xffffffffffffffc0,%rsi
  28e8ae:	mov    %rdx,0xa8(%rsp)
  28e8b6:	mov    %rcx,0xb0(%rsp)
  28e8be:	mov    %r8,0xb8(%rsp)
  28e8c6:	mov    %r9,0xc0(%rsp)
  28e8ce:	mov    %r10,0xc8(%rsp)
  28e8d6:	mov    %rax,0xd0(%rsp)
  28e8de:	mov    %rbx,0xd8(%rsp)
  28e8e6:	mov    %r14,0xe0(%rsp)
  28e8ee:	mov    %r12,0xe8(%rsp)
  28e8f6:	test   %rsi,%rsi
  28e8f9:	jne    28e922 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x142>
  28e8fb:	call   104a30 <rayon_core::registry::global_registry>
  28e900:	mov    (%rax),%rdi
  28e903:	mov    %fs:0xffffffffffffffc0,%rsi
  28e90c:	test   %rsi,%rsi
  28e90f:	je     28f4c4 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xce4>
  28e915:	cmp    %rdi,0x110(%rsi)
  28e91c:	jne    28f4d7 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xcf7>
  28e922:	lea    0x40(%rsp),%rdi
  28e927:	call   2912b0 <rayon_core::join::join_context::<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}>
  28e92c:	add    $0x6c8,%rsp
  28e933:	pop    %rbx
  28e934:	pop    %r12
  28e936:	pop    %r13
  28e938:	pop    %r14
  28e93a:	pop    %r15
  28e93c:	pop    %rbp
  28e93d:	ret
  28e93e:	cmp    $0x1,%rax
  28e942:	je     28f52b <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd4b>
  28e948:	mov    (%r10),%r11
  28e94b:	mov    0x8(%r10),%r13
  28e94f:	cmp    %r11,%r13
  28e952:	jb     28f47f <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc9f>
  28e958:	cmp    %r9,%r13
  28e95b:	ja     28f47f <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc9f>
  28e961:	cmp    %r11,%r13
  28e964:	je     28f541 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd61>
  28e96a:	shl    $0x4,%r11
  28e96e:	shl    $0x4,%r13
  28e972:	mov    (%r8,%r11,1),%r14
  28e976:	mov    -0x8(%r8,%r13,1),%r15
  28e97b:	cmp    %r14,%r15
  28e97e:	jb     28f494 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xcb4>
  28e984:	cmp    %rcx,%r15
  28e987:	ja     28f494 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xcb4>
  28e98d:	mov    %r8,0x20(%rsp)
  28e992:	lea    0x0(,%r14,4),%r8
  28e99a:	shl    $0x2,%r15
  28e99e:	movabs $0x7fffffffffffffc1,%rcx
  28e9a8:	movabs $0x7fffffffffffffd,%rax
  28e9b2:	mov    %r15,%rbp
  28e9b5:	add    $0x37,%rcx
  28e9b9:	sub    %r8,%rbp
  28e9bc:	mov    %rbp,%r12
  28e9bf:	shl    $0x5,%r12
  28e9c3:	cmp    %rax,%rbp
  28e9c6:	setb   %al
  28e9c9:	cmp    %rcx,%r12
  28e9cc:	setbe  %cl
  28e9cf:	test   %cl,%al
  28e9d1:	jne    28e9d8 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x1f8>
  28e9d3:	call   edf50 <alloc::raw_vec::capacity_overflow>
  28e9d8:	shr    $0x2,%rbp
  28e9dc:	mov    %r11,0x10(%rsp)
  28e9e1:	test   %r12,%r12
  28e9e4:	je     28ea42 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x262>
  28e9e6:	mov    %rdi,0x18(%rsp)
  28e9eb:	mov    $0x8,%edi
  28e9f0:	mov    %r13,0x30(%rsp)
  28e9f5:	mov    %rsi,%r13
  28e9f8:	mov    %rdx,%rbx
  28e9fb:	mov    %r12,%rsi
  28e9fe:	xor    %edx,%edx
  28ea00:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  28ea05:	mov    %r13,%rsi
  28ea08:	mov    0x10(%rsp),%r11
  28ea0d:	mov    0x18(%rsp),%rdi
  28ea12:	mov    0x30(%rsp),%r13
  28ea17:	lea    0x0(,%r14,4),%r8
  28ea1f:	mov    %rbx,%rdx
  28ea22:	mov    %rax,%rbx
  28ea25:	mov    %rbp,%rax
  28ea28:	test   %rbx,%rbx
  28ea2b:	je     28f573 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd93>
  28ea31:	mov    %rax,0x28(%rsp)
  28ea36:	cmp    %r8,%r15
  28ea39:	jne    28ea53 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x273>
  28ea3b:	xor    %ebp,%ebp
  28ea3d:	jmp    28ebaa <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x3ca>
  28ea42:	mov    $0x8,%ebx
  28ea47:	xor    %eax,%eax
  28ea49:	mov    %rax,0x28(%rsp)
  28ea4e:	cmp    %r8,%r15
  28ea51:	je     28ea3b <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x25b>
  28ea53:	lea    (%rdx,%r14,4),%rcx
  28ea57:	lea    0x110(%rsp),%rdx
  28ea5f:	lea    0x150(%rsp),%r8
  28ea67:	lea    0x1b0(%rsp),%r9
  28ea6f:	xor    %r10d,%r10d
  28ea72:	vpxor  %xmm0,%xmm0,%xmm0
  28ea76:	mov    %rbx,%r11
  28ea79:	nopl   0x0(%rax)
  28ea80:	mov    (%rcx,%r10,4),%eax
  28ea84:	cmp    %rax,%rsi
  28ea87:	jbe    28f4ef <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd0f>
  28ea8d:	shl    $0x7,%rax
  28ea91:	vmovups (%rdi,%rax,1),%zmm1
  28ea98:	vmovups 0x40(%rdi,%rax,1),%zmm2
  28eaa0:	vmovups %zmm1,0x270(%rsp)
  28eaab:	vmovdqu (%rdi,%rax,1),%ymm1
  28eab0:	vmovups %zmm2,0x2b0(%rsp)
  28eabb:	vptest %ymm1,%ymm1
  28eac0:	jne    28eb00 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x320>
  28eac2:	vmovdqu 0x290(%rsp),%ymm1
  28eacb:	vptest %ymm1,%ymm1
  28ead0:	jne    28eb00 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x320>
  28ead2:	vmovdqu 0x2b0(%rsp),%ymm1
  28eadb:	vptest %ymm1,%ymm1
  28eae0:	jne    28eb00 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x320>
  28eae2:	vmovdqu 0x2d0(%rsp),%ymm1
  28eaeb:	vptest %ymm1,%ymm1
  28eaf0:	je     28eb8d <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x3ad>
  28eaf6:	cs nopw 0x0(%rax,%rax,1)
  28eb00:	add    %rdi,%rax
  28eb03:	vmovdqu 0x20(%rax),%ymm1
  28eb08:	vmovups 0x60(%rax),%ymm2
  28eb0d:	vmovups (%rax),%ymm3
  28eb11:	vmovdqu 0x40(%rax),%ymm4
  28eb16:	vmovdqu %ymm1,(%rdx)
  28eb1a:	vmovups %ymm3,0xf0(%rsp)
  28eb23:	vmovups %ymm2,(%r8)
  28eb28:	vmovdqu %ymm4,0x130(%rsp)
  28eb31:	vmovdqu64 0xf0(%rsp),%zmm3
  28eb3c:	vmovups 0x130(%rsp),%zmm2
  28eb47:	vmovdqu64 %zmm3,0x170(%rsp)
  28eb52:	vmovups %zmm2,(%r9)
  28eb58:	vmovdqu64 0x170(%rsp),%zmm1
  28eb63:	vmovdqu64 0x1b0(%rsp),%zmm2
  28eb6e:	inc    %r10
  28eb71:	vmovdqu64 %zmm2,0x40(%r11)
  28eb78:	vmovdqu64 %zmm1,(%r11)
  28eb7e:	sub    $0xffffffffffffff80,%r11
  28eb82:	cmp    %r10,%rbp
  28eb85:	jne    28ea80 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x2a0>
  28eb8b:	jmp    28eba5 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x3c5>
  28eb8d:	vmovdqu64 %zmm0,0x1b0(%rsp)
  28eb98:	vmovdqu64 %zmm0,0x170(%rsp)
  28eba3:	jmp    28eb58 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x378>
  28eba5:	mov    0x10(%rsp),%r11
  28ebaa:	sub    %r11,%r13
  28ebad:	mov    $0x8,%edi
  28ebb2:	xor    %edx,%edx
  28ebb4:	mov    %r13,%rsi
  28ebb7:	shr    $1,%rsi
  28ebba:	mov    %rsi,0x18(%rsp)
  28ebbf:	vzeroupper
  28ebc2:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  28ebc7:	test   %rax,%rax
  28ebca:	je     28f551 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd71>
  28ebd0:	mov    %r13,%r15
  28ebd3:	shr    $0x4,%r15
  28ebd7:	mov    %rax,%r12
  28ebda:	cmp    $0x50,%r13
  28ebde:	jae    28ebec <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x40c>
  28ebe0:	mov    0x10(%rsp),%rdi
  28ebe5:	xor    %eax,%eax
  28ebe7:	jmp    28ed16 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x536>
  28ebec:	mov    0x10(%rsp),%rdi
  28ebf1:	cmp    $0x210,%r13
  28ebf8:	jae    28ec01 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x421>
  28ebfa:	xor    %eax,%eax
  28ebfc:	jmp    28ecc3 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x4e3>
  28ec01:	vmovdqa64 -0x2819cb(%rip),%zmm1        # d240 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x1700>
  28ec0b:	mov    0x20(%rsp),%rdx
  28ec10:	mov    %r15d,%eax
  28ec13:	and    $0x1f,%eax
  28ec16:	mov    $0x20,%ecx
  28ec1b:	vpbroadcastq %r14,%zmm0
  28ec21:	cmovne %rax,%rcx
  28ec25:	mov    %r15,%rax
  28ec28:	xor    %esi,%esi
  28ec2a:	sub    %rcx,%rax
  28ec2d:	lea    0x180(%rdi,%rdx,1),%rdx
  28ec35:	data16 cs nopw 0x0(%rax,%rax,1)
  28ec40:	vmovdqu64 -0x180(%rdx),%zmm2
  28ec47:	vmovdqu64 -0x100(%rdx),%zmm3
  28ec4e:	vmovdqu64 -0x80(%rdx),%zmm4
  28ec55:	vmovdqu64 (%rdx),%zmm5
  28ec5b:	vpermt2q -0x140(%rdx),%zmm1,%zmm2
  28ec62:	vpermt2q -0xc0(%rdx),%zmm1,%zmm3
  28ec69:	vpermt2q -0x40(%rdx),%zmm1,%zmm4
  28ec70:	vpermt2q 0x40(%rdx),%zmm1,%zmm5
  28ec77:	add    $0x200,%rdx
  28ec7e:	vpsubq %zmm0,%zmm2,%zmm2
  28ec84:	vpsubq %zmm0,%zmm3,%zmm3
  28ec8a:	vpsubq %zmm0,%zmm4,%zmm4
  28ec90:	vpsubq %zmm0,%zmm5,%zmm5
  28ec96:	vmovdqu64 %zmm2,(%r12,%rsi,8)
  28ec9d:	vmovdqu64 %zmm3,0x40(%r12,%rsi,8)
  28eca5:	vmovdqu64 %zmm4,0x80(%r12,%rsi,8)
  28ecad:	vmovdqu64 %zmm5,0xc0(%r12,%rsi,8)
  28ecb5:	add    $0x20,%rsi
  28ecb9:	cmp    %rsi,%rax
  28ecbc:	jne    28ec40 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x460>
  28ecbe:	cmp    $0x4,%ecx
  28ecc1:	jbe    28ed16 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x536>
  28ecc3:	mov    %rax,%rcx
  28ecc6:	mov    %r15d,%eax
  28ecc9:	and    $0x3,%eax
  28eccc:	mov    $0x4,%edx
  28ecd1:	vpbroadcastq %r14,%ymm0
  28ecd7:	cmovne %rax,%rdx
  28ecdb:	mov    %r15,%rax
  28ecde:	sub    %rdx,%rax
  28ece1:	mov    %rcx,%rdx
  28ece4:	shl    $0x4,%rdx
  28ece8:	add    %rdi,%rdx
  28eceb:	add    0x20(%rsp),%rdx
  28ecf0:	vmovdqu (%rdx),%ymm1
  28ecf4:	vpunpcklqdq 0x20(%rdx),%ymm1,%ymm1
  28ecf9:	add    $0x40,%rdx
  28ecfd:	vpermq $0xd8,%ymm1,%ymm1
  28ed03:	vpsubq %ymm0,%ymm1,%ymm1
  28ed07:	vmovdqu %ymm1,(%r12,%rcx,8)
  28ed0d:	add    $0x4,%rcx
  28ed11:	cmp    %rcx,%rax
  28ed14:	jne    28ecf0 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x510>
  28ed16:	mov    %rax,%rcx
  28ed19:	shl    $0x4,%rcx
  28ed1d:	add    %rdi,%rcx
  28ed20:	add    0x20(%rsp),%rcx
  28ed25:	data16 cs nopw 0x0(%rax,%rax,1)
  28ed30:	mov    (%rcx),%rdx
  28ed33:	add    $0x10,%rcx
  28ed37:	sub    %r14,%rdx
  28ed3a:	mov    %rdx,(%r12,%rax,8)
  28ed3e:	inc    %rax
  28ed41:	cmp    %rax,%r15
  28ed44:	jne    28ed30 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x550>
  28ed46:	mov    0x18(%rsp),%rsi
  28ed4b:	mov    $0x8,%edi
  28ed50:	xor    %edx,%edx
  28ed52:	vzeroupper
  28ed55:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  28ed5a:	test   %rax,%rax
  28ed5d:	je     28f562 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd82>
  28ed63:	mov    %rax,%r14
  28ed66:	cmp    $0x40,%r13
  28ed6a:	jae    28ed78 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x598>
  28ed6c:	mov    0x10(%rsp),%rdi
  28ed71:	xor    %eax,%eax
  28ed73:	jmp    28eef9 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x719>
  28ed78:	mov    0x10(%rsp),%rdi
  28ed7d:	movabs $0xfffffffffffffe0,%rcx
  28ed87:	cmp    $0x200,%r13
  28ed8e:	jae    28ed97 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x5b7>
  28ed90:	xor    %eax,%eax
  28ed92:	jmp    28ee9c <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x6bc>
  28ed97:	vmovdqa64 -0x281b61(%rip),%zmm0        # d240 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x1700>
  28eda1:	vmovdqa64 -0x281b2b(%rip),%zmm1        # d280 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x1740>
  28edab:	mov    0x20(%rsp),%rdx
  28edb0:	mov    %r15,%rax
  28edb3:	and    %rcx,%rax
  28edb6:	xor    %esi,%esi
  28edb8:	lea    0x180(%rdi,%rdx,1),%rdx
  28edc0:	vmovdqu64 -0x180(%rdx),%zmm2
  28edc7:	vmovdqu64 -0x140(%rdx),%zmm3
  28edce:	vmovdqu64 -0x100(%rdx),%zmm4
  28edd5:	vmovdqu64 -0xc0(%rdx),%zmm5
  28eddc:	vmovdqu64 -0x40(%rdx),%zmm7
  28ede3:	vmovdqu64 (%rdx),%zmm8
  28ede9:	vmovdqa64 %zmm2,%zmm6
  28edef:	vpermt2q %zmm3,%zmm0,%zmm6
  28edf5:	vpermt2q %zmm3,%zmm1,%zmm2
  28edfb:	vpsubq %zmm6,%zmm2,%zmm2
  28ee01:	vmovdqa64 %zmm4,%zmm6
  28ee07:	vpermt2q %zmm5,%zmm0,%zmm6
  28ee0d:	vpermt2q %zmm5,%zmm1,%zmm4
  28ee13:	vmovdqu64 -0x80(%rdx),%zmm5
  28ee1a:	vmovdqu64 %zmm2,(%r14,%rsi,8)
  28ee21:	vpsubq %zmm6,%zmm4,%zmm3
  28ee27:	vmovdqu64 %zmm3,0x40(%r14,%rsi,8)
  28ee2f:	vmovdqa64 %zmm5,%zmm6
  28ee35:	vpermt2q %zmm7,%zmm0,%zmm6
  28ee3b:	vpermt2q %zmm7,%zmm1,%zmm5
  28ee41:	vmovdqa64 %zmm8,%zmm7
  28ee47:	vpsubq %zmm6,%zmm5,%zmm4
  28ee4d:	vmovdqu64 0x40(%rdx),%zmm5
  28ee54:	add    $0x200,%rdx
  28ee5b:	vmovdqu64 %zmm4,0x80(%r14,%rsi,8)
  28ee63:	vpermt2q %zmm5,%zmm0,%zmm7
  28ee69:	vpermt2q %zmm5,%zmm1,%zmm8
  28ee6f:	vpsubq %zmm7,%zmm8,%zmm5
  28ee75:	vmovdqu64 %zmm5,0xc0(%r14,%rsi,8)
  28ee7d:	add    $0x20,%rsi
  28ee81:	cmp    %rsi,%rax
  28ee84:	jne    28edc0 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x5e0>
  28ee8a:	cmp    %rax,%r15
  28ee8d:	je     28ef27 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x747>
  28ee93:	test   $0x1c0,%r13d
  28ee9a:	je     28eef9 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x719>
  28ee9c:	add    $0x1c,%rcx
  28eea0:	mov    %rax,%rdx
  28eea3:	mov    %rcx,%rax
  28eea6:	mov    %rdx,%rcx
  28eea9:	shl    $0x4,%rcx
  28eead:	and    %r15,%rax
  28eeb0:	add    %rdi,%rcx
  28eeb3:	add    0x20(%rsp),%rcx
  28eeb8:	nopl   0x0(%rax,%rax,1)
  28eec0:	vmovdqu (%rcx),%ymm0
  28eec4:	vmovdqu 0x20(%rcx),%ymm1
  28eec9:	add    $0x40,%rcx
  28eecd:	vpunpcklqdq %ymm1,%ymm0,%ymm2
  28eed1:	vpunpckhqdq %ymm1,%ymm0,%ymm0
  28eed5:	vpermq $0xd8,%ymm2,%ymm2
  28eedb:	vpermq $0xd8,%ymm0,%ymm0
  28eee1:	vpsubq %ymm2,%ymm0,%ymm0
  28eee5:	vmovdqu %ymm0,(%r14,%rdx,8)
  28eeeb:	add    $0x4,%rdx
  28eeef:	cmp    %rdx,%rax
  28eef2:	jne    28eec0 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x6e0>
  28eef4:	cmp    %rax,%r15
  28eef7:	je     28ef27 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x747>
  28eef9:	mov    0x20(%rsp),%rdx
  28eefe:	mov    %rax,%rcx
  28ef01:	shl    $0x4,%rcx
  28ef05:	add    %rdi,%rcx
  28ef08:	lea    0x8(%rdx,%rcx,1),%rcx
  28ef0d:	nopl   (%rax)
  28ef10:	mov    (%rcx),%rdx
  28ef13:	sub    -0x8(%rcx),%rdx
  28ef17:	add    $0x10,%rcx
  28ef1b:	mov    %rdx,(%r14,%rax,8)
  28ef1f:	inc    %rax
  28ef22:	cmp    %rax,%r15
  28ef25:	jne    28ef10 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x730>
  28ef27:	lea    -0x2048f7(%rip),%rsi        # 8a637 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x22f>
  28ef2e:	lea    0x270(%rsp),%rdi
  28ef36:	mov    $0x1d,%edx
  28ef3b:	vzeroupper
  28ef3e:	call   143720 <std::sys::env::unix::getenv>
  28ef43:	mov    0x270(%rsp),%rax
  28ef4b:	mov    %r13,0x30(%rsp)
  28ef50:	cmp    $0xffffffffffffffff,%rax
  28ef54:	je     28f015 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x835>
  28ef5a:	mov    0x278(%rsp),%rsi
  28ef62:	mov    0x280(%rsp),%rdx
  28ef6a:	lea    0x270(%rsp),%rdi
  28ef72:	mov    %rax,0x20(%rsp)
  28ef77:	mov    %rsi,0x10(%rsp)
  28ef7c:	mov    %rdx,0x38(%rsp)
  28ef81:	call   f0a00 <core::str::converts::from_utf8>
  28ef86:	cmpb   $0x0,0x270(%rsp)
  28ef8e:	je     28efbf <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x7df>
  28ef90:	mov    0x20(%rsp),%rax
  28ef95:	mov    $0x1,%cl
  28ef97:	test   %rax,%rax
  28ef9a:	jle    28f017 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x837>
  28ef9c:	cmp    $0x1ffff,%rax
  28efa2:	jbe    28f00a <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x82a>
  28efa4:	mov    0x10(%rsp),%rdi
  28efa9:	mov    %rax,%rsi
  28efac:	inc    %rsi
  28efaf:	js     28f002 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x822>
  28efb1:	cmpb   $0x1,(%rdi,%rax,1)
  28efb5:	jne    28f00f <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x82f>
  28efb7:	call   *0x25bc8b(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28efbd:	jmp    28f015 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x835>
  28efbf:	cmpq   $0x1,0x38(%rsp)
  28efc5:	mov    0x10(%rsp),%rdi
  28efca:	mov    0x20(%rsp),%rsi
  28efcf:	mov    $0x1,%cl
  28efd1:	jne    28efd9 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x7f9>
  28efd3:	cmpb   $0x30,(%rdi)
  28efd6:	setne  %cl
  28efd9:	test   %rsi,%rsi
  28efdc:	je     28f017 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x837>
  28efde:	cmp    $0x1ffff,%rsi
  28efe5:	jbe    28f454 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc74>
  28efeb:	movabs $0x7fffffffffffffc1,%rax
  28eff5:	add    $0x3e,%rax
  28eff9:	cmp    %rax,%rsi
  28effc:	jb     28f465 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc85>
  28f002:	vzeroupper
  28f005:	call   110970 <std::process::abort>
  28f00a:	mov    0x10(%rsp),%rdi
  28f00f:	call   *0x25bc3b(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f015:	mov    $0x1,%cl
  28f017:	lea    0x2b0(%rsp),%rdi
  28f01f:	mov    $0x408,%edx
  28f024:	mov    %cl,0x6b8(%rsp)
  28f02b:	movq   $0x0,0x270(%rsp)
  28f037:	xor    %esi,%esi
  28f039:	movq   $0x8,0x278(%rsp)
  28f045:	vpxor  %xmm0,%xmm0,%xmm0
  28f049:	vmovdqu %xmm0,0x280(%rsp)
  28f052:	movq   $0x40,0x290(%rsp)
  28f05e:	vmovdqu %xmm0,0x298(%rsp)
  28f067:	movq   $0x40,0x2a8(%rsp)
  28f073:	call   *0x25bc57(%rip)        # 4eacd0 <memset@GLIBC_2.2.5>
  28f079:	lea    (%r14,%r15,8),%r13
  28f07d:	mov    %r14,%rax
  28f080:	cmp    %r13,%rax
  28f083:	je     28f0b7 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x8d7>
  28f085:	cmpq   $0x1,(%rax)
  28f089:	lea    0x8(%rax),%rax
  28f08d:	jbe    28f080 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x8a0>
  28f08f:	lea    0x270(%rsp),%rax
  28f097:	mov    %rbx,%rdi
  28f09a:	mov    %rbp,%rsi
  28f09d:	mov    %r12,%rdx
  28f0a0:	mov    %r15,%rcx
  28f0a3:	mov    %r14,%r8
  28f0a6:	mov    %r15,%r9
  28f0a9:	mov    %rax,(%rsp)
  28f0ad:	call   22cb80 <zopro_core::msm::reduce_layer::<zopro_bn254::curve::G2Config>>
  28f0b2:	mov    %r14,%rax
  28f0b5:	jmp    28f080 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x8a0>
  28f0b7:	mov    0x718(%rsp),%r13
  28f0bf:	mov    0x710(%rsp),%r11
  28f0c7:	test   %r13,%r13
  28f0ca:	je     28f25b <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xa7b>
  28f0d0:	shl    $0x7,%r13
  28f0d4:	lea    0x110(%rsp),%rax
  28f0dc:	lea    0x150(%rsp),%rcx
  28f0e4:	lea    0x230(%rsp),%rsi
  28f0ec:	lea    0x256f95(%rip),%rdx        # 4e6088 <__frame_dummy_init_array_entry+0x4228>
  28f0f3:	lea    0x256fa6(%rip),%r8        # 4e60a0 <__frame_dummy_init_array_entry+0x4240>
  28f0fa:	xor    %edi,%edi
  28f0fc:	vpxor  %xmm0,%xmm0,%xmm0
  28f100:	xor    %r9d,%r9d
  28f103:	jmp    28f1a2 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x9c2>
  28f108:	nopl   0x0(%rax,%rax,1)
  28f110:	add    %rbx,%r10
  28f113:	vmovups 0x20(%r10),%ymm1
  28f119:	vmovups %ymm1,(%rax)
  28f11d:	vmovups (%r10),%ymm1
  28f122:	vmovups %ymm1,0xf0(%rsp)
  28f12b:	vmovups 0x60(%r10),%ymm1
  28f131:	vmovdqu64 0xf0(%rsp),%zmm3
  28f13c:	vmovups %ymm1,(%rcx)
  28f140:	vmovdqu 0x40(%r10),%ymm1
  28f146:	vmovdqu64 %zmm3,0x1f0(%rsp)
  28f151:	vmovdqu %ymm1,0x130(%rsp)
  28f15a:	vmovdqu64 0x130(%rsp),%zmm2
  28f165:	vmovdqu64 %zmm2,(%rsi)
  28f16b:	vmovdqu64 0x1f0(%rsp),%zmm1
  28f176:	vmovdqu64 0x230(%rsp),%zmm2
  28f181:	lea    (%r11,%rdi,1),%r10
  28f185:	sub    $0xffffffffffffff80,%rdi
  28f189:	inc    %r9
  28f18c:	vmovdqu64 %zmm2,0x40(%r10)
  28f193:	vmovdqu64 %zmm1,(%r10)
  28f199:	cmp    %rdi,%r13
  28f19c:	je     28f25b <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xa7b>
  28f1a2:	cmp    %r9,%r15
  28f1a5:	je     28f4ea <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd0a>
  28f1ab:	cmpq   $0x0,(%r14,%r9,8)
  28f1b0:	je     28f240 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xa60>
  28f1b6:	mov    (%r12,%r9,8),%r10
  28f1ba:	cmp    %rbp,%r10
  28f1bd:	jae    28f503 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd23>
  28f1c3:	shl    $0x7,%r10
  28f1c7:	vmovups (%rbx,%r10,1),%zmm1
  28f1ce:	vmovdqu64 0x40(%rbx,%r10,1),%zmm2
  28f1d6:	vmovdqu64 %zmm2,0x1b0(%rsp)
  28f1e1:	vmovups %zmm1,0x170(%rsp)
  28f1ec:	vmovdqu (%rbx,%r10,1),%ymm1
  28f1f2:	vptest %ymm1,%ymm1
  28f1f7:	jne    28f110 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x930>
  28f1fd:	vmovdqu 0x190(%rsp),%ymm1
  28f206:	vptest %ymm1,%ymm1
  28f20b:	jne    28f110 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x930>
  28f211:	vmovdqu 0x1b0(%rsp),%ymm1
  28f21a:	vptest %ymm1,%ymm1
  28f21f:	jne    28f110 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x930>
  28f225:	vmovdqu 0x1d0(%rsp),%ymm1
  28f22e:	vptest %ymm1,%ymm1
  28f233:	jne    28f110 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x930>
  28f239:	nopl   0x0(%rax)
  28f240:	vmovdqu64 %zmm0,0x230(%rsp)
  28f24b:	vmovdqu64 %zmm0,0x1f0(%rsp)
  28f256:	jmp    28f16b <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x98b>
  28f25b:	mov    0x270(%rsp),%rax
  28f263:	movabs $0x7fffffffffffffc1,%r13
  28f26d:	test   %rax,%rax
  28f270:	je     28f2b4 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xad4>
  28f272:	mov    0x278(%rsp),%rdi
  28f27a:	cmp    $0x3fff,%rax
  28f280:	jbe    28f2ab <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xacb>
  28f282:	shl    $0x3,%rax
  28f286:	lea    0x38(%r13),%rcx
  28f28a:	mov    %rax,%rsi
  28f28d:	or     $0x1,%rsi
  28f291:	cmp    %rcx,%rsi
  28f294:	jae    28f002 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x822>
  28f29a:	cmpb   $0x1,(%rdi,%rax,1)
  28f29e:	jne    28f2ab <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xacb>
  28f2a0:	vzeroupper
  28f2a3:	call   *0x25b99f(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f2a9:	jmp    28f2b4 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xad4>
  28f2ab:	vzeroupper
  28f2ae:	call   *0x25b99c(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f2b4:	mov    0x288(%rsp),%rax
  28f2bc:	test   %rax,%rax
  28f2bf:	je     28f300 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xb20>
  28f2c1:	mov    0x290(%rsp),%rdi
  28f2c9:	cmp    $0x2d,%rax
  28f2cd:	jbe    28f2f7 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xb17>
  28f2cf:	imul   $0xb40,%rax,%rax
  28f2d6:	mov    %rax,%rsi
  28f2d9:	or     $0x1,%rsi
  28f2dd:	cmp    %r13,%rsi
  28f2e0:	jae    28f002 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x822>
  28f2e6:	cmpb   $0x1,(%rdi,%rax,1)
  28f2ea:	jne    28f2f7 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xb17>
  28f2ec:	vzeroupper
  28f2ef:	call   *0x25b953(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f2f5:	jmp    28f300 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xb20>
  28f2f7:	vzeroupper
  28f2fa:	call   *0x25b950(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f300:	mov    0x2a0(%rsp),%rax
  28f308:	test   %rax,%rax
  28f30b:	je     28f348 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xb68>
  28f30d:	mov    0x2a8(%rsp),%rdi
  28f315:	cmp    $0xcc,%rax
  28f31b:	jbe    28f3a5 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xbc5>
  28f321:	shl    $0x7,%rax
  28f325:	lea    (%rax,%rax,4),%rax
  28f329:	mov    %rax,%rsi
  28f32c:	or     $0x1,%rsi
  28f330:	cmp    %r13,%rsi
  28f333:	jae    28f002 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x822>
  28f339:	cmpb   $0x1,(%rdi,%rax,1)
  28f33d:	jne    28f3a5 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xbc5>
  28f33f:	vzeroupper
  28f342:	call   *0x25b900(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f348:	mov    0x30(%rsp),%rax
  28f34d:	cmp    $0x3ffff,%rax
  28f353:	jb     28f3bb <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xbdb>
  28f355:	cmp    $0xfffffffffffffff0,%rax
  28f359:	je     28f002 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x822>
  28f35f:	mov    0x18(%rsp),%rax
  28f364:	mov    %rax,%r15
  28f367:	or     $0x1,%r15
  28f36b:	cmpb   $0x1,(%r14,%rax,1)
  28f370:	jne    28f3e0 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc00>
  28f372:	mov    %r14,%rdi
  28f375:	mov    %r15,%rsi
  28f378:	vzeroupper
  28f37b:	call   *0x25b8c7(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f381:	mov    0x18(%rsp),%rax
  28f386:	cmpb   $0x1,(%r12,%rax,1)
  28f38b:	je     28f3f8 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc18>
  28f38d:	mov    %r12,%rdi
  28f390:	call   *0x25b8ba(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f396:	mov    0x28(%rsp),%rax
  28f39b:	test   %rax,%rax
  28f39e:	jne    28f412 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc32>
  28f3a0:	jmp    28e92c <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x14c>
  28f3a5:	vzeroupper
  28f3a8:	call   *0x25b8a2(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f3ae:	mov    0x30(%rsp),%rax
  28f3b3:	cmp    $0x3ffff,%rax
  28f3b9:	jae    28f355 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xb75>
  28f3bb:	mov    0x25b88e(%rip),%r15        # 4eac50 <free@GLIBC_2.2.5>
  28f3c2:	mov    %r14,%rdi
  28f3c5:	vzeroupper
  28f3c8:	call   *%r15
  28f3cb:	mov    %r12,%rdi
  28f3ce:	call   *%r15
  28f3d1:	mov    0x28(%rsp),%rax
  28f3d6:	test   %rax,%rax
  28f3d9:	jne    28f412 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc32>
  28f3db:	jmp    28e92c <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x14c>
  28f3e0:	mov    %r14,%rdi
  28f3e3:	vzeroupper
  28f3e6:	call   *0x25b864(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f3ec:	mov    0x18(%rsp),%rax
  28f3f1:	cmpb   $0x1,(%r12,%rax,1)
  28f3f6:	jne    28f38d <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xbad>
  28f3f8:	mov    %r12,%rdi
  28f3fb:	mov    %r15,%rsi
  28f3fe:	call   *0x25b844(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f404:	mov    0x28(%rsp),%rax
  28f409:	test   %rax,%rax
  28f40c:	je     28e92c <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x14c>
  28f412:	cmp    $0x3ff,%rax
  28f418:	jbe    28f446 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc66>
  28f41a:	shl    $0x7,%rax
  28f41e:	add    $0x38,%r13
  28f422:	mov    %rax,%rsi
  28f425:	or     $0x1,%rsi
  28f429:	cmp    %r13,%rsi
  28f42c:	jae    28f002 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x822>
  28f432:	cmpb   $0x1,(%rbx,%rax,1)
  28f436:	jne    28f446 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc66>
  28f438:	mov    %rbx,%rdi
  28f43b:	call   *0x25b807(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f441:	jmp    28e92c <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x14c>
  28f446:	mov    %rbx,%rdi
  28f449:	call   *0x25b801(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f44f:	jmp    28e92c <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x14c>
  28f454:	mov    %ecx,%r13d
  28f457:	call   *0x25b7f3(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f45d:	mov    %r13d,%ecx
  28f460:	jmp    28f017 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x837>
  28f465:	cmpb   $0x1,(%rdi,%rsi,1)
  28f469:	mov    %ecx,%r13d
  28f46c:	jne    28f457 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xc77>
  28f46e:	inc    %rsi
  28f471:	call   *0x25b7d1(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f477:	mov    %r13d,%ecx
  28f47a:	jmp    28f017 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x837>
  28f47f:	lea    0x256c4a(%rip),%rcx        # 4e60d0 <__frame_dummy_init_array_entry+0x4270>
  28f486:	mov    %r11,%rdi
  28f489:	mov    %r13,%rsi
  28f48c:	mov    %r9,%rdx
  28f48f:	call   f0c00 <core::slice::index::slice_index_fail>
  28f494:	lea    0x256c1d(%rip),%rax        # 4e60b8 <__frame_dummy_init_array_entry+0x4258>
  28f49b:	mov    %rcx,%rdx
  28f49e:	mov    %r14,%rdi
  28f4a1:	mov    %r15,%rsi
  28f4a4:	mov    %rax,%rcx
  28f4a7:	call   f0c00 <core::slice::index::slice_index_fail>
  28f4ac:	lea    -0x26f929(%rip),%rdi        # 1fb8a <zmij::STATIC_DATA+0x558a>
  28f4b3:	lea    0x256c46(%rip),%rdx        # 4e6100 <__frame_dummy_init_array_entry+0x42a0>
  28f4ba:	mov    $0x13,%esi
  28f4bf:	call   eec00 <core::panicking::panic_fmt>
  28f4c4:	sub    $0xffffffffffffff80,%rdi
  28f4c8:	lea    0x40(%rsp),%rsi
  28f4cd:	call   2911d0 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>>
  28f4d2:	jmp    28e92c <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x14c>
  28f4d7:	sub    $0xffffffffffffff80,%rdi
  28f4db:	lea    0x40(%rsp),%rdx
  28f4e0:	call   291740 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>>
  28f4e5:	jmp    28e92c <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0x14c>
  28f4ea:	mov    %r15,%rbp
  28f4ed:	jmp    28f509 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd29>
  28f4ef:	lea    0x257b22(%rip),%rdx        # 4e7018 <__frame_dummy_init_array_entry+0x51b8>
  28f4f6:	mov    %rax,%rdi
  28f4f9:	vzeroupper
  28f4fc:	call   efcab <core::panicking::panic_bounds_check>
  28f501:	jmp    28f571 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd91>
  28f503:	mov    %r10,%r15
  28f506:	mov    %r8,%rdx
  28f509:	mov    %r15,%rdi
  28f50c:	mov    %rbp,%rsi
  28f50f:	vzeroupper
  28f512:	call   efcab <core::panicking::panic_bounds_check>
  28f517:	jmp    28f571 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd91>
  28f519:	lea    0x256bc8(%rip),%rdx        # 4e60e8 <__frame_dummy_init_array_entry+0x4288>
  28f520:	mov    %r11,%rdi
  28f523:	mov    %rax,%rsi
  28f526:	call   efcab <core::panicking::panic_bounds_check>
  28f52b:	lea    0x256b26(%rip),%rdx        # 4e6058 <__frame_dummy_init_array_entry+0x41f8>
  28f532:	mov    $0x1,%edi
  28f537:	mov    $0x1,%esi
  28f53c:	call   efcab <core::panicking::panic_bounds_check>
  28f541:	lea    0x256b28(%rip),%rdx        # 4e6070 <__frame_dummy_init_array_entry+0x4210>
  28f548:	xor    %edi,%edi
  28f54a:	xor    %esi,%esi
  28f54c:	call   efcab <core::panicking::panic_bounds_check>
  28f551:	mov    0x18(%rsp),%rsi
  28f556:	mov    $0x8,%edi
  28f55b:	call   ede21 <alloc::raw_vec::handle_error>
  28f560:	jmp    28f571 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xd91>
  28f562:	mov    0x18(%rsp),%rsi
  28f567:	mov    $0x8,%edi
  28f56c:	call   ede21 <alloc::raw_vec::handle_error>
  28f571:	ud2
  28f573:	mov    $0x8,%edi
  28f578:	mov    %r12,%rsi
  28f57b:	call   ede21 <alloc::raw_vec::handle_error>
  28f580:	mov    %rax,%r15
  28f583:	jmp    28f5a3 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xdc3>
  28f585:	mov    %rax,%r15
  28f588:	jmp    28f5b5 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xdd5>
  28f58a:	jmp    28f58e <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xdae>
  28f58c:	jmp    28f593 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xdb3>
  28f58e:	mov    %rax,%r15
  28f591:	jmp    28f5c7 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xde7>
  28f593:	lea    0x270(%rsp),%rdi
  28f59b:	mov    %rax,%r15
  28f59e:	call   2853b0 <core::ptr::drop_glue::<zopro_core::msm::Scratch<zopro_bn254::curve::G2Config>>>
  28f5a3:	mov    0x18(%rsp),%rsi
  28f5a8:	mov    $0x8,%edx
  28f5ad:	mov    %r14,%rdi
  28f5b0:	call   e9120 <__rustc::__rust_dealloc>
  28f5b5:	mov    0x18(%rsp),%rsi
  28f5ba:	mov    $0x8,%edx
  28f5bf:	mov    %r12,%rdi
  28f5c2:	call   e9120 <__rustc::__rust_dealloc>
  28f5c7:	cmpq   $0x0,0x28(%rsp)
  28f5cd:	je     28f5e5 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>+0xe05>
  28f5cf:	mov    0x28(%rsp),%rsi
  28f5d4:	mov    $0x8,%edx
  28f5d9:	mov    %rbx,%rdi
  28f5dc:	shl    $0x7,%rsi
  28f5e0:	call   e9120 <__rustc::__rust_dealloc>
  28f5e5:	mov    %r15,%rdi
  28f5e8:	call   4e0e10 <_Unwind_Resume@plt>
