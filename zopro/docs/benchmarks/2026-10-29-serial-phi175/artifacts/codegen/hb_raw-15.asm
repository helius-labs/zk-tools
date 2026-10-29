
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047b1d0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_>:
  47b1d0:	push   %rbp
  47b1d1:	push   %r15
  47b1d3:	push   %r14
  47b1d5:	push   %r13
  47b1d7:	push   %r12
  47b1d9:	push   %rbx
  47b1da:	sub    $0xc8,%rsp
  47b1e1:	vmovups 0x8(%rdi),%xmm0
  47b1e6:	cmpq   $0x0,0x8(%rdi)
  47b1eb:	movq   $0x0,0x8(%rdi)
  47b1f3:	je     47b366 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x196>
  47b1f9:	mov    %fs:0xffffffffffffffc0,%rdx
  47b202:	mov    %rdi,%rbx
  47b205:	test   %rdx,%rdx
  47b208:	je     47b34c <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x17c>
  47b20e:	lea    0x18(%rbx),%rax
  47b212:	vmovaps %xmm0,0x50(%rsp)
  47b218:	lea    0x38(%rbx),%rcx
  47b21c:	vmovups (%rax),%ymm0
  47b220:	vmovups %ymm0,0x60(%rsp)
  47b226:	vmovups (%rcx),%zmm0
  47b22c:	vmovups %zmm0,0x80(%rsp)
  47b234:	mov    0x40(%rcx),%rax
  47b238:	mov    %rax,0xc0(%rsp)
  47b240:	lea    0x20(%rsp),%rdi
  47b245:	lea    0x50(%rsp),%rsi
  47b24a:	mov    $0x1,%ecx
  47b24f:	vzeroupper
  47b252:	call   47a740 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_>
  47b257:	vmovups 0x30(%rsp),%ymm0
  47b25d:	mov    0x20(%rsp),%r14
  47b262:	mov    0x28(%rsp),%r15
  47b267:	mov    $0x1,%r13d
  47b26d:	vmovups %ymm0,0x50(%rsp)
  47b273:	vmovups %ymm0,(%rsp)
  47b278:	cmpl   $0x2,0x80(%rbx)
  47b27f:	jb     47b30d <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x13d>
  47b285:	mov    0x90(%rbx),%rbp
  47b28c:	mov    0x88(%rbx),%r12
  47b293:	mov    0x0(%rbp),%rax
  47b297:	test   %rax,%rax
  47b29a:	je     47b2a4 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0xd4>
  47b29c:	mov    %r12,%rdi
  47b29f:	vzeroupper
  47b2a2:	call   *%rax
  47b2a4:	mov    0x8(%rbp),%rsi
  47b2a8:	test   %rsi,%rsi
  47b2ab:	je     47b30d <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x13d>
  47b2ad:	mov    0x10(%rbp),%rax
  47b2b1:	cmp    $0x20000,%rsi
  47b2b8:	setae  %cl
  47b2bb:	cmp    $0x1001,%rax
  47b2c1:	setb   %dl
  47b2c4:	test   %cl,%dl
  47b2c6:	je     47b301 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x131>
  47b2c8:	blsr   %eax,%ecx
  47b2cd:	jne    47b374 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x1a4>
  47b2d3:	movabs $0x8000000000000000,%rcx
  47b2dd:	sub    %rax,%rcx
  47b2e0:	cmp    %rcx,%rsi
  47b2e3:	jae    47b374 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x1a4>
  47b2e9:	cmpb   $0x1,(%r12,%rsi,1)
  47b2ee:	jne    47b301 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x131>
  47b2f0:	inc    %rsi
  47b2f3:	mov    %r12,%rdi
  47b2f6:	vzeroupper
  47b2f9:	call   *0x6c1d1(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  47b2ff:	jmp    47b30d <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x13d>
  47b301:	mov    %r12,%rdi
  47b304:	vzeroupper
  47b307:	call   *0x6c1cb(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  47b30d:	mov    %r13,0x80(%rbx)
  47b314:	mov    %r14,0x88(%rbx)
  47b31b:	mov    %r15,0x90(%rbx)
  47b322:	vmovups (%rsp),%ymm0
  47b327:	vmovups %ymm0,0x98(%rbx)
  47b32f:	mov    (%rbx),%rdi
  47b332:	vzeroupper
  47b335:	call   1c2e20 <_RNvXsd_NtCs8zaw6vLE3tn_10rayon_core5latchINtB5_8LatchRefNtB5_9LockLatchENtB5_5Latch3setCsHW5vo2gROj_10zopro_core>
  47b33a:	add    $0xc8,%rsp
  47b341:	pop    %rbx
  47b342:	pop    %r12
  47b344:	pop    %r13
  47b346:	pop    %r14
  47b348:	pop    %r15
  47b34a:	pop    %rbp
  47b34b:	ret
  47b34c:	lea    -0x45bc05(%rip),%rdi        # 1f74e <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x534e>
  47b353:	lea    0x685d6(%rip),%rdx        # 4e3930 <__frame_dummy_init_array_entry+0x5140>
  47b35a:	mov    $0x36,%esi
  47b35f:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47b364:	jmp    47b3bd <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x1ed>
  47b366:	lea    0x6aedb(%rip),%rdi        # 4e6248 <__frame_dummy_init_array_entry+0x7a58>
  47b36d:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  47b372:	jmp    47b3bd <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x1ed>
  47b374:	vzeroupper
  47b377:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  47b37c:	mov    0x8(%rbp),%rsi
  47b380:	test   %rsi,%rsi
  47b383:	je     47b391 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x1c1>
  47b385:	mov    0x10(%rbp),%rdx
  47b389:	mov    %r12,%rdi
  47b38c:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  47b391:	mov    %r13,0x80(%rbx)
  47b398:	mov    %r14,0x88(%rbx)
  47b39f:	mov    %r15,0x90(%rbx)
  47b3a6:	vmovups (%rsp),%ymm0
  47b3ab:	vmovups %ymm0,0x98(%rbx)
  47b3b3:	jmp    47b3b5 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x1e5>
  47b3b5:	vzeroupper
  47b3b8:	call   104a50 <_RNvXNtCs8zaw6vLE3tn_10rayon_core6unwindNtB2_12AbortIfPanicNtNtNtCsgxBkk5gSRhY_4core3ops4drop4Drop4drop>
  47b3bd:	ud2
  47b3bf:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47b3c4:	mov    %rax,%rdi
  47b3c7:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  47b3cc:	mov    $0x2,%r13d
  47b3d2:	mov    %rax,%r14
  47b3d5:	mov    %rdx,%r15
  47b3d8:	cmpl   $0x2,0x80(%rbx)
  47b3df:	jae    47b285 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0xb5>
  47b3e5:	jmp    47b30d <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5range12IterProducerjEINtNtB2Z_3map11MapConsumerINtNtNtB2Z_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB5N_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCB2S_s_0INtB54_13CollectResultB5K_EB82_E0TB82_B82_EE00B8z_ENtB5_3Job7executeB6X_+0x13d>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
