
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000479bb0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_>:
  479bb0:	push   %rbp
  479bb1:	push   %r15
  479bb3:	push   %r14
  479bb5:	push   %r13
  479bb7:	push   %r12
  479bb9:	push   %rbx
  479bba:	sub    $0x158,%rsp
  479bc1:	vmovups (%rsi),%zmm0
  479bc7:	mov    %ecx,(%rsp)
  479bca:	mov    0x100(%rdx),%rcx
  479bd1:	lea    0x110(%rdx),%rax
  479bd8:	mov    %rdi,0x8(%rsp)
  479bdd:	mov    %rsi,%r14
  479be0:	mov    %rdx,%r12
  479be3:	mov    %rax,0x90(%rsp)
  479beb:	movq   $0x0,0x98(%rsp)
  479bf7:	mov    %rcx,0xa0(%rsp)
  479bff:	movb   $0x0,0xa8(%rsp)
  479c07:	vmovups %zmm0,0x30(%rsp)
  479c12:	movq   $0x0,0x70(%rsp)
  479c1b:	mov    0x118(%rdx),%rax
  479c22:	mov    0x108(%rax),%r15
  479c29:	mov    0x100(%rax),%rbx
  479c30:	mov    0x118(%rdx),%rcx
  479c37:	mov    0x108(%rcx),%r13
  479c3e:	mov    0x100(%rcx),%rax
  479c45:	mov    0x128(%rdx),%rsi
  479c4c:	mov    %r13,%rcx
  479c4f:	sub    %rax,%rcx
  479c52:	cmp    %rsi,%rcx
  479c55:	jge    479f08 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x358>
  479c5b:	mov    0x120(%r12),%rax
  479c63:	dec    %rsi
  479c66:	sub    %rbx,%r15
  479c69:	lea    0x6f0(%rip),%rbp        # 47a360 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3s_8G2ConfigEEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEs_0E0B1P_ENtB5_3Job7executeB6R_>
  479c70:	lea    0x30(%rsp),%rbx
  479c75:	movabs $0x100000000,%rcx
  479c7f:	and    %r13,%rsi
  479c82:	inc    %r13
  479c85:	shl    $0x4,%rsi
  479c89:	mov    %rbp,(%rax,%rsi,1)
  479c8d:	mov    %rbx,0x8(%rax,%rsi,1)
  479c92:	mov    0x118(%r12),%rdx
  479c9a:	mov    %r13,0x108(%rdx)
  479ca1:	mov    0x110(%r12),%rsi
  479ca9:	lea    0x1d8(%rsi),%rdi
  479cb0:	mov    0x1f0(%rsi),%rax
  479cb7:	test   %rcx,%rax
  479cba:	jne    479cda <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x12a>
  479cbc:	mov    %rax,%rdx
  479cbf:	or     %rcx,%rdx
  479cc2:	lock cmpxchg %rdx,0x1f0(%rsi)
  479ccb:	jne    479cb0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x100>
  479ccd:	mov    %rdx,%rax
  479cd0:	and    $0xffff,%rax
  479cd6:	jne    479ce8 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x138>
  479cd8:	jmp    479cfc <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x14c>
  479cda:	mov    %rax,%rdx
  479cdd:	mov    %rdx,%rax
  479ce0:	and    $0xffff,%rax
  479ce6:	je     479cfc <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x14c>
  479ce8:	test   %r15,%r15
  479ceb:	jg     479ef6 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x346>
  479cf1:	shr    $0x10,%edx
  479cf4:	cmp    %eax,%edx
  479cf6:	je     479ef6 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x346>
  479cfc:	vmovups 0x58(%r14),%xmm0
  479d02:	mov    0x40(%r14),%rax
  479d06:	mov    0x48(%r14),%rdx
  479d0a:	mov    0x50(%r14),%rdi
  479d0e:	mov    0x68(%r14),%r9
  479d12:	mov    0x70(%r14),%r10
  479d16:	vmovups %xmm0,0xd8(%rsp)
  479d1f:	mov    (%rax),%rsi
  479d22:	mov    (%rdx),%rcx
  479d25:	mov    0x8(%rdx),%r8
  479d29:	mov    %rdi,0xd0(%rsp)
  479d31:	movzbl (%rsp),%r13d
  479d36:	lea    0xd0(%rsp),%rax
  479d3e:	lea    0x10(%rsp),%rdi
  479d43:	mov    %r13d,%edx
  479d46:	push   %rax
  479d47:	push   %r10
  479d49:	vzeroupper
  479d4c:	call   4659e0 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_>
  479d51:	add    $0x10,%rsp
  479d55:	mov    0x18(%rsp),%rax
  479d5a:	mov    0x10(%rsp),%r15
  479d5f:	mov    0x20(%rsp),%r14
  479d64:	mov    %rax,(%rsp)
  479d68:	nopl   0x0(%rax,%rax,1)
  479d70:	mov    0x98(%rsp),%rax
  479d78:	cmp    $0x3,%rax
  479d7c:	je     479db5 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x205>
  479d7e:	mov    %r12,%rdi
  479d81:	call   1c26e0 <_RNvMs8_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_12WorkerThread14take_local_job.565>
  479d86:	test   %rax,%rax
  479d89:	je     479da3 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x1f3>
  479d8b:	mov    %rbx,%rcx
  479d8e:	mov    %rax,%rsi
  479d91:	xor    %rdx,%rcx
  479d94:	xor    %rbp,%rsi
  479d97:	or     %rcx,%rsi
  479d9a:	je     479e0a <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x25a>
  479d9c:	mov    %rdx,%rdi
  479d9f:	call   *%rax
  479da1:	jmp    479d70 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x1c0>
  479da3:	mov    0x98(%rsp),%rax
  479dab:	cmp    $0x3,%rax
  479daf:	jne    479f42 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x392>
  479db5:	mov    0x70(%rsp),%rax
  479dba:	mov    0x78(%rsp),%rdi
  479dbf:	mov    0x80(%rsp),%rsi
  479dc7:	cmp    $0x1,%rax
  479dcb:	je     479de1 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x231>
  479dcd:	cmp    $0x2,%rax
  479dd1:	jne    479f28 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x378>
  479dd7:	call   104520 <_RNvNtCs8zaw6vLE3tn_10rayon_core6unwind16resume_unwinding>
  479ddc:	jmp    479f8a <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x3da>
  479de1:	mov    0x8(%rsp),%rcx
  479de6:	mov    (%rsp),%rdx
  479dea:	mov    0x88(%rsp),%rax
  479df2:	mov    %r15,(%rcx)
  479df5:	mov    %rdx,0x8(%rcx)
  479df9:	mov    %r14,0x10(%rcx)
  479dfd:	mov    %rdi,0x18(%rcx)
  479e01:	mov    %rsi,0x20(%rcx)
  479e05:	jmp    479ee0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x330>
  479e0a:	vmovups 0x30(%rsp),%zmm0
  479e15:	vmovups 0x70(%rsp),%zmm1
  479e20:	vmovups %zmm0,0xd0(%rsp)
  479e2b:	vmovups %zmm1,0x110(%rsp)
  479e36:	mov    0xd0(%rsp),%rax
  479e3e:	test   %rax,%rax
  479e41:	je     479f57 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x3a7>
  479e47:	vmovups 0x48(%rsp),%xmm0
  479e4d:	mov    0x58(%rsp),%rsi
  479e52:	mov    0xd8(%rsp),%rcx
  479e5a:	mov    0xe0(%rsp),%rdx
  479e62:	mov    0x108(%rsp),%r10
  479e6a:	mov    0x100(%rsp),%r9
  479e72:	mov    %rsi,0x20(%rsp)
  479e77:	vmovaps %xmm0,0x10(%rsp)
  479e7d:	mov    (%rax),%rsi
  479e80:	mov    0x8(%rdx),%r8
  479e84:	sub    (%rcx),%rsi
  479e87:	mov    (%rdx),%rcx
  479e8a:	lea    0x10(%rsp),%rax
  479e8f:	mov    %r13d,%edx
  479e92:	lea    0xb8(%rsp),%rdi
  479e9a:	push   %rax
  479e9b:	push   %r10
  479e9d:	vzeroupper
  479ea0:	call   4659e0 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_>
  479ea5:	add    $0x10,%rsp
  479ea9:	lea    0x110(%rsp),%rdi
  479eb1:	call   295170 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job9JobResultINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2X_8G2ConfigEEEEECsHW5vo2gROj_10zopro_core>
  479eb6:	mov    0x8(%rsp),%rcx
  479ebb:	mov    (%rsp),%rax
  479ebf:	vmovups 0xb8(%rsp),%xmm0
  479ec8:	mov    %r15,(%rcx)
  479ecb:	mov    %rax,0x8(%rcx)
  479ecf:	mov    0xc8(%rsp),%rax
  479ed7:	mov    %r14,0x10(%rcx)
  479edb:	vmovups %xmm0,0x18(%rcx)
  479ee0:	mov    %rax,0x28(%rcx)
  479ee4:	add    $0x158,%rsp
  479eeb:	pop    %rbx
  479eec:	pop    %r12
  479eee:	pop    %r13
  479ef0:	pop    %r14
  479ef2:	pop    %r15
  479ef4:	pop    %rbp
  479ef5:	ret
  479ef6:	mov    $0x1,%esi
  479efb:	vzeroupper
  479efe:	call   fea70 <_RNvMNtCs8zaw6vLE3tn_10rayon_core5sleepNtB2_5Sleep16wake_any_threads>
  479f03:	jmp    479cfc <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x14c>
  479f08:	lea    0x118(%r12),%rdi
  479f10:	add    %rsi,%rsi
  479f13:	vzeroupper
  479f16:	call   1c2190 <_RNvMs4_NtCsfQ2VmTO9G43_15crossbeam_deque5dequeINtB5_6WorkerNtNtCs8zaw6vLE3tn_10rayon_core3job6JobRefE6resizeCsHW5vo2gROj_10zopro_core>
  479f1b:	mov    0x128(%r12),%rsi
  479f23:	jmp    479c5b <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0xab>
  479f28:	lea    -0x45ade3(%rip),%rdi        # 1f14c <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x4d4c>
  479f2f:	lea    0x6af4a(%rip),%rdx        # 4e4e80 <__frame_dummy_init_array_entry+0x6690>
  479f36:	mov    $0x28,%esi
  479f3b:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  479f40:	jmp    479f8a <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x3da>
  479f42:	lea    0x98(%rsp),%rsi
  479f4a:	mov    %r12,%rdi
  479f4d:	call   fe230 <_RNvMs8_NtCs8zaw6vLE3tn_10rayon_core8registryNtB5_12WorkerThread15wait_until_cold>
  479f52:	jmp    479db5 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x205>
  479f57:	lea    0x6ace2(%rip),%rdi        # 4e4c40 <__frame_dummy_init_array_entry+0x6450>
  479f5e:	vzeroupper
  479f61:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  479f66:	jmp    479f8a <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x3da>
  479f68:	jmp    479f91 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x3e1>
  479f6a:	jmp    479fc7 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x417>
  479f6c:	mov    %rax,%rdi
  479f6f:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  479f74:	mov    %rdx,%rcx
  479f77:	lea    0x90(%rsp),%rsi
  479f7f:	mov    %r12,%rdi
  479f82:	mov    %rax,%rdx
  479f85:	call   1049d0 <_RNvNtCs8zaw6vLE3tn_10rayon_core4join23join_recover_from_panic>
  479f8a:	ud2
  479f8c:	mov    %rax,%rbx
  479f8f:	jmp    479f9f <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x3ef>
  479f91:	mov    %rax,%rbx
  479f94:	mov    %r15,%rdi
  479f97:	mov    %r14,%rsi
  479f9a:	call   293590 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2e_8G2ConfigEEEECsHW5vo2gROj_10zopro_core>
  479f9f:	lea    0x70(%rsp),%rdi
  479fa4:	call   295170 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job9JobResultINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2X_8G2ConfigEEEEECsHW5vo2gROj_10zopro_core>
  479fa9:	jmp    479fcf <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x41f>
  479fab:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  479fb0:	mov    %rax,%rbx
  479fb3:	lea    0x110(%rsp),%rdi
  479fbb:	call   295170 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtCs8zaw6vLE3tn_10rayon_core3job9JobResultINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2X_8G2ConfigEEEEECsHW5vo2gROj_10zopro_core>
  479fc0:	jmp    479fca <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x41a>
  479fc2:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  479fc7:	mov    %rax,%rbx
  479fca:	test   %r14,%r14
  479fcd:	jne    479fd7 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x427>
  479fcf:	mov    %rbx,%rdi
  479fd2:	call   4dd7a0 <_Unwind_Resume@plt>
  479fd7:	mov    0x6d4f2(%rip),%r13        # 4e74d0 <munmap@GLIBC_2.2.5>
  479fde:	mov    0x6d4f3(%rip),%rbp        # 4e74d8 <free@GLIBC_2.2.5>
  479fe5:	add    $0x8,%r15
  479fe9:	movabs $0x7ffffffffffffff8,%r12
  479ff3:	jmp    47a00b <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x45b>
  479ff5:	data16 cs nopw 0x0(%rax,%rax,1)
  47a000:	call   *%rbp
  47a002:	add    $0x18,%r15
  47a006:	dec    %r14
  47a009:	je     479fcf <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x41f>
  47a00b:	mov    -0x8(%r15),%rax
  47a00f:	test   %rax,%rax
  47a012:	je     47a002 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x452>
  47a014:	mov    (%r15),%rdi
  47a017:	cmp    $0x3ff,%rax
  47a01d:	jbe    47a000 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x450>
  47a01f:	shl    $0x7,%rax
  47a023:	mov    %rax,%rsi
  47a026:	or     $0x1,%rsi
  47a02a:	cmp    %r12,%rsi
  47a02d:	ja     47a03c <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x48c>
  47a02f:	cmpb   $0x1,(%rdi,%rax,1)
  47a033:	jne    47a03a <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x48a>
  47a035:	call   *%r13
  47a038:	jmp    47a002 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x452>
  47a03a:	jmp    47a000 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_+0x450>
  47a03c:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
