
/home/ubuntu/src/zopro-perf-work/g2-gls4-census177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000486be0 <zopro_core::msm_affine::g2_borrowed::gls4::census::flush>:
  486be0:	push   %rbp
  486be1:	push   %r15
  486be3:	push   %r14
  486be5:	push   %r13
  486be7:	push   %r12
  486be9:	push   %rbx
  486bea:	sub    $0xa8,%rsp
  486bf1:	mov    0x28(%rdi),%rax
  486bf5:	mov    0xa0(%rdi),%rcx
  486bfc:	vxorps %xmm0,%xmm0,%xmm0
  486c00:	vmovups %zmm0,0x60(%rsp)
  486c0b:	vmovups %zmm0,0x20(%rsp)
  486c16:	movq   $0x0,(%rsp)
  486c1e:	mov    %rdi,0x10(%rsp)
  486c23:	mov    %rax,0x8(%rsp)
  486c28:	test   %rcx,%rcx
  486c2b:	je     486ced <zopro_core::msm_affine::g2_borrowed::gls4::census::flush+0x10d>
  486c31:	mov    0x1038(%rsi),%r10
  486c38:	mov    0x98(%rdi),%rdx
  486c3f:	mov    0x38(%rdi),%r8
  486c43:	mov    0x40(%rdi),%rax
  486c47:	mov    0x1078(%rsi),%r9
  486c4e:	mov    0x1070(%rsi),%r11
  486c55:	shl    $0x4,%rcx
  486c59:	xor    %r15d,%r15d
  486c5c:	xor    %r13d,%r13d
  486c5f:	inc    %r10
  486c62:	jmp    486c95 <zopro_core::msm_affine::g2_borrowed::gls4::census::flush+0xb5>
  486c64:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  486c70:	inc    %r9
  486c73:	mov    %r9,0x1078(%rsi)
  486c7a:	or     %rbp,%r14
  486c7d:	mov    %r10,0x1038(%rsi)
  486c84:	inc    %r10
  486c87:	add    $0x10,%r13
  486c8b:	mov    %r14,0x20(%rsp,%rdi,8)
  486c90:	cmp    %r13,%rcx
  486c93:	je     486cf0 <zopro_core::msm_affine::g2_borrowed::gls4::census::flush+0x110>
  486c95:	mov    (%rdx,%r13,1),%edi
  486c99:	and    $0x7fffffff,%edi
  486c9f:	cmp    $0x400,%edi
  486ca5:	jae    486d2f <zopro_core::msm_affine::g2_borrowed::gls4::census::flush+0x14f>
  486cab:	cmp    %rdi,%rax
  486cae:	jbe    486d78 <zopro_core::msm_affine::g2_borrowed::gls4::census::flush+0x198>
  486cb4:	cmpl   $0xffffffff,(%r8,%rdi,4)
  486cb9:	mov    $0x1,%ebx
  486cbe:	shlx   %rdi,%rbx,%rbp
  486cc3:	sete   %bl
  486cc6:	shr    $0x6,%edi
  486cc9:	mov    0x20(%rsp,%rdi,8),%r14
  486cce:	test   %rbp,%r14
  486cd1:	sete   %r12b
  486cd5:	test   %r12b,%bl
  486cd8:	je     486c70 <zopro_core::msm_affine::g2_borrowed::gls4::census::flush+0x90>
  486cda:	inc    %r11
  486cdd:	inc    %r15
  486ce0:	mov    %r11,0x1070(%rsi)
  486ce7:	mov    %r15,(%rsp)
  486ceb:	jmp    486c7a <zopro_core::msm_affine::g2_borrowed::gls4::census::flush+0x9a>
  486ced:	xor    %r15d,%r15d
  486cf0:	mov    0x10(%rsp),%rbx
  486cf5:	mov    %rbx,%rdi
  486cf8:	vzeroupper
  486cfb:	call   470200 <<zopro_core::msm_affine::g2_borrowed::Window>::flush_queue>
  486d00:	cmpq   $0x0,0xa0(%rbx)
  486d08:	jne    486d4a <zopro_core::msm_affine::g2_borrowed::gls4::census::flush+0x16a>
  486d0a:	mov    0x28(%rbx),%rax
  486d0e:	sub    0x8(%rsp),%rax
  486d13:	mov    %rax,0x18(%rsp)
  486d18:	cmp    %r15,%rax
  486d1b:	jne    486d62 <zopro_core::msm_affine::g2_borrowed::gls4::census::flush+0x182>
  486d1d:	add    $0xa8,%rsp
  486d24:	pop    %rbx
  486d25:	pop    %r12
  486d27:	pop    %r13
  486d29:	pop    %r14
  486d2b:	pop    %r15
  486d2d:	pop    %rbp
  486d2e:	ret
  486d2f:	lea    -0x3fbf7e(%rip),%rdi        # 8adb8 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x100>
  486d36:	lea    0x6d223(%rip),%rdx        # 4f3f60 <__frame_dummy_init_array_entry+0x7a88>
  486d3d:	mov    $0x1a,%esi
  486d42:	vzeroupper
  486d45:	call   f20c0 <core::panicking::panic>
  486d4a:	lea    -0x3fbf7f(%rip),%rdi        # 8add2 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x11a>
  486d51:	lea    0x6d250(%rip),%rdx        # 4f3fa8 <__frame_dummy_init_array_entry+0x7ad0>
  486d58:	mov    $0x28,%esi
  486d5d:	call   f20c0 <core::panicking::panic>
  486d62:	lea    0x6d227(%rip),%r8        # 4f3f90 <__frame_dummy_init_array_entry+0x7ab8>
  486d69:	lea    0x18(%rsp),%rdi
  486d6e:	mov    %rsp,%rsi
  486d71:	xor    %edx,%edx
  486d73:	call   f1442 <core::panicking::assert_failed::<usize, usize>>
  486d78:	lea    0x6d1f9(%rip),%rdx        # 4f3f78 <__frame_dummy_init_array_entry+0x7aa0>
  486d7f:	mov    %rax,%rsi
  486d82:	vzeroupper
  486d85:	call   f109b <core::panicking::panic_bounds_check>
