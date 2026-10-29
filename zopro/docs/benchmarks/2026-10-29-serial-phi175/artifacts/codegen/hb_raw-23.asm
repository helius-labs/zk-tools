
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004653e0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_>:
  4653e0:	push   %rbp
  4653e1:	push   %r15
  4653e3:	push   %r14
  4653e5:	push   %r13
  4653e7:	push   %r12
  4653e9:	push   %rbx
  4653ea:	sub    $0xf8,%rsp
  4653f1:	mov    %rdx,0xe8(%rsp)
  4653f9:	mov    %rdi,0xd8(%rsp)
  465401:	mov    (%rsi),%rax
  465404:	vmovups 0x8(%rsi),%xmm0
  465409:	mov    0x18(%rsi),%r12
  46540d:	mov    0x20(%rsi),%r13
  465411:	mov    0x28(%rsi),%rbp
  465415:	mov    0x30(%rsi),%rbx
  465419:	mov    (%rax),%r14
  46541c:	movq   $0x0,0x28(%rsp)
  465425:	movq   $0x8,0x30(%rsp)
  46542e:	movq   $0x0,0x38(%rsp)
  465437:	vmovaps %xmm0,0x10(%rsp)
  46543d:	mov    %r14,0xf0(%rsp)
  465445:	test   %r14,%r14
  465448:	jne    465881 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x4a1>
  46544e:	mov    $0x8,%eax
  465453:	xor    %r15d,%r15d
  465456:	vmovaps 0x10(%rsp),%xmm0
  46545c:	lea    0xe8(%rsp),%rcx
  465464:	mov    %r14,0x58(%rsp)
  465469:	vmovaps %xmm0,0x80(%rsp)
  465472:	mov    %r12,0x90(%rsp)
  46547a:	mov    %r13,0x98(%rsp)
  465482:	mov    %rbp,0xa0(%rsp)
  46548a:	mov    %rcx,0xa8(%rsp)
  465492:	lea    (%r15,%r15,2),%rcx
  465496:	mov    %rbx,0xb0(%rsp)
  46549e:	movq   $0x0,0xb8(%rsp)
  4654aa:	mov    %r14,0xc0(%rsp)
  4654b2:	lea    (%rax,%rcx,8),%rax
  4654b6:	lea    0x80(%rsp),%rcx
  4654be:	mov    %rcx,0x40(%rsp)
  4654c3:	mov    %rax,0x48(%rsp)
  4654c8:	mov    %fs:0xffffffffffffffc0,%rax
  4654d1:	mov    %r14,0x50(%rsp)
  4654d6:	test   %rax,%rax
  4654d9:	je     4654e3 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x103>
  4654db:	add    $0x110,%rax
  4654e1:	jmp    4654e8 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x108>
  4654e3:	call   1041b0 <_RNvNtCs8zaw6vLE3tn_10rayon_core8registry15global_registry>
  4654e8:	mov    (%rax),%rax
  4654eb:	mov    0x208(%rax),%rcx
  4654f2:	xor    %eax,%eax
  4654f4:	cmp    $0xffffffffffffffff,%r14
  4654f8:	sete   %al
  4654fb:	cmp    %rcx,%rax
  4654fe:	cmova  %rax,%rcx
  465502:	lea    0x40(%rsp),%r12
  465507:	xor    %edx,%edx
  465509:	lea    0x60(%rsp),%rdi
  46550e:	mov    $0x1,%r8d
  465514:	mov    %r14,%rsi
  465517:	xor    %r9d,%r9d
  46551a:	push   %r12
  46551c:	push   %r14
  46551e:	call   4659e0 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_>
  465523:	add    $0x10,%rsp
  465527:	mov    0x70(%rsp),%rbx
  46552c:	mov    %rbx,0x40(%rsp)
  465531:	cmp    %r14,%rbx
  465534:	jne    4658cb <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x4eb>
  46553a:	vmovups 0x28(%rsp),%xmm0
  465540:	add    %r14,%r15
  465543:	mov    %r15,0x70(%rsp)
  465548:	vmovaps %xmm0,0x60(%rsp)
  46554e:	je     465917 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x537>
  465554:	mov    0x68(%rsp),%rdx
  465559:	dec    %r15
  46555c:	mov    (%rdx),%rax
  46555f:	mov    0x8(%rdx),%rcx
  465563:	mov    0x10(%rdx),%r12
  465567:	mov    %rdx,%r13
  46556a:	mov    %rdx,0x78(%rsp)
  46556f:	mov    %rax,0x8(%rsp)
  465574:	lea    (%r15,%r15,2),%rax
  465578:	mov    %rcx,0x10(%rsp)
  46557d:	vmovups (%rdx,%rax,8),%xmm0
  465582:	mov    0x10(%rdx,%rax,8),%rcx
  465587:	lea    (%rdx,%rax,8),%rbp
  46558b:	mov    %rcx,0x10(%rdx)
  46558f:	vmovups %xmm0,(%rdx)
  465593:	mov    %r15,0x70(%rsp)
  465598:	mov    %rdx,0x40(%rsp)
  46559d:	mov    0x60(%rsp),%rax
  4655a2:	mov    %rax,0x50(%rsp)
  4655a7:	mov    %rax,0xd0(%rsp)
  4655af:	mov    %rbp,0x58(%rsp)
  4655b4:	je     4656fe <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x31e>
  4655ba:	lea    0x1ff(%r12),%rax
  4655c2:	lea    0x18(%rdx),%r13
  4655c6:	shr    $0x9,%rax
  4655ca:	mov    %rax,0xe0(%rsp)
  4655d2:	jmp    4655fa <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x21a>
  4655d4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  4655e0:	mov    %rbx,%rdi
  4655e3:	call   *0x81eef(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  4655e9:	add    $0x18,%r13
  4655ed:	add    $0x18,%r14
  4655f1:	cmp    %rbp,%r14
  4655f4:	je     46578c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x3ac>
  4655fa:	mov    -0x18(%r13),%r15
  4655fe:	cmp    $0xffffffffffffffff,%r15
  465602:	je     4656fe <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x31e>
  465608:	lea    -0x18(%r13),%r14
  46560c:	mov    -0x10(%r13),%rbx
  465610:	mov    0x10(%r14),%rax
  465614:	mov    %r12,0x28(%rsp)
  465619:	mov    %rax,0x80(%rsp)
  465621:	cmp    %rax,%r12
  465624:	jne    46585c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x47c>
  46562a:	mov    0x10(%rsp),%rax
  46562f:	mov    %rax,0x80(%rsp)
  465637:	mov    %fs:0xffffffffffffffc0,%rax
  465640:	mov    %r12,0x88(%rsp)
  465648:	movq   $0x200,0x90(%rsp)
  465654:	mov    %rbx,0x98(%rsp)
  46565c:	mov    %r12,0xa0(%rsp)
  465664:	movq   $0x200,0xa8(%rsp)
  465670:	test   %rax,%rax
  465673:	je     465680 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x2a0>
  465675:	add    $0x110,%rax
  46567b:	jmp    465685 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x2a5>
  46567d:	nopl   (%rax)
  465680:	call   1041b0 <_RNvNtCs8zaw6vLE3tn_10rayon_core8registry15global_registry>
  465685:	mov    (%rax),%rax
  465688:	mov    0x208(%rax),%rdx
  46568f:	mov    0xe0(%rsp),%rdi
  465697:	mov    $0x1,%ecx
  46569c:	lea    0x80(%rsp),%r8
  4656a4:	lea    0x7(%rsp),%r9
  4656a9:	xor    %esi,%esi
  4656ab:	call   24f540 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB6_3zip11ZipProducerINtNtNtB8_5slice6chunks17ChunksMutProducerINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2l_8G2ConfigEEINtB1F_14ChunksProducerB2i_EEINtNtB6_8for_each15ForEachConsumerNCINvNtCsHW5vo2gROj_10zopro_core10msm_affine13merge_bucketsB31_E0EEB4q_>
  4656b0:	test   %r15,%r15
  4656b3:	je     4655e9 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x209>
  4656b9:	cmp    $0x3ff,%r15
  4656c0:	jbe    4655e0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x200>
  4656c6:	shl    $0x7,%r15
  4656ca:	movabs $0x7ffffffffffffff9,%rax
  4656d4:	mov    %r15,%rsi
  4656d7:	or     $0x1,%rsi
  4656db:	cmp    %rax,%rsi
  4656de:	jae    465923 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x543>
  4656e4:	cmpb   $0x1,(%rbx,%r15,1)
  4656e9:	jne    4656f9 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x319>
  4656eb:	mov    %rbx,%rdi
  4656ee:	call   *0x81ddc(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  4656f4:	jmp    4655e9 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x209>
  4656f9:	jmp    4655e0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x200>
  4656fe:	sub    %r13,%rbp
  465701:	movabs $0xaaaaaaaaaaaaaaab,%rax
  46570b:	mov    %rbp,%rdx
  46570e:	mulx   %rax,%rbx,%rbx
  465713:	mov    %r13,0x48(%rsp)
  465718:	je     46578c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x3ac>
  46571a:	mov    0x81daf(%rip),%r14        # 4e74d0 <munmap@GLIBC_2.2.5>
  465721:	mov    0x81db0(%rip),%r15        # 4e74d8 <free@GLIBC_2.2.5>
  465728:	shr    $0x4,%rbx
  46572c:	add    $0x8,%r13
  465730:	jmp    46574c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x36c>
  465732:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  465740:	call   *%r15
  465743:	add    $0x18,%r13
  465747:	dec    %rbx
  46574a:	je     46578c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x3ac>
  46574c:	mov    -0x8(%r13),%rax
  465750:	test   %rax,%rax
  465753:	je     465743 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x363>
  465755:	mov    0x0(%r13),%rdi
  465759:	cmp    $0x3ff,%rax
  46575f:	jbe    465740 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x360>
  465761:	shl    $0x7,%rax
  465765:	movabs $0x7ffffffffffffff9,%rcx
  46576f:	mov    %rax,%rsi
  465772:	or     $0x1,%rsi
  465776:	cmp    %rcx,%rsi
  465779:	jae    46591e <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x53e>
  46577f:	cmpb   $0x1,(%rdi,%rax,1)
  465783:	jne    46578a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x3aa>
  465785:	call   *%r14
  465788:	jmp    465743 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x363>
  46578a:	jmp    465740 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x360>
  46578c:	mov    0xd0(%rsp),%rax
  465794:	mov    0x10(%rsp),%r13
  465799:	mov    0x8(%rsp),%rbp
  46579e:	test   %rax,%rax
  4657a1:	je     4657ed <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x40d>
  4657a3:	cmp    $0x1555,%rax
  4657a9:	jbe    4657e0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x400>
  4657ab:	mov    0x78(%rsp),%rdi
  4657b0:	shl    $0x3,%rax
  4657b4:	movabs $0x7ffffffffffffff9,%rcx
  4657be:	lea    (%rax,%rax,2),%rax
  4657c2:	mov    %rax,%rsi
  4657c5:	or     $0x1,%rsi
  4657c9:	cmp    %rcx,%rsi
  4657cc:	jae    465928 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x548>
  4657d2:	cmpb   $0x1,(%rdi,%rax,1)
  4657d6:	jne    4657e7 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x407>
  4657d8:	call   *0x81cf2(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  4657de:	jmp    4657ed <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x40d>
  4657e0:	mov    0x78(%rsp),%rdi
  4657e5:	jmp    4657e7 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x407>
  4657e7:	call   *0x81ceb(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  4657ed:	mov    0xd8(%rsp),%rdi
  4657f5:	mov    %r13,%rsi
  4657f8:	mov    %r12,%rdx
  4657fb:	call   246b10 <_RINvNtCsHW5vo2gROj_10zopro_core3msm15weighted_affineNtNtCsgHCcee0nhgM_11zopro_bn2545curve8G2ConfigEB4_>
  465800:	test   %rbp,%rbp
  465803:	je     46584a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x46a>
  465805:	cmp    $0x3ff,%rbp
  46580c:	jbe    46583f <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x45f>
  46580e:	shl    $0x7,%rbp
  465812:	movabs $0x7ffffffffffffff9,%rax
  46581c:	mov    %rbp,%rsi
  46581f:	or     $0x1,%rsi
  465823:	cmp    %rax,%rsi
  465826:	jae    46592d <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x54d>
  46582c:	cmpb   $0x1,0x0(%r13,%rbp,1)
  465832:	jne    465841 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x461>
  465834:	mov    %r13,%rdi
  465837:	call   *0x81c93(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  46583d:	jmp    46584a <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x46a>
  46583f:	jmp    465841 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x461>
  465841:	mov    %r13,%rdi
  465844:	call   *0x81c8e(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  46584a:	add    $0xf8,%rsp
  465851:	pop    %rbx
  465852:	pop    %r12
  465854:	pop    %r13
  465856:	pop    %r14
  465858:	pop    %r15
  46585a:	pop    %rbp
  46585b:	ret
  46585c:	mov    %r13,0x48(%rsp)
  465861:	lea    0x7b9c0(%rip),%r8        # 4e1228 <__frame_dummy_init_array_entry+0x2a38>
  465868:	lea    0x28(%rsp),%rdi
  46586d:	lea    0x80(%rsp),%rsi
  465875:	xor    %edx,%edx
  465877:	call   ef7d2 <_RINvNtCsgxBkk5gSRhY_4core9panicking13assert_failedjjEB4_>
  46587c:	jmp    46591c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x53c>
  465881:	lea    0x28(%rsp),%rdi
  465886:	mov    $0x8,%ecx
  46588b:	mov    $0x18,%r8d
  465891:	xor    %esi,%esi
  465893:	mov    %r14,%rdx
  465896:	call   1b5ee0 <_RINvNvMs2_NtCs6i54tJFfzR_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsHW5vo2gROj_10zopro_core>
  46589b:	mov    0x28(%rsp),%rax
  4658a0:	mov    0x38(%rsp),%r15
  4658a5:	sub    %r15,%rax
  4658a8:	cmp    %r14,%rax
  4658ab:	jae    465932 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x552>
  4658b1:	lea    -0x445f11(%rip),%rdi        # 1f9a7 <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x55a7>
  4658b8:	lea    0x7f141(%rip),%rdx        # 4e4a00 <__frame_dummy_init_array_entry+0x6210>
  4658bf:	mov    $0x2f,%esi
  4658c4:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  4658c9:	jmp    46591c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x53c>
  4658cb:	lea    0xf0(%rsp),%rax
  4658d3:	lea    -0x372f0a(%rip),%rcx        # f29d0 <_RNvXsd_NtNtNtCsgxBkk5gSRhY_4core3fmt3num3impyNtB9_7Display3fmt>
  4658da:	mov    %rax,0x80(%rsp)
  4658e2:	mov    %rcx,0x88(%rsp)
  4658ea:	mov    %r12,0x90(%rsp)
  4658f2:	mov    %rcx,0x98(%rsp)
  4658fa:	lea    -0x44ebaf(%rip),%rdi        # 16d52 <_RNvNvXs3_NtNtCshdGlst1c7nL_10serde_json5value5indexNtB9_5ValueINtNtNtCsgxBkk5gSRhY_4core3ops5index5IndexpE5index4NULL+0x52d2>
  465901:	lea    0x7d1a0(%rip),%rdx        # 4e2aa8 <__frame_dummy_init_array_entry+0x42b8>
  465908:	lea    0x80(%rsp),%rsi
  465910:	call   ee380 <_RNvNtCsgxBkk5gSRhY_4core9panicking9panic_fmt>
  465915:	jmp    46591c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x53c>
  465917:	call   edb27 <_RNvNvMs_NtCs6i54tJFfzR_5alloc3vecINtB6_3VecppE11swap_remove13assert_failed>
  46591c:	ud2
  46591e:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  465923:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  465928:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  46592d:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  465932:	mov    0x30(%rsp),%rax
  465937:	jmp    465456 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x76>
  46593c:	mov    %rax,%r14
  46593f:	jmp    4659ae <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x5ce>
  465941:	jmp    465943 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x563>
  465943:	mov    %rax,%r14
  465946:	mov    %r13,0x48(%rsp)
  46594b:	jmp    465981 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x5a1>
  46594d:	mov    %rax,%r14
  465950:	lea    0x60(%rsp),%rdi
  465955:	jmp    465971 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x591>
  465957:	mov    0x60(%rsp),%rdi
  46595c:	mov    %rax,%r14
  46595f:	mov    %rbx,%rsi
  465962:	call   293590 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2e_8G2ConfigEEEECsHW5vo2gROj_10zopro_core>
  465967:	jmp    46596c <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x58c>
  465969:	mov    %rax,%r14
  46596c:	lea    0x28(%rsp),%rdi
  465971:	call   24f1d0 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs6i54tJFfzR_5alloc3vec3VecIBC_INtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB1e_8G2ConfigEEEECsHW5vo2gROj_10zopro_core>
  465976:	mov    %r14,%rdi
  465979:	call   4dd7a0 <_Unwind_Resume@plt>
  46597e:	mov    %rax,%r14
  465981:	mov    0x10(%rsp),%r13
  465986:	mov    0x8(%rsp),%rbp
  46598b:	test   %r15,%r15
  46598e:	je     4659a4 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x5c4>
  465990:	shl    $0x7,%r15
  465994:	mov    $0x8,%edx
  465999:	mov    %rbx,%rdi
  46599c:	mov    %r15,%rsi
  46599f:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  4659a4:	lea    0x40(%rsp),%rdi
  4659a9:	call   24f440 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtNtCs6i54tJFfzR_5alloc3vec9into_iter8IntoIterINtBG_3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB1B_8G2ConfigEEEECsHW5vo2gROj_10zopro_core>
  4659ae:	test   %rbp,%rbp
  4659b1:	jne    4659bb <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0INtB6_5FnMutTjEE8call_mutBV_+0x5db>
  4659b3:	mov    %r14,%rdi
  4659b6:	call   4dd7a0 <_Unwind_Resume@plt>
  4659bb:	shl    $0x7,%rbp
  4659bf:	mov    $0x8,%edx
  4659c4:	mov    %r13,%rdi
  4659c7:	mov    %rbp,%rsi
  4659ca:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  4659cf:	mov    %r14,%rdi
  4659d2:	call   4dd7a0 <_Unwind_Resume@plt>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
