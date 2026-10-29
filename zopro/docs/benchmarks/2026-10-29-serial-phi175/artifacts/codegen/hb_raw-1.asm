
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047bba0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_>:
  47bba0:	push   %r15
  47bba2:	push   %r14
  47bba4:	push   %rbx
  47bba5:	cmpl   $0x2,0x58(%rdi)
  47bba9:	jb     47bc14 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_+0x74>
  47bbab:	mov    0x68(%rdi),%r15
  47bbaf:	mov    0x60(%rdi),%rbx
  47bbb3:	mov    (%r15),%rax
  47bbb6:	test   %rax,%rax
  47bbb9:	je     47bbc0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_+0x20>
  47bbbb:	mov    %rbx,%rdi
  47bbbe:	call   *%rax
  47bbc0:	mov    0x8(%r15),%rsi
  47bbc4:	test   %rsi,%rsi
  47bbc7:	je     47bc14 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_+0x74>
  47bbc9:	mov    0x10(%r15),%rax
  47bbcd:	cmp    $0x20000,%rsi
  47bbd4:	setae  %cl
  47bbd7:	cmp    $0x1001,%rax
  47bbdd:	setb   %dl
  47bbe0:	test   %cl,%dl
  47bbe2:	je     47bc1a <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_+0x7a>
  47bbe4:	blsr   %eax,%ecx
  47bbe9:	jne    47bc28 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_+0x88>
  47bbeb:	movabs $0x8000000000000000,%rcx
  47bbf5:	sub    %rax,%rcx
  47bbf8:	cmp    %rcx,%rsi
  47bbfb:	jae    47bc28 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_+0x88>
  47bbfd:	cmpb   $0x1,(%rbx,%rsi,1)
  47bc01:	jne    47bc1a <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_+0x7a>
  47bc03:	inc    %rsi
  47bc06:	mov    %rbx,%rdi
  47bc09:	pop    %rbx
  47bc0a:	pop    %r14
  47bc0c:	pop    %r15
  47bc0e:	jmp    *0x6b8bc(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  47bc14:	pop    %rbx
  47bc15:	pop    %r14
  47bc17:	pop    %r15
  47bc19:	ret
  47bc1a:	mov    %rbx,%rdi
  47bc1d:	pop    %rbx
  47bc1e:	pop    %r14
  47bc20:	pop    %r15
  47bc22:	jmp    *0x6b8b0(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  47bc28:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  47bc2d:	mov    0x8(%r15),%rsi
  47bc31:	mov    %rax,%r14
  47bc34:	test   %rsi,%rsi
  47bc37:	je     47bc45 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3d_5slice15IterMutProducerAyj4_EINtNtB3b_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB34_s_0uuE0TuuEE0B6J_EEB5B_+0xa5>
  47bc39:	mov    0x10(%r15),%rdx
  47bc3d:	mov    %rbx,%rdi
  47bc40:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  47bc45:	mov    %r14,%rdi
  47bc48:	call   4dd7a0 <_Unwind_Resume@plt>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
