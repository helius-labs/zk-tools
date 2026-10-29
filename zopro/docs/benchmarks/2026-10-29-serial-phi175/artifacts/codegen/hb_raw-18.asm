
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004650c0 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_>:
  4650c0:	push   %rbp
  4650c1:	push   %r15
  4650c3:	push   %r14
  4650c5:	push   %r13
  4650c7:	push   %r12
  4650c9:	push   %rbx
  4650ca:	sub    $0x178,%rsp
  4650d1:	mov    0x1b0(%rsp),%rax
  4650d9:	mov    0x1b8(%rsp),%r15
  4650e1:	mov    %rsi,0x18(%rsp)
  4650e6:	shr    $1,%rsi
  4650e9:	mov    %r9,%r13
  4650ec:	mov    %rdi,%r12
  4650ef:	mov    %r8,0x30(%rsp)
  4650f4:	mov    %rax,(%rsp)
  4650f8:	cmp    %r8,%rsi
  4650fb:	jae    465189 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0xc9>
  465101:	sub    %r13,(%rsp)
  465105:	mov    0x8(%r15),%r14
  465109:	mov    0x10(%r15),%rbx
  46510d:	mov    %r12,0x10(%rsp)
  465112:	mov    %r14,0x8(%rsp)
  465117:	jbe    4651a3 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0xe3>
  46511d:	mov    (%r15),%r15
  465120:	lea    0xb8(%rsp),%r12
  465128:	xor    %ebp,%ebp
  46512a:	nopw   0x0(%rax,%rax,1)
  465130:	lea    0x0(%r13,%rbp,1),%rdx
  465135:	mov    %r12,%rdi
  465138:	mov    %r15,%rsi
  46513b:	vzeroupper
  46513e:	call   4653e0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_>
  465143:	cmp    %rbp,%rbx
  465146:	je     46535f <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x29f>
  46514c:	vmovups (%r12),%zmm0
  465153:	vmovups 0x40(%r12),%zmm1
  46515b:	vmovups 0x80(%r12),%zmm2
  465163:	inc    %rbp
  465166:	vmovups %zmm2,0x80(%r14)
  46516d:	vmovups %zmm1,0x40(%r14)
  465174:	vmovups %zmm0,(%r14)
  46517a:	add    $0xc0,%r14
  465181:	cmp    %rbp,(%rsp)
  465185:	jne    465130 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x70>
  465187:	jmp    4651a5 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0xe5>
  465189:	test   %dl,%dl
  46518b:	je     4651bf <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0xff>
  46518d:	mov    %fs:0xffffffffffffffc0,%rax
  465196:	test   %rax,%rax
  465199:	je     4651cd <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x10d>
  46519b:	add    $0x110,%rax
  4651a1:	jmp    4651de <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x11e>
  4651a3:	xor    %ebp,%ebp
  4651a5:	mov    0x10(%rsp),%rax
  4651aa:	mov    0x8(%rsp),%rcx
  4651af:	mov    %rcx,(%rax)
  4651b2:	mov    %rbx,0x8(%rax)
  4651b6:	mov    %rbp,0x10(%rax)
  4651ba:	jmp    46534a <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x28a>
  4651bf:	test   %rcx,%rcx
  4651c2:	je     465101 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x41>
  4651c8:	shr    $1,%rcx
  4651cb:	jmp    4651f2 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x132>
  4651cd:	mov    %rsi,%rbx
  4651d0:	mov    %rcx,%r14
  4651d3:	call   1041b0 <_RNvNtCs8zaw6vLE3tn_10rayon_core8registry15global_registry>
  4651d8:	mov    %r14,%rcx
  4651db:	mov    %rbx,%rsi
  4651de:	mov    (%rax),%rax
  4651e1:	shr    $1,%rcx
  4651e4:	mov    0x208(%rax),%rax
  4651eb:	cmp    %rax,%rcx
  4651ee:	cmovbe %rax,%rcx
  4651f2:	mov    0x1b0(%rsp),%rdx
  4651fa:	xor    %eax,%eax
  4651fc:	mov    %rcx,0x28(%rsp)
  465201:	mov    %rsi,0x20(%rsp)
  465206:	sub    %r13,%rdx
  465209:	cmovae %rdx,%rax
  46520d:	cmp    %rax,%rsi
  465210:	ja     465377 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x2b7>
  465216:	mov    0x10(%r15),%rax
  46521a:	sub    %rsi,%rax
  46521d:	jb     46538f <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x2cf>
  465223:	mov    0x8(%r15),%rdi
  465227:	mov    (%r15),%rdx
  46522a:	lea    (%rsi,%rsi,2),%r8
  46522e:	lea    0x18(%rsp),%r9
  465233:	lea    0x28(%rsp),%r10
  465238:	lea    (%rsi,%r13,1),%rcx
  46523c:	mov    %r9,0x38(%rsp)
  465241:	lea    0x20(%rsp),%r9
  465246:	shl    $0x6,%r8
  46524a:	mov    %r9,0x40(%rsp)
  46524f:	mov    %r10,0x48(%rsp)
  465254:	add    %rdi,%r8
  465257:	mov    %rdx,0x50(%rsp)
  46525c:	mov    %r8,0x58(%rsp)
  465261:	mov    0x1b0(%rsp),%r8
  465269:	mov    %rax,0x60(%rsp)
  46526e:	mov    %rcx,0x68(%rsp)
  465273:	mov    %r8,0x70(%rsp)
  465278:	mov    %r9,0x78(%rsp)
  46527d:	mov    %r10,0x80(%rsp)
  465285:	mov    %rdx,0x88(%rsp)
  46528d:	mov    %fs:0xffffffffffffffc0,%rdx
  465296:	mov    %rdi,0x90(%rsp)
  46529e:	mov    %rsi,0x98(%rsp)
  4652a6:	mov    %r13,0xa0(%rsp)
  4652ae:	mov    %rcx,0xa8(%rsp)
  4652b6:	test   %rdx,%rdx
  4652b9:	jne    4652e2 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x222>
  4652bb:	call   1041b0 <_RNvNtCs8zaw6vLE3tn_10rayon_core8registry15global_registry>
  4652c0:	mov    (%rax),%rsi
  4652c3:	mov    %fs:0xffffffffffffffc0,%rdx
  4652cc:	test   %rdx,%rdx
  4652cf:	je     4653a7 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x2e7>
  4652d5:	cmp    %rsi,0x110(%rdx)
  4652dc:	jne    4653c2 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x302>
  4652e2:	lea    0xb0(%rsp),%rdi
  4652ea:	lea    0x38(%rsp),%rsi
  4652ef:	xor    %ecx,%ecx
  4652f1:	call   47a740 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_>
  4652f6:	mov    0xc0(%rsp),%rcx
  4652fe:	mov    0xb0(%rsp),%rax
  465306:	lea    (%rcx,%rcx,2),%rdx
  46530a:	shl    $0x6,%rdx
  46530e:	add    %rax,%rdx
  465311:	cmp    0xc8(%rsp),%rdx
  465319:	jne    46532d <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x26d>
  46531b:	mov    0xd0(%rsp),%rdx
  465323:	mov    0xd8(%rsp),%rsi
  46532b:	jmp    465331 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x271>
  46532d:	xor    %edx,%edx
  46532f:	xor    %esi,%esi
  465331:	add    0xb8(%rsp),%rdx
  465339:	add    %rcx,%rsi
  46533c:	mov    %rax,(%r12)
  465340:	mov    %rdx,0x8(%r12)
  465345:	mov    %rsi,0x10(%r12)
  46534a:	add    $0x178,%rsp
  465351:	pop    %rbx
  465352:	pop    %r12
  465354:	pop    %r13
  465356:	pop    %r14
  465358:	pop    %r15
  46535a:	pop    %rbp
  46535b:	vzeroupper
  46535e:	ret
  46535f:	lea    -0x3daf6c(%rip),%rdi        # 8a3fa <_RNvNtNtNtCsgxBkk5gSRhY_4core7unicode12unicode_data11white_space14WHITESPACE_MAP+0x1f2>
  465366:	lea    0x80f53(%rip),%rdx        # 4e62c0 <__frame_dummy_init_array_entry+0x7ad0>
  46536d:	mov    $0x45,%esi
  465372:	call   ee380 <_RNvNtCsgxBkk5gSRhY_4core9panicking9panic_fmt>
  465377:	lea    -0x44c859(%rip),%rdi        # 18b25 <_RNvNvNtNtCseM6JStPZ3Y4_11miniz_oxide7inflate4core12reverse_bits20REVERSED_BITS_LOOKUP+0x611>
  46537e:	lea    0x7a3a3(%rip),%rdx        # 4df728 <__frame_dummy_init_array_entry+0xf38>
  465385:	mov    $0x2b,%esi
  46538a:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  46538f:	lea    -0x3dafba(%rip),%rdi        # 8a3dc <_RNvNtNtNtCsgxBkk5gSRhY_4core7unicode12unicode_data11white_space14WHITESPACE_MAP+0x1d4>
  465396:	lea    0x80ef3(%rip),%rdx        # 4e6290 <__frame_dummy_init_array_entry+0x7aa0>
  46539d:	mov    $0x1e,%esi
  4653a2:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  4653a7:	sub    $0xffffffffffffff80,%rsi
  4653ab:	lea    0xb0(%rsp),%rdi
  4653b3:	lea    0x38(%rsp),%rdx
  4653b8:	call   47a640 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_3map11MapConsumerINtNtNtB1N_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4B_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1G_s_0INtB3S_13CollectResultB4y_EB6Q_E0TB6Q_B6Q_EEB5L_>
  4653bd:	jmp    4652f6 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x236>
  4653c2:	sub    $0xffffffffffffff80,%rsi
  4653c6:	lea    0xb0(%rsp),%rdi
  4653ce:	lea    0x38(%rsp),%rcx
  4653d3:	call   47ac00 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4C_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB6R_E0TB6R_B6R_EEB5M_>
  4653d8:	jmp    4652f6 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_+0x236>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
