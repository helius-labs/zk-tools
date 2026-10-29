
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047b4c0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_>:
  47b4c0:	push   %rbp
  47b4c1:	push   %r15
  47b4c3:	push   %r14
  47b4c5:	push   %r13
  47b4c7:	push   %r12
  47b4c9:	push   %rbx
  47b4ca:	sub    $0xe8,%rsp
  47b4d1:	vmovups (%rdi),%ymm0
  47b4d5:	vmovups 0x10(%rdi),%ymm1
  47b4da:	mov    0x100(%rsi),%rcx
  47b4e1:	lea    0x110(%rsi),%rax
  47b4e8:	mov    %edx,0xc(%rsp)
  47b4ec:	mov    %rsi,%rbx
  47b4ef:	mov    %rdi,%r15
  47b4f2:	mov    %rax,0x58(%rsp)
  47b4f7:	movq   $0x0,0x60(%rsp)
  47b500:	mov    %rcx,0x68(%rsp)
  47b505:	movb   $0x0,0x70(%rsp)
  47b50a:	vmovups %ymm0,0x10(%rsp)
  47b510:	vmovups %ymm1,0x20(%rsp)
  47b516:	movq   $0x0,0x40(%rsp)
  47b51f:	mov    0x118(%rsi),%rax
  47b526:	mov    0x108(%rax),%r12
  47b52d:	mov    0x100(%rax),%r14
  47b534:	mov    0x118(%rsi),%rcx
  47b53b:	mov    0x108(%rcx),%rbp
  47b542:	mov    0x100(%rcx),%rax
  47b549:	mov    0x128(%rsi),%rsi
  47b550:	mov    %rbp,%rcx
  47b553:	sub    %rax,%rcx
  47b556:	cmp    %rsi,%rcx
  47b559:	jge    47b7d2 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x312>
  47b55f:	mov    0x120(%rbx),%rax
  47b566:	dec    %rsi
  47b569:	sub    %r14,%r12
  47b56c:	lea    0x6dd(%rip),%r13        # 47bc50 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_buNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB21_5slice15IterMutProducerAyj4_EINtNtB1Z_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEs_0E0uENtB5_3Job7executeB4p_>
  47b573:	lea    0x10(%rsp),%r14
  47b578:	movabs $0x100000000,%rcx
  47b582:	and    %rbp,%rsi
  47b585:	inc    %rbp
  47b588:	shl    $0x4,%rsi
  47b58c:	mov    %r13,(%rax,%rsi,1)
  47b590:	mov    %r14,0x8(%rax,%rsi,1)
  47b595:	mov    0x118(%rbx),%rdx
  47b59c:	mov    %rbp,0x108(%rdx)
  47b5a3:	mov    0x110(%rbx),%rsi
  47b5aa:	lea    0x1d8(%rsi),%rdi
  47b5b1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  47b5c0:	mov    0x1f0(%rsi),%rax
  47b5c7:	test   %rcx,%rax
  47b5ca:	jne    47b5ea <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x12a>
  47b5cc:	mov    %rax,%rdx
  47b5cf:	or     %rcx,%rdx
  47b5d2:	lock cmpxchg %rdx,0x1f0(%rsi)
  47b5db:	jne    47b5c0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x100>
  47b5dd:	mov    %rdx,%rax
  47b5e0:	and    $0xffff,%rax
  47b5e6:	jne    47b5f8 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x138>
  47b5e8:	jmp    47b60c <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x14c>
  47b5ea:	mov    %rax,%rdx
  47b5ed:	mov    %rdx,%rax
  47b5f0:	and    $0xffff,%rax
  47b5f6:	je     47b60c <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x14c>
  47b5f8:	test   %r12,%r12
  47b5fb:	jg     47b7c0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x300>
  47b601:	shr    $0x10,%edx
  47b604:	cmp    %eax,%edx
  47b606:	je     47b7c0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x300>
  47b60c:	mov    0x38(%r15),%rcx
  47b610:	mov    0x30(%r15),%rax
  47b614:	mov    0x40(%r15),%r8
  47b618:	mov    0x48(%r15),%r9
  47b61c:	mov    0x50(%r15),%rsi
  47b620:	mov    (%rcx),%rdx
  47b623:	mov    (%rax),%rdi
  47b626:	mov    0x8(%rcx),%rcx
  47b62a:	movzbl 0xc(%rsp),%ebp
  47b62f:	mov    %rsi,(%rsp)
  47b633:	mov    %ebp,%esi
  47b635:	vzeroupper
  47b638:	call   464c40 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_>
  47b63d:	nopl   (%rax)
  47b640:	mov    0x60(%rsp),%rax
  47b645:	cmp    $0x3,%rax
  47b649:	je     47b67f <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x1bf>
  47b64b:	mov    %rbx,%rdi
  47b64e:	call   1c26e0 <_RNvMs8_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_12WorkerThread14take_local_job.565>
  47b653:	test   %rax,%rax
  47b656:	je     47b670 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x1b0>
  47b658:	mov    %r14,%rcx
  47b65b:	mov    %rax,%rsi
  47b65e:	xor    %rdx,%rcx
  47b661:	xor    %r13,%rsi
  47b664:	or     %rcx,%rsi
  47b667:	je     47b6a7 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x1e7>
  47b669:	mov    %rdx,%rdi
  47b66c:	call   *%rax
  47b66e:	jmp    47b640 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x180>
  47b670:	mov    0x60(%rsp),%rax
  47b675:	cmp    $0x3,%rax
  47b679:	jne    47b808 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x348>
  47b67f:	mov    0x40(%rsp),%rax
  47b684:	cmp    $0x1,%rax
  47b688:	je     47b7ae <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x2ee>
  47b68e:	cmp    $0x2,%rax
  47b692:	jne    47b7f0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x330>
  47b698:	mov    0x48(%rsp),%rdi
  47b69d:	mov    0x50(%rsp),%rsi
  47b6a2:	call   104520 <_RNvNtCs8zaw6vLE3tn_10rayon_core6unwind16resume_unwinding>
  47b6a7:	vmovups 0x10(%rsp),%zmm0
  47b6b2:	vmovups 0x38(%rsp),%zmm1
  47b6bd:	vmovups %zmm0,0x80(%rsp)
  47b6c5:	vmovups %zmm1,0xa8(%rsp)
  47b6d0:	mov    0x80(%rsp),%rax
  47b6d8:	test   %rax,%rax
  47b6db:	je     47b81a <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x35a>
  47b6e1:	mov    0x90(%rsp),%r10
  47b6e9:	mov    0x88(%rsp),%rcx
  47b6f1:	mov    (%rax),%rdi
  47b6f4:	mov    0xa8(%rsp),%rsi
  47b6fc:	mov    0xa0(%rsp),%r9
  47b704:	mov    0x98(%rsp),%r8
  47b70c:	mov    (%r10),%rdx
  47b70f:	sub    (%rcx),%rdi
  47b712:	mov    0x8(%r10),%rcx
  47b716:	mov    %rsi,(%rsp)
  47b71a:	mov    %ebp,%esi
  47b71c:	vzeroupper
  47b71f:	call   464c40 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_>
  47b724:	cmpq   $0x2,0xb0(%rsp)
  47b72d:	jb     47b7ae <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x2ee>
  47b72f:	mov    0xc0(%rsp),%r15
  47b737:	mov    0xb8(%rsp),%rbx
  47b73f:	mov    (%r15),%rax
  47b742:	test   %rax,%rax
  47b745:	je     47b74c <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x28c>
  47b747:	mov    %rbx,%rdi
  47b74a:	call   *%rax
  47b74c:	mov    0x8(%r15),%rsi
  47b750:	test   %rsi,%rsi
  47b753:	je     47b7ae <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x2ee>
  47b755:	mov    0x10(%r15),%rax
  47b759:	cmp    $0x20000,%rsi
  47b760:	setae  %cl
  47b763:	cmp    $0x1001,%rax
  47b769:	setb   %dl
  47b76c:	test   %cl,%dl
  47b76e:	je     47b7a5 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x2e5>
  47b770:	blsr   %eax,%ecx
  47b775:	jne    47b82b <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x36b>
  47b77b:	movabs $0x8000000000000000,%rcx
  47b785:	sub    %rax,%rcx
  47b788:	cmp    %rcx,%rsi
  47b78b:	jae    47b82b <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x36b>
  47b791:	cmpb   $0x1,(%rbx,%rsi,1)
  47b795:	jne    47b7a5 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x2e5>
  47b797:	inc    %rsi
  47b79a:	mov    %rbx,%rdi
  47b79d:	call   *0x6bd2d(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  47b7a3:	jmp    47b7ae <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x2ee>
  47b7a5:	mov    %rbx,%rdi
  47b7a8:	call   *0x6bd2a(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  47b7ae:	add    $0xe8,%rsp
  47b7b5:	pop    %rbx
  47b7b6:	pop    %r12
  47b7b8:	pop    %r13
  47b7ba:	pop    %r14
  47b7bc:	pop    %r15
  47b7be:	pop    %rbp
  47b7bf:	ret
  47b7c0:	mov    $0x1,%esi
  47b7c5:	vzeroupper
  47b7c8:	call   fea70 <_RNvMNtCs8zaw6vLE3tn_10rayon_core5sleepNtB2_5Sleep16wake_any_threads>
  47b7cd:	jmp    47b60c <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x14c>
  47b7d2:	lea    0x118(%rbx),%rdi
  47b7d9:	add    %rsi,%rsi
  47b7dc:	vzeroupper
  47b7df:	call   1c2190 <_RNvMs4_NtCsfQ2VmTO9G43_15crossbeam_deque5dequeINtB5_6WorkerNtNtCs8zaw6vLE3tn_10rayon_core3job6JobRefE6resizeCsHW5vo2gROj_10zopro_core>
  47b7e4:	mov    0x128(%rbx),%rsi
  47b7eb:	jmp    47b55f <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x9f>
  47b7f0:	lea    -0x45c6ab(%rip),%rdi        # 1f14c <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x4d4c>
  47b7f7:	lea    0x69682(%rip),%rdx        # 4e4e80 <__frame_dummy_init_array_entry+0x6690>
  47b7fe:	mov    $0x28,%esi
  47b803:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47b808:	lea    0x60(%rsp),%rsi
  47b80d:	mov    %rbx,%rdi
  47b810:	call   fe230 <_RNvMs8_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_12WorkerThread15wait_until_cold>
  47b815:	jmp    47b67f <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x1bf>
  47b81a:	lea    0x6941f(%rip),%rdi        # 4e4c40 <__frame_dummy_init_array_entry+0x6450>
  47b821:	vzeroupper
  47b824:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  47b829:	jmp    47b86d <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x3ad>
  47b82b:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  47b830:	jmp    47b871 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x3b1>
  47b832:	mov    0x8(%r15),%rsi
  47b836:	mov    %rax,%r14
  47b839:	test   %rsi,%rsi
  47b83c:	je     47b895 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x3d5>
  47b83e:	mov    0x10(%r15),%rdx
  47b842:	mov    %rbx,%rdi
  47b845:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  47b84a:	mov    %r14,%rdi
  47b84d:	call   4dd7a0 <_Unwind_Resume@plt>
  47b852:	mov    %rax,%rdi
  47b855:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  47b85a:	mov    %rdx,%rcx
  47b85d:	lea    0x58(%rsp),%rsi
  47b862:	mov    %rbx,%rdi
  47b865:	mov    %rax,%rdx
  47b868:	call   1049d0 <_RNvNtCs8zaw6vLE3tn_10rayon_core4join23join_recover_from_panic>
  47b86d:	ud2
  47b86f:	jmp    47b871 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x3b1>
  47b871:	mov    %rax,%r14
  47b874:	lea    0x10(%rsp),%rdi
  47b879:	call   42ba60 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvMs4_NtBG_8registryNtB1P_8Registry15in_worker_crossNCINvNtBG_4join12join_contextNCINvNvB2E_4join4calluNCNvMs_NtCsHW5vo2gROj_10zopro_core8lagrangeNtB3v_7Queries7prepares0_0E0NCIB35_uNCB3q_s1_0E0uuE0TuuEE0B4V_EEB3x_>
  47b87e:	jmp    47b895 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_+0x3d5>
  47b880:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47b885:	mov    %rax,%r14
  47b888:	lea    0xb0(%rsp),%rdi
  47b890:	call   2a73c0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtB4_4cell10UnsafeCellINtNtCs8zaw6vLE3tn_10rayon_core3job9JobResultINtNtB4_6result6ResultuReEEEECsHW5vo2gROj_10zopro_core>
  47b895:	mov    %r14,%rdi
  47b898:	call   4dd7a0 <_Unwind_Resume@plt>
  47b89d:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
