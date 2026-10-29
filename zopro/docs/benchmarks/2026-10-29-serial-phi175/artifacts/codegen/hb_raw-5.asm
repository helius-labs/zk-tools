
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000479ab0 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_3map11MapConsumerINtNtNtB1N_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB57_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1G_s_0INtB3S_13CollectResultB4y_EB7l_E0TB7l_B7l_EEB6f_>:
  479ab0:	push   %r14
  479ab2:	push   %rbx
  479ab3:	sub    $0xb8,%rsp
  479aba:	vmovups (%rdx),%zmm0
  479ac0:	vmovups 0x38(%rdx),%zmm1
  479aca:	mov    %fs:0x0,%rcx
  479ad3:	lea    0x80(%rsp),%r14
  479adb:	mov    %rsi,%rax
  479ade:	mov    %rdi,%rbx
  479ae1:	lea    -0x28(%rcx),%rcx
  479ae8:	mov    %rcx,(%rsp)
  479aec:	vmovups %zmm0,0x8(%rsp)
  479af7:	vmovups %zmm1,0x40(%rsp)
  479aff:	movq   $0x0,0x80(%rsp)
  479b0b:	lea    0x9be(%rip),%rsi        # 47a4d0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB6j_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB2S_s_0INtB54_13CollectResultB5K_EB8x_E0TB8x_B8x_EE00B94_ENtB5_3Job7executeB7r_>
  479b12:	mov    %rsp,%rdx
  479b15:	mov    %rax,%rdi
  479b18:	vzeroupper
  479b1b:	call   104760 <_RNvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_8Registry6inject>
  479b20:	mov    (%rsp),%rdi
  479b24:	call   104530 <_RNvMs3_NtCs8zaw6vLE3tn_10rayon_core5latchNtB5_9LockLatch14wait_and_reset>
  479b29:	vmovdqu 0x88(%rsp),%xmm0
  479b32:	mov    0x80(%rsp),%rax
  479b3a:	cmp    $0x1,%rax
  479b3e:	je     479b56 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_3map11MapConsumerINtNtNtB1N_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB57_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1G_s_0INtB3S_13CollectResultB4y_EB7l_E0TB7l_B7l_EEB6f_+0xa6>
  479b40:	cmp    $0x2,%rax
  479b44:	jne    479b76 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_3map11MapConsumerINtNtNtB1N_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB57_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1G_s_0INtB3S_13CollectResultB4y_EB7l_E0TB7l_B7l_EEB6f_+0xc6>
  479b46:	vmovq  %xmm0,%rdi
  479b4b:	vpextrq $0x1,%xmm0,%rsi
  479b51:	call   104520 <_RNvNtCs8zaw6vLE3tn_10rayon_core6unwind16resume_unwinding>
  479b56:	vmovups 0x98(%rsp),%ymm1
  479b5f:	vmovups %ymm1,0x10(%rbx)
  479b64:	vmovdqu %xmm0,(%rbx)
  479b68:	add    $0xb8,%rsp
  479b6f:	pop    %rbx
  479b70:	pop    %r14
  479b72:	vzeroupper
  479b75:	ret
  479b76:	lea    -0x45aa31(%rip),%rdi        # 1f14c <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x4d4c>
  479b7d:	lea    0x6b2fc(%rip),%rdx        # 4e4e80 <__frame_dummy_init_array_entry+0x6690>
  479b84:	mov    $0x28,%esi
  479b89:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  479b8e:	mov    %rax,%rbx
  479b91:	mov    %r14,%rdi
  479b94:	call   294e30 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job9JobResultTINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2Y_8G2ConfigEEEB1l_EEECsHW5vo2gROj_10zopro_core>
  479b99:	mov    %rbx,%rdi
  479b9c:	call   4dd7a0 <_Unwind_Resume@plt>
  479ba1:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
