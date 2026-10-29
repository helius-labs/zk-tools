
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000464c40 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_>:
  464c40:	push   %r15
  464c42:	push   %r14
  464c44:	push   %r12
  464c46:	push   %rbx
  464c47:	sub    $0x98,%rsp
  464c4e:	mov    %rdi,0x8(%rsp)
  464c53:	shr    $1,%rdi
  464c56:	mov    %r9,%r14
  464c59:	mov    %r8,%rbx
  464c5c:	mov    %rcx,0x18(%rsp)
  464c61:	cmp    %rcx,%rdi
  464c64:	jae    464ca7 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x67>
  464c66:	test   %r14,%r14
  464c69:	je     464d95 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x155>
  464c6f:	shl    $0x5,%r14
  464c73:	lea    0x78(%rsp),%r15
  464c78:	nopl   0x0(%rax,%rax,1)
  464c80:	mov    %r15,%rdi
  464c83:	mov    %rbx,%rsi
  464c86:	vzeroupper
  464c89:	call   464de0 <_RNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi9phi_split>
  464c8e:	vmovups 0x78(%rsp),%ymm0
  464c94:	vmovups %ymm0,(%rbx)
  464c98:	add    $0x20,%rbx
  464c9c:	add    $0xffffffffffffffe0,%r14
  464ca0:	jne    464c80 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x40>
  464ca2:	jmp    464d95 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x155>
  464ca7:	test   %sil,%sil
  464caa:	je     464cc2 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x82>
  464cac:	mov    %fs:0xffffffffffffffc0,%rax
  464cb5:	test   %rax,%rax
  464cb8:	je     464ccc <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x8c>
  464cba:	add    $0x110,%rax
  464cc0:	jmp    464cdd <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x9d>
  464cc2:	test   %rdx,%rdx
  464cc5:	je     464c66 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x26>
  464cc7:	shr    $1,%rdx
  464cca:	jmp    464cf1 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0xb1>
  464ccc:	mov    %rdi,%r15
  464ccf:	mov    %rdx,%r12
  464cd2:	call   1041b0 <_RNvNtCs8zaw6vLE3tn_10rayon_core8registry15global_registry>
  464cd7:	mov    %r12,%rdx
  464cda:	mov    %r15,%rdi
  464cdd:	mov    (%rax),%rax
  464ce0:	shr    $1,%rdx
  464ce3:	mov    0x208(%rax),%rax
  464cea:	cmp    %rax,%rdx
  464ced:	cmovbe %rax,%rdx
  464cf1:	sub    %rdi,%r14
  464cf4:	mov    %rdx,0x10(%rsp)
  464cf9:	mov    %rdi,0x78(%rsp)
  464cfe:	jb     464da7 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x167>
  464d04:	mov    0xc0(%rsp),%rax
  464d0c:	mov    %rdi,%rcx
  464d0f:	shl    $0x5,%rcx
  464d13:	lea    0x8(%rsp),%rdx
  464d18:	lea    0x10(%rsp),%rsi
  464d1d:	mov    %rdx,0x20(%rsp)
  464d22:	lea    0x78(%rsp),%rdx
  464d27:	add    %rbx,%rcx
  464d2a:	mov    %rdx,0x28(%rsp)
  464d2f:	mov    %rsi,0x30(%rsp)
  464d34:	mov    %rcx,0x38(%rsp)
  464d39:	mov    %r14,0x40(%rsp)
  464d3e:	mov    %rax,0x48(%rsp)
  464d43:	mov    %rdx,0x50(%rsp)
  464d48:	mov    %rsi,0x58(%rsp)
  464d4d:	mov    %fs:0xffffffffffffffc0,%rsi
  464d56:	mov    %rbx,0x60(%rsp)
  464d5b:	mov    %rdi,0x68(%rsp)
  464d60:	mov    %rax,0x70(%rsp)
  464d65:	test   %rsi,%rsi
  464d68:	jne    464d89 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x149>
  464d6a:	call   1041b0 <_RNvNtCs8zaw6vLE3tn_10rayon_core8registry15global_registry>
  464d6f:	mov    (%rax),%rdi
  464d72:	mov    %fs:0xffffffffffffffc0,%rsi
  464d7b:	test   %rsi,%rsi
  464d7e:	je     464dbf <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x17f>
  464d80:	cmp    %rdi,0x110(%rsi)
  464d87:	jne    464dcf <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x18f>
  464d89:	lea    0x20(%rsp),%rdi
  464d8e:	xor    %edx,%edx
  464d90:	call   47b4c0 <_RNCINvNtCs8zaw6vLE3tn_10rayon_core4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB10_5slice15IterMutProducerAyj4_EINtNtBY_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCBR_s_0uuE0B3n_>
  464d95:	add    $0x98,%rsp
  464d9c:	pop    %rbx
  464d9d:	pop    %r12
  464d9f:	pop    %r14
  464da1:	pop    %r15
  464da3:	vzeroupper
  464da6:	ret
  464da7:	lea    -0x44543a(%rip),%rdi        # 1f974 <_RNvCs2bkUAOhDEmu_4zmij11STATIC_DATA+0x5574>
  464dae:	lea    0x815a3(%rip),%rdx        # 4e6358 <__frame_dummy_init_array_entry+0x7b68>
  464db5:	mov    $0x13,%esi
  464dba:	call   ee380 <_RNvNtCsgxBkk5gSRhY_4core9panicking9panic_fmt>
  464dbf:	sub    $0xffffffffffffff80,%rdi
  464dc3:	lea    0x20(%rsp),%rsi
  464dc8:	call   47b3f0 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry14in_worker_coldNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1P_5slice15IterMutProducerAyj4_EINtNtB1N_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1G_s_0uuE0TuuEEB4d_>
  464dcd:	jmp    464d95 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x155>
  464dcf:	sub    $0xffffffffffffff80,%rdi
  464dd3:	lea    0x20(%rsp),%rdx
  464dd8:	call   47b8b0 <_RINvMs4_NtCs8zaw6vLE3tn_10rayon_core8registryNtB6_8Registry15in_worker_crossNCINvNtB8_4join12join_contextNCINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB1Q_5slice15IterMutProducerAyj4_EINtNtB1O_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EE0NCB1H_s_0uuE0TuuEEB4e_>
  464ddd:	jmp    464d95 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5slice15IterMutProducerAyj4_EINtNtB6_8for_each15ForEachConsumerNCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws_0EEB2u_+0x155>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
