
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047a050 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB58_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB7m_E0TB7m_B7m_EEB6g_>:
  47a050:	push   %r15
  47a052:	push   %r14
  47a054:	push   %rbx
  47a055:	sub    $0xd0,%rsp
  47a05c:	vmovups (%rcx),%zmm0
  47a062:	vmovups 0x38(%rcx),%zmm1
  47a06c:	mov    %rdx,%r14
  47a06f:	mov    %rsi,%rax
  47a072:	mov    0x100(%r14),%rsi
  47a079:	add    $0x110,%rdx
  47a080:	lea    0x78(%rsp),%r15
  47a085:	mov    %rdi,%rbx
  47a088:	mov    %rdx,0xb0(%rsp)
  47a090:	movq   $0x0,0xb8(%rsp)
  47a09c:	mov    %rsi,0xc0(%rsp)
  47a0a4:	movb   $0x1,0xc8(%rsp)
  47a0ac:	vmovups %zmm0,(%rsp)
  47a0b3:	vmovups %zmm1,0x38(%rsp)
  47a0be:	movq   $0x0,0x78(%rsp)
  47a0c7:	lea    0xb2(%rip),%rsi        # 47a180 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5range12IterProducerjEINtNtB2I_3map11MapConsumerINtNtNtB2I_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB62_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB2B_s_0INtB4N_13CollectResultB5t_EB8g_E0TB8g_B8g_EE0B8N_ENtB5_3Job7executeB7a_>
  47a0ce:	mov    %rsp,%rdx
  47a0d1:	mov    %rax,%rdi
  47a0d4:	vzeroupper
  47a0d7:	call   104760 <_RNvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_8Registry6inject>
  47a0dc:	mov    0xb8(%rsp),%rax
  47a0e4:	cmp    $0x3,%rax
  47a0e8:	jne    47a136 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB58_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB7m_E0TB7m_B7m_EEB6g_+0xe6>
  47a0ea:	vmovdqa 0x80(%rsp),%xmm0
  47a0f3:	mov    0x78(%rsp),%rax
  47a0f8:	cmp    $0x1,%rax
  47a0fc:	je     47a114 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB58_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB7m_E0TB7m_B7m_EEB6g_+0xc4>
  47a0fe:	cmp    $0x2,%rax
  47a102:	jne    47a148 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB58_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB7m_E0TB7m_B7m_EEB6g_+0xf8>
  47a104:	vmovq  %xmm0,%rdi
  47a109:	vpextrq $0x1,%xmm0,%rsi
  47a10f:	call   104520 <_RNvNtCs8zaw6vLE3tn_10rayon_core6unwind16resume_unwinding>
  47a114:	vmovups 0x90(%rsp),%ymm1
  47a11d:	vmovdqu %xmm0,(%rbx)
  47a121:	vmovups %ymm1,0x10(%rbx)
  47a126:	add    $0xd0,%rsp
  47a12d:	pop    %rbx
  47a12e:	pop    %r14
  47a130:	pop    %r15
  47a132:	vzeroupper
  47a135:	ret
  47a136:	lea    0xb8(%rsp),%rsi
  47a13e:	mov    %r14,%rdi
  47a141:	call   fe230 <_RNvMs8_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_12WorkerThread15wait_until_cold>
  47a146:	jmp    47a0ea <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB58_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB7m_E0TB7m_B7m_EEB6g_+0x9a>
  47a148:	lea    -0x45b003(%rip),%rdi        # 1f14c <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x4d4c>
  47a14f:	lea    0x6ad2a(%rip),%rdx        # 4e4e80 <__frame_dummy_init_array_entry+0x6690>
  47a156:	mov    $0x28,%esi
  47a15b:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47a160:	mov    %rax,%rbx
  47a163:	mov    %r15,%rdi
  47a166:	call   294e30 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job9JobResultTINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2Y_8G2ConfigEEEB1l_EEECsHW5vo2gROj_10zopro_core>
  47a16b:	mov    %rbx,%rdi
  47a16e:	call   4dd7a0 <_Unwind_Resume@plt>
  47a173:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
