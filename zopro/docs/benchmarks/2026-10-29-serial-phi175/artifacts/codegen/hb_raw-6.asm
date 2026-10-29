
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047ac00 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4C_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB6R_E0TB6R_B6R_EEB5M_>:
  47ac00:	push   %r14
  47ac02:	push   %rbx
  47ac03:	sub    $0xd8,%rsp
  47ac0a:	vmovups (%rcx),%zmm0
  47ac10:	vmovups 0x38(%rcx),%zmm1
  47ac1a:	mov    %rdx,%r14
  47ac1d:	mov    %rsi,%rax
  47ac20:	mov    0x100(%r14),%rsi
  47ac27:	add    $0x110,%rdx
  47ac2e:	mov    %rdi,%rbx
  47ac31:	mov    %rdx,0xb0(%rsp)
  47ac39:	movq   $0x0,0xb8(%rsp)
  47ac45:	mov    %rsi,0xc0(%rsp)
  47ac4d:	movb   $0x1,0xc8(%rsp)
  47ac55:	vmovups %zmm0,(%rsp)
  47ac5c:	vmovups %zmm1,0x38(%rsp)
  47ac67:	movq   $0x0,0x78(%rsp)
  47ac70:	lea    0xa9(%rip),%rsi        # 47ad20 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5range12IterProducerjEINtNtB2I_3map11MapConsumerINtNtNtB2I_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5w_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2B_s_0INtB4N_13CollectResultB5t_EB7L_E0TB7L_B7L_EE0B8i_ENtB5_3Job7executeB6G_>
  47ac77:	mov    %rsp,%rdx
  47ac7a:	mov    %rax,%rdi
  47ac7d:	vzeroupper
  47ac80:	call   104760 <_RNvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_8Registry6inject>
  47ac85:	mov    0xb8(%rsp),%rax
  47ac8d:	cmp    $0x3,%rax
  47ac91:	jne    47acdd <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4C_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB6R_E0TB6R_B6R_EEB5M_+0xdd>
  47ac93:	vmovdqa 0x80(%rsp),%xmm0
  47ac9c:	mov    0x78(%rsp),%rax
  47aca1:	cmp    $0x1,%rax
  47aca5:	je     47acbd <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4C_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB6R_E0TB6R_B6R_EEB5M_+0xbd>
  47aca7:	cmp    $0x2,%rax
  47acab:	jne    47acef <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4C_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB6R_E0TB6R_B6R_EEB5M_+0xef>
  47acad:	vmovq  %xmm0,%rdi
  47acb2:	vpextrq $0x1,%xmm0,%rsi
  47acb8:	call   104520 <_RNvNtCs8zaw6vLE3tn_10rayon_core6unwind16resume_unwinding>
  47acbd:	vmovups 0x90(%rsp),%ymm1
  47acc6:	vmovdqu %xmm0,(%rbx)
  47acca:	vmovups %ymm1,0x10(%rbx)
  47accf:	add    $0xd8,%rsp
  47acd6:	pop    %rbx
  47acd7:	pop    %r14
  47acd9:	vzeroupper
  47acdc:	ret
  47acdd:	lea    0xb8(%rsp),%rsi
  47ace5:	mov    %r14,%rdi
  47ace8:	call   fe230 <_RNvMs8_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_12WorkerThread15wait_until_cold>
  47aced:	jmp    47ac93 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB4C_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB6R_E0TB6R_B6R_EEB5M_+0x93>
  47acef:	lea    -0x45bbaa(%rip),%rdi        # 1f14c <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x4d4c>
  47acf6:	lea    0x6a183(%rip),%rdx        # 4e4e80 <__frame_dummy_init_array_entry+0x6690>
  47acfd:	mov    $0x28,%esi
  47ad02:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47ad07:	mov    %rax,%rbx
  47ad0a:	mov    %rsp,%rdi
  47ad0d:	call   1fd6e0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5range12IterProducerjEINtNtB3b_3map11MapConsumerINtNtNtB3b_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5Z_8G1ConfigEENCINvNtCsHW5vo2gROj_10zopro_core10msm_affine16msm_unsigned_rawB6K_Es_0EE0NCB34_s_0INtB5g_13CollectResultB5W_EB8l_E0TB8l_B8l_EE0B8S_EEB78_>
  47ad12:	mov    %rbx,%rdi
  47ad15:	call   4dd7a0 <_Unwind_Resume@plt>
  47ad1a:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
