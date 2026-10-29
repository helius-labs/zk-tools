
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047a4d0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB6j_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB2S_s_0INtB54_13CollectResultB5K_EB8x_E0TB8x_B8x_EE00B94_ENtB5_3Job7executeB7r_>:
  47a4d0:	push   %r15
  47a4d2:	push   %r14
  47a4d4:	push   %r12
  47a4d6:	push   %rbx
  47a4d7:	sub    $0xc8,%rsp
  47a4de:	vmovups 0x8(%rdi),%xmm0
  47a4e3:	cmpq   $0x0,0x8(%rdi)
  47a4e8:	movq   $0x0,0x8(%rdi)
  47a4f0:	je     47a5da <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB6j_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB2S_s_0INtB54_13CollectResultB5K_EB8x_E0TB8x_B8x_EE00B94_ENtB5_3Job7executeB7r_+0x10a>
  47a4f6:	mov    %fs:0xffffffffffffffc0,%rdx
  47a4ff:	mov    %rdi,%rbx
  47a502:	test   %rdx,%rdx
  47a505:	je     47a5c0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB6j_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB2S_s_0INtB54_13CollectResultB5K_EB8x_E0TB8x_B8x_EE00B94_ENtB5_3Job7executeB7r_+0xf0>
  47a50b:	lea    0x18(%rbx),%rax
  47a50f:	vmovaps %xmm0,0x50(%rsp)
  47a515:	lea    0x38(%rbx),%rcx
  47a519:	vmovups (%rax),%ymm0
  47a51d:	vmovups %ymm0,0x60(%rsp)
  47a523:	vmovups (%rcx),%zmm0
  47a529:	vmovups %zmm0,0x80(%rsp)
  47a531:	mov    0x40(%rcx),%rax
  47a535:	mov    %rax,0xc0(%rsp)
  47a53d:	lea    0x20(%rsp),%rdi
  47a542:	lea    0x50(%rsp),%rsi
  47a547:	mov    $0x1,%ecx
  47a54c:	vzeroupper
  47a54f:	call   479bb0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_>
  47a554:	vmovups 0x30(%rsp),%ymm0
  47a55a:	mov    0x20(%rsp),%r14
  47a55f:	mov    0x28(%rsp),%r15
  47a564:	mov    $0x1,%r12d
  47a56a:	vmovups %ymm0,0x50(%rsp)
  47a570:	vmovups %ymm0,(%rsp)
  47a575:	lea    0x80(%rbx),%rdi
  47a57c:	vzeroupper
  47a57f:	call   294e30 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job9JobResultTINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2Y_8G2ConfigEEEB1l_EEECsHW5vo2gROj_10zopro_core>
  47a584:	mov    %r12,0x80(%rbx)
  47a58b:	mov    %r14,0x88(%rbx)
  47a592:	mov    %r15,0x90(%rbx)
  47a599:	vmovups (%rsp),%ymm0
  47a59e:	vmovups %ymm0,0x98(%rbx)
  47a5a6:	mov    (%rbx),%rdi
  47a5a9:	vzeroupper
  47a5ac:	call   1c2e20 <_RNvXsd_NtCs8zaw6vLE3tn_10rayon_core5latchINtB5_8LatchRefNtB5_9LockLatchENtB5_5Latch3setCsHW5vo2gROj_10zopro_core>
  47a5b1:	add    $0xc8,%rsp
  47a5b8:	pop    %rbx
  47a5b9:	pop    %r12
  47a5bb:	pop    %r14
  47a5bd:	pop    %r15
  47a5bf:	ret
  47a5c0:	lea    -0x45ae79(%rip),%rdi        # 1f74e <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x534e>
  47a5c7:	lea    0x69362(%rip),%rdx        # 4e3930 <__frame_dummy_init_array_entry+0x5140>
  47a5ce:	mov    $0x36,%esi
  47a5d3:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47a5d8:	jmp    47a614 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB6j_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB2S_s_0INtB54_13CollectResultB5K_EB8x_E0TB8x_B8x_EE00B94_ENtB5_3Job7executeB7r_+0x144>
  47a5da:	lea    0x6bc67(%rip),%rdi        # 4e6248 <__frame_dummy_init_array_entry+0x7a58>
  47a5e1:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  47a5e6:	jmp    47a614 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB6j_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB2S_s_0INtB54_13CollectResultB5K_EB8x_E0TB8x_B8x_EE00B94_ENtB5_3Job7executeB7r_+0x144>
  47a5e8:	mov    %r12,0x80(%rbx)
  47a5ef:	mov    %r14,0x88(%rbx)
  47a5f6:	mov    %r15,0x90(%rbx)
  47a5fd:	vmovups (%rsp),%ymm0
  47a602:	vmovups %ymm0,0x98(%rbx)
  47a60a:	jmp    47a60c <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB6j_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB2S_s_0INtB54_13CollectResultB5K_EB8x_E0TB8x_B8x_EE00B94_ENtB5_3Job7executeB7r_+0x13c>
  47a60c:	vzeroupper
  47a60f:	call   104a50 <_RNvXNtCs8zaw6vLE3tn_10rayon_core6unwindNtB2_12AbortIfPanicNtNtNtCsgxBkk5gSRhY_4core3ops4drop4Drop4drop>
  47a614:	ud2
  47a616:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47a61b:	mov    %rax,%rdi
  47a61e:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  47a623:	mov    $0x2,%r12d
  47a629:	mov    %rax,%r14
  47a62c:	mov    %rdx,%r15
  47a62f:	jmp    47a575 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB6j_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB2S_s_0INtB54_13CollectResultB5K_EB8x_E0TB8x_B8x_EE00B94_ENtB5_3Job7executeB7r_+0xa5>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
