
/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000003fe8d0 <<zopro_core::solver::Plan>::run_poseidon_task>:
  3fe8d0:	push   %rbp
  3fe8d1:	push   %r15
  3fe8d3:	push   %r14
  3fe8d5:	push   %r13
  3fe8d7:	push   %r12
  3fe8d9:	push   %rbx
  3fe8da:	sub    $0x3e8,%rsp
  3fe8e1:	cmpq   $0x7ffffff,0x90(%rsi)
  3fe8ec:	mov    %rdi,%r15
  3fe8ef:	mov    %r8,0x20(%rsp)
  3fe8f4:	mov    %rcx,0xa0(%rsp)
  3fe8fc:	mov    %rsi,0x80(%rsp)
  3fe904:	ja     400d1f <<zopro_core::solver::Plan>::run_poseidon_task+0x244f>
  3fe90a:	mov    $0x8000000,%eax
  3fe90f:	and    (%rdx),%eax
  3fe911:	je     400d1f <<zopro_core::solver::Plan>::run_poseidon_task+0x244f>
  3fe917:	mov    0x20(%rsp),%rax
  3fe91c:	mov    0xa0(%rsp),%rcx
  3fe924:	mov    %rsi,%rbp
  3fe927:	xor    %r14d,%r14d
  3fe92a:	mov    %rdx,0x1f8(%rsp)
  3fe932:	mov    (%rax),%rbx
  3fe935:	neg    %rcx
  3fe938:	mov    %rcx,0x200(%rsp)
  3fe940:	mov    %rbx,0x208(%rsp)
  3fe948:	mov    (%rdx,%r14,4),%r12d
  3fe94c:	mov    %r12d,%edi
  3fe94f:	shr    $0x1c,%edi
  3fe952:	cmp    $0xd,%edi
  3fe955:	ja     400e9b <<zopro_core::solver::Plan>::run_poseidon_task+0x25cb>
  3fe95b:	lea    -0x3eadaa(%rip),%rcx        # 13bb8 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x2138>
  3fe962:	mov    $0x38,%esi
  3fe967:	mov    %rdi,0x8(%rsp)
  3fe96c:	movslq (%rcx,%rdi,4),%rax
  3fe970:	add    %rcx,%rax
  3fe973:	jmp    *%rax
  3fe975:	mov    0x90(%rbp),%rsi
  3fe97c:	and    $0x7ffffff,%r12d
  3fe983:	cmp    %r12,%rsi
  3fe986:	jbe    400fd8 <<zopro_core::solver::Plan>::run_poseidon_task+0x2708>
  3fe98c:	mov    0x20(%rsp),%rax
  3fe991:	mov    %rdx,%r13
  3fe994:	lea    (%r12,%r12,8),%rdx
  3fe998:	lea    0x160(%rsp),%rdi
  3fe9a0:	lea    0x240(%rsp),%rcx
  3fe9a8:	mov    %rbp,%rsi
  3fe9ab:	shl    $0x3,%rdx
  3fe9af:	add    0x88(%rbp),%rdx
  3fe9b6:	vmovups (%rax),%ymm0
  3fe9ba:	vmovups %ymm0,0x240(%rsp)
  3fe9c3:	vzeroupper
  3fe9c6:	call   3fa340 <<zopro_core::solver::Plan>::run>
  3fe9cb:	cmpq   $0xffffffffffffffff,0x160(%rsp)
  3fe9d4:	jne    400e07 <<zopro_core::solver::Plan>::run_poseidon_task+0x2537>
  3fe9da:	inc    %r14
  3fe9dd:	mov    %r13,%rdx
  3fe9e0:	cmp    0xa0(%rsp),%r14
  3fe9e8:	jb     3fe948 <<zopro_core::solver::Plan>::run_poseidon_task+0x78>
  3fe9ee:	jmp    400d13 <<zopro_core::solver::Plan>::run_poseidon_task+0x2443>
  3fe9f3:	mov    $0x720,%r13d
  3fe9f9:	mov    $0x3c,%esi
  3fe9fe:	lea    -0x393695(%rip),%rax        # 6b370 <zopro_core::solver::sparse_poseidon::constants::T12>
  3fea05:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea0a:	mov    $0x4c0,%r13d
  3fea10:	mov    $0x40,%esi
  3fea15:	lea    -0x3c116c(%rip),%rax        # 3d8b0 <zopro_core::solver::sparse_poseidon::constants::T8>
  3fea1c:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea21:	mov    $0xf8,%r13d
  3fea27:	lea    -0x3de13e(%rip),%rax        # 208f0 <zopro_core::solver::sparse_poseidon::constants::T2>
  3fea2e:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea33:	mov    $0x180,%r13d
  3fea39:	mov    $0x39,%esi
  3fea3e:	lea    -0x3dc255(%rip),%rax        # 227f0 <zopro_core::solver::sparse_poseidon::constants::T3>
  3fea45:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea4a:	mov    $0x558,%r13d
  3fea50:	mov    $0x3f,%esi
  3fea55:	lea    -0x3b79ac(%rip),%rax        # 470b0 <zopro_core::solver::sparse_poseidon::constants::T9>
  3fea5c:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea61:	mov    $0x5c8,%r13d
  3fea67:	mov    $0x3c,%esi
  3fea6c:	lea    -0x3acec3(%rip),%rax        # 51bb0 <zopro_core::solver::sparse_poseidon::constants::T10>
  3fea73:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea75:	mov    $0x40c,%r13d
  3fea7b:	mov    $0x3f,%esi
  3fea80:	lea    -0x3c9357(%rip),%rax        # 35730 <zopro_core::solver::sparse_poseidon::constants::T7>
  3fea87:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea89:	mov    $0x854,%r13d
  3fea8f:	mov    $0x41,%esi
  3fea94:	lea    -0x38532b(%rip),%rax        # 79770 <zopro_core::solver::sparse_poseidon::constants::T13>
  3fea9b:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fea9d:	mov    $0x200,%r13d
  3feaa3:	lea    -0x3d92ba(%rip),%rax        # 257f0 <zopro_core::solver::sparse_poseidon::constants::T4>
  3feaaa:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3feaac:	mov    $0x6f6,%r13d
  3feab2:	mov    $0x42,%esi
  3feab7:	lea    -0x3a160e(%rip),%rax        # 5d4b0 <zopro_core::solver::sparse_poseidon::constants::T11>
  3feabe:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3feac0:	mov    $0x348,%r13d
  3feac6:	mov    $0x3c,%esi
  3feacb:	lea    -0x3cfca2(%rip),%rax        # 2ee30 <zopro_core::solver::sparse_poseidon::constants::T6>
  3fead2:	jmp    3feaf0 <<zopro_core::solver::Plan>::run_poseidon_task+0x220>
  3fead4:	mov    $0x2b2,%r13d
  3feada:	mov    $0x3c,%esi
  3feadf:	lea    -0x3d52f6(%rip),%rax        # 297f0 <zopro_core::solver::sparse_poseidon::constants::T5>
  3feae6:	cs nopw 0x0(%rax,%rax,1)
  3feaf0:	mov    %rax,(%rsp)
  3feaf4:	mov    0x8(%rsp),%rax
  3feaf9:	mov    0xa0(%rsp),%rcx
  3feb01:	lea    (%rax,%rax,2),%rdi
  3feb05:	sub    %r14,%rcx
  3feb08:	mov    %rcx,0x30(%rsp)
  3feb0d:	mov    %rdi,0x18(%rsp)
  3feb12:	cmp    %rcx,%rdi
  3feb15:	ja     400e41 <<zopro_core::solver::Plan>::run_poseidon_task+0x2571>
  3feb1b:	lea    0x0(,%rax,4),%eax
  3feb22:	lea    (%rdx,%r14,4),%rcx
  3feb26:	mov    %r14,0x1c0(%rsp)
  3feb2e:	mov    %rsi,0x38(%rsp)
  3feb33:	xor    %r14d,%r14d
  3feb36:	lea    (%rax,%rax,2),%rbx
  3feb3a:	mov    %rcx,0x48(%rsp)
  3feb3f:	nop
  3feb40:	mov    0x48(%rsp),%rax
  3feb45:	mov    0x90(%rbp),%rsi
  3feb4c:	mov    (%rax,%r14,1),%edi
  3feb50:	and    $0x7ffffff,%edi
  3feb56:	cmp    %rdi,%rsi
  3feb59:	jbe    400eb6 <<zopro_core::solver::Plan>::run_poseidon_task+0x25e6>
  3feb5f:	mov    0x20(%rsp),%rax
  3feb64:	lea    (%rdi,%rdi,8),%rdx
  3feb68:	lea    0x160(%rsp),%rdi
  3feb70:	lea    0x240(%rsp),%rcx
  3feb78:	mov    %rbp,%rsi
  3feb7b:	shl    $0x3,%rdx
  3feb7f:	add    0x88(%rbp),%rdx
  3feb86:	vmovups (%rax),%ymm0
  3feb8a:	vmovups %ymm0,0x240(%rsp)
  3feb93:	vzeroupper
  3feb96:	call   3fa340 <<zopro_core::solver::Plan>::run>
  3feb9b:	mov    0x160(%rsp),%rax
  3feba3:	cmp    $0xffffffffffffffff,%rax
  3feba7:	jne    400dc2 <<zopro_core::solver::Plan>::run_poseidon_task+0x24f2>
  3febad:	add    $0x4,%r14
  3febb1:	cmp    %r14,%rbx
  3febb4:	jne    3feb40 <<zopro_core::solver::Plan>::run_poseidon_task+0x270>
  3febb6:	vxorps %xmm0,%xmm0,%xmm0
  3febba:	vmovups %zmm0,0x3a0(%rsp)
  3febc5:	vmovups %zmm0,0x380(%rsp)
  3febcd:	vmovups %zmm0,0x340(%rsp)
  3febd5:	vmovups %zmm0,0x300(%rsp)
  3febdd:	vmovups %zmm0,0x2c0(%rsp)
  3febe5:	vmovups %zmm0,0x280(%rsp)
  3febed:	vmovups %zmm0,0x240(%rsp)
  3febf5:	cmp    $0xe0000000,%r12d
  3febfc:	jae    400e5c <<zopro_core::solver::Plan>::run_poseidon_task+0x258c>
  3fec02:	mov    0x8(%rsp),%rax
  3fec07:	mov    0x38(%rsp),%rcx
  3fec0c:	mov    0x208(%rsp),%r9
  3fec14:	mov    %rax,%rdx
  3fec17:	imul   %rax,%rdx
  3fec1b:	lea    (%rcx,%rax,8),%rcx
  3fec1f:	lea    (%rax,%rax,1),%edi
  3fec22:	mov    %rdi,0xa8(%rsp)
  3fec2a:	mov    %rcx,0x98(%rsp)
  3fec32:	lea    (%rcx,%rdx,1),%r10
  3fec36:	mov    %eax,%ecx
  3fec38:	shl    $0x5,%ecx
  3fec3b:	mov    0x200(%rsp),%rax
  3fec43:	mov    %rcx,0x88(%rsp)
  3fec4b:	mov    0x1c0(%rsp),%rcx
  3fec53:	add    %r10,%rdx
  3fec56:	mov    %rdx,0x220(%rsp)
  3fec5e:	add    %rcx,%rax
  3fec61:	mov    0x30(%rsp),%rcx
  3fec66:	cmp    %rcx,%rdi
  3fec69:	cmovb  %rdi,%rcx
  3fec6d:	xor    %edx,%edx
  3fec6f:	add    %rax,%rcx
  3fec72:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  3fec80:	test   %rcx,%rcx
  3fec83:	je     400ed7 <<zopro_core::solver::Plan>::run_poseidon_task+0x2607>
  3fec89:	mov    0x48(%rsp),%rax
  3fec8e:	mov    0x90(%rbp),%rsi
  3fec95:	mov    (%rax,%rdi,4),%eax
  3fec98:	and    $0x7ffffff,%eax
  3fec9d:	cmp    %rax,%rsi
  3feca0:	jbe    400ec5 <<zopro_core::solver::Plan>::run_poseidon_task+0x25f5>
  3feca6:	mov    0x88(%rbp),%rsi
  3fecad:	lea    (%rax,%rax,8),%rax
  3fecb1:	mov    (%rsi,%rax,8),%r8d
  3fecb5:	cmp    $0x2,%r8d
  3fecb9:	je     400e23 <<zopro_core::solver::Plan>::run_poseidon_task+0x2553>
  3fecbf:	test   %r8d,%r8d
  3fecc2:	je     400e32 <<zopro_core::solver::Plan>::run_poseidon_task+0x2562>
  3fecc8:	lea    (%rsi,%rax,8),%rax
  3feccc:	lea    0x240(%rsp,%rdx,1),%r8
  3fecd4:	add    $0x20,%rdx
  3fecd8:	inc    %rcx
  3fecdb:	inc    %rdi
  3fecde:	mov    0x4(%rax),%eax
  3fece1:	shl    $0x5,%rax
  3fece5:	vmovups (%r9,%rax,1),%ymm0
  3feceb:	vmovups %ymm0,(%r8)
  3fecf0:	cmp    %rdx,0x88(%rsp)
  3fecf8:	jne    3fec80 <<zopro_core::solver::Plan>::run_poseidon_task+0x3b0>
  3fecfa:	mov    %r12,0x1e0(%rsp)
  3fed02:	mov    0x8(%rsp),%r12
  3fed07:	mov    (%rsp),%r14
  3fed0b:	mov    0x98(%rsp),%r8
  3fed13:	lea    0x240(%rsp),%rdi
  3fed1b:	mov    %r10,0x28(%rsp)
  3fed20:	mov    %r15,0x1f0(%rsp)
  3fed28:	mov    %r13,%rcx
  3fed2b:	mov    %r12,%rsi
  3fed2e:	mov    %r14,%rdx
  3fed31:	vzeroupper
  3fed34:	call   402010 <<zopro_core::solver::Plan>::run_poseidon::dense>
  3fed39:	mov    %r12d,%ebx
  3fed3c:	shl    $0x6,%ebx
  3fed3f:	mov    %r12,%rax
  3fed42:	mov    %r12,%r15
  3fed45:	xor    %r12d,%r12d
  3fed48:	mov    %rax,0x10(%rsp)
  3fed4d:	mov    %r13,0x40(%rsp)
  3fed52:	add    %r14,%rbx
  3fed55:	mov    0xa8(%rsp),%r14
  3fed5d:	nopl   (%rax)
  3fed60:	cmp    %r14,%r13
  3fed63:	je     400eee <<zopro_core::solver::Plan>::run_poseidon_task+0x261e>
  3fed69:	vmovups (%rbx,%r12,1),%ymm0
  3fed6f:	vmovups -0x3ed5d7(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3fed77:	lea    0x100(%rsp),%r13
  3fed7f:	mov    0x258(%rsp,%r12,1),%rax
  3fed87:	mov    0x250(%rsp,%r12,1),%rcx
  3fed8f:	mov    0x240(%rsp,%r12,1),%rdx
  3fed97:	mov    0x248(%rsp,%r12,1),%rsi
  3fed9f:	lea    0x160(%rsp),%r11
  3feda7:	vmovups %ymm0,0x100(%rsp)
  3fedb0:	vmovups %ymm1,0x160(%rsp)
  3fedb9:	add    0x0(%r13),%rdx
  3fedbd:	adc    0x8(%r13),%rsi
  3fedc1:	adc    0x10(%r13),%rcx
  3fedc5:	adc    0x18(%r13),%rax
  3fedc9:	mov    %rdx,%rdi
  3fedcc:	mov    %rsi,%r8
  3fedcf:	mov    %rcx,%r9
  3fedd2:	mov    %rax,%r10
  3fedd5:	sub    (%r11),%rdi
  3fedd8:	sbb    0x8(%r11),%r8
  3feddc:	sbb    0x10(%r11),%r9
  3fede0:	sbb    0x18(%r11),%r10
  3fede4:	cmovae %rdi,%rdx
  3fede8:	cmovae %r8,%rsi
  3fedec:	cmovae %r9,%rcx
  3fedf0:	cmovae %r10,%rax
  3fedf4:	mov    %rdx,0x50(%rsp)
  3fedf9:	mov    %rsi,0x58(%rsp)
  3fedfe:	mov    %rcx,0x60(%rsp)
  3fee03:	mov    %rax,0x68(%rsp)
  3fee08:	sub    $0x8,%rsp
  3fee0c:	mov    0x50(%rsp),%rdx
  3fee11:	mov    0x38(%rsp),%rcx
  3fee16:	mov    0x10(%rsp),%r8
  3fee1b:	mov    0x40(%rsp),%r9
  3fee20:	lea    0xc8(%rsp),%rdi
  3fee28:	mov    %rbp,%rsi
  3fee2b:	push   0x28(%rsp)
  3fee2f:	lea    0x60(%rsp),%rax
  3fee34:	push   %rax
  3fee35:	mov    0x28(%rsp),%r13
  3fee3a:	push   %r13
  3fee3c:	vzeroupper
  3fee3f:	call   403080 <<zopro_core::solver::Plan>::emit_poseidon>
  3fee44:	add    $0x20,%rsp
  3fee48:	vmovups 0xc0(%rsp),%ymm0
  3fee51:	inc    %r13
  3fee54:	inc    %r14
  3fee57:	mov    %r13,0x10(%rsp)
  3fee5c:	mov    0x40(%rsp),%r13
  3fee61:	vmovups %ymm0,0x240(%rsp,%r12,1)
  3fee6b:	add    $0x20,%r12
  3fee6f:	dec    %r15
  3fee72:	jne    3fed60 <<zopro_core::solver::Plan>::run_poseidon_task+0x490>
  3fee78:	mov    0x8(%rsp),%rbx
  3fee7d:	mov    (%rsp),%r14
  3fee81:	mov    0x98(%rsp),%r8
  3fee89:	lea    0x240(%rsp),%rdi
  3fee91:	mov    %r13,%rcx
  3fee94:	mov    %rbx,%rsi
  3fee97:	mov    %r14,%rdx
  3fee9a:	vzeroupper
  3fee9d:	call   402010 <<zopro_core::solver::Plan>::run_poseidon::dense>
  3feea2:	mov    0x18(%rsp),%r12
  3feea7:	lea    (%rbx,%rbx,2),%rbx
  3feeab:	mov    %rbx,0x1e8(%rsp)
  3feeb3:	shl    $0x5,%ebx
  3feeb6:	add    %r14,%rbx
  3feeb9:	xor    %r14d,%r14d
  3feebc:	nopl   0x0(%rax)
  3feec0:	cmp    %r12,%r13
  3feec3:	je     400eeb <<zopro_core::solver::Plan>::run_poseidon_task+0x261b>
  3feec9:	vmovups (%rbx,%r14,1),%ymm0
  3feecf:	vmovups -0x3ed737(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3feed7:	mov    0x258(%rsp,%r14,1),%rax
  3feedf:	mov    0x250(%rsp,%r14,1),%rcx
  3feee7:	mov    0x240(%rsp,%r14,1),%rdx
  3feeef:	mov    0x248(%rsp,%r14,1),%rsi
  3feef7:	lea    0x160(%rsp),%r11
  3feeff:	lea    0x100(%rsp),%r15
  3fef07:	vmovups %ymm0,0x100(%rsp)
  3fef10:	vmovups %ymm1,0x160(%rsp)
  3fef19:	add    (%r15),%rdx
  3fef1c:	adc    0x8(%r15),%rsi
  3fef20:	adc    0x10(%r15),%rcx
  3fef24:	adc    0x18(%r15),%rax
  3fef28:	mov    %rdx,%rdi
  3fef2b:	mov    %rsi,%r8
  3fef2e:	mov    %rcx,%r9
  3fef31:	mov    %rax,%r10
  3fef34:	sub    (%r11),%rdi
  3fef37:	sbb    0x8(%r11),%r8
  3fef3b:	sbb    0x10(%r11),%r9
  3fef3f:	sbb    0x18(%r11),%r10
  3fef43:	cmovae %rdi,%rdx
  3fef47:	cmovae %r8,%rsi
  3fef4b:	cmovae %r9,%rcx
  3fef4f:	cmovae %r10,%rax
  3fef53:	mov    %rdx,0x50(%rsp)
  3fef58:	mov    %rsi,0x58(%rsp)
  3fef5d:	mov    %rcx,0x60(%rsp)
  3fef62:	mov    %rax,0x68(%rsp)
  3fef67:	sub    $0x8,%rsp
  3fef6b:	mov    0x50(%rsp),%rdx
  3fef70:	mov    0x38(%rsp),%rcx
  3fef75:	mov    0x10(%rsp),%r8
  3fef7a:	mov    0x40(%rsp),%r9
  3fef7f:	lea    0xc8(%rsp),%rdi
  3fef87:	mov    %rbp,%rsi
  3fef8a:	push   0x28(%rsp)
  3fef8e:	lea    0x60(%rsp),%rax
  3fef93:	push   %rax
  3fef94:	mov    0x28(%rsp),%r15
  3fef99:	push   %r15
  3fef9b:	vzeroupper
  3fef9e:	call   403080 <<zopro_core::solver::Plan>::emit_poseidon>
  3fefa3:	add    $0x20,%rsp
  3fefa7:	vmovups 0xc0(%rsp),%ymm0
  3fefb0:	inc    %r15
  3fefb3:	inc    %r12
  3fefb6:	mov    %r15,0x10(%rsp)
  3fefbb:	vmovups %ymm0,0x240(%rsp,%r14,1)
  3fefc5:	add    $0x20,%r14
  3fefc9:	cmp    %r14,0x88(%rsp)
  3fefd1:	jne    3feec0 <<zopro_core::solver::Plan>::run_poseidon_task+0x5f0>
  3fefd7:	mov    0x8(%rsp),%r14
  3fefdc:	mov    (%rsp),%r15
  3fefe0:	mov    0x28(%rsp),%r8
  3fefe5:	lea    0x240(%rsp),%rdi
  3fefed:	mov    %r13,%rcx
  3feff0:	mov    %r14,%rsi
  3feff3:	mov    %r15,%rdx
  3feff6:	vzeroupper
  3feff9:	call   402010 <<zopro_core::solver::Plan>::run_poseidon::dense>
  3feffe:	mov    %r14d,%edi
  3ff001:	shl    $0x7,%edi
  3ff004:	vmovups -0x3ed86c(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3ff00c:	mov    0x258(%rsp),%rax
  3ff014:	mov    0x250(%rsp),%rcx
  3ff01c:	mov    0x240(%rsp),%rdx
  3ff024:	mov    0x248(%rsp),%rsi
  3ff02c:	lea    0x160(%rsp),%r11
  3ff034:	lea    0x100(%rsp),%rbx
  3ff03c:	vmovups (%r15,%rdi,1),%ymm0
  3ff042:	vmovups %ymm1,0x160(%rsp)
  3ff04b:	vmovups %ymm0,0x100(%rsp)
  3ff054:	add    (%rbx),%rdx
  3ff057:	adc    0x8(%rbx),%rsi
  3ff05b:	adc    0x10(%rbx),%rcx
  3ff05f:	adc    0x18(%rbx),%rax
  3ff063:	mov    %rdx,%rdi
  3ff066:	mov    %rsi,%r8
  3ff069:	mov    %rcx,%r9
  3ff06c:	mov    %rax,%r10
  3ff06f:	sub    (%r11),%rdi
  3ff072:	sbb    0x8(%r11),%r8
  3ff076:	sbb    0x10(%r11),%r9
  3ff07a:	sbb    0x18(%r11),%r10
  3ff07e:	cmovae %rdi,%rdx
  3ff082:	cmovae %r8,%rsi
  3ff086:	cmovae %r9,%rcx
  3ff08a:	cmovae %r10,%rax
  3ff08e:	mov    %rdx,0xe0(%rsp)
  3ff096:	mov    %rsi,0xe8(%rsp)
  3ff09e:	mov    %rcx,0xf0(%rsp)
  3ff0a6:	mov    %rax,0xf8(%rsp)
  3ff0ae:	mov    %r14,%rax
  3ff0b1:	lea    0x1(,%r14,4),%rdi
  3ff0b9:	mov    0xa8(%rsp),%rdx
  3ff0c1:	mov    0x38(%rsp),%r9
  3ff0c6:	lea    0x8(%rdx),%rsi
  3ff0ca:	mov    %r9d,%ecx
  3ff0cd:	shl    $0x5,%ecx
  3ff0d0:	lea    -0x1(%rdx),%r8
  3ff0d4:	add    $0x9,%rdx
  3ff0d8:	imul   %rsi,%rax
  3ff0dc:	add    %r15,%rcx
  3ff0df:	mov    %r8,0x218(%rsp)
  3ff0e7:	lea    0x1(%r9,%rax,1),%rax
  3ff0ec:	mov    %rax,0x1c8(%rsp)
  3ff0f4:	mov    %r14d,%eax
  3ff0f7:	shl    $0x5,%eax
  3ff0fa:	imul   %rax,%rsi
  3ff0fe:	add    %rcx,%rsi
  3ff101:	mov    %rsi,0x1d8(%rsp)
  3ff109:	mov    %r14d,%esi
  3ff10c:	shl    $0x6,%esi
  3ff10f:	imul   %rdx,%r14
  3ff113:	imul   %rax,%rdx
  3ff117:	mov    %rsi,0xb8(%rsp)
  3ff11f:	add    $0xffffffffffffffe0,%rsi
  3ff123:	mov    %rsi,0x210(%rsp)
  3ff12b:	mov    0x88(%rsp),%rsi
  3ff133:	add    %r9,%r14
  3ff136:	add    %rcx,%rdx
  3ff139:	mov    %r14,0x1d0(%rsp)
  3ff141:	xor    %r14d,%r14d
  3ff144:	mov    %rdx,0xa8(%rsp)
  3ff14c:	lea    -0x20(%rsi),%rax
  3ff150:	mov    %rax,0x238(%rsp)
  3ff158:	nopl   0x0(%rax,%rax,1)
  3ff160:	lea    0x1(%r14),%r13
  3ff164:	cmp    %r9,%r13
  3ff167:	jae    3ff1a0 <<zopro_core::solver::Plan>::run_poseidon_task+0x8d0>
  3ff169:	mov    0x40(%rsp),%rbx
  3ff16e:	cmp    %rbx,%rdi
  3ff171:	jae    400f9f <<zopro_core::solver::Plan>::run_poseidon_task+0x26cf>
  3ff177:	mov    (%rsp),%rcx
  3ff17b:	mov    %rdi,%rax
  3ff17e:	shl    $0x5,%rax
  3ff182:	inc    %rdi
  3ff185:	mov    (%rcx,%rax,1),%rbp
  3ff189:	mov    0x8(%rcx,%rax,1),%r15
  3ff18e:	mov    0x10(%rcx,%rax,1),%r10
  3ff193:	mov    0x18(%rcx,%rax,1),%r11
  3ff198:	jmp    3ff1b0 <<zopro_core::solver::Plan>::run_poseidon_task+0x8e0>
  3ff19a:	nopw   0x0(%rax,%rax,1)
  3ff1a0:	mov    0x40(%rsp),%rbx
  3ff1a5:	xor    %ebp,%ebp
  3ff1a7:	xor    %r15d,%r15d
  3ff1aa:	xor    %r10d,%r10d
  3ff1ad:	xor    %r11d,%r11d
  3ff1b0:	imul   %r8,%r14
  3ff1b4:	add    0x220(%rsp),%r14
  3ff1bc:	mov    0x8(%rsp),%r8
  3ff1c1:	mov    %rdi,0xb0(%rsp)
  3ff1c9:	cmp    $0x1,%r8d
  3ff1cd:	je     3ff820 <<zopro_core::solver::Plan>::run_poseidon_task+0xf50>
  3ff1d3:	mov    %r13,0x230(%rsp)
  3ff1db:	mov    %r14,0x228(%rsp)
  3ff1e3:	cmp    $0x3,%r8d
  3ff1e7:	jne    3ff880 <<zopro_core::solver::Plan>::run_poseidon_task+0xfb0>
  3ff1ed:	lea    0x1(%r14),%rax
  3ff1f1:	cmp    %rbx,%rax
  3ff1f4:	jae    400fc3 <<zopro_core::solver::Plan>::run_poseidon_task+0x26f3>
  3ff1fa:	lea    0x2(%r14),%rdi
  3ff1fe:	mov    %r11,0x28(%rsp)
  3ff203:	mov    %r10,0x18(%rsp)
  3ff208:	cmp    %rbx,%rdi
  3ff20b:	jae    400fb1 <<zopro_core::solver::Plan>::run_poseidon_task+0x26e1>
  3ff211:	mov    (%rsp),%rcx
  3ff215:	shl    $0x5,%rax
  3ff219:	shl    $0x5,%rdi
  3ff21d:	lea    0x160(%rsp),%r14
  3ff225:	lea    0x100(%rsp),%r12
  3ff22d:	vmovups (%rcx,%rax,1),%ymm0
  3ff232:	lea    0x260(%rsp),%rax
  3ff23a:	vmovups %ymm0,0x100(%rsp)
  3ff243:	vmovups (%rax),%ymm0
  3ff247:	vmovups %ymm0,0x160(%rsp)
  3ff250:	vmovups (%rcx,%rdi,1),%ymm0
  3ff255:	lea    0x120(%rsp),%rcx
  3ff25d:	vmovups %ymm0,(%rcx)
  3ff261:	vmovups 0x20(%rax),%ymm0
  3ff266:	lea    0x180(%rsp),%rax
  3ff26e:	vmovups %ymm1,0x20(%rcx)
  3ff273:	movabs $0xc2e1f593efffffff,%rcx
  3ff27d:	vmovups %ymm0,(%rax)
  3ff281:	mov    %rcx,0x1a0(%rsp)
  3ff289:	vxorps %xmm0,%xmm0,%xmm0
  3ff28d:	vmovups %xmm0,0x28(%rax)
  3ff292:	movq   $0x0,0x38(%rax)
  3ff29a:	xor    %edi,%edi
  3ff29c:	xor    %esi,%esi
  3ff29e:	xor    %eax,%eax
  3ff2a0:	xor    %ecx,%ecx
  3ff2a2:	xor    %r8d,%r8d
  3ff2a5:	mov    (%r14),%rdx
  3ff2a8:	xor    %r11d,%r11d
  3ff2ab:	mulx   (%r12),%r9,%r10
  3ff2b1:	adcx   %r9,%rdi
  3ff2b7:	adox   %r10,%rsi
  3ff2bd:	mulx   0x8(%r12),%r9,%r10
  3ff2c4:	adcx   %r9,%rsi
  3ff2ca:	adox   %r10,%rax
  3ff2d0:	mulx   0x10(%r12),%r9,%r10
  3ff2d7:	adcx   %r9,%rax
  3ff2dd:	adox   %r10,%rcx
  3ff2e3:	mulx   0x18(%r12),%r9,%r10
  3ff2ea:	adcx   %r9,%rcx
  3ff2f0:	adox   %r10,%r8
  3ff2f6:	adcx   %r11,%r8
  3ff2fc:	mov    0x20(%r14),%rdx
  3ff300:	xor    %r11d,%r11d
  3ff303:	mulx   0x20(%r12),%r9,%r10
  3ff30a:	adcx   %r9,%rdi
  3ff310:	adox   %r10,%rsi
  3ff316:	mulx   0x28(%r12),%r9,%r10
  3ff31d:	adcx   %r9,%rsi
  3ff323:	adox   %r10,%rax
  3ff329:	mulx   0x30(%r12),%r9,%r10
  3ff330:	adcx   %r9,%rax
  3ff336:	adox   %r10,%rcx
  3ff33c:	mulx   0x38(%r12),%r9,%r10
  3ff343:	adcx   %r9,%rcx
  3ff349:	adox   %r10,%r8
  3ff34f:	adcx   %r11,%r8
  3ff355:	mov    %rdi,%rdx
  3ff358:	imul   0x40(%r14),%rdx
  3ff35d:	xor    %r11d,%r11d
  3ff360:	mulx   0x40(%r12),%r9,%r10
  3ff367:	adcx   %r9,%rdi
  3ff36d:	adox   %r10,%rsi
  3ff373:	mulx   0x48(%r12),%r9,%r10
  3ff37a:	adcx   %r9,%rsi
  3ff380:	adox   %r10,%rax
  3ff386:	mulx   0x50(%r12),%r9,%r10
  3ff38d:	adcx   %r9,%rax
  3ff393:	adox   %r10,%rcx
  3ff399:	mulx   0x58(%r12),%r9,%r10
  3ff3a0:	adcx   %r9,%rcx
  3ff3a6:	adox   %r10,%r8
  3ff3ac:	adcx   %r11,%r8
  3ff3b2:	mov    0x8(%r14),%rdx
  3ff3b6:	xor    %r11d,%r11d
  3ff3b9:	mulx   (%r12),%r9,%r10
  3ff3bf:	adcx   %r9,%rsi
  3ff3c5:	adox   %r10,%rax
  3ff3cb:	mulx   0x8(%r12),%r9,%r10
  3ff3d2:	adcx   %r9,%rax
  3ff3d8:	adox   %r10,%rcx
  3ff3de:	mulx   0x10(%r12),%r9,%r10
  3ff3e5:	adcx   %r9,%rcx
  3ff3eb:	adox   %r10,%r8
  3ff3f1:	mulx   0x18(%r12),%r9,%r10
  3ff3f8:	adcx   %r9,%r8
  3ff3fe:	adox   %r10,%rdi
  3ff404:	adcx   %r11,%rdi
  3ff40a:	mov    0x28(%r14),%rdx
  3ff40e:	xor    %r11d,%r11d
  3ff411:	mulx   0x20(%r12),%r9,%r10
  3ff418:	adcx   %r9,%rsi
  3ff41e:	adox   %r10,%rax
  3ff424:	mulx   0x28(%r12),%r9,%r10
  3ff42b:	adcx   %r9,%rax
  3ff431:	adox   %r10,%rcx
  3ff437:	mulx   0x30(%r12),%r9,%r10
  3ff43e:	adcx   %r9,%rcx
  3ff444:	adox   %r10,%r8
  3ff44a:	mulx   0x38(%r12),%r9,%r10
  3ff451:	adcx   %r9,%r8
  3ff457:	adox   %r10,%rdi
  3ff45d:	adcx   %r11,%rdi
  3ff463:	mov    %rsi,%rdx
  3ff466:	imul   0x40(%r14),%rdx
  3ff46b:	xor    %r11d,%r11d
  3ff46e:	mulx   0x40(%r12),%r9,%r10
  3ff475:	adcx   %r9,%rsi
  3ff47b:	adox   %r10,%rax
  3ff481:	mulx   0x48(%r12),%r9,%r10
  3ff488:	adcx   %r9,%rax
  3ff48e:	adox   %r10,%rcx
  3ff494:	mulx   0x50(%r12),%r9,%r10
  3ff49b:	adcx   %r9,%rcx
  3ff4a1:	adox   %r10,%r8
  3ff4a7:	mulx   0x58(%r12),%r9,%r10
  3ff4ae:	adcx   %r9,%r8
  3ff4b4:	adox   %r10,%rdi
  3ff4ba:	adcx   %r11,%rdi
  3ff4c0:	mov    0x10(%r14),%rdx
  3ff4c4:	xor    %r11d,%r11d
  3ff4c7:	mulx   (%r12),%r9,%r10
  3ff4cd:	adcx   %r9,%rax
  3ff4d3:	adox   %r10,%rcx
  3ff4d9:	mulx   0x8(%r12),%r9,%r10
  3ff4e0:	adcx   %r9,%rcx
  3ff4e6:	adox   %r10,%r8
  3ff4ec:	mulx   0x10(%r12),%r9,%r10
  3ff4f3:	adcx   %r9,%r8
  3ff4f9:	adox   %r10,%rdi
  3ff4ff:	mulx   0x18(%r12),%r9,%r10
  3ff506:	adcx   %r9,%rdi
  3ff50c:	adox   %r10,%rsi
  3ff512:	adcx   %r11,%rsi
  3ff518:	mov    0x30(%r14),%rdx
  3ff51c:	xor    %r11d,%r11d
  3ff51f:	mulx   0x20(%r12),%r9,%r10
  3ff526:	adcx   %r9,%rax
  3ff52c:	adox   %r10,%rcx
  3ff532:	mulx   0x28(%r12),%r9,%r10
  3ff539:	adcx   %r9,%rcx
  3ff53f:	adox   %r10,%r8
  3ff545:	mulx   0x30(%r12),%r9,%r10
  3ff54c:	adcx   %r9,%r8
  3ff552:	adox   %r10,%rdi
  3ff558:	mulx   0x38(%r12),%r9,%r10
  3ff55f:	adcx   %r9,%rdi
  3ff565:	adox   %r10,%rsi
  3ff56b:	adcx   %r11,%rsi
  3ff571:	mov    %rax,%rdx
  3ff574:	imul   0x40(%r14),%rdx
  3ff579:	xor    %r11d,%r11d
  3ff57c:	mulx   0x40(%r12),%r9,%r10
  3ff583:	adcx   %r9,%rax
  3ff589:	adox   %r10,%rcx
  3ff58f:	mulx   0x48(%r12),%r9,%r10
  3ff596:	adcx   %r9,%rcx
  3ff59c:	adox   %r10,%r8
  3ff5a2:	mulx   0x50(%r12),%r9,%r10
  3ff5a9:	adcx   %r9,%r8
  3ff5af:	adox   %r10,%rdi
  3ff5b5:	mulx   0x58(%r12),%r9,%r10
  3ff5bc:	adcx   %r9,%rdi
  3ff5c2:	adox   %r10,%rsi
  3ff5c8:	adcx   %r11,%rsi
  3ff5ce:	mov    0x18(%r14),%rdx
  3ff5d2:	xor    %r11d,%r11d
  3ff5d5:	mulx   (%r12),%r9,%r10
  3ff5db:	adcx   %r9,%rcx
  3ff5e1:	adox   %r10,%r8
  3ff5e7:	mulx   0x8(%r12),%r9,%r10
  3ff5ee:	adcx   %r9,%r8
  3ff5f4:	adox   %r10,%rdi
  3ff5fa:	mulx   0x10(%r12),%r9,%r10
  3ff601:	adcx   %r9,%rdi
  3ff607:	adox   %r10,%rsi
  3ff60d:	mulx   0x18(%r12),%r9,%r10
  3ff614:	adcx   %r9,%rsi
  3ff61a:	adox   %r10,%rax
  3ff620:	adcx   %r11,%rax
  3ff626:	mov    0x38(%r14),%rdx
  3ff62a:	xor    %r11d,%r11d
  3ff62d:	mulx   0x20(%r12),%r9,%r10
  3ff634:	adcx   %r9,%rcx
  3ff63a:	adox   %r10,%r8
  3ff640:	mulx   0x28(%r12),%r9,%r10
  3ff647:	adcx   %r9,%r8
  3ff64d:	adox   %r10,%rdi
  3ff653:	mulx   0x30(%r12),%r9,%r10
  3ff65a:	adcx   %r9,%rdi
  3ff660:	adox   %r10,%rsi
  3ff666:	mulx   0x38(%r12),%r9,%r10
  3ff66d:	adcx   %r9,%rsi
  3ff673:	adox   %r10,%rax
  3ff679:	adcx   %r11,%rax
  3ff67f:	mov    %rcx,%rdx
  3ff682:	imul   0x40(%r14),%rdx
  3ff687:	xor    %r11d,%r11d
  3ff68a:	mulx   0x40(%r12),%r9,%r10
  3ff691:	adcx   %r9,%rcx
  3ff697:	adox   %r10,%r8
  3ff69d:	mulx   0x48(%r12),%r9,%r10
  3ff6a4:	adcx   %r9,%r8
  3ff6aa:	adox   %r10,%rdi
  3ff6b0:	mulx   0x50(%r12),%r9,%r10
  3ff6b7:	adcx   %r9,%rdi
  3ff6bd:	adox   %r10,%rsi
  3ff6c3:	mulx   0x58(%r12),%r9,%r10
  3ff6ca:	adcx   %r9,%rsi
  3ff6d0:	adox   %r10,%rax
  3ff6d6:	adcx   %r11,%rax
  3ff6dc:	movabs $0xd7cc17b786468f6f,%rcx
  3ff6e6:	movabs $0x43e1f593f0000001,%rdx
  3ff6f0:	movabs $0x2833e84879b97091,%r9
  3ff6fa:	movabs $0xb85045b68181585d,%r10
  3ff704:	movabs $0x30644e72e131a029,%r11
  3ff70e:	vmovups %ymm1,0x160(%rsp)
  3ff717:	add    %rdi,%rcx
  3ff71a:	cmp    %rdx,%r8
  3ff71d:	sbb    $0x0,%rcx
  3ff721:	setb   %dl
  3ff724:	cmp    %r9,%rdi
  3ff727:	setb   %r9b
  3ff72b:	or     %dl,%r9b
  3ff72e:	movabs $0x47afba497e7ea7a3,%rdx
  3ff738:	add    %rsi,%rdx
  3ff73b:	movzbl %r9b,%r9d
  3ff73f:	sub    %r9,%rdx
  3ff742:	setb   %r9b
  3ff746:	cmp    %r10,%rsi
  3ff749:	setb   %r10b
  3ff74d:	or     %r9b,%r10b
  3ff750:	movabs $0xcf9bb18d1ece5fd7,%r9
  3ff75a:	add    %rax,%r9
  3ff75d:	movzbl %r10b,%r10d
  3ff761:	sub    %r10,%r9
  3ff764:	setb   %r10b
  3ff768:	cmp    %r11,%rax
  3ff76b:	setb   %r11b
  3ff76f:	or     %r10b,%r11b
  3ff772:	movabs $0xbc1e0a6c0fffffff,%r10
  3ff77c:	lea    (%r8,%r10,1),%r10
  3ff780:	cmovne %rdi,%rcx
  3ff784:	cmovne %rsi,%rdx
  3ff788:	cmovne %rax,%r9
  3ff78c:	mov    0x18(%rsp),%rdi
  3ff791:	cmovne %r8,%r10
  3ff795:	mov    0x28(%rsp),%r8
  3ff79a:	mov    %r10,0x100(%rsp)
  3ff7a2:	mov    %rcx,0x108(%rsp)
  3ff7aa:	mov    %rdx,0x110(%rsp)
  3ff7b2:	mov    %r9,0x118(%rsp)
  3ff7ba:	add    (%r12),%rbp
  3ff7be:	adc    0x8(%r12),%r15
  3ff7c3:	adc    0x10(%r12),%rdi
  3ff7c8:	adc    0x18(%r12),%r8
  3ff7cd:	mov    %rbp,%rax
  3ff7d0:	mov    %r15,%rcx
  3ff7d3:	mov    %rdi,%rdx
  3ff7d6:	mov    %r8,%rsi
  3ff7d9:	sub    (%r14),%rax
  3ff7dc:	sbb    0x8(%r14),%rcx
  3ff7e0:	sbb    0x10(%r14),%rdx
  3ff7e4:	sbb    0x18(%r14),%rsi
  3ff7e8:	cmovae %rax,%rbp
  3ff7ec:	cmovae %rcx,%r15
  3ff7f0:	cmovae %rdx,%rdi
  3ff7f4:	cmovae %rsi,%r8
  3ff7f8:	mov    %r8,0x28(%rsp)
  3ff7fd:	mov    $0x3,%r8d
  3ff803:	mov    %rdi,0x18(%rsp)
  3ff808:	mov    %r15,0x78(%rsp)
  3ff80d:	mov    %rbp,0x90(%rsp)
  3ff815:	jmp    3ffd26 <<zopro_core::solver::Plan>::run_poseidon_task+0x1456>
  3ff81a:	nopw   0x0(%rax,%rax,1)
  3ff820:	mov    %r11,0x28(%rsp)
  3ff825:	mov    %r10,0x18(%rsp)
  3ff82a:	mov    %r15,0x78(%rsp)
  3ff82f:	sub    $0x8,%rsp
  3ff833:	mov    0x88(%rsp),%rsi
  3ff83b:	mov    0x50(%rsp),%rdx
  3ff840:	mov    0x38(%rsp),%rcx
  3ff845:	lea    0xc8(%rsp),%rdi
  3ff84d:	push   0x28(%rsp)
  3ff851:	lea    0xf0(%rsp),%rax
  3ff859:	push   %rax
  3ff85a:	push   0x28(%rsp)
  3ff85e:	vzeroupper
  3ff861:	call   403080 <<zopro_core::solver::Plan>::emit_poseidon>
  3ff866:	vmovups -0x3ee0ce(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3ff86e:	add    $0x20,%rsp
  3ff872:	jmp    40021b <<zopro_core::solver::Plan>::run_poseidon_task+0x194b>
  3ff877:	nopw   0x0(%rax,%rax,1)
  3ff880:	mov    0x1c8(%rsp),%rdi
  3ff888:	mov    $0x20,%eax
  3ff88d:	mov    %rbp,0x90(%rsp)
  3ff895:	data16 cs nopw 0x0(%rax,%rax,1)
  3ff8a0:	cmp    %rbx,%rdi
  3ff8a3:	jae    400e89 <<zopro_core::solver::Plan>::run_poseidon_task+0x25b9>
  3ff8a9:	mov    0x1d8(%rsp),%rcx
  3ff8b1:	vmovups 0x240(%rsp,%rax,1),%ymm2
  3ff8ba:	mov    %r15,0x78(%rsp)
  3ff8bf:	vmovups %ymm1,0x160(%rsp)
  3ff8c8:	mov    %r10,0x18(%rsp)
  3ff8cd:	mov    %r11,0x28(%rsp)
  3ff8d2:	lea    0x160(%rsp),%rbp
  3ff8da:	lea    0x100(%rsp),%r12
  3ff8e2:	lea    0x50(%rsp),%r15
  3ff8e7:	movabs $0xc2e1f593efffffff,%rbx
  3ff8f1:	vmovups (%rcx,%rax,1),%ymm0
  3ff8f6:	vmovups %ymm2,0x100(%rsp)
  3ff8ff:	vmovups %ymm0,0x50(%rsp)
  3ff905:	xor    %r8d,%r8d
  3ff908:	xor    %esi,%esi
  3ff90a:	xor    %ecx,%ecx
  3ff90c:	xor    %r10d,%r10d
  3ff90f:	xor    %r9d,%r9d
  3ff912:	mov    (%r12),%rdx
  3ff916:	xor    %r13d,%r13d
  3ff919:	mulx   (%r15),%r11,%r14
  3ff91e:	adcx   %r11,%r8
  3ff924:	adox   %r14,%rsi
  3ff92a:	mulx   0x8(%r15),%r11,%r14
  3ff930:	adcx   %r11,%rsi
  3ff936:	adox   %r14,%rcx
  3ff93c:	mulx   0x10(%r15),%r11,%r14
  3ff942:	adcx   %r11,%rcx
  3ff948:	adox   %r14,%r10
  3ff94e:	mulx   0x18(%r15),%r11,%r14
  3ff954:	adcx   %r11,%r10
  3ff95a:	adox   %r14,%r9
  3ff960:	adcx   %r13,%r9
  3ff966:	mov    %r8,%rdx
  3ff969:	imul   %rbx,%rdx
  3ff96d:	xor    %r13d,%r13d
  3ff970:	mulx   0x0(%rbp),%r11,%r14
  3ff976:	adcx   %r11,%r8
  3ff97c:	adox   %r14,%rsi
  3ff982:	mulx   0x8(%rbp),%r11,%r14
  3ff988:	adcx   %r11,%rsi
  3ff98e:	adox   %r14,%rcx
  3ff994:	mulx   0x10(%rbp),%r11,%r14
  3ff99a:	adcx   %r11,%rcx
  3ff9a0:	adox   %r14,%r10
  3ff9a6:	mulx   0x18(%rbp),%r11,%r14
  3ff9ac:	adcx   %r11,%r10
  3ff9b2:	adox   %r14,%r9
  3ff9b8:	adcx   %r13,%r9
  3ff9be:	mov    0x8(%r12),%rdx
  3ff9c3:	xor    %r13d,%r13d
  3ff9c6:	mulx   (%r15),%r11,%r14
  3ff9cb:	adcx   %r11,%rsi
  3ff9d1:	adox   %r14,%rcx
  3ff9d7:	mulx   0x8(%r15),%r11,%r14
  3ff9dd:	adcx   %r11,%rcx
  3ff9e3:	adox   %r14,%r10
  3ff9e9:	mulx   0x10(%r15),%r11,%r14
  3ff9ef:	adcx   %r11,%r10
  3ff9f5:	adox   %r14,%r9
  3ff9fb:	mulx   0x18(%r15),%r11,%r14
  3ffa01:	adcx   %r11,%r9
  3ffa07:	adox   %r14,%r8
  3ffa0d:	adcx   %r13,%r8
  3ffa13:	mov    %rsi,%rdx
  3ffa16:	imul   %rbx,%rdx
  3ffa1a:	xor    %r13d,%r13d
  3ffa1d:	mulx   0x0(%rbp),%r11,%r14
  3ffa23:	adcx   %r11,%rsi
  3ffa29:	adox   %r14,%rcx
  3ffa2f:	mulx   0x8(%rbp),%r11,%r14
  3ffa35:	adcx   %r11,%rcx
  3ffa3b:	adox   %r14,%r10
  3ffa41:	mulx   0x10(%rbp),%r11,%r14
  3ffa47:	adcx   %r11,%r10
  3ffa4d:	adox   %r14,%r9
  3ffa53:	mulx   0x18(%rbp),%r11,%r14
  3ffa59:	adcx   %r11,%r9
  3ffa5f:	adox   %r14,%r8
  3ffa65:	adcx   %r13,%r8
  3ffa6b:	mov    0x10(%r12),%rdx
  3ffa70:	xor    %r13d,%r13d
  3ffa73:	mulx   (%r15),%r11,%r14
  3ffa78:	adcx   %r11,%rcx
  3ffa7e:	adox   %r14,%r10
  3ffa84:	mulx   0x8(%r15),%r11,%r14
  3ffa8a:	adcx   %r11,%r10
  3ffa90:	adox   %r14,%r9
  3ffa96:	mulx   0x10(%r15),%r11,%r14
  3ffa9c:	adcx   %r11,%r9
  3ffaa2:	adox   %r14,%r8
  3ffaa8:	mulx   0x18(%r15),%r11,%r14
  3ffaae:	adcx   %r11,%r8
  3ffab4:	adox   %r14,%rsi
  3ffaba:	adcx   %r13,%rsi
  3ffac0:	mov    %rcx,%rdx
  3ffac3:	imul   %rbx,%rdx
  3ffac7:	xor    %r13d,%r13d
  3ffaca:	mulx   0x0(%rbp),%r11,%r14
  3ffad0:	adcx   %r11,%rcx
  3ffad6:	adox   %r14,%r10
  3ffadc:	mulx   0x8(%rbp),%r11,%r14
  3ffae2:	adcx   %r11,%r10
  3ffae8:	adox   %r14,%r9
  3ffaee:	mulx   0x10(%rbp),%r11,%r14
  3ffaf4:	adcx   %r11,%r9
  3ffafa:	adox   %r14,%r8
  3ffb00:	mulx   0x18(%rbp),%r11,%r14
  3ffb06:	adcx   %r11,%r8
  3ffb0c:	adox   %r14,%rsi
  3ffb12:	adcx   %r13,%rsi
  3ffb18:	mov    0x18(%r12),%rdx
  3ffb1d:	xor    %r13d,%r13d
  3ffb20:	mulx   (%r15),%r11,%r14
  3ffb25:	adcx   %r11,%r10
  3ffb2b:	adox   %r14,%r9
  3ffb31:	mulx   0x8(%r15),%r11,%r14
  3ffb37:	adcx   %r11,%r9
  3ffb3d:	adox   %r14,%r8
  3ffb43:	mulx   0x10(%r15),%r11,%r14
  3ffb49:	adcx   %r11,%r8
  3ffb4f:	adox   %r14,%rsi
  3ffb55:	mulx   0x18(%r15),%r11,%r14
  3ffb5b:	adcx   %r11,%rsi
  3ffb61:	adox   %r14,%rcx
  3ffb67:	adcx   %r13,%rcx
  3ffb6d:	mov    %r10,%rdx
  3ffb70:	imul   %rbx,%rdx
  3ffb74:	xor    %r13d,%r13d
  3ffb77:	mulx   0x0(%rbp),%r11,%r14
  3ffb7d:	adcx   %r11,%r10
  3ffb83:	adox   %r14,%r9
  3ffb89:	mulx   0x8(%rbp),%r11,%r14
  3ffb8f:	adcx   %r11,%r9
  3ffb95:	adox   %r14,%r8
  3ffb9b:	mulx   0x10(%rbp),%r11,%r14
  3ffba1:	adcx   %r11,%r8
  3ffba7:	adox   %r14,%rsi
  3ffbad:	mulx   0x18(%rbp),%r11,%r14
  3ffbb3:	adcx   %r11,%rsi
  3ffbb9:	adox   %r14,%rcx
  3ffbbf:	adcx   %r13,%rcx
  3ffbc5:	movabs $0xd7cc17b786468f6f,%rdx
  3ffbcf:	movabs $0x43e1f593f0000001,%r10
  3ffbd9:	movabs $0x2833e84879b97091,%r11
  3ffbe3:	movabs $0xb85045b68181585d,%r14
  3ffbed:	movabs $0x30644e72e131a029,%r15
  3ffbf7:	vmovups %ymm1,0x160(%rsp)
  3ffc00:	add    %r8,%rdx
  3ffc03:	cmp    %r10,%r9
  3ffc06:	mov    0x40(%rsp),%rbx
  3ffc0b:	sbb    $0x0,%rdx
  3ffc0f:	setb   %r10b
  3ffc13:	cmp    %r11,%r8
  3ffc16:	setb   %r11b
  3ffc1a:	or     %r10b,%r11b
  3ffc1d:	movabs $0x47afba497e7ea7a3,%r10
  3ffc27:	add    %rsi,%r10
  3ffc2a:	movzbl %r11b,%r11d
  3ffc2e:	sub    %r11,%r10
  3ffc31:	setb   %r11b
  3ffc35:	cmp    %r14,%rsi
  3ffc38:	setb   %r14b
  3ffc3c:	or     %r11b,%r14b
  3ffc3f:	movabs $0xcf9bb18d1ece5fd7,%r11
  3ffc49:	add    %rcx,%r11
  3ffc4c:	movzbl %r14b,%r14d
  3ffc50:	sub    %r14,%r11
  3ffc53:	setb   %r14b
  3ffc57:	cmp    %r15,%rcx
  3ffc5a:	mov    0x78(%rsp),%r15
  3ffc5f:	setb   %r13b
  3ffc63:	or     %r14b,%r13b
  3ffc66:	movabs $0xbc1e0a6c0fffffff,%r14
  3ffc70:	lea    (%r9,%r14,1),%r14
  3ffc74:	cmovne %r8,%rdx
  3ffc78:	cmovne %rsi,%r10
  3ffc7c:	cmovne %rcx,%r11
  3ffc80:	cmovne %r9,%r14
  3ffc84:	mov    0x90(%rsp),%r9
  3ffc8c:	add    $0x20,%rax
  3ffc90:	inc    %rdi
  3ffc93:	mov    %r14,0x100(%rsp)
  3ffc9b:	mov    %rdx,0x108(%rsp)
  3ffca3:	mov    %r10,0x110(%rsp)
  3ffcab:	mov    %r11,0x118(%rsp)
  3ffcb3:	mov    0x18(%rsp),%r10
  3ffcb8:	mov    0x28(%rsp),%r11
  3ffcbd:	add    (%r12),%r9
  3ffcc1:	adc    0x8(%r12),%r15
  3ffcc6:	adc    0x10(%r12),%r10
  3ffccb:	adc    0x18(%r12),%r11
  3ffcd0:	mov    %r9,%rcx
  3ffcd3:	mov    %r15,%rdx
  3ffcd6:	mov    %r10,%rsi
  3ffcd9:	mov    %r11,%r8
  3ffcdc:	sub    0x0(%rbp),%rcx
  3ffce0:	sbb    0x8(%rbp),%rdx
  3ffce4:	sbb    0x10(%rbp),%rsi
  3ffce8:	sbb    0x18(%rbp),%r8
  3ffcec:	cmovae %rcx,%r9
  3ffcf0:	cmovae %rdx,%r15
  3ffcf4:	cmovae %rsi,%r10
  3ffcf8:	cmovae %r8,%r11
  3ffcfc:	mov    %r9,0x90(%rsp)
  3ffd04:	cmp    %rax,0x88(%rsp)
  3ffd0c:	jne    3ff8a0 <<zopro_core::solver::Plan>::run_poseidon_task+0xfd0>
  3ffd12:	mov    0x8(%rsp),%r8
  3ffd17:	mov    %r11,0x28(%rsp)
  3ffd1c:	mov    %r10,0x18(%rsp)
  3ffd21:	mov    %r15,0x78(%rsp)
  3ffd26:	sub    $0x8,%rsp
  3ffd2a:	mov    0x88(%rsp),%rsi
  3ffd32:	mov    0x50(%rsp),%rdx
  3ffd37:	mov    0x38(%rsp),%rcx
  3ffd3c:	mov    0x40(%rsp),%r9
  3ffd41:	lea    0xc8(%rsp),%rdi
  3ffd49:	push   0x28(%rsp)
  3ffd4d:	lea    0xf0(%rsp),%rax
  3ffd55:	push   %rax
  3ffd56:	push   0x28(%rsp)
  3ffd5a:	vzeroupper
  3ffd5d:	call   403080 <<zopro_core::solver::Plan>::emit_poseidon>
  3ffd62:	add    $0x20,%rsp
  3ffd66:	vmovups -0x3ee5ce(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  3ffd6e:	mov    0x1d0(%rsp),%rdi
  3ffd76:	xor    %eax,%eax
  3ffd78:	nopl   0x0(%rax,%rax,1)
  3ffd80:	cmp    %rbx,%rdi
  3ffd83:	jae    400e77 <<zopro_core::solver::Plan>::run_poseidon_task+0x25a7>
  3ffd89:	mov    0xa8(%rsp),%rcx
  3ffd91:	vmovups 0xc0(%rsp),%ymm2
  3ffd9a:	vmovups %ymm1,0x160(%rsp)
  3ffda3:	lea    0x160(%rsp),%rbp
  3ffdab:	lea    0x100(%rsp),%r12
  3ffdb3:	lea    0x50(%rsp),%r15
  3ffdb8:	movabs $0xc2e1f593efffffff,%rbx
  3ffdc2:	vmovups (%rcx,%rax,1),%ymm0
  3ffdc7:	vmovups %ymm2,0x100(%rsp)
  3ffdd0:	vmovups %ymm0,0x50(%rsp)
  3ffdd6:	xor    %r9d,%r9d
  3ffdd9:	xor    %r8d,%r8d
  3ffddc:	xor    %esi,%esi
  3ffdde:	xor    %ecx,%ecx
  3ffde0:	xor    %r10d,%r10d
  3ffde3:	mov    (%r12),%rdx
  3ffde7:	xor    %r13d,%r13d
  3ffdea:	mulx   (%r15),%r11,%r14
  3ffdef:	adcx   %r11,%r9
  3ffdf5:	adox   %r14,%r8
  3ffdfb:	mulx   0x8(%r15),%r11,%r14
  3ffe01:	adcx   %r11,%r8
  3ffe07:	adox   %r14,%rsi
  3ffe0d:	mulx   0x10(%r15),%r11,%r14
  3ffe13:	adcx   %r11,%rsi
  3ffe19:	adox   %r14,%rcx
  3ffe1f:	mulx   0x18(%r15),%r11,%r14
  3ffe25:	adcx   %r11,%rcx
  3ffe2b:	adox   %r14,%r10
  3ffe31:	adcx   %r13,%r10
  3ffe37:	mov    %r9,%rdx
  3ffe3a:	imul   %rbx,%rdx
  3ffe3e:	xor    %r13d,%r13d
  3ffe41:	mulx   0x0(%rbp),%r11,%r14
  3ffe47:	adcx   %r11,%r9
  3ffe4d:	adox   %r14,%r8
  3ffe53:	mulx   0x8(%rbp),%r11,%r14
  3ffe59:	adcx   %r11,%r8
  3ffe5f:	adox   %r14,%rsi
  3ffe65:	mulx   0x10(%rbp),%r11,%r14
  3ffe6b:	adcx   %r11,%rsi
  3ffe71:	adox   %r14,%rcx
  3ffe77:	mulx   0x18(%rbp),%r11,%r14
  3ffe7d:	adcx   %r11,%rcx
  3ffe83:	adox   %r14,%r10
  3ffe89:	adcx   %r13,%r10
  3ffe8f:	mov    0x8(%r12),%rdx
  3ffe94:	xor    %r13d,%r13d
  3ffe97:	mulx   (%r15),%r11,%r14
  3ffe9c:	adcx   %r11,%r8
  3ffea2:	adox   %r14,%rsi
  3ffea8:	mulx   0x8(%r15),%r11,%r14
  3ffeae:	adcx   %r11,%rsi
  3ffeb4:	adox   %r14,%rcx
  3ffeba:	mulx   0x10(%r15),%r11,%r14
  3ffec0:	adcx   %r11,%rcx
  3ffec6:	adox   %r14,%r10
  3ffecc:	mulx   0x18(%r15),%r11,%r14
  3ffed2:	adcx   %r11,%r10
  3ffed8:	adox   %r14,%r9
  3ffede:	adcx   %r13,%r9
  3ffee4:	mov    %r8,%rdx
  3ffee7:	imul   %rbx,%rdx
  3ffeeb:	xor    %r13d,%r13d
  3ffeee:	mulx   0x0(%rbp),%r11,%r14
  3ffef4:	adcx   %r11,%r8
  3ffefa:	adox   %r14,%rsi
  3fff00:	mulx   0x8(%rbp),%r11,%r14
  3fff06:	adcx   %r11,%rsi
  3fff0c:	adox   %r14,%rcx
  3fff12:	mulx   0x10(%rbp),%r11,%r14
  3fff18:	adcx   %r11,%rcx
  3fff1e:	adox   %r14,%r10
  3fff24:	mulx   0x18(%rbp),%r11,%r14
  3fff2a:	adcx   %r11,%r10
  3fff30:	adox   %r14,%r9
  3fff36:	adcx   %r13,%r9
  3fff3c:	mov    0x10(%r12),%rdx
  3fff41:	xor    %r13d,%r13d
  3fff44:	mulx   (%r15),%r11,%r14
  3fff49:	adcx   %r11,%rsi
  3fff4f:	adox   %r14,%rcx
  3fff55:	mulx   0x8(%r15),%r11,%r14
  3fff5b:	adcx   %r11,%rcx
  3fff61:	adox   %r14,%r10
  3fff67:	mulx   0x10(%r15),%r11,%r14
  3fff6d:	adcx   %r11,%r10
  3fff73:	adox   %r14,%r9
  3fff79:	mulx   0x18(%r15),%r11,%r14
  3fff7f:	adcx   %r11,%r9
  3fff85:	adox   %r14,%r8
  3fff8b:	adcx   %r13,%r8
  3fff91:	mov    %rsi,%rdx
  3fff94:	imul   %rbx,%rdx
  3fff98:	xor    %r13d,%r13d
  3fff9b:	mulx   0x0(%rbp),%r11,%r14
  3fffa1:	adcx   %r11,%rsi
  3fffa7:	adox   %r14,%rcx
  3fffad:	mulx   0x8(%rbp),%r11,%r14
  3fffb3:	adcx   %r11,%rcx
  3fffb9:	adox   %r14,%r10
  3fffbf:	mulx   0x10(%rbp),%r11,%r14
  3fffc5:	adcx   %r11,%r10
  3fffcb:	adox   %r14,%r9
  3fffd1:	mulx   0x18(%rbp),%r11,%r14
  3fffd7:	adcx   %r11,%r9
  3fffdd:	adox   %r14,%r8
  3fffe3:	adcx   %r13,%r8
  3fffe9:	mov    0x18(%r12),%rdx
  3fffee:	xor    %r13d,%r13d
  3ffff1:	mulx   (%r15),%r11,%r14
  3ffff6:	adcx   %r11,%rcx
  3ffffc:	adox   %r14,%r10
  400002:	mulx   0x8(%r15),%r11,%r14
  400008:	adcx   %r11,%r10
  40000e:	adox   %r14,%r9
  400014:	mulx   0x10(%r15),%r11,%r14
  40001a:	adcx   %r11,%r9
  400020:	adox   %r14,%r8
  400026:	mulx   0x18(%r15),%r11,%r14
  40002c:	adcx   %r11,%r8
  400032:	adox   %r14,%rsi
  400038:	adcx   %r13,%rsi
  40003e:	mov    %rcx,%rdx
  400041:	imul   %rbx,%rdx
  400045:	xor    %r13d,%r13d
  400048:	mulx   0x0(%rbp),%r11,%r14
  40004e:	adcx   %r11,%rcx
  400054:	adox   %r14,%r10
  40005a:	mulx   0x8(%rbp),%r11,%r14
  400060:	adcx   %r11,%r10
  400066:	adox   %r14,%r9
  40006c:	mulx   0x10(%rbp),%r11,%r14
  400072:	adcx   %r11,%r9
  400078:	adox   %r14,%r8
  40007e:	mulx   0x18(%rbp),%r11,%r14
  400084:	adcx   %r11,%r8
  40008a:	adox   %r14,%rsi
  400090:	adcx   %r13,%rsi
  400096:	movabs $0xd7cc17b786468f6f,%rcx
  4000a0:	movabs $0x43e1f593f0000001,%rdx
  4000aa:	movabs $0x2833e84879b97091,%r11
  4000b4:	movabs $0xb85045b68181585d,%r14
  4000be:	movabs $0x30644e72e131a029,%r15
  4000c8:	vmovups %ymm1,0x160(%rsp)
  4000d1:	add    %r9,%rcx
  4000d4:	cmp    %rdx,%r10
  4000d7:	mov    0x40(%rsp),%rbx
  4000dc:	sbb    $0x0,%rcx
  4000e0:	setb   %dl
  4000e3:	cmp    %r11,%r9
  4000e6:	setb   %r11b
  4000ea:	or     %dl,%r11b
  4000ed:	movabs $0x47afba497e7ea7a3,%rdx
  4000f7:	add    %r8,%rdx
  4000fa:	movzbl %r11b,%r11d
  4000fe:	sub    %r11,%rdx
  400101:	setb   %r11b
  400105:	cmp    %r14,%r8
  400108:	setb   %r14b
  40010c:	or     %r11b,%r14b
  40010f:	movabs $0xcf9bb18d1ece5fd7,%r11
  400119:	add    %rsi,%r11
  40011c:	movzbl %r14b,%r14d
  400120:	sub    %r14,%r11
  400123:	setb   %r14b
  400127:	cmp    %r15,%rsi
  40012a:	setb   %r13b
  40012e:	or     %r14b,%r13b
  400131:	movabs $0xbc1e0a6c0fffffff,%r14
  40013b:	lea    (%r10,%r14,1),%r14
  40013f:	cmovne %r9,%rcx
  400143:	cmovne %r8,%rdx
  400147:	cmovne %rsi,%r11
  40014b:	mov    0x260(%rsp,%rax,1),%rsi
  400153:	mov    0x268(%rsp,%rax,1),%r8
  40015b:	mov    0x270(%rsp,%rax,1),%r9
  400163:	cmovne %r10,%r14
  400167:	mov    0x278(%rsp,%rax,1),%r10
  40016f:	inc    %rdi
  400172:	mov    %r14,0x100(%rsp)
  40017a:	mov    %rcx,0x108(%rsp)
  400182:	mov    %rdx,0x110(%rsp)
  40018a:	mov    %r11,0x118(%rsp)
  400192:	add    (%r12),%rsi
  400196:	adc    0x8(%r12),%r8
  40019b:	adc    0x10(%r12),%r9
  4001a0:	adc    0x18(%r12),%r10
  4001a5:	mov    %rsi,%rcx
  4001a8:	mov    %r8,%rdx
  4001ab:	mov    %r9,%r11
  4001ae:	mov    %r10,%r14
  4001b1:	sub    0x0(%rbp),%rcx
  4001b5:	sbb    0x8(%rbp),%rdx
  4001b9:	sbb    0x10(%rbp),%r11
  4001bd:	sbb    0x18(%rbp),%r14
  4001c1:	cmovae %rcx,%rsi
  4001c5:	cmovae %rdx,%r8
  4001c9:	cmovae %r11,%r9
  4001cd:	cmovae %r14,%r10
  4001d1:	mov    %rsi,0x260(%rsp,%rax,1)
  4001d9:	mov    %r8,0x268(%rsp,%rax,1)
  4001e1:	mov    %r9,0x270(%rsp,%rax,1)
  4001e9:	mov    %r10,0x278(%rsp,%rax,1)
  4001f1:	add    $0x20,%rax
  4001f5:	cmp    %rax,0x238(%rsp)
  4001fd:	jne    3ffd80 <<zopro_core::solver::Plan>::run_poseidon_task+0x14b0>
  400203:	mov    0x230(%rsp),%r13
  40020b:	mov    0x90(%rsp),%rbp
  400213:	mov    0x228(%rsp),%r14
  40021b:	cmp    %rbx,%r14
  40021e:	jae    400f05 <<zopro_core::solver::Plan>::run_poseidon_task+0x2635>
  400224:	mov    (%rsp),%rax
  400228:	shl    $0x5,%r14
  40022c:	vmovups 0xc0(%rsp),%ymm2
  400235:	incq   0x10(%rsp)
  40023a:	vmovups %ymm1,0x160(%rsp)
  400243:	lea    0x100(%rsp),%r12
  40024b:	lea    0x50(%rsp),%r15
  400250:	movabs $0xc2e1f593efffffff,%rbx
  40025a:	vmovups (%rax,%r14,1),%ymm0
  400260:	lea    0x160(%rsp),%r14
  400268:	vmovups %ymm2,0x100(%rsp)
  400271:	vmovups %ymm0,0x50(%rsp)
  400277:	xor    %r8d,%r8d
  40027a:	xor    %edi,%edi
  40027c:	xor    %esi,%esi
  40027e:	xor    %eax,%eax
  400280:	xor    %r9d,%r9d
  400283:	mov    (%r12),%rdx
  400287:	xor    %r11d,%r11d
  40028a:	mulx   (%r15),%rcx,%r10
  40028f:	adcx   %rcx,%r8
  400295:	adox   %r10,%rdi
  40029b:	mulx   0x8(%r15),%rcx,%r10
  4002a1:	adcx   %rcx,%rdi
  4002a7:	adox   %r10,%rsi
  4002ad:	mulx   0x10(%r15),%rcx,%r10
  4002b3:	adcx   %rcx,%rsi
  4002b9:	adox   %r10,%rax
  4002bf:	mulx   0x18(%r15),%rcx,%r10
  4002c5:	adcx   %rcx,%rax
  4002cb:	adox   %r10,%r9
  4002d1:	adcx   %r11,%r9
  4002d7:	mov    %r8,%rdx
  4002da:	imul   %rbx,%rdx
  4002de:	xor    %r11d,%r11d
  4002e1:	mulx   (%r14),%rcx,%r10
  4002e6:	adcx   %rcx,%r8
  4002ec:	adox   %r10,%rdi
  4002f2:	mulx   0x8(%r14),%rcx,%r10
  4002f8:	adcx   %rcx,%rdi
  4002fe:	adox   %r10,%rsi
  400304:	mulx   0x10(%r14),%rcx,%r10
  40030a:	adcx   %rcx,%rsi
  400310:	adox   %r10,%rax
  400316:	mulx   0x18(%r14),%rcx,%r10
  40031c:	adcx   %rcx,%rax
  400322:	adox   %r10,%r9
  400328:	adcx   %r11,%r9
  40032e:	mov    0x8(%r12),%rdx
  400333:	xor    %r11d,%r11d
  400336:	mulx   (%r15),%rcx,%r10
  40033b:	adcx   %rcx,%rdi
  400341:	adox   %r10,%rsi
  400347:	mulx   0x8(%r15),%rcx,%r10
  40034d:	adcx   %rcx,%rsi
  400353:	adox   %r10,%rax
  400359:	mulx   0x10(%r15),%rcx,%r10
  40035f:	adcx   %rcx,%rax
  400365:	adox   %r10,%r9
  40036b:	mulx   0x18(%r15),%rcx,%r10
  400371:	adcx   %rcx,%r9
  400377:	adox   %r10,%r8
  40037d:	adcx   %r11,%r8
  400383:	mov    %rdi,%rdx
  400386:	imul   %rbx,%rdx
  40038a:	xor    %r11d,%r11d
  40038d:	mulx   (%r14),%rcx,%r10
  400392:	adcx   %rcx,%rdi
  400398:	adox   %r10,%rsi
  40039e:	mulx   0x8(%r14),%rcx,%r10
  4003a4:	adcx   %rcx,%rsi
  4003aa:	adox   %r10,%rax
  4003b0:	mulx   0x10(%r14),%rcx,%r10
  4003b6:	adcx   %rcx,%rax
  4003bc:	adox   %r10,%r9
  4003c2:	mulx   0x18(%r14),%rcx,%r10
  4003c8:	adcx   %rcx,%r9
  4003ce:	adox   %r10,%r8
  4003d4:	adcx   %r11,%r8
  4003da:	mov    0x10(%r12),%rdx
  4003df:	xor    %r11d,%r11d
  4003e2:	mulx   (%r15),%rcx,%r10
  4003e7:	adcx   %rcx,%rsi
  4003ed:	adox   %r10,%rax
  4003f3:	mulx   0x8(%r15),%rcx,%r10
  4003f9:	adcx   %rcx,%rax
  4003ff:	adox   %r10,%r9
  400405:	mulx   0x10(%r15),%rcx,%r10
  40040b:	adcx   %rcx,%r9
  400411:	adox   %r10,%r8
  400417:	mulx   0x18(%r15),%rcx,%r10
  40041d:	adcx   %rcx,%r8
  400423:	adox   %r10,%rdi
  400429:	adcx   %r11,%rdi
  40042f:	mov    %rsi,%rdx
  400432:	imul   %rbx,%rdx
  400436:	xor    %r11d,%r11d
  400439:	mulx   (%r14),%rcx,%r10
  40043e:	adcx   %rcx,%rsi
  400444:	adox   %r10,%rax
  40044a:	mulx   0x8(%r14),%rcx,%r10
  400450:	adcx   %rcx,%rax
  400456:	adox   %r10,%r9
  40045c:	mulx   0x10(%r14),%rcx,%r10
  400462:	adcx   %rcx,%r9
  400468:	adox   %r10,%r8
  40046e:	mulx   0x18(%r14),%rcx,%r10
  400474:	adcx   %rcx,%r8
  40047a:	adox   %r10,%rdi
  400480:	adcx   %r11,%rdi
  400486:	mov    0x18(%r12),%rdx
  40048b:	xor    %r11d,%r11d
  40048e:	mulx   (%r15),%rcx,%r10
  400493:	adcx   %rcx,%rax
  400499:	adox   %r10,%r9
  40049f:	mulx   0x8(%r15),%rcx,%r10
  4004a5:	adcx   %rcx,%r9
  4004ab:	adox   %r10,%r8
  4004b1:	mulx   0x10(%r15),%rcx,%r10
  4004b7:	adcx   %rcx,%r8
  4004bd:	adox   %r10,%rdi
  4004c3:	mulx   0x18(%r15),%rcx,%r10
  4004c9:	adcx   %rcx,%rdi
  4004cf:	adox   %r10,%rsi
  4004d5:	adcx   %r11,%rsi
  4004db:	mov    %rax,%rdx
  4004de:	imul   %rbx,%rdx
  4004e2:	xor    %r11d,%r11d
  4004e5:	mulx   (%r14),%rcx,%r10
  4004ea:	adcx   %rcx,%rax
  4004f0:	adox   %r10,%r9
  4004f6:	mulx   0x8(%r14),%rcx,%r10
  4004fc:	adcx   %rcx,%r9
  400502:	adox   %r10,%r8
  400508:	mulx   0x10(%r14),%rcx,%r10
  40050e:	adcx   %rcx,%r8
  400514:	adox   %r10,%rdi
  40051a:	mulx   0x18(%r14),%rcx,%r10
  400520:	adcx   %rcx,%rdi
  400526:	adox   %r10,%rsi
  40052c:	adcx   %r11,%rsi
  400532:	movabs $0xd7cc17b786468f6f,%rax
  40053c:	movabs $0x43e1f593f0000001,%rcx
  400546:	movabs $0x2833e84879b97091,%rdx
  400550:	movabs $0xb85045b68181585d,%r10
  40055a:	movabs $0x30644e72e131a029,%r11
  400564:	mov    %rbp,0x100(%rsp)
  40056c:	vmovups %ymm1,0x160(%rsp)
  400575:	add    %r8,%rax
  400578:	cmp    %rcx,%r9
  40057b:	mov    0x78(%rsp),%rbx
  400580:	sbb    $0x0,%rax
  400584:	setb   %cl
  400587:	cmp    %rdx,%r8
  40058a:	setb   %dl
  40058d:	or     %cl,%dl
  40058f:	movabs $0x47afba497e7ea7a3,%rcx
  400599:	add    %rdi,%rcx
  40059c:	movzbl %dl,%edx
  40059f:	mov    %rbx,0x108(%rsp)
  4005a7:	sub    %rdx,%rcx
  4005aa:	setb   %dl
  4005ad:	cmp    %r10,%rdi
  4005b0:	setb   %r10b
  4005b4:	or     %dl,%r10b
  4005b7:	movabs $0xcf9bb18d1ece5fd7,%rdx
  4005c1:	add    %rsi,%rdx
  4005c4:	movzbl %r10b,%r10d
  4005c8:	sub    %r10,%rdx
  4005cb:	setb   %r10b
  4005cf:	cmp    %r11,%rsi
  4005d2:	setb   %r11b
  4005d6:	or     %r10b,%r11b
  4005d9:	mov    0x18(%rsp),%r11
  4005de:	movabs $0xbc1e0a6c0fffffff,%r10
  4005e8:	cmovne %rsi,%rdx
  4005ec:	mov    0x28(%rsp),%rsi
  4005f1:	lea    (%r9,%r10,1),%r10
  4005f5:	cmovne %r8,%rax
  4005f9:	cmovne %rdi,%rcx
  4005fd:	cmovne %r9,%r10
  400601:	mov    %r11,0x110(%rsp)
  400609:	mov    %rsi,0x118(%rsp)
  400611:	add    (%r12),%r10
  400615:	adc    0x8(%r12),%rax
  40061a:	adc    0x10(%r12),%rcx
  40061f:	adc    0x18(%r12),%rdx
  400624:	mov    %r10,%rsi
  400627:	mov    %rax,%rdi
  40062a:	mov    %rcx,%r8
  40062d:	mov    %rdx,%r9
  400630:	sub    (%r14),%rsi
  400633:	sbb    0x8(%r14),%rdi
  400637:	sbb    0x10(%r14),%r8
  40063b:	sbb    0x18(%r14),%r9
  40063f:	cmovae %rsi,%r10
  400643:	cmovae %rdi,%rax
  400647:	cmovae %r8,%rcx
  40064b:	cmovae %r9,%rdx
  40064f:	mov    %r10,0xe0(%rsp)
  400657:	mov    %rax,0xe8(%rsp)
  40065f:	mov    %rcx,0xf0(%rsp)
  400667:	mov    %rdx,0xf8(%rsp)
  40066f:	mov    %r13,%r14
  400672:	mov    0x218(%rsp),%r8
  40067a:	mov    0x210(%rsp),%rax
  400682:	mov    0x38(%rsp),%r9
  400687:	mov    0x80(%rsp),%rbp
  40068f:	mov    0xb0(%rsp),%rdi
  400697:	add    %r8,0x1c8(%rsp)
  40069f:	add    %rax,0x1d8(%rsp)
  4006a7:	add    %r8,0x1d0(%rsp)
  4006af:	add    %rax,0xa8(%rsp)
  4006b7:	cmp    %r9,%r13
  4006ba:	mov    0x20(%rsp),%r13
  4006bf:	jne    3ff160 <<zopro_core::solver::Plan>::run_poseidon_task+0x890>
  4006c5:	mov    0xe8(%rsp),%rdx
  4006cd:	mov    0xf0(%rsp),%rcx
  4006d5:	mov    0xe0(%rsp),%rax
  4006dd:	mov    %rdx,0x248(%rsp)
  4006e5:	mov    0xf8(%rsp),%rdx
  4006ed:	mov    %rcx,0x250(%rsp)
  4006f5:	mov    0x8(%rsp),%rcx
  4006fa:	mov    %rax,0x240(%rsp)
  400702:	mov    %rdx,0x258(%rsp)
  40070a:	mov    0x40(%rsp),%rdx
  40070f:	neg    %rcx
  400712:	mov    %rcx,0x28(%rsp)
  400717:	mov    %rdx,%rcx
  40071a:	neg    %rcx
  40071d:	cmp    %rdx,%rdi
  400720:	mov    %rdx,%r14
  400723:	mov    %rcx,0x78(%rsp)
  400728:	mov    (%rsp),%rcx
  40072c:	cmovb  %rdi,%r14
  400730:	shl    $0x5,%rdi
  400734:	xor    %ebx,%ebx
  400736:	xor    %r15d,%r15d
  400739:	sub    %rdx,%r14
  40073c:	mov    %rdi,0x90(%rsp)
  400744:	lea    (%rcx,%rdi,1),%rax
  400748:	mov    %rax,0x18(%rsp)
  40074d:	nopl   (%rax)
  400750:	vmovups -0x3eefb8(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  400758:	cmp    %rbx,%r14
  40075b:	je     400f1c <<zopro_core::solver::Plan>::run_poseidon_task+0x264c>
  400761:	mov    0x18(%rsp),%rdi
  400766:	vmovups %ymm1,0x160(%rsp)
  40076f:	mov    0x258(%rsp,%r15,1),%rax
  400777:	mov    0x250(%rsp,%r15,1),%rcx
  40077f:	mov    0x240(%rsp,%r15,1),%rdx
  400787:	mov    0x248(%rsp,%r15,1),%rsi
  40078f:	lea    0x160(%rsp),%r11
  400797:	lea    0x100(%rsp),%r12
  40079f:	vmovups (%rdi,%r15,1),%ymm0
  4007a5:	vmovups %ymm0,0x100(%rsp)
  4007ae:	add    (%r12),%rdx
  4007b2:	adc    0x8(%r12),%rsi
  4007b7:	adc    0x10(%r12),%rcx
  4007bc:	adc    0x18(%r12),%rax
  4007c1:	mov    %rdx,%rdi
  4007c4:	mov    %rsi,%r8
  4007c7:	mov    %rcx,%r9
  4007ca:	mov    %rax,%r10
  4007cd:	sub    (%r11),%rdi
  4007d0:	sbb    0x8(%r11),%r8
  4007d4:	sbb    0x10(%r11),%r9
  4007d8:	sbb    0x18(%r11),%r10
  4007dc:	cmovae %rdi,%rdx
  4007e0:	cmovae %r8,%rsi
  4007e4:	cmovae %r9,%rcx
  4007e8:	cmovae %r10,%rax
  4007ec:	mov    %rdx,0x50(%rsp)
  4007f1:	mov    %rsi,0x58(%rsp)
  4007f6:	mov    %rcx,0x60(%rsp)
  4007fb:	mov    %rax,0x68(%rsp)
  400800:	sub    $0x8,%rsp
  400804:	mov    0x50(%rsp),%rdx
  400809:	mov    0x38(%rsp),%rcx
  40080e:	mov    0x10(%rsp),%r8
  400813:	mov    0x40(%rsp),%r9
  400818:	lea    0xc8(%rsp),%rdi
  400820:	mov    %rbp,%rsi
  400823:	push   %r13
  400825:	lea    0x60(%rsp),%rax
  40082a:	push   %rax
  40082b:	mov    0x28(%rsp),%r12
  400830:	push   %r12
  400832:	vzeroupper
  400835:	call   403080 <<zopro_core::solver::Plan>::emit_poseidon>
  40083a:	add    $0x20,%rsp
  40083e:	vmovups 0xc0(%rsp),%ymm0
  400847:	inc    %r12
  40084a:	dec    %rbx
  40084d:	mov    %r12,0x10(%rsp)
  400852:	vmovups %ymm0,0x240(%rsp,%r15,1)
  40085c:	add    $0x20,%r15
  400860:	cmp    %rbx,0x28(%rsp)
  400865:	jne    400750 <<zopro_core::solver::Plan>::run_poseidon_task+0x1e80>
  40086b:	mov    (%rsp),%r15
  40086f:	mov    0x40(%rsp),%rbx
  400874:	mov    0x8(%rsp),%rsi
  400879:	mov    0xb0(%rsp),%rax
  400881:	mov    0x98(%rsp),%r8
  400889:	lea    0x240(%rsp),%rdi
  400891:	lea    (%rax,%rsi,1),%r14
  400895:	mov    %r15,%rdx
  400898:	mov    %rbx,%rcx
  40089b:	vzeroupper
  40089e:	call   402010 <<zopro_core::solver::Plan>::run_poseidon::dense>
  4008a3:	cmp    %r14,%rbx
  4008a6:	mov    %r14,%rax
  4008a9:	cmovb  %rbx,%rax
  4008ad:	mov    %r14,%rbx
  4008b0:	shl    $0x5,%rbx
  4008b4:	xor    %ebp,%ebp
  4008b6:	add    %rax,0x78(%rsp)
  4008bb:	add    %r15,%rbx
  4008be:	xor    %r15d,%r15d
  4008c1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  4008d0:	vmovups -0x3ef138(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  4008d8:	cmp    %r15,0x78(%rsp)
  4008dd:	je     400f3b <<zopro_core::solver::Plan>::run_poseidon_task+0x266b>
  4008e3:	vmovups (%rbx,%rbp,1),%ymm0
  4008e8:	vmovups %ymm1,0x160(%rsp)
  4008f1:	mov    0x258(%rsp,%rbp,1),%rax
  4008f9:	mov    0x250(%rsp,%rbp,1),%rcx
  400901:	mov    0x240(%rsp,%rbp,1),%rdx
  400909:	mov    0x248(%rsp,%rbp,1),%rsi
  400911:	lea    0x160(%rsp),%r11
  400919:	lea    0x100(%rsp),%r13
  400921:	vmovups %ymm0,0x100(%rsp)
  40092a:	add    0x0(%r13),%rdx
  40092e:	adc    0x8(%r13),%rsi
  400932:	adc    0x10(%r13),%rcx
  400936:	adc    0x18(%r13),%rax
  40093a:	mov    %rdx,%rdi
  40093d:	mov    %rsi,%r8
  400940:	mov    %rcx,%r9
  400943:	mov    %rax,%r10
  400946:	sub    (%r11),%rdi
  400949:	sbb    0x8(%r11),%r8
  40094d:	sbb    0x10(%r11),%r9
  400951:	sbb    0x18(%r11),%r10
  400955:	cmovae %rdi,%rdx
  400959:	cmovae %r8,%rsi
  40095d:	cmovae %r9,%rcx
  400961:	cmovae %r10,%rax
  400965:	mov    %rdx,0x50(%rsp)
  40096a:	mov    %rsi,0x58(%rsp)
  40096f:	mov    %rcx,0x60(%rsp)
  400974:	mov    %rax,0x68(%rsp)
  400979:	sub    $0x8,%rsp
  40097d:	mov    0x88(%rsp),%rsi
  400985:	mov    0x50(%rsp),%rdx
  40098a:	mov    0x38(%rsp),%rcx
  40098f:	mov    0x10(%rsp),%r8
  400994:	mov    0x40(%rsp),%r9
  400999:	lea    0xc8(%rsp),%rdi
  4009a1:	push   0x28(%rsp)
  4009a5:	lea    0x60(%rsp),%rax
  4009aa:	push   %rax
  4009ab:	mov    0x28(%rsp),%r12
  4009b0:	push   %r12
  4009b2:	vzeroupper
  4009b5:	call   403080 <<zopro_core::solver::Plan>::emit_poseidon>
  4009ba:	add    $0x20,%rsp
  4009be:	vmovups 0xc0(%rsp),%ymm0
  4009c7:	inc    %r12
  4009ca:	dec    %r15
  4009cd:	mov    %r12,0x10(%rsp)
  4009d2:	vmovups %ymm0,0x240(%rsp,%rbp,1)
  4009db:	add    $0x20,%rbp
  4009df:	cmp    %r15,0x28(%rsp)
  4009e4:	jne    4008d0 <<zopro_core::solver::Plan>::run_poseidon_task+0x2000>
  4009ea:	mov    0x8(%rsp),%rsi
  4009ef:	mov    (%rsp),%rbx
  4009f3:	mov    0x40(%rsp),%r13
  4009f8:	mov    0x98(%rsp),%r8
  400a00:	lea    0x240(%rsp),%rdi
  400a08:	add    %rsi,%r14
  400a0b:	mov    %rbx,%rdx
  400a0e:	mov    %r13,%rcx
  400a11:	vzeroupper
  400a14:	call   402010 <<zopro_core::solver::Plan>::run_poseidon::dense>
  400a19:	mov    0xb8(%rsp),%rax
  400a21:	mov    0x80(%rsp),%rbp
  400a29:	mov    %r14,%r15
  400a2c:	add    %rbx,%rax
  400a2f:	add    0x90(%rsp),%rax
  400a37:	xor    %ebx,%ebx
  400a39:	mov    %rax,0xb8(%rsp)
  400a41:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  400a50:	vmovups -0x3ef2b8(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  400a58:	cmp    %r13,%r15
  400a5b:	jae    400f65 <<zopro_core::solver::Plan>::run_poseidon_task+0x2695>
  400a61:	mov    0xb8(%rsp),%rdi
  400a69:	vmovups %ymm1,0x160(%rsp)
  400a72:	mov    0x258(%rsp,%rbx,1),%rax
  400a7a:	mov    0x250(%rsp,%rbx,1),%rcx
  400a82:	mov    0x240(%rsp,%rbx,1),%rdx
  400a8a:	mov    0x248(%rsp,%rbx,1),%rsi
  400a92:	lea    0x160(%rsp),%r11
  400a9a:	lea    0x100(%rsp),%r12
  400aa2:	vmovups (%rdi,%rbx,1),%ymm0
  400aa7:	vmovups %ymm0,0x100(%rsp)
  400ab0:	add    (%r12),%rdx
  400ab4:	adc    0x8(%r12),%rsi
  400ab9:	adc    0x10(%r12),%rcx
  400abe:	adc    0x18(%r12),%rax
  400ac3:	mov    %rdx,%rdi
  400ac6:	mov    %rsi,%r8
  400ac9:	mov    %rcx,%r9
  400acc:	mov    %rax,%r10
  400acf:	sub    (%r11),%rdi
  400ad2:	sbb    0x8(%r11),%r8
  400ad6:	sbb    0x10(%r11),%r9
  400ada:	sbb    0x18(%r11),%r10
  400ade:	cmovae %rdi,%rdx
  400ae2:	cmovae %r8,%rsi
  400ae6:	cmovae %r9,%rcx
  400aea:	cmovae %r10,%rax
  400aee:	mov    %rdx,0x50(%rsp)
  400af3:	mov    %rsi,0x58(%rsp)
  400af8:	mov    %rcx,0x60(%rsp)
  400afd:	mov    %rax,0x68(%rsp)
  400b02:	sub    $0x8,%rsp
  400b06:	mov    0x50(%rsp),%rdx
  400b0b:	mov    0x38(%rsp),%rcx
  400b10:	mov    0x10(%rsp),%r8
  400b15:	mov    0x40(%rsp),%r9
  400b1a:	lea    0xc8(%rsp),%rdi
  400b22:	mov    %rbp,%rsi
  400b25:	push   0x28(%rsp)
  400b29:	lea    0x60(%rsp),%rax
  400b2e:	push   %rax
  400b2f:	mov    0x28(%rsp),%r12
  400b34:	push   %r12
  400b36:	vzeroupper
  400b39:	call   403080 <<zopro_core::solver::Plan>::emit_poseidon>
  400b3e:	add    $0x20,%rsp
  400b42:	vmovups 0xc0(%rsp),%ymm0
  400b4b:	inc    %r12
  400b4e:	inc    %r15
  400b51:	mov    %r12,0x10(%rsp)
  400b56:	vmovups %ymm0,0x240(%rsp,%rbx,1)
  400b5f:	add    $0x20,%rbx
  400b63:	cmp    %rbx,0x88(%rsp)
  400b6b:	jne    400a50 <<zopro_core::solver::Plan>::run_poseidon_task+0x2180>
  400b71:	mov    0x8(%rsp),%rsi
  400b76:	mov    (%rsp),%rdx
  400b7a:	mov    0x98(%rsp),%r8
  400b82:	lea    0x240(%rsp),%rdi
  400b8a:	mov    %r13,%rcx
  400b8d:	add    %rsi,%r14
  400b90:	vzeroupper
  400b93:	call   402010 <<zopro_core::solver::Plan>::run_poseidon::dense>
  400b98:	mov    0x1e8(%rsp),%rax
  400ba0:	xor    %ebx,%ebx
  400ba2:	shl    $0x5,%eax
  400ba5:	add    %rax,0x18(%rsp)
  400baa:	nopw   0x0(%rax,%rax,1)
  400bb0:	vmovups -0x3ef418(%rip),%ymm1        # 117a0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d60>
  400bb8:	cmp    %r13,%r14
  400bbb:	jae    400f7c <<zopro_core::solver::Plan>::run_poseidon_task+0x26ac>
  400bc1:	mov    0x18(%rsp),%rdi
  400bc6:	vmovups %ymm1,0x160(%rsp)
  400bcf:	mov    0x258(%rsp,%rbx,1),%rax
  400bd7:	mov    0x250(%rsp,%rbx,1),%rcx
  400bdf:	mov    0x240(%rsp,%rbx,1),%rdx
  400be7:	mov    0x248(%rsp,%rbx,1),%rsi
  400bef:	lea    0x160(%rsp),%r11
  400bf7:	lea    0x100(%rsp),%r12
  400bff:	vmovups (%rdi,%rbx,1),%ymm0
  400c04:	vmovups %ymm0,0x100(%rsp)
  400c0d:	add    (%r12),%rdx
  400c11:	adc    0x8(%r12),%rsi
  400c16:	adc    0x10(%r12),%rcx
  400c1b:	adc    0x18(%r12),%rax
  400c20:	mov    %rdx,%rdi
  400c23:	mov    %rsi,%r8
  400c26:	mov    %rcx,%r9
  400c29:	mov    %rax,%r10
  400c2c:	sub    (%r11),%rdi
  400c2f:	sbb    0x8(%r11),%r8
  400c33:	sbb    0x10(%r11),%r9
  400c37:	sbb    0x18(%r11),%r10
  400c3b:	cmovae %rdi,%rdx
  400c3f:	cmovae %r8,%rsi
  400c43:	cmovae %r9,%rcx
  400c47:	cmovae %r10,%rax
  400c4b:	mov    %rdx,0x50(%rsp)
  400c50:	mov    %rsi,0x58(%rsp)
  400c55:	mov    %rcx,0x60(%rsp)
  400c5a:	mov    %rax,0x68(%rsp)
  400c5f:	sub    $0x8,%rsp
  400c63:	mov    0x50(%rsp),%rdx
  400c68:	mov    0x38(%rsp),%rcx
  400c6d:	mov    0x10(%rsp),%r8
  400c72:	mov    0x40(%rsp),%r9
  400c77:	lea    0xc8(%rsp),%rdi
  400c7f:	mov    %rbp,%rsi
  400c82:	push   0x28(%rsp)
  400c86:	lea    0x60(%rsp),%rax
  400c8b:	push   %rax
  400c8c:	mov    0x28(%rsp),%r15
  400c91:	push   %r15
  400c93:	vzeroupper
  400c96:	call   403080 <<zopro_core::solver::Plan>::emit_poseidon>
  400c9b:	add    $0x20,%rsp
  400c9f:	vmovups 0xc0(%rsp),%ymm0
  400ca8:	inc    %r15
  400cab:	inc    %r14
  400cae:	mov    %r15,0x10(%rsp)
  400cb3:	vmovups %ymm0,0x240(%rsp,%rbx,1)
  400cbc:	add    $0x20,%rbx
  400cc0:	cmp    %rbx,0x88(%rsp)
  400cc8:	jne    400bb0 <<zopro_core::solver::Plan>::run_poseidon_task+0x22e0>
  400cce:	mov    0x8(%rsp),%rcx
  400cd3:	mov    0x1c0(%rsp),%r14
  400cdb:	mov    0x1f0(%rsp),%r15
  400ce3:	mov    0x1f8(%rsp),%rdx
  400ceb:	lea    0x0(,%rcx,8),%rax
  400cf3:	sub    %rcx,%rax
  400cf6:	mov    0x38(%rsp),%rcx
  400cfb:	add    %rax,%rcx
  400cfe:	lea    (%rcx,%rcx,2),%rax
  400d02:	add    %rax,%r14
  400d05:	cmp    0xa0(%rsp),%r14
  400d0d:	jb     3fe948 <<zopro_core::solver::Plan>::run_poseidon_task+0x78>
  400d13:	movq   $0xffffffffffffffff,(%r15)
  400d1a:	jmp    400df2 <<zopro_core::solver::Plan>::run_poseidon_task+0x2522>
  400d1f:	shlq   $0x2,0xa0(%rsp)
  400d28:	lea    0x160(%rsp),%rbx
  400d30:	lea    0x240(%rsp),%r14
  400d38:	mov    %r15,%r12
  400d3b:	mov    %rdx,%rbp
  400d3e:	xor    %r15d,%r15d
  400d41:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  400d50:	mov    0x80(%rsp),%rax
  400d58:	mov    0x0(%rbp,%r15,1),%edi
  400d5d:	mov    0x90(%rax),%rsi
  400d64:	cmp    %rdi,%rsi
  400d67:	jbe    400f93 <<zopro_core::solver::Plan>::run_poseidon_task+0x26c3>
  400d6d:	mov    0x20(%rsp),%rcx
  400d72:	lea    (%rdi,%rdi,8),%rdx
  400d76:	mov    %rbx,%rdi
  400d79:	mov    %rax,%rsi
  400d7c:	shl    $0x3,%rdx
  400d80:	add    0x88(%rax),%rdx
  400d87:	vmovups (%rcx),%ymm0
  400d8b:	mov    %r14,%rcx
  400d8e:	vmovups %ymm0,0x240(%rsp)
  400d97:	vzeroupper
  400d9a:	call   3fa340 <<zopro_core::solver::Plan>::run>
  400d9f:	cmpq   $0xffffffffffffffff,0x160(%rsp)
  400da8:	jne    400dd6 <<zopro_core::solver::Plan>::run_poseidon_task+0x2506>
  400daa:	add    $0x4,%r15
  400dae:	cmp    %r15,0xa0(%rsp)
  400db6:	jne    400d50 <<zopro_core::solver::Plan>::run_poseidon_task+0x2480>
  400db8:	movq   $0xffffffffffffffff,(%r12)
  400dc0:	jmp    400df2 <<zopro_core::solver::Plan>::run_poseidon_task+0x2522>
  400dc2:	vmovups 0x168(%rsp),%xmm0
  400dcb:	vmovups %xmm0,0x8(%r15)
  400dd1:	mov    %rax,(%r15)
  400dd4:	jmp    400df2 <<zopro_core::solver::Plan>::run_poseidon_task+0x2522>
  400dd6:	vmovups 0x160(%rsp),%xmm0
  400ddf:	mov    0x170(%rsp),%rax
  400de7:	mov    %rax,0x10(%r12)
  400dec:	vmovups %xmm0,(%r12)
  400df2:	add    $0x3e8,%rsp
  400df9:	pop    %rbx
  400dfa:	pop    %r12
  400dfc:	pop    %r13
  400dfe:	pop    %r14
  400e00:	pop    %r15
  400e02:	pop    %rbp
  400e03:	vzeroupper
  400e06:	ret
  400e07:	vmovups 0x160(%rsp),%xmm0
  400e10:	mov    0x170(%rsp),%rax
  400e18:	mov    %rax,0x10(%r15)
  400e1c:	vmovups %xmm0,(%r15)
  400e21:	jmp    400df2 <<zopro_core::solver::Plan>::run_poseidon_task+0x2522>
  400e23:	lea    0xe4126(%rip),%rdi        # 4e4f50 <__frame_dummy_init_array_entry+0x6930>
  400e2a:	vzeroupper
  400e2d:	call   f16f0 <core::option::unwrap_failed>
  400e32:	lea    0xe40ff(%rip),%rdi        # 4e4f38 <__frame_dummy_init_array_entry+0x6918>
  400e39:	vzeroupper
  400e3c:	call   f16f0 <core::option::unwrap_failed>
  400e41:	mov    0x18(%rsp),%rsi
  400e46:	mov    0x30(%rsp),%rdx
  400e4b:	lea    0xe412e(%rip),%rcx        # 4e4f80 <__frame_dummy_init_array_entry+0x6960>
  400e52:	xor    %edi,%edi
  400e54:	vzeroupper
  400e57:	call   f0300 <core::slice::index::slice_index_fail>
  400e5c:	mov    0x8(%rsp),%rsi
  400e61:	lea    0xe4100(%rip),%rcx        # 4e4f68 <__frame_dummy_init_array_entry+0x6948>
  400e68:	mov    $0xd,%edx
  400e6d:	xor    %edi,%edi
  400e6f:	vzeroupper
  400e72:	call   f0300 <core::slice::index::slice_index_fail>
  400e77:	lea    0xe405a(%rip),%rdx        # 4e4ed8 <__frame_dummy_init_array_entry+0x68b8>
  400e7e:	mov    %rbx,%rsi
  400e81:	vzeroupper
  400e84:	call   ef3ab <core::panicking::panic_bounds_check>
  400e89:	lea    0xe4060(%rip),%rdx        # 4e4ef0 <__frame_dummy_init_array_entry+0x68d0>
  400e90:	mov    %rbx,%rsi
  400e93:	vzeroupper
  400e96:	call   ef3ab <core::panicking::panic_bounds_check>
  400e9b:	lea    -0x3e05f9(%rip),%rdi        # 208a9 <zmij::STATIC_DATA+0x64a9>
  400ea2:	lea    0xe5067(%rip),%rdx        # 4e5f10 <__frame_dummy_init_array_entry+0x78f0>
  400ea9:	mov    $0x85,%esi
  400eae:	vzeroupper
  400eb1:	call   ee300 <core::panicking::panic_fmt>
  400eb6:	lea    0xe3f8b(%rip),%rdx        # 4e4e48 <__frame_dummy_init_array_entry+0x6828>
  400ebd:	vzeroupper
  400ec0:	call   ef3ab <core::panicking::panic_bounds_check>
  400ec5:	lea    0xe34b4(%rip),%rdx        # 4e4380 <__frame_dummy_init_array_entry+0x5d60>
  400ecc:	mov    %rax,%rdi
  400ecf:	vzeroupper
  400ed2:	call   ef3ab <core::panicking::panic_bounds_check>
  400ed7:	mov    0x30(%rsp),%rsi
  400edc:	lea    0xe403d(%rip),%rdx        # 4e4f20 <__frame_dummy_init_array_entry+0x6900>
  400ee3:	vzeroupper
  400ee6:	call   ef3ab <core::panicking::panic_bounds_check>
  400eeb:	mov    %r12,%r14
  400eee:	lea    0xe4013(%rip),%rdx        # 4e4f08 <__frame_dummy_init_array_entry+0x68e8>
  400ef5:	mov    0x40(%rsp),%rsi
  400efa:	mov    %r14,%rdi
  400efd:	vzeroupper
  400f00:	call   ef3ab <core::panicking::panic_bounds_check>
  400f05:	lea    0xe3fb4(%rip),%rdx        # 4e4ec0 <__frame_dummy_init_array_entry+0x68a0>
  400f0c:	mov    0x40(%rsp),%rsi
  400f11:	mov    %r14,%rdi
  400f14:	vzeroupper
  400f17:	call   ef3ab <core::panicking::panic_bounds_check>
  400f1c:	mov    0xb0(%rsp),%rdi
  400f24:	sub    %rbx,%rdi
  400f27:	mov    0x40(%rsp),%rsi
  400f2c:	lea    0xe3f2d(%rip),%rdx        # 4e4e60 <__frame_dummy_init_array_entry+0x6840>
  400f33:	vzeroupper
  400f36:	call   ef3ab <core::panicking::panic_bounds_check>
  400f3b:	mov    0x1e0(%rsp),%rdi
  400f43:	shr    $0x1c,%edi
  400f46:	add    0xb0(%rsp),%rdi
  400f4e:	sub    %r15,%rdi
  400f51:	mov    0x40(%rsp),%rsi
  400f56:	lea    0xe3f03(%rip),%rdx        # 4e4e60 <__frame_dummy_init_array_entry+0x6840>
  400f5d:	vzeroupper
  400f60:	call   ef3ab <core::panicking::panic_bounds_check>
  400f65:	mov    %r15,%rdi
  400f68:	mov    0x40(%rsp),%rsi
  400f6d:	lea    0xe3eec(%rip),%rdx        # 4e4e60 <__frame_dummy_init_array_entry+0x6840>
  400f74:	vzeroupper
  400f77:	call   ef3ab <core::panicking::panic_bounds_check>
  400f7c:	mov    %r14,%rdi
  400f7f:	mov    0x40(%rsp),%rsi
  400f84:	lea    0xe3ed5(%rip),%rdx        # 4e4e60 <__frame_dummy_init_array_entry+0x6840>
  400f8b:	vzeroupper
  400f8e:	call   ef3ab <core::panicking::panic_bounds_check>
  400f93:	lea    0xe40ee(%rip),%rdx        # 4e5088 <__frame_dummy_init_array_entry+0x6a68>
  400f9a:	call   ef3ab <core::panicking::panic_bounds_check>
  400f9f:	lea    0xe3ed2(%rip),%rdx        # 4e4e78 <__frame_dummy_init_array_entry+0x6858>
  400fa6:	mov    %rbx,%rsi
  400fa9:	vzeroupper
  400fac:	call   ef3ab <core::panicking::panic_bounds_check>
  400fb1:	lea    0xe3ef0(%rip),%rdx        # 4e4ea8 <__frame_dummy_init_array_entry+0x6888>
  400fb8:	mov    %rbx,%rsi
  400fbb:	vzeroupper
  400fbe:	call   ef3ab <core::panicking::panic_bounds_check>
  400fc3:	lea    0xe3ec6(%rip),%rdx        # 4e4e90 <__frame_dummy_init_array_entry+0x6870>
  400fca:	mov    %rax,%rdi
  400fcd:	mov    %rbx,%rsi
  400fd0:	vzeroupper
  400fd3:	call   ef3ab <core::panicking::panic_bounds_check>
  400fd8:	lea    0xe40c1(%rip),%rdx        # 4e50a0 <__frame_dummy_init_array_entry+0x6a80>
  400fdf:	mov    %r12,%rdi
  400fe2:	vzeroupper
  400fe5:	call   ef3ab <core::panicking::panic_bounds_check>
