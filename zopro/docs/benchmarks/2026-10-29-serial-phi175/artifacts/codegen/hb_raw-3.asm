
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047b8b0 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5slice15IterMutProducerAyj4_EINtNtB1O_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1H_s_0uuE0TuuEEB4e_>:
  47b8b0:	push   %rbx
  47b8b1:	sub    $0x90,%rsp
  47b8b8:	vmovups (%rdx),%zmm0
  47b8be:	vmovups 0x18(%rdx),%zmm1
  47b8c8:	mov    0x100(%rsi),%rcx
  47b8cf:	lea    0x110(%rsi),%rax
  47b8d6:	mov    %rsi,%rbx
  47b8d9:	mov    %rax,0x70(%rsp)
  47b8de:	movq   $0x0,0x78(%rsp)
  47b8e7:	mov    %rcx,0x80(%rsp)
  47b8ef:	movb   $0x1,0x88(%rsp)
  47b8f7:	vmovups %zmm0,(%rsp)
  47b8fe:	vmovups %zmm1,0x18(%rsp)
  47b909:	movq   $0x0,0x58(%rsp)
  47b912:	lea    0x87(%rip),%rsi        # 47b9a0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_>
  47b919:	mov    %rsp,%rdx
  47b91c:	vzeroupper
  47b91f:	call   104760 <_RNvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_8Registry6inject>
  47b924:	mov    0x78(%rsp),%rax
  47b929:	cmp    $0x3,%rax
  47b92d:	jne    47b958 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5slice15IterMutProducerAyj4_EINtNtB1O_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1H_s_0uuE0TuuEEB4e_+0xa8>
  47b92f:	mov    0x58(%rsp),%rax
  47b934:	cmp    $0x1,%rax
  47b938:	je     47b94f <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5slice15IterMutProducerAyj4_EINtNtB1O_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1H_s_0uuE0TuuEEB4e_+0x9f>
  47b93a:	cmp    $0x2,%rax
  47b93e:	jne    47b967 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5slice15IterMutProducerAyj4_EINtNtB1O_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1H_s_0uuE0TuuEEB4e_+0xb7>
  47b940:	mov    0x60(%rsp),%rdi
  47b945:	mov    0x68(%rsp),%rsi
  47b94a:	call   104520 <_RNvNtCs8zaw6vLE3tn_10rayon_core6unwind16resume_unwinding>
  47b94f:	add    $0x90,%rsp
  47b956:	pop    %rbx
  47b957:	ret
  47b958:	lea    0x78(%rsp),%rsi
  47b95d:	mov    %rbx,%rdi
  47b960:	call   fe230 <_RNvMs8_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_12WorkerThread15wait_until_cold>
  47b965:	jmp    47b92f <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5slice15IterMutProducerAyj4_EINtNtB1O_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1H_s_0uuE0TuuEEB4e_+0x7f>
  47b967:	lea    -0x45c822(%rip),%rdi        # 1f14c <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x4d4c>
  47b96e:	lea    0x6950b(%rip),%rdx        # 4e4e80 <__frame_dummy_init_array_entry+0x6690>
  47b975:	mov    $0x28,%esi
  47b97a:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47b97f:	mov    %rax,%rbx
  47b982:	mov    %rsp,%rdi
  47b985:	call   47bba0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_>
  47b98a:	mov    %rbx,%rdi
  47b98d:	call   4dd7a0 <_Unwind_Resume@plt>
  47b992:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
