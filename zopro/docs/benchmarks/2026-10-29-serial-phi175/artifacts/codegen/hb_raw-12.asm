
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047bc50 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_>:
  47bc50:	push   %rbp
  47bc51:	push   %r15
  47bc53:	push   %r14
  47bc55:	push   %r13
  47bc57:	push   %r12
  47bc59:	push   %rbx
  47bc5a:	push   %rax
  47bc5b:	mov    (%rdi),%rsi
  47bc5e:	mov    0x8(%rdi),%rdx
  47bc62:	mov    0x10(%rdi),%rcx
  47bc66:	mov    0x18(%rdi),%r8
  47bc6a:	mov    0x20(%rdi),%r9
  47bc6e:	mov    0x28(%rdi),%rax
  47bc72:	movq   $0x0,(%rdi)
  47bc79:	test   %rsi,%rsi
  47bc7c:	je     47bd95 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x145>
  47bc82:	mov    %rdi,%rbx
  47bc85:	mov    (%rsi),%rdi
  47bc88:	sub    (%rdx),%rdi
  47bc8b:	mov    (%rcx),%rdx
  47bc8e:	mov    0x8(%rcx),%rcx
  47bc92:	mov    $0x1,%esi
  47bc97:	mov    %rax,(%rsp)
  47bc9b:	call   464c40 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_>
  47bca0:	mov    $0x1,%r13d
  47bca6:	cmpl   $0x2,0x30(%rbx)
  47bcaa:	jb     47bd25 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0xd5>
  47bcac:	mov    0x40(%rbx),%rbp
  47bcb0:	mov    0x38(%rbx),%r12
  47bcb4:	mov    0x0(%rbp),%rax
  47bcb8:	test   %rax,%rax
  47bcbb:	je     47bcc2 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x72>
  47bcbd:	mov    %r12,%rdi
  47bcc0:	call   *%rax
  47bcc2:	mov    0x8(%rbp),%rsi
  47bcc6:	test   %rsi,%rsi
  47bcc9:	je     47bd25 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0xd5>
  47bccb:	mov    0x10(%rbp),%rax
  47bccf:	cmp    $0x20000,%rsi
  47bcd6:	setae  %cl
  47bcd9:	cmp    $0x1001,%rax
  47bcdf:	setb   %dl
  47bce2:	test   %cl,%dl
  47bce4:	je     47bd1c <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0xcc>
  47bce6:	blsr   %eax,%ecx
  47bceb:	jne    47bda3 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x153>
  47bcf1:	movabs $0x8000000000000000,%rcx
  47bcfb:	sub    %rax,%rcx
  47bcfe:	cmp    %rcx,%rsi
  47bd01:	jae    47bda3 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x153>
  47bd07:	cmpb   $0x1,(%r12,%rsi,1)
  47bd0c:	jne    47bd1c <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0xcc>
  47bd0e:	inc    %rsi
  47bd11:	mov    %r12,%rdi
  47bd14:	call   *0x6b7b6(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  47bd1a:	jmp    47bd25 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0xd5>
  47bd1c:	mov    %r12,%rdi
  47bd1f:	call   *0x6b7b3(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  47bd25:	mov    %r13,0x30(%rbx)
  47bd29:	mov    %r15,0x38(%rbx)
  47bd2d:	mov    %r14,0x40(%rbx)
  47bd31:	mov    0x48(%rbx),%rax
  47bd35:	movzbl 0x60(%rbx),%ebp
  47bd39:	mov    (%rax),%rdi
  47bd3c:	cmp    $0x1,%bpl
  47bd40:	jne    47bd54 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x104>
  47bd42:	lock incq (%rdi)
  47bd46:	jle    47be0d <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x1bd>
  47bd4c:	mov    (%rax),%r14
  47bd4f:	mov    %r14,%rdi
  47bd52:	jmp    47bd54 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x104>
  47bd54:	mov    $0x3,%eax
  47bd59:	mov    0x58(%rbx),%rsi
  47bd5d:	xchg   %rax,0x50(%rbx)
  47bd61:	cmp    $0x2,%rax
  47bd65:	jne    47bd73 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x123>
  47bd67:	add    $0x1d8,%rdi
  47bd6e:	call   fef40 <_RNvMNtCs8zaw6vLE3tn_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread>
  47bd73:	test   %bpl,%bpl
  47bd76:	je     47bd86 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x136>
  47bd78:	lock decq (%r14)
  47bd7c:	jne    47bd86 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x136>
  47bd7e:	mov    %r14,%rdi
  47bd81:	call   ffd00 <_RNvMsn_NtCs6i54tJFfzR_5alloc4syncINtB5_3ArcNtNtCs8zaw6vLE3tn_10rayon_core8registry8RegistryE9drop_slowBJ_>
  47bd86:	add    $0x8,%rsp
  47bd8a:	pop    %rbx
  47bd8b:	pop    %r12
  47bd8d:	pop    %r13
  47bd8f:	pop    %r14
  47bd91:	pop    %r15
  47bd93:	pop    %rbp
  47bd94:	ret
  47bd95:	lea    0x6a4ac(%rip),%rdi        # 4e6248 <__frame_dummy_init_array_entry+0x7a58>
  47bd9c:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  47bda1:	jmp    47be0d <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x1bd>
  47bda3:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  47bda8:	mov    0x8(%rbp),%rsi
  47bdac:	test   %rsi,%rsi
  47bdaf:	je     47bdbd <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x16d>
  47bdb1:	mov    0x10(%rbp),%rdx
  47bdb5:	mov    %r12,%rdi
  47bdb8:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  47bdbd:	mov    %r13,0x30(%rbx)
  47bdc1:	mov    %r15,0x38(%rbx)
  47bdc5:	mov    %r14,0x40(%rbx)
  47bdc9:	jmp    47be08 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x1b8>
  47bdcb:	test   %bpl,%bpl
  47bdce:	je     47be08 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x1b8>
  47bdd0:	lock decq (%r14)
  47bdd4:	jne    47be08 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x1b8>
  47bdd6:	mov    %r14,%rdi
  47bdd9:	call   ffd00 <_RNvMsn_NtCs6i54tJFfzR_5alloc4syncINtB5_3ArcNtNtCs8zaw6vLE3tn_10rayon_core8registry8RegistryE9drop_slowBJ_>
  47bdde:	jmp    47be08 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x1b8>
  47bde0:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47bde5:	mov    %rax,%rdi
  47bde8:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  47bded:	mov    $0x2,%r13d
  47bdf3:	mov    %rax,%r15
  47bdf6:	mov    %rdx,%r14
  47bdf9:	cmpl   $0x2,0x30(%rbx)
  47bdfd:	jae    47bcac <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0x5c>
  47be03:	jmp    47bd25 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_+0xd5>
  47be08:	call   104a50 <_RNvXNtCs8zaw6vLE3tn_10rayon_core6unwindNtB2_12AbortIfPanicNtNtNtCsgxBkk5gSRhY_4core3ops4drop4Drop4drop>
  47be0d:	ud2
  47be0f:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
