
/home/ubuntu/src/zopro-perf-work/g2-gls4-census177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000486d90 <zopro_core::msm_affine::g2_borrowed::gls4::census::add>:
  486d90:	push   %rbp
  486d91:	push   %r15
  486d93:	push   %r14
  486d95:	push   %r13
  486d97:	push   %r12
  486d99:	push   %rbx
  486d9a:	sub    $0x168,%rsp
  486da1:	mov    0x10(%rdi),%rax
  486da5:	mov    %esi,%ebx
  486da7:	and    $0x7fffffff,%ebx
  486dad:	mov    %rcx,0x58(%rsp)
  486db2:	cmp    %rbx,%rax
  486db5:	jbe    48741a <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x68a>
  486dbb:	mov    %rdx,%r15
  486dbe:	mov    0x8(%rdi),%rdx
  486dc2:	mov    %rbx,%rax
  486dc5:	shl    $0x7,%rax
  486dc9:	xor    %r9d,%r9d
  486dcc:	mov    %rax,0x50(%rsp)
  486dd1:	vmovdqu (%rdx,%rax,1),%ymm0
  486dd6:	add    %rax,%rdx
  486dd9:	vptest %ymm0,%ymm0
  486dde:	setne  %r10b
  486de2:	jne    486e00 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x70>
  486de4:	vmovdqu 0x20(%rdx),%ymm0
  486de9:	vptest %ymm0,%ymm0
  486dee:	jne    486e00 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x70>
  486df0:	vmovdqu 0x40(%rdx),%ymm0
  486df5:	vptest %ymm0,%ymm0
  486dfa:	je     486f35 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x1a5>
  486e00:	movl   $0x0,0xc(%rsp)
  486e08:	mov    0x88(%rdi),%r13
  486e0f:	mov    0x40(%rdi),%rax
  486e13:	mov    %r13,0x98(%rsp)
  486e1b:	cmp    %rbx,%rax
  486e1e:	jbe    48742c <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x69c>
  486e24:	mov    0x38(%rdi),%rax
  486e28:	mov    %r10b,%r9b
  486e2b:	mov    (%rax,%rbx,4),%ebp
  486e2e:	test   %r9d,%r9d
  486e31:	jne    486e5b <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0xcb>
  486e33:	vmovdqu 0x20(%rdx),%ymm0
  486e38:	vptest %ymm0,%ymm0
  486e3d:	jne    486e5b <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0xcb>
  486e3f:	vmovdqu 0x40(%rdx),%ymm0
  486e44:	vptest %ymm0,%ymm0
  486e49:	jne    486e5b <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0xcb>
  486e4b:	vmovdqu 0x60(%rdx),%ymm0
  486e50:	vptest %ymm0,%ymm0
  486e55:	je     48720c <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x47c>
  486e5b:	vmovdqu (%rdx),%ymm0
  486e5f:	vpxor  (%r15),%ymm0,%ymm0
  486e64:	vptest %ymm0,%ymm0
  486e69:	jne    486e7d <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0xed>
  486e6b:	vmovdqu 0x20(%rdx),%ymm0
  486e70:	vpxor  0x20(%r15),%ymm0,%ymm0
  486e76:	vptest %ymm0,%ymm0
  486e7b:	je     486ef0 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x160>
  486e7d:	mov    0x70(%rdi),%r8
  486e81:	mov    %ebx,%eax
  486e83:	shr    $0x6,%eax
  486e86:	cmp    %r8,%rax
  486e89:	jae    487441 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x6b1>
  486e8f:	mov    0x68(%rdi),%r8
  486e93:	mov    $0x1,%r9d
  486e99:	shlx   %rbx,%r9,%r9
  486e9e:	or     %r9,(%r8,%rax,8)
  486ea2:	cmp    0x78(%rdi),%r13
  486ea6:	je     4871ee <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x45e>
  486eac:	mov    0x80(%rdi),%rax
  486eb3:	mov    %r13,%r8
  486eb6:	shl    $0x4,%r8
  486eba:	inc    %r13
  486ebd:	mov    %esi,(%rax,%r8,1)
  486ec1:	mov    %r15,0x8(%rax,%r8,1)
  486ec6:	mov    %r13,0x88(%rdi)
  486ecd:	mov    $0x1058,%eax
  486ed2:	mov    0x58(%rsp),%rcx
  486ed7:	incq   (%rcx,%rax,1)
  486edb:	add    $0x168,%rsp
  486ee2:	pop    %rbx
  486ee3:	pop    %r12
  486ee5:	pop    %r13
  486ee7:	pop    %r14
  486ee9:	pop    %r15
  486eeb:	pop    %rbp
  486eec:	vzeroupper
  486eef:	ret
  486ef0:	test   %esi,%esi
  486ef2:	js     486f4b <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x1bb>
  486ef4:	vmovups (%r15),%zmm0
  486efa:	vmovups 0x40(%r15),%zmm1
  486f01:	vmovups %zmm1,0x120(%rsp)
  486f0c:	vmovups %zmm0,0xe0(%rsp)
  486f17:	vmovdqu 0x40(%rdx),%ymm0
  486f1c:	vpxor  0x120(%rsp),%ymm0,%ymm0
  486f25:	vptest %ymm0,%ymm0
  486f2a:	jne    48715d <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x3cd>
  486f30:	jmp    487144 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x3b4>
  486f35:	vmovdqu 0x60(%rdx),%ymm0
  486f3a:	vptest %ymm0,%ymm0
  486f3f:	sete   %al
  486f42:	mov    %eax,0xc(%rsp)
  486f46:	jmp    486e08 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x78>
  486f4b:	vmovups 0x40(%r15),%ymm0
  486f51:	vmovdqu -0x474c79(%rip),%ymm2        # 122e0 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0xe0>
  486f59:	movq   $0x0,0x20(%rsp)
  486f62:	movq   $0x0,0x38(%rsp)
  486f6b:	movq   $0x0,0x30(%rsp)
  486f74:	mov    %rbx,0x90(%rsp)
  486f7c:	mov    %r15,0x88(%rsp)
  486f84:	movq   $0x0,0x10(%rsp)
  486f8d:	movq   $0x0,0x28(%rsp)
  486f96:	mov    %ebp,0x1c(%rsp)
  486f9a:	mov    %r15,%r8
  486f9d:	lea    0x60(%rsp),%rcx
  486fa2:	lea    0xa0(%rsp),%rbx
  486faa:	mov    0x38(%rsp),%r12
  486faf:	mov    0x30(%rsp),%r11
  486fb4:	mov    0x20(%rsp),%r14
  486fb9:	mov    0x28(%rsp),%r15
  486fbe:	vmovups %ymm0,0x60(%rsp)
  486fc4:	vmovdqu %ymm2,0xa0(%rsp)
  486fcd:	sub    (%rcx),%r14
  486fd0:	sbb    0x8(%rcx),%r15
  486fd4:	sbb    0x10(%rcx),%r12
  486fd8:	sbb    0x18(%rcx),%r11
  486fdc:	sbb    %r10,%r10
  486fdf:	mov    (%rbx),%r9
  486fe2:	mov    0x8(%rbx),%rsi
  486fe6:	mov    0x10(%rbx),%rbp
  486fea:	mov    0x18(%rbx),%rax
  486fee:	and    %r10,%r9
  486ff1:	and    %r10,%rsi
  486ff4:	and    %r10,%rbp
  486ff7:	and    %r10,%rax
  486ffa:	add    %r9,%r14
  486ffd:	adc    %rsi,%r15
  487000:	adc    %rbp,%r12
  487003:	adc    %rax,%r11
  487006:	movq   $0x0,0x48(%rsp)
  48700f:	mov    %r15,0x28(%rsp)
  487014:	movq   $0x0,0x40(%rsp)
  48701d:	mov    %r14,0x20(%rsp)
  487022:	mov    %r11,0x30(%rsp)
  487027:	mov    %r12,0x38(%rsp)
  48702c:	vmovdqu %ymm2,0xa0(%rsp)
  487035:	xor    %ecx,%ecx
  487037:	lea    0x60(%rsp),%r10
  48703c:	vmovups 0x60(%r8),%ymm1
  487042:	mov    0x90(%rsp),%rbx
  48704a:	mov    0x10(%rsp),%r9
  48704f:	mov    0x48(%rsp),%rbp
  487054:	mov    0x40(%rsp),%r15
  487059:	lea    0xa0(%rsp),%r8
  487061:	vmovups %ymm1,0x60(%rsp)
  487067:	sub    (%r10),%r9
  48706a:	sbb    0x8(%r10),%rbp
  48706e:	sbb    0x10(%r10),%r15
  487072:	sbb    0x18(%r10),%rcx
  487076:	sbb    %r12,%r12
  487079:	mov    (%r8),%rax
  48707c:	mov    0x8(%r8),%rsi
  487080:	mov    0x10(%r8),%r11
  487084:	mov    0x18(%r8),%r14
  487088:	and    %r12,%rax
  48708b:	and    %r12,%rsi
  48708e:	and    %r12,%r11
  487091:	and    %r12,%r14
  487094:	add    %rax,%r9
  487097:	adc    %rsi,%rbp
  48709a:	adc    %r11,%r15
  48709d:	adc    %r14,%rcx
  4870a0:	mov    %r9,0x10(%rsp)
  4870a5:	mov    %rbp,0x48(%rsp)
  4870aa:	mov    %r15,0x40(%rsp)
  4870af:	mov    0x88(%rsp),%rax
  4870b7:	mov    0x28(%rsp),%r8
  4870bc:	mov    0x38(%rsp),%rsi
  4870c1:	mov    0x1c(%rsp),%ebp
  4870c5:	vmovups (%rax),%zmm0
  4870cb:	mov    0x20(%rsp),%rax
  4870d0:	vmovups %zmm0,0xe0(%rsp)
  4870db:	mov    %rax,0x120(%rsp)
  4870e3:	mov    %r8,0x128(%rsp)
  4870eb:	mov    %rsi,0x130(%rsp)
  4870f3:	mov    0x30(%rsp),%r8
  4870f8:	mov    0x10(%rsp),%rsi
  4870fd:	mov    %r8,0x138(%rsp)
  487105:	mov    %rsi,0x140(%rsp)
  48710d:	mov    0x48(%rsp),%r8
  487112:	mov    0x40(%rsp),%rsi
  487117:	mov    %r8,0x148(%rsp)
  48711f:	mov    %rsi,0x150(%rsp)
  487127:	mov    %rcx,0x158(%rsp)
  48712f:	vmovdqu 0x40(%rdx),%ymm0
  487134:	vpxor  0x120(%rsp),%ymm0,%ymm0
  48713d:	vptest %ymm0,%ymm0
  487142:	jne    48715d <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x3cd>
  487144:	vmovdqu 0x60(%rdx),%ymm0
  487149:	vpxor  0x140(%rsp),%ymm0,%ymm0
  487152:	vptest %ymm0,%ymm0
  487157:	je     48722d <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x49d>
  48715d:	vpxor  %xmm0,%xmm0,%xmm0
  487161:	vmovdqu64 %zmm0,0x40(%rdx)
  487168:	vmovdqu64 %zmm0,(%rdx)
  48716e:	cmpb   $0x0,0xc(%rsp)
  487173:	mov    $0x1040,%eax
  487178:	jne    486ed2 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x142>
  48717e:	mov    0x10(%rdi),%rsi
  487182:	cmp    %rbx,%rsi
  487185:	jbe    487456 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x6c6>
  48718b:	mov    0x8(%rdi),%rax
  48718f:	mov    0x50(%rsp),%rcx
  487194:	vmovdqu (%rax,%rcx,1),%ymm0
  487199:	vptest %ymm0,%ymm0
  48719e:	jne    4871cd <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x43d>
  4871a0:	add    0x50(%rsp),%rax
  4871a5:	vmovdqu 0x20(%rax),%ymm0
  4871aa:	vptest %ymm0,%ymm0
  4871af:	jne    4871cd <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x43d>
  4871b1:	vmovdqu 0x40(%rax),%ymm0
  4871b6:	vptest %ymm0,%ymm0
  4871bb:	jne    4871cd <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x43d>
  4871bd:	vmovdqu 0x60(%rax),%ymm0
  4871c2:	vptest %ymm0,%ymm0
  4871c7:	je     487410 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x680>
  4871cd:	mov    0x58(%rsp),%rcx
  4871d2:	xor    %eax,%eax
  4871d4:	cmp    $0xffffffff,%ebp
  4871d7:	setne  %al
  4871da:	lea    0x1070(,%rax,8),%rax
  4871e2:	incq   0x1048(%rcx)
  4871e9:	jmp    486ed2 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x142>
  4871ee:	lea    0x78(%rdi),%rax
  4871f2:	mov    %rdi,%rbx
  4871f5:	mov    %esi,%ebp
  4871f7:	mov    %rax,%rdi
  4871fa:	vzeroupper
  4871fd:	call   2b63d0 <<alloc::raw_vec::RawVec<zopro_core::preparation::Reservation>>::grow_one>
  487202:	mov    %ebp,%esi
  487204:	mov    %rbx,%rdi
  487207:	jmp    486eac <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x11c>
  48720c:	test   %esi,%esi
  48720e:	js     48728e <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x4fe>
  487210:	vmovups (%r15),%zmm0
  487216:	vmovups %zmm0,0xa0(%rsp)
  487221:	vmovdqu64 0x40(%r15),%zmm0
  487228:	jmp    4873f3 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x663>
  48722d:	lea    0xe0(%rsp),%rdx
  487235:	mov    %rbx,%rsi
  487238:	mov    %rdi,%r14
  48723b:	vzeroupper
  48723e:	call   470420 <<zopro_core::msm_affine::g2_borrowed::Window>::projective>
  487243:	mov    0x88(%r14),%rdx
  48724a:	lea    0x1(%r13),%rsi
  48724e:	mov    $0x1058,%eax
  487253:	cmp    %rsi,%rdx
  487256:	je     486ed2 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x142>
  48725c:	mov    %r14,%rdi
  48725f:	mov    %rdx,0xa0(%rsp)
  487267:	cmp    %r13,%rdx
  48726a:	je     48716e <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x3de>
  487270:	lea    0x6ccb9(%rip),%r8        # 4f3f30 <__frame_dummy_init_array_entry+0x7a58>
  487277:	lea    0xa0(%rsp),%rdi
  48727f:	lea    0x98(%rsp),%rsi
  487287:	xor    %edx,%edx
  487289:	call   f1442 <core::panicking::assert_failed::<usize, usize>>
  48728e:	lea    0x40(%r15),%rax
  487292:	vmovups -0x474fba(%rip),%ymm4        # 122e0 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0xe0>
  48729a:	movq   $0x0,0x10(%rsp)
  4872a3:	mov    %ebp,0x1c(%rsp)
  4872a7:	mov    %r15,%r8
  4872aa:	lea    0x60(%rsp),%r12
  4872af:	lea    0xa0(%rsp),%r15
  4872b7:	xor    %r10d,%r10d
  4872ba:	xor    %r11d,%r11d
  4872bd:	xor    %ecx,%ecx
  4872bf:	xor    %r14d,%r14d
  4872c2:	vmovups (%rax),%ymm0
  4872c6:	vmovups %ymm4,0xa0(%rsp)
  4872cf:	vmovups %ymm0,0x60(%rsp)
  4872d5:	sub    (%r12),%r10
  4872d9:	sbb    0x8(%r12),%r11
  4872de:	sbb    0x10(%r12),%rcx
  4872e3:	sbb    0x18(%r12),%r14
  4872e8:	sbb    %rax,%rax
  4872eb:	mov    (%r15),%rbp
  4872ee:	mov    0x8(%r15),%r13
  4872f2:	mov    0x10(%r15),%rsi
  4872f6:	mov    0x18(%r15),%r9
  4872fa:	and    %rax,%rbp
  4872fd:	and    %rax,%r13
  487300:	and    %rax,%rsi
  487303:	and    %rax,%r9
  487306:	add    %rbp,%r10
  487309:	adc    %r13,%r11
  48730c:	adc    %rsi,%rcx
  48730f:	adc    %r9,%r14
  487312:	mov    %rcx,0x20(%rsp)
  487317:	mov    %r14,0x28(%rsp)
  48731c:	mov    %r10,0x30(%rsp)
  487321:	mov    %r11,0x38(%rsp)
  487326:	vmovups %ymm4,0xa0(%rsp)
  48732f:	mov    %r8,%rcx
  487332:	xor    %r12d,%r12d
  487335:	xor    %r13d,%r13d
  487338:	xor    %ebp,%ebp
  48733a:	vmovups 0x60(%r8),%ymm1
  487340:	mov    0x10(%rsp),%r14
  487345:	lea    0x60(%rsp),%r8
  48734a:	vmovups %ymm1,0x60(%rsp)
  487350:	sub    (%r8),%r12
  487353:	sbb    0x8(%r8),%r13
  487357:	sbb    0x10(%r8),%rbp
  48735b:	sbb    0x18(%r8),%r14
  48735f:	sbb    %r9,%r9
  487362:	mov    (%r15),%rax
  487365:	mov    0x8(%r15),%rsi
  487369:	mov    0x10(%r15),%r10
  48736d:	mov    0x18(%r15),%r11
  487371:	and    %r9,%rax
  487374:	and    %r9,%rsi
  487377:	and    %r9,%r10
  48737a:	and    %r9,%r11
  48737d:	add    %rax,%r12
  487380:	adc    %rsi,%r13
  487383:	adc    %r10,%rbp
  487386:	adc    %r11,%r14
  487389:	mov    %r14,0x10(%rsp)
  48738e:	vmovq  %r12,%xmm3
  487393:	vmovups (%rcx),%zmm0
  487399:	vmovq  0x20(%rsp),%xmm1
  48739f:	vmovq  0x30(%rsp),%xmm2
  4873a5:	vmovups %zmm0,0xa0(%rsp)
  4873b0:	vmovq  0x28(%rsp),%xmm0
  4873b6:	vpunpcklqdq %xmm0,%xmm1,%xmm0
  4873ba:	vmovq  0x38(%rsp),%xmm1
  4873c0:	vpunpcklqdq %xmm1,%xmm2,%xmm1
  4873c4:	vmovq  %rbp,%xmm2
  4873c9:	mov    0x1c(%rsp),%ebp
  4873cd:	vinserti128 $0x1,%xmm0,%ymm1,%ymm0
  4873d3:	vmovq  0x10(%rsp),%xmm1
  4873d9:	vpunpcklqdq %xmm1,%xmm2,%xmm1
  4873dd:	vmovq  %r13,%xmm2
  4873e2:	vpunpcklqdq %xmm2,%xmm3,%xmm2
  4873e6:	vinserti128 $0x1,%xmm1,%ymm2,%ymm1
  4873ec:	vinserti64x4 $0x1,%ymm1,%zmm0,%zmm0
  4873f3:	vmovups 0xa0(%rsp),%zmm1
  4873fe:	vmovups %zmm1,(%rdx)
  487404:	vmovdqu64 %zmm0,0x40(%rdx)
  48740b:	jmp    48716e <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x3de>
  487410:	mov    $0x1050,%eax
  487415:	jmp    486ed2 <zopro_core::msm_affine::g2_borrowed::gls4::census::add+0x142>
  48741a:	lea    0x6cadf(%rip),%rdx        # 4f3f00 <__frame_dummy_init_array_entry+0x7a28>
  487421:	mov    %rbx,%rdi
  487424:	mov    %rax,%rsi
  487427:	call   f109b <core::panicking::panic_bounds_check>
  48742c:	lea    0x6cae5(%rip),%rdx        # 4f3f18 <__frame_dummy_init_array_entry+0x7a40>
  487433:	mov    %rbx,%rdi
  487436:	mov    %rax,%rsi
  487439:	vzeroupper
  48743c:	call   f109b <core::panicking::panic_bounds_check>
  487441:	lea    0x6ad00(%rip),%rdx        # 4f2148 <__frame_dummy_init_array_entry+0x5c70>
  487448:	mov    %rax,%rdi
  48744b:	mov    %r8,%rsi
  48744e:	vzeroupper
  487451:	call   f109b <core::panicking::panic_bounds_check>
  487456:	lea    0x6caeb(%rip),%rdx        # 4f3f48 <__frame_dummy_init_array_entry+0x7a70>
  48745d:	mov    %rbx,%rdi
  487460:	vzeroupper
  487463:	call   f109b <core::panicking::panic_bounds_check>
