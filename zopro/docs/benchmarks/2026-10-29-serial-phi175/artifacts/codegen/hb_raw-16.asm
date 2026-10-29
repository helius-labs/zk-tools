
/home/ubuntu/src/zopro-perf-work/serial-phi175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047afb0 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_>:
  47afb0:	push   %rbp
  47afb1:	push   %r15
  47afb3:	push   %r14
  47afb5:	push   %r13
  47afb7:	push   %r12
  47afb9:	push   %rbx
  47afba:	sub    $0x68,%rsp
  47afbe:	mov    (%rdi),%rdx
  47afc1:	mov    0x8(%rdi),%rcx
  47afc5:	mov    0x10(%rdi),%rax
  47afc9:	movq   $0x0,(%rdi)
  47afd0:	test   %rdx,%rdx
  47afd3:	je     47b14b <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x19b>
  47afd9:	mov    0x38(%rdi),%r11
  47afdd:	mov    %rdi,%rbx
  47afe0:	lea    0x40(%rsp),%r10
  47afe5:	mov    %r11,0x60(%rsp)
  47afea:	vmovups 0x18(%rdi),%ymm0
  47afef:	vmovups %ymm0,0x40(%rsp)
  47aff5:	mov    %rdx,0x28(%rsp)
  47affa:	mov    %rcx,0x30(%rsp)
  47afff:	mov    %rax,0x38(%rsp)
  47b004:	mov    (%rdx),%rsi
  47b007:	mov    0x58(%rsp),%r9
  47b00c:	mov    0x8(%rax),%r8
  47b010:	sub    (%rcx),%rsi
  47b013:	mov    (%rax),%rcx
  47b016:	mov    $0x1,%edx
  47b01b:	lea    0x10(%rsp),%rdi
  47b020:	push   %r10
  47b022:	push   %r11
  47b024:	vzeroupper
  47b027:	call   4650c0 <_RINvNvNtNtCsc3XWelWpAp8_5rayon4iter8plumbing24bridge_producer_consumer6helperINtNtB8_5range12IterProducerjEINtNtB6_3map11MapConsumerINtNtNtB6_7collect8consumer15CollectConsumerINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2R_8G2ConfigEENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEB41_>
  47b02c:	add    $0x10,%rsp
  47b030:	mov    0x10(%rsp),%rax
  47b035:	mov    0x18(%rsp),%r15
  47b03a:	mov    0x20(%rsp),%r13
  47b03f:	mov    $0x1,%ebp
  47b044:	cmpl   $0x2,0x40(%rbx)
  47b048:	jb     47b0d7 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x127>
  47b04e:	mov    0x50(%rbx),%r14
  47b052:	mov    %rax,0x8(%rsp)
  47b057:	mov    0x48(%rbx),%r12
  47b05b:	mov    (%r14),%rax
  47b05e:	test   %rax,%rax
  47b061:	je     47b068 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0xb8>
  47b063:	mov    %r12,%rdi
  47b066:	call   *%rax
  47b068:	mov    0x8(%r14),%rsi
  47b06c:	mov    0x8(%rsp),%rax
  47b071:	test   %rsi,%rsi
  47b074:	je     47b0d7 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x127>
  47b076:	mov    0x10(%r14),%rax
  47b07a:	cmp    $0x20000,%rsi
  47b081:	setae  %cl
  47b084:	cmp    $0x1001,%rax
  47b08a:	setb   %dl
  47b08d:	test   %cl,%dl
  47b08f:	je     47b0c7 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x117>
  47b091:	blsr   %eax,%ecx
  47b096:	jne    47b159 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x1a9>
  47b09c:	movabs $0x8000000000000000,%rcx
  47b0a6:	sub    %rax,%rcx
  47b0a9:	cmp    %rcx,%rsi
  47b0ac:	jae    47b159 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x1a9>
  47b0b2:	cmpb   $0x1,(%r12,%rsi,1)
  47b0b7:	jne    47b0c9 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x119>
  47b0b9:	inc    %rsi
  47b0bc:	mov    %r12,%rdi
  47b0bf:	call   *0x6c40b(%rip)        # 4e74d0 <munmap@GLIBC_2.2.5>
  47b0c5:	jmp    47b0d2 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x122>
  47b0c7:	jmp    47b0c9 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x119>
  47b0c9:	mov    %r12,%rdi
  47b0cc:	call   *0x6c406(%rip)        # 4e74d8 <free@GLIBC_2.2.5>
  47b0d2:	mov    0x8(%rsp),%rax
  47b0d7:	mov    %rbp,0x40(%rbx)
  47b0db:	mov    %rax,0x48(%rbx)
  47b0df:	mov    %r15,0x50(%rbx)
  47b0e3:	mov    %r13,0x58(%rbx)
  47b0e7:	mov    0x60(%rbx),%rax
  47b0eb:	movzbl 0x78(%rbx),%ebp
  47b0ef:	mov    (%rax),%rdi
  47b0f2:	cmp    $0x1,%bpl
  47b0f6:	jne    47b10a <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x15a>
  47b0f8:	lock incq (%rdi)
  47b0fc:	jle    47b1c8 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x218>
  47b102:	mov    (%rax),%r14
  47b105:	mov    %r14,%rdi
  47b108:	jmp    47b10a <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x15a>
  47b10a:	mov    $0x3,%eax
  47b10f:	mov    0x70(%rbx),%rsi
  47b113:	xchg   %rax,0x68(%rbx)
  47b117:	cmp    $0x2,%rax
  47b11b:	jne    47b129 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x179>
  47b11d:	add    $0x1d8,%rdi
  47b124:	call   fef40 <_RNvMNtCs8zaw6vLE3tn_10rayon_core5sleepNtB2_5Sleep20wake_specific_thread>
  47b129:	test   %bpl,%bpl
  47b12c:	je     47b13c <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x18c>
  47b12e:	lock decq (%r14)
  47b132:	jne    47b13c <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x18c>
  47b134:	mov    %r14,%rdi
  47b137:	call   ffd00 <_RNvMsn_NtCs6i54tJFfzR_5alloc4syncINtB5_3ArcNtNtCs8zaw6vLE3tn_10rayon_core8registry8RegistryE9drop_slowBJ_>
  47b13c:	add    $0x68,%rsp
  47b140:	pop    %rbx
  47b141:	pop    %r12
  47b143:	pop    %r13
  47b145:	pop    %r14
  47b147:	pop    %r15
  47b149:	pop    %rbp
  47b14a:	ret
  47b14b:	lea    0x6b0f6(%rip),%rdi        # 4e6248 <__frame_dummy_init_array_entry+0x7a58>
  47b152:	call   f1770 <_RNvNtCsgxBkk5gSRhY_4core6option13unwrap_failed>
  47b157:	jmp    47b1c8 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x218>
  47b159:	call   1100f0 <_RNvNtCs9k3SxhrAWiO_3std7process5abort>
  47b15e:	mov    0x8(%r14),%rsi
  47b162:	test   %rsi,%rsi
  47b165:	je     47b173 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x1c3>
  47b167:	mov    0x10(%r14),%rdx
  47b16b:	mov    %r12,%rdi
  47b16e:	call   e88a0 <_RNvCs1njKG4L9aB3_7___rustc14___rust_dealloc>
  47b173:	mov    0x8(%rsp),%rax
  47b178:	mov    %rbp,0x40(%rbx)
  47b17c:	mov    %rax,0x48(%rbx)
  47b180:	mov    %r15,0x50(%rbx)
  47b184:	mov    %r13,0x58(%rbx)
  47b188:	jmp    47b1c3 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x213>
  47b18a:	test   %bpl,%bpl
  47b18d:	je     47b1c3 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x213>
  47b18f:	lock decq (%r14)
  47b193:	jne    47b1c3 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x213>
  47b195:	mov    %r14,%rdi
  47b198:	call   ffd00 <_RNvMsn_NtCs6i54tJFfzR_5alloc4syncINtB5_3ArcNtNtCs8zaw6vLE3tn_10rayon_core8registry8RegistryE9drop_slowBJ_>
  47b19d:	jmp    47b1c3 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x213>
  47b19f:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>
  47b1a4:	mov    %rax,%rdi
  47b1a7:	call   143fda <_RNvNvNtCs9k3SxhrAWiO_3std9panicking12catch_unwind7cleanup>
  47b1ac:	mov    $0x2,%ebp
  47b1b1:	mov    %rdx,%r15
  47b1b4:	cmpl   $0x2,0x40(%rbx)
  47b1b8:	jae    47b04e <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x9e>
  47b1be:	jmp    47b0d7 <_RNvXs2_NtCs8zaw6vLE3tn_10rayon_core3jobINtB5_8StackJobNtNtB7_5latch9SpinLatchNCINvNvNtB7_4join12join_context6call_bINtNtNtNtCsc3XWelWpAp8_5rayon4iter7collect8consumer13CollectResultINtNtCsgHCcee0nhgM_11zopro_bn2545curve10ProjectiveNtB2W_8G2ConfigEENCINvNvNtB1W_8plumbing24bridge_producer_consumer6helperINtNtB1Y_5range12IterProducerjEINtNtB1W_3map11MapConsumerINtB1S_15CollectConsumerB2T_ENCNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine6g2_phi6hb_raws0_0EEs_0E0B1P_ENtB5_3Job7executeB6n_+0x127>
  47b1c3:	call   104a50 <_RNvXNtCs8zaw6vLE3tn_10rayon_core6unwindNtB2_12AbortIfPanicNtNtNtCsgxBkk5gSRhY_4core3ops4drop4Drop4drop>
  47b1c8:	ud2
  47b1ca:	call   f18e3 <_RNvNtCsgxBkk5gSRhY_4core9panicking16panic_in_cleanup>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
