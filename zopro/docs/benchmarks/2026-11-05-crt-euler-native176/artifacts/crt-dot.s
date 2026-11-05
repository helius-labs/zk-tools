
/home/ubuntu/src/zopro-perf-work/crt-euler-native176/bench-fresh:     file format elf64-x86-64


Disassembly of section .text:

000000000004b920 <_RNvMs_NtNtNtCs5rHCImu4O59_5bench4base6direct6nativeNtB4_9NativeCrt8recover8>:
   4b920:	push   %r15
   4b922:	push   %r14
   4b924:	push   %rbx
   4b925:	vpbroadcastq -0x3e147(%rip),%zmm5        # d7e8 <GCC_except_table985+0x2874>
   4b92f:	mov    0x8(%rsi),%r10
   4b933:	mov    0x10(%rsi),%rax
   4b937:	mov    0x28(%rsi),%rcx
   4b93b:	mov    0x40(%rsi),%r8
   4b93f:	mov    0x20(%rsi),%r11
   4b943:	mov    0x38(%rsi),%rbx
   4b947:	mov    $0xffffffffffffffa6,%r14
   4b94e:	vpxor  %xmm0,%xmm0,%xmm0
   4b952:	xor    %r9d,%r9d
   4b955:	vpxor  %xmm2,%xmm2,%xmm2
   4b959:	vpxor  %xmm3,%xmm3,%xmm3
   4b95d:	vpxor  %xmm4,%xmm4,%xmm4
   4b961:	vpxor  %xmm6,%xmm6,%xmm6
   4b965:	vpxor  %xmm1,%xmm1,%xmm1
   4b969:	vpxor  %xmm7,%xmm7,%xmm7
   4b96d:	jmp    4ba0f <_RNvMs_NtNtNtCs5rHCImu4O59_5bench4base6direct6nativeNtB4_9NativeCrt8recover8+0xef>
   4b972:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   4b980:	vpaddq %zmm5,%zmm7,%zmm8
   4b986:	vpsrlq $0x20,%zmm8,%zmm8
   4b98d:	vpbroadcastq 0x370(%rsi,%r14,8),%zmm9
   4b995:	vpbroadcastq 0x378(%rsi,%r14,8),%zmm11
   4b99d:	vpbroadcastq 0x380(%rsi,%r14,8),%zmm10
   4b9a5:	add    $0x20,%rdx
   4b9a9:	vpmadd52huq %zmm9,%zmm8,%zmm4
   4b9af:	vpmadd52huq %zmm11,%zmm8,%zmm3
   4b9b5:	vpmadd52huq %zmm10,%zmm8,%zmm2
   4b9bb:	vpmadd52luq %zmm9,%zmm8,%zmm6
   4b9c1:	vpmadd52luq %zmm11,%zmm8,%zmm4
   4b9c7:	vpbroadcastq 0x388(%rsi,%r14,8),%zmm11
   4b9cf:	vpmadd52luq %zmm10,%zmm8,%zmm3
   4b9d5:	vpmullq 0x370(%rsi,%r9,8){1to8},%zmm8,%zmm10
   4b9dd:	inc    %r9
   4b9e0:	vpmadd52huq %zmm11,%zmm8,%zmm0
   4b9e6:	vpmadd52luq 0x390(%rsi,%r14,8){1to8},%zmm8,%zmm0
   4b9ee:	vpmadd52luq %zmm11,%zmm8,%zmm2
   4b9f4:	add    $0x5,%r14
   4b9f8:	vpsrlq $0x1f,%zmm10,%zmm8
   4b9ff:	vpaddq %zmm1,%zmm8,%zmm1
   4ba05:	cmp    $0x12,%r9
   4ba09:	je     4ba97 <_RNvMs_NtNtNtCs5rHCImu4O59_5bench4base6direct6nativeNtB4_9NativeCrt8recover8+0x177>
   4ba0f:	cmp    $0x11,%r9
   4ba13:	je     4b980 <_RNvMs_NtNtNtCs5rHCImu4O59_5bench4base6direct6nativeNtB4_9NativeCrt8recover8+0x60>
   4ba19:	cmp    %rax,%r9
   4ba1c:	jae    4be2a <_RNvMs_NtNtNtCs5rHCImu4O59_5bench4base6direct6nativeNtB4_9NativeCrt8recover8+0x50a>
   4ba22:	cmp    %rcx,%r9
   4ba25:	jae    4be3f <_RNvMs_NtNtNtCs5rHCImu4O59_5bench4base6direct6nativeNtB4_9NativeCrt8recover8+0x51f>
   4ba2b:	cmp    %r8,%r9
   4ba2e:	jae    4be15 <_RNvMs_NtNtNtCs5rHCImu4O59_5bench4base6direct6nativeNtB4_9NativeCrt8recover8+0x4f5>
   4ba34:	vpmovzxdq (%rdx),%zmm8
   4ba3a:	mov    (%r11,%r9,4),%r15d
   4ba3e:	vpbroadcastq %r15,%zmm9
   4ba44:	mov    (%r10,%r9,4),%r15d
   4ba48:	vpbroadcastq %r15,%zmm10
   4ba4e:	vpmuludq %zmm8,%zmm9,%zmm9
   4ba54:	vpmullq 0x400(%rsi,%r9,8){1to8},%zmm8,%zmm8
   4ba5f:	vpsrlq $0x20,%zmm8,%zmm8
   4ba66:	vpmuludq %zmm10,%zmm8,%zmm8
   4ba6c:	vpsubq %zmm8,%zmm9,%zmm8
   4ba72:	vpsubq %zmm10,%zmm8,%zmm9
   4ba78:	vpminuq %zmm9,%zmm8,%zmm8
   4ba7e:	vpmullq (%rbx,%r9,8){1to8},%zmm8,%zmm9
   4ba85:	vpsrlq $0x1f,%zmm9,%zmm9
   4ba8c:	vpaddq %zmm7,%zmm9,%zmm7
   4ba92:	jmp    4b98d <_RNvMs_NtNtNtCs5rHCImu4O59_5bench4base6direct6nativeNtB4_9NativeCrt8recover8+0x6d>
   4ba97:	vpbroadcastq 0x488(%rsi),%zmm5
   4baa1:	vpbroadcastq 0x490(%rsi),%zmm18
   4baab:	vpbroadcastq 0x498(%rsi),%zmm17
   4bab5:	vpxor  %xmm9,%xmm9,%xmm9
   4baba:	vpxor  %xmm10,%xmm10,%xmm10
   4babf:	vpbroadcastq 0x4a0(%rsi),%zmm16
   4bac9:	vpxor  %xmm11,%xmm11,%xmm11
   4bace:	vpbroadcastq 0x4a8(%rsi),%zmm15
   4bad8:	vpxor  %xmm12,%xmm12,%xmm12
   4badd:	vpxor  %xmm8,%xmm8,%xmm8
   4bae2:	vpxor  %xmm13,%xmm13,%xmm13
   4bae7:	vpxor  %xmm14,%xmm14,%xmm14
   4baec:	vpxor  %xmm7,%xmm7,%xmm7
   4baf0:	vpmadd52huq %zmm5,%zmm1,%zmm9
   4baf6:	vpmadd52luq %zmm18,%zmm1,%zmm10
   4bafc:	vpmadd52luq %zmm17,%zmm1,%zmm11
   4bb02:	vpmadd52luq %zmm16,%zmm1,%zmm12
   4bb08:	vpmadd52luq %zmm5,%zmm1,%zmm8
   4bb0e:	vpmadd52luq %zmm15,%zmm1,%zmm13
   4bb14:	vpaddq %zmm10,%zmm9,%zmm9
   4bb1a:	vpxor  %xmm10,%xmm10,%xmm10
   4bb1f:	vpmadd52huq %zmm18,%zmm1,%zmm10
   4bb25:	vpsubq %zmm8,%zmm6,%zmm8
   4bb2b:	vpsubq %zmm9,%zmm4,%zmm9
   4bb31:	vpaddq %zmm11,%zmm10,%zmm10
   4bb37:	vpxor  %xmm11,%xmm11,%xmm11
   4bb3c:	vpmadd52huq %zmm17,%zmm1,%zmm11
   4bb42:	vpsubq %zmm10,%zmm3,%zmm10
   4bb48:	vpaddq %zmm12,%zmm11,%zmm11
   4bb4e:	vpxor  %xmm12,%xmm12,%xmm12
   4bb53:	vpmadd52huq %zmm16,%zmm1,%zmm12
   4bb59:	vpsubq %zmm11,%zmm2,%zmm11
   4bb5f:	vpaddq %zmm13,%zmm12,%zmm1
   4bb65:	vpsraq $0x34,%zmm8,%zmm12
   4bb6c:	vpxor  %xmm13,%xmm13,%xmm13
   4bb71:	vpaddq %zmm12,%zmm9,%zmm9
   4bb77:	vpsubq %zmm1,%zmm0,%zmm1
   4bb7d:	vpbroadcastq -0x3e4c7(%rip),%zmm0        # d6c0 <GCC_except_table985+0x274c>
   4bb87:	vpsraq $0x34,%zmm9,%zmm12
   4bb8e:	vpaddq %zmm12,%zmm10,%zmm10
   4bb94:	vpsraq $0x34,%zmm10,%zmm12
   4bb9b:	vpaddq %zmm12,%zmm11,%zmm11
   4bba1:	vpsraq $0x34,%zmm11,%zmm12
   4bba8:	vpaddq %zmm12,%zmm1,%zmm1
   4bbae:	vpxor  %xmm12,%xmm12,%xmm12
   4bbb3:	vpmadd52huq 0x4b0(%rsi){1to8},%zmm1,%zmm12
   4bbbd:	vpandq %zmm0,%zmm8,%zmm8
   4bbc3:	vpandq %zmm0,%zmm9,%zmm9
   4bbc9:	vpandq %zmm0,%zmm10,%zmm10
   4bbcf:	vpandq %zmm0,%zmm11,%zmm11
   4bbd5:	vpmadd52luq %zmm5,%zmm12,%zmm13
   4bbdb:	vpmadd52luq %zmm18,%zmm12,%zmm14
   4bbe1:	vpmadd52luq %zmm15,%zmm12,%zmm7
   4bbe7:	vpsubq %zmm13,%zmm8,%zmm8
   4bbed:	vpxor  %xmm13,%xmm13,%xmm13
   4bbf2:	vpmadd52huq %zmm5,%zmm12,%zmm13
   4bbf8:	vpaddq %zmm14,%zmm13,%zmm13
   4bbfe:	vpxor  %xmm14,%xmm14,%xmm14
   4bc03:	vpmadd52luq %zmm17,%zmm12,%zmm14
   4bc09:	vpsubq %zmm13,%zmm9,%zmm9
   4bc0f:	vpxor  %xmm13,%xmm13,%xmm13
   4bc14:	vpmadd52huq %zmm18,%zmm12,%zmm13
   4bc1a:	vpaddq %zmm14,%zmm13,%zmm13
   4bc20:	vpxor  %xmm14,%xmm14,%xmm14
   4bc25:	vpmadd52luq %zmm16,%zmm12,%zmm14
   4bc2b:	vpsubq %zmm13,%zmm10,%zmm10
   4bc31:	vpxor  %xmm13,%xmm13,%xmm13
   4bc36:	vpmadd52huq %zmm17,%zmm12,%zmm13
   4bc3c:	vpaddq %zmm14,%zmm13,%zmm13
   4bc42:	vpsubq %zmm13,%zmm11,%zmm11
   4bc48:	vpxor  %xmm13,%xmm13,%xmm13
   4bc4d:	vpmadd52huq %zmm16,%zmm12,%zmm13
   4bc53:	vpaddq %zmm7,%zmm13,%zmm7
   4bc59:	vpsubq %zmm7,%zmm1,%zmm1
   4bc5f:	vpsraq $0x34,%zmm8,%zmm7
   4bc66:	vpandq %zmm0,%zmm8,%zmm8
   4bc6c:	vpaddq %zmm7,%zmm9,%zmm7
   4bc72:	vpsubq %zmm5,%zmm8,%zmm5
   4bc78:	vpsraq $0x34,%zmm7,%zmm9
   4bc7f:	vpandq %zmm0,%zmm7,%zmm7
   4bc85:	vpsraq $0x34,%zmm5,%zmm12
   4bc8c:	vpaddq %zmm9,%zmm10,%zmm9
   4bc92:	vpsubq %zmm18,%zmm7,%zmm6
   4bc98:	vpsraq $0x34,%zmm9,%zmm10
   4bc9f:	vpaddq %zmm12,%zmm6,%zmm6
   4bca5:	vpandq %zmm0,%zmm9,%zmm9
   4bcab:	vpaddq %zmm10,%zmm11,%zmm10
   4bcb1:	vpsraq $0x34,%zmm6,%zmm12
   4bcb8:	vpsubq %zmm17,%zmm9,%zmm4
   4bcbe:	vpsraq $0x34,%zmm10,%zmm11
   4bcc5:	vpaddq %zmm12,%zmm4,%zmm4
   4bccb:	vpandq %zmm0,%zmm10,%zmm10
   4bcd1:	vpaddq %zmm11,%zmm1,%zmm1
   4bcd7:	vpsraq $0x34,%zmm4,%zmm12
   4bcde:	vpsubq %zmm16,%zmm10,%zmm3
   4bce4:	vpaddq %zmm12,%zmm3,%zmm3
   4bcea:	vpandq %zmm0,%zmm1,%zmm11
   4bcf0:	vpsraq $0x34,%zmm3,%zmm12
   4bcf7:	vpsubq %zmm15,%zmm11,%zmm2
   4bcfd:	vpaddq %zmm12,%zmm2,%zmm2
   4bd03:	vpcmpltuq -0x3e5d6(%rip){1to8},%zmm2,%k1        # d738 <GCC_except_table985+0x27c4>
   4bd0e:	vpandq %zmm0,%zmm6,%zmm7{%k1}
   4bd14:	vpandq %zmm0,%zmm4,%zmm9{%k1}
   4bd1a:	vpandq %zmm0,%zmm3,%zmm10{%k1}
   4bd20:	vmovdqa64 %zmm2,%zmm1{%k1}
   4bd26:	vpandq %zmm0,%zmm5,%zmm8{%k1}
   4bd2c:	vpsllq $0x10,%zmm1,%zmm1
   4bd33:	vpsllq $0x34,%zmm7,%zmm6
   4bd3a:	vpsrlq $0x24,%zmm10,%zmm2
   4bd41:	vpsllq $0x28,%zmm9,%zmm4
   4bd48:	vpsrlq $0xc,%zmm7,%zmm11
   4bd4f:	vpsllq $0x1c,%zmm10,%zmm0
   4bd56:	vpsrlq $0x18,%zmm9,%zmm3
   4bd5d:	vporq  %zmm8,%zmm6,%zmm5
   4bd63:	vporq  %zmm11,%zmm4,%zmm4
   4bd69:	vporq  %zmm3,%zmm0,%zmm0
   4bd6f:	vporq  %zmm2,%zmm1,%zmm1
   4bd75:	vpunpckhqdq %zmm1,%zmm0,%zmm2
   4bd7b:	vpunpckhqdq %zmm4,%zmm5,%zmm6
   4bd81:	vpunpcklqdq %zmm4,%zmm5,%zmm3
   4bd87:	vpunpcklqdq %zmm1,%zmm0,%zmm0
   4bd8d:	vinserti128 $0x1,%xmm0,%ymm3,%ymm1
   4bd93:	vperm2i128 $0x31,%ymm0,%ymm3,%ymm5
   4bd99:	vinserti128 $0x1,%xmm2,%ymm6,%ymm4
   4bd9f:	vperm2i128 $0x31,%ymm2,%ymm6,%ymm7
   4bda5:	vextracti64x4 $0x1,%zmm3,%ymm3
   4bdac:	vextracti64x4 $0x1,%zmm0,%ymm0
   4bdb3:	vextracti64x4 $0x1,%zmm6,%ymm6
   4bdba:	vextracti64x4 $0x1,%zmm2,%ymm2
   4bdc1:	vinserti128 $0x1,%xmm0,%ymm3,%ymm8
   4bdc7:	vinserti128 $0x1,%xmm2,%ymm6,%ymm9
   4bdcd:	vperm2i128 $0x31,%ymm0,%ymm3,%ymm0
   4bdd3:	vperm2i128 $0x31,%ymm2,%ymm6,%ymm2
   4bdd9:	vmovdqu %ymm1,(%rdi)
   4bddd:	vmovdqu %ymm4,0x20(%rdi)
   4bde2:	vmovdqu %ymm5,0x40(%rdi)
   4bde7:	vmovdqu %ymm7,0x60(%rdi)
   4bdec:	vmovdqu %ymm8,0x80(%rdi)
   4bdf4:	vmovdqu %ymm9,0xa0(%rdi)
   4bdfc:	vmovdqu %ymm0,0xc0(%rdi)
   4be04:	vmovdqu %ymm2,0xe0(%rdi)
   4be0c:	pop    %rbx
   4be0d:	pop    %r14
   4be0f:	pop    %r15
   4be11:	vzeroupper
   4be14:	ret
   4be15:	lea    0xa8864(%rip),%rdx        # f4680 <__frame_dummy_init_array_entry+0x830>
   4be1c:	mov    %r9,%rdi
   4be1f:	mov    %r8,%rsi
   4be22:	vzeroupper
   4be25:	call   5c02b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
   4be2a:	lea    0xa881f(%rip),%rdx        # f4650 <__frame_dummy_init_array_entry+0x800>
   4be31:	mov    %r9,%rdi
   4be34:	mov    %rax,%rsi
   4be37:	vzeroupper
   4be3a:	call   5c02b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
   4be3f:	lea    0xa8822(%rip),%rdx        # f4668 <__frame_dummy_init_array_entry+0x818>
   4be46:	mov    %r9,%rdi
   4be49:	mov    %rcx,%rsi
   4be4c:	vzeroupper
   4be4f:	call   5c02b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
