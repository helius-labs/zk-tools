
/home/ubuntu/src/zopro-perf-work/fr-dot3-175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000003fe8c0 <<zopro_core::solver::Plan>::run_poseidon_task>:
  3fe8c0:	push   %rbp
  3fe8c1:	push   %r15
  3fe8c3:	push   %r14
  3fe8c5:	push   %r13
  3fe8c7:	push   %r12
  3fe8c9:	push   %rbx
  3fe8ca:	sub    $0x3e8,%rsp
  3fe8d1:	cmpq   $0x7ffffff,0x90(%rsi)
  3fe8dc:	mov    %rdi,%r15
  3fe8df:	mov    %r8,0x20(%rsp)
  3fe8e4:	mov    %rcx,0xa0(%rsp)
  3fe8ec:	mov    %rsi,0x80(%rsp)
  3fe8f4:	ja     400d0f <<zopro_core::solver::Plan>::run_poseidon_task+0x244f>
  3fe8fa:	mov    $0x8000000,%eax
  3fe8ff:	and    (%rdx),%eax
  3fe901:	je     400d0f <<zopro_core::solver::Plan>::run_poseidon_task+0x244f>
  3fe907:	mov    0x20(%rsp),%rax
  3fe90c:	mov    0xa0(%rsp),%rcx
  3fe914:	mov    %rsi,%rbp
  3fe917:	xor    %r14d,%r14d
  3fe91a:	mov    %rdx,0x1f8(%rsp)
  3fe922:	mov    (%rax),%rbx
  3fe925:	neg    %rcx
  3fe928:	mov    %rcx,0x200(%rsp)
  3fe930:	mov    %rbx,0x208(%rsp)
  3fe938:	mov    (%rdx,%r14,4),%r12d
  3fe93c:	mov    %r12d,%edi
  3fe93f:	shr    $0x1c,%edi
  3fe942:	cmp    $0xd,%edi
  3fe945:	ja     400e8b <<zopro_core::solver::Plan>::run_poseidon_task+0x25cb>
  3fe94b:	lea    -0x3ead9a(%rip),%rcx        # 13bb8 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x2138>
  3fe952:	mov    $0x38,%esi
  3fe957:	mov    %rdi,0x8(%rsp)
  3fe95c:	movslq (%rcx,%rdi,4),%rax
  3fe960:	add    %rcx,%rax
  3fe963:	jmp    *%rax
  3fe965:	mov    0x90(%rbp),%rsi
  3fe96c:	and    $0x7ffffff,%r12d
  3fe973:	cmp    %r12,%rsi
  3fe976:	jbe    400fc8 <<zopro_core::solver::Plan>::run_poseidon_task+0x2708>
  3fe97c:	mov    0x20(%rsp),%rax
  3fe981:	mov    %rdx,%r13
  3fe984:	lea    (%r12,%r12,8),%rdx
  3fe988:	lea    0x160(%rsp),%rdi
  3fe990:	lea    0x240(%rsp),%rcx
  3fe998:	mov    %rbp,%rsi
  3fe99b:	shl    $0x3,%rdx
  3fe99f:	add    0x88(%rbp),%rdx
  3fe9a6:	vmovups (%rax),%ymm0
  3fe9aa:	vmovups %ymm0,0x240(%rsp)
  3fe9b3:	vzeroupper
  3fe9b6:	call   3fa330 <<zopro_core::solver::Plan>::run>
  3fe9bb:	cmpq   $0xffffffffffffffff,0x160(%rsp)
  3fe9c4:	jne    400df7 <<zopro_core::solver::Plan>::run_poseidon_task+0x2537>
  3fe9ca:	inc    %r14
  3fe9cd:	mov    %r13,%rdx
  3fe9d0:	cmp    0xa0(%rsp),%r14
  3fe9d8:	jb     3fe938 <<zopro_core::solver::Plan>::run_poseidon_task+0x78>
  3fe9de:	jmp    400d03 <<zopro_core::solver::Plan>::run_poseidon_task+0x2443>
  3fe9e3:	mov    $0x720,%r13d
  3fe9e9:	mov    $0x3c,%esi
  3fe9ee:	lea    -0x393685(%rip),%rax        # 6b370 <zopro_core::solver::sparse_poseidon::constants::T12>
  3fe9f5:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fe9fa:	mov    $0x4c0,%r13d
  3fea00:	mov    $0x40,%esi
  3fea05:	lea    -0x3c115c(%rip),%rax        # 3d8b0 <zopro_core::solver::sparse_poseidon::constants::T8>
  3fea0c:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea11:	mov    $0xf8,%r13d
  3fea17:	lea    -0x3de12e(%rip),%rax        # 208f0 <zopro_core::solver::sparse_poseidon::constants::T2>
  3fea1e:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea23:	mov    $0x180,%r13d
  3fea29:	mov    $0x39,%esi
  3fea2e:	lea    -0x3dc245(%rip),%rax        # 227f0 <zopro_core::solver::sparse_poseidon::constants::T3>
  3fea35:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea3a:	mov    $0x558,%r13d
  3fea40:	mov    $0x3f,%esi
  3fea45:	lea    -0x3b799c(%rip),%rax        # 470b0 <zopro_core::solver::sparse_poseidon::constants::T9>
  3fea4c:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea51:	mov    $0x5c8,%r13d
  3fea57:	mov    $0x3c,%esi
  3fea5c:	lea    -0x3aceb3(%rip),%rax        # 51bb0 <zopro_core::solver::sparse_poseidon::constants::T10>
  3fea63:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea65:	mov    $0x40c,%r13d
  3fea6b:	mov    $0x3f,%esi
  3fea70:	lea    -0x3c9347(%rip),%rax        # 35730 <zopro_core::solver::sparse_poseidon::constants::T7>
  3fea77:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea79:	mov    $0x854,%r13d
  3fea7f:	mov    $0x41,%esi
  3fea84:	lea    -0x38531b(%rip),%rax        # 79770 <zopro_core::solver::sparse_poseidon::constants::T13>
  3fea8b:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea8d:	mov    $0x200,%r13d
  3fea93:	lea    -0x3d92aa(%rip),%rax        # 257f0 <zopro_core::solver::sparse_poseidon::constants::T4>
  3fea9a:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea9c:	mov    $0x6f6,%r13d
  3feaa2:	mov    $0x42,%esi
  3feaa7:	lea    -0x3a15fe(%rip),%rax        # 5d4b0 <zopro_core::solver::sparse_poseidon::constants::T11>
  3feaae:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3feab0:	mov    $0x348,%r13d
  3feab6:	mov    $0x3c,%esi
  3feabb:	lea    -0x3cfc92(%rip),%rax        # 2ee30 <zopro_core::solver::sparse_poseidon::constants::T6>
  3feac2:	jmp    3feae0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3feac4:	mov    $0x2b2,%r13d
  3feaca:	mov    $0x3c,%esi
  3feacf:	lea    -0x3d52e6(%rip),%rax        # 297f0 <zopro_core::solver::sparse_poseidon::constants::T5>
  3fead6:	cs nopw 0x0(%rax,%rax,1)
  3feae0:	mov    %rax,(%rsp)
  3feae4:	mov    0x8(%rsp),%rax
  3feae9:	mov    0xa0(%rsp),%rcx
  3feaf1:	lea    (%rax,%rax,2),%rdi
  3feaf5:	sub    %r14,%rcx
  3feaf8:	mov    %rcx,0x30(%rsp)
  3feafd:	mov    %rdi,0x18(%rsp)
  3feb02:	cmp    %rcx,%rdi
  3feb05:	ja     400e31 <<zopro_core::solver::Plan>::run_poseidon_task+0x2571>
  3feb0b:	lea    0x0(,%rax,4),%eax
  3feb12:	lea    (%rdx,%r14,4),%rcx
  3feb16:	mov    %r14,0x1c0(%rsp)
  3feb1e:	mov    %rsi,0x38(%rsp)
  3feb23:	xor    %r14d,%r14d
  3feb26:	lea    (%rax,%rax,2),%rbx
  3feb2a:	mov    %rcx,0x48(%rsp)
  3feb2f:	nop
  3feb30:	mov    0x48(%rsp),%rax
  3feb35:	mov    0x90(%rbp),%rsi
  3feb3c:	mov    (%rax,%r14,1),%edi
  3feb40:	and    $0x7ffffff,%edi
  3feb46:	cmp    %rdi,%rsi
  3feb49:	jbe    400ea6 <<zopro_core::solver::Plan>::run_poseidon_task+0x25e6>
  3feb4f:	mov    0x20(%rsp),%rax
  3feb54:	lea    (%rdi,%rdi,8),%rdx
  3feb58:	lea    0x160(%rsp),%rdi
  3feb60:	lea    0x240(%rsp),%rcx
  3feb68:	mov    %rbp,%rsi
  3feb6b:	shl    $0x3,%rdx
  3feb6f:	add    0x88(%rbp),%rdx
  3feb76:	vmovups (%rax),%ymm0
  3feb7a:	vmovups %ymm0,0x240(%rsp)
  3feb83:	vzeroupper
  3feb86:	call   3fa330 <<zopro_core::solver::Plan>::run>
  3feb8b:	mov    0x160(%rsp),%rax
  3feb93:	cmp    $0xffffffffffffffff,%rax
  3feb97:	jne    400db2 <<zopro_core::solver::Plan>::run_poseidon_task+0x24f2>
  3feb9d:	add    $0x4,%r14
  3feba1:	cmp    %r14,%rbx
  3feba4:	jne    3feb30 <<zopro_core::solver::Plan>::run_poseidon_task+0x270>
  3feba6:	vxorps %xmm0,%xmm0,%xmm0
  3febaa:	vmovups %zmm0,0x3a0(%rsp)
  3febb5:	vmovups %zmm0,0x380(%rsp)
  3febbd:	vmovups %zmm0,0x340(%rsp)
  3febc5:	vmovups %zmm0,0x300(%rsp)
  3febcd:	vmovups %zmm0,0x2c0(%rsp)
  3febd5:	vmovups %zmm0,0x280(%rsp)
  3febdd:	vmovups %zmm0,0x240(%rsp)
  3febe5:	cmp    $0xe0000000,%r12d
  3febec:	jae    400e4c <<zopro_core::solver::Plan>::run_poseidon_task+0x258c>
  3febf2:	mov    0x8(%rsp),%rax
  3febf7:	mov    0x38(%rsp),%rcx
  3febfc:	mov    0x208(%rsp),%r9
  3fec04:	mov    %rax,%rdx
  3fec07:	imul   %rax,%rdx
  3fec0b:	lea    (%rcx,%rax,8),%rcx
  3fec0f:	lea    (%rax,%rax,1),%edi
  3fec12:	mov    %rdi,0xa8(%rsp)
  3fec1a:	mov    %rcx,0x98(%rsp)
  3fec22:	lea    (%rcx,%rdx,1),%r10
  3fec26:	mov    %eax,%ecx
  3fec28:	shl    $0x5,%ecx
  3fec2b:	mov    0x200(%rsp),%rax
  3fec33:	mov    %rcx,0x88(%rsp)
  3fec3b:	mov    0x1c0(%rsp),%rcx
  3fec43:	add    %r10,%rdx
  3fec46:	mov    %rdx,0x220(%rsp)
  3fec4e:	add    %rcx,%rax
  3fec51:	mov    0x30(%rsp),%rcx
  3fec56:	cmp    %rcx,%rdi
  3fec59:	cmovb  %rdi,%rcx
  3fec5d:	xor    %edx,%edx
  3fec5f:	add    %rax,%rcx
  3fec62:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  3fec70:	test   %rcx,%rcx
  3fec73:	je     400ec7 <<zopro_core::solver::Plan>::run_poseidon_task+0x2607>
  3fec79:	mov    0x48(%rsp),%rax
  3fec7e:	mov    0x90(%rbp),%rsi
  3fec85:	mov    (%rax,%rdi,4),%eax
  3fec88:	and    $0x7ffffff,%eax
  3fec8d:	cmp    %rax,%rsi
  3fec90:	jbe    400eb5 <<zopro_core::solver::Plan>::run_poseidon_task+0x25f5>
  3fec96:	mov    0x88(%rbp),%rsi
  3fec9d:	lea    (%rax,%rax,8),%rax
  3feca1:	mov    (%rsi,%rax,8),%r8d
  3feca5:	cmp    $0x2,%r8d
  3feca9:	je     400e13 <<zopro_core::solver::Plan>::run_poseidon_task+0x2553>
  3fecaf:	test   %r8d,%r8d
  3fecb2:	je     400e22 <<zopro_core::solver::Plan>::run_poseidon_task+0x2562>
  3fecb8:	lea    (%rsi,%rax,8),%rax
  3fecbc:	lea    0x240(%rsp,%rdx,1),%r8
  3fecc4:	add    $0x20,%rdx
  3fecc8:	inc    %rcx
  3feccb:	inc    %rdi
  3fecce:	mov    0x4(%rax),%eax
  3fecd1:	shl    $0x5,%rax
  3fecd5:	vmovups (%r9,%rax,1),%ymm0
  3fecdb:	vmovups %ymm0,(%r8)
  3fece0:	cmp    %rdx,0x88(%rsp)
  3fece8:	jne    3fec70 <<zopro_core::solver::Plan>::run_poseidon_task+0x3b0>
  3fecea:	mov    %r12,0x1e0(%rsp)
  3fecf2:	mov    0x8(%rsp),%r12
  3fecf7:	mov    (%rsp),%r14
  3fecfb:	mov    0x98(%rsp),%r8
  3fed03:	lea    0x240(%rsp),%rdi
  3fed0b:	mov    %r10,0x28(%rsp)
  3fed10:	mov    %r15,0x1f0(%rsp)
  3fed18:	mov    %r13,%rcx
  3fed1b:	mov    %r12,%rsi
  3fed1e:	mov    %r14,%rdx
  3fed21:	vzeroupper
  3fed24:	call   402000 <<zopro_core::solver::Plan>::run_poseidon::dense>
  3fed29:	mov    %r12d,%ebx
  3fed2c:	shl    $0x6,%ebx
  3fed2f:	mov    %r12,%rax
  3fed32:	mov    %r12,%r15
  3fed35:	xor    %r12d,%r12d
  3fed38:	mov    %rax,0x10(%rsp)
  3fed3d:	mov    %r13,0x40(%rsp)
  3fed42:	add    %r14,%rbx
  3fed45:	mov    0xa8(%rsp),%r14
  3fed4d:	nopl   (%rax)
  3fed50:	cmp    %r14,%r13
  3fed53:	je     400ede <<zopro_core::solver::Plan>::run_poseidon_task+0x261e>
  3fed59:	vmovups (%rbx,%r12,1),%ymm0
  3fed5f:	vmovups -0x3ed5c7(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3fed67:	lea    0x100(%rsp),%r13
  3fed6f:	mov    0x258(%rsp,%r12,1),%rax
  3fed77:	mov    0x250(%rsp,%r12,1),%rcx
  3fed7f:	mov    0x240(%rsp,%r12,1),%rdx
  3fed87:	mov    0x248(%rsp,%r12,1),%rsi
  3fed8f:	lea    0x160(%rsp),%r11
  3fed97:	vmovups %ymm0,0x100(%rsp)
  3feda0:	vmovups %ymm1,0x160(%rsp)
  3feda9:	add    0x0(%r13),%rdx
  3fedad:	adc    0x8(%r13),%rsi
  3fedb1:	adc    0x10(%r13),%rcx
  3fedb5:	adc    0x18(%r13),%rax
  3fedb9:	mov    %rdx,%rdi
  3fedbc:	mov    %rsi,%r8
  3fedbf:	mov    %rcx,%r9
  3fedc2:	mov    %rax,%r10
  3fedc5:	sub    (%r11),%rdi
  3fedc8:	sbb    0x8(%r11),%r8
  3fedcc:	sbb    0x10(%r11),%r9
  3fedd0:	sbb    0x18(%r11),%r10
  3fedd4:	cmovae %rdi,%rdx
  3fedd8:	cmovae %r8,%rsi
  3feddc:	cmovae %r9,%rcx
  3fede0:	cmovae %r10,%rax
  3fede4:	mov    %rdx,0x50(%rsp)
  3fede9:	mov    %rsi,0x58(%rsp)
  3fedee:	mov    %rcx,0x60(%rsp)
  3fedf3:	mov    %rax,0x68(%rsp)
  3fedf8:	sub    $0x8,%rsp
  3fedfc:	mov    0x50(%rsp),%rdx
  3fee01:	mov    0x38(%rsp),%rcx
  3fee06:	mov    0x10(%rsp),%r8
  3fee0b:	mov    0x40(%rsp),%r9
  3fee10:	lea    0xc8(%rsp),%rdi
  3fee18:	mov    %rbp,%rsi
  3fee1b:	push   0x28(%rsp)
  3fee1f:	lea    0x60(%rsp),%rax
  3fee24:	push   %rax
  3fee25:	mov    0x28(%rsp),%r13
  3fee2a:	push   %r13
  3fee2c:	vzeroupper
  3fee2f:	call   402a00 <<zopro_core::solver::Plan>::emit_poseidon>
  3fee34:	add    $0x20,%rsp
  3fee38:	vmovups 0xc0(%rsp),%ymm0
  3fee41:	inc    %r13
  3fee44:	inc    %r14
  3fee47:	mov    %r13,0x10(%rsp)
  3fee4c:	mov    0x40(%rsp),%r13
  3fee51:	vmovups %ymm0,0x240(%rsp,%r12,1)
  3fee5b:	add    $0x20,%r12
  3fee5f:	dec    %r15
  3fee62:	jne    3fed50 <<zopro_core::solver::Plan>::run_poseidon_task+0x490>
  3fee68:	mov    0x8(%rsp),%rbx
  3fee6d:	mov    (%rsp),%r14
  3fee71:	mov    0x98(%rsp),%r8
  3fee79:	lea    0x240(%rsp),%rdi
  3fee81:	mov    %r13,%rcx
  3fee84:	mov    %rbx,%rsi
  3fee87:	mov    %r14,%rdx
  3fee8a:	vzeroupper
  3fee8d:	call   402000 <<zopro_core::solver::Plan>::run_poseidon::dense>
  3fee92:	mov    0x18(%rsp),%r12
  3fee97:	lea    (%rbx,%rbx,2),%rbx
  3fee9b:	mov    %rbx,0x1e8(%rsp)
  3feea3:	shl    $0x5,%ebx
  3feea6:	add    %r14,%rbx
  3feea9:	xor    %r14d,%r14d
  3feeac:	nopl   0x0(%rax)
  3feeb0:	cmp    %r12,%r13
  3feeb3:	je     400edb <<zopro_core::solver::Plan>::run_poseidon_task+0x261b>
  3feeb9:	vmovups (%rbx,%r14,1),%ymm0
  3feebf:	vmovups -0x3ed727(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3feec7:	mov    0x258(%rsp,%r14,1),%rax
  3feecf:	mov    0x250(%rsp,%r14,1),%rcx
  3feed7:	mov    0x240(%rsp,%r14,1),%rdx
  3feedf:	mov    0x248(%rsp,%r14,1),%rsi
  3feee7:	lea    0x160(%rsp),%r11
  3feeef:	lea    0x100(%rsp),%r15
  3feef7:	vmovups %ymm0,0x100(%rsp)
  3fef00:	vmovups %ymm1,0x160(%rsp)
  3fef09:	add    (%r15),%rdx
  3fef0c:	adc    0x8(%r15),%rsi
  3fef10:	adc    0x10(%r15),%rcx
  3fef14:	adc    0x18(%r15),%rax
  3fef18:	mov    %rdx,%rdi
  3fef1b:	mov    %rsi,%r8
  3fef1e:	mov    %rcx,%r9
  3fef21:	mov    %rax,%r10
  3fef24:	sub    (%r11),%rdi
  3fef27:	sbb    0x8(%r11),%r8
  3fef2b:	sbb    0x10(%r11),%r9
  3fef2f:	sbb    0x18(%r11),%r10
  3fef33:	cmovae %rdi,%rdx
  3fef37:	cmovae %r8,%rsi
  3fef3b:	cmovae %r9,%rcx
  3fef3f:	cmovae %r10,%rax
  3fef43:	mov    %rdx,0x50(%rsp)
  3fef48:	mov    %rsi,0x58(%rsp)
  3fef4d:	mov    %rcx,0x60(%rsp)
  3fef52:	mov    %rax,0x68(%rsp)
  3fef57:	sub    $0x8,%rsp
  3fef5b:	mov    0x50(%rsp),%rdx
  3fef60:	mov    0x38(%rsp),%rcx
  3fef65:	mov    0x10(%rsp),%r8
  3fef6a:	mov    0x40(%rsp),%r9
  3fef6f:	lea    0xc8(%rsp),%rdi
  3fef77:	mov    %rbp,%rsi
  3fef7a:	push   0x28(%rsp)
  3fef7e:	lea    0x60(%rsp),%rax
  3fef83:	push   %rax
  3fef84:	mov    0x28(%rsp),%r15
  3fef89:	push   %r15
  3fef8b:	vzeroupper
  3fef8e:	call   402a00 <<zopro_core::solver::Plan>::emit_poseidon>
  3fef93:	add    $0x20,%rsp
  3fef97:	vmovups 0xc0(%rsp),%ymm0
  3fefa0:	inc    %r15
  3fefa3:	inc    %r12
  3fefa6:	mov    %r15,0x10(%rsp)
  3fefab:	vmovups %ymm0,0x240(%rsp,%r14,1)
  3fefb5:	add    $0x20,%r14
  3fefb9:	cmp    %r14,0x88(%rsp)
  3fefc1:	jne    3feeb0 <<zopro_core::solver::Plan>::run_poseidon_task+0x5f0>
  3fefc7:	mov    0x8(%rsp),%r14
  3fefcc:	mov    (%rsp),%r15
  3fefd0:	mov    0x28(%rsp),%r8
  3fefd5:	lea    0x240(%rsp),%rdi
  3fefdd:	mov    %r13,%rcx
  3fefe0:	mov    %r14,%rsi
  3fefe3:	mov    %r15,%rdx
  3fefe6:	vzeroupper
  3fefe9:	call   402000 <<zopro_core::solver::Plan>::run_poseidon::dense>
  3fefee:	mov    %r14d,%edi
  3feff1:	shl    $0x7,%edi
  3feff4:	vmovups -0x3ed85c(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3feffc:	mov    0x258(%rsp),%rax
  3ff004:	mov    0x250(%rsp),%rcx
  3ff00c:	mov    0x240(%rsp),%rdx
  3ff014:	mov    0x248(%rsp),%rsi
  3ff01c:	lea    0x160(%rsp),%r11
  3ff024:	lea    0x100(%rsp),%rbx
  3ff02c:	vmovups (%r15,%rdi,1),%ymm0
  3ff032:	vmovups %ymm1,0x160(%rsp)
  3ff03b:	vmovups %ymm0,0x100(%rsp)
  3ff044:	add    (%rbx),%rdx
  3ff047:	adc    0x8(%rbx),%rsi
  3ff04b:	adc    0x10(%rbx),%rcx
  3ff04f:	adc    0x18(%rbx),%rax
  3ff053:	mov    %rdx,%rdi
  3ff056:	mov    %rsi,%r8
  3ff059:	mov    %rcx,%r9
  3ff05c:	mov    %rax,%r10
  3ff05f:	sub    (%r11),%rdi
  3ff062:	sbb    0x8(%r11),%r8
  3ff066:	sbb    0x10(%r11),%r9
  3ff06a:	sbb    0x18(%r11),%r10
  3ff06e:	cmovae %rdi,%rdx
  3ff072:	cmovae %r8,%rsi
  3ff076:	cmovae %r9,%rcx
  3ff07a:	cmovae %r10,%rax
  3ff07e:	mov    %rdx,0xe0(%rsp)
  3ff086:	mov    %rsi,0xe8(%rsp)
  3ff08e:	mov    %rcx,0xf0(%rsp)
  3ff096:	mov    %rax,0xf8(%rsp)
  3ff09e:	mov    %r14,%rax
  3ff0a1:	lea    0x1(,%r14,4),%rdi
  3ff0a9:	mov    0xa8(%rsp),%rdx
  3ff0b1:	mov    0x38(%rsp),%r9
  3ff0b6:	lea    0x8(%rdx),%rsi
  3ff0ba:	mov    %r9d,%ecx
  3ff0bd:	shl    $0x5,%ecx
  3ff0c0:	lea    -0x1(%rdx),%r8
  3ff0c4:	add    $0x9,%rdx
  3ff0c8:	imul   %rsi,%rax
  3ff0cc:	add    %r15,%rcx
  3ff0cf:	mov    %r8,0x218(%rsp)
  3ff0d7:	lea    0x1(%r9,%rax,1),%rax
  3ff0dc:	mov    %rax,0x1c8(%rsp)
  3ff0e4:	mov    %r14d,%eax
  3ff0e7:	shl    $0x5,%eax
  3ff0ea:	imul   %rax,%rsi
  3ff0ee:	add    %rcx,%rsi
  3ff0f1:	mov    %rsi,0x1d8(%rsp)
  3ff0f9:	mov    %r14d,%esi
  3ff0fc:	shl    $0x6,%esi
  3ff0ff:	imul   %rdx,%r14
  3ff103:	imul   %rax,%rdx
  3ff107:	mov    %rsi,0xb8(%rsp)
  3ff10f:	add    $0xffffffffffffffe0,%rsi
  3ff113:	mov    %rsi,0x210(%rsp)
  3ff11b:	mov    0x88(%rsp),%rsi
  3ff123:	add    %r9,%r14
  3ff126:	add    %rcx,%rdx
  3ff129:	mov    %r14,0x1d0(%rsp)
  3ff131:	xor    %r14d,%r14d
  3ff134:	mov    %rdx,0xa8(%rsp)
  3ff13c:	lea    -0x20(%rsi),%rax
  3ff140:	mov    %rax,0x238(%rsp)
  3ff148:	nopl   0x0(%rax,%rax,1)
  3ff150:	lea    0x1(%r14),%r13
  3ff154:	cmp    %r9,%r13
  3ff157:	jae    3ff190 <<zopro_core::solver::Plan>::run_poseidon_task+0x8d0>
  3ff159:	mov    0x40(%rsp),%rbx
  3ff15e:	cmp    %rbx,%rdi
  3ff161:	jae    400f8f <<zopro_core::solver::Plan>::run_poseidon_task+0x26cf>
  3ff167:	mov    (%rsp),%rcx
  3ff16b:	mov    %rdi,%rax
  3ff16e:	shl    $0x5,%rax
  3ff172:	inc    %rdi
  3ff175:	mov    (%rcx,%rax,1),%rbp
  3ff179:	mov    0x8(%rcx,%rax,1),%r15
  3ff17e:	mov    0x10(%rcx,%rax,1),%r10
  3ff183:	mov    0x18(%rcx,%rax,1),%r11
  3ff188:	jmp    3ff1a0 <<zopro_core::solver::Plan>::run_poseidon_task+0x8e0>
  3ff18a:	nopw   0x0(%rax,%rax,1)
  3ff190:	mov    0x40(%rsp),%rbx
  3ff195:	xor    %ebp,%ebp
  3ff197:	xor    %r15d,%r15d
  3ff19a:	xor    %r10d,%r10d
  3ff19d:	xor    %r11d,%r11d
  3ff1a0:	imul   %r8,%r14
  3ff1a4:	add    0x220(%rsp),%r14
  3ff1ac:	mov    0x8(%rsp),%r8
  3ff1b1:	mov    %rdi,0xb0(%rsp)
  3ff1b9:	cmp    $0x1,%r8d
  3ff1bd:	je     3ff810 <<zopro_core::solver::Plan>::run_poseidon_task+0xf50>
  3ff1c3:	mov    %r13,0x230(%rsp)
  3ff1cb:	mov    %r14,0x228(%rsp)
  3ff1d3:	cmp    $0x3,%r8d
  3ff1d7:	jne    3ff870 <<zopro_core::solver::Plan>::run_poseidon_task+0xfb0>
  3ff1dd:	lea    0x1(%r14),%rax
  3ff1e1:	cmp    %rbx,%rax
  3ff1e4:	jae    400fb3 <<zopro_core::solver::Plan>::run_poseidon_task+0x26f3>
  3ff1ea:	lea    0x2(%r14),%rdi
  3ff1ee:	mov    %r11,0x28(%rsp)
  3ff1f3:	mov    %r10,0x18(%rsp)
  3ff1f8:	cmp    %rbx,%rdi
  3ff1fb:	jae    400fa1 <<zopro_core::solver::Plan>::run_poseidon_task+0x26e1>
  3ff201:	mov    (%rsp),%rcx
  3ff205:	shl    $0x5,%rax
  3ff209:	shl    $0x5,%rdi
  3ff20d:	lea    0x160(%rsp),%r14
  3ff215:	lea    0x100(%rsp),%r12
  3ff21d:	vmovups (%rcx,%rax,1),%ymm0
  3ff222:	lea    0x260(%rsp),%rax
  3ff22a:	vmovups %ymm0,0x100(%rsp)
  3ff233:	vmovups (%rax),%ymm0
  3ff237:	vmovups %ymm0,0x160(%rsp)
  3ff240:	vmovups (%rcx,%rdi,1),%ymm0
  3ff245:	lea    0x120(%rsp),%rcx
  3ff24d:	vmovups %ymm0,(%rcx)
  3ff251:	vmovups 0x20(%rax),%ymm0
  3ff256:	lea    0x180(%rsp),%rax
  3ff25e:	vmovups %ymm1,0x20(%rcx)
  3ff263:	movabs $0xc2e1f593efffffff,%rcx
  3ff26d:	vmovups %ymm0,(%rax)
  3ff271:	mov    %rcx,0x1a0(%rsp)
  3ff279:	vxorps %xmm0,%xmm0,%xmm0
  3ff27d:	vmovups %xmm0,0x28(%rax)
  3ff282:	movq   $0x0,0x38(%rax)
  3ff28a:	xor    %edi,%edi
  3ff28c:	xor    %esi,%esi
  3ff28e:	xor    %eax,%eax
  3ff290:	xor    %ecx,%ecx
  3ff292:	xor    %r8d,%r8d
  3ff295:	mov    (%r14),%rdx
  3ff298:	xor    %r11d,%r11d
  3ff29b:	mulx   (%r12),%r9,%r10
  3ff2a1:	adcx   %r9,%rdi
  3ff2a7:	adox   %r10,%rsi
  3ff2ad:	mulx   0x8(%r12),%r9,%r10
  3ff2b4:	adcx   %r9,%rsi
  3ff2ba:	adox   %r10,%rax
  3ff2c0:	mulx   0x10(%r12),%r9,%r10
  3ff2c7:	adcx   %r9,%rax
  3ff2cd:	adox   %r10,%rcx
  3ff2d3:	mulx   0x18(%r12),%r9,%r10
  3ff2da:	adcx   %r9,%rcx
  3ff2e0:	adox   %r10,%r8
  3ff2e6:	adcx   %r11,%r8
  3ff2ec:	mov    0x20(%r14),%rdx
  3ff2f0:	xor    %r11d,%r11d
  3ff2f3:	mulx   0x20(%r12),%r9,%r10
  3ff2fa:	adcx   %r9,%rdi
  3ff300:	adox   %r10,%rsi
  3ff306:	mulx   0x28(%r12),%r9,%r10
  3ff30d:	adcx   %r9,%rsi
  3ff313:	adox   %r10,%rax
  3ff319:	mulx   0x30(%r12),%r9,%r10
  3ff320:	adcx   %r9,%rax
  3ff326:	adox   %r10,%rcx
  3ff32c:	mulx   0x38(%r12),%r9,%r10
  3ff333:	adcx   %r9,%rcx
  3ff339:	adox   %r10,%r8
  3ff33f:	adcx   %r11,%r8
  3ff345:	mov    %rdi,%rdx
  3ff348:	imul   0x40(%r14),%rdx
  3ff34d:	xor    %r11d,%r11d
  3ff350:	mulx   0x40(%r12),%r9,%r10
  3ff357:	adcx   %r9,%rdi
  3ff35d:	adox   %r10,%rsi
  3ff363:	mulx   0x48(%r12),%r9,%r10
  3ff36a:	adcx   %r9,%rsi
  3ff370:	adox   %r10,%rax
  3ff376:	mulx   0x50(%r12),%r9,%r10
  3ff37d:	adcx   %r9,%rax
  3ff383:	adox   %r10,%rcx
  3ff389:	mulx   0x58(%r12),%r9,%r10
  3ff390:	adcx   %r9,%rcx
  3ff396:	adox   %r10,%r8
  3ff39c:	adcx   %r11,%r8
  3ff3a2:	mov    0x8(%r14),%rdx
  3ff3a6:	xor    %r11d,%r11d
  3ff3a9:	mulx   (%r12),%r9,%r10
  3ff3af:	adcx   %r9,%rsi
  3ff3b5:	adox   %r10,%rax
  3ff3bb:	mulx   0x8(%r12),%r9,%r10
  3ff3c2:	adcx   %r9,%rax
  3ff3c8:	adox   %r10,%rcx
  3ff3ce:	mulx   0x10(%r12),%r9,%r10
  3ff3d5:	adcx   %r9,%rcx
  3ff3db:	adox   %r10,%r8
  3ff3e1:	mulx   0x18(%r12),%r9,%r10
  3ff3e8:	adcx   %r9,%r8
  3ff3ee:	adox   %r10,%rdi
  3ff3f4:	adcx   %r11,%rdi
  3ff3fa:	mov    0x28(%r14),%rdx
  3ff3fe:	xor    %r11d,%r11d
  3ff401:	mulx   0x20(%r12),%r9,%r10
  3ff408:	adcx   %r9,%rsi
  3ff40e:	adox   %r10,%rax
  3ff414:	mulx   0x28(%r12),%r9,%r10
  3ff41b:	adcx   %r9,%rax
  3ff421:	adox   %r10,%rcx
  3ff427:	mulx   0x30(%r12),%r9,%r10
  3ff42e:	adcx   %r9,%rcx
  3ff434:	adox   %r10,%r8
  3ff43a:	mulx   0x38(%r12),%r9,%r10
  3ff441:	adcx   %r9,%r8
  3ff447:	adox   %r10,%rdi
  3ff44d:	adcx   %r11,%rdi
  3ff453:	mov    %rsi,%rdx
  3ff456:	imul   0x40(%r14),%rdx
  3ff45b:	xor    %r11d,%r11d
  3ff45e:	mulx   0x40(%r12),%r9,%r10
  3ff465:	adcx   %r9,%rsi
  3ff46b:	adox   %r10,%rax
  3ff471:	mulx   0x48(%r12),%r9,%r10
  3ff478:	adcx   %r9,%rax
  3ff47e:	adox   %r10,%rcx
  3ff484:	mulx   0x50(%r12),%r9,%r10
  3ff48b:	adcx   %r9,%rcx
  3ff491:	adox   %r10,%r8
  3ff497:	mulx   0x58(%r12),%r9,%r10
  3ff49e:	adcx   %r9,%r8
  3ff4a4:	adox   %r10,%rdi
  3ff4aa:	adcx   %r11,%rdi
  3ff4b0:	mov    0x10(%r14),%rdx
  3ff4b4:	xor    %r11d,%r11d
  3ff4b7:	mulx   (%r12),%r9,%r10
  3ff4bd:	adcx   %r9,%rax
  3ff4c3:	adox   %r10,%rcx
  3ff4c9:	mulx   0x8(%r12),%r9,%r10
  3ff4d0:	adcx   %r9,%rcx
  3ff4d6:	adox   %r10,%r8
  3ff4dc:	mulx   0x10(%r12),%r9,%r10
  3ff4e3:	adcx   %r9,%r8
  3ff4e9:	adox   %r10,%rdi
  3ff4ef:	mulx   0x18(%r12),%r9,%r10
  3ff4f6:	adcx   %r9,%rdi
  3ff4fc:	adox   %r10,%rsi
  3ff502:	adcx   %r11,%rsi
  3ff508:	mov    0x30(%r14),%rdx
  3ff50c:	xor    %r11d,%r11d
  3ff50f:	mulx   0x20(%r12),%r9,%r10
  3ff516:	adcx   %r9,%rax
  3ff51c:	adox   %r10,%rcx
  3ff522:	mulx   0x28(%r12),%r9,%r10
  3ff529:	adcx   %r9,%rcx
  3ff52f:	adox   %r10,%r8
  3ff535:	mulx   0x30(%r12),%r9,%r10
  3ff53c:	adcx   %r9,%r8
  3ff542:	adox   %r10,%rdi
  3ff548:	mulx   0x38(%r12),%r9,%r10
  3ff54f:	adcx   %r9,%rdi
  3ff555:	adox   %r10,%rsi
  3ff55b:	adcx   %r11,%rsi
  3ff561:	mov    %rax,%rdx
  3ff564:	imul   0x40(%r14),%rdx
  3ff569:	xor    %r11d,%r11d
  3ff56c:	mulx   0x40(%r12),%r9,%r10
  3ff573:	adcx   %r9,%rax
  3ff579:	adox   %r10,%rcx
  3ff57f:	mulx   0x48(%r12),%r9,%r10
  3ff586:	adcx   %r9,%rcx
  3ff58c:	adox   %r10,%r8
  3ff592:	mulx   0x50(%r12),%r9,%r10
  3ff599:	adcx   %r9,%r8
  3ff59f:	adox   %r10,%rdi
  3ff5a5:	mulx   0x58(%r12),%r9,%r10
  3ff5ac:	adcx   %r9,%rdi
  3ff5b2:	adox   %r10,%rsi
  3ff5b8:	adcx   %r11,%rsi
  3ff5be:	mov    0x18(%r14),%rdx
  3ff5c2:	xor    %r11d,%r11d
  3ff5c5:	mulx   (%r12),%r9,%r10
  3ff5cb:	adcx   %r9,%rcx
  3ff5d1:	adox   %r10,%r8
  3ff5d7:	mulx   0x8(%r12),%r9,%r10
  3ff5de:	adcx   %r9,%r8
  3ff5e4:	adox   %r10,%rdi
  3ff5ea:	mulx   0x10(%r12),%r9,%r10
  3ff5f1:	adcx   %r9,%rdi
  3ff5f7:	adox   %r10,%rsi
  3ff5fd:	mulx   0x18(%r12),%r9,%r10
  3ff604:	adcx   %r9,%rsi
  3ff60a:	adox   %r10,%rax
  3ff610:	adcx   %r11,%rax
  3ff616:	mov    0x38(%r14),%rdx
  3ff61a:	xor    %r11d,%r11d
  3ff61d:	mulx   0x20(%r12),%r9,%r10
  3ff624:	adcx   %r9,%rcx
  3ff62a:	adox   %r10,%r8
  3ff630:	mulx   0x28(%r12),%r9,%r10
  3ff637:	adcx   %r9,%r8
  3ff63d:	adox   %r10,%rdi
  3ff643:	mulx   0x30(%r12),%r9,%r10
  3ff64a:	adcx   %r9,%rdi
  3ff650:	adox   %r10,%rsi
  3ff656:	mulx   0x38(%r12),%r9,%r10
  3ff65d:	adcx   %r9,%rsi
  3ff663:	adox   %r10,%rax
  3ff669:	adcx   %r11,%rax
  3ff66f:	mov    %rcx,%rdx
  3ff672:	imul   0x40(%r14),%rdx
  3ff677:	xor    %r11d,%r11d
  3ff67a:	mulx   0x40(%r12),%r9,%r10
  3ff681:	adcx   %r9,%rcx
  3ff687:	adox   %r10,%r8
  3ff68d:	mulx   0x48(%r12),%r9,%r10
  3ff694:	adcx   %r9,%r8
  3ff69a:	adox   %r10,%rdi
  3ff6a0:	mulx   0x50(%r12),%r9,%r10
  3ff6a7:	adcx   %r9,%rdi
  3ff6ad:	adox   %r10,%rsi
  3ff6b3:	mulx   0x58(%r12),%r9,%r10
  3ff6ba:	adcx   %r9,%rsi
  3ff6c0:	adox   %r10,%rax
  3ff6c6:	adcx   %r11,%rax
  3ff6cc:	movabs $0xd7cc17b786468f6f,%rcx
  3ff6d6:	movabs $0x43e1f593f0000001,%rdx
  3ff6e0:	movabs $0x2833e84879b97091,%r9
  3ff6ea:	movabs $0xb85045b68181585d,%r10
  3ff6f4:	movabs $0x30644e72e131a029,%r11
  3ff6fe:	vmovups %ymm1,0x160(%rsp)
  3ff707:	add    %rdi,%rcx
  3ff70a:	cmp    %rdx,%r8
  3ff70d:	sbb    $0x0,%rcx
  3ff711:	setb   %dl
  3ff714:	cmp    %r9,%rdi
  3ff717:	setb   %r9b
  3ff71b:	or     %dl,%r9b
  3ff71e:	movabs $0x47afba497e7ea7a3,%rdx
  3ff728:	add    %rsi,%rdx
  3ff72b:	movzbl %r9b,%r9d
  3ff72f:	sub    %r9,%rdx
  3ff732:	setb   %r9b
  3ff736:	cmp    %r10,%rsi
  3ff739:	setb   %r10b
  3ff73d:	or     %r9b,%r10b
  3ff740:	movabs $0xcf9bb18d1ece5fd7,%r9
  3ff74a:	add    %rax,%r9
  3ff74d:	movzbl %r10b,%r10d
  3ff751:	sub    %r10,%r9
  3ff754:	setb   %r10b
  3ff758:	cmp    %r11,%rax
  3ff75b:	setb   %r11b
  3ff75f:	or     %r10b,%r11b
  3ff762:	movabs $0xbc1e0a6c0fffffff,%r10
  3ff76c:	lea    (%r8,%r10,1),%r10
  3ff770:	cmovne %rdi,%rcx
  3ff774:	cmovne %rsi,%rdx
  3ff778:	cmovne %rax,%r9
  3ff77c:	mov    0x18(%rsp),%rdi
  3ff781:	cmovne %r8,%r10
  3ff785:	mov    0x28(%rsp),%r8
  3ff78a:	mov    %r10,0x100(%rsp)
  3ff792:	mov    %rcx,0x108(%rsp)
  3ff79a:	mov    %rdx,0x110(%rsp)
  3ff7a2:	mov    %r9,0x118(%rsp)
  3ff7aa:	add    (%r12),%rbp
  3ff7ae:	adc    0x8(%r12),%r15
  3ff7b3:	adc    0x10(%r12),%rdi
  3ff7b8:	adc    0x18(%r12),%r8
  3ff7bd:	mov    %rbp,%rax
  3ff7c0:	mov    %r15,%rcx
  3ff7c3:	mov    %rdi,%rdx
  3ff7c6:	mov    %r8,%rsi
  3ff7c9:	sub    (%r14),%rax
  3ff7cc:	sbb    0x8(%r14),%rcx
  3ff7d0:	sbb    0x10(%r14),%rdx
  3ff7d4:	sbb    0x18(%r14),%rsi
  3ff7d8:	cmovae %rax,%rbp
  3ff7dc:	cmovae %rcx,%r15
  3ff7e0:	cmovae %rdx,%rdi
  3ff7e4:	cmovae %rsi,%r8
  3ff7e8:	mov    %r8,0x28(%rsp)
  3ff7ed:	mov    $0x3,%r8d
  3ff7f3:	mov    %rdi,0x18(%rsp)
  3ff7f8:	mov    %r15,0x78(%rsp)
  3ff7fd:	mov    %rbp,0x90(%rsp)
  3ff805:	jmp    3ffd16 <<zopro_core::solver::Plan>::run_poseidon_task+0x1456>
  3ff80a:	nopw   0x0(%rax,%rax,1)
  3ff810:	mov    %r11,0x28(%rsp)
  3ff815:	mov    %r10,0x18(%rsp)
  3ff81a:	mov    %r15,0x78(%rsp)
  3ff81f:	sub    $0x8,%rsp
  3ff823:	mov    0x88(%rsp),%rsi
  3ff82b:	mov    0x50(%rsp),%rdx
  3ff830:	mov    0x38(%rsp),%rcx
  3ff835:	lea    0xc8(%rsp),%rdi
  3ff83d:	push   0x28(%rsp)
  3ff841:	lea    0xf0(%rsp),%rax
  3ff849:	push   %rax
  3ff84a:	push   0x28(%rsp)
  3ff84e:	vzeroupper
  3ff851:	call   402a00 <<zopro_core::solver::Plan>::emit_poseidon>
  3ff856:	vmovups -0x3ee0be(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3ff85e:	add    $0x20,%rsp
  3ff862:	jmp    40020b <<zopro_core::solver::Plan>::run_poseidon_task+0x194b>
  3ff867:	nopw   0x0(%rax,%rax,1)
  3ff870:	mov    0x1c8(%rsp),%rdi
  3ff878:	mov    $0x20,%eax
  3ff87d:	mov    %rbp,0x90(%rsp)
  3ff885:	data16 cs nopw 0x0(%rax,%rax,1)
  3ff890:	cmp    %rbx,%rdi
  3ff893:	jae    400e79 <<zopro_core::solver::Plan>::run_poseidon_task+0x25b9>
  3ff899:	mov    0x1d8(%rsp),%rcx
  3ff8a1:	vmovups 0x240(%rsp,%rax,1),%ymm2
  3ff8aa:	mov    %r15,0x78(%rsp)
  3ff8af:	vmovups %ymm1,0x160(%rsp)
  3ff8b8:	mov    %r10,0x18(%rsp)
  3ff8bd:	mov    %r11,0x28(%rsp)
  3ff8c2:	lea    0x160(%rsp),%rbp
  3ff8ca:	lea    0x100(%rsp),%r12
  3ff8d2:	lea    0x50(%rsp),%r15
  3ff8d7:	movabs $0xc2e1f593efffffff,%rbx
  3ff8e1:	vmovups (%rcx,%rax,1),%ymm0
  3ff8e6:	vmovups %ymm2,0x100(%rsp)
  3ff8ef:	vmovups %ymm0,0x50(%rsp)
  3ff8f5:	xor    %r8d,%r8d
  3ff8f8:	xor    %esi,%esi
  3ff8fa:	xor    %ecx,%ecx
  3ff8fc:	xor    %r10d,%r10d
  3ff8ff:	xor    %r9d,%r9d
  3ff902:	mov    (%r12),%rdx
  3ff906:	xor    %r13d,%r13d
  3ff909:	mulx   (%r15),%r11,%r14
  3ff90e:	adcx   %r11,%r8
  3ff914:	adox   %r14,%rsi
  3ff91a:	mulx   0x8(%r15),%r11,%r14
  3ff920:	adcx   %r11,%rsi
  3ff926:	adox   %r14,%rcx
  3ff92c:	mulx   0x10(%r15),%r11,%r14
  3ff932:	adcx   %r11,%rcx
  3ff938:	adox   %r14,%r10
  3ff93e:	mulx   0x18(%r15),%r11,%r14
  3ff944:	adcx   %r11,%r10
  3ff94a:	adox   %r14,%r9
  3ff950:	adcx   %r13,%r9
  3ff956:	mov    %r8,%rdx
  3ff959:	imul   %rbx,%rdx
  3ff95d:	xor    %r13d,%r13d
  3ff960:	mulx   0x0(%rbp),%r11,%r14
  3ff966:	adcx   %r11,%r8
  3ff96c:	adox   %r14,%rsi
  3ff972:	mulx   0x8(%rbp),%r11,%r14
  3ff978:	adcx   %r11,%rsi
  3ff97e:	adox   %r14,%rcx
  3ff984:	mulx   0x10(%rbp),%r11,%r14
  3ff98a:	adcx   %r11,%rcx
  3ff990:	adox   %r14,%r10
  3ff996:	mulx   0x18(%rbp),%r11,%r14
  3ff99c:	adcx   %r11,%r10
  3ff9a2:	adox   %r14,%r9
  3ff9a8:	adcx   %r13,%r9
  3ff9ae:	mov    0x8(%r12),%rdx
  3ff9b3:	xor    %r13d,%r13d
  3ff9b6:	mulx   (%r15),%r11,%r14
  3ff9bb:	adcx   %r11,%rsi
  3ff9c1:	adox   %r14,%rcx
  3ff9c7:	mulx   0x8(%r15),%r11,%r14
  3ff9cd:	adcx   %r11,%rcx
  3ff9d3:	adox   %r14,%r10
  3ff9d9:	mulx   0x10(%r15),%r11,%r14
  3ff9df:	adcx   %r11,%r10
  3ff9e5:	adox   %r14,%r9
  3ff9eb:	mulx   0x18(%r15),%r11,%r14
  3ff9f1:	adcx   %r11,%r9
  3ff9f7:	adox   %r14,%r8
  3ff9fd:	adcx   %r13,%r8
  3ffa03:	mov    %rsi,%rdx
  3ffa06:	imul   %rbx,%rdx
  3ffa0a:	xor    %r13d,%r13d
  3ffa0d:	mulx   0x0(%rbp),%r11,%r14
  3ffa13:	adcx   %r11,%rsi
  3ffa19:	adox   %r14,%rcx
  3ffa1f:	mulx   0x8(%rbp),%r11,%r14
  3ffa25:	adcx   %r11,%rcx
  3ffa2b:	adox   %r14,%r10
  3ffa31:	mulx   0x10(%rbp),%r11,%r14
  3ffa37:	adcx   %r11,%r10
  3ffa3d:	adox   %r14,%r9
  3ffa43:	mulx   0x18(%rbp),%r11,%r14
  3ffa49:	adcx   %r11,%r9
  3ffa4f:	adox   %r14,%r8
  3ffa55:	adcx   %r13,%r8
  3ffa5b:	mov    0x10(%r12),%rdx
  3ffa60:	xor    %r13d,%r13d
  3ffa63:	mulx   (%r15),%r11,%r14
  3ffa68:	adcx   %r11,%rcx
  3ffa6e:	adox   %r14,%r10
  3ffa74:	mulx   0x8(%r15),%r11,%r14
  3ffa7a:	adcx   %r11,%r10
  3ffa80:	adox   %r14,%r9
  3ffa86:	mulx   0x10(%r15),%r11,%r14
  3ffa8c:	adcx   %r11,%r9
  3ffa92:	adox   %r14,%r8
  3ffa98:	mulx   0x18(%r15),%r11,%r14
  3ffa9e:	adcx   %r11,%r8
  3ffaa4:	adox   %r14,%rsi
  3ffaaa:	adcx   %r13,%rsi
  3ffab0:	mov    %rcx,%rdx
  3ffab3:	imul   %rbx,%rdx
  3ffab7:	xor    %r13d,%r13d
  3ffaba:	mulx   0x0(%rbp),%r11,%r14
  3ffac0:	adcx   %r11,%rcx
  3ffac6:	adox   %r14,%r10
  3ffacc:	mulx   0x8(%rbp),%r11,%r14
  3ffad2:	adcx   %r11,%r10
  3ffad8:	adox   %r14,%r9
  3ffade:	mulx   0x10(%rbp),%r11,%r14
  3ffae4:	adcx   %r11,%r9
  3ffaea:	adox   %r14,%r8
  3ffaf0:	mulx   0x18(%rbp),%r11,%r14
  3ffaf6:	adcx   %r11,%r8
  3ffafc:	adox   %r14,%rsi
  3ffb02:	adcx   %r13,%rsi
  3ffb08:	mov    0x18(%r12),%rdx
  3ffb0d:	xor    %r13d,%r13d
  3ffb10:	mulx   (%r15),%r11,%r14
  3ffb15:	adcx   %r11,%r10
  3ffb1b:	adox   %r14,%r9
  3ffb21:	mulx   0x8(%r15),%r11,%r14
  3ffb27:	adcx   %r11,%r9
  3ffb2d:	adox   %r14,%r8
  3ffb33:	mulx   0x10(%r15),%r11,%r14
  3ffb39:	adcx   %r11,%r8
  3ffb3f:	adox   %r14,%rsi
  3ffb45:	mulx   0x18(%r15),%r11,%r14
  3ffb4b:	adcx   %r11,%rsi
  3ffb51:	adox   %r14,%rcx
  3ffb57:	adcx   %r13,%rcx
  3ffb5d:	mov    %r10,%rdx
  3ffb60:	imul   %rbx,%rdx
  3ffb64:	xor    %r13d,%r13d
  3ffb67:	mulx   0x0(%rbp),%r11,%r14
  3ffb6d:	adcx   %r11,%r10
  3ffb73:	adox   %r14,%r9
  3ffb79:	mulx   0x8(%rbp),%r11,%r14
  3ffb7f:	adcx   %r11,%r9
  3ffb85:	adox   %r14,%r8
  3ffb8b:	mulx   0x10(%rbp),%r11,%r14
  3ffb91:	adcx   %r11,%r8
  3ffb97:	adox   %r14,%rsi
  3ffb9d:	mulx   0x18(%rbp),%r11,%r14
  3ffba3:	adcx   %r11,%rsi
  3ffba9:	adox   %r14,%rcx
  3ffbaf:	adcx   %r13,%rcx
  3ffbb5:	movabs $0xd7cc17b786468f6f,%rdx
  3ffbbf:	movabs $0x43e1f593f0000001,%r10
  3ffbc9:	movabs $0x2833e84879b97091,%r11
  3ffbd3:	movabs $0xb85045b68181585d,%r14
  3ffbdd:	movabs $0x30644e72e131a029,%r15
  3ffbe7:	vmovups %ymm1,0x160(%rsp)
  3ffbf0:	add    %r8,%rdx
  3ffbf3:	cmp    %r10,%r9
  3ffbf6:	mov    0x40(%rsp),%rbx
  3ffbfb:	sbb    $0x0,%rdx
  3ffbff:	setb   %r10b
  3ffc03:	cmp    %r11,%r8
  3ffc06:	setb   %r11b
  3ffc0a:	or     %r10b,%r11b
  3ffc0d:	movabs $0x47afba497e7ea7a3,%r10
  3ffc17:	add    %rsi,%r10
  3ffc1a:	movzbl %r11b,%r11d
  3ffc1e:	sub    %r11,%r10
  3ffc21:	setb   %r11b
  3ffc25:	cmp    %r14,%rsi
  3ffc28:	setb   %r14b
  3ffc2c:	or     %r11b,%r14b
  3ffc2f:	movabs $0xcf9bb18d1ece5fd7,%r11
  3ffc39:	add    %rcx,%r11
  3ffc3c:	movzbl %r14b,%r14d
  3ffc40:	sub    %r14,%r11
  3ffc43:	setb   %r14b
  3ffc47:	cmp    %r15,%rcx
  3ffc4a:	mov    0x78(%rsp),%r15
  3ffc4f:	setb   %r13b
  3ffc53:	or     %r14b,%r13b
  3ffc56:	movabs $0xbc1e0a6c0fffffff,%r14
  3ffc60:	lea    (%r9,%r14,1),%r14
  3ffc64:	cmovne %r8,%rdx
  3ffc68:	cmovne %rsi,%r10
  3ffc6c:	cmovne %rcx,%r11
  3ffc70:	cmovne %r9,%r14
  3ffc74:	mov    0x90(%rsp),%r9
  3ffc7c:	add    $0x20,%rax
  3ffc80:	inc    %rdi
  3ffc83:	mov    %r14,0x100(%rsp)
  3ffc8b:	mov    %rdx,0x108(%rsp)
  3ffc93:	mov    %r10,0x110(%rsp)
  3ffc9b:	mov    %r11,0x118(%rsp)
  3ffca3:	mov    0x18(%rsp),%r10
  3ffca8:	mov    0x28(%rsp),%r11
  3ffcad:	add    (%r12),%r9
  3ffcb1:	adc    0x8(%r12),%r15
  3ffcb6:	adc    0x10(%r12),%r10
  3ffcbb:	adc    0x18(%r12),%r11
  3ffcc0:	mov    %r9,%rcx
  3ffcc3:	mov    %r15,%rdx
  3ffcc6:	mov    %r10,%rsi
  3ffcc9:	mov    %r11,%r8
  3ffccc:	sub    0x0(%rbp),%rcx
  3ffcd0:	sbb    0x8(%rbp),%rdx
  3ffcd4:	sbb    0x10(%rbp),%rsi
  3ffcd8:	sbb    0x18(%rbp),%r8
  3ffcdc:	cmovae %rcx,%r9
  3ffce0:	cmovae %rdx,%r15
  3ffce4:	cmovae %rsi,%r10
  3ffce8:	cmovae %r8,%r11
  3ffcec:	mov    %r9,0x90(%rsp)
  3ffcf4:	cmp    %rax,0x88(%rsp)
  3ffcfc:	jne    3ff890 <<zopro_core::solver::Plan>::run_poseidon_task+0xfd0>
  3ffd02:	mov    0x8(%rsp),%r8
  3ffd07:	mov    %r11,0x28(%rsp)
  3ffd0c:	mov    %r10,0x18(%rsp)
  3ffd11:	mov    %r15,0x78(%rsp)
  3ffd16:	sub    $0x8,%rsp
  3ffd1a:	mov    0x88(%rsp),%rsi
  3ffd22:	mov    0x50(%rsp),%rdx
  3ffd27:	mov    0x38(%rsp),%rcx
  3ffd2c:	mov    0x40(%rsp),%r9
  3ffd31:	lea    0xc8(%rsp),%rdi
  3ffd39:	push   0x28(%rsp)
  3ffd3d:	lea    0xf0(%rsp),%rax
  3ffd45:	push   %rax
  3ffd46:	push   0x28(%rsp)
  3ffd4a:	vzeroupper
  3ffd4d:	call   402a00 <<zopro_core::solver::Plan>::emit_poseidon>
  3ffd52:	add    $0x20,%rsp
  3ffd56:	vmovups -0x3ee5be(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3ffd5e:	mov    0x1d0(%rsp),%rdi
  3ffd66:	xor    %eax,%eax
  3ffd68:	nopl   0x0(%rax,%rax,1)
  3ffd70:	cmp    %rbx,%rdi
  3ffd73:	jae    400e67 <<zopro_core::solver::Plan>::run_poseidon_task+0x25a7>
  3ffd79:	mov    0xa8(%rsp),%rcx
  3ffd81:	vmovups 0xc0(%rsp),%ymm2
  3ffd8a:	vmovups %ymm1,0x160(%rsp)
  3ffd93:	lea    0x160(%rsp),%rbp
  3ffd9b:	lea    0x100(%rsp),%r12
  3ffda3:	lea    0x50(%rsp),%r15
  3ffda8:	movabs $0xc2e1f593efffffff,%rbx
  3ffdb2:	vmovups (%rcx,%rax,1),%ymm0
  3ffdb7:	vmovups %ymm2,0x100(%rsp)
  3ffdc0:	vmovups %ymm0,0x50(%rsp)
  3ffdc6:	xor    %r9d,%r9d
  3ffdc9:	xor    %r8d,%r8d
  3ffdcc:	xor    %esi,%esi
  3ffdce:	xor    %ecx,%ecx
  3ffdd0:	xor    %r10d,%r10d
  3ffdd3:	mov    (%r12),%rdx
  3ffdd7:	xor    %r13d,%r13d
  3ffdda:	mulx   (%r15),%r11,%r14
  3ffddf:	adcx   %r11,%r9
  3ffde5:	adox   %r14,%r8
  3ffdeb:	mulx   0x8(%r15),%r11,%r14
  3ffdf1:	adcx   %r11,%r8
  3ffdf7:	adox   %r14,%rsi
  3ffdfd:	mulx   0x10(%r15),%r11,%r14
  3ffe03:	adcx   %r11,%rsi
  3ffe09:	adox   %r14,%rcx
  3ffe0f:	mulx   0x18(%r15),%r11,%r14
  3ffe15:	adcx   %r11,%rcx
  3ffe1b:	adox   %r14,%r10
  3ffe21:	adcx   %r13,%r10
  3ffe27:	mov    %r9,%rdx
  3ffe2a:	imul   %rbx,%rdx
  3ffe2e:	xor    %r13d,%r13d
  3ffe31:	mulx   0x0(%rbp),%r11,%r14
  3ffe37:	adcx   %r11,%r9
  3ffe3d:	adox   %r14,%r8
  3ffe43:	mulx   0x8(%rbp),%r11,%r14
  3ffe49:	adcx   %r11,%r8
  3ffe4f:	adox   %r14,%rsi
  3ffe55:	mulx   0x10(%rbp),%r11,%r14
  3ffe5b:	adcx   %r11,%rsi
  3ffe61:	adox   %r14,%rcx
  3ffe67:	mulx   0x18(%rbp),%r11,%r14
  3ffe6d:	adcx   %r11,%rcx
  3ffe73:	adox   %r14,%r10
  3ffe79:	adcx   %r13,%r10
  3ffe7f:	mov    0x8(%r12),%rdx
  3ffe84:	xor    %r13d,%r13d
  3ffe87:	mulx   (%r15),%r11,%r14
  3ffe8c:	adcx   %r11,%r8
  3ffe92:	adox   %r14,%rsi
  3ffe98:	mulx   0x8(%r15),%r11,%r14
  3ffe9e:	adcx   %r11,%rsi
  3ffea4:	adox   %r14,%rcx
  3ffeaa:	mulx   0x10(%r15),%r11,%r14
  3ffeb0:	adcx   %r11,%rcx
  3ffeb6:	adox   %r14,%r10
  3ffebc:	mulx   0x18(%r15),%r11,%r14
  3ffec2:	adcx   %r11,%r10
  3ffec8:	adox   %r14,%r9
  3ffece:	adcx   %r13,%r9
  3ffed4:	mov    %r8,%rdx
  3ffed7:	imul   %rbx,%rdx
  3ffedb:	xor    %r13d,%r13d
  3ffede:	mulx   0x0(%rbp),%r11,%r14
  3ffee4:	adcx   %r11,%r8
  3ffeea:	adox   %r14,%rsi
  3ffef0:	mulx   0x8(%rbp),%r11,%r14
  3ffef6:	adcx   %r11,%rsi
  3ffefc:	adox   %r14,%rcx
  3fff02:	mulx   0x10(%rbp),%r11,%r14
  3fff08:	adcx   %r11,%rcx
  3fff0e:	adox   %r14,%r10
  3fff14:	mulx   0x18(%rbp),%r11,%r14
  3fff1a:	adcx   %r11,%r10
  3fff20:	adox   %r14,%r9
  3fff26:	adcx   %r13,%r9
  3fff2c:	mov    0x10(%r12),%rdx
  3fff31:	xor    %r13d,%r13d
  3fff34:	mulx   (%r15),%r11,%r14
  3fff39:	adcx   %r11,%rsi
  3fff3f:	adox   %r14,%rcx
  3fff45:	mulx   0x8(%r15),%r11,%r14
  3fff4b:	adcx   %r11,%rcx
  3fff51:	adox   %r14,%r10
  3fff57:	mulx   0x10(%r15),%r11,%r14
  3fff5d:	adcx   %r11,%r10
  3fff63:	adox   %r14,%r9
  3fff69:	mulx   0x18(%r15),%r11,%r14
  3fff6f:	adcx   %r11,%r9
  3fff75:	adox   %r14,%r8
  3fff7b:	adcx   %r13,%r8
  3fff81:	mov    %rsi,%rdx
  3fff84:	imul   %rbx,%rdx
  3fff88:	xor    %r13d,%r13d
  3fff8b:	mulx   0x0(%rbp),%r11,%r14
  3fff91:	adcx   %r11,%rsi
  3fff97:	adox   %r14,%rcx
  3fff9d:	mulx   0x8(%rbp),%r11,%r14
  3fffa3:	adcx   %r11,%rcx
  3fffa9:	adox   %r14,%r10
  3fffaf:	mulx   0x10(%rbp),%r11,%r14
  3fffb5:	adcx   %r11,%r10
  3fffbb:	adox   %r14,%r9
  3fffc1:	mulx   0x18(%rbp),%r11,%r14
  3fffc7:	adcx   %r11,%r9
  3fffcd:	adox   %r14,%r8
  3fffd3:	adcx   %r13,%r8
  3fffd9:	mov    0x18(%r12),%rdx
  3fffde:	xor    %r13d,%r13d
  3fffe1:	mulx   (%r15),%r11,%r14
  3fffe6:	adcx   %r11,%rcx
  3fffec:	adox   %r14,%r10
  3ffff2:	mulx   0x8(%r15),%r11,%r14
  3ffff8:	adcx   %r11,%r10
  3ffffe:	adox   %r14,%r9
  400004:	mulx   0x10(%r15),%r11,%r14
  40000a:	adcx   %r11,%r9
  400010:	adox   %r14,%r8
  400016:	mulx   0x18(%r15),%r11,%r14
  40001c:	adcx   %r11,%r8
  400022:	adox   %r14,%rsi
  400028:	adcx   %r13,%rsi
  40002e:	mov    %rcx,%rdx
  400031:	imul   %rbx,%rdx
  400035:	xor    %r13d,%r13d
  400038:	mulx   0x0(%rbp),%r11,%r14
  40003e:	adcx   %r11,%rcx
  400044:	adox   %r14,%r10
  40004a:	mulx   0x8(%rbp),%r11,%r14
  400050:	adcx   %r11,%r10
  400056:	adox   %r14,%r9
  40005c:	mulx   0x10(%rbp),%r11,%r14
  400062:	adcx   %r11,%r9
  400068:	adox   %r14,%r8
  40006e:	mulx   0x18(%rbp),%r11,%r14
  400074:	adcx   %r11,%r8
  40007a:	adox   %r14,%rsi
  400080:	adcx   %r13,%rsi
  400086:	movabs $0xd7cc17b786468f6f,%rcx
  400090:	movabs $0x43e1f593f0000001,%rdx
  40009a:	movabs $0x2833e84879b97091,%r11
  4000a4:	movabs $0xb85045b68181585d,%r14
  4000ae:	movabs $0x30644e72e131a029,%r15
  4000b8:	vmovups %ymm1,0x160(%rsp)
  4000c1:	add    %r9,%rcx
  4000c4:	cmp    %rdx,%r10
  4000c7:	mov    0x40(%rsp),%rbx
  4000cc:	sbb    $0x0,%rcx
  4000d0:	setb   %dl
  4000d3:	cmp    %r11,%r9
  4000d6:	setb   %r11b
  4000da:	or     %dl,%r11b
  4000dd:	movabs $0x47afba497e7ea7a3,%rdx
  4000e7:	add    %r8,%rdx
  4000ea:	movzbl %r11b,%r11d
  4000ee:	sub    %r11,%rdx
  4000f1:	setb   %r11b
  4000f5:	cmp    %r14,%r8
  4000f8:	setb   %r14b
  4000fc:	or     %r11b,%r14b
  4000ff:	movabs $0xcf9bb18d1ece5fd7,%r11
  400109:	add    %rsi,%r11
  40010c:	movzbl %r14b,%r14d
  400110:	sub    %r14,%r11
  400113:	setb   %r14b
  400117:	cmp    %r15,%rsi
  40011a:	setb   %r13b
  40011e:	or     %r14b,%r13b
  400121:	movabs $0xbc1e0a6c0fffffff,%r14
  40012b:	lea    (%r10,%r14,1),%r14
  40012f:	cmovne %r9,%rcx
  400133:	cmovne %r8,%rdx
  400137:	cmovne %rsi,%r11
  40013b:	mov    0x260(%rsp,%rax,1),%rsi
  400143:	mov    0x268(%rsp,%rax,1),%r8
  40014b:	mov    0x270(%rsp,%rax,1),%r9
  400153:	cmovne %r10,%r14
  400157:	mov    0x278(%rsp,%rax,1),%r10
  40015f:	inc    %rdi
  400162:	mov    %r14,0x100(%rsp)
  40016a:	mov    %rcx,0x108(%rsp)
  400172:	mov    %rdx,0x110(%rsp)
  40017a:	mov    %r11,0x118(%rsp)
  400182:	add    (%r12),%rsi
  400186:	adc    0x8(%r12),%r8
  40018b:	adc    0x10(%r12),%r9
  400190:	adc    0x18(%r12),%r10
  400195:	mov    %rsi,%rcx
  400198:	mov    %r8,%rdx
  40019b:	mov    %r9,%r11
  40019e:	mov    %r10,%r14
  4001a1:	sub    0x0(%rbp),%rcx
  4001a5:	sbb    0x8(%rbp),%rdx
  4001a9:	sbb    0x10(%rbp),%r11
  4001ad:	sbb    0x18(%rbp),%r14
  4001b1:	cmovae %rcx,%rsi
  4001b5:	cmovae %rdx,%r8
  4001b9:	cmovae %r11,%r9
  4001bd:	cmovae %r14,%r10
  4001c1:	mov    %rsi,0x260(%rsp,%rax,1)
  4001c9:	mov    %r8,0x268(%rsp,%rax,1)
  4001d1:	mov    %r9,0x270(%rsp,%rax,1)
  4001d9:	mov    %r10,0x278(%rsp,%rax,1)
  4001e1:	add    $0x20,%rax
  4001e5:	cmp    %rax,0x238(%rsp)
  4001ed:	jne    3ffd70 <<zopro_core::solver::Plan>::run_poseidon_task+0x14b0>
  4001f3:	mov    0x230(%rsp),%r13
  4001fb:	mov    0x90(%rsp),%rbp
  400203:	mov    0x228(%rsp),%r14
  40020b:	cmp    %rbx,%r14
  40020e:	jae    400ef5 <<zopro_core::solver::Plan>::run_poseidon_task+0x2635>
  400214:	mov    (%rsp),%rax
  400218:	shl    $0x5,%r14
  40021c:	vmovups 0xc0(%rsp),%ymm2
  400225:	incq   0x10(%rsp)
  40022a:	vmovups %ymm1,0x160(%rsp)
  400233:	lea    0x100(%rsp),%r12
  40023b:	lea    0x50(%rsp),%r15
  400240:	movabs $0xc2e1f593efffffff,%rbx
  40024a:	vmovups (%rax,%r14,1),%ymm0
  400250:	lea    0x160(%rsp),%r14
  400258:	vmovups %ymm2,0x100(%rsp)
  400261:	vmovups %ymm0,0x50(%rsp)
  400267:	xor    %r8d,%r8d
  40026a:	xor    %edi,%edi
  40026c:	xor    %esi,%esi
  40026e:	xor    %eax,%eax
  400270:	xor    %r9d,%r9d
  400273:	mov    (%r12),%rdx
  400277:	xor    %r11d,%r11d
  40027a:	mulx   (%r15),%rcx,%r10
  40027f:	adcx   %rcx,%r8
  400285:	adox   %r10,%rdi
  40028b:	mulx   0x8(%r15),%rcx,%r10
  400291:	adcx   %rcx,%rdi
  400297:	adox   %r10,%rsi
  40029d:	mulx   0x10(%r15),%rcx,%r10
  4002a3:	adcx   %rcx,%rsi
  4002a9:	adox   %r10,%rax
  4002af:	mulx   0x18(%r15),%rcx,%r10
  4002b5:	adcx   %rcx,%rax
  4002bb:	adox   %r10,%r9
  4002c1:	adcx   %r11,%r9
  4002c7:	mov    %r8,%rdx
  4002ca:	imul   %rbx,%rdx
  4002ce:	xor    %r11d,%r11d
  4002d1:	mulx   (%r14),%rcx,%r10
  4002d6:	adcx   %rcx,%r8
  4002dc:	adox   %r10,%rdi
  4002e2:	mulx   0x8(%r14),%rcx,%r10
  4002e8:	adcx   %rcx,%rdi
  4002ee:	adox   %r10,%rsi
  4002f4:	mulx   0x10(%r14),%rcx,%r10
  4002fa:	adcx   %rcx,%rsi
  400300:	adox   %r10,%rax
  400306:	mulx   0x18(%r14),%rcx,%r10
  40030c:	adcx   %rcx,%rax
  400312:	adox   %r10,%r9
  400318:	adcx   %r11,%r9
  40031e:	mov    0x8(%r12),%rdx
  400323:	xor    %r11d,%r11d
  400326:	mulx   (%r15),%rcx,%r10
  40032b:	adcx   %rcx,%rdi
  400331:	adox   %r10,%rsi
  400337:	mulx   0x8(%r15),%rcx,%r10
  40033d:	adcx   %rcx,%rsi
  400343:	adox   %r10,%rax
  400349:	mulx   0x10(%r15),%rcx,%r10
  40034f:	adcx   %rcx,%rax
  400355:	adox   %r10,%r9
  40035b:	mulx   0x18(%r15),%rcx,%r10
  400361:	adcx   %rcx,%r9
  400367:	adox   %r10,%r8
  40036d:	adcx   %r11,%r8
  400373:	mov    %rdi,%rdx
  400376:	imul   %rbx,%rdx
  40037a:	xor    %r11d,%r11d
  40037d:	mulx   (%r14),%rcx,%r10
  400382:	adcx   %rcx,%rdi
  400388:	adox   %r10,%rsi
  40038e:	mulx   0x8(%r14),%rcx,%r10
  400394:	adcx   %rcx,%rsi
  40039a:	adox   %r10,%rax
  4003a0:	mulx   0x10(%r14),%rcx,%r10
  4003a6:	adcx   %rcx,%rax
  4003ac:	adox   %r10,%r9
  4003b2:	mulx   0x18(%r14),%rcx,%r10
  4003b8:	adcx   %rcx,%r9
  4003be:	adox   %r10,%r8
  4003c4:	adcx   %r11,%r8
  4003ca:	mov    0x10(%r12),%rdx
  4003cf:	xor    %r11d,%r11d
  4003d2:	mulx   (%r15),%rcx,%r10
  4003d7:	adcx   %rcx,%rsi
  4003dd:	adox   %r10,%rax
  4003e3:	mulx   0x8(%r15),%rcx,%r10
  4003e9:	adcx   %rcx,%rax
  4003ef:	adox   %r10,%r9
  4003f5:	mulx   0x10(%r15),%rcx,%r10
  4003fb:	adcx   %rcx,%r9
  400401:	adox   %r10,%r8
  400407:	mulx   0x18(%r15),%rcx,%r10
  40040d:	adcx   %rcx,%r8
  400413:	adox   %r10,%rdi
  400419:	adcx   %r11,%rdi
  40041f:	mov    %rsi,%rdx
  400422:	imul   %rbx,%rdx
  400426:	xor    %r11d,%r11d
  400429:	mulx   (%r14),%rcx,%r10
  40042e:	adcx   %rcx,%rsi
  400434:	adox   %r10,%rax
  40043a:	mulx   0x8(%r14),%rcx,%r10
  400440:	adcx   %rcx,%rax
  400446:	adox   %r10,%r9
  40044c:	mulx   0x10(%r14),%rcx,%r10
  400452:	adcx   %rcx,%r9
  400458:	adox   %r10,%r8
  40045e:	mulx   0x18(%r14),%rcx,%r10
  400464:	adcx   %rcx,%r8
  40046a:	adox   %r10,%rdi
  400470:	adcx   %r11,%rdi
  400476:	mov    0x18(%r12),%rdx
  40047b:	xor    %r11d,%r11d
  40047e:	mulx   (%r15),%rcx,%r10
  400483:	adcx   %rcx,%rax
  400489:	adox   %r10,%r9
  40048f:	mulx   0x8(%r15),%rcx,%r10
  400495:	adcx   %rcx,%r9
  40049b:	adox   %r10,%r8
  4004a1:	mulx   0x10(%r15),%rcx,%r10
  4004a7:	adcx   %rcx,%r8
  4004ad:	adox   %r10,%rdi
  4004b3:	mulx   0x18(%r15),%rcx,%r10
  4004b9:	adcx   %rcx,%rdi
  4004bf:	adox   %r10,%rsi
  4004c5:	adcx   %r11,%rsi
  4004cb:	mov    %rax,%rdx
  4004ce:	imul   %rbx,%rdx
  4004d2:	xor    %r11d,%r11d
  4004d5:	mulx   (%r14),%rcx,%r10
  4004da:	adcx   %rcx,%rax
  4004e0:	adox   %r10,%r9
  4004e6:	mulx   0x8(%r14),%rcx,%r10
  4004ec:	adcx   %rcx,%r9
  4004f2:	adox   %r10,%r8
  4004f8:	mulx   0x10(%r14),%rcx,%r10
  4004fe:	adcx   %rcx,%r8
  400504:	adox   %r10,%rdi
  40050a:	mulx   0x18(%r14),%rcx,%r10
  400510:	adcx   %rcx,%rdi
  400516:	adox   %r10,%rsi
  40051c:	adcx   %r11,%rsi
  400522:	movabs $0xd7cc17b786468f6f,%rax
  40052c:	movabs $0x43e1f593f0000001,%rcx
  400536:	movabs $0x2833e84879b97091,%rdx
  400540:	movabs $0xb85045b68181585d,%r10
  40054a:	movabs $0x30644e72e131a029,%r11
  400554:	mov    %rbp,0x100(%rsp)
  40055c:	vmovups %ymm1,0x160(%rsp)
  400565:	add    %r8,%rax
  400568:	cmp    %rcx,%r9
  40056b:	mov    0x78(%rsp),%rbx
  400570:	sbb    $0x0,%rax
  400574:	setb   %cl
  400577:	cmp    %rdx,%r8
  40057a:	setb   %dl
  40057d:	or     %cl,%dl
  40057f:	movabs $0x47afba497e7ea7a3,%rcx
  400589:	add    %rdi,%rcx
  40058c:	movzbl %dl,%edx
  40058f:	mov    %rbx,0x108(%rsp)
  400597:	sub    %rdx,%rcx
  40059a:	setb   %dl
  40059d:	cmp    %r10,%rdi
  4005a0:	setb   %r10b
  4005a4:	or     %dl,%r10b
  4005a7:	movabs $0xcf9bb18d1ece5fd7,%rdx
  4005b1:	add    %rsi,%rdx
  4005b4:	movzbl %r10b,%r10d
  4005b8:	sub    %r10,%rdx
  4005bb:	setb   %r10b
  4005bf:	cmp    %r11,%rsi
  4005c2:	setb   %r11b
  4005c6:	or     %r10b,%r11b
  4005c9:	mov    0x18(%rsp),%r11
  4005ce:	movabs $0xbc1e0a6c0fffffff,%r10
  4005d8:	cmovne %rsi,%rdx
  4005dc:	mov    0x28(%rsp),%rsi
  4005e1:	lea    (%r9,%r10,1),%r10
  4005e5:	cmovne %r8,%rax
  4005e9:	cmovne %rdi,%rcx
  4005ed:	cmovne %r9,%r10
  4005f1:	mov    %r11,0x110(%rsp)
  4005f9:	mov    %rsi,0x118(%rsp)
  400601:	add    (%r12),%r10
  400605:	adc    0x8(%r12),%rax
  40060a:	adc    0x10(%r12),%rcx
  40060f:	adc    0x18(%r12),%rdx
  400614:	mov    %r10,%rsi
  400617:	mov    %rax,%rdi
  40061a:	mov    %rcx,%r8
  40061d:	mov    %rdx,%r9
  400620:	sub    (%r14),%rsi
  400623:	sbb    0x8(%r14),%rdi
  400627:	sbb    0x10(%r14),%r8
  40062b:	sbb    0x18(%r14),%r9
  40062f:	cmovae %rsi,%r10
  400633:	cmovae %rdi,%rax
  400637:	cmovae %r8,%rcx
  40063b:	cmovae %r9,%rdx
  40063f:	mov    %r10,0xe0(%rsp)
  400647:	mov    %rax,0xe8(%rsp)
  40064f:	mov    %rcx,0xf0(%rsp)
  400657:	mov    %rdx,0xf8(%rsp)
  40065f:	mov    %r13,%r14
  400662:	mov    0x218(%rsp),%r8
  40066a:	mov    0x210(%rsp),%rax
  400672:	mov    0x38(%rsp),%r9
  400677:	mov    0x80(%rsp),%rbp
  40067f:	mov    0xb0(%rsp),%rdi
  400687:	add    %r8,0x1c8(%rsp)
  40068f:	add    %rax,0x1d8(%rsp)
  400697:	add    %r8,0x1d0(%rsp)
  40069f:	add    %rax,0xa8(%rsp)
  4006a7:	cmp    %r9,%r13
  4006aa:	mov    0x20(%rsp),%r13
  4006af:	jne    3ff150 <<zopro_core::solver::Plan>::run_poseidon_task+0x890>
  4006b5:	mov    0xe8(%rsp),%rdx
  4006bd:	mov    0xf0(%rsp),%rcx
  4006c5:	mov    0xe0(%rsp),%rax
  4006cd:	mov    %rdx,0x248(%rsp)
  4006d5:	mov    0xf8(%rsp),%rdx
  4006dd:	mov    %rcx,0x250(%rsp)
  4006e5:	mov    0x8(%rsp),%rcx
  4006ea:	mov    %rax,0x240(%rsp)
  4006f2:	mov    %rdx,0x258(%rsp)
  4006fa:	mov    0x40(%rsp),%rdx
  4006ff:	neg    %rcx
  400702:	mov    %rcx,0x28(%rsp)
  400707:	mov    %rdx,%rcx
  40070a:	neg    %rcx
  40070d:	cmp    %rdx,%rdi
  400710:	mov    %rdx,%r14
  400713:	mov    %rcx,0x78(%rsp)
  400718:	mov    (%rsp),%rcx
  40071c:	cmovb  %rdi,%r14
  400720:	shl    $0x5,%rdi
  400724:	xor    %ebx,%ebx
  400726:	xor    %r15d,%r15d
  400729:	sub    %rdx,%r14
  40072c:	mov    %rdi,0x90(%rsp)
  400734:	lea    (%rcx,%rdi,1),%rax
  400738:	mov    %rax,0x18(%rsp)
  40073d:	nopl   (%rax)
  400740:	vmovups -0x3eefa8(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  400748:	cmp    %rbx,%r14
  40074b:	je     400f0c <<zopro_core::solver::Plan>::run_poseidon_task+0x264c>
  400751:	mov    0x18(%rsp),%rdi
  400756:	vmovups %ymm1,0x160(%rsp)
  40075f:	mov    0x258(%rsp,%r15,1),%rax
  400767:	mov    0x250(%rsp,%r15,1),%rcx
  40076f:	mov    0x240(%rsp,%r15,1),%rdx
  400777:	mov    0x248(%rsp,%r15,1),%rsi
  40077f:	lea    0x160(%rsp),%r11
  400787:	lea    0x100(%rsp),%r12
  40078f:	vmovups (%rdi,%r15,1),%ymm0
  400795:	vmovups %ymm0,0x100(%rsp)
  40079e:	add    (%r12),%rdx
  4007a2:	adc    0x8(%r12),%rsi
  4007a7:	adc    0x10(%r12),%rcx
  4007ac:	adc    0x18(%r12),%rax
  4007b1:	mov    %rdx,%rdi
  4007b4:	mov    %rsi,%r8
  4007b7:	mov    %rcx,%r9
  4007ba:	mov    %rax,%r10
  4007bd:	sub    (%r11),%rdi
  4007c0:	sbb    0x8(%r11),%r8
  4007c4:	sbb    0x10(%r11),%r9
  4007c8:	sbb    0x18(%r11),%r10
  4007cc:	cmovae %rdi,%rdx
  4007d0:	cmovae %r8,%rsi
  4007d4:	cmovae %r9,%rcx
  4007d8:	cmovae %r10,%rax
  4007dc:	mov    %rdx,0x50(%rsp)
  4007e1:	mov    %rsi,0x58(%rsp)
  4007e6:	mov    %rcx,0x60(%rsp)
  4007eb:	mov    %rax,0x68(%rsp)
  4007f0:	sub    $0x8,%rsp
  4007f4:	mov    0x50(%rsp),%rdx
  4007f9:	mov    0x38(%rsp),%rcx
  4007fe:	mov    0x10(%rsp),%r8
  400803:	mov    0x40(%rsp),%r9
  400808:	lea    0xc8(%rsp),%rdi
  400810:	mov    %rbp,%rsi
  400813:	push   %r13
  400815:	lea    0x60(%rsp),%rax
  40081a:	push   %rax
  40081b:	mov    0x28(%rsp),%r12
  400820:	push   %r12
  400822:	vzeroupper
  400825:	call   402a00 <<zopro_core::solver::Plan>::emit_poseidon>
  40082a:	add    $0x20,%rsp
  40082e:	vmovups 0xc0(%rsp),%ymm0
  400837:	inc    %r12
  40083a:	dec    %rbx
  40083d:	mov    %r12,0x10(%rsp)
  400842:	vmovups %ymm0,0x240(%rsp,%r15,1)
  40084c:	add    $0x20,%r15
  400850:	cmp    %rbx,0x28(%rsp)
  400855:	jne    400740 <<zopro_core::solver::Plan>::run_poseidon_task+0x1e80>
  40085b:	mov    (%rsp),%r15
  40085f:	mov    0x40(%rsp),%rbx
  400864:	mov    0x8(%rsp),%rsi
  400869:	mov    0xb0(%rsp),%rax
  400871:	mov    0x98(%rsp),%r8
  400879:	lea    0x240(%rsp),%rdi
  400881:	lea    (%rax,%rsi,1),%r14
  400885:	mov    %r15,%rdx
  400888:	mov    %rbx,%rcx
  40088b:	vzeroupper
  40088e:	call   402000 <<zopro_core::solver::Plan>::run_poseidon::dense>
  400893:	cmp    %r14,%rbx
  400896:	mov    %r14,%rax
  400899:	cmovb  %rbx,%rax
  40089d:	mov    %r14,%rbx
  4008a0:	shl    $0x5,%rbx
  4008a4:	xor    %ebp,%ebp
  4008a6:	add    %rax,0x78(%rsp)
  4008ab:	add    %r15,%rbx
  4008ae:	xor    %r15d,%r15d
  4008b1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4008c0:	vmovups -0x3ef128(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  4008c8:	cmp    %r15,0x78(%rsp)
  4008cd:	je     400f2b <<zopro_core::solver::Plan>::run_poseidon_task+0x266b>
  4008d3:	vmovups (%rbx,%rbp,1),%ymm0
  4008d8:	vmovups %ymm1,0x160(%rsp)
  4008e1:	mov    0x258(%rsp,%rbp,1),%rax
  4008e9:	mov    0x250(%rsp,%rbp,1),%rcx
  4008f1:	mov    0x240(%rsp,%rbp,1),%rdx
  4008f9:	mov    0x248(%rsp,%rbp,1),%rsi
  400901:	lea    0x160(%rsp),%r11
  400909:	lea    0x100(%rsp),%r13
  400911:	vmovups %ymm0,0x100(%rsp)
  40091a:	add    0x0(%r13),%rdx
  40091e:	adc    0x8(%r13),%rsi
  400922:	adc    0x10(%r13),%rcx
  400926:	adc    0x18(%r13),%rax
  40092a:	mov    %rdx,%rdi
  40092d:	mov    %rsi,%r8
  400930:	mov    %rcx,%r9
  400933:	mov    %rax,%r10
  400936:	sub    (%r11),%rdi
  400939:	sbb    0x8(%r11),%r8
  40093d:	sbb    0x10(%r11),%r9
  400941:	sbb    0x18(%r11),%r10
  400945:	cmovae %rdi,%rdx
  400949:	cmovae %r8,%rsi
  40094d:	cmovae %r9,%rcx
  400951:	cmovae %r10,%rax
  400955:	mov    %rdx,0x50(%rsp)
  40095a:	mov    %rsi,0x58(%rsp)
  40095f:	mov    %rcx,0x60(%rsp)
  400964:	mov    %rax,0x68(%rsp)
  400969:	sub    $0x8,%rsp
  40096d:	mov    0x88(%rsp),%rsi
  400975:	mov    0x50(%rsp),%rdx
  40097a:	mov    0x38(%rsp),%rcx
  40097f:	mov    0x10(%rsp),%r8
  400984:	mov    0x40(%rsp),%r9
  400989:	lea    0xc8(%rsp),%rdi
  400991:	push   0x28(%rsp)
  400995:	lea    0x60(%rsp),%rax
  40099a:	push   %rax
  40099b:	mov    0x28(%rsp),%r12
  4009a0:	push   %r12
  4009a2:	vzeroupper
  4009a5:	call   402a00 <<zopro_core::solver::Plan>::emit_poseidon>
  4009aa:	add    $0x20,%rsp
  4009ae:	vmovups 0xc0(%rsp),%ymm0
  4009b7:	inc    %r12
  4009ba:	dec    %r15
  4009bd:	mov    %r12,0x10(%rsp)
  4009c2:	vmovups %ymm0,0x240(%rsp,%rbp,1)
  4009cb:	add    $0x20,%rbp
  4009cf:	cmp    %r15,0x28(%rsp)
  4009d4:	jne    4008c0 <<zopro_core::solver::Plan>::run_poseidon_task+0x2000>
  4009da:	mov    0x8(%rsp),%rsi
  4009df:	mov    (%rsp),%rbx
  4009e3:	mov    0x40(%rsp),%r13
  4009e8:	mov    0x98(%rsp),%r8
  4009f0:	lea    0x240(%rsp),%rdi
  4009f8:	add    %rsi,%r14
  4009fb:	mov    %rbx,%rdx
  4009fe:	mov    %r13,%rcx
  400a01:	vzeroupper
  400a04:	call   402000 <<zopro_core::solver::Plan>::run_poseidon::dense>
  400a09:	mov    0xb8(%rsp),%rax
  400a11:	mov    0x80(%rsp),%rbp
  400a19:	mov    %r14,%r15
  400a1c:	add    %rbx,%rax
  400a1f:	add    0x90(%rsp),%rax
  400a27:	xor    %ebx,%ebx
  400a29:	mov    %rax,0xb8(%rsp)
  400a31:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  400a40:	vmovups -0x3ef2a8(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  400a48:	cmp    %r13,%r15
  400a4b:	jae    400f55 <<zopro_core::solver::Plan>::run_poseidon_task+0x2695>
  400a51:	mov    0xb8(%rsp),%rdi
  400a59:	vmovups %ymm1,0x160(%rsp)
  400a62:	mov    0x258(%rsp,%rbx,1),%rax
  400a6a:	mov    0x250(%rsp,%rbx,1),%rcx
  400a72:	mov    0x240(%rsp,%rbx,1),%rdx
  400a7a:	mov    0x248(%rsp,%rbx,1),%rsi
  400a82:	lea    0x160(%rsp),%r11
  400a8a:	lea    0x100(%rsp),%r12
  400a92:	vmovups (%rdi,%rbx,1),%ymm0
  400a97:	vmovups %ymm0,0x100(%rsp)
  400aa0:	add    (%r12),%rdx
  400aa4:	adc    0x8(%r12),%rsi
  400aa9:	adc    0x10(%r12),%rcx
  400aae:	adc    0x18(%r12),%rax
  400ab3:	mov    %rdx,%rdi
  400ab6:	mov    %rsi,%r8
  400ab9:	mov    %rcx,%r9
  400abc:	mov    %rax,%r10
  400abf:	sub    (%r11),%rdi
  400ac2:	sbb    0x8(%r11),%r8
  400ac6:	sbb    0x10(%r11),%r9
  400aca:	sbb    0x18(%r11),%r10
  400ace:	cmovae %rdi,%rdx
  400ad2:	cmovae %r8,%rsi
  400ad6:	cmovae %r9,%rcx
  400ada:	cmovae %r10,%rax
  400ade:	mov    %rdx,0x50(%rsp)
  400ae3:	mov    %rsi,0x58(%rsp)
  400ae8:	mov    %rcx,0x60(%rsp)
  400aed:	mov    %rax,0x68(%rsp)
  400af2:	sub    $0x8,%rsp
  400af6:	mov    0x50(%rsp),%rdx
  400afb:	mov    0x38(%rsp),%rcx
  400b00:	mov    0x10(%rsp),%r8
  400b05:	mov    0x40(%rsp),%r9
  400b0a:	lea    0xc8(%rsp),%rdi
  400b12:	mov    %rbp,%rsi
  400b15:	push   0x28(%rsp)
  400b19:	lea    0x60(%rsp),%rax
  400b1e:	push   %rax
  400b1f:	mov    0x28(%rsp),%r12
  400b24:	push   %r12
  400b26:	vzeroupper
  400b29:	call   402a00 <<zopro_core::solver::Plan>::emit_poseidon>
  400b2e:	add    $0x20,%rsp
  400b32:	vmovups 0xc0(%rsp),%ymm0
  400b3b:	inc    %r12
  400b3e:	inc    %r15
  400b41:	mov    %r12,0x10(%rsp)
  400b46:	vmovups %ymm0,0x240(%rsp,%rbx,1)
  400b4f:	add    $0x20,%rbx
  400b53:	cmp    %rbx,0x88(%rsp)
  400b5b:	jne    400a40 <<zopro_core::solver::Plan>::run_poseidon_task+0x2180>
  400b61:	mov    0x8(%rsp),%rsi
  400b66:	mov    (%rsp),%rdx
  400b6a:	mov    0x98(%rsp),%r8
  400b72:	lea    0x240(%rsp),%rdi
  400b7a:	mov    %r13,%rcx
  400b7d:	add    %rsi,%r14
  400b80:	vzeroupper
  400b83:	call   402000 <<zopro_core::solver::Plan>::run_poseidon::dense>
  400b88:	mov    0x1e8(%rsp),%rax
  400b90:	xor    %ebx,%ebx
  400b92:	shl    $0x5,%eax
  400b95:	add    %rax,0x18(%rsp)
  400b9a:	nopw   0x0(%rax,%rax,1)
  400ba0:	vmovups -0x3ef408(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  400ba8:	cmp    %r13,%r14
  400bab:	jae    400f6c <<zopro_core::solver::Plan>::run_poseidon_task+0x26ac>
  400bb1:	mov    0x18(%rsp),%rdi
  400bb6:	vmovups %ymm1,0x160(%rsp)
  400bbf:	mov    0x258(%rsp,%rbx,1),%rax
  400bc7:	mov    0x250(%rsp,%rbx,1),%rcx
  400bcf:	mov    0x240(%rsp,%rbx,1),%rdx
  400bd7:	mov    0x248(%rsp,%rbx,1),%rsi
  400bdf:	lea    0x160(%rsp),%r11
  400be7:	lea    0x100(%rsp),%r12
  400bef:	vmovups (%rdi,%rbx,1),%ymm0
  400bf4:	vmovups %ymm0,0x100(%rsp)
  400bfd:	add    (%r12),%rdx
  400c01:	adc    0x8(%r12),%rsi
  400c06:	adc    0x10(%r12),%rcx
  400c0b:	adc    0x18(%r12),%rax
  400c10:	mov    %rdx,%rdi
  400c13:	mov    %rsi,%r8
  400c16:	mov    %rcx,%r9
  400c19:	mov    %rax,%r10
  400c1c:	sub    (%r11),%rdi
  400c1f:	sbb    0x8(%r11),%r8
  400c23:	sbb    0x10(%r11),%r9
  400c27:	sbb    0x18(%r11),%r10
  400c2b:	cmovae %rdi,%rdx
  400c2f:	cmovae %r8,%rsi
  400c33:	cmovae %r9,%rcx
  400c37:	cmovae %r10,%rax
  400c3b:	mov    %rdx,0x50(%rsp)
  400c40:	mov    %rsi,0x58(%rsp)
  400c45:	mov    %rcx,0x60(%rsp)
  400c4a:	mov    %rax,0x68(%rsp)
  400c4f:	sub    $0x8,%rsp
  400c53:	mov    0x50(%rsp),%rdx
  400c58:	mov    0x38(%rsp),%rcx
  400c5d:	mov    0x10(%rsp),%r8
  400c62:	mov    0x40(%rsp),%r9
  400c67:	lea    0xc8(%rsp),%rdi
  400c6f:	mov    %rbp,%rsi
  400c72:	push   0x28(%rsp)
  400c76:	lea    0x60(%rsp),%rax
  400c7b:	push   %rax
  400c7c:	mov    0x28(%rsp),%r15
  400c81:	push   %r15
  400c83:	vzeroupper
  400c86:	call   402a00 <<zopro_core::solver::Plan>::emit_poseidon>
  400c8b:	add    $0x20,%rsp
  400c8f:	vmovups 0xc0(%rsp),%ymm0
  400c98:	inc    %r15
  400c9b:	inc    %r14
  400c9e:	mov    %r15,0x10(%rsp)
  400ca3:	vmovups %ymm0,0x240(%rsp,%rbx,1)
  400cac:	add    $0x20,%rbx
  400cb0:	cmp    %rbx,0x88(%rsp)
  400cb8:	jne    400ba0 <<zopro_core::solver::Plan>::run_poseidon_task+0x22e0>
  400cbe:	mov    0x8(%rsp),%rcx
  400cc3:	mov    0x1c0(%rsp),%r14
  400ccb:	mov    0x1f0(%rsp),%r15
  400cd3:	mov    0x1f8(%rsp),%rdx
  400cdb:	lea    0x0(,%rcx,8),%rax
  400ce3:	sub    %rcx,%rax
  400ce6:	mov    0x38(%rsp),%rcx
  400ceb:	add    %rax,%rcx
  400cee:	lea    (%rcx,%rcx,2),%rax
  400cf2:	add    %rax,%r14
  400cf5:	cmp    0xa0(%rsp),%r14
  400cfd:	jb     3fe938 <<zopro_core::solver::Plan>::run_poseidon_task+0x78>
  400d03:	movq   $0xffffffffffffffff,(%r15)
  400d0a:	jmp    400de2 <<zopro_core::solver::Plan>::run_poseidon_task+0x2522>
  400d0f:	shlq   $0x2,0xa0(%rsp)
  400d18:	lea    0x160(%rsp),%rbx
  400d20:	lea    0x240(%rsp),%r14
  400d28:	mov    %r15,%r12
  400d2b:	mov    %rdx,%rbp
  400d2e:	xor    %r15d,%r15d
  400d31:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  400d40:	mov    0x80(%rsp),%rax
  400d48:	mov    0x0(%rbp,%r15,1),%edi
  400d4d:	mov    0x90(%rax),%rsi
  400d54:	cmp    %rdi,%rsi
  400d57:	jbe    400f83 <<zopro_core::solver::Plan>::run_poseidon_task+0x26c3>
  400d5d:	mov    0x20(%rsp),%rcx
  400d62:	lea    (%rdi,%rdi,8),%rdx
  400d66:	mov    %rbx,%rdi
  400d69:	mov    %rax,%rsi
  400d6c:	shl    $0x3,%rdx
  400d70:	add    0x88(%rax),%rdx
  400d77:	vmovups (%rcx),%ymm0
  400d7b:	mov    %r14,%rcx
  400d7e:	vmovups %ymm0,0x240(%rsp)
  400d87:	vzeroupper
  400d8a:	call   3fa330 <<zopro_core::solver::Plan>::run>
  400d8f:	cmpq   $0xffffffffffffffff,0x160(%rsp)
  400d98:	jne    400dc6 <<zopro_core::solver::Plan>::run_poseidon_task+0x2506>
  400d9a:	add    $0x4,%r15
  400d9e:	cmp    %r15,0xa0(%rsp)
  400da6:	jne    400d40 <<zopro_core::solver::Plan>::run_poseidon_task+0x2480>
  400da8:	movq   $0xffffffffffffffff,(%r12)
  400db0:	jmp    400de2 <<zopro_core::solver::Plan>::run_poseidon_task+0x2522>
  400db2:	vmovups 0x168(%rsp),%xmm0
  400dbb:	vmovups %xmm0,0x8(%r15)
  400dc1:	mov    %rax,(%r15)
  400dc4:	jmp    400de2 <<zopro_core::solver::Plan>::run_poseidon_task+0x2522>
  400dc6:	vmovups 0x160(%rsp),%xmm0
  400dcf:	mov    0x170(%rsp),%rax
  400dd7:	mov    %rax,0x10(%r12)
  400ddc:	vmovups %xmm0,(%r12)
  400de2:	add    $0x3e8,%rsp
  400de9:	pop    %rbx
  400dea:	pop    %r12
  400dec:	pop    %r13
  400dee:	pop    %r14
  400df0:	pop    %r15
  400df2:	pop    %rbp
  400df3:	vzeroupper
  400df6:	ret
  400df7:	vmovups 0x160(%rsp),%xmm0
  400e00:	mov    0x170(%rsp),%rax
  400e08:	mov    %rax,0x10(%r15)
  400e0c:	vmovups %xmm0,(%r15)
  400e11:	jmp    400de2 <<zopro_core::solver::Plan>::run_poseidon_task+0x2522>
  400e13:	lea    0xe3ab6(%rip),%rdi        # 4e48d0 <__frame_dummy_init_array_entry+0x6930>
  400e1a:	vzeroupper
  400e1d:	call   f16f0 <core::option::unwrap_failed>
  400e22:	lea    0xe3a8f(%rip),%rdi        # 4e48b8 <__frame_dummy_init_array_entry+0x6918>
  400e29:	vzeroupper
  400e2c:	call   f16f0 <core::option::unwrap_failed>
  400e31:	mov    0x18(%rsp),%rsi
  400e36:	mov    0x30(%rsp),%rdx
  400e3b:	lea    0xe3abe(%rip),%rcx        # 4e4900 <__frame_dummy_init_array_entry+0x6960>
  400e42:	xor    %edi,%edi
  400e44:	vzeroupper
  400e47:	call   f0300 <core::slice::index::slice_index_fail>
  400e4c:	mov    0x8(%rsp),%rsi
  400e51:	lea    0xe3a90(%rip),%rcx        # 4e48e8 <__frame_dummy_init_array_entry+0x6948>
  400e58:	mov    $0xd,%edx
  400e5d:	xor    %edi,%edi
  400e5f:	vzeroupper
  400e62:	call   f0300 <core::slice::index::slice_index_fail>
  400e67:	lea    0xe39ea(%rip),%rdx        # 4e4858 <__frame_dummy_init_array_entry+0x68b8>
  400e6e:	mov    %rbx,%rsi
  400e71:	vzeroupper
  400e74:	call   ef3ab <core::panicking::panic_bounds_check>
  400e79:	lea    0xe39f0(%rip),%rdx        # 4e4870 <__frame_dummy_init_array_entry+0x68d0>
  400e80:	mov    %rbx,%rsi
  400e83:	vzeroupper
  400e86:	call   ef3ab <core::panicking::panic_bounds_check>
  400e8b:	lea    -0x3e05e9(%rip),%rdi        # 208a9 <zmij::STATIC_DATA+0x64a9>
  400e92:	lea    0xe49f7(%rip),%rdx        # 4e5890 <__frame_dummy_init_array_entry+0x78f0>
  400e99:	mov    $0x85,%esi
  400e9e:	vzeroupper
  400ea1:	call   ee300 <core::panicking::panic_fmt>
  400ea6:	lea    0xe391b(%rip),%rdx        # 4e47c8 <__frame_dummy_init_array_entry+0x6828>
  400ead:	vzeroupper
  400eb0:	call   ef3ab <core::panicking::panic_bounds_check>
  400eb5:	lea    0xe2e44(%rip),%rdx        # 4e3d00 <__frame_dummy_init_array_entry+0x5d60>
  400ebc:	mov    %rax,%rdi
  400ebf:	vzeroupper
  400ec2:	call   ef3ab <core::panicking::panic_bounds_check>
  400ec7:	mov    0x30(%rsp),%rsi
  400ecc:	lea    0xe39cd(%rip),%rdx        # 4e48a0 <__frame_dummy_init_array_entry+0x6900>
  400ed3:	vzeroupper
  400ed6:	call   ef3ab <core::panicking::panic_bounds_check>
  400edb:	mov    %r12,%r14
  400ede:	lea    0xe39a3(%rip),%rdx        # 4e4888 <__frame_dummy_init_array_entry+0x68e8>
  400ee5:	mov    0x40(%rsp),%rsi
  400eea:	mov    %r14,%rdi
  400eed:	vzeroupper
  400ef0:	call   ef3ab <core::panicking::panic_bounds_check>
  400ef5:	lea    0xe3944(%rip),%rdx        # 4e4840 <__frame_dummy_init_array_entry+0x68a0>
  400efc:	mov    0x40(%rsp),%rsi
  400f01:	mov    %r14,%rdi
  400f04:	vzeroupper
  400f07:	call   ef3ab <core::panicking::panic_bounds_check>
  400f0c:	mov    0xb0(%rsp),%rdi
  400f14:	sub    %rbx,%rdi
  400f17:	mov    0x40(%rsp),%rsi
  400f1c:	lea    0xe38bd(%rip),%rdx        # 4e47e0 <__frame_dummy_init_array_entry+0x6840>
  400f23:	vzeroupper
  400f26:	call   ef3ab <core::panicking::panic_bounds_check>
  400f2b:	mov    0x1e0(%rsp),%rdi
  400f33:	shr    $0x1c,%edi
  400f36:	add    0xb0(%rsp),%rdi
  400f3e:	sub    %r15,%rdi
  400f41:	mov    0x40(%rsp),%rsi
  400f46:	lea    0xe3893(%rip),%rdx        # 4e47e0 <__frame_dummy_init_array_entry+0x6840>
  400f4d:	vzeroupper
  400f50:	call   ef3ab <core::panicking::panic_bounds_check>
  400f55:	mov    %r15,%rdi
  400f58:	mov    0x40(%rsp),%rsi
  400f5d:	lea    0xe387c(%rip),%rdx        # 4e47e0 <__frame_dummy_init_array_entry+0x6840>
  400f64:	vzeroupper
  400f67:	call   ef3ab <core::panicking::panic_bounds_check>
  400f6c:	mov    %r14,%rdi
  400f6f:	mov    0x40(%rsp),%rsi
  400f74:	lea    0xe3865(%rip),%rdx        # 4e47e0 <__frame_dummy_init_array_entry+0x6840>
  400f7b:	vzeroupper
  400f7e:	call   ef3ab <core::panicking::panic_bounds_check>
  400f83:	lea    0xe3a7e(%rip),%rdx        # 4e4a08 <__frame_dummy_init_array_entry+0x6a68>
  400f8a:	call   ef3ab <core::panicking::panic_bounds_check>
  400f8f:	lea    0xe3862(%rip),%rdx        # 4e47f8 <__frame_dummy_init_array_entry+0x6858>
  400f96:	mov    %rbx,%rsi
  400f99:	vzeroupper
  400f9c:	call   ef3ab <core::panicking::panic_bounds_check>
  400fa1:	lea    0xe3880(%rip),%rdx        # 4e4828 <__frame_dummy_init_array_entry+0x6888>
  400fa8:	mov    %rbx,%rsi
  400fab:	vzeroupper
  400fae:	call   ef3ab <core::panicking::panic_bounds_check>
  400fb3:	lea    0xe3856(%rip),%rdx        # 4e4810 <__frame_dummy_init_array_entry+0x6870>
  400fba:	mov    %rax,%rdi
  400fbd:	mov    %rbx,%rsi
  400fc0:	vzeroupper
  400fc3:	call   ef3ab <core::panicking::panic_bounds_check>
  400fc8:	lea    0xe3a51(%rip),%rdx        # 4e4a20 <__frame_dummy_init_array_entry+0x6a80>
  400fcf:	mov    %r12,%rdi
  400fd2:	vzeroupper
  400fd5:	call   ef3ab <core::panicking::panic_bounds_check>
