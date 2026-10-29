
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047a740 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_>:
  47a740:	push   %rbp
  47a741:	push   %r15
  47a743:	push   %r14
  47a745:	push   %r13
  47a747:	push   %r12
  47a749:	push   %rbx
  47a74a:	sub    $0x168,%rsp
  47a751:	vmovups (%rsi),%zmm0
  47a757:	mov    %ecx,0x8(%rsp)
  47a75b:	mov    0x100(%rdx),%rcx
  47a762:	lea    0x110(%rdx),%rax
  47a769:	mov    %rdi,0x18(%rsp)
  47a76e:	mov    %rsi,%r12
  47a771:	mov    %rdx,%r14
  47a774:	mov    %rax,0xa0(%rsp)
  47a77c:	movq   $0x0,0xa8(%rsp)
  47a788:	mov    %rcx,0xb0(%rsp)
  47a790:	movb   $0x0,0xb8(%rsp)
  47a798:	vmovups %zmm0,0x40(%rsp)
  47a7a0:	movq   $0x0,0x80(%rsp)
  47a7ac:	mov    0x118(%rdx),%rax
  47a7b3:	mov    0x108(%rax),%rbx
  47a7ba:	mov    0x100(%rax),%r15
  47a7c1:	mov    0x118(%rdx),%rcx
  47a7c8:	mov    0x108(%rcx),%rbp
  47a7cf:	mov    0x100(%rcx),%rax
  47a7d6:	mov    0x128(%rdx),%rsi
  47a7dd:	mov    %rbp,%rcx
  47a7e0:	sub    %rax,%rcx
  47a7e3:	cmp    %rsi,%rcx
  47a7e6:	jge    47ab26 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x3e6>
  47a7ec:	mov    0x120(%r14),%rax
  47a7f3:	dec    %rsi
  47a7f6:	sub    %r15,%rbx
  47a7f9:	lea    0x7b0(%rip),%r15        # 47afb0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_>
  47a800:	lea    0x40(%rsp),%r13
  47a805:	movabs $0x100000000,%rcx
  47a80f:	and    %rbp,%rsi
  47a812:	inc    %rbp
  47a815:	shl    $0x4,%rsi
  47a819:	mov    %r15,(%rax,%rsi,1)
  47a81d:	mov    %r13,0x8(%rax,%rsi,1)
  47a822:	mov    0x118(%r14),%rdx
  47a829:	mov    %rbp,0x108(%rdx)
  47a830:	mov    0x110(%r14),%rsi
  47a837:	lea    0x1d8(%rsi),%rdi
  47a83e:	xchg   %ax,%ax
  47a840:	mov    0x1f0(%rsi),%rax
  47a847:	test   %rcx,%rax
  47a84a:	jne    47a86a <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x12a>
  47a84c:	mov    %rax,%rdx
  47a84f:	or     %rcx,%rdx
  47a852:	lock cmpxchg %rdx,0x1f0(%rsi)
  47a85b:	jne    47a840 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x100>
  47a85d:	mov    %rdx,%rax
  47a860:	and    $0xffff,%rax
  47a866:	jne    47a878 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x138>
  47a868:	jmp    47a88c <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x14c>
  47a86a:	mov    %rax,%rdx
  47a86d:	mov    %rdx,%rax
  47a870:	and    $0xffff,%rax
  47a876:	je     47a88c <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x14c>
  47a878:	test   %rbx,%rbx
  47a87b:	jg     47ab14 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x3d4>
  47a881:	shr    $0x10,%edx
  47a884:	cmp    %eax,%edx
  47a886:	je     47ab14 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x3d4>
  47a88c:	vmovups 0x58(%r12),%xmm0
  47a893:	mov    0x40(%r12),%rax
  47a898:	mov    0x48(%r12),%rdx
  47a89d:	mov    0x50(%r12),%rdi
  47a8a2:	mov    0x68(%r12),%r9
  47a8a7:	mov    0x70(%r12),%r10
  47a8ac:	vmovups %xmm0,0xe8(%rsp)
  47a8b5:	mov    (%rax),%rsi
  47a8b8:	mov    (%rdx),%rcx
  47a8bb:	mov    0x8(%rdx),%r8
  47a8bf:	mov    %rdi,0xe0(%rsp)
  47a8c7:	movzbl 0x8(%rsp),%ebp
  47a8cc:	lea    0x20(%rsp),%r12
  47a8d1:	lea    0xe0(%rsp),%rax
  47a8d9:	mov    %r12,%rdi
  47a8dc:	mov    %ebp,%edx
  47a8de:	push   %rax
  47a8df:	push   %r10
  47a8e1:	vzeroupper
  47a8e4:	call   4650c0 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_>
  47a8e9:	add    $0x10,%rsp
  47a8ed:	mov    0x20(%rsp),%rax
  47a8f2:	mov    0x28(%rsp),%rcx
  47a8f7:	mov    0x30(%rsp),%rbx
  47a8fc:	mov    %rax,0x10(%rsp)
  47a901:	mov    %rcx,0x8(%rsp)
  47a906:	cs nopw 0x0(%rax,%rax,1)
  47a910:	mov    0xa8(%rsp),%rax
  47a918:	cmp    $0x3,%rax
  47a91c:	je     47a955 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x215>
  47a91e:	mov    %r14,%rdi
  47a921:	call   1c26e0 <_RNvMs8_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_12WorkerThread14take_local_job.565>
  47a926:	test   %rax,%rax
  47a929:	je     47a943 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x203>
  47a92b:	mov    %r13,%rcx
  47a92e:	mov    %rax,%rsi
  47a931:	xor    %rdx,%rcx
  47a934:	xor    %r15,%rsi
  47a937:	or     %rcx,%rsi
  47a93a:	je     47a9a9 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x269>
  47a93c:	mov    %rdx,%rdi
  47a93f:	call   *%rax
  47a941:	jmp    47a910 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x1d0>
  47a943:	mov    0xa8(%rsp),%rax
  47a94b:	cmp    $0x3,%rax
  47a94f:	jne    47ab5c <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x41c>
  47a955:	vmovdqu 0x88(%rsp),%xmm0
  47a95e:	mov    0x80(%rsp),%rax
  47a966:	cmp    $0x1,%rax
  47a96a:	je     47a986 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x246>
  47a96c:	cmp    $0x2,%rax
  47a970:	jne    47ab44 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x404>
  47a976:	vmovq  %xmm0,%rdi
  47a97b:	vpextrq $0x1,%xmm0,%rsi
  47a981:	call   104520 <_RNvNtCs8zaw6vLE3tn_10rayon_core6unwind16resume_unwinding>
  47a986:	mov    0x18(%rsp),%rcx
  47a98b:	mov    0x10(%rsp),%rdx
  47a990:	mov    0x8(%rsp),%rsi
  47a995:	mov    0x98(%rsp),%rax
  47a99d:	mov    %rdx,(%rcx)
  47a9a0:	mov    %rsi,0x8(%rcx)
  47a9a4:	jmp    47aaf5 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x3b5>
  47a9a9:	vmovups 0x40(%rsp),%zmm0
  47a9b1:	vmovups 0x80(%rsp),%zmm1
  47a9b9:	vmovups %zmm0,0xe0(%rsp)
  47a9c4:	vmovups %zmm1,0x120(%rsp)
  47a9cf:	mov    0xe0(%rsp),%rax
  47a9d7:	test   %rax,%rax
  47a9da:	je     47ab71 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x431>
  47a9e0:	vmovups 0x58(%rsp),%xmm0
  47a9e6:	mov    0x68(%rsp),%rsi
  47a9eb:	mov    0xe8(%rsp),%rcx
  47a9f3:	mov    0xf0(%rsp),%rdx
  47a9fb:	mov    0x118(%rsp),%r10
  47aa03:	mov    0x110(%rsp),%r9
  47aa0b:	mov    %rsi,0x30(%rsp)
  47aa10:	vmovaps %xmm0,0x20(%rsp)
  47aa16:	mov    (%rax),%rsi
  47aa19:	mov    0x8(%rdx),%r8
  47aa1d:	sub    (%rcx),%rsi
  47aa20:	mov    (%rdx),%rcx
  47aa23:	mov    %ebp,%edx
  47aa25:	lea    0xc8(%rsp),%rdi
  47aa2d:	push   %r12
  47aa2f:	push   %r10
  47aa31:	vzeroupper
  47aa34:	call   4650c0 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_>
  47aa39:	add    $0x10,%rsp
  47aa3d:	cmpq   $0x2,0x120(%rsp)
  47aa46:	jb     47aace <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x38e>
  47aa4c:	mov    0x130(%rsp),%r15
  47aa54:	mov    0x128(%rsp),%r14
  47aa5c:	mov    (%r15),%rax
  47aa5f:	test   %rax,%rax
  47aa62:	je     47aa69 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x329>
  47aa64:	mov    %r14,%rdi
  47aa67:	call   *%rax
  47aa69:	mov    0x8(%r15),%rsi
  47aa6d:	test   %rsi,%rsi
  47aa70:	je     47aace <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x38e>
  47aa72:	mov    0x10(%r15),%rax
  47aa76:	cmp    $0x20000,%rsi
  47aa7d:	setae  %cl
  47aa80:	cmp    $0x1001,%rax
  47aa86:	setb   %dl
  47aa89:	test   %cl,%dl
  47aa8b:	je     47aac3 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x383>
  47aa8d:	blsr   %eax,%ecx
  47aa92:	jne    47ab82 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x442>
  47aa98:	movabs $0x8000000000000000,%rcx
  47aaa2:	sub    %rax,%rcx
  47aaa5:	cmp    %rcx,%rsi
  47aaa8:	jae    47ab82 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x442>
  47aaae:	cmpb   $0x1,(%r14,%rsi,1)
  47aab3:	jne    47aac5 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x385>
  47aab5:	inc    %rsi
  47aab8:	mov    %r14,%rdi
  47aabb:	call   *0x6ca0f(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  47aac1:	jmp    47aace <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x38e>
  47aac3:	jmp    47aac5 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x385>
  47aac5:	mov    %r14,%rdi
  47aac8:	call   *0x6ca0a(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  47aace:	mov    0x18(%rsp),%rcx
  47aad3:	mov    0x10(%rsp),%rax
  47aad8:	vmovdqu 0xc8(%rsp),%xmm0
  47aae1:	mov    0x8(%rsp),%rdx
  47aae6:	mov    %rax,(%rcx)
  47aae9:	mov    0xd8(%rsp),%rax
  47aaf1:	mov    %rdx,0x8(%rcx)
  47aaf5:	mov    %rbx,0x10(%rcx)
  47aaf9:	vmovdqu %xmm0,0x18(%rcx)
  47aafe:	mov    %rax,0x28(%rcx)
  47ab02:	add    $0x168,%rsp
  47ab09:	pop    %rbx
  47ab0a:	pop    %r12
  47ab0c:	pop    %r13
  47ab0e:	pop    %r14
  47ab10:	pop    %r15
  47ab12:	pop    %rbp
  47ab13:	ret
  47ab14:	mov    $0x1,%esi
  47ab19:	vzeroupper
  47ab1c:	call   fea70 <_RNvMNtCs8zaw6vLE3tn_10rayon_core5sleepNtB2_5Sleep16wake_any_threads>
  47ab21:	jmp    47a88c <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x14c>
  47ab26:	lea    0x118(%r14),%rdi
  47ab2d:	add    %rsi,%rsi
  47ab30:	vzeroupper
  47ab33:	call   1c2190 <_RNvMs4_NtCsfQ2VmTO9G43_15crossbeam_deque5dequeINtB5_6WorkerNtNtCs8zaw6vLE3tn_10rayon_core3job6JobRefE6resizeCsHW5vo2gROj_10zopro_core>
  47ab38:	mov    0x128(%r14),%rsi
  47ab3f:	jmp    47a7ec <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0xac>
  47ab44:	lea    -0x45b9ff(%rip),%rdi        # 1f14c <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x4d4c>
  47ab4b:	lea    0x6a32e(%rip),%rdx        # 4e4e80 <__frame_dummy_init_array_entry+0x6690>
  47ab52:	mov    $0x28,%esi
  47ab57:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  47ab5c:	lea    0xa8(%rsp),%rsi
  47ab64:	mov    %r14,%rdi
  47ab67:	call   fe230 <_RNvMs8_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_12WorkerThread15wait_until_cold>
  47ab6c:	jmp    47a955 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x215>
  47ab71:	lea    0x6a0c8(%rip),%rdi        # 4e4c40 <__frame_dummy_init_array_entry+0x6450>
  47ab78:	vzeroupper
  47ab7b:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  47ab80:	jmp    47abc7 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x487>
  47ab82:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  47ab87:	jmp    47abcb <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x48b>
  47ab89:	mov    0x8(%r15),%rsi
  47ab8d:	mov    %rax,%rbx
  47ab90:	test   %rsi,%rsi
  47ab93:	je     47abef <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x4af>
  47ab95:	mov    0x10(%r15),%rdx
  47ab99:	mov    %r14,%rdi
  47ab9c:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  47aba1:	mov    %rbx,%rdi
  47aba4:	call   4dd7a0 <_Unwind_Resume@plt>
  47aba9:	mov    %rax,%rdi
  47abac:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  47abb1:	mov    %rdx,%rcx
  47abb4:	lea    0xa0(%rsp),%rsi
  47abbc:	mov    %r14,%rdi
  47abbf:	mov    %rax,%rdx
  47abc2:	call   1049d0 <_RNvNtCs8zaw6vLE3tn_10rayon_core4join23join_recover_from_panic>
  47abc7:	ud2
  47abc9:	jmp    47abcb <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x48b>
  47abcb:	mov    %rax,%rbx
  47abce:	lea    0x40(%rsp),%rdi
  47abd3:	call   220200 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job8StackJobNtNtBG_5latch9SpinLatchNCINvNvNtBG_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultAyj4_ENCINvNvNtB2p_8plumbing24bridge_producer_consumer6helperINtNtB2r_5slice12IterProducerINtNtCsgHCcee0nhgM_11zopro_bn2542fp2FpNtB4T_8FrConfigEEINtNtB2p_3map11MapConsumerINtB2l_15CollectConsumerB3m_ENvMB4R_B4O_12to_canonicalEEs_0E0B2i_EECsHW5vo2gROj_10zopro_core>
  47abd8:	jmp    47abef <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB3K_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EE0NCBR_s_0INtB32_13CollectResultB3H_EB5Y_E0B4U_+0x4af>
  47abda:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47abdf:	mov    %rax,%rbx
  47abe2:	lea    0x120(%rsp),%rdi
  47abea:	call   220150 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtB4_4cell10UnsafeCellINtNtCs8zaw6vLE3tn_10rayon_core3job9JobResultINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultAyj4_EEEECsHW5vo2gROj_10zopro_core>
  47abef:	mov    %rbx,%rdi
  47abf2:	call   4dd7a0 <_Unwind_Resume@plt>
  47abf7:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
