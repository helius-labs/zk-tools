
/home/ubuntu/src/zopro-perf-work/crt-euler-native176/bench-fresh:     file format elf64-x86-64


Disassembly of section .text:

0000000000041a50 <_RNvMNtNtNtCs5rHCImu4O59_5bench4base6direct6nativeNtB4_10Projection8project8>:
   41a50:	vmovdqu64 (%rdx),%zmm0
   41a56:	vmovdqu64 0x40(%rdx),%zmm1
   41a5d:	vmovdqu64 0x80(%rdx),%zmm2
   41a64:	vmovdqu64 0xc0(%rdx),%zmm3
   41a6b:	vmovdqa64 -0x36735(%rip),%zmm6        # b340 <GCC_except_table985+0x3cc>
   41a75:	vpbroadcastq -0x3436f(%rip),%zmm9        # d710 <GCC_except_table985+0x279c>
   41a7f:	vpbroadcastq 0x68(%rsi),%ymm11
   41a85:	vmovdqa64 -0x3670f(%rip),%zmm13        # b380 <GCC_except_table985+0x40c>
   41a8f:	mov    0x88(%rsi),%eax
   41a95:	vpunpckhqdq %zmm1,%zmm0,%zmm4
   41a9b:	vpunpcklqdq %zmm1,%zmm0,%zmm0
   41aa1:	vpunpckhqdq %zmm3,%zmm2,%zmm5
   41aa7:	vpunpcklqdq %zmm3,%zmm2,%zmm1
   41aad:	vpbroadcastq %xmm11,%zmm11
   41ab3:	vmovdqa64 %zmm4,%zmm7
   41ab9:	vpermt2q %zmm5,%zmm6,%zmm7
   41abf:	vpermi2q %zmm1,%zmm0,%zmm6
   41ac5:	vpermt2q %zmm5,%zmm13,%zmm4
   41acb:	vpermt2q %zmm1,%zmm13,%zmm0
   41ad1:	vpsrlq $0x20,%zmm6,%zmm2
   41ad8:	vpandd %zmm9,%zmm6,%zmm6
   41ade:	vpmullq 0x60(%rsi){1to8},%zmm6,%zmm12
   41ae5:	vpsrlq $0x20,%zmm7,%zmm8
   41aec:	vpandd %zmm9,%zmm7,%zmm7
   41af2:	vpmullq 0x70(%rsi){1to8},%zmm7,%zmm10
   41af9:	vpsrlq $0x20,%zmm4,%zmm5
   41b00:	vpsrlq $0x20,%zmm0,%zmm1
   41b07:	vpandd %zmm9,%zmm4,%zmm4
   41b0d:	vpandd %zmm9,%zmm0,%zmm0
   41b13:	vpmullq 0x40(%rsi){1to8},%zmm0,%zmm9
   41b1a:	vpmullq (%rsi){1to8},%zmm0,%zmm0
   41b20:	vpmullq 0x20(%rsi){1to8},%zmm6,%zmm6
   41b27:	vpmullq 0x30(%rsi){1to8},%zmm7,%zmm7
   41b2e:	vpmullq %zmm2,%zmm11,%zmm3
   41b34:	vpmullq 0x28(%rsi){1to8},%zmm2,%zmm2
   41b3b:	vpsrlq $0x20,%zmm3,%zmm3
   41b42:	vpsrlq $0x20,%zmm12,%zmm11
   41b49:	vpsrlq $0x20,%zmm10,%zmm10
   41b50:	vpmullq 0x50(%rsi){1to8},%zmm4,%zmm12
   41b57:	vpmullq 0x10(%rsi){1to8},%zmm4,%zmm4
   41b5e:	vpsrlq $0x20,%zmm9,%zmm9
   41b65:	vpaddq %zmm11,%zmm3,%zmm3
   41b6b:	vpmullq 0x48(%rsi){1to8},%zmm1,%zmm11
   41b72:	vpmullq 0x8(%rsi){1to8},%zmm1,%zmm1
   41b79:	vpaddq %zmm6,%zmm2,%zmm2
   41b7f:	vpaddq %zmm3,%zmm10,%zmm3
   41b85:	vpbroadcastq 0x58(%rsi),%xmm10
   41b8b:	vpaddq %zmm2,%zmm7,%zmm2
   41b91:	vpbroadcastq %xmm10,%zmm10
   41b97:	vpsrlq $0x20,%zmm12,%zmm12
   41b9e:	vpmullq %zmm5,%zmm10,%zmm10
   41ba4:	vpmullq 0x18(%rsi){1to8},%zmm5,%zmm5
   41bab:	vpaddq %zmm0,%zmm1,%zmm0
   41bb1:	vpbroadcastq 0x78(%rsi),%xmm1
   41bb7:	vpsrlq $0x20,%zmm11,%zmm11
   41bbe:	vpaddq %zmm9,%zmm11,%zmm9
   41bc4:	vpsrlq $0x20,%zmm10,%zmm10
   41bcb:	vpaddq %zmm12,%zmm10,%zmm10
   41bd1:	vpaddq %zmm9,%zmm10,%zmm9
   41bd7:	vpaddq %zmm9,%zmm3,%zmm3
   41bdd:	vpbroadcastq %xmm1,%zmm1
   41be3:	vpmullq %zmm8,%zmm1,%zmm1
   41be9:	vpaddq %zmm4,%zmm5,%zmm4
   41bef:	vpaddq %zmm0,%zmm4,%zmm0
   41bf5:	vpmullq 0x38(%rsi){1to8},%zmm8,%zmm4
   41bfc:	vpsrlq $0x20,%zmm1,%zmm1
   41c03:	vpaddq %zmm0,%zmm2,%zmm0
   41c09:	vpaddq %zmm3,%zmm1,%zmm1
   41c0f:	vpbroadcastq %rax,%zmm3
   41c15:	vpmullq %zmm1,%zmm3,%zmm1
   41c1b:	vpaddq %zmm0,%zmm4,%zmm0
   41c21:	vpsubq %zmm1,%zmm0,%zmm0
   41c27:	vpmullq 0x80(%rsi){1to8},%zmm0,%zmm1
   41c2e:	vpsrlq $0x30,%zmm1,%zmm1
   41c35:	vpmuludq %zmm3,%zmm1,%zmm1
   41c3b:	vpsubq %zmm1,%zmm0,%zmm0
   41c41:	vpsubq %zmm3,%zmm0,%zmm1
   41c47:	vpminuq %zmm1,%zmm0,%zmm0
   41c4d:	vpmovqd %zmm0,(%rdi)
   41c53:	vzeroupper
   41c56:	ret
