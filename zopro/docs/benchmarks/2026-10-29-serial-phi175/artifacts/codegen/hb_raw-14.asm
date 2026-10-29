
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047b9a0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_>:
  47b9a0:	push   %rbp
  47b9a1:	push   %r15
  47b9a3:	push   %r14
  47b9a5:	push   %r13
  47b9a7:	push   %r12
  47b9a9:	push   %rbx
  47b9aa:	sub    $0x58,%rsp
  47b9ae:	vmovups (%rdi),%xmm0
  47b9b2:	cmpq   $0x0,(%rdi)
  47b9b6:	movq   $0x0,(%rdi)
  47b9bd:	je     47bb20 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x180>
  47b9c3:	mov    %fs:0xffffffffffffffc0,%rsi
  47b9cc:	mov    %rdi,%rbx
  47b9cf:	test   %rsi,%rsi
  47b9d2:	je     47bb06 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x166>
  47b9d8:	lea    0x10(%rbx),%rax
  47b9dc:	vmovaps %xmm0,(%rsp)
  47b9e1:	vmovups (%rax),%zmm0
  47b9e7:	vmovups %zmm0,0x10(%rsp)
  47b9f2:	mov    0x40(%rax),%rax
  47b9f6:	mov    %rax,0x50(%rsp)
  47b9fb:	mov    $0x1,%edx
  47ba00:	mov    %rsp,%rdi
  47ba03:	vzeroupper
  47ba06:	call   47b4c0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_>
  47ba0b:	mov    $0x1,%r13d
  47ba11:	cmpl   $0x2,0x58(%rbx)
  47ba15:	jb     47ba90 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0xf0>
  47ba17:	mov    0x68(%rbx),%rbp
  47ba1b:	mov    0x60(%rbx),%r12
  47ba1f:	mov    0x0(%rbp),%rax
  47ba23:	test   %rax,%rax
  47ba26:	je     47ba2d <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x8d>
  47ba28:	mov    %r12,%rdi
  47ba2b:	call   *%rax
  47ba2d:	mov    0x8(%rbp),%rsi
  47ba31:	test   %rsi,%rsi
  47ba34:	je     47ba90 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0xf0>
  47ba36:	mov    0x10(%rbp),%rax
  47ba3a:	cmp    $0x20000,%rsi
  47ba41:	setae  %cl
  47ba44:	cmp    $0x1001,%rax
  47ba4a:	setb   %dl
  47ba4d:	test   %cl,%dl
  47ba4f:	je     47ba87 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0xe7>
  47ba51:	blsr   %eax,%ecx
  47ba56:	jne    47bb2e <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x18e>
  47ba5c:	movabs $0x8000000000000000,%rcx
  47ba66:	sub    %rax,%rcx
  47ba69:	cmp    %rcx,%rsi
  47ba6c:	jae    47bb2e <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x18e>
  47ba72:	cmpb   $0x1,(%r12,%rsi,1)
  47ba77:	jne    47ba87 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0xe7>
  47ba79:	inc    %rsi
  47ba7c:	mov    %r12,%rdi
  47ba7f:	call   *0x6ba4b(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  47ba85:	jmp    47ba90 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0xf0>
  47ba87:	mov    %r12,%rdi
  47ba8a:	call   *0x6ba48(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  47ba90:	mov    %r13,0x58(%rbx)
  47ba94:	mov    %r15,0x60(%rbx)
  47ba98:	mov    %r14,0x68(%rbx)
  47ba9c:	mov    0x70(%rbx),%rax
  47baa0:	movzbl 0x88(%rbx),%ebp
  47baa7:	mov    (%rax),%rdi
  47baaa:	cmp    $0x1,%bpl
  47baae:	jne    47bac2 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x122>
  47bab0:	lock incq (%rdi)
  47bab4:	jle    47bb75 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x1d5>
  47baba:	mov    (%rax),%r14
  47babd:	mov    %r14,%rdi
  47bac0:	jmp    47bac2 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x122>
  47bac2:	mov    $0x3,%eax
  47bac7:	mov    0x80(%rbx),%rsi
  47bace:	xchg   %rax,0x78(%rbx)
  47bad2:	cmp    $0x2,%rax
  47bad6:	jne    47bae4 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x144>
  47bad8:	add    $0x1d8,%rdi
  47badf:	call   fef40 <_RNvMNtCs8zaw6vLE3tn_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread>
  47bae4:	test   %bpl,%bpl
  47bae7:	je     47baf7 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x157>
  47bae9:	lock decq (%r14)
  47baed:	jne    47baf7 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x157>
  47baef:	mov    %r14,%rdi
  47baf2:	call   ffd00 <_RNvMsn_NtCs6i54tJFfzR_5alloc4syncINtB5_3ArcNtNtCs8zaw6vLE3tn_10rayon_core8registry8RegistryE9drop_slowBJ_>
  47baf7:	add    $0x58,%rsp
  47bafb:	pop    %rbx
  47bafc:	pop    %r12
  47bafe:	pop    %r13
  47bb00:	pop    %r14
  47bb02:	pop    %r15
  47bb04:	pop    %rbp
  47bb05:	ret
  47bb06:	lea    -0x45c3bf(%rip),%rdi        # 1f74e <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x534e>
  47bb0d:	lea    0x66fc4(%rip),%rdx        # 4e2ad8 <__frame_dummy_init_array_entry+0x42e8>
  47bb14:	mov    $0x36,%esi
  47bb19:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47bb1e:	jmp    47bb75 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x1d5>
  47bb20:	lea    0x6a721(%rip),%rdi        # 4e6248 <__frame_dummy_init_array_entry+0x7a58>
  47bb27:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  47bb2c:	jmp    47bb75 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x1d5>
  47bb2e:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  47bb33:	mov    0x8(%rbp),%rsi
  47bb37:	test   %rsi,%rsi
  47bb3a:	je     47bb48 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x1a8>
  47bb3c:	mov    0x10(%rbp),%rdx
  47bb40:	mov    %r12,%rdi
  47bb43:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  47bb48:	mov    %r13,0x58(%rbx)
  47bb4c:	mov    %r15,0x60(%rbx)
  47bb50:	mov    %r14,0x68(%rbx)
  47bb54:	jmp    47bb70 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x1d0>
  47bb56:	test   %bpl,%bpl
  47bb59:	je     47bb70 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x1d0>
  47bb5b:	lock decq (%r14)
  47bb5f:	jne    47bb70 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x1d0>
  47bb61:	mov    %r14,%rdi
  47bb64:	call   ffd00 <_RNvMsn_NtCs6i54tJFfzR_5alloc4syncINtB5_3ArcNtNtCs8zaw6vLE3tn_10rayon_core8registry8RegistryE9drop_slowBJ_>
  47bb69:	jmp    47bb70 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x1d0>
  47bb6b:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47bb70:	call   104a50 <_RNvXNtCs8zaw6vLE3tn_10rayon_core6unwindNtB2_12AbortIfPanicNtNtNtCsgxBkk5gSRhY_4core3ops4drop4Drop4drop>
  47bb75:	ud2
  47bb77:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47bb7c:	mov    %rax,%rdi
  47bb7f:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  47bb84:	mov    $0x2,%r13d
  47bb8a:	mov    %rax,%r15
  47bb8d:	mov    %rdx,%r14
  47bb90:	cmpl   $0x2,0x58(%rbx)
  47bb94:	jae    47ba17 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0x77>
  47bb9a:	jmp    47ba90 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvMs4_NtB7_8registryNtB1m_8Registry15in_worker_crossNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB2K_5slice15IterMutProducerAyj4_EINtNtB2I_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2B_s_0uuE0TuuEE0B6g_ENtB5_3Job7executeB58_+0xf0>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
