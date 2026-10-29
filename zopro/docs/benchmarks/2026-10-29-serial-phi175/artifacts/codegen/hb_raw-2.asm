
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047b3f0 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5slice15IterMutProducerAyj4_EINtNtB1N_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1G_s_0uuE0TuuEEB4d_>:
  47b3f0:	push   %rbx
  47b3f1:	sub    $0x80,%rsp
  47b3f8:	vmovups (%rsi),%zmm0
  47b3fe:	vmovups 0x18(%rsi),%zmm1
  47b408:	mov    %fs:0x0,%rax
  47b411:	lea    -0x28(%rax),%rax
  47b418:	mov    %rax,0x8(%rsp)
  47b41d:	vmovups %zmm0,0x10(%rsp)
  47b428:	vmovups %zmm1,0x28(%rsp)
  47b433:	movq   $0x0,0x68(%rsp)
  47b43c:	lea    0x9dd(%rip),%rsi        # 47be20 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_>
  47b443:	lea    0x8(%rsp),%rdx
  47b448:	vzeroupper
  47b44b:	call   104760 <_RNvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_8Registry6inject>
  47b450:	mov    0x8(%rsp),%rdi
  47b455:	call   104530 <_RNvMs3_NtCs8zaw6vLE3tn_10rayon_core5latchNtB5_9LockLatch14wait_and_reset>
  47b45a:	mov    0x68(%rsp),%rax
  47b45f:	cmp    $0x1,%rax
  47b463:	je     47b47a <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5slice15IterMutProducerAyj4_EINtNtB1N_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1G_s_0uuE0TuuEEB4d_+0x8a>
  47b465:	cmp    $0x2,%rax
  47b469:	jne    47b483 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5slice15IterMutProducerAyj4_EINtNtB1N_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1G_s_0uuE0TuuEEB4d_+0x93>
  47b46b:	mov    0x70(%rsp),%rdi
  47b470:	mov    0x78(%rsp),%rsi
  47b475:	call   104520 <_RNvNtCs8zaw6vLE3tn_10rayon_core6unwind16resume_unwinding>
  47b47a:	add    $0x80,%rsp
  47b481:	pop    %rbx
  47b482:	ret
  47b483:	lea    -0x45c33e(%rip),%rdi        # 1f14c <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x4d4c>
  47b48a:	lea    0x699ef(%rip),%rdx        # 4e4e80 <__frame_dummy_init_array_entry+0x6690>
  47b491:	mov    $0x28,%esi
  47b496:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47b49b:	mov    %rax,%rbx
  47b49e:	lea    0x8(%rsp),%rdi
  47b4a3:	call   47bfb0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_>
  47b4a8:	mov    %rbx,%rdi
  47b4ab:	call   4dd7a0 <_Unwind_Resume@plt>
  47b4b0:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
