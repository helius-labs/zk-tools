
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047a360 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_>:
  47a360:	push   %rbp
  47a361:	push   %r15
  47a363:	push   %r14
  47a365:	push   %r13
  47a367:	push   %r12
  47a369:	push   %rbx
  47a36a:	sub    $0x58,%rsp
  47a36e:	mov    (%rdi),%rdx
  47a371:	mov    0x8(%rdi),%rcx
  47a375:	mov    0x10(%rdi),%rax
  47a379:	movq   $0x0,(%rdi)
  47a380:	test   %rdx,%rdx
  47a383:	je     47a46f <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x10f>
  47a389:	mov    0x38(%rdi),%r11
  47a38d:	mov    %rdi,%rbx
  47a390:	lea    0x30(%rsp),%r10
  47a395:	mov    %r11,0x50(%rsp)
  47a39a:	vmovups 0x18(%rdi),%ymm0
  47a39f:	vmovups %ymm0,0x30(%rsp)
  47a3a5:	mov    %rdx,0x18(%rsp)
  47a3aa:	mov    %rcx,0x20(%rsp)
  47a3af:	mov    %rax,0x28(%rsp)
  47a3b4:	mov    (%rdx),%rsi
  47a3b7:	mov    0x48(%rsp),%r9
  47a3bc:	mov    0x8(%rax),%r8
  47a3c0:	sub    (%rcx),%rsi
  47a3c3:	mov    (%rax),%rcx
  47a3c6:	mov    $0x1,%edx
  47a3cb:	mov    %rsp,%rdi
  47a3ce:	push   %r10
  47a3d0:	push   %r11
  47a3d2:	vzeroupper
  47a3d5:	call   4659e0 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_>
  47a3da:	add    $0x10,%rsp
  47a3de:	mov    (%rsp),%r14
  47a3e2:	mov    0x8(%rsp),%r15
  47a3e7:	mov    0x10(%rsp),%r12
  47a3ec:	mov    $0x1,%r13d
  47a3f2:	lea    0x40(%rbx),%rdi
  47a3f6:	call   295170 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job9JobResultINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2X_8G2ConfigEEEEECsHW5vo2gROj_10zopro_core>
  47a3fb:	mov    %r13,0x40(%rbx)
  47a3ff:	mov    %r14,0x48(%rbx)
  47a403:	mov    %r15,0x50(%rbx)
  47a407:	mov    %r12,0x58(%rbx)
  47a40b:	mov    0x60(%rbx),%rax
  47a40f:	movzbl 0x78(%rbx),%ebp
  47a413:	mov    (%rax),%rdi
  47a416:	cmp    $0x1,%bpl
  47a41a:	jne    47a42e <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0xce>
  47a41c:	lock incq (%rdi)
  47a420:	jle    47a4c7 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x167>
  47a426:	mov    (%rax),%r14
  47a429:	mov    %r14,%rdi
  47a42c:	jmp    47a42e <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0xce>
  47a42e:	mov    $0x3,%eax
  47a433:	mov    0x70(%rbx),%rsi
  47a437:	xchg   %rax,0x68(%rbx)
  47a43b:	cmp    $0x2,%rax
  47a43f:	jne    47a44d <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0xed>
  47a441:	add    $0x1d8,%rdi
  47a448:	call   fef40 <_RNvMNtCs8zaw6vLE3tn_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread>
  47a44d:	test   %bpl,%bpl
  47a450:	je     47a460 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x100>
  47a452:	lock decq (%r14)
  47a456:	jne    47a460 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x100>
  47a458:	mov    %r14,%rdi
  47a45b:	call   ffd00 <_RNvMsn_NtCs6i54tJFfzR_5alloc4syncINtB5_3ArcNtNtCs8zaw6vLE3tn_10rayon_core8registry8RegistryE9drop_slowBJ_>
  47a460:	add    $0x58,%rsp
  47a464:	pop    %rbx
  47a465:	pop    %r12
  47a467:	pop    %r13
  47a469:	pop    %r14
  47a46b:	pop    %r15
  47a46d:	pop    %rbp
  47a46e:	ret
  47a46f:	lea    0x6bdd2(%rip),%rdi        # 4e6248 <__frame_dummy_init_array_entry+0x7a58>
  47a476:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  47a47b:	jmp    47a4c7 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x167>
  47a47d:	test   %bpl,%bpl
  47a480:	je     47a4c2 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x162>
  47a482:	lock decq (%r14)
  47a486:	jne    47a4c2 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x162>
  47a488:	mov    %r14,%rdi
  47a48b:	call   ffd00 <_RNvMsn_NtCs6i54tJFfzR_5alloc4syncINtB5_3ArcNtNtCs8zaw6vLE3tn_10rayon_core8registry8RegistryE9drop_slowBJ_>
  47a490:	jmp    47a4c2 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x162>
  47a492:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47a497:	mov    %r13,0x40(%rbx)
  47a49b:	mov    %r14,0x48(%rbx)
  47a49f:	mov    %r15,0x50(%rbx)
  47a4a3:	mov    %r12,0x58(%rbx)
  47a4a7:	jmp    47a4c2 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x162>
  47a4a9:	mov    %rax,%rdi
  47a4ac:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  47a4b1:	mov    $0x2,%r13d
  47a4b7:	mov    %rax,%r14
  47a4ba:	mov    %rdx,%r15
  47a4bd:	jmp    47a3f2 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_+0x92>
  47a4c2:	call   104a50 <_RNvXNtCs8zaw6vLE3tn_10rayon_core6unwindNtB2_12AbortIfPanicNtNtNtCsgxBkk5gSRhY_4core3ops4drop4Drop4drop>
  47a4c7:	ud2
  47a4c9:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
