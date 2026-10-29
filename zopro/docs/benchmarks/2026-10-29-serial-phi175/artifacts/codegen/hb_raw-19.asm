
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004659e0 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_>:
  4659e0:	push   %rbp
  4659e1:	push   %r15
  4659e3:	push   %r14
  4659e5:	push   %r13
  4659e7:	push   %r12
  4659e9:	push   %rbx
  4659ea:	sub    $0xe8,%rsp
  4659f1:	mov    0x128(%rsp),%r15
  4659f9:	mov    0x120(%rsp),%rbx
  465a01:	mov    %rsi,0x50(%rsp)
  465a06:	shr    $1,%rsi
  465a09:	mov    %rcx,0x58(%rsp)
  465a0e:	mov    %r9,0x8(%rsp)
  465a13:	mov    %rdi,%r13
  465a16:	mov    %r8,0x60(%rsp)
  465a1b:	cmp    %r8,%rsi
  465a1e:	jae    465aa2 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0xc2>
  465a24:	mov    0x8(%rsp),%r14
  465a29:	mov    0x8(%r15),%rax
  465a2d:	mov    0x10(%r15),%rcx
  465a31:	mov    %r13,0x48(%rsp)
  465a36:	sub    %rbx,%r14
  465a39:	mov    %rcx,(%rsp)
  465a3d:	mov    %rax,0x10(%rsp)
  465a42:	jae    465abc <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0xdc>
  465a44:	mov    (%r15),%r13
  465a47:	lea    0x10(%rax),%rbp
  465a4b:	xor    %r12d,%r12d
  465a4e:	xchg   %ax,%ax
  465a50:	mov    0x8(%rsp),%rax
  465a55:	lea    (%rax,%r12,1),%rdx
  465a59:	lea    0x18(%rsp),%rdi
  465a5e:	mov    %r13,%rsi
  465a61:	call   465da0 <_RNvXs_NtNtNtCsgxBkk5gSRhY_4core3ops8function5implsRNCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00INtB6_5FnMutTjEE8call_mutBX_>
  465a66:	mov    0x18(%rsp),%r15
  465a6b:	cmp    $0xffffffffffffffff,%r15
  465a6f:	je     465abf <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0xdf>
  465a71:	mov    0x20(%rsp),%rbx
  465a76:	cmp    %r12,(%rsp)
  465a7a:	je     465ce1 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x301>
  465a80:	mov    0x28(%rsp),%rax
  465a85:	inc    %r12
  465a88:	mov    %r15,-0x10(%rbp)
  465a8c:	mov    %rbx,-0x8(%rbp)
  465a90:	mov    %rax,0x0(%rbp)
  465a94:	add    $0x18,%rbp
  465a98:	mov    %r14,%rax
  465a9b:	add    %r12,%rax
  465a9e:	jne    465a50 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x70>
  465aa0:	jmp    465abf <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0xdf>
  465aa2:	test   %dl,%dl
  465aa4:	je     465add <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0xfd>
  465aa6:	mov    %fs:0xffffffffffffffc0,%rax
  465aaf:	test   %rax,%rax
  465ab2:	je     465aeb <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x10b>
  465ab4:	add    $0x110,%rax
  465aba:	jmp    465afc <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x11c>
  465abc:	xor    %r12d,%r12d
  465abf:	mov    0x48(%rsp),%rax
  465ac4:	mov    0x10(%rsp),%rcx
  465ac9:	mov    (%rsp),%rdx
  465acd:	mov    %rcx,(%rax)
  465ad0:	mov    %rdx,0x8(%rax)
  465ad4:	mov    %r12,0x10(%rax)
  465ad8:	jmp    465ccf <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x2ef>
  465add:	test   %rcx,%rcx
  465ae0:	je     465a24 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x44>
  465ae6:	shr    $1,%rcx
  465ae9:	jmp    465b10 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x130>
  465aeb:	mov    %rsi,%r14
  465aee:	mov    %rcx,%r12
  465af1:	call   1041b0 <_RNvNtCs8zaw6vLE3tn_10rayon_core8registry15global_registry>
  465af6:	mov    %r12,%rcx
  465af9:	mov    %r14,%rsi
  465afc:	mov    (%rax),%rax
  465aff:	shr    $1,%rcx
  465b02:	mov    0x208(%rax),%rax
  465b09:	cmp    %rax,%rcx
  465b0c:	cmovbe %rax,%rcx
  465b10:	mov    %rcx,0x58(%rsp)
  465b15:	xor    %eax,%eax
  465b17:	mov    %rbx,%rcx
  465b1a:	sub    0x8(%rsp),%rcx
  465b1f:	mov    %rsi,0x68(%rsp)
  465b24:	cmovae %rcx,%rax
  465b28:	cmp    %rax,%rsi
  465b2b:	ja     465cfb <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x31b>
  465b31:	mov    0x10(%r15),%rax
  465b35:	sub    %rsi,%rax
  465b38:	jb     465d13 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x333>
  465b3e:	mov    0x8(%r15),%rdi
  465b42:	mov    0x8(%rsp),%r11
  465b47:	mov    (%r15),%rdx
  465b4a:	lea    (%rsi,%rsi,2),%r8
  465b4e:	lea    0x50(%rsp),%r9
  465b53:	lea    0x58(%rsp),%r10
  465b58:	mov    %r9,0x70(%rsp)
  465b5d:	lea    0x68(%rsp),%r9
  465b62:	mov    %r9,0x78(%rsp)
  465b67:	mov    %r10,0x80(%rsp)
  465b6f:	lea    (%rdi,%r8,8),%r8
  465b73:	lea    (%rsi,%r11,1),%rcx
  465b77:	mov    %rdx,0x88(%rsp)
  465b7f:	mov    %r8,0x90(%rsp)
  465b87:	mov    %rax,0x98(%rsp)
  465b8f:	mov    %rcx,0xa0(%rsp)
  465b97:	mov    %rbx,0xa8(%rsp)
  465b9f:	mov    %r9,0xb0(%rsp)
  465ba7:	mov    %r10,0xb8(%rsp)
  465baf:	mov    %rdx,0xc0(%rsp)
  465bb7:	mov    %fs:0xffffffffffffffc0,%rdx
  465bc0:	mov    %rdi,0xc8(%rsp)
  465bc8:	mov    %rsi,0xd0(%rsp)
  465bd0:	mov    %r11,0xd8(%rsp)
  465bd8:	mov    %rcx,0xe0(%rsp)
  465be0:	test   %rdx,%rdx
  465be3:	jne    465c0c <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x22c>
  465be5:	call   1041b0 <_RNvNtCs8zaw6vLE3tn_10rayon_core8registry15global_registry>
  465bea:	mov    (%rax),%rsi
  465bed:	mov    %fs:0xffffffffffffffc0,%rdx
  465bf6:	test   %rdx,%rdx
  465bf9:	je     465d2b <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x34b>
  465bff:	cmp    %rsi,0x110(%rdx)
  465c06:	jne    465d43 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x363>
  465c0c:	lea    0x18(%rsp),%rdi
  465c11:	lea    0x70(%rsp),%rsi
  465c16:	xor    %ecx,%ecx
  465c18:	call   479bb0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5range12IterProducerjEINtNtBY_3map11MapConsumerINtNtNtBY_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB4g_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCBR_s_0INtB32_13CollectResultB3H_EB6t_E0B5o_>
  465c1d:	mov    0x28(%rsp),%rcx
  465c22:	vmovdqu 0x20(%rsp),%xmm0
  465c28:	mov    0x18(%rsp),%rax
  465c2d:	mov    0x30(%rsp),%rbx
  465c32:	lea    (%rcx,%rcx,2),%rcx
  465c36:	lea    (%rax,%rcx,8),%rcx
  465c3a:	cmp    %rbx,%rcx
  465c3d:	je     465cbf <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x2df>
  465c43:	mov    0x40(%rsp),%r14
  465c48:	mov    %rax,0x0(%r13)
  465c4c:	vmovdqu %xmm0,0x8(%r13)
  465c52:	test   %r14,%r14
  465c55:	je     465ccf <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x2ef>
  465c57:	mov    0x81872(%rip),%r12        # 4e74d0 <munmap@GLIBC_2.2.5>
  465c5e:	mov    0x81873(%rip),%r13        # 4e74d8 <free@GLIBC_2.2.5>
  465c65:	add    $0x8,%rbx
  465c69:	movabs $0x7ffffffffffffff8,%r15
  465c73:	jmp    465c8c <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x2ac>
  465c75:	data16 cs nopw 0x0(%rax,%rax,1)
  465c80:	call   *%r13
  465c83:	add    $0x18,%rbx
  465c87:	dec    %r14
  465c8a:	je     465ccf <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x2ef>
  465c8c:	mov    -0x8(%rbx),%rax
  465c90:	test   %rax,%rax
  465c93:	je     465c83 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x2a3>
  465c95:	mov    (%rbx),%rdi
  465c98:	cmp    $0x3ff,%rax
  465c9e:	jbe    465c80 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x2a0>
  465ca0:	shl    $0x7,%rax
  465ca4:	mov    %rax,%rsi
  465ca7:	or     $0x1,%rsi
  465cab:	cmp    %r15,%rsi
  465cae:	ja     465d5b <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x37b>
  465cb4:	cmpb   $0x1,(%rdi,%rax,1)
  465cb8:	jne    465c80 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x2a0>
  465cba:	call   *%r12
  465cbd:	jmp    465c83 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x2a3>
  465cbf:	vpaddq 0x38(%rsp),%xmm0,%xmm0
  465cc5:	mov    %rax,0x0(%r13)
  465cc9:	vmovdqu %xmm0,0x8(%r13)
  465ccf:	add    $0xe8,%rsp
  465cd6:	pop    %rbx
  465cd7:	pop    %r12
  465cd9:	pop    %r13
  465cdb:	pop    %r14
  465cdd:	pop    %r15
  465cdf:	pop    %rbp
  465ce0:	ret
  465ce1:	lea    -0x3db8ee(%rip),%rdi        # 8a3fa <_RNvNtNtNtCsgxBkk5gSRhY_4core7unicode12unicode_data11white_space14WHITESPACE_MAP+0x1f2>
  465ce8:	lea    0x805d1(%rip),%rdx        # 4e62c0 <__frame_dummy_init_array_entry+0x7ad0>
  465cef:	mov    $0x45,%esi
  465cf4:	call   ee380 <_RNvNtCsgxBkk5gSRhY_4core9panicking9panic_fmt>
  465cf9:	ud2
  465cfb:	lea    -0x44d1dd(%rip),%rdi        # 18b25 <_RNvNvNtNtCseM6JStPZ3Y4_11miniz_oxide7inflate4core12reverse_bits20REVERSED_BITS_LOOKUP+0x611>
  465d02:	lea    0x79a1f(%rip),%rdx        # 4df728 <__frame_dummy_init_array_entry+0xf38>
  465d09:	mov    $0x2b,%esi
  465d0e:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  465d13:	lea    -0x3db93e(%rip),%rdi        # 8a3dc <_RNvNtNtNtCsgxBkk5gSRhY_4core7unicode12unicode_data11white_space14WHITESPACE_MAP+0x1d4>
  465d1a:	lea    0x8056f(%rip),%rdx        # 4e6290 <__frame_dummy_init_array_entry+0x7aa0>
  465d21:	mov    $0x1e,%esi
  465d26:	call   f0450 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
  465d2b:	sub    $0xffffffffffffff80,%rsi
  465d2f:	lea    0x18(%rsp),%rdi
  465d34:	lea    0x70(%rsp),%rdx
  465d39:	call   479ab0 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5range12IterProducerjEINtNtB1N_3map11MapConsumerINtNtNtB1N_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB57_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1G_s_0INtB3S_13CollectResultB4y_EB7l_E0TB7l_B7l_EEB6f_>
  465d3e:	jmp    465c1d <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x23d>
  465d43:	sub    $0xffffffffffffff80,%rsi
  465d47:	lea    0x18(%rsp),%rdi
  465d4c:	lea    0x70(%rsp),%rcx
  465d51:	call   47a050 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5range12IterProducerjEINtNtB1O_3map11MapConsumerINtNtNtB1O_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB58_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EE0NCB1H_s_0INtB3T_13CollectResultB4z_EB7m_E0TB7m_B7m_EEB6g_>
  465d56:	jmp    465c1d <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x23d>
  465d5b:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  465d60:	mov    %rax,%r14
  465d63:	jmp    465d85 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x3a5>
  465d65:	mov    %rax,%r14
  465d68:	test   %r15,%r15
  465d6b:	je     465d81 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB3n_8G2ConfigEEENCNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_00EEB4v_+0x3a1>
  465d6d:	shl    $0x7,%r15
  465d71:	mov    $0x8,%edx
  465d76:	mov    %rbx,%rdi
  465d79:	mov    %r15,%rsi
  465d7c:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  465d81:	mov    (%rsp),%r12
  465d85:	mov    0x10(%rsp),%rdi
  465d8a:	mov    %r12,%rsi
  465d8d:	call   293590 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCs6i54tJFfzR_5alloc3vec3VecINtNtCsgHCcee0nhgM_11zopro_bn2545curve6AffineNtB2e_8G2ConfigEEEECsHW5vo2gROj_10zopro_core>
  465d92:	mov    %r14,%rdi
  465d95:	call   4dd7a0 <_Unwind_Resume@plt>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
