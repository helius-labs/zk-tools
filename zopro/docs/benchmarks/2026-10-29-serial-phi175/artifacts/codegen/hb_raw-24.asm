
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000465da0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_>:
  465da0:	push   %rbp
  465da1:	push   %r15
  465da3:	push   %r14
  465da5:	push   %r13
  465da7:	push   %r12
  465da9:	push   %rbx
  465daa:	sub    $0x1000,%rsp
  465db1:	movq   $0x0,(%rsp)
  465db9:	sub    $0x308,%rsp
  465dc0:	mov    (%rsi),%rax
  465dc3:	mov    %rdx,%r9
  465dc6:	mov    0x10(%rsi),%rdx
  465dca:	mov    %rdi,0xa8(%rsp)
  465dd2:	mov    (%rax),%rax
  465dd5:	imul   %rax,%r9
  465dd9:	cmp    %r9,%rdx
  465ddc:	cmovb  %rdx,%r9
  465de0:	lea    (%r9,%rax,1),%r10
  465de4:	cmp    %r10,%rdx
  465de7:	cmovb  %rdx,%r10
  465deb:	add    %r9,%rax
  465dee:	jb     46a388 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x45e8>
  465df4:	mov    0x20(%rsi),%rdx
  465df8:	cmp    %rdx,%r10
  465dfb:	ja     46a39a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x45fa>
  465e01:	mov    0x30(%rsi),%rax
  465e05:	mov    (%rax),%rbp
  465e08:	test   %rbp,%rbp
  465e0b:	je     46a463 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x46c3>
  465e11:	mov    0x28(%rsi),%rax
  465e15:	mov    0x8(%rsi),%r12
  465e19:	mov    0x18(%rsi),%rdi
  465e1d:	mov    %r9,%rsi
  465e20:	mov    %r9,%r8
  465e23:	shl    $0x7,%rsi
  465e27:	shl    $0x5,%r8
  465e2b:	mov    (%rax),%rax
  465e2e:	lea    0x1(%rax),%rcx
  465e32:	mov    %rax,0x70(%rsp)
  465e37:	cmp    $0x7f,%rbp
  465e3b:	ja     465e48 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xa8>
  465e3d:	mov    $0x7f,%eax
  465e42:	xor    %edx,%edx
  465e44:	div    %ebp
  465e46:	jmp    465e4f <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xaf>
  465e48:	mov    $0x7f,%edx
  465e4d:	xor    %eax,%eax
  465e4f:	add    %rsi,%r12
  465e52:	mov    %r10,%rsi
  465e55:	sub    %r9,%rsi
  465e58:	cmp    $0x1,%rdx
  465e5c:	lea    (%rdi,%r8,1),%r13
  465e60:	mov    %rbp,0x78(%rsp)
  465e65:	sbb    $0xffffffffffffffff,%rax
  465e69:	mov    %rsi,0x98(%rsp)
  465e71:	mov    %r12,0x80(%rsp)
  465e79:	cmp    %rax,%rcx
  465e7c:	jne    4667ec <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xa4c>
  465e82:	mov    %rbp,%rax
  465e85:	imul   0x70(%rsp),%rax
  465e8b:	mov    $0x7e,%r11d
  465e91:	sub    %rax,%r11
  465e94:	cmp    $0x9,%r11
  465e98:	jae    4667ec <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xa4c>
  465e9e:	sub    $0x8,%rsp
  465ea2:	mov    0xa0(%rsp),%rdx
  465eaa:	lea    0x108(%rsp),%rdi
  465eb2:	mov    %r12,%rsi
  465eb5:	mov    %r13,0x68(%rsp)
  465eba:	mov    %r13,%rcx
  465ebd:	mov    $0x1,%r9d
  465ec3:	mov    %r11,0x60(%rsp)
  465ec8:	mov    %rdx,%r8
  465ecb:	push   %r11
  465ecd:	push   %rbp
  465ece:	push   0x88(%rsp)
  465ed5:	call   46b920 <_RNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi16hb_top_component>
  465eda:	add    $0x20,%rsp
  465ede:	mov    0x108(%rsp),%rax
  465ee6:	mov    0x110(%rsp),%rcx
  465eee:	mov    %rax,0x68(%rsp)
  465ef3:	mov    %rcx,0x50(%rsp)
  465ef8:	test   %rcx,%rcx
  465efb:	je     46672a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x98a>
  465f01:	mov    0x50(%rsp),%rax
  465f06:	mov    0x68(%rsp),%r13
  465f0b:	shl    $0x7,%rax
  465f0f:	nop
  465f10:	vmovdqu 0x0(%r13),%ymm0
  465f16:	vptest %ymm0,%ymm0
  465f1b:	jne    465f50 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1b0>
  465f1d:	vmovdqu 0x20(%r13),%ymm0
  465f23:	vptest %ymm0,%ymm0
  465f28:	jne    465f50 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1b0>
  465f2a:	vmovdqu 0x40(%r13),%ymm0
  465f30:	vptest %ymm0,%ymm0
  465f35:	jne    465f50 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1b0>
  465f37:	vmovdqu 0x60(%r13),%ymm0
  465f3d:	vptest %ymm0,%ymm0
  465f42:	je     46671c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x97c>
  465f48:	nopl   0x0(%rax,%rax,1)
  465f50:	vmovdqu 0x0(%r13),%ymm0
  465f56:	vmovups -0x4542de(%rip),%ymm3        # 11c80 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0x200>
  465f5e:	vmovups -0x454406(%rip),%ymm1        # 11b60 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0xe0>
  465f66:	mov    %rax,0x8(%rsp)
  465f6b:	lea    0xde0(%rsp),%rbx
  465f73:	lea    0x7d0(%rsp),%r14
  465f7b:	lea    0x1c0(%rsp),%r15
  465f83:	movabs $0x87d20782e4866389,%r12
  465f8d:	lea    0xde0(%rsp),%rbp
  465f95:	vmovdqu %ymm0,0xde0(%rsp)
  465f9e:	vmovups %ymm3,0x7d0(%rsp)
  465fa7:	vmovups %ymm1,0x1c0(%rsp)
  465fb0:	xor    %r9d,%r9d
  465fb3:	xor    %r8d,%r8d
  465fb6:	xor    %edi,%edi
  465fb8:	xor    %eax,%eax
  465fba:	xor    %r10d,%r10d
  465fbd:	mov    (%r14),%rdx
  465fc0:	xor    %r11d,%r11d
  465fc3:	mulx   (%rbx),%rcx,%rsi
  465fc8:	adcx   %rcx,%r9
  465fce:	adox   %rsi,%r8
  465fd4:	mulx   0x8(%rbx),%rcx,%rsi
  465fda:	adcx   %rcx,%r8
  465fe0:	adox   %rsi,%rdi
  465fe6:	mulx   0x10(%rbx),%rcx,%rsi
  465fec:	adcx   %rcx,%rdi
  465ff2:	adox   %rsi,%rax
  465ff8:	mulx   0x18(%rbx),%rcx,%rsi
  465ffe:	adcx   %rcx,%rax
  466004:	adox   %rsi,%r10
  46600a:	adcx   %r11,%r10
  466010:	mov    %r9,%rdx
  466013:	imul   %r12,%rdx
  466017:	xor    %r11d,%r11d
  46601a:	mulx   (%r15),%rcx,%rsi
  46601f:	adcx   %rcx,%r9
  466025:	adox   %rsi,%r8
  46602b:	mulx   0x8(%r15),%rcx,%rsi
  466031:	adcx   %rcx,%r8
  466037:	adox   %rsi,%rdi
  46603d:	mulx   0x10(%r15),%rcx,%rsi
  466043:	adcx   %rcx,%rdi
  466049:	adox   %rsi,%rax
  46604f:	mulx   0x18(%r15),%rcx,%rsi
  466055:	adcx   %rcx,%rax
  46605b:	adox   %rsi,%r10
  466061:	adcx   %r11,%r10
  466067:	mov    0x8(%r14),%rdx
  46606b:	xor    %r11d,%r11d
  46606e:	mulx   (%rbx),%rcx,%rsi
  466073:	adcx   %rcx,%r8
  466079:	adox   %rsi,%rdi
  46607f:	mulx   0x8(%rbx),%rcx,%rsi
  466085:	adcx   %rcx,%rdi
  46608b:	adox   %rsi,%rax
  466091:	mulx   0x10(%rbx),%rcx,%rsi
  466097:	adcx   %rcx,%rax
  46609d:	adox   %rsi,%r10
  4660a3:	mulx   0x18(%rbx),%rcx,%rsi
  4660a9:	adcx   %rcx,%r10
  4660af:	adox   %rsi,%r9
  4660b5:	adcx   %r11,%r9
  4660bb:	mov    %r8,%rdx
  4660be:	imul   %r12,%rdx
  4660c2:	xor    %r11d,%r11d
  4660c5:	mulx   (%r15),%rcx,%rsi
  4660ca:	adcx   %rcx,%r8
  4660d0:	adox   %rsi,%rdi
  4660d6:	mulx   0x8(%r15),%rcx,%rsi
  4660dc:	adcx   %rcx,%rdi
  4660e2:	adox   %rsi,%rax
  4660e8:	mulx   0x10(%r15),%rcx,%rsi
  4660ee:	adcx   %rcx,%rax
  4660f4:	adox   %rsi,%r10
  4660fa:	mulx   0x18(%r15),%rcx,%rsi
  466100:	adcx   %rcx,%r10
  466106:	adox   %rsi,%r9
  46610c:	adcx   %r11,%r9
  466112:	mov    0x10(%r14),%rdx
  466116:	xor    %r11d,%r11d
  466119:	mulx   (%rbx),%rcx,%rsi
  46611e:	adcx   %rcx,%rdi
  466124:	adox   %rsi,%rax
  46612a:	mulx   0x8(%rbx),%rcx,%rsi
  466130:	adcx   %rcx,%rax
  466136:	adox   %rsi,%r10
  46613c:	mulx   0x10(%rbx),%rcx,%rsi
  466142:	adcx   %rcx,%r10
  466148:	adox   %rsi,%r9
  46614e:	mulx   0x18(%rbx),%rcx,%rsi
  466154:	adcx   %rcx,%r9
  46615a:	adox   %rsi,%r8
  466160:	adcx   %r11,%r8
  466166:	mov    %rdi,%rdx
  466169:	imul   %r12,%rdx
  46616d:	xor    %r11d,%r11d
  466170:	mulx   (%r15),%rcx,%rsi
  466175:	adcx   %rcx,%rdi
  46617b:	adox   %rsi,%rax
  466181:	mulx   0x8(%r15),%rcx,%rsi
  466187:	adcx   %rcx,%rax
  46618d:	adox   %rsi,%r10
  466193:	mulx   0x10(%r15),%rcx,%rsi
  466199:	adcx   %rcx,%r10
  46619f:	adox   %rsi,%r9
  4661a5:	mulx   0x18(%r15),%rcx,%rsi
  4661ab:	adcx   %rcx,%r9
  4661b1:	adox   %rsi,%r8
  4661b7:	adcx   %r11,%r8
  4661bd:	mov    0x18(%r14),%rdx
  4661c1:	xor    %r11d,%r11d
  4661c4:	mulx   (%rbx),%rcx,%rsi
  4661c9:	adcx   %rcx,%rax
  4661cf:	adox   %rsi,%r10
  4661d5:	mulx   0x8(%rbx),%rcx,%rsi
  4661db:	adcx   %rcx,%r10
  4661e1:	adox   %rsi,%r9
  4661e7:	mulx   0x10(%rbx),%rcx,%rsi
  4661ed:	adcx   %rcx,%r9
  4661f3:	adox   %rsi,%r8
  4661f9:	mulx   0x18(%rbx),%rcx,%rsi
  4661ff:	adcx   %rcx,%r8
  466205:	adox   %rsi,%rdi
  46620b:	adcx   %r11,%rdi
  466211:	mov    %rax,%rdx
  466214:	imul   %r12,%rdx
  466218:	xor    %r11d,%r11d
  46621b:	mulx   (%r15),%rcx,%rsi
  466220:	adcx   %rcx,%rax
  466226:	adox   %rsi,%r10
  46622c:	mulx   0x8(%r15),%rcx,%rsi
  466232:	adcx   %rcx,%r10
  466238:	adox   %rsi,%r9
  46623e:	mulx   0x10(%r15),%rcx,%rsi
  466244:	adcx   %rcx,%r9
  46624a:	adox   %rsi,%r8
  466250:	mulx   0x18(%r15),%rcx,%rsi
  466256:	adcx   %rcx,%r8
  46625c:	adox   %rsi,%rdi
  466262:	adcx   %r11,%rdi
  466268:	movabs $0x687e956e978e3573,%rax
  466272:	movabs $0x97816a916871ca8d,%rdx
  46627c:	mov    %r9,0x10(%rsp)
  466281:	mov    %rdi,0x18(%rsp)
  466286:	mov    %r10,0x20(%rsp)
  46628b:	mov    %r8,0x28(%rsp)
  466290:	lea    0x7d0(%rsp),%rsi
  466298:	lea    (%r9,%rax,1),%rcx
  46629c:	movabs $0x3c208c16d87cfd47,%rax
  4662a6:	vmovups 0x20(%r13),%ymm2
  4662ac:	vmovups %ymm3,0x7d0(%rsp)
  4662b5:	vmovups %ymm1,0x1c0(%rsp)
  4662be:	cmp    %rax,%r10
  4662c1:	sbb    $0x0,%rcx
  4662c5:	setb   %al
  4662c8:	cmp    %rdx,%r9
  4662cb:	mov    %rcx,0x48(%rsp)
  4662d0:	movabs $0xb85045b68181585d,%r9
  4662da:	setb   %dl
  4662dd:	or     %al,%dl
  4662df:	movabs $0x47afba497e7ea7a3,%rax
  4662e9:	lea    (%r8,%rax,1),%rcx
  4662ed:	movzbl %dl,%eax
  4662f0:	sub    %rax,%rcx
  4662f3:	setb   %al
  4662f6:	cmp    %r9,%r8
  4662f9:	mov    %rcx,0x38(%rsp)
  4662fe:	vmovups %ymm2,0xde0(%rsp)
  466307:	lea    0x1c0(%rsp),%r9
  46630f:	movabs $0x87d20782e4866389,%rcx
  466319:	setb   %dl
  46631c:	or     %al,%dl
  46631e:	movabs $0xcf9bb18d1ece5fd7,%rax
  466328:	lea    (%rdi,%rax,1),%r11
  46632c:	movzbl %dl,%eax
  46632f:	sub    %rax,%r11
  466332:	setb   0x30(%rsp)
  466337:	xor    %r15d,%r15d
  46633a:	xor    %ebx,%ebx
  46633c:	xor    %r12d,%r12d
  46633f:	xor    %eax,%eax
  466341:	xor    %r10d,%r10d
  466344:	mov    (%rsi),%rdx
  466347:	xor    %r8d,%r8d
  46634a:	mulx   0x0(%rbp),%r14,%rdi
  466350:	adcx   %r14,%r15
  466356:	adox   %rdi,%rbx
  46635c:	mulx   0x8(%rbp),%r14,%rdi
  466362:	adcx   %r14,%rbx
  466368:	adox   %rdi,%r12
  46636e:	mulx   0x10(%rbp),%r14,%rdi
  466374:	adcx   %r14,%r12
  46637a:	adox   %rdi,%rax
  466380:	mulx   0x18(%rbp),%r14,%rdi
  466386:	adcx   %r14,%rax
  46638c:	adox   %rdi,%r10
  466392:	adcx   %r8,%r10
  466398:	mov    %r15,%rdx
  46639b:	imul   %rcx,%rdx
  46639f:	xor    %r8d,%r8d
  4663a2:	mulx   (%r9),%r14,%rdi
  4663a7:	adcx   %r14,%r15
  4663ad:	adox   %rdi,%rbx
  4663b3:	mulx   0x8(%r9),%r14,%rdi
  4663b9:	adcx   %r14,%rbx
  4663bf:	adox   %rdi,%r12
  4663c5:	mulx   0x10(%r9),%r14,%rdi
  4663cb:	adcx   %r14,%r12
  4663d1:	adox   %rdi,%rax
  4663d7:	mulx   0x18(%r9),%r14,%rdi
  4663dd:	adcx   %r14,%rax
  4663e3:	adox   %rdi,%r10
  4663e9:	adcx   %r8,%r10
  4663ef:	mov    0x8(%rsi),%rdx
  4663f3:	xor    %r8d,%r8d
  4663f6:	mulx   0x0(%rbp),%r14,%rdi
  4663fc:	adcx   %r14,%rbx
  466402:	adox   %rdi,%r12
  466408:	mulx   0x8(%rbp),%r14,%rdi
  46640e:	adcx   %r14,%r12
  466414:	adox   %rdi,%rax
  46641a:	mulx   0x10(%rbp),%r14,%rdi
  466420:	adcx   %r14,%rax
  466426:	adox   %rdi,%r10
  46642c:	mulx   0x18(%rbp),%r14,%rdi
  466432:	adcx   %r14,%r10
  466438:	adox   %rdi,%r15
  46643e:	adcx   %r8,%r15
  466444:	mov    %rbx,%rdx
  466447:	imul   %rcx,%rdx
  46644b:	xor    %r8d,%r8d
  46644e:	mulx   (%r9),%r14,%rdi
  466453:	adcx   %r14,%rbx
  466459:	adox   %rdi,%r12
  46645f:	mulx   0x8(%r9),%r14,%rdi
  466465:	adcx   %r14,%r12
  46646b:	adox   %rdi,%rax
  466471:	mulx   0x10(%r9),%r14,%rdi
  466477:	adcx   %r14,%rax
  46647d:	adox   %rdi,%r10
  466483:	mulx   0x18(%r9),%r14,%rdi
  466489:	adcx   %r14,%r10
  46648f:	adox   %rdi,%r15
  466495:	adcx   %r8,%r15
  46649b:	mov    0x10(%rsi),%rdx
  46649f:	xor    %r8d,%r8d
  4664a2:	mulx   0x0(%rbp),%r14,%rdi
  4664a8:	adcx   %r14,%r12
  4664ae:	adox   %rdi,%rax
  4664b4:	mulx   0x8(%rbp),%r14,%rdi
  4664ba:	adcx   %r14,%rax
  4664c0:	adox   %rdi,%r10
  4664c6:	mulx   0x10(%rbp),%r14,%rdi
  4664cc:	adcx   %r14,%r10
  4664d2:	adox   %rdi,%r15
  4664d8:	mulx   0x18(%rbp),%r14,%rdi
  4664de:	adcx   %r14,%r15
  4664e4:	adox   %rdi,%rbx
  4664ea:	adcx   %r8,%rbx
  4664f0:	mov    %r12,%rdx
  4664f3:	imul   %rcx,%rdx
  4664f7:	xor    %r8d,%r8d
  4664fa:	mulx   (%r9),%r14,%rdi
  4664ff:	adcx   %r14,%r12
  466505:	adox   %rdi,%rax
  46650b:	mulx   0x8(%r9),%r14,%rdi
  466511:	adcx   %r14,%rax
  466517:	adox   %rdi,%r10
  46651d:	mulx   0x10(%r9),%r14,%rdi
  466523:	adcx   %r14,%r10
  466529:	adox   %rdi,%r15
  46652f:	mulx   0x18(%r9),%r14,%rdi
  466535:	adcx   %r14,%r15
  46653b:	adox   %rdi,%rbx
  466541:	adcx   %r8,%rbx
  466547:	mov    0x18(%rsi),%rdx
  46654b:	xor    %r8d,%r8d
  46654e:	mulx   0x0(%rbp),%r14,%rdi
  466554:	adcx   %r14,%rax
  46655a:	adox   %rdi,%r10
  466560:	mulx   0x8(%rbp),%r14,%rdi
  466566:	adcx   %r14,%r10
  46656c:	adox   %rdi,%r15
  466572:	mulx   0x10(%rbp),%r14,%rdi
  466578:	adcx   %r14,%r15
  46657e:	adox   %rdi,%rbx
  466584:	mulx   0x18(%rbp),%r14,%rdi
  46658a:	adcx   %r14,%rbx
  466590:	adox   %rdi,%r12
  466596:	adcx   %r8,%r12
  46659c:	mov    %rax,%rdx
  46659f:	imul   %rcx,%rdx
  4665a3:	xor    %r8d,%r8d
  4665a6:	mulx   (%r9),%r14,%rdi
  4665ab:	adcx   %r14,%rax
  4665b1:	adox   %rdi,%r10
  4665b7:	mulx   0x8(%r9),%r14,%rdi
  4665bd:	adcx   %r14,%r10
  4665c3:	adox   %rdi,%r15
  4665c9:	mulx   0x10(%r9),%r14,%rdi
  4665cf:	adcx   %r14,%r15
  4665d5:	adox   %rdi,%rbx
  4665db:	mulx   0x18(%r9),%r14,%rdi
  4665e1:	adcx   %r14,%rbx
  4665e7:	adox   %rdi,%r12
  4665ed:	adcx   %r8,%r12
  4665f3:	movabs $0x687e956e978e3573,%rax
  4665fd:	movabs $0x97816a916871ca8d,%rdx
  466607:	mov    %r12,0x40(%rsp)
  46660c:	mov    %r10,%r12
  46660f:	movabs $0xb85045b68181585d,%rcx
  466619:	movabs $0x30644e72e131a029,%r9
  466623:	mov    %rbx,%rbp
  466626:	lea    (%r15,%rax,1),%rsi
  46662a:	movabs $0x3c208c16d87cfd47,%rax
  466634:	cmp    %rax,%r10
  466637:	mov    0x40(%rsp),%r10
  46663c:	sbb    $0x0,%rsi
  466640:	setb   %al
  466643:	cmp    %rdx,%r15
  466646:	mov    0x18(%rsp),%rdx
  46664b:	setb   %dil
  46664f:	or     %al,%dil
  466652:	movabs $0x47afba497e7ea7a3,%rax
  46665c:	lea    (%rbx,%rax,1),%r14
  466660:	movzbl %dil,%eax
  466664:	sub    %rax,%r14
  466667:	setb   %al
  46666a:	cmp    %rcx,%rbx
  46666d:	movabs $0xcf9bb18d1ece5fd7,%rcx
  466677:	mov    0x38(%rsp),%rbx
  46667c:	setb   %dil
  466680:	add    %r10,%rcx
  466683:	or     %al,%dil
  466686:	mov    0x20(%rsp),%rax
  46668b:	movzbl %dil,%edi
  46668f:	sub    %rdi,%rcx
  466692:	setb   %dil
  466696:	cmp    %r9,%r10
  466699:	setb   %r8b
  46669d:	or     %dil,%r8b
  4666a0:	cmp    %r9,%rdx
  4666a3:	movabs $0xc3df73e9278302b9,%r9
  4666ad:	setb   %dil
  4666b1:	or     0x30(%rsp),%dil
  4666b6:	lea    (%rax,%r9,1),%rdi
  4666ba:	cmovne %rax,%rdi
  4666be:	mov    0x48(%rsp),%rax
  4666c3:	cmovne 0x28(%rsp),%rbx
  4666c9:	cmovne %rdx,%r11
  4666cd:	cmovne 0x10(%rsp),%rax
  4666d3:	test   %r8b,%r8b
  4666d6:	lea    (%r12,%r9,1),%r8
  4666da:	mov    %rdi,0x0(%r13)
  4666de:	cmovne %r12,%r8
  4666e2:	cmovne %rbp,%r14
  4666e6:	mov    0x80(%rsp),%r12
  4666ee:	mov    0x78(%rsp),%rbp
  4666f3:	cmovne %r15,%rsi
  4666f7:	cmovne %r10,%rcx
  4666fb:	mov    %rax,0x8(%r13)
  4666ff:	mov    0x8(%rsp),%rax
  466704:	mov    %rbx,0x10(%r13)
  466708:	mov    %r11,0x18(%r13)
  46670c:	mov    %r8,0x20(%r13)
  466710:	mov    %rsi,0x28(%r13)
  466714:	mov    %r14,0x30(%r13)
  466718:	mov    %rcx,0x38(%r13)
  46671c:	sub    $0xffffffffffffff80,%r13
  466720:	add    $0xffffffffffffff80,%rax
  466724:	jne    465f10 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x170>
  46672a:	sub    $0x8,%rsp
  46672e:	mov    0xa0(%rsp),%rdx
  466736:	mov    0x68(%rsp),%rcx
  46673b:	lea    0x7d8(%rsp),%rdi
  466743:	mov    %r12,%rsi
  466746:	xor    %r9d,%r9d
  466749:	mov    %rdx,%r8
  46674c:	push   0x60(%rsp)
  466750:	push   %rbp
  466751:	push   0x88(%rsp)
  466758:	vzeroupper
  46675b:	call   46b920 <_RNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi16hb_top_component>
  466760:	add    $0x20,%rsp
  466764:	mov    0x7e0(%rsp),%rax
  46676c:	mov    0x50(%rsp),%rbx
  466771:	mov    0x7d8(%rsp),%r14
  466779:	mov    %rbx,0xde0(%rsp)
  466781:	mov    %rax,0x1c0(%rsp)
  466789:	cmp    %rax,%rbx
  46678c:	jne    46a3c9 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4629>
  466792:	mov    0x68(%rsp),%rax
  466797:	mov    %rax,0x1c0(%rsp)
  46679f:	mov    %fs:0xffffffffffffffc0,%rax
  4667a8:	mov    %rbx,0x1c8(%rsp)
  4667b0:	movq   $0x200,0x1d0(%rsp)
  4667bc:	mov    %r14,0x1d8(%rsp)
  4667c4:	mov    %rbx,0x1e0(%rsp)
  4667cc:	movq   $0x200,0x1e8(%rsp)
  4667d8:	test   %rax,%rax
  4667db:	je     46693a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xb9a>
  4667e1:	add    $0x110,%rax
  4667e7:	jmp    46693f <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xb9f>
  4667ec:	mov    %r10,0xc0(%rsp)
  4667f4:	mov    %r9,0xc8(%rsp)
  4667fc:	mov    %r8,0xb0(%rsp)
  466804:	mov    %rdi,0xb8(%rsp)
  46680c:	lea    0xde0(%rsp),%rdi
  466814:	call   24fda0 <_RNvXs0_NtCsHW5vo2gROj_10zopro_core3msmINtB5_8BatchAddNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigENtNtCsgxBkk5gSRhY_4core7default7Default7defaultB7_>
  466819:	cmpb   $0x1,0x1300(%rsp)
  466821:	jne    466a0a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc6a>
  466827:	cmpb   $0x0,0x12f9(%rsp)
  46682f:	je     466a0a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc6a>
  466835:	cmpb   $0x0,0x12f8(%rsp)
  46683d:	je     466a0a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc6a>
  466843:	cmpb   $0x0,0x12fa(%rsp)
  46684b:	je     466a0a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc6a>
  466851:	lea    -0x45b378(%rip),%rsi        # b4e0 <__abi_tag+0xb1e4>
  466858:	lea    0x1c0(%rsp),%rdi
  466860:	mov    $0x10,%edx
  466865:	call   142ea0 <_RNvNtNtNtCs9k3SxhrAWiO_3std3sys3env4unix6getenv>
  46686a:	mov    0x1c0(%rsp),%r14
  466872:	cmp    $0xffffffffffffffff,%r14
  466876:	je     46894b <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2bab>
  46687c:	mov    0x1c8(%rsp),%r15
  466884:	mov    0x1d0(%rsp),%r12
  46688c:	lea    0x1c0(%rsp),%rdi
  466894:	mov    %r15,%rsi
  466897:	mov    %r12,%rdx
  46689a:	call   f0180 <_RNvNtNtCsgxBkk5gSRhY_4core3str8converts9from_utf8>
  46689f:	cmpb   $0x0,0x1c0(%rsp)
  4668a7:	je     4668ec <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xb4c>
  4668a9:	mov    0x80(%rsp),%r12
  4668b1:	test   %r14,%r14
  4668b4:	jle    46894b <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2bab>
  4668ba:	cmp    $0x1ffff,%r14
  4668c1:	jbe    4669f0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc50>
  4668c7:	mov    %r14,%rsi
  4668ca:	inc    %rsi
  4668cd:	js     46a4c3 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4723>
  4668d3:	cmpb   $0x1,(%r15,%r14,1)
  4668d8:	jne    468942 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2ba2>
  4668de:	mov    %r15,%rdi
  4668e1:	call   *0x80be9(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  4668e7:	jmp    46894b <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2bab>
  4668ec:	mov    %rbp,%rbx
  4668ef:	mov    $0x1,%bpl
  4668f2:	cmp    $0x1,%r12
  4668f6:	jne    466900 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xb60>
  4668f8:	cmpb   $0x30,(%r15)
  4668fc:	setne  %bpl
  466900:	mov    0x80(%rsp),%r12
  466908:	test   %r14,%r14
  46690b:	je     4669fe <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc5e>
  466911:	cmp    $0x1ffff,%r14
  466918:	jbe    4669f5 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc55>
  46691e:	movabs $0x7ffffffffffffff9,%rax
  466928:	or     $0x6,%rax
  46692c:	cmp    %rax,%r14
  46692f:	jb     46a09d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x42fd>
  466935:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46693a:	call   1041b0 <_RNvNtCs8zaw6vLE3tn_10rayon_core8registry15global_registry>
  46693f:	mov    (%rax),%rax
  466942:	add    $0x1ff,%rbx
  466949:	shr    $0x9,%rbx
  46694d:	mov    0x208(%rax),%rdx
  466954:	lea    0x1c0(%rsp),%r8
  46695c:	lea    0x97(%rsp),%r9
  466964:	mov    $0x1,%ecx
  466969:	mov    %rbx,%rdi
  46696c:	xor    %esi,%esi
  46696e:	call   24f540 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2l_8G2ConfigEEINtB1F_14ChunksProducerB2i_EEINtNtB6_8for_each15ForEachConsumerNCINvNtCsHW5vo2gROj_10zopro_core10msm_affine13merge_bucketsB31_E0EEB4q_>
  466973:	mov    0x110(%rsp),%rax
  46697b:	mov    0xa8(%rsp),%rcx
  466983:	vmovdqu 0x100(%rsp),%xmm0
  46698c:	mov    %rax,0x10(%rcx)
  466990:	mov    0x7d0(%rsp),%rax
  466998:	vmovdqu %xmm0,(%rcx)
  46699c:	test   %rax,%rax
  46699f:	je     467ba0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1e00>
  4669a5:	cmp    $0x3ff,%rax
  4669ab:	jbe    4669e0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc40>
  4669ad:	shl    $0x7,%rax
  4669b1:	movabs $0x7ffffffffffffff9,%rcx
  4669bb:	mov    %rax,%rsi
  4669be:	or     $0x1,%rsi
  4669c2:	cmp    %rcx,%rsi
  4669c5:	jae    46a4be <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471e>
  4669cb:	cmpb   $0x1,(%r14,%rax,1)
  4669d0:	jne    4669e2 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc42>
  4669d2:	mov    %r14,%rdi
  4669d5:	call   *0x80af5(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  4669db:	jmp    467ba0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1e00>
  4669e0:	jmp    4669e2 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc42>
  4669e2:	mov    %r14,%rdi
  4669e5:	call   *0x80aed(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  4669eb:	jmp    467ba0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1e00>
  4669f0:	jmp    468942 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2ba2>
  4669f5:	mov    %r15,%rdi
  4669f8:	call   *0x80ada(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  4669fe:	test   %bpl,%bpl
  466a01:	mov    %rbx,%rbp
  466a04:	jne    46894b <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2bab>
  466a0a:	lea    0xde0(%rsp),%rdi
  466a12:	call   250740 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCsHW5vo2gROj_10zopro_core3msm8BatchAddNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEEBG_>
  466a17:	lea    -0x1(%rbp),%r14d
  466a1b:	mov    $0x1,%eax
  466a20:	and    $0x3f,%r14d
  466a24:	shlx   %r14,%rax,%r13
  466a29:	mov    %rbp,%rdi
  466a2c:	call   1c02c0 <_RNvNtCsHW5vo2gROj_10zopro_core10msm_affine10batch_size>
  466a31:	cmp    $0x21,%rax
  466a35:	mov    $0x20,%edx
  466a3a:	cmovae %rax,%rdx
  466a3e:	lea    0x1c0(%rsp),%rdi
  466a46:	mov    %r13,%rsi
  466a49:	mov    %rdx,0x60(%rsp)
  466a4e:	call   2471d0 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE3newB4_>
  466a53:	mov    0xc0(%rsp),%rax
  466a5b:	movb   $0x0,0x7c1(%rsp)
  466a63:	mov    %r14,0x68(%rsp)
  466a68:	cmp    0xc8(%rsp),%rax
  466a70:	jne    467bb2 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1e12>
  466a76:	lea    0x1c0(%rsp),%rdi
  466a7e:	vzeroupper
  466a81:	call   247cd0 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE7executeB4_>
  466a86:	lea    0x1c0(%rsp),%rdi
  466a8e:	call   24d370 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE11flush_queueB4_>
  466a93:	lea    0x7d0(%rsp),%rdi
  466a9b:	lea    0x1c0(%rsp),%rsi
  466aa3:	mov    $0x608,%edx
  466aa8:	call   *0x80a0a(%rip)        # 4e74b8 <memcpy@GLIBC_2.14>
  466aae:	cmpq   $0x0,0x858(%rsp)
  466ab7:	jne    46a3ac <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x460c>
  466abd:	cmpq   $0x0,0x870(%rsp)
  466ac6:	jne    46a3ac <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x460c>
  466acc:	mov    0x7e0(%rsp),%rdx
  466ad4:	movabs $0xcf9bb18d1ece5fd7,%rcx
  466ade:	movabs $0x97816a916871ca8d,%rdi
  466ae8:	test   %rdx,%rdx
  466aeb:	je     467337 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1597>
  466af1:	mov    0x7d8(%rsp),%r14
  466af9:	shl    $0x7,%rdx
  466afd:	nopl   (%rax)
  466b00:	vmovdqu (%r14),%ymm0
  466b05:	vptest %ymm0,%ymm0
  466b0a:	jne    466b40 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xda0>
  466b0c:	vmovdqu 0x20(%r14),%ymm0
  466b12:	vptest %ymm0,%ymm0
  466b17:	jne    466b40 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xda0>
  466b19:	vmovdqu 0x40(%r14),%ymm0
  466b1f:	vptest %ymm0,%ymm0
  466b24:	jne    466b40 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xda0>
  466b26:	vmovdqu 0x60(%r14),%ymm0
  466b2c:	vptest %ymm0,%ymm0
  466b31:	je     467329 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1589>
  466b37:	nopw   0x0(%rax,%rax,1)
  466b40:	vmovdqu (%r14),%ymm0
  466b45:	vmovups -0x454ecd(%rip),%ymm3        # 11c80 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0x200>
  466b4d:	vmovups -0x454ff5(%rip),%ymm1        # 11b60 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0xe0>
  466b55:	mov    %rdx,0x8(%rsp)
  466b5a:	lea    0xde0(%rsp),%rsi
  466b62:	lea    0x1c0(%rsp),%r12
  466b6a:	movabs $0x87d20782e4866389,%rbx
  466b74:	lea    0x100(%rsp),%r13
  466b7c:	vmovdqu %ymm0,0x100(%rsp)
  466b85:	vmovups %ymm3,0xde0(%rsp)
  466b8e:	vmovups %ymm1,0x1c0(%rsp)
  466b97:	xor    %r15d,%r15d
  466b9a:	xor    %ecx,%ecx
  466b9c:	xor    %ebp,%ebp
  466b9e:	xor    %eax,%eax
  466ba0:	xor    %r8d,%r8d
  466ba3:	mov    (%rsi),%rdx
  466ba6:	xor    %r11d,%r11d
  466ba9:	mulx   0x0(%r13),%r9,%r10
  466baf:	adcx   %r9,%r15
  466bb5:	adox   %r10,%rcx
  466bbb:	mulx   0x8(%r13),%r9,%r10
  466bc1:	adcx   %r9,%rcx
  466bc7:	adox   %r10,%rbp
  466bcd:	mulx   0x10(%r13),%r9,%r10
  466bd3:	adcx   %r9,%rbp
  466bd9:	adox   %r10,%rax
  466bdf:	mulx   0x18(%r13),%r9,%r10
  466be5:	adcx   %r9,%rax
  466beb:	adox   %r10,%r8
  466bf1:	adcx   %r11,%r8
  466bf7:	mov    %r15,%rdx
  466bfa:	imul   %rbx,%rdx
  466bfe:	xor    %r11d,%r11d
  466c01:	mulx   (%r12),%r9,%r10
  466c07:	adcx   %r9,%r15
  466c0d:	adox   %r10,%rcx
  466c13:	mulx   0x8(%r12),%r9,%r10
  466c1a:	adcx   %r9,%rcx
  466c20:	adox   %r10,%rbp
  466c26:	mulx   0x10(%r12),%r9,%r10
  466c2d:	adcx   %r9,%rbp
  466c33:	adox   %r10,%rax
  466c39:	mulx   0x18(%r12),%r9,%r10
  466c40:	adcx   %r9,%rax
  466c46:	adox   %r10,%r8
  466c4c:	adcx   %r11,%r8
  466c52:	mov    0x8(%rsi),%rdx
  466c56:	xor    %r11d,%r11d
  466c59:	mulx   0x0(%r13),%r9,%r10
  466c5f:	adcx   %r9,%rcx
  466c65:	adox   %r10,%rbp
  466c6b:	mulx   0x8(%r13),%r9,%r10
  466c71:	adcx   %r9,%rbp
  466c77:	adox   %r10,%rax
  466c7d:	mulx   0x10(%r13),%r9,%r10
  466c83:	adcx   %r9,%rax
  466c89:	adox   %r10,%r8
  466c8f:	mulx   0x18(%r13),%r9,%r10
  466c95:	adcx   %r9,%r8
  466c9b:	adox   %r10,%r15
  466ca1:	adcx   %r11,%r15
  466ca7:	mov    %rcx,%rdx
  466caa:	imul   %rbx,%rdx
  466cae:	xor    %r11d,%r11d
  466cb1:	mulx   (%r12),%r9,%r10
  466cb7:	adcx   %r9,%rcx
  466cbd:	adox   %r10,%rbp
  466cc3:	mulx   0x8(%r12),%r9,%r10
  466cca:	adcx   %r9,%rbp
  466cd0:	adox   %r10,%rax
  466cd6:	mulx   0x10(%r12),%r9,%r10
  466cdd:	adcx   %r9,%rax
  466ce3:	adox   %r10,%r8
  466ce9:	mulx   0x18(%r12),%r9,%r10
  466cf0:	adcx   %r9,%r8
  466cf6:	adox   %r10,%r15
  466cfc:	adcx   %r11,%r15
  466d02:	mov    0x10(%rsi),%rdx
  466d06:	xor    %r11d,%r11d
  466d09:	mulx   0x0(%r13),%r9,%r10
  466d0f:	adcx   %r9,%rbp
  466d15:	adox   %r10,%rax
  466d1b:	mulx   0x8(%r13),%r9,%r10
  466d21:	adcx   %r9,%rax
  466d27:	adox   %r10,%r8
  466d2d:	mulx   0x10(%r13),%r9,%r10
  466d33:	adcx   %r9,%r8
  466d39:	adox   %r10,%r15
  466d3f:	mulx   0x18(%r13),%r9,%r10
  466d45:	adcx   %r9,%r15
  466d4b:	adox   %r10,%rcx
  466d51:	adcx   %r11,%rcx
  466d57:	mov    %rbp,%rdx
  466d5a:	imul   %rbx,%rdx
  466d5e:	xor    %r11d,%r11d
  466d61:	mulx   (%r12),%r9,%r10
  466d67:	adcx   %r9,%rbp
  466d6d:	adox   %r10,%rax
  466d73:	mulx   0x8(%r12),%r9,%r10
  466d7a:	adcx   %r9,%rax
  466d80:	adox   %r10,%r8
  466d86:	mulx   0x10(%r12),%r9,%r10
  466d8d:	adcx   %r9,%r8
  466d93:	adox   %r10,%r15
  466d99:	mulx   0x18(%r12),%r9,%r10
  466da0:	adcx   %r9,%r15
  466da6:	adox   %r10,%rcx
  466dac:	adcx   %r11,%rcx
  466db2:	mov    0x18(%rsi),%rdx
  466db6:	xor    %r11d,%r11d
  466db9:	mulx   0x0(%r13),%r9,%r10
  466dbf:	adcx   %r9,%rax
  466dc5:	adox   %r10,%r8
  466dcb:	mulx   0x8(%r13),%r9,%r10
  466dd1:	adcx   %r9,%r8
  466dd7:	adox   %r10,%r15
  466ddd:	mulx   0x10(%r13),%r9,%r10
  466de3:	adcx   %r9,%r15
  466de9:	adox   %r10,%rcx
  466def:	mulx   0x18(%r13),%r9,%r10
  466df5:	adcx   %r9,%rcx
  466dfb:	adox   %r10,%rbp
  466e01:	adcx   %r11,%rbp
  466e07:	mov    %rax,%rdx
  466e0a:	imul   %rbx,%rdx
  466e0e:	xor    %r11d,%r11d
  466e11:	mulx   (%r12),%r9,%r10
  466e17:	adcx   %r9,%rax
  466e1d:	adox   %r10,%r8
  466e23:	mulx   0x8(%r12),%r9,%r10
  466e2a:	adcx   %r9,%r8
  466e30:	adox   %r10,%r15
  466e36:	mulx   0x10(%r12),%r9,%r10
  466e3d:	adcx   %r9,%r15
  466e43:	adox   %r10,%rcx
  466e49:	mulx   0x18(%r12),%r9,%r10
  466e50:	adcx   %r9,%rcx
  466e56:	adox   %r10,%rbp
  466e5c:	adcx   %r11,%rbp
  466e62:	movabs $0x687e956e978e3573,%rax
  466e6c:	mov    %r15,0x28(%rsp)
  466e71:	mov    %rbp,0x30(%rsp)
  466e76:	mov    %r8,0x20(%rsp)
  466e7b:	mov    %rcx,0x10(%rsp)
  466e80:	movabs $0x87d20782e4866389,%r11
  466e8a:	lea    (%r15,%rax,1),%rsi
  466e8e:	movabs $0x3c208c16d87cfd47,%rax
  466e98:	vmovups 0x20(%r14),%ymm2
  466e9e:	vmovups %ymm3,0xde0(%rsp)
  466ea7:	vmovups %ymm1,0x1c0(%rsp)
  466eb0:	cmp    %rax,%r8
  466eb3:	sbb    $0x0,%rsi
  466eb7:	setb   %al
  466eba:	cmp    %rdi,%r15
  466ebd:	movabs $0xb85045b68181585d,%rdi
  466ec7:	mov    %rsi,0x48(%rsp)
  466ecc:	setb   %dl
  466ecf:	or     %al,%dl
  466ed1:	movabs $0x47afba497e7ea7a3,%rax
  466edb:	add    %rcx,%rax
  466ede:	movzbl %dl,%edx
  466ee1:	sub    %rdx,%rax
  466ee4:	setb   %dl
  466ee7:	cmp    %rdi,%rcx
  466eea:	mov    %rax,0x50(%rsp)
  466eef:	movabs $0xcf9bb18d1ece5fd7,%rax
  466ef9:	vmovups %ymm2,0x100(%rsp)
  466f02:	lea    0xde0(%rsp),%rdi
  466f0a:	setb   %r10b
  466f0e:	lea    0x0(%rbp,%rax,1),%r15
  466f13:	lea    0x100(%rsp),%rax
  466f1b:	or     %dl,%r10b
  466f1e:	movzbl %r10b,%edx
  466f22:	lea    0x1c0(%rsp),%r10
  466f2a:	sub    %rdx,%r15
  466f2d:	setb   0x18(%rsp)
  466f32:	xor    %r8d,%r8d
  466f35:	xor    %r12d,%r12d
  466f38:	xor    %r13d,%r13d
  466f3b:	xor    %ebx,%ebx
  466f3d:	xor    %r9d,%r9d
  466f40:	mov    (%rdi),%rdx
  466f43:	xor    %esi,%esi
  466f45:	mulx   (%rax),%rbp,%rcx
  466f4a:	adcx   %rbp,%r8
  466f50:	adox   %rcx,%r12
  466f56:	mulx   0x8(%rax),%rbp,%rcx
  466f5c:	adcx   %rbp,%r12
  466f62:	adox   %rcx,%r13
  466f68:	mulx   0x10(%rax),%rbp,%rcx
  466f6e:	adcx   %rbp,%r13
  466f74:	adox   %rcx,%rbx
  466f7a:	mulx   0x18(%rax),%rbp,%rcx
  466f80:	adcx   %rbp,%rbx
  466f86:	adox   %rcx,%r9
  466f8c:	adcx   %rsi,%r9
  466f92:	mov    %r8,%rdx
  466f95:	imul   %r11,%rdx
  466f99:	xor    %esi,%esi
  466f9b:	mulx   (%r10),%rbp,%rcx
  466fa0:	adcx   %rbp,%r8
  466fa6:	adox   %rcx,%r12
  466fac:	mulx   0x8(%r10),%rbp,%rcx
  466fb2:	adcx   %rbp,%r12
  466fb8:	adox   %rcx,%r13
  466fbe:	mulx   0x10(%r10),%rbp,%rcx
  466fc4:	adcx   %rbp,%r13
  466fca:	adox   %rcx,%rbx
  466fd0:	mulx   0x18(%r10),%rbp,%rcx
  466fd6:	adcx   %rbp,%rbx
  466fdc:	adox   %rcx,%r9
  466fe2:	adcx   %rsi,%r9
  466fe8:	mov    0x8(%rdi),%rdx
  466fec:	xor    %esi,%esi
  466fee:	mulx   (%rax),%rbp,%rcx
  466ff3:	adcx   %rbp,%r12
  466ff9:	adox   %rcx,%r13
  466fff:	mulx   0x8(%rax),%rbp,%rcx
  467005:	adcx   %rbp,%r13
  46700b:	adox   %rcx,%rbx
  467011:	mulx   0x10(%rax),%rbp,%rcx
  467017:	adcx   %rbp,%rbx
  46701d:	adox   %rcx,%r9
  467023:	mulx   0x18(%rax),%rbp,%rcx
  467029:	adcx   %rbp,%r9
  46702f:	adox   %rcx,%r8
  467035:	adcx   %rsi,%r8
  46703b:	mov    %r12,%rdx
  46703e:	imul   %r11,%rdx
  467042:	xor    %esi,%esi
  467044:	mulx   (%r10),%rbp,%rcx
  467049:	adcx   %rbp,%r12
  46704f:	adox   %rcx,%r13
  467055:	mulx   0x8(%r10),%rbp,%rcx
  46705b:	adcx   %rbp,%r13
  467061:	adox   %rcx,%rbx
  467067:	mulx   0x10(%r10),%rbp,%rcx
  46706d:	adcx   %rbp,%rbx
  467073:	adox   %rcx,%r9
  467079:	mulx   0x18(%r10),%rbp,%rcx
  46707f:	adcx   %rbp,%r9
  467085:	adox   %rcx,%r8
  46708b:	adcx   %rsi,%r8
  467091:	mov    0x10(%rdi),%rdx
  467095:	xor    %esi,%esi
  467097:	mulx   (%rax),%rbp,%rcx
  46709c:	adcx   %rbp,%r13
  4670a2:	adox   %rcx,%rbx
  4670a8:	mulx   0x8(%rax),%rbp,%rcx
  4670ae:	adcx   %rbp,%rbx
  4670b4:	adox   %rcx,%r9
  4670ba:	mulx   0x10(%rax),%rbp,%rcx
  4670c0:	adcx   %rbp,%r9
  4670c6:	adox   %rcx,%r8
  4670cc:	mulx   0x18(%rax),%rbp,%rcx
  4670d2:	adcx   %rbp,%r8
  4670d8:	adox   %rcx,%r12
  4670de:	adcx   %rsi,%r12
  4670e4:	mov    %r13,%rdx
  4670e7:	imul   %r11,%rdx
  4670eb:	xor    %esi,%esi
  4670ed:	mulx   (%r10),%rbp,%rcx
  4670f2:	adcx   %rbp,%r13
  4670f8:	adox   %rcx,%rbx
  4670fe:	mulx   0x8(%r10),%rbp,%rcx
  467104:	adcx   %rbp,%rbx
  46710a:	adox   %rcx,%r9
  467110:	mulx   0x10(%r10),%rbp,%rcx
  467116:	adcx   %rbp,%r9
  46711c:	adox   %rcx,%r8
  467122:	mulx   0x18(%r10),%rbp,%rcx
  467128:	adcx   %rbp,%r8
  46712e:	adox   %rcx,%r12
  467134:	adcx   %rsi,%r12
  46713a:	mov    0x18(%rdi),%rdx
  46713e:	xor    %esi,%esi
  467140:	mulx   (%rax),%rbp,%rcx
  467145:	adcx   %rbp,%rbx
  46714b:	adox   %rcx,%r9
  467151:	mulx   0x8(%rax),%rbp,%rcx
  467157:	adcx   %rbp,%r9
  46715d:	adox   %rcx,%r8
  467163:	mulx   0x10(%rax),%rbp,%rcx
  467169:	adcx   %rbp,%r8
  46716f:	adox   %rcx,%r12
  467175:	mulx   0x18(%rax),%rbp,%rcx
  46717b:	adcx   %rbp,%r12
  467181:	adox   %rcx,%r13
  467187:	adcx   %rsi,%r13
  46718d:	mov    %rbx,%rdx
  467190:	imul   %r11,%rdx
  467194:	xor    %esi,%esi
  467196:	mulx   (%r10),%rbp,%rcx
  46719b:	adcx   %rbp,%rbx
  4671a1:	adox   %rcx,%r9
  4671a7:	mulx   0x8(%r10),%rbp,%rcx
  4671ad:	adcx   %rbp,%r9
  4671b3:	adox   %rcx,%r8
  4671b9:	mulx   0x10(%r10),%rbp,%rcx
  4671bf:	adcx   %rbp,%r8
  4671c5:	adox   %rcx,%r12
  4671cb:	mulx   0x18(%r10),%rbp,%rcx
  4671d1:	adcx   %rbp,%r12
  4671d7:	adox   %rcx,%r13
  4671dd:	adcx   %rsi,%r13
  4671e3:	movabs $0x687e956e978e3573,%rcx
  4671ed:	movabs $0x97816a916871ca8d,%rsi
  4671f7:	mov    %r13,0x38(%rsp)
  4671fc:	mov    %r8,0x40(%rsp)
  467201:	mov    %r9,%r13
  467204:	lea    (%r8,%rcx,1),%rdx
  467208:	movabs $0x3c208c16d87cfd47,%rcx
  467212:	mov    0x38(%rsp),%r10
  467217:	mov    0x30(%rsp),%rax
  46721c:	mov    0x20(%rsp),%rdi
  467221:	cmp    %rcx,%r9
  467224:	mov    0x50(%rsp),%r9
  467229:	sbb    $0x0,%rdx
  46722d:	setb   %cl
  467230:	cmp    %rsi,%r8
  467233:	mov    0x48(%rsp),%r8
  467238:	setb   %sil
  46723c:	or     %cl,%sil
  46723f:	movabs $0x47afba497e7ea7a3,%rcx
  467249:	lea    (%r12,%rcx,1),%rbp
  46724d:	movzbl %sil,%ecx
  467251:	movabs $0xb85045b68181585d,%rsi
  46725b:	sub    %rcx,%rbp
  46725e:	setb   %cl
  467261:	cmp    %rsi,%r12
  467264:	setb   %sil
  467268:	or     %cl,%sil
  46726b:	movabs $0xcf9bb18d1ece5fd7,%rcx
  467275:	lea    (%r10,%rcx,1),%rbx
  467279:	movzbl %sil,%ecx
  46727d:	movabs $0x30644e72e131a029,%rsi
  467287:	sub    %rcx,%rbx
  46728a:	setb   %cl
  46728d:	cmp    %rsi,%r10
  467290:	setb   %sil
  467294:	or     %cl,%sil
  467297:	movabs $0x30644e72e131a029,%rcx
  4672a1:	cmp    %rcx,%rax
  4672a4:	setb   %cl
  4672a7:	or     0x18(%rsp),%cl
  4672ab:	movabs $0xc3df73e9278302b9,%rcx
  4672b5:	lea    (%rdi,%rcx,1),%rcx
  4672b9:	cmovne %rdi,%rcx
  4672bd:	cmovne 0x28(%rsp),%r8
  4672c3:	cmovne 0x10(%rsp),%r9
  4672c9:	cmovne %rax,%r15
  4672cd:	test   %sil,%sil
  4672d0:	movabs $0xc3df73e9278302b9,%rdi
  4672da:	cmovne 0x40(%rsp),%rdx
  4672e0:	lea    0x0(%r13,%rdi,1),%rsi
  4672e5:	mov    %rcx,(%r14)
  4672e8:	cmovne %r12,%rbp
  4672ec:	cmovne %r10,%rbx
  4672f0:	movabs $0xcf9bb18d1ece5fd7,%rcx
  4672fa:	movabs $0x97816a916871ca8d,%rdi
  467304:	cmovne %r13,%rsi
  467308:	mov    %r8,0x8(%r14)
  46730c:	mov    %r9,0x10(%r14)
  467310:	mov    %r15,0x18(%r14)
  467314:	mov    %rsi,0x20(%r14)
  467318:	mov    %rdx,0x28(%r14)
  46731c:	mov    0x8(%rsp),%rdx
  467321:	mov    %rbp,0x30(%r14)
  467325:	mov    %rbx,0x38(%r14)
  467329:	sub    $0xffffffffffffff80,%r14
  46732d:	add    $0xffffffffffffff80,%rdx
  467331:	jne    466b00 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xd60>
  467337:	mov    0x7f8(%rsp),%rax
  46733f:	test   %rax,%rax
  467342:	je     467b2d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1d8d>
  467348:	vmovdqu -0x4556d0(%rip),%ymm0        # 11c80 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0x200>
  467350:	vmovups -0x4557f8(%rip),%ymm1        # 11b60 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0xe0>
  467358:	mov    0x7f0(%rsp),%rbx
  467360:	lea    (%rax,%rax,2),%rax
  467364:	mov    %rcx,%r14
  467367:	shl    $0x6,%rax
  46736b:	add    %rbx,%rax
  46736e:	mov    %rax,0x38(%rsp)
  467373:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  467380:	vmovups (%rbx),%ymm2
  467384:	vmovdqu %ymm0,0xde0(%rsp)
  46738d:	vmovups %ymm1,0x1c0(%rsp)
  467396:	movabs $0x87d20782e4866389,%r13
  4673a0:	lea    0x100(%rsp),%rsi
  4673a8:	lea    0xde0(%rsp),%r15
  4673b0:	lea    0x1c0(%rsp),%rdi
  4673b8:	vmovups %ymm2,0x100(%rsp)
  4673c1:	xor    %r8d,%r8d
  4673c4:	xor    %r12d,%r12d
  4673c7:	xor    %ebp,%ebp
  4673c9:	xor    %eax,%eax
  4673cb:	xor    %r9d,%r9d
  4673ce:	mov    (%r15),%rdx
  4673d1:	xor    %r11d,%r11d
  4673d4:	mulx   (%rsi),%rcx,%r10
  4673d9:	adcx   %rcx,%r8
  4673df:	adox   %r10,%r12
  4673e5:	mulx   0x8(%rsi),%rcx,%r10
  4673eb:	adcx   %rcx,%r12
  4673f1:	adox   %r10,%rbp
  4673f7:	mulx   0x10(%rsi),%rcx,%r10
  4673fd:	adcx   %rcx,%rbp
  467403:	adox   %r10,%rax
  467409:	mulx   0x18(%rsi),%rcx,%r10
  46740f:	adcx   %rcx,%rax
  467415:	adox   %r10,%r9
  46741b:	adcx   %r11,%r9
  467421:	mov    %r8,%rdx
  467424:	imul   %r13,%rdx
  467428:	xor    %r11d,%r11d
  46742b:	mulx   (%rdi),%rcx,%r10
  467430:	adcx   %rcx,%r8
  467436:	adox   %r10,%r12
  46743c:	mulx   0x8(%rdi),%rcx,%r10
  467442:	adcx   %rcx,%r12
  467448:	adox   %r10,%rbp
  46744e:	mulx   0x10(%rdi),%rcx,%r10
  467454:	adcx   %rcx,%rbp
  46745a:	adox   %r10,%rax
  467460:	mulx   0x18(%rdi),%rcx,%r10
  467466:	adcx   %rcx,%rax
  46746c:	adox   %r10,%r9
  467472:	adcx   %r11,%r9
  467478:	mov    0x8(%r15),%rdx
  46747c:	xor    %r11d,%r11d
  46747f:	mulx   (%rsi),%rcx,%r10
  467484:	adcx   %rcx,%r12
  46748a:	adox   %r10,%rbp
  467490:	mulx   0x8(%rsi),%rcx,%r10
  467496:	adcx   %rcx,%rbp
  46749c:	adox   %r10,%rax
  4674a2:	mulx   0x10(%rsi),%rcx,%r10
  4674a8:	adcx   %rcx,%rax
  4674ae:	adox   %r10,%r9
  4674b4:	mulx   0x18(%rsi),%rcx,%r10
  4674ba:	adcx   %rcx,%r9
  4674c0:	adox   %r10,%r8
  4674c6:	adcx   %r11,%r8
  4674cc:	mov    %r12,%rdx
  4674cf:	imul   %r13,%rdx
  4674d3:	xor    %r11d,%r11d
  4674d6:	mulx   (%rdi),%rcx,%r10
  4674db:	adcx   %rcx,%r12
  4674e1:	adox   %r10,%rbp
  4674e7:	mulx   0x8(%rdi),%rcx,%r10
  4674ed:	adcx   %rcx,%rbp
  4674f3:	adox   %r10,%rax
  4674f9:	mulx   0x10(%rdi),%rcx,%r10
  4674ff:	adcx   %rcx,%rax
  467505:	adox   %r10,%r9
  46750b:	mulx   0x18(%rdi),%rcx,%r10
  467511:	adcx   %rcx,%r9
  467517:	adox   %r10,%r8
  46751d:	adcx   %r11,%r8
  467523:	mov    0x10(%r15),%rdx
  467527:	xor    %r11d,%r11d
  46752a:	mulx   (%rsi),%rcx,%r10
  46752f:	adcx   %rcx,%rbp
  467535:	adox   %r10,%rax
  46753b:	mulx   0x8(%rsi),%rcx,%r10
  467541:	adcx   %rcx,%rax
  467547:	adox   %r10,%r9
  46754d:	mulx   0x10(%rsi),%rcx,%r10
  467553:	adcx   %rcx,%r9
  467559:	adox   %r10,%r8
  46755f:	mulx   0x18(%rsi),%rcx,%r10
  467565:	adcx   %rcx,%r8
  46756b:	adox   %r10,%r12
  467571:	adcx   %r11,%r12
  467577:	mov    %rbp,%rdx
  46757a:	imul   %r13,%rdx
  46757e:	xor    %r11d,%r11d
  467581:	mulx   (%rdi),%rcx,%r10
  467586:	adcx   %rcx,%rbp
  46758c:	adox   %r10,%rax
  467592:	mulx   0x8(%rdi),%rcx,%r10
  467598:	adcx   %rcx,%rax
  46759e:	adox   %r10,%r9
  4675a4:	mulx   0x10(%rdi),%rcx,%r10
  4675aa:	adcx   %rcx,%r9
  4675b0:	adox   %r10,%r8
  4675b6:	mulx   0x18(%rdi),%rcx,%r10
  4675bc:	adcx   %rcx,%r8
  4675c2:	adox   %r10,%r12
  4675c8:	adcx   %r11,%r12
  4675ce:	mov    0x18(%r15),%rdx
  4675d2:	xor    %r11d,%r11d
  4675d5:	mulx   (%rsi),%rcx,%r10
  4675da:	adcx   %rcx,%rax
  4675e0:	adox   %r10,%r9
  4675e6:	mulx   0x8(%rsi),%rcx,%r10
  4675ec:	adcx   %rcx,%r9
  4675f2:	adox   %r10,%r8
  4675f8:	mulx   0x10(%rsi),%rcx,%r10
  4675fe:	adcx   %rcx,%r8
  467604:	adox   %r10,%r12
  46760a:	mulx   0x18(%rsi),%rcx,%r10
  467610:	adcx   %rcx,%r12
  467616:	adox   %r10,%rbp
  46761c:	adcx   %r11,%rbp
  467622:	mov    %rax,%rdx
  467625:	imul   %r13,%rdx
  467629:	xor    %r11d,%r11d
  46762c:	mulx   (%rdi),%rcx,%r10
  467631:	adcx   %rcx,%rax
  467637:	adox   %r10,%r9
  46763d:	mulx   0x8(%rdi),%rcx,%r10
  467643:	adcx   %rcx,%r9
  467649:	adox   %r10,%r8
  46764f:	mulx   0x10(%rdi),%rcx,%r10
  467655:	adcx   %rcx,%r8
  46765b:	adox   %r10,%r12
  467661:	mulx   0x18(%rdi),%rcx,%r10
  467667:	adcx   %rcx,%r12
  46766d:	adox   %r10,%rbp
  467673:	adcx   %r11,%rbp
  467679:	movabs $0x687e956e978e3573,%rcx
  467683:	movabs $0x3c208c16d87cfd47,%rdx
  46768d:	lea    0x0(%rbp,%r14,1),%r10
  467692:	mov    %rbp,0x30(%rsp)
  467697:	mov    %r9,0x20(%rsp)
  46769c:	mov    %r8,0x8(%rsp)
  4676a1:	mov    %r12,0x10(%rsp)
  4676a6:	cmp    %rdx,%r9
  4676a9:	lea    (%r8,%rcx,1),%rcx
  4676ad:	movabs $0x97816a916871ca8d,%rdx
  4676b7:	vmovups 0x20(%rbx),%ymm2
  4676bc:	vmovdqu %ymm0,0xde0(%rsp)
  4676c5:	vmovups %ymm1,0x1c0(%rsp)
  4676ce:	sbb    $0x0,%rcx
  4676d2:	setb   %al
  4676d5:	cmp    %rdx,%r8
  4676d8:	mov    %rcx,0x28(%rsp)
  4676dd:	movabs $0x47afba497e7ea7a3,%rcx
  4676e7:	lea    0xde0(%rsp),%r8
  4676ef:	setb   %dl
  4676f2:	or     %al,%dl
  4676f4:	lea    (%r12,%rcx,1),%rax
  4676f8:	movabs $0xb85045b68181585d,%rcx
  467702:	movzbl %dl,%edx
  467705:	sub    %rdx,%rax
  467708:	setb   %dl
  46770b:	cmp    %rcx,%r12
  46770e:	mov    %rax,0x40(%rsp)
  467713:	lea    0x100(%rsp),%rax
  46771b:	lea    0x1c0(%rsp),%rcx
  467723:	setb   %r11b
  467727:	vmovups %ymm2,0x100(%rsp)
  467730:	or     %dl,%r11b
  467733:	movzbl %r11b,%edx
  467737:	movabs $0x87d20782e4866389,%r11
  467741:	sub    %rdx,%r10
  467744:	setb   0x18(%rsp)
  467749:	xor    %r12d,%r12d
  46774c:	xor    %r9d,%r9d
  46774f:	xor    %r15d,%r15d
  467752:	xor    %r14d,%r14d
  467755:	xor    %ebp,%ebp
  467757:	mov    (%r8),%rdx
  46775a:	xor    %edi,%edi
  46775c:	mulx   (%rax),%r13,%rsi
  467761:	adcx   %r13,%r12
  467767:	adox   %rsi,%r9
  46776d:	mulx   0x8(%rax),%r13,%rsi
  467773:	adcx   %r13,%r9
  467779:	adox   %rsi,%r15
  46777f:	mulx   0x10(%rax),%r13,%rsi
  467785:	adcx   %r13,%r15
  46778b:	adox   %rsi,%r14
  467791:	mulx   0x18(%rax),%r13,%rsi
  467797:	adcx   %r13,%r14
  46779d:	adox   %rsi,%rbp
  4677a3:	adcx   %rdi,%rbp
  4677a9:	mov    %r12,%rdx
  4677ac:	imul   %r11,%rdx
  4677b0:	xor    %edi,%edi
  4677b2:	mulx   (%rcx),%r13,%rsi
  4677b7:	adcx   %r13,%r12
  4677bd:	adox   %rsi,%r9
  4677c3:	mulx   0x8(%rcx),%r13,%rsi
  4677c9:	adcx   %r13,%r9
  4677cf:	adox   %rsi,%r15
  4677d5:	mulx   0x10(%rcx),%r13,%rsi
  4677db:	adcx   %r13,%r15
  4677e1:	adox   %rsi,%r14
  4677e7:	mulx   0x18(%rcx),%r13,%rsi
  4677ed:	adcx   %r13,%r14
  4677f3:	adox   %rsi,%rbp
  4677f9:	adcx   %rdi,%rbp
  4677ff:	mov    0x8(%r8),%rdx
  467803:	xor    %edi,%edi
  467805:	mulx   (%rax),%r13,%rsi
  46780a:	adcx   %r13,%r9
  467810:	adox   %rsi,%r15
  467816:	mulx   0x8(%rax),%r13,%rsi
  46781c:	adcx   %r13,%r15
  467822:	adox   %rsi,%r14
  467828:	mulx   0x10(%rax),%r13,%rsi
  46782e:	adcx   %r13,%r14
  467834:	adox   %rsi,%rbp
  46783a:	mulx   0x18(%rax),%r13,%rsi
  467840:	adcx   %r13,%rbp
  467846:	adox   %rsi,%r12
  46784c:	adcx   %rdi,%r12
  467852:	mov    %r9,%rdx
  467855:	imul   %r11,%rdx
  467859:	xor    %edi,%edi
  46785b:	mulx   (%rcx),%r13,%rsi
  467860:	adcx   %r13,%r9
  467866:	adox   %rsi,%r15
  46786c:	mulx   0x8(%rcx),%r13,%rsi
  467872:	adcx   %r13,%r15
  467878:	adox   %rsi,%r14
  46787e:	mulx   0x10(%rcx),%r13,%rsi
  467884:	adcx   %r13,%r14
  46788a:	adox   %rsi,%rbp
  467890:	mulx   0x18(%rcx),%r13,%rsi
  467896:	adcx   %r13,%rbp
  46789c:	adox   %rsi,%r12
  4678a2:	adcx   %rdi,%r12
  4678a8:	mov    0x10(%r8),%rdx
  4678ac:	xor    %edi,%edi
  4678ae:	mulx   (%rax),%r13,%rsi
  4678b3:	adcx   %r13,%r15
  4678b9:	adox   %rsi,%r14
  4678bf:	mulx   0x8(%rax),%r13,%rsi
  4678c5:	adcx   %r13,%r14
  4678cb:	adox   %rsi,%rbp
  4678d1:	mulx   0x10(%rax),%r13,%rsi
  4678d7:	adcx   %r13,%rbp
  4678dd:	adox   %rsi,%r12
  4678e3:	mulx   0x18(%rax),%r13,%rsi
  4678e9:	adcx   %r13,%r12
  4678ef:	adox   %rsi,%r9
  4678f5:	adcx   %rdi,%r9
  4678fb:	mov    %r15,%rdx
  4678fe:	imul   %r11,%rdx
  467902:	xor    %edi,%edi
  467904:	mulx   (%rcx),%r13,%rsi
  467909:	adcx   %r13,%r15
  46790f:	adox   %rsi,%r14
  467915:	mulx   0x8(%rcx),%r13,%rsi
  46791b:	adcx   %r13,%r14
  467921:	adox   %rsi,%rbp
  467927:	mulx   0x10(%rcx),%r13,%rsi
  46792d:	adcx   %r13,%rbp
  467933:	adox   %rsi,%r12
  467939:	mulx   0x18(%rcx),%r13,%rsi
  46793f:	adcx   %r13,%r12
  467945:	adox   %rsi,%r9
  46794b:	adcx   %rdi,%r9
  467951:	mov    0x18(%r8),%rdx
  467955:	xor    %edi,%edi
  467957:	mulx   (%rax),%r13,%rsi
  46795c:	adcx   %r13,%r14
  467962:	adox   %rsi,%rbp
  467968:	mulx   0x8(%rax),%r13,%rsi
  46796e:	adcx   %r13,%rbp
  467974:	adox   %rsi,%r12
  46797a:	mulx   0x10(%rax),%r13,%rsi
  467980:	adcx   %r13,%r12
  467986:	adox   %rsi,%r9
  46798c:	mulx   0x18(%rax),%r13,%rsi
  467992:	adcx   %r13,%r9
  467998:	adox   %rsi,%r15
  46799e:	adcx   %rdi,%r15
  4679a4:	mov    %r14,%rdx
  4679a7:	imul   %r11,%rdx
  4679ab:	xor    %edi,%edi
  4679ad:	mulx   (%rcx),%r13,%rsi
  4679b2:	adcx   %r13,%r14
  4679b8:	adox   %rsi,%rbp
  4679be:	mulx   0x8(%rcx),%r13,%rsi
  4679c4:	adcx   %r13,%rbp
  4679ca:	adox   %rsi,%r12
  4679d0:	mulx   0x10(%rcx),%r13,%rsi
  4679d6:	adcx   %r13,%r12
  4679dc:	adox   %rsi,%r9
  4679e2:	mulx   0x18(%rcx),%r13,%rsi
  4679e8:	adcx   %r13,%r9
  4679ee:	adox   %rsi,%r15
  4679f4:	adcx   %rdi,%r15
  4679fa:	movabs $0x687e956e978e3573,%rdx
  467a04:	movabs $0x3c208c16d87cfd47,%rsi
  467a0e:	movabs $0x97816a916871ca8d,%rax
  467a18:	mov    %r15,0x48(%rsp)
  467a1d:	mov    %r9,%r15
  467a20:	lea    (%r12,%rdx,1),%r8
  467a24:	cmp    %rsi,%rbp
  467a27:	mov    0x48(%rsp),%r11
  467a2c:	mov    0x30(%rsp),%rdx
  467a31:	mov    0x20(%rsp),%rcx
  467a36:	sbb    $0x0,%r8
  467a3a:	setb   %sil
  467a3e:	cmp    %rax,%r12
  467a41:	movabs $0xb85045b68181585d,%rax
  467a4b:	setb   %dil
  467a4f:	or     %sil,%dil
  467a52:	movabs $0x47afba497e7ea7a3,%rsi
  467a5c:	lea    (%r9,%rsi,1),%r13
  467a60:	movzbl %dil,%esi
  467a64:	sub    %rsi,%r13
  467a67:	setb   %sil
  467a6b:	cmp    %rax,%r9
  467a6e:	movabs $0xcf9bb18d1ece5fd7,%rax
  467a78:	mov    0x40(%rsp),%r9
  467a7d:	setb   %dil
  467a81:	lea    (%r11,%rax,1),%r14
  467a85:	movabs $0x30644e72e131a029,%rax
  467a8f:	or     %sil,%dil
  467a92:	movzbl %dil,%esi
  467a96:	sub    %rsi,%r14
  467a99:	setb   %sil
  467a9d:	cmp    %rax,%r11
  467aa0:	setb   %dil
  467aa4:	or     %sil,%dil
  467aa7:	cmp    %rax,%rdx
  467aaa:	movabs $0xc3df73e9278302b9,%rax
  467ab4:	setb   %sil
  467ab8:	or     0x18(%rsp),%sil
  467abd:	lea    (%rcx,%rax,1),%rsi
  467ac1:	cmovne %rcx,%rsi
  467ac5:	mov    0x28(%rsp),%rcx
  467aca:	cmovne 0x10(%rsp),%r9
  467ad0:	cmovne %rdx,%r10
  467ad4:	cmovne 0x8(%rsp),%rcx
  467ada:	test   %dil,%dil
  467add:	lea    0x0(%rbp,%rax,1),%rdi
  467ae2:	mov    %rsi,(%rbx)
  467ae5:	cmovne %rbp,%rdi
  467ae9:	cmovne %r12,%r8
  467aed:	cmovne %r15,%r13
  467af1:	cmovne %r11,%r14
  467af5:	mov    %rcx,0x8(%rbx)
  467af9:	mov    %r9,0x10(%rbx)
  467afd:	mov    %r10,0x18(%rbx)
  467b01:	mov    %rdi,0x20(%rbx)
  467b05:	mov    %r8,0x28(%rbx)
  467b09:	mov    %r13,0x30(%rbx)
  467b0d:	mov    %r14,0x38(%rbx)
  467b11:	movabs $0xcf9bb18d1ece5fd7,%r14
  467b1b:	add    $0xc0,%rbx
  467b22:	cmp    0x38(%rsp),%rbx
  467b27:	jne    467380 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x15e0>
  467b2d:	mov    0x78(%rsp),%r15
  467b32:	mov    0x80(%rsp),%r12
  467b3a:	mov    0x68(%rsp),%rbp
  467b3f:	mov    0xc0(%rsp),%rax
  467b47:	cmp    0xc8(%rsp),%rax
  467b4f:	jne    46829a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x24fa>
  467b55:	lea    0x7d0(%rsp),%rdi
  467b5d:	vzeroupper
  467b60:	call   247cd0 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE7executeB4_>
  467b65:	lea    0x7d0(%rsp),%rdi
  467b6d:	call   24d370 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE11flush_queueB4_>
  467b72:	lea    0x1c0(%rsp),%rbx
  467b7a:	lea    0x7d0(%rsp),%rsi
  467b82:	mov    $0x608,%edx
  467b87:	mov    %rbx,%rdi
  467b8a:	call   *0x7f928(%rip)        # 4e74b8 <memcpy@GLIBC_2.14>
  467b90:	mov    0xa8(%rsp),%rdi
  467b98:	mov    %rbx,%rsi
  467b9b:	call   24de50 <_RINvNtCsHW5vo2gROj_10zopro_core10msm_affine14affine_bucketsNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEB4_>
  467ba0:	add    $0x1308,%rsp
  467ba7:	pop    %rbx
  467ba8:	pop    %r12
  467baa:	pop    %r13
  467bac:	pop    %r14
  467bae:	pop    %r15
  467bb0:	pop    %rbp
  467bb1:	ret
  467bb2:	mov    0x60(%rsp),%rax
  467bb7:	mov    0x78(%rsp),%rdx
  467bbc:	mov    0x98(%rsp),%r14
  467bc4:	lea    0x7e0(%rsp),%rbp
  467bcc:	mov    %r12,%r15
  467bcf:	add    $0xfffffffffffffffe,%rax
  467bd3:	mov    %rax,0x48(%rsp)
  467bd8:	mov    %rdx,%rax
  467bdb:	imul   0x70(%rsp),%rax
  467be1:	lea    -0x1(%rax),%rsi
  467be5:	mov    %rsi,%rcx
  467be8:	shr    $0x6,%rcx
  467bec:	mov    %rsi,%r12
  467bef:	mov    %rsi,0xf8(%rsp)
  467bf7:	mov    %rcx,0x50(%rsp)
  467bfc:	mov    %esi,%ecx
  467bfe:	and    $0x3f,%ecx
  467c01:	mov    %rcx,0x58(%rsp)
  467c06:	lea    -0x40(%rdx,%rcx,1),%rcx
  467c0b:	cmp    $0xffffffffffffffbf,%rcx
  467c0f:	setb   %cl
  467c12:	cmp    $0xc0,%rsi
  467c19:	setb   %dil
  467c1d:	and    %cl,%dil
  467c20:	mov    $0x1,%ecx
  467c25:	sub    %eax,%ecx
  467c27:	mov    %dil,0x88(%rsp)
  467c2f:	lea    0x1(%rdx),%eax
  467c32:	mov    $0xffffffffffffffff,%rdi
  467c39:	and    $0x3f,%edx
  467c3c:	and    $0x3f,%ecx
  467c3f:	shlx   %rax,%rdi,%rax
  467c44:	mov    %rdx,0x40(%rsp)
  467c49:	mov    %rcx,0xf0(%rsp)
  467c51:	shlx   %rdx,%rdi,%rcx
  467c56:	not    %rax
  467c59:	mov    %rcx,0x20(%rsp)
  467c5e:	not    %rcx
  467c61:	mov    %rax,0x28(%rsp)
  467c66:	mov    0xb8(%rsp),%rax
  467c6e:	mov    %rcx,0x38(%rsp)
  467c73:	mov    0xb0(%rsp),%rcx
  467c7b:	lea    0x18(%rcx,%rax,1),%rcx
  467c80:	lea    0x7c4c1(%rip),%rax        # 4e4148 <__frame_dummy_init_array_entry+0x5958>
  467c87:	mov    %rax,0xa0(%rsp)
  467c8f:	jmp    467cb6 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1f16>
  467c91:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  467ca0:	mov    0x10(%rsp),%rcx
  467ca5:	sub    $0xffffffffffffff80,%r15
  467ca9:	add    $0x20,%rcx
  467cad:	dec    %r14
  467cb0:	je     466a76 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xcd6>
  467cb6:	mov    -0x8(%rcx),%rax
  467cba:	mov    %rcx,0x10(%rsp)
  467cbf:	mov    (%rcx),%rcx
  467cc2:	mov    $0x3f,%dl
  467cc4:	mov    %r14,0x8(%rsp)
  467cc9:	vpxor  %xmm0,%xmm0,%xmm0
  467ccd:	bzhi   %rdx,%rcx,%rdx
  467cd2:	cmpq   $0x0,0x70(%rsp)
  467cd8:	mov    %rax,0x7d0(%rsp)
  467ce0:	mov    %rdx,0x7d8(%rsp)
  467ce8:	vmovdqu %xmm0,0x0(%rbp)
  467ced:	je     467d30 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1f90>
  467cef:	cmp    $0xff,%r12
  467cf6:	ja     467d50 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1fb0>
  467cf8:	mov    0x50(%rsp),%rax
  467cfd:	mov    0x58(%rsp),%rdx
  467d02:	cmpb   $0x0,0x88(%rsp)
  467d0a:	shrx   %rdx,0x7d0(%rsp,%rax,8),%rdx
  467d14:	je     467d52 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1fb2>
  467d16:	mov    0xf0(%rsp),%rsi
  467d1e:	shlx   %rsi,0x7d8(%rsp,%rax,8),%rax
  467d28:	or     %rax,%rdx
  467d2b:	jmp    467d52 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1fb2>
  467d2d:	nopl   (%rax)
  467d30:	and    0x38(%rsp),%rax
  467d35:	mov    0x68(%rsp),%rdx
  467d3a:	mov    0x40(%rsp),%rsi
  467d3f:	shrx   %rdx,%rax,%rdx
  467d44:	shlx   %rsi,%rdx,%rdx
  467d49:	jmp    467d63 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1fc3>
  467d4b:	nopl   0x0(%rax,%rax,1)
  467d50:	xor    %edx,%edx
  467d52:	and    0x28(%rsp),%rdx
  467d57:	lea    0x1(%rdx),%rax
  467d5b:	and    0x20(%rsp),%rdx
  467d60:	shr    $1,%rax
  467d63:	mov    0x8(%rsp),%r14
  467d68:	sub    %rdx,%rax
  467d6b:	mov    %eax,%edx
  467d6d:	neg    %edx
  467d6f:	test   %rcx,%rcx
  467d72:	cmovns %eax,%edx
  467d75:	test   %dx,%dx
  467d78:	je     467ca0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1f00>
  467d7e:	vmovdqu (%r15),%ymm0
  467d83:	vptest %ymm0,%ymm0
  467d88:	jne    467dc0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2020>
  467d8a:	vmovdqu 0x20(%r15),%ymm0
  467d90:	vptest %ymm0,%ymm0
  467d95:	jne    467dc0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2020>
  467d97:	vmovdqu 0x40(%r15),%ymm0
  467d9d:	vptest %ymm0,%ymm0
  467da2:	jne    467dc0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2020>
  467da4:	vmovdqu 0x60(%r15),%ymm0
  467daa:	vptest %ymm0,%ymm0
  467daf:	je     467ca0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1f00>
  467db5:	data16 cs nopw 0x0(%rax,%rax,1)
  467dc0:	mov    %eax,%ecx
  467dc2:	neg    %cx
  467dc5:	cmovs  %ax,%cx
  467dc9:	test   %dx,%dx
  467dcc:	js     467e00 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2060>
  467dce:	vmovdqu64 (%r15),%zmm0
  467dd4:	vmovups 0x40(%r15),%zmm1
  467ddb:	vmovups %zmm1,0xe20(%rsp)
  467de6:	vmovdqu64 %zmm0,0xde0(%rsp)
  467df1:	jmp    467f54 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x21b4>
  467df6:	cs nopw 0x0(%rax,%rax,1)
  467e00:	vmovups (%r15),%zmm0
  467e06:	vmovups -0x4562ae(%rip),%ymm2        # 11b60 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0xe0>
  467e0e:	xor    %eax,%eax
  467e10:	xor    %edx,%edx
  467e12:	xor    %esi,%esi
  467e14:	xor    %edi,%edi
  467e16:	lea    0x100(%rsp),%r14
  467e1e:	lea    0x7d0(%rsp),%r13
  467e26:	vmovups %zmm0,0xde0(%rsp)
  467e31:	vmovdqu 0x40(%r15),%ymm0
  467e37:	vmovups %ymm2,0x7d0(%rsp)
  467e40:	vmovdqu %ymm0,0x100(%rsp)
  467e49:	sub    (%r14),%rax
  467e4c:	sbb    0x8(%r14),%rdx
  467e50:	sbb    0x10(%r14),%rsi
  467e54:	sbb    0x18(%r14),%rdi
  467e58:	sbb    %rbx,%rbx
  467e5b:	mov    0x0(%r13),%r8
  467e5f:	mov    0x8(%r13),%r9
  467e63:	mov    0x10(%r13),%r10
  467e67:	mov    0x18(%r13),%r11
  467e6b:	and    %rbx,%r8
  467e6e:	and    %rbx,%r9
  467e71:	and    %rbx,%r10
  467e74:	and    %rbx,%r11
  467e77:	add    %r8,%rax
  467e7a:	adc    %r9,%rdx
  467e7d:	adc    %r10,%rsi
  467e80:	adc    %r11,%rdi
  467e83:	mov    %rax,0x30(%rsp)
  467e88:	mov    %r15,0x18(%rsp)
  467e8d:	xor    %r8d,%r8d
  467e90:	xor    %r9d,%r9d
  467e93:	xor    %r10d,%r10d
  467e96:	xor    %r11d,%r11d
  467e99:	lea    0x7d0(%rsp),%rax
  467ea1:	vmovups 0x60(%r15),%ymm1
  467ea7:	vmovups %ymm2,0x7d0(%rsp)
  467eb0:	lea    0x100(%rsp),%r15
  467eb8:	vmovups %ymm1,0x100(%rsp)
  467ec1:	sub    (%r15),%r9
  467ec4:	sbb    0x8(%r15),%r10
  467ec8:	sbb    0x10(%r15),%r11
  467ecc:	sbb    0x18(%r15),%r8
  467ed0:	sbb    %rbp,%rbp
  467ed3:	mov    (%rax),%rbx
  467ed6:	mov    0x8(%rax),%r13
  467eda:	mov    0x10(%rax),%r14
  467ede:	mov    0x18(%rax),%r12
  467ee2:	and    %rbp,%rbx
  467ee5:	and    %rbp,%r13
  467ee8:	and    %rbp,%r14
  467eeb:	and    %rbp,%r12
  467eee:	add    %rbx,%r9
  467ef1:	adc    %r13,%r10
  467ef4:	adc    %r14,%r11
  467ef7:	adc    %r12,%r8
  467efa:	lea    0x7e0(%rsp),%rbp
  467f02:	mov    0x30(%rsp),%rax
  467f07:	mov    0x18(%rsp),%r15
  467f0c:	mov    0xf8(%rsp),%r12
  467f14:	mov    %rax,0xe20(%rsp)
  467f1c:	mov    %rdx,0xe28(%rsp)
  467f24:	mov    %rsi,0xe30(%rsp)
  467f2c:	mov    %rdi,0xe38(%rsp)
  467f34:	mov    %r9,0xe40(%rsp)
  467f3c:	mov    %r10,0xe48(%rsp)
  467f44:	mov    %r11,0xe50(%rsp)
  467f4c:	mov    %r8,0xe58(%rsp)
  467f54:	mov    0x230(%rsp),%rax
  467f5c:	movzwl %cx,%r13d
  467f60:	dec    %r13
  467f63:	mov    %r13,%rsi
  467f66:	shr    $0x6,%rsi
  467f6a:	cmp    %rax,%rsi
  467f6d:	jae    46a44b <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x46ab>
  467f73:	mov    0x228(%rsp),%rcx
  467f7b:	mov    (%rcx,%rsi,8),%rdx
  467f7f:	bt     %r13,%rdx
  467f83:	jae    468060 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x22c0>
  467f89:	vmovdqu64 0xde0(%rsp),%zmm0
  467f94:	vmovups 0xe20(%rsp),%zmm1
  467f9f:	mov    0x260(%rsp),%rbx
  467fa7:	mov    0x8(%rsp),%r14
  467fac:	vmovups %zmm1,0x34(%rbp)
  467fb6:	vmovdqu64 %zmm0,-0xc(%rbp)
  467fc0:	cmp    0x250(%rsp),%rbx
  467fc8:	jne    467fda <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x223a>
  467fca:	lea    0x250(%rsp),%rdi
  467fd2:	vzeroupper
  467fd5:	call   24d5c0 <_RNvMs4_NtCs6i54tJFfzR_5alloc7raw_vecINtB5_6RawVecTmINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtBQ_8G2ConfigEEE8grow_oneCsHW5vo2gROj_10zopro_core>
  467fda:	mov    0x258(%rsp),%rax
  467fe2:	mov    %rbx,%rcx
  467fe5:	shl    $0x7,%rcx
  467fe9:	lea    (%rcx,%rbx,8),%rcx
  467fed:	mov    %r13d,(%rax,%rcx,1)
  467ff1:	vmovdqu64 0x7d0(%rsp),%zmm0
  467ffc:	vmovups 0x810(%rsp),%zmm1
  468007:	vmovdqu64 %zmm0,0x4(%rax,%rcx,1)
  468012:	vmovups %zmm1,0x44(%rax,%rcx,1)
  46801d:	mov    0x850(%rsp),%edx
  468024:	mov    %edx,0x84(%rax,%rcx,1)
  46802b:	lea    0x1(%rbx),%rcx
  46802f:	mov    %rcx,0x260(%rsp)
  468037:	cmp    0x48(%rsp),%rbx
  46803c:	jne    467ca0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1f00>
  468042:	lea    0x1c0(%rsp),%rdi
  46804a:	vzeroupper
  46804d:	call   24d370 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE11flush_queueB4_>
  468052:	jmp    467ca0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1f00>
  468057:	nopw   0x0(%rax,%rax,1)
  468060:	mov    0x1d0(%rsp),%rdx
  468068:	mov    %r13d,%esi
  46806b:	cmp    %rsi,%rdx
  46806e:	jbe    46a3ec <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x464c>
  468074:	mov    0x1c8(%rsp),%rdi
  46807c:	mov    %rsi,%r8
  46807f:	shl    $0x7,%r8
  468083:	vmovdqu (%rdi,%r8,1),%ymm0
  468089:	lea    (%rdi,%r8,1),%rdx
  46808d:	vptest %ymm0,%ymm0
  468092:	jne    4680c0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2320>
  468094:	vmovdqu 0x20(%rdx),%ymm0
  468099:	vptest %ymm0,%ymm0
  46809e:	jne    4680c0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2320>
  4680a0:	vmovdqu 0x40(%rdx),%ymm0
  4680a5:	vptest %ymm0,%ymm0
  4680aa:	jne    4680c0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2320>
  4680ac:	vmovdqu 0x60(%rdx),%ymm0
  4680b1:	vptest %ymm0,%ymm0
  4680b6:	je     468255 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x24b5>
  4680bc:	nopl   0x0(%rax)
  4680c0:	vmovdqu (%rdx),%ymm0
  4680c4:	vpxor  0xde0(%rsp),%ymm0,%ymm0
  4680cd:	vptest %ymm0,%ymm0
  4680d2:	jne    4680ed <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x234d>
  4680d4:	vmovdqu 0x20(%rdx),%ymm0
  4680d9:	vpxor  0xe00(%rsp),%ymm0,%ymm0
  4680e2:	vptest %ymm0,%ymm0
  4680e7:	je     4681e9 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2449>
  4680ed:	shr    $0x6,%esi
  4680f0:	cmp    %rax,%rsi
  4680f3:	jae    46a43c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x469c>
  4680f9:	mov    $0x1,%eax
  4680fe:	shlx   %r13,%rax,%rax
  468103:	or     %rax,(%rcx,%rsi,8)
  468107:	vmovdqu64 0xde0(%rsp),%zmm0
  468112:	vmovups 0xe20(%rsp),%zmm1
  46811d:	mov    0x248(%rsp),%rbx
  468125:	vmovups %zmm1,0x34(%rbp)
  46812f:	vmovdqu64 %zmm0,-0xc(%rbp)
  468139:	cmp    0x238(%rsp),%rbx
  468141:	jne    468153 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x23b3>
  468143:	lea    0x238(%rsp),%rdi
  46814b:	vzeroupper
  46814e:	call   24d5c0 <_RNvMs4_NtCs6i54tJFfzR_5alloc7raw_vecINtB5_6RawVecTmINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtBQ_8G2ConfigEEE8grow_oneCsHW5vo2gROj_10zopro_core>
  468153:	mov    0x240(%rsp),%rax
  46815b:	mov    %rbx,%rcx
  46815e:	shl    $0x7,%rcx
  468162:	lea    (%rcx,%rbx,8),%rcx
  468166:	inc    %rbx
  468169:	mov    %r13d,(%rax,%rcx,1)
  46816d:	vmovdqu64 0x7d0(%rsp),%zmm0
  468178:	vmovups 0x810(%rsp),%zmm1
  468183:	vmovdqu64 %zmm0,0x4(%rax,%rcx,1)
  46818e:	vmovups %zmm1,0x44(%rax,%rcx,1)
  468199:	mov    0x850(%rsp),%edx
  4681a0:	mov    %edx,0x84(%rax,%rcx,1)
  4681a7:	mov    %rbx,0x248(%rsp)
  4681af:	mov    0x60(%rsp),%rax
  4681b4:	mov    0x8(%rsp),%r14
  4681b9:	cmp    %rax,0x248(%rsp)
  4681c1:	jne    467ca0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1f00>
  4681c7:	lea    0x1c0(%rsp),%rdi
  4681cf:	vzeroupper
  4681d2:	call   247cd0 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE7executeB4_>
  4681d7:	lea    0x1c0(%rsp),%rdi
  4681df:	call   24d840 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE15take_from_queueB4_>
  4681e4:	jmp    467ca0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1f00>
  4681e9:	vmovups 0xde0(%rsp),%zmm0
  4681f4:	vmovups 0xe20(%rsp),%zmm1
  4681ff:	vmovups %zmm1,0x810(%rsp)
  46820a:	vmovups %zmm0,0x7d0(%rsp)
  468215:	vmovdqu 0x40(%rdx),%ymm0
  46821a:	vpxor  0x810(%rsp),%ymm0,%ymm0
  468223:	vptest %ymm0,%ymm0
  468228:	jne    46823f <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x249f>
  46822a:	vmovdqu 0x60(%rdx),%ymm0
  46822f:	vpxor  0x830(%rsp),%ymm0,%ymm0
  468238:	vptest %ymm0,%ymm0
  46823d:	je     46827d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x24dd>
  46823f:	vpxor  %xmm0,%xmm0,%xmm0
  468243:	vmovdqu64 %zmm0,0x40(%rdx)
  46824a:	vmovdqu64 %zmm0,(%rdx)
  468250:	jmp    4681af <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x240f>
  468255:	vmovups 0xde0(%rsp),%zmm0
  468260:	vmovups %zmm0,(%rdx)
  468266:	vmovdqu64 0xe20(%rsp),%zmm0
  468271:	vmovdqu64 %zmm0,0x40(%rdx)
  468278:	jmp    4681af <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x240f>
  46827d:	lea    0x1c0(%rsp),%rdi
  468285:	lea    0xde0(%rsp),%rdx
  46828d:	vzeroupper
  468290:	call   24d630 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE14add_projectiveB4_>
  468295:	jmp    4681af <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x240f>
  46829a:	mov    %r15,%rax
  46829d:	imul   0x70(%rsp),%rax
  4682a3:	lea    -0x1(%rax),%rdx
  4682a7:	mov    %rdx,%rcx
  4682aa:	shr    $0x6,%rcx
  4682ae:	mov    %rdx,0x8(%rsp)
  4682b3:	mov    %rcx,0x40(%rsp)
  4682b8:	mov    %edx,%ecx
  4682ba:	and    $0x3f,%ecx
  4682bd:	mov    %rcx,0x38(%rsp)
  4682c2:	lea    -0x40(%r15,%rcx,1),%rcx
  4682c7:	cmp    $0xffffffffffffffbf,%rcx
  4682cb:	setb   %cl
  4682ce:	cmp    $0xc0,%rdx
  4682d5:	setb   %dl
  4682d8:	and    %cl,%dl
  4682da:	mov    $0x1,%ecx
  4682df:	sub    %eax,%ecx
  4682e1:	mov    %dl,0x50(%rsp)
  4682e5:	lea    0x1(%r15),%eax
  4682e9:	mov    $0xffffffffffffffff,%rdx
  4682f0:	and    $0x3f,%r15d
  4682f4:	and    $0x3f,%ecx
  4682f7:	shlx   %rax,%rdx,%rax
  4682fc:	shlx   %r15,%rdx,%rdx
  468301:	mov    %r15,0x78(%rsp)
  468306:	mov    %rcx,0x58(%rsp)
  46830b:	mov    0x60(%rsp),%rcx
  468310:	not    %rax
  468313:	mov    %rdx,0x28(%rsp)
  468318:	not    %rdx
  46831b:	mov    %rax,0x10(%rsp)
  468320:	mov    %rdx,0x48(%rsp)
  468325:	lea    -0x2(%rcx),%rax
  468329:	mov    0xb0(%rsp),%rcx
  468331:	mov    %rax,0x30(%rsp)
  468336:	mov    0xb8(%rsp),%rax
  46833e:	lea    0x8(%rcx,%rax,1),%rbx
  468343:	lea    0x7bdfe(%rip),%rax        # 4e4148 <__frame_dummy_init_array_entry+0x5958>
  46834a:	mov    %rax,0x88(%rsp)
  468352:	jmp    468376 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x25d6>
  468354:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  468360:	add    $0x20,%rbx
  468364:	sub    $0xffffffffffffff80,%r12
  468368:	decq   0x98(%rsp)
  468370:	je     467b55 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1db5>
  468376:	mov    (%rbx),%rcx
  468379:	mov    -0x8(%rbx),%rdx
  46837d:	mov    $0x3f,%al
  46837f:	vpxor  %xmm0,%xmm0,%xmm0
  468383:	bzhi   %rax,%rcx,%rax
  468388:	cmpq   $0x0,0x70(%rsp)
  46838e:	mov    %rdx,0x1c0(%rsp)
  468396:	mov    %rax,0x1c8(%rsp)
  46839e:	lea    0x1d0(%rsp),%rax
  4683a6:	vmovdqu %xmm0,(%rax)
  4683aa:	je     4683f0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2650>
  4683ac:	cmpq   $0xff,0x8(%rsp)
  4683b5:	ja     468410 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2670>
  4683b7:	mov    0x40(%rsp),%rdx
  4683bc:	mov    0x38(%rsp),%rax
  4683c1:	cmpb   $0x0,0x50(%rsp)
  4683c6:	shrx   %rax,0x1c0(%rsp,%rdx,8),%rax
  4683d0:	je     468412 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2672>
  4683d2:	mov    0x58(%rsp),%rsi
  4683d7:	shlx   %rsi,0x1c8(%rsp,%rdx,8),%rdx
  4683e1:	or     %rdx,%rax
  4683e4:	jmp    468412 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2672>
  4683e6:	cs nopw 0x0(%rax,%rax,1)
  4683f0:	and    0x48(%rsp),%rdx
  4683f5:	shrx   %rbp,%rdx,%rax
  4683fa:	shlx   %r15,%rax,%rax
  4683ff:	jmp    468423 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2683>
  468401:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  468410:	xor    %eax,%eax
  468412:	and    0x10(%rsp),%rax
  468417:	lea    0x1(%rax),%rdx
  46841b:	and    0x28(%rsp),%rax
  468420:	shr    $1,%rdx
  468423:	sub    %rax,%rdx
  468426:	mov    %edx,%eax
  468428:	neg    %eax
  46842a:	test   %rcx,%rcx
  46842d:	cmovns %edx,%eax
  468430:	test   %ax,%ax
  468433:	je     468360 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x25c0>
  468439:	vmovdqu (%r12),%ymm0
  46843f:	vptest %ymm0,%ymm0
  468444:	jne    468480 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x26e0>
  468446:	vmovdqu 0x20(%r12),%ymm0
  46844d:	vptest %ymm0,%ymm0
  468452:	jne    468480 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x26e0>
  468454:	vmovdqu 0x40(%r12),%ymm0
  46845b:	vptest %ymm0,%ymm0
  468460:	jne    468480 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x26e0>
  468462:	vmovdqu 0x60(%r12),%ymm0
  468469:	vptest %ymm0,%ymm0
  46846e:	je     468360 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x25c0>
  468474:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  468480:	test   %ax,%ax
  468483:	js     4684b0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2710>
  468485:	vmovups (%r12),%zmm0
  46848c:	vmovups 0x40(%r12),%zmm1
  468494:	vmovups %zmm1,0xe20(%rsp)
  46849f:	vmovups %zmm0,0xde0(%rsp)
  4684aa:	jmp    468614 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2874>
  4684af:	nop
  4684b0:	vmovups (%r12),%zmm0
  4684b7:	vmovups -0x45695f(%rip),%ymm2        # 11b60 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0xe0>
  4684bf:	xor    %ecx,%ecx
  4684c1:	xor    %edx,%edx
  4684c3:	xor    %esi,%esi
  4684c5:	xor    %edi,%edi
  4684c7:	lea    0x100(%rsp),%r13
  4684cf:	lea    0x1c0(%rsp),%rbp
  4684d7:	neg    %eax
  4684d9:	vmovups %zmm0,0xde0(%rsp)
  4684e4:	vmovups 0x40(%r12),%ymm0
  4684eb:	vmovups %ymm2,0x1c0(%rsp)
  4684f4:	vmovups %ymm0,0x100(%rsp)
  4684fd:	sub    0x0(%r13),%rcx
  468501:	sbb    0x8(%r13),%rdx
  468505:	sbb    0x10(%r13),%rsi
  468509:	sbb    0x18(%r13),%rdi
  46850d:	sbb    %r14,%r14
  468510:	mov    0x0(%rbp),%r8
  468514:	mov    0x8(%rbp),%r9
  468518:	mov    0x10(%rbp),%r10
  46851c:	mov    0x18(%rbp),%r11
  468520:	and    %r14,%r8
  468523:	and    %r14,%r9
  468526:	and    %r14,%r10
  468529:	and    %r14,%r11
  46852c:	add    %r8,%rcx
  46852f:	adc    %r9,%rdx
  468532:	adc    %r10,%rsi
  468535:	adc    %r11,%rdi
  468538:	mov    %rcx,0x18(%rsp)
  46853d:	mov    %rbx,0x20(%rsp)
  468542:	mov    %r12,0x80(%rsp)
  46854a:	xor    %r8d,%r8d
  46854d:	xor    %r9d,%r9d
  468550:	xor    %r10d,%r10d
  468553:	xor    %r11d,%r11d
  468556:	lea    0x100(%rsp),%rbx
  46855e:	lea    0x1c0(%rsp),%rcx
  468566:	vmovups 0x60(%r12),%ymm1
  46856d:	vmovups %ymm2,0x1c0(%rsp)
  468576:	vmovups %ymm1,0x100(%rsp)
  46857f:	sub    (%rbx),%r9
  468582:	sbb    0x8(%rbx),%r10
  468586:	sbb    0x10(%rbx),%r11
  46858a:	sbb    0x18(%rbx),%r8
  46858e:	sbb    %r12,%r12
  468591:	mov    (%rcx),%r14
  468594:	mov    0x8(%rcx),%r13
  468598:	mov    0x10(%rcx),%r15
  46859c:	mov    0x18(%rcx),%rbp
  4685a0:	and    %r12,%r14
  4685a3:	and    %r12,%r13
  4685a6:	and    %r12,%r15
  4685a9:	and    %r12,%rbp
  4685ac:	add    %r14,%r9
  4685af:	adc    %r13,%r10
  4685b2:	adc    %r15,%r11
  4685b5:	adc    %rbp,%r8
  4685b8:	mov    0x18(%rsp),%rcx
  4685bd:	mov    0x20(%rsp),%rbx
  4685c2:	mov    0x68(%rsp),%rbp
  4685c7:	mov    0x80(%rsp),%r12
  4685cf:	mov    0x78(%rsp),%r15
  4685d4:	mov    %rcx,0xe20(%rsp)
  4685dc:	mov    %rdx,0xe28(%rsp)
  4685e4:	mov    %rsi,0xe30(%rsp)
  4685ec:	mov    %rdi,0xe38(%rsp)
  4685f4:	mov    %r9,0xe40(%rsp)
  4685fc:	mov    %r10,0xe48(%rsp)
  468604:	mov    %r11,0xe50(%rsp)
  46860c:	mov    %r8,0xe58(%rsp)
  468614:	mov    0x840(%rsp),%rsi
  46861c:	movzwl %ax,%r14d
  468620:	dec    %r14
  468623:	mov    %r14,%rdi
  468626:	shr    $0x6,%rdi
  46862a:	cmp    %rsi,%rdi
  46862d:	jae    46a40a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x466a>
  468633:	mov    0x838(%rsp),%rcx
  46863b:	mov    (%rcx,%rdi,8),%rdx
  46863f:	bt     %r14,%rdx
  468643:	jae    468720 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2980>
  468649:	vmovdqu64 0xde0(%rsp),%zmm0
  468654:	vmovups 0xe20(%rsp),%zmm1
  46865f:	mov    0x870(%rsp),%r13
  468667:	lea    0x1d0(%rsp),%rax
  46866f:	vmovups %zmm1,0x34(%rax)
  468679:	vmovdqu64 %zmm0,-0xc(%rax)
  468683:	cmp    0x860(%rsp),%r13
  46868b:	jne    46869d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x28fd>
  46868d:	lea    0x860(%rsp),%rdi
  468695:	vzeroupper
  468698:	call   24d5c0 <_RNvMs4_NtCs6i54tJFfzR_5alloc7raw_vecINtB5_6RawVecTmINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtBQ_8G2ConfigEEE8grow_oneCsHW5vo2gROj_10zopro_core>
  46869d:	mov    0x868(%rsp),%rax
  4686a5:	mov    %r13,%rcx
  4686a8:	shl    $0x7,%rcx
  4686ac:	lea    (%rcx,%r13,8),%rcx
  4686b0:	mov    %r14d,(%rax,%rcx,1)
  4686b4:	vmovdqu64 0x1c0(%rsp),%zmm0
  4686bc:	vmovups 0x200(%rsp),%zmm1
  4686c4:	vmovdqu64 %zmm0,0x4(%rax,%rcx,1)
  4686cf:	vmovups %zmm1,0x44(%rax,%rcx,1)
  4686da:	mov    0x240(%rsp),%edx
  4686e1:	mov    %edx,0x84(%rax,%rcx,1)
  4686e8:	lea    0x1(%r13),%rcx
  4686ec:	mov    %rcx,0x870(%rsp)
  4686f4:	cmp    0x30(%rsp),%r13
  4686f9:	jne    468360 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x25c0>
  4686ff:	lea    0x7d0(%rsp),%rdi
  468707:	vzeroupper
  46870a:	call   24d370 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE11flush_queueB4_>
  46870f:	jmp    468360 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x25c0>
  468714:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  468720:	mov    0x7e0(%rsp),%rsi
  468728:	cmp    %r14,%rsi
  46872b:	jbe    46a3f8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4658>
  468731:	mov    0x7d8(%rsp),%rsi
  468739:	mov    %r14,%r8
  46873c:	shl    $0x7,%r8
  468740:	vmovdqu (%rsi,%r8,1),%ymm0
  468746:	lea    (%rsi,%r8,1),%rax
  46874a:	vptest %ymm0,%ymm0
  46874f:	jne    468780 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x29e0>
  468751:	vmovdqu 0x20(%rax),%ymm0
  468756:	vptest %ymm0,%ymm0
  46875b:	jne    468780 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x29e0>
  46875d:	vmovdqu 0x40(%rax),%ymm0
  468762:	vptest %ymm0,%ymm0
  468767:	jne    468780 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x29e0>
  468769:	vmovdqu 0x60(%rax),%ymm0
  46876e:	vptest %ymm0,%ymm0
  468773:	je     468903 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2b63>
  468779:	nopl   0x0(%rax)
  468780:	vmovdqu (%rax),%ymm0
  468784:	vpxor  0xde0(%rsp),%ymm0,%ymm0
  46878d:	vptest %ymm0,%ymm0
  468792:	jne    4687ad <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2a0d>
  468794:	vmovdqu 0x20(%rax),%ymm0
  468799:	vpxor  0xe00(%rsp),%ymm0,%ymm0
  4687a2:	vptest %ymm0,%ymm0
  4687a7:	je     46889d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2afd>
  4687ad:	mov    $0x1,%eax
  4687b2:	shlx   %r14,%rax,%rax
  4687b7:	or     %rax,%rdx
  4687ba:	lea    0x1d0(%rsp),%rax
  4687c2:	mov    %rdx,(%rcx,%rdi,8)
  4687c6:	vmovdqu64 0xde0(%rsp),%zmm0
  4687d1:	vmovups 0xe20(%rsp),%zmm1
  4687dc:	mov    0x858(%rsp),%r13
  4687e4:	vmovups %zmm1,0x34(%rax)
  4687ee:	vmovdqu64 %zmm0,-0xc(%rax)
  4687f8:	cmp    0x848(%rsp),%r13
  468800:	jne    468812 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2a72>
  468802:	lea    0x848(%rsp),%rdi
  46880a:	vzeroupper
  46880d:	call   24d5c0 <_RNvMs4_NtCs6i54tJFfzR_5alloc7raw_vecINtB5_6RawVecTmINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtBQ_8G2ConfigEEE8grow_oneCsHW5vo2gROj_10zopro_core>
  468812:	mov    0x850(%rsp),%rax
  46881a:	mov    %r13,%rcx
  46881d:	shl    $0x7,%rcx
  468821:	lea    (%rcx,%r13,8),%rcx
  468825:	inc    %r13
  468828:	mov    %r14d,(%rax,%rcx,1)
  46882c:	vmovdqu64 0x1c0(%rsp),%zmm0
  468834:	vmovups 0x200(%rsp),%zmm1
  46883c:	vmovdqu64 %zmm0,0x4(%rax,%rcx,1)
  468847:	vmovups %zmm1,0x44(%rax,%rcx,1)
  468852:	mov    0x240(%rsp),%edx
  468859:	mov    %edx,0x84(%rax,%rcx,1)
  468860:	mov    %r13,0x858(%rsp)
  468868:	mov    0x60(%rsp),%rax
  46886d:	cmp    %rax,0x858(%rsp)
  468875:	jne    468360 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x25c0>
  46887b:	lea    0x7d0(%rsp),%rdi
  468883:	vzeroupper
  468886:	call   247cd0 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE7executeB4_>
  46888b:	lea    0x7d0(%rsp),%rdi
  468893:	call   24d840 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE15take_from_queueB4_>
  468898:	jmp    468360 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x25c0>
  46889d:	vmovups 0xde0(%rsp),%zmm0
  4688a8:	vmovups 0xe20(%rsp),%zmm1
  4688b3:	vmovups %zmm1,0x200(%rsp)
  4688bb:	vmovups %zmm0,0x1c0(%rsp)
  4688c3:	vmovdqu 0x40(%rax),%ymm0
  4688c8:	vpxor  0x200(%rsp),%ymm0,%ymm0
  4688d1:	vptest %ymm0,%ymm0
  4688d6:	jne    4688ed <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2b4d>
  4688d8:	vmovdqu 0x60(%rax),%ymm0
  4688dd:	vpxor  0x220(%rsp),%ymm0,%ymm0
  4688e6:	vptest %ymm0,%ymm0
  4688eb:	je     468922 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2b82>
  4688ed:	vpxor  %xmm0,%xmm0,%xmm0
  4688f1:	vmovdqu64 %zmm0,0x40(%rax)
  4688f8:	vmovdqu64 %zmm0,(%rax)
  4688fe:	jmp    468868 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2ac8>
  468903:	vmovdqu64 0xde0(%rsp),%zmm0
  46890e:	vmovups 0xe20(%rsp),%zmm1
  468919:	vmovups %zmm1,0x40(%rax)
  468920:	jmp    4688f8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2b58>
  468922:	lea    0x7d0(%rsp),%rdi
  46892a:	lea    0xde0(%rsp),%rdx
  468932:	mov    %r14,%rsi
  468935:	vzeroupper
  468938:	call   24d630 <_RNvMNtCsHW5vo2gROj_10zopro_core10msm_affineINtB2_6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigE14add_projectiveB4_>
  46893d:	jmp    468868 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2ac8>
  468942:	mov    %r15,%rdi
  468945:	call   *0x7eb8d(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46894b:	mov    %rbp,%rdi
  46894e:	call   1c02c0 <_RNvNtCsHW5vo2gROj_10zopro_core10msm_affine10batch_size>
  468953:	mov    %r13,0x60(%rsp)
  468958:	cmp    $0x21,%rax
  46895c:	mov    $0x20,%edx
  468961:	lea    -0x1(%rbp),%r13d
  468965:	movabs $0x7ffffffffffffff9,%rcx
  46896f:	cmovae %rax,%rdx
  468973:	and    $0x3f,%r13d
  468977:	mov    $0x80,%eax
  46897c:	dec    %rcx
  46897f:	shlx   %r13,%rax,%r14
  468984:	cmp    $0x39,%r13d
  468988:	setae  %al
  46898b:	cmp    %rcx,%r14
  46898e:	seta   %cl
  468991:	or     %al,%cl
  468993:	je     4689a8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2c08>
  468995:	xor    %r15d,%r15d
  468998:	mov    %r15,%rdi
  46899b:	mov    %r14,%rsi
  46899e:	call   ed5a1 <_RNvNtCs6i54tJFfzR_5alloc7raw_vec12handle_error>
  4689a3:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  4689a8:	mov    %rbp,%rbx
  4689ab:	mov    %rdx,0x58(%rsp)
  4689b0:	mov    $0x8,%edi
  4689b5:	mov    $0x8,%r15d
  4689bb:	mov    %r14,%rsi
  4689be:	xor    %edx,%edx
  4689c0:	call   4b8b10 <_RNvMs2_NtNtCsHW5vo2gROj_10zopro_core5alloc5linuxNtB5_9Allocator8allocate>
  4689c5:	test   %rax,%rax
  4689c8:	je     468998 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2bf8>
  4689ca:	mov    %rax,%rdx
  4689cd:	mov    $0x1,%eax
  4689d2:	mov    %r14,0x28(%rsp)
  4689d7:	mov    %ebx,%r14d
  4689da:	and    $0x3f,%r14d
  4689de:	mov    %rdx,0x8(%rsp)
  4689e3:	shlx   %r13,%rax,%rbp
  4689e8:	cmp    $0x1,%r14
  4689ec:	je     468b9c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2dfc>
  4689f2:	mov    0x8(%rsp),%rdx
  4689f7:	lea    -0x1(%rbp),%rsi
  4689fb:	lea    0x200(%rsp),%rax
  468a03:	lea    -0x2(%rbp),%rdi
  468a07:	mov    %esi,%ecx
  468a09:	and    $0x7,%ecx
  468a0c:	cmp    $0x7,%rdi
  468a10:	jb     468b65 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2dc5>
  468a16:	mov    0x8(%rsp),%rdx
  468a1b:	and    $0xfffffffffffffff8,%rsi
  468a1f:	vpxor  %xmm0,%xmm0,%xmm0
  468a23:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  468a30:	vmovdqu64 %zmm0,(%rax)
  468a36:	vmovdqu64 %zmm0,0x1c0(%rsp)
  468a3e:	vmovdqu64 %zmm0,(%rdx)
  468a44:	vmovups 0x200(%rsp),%zmm1
  468a4c:	vmovdqu64 %zmm0,(%rax)
  468a52:	vmovdqu64 %zmm0,0x1c0(%rsp)
  468a5a:	vmovups 0x200(%rsp),%zmm2
  468a62:	vmovdqu64 %zmm0,(%rax)
  468a68:	vmovdqu64 %zmm0,0x1c0(%rsp)
  468a70:	vmovups 0x200(%rsp),%zmm3
  468a78:	vmovdqu64 %zmm0,(%rax)
  468a7e:	vmovdqu64 %zmm0,0x1c0(%rsp)
  468a86:	vmovups %zmm1,0x40(%rdx)
  468a8d:	vmovdqu64 %zmm0,0x80(%rdx)
  468a94:	vmovups %zmm2,0xc0(%rdx)
  468a9b:	vmovups 0x200(%rsp),%zmm2
  468aa3:	vmovdqu64 %zmm0,(%rax)
  468aa9:	vmovdqu64 %zmm0,0x1c0(%rsp)
  468ab1:	vmovdqu64 %zmm0,0x100(%rdx)
  468ab8:	vmovups %zmm3,0x140(%rdx)
  468abf:	vmovups 0x200(%rsp),%zmm3
  468ac7:	vmovdqu64 %zmm0,(%rax)
  468acd:	vmovdqu64 %zmm0,0x1c0(%rsp)
  468ad5:	vmovdqu64 %zmm0,0x180(%rdx)
  468adc:	vmovups %zmm2,0x1c0(%rdx)
  468ae3:	vmovups 0x200(%rsp),%zmm2
  468aeb:	vmovdqu64 %zmm0,(%rax)
  468af1:	vmovdqu64 %zmm0,0x1c0(%rsp)
  468af9:	vmovdqu64 %zmm0,0x200(%rdx)
  468b00:	vmovups %zmm3,0x240(%rdx)
  468b07:	vmovups 0x200(%rsp),%zmm3
  468b0f:	vmovdqu64 %zmm0,(%rax)
  468b15:	vmovdqu64 %zmm0,0x280(%rdx)
  468b1c:	vmovdqu64 %zmm0,0x1c0(%rsp)
  468b24:	vmovups %zmm2,0x2c0(%rdx)
  468b2b:	vmovups 0x200(%rsp),%zmm2
  468b33:	vmovdqu64 %zmm0,0x300(%rdx)
  468b3a:	vmovups %zmm3,0x340(%rdx)
  468b41:	vmovdqu64 %zmm0,0x380(%rdx)
  468b48:	vmovups %zmm2,0x3c0(%rdx)
  468b4f:	add    $0x400,%rdx
  468b56:	add    $0xfffffffffffffff8,%rsi
  468b5a:	jne    468a30 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2c90>
  468b60:	test   %rcx,%rcx
  468b63:	je     468b9c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2dfc>
  468b65:	vpxor  %xmm0,%xmm0,%xmm0
  468b69:	nopl   0x0(%rax)
  468b70:	vmovdqu64 %zmm0,(%rax)
  468b76:	vmovdqu64 %zmm0,(%rdx)
  468b7c:	vmovdqu64 %zmm0,0x1c0(%rsp)
  468b84:	vmovups 0x200(%rsp),%zmm1
  468b8c:	vmovups %zmm1,0x40(%rdx)
  468b93:	sub    $0xffffffffffffff80,%rdx
  468b97:	dec    %rcx
  468b9a:	jne    468b70 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2dd0>
  468b9c:	mov    $0x4,%eax
  468ba1:	vpxor  %xmm0,%xmm0,%xmm0
  468ba5:	vmovdqu64 %zmm0,0x40(%rdx)
  468bac:	vmovdqu64 %zmm0,(%rdx)
  468bb2:	shlx   %r13,%rax,%r15
  468bb7:	mov    $0x4,%edi
  468bbc:	mov    %r15,%rsi
  468bbf:	xor    %edx,%edx
  468bc1:	vzeroupper
  468bc4:	call   4b8b10 <_RNvMs2_NtNtCsHW5vo2gROj_10zopro_core5alloc5linuxNtB5_9Allocator8allocate>
  468bc9:	test   %rax,%rax
  468bcc:	je     46a480 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x46e0>
  468bd2:	mov    %rax,%rbx
  468bd5:	cmp    $0x1,%r14d
  468bd9:	je     468bf2 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2e52>
  468bdb:	lea    -0x4(%r15),%rdx
  468bdf:	mov    $0xff,%esi
  468be4:	mov    %rbx,%rdi
  468be7:	call   *0x7e96b(%rip)        # 4e7558 <memset@GLIBC_2.2.5>
  468bed:	lea    -0x4(%rbx,%r15,1),%rax
  468bf2:	mov    %r15,0x18(%rsp)
  468bf7:	lea    0x3f(%rbp),%r15
  468bfb:	mov    %r13,0x68(%rsp)
  468c00:	movl   $0xffffffff,(%rax)
  468c06:	mov    %rbp,0x88(%rsp)
  468c0e:	shr    $0x6,%r15
  468c12:	lea    0x0(,%r15,8),%rsi
  468c1a:	mov    $0x8,%edi
  468c1f:	mov    $0x1,%edx
  468c24:	mov    %rsi,0x20(%rsp)
  468c29:	call   4b8b10 <_RNvMs2_NtNtCsHW5vo2gROj_10zopro_core5alloc5linuxNtB5_9Allocator8allocate>
  468c2e:	mov    %rax,0x10(%rsp)
  468c33:	test   %rax,%rax
  468c36:	je     46a48f <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x46ef>
  468c3c:	mov    0x58(%rsp),%r14
  468c41:	shl    $0x4,%r14d
  468c45:	mov    $0x8,%edi
  468c4a:	mov    %r14,%rsi
  468c4d:	xor    %edx,%edx
  468c4f:	call   4b8b10 <_RNvMs2_NtNtCsHW5vo2gROj_10zopro_core5alloc5linuxNtB5_9Allocator8allocate>
  468c54:	mov    0x60(%rsp),%r13
  468c59:	test   %rax,%rax
  468c5c:	je     46a4a0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4700>
  468c62:	mov    %rax,%rbp
  468c65:	mov    $0x8,%edi
  468c6a:	mov    %r14,%rsi
  468c6d:	xor    %edx,%edx
  468c6f:	call   4b8b10 <_RNvMs2_NtNtCsHW5vo2gROj_10zopro_core5alloc5linuxNtB5_9Allocator8allocate>
  468c74:	test   %rax,%rax
  468c77:	je     46a4af <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x470f>
  468c7d:	mov    0x88(%rsp),%rcx
  468c85:	mov    0x8(%rsp),%rdx
  468c8a:	mov    0x10(%rsp),%rdi
  468c8f:	mov    %rcx,0x7d0(%rsp)
  468c97:	mov    %rdx,0x7d8(%rsp)
  468c9f:	mov    %rcx,0x7e0(%rsp)
  468ca7:	movq   $0x0,0x7e8(%rsp)
  468cb3:	movq   $0x8,0x7f0(%rsp)
  468cbf:	movq   $0x0,0x7f8(%rsp)
  468ccb:	mov    %rcx,0x800(%rsp)
  468cd3:	mov    %rbx,0x808(%rsp)
  468cdb:	mov    0x58(%rsp),%rbx
  468ce0:	mov    %rcx,0x810(%rsp)
  468ce8:	movq   $0x0,0x818(%rsp)
  468cf4:	movq   $0x8,0x820(%rsp)
  468d00:	movq   $0x0,0x828(%rsp)
  468d0c:	mov    %r15,0x830(%rsp)
  468d14:	mov    %rdi,0x838(%rsp)
  468d1c:	mov    %r15,0x840(%rsp)
  468d24:	lea    0x878(%rsp),%rdi
  468d2c:	mov    %rbx,0x848(%rsp)
  468d34:	mov    %rbp,0x850(%rsp)
  468d3c:	movq   $0x0,0x858(%rsp)
  468d48:	mov    %rbx,0x860(%rsp)
  468d50:	mov    %rax,0x868(%rsp)
  468d58:	movq   $0x0,0x870(%rsp)
  468d64:	lea    0xde0(%rsp),%rsi
  468d6c:	mov    $0x528,%edx
  468d71:	call   *0x7e741(%rip)        # 4e74b8 <memcpy@GLIBC_2.14>
  468d77:	sub    $0x8,%rsp
  468d7b:	mov    0xa0(%rsp),%rdx
  468d83:	lea    0x7d8(%rsp),%rdi
  468d8b:	mov    %r12,%rsi
  468d8e:	mov    %r13,%rcx
  468d91:	mov    $0x1,%r9d
  468d97:	mov    %rdx,%r8
  468d9a:	push   %rbx
  468d9b:	push   0x88(%rsp)
  468da2:	push   0x88(%rsp)
  468da9:	call   46a910 <_RNvMNtNtCsHW5vo2gROj_10zopro_core10msm_affine11g2_borrowedNtB2_6Window4feed>
  468dae:	add    $0x20,%rsp
  468db2:	mov    0x7e0(%rsp),%rdx
  468dba:	movabs $0xcf9bb18d1ece5fd7,%rcx
  468dc4:	movabs $0x97816a916871ca8d,%rdi
  468dce:	test   %rdx,%rdx
  468dd1:	je     469611 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x3871>
  468dd7:	mov    0x7d8(%rsp),%r14
  468ddf:	shl    $0x7,%rdx
  468de3:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  468df0:	vmovdqu (%r14),%ymm0
  468df5:	vptest %ymm0,%ymm0
  468dfa:	jne    468e30 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x3090>
  468dfc:	vmovdqu 0x20(%r14),%ymm0
  468e02:	vptest %ymm0,%ymm0
  468e07:	jne    468e30 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x3090>
  468e09:	vmovdqu 0x40(%r14),%ymm0
  468e0f:	vptest %ymm0,%ymm0
  468e14:	jne    468e30 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x3090>
  468e16:	vmovdqu 0x60(%r14),%ymm0
  468e1c:	vptest %ymm0,%ymm0
  468e21:	je     469603 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x3863>
  468e27:	nopw   0x0(%rax,%rax,1)
  468e30:	vmovdqu (%r14),%ymm0
  468e35:	vmovups -0x4571bd(%rip),%ymm3        # 11c80 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0x200>
  468e3d:	vmovups -0x4572e5(%rip),%ymm1        # 11b60 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0xe0>
  468e45:	mov    %rdx,0x10(%rsp)
  468e4a:	lea    0x100(%rsp),%rsi
  468e52:	lea    0x1c0(%rsp),%rbp
  468e5a:	movabs $0x87d20782e4866389,%rbx
  468e64:	lea    0xd0(%rsp),%r15
  468e6c:	vmovdqu %ymm0,0xd0(%rsp)
  468e75:	vmovups %ymm3,0x100(%rsp)
  468e7e:	vmovups %ymm1,0x1c0(%rsp)
  468e87:	xor    %r13d,%r13d
  468e8a:	xor    %r12d,%r12d
  468e8d:	xor    %ecx,%ecx
  468e8f:	xor    %eax,%eax
  468e91:	xor    %r8d,%r8d
  468e94:	mov    (%rsi),%rdx
  468e97:	xor    %r11d,%r11d
  468e9a:	mulx   (%r15),%r9,%r10
  468e9f:	adcx   %r9,%r13
  468ea5:	adox   %r10,%r12
  468eab:	mulx   0x8(%r15),%r9,%r10
  468eb1:	adcx   %r9,%r12
  468eb7:	adox   %r10,%rcx
  468ebd:	mulx   0x10(%r15),%r9,%r10
  468ec3:	adcx   %r9,%rcx
  468ec9:	adox   %r10,%rax
  468ecf:	mulx   0x18(%r15),%r9,%r10
  468ed5:	adcx   %r9,%rax
  468edb:	adox   %r10,%r8
  468ee1:	adcx   %r11,%r8
  468ee7:	mov    %r13,%rdx
  468eea:	imul   %rbx,%rdx
  468eee:	xor    %r11d,%r11d
  468ef1:	mulx   0x0(%rbp),%r9,%r10
  468ef7:	adcx   %r9,%r13
  468efd:	adox   %r10,%r12
  468f03:	mulx   0x8(%rbp),%r9,%r10
  468f09:	adcx   %r9,%r12
  468f0f:	adox   %r10,%rcx
  468f15:	mulx   0x10(%rbp),%r9,%r10
  468f1b:	adcx   %r9,%rcx
  468f21:	adox   %r10,%rax
  468f27:	mulx   0x18(%rbp),%r9,%r10
  468f2d:	adcx   %r9,%rax
  468f33:	adox   %r10,%r8
  468f39:	adcx   %r11,%r8
  468f3f:	mov    0x8(%rsi),%rdx
  468f43:	xor    %r11d,%r11d
  468f46:	mulx   (%r15),%r9,%r10
  468f4b:	adcx   %r9,%r12
  468f51:	adox   %r10,%rcx
  468f57:	mulx   0x8(%r15),%r9,%r10
  468f5d:	adcx   %r9,%rcx
  468f63:	adox   %r10,%rax
  468f69:	mulx   0x10(%r15),%r9,%r10
  468f6f:	adcx   %r9,%rax
  468f75:	adox   %r10,%r8
  468f7b:	mulx   0x18(%r15),%r9,%r10
  468f81:	adcx   %r9,%r8
  468f87:	adox   %r10,%r13
  468f8d:	adcx   %r11,%r13
  468f93:	mov    %r12,%rdx
  468f96:	imul   %rbx,%rdx
  468f9a:	xor    %r11d,%r11d
  468f9d:	mulx   0x0(%rbp),%r9,%r10
  468fa3:	adcx   %r9,%r12
  468fa9:	adox   %r10,%rcx
  468faf:	mulx   0x8(%rbp),%r9,%r10
  468fb5:	adcx   %r9,%rcx
  468fbb:	adox   %r10,%rax
  468fc1:	mulx   0x10(%rbp),%r9,%r10
  468fc7:	adcx   %r9,%rax
  468fcd:	adox   %r10,%r8
  468fd3:	mulx   0x18(%rbp),%r9,%r10
  468fd9:	adcx   %r9,%r8
  468fdf:	adox   %r10,%r13
  468fe5:	adcx   %r11,%r13
  468feb:	mov    0x10(%rsi),%rdx
  468fef:	xor    %r11d,%r11d
  468ff2:	mulx   (%r15),%r9,%r10
  468ff7:	adcx   %r9,%rcx
  468ffd:	adox   %r10,%rax
  469003:	mulx   0x8(%r15),%r9,%r10
  469009:	adcx   %r9,%rax
  46900f:	adox   %r10,%r8
  469015:	mulx   0x10(%r15),%r9,%r10
  46901b:	adcx   %r9,%r8
  469021:	adox   %r10,%r13
  469027:	mulx   0x18(%r15),%r9,%r10
  46902d:	adcx   %r9,%r13
  469033:	adox   %r10,%r12
  469039:	adcx   %r11,%r12
  46903f:	mov    %rcx,%rdx
  469042:	imul   %rbx,%rdx
  469046:	xor    %r11d,%r11d
  469049:	mulx   0x0(%rbp),%r9,%r10
  46904f:	adcx   %r9,%rcx
  469055:	adox   %r10,%rax
  46905b:	mulx   0x8(%rbp),%r9,%r10
  469061:	adcx   %r9,%rax
  469067:	adox   %r10,%r8
  46906d:	mulx   0x10(%rbp),%r9,%r10
  469073:	adcx   %r9,%r8
  469079:	adox   %r10,%r13
  46907f:	mulx   0x18(%rbp),%r9,%r10
  469085:	adcx   %r9,%r13
  46908b:	adox   %r10,%r12
  469091:	adcx   %r11,%r12
  469097:	mov    0x18(%rsi),%rdx
  46909b:	xor    %r11d,%r11d
  46909e:	mulx   (%r15),%r9,%r10
  4690a3:	adcx   %r9,%rax
  4690a9:	adox   %r10,%r8
  4690af:	mulx   0x8(%r15),%r9,%r10
  4690b5:	adcx   %r9,%r8
  4690bb:	adox   %r10,%r13
  4690c1:	mulx   0x10(%r15),%r9,%r10
  4690c7:	adcx   %r9,%r13
  4690cd:	adox   %r10,%r12
  4690d3:	mulx   0x18(%r15),%r9,%r10
  4690d9:	adcx   %r9,%r12
  4690df:	adox   %r10,%rcx
  4690e5:	adcx   %r11,%rcx
  4690eb:	mov    %rax,%rdx
  4690ee:	imul   %rbx,%rdx
  4690f2:	xor    %r11d,%r11d
  4690f5:	mulx   0x0(%rbp),%r9,%r10
  4690fb:	adcx   %r9,%rax
  469101:	adox   %r10,%r8
  469107:	mulx   0x8(%rbp),%r9,%r10
  46910d:	adcx   %r9,%r8
  469113:	adox   %r10,%r13
  469119:	mulx   0x10(%rbp),%r9,%r10
  46911f:	adcx   %r9,%r13
  469125:	adox   %r10,%r12
  46912b:	mulx   0x18(%rbp),%r9,%r10
  469131:	adcx   %r9,%r12
  469137:	adox   %r10,%rcx
  46913d:	adcx   %r11,%rcx
  469143:	movabs $0x687e956e978e3573,%rax
  46914d:	mov    %r13,0x28(%rsp)
  469152:	mov    %rcx,0x8(%rsp)
  469157:	mov    %r8,0x18(%rsp)
  46915c:	mov    %r12,0x20(%rsp)
  469161:	lea    0x1c0(%rsp),%r11
  469169:	lea    0x0(%r13,%rax,1),%rsi
  46916e:	movabs $0x3c208c16d87cfd47,%rax
  469178:	vmovups 0x20(%r14),%ymm2
  46917e:	vmovups %ymm3,0x100(%rsp)
  469187:	vmovups %ymm1,0x1c0(%rsp)
  469190:	cmp    %rax,%r8
  469193:	lea    0xd0(%rsp),%r8
  46919b:	sbb    $0x0,%rsi
  46919f:	setb   %al
  4691a2:	cmp    %rdi,%r13
  4691a5:	movabs $0xb85045b68181585d,%rdi
  4691af:	mov    %rsi,0x48(%rsp)
  4691b4:	setb   %dl
  4691b7:	or     %al,%dl
  4691b9:	movabs $0x47afba497e7ea7a3,%rax
  4691c3:	add    %r12,%rax
  4691c6:	movzbl %dl,%edx
  4691c9:	sub    %rdx,%rax
  4691cc:	setb   %dl
  4691cf:	cmp    %rdi,%r12
  4691d2:	mov    %rax,0x38(%rsp)
  4691d7:	movabs $0xcf9bb18d1ece5fd7,%rax
  4691e1:	vmovups %ymm2,0xd0(%rsp)
  4691ea:	lea    0x100(%rsp),%rdi
  4691f2:	setb   %r10b
  4691f6:	lea    (%rcx,%rax,1),%r13
  4691fa:	or     %dl,%r10b
  4691fd:	movzbl %r10b,%edx
  469201:	movabs $0x87d20782e4866389,%r10
  46920b:	sub    %rdx,%r13
  46920e:	setb   0x30(%rsp)
  469213:	xor    %eax,%eax
  469215:	xor    %r15d,%r15d
  469218:	xor    %ebp,%ebp
  46921a:	xor    %ebx,%ebx
  46921c:	xor    %r9d,%r9d
  46921f:	mov    (%rdi),%rdx
  469222:	xor    %esi,%esi
  469224:	mulx   (%r8),%r12,%rcx
  469229:	adcx   %r12,%rax
  46922f:	adox   %rcx,%r15
  469235:	mulx   0x8(%r8),%r12,%rcx
  46923b:	adcx   %r12,%r15
  469241:	adox   %rcx,%rbp
  469247:	mulx   0x10(%r8),%r12,%rcx
  46924d:	adcx   %r12,%rbp
  469253:	adox   %rcx,%rbx
  469259:	mulx   0x18(%r8),%r12,%rcx
  46925f:	adcx   %r12,%rbx
  469265:	adox   %rcx,%r9
  46926b:	adcx   %rsi,%r9
  469271:	mov    %rax,%rdx
  469274:	imul   %r10,%rdx
  469278:	xor    %esi,%esi
  46927a:	mulx   (%r11),%r12,%rcx
  46927f:	adcx   %r12,%rax
  469285:	adox   %rcx,%r15
  46928b:	mulx   0x8(%r11),%r12,%rcx
  469291:	adcx   %r12,%r15
  469297:	adox   %rcx,%rbp
  46929d:	mulx   0x10(%r11),%r12,%rcx
  4692a3:	adcx   %r12,%rbp
  4692a9:	adox   %rcx,%rbx
  4692af:	mulx   0x18(%r11),%r12,%rcx
  4692b5:	adcx   %r12,%rbx
  4692bb:	adox   %rcx,%r9
  4692c1:	adcx   %rsi,%r9
  4692c7:	mov    0x8(%rdi),%rdx
  4692cb:	xor    %esi,%esi
  4692cd:	mulx   (%r8),%r12,%rcx
  4692d2:	adcx   %r12,%r15
  4692d8:	adox   %rcx,%rbp
  4692de:	mulx   0x8(%r8),%r12,%rcx
  4692e4:	adcx   %r12,%rbp
  4692ea:	adox   %rcx,%rbx
  4692f0:	mulx   0x10(%r8),%r12,%rcx
  4692f6:	adcx   %r12,%rbx
  4692fc:	adox   %rcx,%r9
  469302:	mulx   0x18(%r8),%r12,%rcx
  469308:	adcx   %r12,%r9
  46930e:	adox   %rcx,%rax
  469314:	adcx   %rsi,%rax
  46931a:	mov    %r15,%rdx
  46931d:	imul   %r10,%rdx
  469321:	xor    %esi,%esi
  469323:	mulx   (%r11),%r12,%rcx
  469328:	adcx   %r12,%r15
  46932e:	adox   %rcx,%rbp
  469334:	mulx   0x8(%r11),%r12,%rcx
  46933a:	adcx   %r12,%rbp
  469340:	adox   %rcx,%rbx
  469346:	mulx   0x10(%r11),%r12,%rcx
  46934c:	adcx   %r12,%rbx
  469352:	adox   %rcx,%r9
  469358:	mulx   0x18(%r11),%r12,%rcx
  46935e:	adcx   %r12,%r9
  469364:	adox   %rcx,%rax
  46936a:	adcx   %rsi,%rax
  469370:	mov    0x10(%rdi),%rdx
  469374:	xor    %esi,%esi
  469376:	mulx   (%r8),%r12,%rcx
  46937b:	adcx   %r12,%rbp
  469381:	adox   %rcx,%rbx
  469387:	mulx   0x8(%r8),%r12,%rcx
  46938d:	adcx   %r12,%rbx
  469393:	adox   %rcx,%r9
  469399:	mulx   0x10(%r8),%r12,%rcx
  46939f:	adcx   %r12,%r9
  4693a5:	adox   %rcx,%rax
  4693ab:	mulx   0x18(%r8),%r12,%rcx
  4693b1:	adcx   %r12,%rax
  4693b7:	adox   %rcx,%r15
  4693bd:	adcx   %rsi,%r15
  4693c3:	mov    %rbp,%rdx
  4693c6:	imul   %r10,%rdx
  4693ca:	xor    %esi,%esi
  4693cc:	mulx   (%r11),%r12,%rcx
  4693d1:	adcx   %r12,%rbp
  4693d7:	adox   %rcx,%rbx
  4693dd:	mulx   0x8(%r11),%r12,%rcx
  4693e3:	adcx   %r12,%rbx
  4693e9:	adox   %rcx,%r9
  4693ef:	mulx   0x10(%r11),%r12,%rcx
  4693f5:	adcx   %r12,%r9
  4693fb:	adox   %rcx,%rax
  469401:	mulx   0x18(%r11),%r12,%rcx
  469407:	adcx   %r12,%rax
  46940d:	adox   %rcx,%r15
  469413:	adcx   %rsi,%r15
  469419:	mov    0x18(%rdi),%rdx
  46941d:	xor    %esi,%esi
  46941f:	mulx   (%r8),%r12,%rcx
  469424:	adcx   %r12,%rbx
  46942a:	adox   %rcx,%r9
  469430:	mulx   0x8(%r8),%r12,%rcx
  469436:	adcx   %r12,%r9
  46943c:	adox   %rcx,%rax
  469442:	mulx   0x10(%r8),%r12,%rcx
  469448:	adcx   %r12,%rax
  46944e:	adox   %rcx,%r15
  469454:	mulx   0x18(%r8),%r12,%rcx
  46945a:	adcx   %r12,%r15
  469460:	adox   %rcx,%rbp
  469466:	adcx   %rsi,%rbp
  46946c:	mov    %rbx,%rdx
  46946f:	imul   %r10,%rdx
  469473:	xor    %esi,%esi
  469475:	mulx   (%r11),%r12,%rcx
  46947a:	adcx   %r12,%rbx
  469480:	adox   %rcx,%r9
  469486:	mulx   0x8(%r11),%r12,%rcx
  46948c:	adcx   %r12,%r9
  469492:	adox   %rcx,%rax
  469498:	mulx   0x10(%r11),%r12,%rcx
  46949e:	adcx   %r12,%rax
  4694a4:	adox   %rcx,%r15
  4694aa:	mulx   0x18(%r11),%r12,%rcx
  4694b0:	adcx   %r12,%r15
  4694b6:	adox   %rcx,%rbp
  4694bc:	adcx   %rsi,%rbp
  4694c2:	movabs $0x687e956e978e3573,%rcx
  4694cc:	movabs $0x97816a916871ca8d,%rsi
  4694d6:	mov    %rbp,0x40(%rsp)
  4694db:	mov    %r9,%rbp
  4694de:	mov    %rax,%r11
  4694e1:	lea    (%rax,%rcx,1),%rdx
  4694e5:	movabs $0x3c208c16d87cfd47,%rcx
  4694ef:	mov    0x40(%rsp),%r8
  4694f4:	mov    0x18(%rsp),%rdi
  4694f9:	cmp    %rcx,%r9
  4694fc:	mov    0x38(%rsp),%r9
  469501:	sbb    $0x0,%rdx
  469505:	setb   %cl
  469508:	cmp    %rsi,%rax
  46950b:	mov    0x48(%rsp),%rax
  469510:	setb   %sil
  469514:	or     %cl,%sil
  469517:	movabs $0x47afba497e7ea7a3,%rcx
  469521:	lea    (%r15,%rcx,1),%r12
  469525:	movzbl %sil,%ecx
  469529:	movabs $0xb85045b68181585d,%rsi
  469533:	sub    %rcx,%r12
  469536:	setb   %cl
  469539:	cmp    %rsi,%r15
  46953c:	setb   %sil
  469540:	or     %cl,%sil
  469543:	movabs $0xcf9bb18d1ece5fd7,%rcx
  46954d:	lea    (%r8,%rcx,1),%rbx
  469551:	movzbl %sil,%ecx
  469555:	movabs $0x30644e72e131a029,%rsi
  46955f:	sub    %rcx,%rbx
  469562:	setb   %cl
  469565:	cmp    %rsi,%r8
  469568:	setb   %sil
  46956c:	or     %cl,%sil
  46956f:	movabs $0x30644e72e131a029,%rcx
  469579:	cmp    %rcx,0x8(%rsp)
  46957e:	setb   %cl
  469581:	or     0x30(%rsp),%cl
  469585:	movabs $0xc3df73e9278302b9,%rcx
  46958f:	lea    (%rdi,%rcx,1),%rcx
  469593:	cmovne 0x28(%rsp),%rax
  469599:	cmovne 0x20(%rsp),%r9
  46959f:	cmovne 0x8(%rsp),%r13
  4695a5:	cmovne %rdi,%rcx
  4695a9:	movabs $0xc3df73e9278302b9,%rdi
  4695b3:	test   %sil,%sil
  4695b6:	lea    0x0(%rbp,%rdi,1),%rsi
  4695bb:	cmovne %r11,%rdx
  4695bf:	mov    %rcx,(%r14)
  4695c2:	cmovne %r15,%r12
  4695c6:	cmovne %r8,%rbx
  4695ca:	movabs $0xcf9bb18d1ece5fd7,%rcx
  4695d4:	movabs $0x97816a916871ca8d,%rdi
  4695de:	cmovne %rbp,%rsi
  4695e2:	mov    %rax,0x8(%r14)
  4695e6:	mov    %r9,0x10(%r14)
  4695ea:	mov    %r13,0x18(%r14)
  4695ee:	mov    %rsi,0x20(%r14)
  4695f2:	mov    %rdx,0x28(%r14)
  4695f6:	mov    0x10(%rsp),%rdx
  4695fb:	mov    %r12,0x30(%r14)
  4695ff:	mov    %rbx,0x38(%r14)
  469603:	sub    $0xffffffffffffff80,%r14
  469607:	add    $0xffffffffffffff80,%rdx
  46960b:	jne    468df0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x3050>
  469611:	mov    0x7f8(%rsp),%rax
  469619:	test   %rax,%rax
  46961c:	je     469e02 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4062>
  469622:	vmovdqu -0x4579aa(%rip),%ymm0        # 11c80 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0x200>
  46962a:	vmovups -0x457ad2(%rip),%ymm1        # 11b60 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0xe0>
  469632:	mov    0x7f0(%rsp),%rbx
  46963a:	lea    (%rax,%rax,2),%rax
  46963e:	mov    %rcx,%r14
  469641:	shl    $0x6,%rax
  469645:	add    %rbx,%rax
  469648:	mov    %rax,0x50(%rsp)
  46964d:	nopl   (%rax)
  469650:	vmovups (%rbx),%ymm2
  469654:	vmovdqu %ymm0,0x100(%rsp)
  46965d:	vmovups %ymm1,0x1c0(%rsp)
  469666:	movabs $0x87d20782e4866389,%r15
  469670:	lea    0xd0(%rsp),%rsi
  469678:	lea    0x100(%rsp),%rbp
  469680:	lea    0x1c0(%rsp),%rdi
  469688:	vmovups %ymm2,0xd0(%rsp)
  469691:	xor    %r8d,%r8d
  469694:	xor    %r12d,%r12d
  469697:	xor    %r13d,%r13d
  46969a:	xor    %eax,%eax
  46969c:	xor    %r9d,%r9d
  46969f:	mov    0x0(%rbp),%rdx
  4696a3:	xor    %r11d,%r11d
  4696a6:	mulx   (%rsi),%rcx,%r10
  4696ab:	adcx   %rcx,%r8
  4696b1:	adox   %r10,%r12
  4696b7:	mulx   0x8(%rsi),%rcx,%r10
  4696bd:	adcx   %rcx,%r12
  4696c3:	adox   %r10,%r13
  4696c9:	mulx   0x10(%rsi),%rcx,%r10
  4696cf:	adcx   %rcx,%r13
  4696d5:	adox   %r10,%rax
  4696db:	mulx   0x18(%rsi),%rcx,%r10
  4696e1:	adcx   %rcx,%rax
  4696e7:	adox   %r10,%r9
  4696ed:	adcx   %r11,%r9
  4696f3:	mov    %r8,%rdx
  4696f6:	imul   %r15,%rdx
  4696fa:	xor    %r11d,%r11d
  4696fd:	mulx   (%rdi),%rcx,%r10
  469702:	adcx   %rcx,%r8
  469708:	adox   %r10,%r12
  46970e:	mulx   0x8(%rdi),%rcx,%r10
  469714:	adcx   %rcx,%r12
  46971a:	adox   %r10,%r13
  469720:	mulx   0x10(%rdi),%rcx,%r10
  469726:	adcx   %rcx,%r13
  46972c:	adox   %r10,%rax
  469732:	mulx   0x18(%rdi),%rcx,%r10
  469738:	adcx   %rcx,%rax
  46973e:	adox   %r10,%r9
  469744:	adcx   %r11,%r9
  46974a:	mov    0x8(%rbp),%rdx
  46974e:	xor    %r11d,%r11d
  469751:	mulx   (%rsi),%rcx,%r10
  469756:	adcx   %rcx,%r12
  46975c:	adox   %r10,%r13
  469762:	mulx   0x8(%rsi),%rcx,%r10
  469768:	adcx   %rcx,%r13
  46976e:	adox   %r10,%rax
  469774:	mulx   0x10(%rsi),%rcx,%r10
  46977a:	adcx   %rcx,%rax
  469780:	adox   %r10,%r9
  469786:	mulx   0x18(%rsi),%rcx,%r10
  46978c:	adcx   %rcx,%r9
  469792:	adox   %r10,%r8
  469798:	adcx   %r11,%r8
  46979e:	mov    %r12,%rdx
  4697a1:	imul   %r15,%rdx
  4697a5:	xor    %r11d,%r11d
  4697a8:	mulx   (%rdi),%rcx,%r10
  4697ad:	adcx   %rcx,%r12
  4697b3:	adox   %r10,%r13
  4697b9:	mulx   0x8(%rdi),%rcx,%r10
  4697bf:	adcx   %rcx,%r13
  4697c5:	adox   %r10,%rax
  4697cb:	mulx   0x10(%rdi),%rcx,%r10
  4697d1:	adcx   %rcx,%rax
  4697d7:	adox   %r10,%r9
  4697dd:	mulx   0x18(%rdi),%rcx,%r10
  4697e3:	adcx   %rcx,%r9
  4697e9:	adox   %r10,%r8
  4697ef:	adcx   %r11,%r8
  4697f5:	mov    0x10(%rbp),%rdx
  4697f9:	xor    %r11d,%r11d
  4697fc:	mulx   (%rsi),%rcx,%r10
  469801:	adcx   %rcx,%r13
  469807:	adox   %r10,%rax
  46980d:	mulx   0x8(%rsi),%rcx,%r10
  469813:	adcx   %rcx,%rax
  469819:	adox   %r10,%r9
  46981f:	mulx   0x10(%rsi),%rcx,%r10
  469825:	adcx   %rcx,%r9
  46982b:	adox   %r10,%r8
  469831:	mulx   0x18(%rsi),%rcx,%r10
  469837:	adcx   %rcx,%r8
  46983d:	adox   %r10,%r12
  469843:	adcx   %r11,%r12
  469849:	mov    %r13,%rdx
  46984c:	imul   %r15,%rdx
  469850:	xor    %r11d,%r11d
  469853:	mulx   (%rdi),%rcx,%r10
  469858:	adcx   %rcx,%r13
  46985e:	adox   %r10,%rax
  469864:	mulx   0x8(%rdi),%rcx,%r10
  46986a:	adcx   %rcx,%rax
  469870:	adox   %r10,%r9
  469876:	mulx   0x10(%rdi),%rcx,%r10
  46987c:	adcx   %rcx,%r9
  469882:	adox   %r10,%r8
  469888:	mulx   0x18(%rdi),%rcx,%r10
  46988e:	adcx   %rcx,%r8
  469894:	adox   %r10,%r12
  46989a:	adcx   %r11,%r12
  4698a0:	mov    0x18(%rbp),%rdx
  4698a4:	xor    %r11d,%r11d
  4698a7:	mulx   (%rsi),%rcx,%r10
  4698ac:	adcx   %rcx,%rax
  4698b2:	adox   %r10,%r9
  4698b8:	mulx   0x8(%rsi),%rcx,%r10
  4698be:	adcx   %rcx,%r9
  4698c4:	adox   %r10,%r8
  4698ca:	mulx   0x10(%rsi),%rcx,%r10
  4698d0:	adcx   %rcx,%r8
  4698d6:	adox   %r10,%r12
  4698dc:	mulx   0x18(%rsi),%rcx,%r10
  4698e2:	adcx   %rcx,%r12
  4698e8:	adox   %r10,%r13
  4698ee:	adcx   %r11,%r13
  4698f4:	mov    %rax,%rdx
  4698f7:	imul   %r15,%rdx
  4698fb:	xor    %r11d,%r11d
  4698fe:	mulx   (%rdi),%rcx,%r10
  469903:	adcx   %rcx,%rax
  469909:	adox   %r10,%r9
  46990f:	mulx   0x8(%rdi),%rcx,%r10
  469915:	adcx   %rcx,%r9
  46991b:	adox   %r10,%r8
  469921:	mulx   0x10(%rdi),%rcx,%r10
  469927:	adcx   %rcx,%r8
  46992d:	adox   %r10,%r12
  469933:	mulx   0x18(%rdi),%rcx,%r10
  469939:	adcx   %rcx,%r12
  46993f:	adox   %r10,%r13
  469945:	adcx   %r11,%r13
  46994b:	movabs $0x687e956e978e3573,%rcx
  469955:	movabs $0x47afba497e7ea7a3,%rax
  46995f:	mov    %r12,0x10(%rsp)
  469964:	mov    %r13,0x30(%rsp)
  469969:	mov    %r9,0x20(%rsp)
  46996e:	mov    %r8,0x8(%rsp)
  469973:	lea    (%r8,%rcx,1),%rdx
  469977:	movabs $0x3c208c16d87cfd47,%rcx
  469981:	vmovups 0x20(%rbx),%ymm2
  469986:	add    %r12,%rax
  469989:	vmovdqu %ymm0,0x100(%rsp)
  469992:	vmovups %ymm1,0x1c0(%rsp)
  46999b:	cmp    %rcx,%r9
  46999e:	sbb    $0x0,%rdx
  4699a2:	mov    %rdx,0x28(%rsp)
  4699a7:	movabs $0x97816a916871ca8d,%rdx
  4699b1:	setb   %cl
  4699b4:	cmp    %rdx,%r8
  4699b7:	lea    0x100(%rsp),%r8
  4699bf:	setb   %dl
  4699c2:	or     %cl,%dl
  4699c4:	movabs $0xb85045b68181585d,%rcx
  4699ce:	movzbl %dl,%edx
  4699d1:	sub    %rdx,%rax
  4699d4:	vmovups %ymm2,0xd0(%rsp)
  4699dd:	setb   %dl
  4699e0:	cmp    %rcx,%r12
  4699e3:	lea    0x0(%r13,%r14,1),%r12
  4699e8:	mov    %rax,0x40(%rsp)
  4699ed:	lea    0xd0(%rsp),%rax
  4699f5:	lea    0x1c0(%rsp),%rcx
  4699fd:	setb   %r11b
  469a01:	or     %dl,%r11b
  469a04:	movzbl %r11b,%edx
  469a08:	movabs $0x87d20782e4866389,%r11
  469a12:	sub    %rdx,%r12
  469a15:	setb   0x18(%rsp)
  469a1a:	xor    %r10d,%r10d
  469a1d:	xor    %ebp,%ebp
  469a1f:	xor    %r15d,%r15d
  469a22:	xor    %r14d,%r14d
  469a25:	xor    %r9d,%r9d
  469a28:	mov    (%r8),%rdx
  469a2b:	xor    %edi,%edi
  469a2d:	mulx   (%rax),%r13,%rsi
  469a32:	adcx   %r13,%r10
  469a38:	adox   %rsi,%rbp
  469a3e:	mulx   0x8(%rax),%r13,%rsi
  469a44:	adcx   %r13,%rbp
  469a4a:	adox   %rsi,%r15
  469a50:	mulx   0x10(%rax),%r13,%rsi
  469a56:	adcx   %r13,%r15
  469a5c:	adox   %rsi,%r14
  469a62:	mulx   0x18(%rax),%r13,%rsi
  469a68:	adcx   %r13,%r14
  469a6e:	adox   %rsi,%r9
  469a74:	adcx   %rdi,%r9
  469a7a:	mov    %r10,%rdx
  469a7d:	imul   %r11,%rdx
  469a81:	xor    %edi,%edi
  469a83:	mulx   (%rcx),%r13,%rsi
  469a88:	adcx   %r13,%r10
  469a8e:	adox   %rsi,%rbp
  469a94:	mulx   0x8(%rcx),%r13,%rsi
  469a9a:	adcx   %r13,%rbp
  469aa0:	adox   %rsi,%r15
  469aa6:	mulx   0x10(%rcx),%r13,%rsi
  469aac:	adcx   %r13,%r15
  469ab2:	adox   %rsi,%r14
  469ab8:	mulx   0x18(%rcx),%r13,%rsi
  469abe:	adcx   %r13,%r14
  469ac4:	adox   %rsi,%r9
  469aca:	adcx   %rdi,%r9
  469ad0:	mov    0x8(%r8),%rdx
  469ad4:	xor    %edi,%edi
  469ad6:	mulx   (%rax),%r13,%rsi
  469adb:	adcx   %r13,%rbp
  469ae1:	adox   %rsi,%r15
  469ae7:	mulx   0x8(%rax),%r13,%rsi
  469aed:	adcx   %r13,%r15
  469af3:	adox   %rsi,%r14
  469af9:	mulx   0x10(%rax),%r13,%rsi
  469aff:	adcx   %r13,%r14
  469b05:	adox   %rsi,%r9
  469b0b:	mulx   0x18(%rax),%r13,%rsi
  469b11:	adcx   %r13,%r9
  469b17:	adox   %rsi,%r10
  469b1d:	adcx   %rdi,%r10
  469b23:	mov    %rbp,%rdx
  469b26:	imul   %r11,%rdx
  469b2a:	xor    %edi,%edi
  469b2c:	mulx   (%rcx),%r13,%rsi
  469b31:	adcx   %r13,%rbp
  469b37:	adox   %rsi,%r15
  469b3d:	mulx   0x8(%rcx),%r13,%rsi
  469b43:	adcx   %r13,%r15
  469b49:	adox   %rsi,%r14
  469b4f:	mulx   0x10(%rcx),%r13,%rsi
  469b55:	adcx   %r13,%r14
  469b5b:	adox   %rsi,%r9
  469b61:	mulx   0x18(%rcx),%r13,%rsi
  469b67:	adcx   %r13,%r9
  469b6d:	adox   %rsi,%r10
  469b73:	adcx   %rdi,%r10
  469b79:	mov    0x10(%r8),%rdx
  469b7d:	xor    %edi,%edi
  469b7f:	mulx   (%rax),%r13,%rsi
  469b84:	adcx   %r13,%r15
  469b8a:	adox   %rsi,%r14
  469b90:	mulx   0x8(%rax),%r13,%rsi
  469b96:	adcx   %r13,%r14
  469b9c:	adox   %rsi,%r9
  469ba2:	mulx   0x10(%rax),%r13,%rsi
  469ba8:	adcx   %r13,%r9
  469bae:	adox   %rsi,%r10
  469bb4:	mulx   0x18(%rax),%r13,%rsi
  469bba:	adcx   %r13,%r10
  469bc0:	adox   %rsi,%rbp
  469bc6:	adcx   %rdi,%rbp
  469bcc:	mov    %r15,%rdx
  469bcf:	imul   %r11,%rdx
  469bd3:	xor    %edi,%edi
  469bd5:	mulx   (%rcx),%r13,%rsi
  469bda:	adcx   %r13,%r15
  469be0:	adox   %rsi,%r14
  469be6:	mulx   0x8(%rcx),%r13,%rsi
  469bec:	adcx   %r13,%r14
  469bf2:	adox   %rsi,%r9
  469bf8:	mulx   0x10(%rcx),%r13,%rsi
  469bfe:	adcx   %r13,%r9
  469c04:	adox   %rsi,%r10
  469c0a:	mulx   0x18(%rcx),%r13,%rsi
  469c10:	adcx   %r13,%r10
  469c16:	adox   %rsi,%rbp
  469c1c:	adcx   %rdi,%rbp
  469c22:	mov    0x18(%r8),%rdx
  469c26:	xor    %edi,%edi
  469c28:	mulx   (%rax),%r13,%rsi
  469c2d:	adcx   %r13,%r14
  469c33:	adox   %rsi,%r9
  469c39:	mulx   0x8(%rax),%r13,%rsi
  469c3f:	adcx   %r13,%r9
  469c45:	adox   %rsi,%r10
  469c4b:	mulx   0x10(%rax),%r13,%rsi
  469c51:	adcx   %r13,%r10
  469c57:	adox   %rsi,%rbp
  469c5d:	mulx   0x18(%rax),%r13,%rsi
  469c63:	adcx   %r13,%rbp
  469c69:	adox   %rsi,%r15
  469c6f:	adcx   %rdi,%r15
  469c75:	mov    %r14,%rdx
  469c78:	imul   %r11,%rdx
  469c7c:	xor    %edi,%edi
  469c7e:	mulx   (%rcx),%r13,%rsi
  469c83:	adcx   %r13,%r14
  469c89:	adox   %rsi,%r9
  469c8f:	mulx   0x8(%rcx),%r13,%rsi
  469c95:	adcx   %r13,%r9
  469c9b:	adox   %rsi,%r10
  469ca1:	mulx   0x10(%rcx),%r13,%rsi
  469ca7:	adcx   %r13,%r10
  469cad:	adox   %rsi,%rbp
  469cb3:	mulx   0x18(%rcx),%r13,%rsi
  469cb9:	adcx   %r13,%rbp
  469cbf:	adox   %rsi,%r15
  469cc5:	adcx   %rdi,%r15
  469ccb:	movabs $0x687e956e978e3573,%rdx
  469cd5:	movabs $0x3c208c16d87cfd47,%rsi
  469cdf:	movabs $0x97816a916871ca8d,%rax
  469ce9:	mov    %r15,0x48(%rsp)
  469cee:	mov    %r10,0x38(%rsp)
  469cf3:	lea    (%r10,%rdx,1),%r8
  469cf7:	cmp    %rsi,%r9
  469cfa:	mov    0x48(%rsp),%r11
  469cff:	mov    0x30(%rsp),%rdx
  469d04:	mov    0x20(%rsp),%rcx
  469d09:	mov    0x28(%rsp),%r15
  469d0e:	sbb    $0x0,%r8
  469d12:	setb   %sil
  469d16:	cmp    %rax,%r10
  469d19:	movabs $0xb85045b68181585d,%rax
  469d23:	mov    0x40(%rsp),%r10
  469d28:	setb   %dil
  469d2c:	or     %sil,%dil
  469d2f:	movabs $0x47afba497e7ea7a3,%rsi
  469d39:	lea    0x0(%rbp,%rsi,1),%r13
  469d3e:	movzbl %dil,%esi
  469d42:	sub    %rsi,%r13
  469d45:	setb   %sil
  469d49:	cmp    %rax,%rbp
  469d4c:	movabs $0xcf9bb18d1ece5fd7,%rax
  469d56:	setb   %dil
  469d5a:	lea    (%r11,%rax,1),%r14
  469d5e:	movabs $0x30644e72e131a029,%rax
  469d68:	or     %sil,%dil
  469d6b:	movzbl %dil,%esi
  469d6f:	sub    %rsi,%r14
  469d72:	setb   %sil
  469d76:	cmp    %rax,%r11
  469d79:	setb   %dil
  469d7d:	or     %sil,%dil
  469d80:	cmp    %rax,%rdx
  469d83:	movabs $0xc3df73e9278302b9,%rax
  469d8d:	setb   %sil
  469d91:	or     0x18(%rsp),%sil
  469d96:	lea    (%rcx,%rax,1),%rsi
  469d9a:	cmovne %rcx,%rsi
  469d9e:	cmovne 0x8(%rsp),%r15
  469da4:	cmovne 0x10(%rsp),%r10
  469daa:	cmovne %rdx,%r12
  469dae:	test   %dil,%dil
  469db1:	lea    (%r9,%rax,1),%rdi
  469db5:	cmovne 0x38(%rsp),%r8
  469dbb:	cmovne %r9,%rdi
  469dbf:	cmovne %rbp,%r13
  469dc3:	cmovne %r11,%r14
  469dc7:	mov    %rsi,(%rbx)
  469dca:	mov    %r15,0x8(%rbx)
  469dce:	mov    %r10,0x10(%rbx)
  469dd2:	mov    %r12,0x18(%rbx)
  469dd6:	mov    %rdi,0x20(%rbx)
  469dda:	mov    %r8,0x28(%rbx)
  469dde:	mov    %r13,0x30(%rbx)
  469de2:	mov    %r14,0x38(%rbx)
  469de6:	movabs $0xcf9bb18d1ece5fd7,%r14
  469df0:	add    $0xc0,%rbx
  469df7:	cmp    0x50(%rsp),%rbx
  469dfc:	jne    469650 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x38b0>
  469e02:	sub    $0x8,%rsp
  469e06:	mov    0x88(%rsp),%r15
  469e0e:	mov    0xa0(%rsp),%rdx
  469e16:	mov    0x68(%rsp),%rcx
  469e1b:	lea    0x7d8(%rsp),%rdi
  469e23:	xor    %r9d,%r9d
  469e26:	mov    %r15,%rsi
  469e29:	mov    %rdx,%r8
  469e2c:	push   0x60(%rsp)
  469e30:	mov    0x88(%rsp),%rbx
  469e38:	push   %rbx
  469e39:	push   0x88(%rsp)
  469e40:	vzeroupper
  469e43:	call   46a910 <_RNvMNtNtCsHW5vo2gROj_10zopro_core10msm_affine11g2_borrowedNtB2_6Window4feed>
  469e48:	add    $0x20,%rsp
  469e4c:	lea    0x1c0(%rsp),%rdi
  469e54:	lea    0x7d0(%rsp),%rsi
  469e5c:	mov    $0x5d0,%edx
  469e61:	call   *0x7d651(%rip)        # 4e74b8 <memcpy@GLIBC_2.14>
  469e67:	mov    0x218(%rsp),%rax
  469e6f:	mov    0x1e8(%rsp),%rdx
  469e77:	mov    0x1e0(%rsp),%r13
  469e7f:	mov    0x210(%rsp),%r14
  469e87:	cmp    %rdx,%rax
  469e8a:	mov    %rax,0x28(%rsp)
  469e8f:	cmovb  %rax,%rdx
  469e93:	mov    %rdx,%rax
  469e96:	shl    $0x6,%rax
  469e9a:	lea    (%rax,%rax,2),%r12
  469e9e:	movabs $0xaaaaaaaaaaaaaa,%rax
  469ea8:	cmp    %rax,%rdx
  469eab:	jbe    469ebf <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x411f>
  469ead:	xor    %ebx,%ebx
  469eaf:	mov    %rbx,%rdi
  469eb2:	mov    %r12,%rsi
  469eb5:	call   ed5a1 <_RNvNtCs6i54tJFfzR_5alloc7raw_vec12handle_error>
  469eba:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  469ebf:	mov    %r14,0x8(%rsp)
  469ec4:	mov    %rdx,0x10(%rsp)
  469ec9:	mov    %r12,0x30(%rsp)
  469ece:	mov    %r13,0x20(%rsp)
  469ed3:	test   %rdx,%rdx
  469ed6:	je     469fa5 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4205>
  469edc:	mov    $0x8,%edi
  469ee1:	mov    $0x8,%ebx
  469ee6:	mov    %r12,%rsi
  469ee9:	xor    %edx,%edx
  469eeb:	call   4b8b10 <_RNvMs2_NtNtCsHW5vo2gROj_10zopro_core5alloc5linuxNtB5_9Allocator8allocate>
  469ef0:	test   %rax,%rax
  469ef3:	je     469eaf <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x410f>
  469ef5:	mov    %r13,%r12
  469ef8:	mov    0x1c8(%rsp),%r13
  469f00:	mov    0x1d0(%rsp),%rbx
  469f08:	mov    %rax,%rbp
  469f0b:	xor    %r15d,%r15d
  469f0e:	xor    %r14d,%r14d
  469f11:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  469f20:	mov    0x8(%rsp),%rax
  469f25:	mov    (%rax,%r14,8),%rdx
  469f29:	cmp    %rbx,%rdx
  469f2c:	jae    46a41f <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x467f>
  469f32:	shl    $0x7,%rdx
  469f36:	inc    %r14
  469f39:	lea    (%r12,%r15,1),%rsi
  469f3d:	add    %r13,%rdx
  469f40:	lea    0x100(%rsp),%rdi
  469f48:	vzeroupper
  469f4b:	call   17b0f0 <_RNvMs2_NtCsgHCcee0nhgM_11zopro_bn2545curveINtB5_10ProjectiveNtB5_8G2ConfigE10add_affineB7_>
  469f50:	vmovdqu64 0x100(%rsp),%zmm0
  469f58:	vmovups 0x140(%rsp),%zmm1
  469f60:	vmovups 0x180(%rsp),%zmm2
  469f68:	mov    0x10(%rsp),%rdx
  469f6d:	vmovups %zmm2,0x80(%rbp,%r15,1)
  469f75:	vmovups %zmm1,0x40(%rbp,%r15,1)
  469f7d:	vmovdqu64 %zmm0,0x0(%rbp,%r15,1)
  469f85:	add    $0xc0,%r15
  469f8c:	cmp    %r14,%rdx
  469f8f:	jne    469f20 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4180>
  469f91:	mov    0x78(%rsp),%rbx
  469f96:	mov    0x80(%rsp),%r13
  469f9e:	mov    0x8(%rsp),%r14
  469fa3:	jmp    469fad <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x420d>
  469fa5:	mov    %r15,%r13
  469fa8:	mov    $0x8,%ebp
  469fad:	lea    0x100(%rsp),%rdi
  469fb5:	mov    %rbp,%rsi
  469fb8:	vzeroupper
  469fbb:	call   284ff0 <_RNvMs2_NtCsgHCcee0nhgM_11zopro_bn2545curveINtB5_10ProjectiveNtB5_8G2ConfigE15batch_to_affineCsHW5vo2gROj_10zopro_core>
  469fc0:	mov    0x100(%rsp),%rax
  469fc8:	mov    %r13,%r12
  469fcb:	mov    0x108(%rsp),%r15
  469fd3:	mov    0x20(%rsp),%r13
  469fd8:	mov    %rax,0x18(%rsp)
  469fdd:	mov    0x110(%rsp),%rax
  469fe5:	test   %rax,%rax
  469fe8:	je     46a055 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x42b5>
  469fea:	mov    0x1c8(%rsp),%rcx
  469ff2:	mov    0x1d0(%rsp),%rsi
  469ffa:	shlq   $0x3,0x28(%rsp)
  46a000:	shl    $0x7,%rax
  46a004:	xor    %edx,%edx
  46a006:	xor    %r8d,%r8d
  46a009:	nopl   0x0(%rax)
  46a010:	cmp    %r8,0x28(%rsp)
  46a015:	je     46a055 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x42b5>
  46a017:	mov    (%r14,%r8,1),%rdi
  46a01b:	cmp    %rsi,%rdi
  46a01e:	jae    46a46f <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x46cf>
  46a024:	lea    (%r15,%rdx,1),%r9
  46a028:	shl    $0x7,%rdi
  46a02c:	add    $0x8,%r8
  46a030:	sub    $0xffffffffffffff80,%rdx
  46a034:	vmovdqu64 (%r9),%zmm0
  46a03a:	vmovups 0x40(%r9),%zmm1
  46a041:	vmovups %zmm1,0x40(%rcx,%rdi,1)
  46a049:	vmovdqu64 %zmm0,(%rcx,%rdi,1)
  46a050:	cmp    %rdx,%rax
  46a053:	jne    46a010 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4270>
  46a055:	mov    0x18(%rsp),%rcx
  46a05a:	test   %rcx,%rcx
  46a05d:	je     46a0d4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4334>
  46a05f:	cmp    $0x3ff,%rcx
  46a066:	jbe    46a09b <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x42fb>
  46a068:	shl    $0x7,%rcx
  46a06c:	movabs $0x7ffffffffffffff9,%rax
  46a076:	mov    %rcx,%rsi
  46a079:	or     $0x1,%rsi
  46a07d:	cmp    %rax,%rsi
  46a080:	jae    46a4c8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4728>
  46a086:	cmpb   $0x1,(%r15,%rcx,1)
  46a08b:	jne    46a0c8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4328>
  46a08d:	mov    %r15,%rdi
  46a090:	vzeroupper
  46a093:	call   *0x7d437(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a099:	jmp    46a0d4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4334>
  46a09b:	jmp    46a0c8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4328>
  46a09d:	cmpb   $0x1,(%r15,%r14,1)
  46a0a2:	jne    46a383 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x45e3>
  46a0a8:	inc    %r14
  46a0ab:	mov    %r15,%rdi
  46a0ae:	mov    %r14,%rsi
  46a0b1:	call   *0x7d419(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a0b7:	test   %bpl,%bpl
  46a0ba:	mov    %rbx,%rbp
  46a0bd:	je     466a0a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc6a>
  46a0c3:	jmp    46894b <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x2bab>
  46a0c8:	mov    %r15,%rdi
  46a0cb:	vzeroupper
  46a0ce:	call   *0x7d404(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a0d4:	vmovdqu 0x1c8(%rsp),%xmm0
  46a0dd:	mov    0x1c0(%rsp),%r15
  46a0e5:	mov    0x10(%rsp),%rax
  46a0ea:	vmovdqa %xmm0,0xd0(%rsp)
  46a0f3:	test   %rax,%rax
  46a0f6:	je     46a12c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x438c>
  46a0f8:	cmp    $0x2aa,%rax
  46a0fe:	jbe    46a11e <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x437e>
  46a100:	mov    0x30(%rsp),%rsi
  46a105:	cmpb   $0x1,0x0(%rbp,%rsi,1)
  46a10a:	jne    46a120 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4380>
  46a10c:	or     $0x1,%rsi
  46a110:	mov    %rbp,%rdi
  46a113:	vzeroupper
  46a116:	call   *0x7d3b4(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a11c:	jmp    46a12c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x438c>
  46a11e:	jmp    46a120 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4380>
  46a120:	mov    %rbp,%rdi
  46a123:	vzeroupper
  46a126:	call   *0x7d3ac(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a12c:	mov    %r13,%rdi
  46a12f:	mov    0x1d8(%rsp),%rax
  46a137:	mov    0x68(%rsp),%r14
  46a13c:	mov    0x88(%rsp),%r13
  46a144:	mov    %rbx,%rbp
  46a147:	test   %rax,%rax
  46a14a:	je     46a192 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x43f2>
  46a14c:	cmp    $0x2aa,%rax
  46a152:	jbe    46a187 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x43e7>
  46a154:	shl    $0x6,%rax
  46a158:	movabs $0x7ffffffffffffff9,%rcx
  46a162:	lea    (%rax,%rax,2),%rax
  46a166:	mov    %rax,%rsi
  46a169:	or     $0x1,%rsi
  46a16d:	cmp    %rcx,%rsi
  46a170:	jae    46a4d0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4730>
  46a176:	cmpb   $0x1,(%rdi,%rax,1)
  46a17a:	jne    46a189 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x43e9>
  46a17c:	vzeroupper
  46a17f:	call   *0x7d34b(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a185:	jmp    46a192 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x43f2>
  46a187:	jmp    46a189 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x43e9>
  46a189:	vzeroupper
  46a18c:	call   *0x7d346(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a192:	mov    0x1f0(%rsp),%rax
  46a19a:	test   %rax,%rax
  46a19d:	je     46a1ed <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x444d>
  46a19f:	mov    0x1f8(%rsp),%rdi
  46a1a7:	cmp    $0x7fff,%rax
  46a1ad:	jbe    46a1e2 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4442>
  46a1af:	movabs $0x7ffffffffffffff9,%rcx
  46a1b9:	shl    $0x2,%rax
  46a1bd:	mov    %rax,%rsi
  46a1c0:	or     $0x1,%rsi
  46a1c4:	add    $0x4,%rcx
  46a1c8:	cmp    %rcx,%rsi
  46a1cb:	jae    46a4d8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4738>
  46a1d1:	cmpb   $0x1,(%rdi,%rax,1)
  46a1d5:	jne    46a1e4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4444>
  46a1d7:	vzeroupper
  46a1da:	call   *0x7d2f0(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a1e0:	jmp    46a1ed <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x444d>
  46a1e2:	jmp    46a1e4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4444>
  46a1e4:	vzeroupper
  46a1e7:	call   *0x7d2eb(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a1ed:	mov    0x208(%rsp),%rax
  46a1f5:	test   %rax,%rax
  46a1f8:	je     46a246 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x44a6>
  46a1fa:	cmp    $0x3fff,%rax
  46a200:	jbe    46a236 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4496>
  46a202:	shl    $0x3,%rax
  46a206:	movabs $0x7ffffffffffffff9,%rcx
  46a210:	mov    %rax,%rsi
  46a213:	or     $0x1,%rsi
  46a217:	cmp    %rcx,%rsi
  46a21a:	jae    46a4e0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4740>
  46a220:	mov    0x8(%rsp),%rdi
  46a225:	cmpb   $0x1,(%rdi,%rax,1)
  46a229:	jne    46a23d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x449d>
  46a22b:	vzeroupper
  46a22e:	call   *0x7d29c(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a234:	jmp    46a246 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x44a6>
  46a236:	mov    0x8(%rsp),%rdi
  46a23b:	jmp    46a23d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x449d>
  46a23d:	vzeroupper
  46a240:	call   *0x7d292(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a246:	mov    0x220(%rsp),%rax
  46a24e:	test   %rax,%rax
  46a251:	je     46a29d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x44fd>
  46a253:	mov    0x228(%rsp),%rdi
  46a25b:	cmp    $0x3fff,%rax
  46a261:	jbe    46a292 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x44f2>
  46a263:	shl    $0x3,%rax
  46a267:	movabs $0x7ffffffffffffff9,%rcx
  46a271:	mov    %rax,%rsi
  46a274:	or     $0x1,%rsi
  46a278:	cmp    %rcx,%rsi
  46a27b:	jae    46a4e8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4748>
  46a281:	cmpb   $0x1,(%rdi,%rax,1)
  46a285:	jne    46a294 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x44f4>
  46a287:	vzeroupper
  46a28a:	call   *0x7d240(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a290:	jmp    46a29d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x44fd>
  46a292:	jmp    46a294 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x44f4>
  46a294:	vzeroupper
  46a297:	call   *0x7d23b(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a29d:	mov    0x238(%rsp),%rax
  46a2a5:	test   %rax,%rax
  46a2a8:	je     46a2f4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4554>
  46a2aa:	mov    0x240(%rsp),%rdi
  46a2b2:	cmp    $0x1fff,%rax
  46a2b8:	jbe    46a2e9 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4549>
  46a2ba:	shl    $0x4,%rax
  46a2be:	movabs $0x7ffffffffffffff9,%rcx
  46a2c8:	mov    %rax,%rsi
  46a2cb:	or     $0x1,%rsi
  46a2cf:	cmp    %rcx,%rsi
  46a2d2:	jae    46a4f0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4750>
  46a2d8:	cmpb   $0x1,(%rdi,%rax,1)
  46a2dc:	jne    46a2eb <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x454b>
  46a2de:	vzeroupper
  46a2e1:	call   *0x7d1e9(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a2e7:	jmp    46a2f4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4554>
  46a2e9:	jmp    46a2eb <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x454b>
  46a2eb:	vzeroupper
  46a2ee:	call   *0x7d1e4(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a2f4:	mov    0x250(%rsp),%rax
  46a2fc:	test   %rax,%rax
  46a2ff:	je     46a34b <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x45ab>
  46a301:	mov    0x258(%rsp),%rdi
  46a309:	cmp    $0x1fff,%rax
  46a30f:	jbe    46a340 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x45a0>
  46a311:	shl    $0x4,%rax
  46a315:	movabs $0x7ffffffffffffff9,%rcx
  46a31f:	mov    %rax,%rsi
  46a322:	or     $0x1,%rsi
  46a326:	cmp    %rcx,%rsi
  46a329:	jae    46a4f8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4758>
  46a32f:	cmpb   $0x1,(%rdi,%rax,1)
  46a333:	jne    46a342 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x45a2>
  46a335:	vzeroupper
  46a338:	call   *0x7d192(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a33e:	jmp    46a34b <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x45ab>
  46a340:	jmp    46a342 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x45a2>
  46a342:	vzeroupper
  46a345:	call   *0x7d18d(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a34b:	lea    0x268(%rsp),%rdi
  46a353:	vzeroupper
  46a356:	call   250740 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCsHW5vo2gROj_10zopro_core3msm8BatchAddNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEEBG_>
  46a35b:	cmp    $0xffffffffffffffff,%r15
  46a35f:	je     466a29 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc89>
  46a365:	vmovdqa 0xd0(%rsp),%xmm0
  46a36e:	mov    0xa8(%rsp),%rax
  46a376:	mov    %r15,(%rax)
  46a379:	vmovdqu %xmm0,0x8(%rax)
  46a37e:	jmp    467ba0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x1e00>
  46a383:	jmp    4669f5 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0xc55>
  46a388:	lea    0x797b1(%rip),%rcx        # 4e3b40 <__frame_dummy_init_array_entry+0x5350>
  46a38f:	mov    %r9,%rdi
  46a392:	mov    %r10,%rsi
  46a395:	call   f0380 <_RNvNtNtCsgxBkk5gSRhY_4core5slice5index16slice_index_fail>
  46a39a:	lea    0x79787(%rip),%rcx        # 4e3b28 <__frame_dummy_init_array_entry+0x5338>
  46a3a1:	mov    %r9,%rdi
  46a3a4:	mov    %r10,%rsi
  46a3a7:	call   f0380 <_RNvNtNtCsgxBkk5gSRhY_4core5slice5index16slice_index_fail>
  46a3ac:	lea    -0x449d15(%rip),%rdi        # 2069e <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x629e>
  46a3b3:	lea    0x7b846(%rip),%rdx        # 4e5c00 <__frame_dummy_init_array_entry+0x7410>
  46a3ba:	mov    $0x42,%esi
  46a3bf:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  46a3c4:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  46a3c9:	lea    0x76e58(%rip),%r8        # 4e1228 <__frame_dummy_init_array_entry+0x2a38>
  46a3d0:	lea    0xde0(%rsp),%rdi
  46a3d8:	lea    0x1c0(%rsp),%rsi
  46a3e0:	xor    %edx,%edx
  46a3e2:	call   ef7d2 <_RINvNtCsgxBkk5gSRhY_4core9panicking13assert_failedjjEB4_>
  46a3e7:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  46a3ec:	lea    0x79cdd(%rip),%rcx        # 4e40d0 <__frame_dummy_init_array_entry+0x58e0>
  46a3f3:	mov    %rdx,%rax
  46a3f6:	jmp    46a443 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x46a3>
  46a3f8:	lea    0x79cd1(%rip),%rax        # 4e40d0 <__frame_dummy_init_array_entry+0x58e0>
  46a3ff:	mov    %r14,%rdi
  46a402:	mov    %rax,0x88(%rsp)
  46a40a:	mov    0x88(%rsp),%rdx
  46a412:	vzeroupper
  46a415:	call   ef42b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
  46a41a:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  46a41f:	lea    0x7983a(%rip),%rax        # 4e3c60 <__frame_dummy_init_array_entry+0x5470>
  46a426:	mov    %rdx,%rdi
  46a429:	mov    %rbx,%rsi
  46a42c:	mov    %rax,%rdx
  46a42f:	vzeroupper
  46a432:	call   ef42b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
  46a437:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  46a43c:	lea    0x79ca5(%rip),%rcx        # 4e40e8 <__frame_dummy_init_array_entry+0x58f8>
  46a443:	mov    %rcx,0xa0(%rsp)
  46a44b:	mov    0xa0(%rsp),%rdx
  46a453:	mov    %rsi,%rdi
  46a456:	mov    %rax,%rsi
  46a459:	vzeroupper
  46a45c:	call   ef42b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
  46a461:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  46a463:	lea    0x7b77e(%rip),%rdi        # 4e5be8 <__frame_dummy_init_array_entry+0x73f8>
  46a46a:	call   f1b20 <_RNvNtNtCsgxBkk5gSRhY_4core9panicking11panic_const23panic_const_div_by_zero>
  46a46f:	lea    0x79fd2(%rip),%rdx        # 4e4448 <__frame_dummy_init_array_entry+0x5c58>
  46a476:	vzeroupper
  46a479:	call   ef42b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
  46a47e:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  46a480:	mov    $0x4,%edi
  46a485:	mov    %r15,%rsi
  46a488:	call   ed5a1 <_RNvNtCs6i54tJFfzR_5alloc7raw_vec12handle_error>
  46a48d:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  46a48f:	mov    0x20(%rsp),%rsi
  46a494:	mov    $0x8,%edi
  46a499:	call   ed5a1 <_RNvNtCs6i54tJFfzR_5alloc7raw_vec12handle_error>
  46a49e:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  46a4a0:	mov    $0x8,%edi
  46a4a5:	mov    %r14,%rsi
  46a4a8:	call   ed5a1 <_RNvNtCs6i54tJFfzR_5alloc7raw_vec12handle_error>
  46a4ad:	jmp    46a4bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x471c>
  46a4af:	mov    $0x8,%edi
  46a4b4:	mov    %r14,%rsi
  46a4b7:	call   ed5a1 <_RNvNtCs6i54tJFfzR_5alloc7raw_vec12handle_error>
  46a4bc:	ud2
  46a4be:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a4c3:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a4c8:	vzeroupper
  46a4cb:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a4d0:	vzeroupper
  46a4d3:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a4d8:	vzeroupper
  46a4db:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a4e0:	vzeroupper
  46a4e3:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a4e8:	vzeroupper
  46a4eb:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a4f0:	vzeroupper
  46a4f3:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a4f8:	vzeroupper
  46a4fb:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a500:	mov    0x20(%rsp),%r13
  46a505:	mov    %rax,%r12
  46a508:	jmp    46a5c4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4824>
  46a50d:	mov    %rax,%r15
  46a510:	mov    $0x8,%edx
  46a515:	mov    %rbp,%rdi
  46a518:	mov    %r14,%rsi
  46a51b:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  46a520:	jmp    46a525 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4785>
  46a522:	mov    %rax,%r15
  46a525:	mov    0x10(%rsp),%rdi
  46a52a:	mov    0x20(%rsp),%rsi
  46a52f:	mov    $0x8,%edx
  46a534:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  46a539:	jmp    46a53e <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x479e>
  46a53b:	mov    %rax,%r15
  46a53e:	mov    0x18(%rsp),%rsi
  46a543:	mov    $0x4,%edx
  46a548:	mov    %rbx,%rdi
  46a54b:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  46a550:	jmp    46a555 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x47b5>
  46a552:	mov    %rax,%r15
  46a555:	mov    0x8(%rsp),%rdi
  46a55a:	mov    0x28(%rsp),%rsi
  46a55f:	mov    $0x8,%edx
  46a564:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  46a569:	lea    0xde0(%rsp),%rdi
  46a571:	call   250740 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCsHW5vo2gROj_10zopro_core3msm8BatchAddNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEEBG_>
  46a576:	mov    %r15,%rdi
  46a579:	call   4dd7a0 <_Unwind_Resume@plt>
  46a57e:	mov    %rax,%r15
  46a581:	jmp    46a660 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x48c0>
  46a586:	mov    %rax,%r15
  46a589:	lea    0x7d0(%rsp),%rdi
  46a591:	call   46b700 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueNtNtNtCsHW5vo2gROj_10zopro_core10msm_affine11g2_borrowed6WindowEBH_>
  46a596:	mov    %r15,%rdi
  46a599:	call   4dd7a0 <_Unwind_Resume@plt>
  46a59e:	jmp    46a8f4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b54>
  46a5a3:	cmpq   $0x0,0x18(%rsp)
  46a5a9:	mov    %rax,%r12
  46a5ac:	je     46a5c4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4824>
  46a5ae:	mov    0x18(%rsp),%rsi
  46a5b3:	shl    $0x7,%rsi
  46a5b7:	mov    $0x8,%edx
  46a5bc:	mov    %r15,%rdi
  46a5bf:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  46a5c4:	cmpq   $0x0,0x10(%rsp)
  46a5ca:	je     46a5de <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x483e>
  46a5cc:	mov    0x30(%rsp),%rsi
  46a5d1:	mov    $0x8,%edx
  46a5d6:	mov    %rbp,%rdi
  46a5d9:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  46a5de:	mov    0x8(%rsp),%r14
  46a5e3:	movabs $0x7ffffffffffffff9,%rbx
  46a5ed:	mov    %r12,%r15
  46a5f0:	jmp    46a6b5 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4915>
  46a5f5:	jmp    46a627 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4887>
  46a5f7:	mov    %rax,%r15
  46a5fa:	mov    0x30(%rsp),%rsi
  46a5ff:	mov    $0x8,%edx
  46a604:	mov    %rbp,%rdi
  46a607:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  46a60c:	mov    0x8(%rsp),%r14
  46a611:	movabs $0x7ffffffffffffff9,%rbx
  46a61b:	mov    %r12,%r13
  46a61e:	jmp    46a6b5 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4915>
  46a623:	jmp    46a690 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x48f0>
  46a625:	jmp    46a627 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4887>
  46a627:	mov    %rax,%r15
  46a62a:	lea    0x1c0(%rsp),%rdi
  46a632:	call   247a00 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCsHW5vo2gROj_10zopro_core10msm_affine6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEEBG_>
  46a637:	mov    %r15,%rdi
  46a63a:	call   4dd7a0 <_Unwind_Resume@plt>
  46a63f:	mov    0x7d0(%rsp),%rsi
  46a647:	mov    %rax,%r15
  46a64a:	test   %rsi,%rsi
  46a64d:	je     46a660 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x48c0>
  46a64f:	shl    $0x7,%rsi
  46a653:	mov    $0x8,%edx
  46a658:	mov    %r14,%rdi
  46a65b:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  46a660:	mov    0x100(%rsp),%rsi
  46a668:	test   %rsi,%rsi
  46a66b:	jne    46a675 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x48d5>
  46a66d:	mov    %r15,%rdi
  46a670:	call   4dd7a0 <_Unwind_Resume@plt>
  46a675:	shl    $0x7,%rsi
  46a679:	mov    0x68(%rsp),%rdi
  46a67e:	mov    $0x8,%edx
  46a683:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  46a688:	mov    %r15,%rdi
  46a68b:	call   4dd7a0 <_Unwind_Resume@plt>
  46a690:	mov    %rax,%r15
  46a693:	lea    0x7d0(%rsp),%rdi
  46a69b:	call   247a00 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCsHW5vo2gROj_10zopro_core10msm_affine6WindowNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEEBG_>
  46a6a0:	mov    %r15,%rdi
  46a6a3:	call   4dd7a0 <_Unwind_Resume@plt>
  46a6a8:	movabs $0x7ffffffffffffff9,%rbx
  46a6b2:	mov    %rax,%r15
  46a6b5:	mov    0x1c0(%rsp),%rax
  46a6bd:	test   %rax,%rax
  46a6c0:	je     46a6fc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x495c>
  46a6c2:	mov    0x1c8(%rsp),%rdi
  46a6ca:	cmp    $0x3ff,%rax
  46a6d0:	jbe    46a6f4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4954>
  46a6d2:	shl    $0x7,%rax
  46a6d6:	mov    %rax,%rsi
  46a6d9:	or     $0x1,%rsi
  46a6dd:	cmp    %rbx,%rsi
  46a6e0:	jae    46a8d1 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b31>
  46a6e6:	cmpb   $0x1,(%rdi,%rax,1)
  46a6ea:	jne    46a6f6 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4956>
  46a6ec:	call   *0x7cdde(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a6f2:	jmp    46a6fc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x495c>
  46a6f4:	jmp    46a6f6 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4956>
  46a6f6:	call   *0x7cddc(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a6fc:	mov    0x1d8(%rsp),%rax
  46a704:	test   %rax,%rax
  46a707:	je     46a747 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x49a7>
  46a709:	cmp    $0x2aa,%rax
  46a70f:	jbe    46a73c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x499c>
  46a711:	shl    $0x6,%rax
  46a715:	lea    (%rax,%rax,2),%rax
  46a719:	mov    %rax,%rsi
  46a71c:	or     $0x1,%rsi
  46a720:	cmp    %rbx,%rsi
  46a723:	jae    46a8d6 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b36>
  46a729:	cmpb   $0x1,0x0(%r13,%rax,1)
  46a72f:	jne    46a73e <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x499e>
  46a731:	mov    %r13,%rdi
  46a734:	call   *0x7cd96(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a73a:	jmp    46a747 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x49a7>
  46a73c:	jmp    46a73e <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x499e>
  46a73e:	mov    %r13,%rdi
  46a741:	call   *0x7cd91(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a747:	mov    0x1f0(%rsp),%rax
  46a74f:	test   %rax,%rax
  46a752:	je     46a792 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x49f2>
  46a754:	mov    0x1f8(%rsp),%rdi
  46a75c:	cmp    $0x7fff,%rax
  46a762:	jbe    46a78a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x49ea>
  46a764:	shl    $0x2,%rax
  46a768:	lea    0x4(%rbx),%rcx
  46a76c:	mov    %rax,%rsi
  46a76f:	or     $0x1,%rsi
  46a773:	cmp    %rcx,%rsi
  46a776:	jae    46a8db <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b3b>
  46a77c:	cmpb   $0x1,(%rdi,%rax,1)
  46a780:	jne    46a78c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x49ec>
  46a782:	call   *0x7cd48(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a788:	jmp    46a792 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x49f2>
  46a78a:	jmp    46a78c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x49ec>
  46a78c:	call   *0x7cd46(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a792:	mov    0x208(%rsp),%rax
  46a79a:	test   %rax,%rax
  46a79d:	je     46a7d8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a38>
  46a79f:	cmp    $0x3fff,%rax
  46a7a5:	jbe    46a7cd <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a2d>
  46a7a7:	shl    $0x3,%rax
  46a7ab:	mov    %rax,%rsi
  46a7ae:	or     $0x1,%rsi
  46a7b2:	cmp    %rbx,%rsi
  46a7b5:	jae    46a8e0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b40>
  46a7bb:	cmpb   $0x1,(%r14,%rax,1)
  46a7c0:	jne    46a7cf <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a2f>
  46a7c2:	mov    %r14,%rdi
  46a7c5:	call   *0x7cd05(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a7cb:	jmp    46a7d8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a38>
  46a7cd:	jmp    46a7cf <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a2f>
  46a7cf:	mov    %r14,%rdi
  46a7d2:	call   *0x7cd00(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a7d8:	mov    0x220(%rsp),%rax
  46a7e0:	test   %rax,%rax
  46a7e3:	je     46a81f <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a7f>
  46a7e5:	mov    0x228(%rsp),%rdi
  46a7ed:	cmp    $0x3fff,%rax
  46a7f3:	jbe    46a817 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a77>
  46a7f5:	shl    $0x3,%rax
  46a7f9:	mov    %rax,%rsi
  46a7fc:	or     $0x1,%rsi
  46a800:	cmp    %rbx,%rsi
  46a803:	jae    46a8e5 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b45>
  46a809:	cmpb   $0x1,(%rdi,%rax,1)
  46a80d:	jne    46a819 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a79>
  46a80f:	call   *0x7ccbb(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a815:	jmp    46a81f <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a7f>
  46a817:	jmp    46a819 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4a79>
  46a819:	call   *0x7ccb9(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a81f:	mov    0x238(%rsp),%rax
  46a827:	test   %rax,%rax
  46a82a:	je     46a866 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4ac6>
  46a82c:	mov    0x240(%rsp),%rdi
  46a834:	cmp    $0x1fff,%rax
  46a83a:	jbe    46a85e <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4abe>
  46a83c:	shl    $0x4,%rax
  46a840:	mov    %rax,%rsi
  46a843:	or     $0x1,%rsi
  46a847:	cmp    %rbx,%rsi
  46a84a:	jae    46a8ea <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b4a>
  46a850:	cmpb   $0x1,(%rdi,%rax,1)
  46a854:	jne    46a860 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4ac0>
  46a856:	call   *0x7cc74(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a85c:	jmp    46a866 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4ac6>
  46a85e:	jmp    46a860 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4ac0>
  46a860:	call   *0x7cc72(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a866:	mov    0x250(%rsp),%rax
  46a86e:	test   %rax,%rax
  46a871:	je     46a8bc <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b1c>
  46a873:	mov    0x258(%rsp),%rdi
  46a87b:	cmp    $0x1fff,%rax
  46a881:	jbe    46a8b4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b14>
  46a883:	shl    $0x4,%rax
  46a887:	mov    %rax,%rsi
  46a88a:	or     $0x1,%rsi
  46a88e:	cmp    %rbx,%rsi
  46a891:	jae    46a8ef <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b4f>
  46a893:	cmpb   $0x1,(%rdi,%rax,1)
  46a897:	jne    46a8b6 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b16>
  46a899:	call   *0x7cc31(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46a89f:	lea    0x268(%rsp),%rdi
  46a8a7:	call   250740 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCsHW5vo2gROj_10zopro_core3msm8BatchAddNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEEBG_>
  46a8ac:	mov    %r15,%rdi
  46a8af:	call   4dd7a0 <_Unwind_Resume@plt>
  46a8b4:	jmp    46a8b6 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_+0x4b16>
  46a8b6:	call   *0x7cc1c(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46a8bc:	lea    0x268(%rsp),%rdi
  46a8c4:	call   250740 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCsHW5vo2gROj_10zopro_core3msm8BatchAddNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEEBG_>
  46a8c9:	mov    %r15,%rdi
  46a8cc:	call   4dd7a0 <_Unwind_Resume@plt>
  46a8d1:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a8d6:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a8db:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a8e0:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a8e5:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a8ea:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a8ef:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46a8f4:	mov    %rax,%r15
  46a8f7:	lea    0xde0(%rsp),%rdi
  46a8ff:	call   250740 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCsHW5vo2gROj_10zopro_core3msm8BatchAddNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEEBG_>
  46a904:	mov    %r15,%rdi
  46a907:	call   4dd7a0 <_Unwind_Resume@plt>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
