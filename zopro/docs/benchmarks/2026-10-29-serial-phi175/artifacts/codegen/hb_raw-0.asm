
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047bfb0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_>:
  47bfb0:	push   %r15
  47bfb2:	push   %r14
  47bfb4:	push   %rbx
  47bfb5:	cmpl   $0x2,0x60(%rdi)
  47bfb9:	jb     47c024 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_+0x74>
  47bfbb:	mov    0x70(%rdi),%r15
  47bfbf:	mov    0x68(%rdi),%rbx
  47bfc3:	mov    (%r15),%rax
  47bfc6:	test   %rax,%rax
  47bfc9:	je     47bfd0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_+0x20>
  47bfcb:	mov    %rbx,%rdi
  47bfce:	call   *%rax
  47bfd0:	mov    0x8(%r15),%rsi
  47bfd4:	test   %rsi,%rsi
  47bfd7:	je     47c024 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_+0x74>
  47bfd9:	mov    0x10(%r15),%rax
  47bfdd:	cmp    $0x20000,%rsi
  47bfe4:	setae  %cl
  47bfe7:	cmp    $0x1001,%rax
  47bfed:	setb   %dl
  47bff0:	test   %cl,%dl
  47bff2:	je     47c02a <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_+0x7a>
  47bff4:	blsr   %eax,%ecx
  47bff9:	jne    47c038 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_+0x88>
  47bffb:	movabs $0x8000000000000000,%rcx
  47c005:	sub    %rax,%rcx
  47c008:	cmp    %rcx,%rsi
  47c00b:	jae    47c038 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_+0x88>
  47c00d:	cmpb   $0x1,(%rbx,%rsi,1)
  47c011:	jne    47c02a <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_+0x7a>
  47c013:	inc    %rsi
  47c016:	mov    %rbx,%rdi
  47c019:	pop    %rbx
  47c01a:	pop    %r14
  47c01c:	pop    %r15
  47c01e:	jmp    *0x6b4ac(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  47c024:	pop    %rbx
  47c025:	pop    %r14
  47c027:	pop    %r15
  47c029:	ret
  47c02a:	mov    %rbx,%rdi
  47c02d:	pop    %rbx
  47c02e:	pop    %r14
  47c030:	pop    %r15
  47c032:	jmp    *0x6b4a0(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  47c038:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  47c03d:	mov    0x8(%r15),%rsi
  47c041:	mov    %rax,%r14
  47c044:	test   %rsi,%rsi
  47c047:	je     47c055 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobINtNtBG_5latch8LatchRefNtB1m_9LockLatchENCNCINvMs4_NtBG_8registryNtB28_8Registry14in_worker_coldNCINvNtBG_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB3v_5slice15IterMutProducerAyj4_EINtNtB3t_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB3m_s_0uuE0TuuEE00B71_EEB5T_+0xa5>
  47c049:	mov    0x10(%r15),%rdx
  47c04d:	mov    %rbx,%rdi
  47c050:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  47c055:	mov    %r14,%rdi
  47c058:	call   4dd7a0 <_Unwind_Resume@plt>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
