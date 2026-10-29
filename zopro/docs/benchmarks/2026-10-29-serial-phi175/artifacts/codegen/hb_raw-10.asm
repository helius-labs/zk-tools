
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047be20 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_>:
  47be20:	push   %rbp
  47be21:	push   %r15
  47be23:	push   %r14
  47be25:	push   %r13
  47be27:	push   %r12
  47be29:	push   %rbx
  47be2a:	sub    $0x58,%rsp
  47be2e:	vmovups 0x8(%rdi),%xmm0
  47be33:	cmpq   $0x0,0x8(%rdi)
  47be38:	movq   $0x0,0x8(%rdi)
  47be40:	je     47bf48 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x128>
  47be46:	mov    %fs:0xffffffffffffffc0,%rsi
  47be4f:	mov    %rdi,%rbx
  47be52:	test   %rsi,%rsi
  47be55:	je     47bf2e <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x10e>
  47be5b:	lea    0x18(%rbx),%rax
  47be5f:	vmovaps %xmm0,(%rsp)
  47be64:	vmovups (%rax),%zmm0
  47be6a:	vmovups %zmm0,0x10(%rsp)
  47be75:	mov    0x40(%rax),%rax
  47be79:	mov    %rax,0x50(%rsp)
  47be7e:	mov    $0x1,%edx
  47be83:	mov    %rsp,%rdi
  47be86:	vzeroupper
  47be89:	call   47b4c0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_>
  47be8e:	mov    $0x1,%r13d
  47be94:	cmpl   $0x2,0x60(%rbx)
  47be98:	jb     47bf0b <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0xeb>
  47be9a:	mov    0x70(%rbx),%rbp
  47be9e:	mov    0x68(%rbx),%r12
  47bea2:	mov    0x0(%rbp),%rax
  47bea6:	test   %rax,%rax
  47bea9:	je     47beb0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x90>
  47beab:	mov    %r12,%rdi
  47beae:	call   *%rax
  47beb0:	mov    0x8(%rbp),%rsi
  47beb4:	test   %rsi,%rsi
  47beb7:	je     47bf0b <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0xeb>
  47beb9:	mov    0x10(%rbp),%rax
  47bebd:	cmp    $0x20000,%rsi
  47bec4:	setae  %cl
  47bec7:	cmp    $0x1001,%rax
  47becd:	setb   %dl
  47bed0:	test   %cl,%dl
  47bed2:	je     47bf02 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0xe2>
  47bed4:	blsr   %eax,%ecx
  47bed9:	jne    47bf56 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x136>
  47bedb:	movabs $0x8000000000000000,%rcx
  47bee5:	sub    %rax,%rcx
  47bee8:	cmp    %rcx,%rsi
  47beeb:	jae    47bf56 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x136>
  47beed:	cmpb   $0x1,(%r12,%rsi,1)
  47bef2:	jne    47bf02 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0xe2>
  47bef4:	inc    %rsi
  47bef7:	mov    %r12,%rdi
  47befa:	call   *0x6b5d0(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  47bf00:	jmp    47bf0b <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0xeb>
  47bf02:	mov    %r12,%rdi
  47bf05:	call   *0x6b5cd(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  47bf0b:	mov    %r13,0x60(%rbx)
  47bf0f:	mov    %r15,0x68(%rbx)
  47bf13:	mov    %r14,0x70(%rbx)
  47bf17:	mov    (%rbx),%rdi
  47bf1a:	call   1c2e20 <_RNvXsd_NtCs8zaw6vLE3tn_10rayon_core5latchINtB5_8LatchRefNtB5_9LockLatchENtB5_5Latch3setCsHW5vo2gROj_10zopro_core>
  47bf1f:	add    $0x58,%rsp
  47bf23:	pop    %rbx
  47bf24:	pop    %r12
  47bf26:	pop    %r13
  47bf28:	pop    %r14
  47bf2a:	pop    %r15
  47bf2c:	pop    %rbp
  47bf2d:	ret
  47bf2e:	lea    -0x45c7e7(%rip),%rdi        # 1f74e <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x534e>
  47bf35:	lea    0x679f4(%rip),%rdx        # 4e3930 <__frame_dummy_init_array_entry+0x5140>
  47bf3c:	mov    $0x36,%esi
  47bf41:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47bf46:	jmp    47bf83 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x163>
  47bf48:	lea    0x6a2f9(%rip),%rdi        # 4e6248 <__frame_dummy_init_array_entry+0x7a58>
  47bf4f:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  47bf54:	jmp    47bf83 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x163>
  47bf56:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  47bf5b:	mov    0x8(%rbp),%rsi
  47bf5f:	test   %rsi,%rsi
  47bf62:	je     47bf70 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x150>
  47bf64:	mov    0x10(%rbp),%rdx
  47bf68:	mov    %r12,%rdi
  47bf6b:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  47bf70:	mov    %r13,0x60(%rbx)
  47bf74:	mov    %r15,0x68(%rbx)
  47bf78:	mov    %r14,0x70(%rbx)
  47bf7c:	jmp    47bf7e <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x15e>
  47bf7e:	call   104a50 <_RNvXNtCs8zaw6vLE3tn_10rayon_core6unwindNtB2_12AbortIfPanicNtNtNtCsgxBkk5gSRhY_4core3ops4drop4Drop4drop>
  47bf83:	ud2
  47bf85:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47bf8a:	mov    %rax,%rdi
  47bf8d:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  47bf92:	mov    $0x2,%r13d
  47bf98:	mov    %rax,%r15
  47bf9b:	mov    %rdx,%r14
  47bf9e:	cmpl   $0x2,0x60(%rbx)
  47bfa2:	jae    47be9a <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0x7a>
  47bfa8:	jmp    47bf0b <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobINtNtB7_5latch8LatchRefNtBT_9LockLatchENCNCINvMs4_NtB7_8registryNtB1E_8Registry14in_worker_coldNCINvNtB7_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB31_5slice15IterMutProducerAyj4_EINtNtB2Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB2S_s_0uuE0TuuEE00B6x_ENtB5_3Job7executeB5p_+0xeb>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
