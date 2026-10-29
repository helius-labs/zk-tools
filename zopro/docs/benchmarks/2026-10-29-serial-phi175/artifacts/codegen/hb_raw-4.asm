
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047a640 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_3map11MapConsumerINtNtNtB1N_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4B_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1G_s_0INtB3S_13CollectResultB4y_EB6Q_E0TB6Q_B6Q_EEB5L_>:
  47a640:	push   %rbx
  47a641:	sub    $0xc0,%rsp
  47a648:	vmovups (%rdx),%zmm0
  47a64e:	vmovups 0x38(%rdx),%zmm1
  47a658:	mov    %fs:0x0,%rcx
  47a661:	mov    %rsi,%rax
  47a664:	mov    %rdi,%rbx
  47a667:	lea    -0x28(%rcx),%rcx
  47a66e:	mov    %rcx,0x8(%rsp)
  47a673:	vmovups %zmm0,0x10(%rsp)
  47a67e:	vmovups %zmm1,0x48(%rsp)
  47a689:	movq   $0x0,0x88(%rsp)
  47a695:	lea    0xb34(%rip),%rsi        # 47b1d0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_>
  47a69c:	lea    0x8(%rsp),%rdx
  47a6a1:	mov    %rax,%rdi
  47a6a4:	vzeroupper
  47a6a7:	call   104760 <_RNvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_8Registry6inject>
  47a6ac:	mov    0x8(%rsp),%rdi
  47a6b1:	call   104530 <_RNvMs3_NtCs8zaw6vLE3tn_10rayon_core5latchNtB5_9LockLatch14wait_and_reset>
  47a6b6:	vmovdqu 0x90(%rsp),%xmm0
  47a6bf:	mov    0x88(%rsp),%rax
  47a6c7:	cmp    $0x1,%rax
  47a6cb:	je     47a6e3 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_3map11MapConsumerINtNtNtB1N_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4B_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1G_s_0INtB3S_13CollectResultB4y_EB6Q_E0TB6Q_B6Q_EEB5L_+0xa3>
  47a6cd:	cmp    $0x2,%rax
  47a6d1:	jne    47a701 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_3map11MapConsumerINtNtNtB1N_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4B_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1G_s_0INtB3S_13CollectResultB4y_EB6Q_E0TB6Q_B6Q_EEB5L_+0xc1>
  47a6d3:	vmovq  %xmm0,%rdi
  47a6d8:	vpextrq $0x1,%xmm0,%rsi
  47a6de:	call   104520 <_RNvNtCs8zaw6vLE3tn_10rayon_core6unwind16resume_unwinding>
  47a6e3:	vmovups 0xa0(%rsp),%ymm1
  47a6ec:	vmovups %ymm1,0x10(%rbx)
  47a6f1:	vmovdqu %xmm0,(%rbx)
  47a6f5:	add    $0xc0,%rsp
  47a6fc:	pop    %rbx
  47a6fd:	vzeroupper
  47a700:	ret
  47a701:	lea    -0x45b5bc(%rip),%rdi        # 1f14c <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x4d4c>
  47a708:	lea    0x6a771(%rip),%rdx        # 4e4e80 <__frame_dummy_init_array_entry+0x6690>
  47a70f:	mov    $0x28,%esi
  47a714:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47a719:	mov    %rax,%rbx
  47a71c:	lea    0x8(%rsp),%rdi
  47a721:	call   1fdbe0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5range12IterProducerjEINtNtB3t_3map11MapConsumerINtNtNtB3t_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB6h_8G1ConfigEENCINvNtCsHW5vo2gROj_10zopro_core10msm_affine16msm_unsigned_rawB72_Es_0EE0NCB3m_s_0INtB5y_13CollectResultB6e_EB8D_E0TB8D_B8D_EE00B9a_EEB7q_>
  47a726:	mov    %rbx,%rdi
  47a729:	call   4dd7a0 <_Unwind_Resume@plt>
  47a72e:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
