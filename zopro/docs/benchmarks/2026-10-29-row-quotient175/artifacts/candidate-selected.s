000000000001a670 <probe::chain::<zopro_bn254::FqConfig, true, true>>:
   1a670:	55                   	push   %rbp
   1a671:	48 89 e5             	mov    %rsp,%rbp
   1a674:	48 83 e4 c0          	and    $0xffffffffffffffc0,%rsp
   1a678:	48 81 ec c0 01 00 00 	sub    $0x1c0,%rsp
   1a67f:	62 f1 fd 48 6f 29    	vmovdqa64 (%rcx),%zmm5
   1a685:	62 f1 fd 48 6f 02    	vmovdqa64 (%rdx),%zmm0
   1a68b:	62 f1 fd 48 6f 4a 01 	vmovdqa64 0x40(%rdx),%zmm1
   1a692:	62 f1 fd 48 6f 52 02 	vmovdqa64 0x80(%rdx),%zmm2
   1a699:	62 f1 fd 48 6f 5a 03 	vmovdqa64 0xc0(%rdx),%zmm3
   1a6a0:	62 f1 fd 48 6f 62 04 	vmovdqa64 0x100(%rdx),%zmm4
   1a6a7:	62 f1 fd 48 6f 71 01 	vmovdqa64 0x40(%rcx),%zmm6
   1a6ae:	62 f1 fd 48 6f 79 02 	vmovdqa64 0x80(%rcx),%zmm7
   1a6b5:	62 71 fd 48 6f 41 03 	vmovdqa64 0xc0(%rcx),%zmm8
   1a6bc:	62 71 fd 48 6f 49 04 	vmovdqa64 0x100(%rcx),%zmm9
   1a6c3:	62 61 fd 48 6f 06    	vmovdqa64 (%rsi),%zmm24
   1a6c9:	62 e1 fd 48 6f 7e 01 	vmovdqa64 0x40(%rsi),%zmm23
   1a6d0:	62 e1 fd 48 6f 76 02 	vmovdqa64 0x80(%rsi),%zmm22
   1a6d7:	62 e1 fd 48 6f 6e 03 	vmovdqa64 0xc0(%rsi),%zmm21
   1a6de:	62 e1 fd 48 6f 66 04 	vmovdqa64 0x100(%rsi),%zmm20
   1a6e5:	62 72 fd 48 59 1d 09 	vpbroadcastq -0x167f7(%rip),%zmm11        # 3ef8 <__abi_tag+0x3bfc>
   1a6ec:	98 fe ff
   1a6ef:	62 72 fd 48 59 25 07 	vpbroadcastq -0x168f9(%rip),%zmm12        # 3e00 <__abi_tag+0x3b04>
   1a6f6:	97 fe ff
   1a6f9:	62 72 fd 48 59 2d 35 	vpbroadcastq -0x168cb(%rip),%zmm13        # 3e38 <__abi_tag+0x3b3c>
   1a700:	97 fe ff
   1a703:	62 e2 fd 48 59 05 93 	vpbroadcastq -0x1686d(%rip),%zmm16        # 3ea0 <__abi_tag+0x3ba4>
   1a70a:	97 fe ff
   1a70d:	48 b8 31 e1 72 4e 64 	movabs $0x30644e72e131,%rax
   1a714:	30 00 00
   1a717:	48 b9 63 c2 e5 9c c8 	movabs $0x60c89ce5c263,%rcx
   1a71e:	60 00 00
   1a721:	62 f1 fd 48 7f 6c 24 	vmovdqa64 %zmm5,0x140(%rsp)
   1a728:	05
   1a729:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   1a730:	c5 fd 6f 2d a8 90 fe 	vmovdqa -0x16f58(%rip),%ymm5        # 37e0 <__abi_tag+0x34e4>
   1a737:	ff
   1a738:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   1a73d:	62 52 bd 40 b4 d3    	vpmadd52luq %zmm11,%zmm24,%zmm10
   1a743:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1a748:	62 72 cd 40 b5 f8    	vpmadd52huq %zmm0,%zmm22,%zmm15
   1a74e:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   1a754:	62 62 c5 40 b5 f8    	vpmadd52huq %zmm0,%zmm23,%zmm31
   1a75a:	62 a1 6d 00 ef d2    	vpxord %xmm18,%xmm18,%xmm18
   1a760:	62 a1 65 00 ef db    	vpxord %xmm19,%xmm19,%xmm19
   1a766:	62 a1 75 00 ef c9    	vpxord %xmm17,%xmm17,%xmm17
   1a76c:	c4 41 09 ef f6       	vpxor  %xmm14,%xmm14,%xmm14
   1a771:	62 01 15 00 ef ed    	vpxord %xmm29,%xmm29,%xmm29
   1a777:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   1a77d:	62 62 bd 40 b4 f0    	vpmadd52luq %zmm0,%zmm24,%zmm30
   1a783:	49 ff c8             	dec    %r8
   1a786:	62 e2 ad 48 b4 d0    	vpmadd52luq %zmm0,%zmm10,%zmm18
   1a78c:	62 e2 ad 48 b4 d9    	vpmadd52luq %zmm1,%zmm10,%zmm19
   1a792:	62 e2 ad 48 b4 ca    	vpmadd52luq %zmm2,%zmm10,%zmm17
   1a798:	62 72 ad 48 b4 f3    	vpmadd52luq %zmm3,%zmm10,%zmm14
   1a79e:	62 62 ad 48 b4 ec    	vpmadd52luq %zmm4,%zmm10,%zmm29
   1a7a4:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   1a7a9:	62 72 d5 40 b5 d0    	vpmadd52huq %zmm0,%zmm21,%zmm10
   1a7af:	62 72 d5 40 b4 f8    	vpmadd52luq %zmm0,%zmm21,%zmm15
   1a7b5:	62 62 cd 40 b4 f8    	vpmadd52luq %zmm0,%zmm22,%zmm31
   1a7bb:	62 41 8d 40 d4 f4    	vpaddq %zmm12,%zmm30,%zmm30
   1a7c1:	62 91 8d 40 73 d6 34 	vpsrlq $0x34,%zmm30,%zmm30
   1a7c8:	c5 fe 7f 2c 24       	vmovdqu %ymm5,(%rsp)
   1a7cd:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
   1a7d2:	c5 d1 ef ed          	vpxor  %xmm5,%xmm5,%xmm5
   1a7d6:	62 f2 bd 40 b5 e8    	vpmadd52huq %zmm0,%zmm24,%zmm5
   1a7dc:	62 72 dd 40 b4 d0    	vpmadd52luq %zmm0,%zmm20,%zmm10
   1a7e2:	62 42 ed 40 b5 f5    	vpmadd52huq %zmm13,%zmm18,%zmm30
   1a7e8:	62 62 fd 48 59 5c 24 	vpbroadcastq 0x10(%rsp),%zmm27
   1a7ef:	02
   1a7f0:	62 62 fd 48 59 64 24 	vpbroadcastq 0x8(%rsp),%zmm28
   1a7f7:	01
   1a7f8:	62 62 fd 48 59 54 24 	vpbroadcastq 0x18(%rsp),%zmm26
   1a7ff:	03
   1a800:	62 62 fd 48 59 4c 24 	vpbroadcastq 0x20(%rsp),%zmm25
   1a807:	04
   1a808:	62 f2 c5 40 b4 e8    	vpmadd52luq %zmm0,%zmm23,%zmm5
   1a80e:	62 12 ed 40 b5 fb    	vpmadd52huq %zmm27,%zmm18,%zmm15
   1a814:	62 02 ed 40 b5 fc    	vpmadd52huq %zmm28,%zmm18,%zmm31
   1a81a:	62 12 ed 40 b5 d2    	vpmadd52huq %zmm26,%zmm18,%zmm10
   1a820:	62 92 ed 40 b4 ec    	vpmadd52luq %zmm28,%zmm18,%zmm5
   1a826:	62 12 ed 40 b4 fa    	vpmadd52luq %zmm26,%zmm18,%zmm15
   1a82c:	62 02 ed 40 b4 fb    	vpmadd52luq %zmm27,%zmm18,%zmm31
   1a832:	62 12 ed 40 b4 d1    	vpmadd52luq %zmm25,%zmm18,%zmm10
   1a838:	62 91 d5 48 d4 ee    	vpaddq %zmm30,%zmm5,%zmm5
   1a83e:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   1a844:	62 62 dd 40 b5 f0    	vpmadd52huq %zmm0,%zmm20,%zmm30
   1a84a:	62 c2 d5 48 b4 db    	vpmadd52luq %zmm11,%zmm5,%zmm19
   1a850:	62 f2 bd 40 b4 e9    	vpmadd52luq %zmm1,%zmm24,%zmm5
   1a856:	62 72 c5 40 b5 f9    	vpmadd52huq %zmm1,%zmm23,%zmm15
   1a85c:	62 62 bd 40 b5 f9    	vpmadd52huq %zmm1,%zmm24,%zmm31
   1a862:	62 72 cd 40 b5 d1    	vpmadd52huq %zmm1,%zmm22,%zmm10
   1a868:	62 02 ed 40 b5 f1    	vpmadd52huq %zmm25,%zmm18,%zmm30
   1a86e:	62 d1 d5 48 d4 ec    	vpaddq %zmm12,%zmm5,%zmm5
   1a874:	62 c1 e5 40 db d4    	vpandq %zmm12,%zmm19,%zmm18
   1a87a:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   1a881:	62 d2 ed 40 b5 ed    	vpmadd52huq %zmm13,%zmm18,%zmm5
   1a887:	62 72 cd 40 b4 f9    	vpmadd52luq %zmm1,%zmm22,%zmm15
   1a88d:	62 62 c5 40 b4 f9    	vpmadd52luq %zmm1,%zmm23,%zmm31
   1a893:	62 72 d5 40 b4 d1    	vpmadd52luq %zmm1,%zmm21,%zmm10
   1a899:	62 62 d5 40 b5 f1    	vpmadd52huq %zmm1,%zmm21,%zmm30
   1a89f:	62 12 ed 40 b5 fc    	vpmadd52huq %zmm28,%zmm18,%zmm15
   1a8a5:	62 02 ed 40 b4 fc    	vpmadd52luq %zmm28,%zmm18,%zmm31
   1a8ab:	62 12 ed 40 b5 d3    	vpmadd52huq %zmm27,%zmm18,%zmm10
   1a8b1:	62 62 dd 40 b4 f1    	vpmadd52luq %zmm1,%zmm20,%zmm30
   1a8b7:	62 12 ed 40 b4 fb    	vpmadd52luq %zmm27,%zmm18,%zmm15
   1a8bd:	62 f1 85 40 d4 ed    	vpaddq %zmm5,%zmm31,%zmm5
   1a8c3:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   1a8c9:	62 62 dd 40 b5 f9    	vpmadd52huq %zmm1,%zmm20,%zmm31
   1a8cf:	62 12 ed 40 b4 d2    	vpmadd52luq %zmm26,%zmm18,%zmm10
   1a8d5:	62 02 ed 40 b5 f2    	vpmadd52huq %zmm26,%zmm18,%zmm30
   1a8db:	62 c2 d5 48 b4 cb    	vpmadd52luq %zmm11,%zmm5,%zmm17
   1a8e1:	62 f2 bd 40 b4 ea    	vpmadd52luq %zmm2,%zmm24,%zmm5
   1a8e7:	62 72 bd 40 b5 fa    	vpmadd52huq %zmm2,%zmm24,%zmm15
   1a8ed:	62 02 ed 40 b5 f9    	vpmadd52huq %zmm25,%zmm18,%zmm31
   1a8f3:	62 72 c5 40 b5 d2    	vpmadd52huq %zmm2,%zmm23,%zmm10
   1a8f9:	62 02 ed 40 b4 f1    	vpmadd52luq %zmm25,%zmm18,%zmm30
   1a8ff:	62 d1 d5 48 d4 ec    	vpaddq %zmm12,%zmm5,%zmm5
   1a905:	62 c1 f5 40 db cc    	vpandq %zmm12,%zmm17,%zmm17
   1a90b:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   1a912:	62 d2 f5 40 b5 ed    	vpmadd52huq %zmm13,%zmm17,%zmm5
   1a918:	62 72 c5 40 b4 fa    	vpmadd52luq %zmm2,%zmm23,%zmm15
   1a91e:	62 62 d5 40 b5 fa    	vpmadd52huq %zmm2,%zmm21,%zmm31
   1a924:	62 72 cd 40 b4 d2    	vpmadd52luq %zmm2,%zmm22,%zmm10
   1a92a:	62 62 cd 40 b5 f2    	vpmadd52huq %zmm2,%zmm22,%zmm30
   1a930:	62 12 f5 40 b4 fc    	vpmadd52luq %zmm28,%zmm17,%zmm15
   1a936:	62 62 dd 40 b4 fa    	vpmadd52luq %zmm2,%zmm20,%zmm31
   1a93c:	62 12 f5 40 b5 d4    	vpmadd52huq %zmm28,%zmm17,%zmm10
   1a942:	62 62 d5 40 b4 f2    	vpmadd52luq %zmm2,%zmm21,%zmm30
   1a948:	62 f1 85 48 d4 ed    	vpaddq %zmm5,%zmm15,%zmm5
   1a94e:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1a953:	62 72 dd 40 b5 fa    	vpmadd52huq %zmm2,%zmm20,%zmm15
   1a959:	62 02 f5 40 b5 fa    	vpmadd52huq %zmm26,%zmm17,%zmm31
   1a95f:	62 12 f5 40 b4 d3    	vpmadd52luq %zmm27,%zmm17,%zmm10
   1a965:	62 02 f5 40 b5 f3    	vpmadd52huq %zmm27,%zmm17,%zmm30
   1a96b:	62 52 d5 48 b4 f3    	vpmadd52luq %zmm11,%zmm5,%zmm14
   1a971:	62 f2 bd 40 b4 eb    	vpmadd52luq %zmm3,%zmm24,%zmm5
   1a977:	62 12 f5 40 b5 f9    	vpmadd52huq %zmm25,%zmm17,%zmm15
   1a97d:	62 02 f5 40 b4 f9    	vpmadd52luq %zmm25,%zmm17,%zmm31
   1a983:	62 72 bd 40 b5 d3    	vpmadd52huq %zmm3,%zmm24,%zmm10
   1a989:	62 02 f5 40 b4 f2    	vpmadd52luq %zmm26,%zmm17,%zmm30
   1a98f:	62 d1 d5 48 d4 ec    	vpaddq %zmm12,%zmm5,%zmm5
   1a995:	62 51 8d 48 db f4    	vpandq %zmm12,%zmm14,%zmm14
   1a99b:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   1a9a2:	62 d2 8d 48 b5 ed    	vpmadd52huq %zmm13,%zmm14,%zmm5
   1a9a8:	62 72 d5 40 b5 fb    	vpmadd52huq %zmm3,%zmm21,%zmm15
   1a9ae:	62 62 cd 40 b5 fb    	vpmadd52huq %zmm3,%zmm22,%zmm31
   1a9b4:	62 72 c5 40 b4 d3    	vpmadd52luq %zmm3,%zmm23,%zmm10
   1a9ba:	62 62 c5 40 b5 f3    	vpmadd52huq %zmm3,%zmm23,%zmm30
   1a9c0:	62 72 dd 40 b4 fb    	vpmadd52luq %zmm3,%zmm20,%zmm15
   1a9c6:	62 62 d5 40 b4 fb    	vpmadd52luq %zmm3,%zmm21,%zmm31
   1a9cc:	62 12 8d 48 b4 d4    	vpmadd52luq %zmm28,%zmm14,%zmm10
   1a9d2:	62 62 cd 40 b4 f3    	vpmadd52luq %zmm3,%zmm22,%zmm30
   1a9d8:	62 12 8d 48 b5 fa    	vpmadd52huq %zmm26,%zmm14,%zmm15
   1a9de:	62 02 8d 48 b5 fb    	vpmadd52huq %zmm27,%zmm14,%zmm31
   1a9e4:	62 f1 ad 48 d4 ed    	vpaddq %zmm5,%zmm10,%zmm5
   1a9ea:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   1a9ef:	62 72 dd 40 b5 d3    	vpmadd52huq %zmm3,%zmm20,%zmm10
   1a9f5:	62 02 8d 48 b5 f4    	vpmadd52huq %zmm28,%zmm14,%zmm30
   1a9fb:	62 42 d5 48 b4 eb    	vpmadd52luq %zmm11,%zmm5,%zmm29
   1aa01:	62 e1 95 50 db 15 fd 	vpandq -0x16c03(%rip){1to8},%zmm29,%zmm18        # 3e08 <__abi_tag+0x3b0c>
   1aa08:	93 fe ff
   1aa0b:	62 f2 bd 40 b4 ec    	vpmadd52luq %zmm4,%zmm24,%zmm5
   1aa11:	62 f1 d5 58 d4 2d ed 	vpaddq -0x16c13(%rip){1to8},%zmm5,%zmm5        # 3e08 <__abi_tag+0x3b0c>
   1aa18:	93 fe ff
   1aa1b:	62 12 8d 48 b4 f9    	vpmadd52luq %zmm25,%zmm14,%zmm15
   1aa21:	62 02 8d 48 b4 fa    	vpmadd52luq %zmm26,%zmm14,%zmm31
   1aa27:	62 12 8d 48 b5 d1    	vpmadd52huq %zmm25,%zmm14,%zmm10
   1aa2d:	62 02 8d 48 b4 f3    	vpmadd52luq %zmm27,%zmm14,%zmm30
   1aa33:	c4 41 09 ef f6       	vpxor  %xmm14,%xmm14,%xmm14
   1aa38:	62 72 dd 40 b5 f4    	vpmadd52huq %zmm4,%zmm20,%zmm14
   1aa3e:	62 91 f5 40 73 f5 04 	vpsllq $0x4,%zmm29,%zmm17
   1aa45:	62 72 cd 40 b5 fc    	vpmadd52huq %zmm4,%zmm22,%zmm15
   1aa4b:	62 62 c5 40 b5 fc    	vpmadd52huq %zmm4,%zmm23,%zmm31
   1aa51:	62 72 d5 40 b5 d4    	vpmadd52huq %zmm4,%zmm21,%zmm10
   1aa57:	62 62 bd 40 b5 f4    	vpmadd52huq %zmm4,%zmm24,%zmm30
   1aa5d:	62 f1 d5 48 73 d5 30 	vpsrlq $0x30,%zmm5,%zmm5
   1aa64:	62 12 ed 40 b5 f1    	vpmadd52huq %zmm25,%zmm18,%zmm14
   1aa6a:	62 d2 f5 40 b5 ed    	vpmadd52huq %zmm13,%zmm17,%zmm5
   1aa70:	62 72 d5 40 b4 fc    	vpmadd52luq %zmm4,%zmm21,%zmm15
   1aa76:	62 62 cd 40 b4 fc    	vpmadd52luq %zmm4,%zmm22,%zmm31
   1aa7c:	62 72 dd 40 b4 d4    	vpmadd52luq %zmm4,%zmm20,%zmm10
   1aa82:	62 62 c5 40 b4 f4    	vpmadd52luq %zmm4,%zmm23,%zmm30
   1aa88:	62 d1 8d 48 73 f6 04 	vpsllq $0x4,%zmm14,%zmm14
   1aa8f:	62 12 ed 40 b5 fb    	vpmadd52huq %zmm27,%zmm18,%zmm15
   1aa95:	62 02 ed 40 b5 fc    	vpmadd52huq %zmm28,%zmm18,%zmm31
   1aa9b:	62 12 ed 40 b5 d2    	vpmadd52huq %zmm26,%zmm18,%zmm10
   1aaa1:	62 02 ed 40 b4 f4    	vpmadd52luq %zmm28,%zmm18,%zmm30
   1aaa7:	62 12 ed 40 b4 fa    	vpmadd52luq %zmm26,%zmm18,%zmm15
   1aaad:	62 02 ed 40 b4 fb    	vpmadd52luq %zmm27,%zmm18,%zmm31
   1aab3:	62 12 ed 40 b4 d1    	vpmadd52luq %zmm25,%zmm18,%zmm10
   1aab9:	62 91 f5 40 73 f6 04 	vpsllq $0x4,%zmm30,%zmm17
   1aac0:	62 a1 f5 40 db c8    	vpandq %zmm16,%zmm17,%zmm17
   1aac6:	62 f1 f5 40 d4 ed    	vpaddq %zmm5,%zmm17,%zmm5
   1aacc:	62 91 f5 40 73 d6 30 	vpsrlq $0x30,%zmm30,%zmm17
   1aad3:	62 d1 dd 40 73 f7 04 	vpsllq $0x4,%zmm15,%zmm20
   1aada:	62 91 e5 40 73 d7 30 	vpsrlq $0x30,%zmm31,%zmm19
   1aae1:	62 d1 85 48 73 d7 30 	vpsrlq $0x30,%zmm15,%zmm15
   1aae8:	62 91 ed 40 73 f7 04 	vpsllq $0x4,%zmm31,%zmm18
   1aaef:	62 a1 dd 40 db e0    	vpandq %zmm16,%zmm20,%zmm20
   1aaf5:	62 a1 ed 40 db d0    	vpandq %zmm16,%zmm18,%zmm18
   1aafb:	62 a1 dd 40 d4 db    	vpaddq %zmm19,%zmm20,%zmm19
   1ab01:	62 d1 dd 40 73 f2 04 	vpsllq $0x4,%zmm10,%zmm20
   1ab08:	62 d1 ad 48 73 d2 30 	vpsrlq $0x30,%zmm10,%zmm10
   1ab0f:	62 a1 dd 40 db e0    	vpandq %zmm16,%zmm20,%zmm20
   1ab15:	62 51 8d 48 d4 d2    	vpaddq %zmm10,%zmm14,%zmm10
   1ab1b:	62 51 dd 40 d4 ff    	vpaddq %zmm15,%zmm20,%zmm15
   1ab21:	62 f1 dd 40 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm20
   1ab28:	62 d1 d5 48 db ec    	vpandq %zmm12,%zmm5,%zmm5
   1ab2e:	62 f1 d5 48 d4 6c 24 	vpaddq 0x140(%rsp),%zmm5,%zmm5
   1ab35:	05
   1ab36:	62 51 b5 48 d4 d2    	vpaddq %zmm10,%zmm9,%zmm10
   1ab3c:	62 31 dd 40 d4 f1    	vpaddq %zmm17,%zmm20,%zmm14
   1ab42:	62 31 8d 48 d4 f2    	vpaddq %zmm18,%zmm14,%zmm14
   1ab48:	62 d1 f5 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm17
   1ab4f:	62 51 8d 48 db f4    	vpandq %zmm12,%zmm14,%zmm14
   1ab55:	62 a1 f5 40 d4 cb    	vpaddq %zmm19,%zmm17,%zmm17
   1ab5b:	62 b1 ed 40 73 d1 34 	vpsrlq $0x34,%zmm17,%zmm18
   1ab62:	62 c1 f5 40 db cc    	vpandq %zmm12,%zmm17,%zmm17
   1ab68:	62 51 ed 40 d4 ff    	vpaddq %zmm15,%zmm18,%zmm15
   1ab6e:	62 a1 c5 48 d4 c9    	vpaddq %zmm17,%zmm7,%zmm17
   1ab74:	62 d1 ed 40 73 d7 34 	vpsrlq $0x34,%zmm15,%zmm18
   1ab7b:	62 51 85 48 db fc    	vpandq %zmm12,%zmm15,%zmm15
   1ab81:	62 f1 e5 40 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm19
   1ab88:	62 31 ad 48 d4 d2    	vpaddq %zmm18,%zmm10,%zmm10
   1ab8e:	62 41 d5 48 db c4    	vpandq %zmm12,%zmm5,%zmm24
   1ab94:	62 51 bd 48 d4 ff    	vpaddq %zmm15,%zmm8,%zmm15
   1ab9a:	62 e1 e5 40 d4 d6    	vpaddq %zmm6,%zmm19,%zmm18
   1aba0:	62 51 ed 40 d4 f6    	vpaddq %zmm14,%zmm18,%zmm14
   1aba6:	62 d1 d5 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm5
   1abad:	62 c1 8d 48 db fc    	vpandq %zmm12,%zmm14,%zmm23
   1abb3:	62 b1 d5 48 d4 e9    	vpaddq %zmm17,%zmm5,%zmm5
   1abb9:	62 e1 bd 50 d4 0d ed 	vpaddq -0x16d13(%rip){1to8},%zmm24,%zmm17        # 3eb0 <__abi_tag+0x3bb4>
   1abc0:	92 fe ff
   1abc3:	62 f1 8d 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm14
   1abca:	62 c1 d5 48 db f4    	vpandq %zmm12,%zmm5,%zmm22
   1abd0:	62 51 8d 48 d4 f7    	vpaddq %zmm15,%zmm14,%zmm14
   1abd6:	62 d1 d5 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm5
   1abdd:	62 c1 8d 48 db ec    	vpandq %zmm12,%zmm14,%zmm21
   1abe3:	62 c1 d5 48 d4 e2    	vpaddq %zmm10,%zmm5,%zmm20
   1abe9:	c5 7c 28 15 ef 8f fe 	vmovaps -0x17011(%rip),%ymm10        # 3be0 <__abi_tag+0x38e4>
   1abf0:	ff
   1abf1:	62 b1 ed 40 72 e1 34 	vpsraq $0x34,%zmm17,%zmm18
   1abf8:	c5 7c 11 14 24       	vmovups %ymm10,(%rsp)
   1abfd:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   1ac02:	62 f1 c5 50 fb 6c 24 	vpsubq 0x8(%rsp){1to8},%zmm23,%zmm5
   1ac09:	01
   1ac0a:	62 71 cd 50 fb 54 24 	vpsubq 0x10(%rsp){1to8},%zmm22,%zmm10
   1ac11:	02
   1ac12:	62 71 d5 50 fb 74 24 	vpsubq 0x18(%rsp){1to8},%zmm21,%zmm14
   1ac19:	03
   1ac1a:	62 71 dd 50 fb 7c 24 	vpsubq 0x20(%rsp){1to8},%zmm20,%zmm15
   1ac21:	04
   1ac22:	62 b1 d5 48 d4 ea    	vpaddq %zmm18,%zmm5,%zmm5
   1ac28:	62 f1 ed 40 72 e5 34 	vpsraq $0x34,%zmm5,%zmm18
   1ac2f:	62 31 ad 48 d4 d2    	vpaddq %zmm18,%zmm10,%zmm10
   1ac35:	62 d1 ed 40 72 e2 34 	vpsraq $0x34,%zmm10,%zmm18
   1ac3c:	62 31 8d 48 d4 f2    	vpaddq %zmm18,%zmm14,%zmm14
   1ac42:	62 d1 ed 40 72 e6 34 	vpsraq $0x34,%zmm14,%zmm18
   1ac49:	62 31 85 48 d4 fa    	vpaddq %zmm18,%zmm15,%zmm15
   1ac4f:	62 f3 85 58 1e 0d f6 	vpcmpltuq -0x16e0a(%rip){1to8},%zmm15,%k1        # 3e50 <__abi_tag+0x3b54>
   1ac56:	91 fe ff 01
   1ac5a:	62 41 f5 41 db c4    	vpandq %zmm12,%zmm17,%zmm24{%k1}
   1ac60:	62 c1 d5 49 db fc    	vpandq %zmm12,%zmm5,%zmm23{%k1}
   1ac66:	62 c1 ad 49 db f4    	vpandq %zmm12,%zmm10,%zmm22{%k1}
   1ac6c:	62 c1 8d 49 db ec    	vpandq %zmm12,%zmm14,%zmm21{%k1}
   1ac72:	62 c1 85 49 db e4    	vpandq %zmm12,%zmm15,%zmm20{%k1}
   1ac78:	0f 85 b2 fa ff ff    	jne    1a730 <probe::chain::<zopro_bn254::FqConfig, true, true>+0xc0>
   1ac7e:	62 61 fd 48 7f 06    	vmovdqa64 %zmm24,(%rsi)
   1ac84:	62 e1 fd 48 7f 7e 01 	vmovdqa64 %zmm23,0x40(%rsi)
   1ac8b:	62 e1 fd 48 7f 76 02 	vmovdqa64 %zmm22,0x80(%rsi)
   1ac92:	62 e1 fd 48 7f 6e 03 	vmovdqa64 %zmm21,0xc0(%rsi)
   1ac99:	62 e1 fd 48 7f 66 04 	vmovdqa64 %zmm20,0x100(%rsi)
   1aca0:	48 89 e0             	mov    %rsp,%rax
   1aca3:	62 f1 7c 48 28 46 04 	vmovaps 0x100(%rsi),%zmm0
   1acaa:	62 f1 7c 48 28 2e    	vmovaps (%rsi),%zmm5
   1acb0:	62 f1 7c 48 28 4e 01 	vmovaps 0x40(%rsi),%zmm1
   1acb7:	62 f1 7c 48 28 56 02 	vmovaps 0x80(%rsi),%zmm2
   1acbe:	62 f1 7c 48 28 5e 03 	vmovaps 0xc0(%rsi),%zmm3
   1acc5:	62 f1 7c 48 29 44 24 	vmovaps %zmm0,0x100(%rsp)
   1accc:	04
   1accd:	62 f1 7c 48 29 5c 24 	vmovaps %zmm3,0xc0(%rsp)
   1acd4:	03
   1acd5:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x80(%rsp)
   1acdc:	02
   1acdd:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x40(%rsp)
   1ace4:	01
   1ace5:	62 f1 7c 48 29 2c 24 	vmovaps %zmm5,(%rsp)
   1acec:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1acf3:	01
   1acf4:	62 f1 7c 48 28 44 24 	vmovaps 0x100(%rsp),%zmm0
   1acfb:	04
   1acfc:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1ad03:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1ad0a:	02
   1ad0b:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1ad12:	03
   1ad13:	62 f1 7c 48 28 2c 24 	vmovaps (%rsp),%zmm5
   1ad1a:	62 f1 7c 48 29 4f 01 	vmovaps %zmm1,0x40(%rdi)
   1ad21:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1ad28:	04
   1ad29:	62 f1 7c 48 29 5f 03 	vmovaps %zmm3,0xc0(%rdi)
   1ad30:	62 f1 7c 48 29 57 02 	vmovaps %zmm2,0x80(%rdi)
   1ad37:	62 f1 7c 48 29 27    	vmovaps %zmm4,(%rdi)
   1ad3d:	62 f1 7c 48 29 47 04 	vmovaps %zmm0,0x100(%rdi)
   1ad44:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1ad4b:	02
   1ad4c:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1ad53:	03
   1ad54:	62 f1 7c 48 29 6f 05 	vmovaps %zmm5,0x140(%rdi)
   1ad5b:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1ad62:	62 f1 7c 48 29 4f 09 	vmovaps %zmm1,0x240(%rdi)
   1ad69:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1ad70:	01
   1ad71:	62 f1 7c 48 29 57 07 	vmovaps %zmm2,0x1c0(%rdi)
   1ad78:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1ad7f:	02
   1ad80:	62 f1 7c 48 29 5f 08 	vmovaps %zmm3,0x200(%rdi)
   1ad87:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1ad8e:	03
   1ad8f:	62 f1 7c 48 29 4f 06 	vmovaps %zmm1,0x180(%rdi)
   1ad96:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1ad9d:	01
   1ad9e:	62 f1 7c 48 29 57 0c 	vmovaps %zmm2,0x300(%rdi)
   1ada5:	62 f1 7c 48 28 54 24 	vmovaps 0x100(%rsp),%zmm2
   1adac:	04
   1adad:	62 f1 7c 48 29 67 0a 	vmovaps %zmm4,0x280(%rdi)
   1adb4:	62 f1 7c 48 29 5f 0d 	vmovaps %zmm3,0x340(%rdi)
   1adbb:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1adc2:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1adc9:	03
   1adca:	62 f1 7c 48 29 4f 0b 	vmovaps %zmm1,0x2c0(%rdi)
   1add1:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1add8:	04
   1add9:	62 f1 7c 48 29 57 0e 	vmovaps %zmm2,0x380(%rdi)
   1ade0:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1ade7:	02
   1ade8:	62 f1 7c 48 29 5f 12 	vmovaps %zmm3,0x480(%rdi)
   1adef:	62 f1 7c 48 29 67 0f 	vmovaps %zmm4,0x3c0(%rdi)
   1adf6:	62 f1 7c 48 29 4f 13 	vmovaps %zmm1,0x4c0(%rdi)
   1adfd:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1ae04:	01
   1ae05:	62 f1 7c 48 29 57 11 	vmovaps %zmm2,0x440(%rdi)
   1ae0c:	62 f1 7c 48 29 4f 10 	vmovaps %zmm1,0x400(%rdi)
   1ae13:	48 89 ec             	mov    %rbp,%rsp
   1ae16:	5d                   	pop    %rbp
   1ae17:	c5 f8 77             	vzeroupper
   1ae1a:	c3                   	ret
   1ae1b:	cc                   	int3
   1ae1c:	cc                   	int3
   1ae1d:	cc                   	int3
   1ae1e:	cc                   	int3
   1ae1f:	cc                   	int3

000000000001ae20 <probe::chain::<zopro_bn254::FqConfig, true, false>>:
   1ae20:	55                   	push   %rbp
   1ae21:	48 89 e5             	mov    %rsp,%rbp
   1ae24:	48 83 e4 c0          	and    $0xffffffffffffffc0,%rsp
   1ae28:	48 81 ec 80 01 00 00 	sub    $0x180,%rsp
   1ae2f:	62 f1 fd 48 6f 02    	vmovdqa64 (%rdx),%zmm0
   1ae35:	62 f1 fd 48 6f 4a 01 	vmovdqa64 0x40(%rdx),%zmm1
   1ae3c:	62 f1 fd 48 6f 52 02 	vmovdqa64 0x80(%rdx),%zmm2
   1ae43:	62 f1 fd 48 6f 5a 03 	vmovdqa64 0xc0(%rdx),%zmm3
   1ae4a:	62 f1 fd 48 6f 62 04 	vmovdqa64 0x100(%rdx),%zmm4
   1ae51:	62 f1 fd 48 6f 29    	vmovdqa64 (%rcx),%zmm5
   1ae57:	62 f1 fd 48 6f 71 01 	vmovdqa64 0x40(%rcx),%zmm6
   1ae5e:	62 f1 fd 48 6f 79 02 	vmovdqa64 0x80(%rcx),%zmm7
   1ae65:	62 71 fd 48 6f 41 03 	vmovdqa64 0xc0(%rcx),%zmm8
   1ae6c:	62 71 fd 48 6f 49 04 	vmovdqa64 0x100(%rcx),%zmm9
   1ae73:	62 61 fd 48 6f 06    	vmovdqa64 (%rsi),%zmm24
   1ae79:	62 e1 fd 48 6f 7e 01 	vmovdqa64 0x40(%rsi),%zmm23
   1ae80:	62 e1 fd 48 6f 76 02 	vmovdqa64 0x80(%rsi),%zmm22
   1ae87:	62 e1 fd 48 6f 6e 03 	vmovdqa64 0xc0(%rsi),%zmm21
   1ae8e:	62 e1 fd 48 6f 66 04 	vmovdqa64 0x100(%rsi),%zmm20
   1ae95:	62 72 fd 48 59 1d 59 	vpbroadcastq -0x16fa7(%rip),%zmm11        # 3ef8 <__abi_tag+0x3bfc>
   1ae9c:	90 fe ff
   1ae9f:	62 72 fd 48 59 25 57 	vpbroadcastq -0x170a9(%rip),%zmm12        # 3e00 <__abi_tag+0x3b04>
   1aea6:	8f fe ff
   1aea9:	62 72 fd 48 59 2d 85 	vpbroadcastq -0x1707b(%rip),%zmm13        # 3e38 <__abi_tag+0x3b3c>
   1aeb0:	8f fe ff
   1aeb3:	62 e2 fd 48 59 05 e3 	vpbroadcastq -0x1701d(%rip),%zmm16        # 3ea0 <__abi_tag+0x3ba4>
   1aeba:	8f fe ff
   1aebd:	62 e1 7c 28 28 0d 19 	vmovaps -0x172e7(%rip),%ymm17        # 3be0 <__abi_tag+0x38e4>
   1aec4:	8d fe ff
   1aec7:	62 e2 fd 48 59 15 df 	vpbroadcastq -0x17021(%rip),%zmm18        # 3eb0 <__abi_tag+0x3bb4>
   1aece:	8f fe ff
   1aed1:	62 e2 fd 48 59 1d 75 	vpbroadcastq -0x1708b(%rip),%zmm19        # 3e50 <__abi_tag+0x3b54>
   1aed8:	8f fe ff
   1aedb:	48 b8 31 e1 72 4e 64 	movabs $0x30644e72e131,%rax
   1aee2:	30 00 00
   1aee5:	48 b9 63 c2 e5 9c c8 	movabs $0x60c89ce5c263,%rcx
   1aeec:	60 00 00
   1aeef:	90                   	nop
   1aef0:	c5 7d 6f 15 e8 88 fe 	vmovdqa -0x17718(%rip),%ymm10        # 37e0 <__abi_tag+0x34e4>
   1aef7:	ff
   1aef8:	62 01 15 00 ef ed    	vpxord %xmm29,%xmm29,%xmm29
   1aefe:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   1af04:	62 62 d5 40 b5 f8    	vpmadd52huq %zmm0,%zmm21,%zmm31
   1af0a:	62 62 bd 40 b4 e8    	vpmadd52luq %zmm0,%zmm24,%zmm29
   1af10:	c4 41 09 ef f6       	vpxor  %xmm14,%xmm14,%xmm14
   1af15:	62 72 bd 40 b5 f0    	vpmadd52huq %zmm0,%zmm24,%zmm14
   1af1b:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   1af21:	62 62 c5 40 b5 f0    	vpmadd52huq %zmm0,%zmm23,%zmm30
   1af27:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1af2c:	49 ff c8             	dec    %r8
   1af2f:	62 52 95 40 b4 fb    	vpmadd52luq %zmm11,%zmm29,%zmm15
   1af35:	62 62 dd 40 b4 f8    	vpmadd52luq %zmm0,%zmm20,%zmm31
   1af3b:	62 72 c5 40 b4 f0    	vpmadd52luq %zmm0,%zmm23,%zmm14
   1af41:	62 62 cd 40 b4 f0    	vpmadd52luq %zmm0,%zmm22,%zmm30
   1af47:	62 41 95 40 d4 ec    	vpaddq %zmm12,%zmm29,%zmm29
   1af4d:	62 91 95 40 73 d5 34 	vpsrlq $0x34,%zmm29,%zmm29
   1af54:	c5 7e 7f 14 24       	vmovdqu %ymm10,(%rsp)
   1af59:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
   1af5e:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   1af63:	62 72 cd 40 b5 d0    	vpmadd52huq %zmm0,%zmm22,%zmm10
   1af69:	62 42 85 48 b5 ed    	vpmadd52huq %zmm13,%zmm15,%zmm29
   1af6f:	62 62 fd 48 59 54 24 	vpbroadcastq 0x18(%rsp),%zmm26
   1af76:	03
   1af77:	62 62 fd 48 59 64 24 	vpbroadcastq 0x8(%rsp),%zmm28
   1af7e:	01
   1af7f:	62 62 fd 48 59 5c 24 	vpbroadcastq 0x10(%rsp),%zmm27
   1af86:	02
   1af87:	62 62 fd 48 59 4c 24 	vpbroadcastq 0x20(%rsp),%zmm25
   1af8e:	04
   1af8f:	62 e1 7c 28 11 0c 24 	vmovups %ymm17,(%rsp)
   1af96:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   1af9b:	62 72 d5 40 b4 d0    	vpmadd52luq %zmm0,%zmm21,%zmm10
   1afa1:	62 02 85 48 b5 fa    	vpmadd52huq %zmm26,%zmm15,%zmm31
   1afa7:	62 12 85 48 b4 f4    	vpmadd52luq %zmm28,%zmm15,%zmm14
   1afad:	62 02 85 48 b5 f4    	vpmadd52huq %zmm28,%zmm15,%zmm30
   1afb3:	62 12 85 48 b5 d3    	vpmadd52huq %zmm27,%zmm15,%zmm10
   1afb9:	62 02 85 48 b4 f9    	vpmadd52luq %zmm25,%zmm15,%zmm31
   1afbf:	62 11 8d 48 d4 f5    	vpaddq %zmm29,%zmm14,%zmm14
   1afc5:	62 02 85 48 b4 f3    	vpmadd52luq %zmm27,%zmm15,%zmm30
   1afcb:	62 12 85 48 b4 d2    	vpmadd52luq %zmm26,%zmm15,%zmm10
   1afd1:	62 01 15 00 ef ed    	vpxord %xmm29,%xmm29,%xmm29
   1afd7:	62 62 dd 40 b5 e8    	vpmadd52huq %zmm0,%zmm20,%zmm29
   1afdd:	62 72 bd 40 b4 f1    	vpmadd52luq %zmm1,%zmm24,%zmm14
   1afe3:	62 62 cd 40 b5 f9    	vpmadd52huq %zmm1,%zmm22,%zmm31
   1afe9:	62 62 bd 40 b5 f1    	vpmadd52huq %zmm1,%zmm24,%zmm30
   1afef:	62 72 c5 40 b5 d1    	vpmadd52huq %zmm1,%zmm23,%zmm10
   1aff5:	62 02 85 48 b5 e9    	vpmadd52huq %zmm25,%zmm15,%zmm29
   1affb:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1b000:	62 52 8d 48 b4 fb    	vpmadd52luq %zmm11,%zmm14,%zmm15
   1b006:	62 51 8d 48 d4 f4    	vpaddq %zmm12,%zmm14,%zmm14
   1b00c:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   1b013:	62 62 d5 40 b4 f9    	vpmadd52luq %zmm1,%zmm21,%zmm31
   1b019:	62 62 c5 40 b4 f1    	vpmadd52luq %zmm1,%zmm23,%zmm30
   1b01f:	62 72 cd 40 b4 d1    	vpmadd52luq %zmm1,%zmm22,%zmm10
   1b025:	62 62 d5 40 b5 e9    	vpmadd52huq %zmm1,%zmm21,%zmm29
   1b02b:	62 52 85 48 b5 f5    	vpmadd52huq %zmm13,%zmm15,%zmm14
   1b031:	62 02 85 48 b5 fb    	vpmadd52huq %zmm27,%zmm15,%zmm31
   1b037:	62 12 85 48 b5 d4    	vpmadd52huq %zmm28,%zmm15,%zmm10
   1b03d:	62 02 85 48 b4 f4    	vpmadd52luq %zmm28,%zmm15,%zmm30
   1b043:	62 62 dd 40 b4 e9    	vpmadd52luq %zmm1,%zmm20,%zmm29
   1b049:	62 02 85 48 b4 fa    	vpmadd52luq %zmm26,%zmm15,%zmm31
   1b04f:	62 51 8d 40 d4 f6    	vpaddq %zmm14,%zmm30,%zmm14
   1b055:	62 12 85 48 b4 d3    	vpmadd52luq %zmm27,%zmm15,%zmm10
   1b05b:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   1b061:	62 62 dd 40 b5 f1    	vpmadd52huq %zmm1,%zmm20,%zmm30
   1b067:	62 02 85 48 b5 ea    	vpmadd52huq %zmm26,%zmm15,%zmm29
   1b06d:	62 72 bd 40 b4 f2    	vpmadd52luq %zmm2,%zmm24,%zmm14
   1b073:	62 62 c5 40 b5 fa    	vpmadd52huq %zmm2,%zmm23,%zmm31
   1b079:	62 72 bd 40 b5 d2    	vpmadd52huq %zmm2,%zmm24,%zmm10
   1b07f:	62 02 85 48 b4 e9    	vpmadd52luq %zmm25,%zmm15,%zmm29
   1b085:	62 02 85 48 b5 f1    	vpmadd52huq %zmm25,%zmm15,%zmm30
   1b08b:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1b090:	62 52 8d 48 b4 fb    	vpmadd52luq %zmm11,%zmm14,%zmm15
   1b096:	62 51 8d 48 d4 f4    	vpaddq %zmm12,%zmm14,%zmm14
   1b09c:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   1b0a3:	62 62 cd 40 b4 fa    	vpmadd52luq %zmm2,%zmm22,%zmm31
   1b0a9:	62 72 c5 40 b4 d2    	vpmadd52luq %zmm2,%zmm23,%zmm10
   1b0af:	62 62 cd 40 b5 ea    	vpmadd52huq %zmm2,%zmm22,%zmm29
   1b0b5:	62 62 d5 40 b5 f2    	vpmadd52huq %zmm2,%zmm21,%zmm30
   1b0bb:	62 52 85 48 b5 f5    	vpmadd52huq %zmm13,%zmm15,%zmm14
   1b0c1:	62 02 85 48 b5 fc    	vpmadd52huq %zmm28,%zmm15,%zmm31
   1b0c7:	62 12 85 48 b4 d4    	vpmadd52luq %zmm28,%zmm15,%zmm10
   1b0cd:	62 62 d5 40 b4 ea    	vpmadd52luq %zmm2,%zmm21,%zmm29
   1b0d3:	62 62 dd 40 b4 f2    	vpmadd52luq %zmm2,%zmm20,%zmm30
   1b0d9:	62 02 85 48 b4 fb    	vpmadd52luq %zmm27,%zmm15,%zmm31
   1b0df:	62 51 ad 48 d4 f6    	vpaddq %zmm14,%zmm10,%zmm14
   1b0e5:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   1b0ea:	62 72 dd 40 b5 d2    	vpmadd52huq %zmm2,%zmm20,%zmm10
   1b0f0:	62 02 85 48 b5 eb    	vpmadd52huq %zmm27,%zmm15,%zmm29
   1b0f6:	62 02 85 48 b5 f2    	vpmadd52huq %zmm26,%zmm15,%zmm30
   1b0fc:	62 72 bd 40 b4 f3    	vpmadd52luq %zmm3,%zmm24,%zmm14
   1b102:	62 62 bd 40 b5 fb    	vpmadd52huq %zmm3,%zmm24,%zmm31
   1b108:	62 02 85 48 b4 ea    	vpmadd52luq %zmm26,%zmm15,%zmm29
   1b10e:	62 02 85 48 b4 f1    	vpmadd52luq %zmm25,%zmm15,%zmm30
   1b114:	62 12 85 48 b5 d1    	vpmadd52huq %zmm25,%zmm15,%zmm10
   1b11a:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1b11f:	62 52 8d 48 b4 fb    	vpmadd52luq %zmm11,%zmm14,%zmm15
   1b125:	62 51 8d 48 d4 f4    	vpaddq %zmm12,%zmm14,%zmm14
   1b12b:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   1b132:	62 62 c5 40 b4 fb    	vpmadd52luq %zmm3,%zmm23,%zmm31
   1b138:	62 72 d5 40 b5 d3    	vpmadd52huq %zmm3,%zmm21,%zmm10
   1b13e:	62 62 c5 40 b5 eb    	vpmadd52huq %zmm3,%zmm23,%zmm29
   1b144:	62 62 cd 40 b5 f3    	vpmadd52huq %zmm3,%zmm22,%zmm30
   1b14a:	62 52 85 48 b5 f5    	vpmadd52huq %zmm13,%zmm15,%zmm14
   1b150:	62 02 85 48 b4 fc    	vpmadd52luq %zmm28,%zmm15,%zmm31
   1b156:	62 72 dd 40 b4 d3    	vpmadd52luq %zmm3,%zmm20,%zmm10
   1b15c:	62 62 cd 40 b4 eb    	vpmadd52luq %zmm3,%zmm22,%zmm29
   1b162:	62 62 d5 40 b4 f3    	vpmadd52luq %zmm3,%zmm21,%zmm30
   1b168:	62 51 85 40 d4 f6    	vpaddq %zmm14,%zmm31,%zmm14
   1b16e:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   1b174:	62 62 dd 40 b5 fb    	vpmadd52huq %zmm3,%zmm20,%zmm31
   1b17a:	62 12 85 48 b5 d2    	vpmadd52huq %zmm26,%zmm15,%zmm10
   1b180:	62 02 85 48 b5 ec    	vpmadd52huq %zmm28,%zmm15,%zmm29
   1b186:	62 02 85 48 b5 f3    	vpmadd52huq %zmm27,%zmm15,%zmm30
   1b18c:	62 72 bd 40 b4 f4    	vpmadd52luq %zmm4,%zmm24,%zmm14
   1b192:	62 02 85 48 b5 f9    	vpmadd52huq %zmm25,%zmm15,%zmm31
   1b198:	62 12 85 48 b4 d1    	vpmadd52luq %zmm25,%zmm15,%zmm10
   1b19e:	62 02 85 48 b4 eb    	vpmadd52luq %zmm27,%zmm15,%zmm29
   1b1a4:	62 02 85 48 b4 f2    	vpmadd52luq %zmm26,%zmm15,%zmm30
   1b1aa:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1b1af:	62 72 dd 40 b5 fc    	vpmadd52huq %zmm4,%zmm20,%zmm15
   1b1b5:	62 62 d5 40 b5 fc    	vpmadd52huq %zmm4,%zmm21,%zmm31
   1b1bb:	62 72 cd 40 b5 d4    	vpmadd52huq %zmm4,%zmm22,%zmm10
   1b1c1:	62 62 c5 40 b5 f4    	vpmadd52huq %zmm4,%zmm23,%zmm30
   1b1c7:	62 62 bd 40 b5 ec    	vpmadd52huq %zmm4,%zmm24,%zmm29
   1b1cd:	62 62 dd 40 b4 fc    	vpmadd52luq %zmm4,%zmm20,%zmm31
   1b1d3:	62 a1 5d 00 ef e4    	vpxord %xmm20,%xmm20,%xmm20
   1b1d9:	62 c2 8d 48 b4 e3    	vpmadd52luq %zmm11,%zmm14,%zmm20
   1b1df:	62 72 d5 40 b4 d4    	vpmadd52luq %zmm4,%zmm21,%zmm10
   1b1e5:	62 62 cd 40 b4 f4    	vpmadd52luq %zmm4,%zmm22,%zmm30
   1b1eb:	62 71 8d 58 d4 35 13 	vpaddq -0x173ed(%rip){1to8},%zmm14,%zmm14        # 3e08 <__abi_tag+0x3b0c>
   1b1f2:	8c fe ff
   1b1f5:	62 62 c5 40 b4 ec    	vpmadd52luq %zmm4,%zmm23,%zmm29
   1b1fb:	62 b1 d5 40 73 f4 04 	vpsllq $0x4,%zmm20,%zmm21
   1b202:	62 e1 dd 50 db 25 fc 	vpandq -0x17404(%rip){1to8},%zmm20,%zmm20        # 3e08 <__abi_tag+0x3b0c>
   1b209:	8b fe ff
   1b20c:	62 d1 8d 48 73 d6 30 	vpsrlq $0x30,%zmm14,%zmm14
   1b213:	62 52 d5 40 b5 f5    	vpmadd52huq %zmm13,%zmm21,%zmm14
   1b219:	62 12 dd 40 b5 d3    	vpmadd52huq %zmm27,%zmm20,%zmm10
   1b21f:	62 02 dd 40 b5 f4    	vpmadd52huq %zmm28,%zmm20,%zmm30
   1b225:	62 02 dd 40 b5 fa    	vpmadd52huq %zmm26,%zmm20,%zmm31
   1b22b:	62 02 dd 40 b4 ec    	vpmadd52luq %zmm28,%zmm20,%zmm29
   1b231:	62 12 dd 40 b5 f9    	vpmadd52huq %zmm25,%zmm20,%zmm15
   1b237:	62 12 dd 40 b4 d2    	vpmadd52luq %zmm26,%zmm20,%zmm10
   1b23d:	62 02 dd 40 b4 f3    	vpmadd52luq %zmm27,%zmm20,%zmm30
   1b243:	62 02 dd 40 b4 f9    	vpmadd52luq %zmm25,%zmm20,%zmm31
   1b249:	62 91 dd 40 73 f5 04 	vpsllq $0x4,%zmm29,%zmm20
   1b250:	62 d1 85 48 73 f7 04 	vpsllq $0x4,%zmm15,%zmm15
   1b257:	62 a1 dd 40 db e0    	vpandq %zmm16,%zmm20,%zmm20
   1b25d:	62 51 dd 40 d4 f6    	vpaddq %zmm14,%zmm20,%zmm14
   1b263:	62 91 dd 40 73 d5 30 	vpsrlq $0x30,%zmm29,%zmm20
   1b26a:	62 d1 c5 40 73 f2 04 	vpsllq $0x4,%zmm10,%zmm23
   1b271:	62 91 cd 40 73 d6 30 	vpsrlq $0x30,%zmm30,%zmm22
   1b278:	62 d1 ad 48 73 d2 30 	vpsrlq $0x30,%zmm10,%zmm10
   1b27f:	62 91 d5 40 73 f6 04 	vpsllq $0x4,%zmm30,%zmm21
   1b286:	62 a1 c5 40 db f8    	vpandq %zmm16,%zmm23,%zmm23
   1b28c:	62 a1 d5 40 db e8    	vpandq %zmm16,%zmm21,%zmm21
   1b292:	62 a1 c5 40 d4 f6    	vpaddq %zmm22,%zmm23,%zmm22
   1b298:	62 91 c5 40 73 f7 04 	vpsllq $0x4,%zmm31,%zmm23
   1b29f:	62 a1 c5 40 db f8    	vpandq %zmm16,%zmm23,%zmm23
   1b2a5:	62 51 c5 40 d4 d2    	vpaddq %zmm10,%zmm23,%zmm10
   1b2ab:	62 91 c5 40 73 d7 30 	vpsrlq $0x30,%zmm31,%zmm23
   1b2b2:	62 31 85 48 d4 ff    	vpaddq %zmm23,%zmm15,%zmm15
   1b2b8:	62 d1 c5 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm23
   1b2bf:	62 51 8d 48 db f4    	vpandq %zmm12,%zmm14,%zmm14
   1b2c5:	62 a1 c5 40 d4 e4    	vpaddq %zmm20,%zmm23,%zmm20
   1b2cb:	62 51 d5 48 d4 f6    	vpaddq %zmm14,%zmm5,%zmm14
   1b2d1:	62 51 b5 48 d4 ff    	vpaddq %zmm15,%zmm9,%zmm15
   1b2d7:	62 a1 dd 40 d4 e5    	vpaddq %zmm21,%zmm20,%zmm20
   1b2dd:	62 41 8d 48 db c4    	vpandq %zmm12,%zmm14,%zmm24
   1b2e3:	62 b1 d5 40 73 d4 34 	vpsrlq $0x34,%zmm20,%zmm21
   1b2ea:	62 c1 dd 40 db e4    	vpandq %zmm12,%zmm20,%zmm20
   1b2f0:	62 21 bd 40 d4 d2    	vpaddq %zmm18,%zmm24,%zmm26
   1b2f6:	62 a1 d5 40 d4 ee    	vpaddq %zmm22,%zmm21,%zmm21
   1b2fc:	62 91 a5 40 72 e2 34 	vpsraq $0x34,%zmm26,%zmm27
   1b303:	62 b1 cd 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm22
   1b30a:	62 c1 d5 40 db ec    	vpandq %zmm12,%zmm21,%zmm21
   1b310:	62 51 cd 40 d4 d2    	vpaddq %zmm10,%zmm22,%zmm10
   1b316:	62 a1 c5 48 d4 ed    	vpaddq %zmm21,%zmm7,%zmm21
   1b31c:	62 d1 cd 40 73 d2 34 	vpsrlq $0x34,%zmm10,%zmm22
   1b323:	62 51 ad 48 db d4    	vpandq %zmm12,%zmm10,%zmm10
   1b329:	62 31 85 48 d4 fe    	vpaddq %zmm22,%zmm15,%zmm15
   1b32f:	62 d1 cd 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm22
   1b336:	62 51 bd 48 d4 d2    	vpaddq %zmm10,%zmm8,%zmm10
   1b33c:	62 e1 cd 40 d4 f6    	vpaddq %zmm6,%zmm22,%zmm22
   1b342:	62 a1 cd 40 d4 e4    	vpaddq %zmm20,%zmm22,%zmm20
   1b348:	62 b1 8d 48 73 d4 34 	vpsrlq $0x34,%zmm20,%zmm14
   1b34f:	62 c1 dd 40 db fc    	vpandq %zmm12,%zmm20,%zmm23
   1b355:	62 61 c5 50 fb 64 24 	vpsubq 0x8(%rsp){1to8},%zmm23,%zmm28
   1b35c:	01
   1b35d:	62 31 8d 48 d4 f5    	vpaddq %zmm21,%zmm14,%zmm14
   1b363:	62 d1 dd 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm20
   1b36a:	62 c1 8d 48 db f4    	vpandq %zmm12,%zmm14,%zmm22
   1b370:	62 51 dd 40 d4 d2    	vpaddq %zmm10,%zmm20,%zmm10
   1b376:	62 d1 8d 48 73 d2 34 	vpsrlq $0x34,%zmm10,%zmm14
   1b37d:	62 c1 ad 48 db ec    	vpandq %zmm12,%zmm10,%zmm21
   1b383:	62 c1 8d 48 d4 e7    	vpaddq %zmm15,%zmm14,%zmm20
   1b389:	62 71 cd 50 fb 74 24 	vpsubq 0x10(%rsp){1to8},%zmm22,%zmm14
   1b390:	02
   1b391:	62 71 d5 50 fb 7c 24 	vpsubq 0x18(%rsp){1to8},%zmm21,%zmm15
   1b398:	03
   1b399:	62 61 dd 50 fb 4c 24 	vpsubq 0x20(%rsp){1to8},%zmm20,%zmm25
   1b3a0:	04
   1b3a1:	62 11 9d 40 d4 d3    	vpaddq %zmm27,%zmm28,%zmm10
   1b3a7:	62 d1 a5 40 72 e2 34 	vpsraq $0x34,%zmm10,%zmm27
   1b3ae:	62 11 8d 48 d4 f3    	vpaddq %zmm27,%zmm14,%zmm14
   1b3b4:	62 d1 a5 40 72 e6 34 	vpsraq $0x34,%zmm14,%zmm27
   1b3bb:	62 11 85 48 d4 fb    	vpaddq %zmm27,%zmm15,%zmm15
   1b3c1:	62 d1 a5 40 72 e7 34 	vpsraq $0x34,%zmm15,%zmm27
   1b3c8:	62 01 b5 40 d4 cb    	vpaddq %zmm27,%zmm25,%zmm25
   1b3ce:	62 b3 b5 40 1e cb 01 	vpcmpltuq %zmm19,%zmm25,%k1
   1b3d5:	62 41 ad 41 db c4    	vpandq %zmm12,%zmm26,%zmm24{%k1}
   1b3db:	62 c1 ad 49 db fc    	vpandq %zmm12,%zmm10,%zmm23{%k1}
   1b3e1:	62 c1 8d 49 db f4    	vpandq %zmm12,%zmm14,%zmm22{%k1}
   1b3e7:	62 c1 85 49 db ec    	vpandq %zmm12,%zmm15,%zmm21{%k1}
   1b3ed:	62 c1 b5 41 db e4    	vpandq %zmm12,%zmm25,%zmm20{%k1}
   1b3f3:	0f 85 f7 fa ff ff    	jne    1aef0 <probe::chain::<zopro_bn254::FqConfig, true, false>+0xd0>
   1b3f9:	62 61 fd 48 7f 06    	vmovdqa64 %zmm24,(%rsi)
   1b3ff:	62 e1 fd 48 7f 7e 01 	vmovdqa64 %zmm23,0x40(%rsi)
   1b406:	62 e1 fd 48 7f 76 02 	vmovdqa64 %zmm22,0x80(%rsi)
   1b40d:	62 e1 fd 48 7f 6e 03 	vmovdqa64 %zmm21,0xc0(%rsi)
   1b414:	62 e1 fd 48 7f 66 04 	vmovdqa64 %zmm20,0x100(%rsi)
   1b41b:	48 89 e0             	mov    %rsp,%rax
   1b41e:	62 f1 7c 48 28 46 04 	vmovaps 0x100(%rsi),%zmm0
   1b425:	62 f1 7c 48 28 2e    	vmovaps (%rsi),%zmm5
   1b42b:	62 f1 7c 48 28 4e 01 	vmovaps 0x40(%rsi),%zmm1
   1b432:	62 f1 7c 48 28 56 02 	vmovaps 0x80(%rsi),%zmm2
   1b439:	62 f1 7c 48 28 5e 03 	vmovaps 0xc0(%rsi),%zmm3
   1b440:	62 f1 7c 48 29 44 24 	vmovaps %zmm0,0x100(%rsp)
   1b447:	04
   1b448:	62 f1 7c 48 29 5c 24 	vmovaps %zmm3,0xc0(%rsp)
   1b44f:	03
   1b450:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x80(%rsp)
   1b457:	02
   1b458:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x40(%rsp)
   1b45f:	01
   1b460:	62 f1 7c 48 29 2c 24 	vmovaps %zmm5,(%rsp)
   1b467:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1b46e:	01
   1b46f:	62 f1 7c 48 28 44 24 	vmovaps 0x100(%rsp),%zmm0
   1b476:	04
   1b477:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1b47e:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1b485:	02
   1b486:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1b48d:	03
   1b48e:	62 f1 7c 48 28 2c 24 	vmovaps (%rsp),%zmm5
   1b495:	62 f1 7c 48 29 4f 01 	vmovaps %zmm1,0x40(%rdi)
   1b49c:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1b4a3:	04
   1b4a4:	62 f1 7c 48 29 5f 03 	vmovaps %zmm3,0xc0(%rdi)
   1b4ab:	62 f1 7c 48 29 57 02 	vmovaps %zmm2,0x80(%rdi)
   1b4b2:	62 f1 7c 48 29 27    	vmovaps %zmm4,(%rdi)
   1b4b8:	62 f1 7c 48 29 47 04 	vmovaps %zmm0,0x100(%rdi)
   1b4bf:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1b4c6:	02
   1b4c7:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1b4ce:	03
   1b4cf:	62 f1 7c 48 29 6f 05 	vmovaps %zmm5,0x140(%rdi)
   1b4d6:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1b4dd:	62 f1 7c 48 29 4f 09 	vmovaps %zmm1,0x240(%rdi)
   1b4e4:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1b4eb:	01
   1b4ec:	62 f1 7c 48 29 57 07 	vmovaps %zmm2,0x1c0(%rdi)
   1b4f3:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1b4fa:	02
   1b4fb:	62 f1 7c 48 29 5f 08 	vmovaps %zmm3,0x200(%rdi)
   1b502:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1b509:	03
   1b50a:	62 f1 7c 48 29 4f 06 	vmovaps %zmm1,0x180(%rdi)
   1b511:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1b518:	01
   1b519:	62 f1 7c 48 29 57 0c 	vmovaps %zmm2,0x300(%rdi)
   1b520:	62 f1 7c 48 28 54 24 	vmovaps 0x100(%rsp),%zmm2
   1b527:	04
   1b528:	62 f1 7c 48 29 67 0a 	vmovaps %zmm4,0x280(%rdi)
   1b52f:	62 f1 7c 48 29 5f 0d 	vmovaps %zmm3,0x340(%rdi)
   1b536:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1b53d:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1b544:	03
   1b545:	62 f1 7c 48 29 4f 0b 	vmovaps %zmm1,0x2c0(%rdi)
   1b54c:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1b553:	04
   1b554:	62 f1 7c 48 29 57 0e 	vmovaps %zmm2,0x380(%rdi)
   1b55b:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1b562:	02
   1b563:	62 f1 7c 48 29 5f 12 	vmovaps %zmm3,0x480(%rdi)
   1b56a:	62 f1 7c 48 29 67 0f 	vmovaps %zmm4,0x3c0(%rdi)
   1b571:	62 f1 7c 48 29 4f 13 	vmovaps %zmm1,0x4c0(%rdi)
   1b578:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1b57f:	01
   1b580:	62 f1 7c 48 29 57 11 	vmovaps %zmm2,0x440(%rdi)
   1b587:	62 f1 7c 48 29 4f 10 	vmovaps %zmm1,0x400(%rdi)
   1b58e:	48 89 ec             	mov    %rbp,%rsp
   1b591:	5d                   	pop    %rbp
   1b592:	c5 f8 77             	vzeroupper
   1b595:	c3                   	ret
   1b596:	cc                   	int3
   1b597:	cc                   	int3
   1b598:	cc                   	int3
   1b599:	cc                   	int3
   1b59a:	cc                   	int3
   1b59b:	cc                   	int3
   1b59c:	cc                   	int3
   1b59d:	cc                   	int3
   1b59e:	cc                   	int3
   1b59f:	cc                   	int3

000000000001e2e0 <probe::fused::<zopro_bn254::FqConfig, true, true>>:
   1e2e0:	55                   	push   %rbp
   1e2e1:	48 89 e5             	mov    %rsp,%rbp
   1e2e4:	48 83 e4 c0          	and    $0xffffffffffffffc0,%rsp
   1e2e8:	48 81 ec 80 05 00 00 	sub    $0x580,%rsp
   1e2ef:	62 f1 7c 48 28 52 01 	vmovaps 0x40(%rdx),%zmm2
   1e2f6:	62 f1 7c 48 28 4a 02 	vmovaps 0x80(%rdx),%zmm1
   1e2fd:	62 f1 7c 48 28 02    	vmovaps (%rdx),%zmm0
   1e303:	62 f1 7c 48 28 71 01 	vmovaps 0x40(%rcx),%zmm6
   1e30a:	62 f1 fd 48 6f 29    	vmovdqa64 (%rcx),%zmm5
   1e310:	62 61 fd 48 6f 1e    	vmovdqa64 (%rsi),%zmm27
   1e316:	62 61 fd 48 6f 7e 01 	vmovdqa64 0x40(%rsi),%zmm31
   1e31d:	62 61 fd 48 6f 76 02 	vmovdqa64 0x80(%rsi),%zmm30
   1e324:	62 61 fd 48 6f 6e 03 	vmovdqa64 0xc0(%rsi),%zmm29
   1e32b:	62 61 fd 48 6f 66 04 	vmovdqa64 0x100(%rsi),%zmm28
   1e332:	62 72 fd 48 59 35 c4 	vpbroadcastq -0x1a53c(%rip),%zmm14        # 3e00 <__abi_tag+0x3b04>
   1e339:	5a fe ff
   1e33c:	48 b8 31 e1 72 4e 64 	movabs $0x30644e72e131,%rax
   1e343:	30 00 00
   1e346:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x380(%rsp)
   1e34d:	0e
   1e34e:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x340(%rsp)
   1e355:	0d
   1e356:	62 f1 7c 48 28 52 03 	vmovaps 0xc0(%rdx),%zmm2
   1e35d:	62 f1 7c 48 28 4a 04 	vmovaps 0x100(%rdx),%zmm1
   1e364:	62 f1 7c 48 29 44 24 	vmovaps %zmm0,0x400(%rsp)
   1e36b:	10
   1e36c:	62 d1 fd 48 6f 40 04 	vmovdqa64 0x100(%r8),%zmm0
   1e373:	62 f1 7c 48 29 74 24 	vmovaps %zmm6,0x4c0(%rsp)
   1e37a:	13
   1e37b:	62 f1 fd 48 6f 71 02 	vmovdqa64 0x80(%rcx),%zmm6
   1e382:	62 f1 fd 48 7f 6c 24 	vmovdqa64 %zmm5,0x500(%rsp)
   1e389:	14
   1e38a:	62 f1 fd 48 6f 7c 24 	vmovdqa64 0x500(%rsp),%zmm7
   1e391:	14
   1e392:	62 71 fd 48 6f 44 24 	vmovdqa64 0x4c0(%rsp),%zmm8
   1e399:	13
   1e39a:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x300(%rsp)
   1e3a1:	0c
   1e3a2:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x2c0(%rsp)
   1e3a9:	0b
   1e3aa:	62 f1 7c 48 28 51 03 	vmovaps 0xc0(%rcx),%zmm2
   1e3b1:	62 f1 7c 48 28 49 04 	vmovaps 0x100(%rcx),%zmm1
   1e3b8:	62 f1 fd 48 7f 44 24 	vmovdqa64 %zmm0,0x140(%rsp)
   1e3bf:	05
   1e3c0:	c5 f9 ef c0          	vpxor  %xmm0,%xmm0,%xmm0
   1e3c4:	62 f2 d5 58 b4 05 2a 	vpmadd52luq -0x1a4d6(%rip){1to8},%zmm5,%zmm0        # 3ef8 <__abi_tag+0x3bfc>
   1e3cb:	5b fe ff
   1e3ce:	62 f1 fd 48 7f 74 24 	vmovdqa64 %zmm6,0x480(%rsp)
   1e3d5:	12
   1e3d6:	48 b9 63 c2 e5 9c c8 	movabs $0x60c89ce5c263,%rcx
   1e3dd:	60 00 00
   1e3e0:	62 f1 fd 48 6f 64 24 	vmovdqa64 0x300(%rsp),%zmm4
   1e3e7:	0c
   1e3e8:	62 71 fd 48 6f 54 24 	vmovdqa64 0x2c0(%rsp),%zmm10
   1e3ef:	0b
   1e3f0:	62 71 fd 48 6f 4c 24 	vmovdqa64 0x480(%rsp),%zmm9
   1e3f7:	12
   1e3f8:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x440(%rsp)
   1e3ff:	11
   1e400:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x3c0(%rsp)
   1e407:	0f
   1e408:	62 d1 7c 48 28 10    	vmovaps (%r8),%zmm2
   1e40e:	62 d1 7c 48 28 48 01 	vmovaps 0x40(%r8),%zmm1
   1e415:	62 f1 fd 48 7f 44 24 	vmovdqa64 %zmm0,0x200(%rsp)
   1e41c:	08
   1e41d:	62 71 fd 48 6f 6c 24 	vmovdqa64 0x200(%rsp),%zmm13
   1e424:	08
   1e425:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x280(%rsp)
   1e42c:	0a
   1e42d:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x240(%rsp)
   1e434:	09
   1e435:	62 d1 fd 48 6f 50 02 	vmovdqa64 0x80(%r8),%zmm2
   1e43c:	62 d1 fd 48 6f 48 03 	vmovdqa64 0xc0(%r8),%zmm1
   1e443:	62 f1 fd 48 7f 54 24 	vmovdqa64 %zmm2,0x1c0(%rsp)
   1e44a:	07
   1e44b:	62 f1 fd 48 7f 4c 24 	vmovdqa64 %zmm1,0x180(%rsp)
   1e452:	06
   1e453:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   1e45a:	84 00 00 00 00 00
   1e460:	62 71 fd 48 6f 64 24 	vmovdqa64 0x400(%rsp),%zmm12
   1e467:	10
   1e468:	62 f2 fd 48 59 0d 86 	vpbroadcastq -0x1a57a(%rip),%zmm1        # 3ef8 <__abi_tag+0x3bfc>
   1e46f:	5a fe ff
   1e472:	62 f1 fd 48 6f 54 24 	vmovdqa64 0x380(%rsp),%zmm2
   1e479:	0e
   1e47a:	62 f1 fd 48 6f 5c 24 	vmovdqa64 0x340(%rsp),%zmm3
   1e481:	0d
   1e482:	c5 fc 28 05 56 53 fe 	vmovaps -0x1acaa(%rip),%ymm0        # 37e0 <__abi_tag+0x34e4>
   1e489:	ff
   1e48a:	62 a1 55 00 ef ed    	vpxord %xmm21,%xmm21,%xmm21
   1e490:	62 71 fd 48 6f 5c 24 	vmovdqa64 0x280(%rsp),%zmm11
   1e497:	0a
   1e498:	c5 d1 ef ed          	vpxor  %xmm5,%xmm5,%xmm5
   1e49c:	62 a1 45 00 ef ff    	vpxord %xmm23,%xmm23,%xmm23
   1e4a2:	c5 c9 ef f6          	vpxor  %xmm6,%xmm6,%xmm6
   1e4a6:	62 a1 6d 00 ef d2    	vpxord %xmm18,%xmm18,%xmm18
   1e4ac:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1e4b1:	62 a1 75 00 ef c9    	vpxord %xmm17,%xmm17,%xmm17
   1e4b7:	62 a1 5d 00 ef e4    	vpxord %xmm20,%xmm20,%xmm20
   1e4bd:	62 a1 65 00 ef db    	vpxord %xmm19,%xmm19,%xmm19
   1e4c3:	62 a1 4d 00 ef f6    	vpxord %xmm22,%xmm22,%xmm22
   1e4c9:	62 c2 a5 40 b5 ec    	vpmadd52huq %zmm12,%zmm27,%zmm21
   1e4cf:	62 f2 a5 40 b4 e9    	vpmadd52luq %zmm1,%zmm27,%zmm5
   1e4d5:	62 c2 85 40 b5 fc    	vpmadd52huq %zmm12,%zmm31,%zmm23
   1e4db:	c5 fc 11 04 24       	vmovups %ymm0,(%rsp)
   1e4e0:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
   1e4e5:	62 c2 8d 40 b5 dc    	vpmadd52huq %zmm12,%zmm30,%zmm19
   1e4eb:	62 f2 fd 48 59 0d 43 	vpbroadcastq -0x1a6bd(%rip),%zmm1        # 3e38 <__abi_tag+0x3b3c>
   1e4f2:	59 fe ff
   1e4f5:	62 f1 fd 48 6f 44 24 	vmovdqa64 0x440(%rsp),%zmm0
   1e4fc:	11
   1e4fd:	62 c2 95 40 b5 f4    	vpmadd52huq %zmm12,%zmm29,%zmm22
   1e503:	62 e2 fd 48 59 44 24 	vpbroadcastq 0x8(%rsp),%zmm16
   1e50a:	01
   1e50b:	62 62 fd 48 59 54 24 	vpbroadcastq 0x10(%rsp),%zmm26
   1e512:	02
   1e513:	62 62 fd 48 59 4c 24 	vpbroadcastq 0x18(%rsp),%zmm25
   1e51a:	03
   1e51b:	62 62 fd 48 59 44 24 	vpbroadcastq 0x20(%rsp),%zmm24
   1e522:	04
   1e523:	62 c2 85 40 b4 ec    	vpmadd52luq %zmm12,%zmm31,%zmm21
   1e529:	62 d2 d5 48 b4 f4    	vpmadd52luq %zmm12,%zmm5,%zmm6
   1e52f:	62 e2 d5 48 b4 d2    	vpmadd52luq %zmm2,%zmm5,%zmm18
   1e535:	62 72 d5 48 b4 fb    	vpmadd52luq %zmm3,%zmm5,%zmm15
   1e53b:	62 e2 d5 48 b4 cc    	vpmadd52luq %zmm4,%zmm5,%zmm17
   1e541:	62 c2 d5 48 b4 e2    	vpmadd52luq %zmm10,%zmm5,%zmm20
   1e547:	c5 d1 ef ed          	vpxor  %xmm5,%xmm5,%xmm5
   1e54b:	62 d2 a5 40 b4 ec    	vpmadd52luq %zmm12,%zmm27,%zmm5
   1e551:	62 c2 8d 40 b4 fc    	vpmadd52luq %zmm12,%zmm30,%zmm23
   1e557:	62 c2 95 40 b4 dc    	vpmadd52luq %zmm12,%zmm29,%zmm19
   1e55d:	62 c2 9d 40 b4 f4    	vpmadd52luq %zmm12,%zmm28,%zmm22
   1e563:	62 c2 c5 48 b5 eb    	vpmadd52huq %zmm11,%zmm7,%zmm21
   1e569:	62 d2 c5 48 b4 eb    	vpmadd52luq %zmm11,%zmm7,%zmm5
   1e56f:	62 d2 95 48 b4 f3    	vpmadd52luq %zmm11,%zmm13,%zmm6
   1e575:	62 c2 bd 48 b5 fb    	vpmadd52huq %zmm11,%zmm8,%zmm23
   1e57b:	62 c2 b5 48 b5 db    	vpmadd52huq %zmm11,%zmm9,%zmm19
   1e581:	62 c2 fd 48 b5 f3    	vpmadd52huq %zmm11,%zmm0,%zmm22
   1e587:	62 c2 bd 48 b4 eb    	vpmadd52luq %zmm11,%zmm8,%zmm21
   1e58d:	62 d1 d5 48 d4 ee    	vpaddq %zmm14,%zmm5,%zmm5
   1e593:	62 d1 cd 48 db f6    	vpandq %zmm14,%zmm6,%zmm6
   1e599:	62 c2 b5 48 b4 fb    	vpmadd52luq %zmm11,%zmm9,%zmm23
   1e59f:	62 c2 fd 48 b4 db    	vpmadd52luq %zmm11,%zmm0,%zmm19
   1e5a5:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   1e5ac:	62 f2 cd 48 b5 e9    	vpmadd52huq %zmm1,%zmm6,%zmm5
   1e5b2:	62 f1 fd 48 6f 4c 24 	vmovdqa64 0x3c0(%rsp),%zmm1
   1e5b9:	0f
   1e5ba:	62 a2 cd 48 b4 e8    	vpmadd52luq %zmm16,%zmm6,%zmm21
   1e5c0:	62 a2 cd 48 b5 f8    	vpmadd52huq %zmm16,%zmm6,%zmm23
   1e5c6:	62 82 cd 48 b5 da    	vpmadd52huq %zmm26,%zmm6,%zmm19
   1e5cc:	62 f1 d5 40 d4 ed    	vpaddq %zmm5,%zmm21,%zmm5
   1e5d2:	62 a1 55 00 ef ed    	vpxord %xmm21,%xmm21,%xmm21
   1e5d8:	62 c2 9d 40 b5 ec    	vpmadd52huq %zmm12,%zmm28,%zmm21
   1e5de:	62 71 fd 48 6f 64 24 	vmovdqa64 0x240(%rsp),%zmm12
   1e5e5:	09
   1e5e6:	62 82 cd 48 b4 fa    	vpmadd52luq %zmm26,%zmm6,%zmm23
   1e5ec:	62 82 cd 48 b4 d9    	vpmadd52luq %zmm25,%zmm6,%zmm19
   1e5f2:	62 c2 f5 48 b4 f3    	vpmadd52luq %zmm11,%zmm1,%zmm22
   1e5f8:	62 e2 a5 40 b5 fa    	vpmadd52huq %zmm2,%zmm27,%zmm23
   1e5fe:	62 e2 85 40 b5 da    	vpmadd52huq %zmm2,%zmm31,%zmm19
   1e604:	62 c2 f5 48 b5 eb    	vpmadd52huq %zmm11,%zmm1,%zmm21
   1e60a:	62 72 fd 48 59 1d e4 	vpbroadcastq -0x1a71c(%rip),%zmm11        # 3ef8 <__abi_tag+0x3bfc>
   1e611:	58 fe ff
   1e614:	62 82 cd 48 b5 f1    	vpmadd52huq %zmm25,%zmm6,%zmm22
   1e61a:	62 c2 95 48 b4 d4    	vpmadd52luq %zmm12,%zmm13,%zmm18
   1e620:	62 e2 d5 58 b4 15 ce 	vpmadd52luq -0x1a732(%rip){1to8},%zmm5,%zmm18        # 3ef8 <__abi_tag+0x3bfc>
   1e627:	58 fe ff
   1e62a:	62 f2 a5 40 b4 ea    	vpmadd52luq %zmm2,%zmm27,%zmm5
   1e630:	62 e2 85 40 b4 fa    	vpmadd52luq %zmm2,%zmm31,%zmm23
   1e636:	62 e2 8d 40 b4 da    	vpmadd52luq %zmm2,%zmm30,%zmm19
   1e63c:	62 82 cd 48 b5 e8    	vpmadd52huq %zmm24,%zmm6,%zmm21
   1e642:	62 82 cd 48 b4 f0    	vpmadd52luq %zmm24,%zmm6,%zmm22
   1e648:	62 d2 c5 48 b4 ec    	vpmadd52luq %zmm12,%zmm7,%zmm5
   1e64e:	62 c2 c5 48 b5 fc    	vpmadd52huq %zmm12,%zmm7,%zmm23
   1e654:	62 c2 bd 48 b5 dc    	vpmadd52huq %zmm12,%zmm8,%zmm19
   1e65a:	62 e2 95 40 b5 ea    	vpmadd52huq %zmm2,%zmm29,%zmm21
   1e660:	62 e2 8d 40 b5 f2    	vpmadd52huq %zmm2,%zmm30,%zmm22
   1e666:	62 d1 d5 48 d4 ee    	vpaddq %zmm14,%zmm5,%zmm5
   1e66c:	62 c2 bd 48 b4 fc    	vpmadd52luq %zmm12,%zmm8,%zmm23
   1e672:	62 c2 b5 48 b4 dc    	vpmadd52luq %zmm12,%zmm9,%zmm19
   1e678:	62 e2 9d 40 b4 ea    	vpmadd52luq %zmm2,%zmm28,%zmm21
   1e67e:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   1e685:	62 d1 ed 40 db f6    	vpandq %zmm14,%zmm18,%zmm6
   1e68b:	62 f2 cd 58 b5 2d a3 	vpmadd52huq -0x1a85d(%rip){1to8},%zmm6,%zmm5        # 3e38 <__abi_tag+0x3b3c>
   1e692:	57 fe ff
   1e695:	62 e2 95 40 b4 f2    	vpmadd52luq %zmm2,%zmm29,%zmm22
   1e69b:	62 a1 6d 00 ef d2    	vpxord %xmm18,%xmm18,%xmm18
   1e6a1:	62 c2 9d 40 b5 d2    	vpmadd52huq %zmm10,%zmm28,%zmm18
   1e6a7:	62 a2 cd 48 b4 f8    	vpmadd52luq %zmm16,%zmm6,%zmm23
   1e6ad:	62 a2 cd 48 b5 d8    	vpmadd52huq %zmm16,%zmm6,%zmm19
   1e6b3:	62 c2 fd 48 b5 ec    	vpmadd52huq %zmm12,%zmm0,%zmm21
   1e6b9:	62 c2 b5 48 b5 f4    	vpmadd52huq %zmm12,%zmm9,%zmm22
   1e6bf:	62 82 cd 48 b4 da    	vpmadd52luq %zmm26,%zmm6,%zmm19
   1e6c5:	62 c2 f5 48 b4 ec    	vpmadd52luq %zmm12,%zmm1,%zmm21
   1e6cb:	62 c2 fd 48 b4 f4    	vpmadd52luq %zmm12,%zmm0,%zmm22
   1e6d1:	62 f1 c5 40 d4 ed    	vpaddq %zmm5,%zmm23,%zmm5
   1e6d7:	62 a1 45 00 ef ff    	vpxord %xmm23,%xmm23,%xmm23
   1e6dd:	62 e2 9d 40 b5 fa    	vpmadd52huq %zmm2,%zmm28,%zmm23
   1e6e3:	62 f1 fd 48 6f 54 24 	vmovdqa64 0x1c0(%rsp),%zmm2
   1e6ea:	07
   1e6eb:	62 e2 a5 40 b5 db    	vpmadd52huq %zmm3,%zmm27,%zmm19
   1e6f1:	62 82 cd 48 b5 e9    	vpmadd52huq %zmm25,%zmm6,%zmm21
   1e6f7:	62 82 cd 48 b5 f2    	vpmadd52huq %zmm26,%zmm6,%zmm22
   1e6fd:	62 c2 f5 48 b5 fc    	vpmadd52huq %zmm12,%zmm1,%zmm23
   1e703:	62 e2 85 40 b4 db    	vpmadd52luq %zmm3,%zmm31,%zmm19
   1e709:	62 82 cd 48 b4 e8    	vpmadd52luq %zmm24,%zmm6,%zmm21
   1e70f:	62 71 fd 48 6f 64 24 	vmovdqa64 0x180(%rsp),%zmm12
   1e716:	06
   1e717:	62 82 cd 48 b4 f1    	vpmadd52luq %zmm25,%zmm6,%zmm22
   1e71d:	62 72 95 48 b4 fa    	vpmadd52luq %zmm2,%zmm13,%zmm15
   1e723:	62 82 cd 48 b5 f8    	vpmadd52huq %zmm24,%zmm6,%zmm23
   1e729:	62 e2 c5 48 b5 da    	vpmadd52huq %zmm2,%zmm7,%zmm19
   1e72f:	62 e2 8d 40 b5 eb    	vpmadd52huq %zmm3,%zmm30,%zmm21
   1e735:	62 e2 85 40 b5 f3    	vpmadd52huq %zmm3,%zmm31,%zmm22
   1e73b:	62 52 d5 48 b4 fb    	vpmadd52luq %zmm11,%zmm5,%zmm15
   1e741:	62 f2 a5 40 b4 eb    	vpmadd52luq %zmm3,%zmm27,%zmm5
   1e747:	62 e2 95 40 b5 fb    	vpmadd52huq %zmm3,%zmm29,%zmm23
   1e74d:	62 e2 bd 48 b4 da    	vpmadd52luq %zmm2,%zmm8,%zmm19
   1e753:	62 e2 95 40 b4 eb    	vpmadd52luq %zmm3,%zmm29,%zmm21
   1e759:	62 c2 95 48 b4 cc    	vpmadd52luq %zmm12,%zmm13,%zmm17
   1e75f:	62 e2 8d 40 b4 f3    	vpmadd52luq %zmm3,%zmm30,%zmm22
   1e765:	62 d1 85 48 db f6    	vpandq %zmm14,%zmm15,%zmm6
   1e76b:	62 72 fd 48 59 3d c3 	vpbroadcastq -0x1a93d(%rip),%zmm15        # 3e38 <__abi_tag+0x3b3c>
   1e772:	56 fe ff
   1e775:	62 f2 c5 48 b4 ea    	vpmadd52luq %zmm2,%zmm7,%zmm5
   1e77b:	62 e2 9d 40 b4 fb    	vpmadd52luq %zmm3,%zmm28,%zmm23
   1e781:	62 e2 b5 48 b5 ea    	vpmadd52huq %zmm2,%zmm9,%zmm21
   1e787:	62 a2 cd 48 b4 d8    	vpmadd52luq %zmm16,%zmm6,%zmm19
   1e78d:	62 e2 bd 48 b5 f2    	vpmadd52huq %zmm2,%zmm8,%zmm22
   1e793:	62 d1 d5 48 d4 ee    	vpaddq %zmm14,%zmm5,%zmm5
   1e799:	62 e2 fd 48 b5 fa    	vpmadd52huq %zmm2,%zmm0,%zmm23
   1e79f:	62 e2 fd 48 b4 ea    	vpmadd52luq %zmm2,%zmm0,%zmm21
   1e7a5:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   1e7ac:	62 e2 b5 48 b4 f2    	vpmadd52luq %zmm2,%zmm9,%zmm22
   1e7b2:	62 d2 cd 48 b5 ef    	vpmadd52huq %zmm15,%zmm6,%zmm5
   1e7b8:	62 e2 f5 48 b4 fa    	vpmadd52luq %zmm2,%zmm1,%zmm23
   1e7be:	62 82 cd 48 b5 ea    	vpmadd52huq %zmm26,%zmm6,%zmm21
   1e7c4:	62 a2 cd 48 b5 f0    	vpmadd52huq %zmm16,%zmm6,%zmm22
   1e7ca:	62 f1 e5 40 d4 ed    	vpaddq %zmm5,%zmm19,%zmm5
   1e7d0:	62 a1 65 00 ef db    	vpxord %xmm19,%xmm19,%xmm19
   1e7d6:	62 e2 9d 40 b5 db    	vpmadd52huq %zmm3,%zmm28,%zmm19
   1e7dc:	62 82 cd 48 b5 f9    	vpmadd52huq %zmm25,%zmm6,%zmm23
   1e7e2:	62 82 cd 48 b4 e9    	vpmadd52luq %zmm25,%zmm6,%zmm21
   1e7e8:	62 d1 fd 48 6f db    	vmovdqa64 %zmm11,%zmm3
   1e7ee:	62 c2 d5 48 b4 cb    	vpmadd52luq %zmm11,%zmm5,%zmm17
   1e7f4:	62 f2 a5 40 b4 ec    	vpmadd52luq %zmm4,%zmm27,%zmm5
   1e7fa:	62 51 fd 48 6f df    	vmovdqa64 %zmm15,%zmm11
   1e800:	62 82 cd 48 b4 f2    	vpmadd52luq %zmm26,%zmm6,%zmm22
   1e806:	62 e2 f5 48 b5 da    	vpmadd52huq %zmm2,%zmm1,%zmm19
   1e80c:	62 82 cd 48 b4 f8    	vpmadd52luq %zmm24,%zmm6,%zmm23
   1e812:	62 e2 85 40 b5 ec    	vpmadd52huq %zmm4,%zmm31,%zmm21
   1e818:	62 f1 fd 48 6f 54 24 	vmovdqa64 0x140(%rsp),%zmm2
   1e81f:	05
   1e820:	62 d2 c5 48 b4 ec    	vpmadd52luq %zmm12,%zmm7,%zmm5
   1e826:	62 e2 a5 40 b5 f4    	vpmadd52huq %zmm4,%zmm27,%zmm22
   1e82c:	62 82 cd 48 b5 d8    	vpmadd52huq %zmm24,%zmm6,%zmm19
   1e832:	62 e2 8d 40 b5 fc    	vpmadd52huq %zmm4,%zmm30,%zmm23
   1e838:	62 e2 8d 40 b4 ec    	vpmadd52luq %zmm4,%zmm30,%zmm21
   1e83e:	62 d1 f5 40 db f6    	vpandq %zmm14,%zmm17,%zmm6
   1e844:	62 d1 d5 48 d4 ee    	vpaddq %zmm14,%zmm5,%zmm5
   1e84a:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   1e851:	62 e2 85 40 b4 f4    	vpmadd52luq %zmm4,%zmm31,%zmm22
   1e857:	62 d2 cd 48 b5 ef    	vpmadd52huq %zmm15,%zmm6,%zmm5
   1e85d:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1e862:	62 72 9d 40 b5 fc    	vpmadd52huq %zmm4,%zmm28,%zmm15
   1e868:	62 e2 95 40 b5 dc    	vpmadd52huq %zmm4,%zmm29,%zmm19
   1e86e:	62 e2 95 40 b4 fc    	vpmadd52luq %zmm4,%zmm29,%zmm23
   1e874:	62 c2 bd 48 b5 ec    	vpmadd52huq %zmm12,%zmm8,%zmm21
   1e87a:	62 e2 95 48 b4 e2    	vpmadd52luq %zmm2,%zmm13,%zmm20
   1e880:	62 e2 f5 48 b5 d2    	vpmadd52huq %zmm2,%zmm1,%zmm18
   1e886:	62 c2 c5 48 b5 f4    	vpmadd52huq %zmm12,%zmm7,%zmm22
   1e88c:	62 52 f5 48 b5 fc    	vpmadd52huq %zmm12,%zmm1,%zmm15
   1e892:	62 e2 9d 40 b4 dc    	vpmadd52luq %zmm4,%zmm28,%zmm19
   1e898:	62 c2 b5 48 b5 fc    	vpmadd52huq %zmm12,%zmm9,%zmm23
   1e89e:	62 c2 b5 48 b4 ec    	vpmadd52luq %zmm12,%zmm9,%zmm21
   1e8a4:	62 c2 bd 48 b4 f4    	vpmadd52luq %zmm12,%zmm8,%zmm22
   1e8aa:	62 12 cd 48 b5 f8    	vpmadd52huq %zmm24,%zmm6,%zmm15
   1e8b0:	62 c2 fd 48 b5 dc    	vpmadd52huq %zmm12,%zmm0,%zmm19
   1e8b6:	62 c2 fd 48 b4 fc    	vpmadd52luq %zmm12,%zmm0,%zmm23
   1e8bc:	62 a2 cd 48 b5 e8    	vpmadd52huq %zmm16,%zmm6,%zmm21
   1e8c2:	62 a2 cd 48 b4 f0    	vpmadd52luq %zmm16,%zmm6,%zmm22
   1e8c8:	62 52 95 40 b5 fa    	vpmadd52huq %zmm10,%zmm29,%zmm15
   1e8ce:	62 c2 f5 48 b4 dc    	vpmadd52luq %zmm12,%zmm1,%zmm19
   1e8d4:	62 82 cd 48 b5 fa    	vpmadd52huq %zmm26,%zmm6,%zmm23
   1e8da:	62 82 cd 48 b4 ea    	vpmadd52luq %zmm26,%zmm6,%zmm21
   1e8e0:	62 e1 cd 40 d4 cd    	vpaddq %zmm5,%zmm22,%zmm17
   1e8e6:	62 52 9d 40 b4 fa    	vpmadd52luq %zmm10,%zmm28,%zmm15
   1e8ec:	62 e2 f5 40 b4 e3    	vpmadd52luq %zmm3,%zmm17,%zmm20
   1e8f2:	62 c2 a5 40 b4 ca    	vpmadd52luq %zmm10,%zmm27,%zmm17
   1e8f8:	62 f1 dd 50 db 2d 06 	vpandq -0x1aafa(%rip){1to8},%zmm20,%zmm5        # 3e08 <__abi_tag+0x3b0c>
   1e8ff:	55 fe ff
   1e902:	62 82 cd 48 b5 d9    	vpmadd52huq %zmm25,%zmm6,%zmm19
   1e908:	62 82 cd 48 b4 f9    	vpmadd52luq %zmm25,%zmm6,%zmm23
   1e90e:	62 c2 a5 40 b5 ea    	vpmadd52huq %zmm10,%zmm27,%zmm21
   1e914:	62 72 fd 48 b5 fa    	vpmadd52huq %zmm2,%zmm0,%zmm15
   1e91a:	62 e2 c5 48 b4 ca    	vpmadd52luq %zmm2,%zmm7,%zmm17
   1e920:	62 82 cd 48 b4 d8    	vpmadd52luq %zmm24,%zmm6,%zmm19
   1e926:	62 c2 85 40 b5 fa    	vpmadd52huq %zmm10,%zmm31,%zmm23
   1e92c:	62 c2 85 40 b4 ea    	vpmadd52luq %zmm10,%zmm31,%zmm21
   1e932:	62 b1 cd 48 73 f4 04 	vpsllq $0x4,%zmm20,%zmm6
   1e939:	62 72 f5 48 b4 fa    	vpmadd52luq %zmm2,%zmm1,%zmm15
   1e93f:	62 82 d5 48 b5 d0    	vpmadd52huq %zmm24,%zmm5,%zmm18
   1e945:	62 c2 8d 40 b5 da    	vpmadd52huq %zmm10,%zmm30,%zmm19
   1e94b:	62 c2 8d 40 b4 fa    	vpmadd52luq %zmm10,%zmm30,%zmm23
   1e951:	62 e2 c5 48 b5 ea    	vpmadd52huq %zmm2,%zmm7,%zmm21
   1e957:	62 12 d5 48 b5 f9    	vpmadd52huq %zmm25,%zmm5,%zmm15
   1e95d:	62 b1 ed 40 73 f2 04 	vpsllq $0x4,%zmm18,%zmm18
   1e964:	62 c2 95 40 b4 da    	vpmadd52luq %zmm10,%zmm29,%zmm19
   1e96a:	62 e2 bd 48 b5 fa    	vpmadd52huq %zmm2,%zmm8,%zmm23
   1e970:	62 e2 bd 48 b4 ea    	vpmadd52luq %zmm2,%zmm8,%zmm21
   1e976:	62 12 d5 48 b4 f8    	vpmadd52luq %zmm24,%zmm5,%zmm15
   1e97c:	62 e2 b5 48 b5 da    	vpmadd52huq %zmm2,%zmm9,%zmm19
   1e982:	62 e2 b5 48 b4 fa    	vpmadd52luq %zmm2,%zmm9,%zmm23
   1e988:	62 a2 d5 48 b4 e8    	vpmadd52luq %zmm16,%zmm5,%zmm21
   1e98e:	62 e2 fd 48 b4 da    	vpmadd52luq %zmm2,%zmm0,%zmm19
   1e994:	62 f1 f5 50 d4 05 6a 	vpaddq -0x1ab96(%rip){1to8},%zmm17,%zmm0        # 3e08 <__abi_tag+0x3b0c>
   1e99b:	54 fe ff
   1e99e:	62 a2 d5 48 b5 f8    	vpmadd52huq %zmm16,%zmm5,%zmm23
   1e9a4:	62 b1 f5 48 73 d5 30 	vpsrlq $0x30,%zmm21,%zmm1
   1e9ab:	62 82 d5 48 b5 da    	vpmadd52huq %zmm26,%zmm5,%zmm19
   1e9b1:	62 82 d5 48 b4 fa    	vpmadd52luq %zmm26,%zmm5,%zmm23
   1e9b7:	62 82 d5 48 b4 d9    	vpmadd52luq %zmm25,%zmm5,%zmm19
   1e9bd:	62 f1 d5 48 73 d0 30 	vpsrlq $0x30,%zmm0,%zmm5
   1e9c4:	62 f2 fd 48 59 05 d2 	vpbroadcastq -0x1ab2e(%rip),%zmm0        # 3ea0 <__abi_tag+0x3ba4>
   1e9cb:	54 fe ff
   1e9ce:	62 b1 fd 40 73 f7 04 	vpsllq $0x4,%zmm23,%zmm16
   1e9d5:	62 d2 cd 48 b5 eb    	vpmadd52huq %zmm11,%zmm6,%zmm5
   1e9db:	62 b1 cd 48 73 f5 04 	vpsllq $0x4,%zmm21,%zmm6
   1e9e2:	62 b1 f5 40 73 f3 04 	vpsllq $0x4,%zmm19,%zmm17
   1e9e9:	62 b1 ed 48 73 d3 30 	vpsrlq $0x30,%zmm19,%zmm2
   1e9f0:	62 f1 cd 48 db f0    	vpandq %zmm0,%zmm6,%zmm6
   1e9f6:	62 e1 fd 40 db c0    	vpandq %zmm0,%zmm16,%zmm16
   1e9fc:	62 e1 f5 40 db c8    	vpandq %zmm0,%zmm17,%zmm17
   1ea02:	62 f1 cd 48 d4 ed    	vpaddq %zmm5,%zmm6,%zmm5
   1ea08:	62 f1 fd 40 d4 f1    	vpaddq %zmm1,%zmm16,%zmm6
   1ea0e:	62 b1 f5 48 73 d7 30 	vpsrlq $0x30,%zmm23,%zmm1
   1ea15:	62 e1 f5 40 d4 c1    	vpaddq %zmm1,%zmm17,%zmm16
   1ea1b:	62 d1 f5 48 73 f7 04 	vpsllq $0x4,%zmm15,%zmm1
   1ea22:	62 d1 85 48 73 d7 30 	vpsrlq $0x30,%zmm15,%zmm15
   1ea29:	62 41 d5 48 db de    	vpandq %zmm14,%zmm5,%zmm27
   1ea2f:	62 e1 f5 48 db d8    	vpandq %zmm0,%zmm1,%zmm19
   1ea35:	62 f1 fd 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm0
   1ea3c:	62 51 ed 40 d4 ff    	vpaddq %zmm15,%zmm18,%zmm15
   1ea42:	62 f1 fd 48 d4 f6    	vpaddq %zmm6,%zmm0,%zmm6
   1ea48:	62 e1 e5 40 d4 ca    	vpaddq %zmm2,%zmm19,%zmm17
   1ea4e:	62 f1 ed 40 73 d6 34 	vpsrlq $0x34,%zmm6,%zmm18
   1ea55:	62 41 cd 48 db fe    	vpandq %zmm14,%zmm6,%zmm31
   1ea5b:	62 a1 ed 40 d4 c0    	vpaddq %zmm16,%zmm18,%zmm16
   1ea61:	62 b1 ed 40 73 d0 34 	vpsrlq $0x34,%zmm16,%zmm18
   1ea68:	62 41 fd 40 db f6    	vpandq %zmm14,%zmm16,%zmm30
   1ea6e:	62 a1 ed 40 d4 c9    	vpaddq %zmm17,%zmm18,%zmm17
   1ea74:	62 b1 ed 40 73 d1 34 	vpsrlq $0x34,%zmm17,%zmm18
   1ea7b:	62 41 f5 40 db ee    	vpandq %zmm14,%zmm17,%zmm29
   1ea81:	62 41 ed 40 d4 e7    	vpaddq %zmm15,%zmm18,%zmm28
   1ea87:	62 f3 9d 50 1e 05 26 	vpcmpnleuq -0x1abda(%rip){1to8},%zmm28,%k0        # 3eb8 <__abi_tag+0x3bbc>
   1ea8e:	54 fe ff 06
   1ea92:	c5 f9 98 c0          	kortestb %k0,%k0
   1ea96:	75 0e                	jne    1eaa6 <probe::fused::<zopro_bn254::FqConfig, true, true>+0x7c6>
   1ea98:	49 ff c9             	dec    %r9
   1ea9b:	0f 85 bf f9 ff ff    	jne    1e460 <probe::fused::<zopro_bn254::FqConfig, true, true>+0x180>
   1eaa1:	e9 ac 00 00 00       	jmp    1eb52 <probe::fused::<zopro_bn254::FqConfig, true, true>+0x872>
   1eaa6:	c5 fc 28 05 32 51 fe 	vmovaps -0x1aece(%rip),%ymm0        # 3be0 <__abi_tag+0x38e4>
   1eaad:	ff
   1eaae:	62 e1 a5 50 d4 0d f8 	vpaddq -0x1ac08(%rip){1to8},%zmm27,%zmm17        # 3eb0 <__abi_tag+0x3bb4>
   1eab5:	53 fe ff
   1eab8:	c5 fc 11 04 24       	vmovups %ymm0,(%rsp)
   1eabd:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   1eac2:	62 b1 ed 40 72 e1 34 	vpsraq $0x34,%zmm17,%zmm18
   1eac9:	62 f2 fd 48 59 05 2d 	vpbroadcastq -0x1acd3(%rip),%zmm0        # 3e00 <__abi_tag+0x3b04>
   1ead0:	53 fe ff
   1ead3:	62 f1 85 50 fb 6c 24 	vpsubq 0x8(%rsp){1to8},%zmm31,%zmm5
   1eada:	01
   1eadb:	62 f1 8d 50 fb 74 24 	vpsubq 0x10(%rsp){1to8},%zmm30,%zmm6
   1eae2:	02
   1eae3:	62 71 95 50 fb 7c 24 	vpsubq 0x18(%rsp){1to8},%zmm29,%zmm15
   1eaea:	03
   1eaeb:	62 e1 9d 50 fb 44 24 	vpsubq 0x20(%rsp){1to8},%zmm28,%zmm16
   1eaf2:	04
   1eaf3:	62 b1 d5 48 d4 ea    	vpaddq %zmm18,%zmm5,%zmm5
   1eaf9:	62 f1 ed 40 72 e5 34 	vpsraq $0x34,%zmm5,%zmm18
   1eb00:	62 b1 cd 48 d4 f2    	vpaddq %zmm18,%zmm6,%zmm6
   1eb06:	62 f1 ed 40 72 e6 34 	vpsraq $0x34,%zmm6,%zmm18
   1eb0d:	62 31 85 48 d4 fa    	vpaddq %zmm18,%zmm15,%zmm15
   1eb13:	62 d1 ed 40 72 e7 34 	vpsraq $0x34,%zmm15,%zmm18
   1eb1a:	62 a1 fd 40 d4 c2    	vpaddq %zmm18,%zmm16,%zmm16
   1eb20:	62 f3 fd 50 1e 0d 25 	vpcmpltuq -0x1acdb(%rip){1to8},%zmm16,%k1        # 3e50 <__abi_tag+0x3b54>
   1eb27:	53 fe ff 01
   1eb2b:	62 61 f5 41 db d8    	vpandq %zmm0,%zmm17,%zmm27{%k1}
   1eb31:	62 61 d5 49 db f8    	vpandq %zmm0,%zmm5,%zmm31{%k1}
   1eb37:	62 61 cd 49 db f0    	vpandq %zmm0,%zmm6,%zmm30{%k1}
   1eb3d:	62 61 85 49 db e8    	vpandq %zmm0,%zmm15,%zmm29{%k1}
   1eb43:	62 61 fd 41 db e0    	vpandq %zmm0,%zmm16,%zmm28{%k1}
   1eb49:	49 ff c9             	dec    %r9
   1eb4c:	0f 85 0e f9 ff ff    	jne    1e460 <probe::fused::<zopro_bn254::FqConfig, true, true>+0x180>
   1eb52:	62 61 fd 48 7f 1e    	vmovdqa64 %zmm27,(%rsi)
   1eb58:	62 61 fd 48 7f 7e 01 	vmovdqa64 %zmm31,0x40(%rsi)
   1eb5f:	62 61 fd 48 7f 76 02 	vmovdqa64 %zmm30,0x80(%rsi)
   1eb66:	62 61 fd 48 7f 6e 03 	vmovdqa64 %zmm29,0xc0(%rsi)
   1eb6d:	62 61 fd 48 7f 66 04 	vmovdqa64 %zmm28,0x100(%rsi)
   1eb74:	48 89 e0             	mov    %rsp,%rax
   1eb77:	62 f1 7c 48 28 46 04 	vmovaps 0x100(%rsi),%zmm0
   1eb7e:	62 f1 7c 48 28 2e    	vmovaps (%rsi),%zmm5
   1eb84:	62 f1 7c 48 28 4e 01 	vmovaps 0x40(%rsi),%zmm1
   1eb8b:	62 f1 7c 48 28 56 02 	vmovaps 0x80(%rsi),%zmm2
   1eb92:	62 f1 7c 48 28 5e 03 	vmovaps 0xc0(%rsi),%zmm3
   1eb99:	62 f1 7c 48 29 44 24 	vmovaps %zmm0,0x100(%rsp)
   1eba0:	04
   1eba1:	62 f1 7c 48 29 5c 24 	vmovaps %zmm3,0xc0(%rsp)
   1eba8:	03
   1eba9:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x80(%rsp)
   1ebb0:	02
   1ebb1:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x40(%rsp)
   1ebb8:	01
   1ebb9:	62 f1 7c 48 29 2c 24 	vmovaps %zmm5,(%rsp)
   1ebc0:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1ebc7:	01
   1ebc8:	62 f1 7c 48 28 44 24 	vmovaps 0x100(%rsp),%zmm0
   1ebcf:	04
   1ebd0:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1ebd7:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1ebde:	02
   1ebdf:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1ebe6:	03
   1ebe7:	62 f1 7c 48 28 2c 24 	vmovaps (%rsp),%zmm5
   1ebee:	62 f1 7c 48 29 4f 01 	vmovaps %zmm1,0x40(%rdi)
   1ebf5:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1ebfc:	04
   1ebfd:	62 f1 7c 48 29 5f 03 	vmovaps %zmm3,0xc0(%rdi)
   1ec04:	62 f1 7c 48 29 57 02 	vmovaps %zmm2,0x80(%rdi)
   1ec0b:	62 f1 7c 48 29 27    	vmovaps %zmm4,(%rdi)
   1ec11:	62 f1 7c 48 29 47 04 	vmovaps %zmm0,0x100(%rdi)
   1ec18:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1ec1f:	02
   1ec20:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1ec27:	03
   1ec28:	62 f1 7c 48 29 6f 05 	vmovaps %zmm5,0x140(%rdi)
   1ec2f:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1ec36:	62 f1 7c 48 29 4f 09 	vmovaps %zmm1,0x240(%rdi)
   1ec3d:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1ec44:	01
   1ec45:	62 f1 7c 48 29 57 07 	vmovaps %zmm2,0x1c0(%rdi)
   1ec4c:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1ec53:	02
   1ec54:	62 f1 7c 48 29 5f 08 	vmovaps %zmm3,0x200(%rdi)
   1ec5b:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1ec62:	03
   1ec63:	62 f1 7c 48 29 4f 06 	vmovaps %zmm1,0x180(%rdi)
   1ec6a:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1ec71:	01
   1ec72:	62 f1 7c 48 29 57 0c 	vmovaps %zmm2,0x300(%rdi)
   1ec79:	62 f1 7c 48 28 54 24 	vmovaps 0x100(%rsp),%zmm2
   1ec80:	04
   1ec81:	62 f1 7c 48 29 67 0a 	vmovaps %zmm4,0x280(%rdi)
   1ec88:	62 f1 7c 48 29 5f 0d 	vmovaps %zmm3,0x340(%rdi)
   1ec8f:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1ec96:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1ec9d:	03
   1ec9e:	62 f1 7c 48 29 4f 0b 	vmovaps %zmm1,0x2c0(%rdi)
   1eca5:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1ecac:	04
   1ecad:	62 f1 7c 48 29 57 0e 	vmovaps %zmm2,0x380(%rdi)
   1ecb4:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1ecbb:	02
   1ecbc:	62 f1 7c 48 29 5f 12 	vmovaps %zmm3,0x480(%rdi)
   1ecc3:	62 f1 7c 48 29 67 0f 	vmovaps %zmm4,0x3c0(%rdi)
   1ecca:	62 f1 7c 48 29 4f 13 	vmovaps %zmm1,0x4c0(%rdi)
   1ecd1:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1ecd8:	01
   1ecd9:	62 f1 7c 48 29 57 11 	vmovaps %zmm2,0x440(%rdi)
   1ece0:	62 f1 7c 48 29 4f 10 	vmovaps %zmm1,0x400(%rdi)
   1ece7:	48 89 ec             	mov    %rbp,%rsp
   1ecea:	5d                   	pop    %rbp
   1eceb:	c5 f8 77             	vzeroupper
   1ecee:	c3                   	ret
   1ecef:	cc                   	int3

000000000001ecf0 <probe::fused::<zopro_bn254::FqConfig, true, false>>:
   1ecf0:	55                   	push   %rbp
   1ecf1:	48 89 e5             	mov    %rsp,%rbp
   1ecf4:	48 83 e4 c0          	and    $0xffffffffffffffc0,%rsp
   1ecf8:	48 81 ec c0 01 00 00 	sub    $0x1c0,%rsp
   1ecff:	62 f1 fd 48 6f 02    	vmovdqa64 (%rdx),%zmm0
   1ed05:	62 f1 fd 48 6f 4a 01 	vmovdqa64 0x40(%rdx),%zmm1
   1ed0c:	62 f1 fd 48 6f 52 02 	vmovdqa64 0x80(%rdx),%zmm2
   1ed13:	62 f1 fd 48 6f 5a 03 	vmovdqa64 0xc0(%rdx),%zmm3
   1ed1a:	62 f1 fd 48 6f 62 04 	vmovdqa64 0x100(%rdx),%zmm4
   1ed21:	62 f1 fd 48 6f 29    	vmovdqa64 (%rcx),%zmm5
   1ed27:	62 f1 fd 48 6f 71 01 	vmovdqa64 0x40(%rcx),%zmm6
   1ed2e:	62 f1 fd 48 6f 79 02 	vmovdqa64 0x80(%rcx),%zmm7
   1ed35:	62 71 fd 48 6f 41 03 	vmovdqa64 0xc0(%rcx),%zmm8
   1ed3c:	62 71 fd 48 6f 49 04 	vmovdqa64 0x100(%rcx),%zmm9
   1ed43:	62 51 fd 48 6f 10    	vmovdqa64 (%r8),%zmm10
   1ed49:	62 51 fd 48 6f 58 01 	vmovdqa64 0x40(%r8),%zmm11
   1ed50:	62 51 fd 48 6f 60 02 	vmovdqa64 0x80(%r8),%zmm12
   1ed57:	62 51 fd 48 6f 68 03 	vmovdqa64 0xc0(%r8),%zmm13
   1ed5e:	62 51 fd 48 6f 70 04 	vmovdqa64 0x100(%r8),%zmm14
   1ed65:	62 61 fd 48 6f 36    	vmovdqa64 (%rsi),%zmm30
   1ed6b:	62 61 fd 48 6f 6e 01 	vmovdqa64 0x40(%rsi),%zmm29
   1ed72:	62 61 fd 48 6f 66 02 	vmovdqa64 0x80(%rsi),%zmm28
   1ed79:	62 61 fd 48 6f 5e 03 	vmovdqa64 0xc0(%rsi),%zmm27
   1ed80:	62 61 fd 48 6f 56 04 	vmovdqa64 0x100(%rsi),%zmm26
   1ed87:	48 b8 31 e1 72 4e 64 	movabs $0x30644e72e131,%rax
   1ed8e:	30 00 00
   1ed91:	48 b9 63 c2 e5 9c c8 	movabs $0x60c89ce5c263,%rcx
   1ed98:	60 00 00
   1ed9b:	62 f1 fd 48 7f 44 24 	vmovdqa64 %zmm0,0x140(%rsp)
   1eda2:	05
   1eda3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   1edaa:	84 00 00 00 00 00
   1edb0:	62 f1 fd 48 6f 44 24 	vmovdqa64 0x140(%rsp),%zmm0
   1edb7:	05
   1edb8:	c5 7d 6f 3d 20 4a fe 	vmovdqa -0x1b5e0(%rip),%ymm15        # 37e0 <__abi_tag+0x34e4>
   1edbf:	ff
   1edc0:	62 a1 4d 00 ef f6    	vpxord %xmm22,%xmm22,%xmm22
   1edc6:	62 a1 65 00 ef db    	vpxord %xmm19,%xmm19,%xmm19
   1edcc:	62 a1 5d 00 ef e4    	vpxord %xmm20,%xmm20,%xmm20
   1edd2:	62 e2 fd 48 59 15 1c 	vpbroadcastq -0x1aee4(%rip),%zmm18        # 3ef8 <__abi_tag+0x3bfc>
   1edd9:	51 fe ff
   1eddc:	62 a1 55 00 ef ed    	vpxord %xmm21,%xmm21,%xmm21
   1ede2:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1ede8:	62 e2 fd 48 59 0d 46 	vpbroadcastq -0x1afba(%rip),%zmm17        # 3e38 <__abi_tag+0x3b3c>
   1edef:	50 fe ff
   1edf2:	62 e2 9d 40 b5 f0    	vpmadd52huq %zmm0,%zmm28,%zmm22
   1edf8:	c5 7e 7f 3c 24       	vmovdqu %ymm15,(%rsp)
   1edfd:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1ee02:	62 72 8d 40 b4 f8    	vpmadd52luq %zmm0,%zmm30,%zmm15
   1ee08:	62 e2 95 40 b5 d8    	vpmadd52huq %zmm0,%zmm29,%zmm19
   1ee0e:	62 e2 a5 40 b5 e0    	vpmadd52huq %zmm0,%zmm27,%zmm20
   1ee14:	62 e2 8d 40 b5 e8    	vpmadd52huq %zmm0,%zmm30,%zmm21
   1ee1a:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
   1ee1f:	62 62 fd 48 59 44 24 	vpbroadcastq 0x10(%rsp),%zmm24
   1ee26:	02
   1ee27:	62 62 fd 48 59 4c 24 	vpbroadcastq 0x8(%rsp),%zmm25
   1ee2e:	01
   1ee2f:	62 e2 fd 48 59 7c 24 	vpbroadcastq 0x18(%rsp),%zmm23
   1ee36:	03
   1ee37:	62 62 fd 48 59 7c 24 	vpbroadcastq 0x20(%rsp),%zmm31
   1ee3e:	04
   1ee3f:	62 e2 a5 40 b4 f0    	vpmadd52luq %zmm0,%zmm27,%zmm22
   1ee45:	62 e2 9d 40 b4 d8    	vpmadd52luq %zmm0,%zmm28,%zmm19
   1ee4b:	62 e2 ad 40 b4 e0    	vpmadd52luq %zmm0,%zmm26,%zmm20
   1ee51:	62 52 d5 48 b4 fa    	vpmadd52luq %zmm10,%zmm5,%zmm15
   1ee57:	62 e2 95 40 b4 e8    	vpmadd52luq %zmm0,%zmm29,%zmm21
   1ee5d:	62 c2 c5 48 b5 f2    	vpmadd52huq %zmm10,%zmm7,%zmm22
   1ee63:	62 c2 cd 48 b5 da    	vpmadd52huq %zmm10,%zmm6,%zmm19
   1ee69:	62 c2 bd 48 b5 e2    	vpmadd52huq %zmm10,%zmm8,%zmm20
   1ee6f:	62 a2 85 48 b4 c2    	vpmadd52luq %zmm18,%zmm15,%zmm16
   1ee75:	62 e2 fd 48 59 15 81 	vpbroadcastq -0x1b07f(%rip),%zmm18        # 3e00 <__abi_tag+0x3b04>
   1ee7c:	4f fe ff
   1ee7f:	62 c2 d5 48 b5 ea    	vpmadd52huq %zmm10,%zmm5,%zmm21
   1ee85:	62 c2 bd 48 b4 f2    	vpmadd52luq %zmm10,%zmm8,%zmm22
   1ee8b:	62 c2 c5 48 b4 da    	vpmadd52luq %zmm10,%zmm7,%zmm19
   1ee91:	62 c2 b5 48 b4 e2    	vpmadd52luq %zmm10,%zmm9,%zmm20
   1ee97:	62 c2 cd 48 b4 ea    	vpmadd52luq %zmm10,%zmm6,%zmm21
   1ee9d:	62 82 fd 40 b5 f0    	vpmadd52huq %zmm24,%zmm16,%zmm22
   1eea3:	62 31 85 48 d4 fa    	vpaddq %zmm18,%zmm15,%zmm15
   1eea9:	62 82 fd 40 b5 d9    	vpmadd52huq %zmm25,%zmm16,%zmm19
   1eeaf:	62 a2 fd 40 b5 e7    	vpmadd52huq %zmm23,%zmm16,%zmm20
   1eeb5:	62 82 fd 40 b4 e9    	vpmadd52luq %zmm25,%zmm16,%zmm21
   1eebb:	62 d1 85 48 73 d7 34 	vpsrlq $0x34,%zmm15,%zmm15
   1eec2:	62 32 fd 40 b5 f9    	vpmadd52huq %zmm17,%zmm16,%zmm15
   1eec8:	62 a2 fd 40 b4 f7    	vpmadd52luq %zmm23,%zmm16,%zmm22
   1eece:	62 82 fd 40 b4 d8    	vpmadd52luq %zmm24,%zmm16,%zmm19
   1eed4:	62 82 fd 40 b4 e7    	vpmadd52luq %zmm31,%zmm16,%zmm20
   1eeda:	62 c1 d5 40 d4 ef    	vpaddq %zmm15,%zmm21,%zmm21
   1eee0:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1eee5:	62 72 ad 40 b5 f8    	vpmadd52huq %zmm0,%zmm26,%zmm15
   1eeeb:	62 f2 fd 48 59 05 03 	vpbroadcastq -0x1affd(%rip),%zmm0        # 3ef8 <__abi_tag+0x3bfc>
   1eef2:	50 fe ff
   1eef5:	62 e2 8d 40 b4 e9    	vpmadd52luq %zmm1,%zmm30,%zmm21
   1eefb:	62 e2 95 40 b5 f1    	vpmadd52huq %zmm1,%zmm29,%zmm22
   1ef01:	62 e2 8d 40 b5 d9    	vpmadd52huq %zmm1,%zmm30,%zmm19
   1ef07:	62 e2 9d 40 b5 e1    	vpmadd52huq %zmm1,%zmm28,%zmm20
   1ef0d:	62 52 b5 48 b5 fa    	vpmadd52huq %zmm10,%zmm9,%zmm15
   1ef13:	62 c2 d5 48 b4 eb    	vpmadd52luq %zmm11,%zmm5,%zmm21
   1ef19:	62 e2 9d 40 b4 f1    	vpmadd52luq %zmm1,%zmm28,%zmm22
   1ef1f:	62 e2 95 40 b4 d9    	vpmadd52luq %zmm1,%zmm29,%zmm19
   1ef25:	62 e2 a5 40 b4 e1    	vpmadd52luq %zmm1,%zmm27,%zmm20
   1ef2b:	62 12 fd 40 b5 ff    	vpmadd52huq %zmm31,%zmm16,%zmm15
   1ef31:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1ef37:	62 e2 d5 40 b4 c0    	vpmadd52luq %zmm0,%zmm21,%zmm16
   1ef3d:	62 a1 d5 40 d4 ea    	vpaddq %zmm18,%zmm21,%zmm21
   1ef43:	62 c2 cd 48 b5 f3    	vpmadd52huq %zmm11,%zmm6,%zmm22
   1ef49:	62 c2 d5 48 b5 db    	vpmadd52huq %zmm11,%zmm5,%zmm19
   1ef4f:	62 c2 c5 48 b5 e3    	vpmadd52huq %zmm11,%zmm7,%zmm20
   1ef55:	62 b1 d5 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm21
   1ef5c:	62 72 a5 40 b5 f9    	vpmadd52huq %zmm1,%zmm27,%zmm15
   1ef62:	62 a2 fd 40 b5 e9    	vpmadd52huq %zmm17,%zmm16,%zmm21
   1ef68:	62 c2 c5 48 b4 f3    	vpmadd52luq %zmm11,%zmm7,%zmm22
   1ef6e:	62 c2 cd 48 b4 db    	vpmadd52luq %zmm11,%zmm6,%zmm19
   1ef74:	62 c2 bd 48 b4 e3    	vpmadd52luq %zmm11,%zmm8,%zmm20
   1ef7a:	62 72 ad 40 b4 f9    	vpmadd52luq %zmm1,%zmm26,%zmm15
   1ef80:	62 82 fd 40 b5 f1    	vpmadd52huq %zmm25,%zmm16,%zmm22
   1ef86:	62 82 fd 40 b5 e0    	vpmadd52huq %zmm24,%zmm16,%zmm20
   1ef8c:	62 82 fd 40 b4 d9    	vpmadd52luq %zmm25,%zmm16,%zmm19
   1ef92:	62 52 bd 48 b5 fb    	vpmadd52huq %zmm11,%zmm8,%zmm15
   1ef98:	62 82 fd 40 b4 f0    	vpmadd52luq %zmm24,%zmm16,%zmm22
   1ef9e:	62 a1 e5 40 d4 ed    	vpaddq %zmm21,%zmm19,%zmm21
   1efa4:	62 a1 65 00 ef db    	vpxord %xmm19,%xmm19,%xmm19
   1efaa:	62 e2 ad 40 b5 d9    	vpmadd52huq %zmm1,%zmm26,%zmm19
   1efb0:	62 a2 fd 40 b4 e7    	vpmadd52luq %zmm23,%zmm16,%zmm20
   1efb6:	62 e2 8d 40 b4 ea    	vpmadd52luq %zmm2,%zmm30,%zmm21
   1efbc:	62 52 b5 48 b4 fb    	vpmadd52luq %zmm11,%zmm9,%zmm15
   1efc2:	62 e2 8d 40 b5 f2    	vpmadd52huq %zmm2,%zmm30,%zmm22
   1efc8:	62 c2 b5 48 b5 db    	vpmadd52huq %zmm11,%zmm9,%zmm19
   1efce:	62 e2 95 40 b5 e2    	vpmadd52huq %zmm2,%zmm29,%zmm20
   1efd4:	62 c2 d5 48 b4 ec    	vpmadd52luq %zmm12,%zmm5,%zmm21
   1efda:	62 32 fd 40 b5 ff    	vpmadd52huq %zmm23,%zmm16,%zmm15
   1efe0:	62 e2 95 40 b4 f2    	vpmadd52luq %zmm2,%zmm29,%zmm22
   1efe6:	62 82 fd 40 b5 df    	vpmadd52huq %zmm31,%zmm16,%zmm19
   1efec:	62 e2 9d 40 b4 e2    	vpmadd52luq %zmm2,%zmm28,%zmm20
   1eff2:	62 12 fd 40 b4 ff    	vpmadd52luq %zmm31,%zmm16,%zmm15
   1eff8:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1effe:	62 e2 d5 40 b4 c0    	vpmadd52luq %zmm0,%zmm21,%zmm16
   1f004:	62 a1 d5 40 d4 ea    	vpaddq %zmm18,%zmm21,%zmm21
   1f00a:	62 b1 d5 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm21
   1f011:	62 c2 d5 48 b5 f4    	vpmadd52huq %zmm12,%zmm5,%zmm22
   1f017:	62 e2 a5 40 b5 da    	vpmadd52huq %zmm2,%zmm27,%zmm19
   1f01d:	62 c2 cd 48 b5 e4    	vpmadd52huq %zmm12,%zmm6,%zmm20
   1f023:	62 72 9d 40 b5 fa    	vpmadd52huq %zmm2,%zmm28,%zmm15
   1f029:	62 a2 fd 40 b5 e9    	vpmadd52huq %zmm17,%zmm16,%zmm21
   1f02f:	62 c2 cd 48 b4 f4    	vpmadd52luq %zmm12,%zmm6,%zmm22
   1f035:	62 e2 ad 40 b4 da    	vpmadd52luq %zmm2,%zmm26,%zmm19
   1f03b:	62 c2 c5 48 b4 e4    	vpmadd52luq %zmm12,%zmm7,%zmm20
   1f041:	62 72 a5 40 b4 fa    	vpmadd52luq %zmm2,%zmm27,%zmm15
   1f047:	62 82 fd 40 b4 f1    	vpmadd52luq %zmm25,%zmm16,%zmm22
   1f04d:	62 c2 bd 48 b5 dc    	vpmadd52huq %zmm12,%zmm8,%zmm19
   1f053:	62 82 fd 40 b5 e1    	vpmadd52huq %zmm25,%zmm16,%zmm20
   1f059:	62 52 c5 48 b5 fc    	vpmadd52huq %zmm12,%zmm7,%zmm15
   1f05f:	62 a1 cd 40 d4 ed    	vpaddq %zmm21,%zmm22,%zmm21
   1f065:	62 a1 4d 00 ef f6    	vpxord %xmm22,%xmm22,%xmm22
   1f06b:	62 e2 ad 40 b5 f2    	vpmadd52huq %zmm2,%zmm26,%zmm22
   1f071:	62 c2 b5 48 b4 dc    	vpmadd52luq %zmm12,%zmm9,%zmm19
   1f077:	62 82 fd 40 b4 e0    	vpmadd52luq %zmm24,%zmm16,%zmm20
   1f07d:	62 e2 8d 40 b4 eb    	vpmadd52luq %zmm3,%zmm30,%zmm21
   1f083:	62 52 bd 48 b4 fc    	vpmadd52luq %zmm12,%zmm8,%zmm15
   1f089:	62 c2 b5 48 b5 f4    	vpmadd52huq %zmm12,%zmm9,%zmm22
   1f08f:	62 a2 fd 40 b5 df    	vpmadd52huq %zmm23,%zmm16,%zmm19
   1f095:	62 e2 8d 40 b5 e3    	vpmadd52huq %zmm3,%zmm30,%zmm20
   1f09b:	62 c2 d5 48 b4 ed    	vpmadd52luq %zmm13,%zmm5,%zmm21
   1f0a1:	62 12 fd 40 b5 f8    	vpmadd52huq %zmm24,%zmm16,%zmm15
   1f0a7:	62 82 fd 40 b5 f7    	vpmadd52huq %zmm31,%zmm16,%zmm22
   1f0ad:	62 82 fd 40 b4 df    	vpmadd52luq %zmm31,%zmm16,%zmm19
   1f0b3:	62 e2 95 40 b4 e3    	vpmadd52luq %zmm3,%zmm29,%zmm20
   1f0b9:	62 32 fd 40 b4 ff    	vpmadd52luq %zmm23,%zmm16,%zmm15
   1f0bf:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1f0c5:	62 e2 d5 40 b4 c0    	vpmadd52luq %zmm0,%zmm21,%zmm16
   1f0cb:	62 a1 d5 40 d4 ea    	vpaddq %zmm18,%zmm21,%zmm21
   1f0d1:	62 b1 d5 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm21
   1f0d8:	62 e2 a5 40 b5 f3    	vpmadd52huq %zmm3,%zmm27,%zmm22
   1f0de:	62 e2 9d 40 b5 db    	vpmadd52huq %zmm3,%zmm28,%zmm19
   1f0e4:	62 c2 d5 48 b5 e5    	vpmadd52huq %zmm13,%zmm5,%zmm20
   1f0ea:	62 72 95 40 b5 fb    	vpmadd52huq %zmm3,%zmm29,%zmm15
   1f0f0:	62 a2 fd 40 b5 e9    	vpmadd52huq %zmm17,%zmm16,%zmm21
   1f0f6:	62 e2 ad 40 b4 f3    	vpmadd52luq %zmm3,%zmm26,%zmm22
   1f0fc:	62 e2 a5 40 b4 db    	vpmadd52luq %zmm3,%zmm27,%zmm19
   1f102:	62 c2 cd 48 b4 e5    	vpmadd52luq %zmm13,%zmm6,%zmm20
   1f108:	62 72 9d 40 b4 fb    	vpmadd52luq %zmm3,%zmm28,%zmm15
   1f10e:	62 c2 bd 48 b5 f5    	vpmadd52huq %zmm13,%zmm8,%zmm22
   1f114:	62 c2 c5 48 b5 dd    	vpmadd52huq %zmm13,%zmm7,%zmm19
   1f11a:	62 82 fd 40 b4 e1    	vpmadd52luq %zmm25,%zmm16,%zmm20
   1f120:	62 52 cd 48 b5 fd    	vpmadd52huq %zmm13,%zmm6,%zmm15
   1f126:	62 c2 b5 48 b4 f5    	vpmadd52luq %zmm13,%zmm9,%zmm22
   1f12c:	62 a1 dd 40 d4 ed    	vpaddq %zmm21,%zmm20,%zmm21
   1f132:	62 a1 5d 00 ef e4    	vpxord %xmm20,%xmm20,%xmm20
   1f138:	62 e2 ad 40 b5 e3    	vpmadd52huq %zmm3,%zmm26,%zmm20
   1f13e:	62 c2 bd 48 b4 dd    	vpmadd52luq %zmm13,%zmm8,%zmm19
   1f144:	62 e2 8d 40 b4 ec    	vpmadd52luq %zmm4,%zmm30,%zmm21
   1f14a:	62 52 c5 48 b4 fd    	vpmadd52luq %zmm13,%zmm7,%zmm15
   1f150:	62 a2 fd 40 b5 f7    	vpmadd52huq %zmm23,%zmm16,%zmm22
   1f156:	62 c2 b5 48 b5 e5    	vpmadd52huq %zmm13,%zmm9,%zmm20
   1f15c:	62 82 fd 40 b5 d8    	vpmadd52huq %zmm24,%zmm16,%zmm19
   1f162:	62 c2 d5 48 b4 ee    	vpmadd52luq %zmm14,%zmm5,%zmm21
   1f168:	62 12 fd 40 b5 f9    	vpmadd52huq %zmm25,%zmm16,%zmm15
   1f16e:	62 82 fd 40 b4 f7    	vpmadd52luq %zmm31,%zmm16,%zmm22
   1f174:	62 a2 fd 40 b4 df    	vpmadd52luq %zmm23,%zmm16,%zmm19
   1f17a:	62 82 fd 40 b5 e7    	vpmadd52huq %zmm31,%zmm16,%zmm20
   1f180:	62 12 fd 40 b4 f8    	vpmadd52luq %zmm24,%zmm16,%zmm15
   1f186:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1f18c:	62 e2 d5 40 b4 c0    	vpmadd52luq %zmm0,%zmm21,%zmm16
   1f192:	62 e1 d5 50 d4 2d 6c 	vpaddq -0x1b394(%rip){1to8},%zmm21,%zmm21        # 3e08 <__abi_tag+0x3b0c>
   1f199:	4c fe ff
   1f19c:	62 e2 9d 40 b5 f4    	vpmadd52huq %zmm4,%zmm28,%zmm22
   1f1a2:	62 e2 95 40 b5 dc    	vpmadd52huq %zmm4,%zmm29,%zmm19
   1f1a8:	62 e2 a5 40 b5 e4    	vpmadd52huq %zmm4,%zmm27,%zmm20
   1f1ae:	62 72 8d 40 b5 fc    	vpmadd52huq %zmm4,%zmm30,%zmm15
   1f1b4:	62 e2 a5 40 b4 f4    	vpmadd52luq %zmm4,%zmm27,%zmm22
   1f1ba:	62 e2 9d 40 b4 dc    	vpmadd52luq %zmm4,%zmm28,%zmm19
   1f1c0:	62 e2 ad 40 b4 e4    	vpmadd52luq %zmm4,%zmm26,%zmm20
   1f1c6:	62 01 25 00 ef db    	vpxord %xmm27,%xmm27,%xmm27
   1f1cc:	62 62 ad 40 b5 dc    	vpmadd52huq %zmm4,%zmm26,%zmm27
   1f1d2:	62 b1 ad 40 73 f0 04 	vpsllq $0x4,%zmm16,%zmm26
   1f1d9:	62 e1 fd 50 db 05 25 	vpandq -0x1b3db(%rip){1to8},%zmm16,%zmm16        # 3e08 <__abi_tag+0x3b0c>
   1f1e0:	4c fe ff
   1f1e3:	62 72 95 40 b4 fc    	vpmadd52luq %zmm4,%zmm29,%zmm15
   1f1e9:	62 b1 d5 40 73 d5 30 	vpsrlq $0x30,%zmm21,%zmm21
   1f1f0:	62 a2 ad 40 b5 e9    	vpmadd52huq %zmm17,%zmm26,%zmm21
   1f1f6:	62 e2 fd 48 59 0d a0 	vpbroadcastq -0x1b360(%rip),%zmm17        # 3ea0 <__abi_tag+0x3ba4>
   1f1fd:	4c fe ff
   1f200:	62 c2 c5 48 b5 f6    	vpmadd52huq %zmm14,%zmm7,%zmm22
   1f206:	62 c2 cd 48 b5 de    	vpmadd52huq %zmm14,%zmm6,%zmm19
   1f20c:	62 c2 bd 48 b5 e6    	vpmadd52huq %zmm14,%zmm8,%zmm20
   1f212:	62 42 b5 48 b5 de    	vpmadd52huq %zmm14,%zmm9,%zmm27
   1f218:	62 52 d5 48 b5 fe    	vpmadd52huq %zmm14,%zmm5,%zmm15
   1f21e:	62 c2 bd 48 b4 f6    	vpmadd52luq %zmm14,%zmm8,%zmm22
   1f224:	62 c2 c5 48 b4 de    	vpmadd52luq %zmm14,%zmm7,%zmm19
   1f22a:	62 c2 b5 48 b4 e6    	vpmadd52luq %zmm14,%zmm9,%zmm20
   1f230:	62 02 fd 40 b5 df    	vpmadd52huq %zmm31,%zmm16,%zmm27
   1f236:	62 52 cd 48 b4 fe    	vpmadd52luq %zmm14,%zmm6,%zmm15
   1f23c:	62 82 fd 40 b5 f0    	vpmadd52huq %zmm24,%zmm16,%zmm22
   1f242:	62 82 fd 40 b5 d9    	vpmadd52huq %zmm25,%zmm16,%zmm19
   1f248:	62 a2 fd 40 b5 e7    	vpmadd52huq %zmm23,%zmm16,%zmm20
   1f24e:	62 12 fd 40 b4 f9    	vpmadd52luq %zmm25,%zmm16,%zmm15
   1f254:	62 a2 fd 40 b4 f7    	vpmadd52luq %zmm23,%zmm16,%zmm22
   1f25a:	62 82 fd 40 b4 d8    	vpmadd52luq %zmm24,%zmm16,%zmm19
   1f260:	62 82 fd 40 b4 e7    	vpmadd52luq %zmm31,%zmm16,%zmm20
   1f266:	62 91 bd 40 73 f3 04 	vpsllq $0x4,%zmm27,%zmm24
   1f26d:	62 d1 fd 40 73 f7 04 	vpsllq $0x4,%zmm15,%zmm16
   1f274:	62 d1 85 48 73 d7 30 	vpsrlq $0x30,%zmm15,%zmm15
   1f27b:	62 a1 fd 40 db c1    	vpandq %zmm17,%zmm16,%zmm16
   1f281:	62 b1 c5 40 73 f6 04 	vpsllq $0x4,%zmm22,%zmm23
   1f288:	62 a1 fd 40 d4 c5    	vpaddq %zmm21,%zmm16,%zmm16
   1f28e:	62 b1 d5 40 73 d3 30 	vpsrlq $0x30,%zmm19,%zmm21
   1f295:	62 b1 fd 48 73 f4 04 	vpsllq $0x4,%zmm20,%zmm0
   1f29c:	62 b1 e5 40 73 f3 04 	vpsllq $0x4,%zmm19,%zmm19
   1f2a3:	62 b1 cd 40 73 d6 30 	vpsrlq $0x30,%zmm22,%zmm22
   1f2aa:	62 b1 dd 40 73 d4 30 	vpsrlq $0x30,%zmm20,%zmm20
   1f2b1:	62 a1 c5 40 db f9    	vpandq %zmm17,%zmm23,%zmm23
   1f2b7:	62 a1 e5 40 db d9    	vpandq %zmm17,%zmm19,%zmm19
   1f2bd:	62 a1 bd 40 d4 e4    	vpaddq %zmm20,%zmm24,%zmm20
   1f2c3:	62 21 fd 40 db f2    	vpandq %zmm18,%zmm16,%zmm30
   1f2c9:	62 a1 c5 40 d4 ed    	vpaddq %zmm21,%zmm23,%zmm21
   1f2cf:	62 a1 fd 48 db f9    	vpandq %zmm17,%zmm0,%zmm23
   1f2d5:	62 b1 fd 48 73 d0 34 	vpsrlq $0x34,%zmm16,%zmm0
   1f2dc:	62 51 fd 48 d4 ff    	vpaddq %zmm15,%zmm0,%zmm15
   1f2e2:	62 a1 c5 40 d4 f6    	vpaddq %zmm22,%zmm23,%zmm22
   1f2e8:	62 31 85 48 d4 fb    	vpaddq %zmm19,%zmm15,%zmm15
   1f2ee:	62 d1 e5 40 73 d7 34 	vpsrlq $0x34,%zmm15,%zmm19
   1f2f5:	62 21 85 48 db ea    	vpandq %zmm18,%zmm15,%zmm29
   1f2fb:	62 a1 e5 40 d4 dd    	vpaddq %zmm21,%zmm19,%zmm19
   1f301:	62 b1 d5 40 73 d3 34 	vpsrlq $0x34,%zmm19,%zmm21
   1f308:	62 21 e5 40 db e2    	vpandq %zmm18,%zmm19,%zmm28
   1f30e:	62 a1 d5 40 d4 ee    	vpaddq %zmm22,%zmm21,%zmm21
   1f314:	62 b1 cd 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm22
   1f31b:	62 21 d5 40 db da    	vpandq %zmm18,%zmm21,%zmm27
   1f321:	62 21 cd 40 d4 d4    	vpaddq %zmm20,%zmm22,%zmm26
   1f327:	62 f3 ad 50 1e 05 86 	vpcmpnleuq -0x1b47a(%rip){1to8},%zmm26,%k0        # 3eb8 <__abi_tag+0x3bbc>
   1f32e:	4b fe ff 06
   1f332:	c5 f9 98 c0          	kortestb %k0,%k0
   1f336:	75 0e                	jne    1f346 <probe::fused::<zopro_bn254::FqConfig, true, false>+0x656>
   1f338:	49 ff c9             	dec    %r9
   1f33b:	0f 85 6f fa ff ff    	jne    1edb0 <probe::fused::<zopro_bn254::FqConfig, true, false>+0xc0>
   1f341:	e9 ac 00 00 00       	jmp    1f3f2 <probe::fused::<zopro_bn254::FqConfig, true, false>+0x702>
   1f346:	c5 7c 28 3d 92 48 fe 	vmovaps -0x1b76e(%rip),%ymm15        # 3be0 <__abi_tag+0x38e4>
   1f34d:	ff
   1f34e:	62 e1 8d 50 d4 2d 58 	vpaddq -0x1b4a8(%rip){1to8},%zmm30,%zmm21        # 3eb0 <__abi_tag+0x3bb4>
   1f355:	4b fe ff
   1f358:	62 e2 fd 48 59 0d 9e 	vpbroadcastq -0x1b562(%rip),%zmm17        # 3e00 <__abi_tag+0x3b04>
   1f35f:	4a fe ff
   1f362:	c5 7c 11 3c 24       	vmovups %ymm15,(%rsp)
   1f367:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   1f36c:	62 b1 cd 40 72 e5 34 	vpsraq $0x34,%zmm21,%zmm22
   1f373:	62 71 95 50 fb 7c 24 	vpsubq 0x8(%rsp){1to8},%zmm29,%zmm15
   1f37a:	01
   1f37b:	62 e1 9d 50 fb 44 24 	vpsubq 0x10(%rsp){1to8},%zmm28,%zmm16
   1f382:	02
   1f383:	62 e1 a5 50 fb 5c 24 	vpsubq 0x18(%rsp){1to8},%zmm27,%zmm19
   1f38a:	03
   1f38b:	62 e1 ad 50 fb 64 24 	vpsubq 0x20(%rsp){1to8},%zmm26,%zmm20
   1f392:	04
   1f393:	62 31 85 48 d4 fe    	vpaddq %zmm22,%zmm15,%zmm15
   1f399:	62 d1 cd 40 72 e7 34 	vpsraq $0x34,%zmm15,%zmm22
   1f3a0:	62 a1 fd 40 d4 c6    	vpaddq %zmm22,%zmm16,%zmm16
   1f3a6:	62 b1 cd 40 72 e0 34 	vpsraq $0x34,%zmm16,%zmm22
   1f3ad:	62 a1 e5 40 d4 de    	vpaddq %zmm22,%zmm19,%zmm19
   1f3b3:	62 b1 cd 40 72 e3 34 	vpsraq $0x34,%zmm19,%zmm22
   1f3ba:	62 a1 dd 40 d4 e6    	vpaddq %zmm22,%zmm20,%zmm20
   1f3c0:	62 f3 dd 50 1e 0d 85 	vpcmpltuq -0x1b57b(%rip){1to8},%zmm20,%k1        # 3e50 <__abi_tag+0x3b54>
   1f3c7:	4a fe ff 01
   1f3cb:	62 21 d5 41 db f1    	vpandq %zmm17,%zmm21,%zmm30{%k1}
   1f3d1:	62 21 85 49 db e9    	vpandq %zmm17,%zmm15,%zmm29{%k1}
   1f3d7:	62 21 fd 41 db e1    	vpandq %zmm17,%zmm16,%zmm28{%k1}
   1f3dd:	62 21 e5 41 db d9    	vpandq %zmm17,%zmm19,%zmm27{%k1}
   1f3e3:	62 21 dd 41 db d1    	vpandq %zmm17,%zmm20,%zmm26{%k1}
   1f3e9:	49 ff c9             	dec    %r9
   1f3ec:	0f 85 be f9 ff ff    	jne    1edb0 <probe::fused::<zopro_bn254::FqConfig, true, false>+0xc0>
   1f3f2:	62 61 fd 48 7f 36    	vmovdqa64 %zmm30,(%rsi)
   1f3f8:	62 61 fd 48 7f 6e 01 	vmovdqa64 %zmm29,0x40(%rsi)
   1f3ff:	62 61 fd 48 7f 66 02 	vmovdqa64 %zmm28,0x80(%rsi)
   1f406:	62 61 fd 48 7f 5e 03 	vmovdqa64 %zmm27,0xc0(%rsi)
   1f40d:	62 61 fd 48 7f 56 04 	vmovdqa64 %zmm26,0x100(%rsi)
   1f414:	48 89 e0             	mov    %rsp,%rax
   1f417:	62 f1 7c 48 28 46 04 	vmovaps 0x100(%rsi),%zmm0
   1f41e:	62 f1 7c 48 28 2e    	vmovaps (%rsi),%zmm5
   1f424:	62 f1 7c 48 28 4e 01 	vmovaps 0x40(%rsi),%zmm1
   1f42b:	62 f1 7c 48 28 56 02 	vmovaps 0x80(%rsi),%zmm2
   1f432:	62 f1 7c 48 28 5e 03 	vmovaps 0xc0(%rsi),%zmm3
   1f439:	62 f1 7c 48 29 44 24 	vmovaps %zmm0,0x100(%rsp)
   1f440:	04
   1f441:	62 f1 7c 48 29 5c 24 	vmovaps %zmm3,0xc0(%rsp)
   1f448:	03
   1f449:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x80(%rsp)
   1f450:	02
   1f451:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x40(%rsp)
   1f458:	01
   1f459:	62 f1 7c 48 29 2c 24 	vmovaps %zmm5,(%rsp)
   1f460:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1f467:	01
   1f468:	62 f1 7c 48 28 44 24 	vmovaps 0x100(%rsp),%zmm0
   1f46f:	04
   1f470:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1f477:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1f47e:	02
   1f47f:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1f486:	03
   1f487:	62 f1 7c 48 28 2c 24 	vmovaps (%rsp),%zmm5
   1f48e:	62 f1 7c 48 29 4f 01 	vmovaps %zmm1,0x40(%rdi)
   1f495:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1f49c:	04
   1f49d:	62 f1 7c 48 29 5f 03 	vmovaps %zmm3,0xc0(%rdi)
   1f4a4:	62 f1 7c 48 29 57 02 	vmovaps %zmm2,0x80(%rdi)
   1f4ab:	62 f1 7c 48 29 27    	vmovaps %zmm4,(%rdi)
   1f4b1:	62 f1 7c 48 29 47 04 	vmovaps %zmm0,0x100(%rdi)
   1f4b8:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1f4bf:	02
   1f4c0:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1f4c7:	03
   1f4c8:	62 f1 7c 48 29 6f 05 	vmovaps %zmm5,0x140(%rdi)
   1f4cf:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1f4d6:	62 f1 7c 48 29 4f 09 	vmovaps %zmm1,0x240(%rdi)
   1f4dd:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1f4e4:	01
   1f4e5:	62 f1 7c 48 29 57 07 	vmovaps %zmm2,0x1c0(%rdi)
   1f4ec:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1f4f3:	02
   1f4f4:	62 f1 7c 48 29 5f 08 	vmovaps %zmm3,0x200(%rdi)
   1f4fb:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1f502:	03
   1f503:	62 f1 7c 48 29 4f 06 	vmovaps %zmm1,0x180(%rdi)
   1f50a:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1f511:	01
   1f512:	62 f1 7c 48 29 57 0c 	vmovaps %zmm2,0x300(%rdi)
   1f519:	62 f1 7c 48 28 54 24 	vmovaps 0x100(%rsp),%zmm2
   1f520:	04
   1f521:	62 f1 7c 48 29 67 0a 	vmovaps %zmm4,0x280(%rdi)
   1f528:	62 f1 7c 48 29 5f 0d 	vmovaps %zmm3,0x340(%rdi)
   1f52f:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1f536:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1f53d:	03
   1f53e:	62 f1 7c 48 29 4f 0b 	vmovaps %zmm1,0x2c0(%rdi)
   1f545:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1f54c:	04
   1f54d:	62 f1 7c 48 29 57 0e 	vmovaps %zmm2,0x380(%rdi)
   1f554:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1f55b:	02
   1f55c:	62 f1 7c 48 29 5f 12 	vmovaps %zmm3,0x480(%rdi)
   1f563:	62 f1 7c 48 29 67 0f 	vmovaps %zmm4,0x3c0(%rdi)
   1f56a:	62 f1 7c 48 29 4f 13 	vmovaps %zmm1,0x4c0(%rdi)
   1f571:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1f578:	01
   1f579:	62 f1 7c 48 29 57 11 	vmovaps %zmm2,0x440(%rdi)
   1f580:	62 f1 7c 48 29 4f 10 	vmovaps %zmm1,0x400(%rdi)
   1f587:	48 89 ec             	mov    %rbp,%rsp
   1f58a:	5d                   	pop    %rbp
   1f58b:	c5 f8 77             	vzeroupper
   1f58e:	c3                   	ret
   1f58f:	cc                   	int3

0000000000022f60 <probe::streams::<zopro_bn254::FqConfig, true, true>>:
   22f60:	53                   	push   %rbx
   22f61:	48 83 ec 70          	sub    $0x70,%rsp
   22f65:	62 f1 fd 48 6f 31    	vmovdqa64 (%rcx),%zmm6
   22f6b:	62 f1 fd 48 6f 02    	vmovdqa64 (%rdx),%zmm0
   22f71:	62 f1 fd 48 6f 4a 01 	vmovdqa64 0x40(%rdx),%zmm1
   22f78:	62 f1 fd 48 6f 52 02 	vmovdqa64 0x80(%rdx),%zmm2
   22f7f:	62 f1 fd 48 6f 5a 03 	vmovdqa64 0xc0(%rdx),%zmm3
   22f86:	62 f1 fd 48 6f 62 04 	vmovdqa64 0x100(%rdx),%zmm4
   22f8d:	62 f1 fd 48 6f 79 01 	vmovdqa64 0x40(%rcx),%zmm7
   22f94:	62 71 fd 48 6f 41 02 	vmovdqa64 0x80(%rcx),%zmm8
   22f9b:	62 71 fd 48 6f 49 03 	vmovdqa64 0xc0(%rcx),%zmm9
   22fa2:	62 e1 fd 48 6f 41 04 	vmovdqa64 0x100(%rcx),%zmm16
   22fa9:	c5 fd 6f 2d 2f 08 fe 	vmovdqa -0x1f7d1(%rip),%ymm5        # 37e0 <__abi_tag+0x34e4>
   22fb0:	ff
   22fb1:	62 72 fd 48 59 2d 3d 	vpbroadcastq -0x1f0c3(%rip),%zmm13        # 3ef8 <__abi_tag+0x3bfc>
   22fb8:	0f fe ff
   22fbb:	62 72 fd 48 59 25 3b 	vpbroadcastq -0x1f1c5(%rip),%zmm12        # 3e00 <__abi_tag+0x3b04>
   22fc2:	0e fe ff
   22fc5:	62 62 fd 48 59 05 69 	vpbroadcastq -0x1f197(%rip),%zmm24        # 3e38 <__abi_tag+0x3b3c>
   22fcc:	0e fe ff
   22fcf:	62 72 7d 48 5a 1d 57 	vbroadcasti32x4 -0x1e5a9(%rip),%zmm11        # 4a30 <__abi_tag+0x4734>
   22fd6:	1a fe ff
   22fd9:	48 89 fb             	mov    %rdi,%rbx
   22fdc:	31 c0                	xor    %eax,%eax
   22fde:	48 b9 31 e1 72 4e 64 	movabs $0x30644e72e131,%rcx
   22fe5:	30 00 00
   22fe8:	48 ba 63 c2 e5 9c c8 	movabs $0x60c89ce5c263,%rdx
   22fef:	60 00 00
   22ff2:	62 f1 fe 48 7f b4 24 	vmovdqu64 %zmm6,0x30(%rsp)
   22ff9:	30 00 00 00
   22ffd:	0f 1f 00             	nopl   (%rax)
   23000:	bf 00 01 00 00       	mov    $0x100,%edi
   23005:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   2300c:	00 00 00 00
   23010:	62 61 fd 48 6f 6c 3e 	vmovdqa64 -0x100(%rsi,%rdi,1),%zmm29
   23017:	fc
   23018:	62 61 fd 48 6f 54 3e 	vmovdqa64 -0x80(%rsi,%rdi,1),%zmm26
   2301f:	fe
   23020:	62 61 fd 48 6f 5c 3e 	vmovdqa64 -0xc0(%rsi,%rdi,1),%zmm27
   23027:	fd
   23028:	62 61 fd 48 6f 4c 3e 	vmovdqa64 -0x40(%rsi,%rdi,1),%zmm25
   2302f:	ff
   23030:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   23035:	62 91 fd 48 6f f0    	vmovdqa64 %zmm24,%zmm6
   2303b:	62 61 fd 48 6f 04 3e 	vmovdqa64 (%rsi,%rdi,1),%zmm24
   23042:	c5 fe 7f 2c 24       	vmovdqu %ymm5,(%rsp)
   23047:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   2304c:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   23051:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   23057:	62 a1 6d 00 ef d2    	vpxord %xmm18,%xmm18,%xmm18
   2305d:	62 a1 65 00 ef db    	vpxord %xmm19,%xmm19,%xmm19
   23063:	62 a1 75 00 ef c9    	vpxord %xmm17,%xmm17,%xmm17
   23069:	c4 41 09 ef f6       	vpxor  %xmm14,%xmm14,%xmm14
   2306e:	62 01 1d 00 ef e4    	vpxord %xmm28,%xmm28,%xmm28
   23074:	c5 d1 ef ed          	vpxor  %xmm5,%xmm5,%xmm5
   23078:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   2307e:	62 e2 fd 48 59 74 24 	vpbroadcastq 0x10(%rsp),%zmm22
   23085:	02
   23086:	62 e2 fd 48 59 7c 24 	vpbroadcastq 0x8(%rsp),%zmm23
   2308d:	01
   2308e:	62 e2 fd 48 59 6c 24 	vpbroadcastq 0x18(%rsp),%zmm21
   23095:	03
   23096:	62 e2 fd 48 59 64 24 	vpbroadcastq 0x20(%rsp),%zmm20
   2309d:	04
   2309e:	62 52 95 40 b4 d5    	vpmadd52luq %zmm13,%zmm29,%zmm10
   230a4:	62 72 ad 40 b5 f8    	vpmadd52huq %zmm0,%zmm26,%zmm15
   230aa:	62 62 a5 40 b5 f8    	vpmadd52huq %zmm0,%zmm27,%zmm31
   230b0:	62 f2 95 40 b5 e8    	vpmadd52huq %zmm0,%zmm29,%zmm5
   230b6:	62 62 95 40 b4 f0    	vpmadd52luq %zmm0,%zmm29,%zmm30
   230bc:	62 e2 ad 48 b4 d0    	vpmadd52luq %zmm0,%zmm10,%zmm18
   230c2:	62 e2 ad 48 b4 d9    	vpmadd52luq %zmm1,%zmm10,%zmm19
   230c8:	62 e2 ad 48 b4 ca    	vpmadd52luq %zmm2,%zmm10,%zmm17
   230ce:	62 72 ad 48 b4 f3    	vpmadd52luq %zmm3,%zmm10,%zmm14
   230d4:	62 62 ad 48 b4 e4    	vpmadd52luq %zmm4,%zmm10,%zmm28
   230da:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   230df:	62 72 b5 40 b5 d0    	vpmadd52huq %zmm0,%zmm25,%zmm10
   230e5:	62 72 b5 40 b4 f8    	vpmadd52luq %zmm0,%zmm25,%zmm15
   230eb:	62 62 ad 40 b4 f8    	vpmadd52luq %zmm0,%zmm26,%zmm31
   230f1:	62 f2 a5 40 b4 e8    	vpmadd52luq %zmm0,%zmm27,%zmm5
   230f7:	62 41 8d 40 d4 f4    	vpaddq %zmm12,%zmm30,%zmm30
   230fd:	62 91 8d 40 73 d6 34 	vpsrlq $0x34,%zmm30,%zmm30
   23104:	62 72 bd 40 b4 d0    	vpmadd52luq %zmm0,%zmm24,%zmm10
   2310a:	62 32 ed 40 b5 fe    	vpmadd52huq %zmm22,%zmm18,%zmm15
   23110:	62 22 ed 40 b5 ff    	vpmadd52huq %zmm23,%zmm18,%zmm31
   23116:	62 62 ed 40 b5 f6    	vpmadd52huq %zmm6,%zmm18,%zmm30
   2311c:	62 b2 ed 40 b4 ef    	vpmadd52luq %zmm23,%zmm18,%zmm5
   23122:	62 32 ed 40 b5 d5    	vpmadd52huq %zmm21,%zmm18,%zmm10
   23128:	62 32 ed 40 b4 fd    	vpmadd52luq %zmm21,%zmm18,%zmm15
   2312e:	62 22 ed 40 b4 fe    	vpmadd52luq %zmm22,%zmm18,%zmm31
   23134:	62 91 d5 48 d4 ee    	vpaddq %zmm30,%zmm5,%zmm5
   2313a:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   23140:	62 62 bd 40 b5 f0    	vpmadd52huq %zmm0,%zmm24,%zmm30
   23146:	62 c2 d5 48 b4 dd    	vpmadd52luq %zmm13,%zmm5,%zmm19
   2314c:	62 f2 95 40 b4 e9    	vpmadd52luq %zmm1,%zmm29,%zmm5
   23152:	62 32 ed 40 b4 d4    	vpmadd52luq %zmm20,%zmm18,%zmm10
   23158:	62 72 a5 40 b5 f9    	vpmadd52huq %zmm1,%zmm27,%zmm15
   2315e:	62 62 95 40 b5 f9    	vpmadd52huq %zmm1,%zmm29,%zmm31
   23164:	62 22 ed 40 b5 f4    	vpmadd52huq %zmm20,%zmm18,%zmm30
   2316a:	62 d1 d5 48 d4 ec    	vpaddq %zmm12,%zmm5,%zmm5
   23170:	62 c1 e5 40 db d4    	vpandq %zmm12,%zmm19,%zmm18
   23176:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   2317d:	62 f2 ed 40 b5 ee    	vpmadd52huq %zmm6,%zmm18,%zmm5
   23183:	62 72 ad 40 b5 d1    	vpmadd52huq %zmm1,%zmm26,%zmm10
   23189:	62 72 ad 40 b4 f9    	vpmadd52luq %zmm1,%zmm26,%zmm15
   2318f:	62 62 a5 40 b4 f9    	vpmadd52luq %zmm1,%zmm27,%zmm31
   23195:	62 62 b5 40 b5 f1    	vpmadd52huq %zmm1,%zmm25,%zmm30
   2319b:	62 72 b5 40 b4 d1    	vpmadd52luq %zmm1,%zmm25,%zmm10
   231a1:	62 32 ed 40 b5 ff    	vpmadd52huq %zmm23,%zmm18,%zmm15
   231a7:	62 22 ed 40 b4 ff    	vpmadd52luq %zmm23,%zmm18,%zmm31
   231ad:	62 62 bd 40 b4 f1    	vpmadd52luq %zmm1,%zmm24,%zmm30
   231b3:	62 32 ed 40 b5 d6    	vpmadd52huq %zmm22,%zmm18,%zmm10
   231b9:	62 32 ed 40 b4 fe    	vpmadd52luq %zmm22,%zmm18,%zmm15
   231bf:	62 f1 85 40 d4 ed    	vpaddq %zmm5,%zmm31,%zmm5
   231c5:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   231cb:	62 62 bd 40 b5 f9    	vpmadd52huq %zmm1,%zmm24,%zmm31
   231d1:	62 22 ed 40 b5 f5    	vpmadd52huq %zmm21,%zmm18,%zmm30
   231d7:	62 c2 d5 48 b4 cd    	vpmadd52luq %zmm13,%zmm5,%zmm17
   231dd:	62 f2 95 40 b4 ea    	vpmadd52luq %zmm2,%zmm29,%zmm5
   231e3:	62 32 ed 40 b4 d5    	vpmadd52luq %zmm21,%zmm18,%zmm10
   231e9:	62 72 95 40 b5 fa    	vpmadd52huq %zmm2,%zmm29,%zmm15
   231ef:	62 22 ed 40 b5 fc    	vpmadd52huq %zmm20,%zmm18,%zmm31
   231f5:	62 22 ed 40 b4 f4    	vpmadd52luq %zmm20,%zmm18,%zmm30
   231fb:	62 d1 d5 48 d4 ec    	vpaddq %zmm12,%zmm5,%zmm5
   23201:	62 c1 f5 40 db cc    	vpandq %zmm12,%zmm17,%zmm17
   23207:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   2320e:	62 f2 f5 40 b5 ee    	vpmadd52huq %zmm6,%zmm17,%zmm5
   23214:	62 72 a5 40 b5 d2    	vpmadd52huq %zmm2,%zmm27,%zmm10
   2321a:	62 72 a5 40 b4 fa    	vpmadd52luq %zmm2,%zmm27,%zmm15
   23220:	62 62 b5 40 b5 fa    	vpmadd52huq %zmm2,%zmm25,%zmm31
   23226:	62 62 ad 40 b5 f2    	vpmadd52huq %zmm2,%zmm26,%zmm30
   2322c:	62 72 ad 40 b4 d2    	vpmadd52luq %zmm2,%zmm26,%zmm10
   23232:	62 32 f5 40 b4 ff    	vpmadd52luq %zmm23,%zmm17,%zmm15
   23238:	62 62 bd 40 b4 fa    	vpmadd52luq %zmm2,%zmm24,%zmm31
   2323e:	62 62 b5 40 b4 f2    	vpmadd52luq %zmm2,%zmm25,%zmm30
   23244:	62 32 f5 40 b5 d7    	vpmadd52huq %zmm23,%zmm17,%zmm10
   2324a:	62 f1 85 48 d4 ed    	vpaddq %zmm5,%zmm15,%zmm5
   23250:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   23255:	62 72 bd 40 b5 fa    	vpmadd52huq %zmm2,%zmm24,%zmm15
   2325b:	62 22 f5 40 b5 fd    	vpmadd52huq %zmm21,%zmm17,%zmm31
   23261:	62 22 f5 40 b5 f6    	vpmadd52huq %zmm22,%zmm17,%zmm30
   23267:	62 52 d5 48 b4 f5    	vpmadd52luq %zmm13,%zmm5,%zmm14
   2326d:	62 f2 95 40 b4 eb    	vpmadd52luq %zmm3,%zmm29,%zmm5
   23273:	62 32 f5 40 b4 d6    	vpmadd52luq %zmm22,%zmm17,%zmm10
   23279:	62 32 f5 40 b5 fc    	vpmadd52huq %zmm20,%zmm17,%zmm15
   2327f:	62 22 f5 40 b4 fc    	vpmadd52luq %zmm20,%zmm17,%zmm31
   23285:	62 22 f5 40 b4 f5    	vpmadd52luq %zmm21,%zmm17,%zmm30
   2328b:	62 d1 d5 48 d4 ec    	vpaddq %zmm12,%zmm5,%zmm5
   23291:	62 51 8d 48 db f4    	vpandq %zmm12,%zmm14,%zmm14
   23297:	62 f1 d5 48 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm5
   2329e:	62 f2 8d 48 b5 ee    	vpmadd52huq %zmm6,%zmm14,%zmm5
   232a4:	62 72 95 40 b5 d3    	vpmadd52huq %zmm3,%zmm29,%zmm10
   232aa:	62 72 b5 40 b5 fb    	vpmadd52huq %zmm3,%zmm25,%zmm15
   232b0:	62 62 ad 40 b5 fb    	vpmadd52huq %zmm3,%zmm26,%zmm31
   232b6:	62 62 a5 40 b5 f3    	vpmadd52huq %zmm3,%zmm27,%zmm30
   232bc:	62 72 a5 40 b4 d3    	vpmadd52luq %zmm3,%zmm27,%zmm10
   232c2:	62 72 bd 40 b4 fb    	vpmadd52luq %zmm3,%zmm24,%zmm15
   232c8:	62 62 b5 40 b4 fb    	vpmadd52luq %zmm3,%zmm25,%zmm31
   232ce:	62 62 ad 40 b4 f3    	vpmadd52luq %zmm3,%zmm26,%zmm30
   232d4:	62 32 8d 48 b4 d7    	vpmadd52luq %zmm23,%zmm14,%zmm10
   232da:	62 32 8d 48 b5 fd    	vpmadd52huq %zmm21,%zmm14,%zmm15
   232e0:	62 22 8d 48 b5 fe    	vpmadd52huq %zmm22,%zmm14,%zmm31
   232e6:	62 22 8d 48 b5 f7    	vpmadd52huq %zmm23,%zmm14,%zmm30
   232ec:	62 f1 ad 48 d4 ed    	vpaddq %zmm5,%zmm10,%zmm5
   232f2:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   232f7:	62 72 bd 40 b5 d3    	vpmadd52huq %zmm3,%zmm24,%zmm10
   232fd:	62 32 8d 48 b4 fc    	vpmadd52luq %zmm20,%zmm14,%zmm15
   23303:	62 22 8d 48 b4 fd    	vpmadd52luq %zmm21,%zmm14,%zmm31
   23309:	62 22 8d 48 b4 f6    	vpmadd52luq %zmm22,%zmm14,%zmm30
   2330f:	62 42 d5 48 b4 e5    	vpmadd52luq %zmm13,%zmm5,%zmm28
   23315:	62 f2 95 40 b4 ec    	vpmadd52luq %zmm4,%zmm29,%zmm5
   2331b:	62 32 8d 48 b5 d4    	vpmadd52huq %zmm20,%zmm14,%zmm10
   23321:	62 72 ad 40 b5 fc    	vpmadd52huq %zmm4,%zmm26,%zmm15
   23327:	62 62 a5 40 b5 fc    	vpmadd52huq %zmm4,%zmm27,%zmm31
   2332d:	62 72 fd 48 59 35 d1 	vpbroadcastq -0x1f52f(%rip),%zmm14        # 3e08 <__abi_tag+0x3b0c>
   23334:	0a fe ff
   23337:	62 62 95 40 b5 f4    	vpmadd52huq %zmm4,%zmm29,%zmm30
   2333d:	62 c1 9d 40 db d3    	vpandq %zmm11,%zmm28,%zmm18
   23343:	62 91 f5 40 73 f4 04 	vpsllq $0x4,%zmm28,%zmm17
   2334a:	62 72 b5 40 b5 d4    	vpmadd52huq %zmm4,%zmm25,%zmm10
   23350:	62 72 b5 40 b4 fc    	vpmadd52luq %zmm4,%zmm25,%zmm15
   23356:	62 62 ad 40 b4 fc    	vpmadd52luq %zmm4,%zmm26,%zmm31
   2335c:	62 62 a5 40 b4 f4    	vpmadd52luq %zmm4,%zmm27,%zmm30
   23362:	62 62 fd 48 59 0d 34 	vpbroadcastq -0x1f4cc(%rip),%zmm25        # 3ea0 <__abi_tag+0x3ba4>
   23369:	0b fe ff
   2336c:	62 72 bd 40 b4 d4    	vpmadd52luq %zmm4,%zmm24,%zmm10
   23372:	62 32 ed 40 b5 fe    	vpmadd52huq %zmm22,%zmm18,%zmm15
   23378:	62 22 ed 40 b5 ff    	vpmadd52huq %zmm23,%zmm18,%zmm31
   2337e:	62 d1 d5 48 d4 ee    	vpaddq %zmm14,%zmm5,%zmm5
   23384:	c4 41 09 ef f6       	vpxor  %xmm14,%xmm14,%xmm14
   23389:	62 72 bd 40 b5 f4    	vpmadd52huq %zmm4,%zmm24,%zmm14
   2338f:	62 22 ed 40 b4 f7    	vpmadd52luq %zmm23,%zmm18,%zmm30
   23395:	62 61 fd 48 6f c6    	vmovdqa64 %zmm6,%zmm24
   2339b:	62 f1 fe 48 6f b4 24 	vmovdqu64 0x30(%rsp),%zmm6
   233a2:	30 00 00 00
   233a6:	62 f1 d5 48 73 d5 30 	vpsrlq $0x30,%zmm5,%zmm5
   233ad:	62 92 f5 40 b5 e8    	vpmadd52huq %zmm24,%zmm17,%zmm5
   233b3:	62 32 ed 40 b5 d5    	vpmadd52huq %zmm21,%zmm18,%zmm10
   233b9:	62 32 ed 40 b4 fd    	vpmadd52luq %zmm21,%zmm18,%zmm15
   233bf:	62 22 ed 40 b4 fe    	vpmadd52luq %zmm22,%zmm18,%zmm31
   233c5:	62 32 ed 40 b5 f4    	vpmadd52huq %zmm20,%zmm18,%zmm14
   233cb:	62 91 f5 40 73 f6 04 	vpsllq $0x4,%zmm30,%zmm17
   233d2:	62 e1 7c 28 28 2d 04 	vmovaps -0x1f7fc(%rip),%ymm21        # 3be0 <__abi_tag+0x38e4>
   233d9:	08 fe ff
   233dc:	62 81 f5 40 db c9    	vpandq %zmm25,%zmm17,%zmm17
   233e2:	62 f1 f5 40 d4 ed    	vpaddq %zmm5,%zmm17,%zmm5
   233e8:	62 91 f5 40 73 d6 30 	vpsrlq $0x30,%zmm30,%zmm17
   233ef:	62 32 ed 40 b4 d4    	vpmadd52luq %zmm20,%zmm18,%zmm10
   233f5:	62 d1 dd 40 73 f7 04 	vpsllq $0x4,%zmm15,%zmm20
   233fc:	62 91 e5 40 73 d7 30 	vpsrlq $0x30,%zmm31,%zmm19
   23403:	62 d1 85 48 73 d7 30 	vpsrlq $0x30,%zmm15,%zmm15
   2340a:	62 91 ed 40 73 f7 04 	vpsllq $0x4,%zmm31,%zmm18
   23411:	62 d1 8d 48 73 f6 04 	vpsllq $0x4,%zmm14,%zmm14
   23418:	62 81 dd 40 db e1    	vpandq %zmm25,%zmm20,%zmm20
   2341e:	62 81 ed 40 db d1    	vpandq %zmm25,%zmm18,%zmm18
   23424:	62 a1 dd 40 d4 db    	vpaddq %zmm19,%zmm20,%zmm19
   2342a:	62 d1 cd 40 73 f2 04 	vpsllq $0x4,%zmm10,%zmm22
   23431:	62 d1 ad 48 73 d2 30 	vpsrlq $0x30,%zmm10,%zmm10
   23438:	62 e1 7c 28 11 2c 24 	vmovups %ymm21,(%rsp)
   2343f:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
   23444:	62 81 cd 40 db e1    	vpandq %zmm25,%zmm22,%zmm20
   2344a:	62 51 8d 48 d4 d2    	vpaddq %zmm10,%zmm14,%zmm10
   23450:	62 e2 fd 48 59 35 56 	vpbroadcastq -0x1f5aa(%rip),%zmm22        # 3eb0 <__abi_tag+0x3bb4>
   23457:	0a fe ff
   2345a:	62 51 dd 40 d4 ff    	vpaddq %zmm15,%zmm20,%zmm15
   23460:	62 f1 dd 40 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm20
   23467:	62 d1 d5 48 db ec    	vpandq %zmm12,%zmm5,%zmm5
   2346d:	62 51 fd 40 d4 d2    	vpaddq %zmm10,%zmm16,%zmm10
   23473:	62 31 dd 40 d4 f1    	vpaddq %zmm17,%zmm20,%zmm14
   23479:	62 f1 cd 48 d4 ed    	vpaddq %zmm5,%zmm6,%zmm5
   2347f:	62 31 8d 48 d4 f2    	vpaddq %zmm18,%zmm14,%zmm14
   23485:	62 d1 f5 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm17
   2348c:	62 51 8d 48 db f4    	vpandq %zmm12,%zmm14,%zmm14
   23492:	62 a1 f5 40 d4 cb    	vpaddq %zmm19,%zmm17,%zmm17
   23498:	62 f1 e5 40 73 d5 34 	vpsrlq $0x34,%zmm5,%zmm19
   2349f:	62 d1 d5 48 db ec    	vpandq %zmm12,%zmm5,%zmm5
   234a5:	62 b1 ed 40 73 d1 34 	vpsrlq $0x34,%zmm17,%zmm18
   234ac:	62 c1 f5 40 db cc    	vpandq %zmm12,%zmm17,%zmm17
   234b2:	62 51 ed 40 d4 ff    	vpaddq %zmm15,%zmm18,%zmm15
   234b8:	62 a1 bd 48 d4 c9    	vpaddq %zmm17,%zmm8,%zmm17
   234be:	62 d1 ed 40 73 d7 34 	vpsrlq $0x34,%zmm15,%zmm18
   234c5:	62 51 85 48 db fc    	vpandq %zmm12,%zmm15,%zmm15
   234cb:	62 a1 d5 48 d4 f6    	vpaddq %zmm22,%zmm5,%zmm22
   234d1:	62 31 ad 48 d4 d2    	vpaddq %zmm18,%zmm10,%zmm10
   234d7:	62 e1 e5 40 d4 d7    	vpaddq %zmm7,%zmm19,%zmm18
   234dd:	62 51 b5 48 d4 ff    	vpaddq %zmm15,%zmm9,%zmm15
   234e3:	62 b1 c5 40 72 e6 34 	vpsraq $0x34,%zmm22,%zmm23
   234ea:	62 51 ed 40 d4 f6    	vpaddq %zmm14,%zmm18,%zmm14
   234f0:	62 d1 ed 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm18
   234f7:	62 51 8d 48 db f4    	vpandq %zmm12,%zmm14,%zmm14
   234fd:	62 f1 8d 58 fb 74 24 	vpsubq 0x8(%rsp){1to8},%zmm14,%zmm6
   23504:	01
   23505:	62 a1 ed 40 d4 c9    	vpaddq %zmm17,%zmm18,%zmm17
   2350b:	62 b1 ed 40 73 d1 34 	vpsrlq $0x34,%zmm17,%zmm18
   23512:	62 c1 f5 40 db cc    	vpandq %zmm12,%zmm17,%zmm17
   23518:	62 e1 f5 50 fb 5c 24 	vpsubq 0x10(%rsp){1to8},%zmm17,%zmm19
   2351f:	02
   23520:	62 51 ed 40 d4 ff    	vpaddq %zmm15,%zmm18,%zmm15
   23526:	62 d1 ed 40 73 d7 34 	vpsrlq $0x34,%zmm15,%zmm18
   2352d:	62 51 85 48 db fc    	vpandq %zmm12,%zmm15,%zmm15
   23533:	62 e1 85 58 fb 64 24 	vpsubq 0x18(%rsp){1to8},%zmm15,%zmm20
   2353a:	03
   2353b:	62 51 ed 40 d4 d2    	vpaddq %zmm10,%zmm18,%zmm10
   23541:	62 e1 ad 58 fb 6c 24 	vpsubq 0x20(%rsp){1to8},%zmm10,%zmm21
   23548:	04
   23549:	62 a1 cd 48 d4 d7    	vpaddq %zmm23,%zmm6,%zmm18
   2354f:	62 f2 fd 48 59 35 f7 	vpbroadcastq -0x1f709(%rip),%zmm6        # 3e50 <__abi_tag+0x3b54>
   23556:	08 fe ff
   23559:	62 b1 c5 40 72 e2 34 	vpsraq $0x34,%zmm18,%zmm23
   23560:	62 a1 e5 40 d4 df    	vpaddq %zmm23,%zmm19,%zmm19
   23566:	62 b1 c5 40 72 e3 34 	vpsraq $0x34,%zmm19,%zmm23
   2356d:	62 a1 dd 40 d4 e7    	vpaddq %zmm23,%zmm20,%zmm20
   23573:	62 b1 c5 40 72 e4 34 	vpsraq $0x34,%zmm20,%zmm23
   2357a:	62 a1 d5 40 d4 ef    	vpaddq %zmm23,%zmm21,%zmm21
   23580:	62 f3 d5 40 1e ce 01 	vpcmpltuq %zmm6,%zmm21,%k1
   23587:	62 d1 cd 41 db ec    	vpandq %zmm12,%zmm22,%zmm5{%k1}
   2358d:	62 51 ed 41 db f4    	vpandq %zmm12,%zmm18,%zmm14{%k1}
   23593:	62 c1 e5 41 db cc    	vpandq %zmm12,%zmm19,%zmm17{%k1}
   23599:	62 51 dd 41 db fc    	vpandq %zmm12,%zmm20,%zmm15{%k1}
   2359f:	62 51 d5 41 db d4    	vpandq %zmm12,%zmm21,%zmm10{%k1}
   235a5:	62 f1 fd 48 7f 6c 3e 	vmovdqa64 %zmm5,-0x100(%rsi,%rdi,1)
   235ac:	fc
   235ad:	c5 fd 6f 2d 2b 02 fe 	vmovdqa -0x1fdd5(%rip),%ymm5        # 37e0 <__abi_tag+0x34e4>
   235b4:	ff
   235b5:	62 71 fd 48 7f 74 3e 	vmovdqa64 %zmm14,-0xc0(%rsi,%rdi,1)
   235bc:	fd
   235bd:	62 e1 fd 48 7f 4c 3e 	vmovdqa64 %zmm17,-0x80(%rsi,%rdi,1)
   235c4:	fe
   235c5:	62 71 fd 48 7f 7c 3e 	vmovdqa64 %zmm15,-0x40(%rsi,%rdi,1)
   235cc:	ff
   235cd:	62 71 fd 48 7f 14 3e 	vmovdqa64 %zmm10,(%rsi,%rdi,1)
   235d4:	48 81 c7 40 01 00 00 	add    $0x140,%rdi
   235db:	48 81 ff 00 06 00 00 	cmp    $0x600,%rdi
   235e2:	0f 85 28 fa ff ff    	jne    23010 <probe::streams::<zopro_bn254::FqConfig, true, true>+0xb0>
   235e8:	48 ff c0             	inc    %rax
   235eb:	4c 39 c0             	cmp    %r8,%rax
   235ee:	0f 85 0c fa ff ff    	jne    23000 <probe::streams::<zopro_bn254::FqConfig, true, true>+0xa0>
   235f4:	ba 00 05 00 00       	mov    $0x500,%edx
   235f9:	48 89 df             	mov    %rbx,%rdi
   235fc:	c5 f8 77             	vzeroupper
   235ff:	ff 15 bb 3d 07 00    	call   *0x73dbb(%rip)        # 973c0 <memcpy@GLIBC_2.14>
   23605:	48 83 c4 70          	add    $0x70,%rsp
   23609:	5b                   	pop    %rbx
   2360a:	c3                   	ret
   2360b:	cc                   	int3
   2360c:	cc                   	int3
   2360d:	cc                   	int3
   2360e:	cc                   	int3
   2360f:	cc                   	int3

0000000000023610 <probe::streams::<zopro_bn254::FqConfig, true, false>>:
   23610:	53                   	push   %rbx
   23611:	48 83 ec 30          	sub    $0x30,%rsp
   23615:	62 e1 fd 48 6f 0a    	vmovdqa64 (%rdx),%zmm17
   2361b:	62 f1 fd 48 6f 4a 01 	vmovdqa64 0x40(%rdx),%zmm1
   23622:	62 f1 fd 48 6f 52 02 	vmovdqa64 0x80(%rdx),%zmm2
   23629:	62 f1 fd 48 6f 5a 03 	vmovdqa64 0xc0(%rdx),%zmm3
   23630:	62 f1 fd 48 6f 62 04 	vmovdqa64 0x100(%rdx),%zmm4
   23637:	62 f1 fd 48 6f 31    	vmovdqa64 (%rcx),%zmm6
   2363d:	62 f1 fd 48 6f 79 01 	vmovdqa64 0x40(%rcx),%zmm7
   23644:	62 71 fd 48 6f 41 02 	vmovdqa64 0x80(%rcx),%zmm8
   2364b:	62 71 fd 48 6f 49 03 	vmovdqa64 0xc0(%rcx),%zmm9
   23652:	62 e1 fd 48 6f 41 04 	vmovdqa64 0x100(%rcx),%zmm16
   23659:	c5 7d 6f 15 7f 01 fe 	vmovdqa -0x1fe81(%rip),%ymm10        # 37e0 <__abi_tag+0x34e4>
   23660:	ff
   23661:	62 72 fd 48 59 2d 8d 	vpbroadcastq -0x1f773(%rip),%zmm13        # 3ef8 <__abi_tag+0x3bfc>
   23668:	08 fe ff
   2366b:	62 f2 fd 48 59 05 8b 	vpbroadcastq -0x1f875(%rip),%zmm0        # 3e00 <__abi_tag+0x3b04>
   23672:	07 fe ff
   23675:	62 72 fd 48 59 25 b9 	vpbroadcastq -0x1f847(%rip),%zmm12        # 3e38 <__abi_tag+0x3b3c>
   2367c:	07 fe ff
   2367f:	62 e2 fd 48 59 1d 7f 	vpbroadcastq -0x1f881(%rip),%zmm19        # 3e08 <__abi_tag+0x3b0c>
   23686:	07 fe ff
   23689:	62 72 fd 48 59 1d 0d 	vpbroadcastq -0x1f7f3(%rip),%zmm11        # 3ea0 <__abi_tag+0x3ba4>
   23690:	08 fe ff
   23693:	c5 fc 28 2d 45 05 fe 	vmovaps -0x1fabb(%rip),%ymm5        # 3be0 <__abi_tag+0x38e4>
   2369a:	ff
   2369b:	62 62 fd 48 59 25 0b 	vpbroadcastq -0x1f7f5(%rip),%zmm28        # 3eb0 <__abi_tag+0x3bb4>
   236a2:	08 fe ff
   236a5:	48 89 fb             	mov    %rdi,%rbx
   236a8:	31 c0                	xor    %eax,%eax
   236aa:	48 b9 31 e1 72 4e 64 	movabs $0x30644e72e131,%rcx
   236b1:	30 00 00
   236b4:	48 ba 63 c2 e5 9c c8 	movabs $0x60c89ce5c263,%rdx
   236bb:	60 00 00
   236be:	66 90                	xchg   %ax,%ax
   236c0:	bf 00 01 00 00       	mov    $0x100,%edi
   236c5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   236cc:	00 00 00 00
   236d0:	62 61 fd 48 6f 6c 3e 	vmovdqa64 -0x100(%rsi,%rdi,1),%zmm29
   236d7:	fc
   236d8:	62 61 fd 48 6f 4c 3e 	vmovdqa64 -0x40(%rsi,%rdi,1),%zmm25
   236df:	ff
   236e0:	62 61 fd 48 6f 54 3e 	vmovdqa64 -0x80(%rsi,%rdi,1),%zmm26
   236e7:	fe
   236e8:	62 e1 7c 28 28 d5    	vmovaps %ymm5,%ymm18
   236ee:	62 91 fd 48 6f ec    	vmovdqa64 %zmm28,%zmm5
   236f4:	62 61 fd 48 6f 64 3e 	vmovdqa64 -0xc0(%rsi,%rdi,1),%zmm28
   236fb:	fd
   236fc:	62 61 fd 48 6f 04 3e 	vmovdqa64 (%rsi,%rdi,1),%zmm24
   23703:	c5 7e 7f 14 24       	vmovdqu %ymm10,(%rsp)
   23708:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   2370d:	62 01 25 00 ef db    	vpxord %xmm27,%xmm27,%xmm27
   23713:	c4 41 09 ef f6       	vpxor  %xmm14,%xmm14,%xmm14
   23718:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   2371e:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   23723:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   23729:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   2372e:	62 e2 fd 48 59 7c 24 	vpbroadcastq 0x8(%rsp),%zmm23
   23735:	01
   23736:	62 e2 fd 48 59 6c 24 	vpbroadcastq 0x18(%rsp),%zmm21
   2373d:	03
   2373e:	62 e2 fd 48 59 74 24 	vpbroadcastq 0x10(%rsp),%zmm22
   23745:	02
   23746:	62 e2 fd 48 59 64 24 	vpbroadcastq 0x20(%rsp),%zmm20
   2374d:	04
   2374e:	62 32 95 40 b5 f1    	vpmadd52huq %zmm17,%zmm29,%zmm14
   23754:	62 22 b5 40 b5 f9    	vpmadd52huq %zmm17,%zmm25,%zmm31
   2375a:	62 22 95 40 b4 d9    	vpmadd52luq %zmm17,%zmm29,%zmm27
   23760:	62 32 ad 40 b5 d1    	vpmadd52huq %zmm17,%zmm26,%zmm10
   23766:	62 22 9d 40 b5 f1    	vpmadd52huq %zmm17,%zmm28,%zmm30
   2376c:	62 52 a5 40 b4 fd    	vpmadd52luq %zmm13,%zmm27,%zmm15
   23772:	62 32 9d 40 b4 f1    	vpmadd52luq %zmm17,%zmm28,%zmm14
   23778:	62 22 bd 40 b4 f9    	vpmadd52luq %zmm17,%zmm24,%zmm31
   2377e:	62 32 b5 40 b4 d1    	vpmadd52luq %zmm17,%zmm25,%zmm10
   23784:	62 22 ad 40 b4 f1    	vpmadd52luq %zmm17,%zmm26,%zmm30
   2378a:	62 61 a5 40 d4 d8    	vpaddq %zmm0,%zmm27,%zmm27
   23790:	62 91 a5 40 73 d3 34 	vpsrlq $0x34,%zmm27,%zmm27
   23797:	62 22 85 48 b5 fd    	vpmadd52huq %zmm21,%zmm15,%zmm31
   2379d:	62 42 85 48 b5 dc    	vpmadd52huq %zmm12,%zmm15,%zmm27
   237a3:	62 32 85 48 b4 f7    	vpmadd52luq %zmm23,%zmm15,%zmm14
   237a9:	62 32 85 48 b5 d6    	vpmadd52huq %zmm22,%zmm15,%zmm10
   237af:	62 22 85 48 b5 f7    	vpmadd52huq %zmm23,%zmm15,%zmm30
   237b5:	62 11 8d 48 d4 f3    	vpaddq %zmm27,%zmm14,%zmm14
   237bb:	62 01 25 00 ef db    	vpxord %xmm27,%xmm27,%xmm27
   237c1:	62 22 bd 40 b5 d9    	vpmadd52huq %zmm17,%zmm24,%zmm27
   237c7:	62 22 85 48 b4 fc    	vpmadd52luq %zmm20,%zmm15,%zmm31
   237cd:	62 32 85 48 b4 d5    	vpmadd52luq %zmm21,%zmm15,%zmm10
   237d3:	62 22 85 48 b4 f6    	vpmadd52luq %zmm22,%zmm15,%zmm30
   237d9:	62 72 95 40 b4 f1    	vpmadd52luq %zmm1,%zmm29,%zmm14
   237df:	62 22 85 48 b5 dc    	vpmadd52huq %zmm20,%zmm15,%zmm27
   237e5:	62 62 ad 40 b5 f9    	vpmadd52huq %zmm1,%zmm26,%zmm31
   237eb:	62 72 9d 40 b5 d1    	vpmadd52huq %zmm1,%zmm28,%zmm10
   237f1:	62 62 95 40 b5 f1    	vpmadd52huq %zmm1,%zmm29,%zmm30
   237f7:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   237fc:	62 52 8d 48 b4 fd    	vpmadd52luq %zmm13,%zmm14,%zmm15
   23802:	62 71 8d 48 d4 f0    	vpaddq %zmm0,%zmm14,%zmm14
   23808:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   2380f:	62 62 b5 40 b4 f9    	vpmadd52luq %zmm1,%zmm25,%zmm31
   23815:	62 62 b5 40 b5 d9    	vpmadd52huq %zmm1,%zmm25,%zmm27
   2381b:	62 72 ad 40 b4 d1    	vpmadd52luq %zmm1,%zmm26,%zmm10
   23821:	62 62 9d 40 b4 f1    	vpmadd52luq %zmm1,%zmm28,%zmm30
   23827:	62 52 85 48 b5 f4    	vpmadd52huq %zmm12,%zmm15,%zmm14
   2382d:	62 62 bd 40 b4 d9    	vpmadd52luq %zmm1,%zmm24,%zmm27
   23833:	62 22 85 48 b5 fe    	vpmadd52huq %zmm22,%zmm15,%zmm31
   23839:	62 32 85 48 b5 d7    	vpmadd52huq %zmm23,%zmm15,%zmm10
   2383f:	62 22 85 48 b4 f7    	vpmadd52luq %zmm23,%zmm15,%zmm30
   23845:	62 22 85 48 b4 fd    	vpmadd52luq %zmm21,%zmm15,%zmm31
   2384b:	62 22 85 48 b5 dd    	vpmadd52huq %zmm21,%zmm15,%zmm27
   23851:	62 32 85 48 b4 d6    	vpmadd52luq %zmm22,%zmm15,%zmm10
   23857:	62 51 8d 40 d4 f6    	vpaddq %zmm14,%zmm30,%zmm14
   2385d:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   23863:	62 62 bd 40 b5 f1    	vpmadd52huq %zmm1,%zmm24,%zmm30
   23869:	62 72 95 40 b4 f2    	vpmadd52luq %zmm2,%zmm29,%zmm14
   2386f:	62 22 85 48 b4 dc    	vpmadd52luq %zmm20,%zmm15,%zmm27
   23875:	62 62 9d 40 b5 fa    	vpmadd52huq %zmm2,%zmm28,%zmm31
   2387b:	62 72 95 40 b5 d2    	vpmadd52huq %zmm2,%zmm29,%zmm10
   23881:	62 22 85 48 b5 f4    	vpmadd52huq %zmm20,%zmm15,%zmm30
   23887:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   2388c:	62 52 8d 48 b4 fd    	vpmadd52luq %zmm13,%zmm14,%zmm15
   23892:	62 71 8d 48 d4 f0    	vpaddq %zmm0,%zmm14,%zmm14
   23898:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   2389f:	62 62 ad 40 b4 fa    	vpmadd52luq %zmm2,%zmm26,%zmm31
   238a5:	62 62 ad 40 b5 da    	vpmadd52huq %zmm2,%zmm26,%zmm27
   238ab:	62 72 9d 40 b4 d2    	vpmadd52luq %zmm2,%zmm28,%zmm10
   238b1:	62 62 b5 40 b5 f2    	vpmadd52huq %zmm2,%zmm25,%zmm30
   238b7:	62 52 85 48 b5 f4    	vpmadd52huq %zmm12,%zmm15,%zmm14
   238bd:	62 62 b5 40 b4 da    	vpmadd52luq %zmm2,%zmm25,%zmm27
   238c3:	62 22 85 48 b5 ff    	vpmadd52huq %zmm23,%zmm15,%zmm31
   238c9:	62 32 85 48 b4 d7    	vpmadd52luq %zmm23,%zmm15,%zmm10
   238cf:	62 62 bd 40 b4 f2    	vpmadd52luq %zmm2,%zmm24,%zmm30
   238d5:	62 22 85 48 b4 fe    	vpmadd52luq %zmm22,%zmm15,%zmm31
   238db:	62 22 85 48 b5 de    	vpmadd52huq %zmm22,%zmm15,%zmm27
   238e1:	62 51 ad 48 d4 f6    	vpaddq %zmm14,%zmm10,%zmm14
   238e7:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   238ec:	62 72 bd 40 b5 d2    	vpmadd52huq %zmm2,%zmm24,%zmm10
   238f2:	62 22 85 48 b5 f5    	vpmadd52huq %zmm21,%zmm15,%zmm30
   238f8:	62 72 95 40 b4 f3    	vpmadd52luq %zmm3,%zmm29,%zmm14
   238fe:	62 22 85 48 b4 dd    	vpmadd52luq %zmm21,%zmm15,%zmm27
   23904:	62 62 95 40 b5 fb    	vpmadd52huq %zmm3,%zmm29,%zmm31
   2390a:	62 32 85 48 b5 d4    	vpmadd52huq %zmm20,%zmm15,%zmm10
   23910:	62 22 85 48 b4 f4    	vpmadd52luq %zmm20,%zmm15,%zmm30
   23916:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   2391b:	62 52 8d 48 b4 fd    	vpmadd52luq %zmm13,%zmm14,%zmm15
   23921:	62 71 8d 48 d4 f0    	vpaddq %zmm0,%zmm14,%zmm14
   23927:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   2392e:	62 62 9d 40 b4 fb    	vpmadd52luq %zmm3,%zmm28,%zmm31
   23934:	62 62 9d 40 b5 db    	vpmadd52huq %zmm3,%zmm28,%zmm27
   2393a:	62 72 b5 40 b5 d3    	vpmadd52huq %zmm3,%zmm25,%zmm10
   23940:	62 62 ad 40 b5 f3    	vpmadd52huq %zmm3,%zmm26,%zmm30
   23946:	62 52 85 48 b5 f4    	vpmadd52huq %zmm12,%zmm15,%zmm14
   2394c:	62 62 ad 40 b4 db    	vpmadd52luq %zmm3,%zmm26,%zmm27
   23952:	62 22 85 48 b4 ff    	vpmadd52luq %zmm23,%zmm15,%zmm31
   23958:	62 72 bd 40 b4 d3    	vpmadd52luq %zmm3,%zmm24,%zmm10
   2395e:	62 62 b5 40 b4 f3    	vpmadd52luq %zmm3,%zmm25,%zmm30
   23964:	62 51 85 40 d4 f6    	vpaddq %zmm14,%zmm31,%zmm14
   2396a:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   23970:	62 62 bd 40 b5 fb    	vpmadd52huq %zmm3,%zmm24,%zmm31
   23976:	62 22 85 48 b5 df    	vpmadd52huq %zmm23,%zmm15,%zmm27
   2397c:	62 32 85 48 b5 d5    	vpmadd52huq %zmm21,%zmm15,%zmm10
   23982:	62 22 85 48 b5 f6    	vpmadd52huq %zmm22,%zmm15,%zmm30
   23988:	62 72 95 40 b4 f4    	vpmadd52luq %zmm4,%zmm29,%zmm14
   2398e:	62 22 85 48 b4 de    	vpmadd52luq %zmm22,%zmm15,%zmm27
   23994:	62 22 85 48 b5 fc    	vpmadd52huq %zmm20,%zmm15,%zmm31
   2399a:	62 32 85 48 b4 d4    	vpmadd52luq %zmm20,%zmm15,%zmm10
   239a0:	62 22 85 48 b4 f5    	vpmadd52luq %zmm21,%zmm15,%zmm30
   239a6:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   239ab:	62 72 bd 40 b5 fc    	vpmadd52huq %zmm4,%zmm24,%zmm15
   239b1:	62 62 95 40 b5 dc    	vpmadd52huq %zmm4,%zmm29,%zmm27
   239b7:	62 62 b5 40 b5 fc    	vpmadd52huq %zmm4,%zmm25,%zmm31
   239bd:	62 62 7d 48 5a 2d 69 	vbroadcasti32x4 -0x1ef97(%rip),%zmm29        # 4a30 <__abi_tag+0x4734>
   239c4:	10 fe ff
   239c7:	62 72 ad 40 b5 d4    	vpmadd52huq %zmm4,%zmm26,%zmm10
   239cd:	62 62 9d 40 b5 f4    	vpmadd52huq %zmm4,%zmm28,%zmm30
   239d3:	62 62 bd 40 b4 fc    	vpmadd52luq %zmm4,%zmm24,%zmm31
   239d9:	62 01 3d 00 ef c0    	vpxord %xmm24,%xmm24,%xmm24
   239df:	62 42 8d 48 b4 c5    	vpmadd52luq %zmm13,%zmm14,%zmm24
   239e5:	62 72 b5 40 b4 d4    	vpmadd52luq %zmm4,%zmm25,%zmm10
   239eb:	62 62 ad 40 b4 f4    	vpmadd52luq %zmm4,%zmm26,%zmm30
   239f1:	62 62 9d 40 b4 dc    	vpmadd52luq %zmm4,%zmm28,%zmm27
   239f7:	62 31 8d 48 d4 f3    	vpaddq %zmm19,%zmm14,%zmm14
   239fd:	62 61 fd 48 6f e5    	vmovdqa64 %zmm5,%zmm28
   23a03:	62 b1 7c 28 28 ea    	vmovaps %ymm18,%ymm5
   23a09:	c5 fc 11 2c 24       	vmovups %ymm5,(%rsp)
   23a0e:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
   23a13:	62 e2 fd 48 59 15 33 	vpbroadcastq -0x1fbcd(%rip),%zmm18        # 3e50 <__abi_tag+0x3b54>
   23a1a:	04 fe ff
   23a1d:	62 d1 8d 48 73 d6 30 	vpsrlq $0x30,%zmm14,%zmm14
   23a24:	62 91 b5 40 73 f0 04 	vpsllq $0x4,%zmm24,%zmm25
   23a2b:	62 01 bd 40 db c5    	vpandq %zmm29,%zmm24,%zmm24
   23a31:	62 32 bd 40 b5 d6    	vpmadd52huq %zmm22,%zmm24,%zmm10
   23a37:	62 22 bd 40 b5 f7    	vpmadd52huq %zmm23,%zmm24,%zmm30
   23a3d:	62 22 bd 40 b5 fd    	vpmadd52huq %zmm21,%zmm24,%zmm31
   23a43:	62 22 bd 40 b4 df    	vpmadd52luq %zmm23,%zmm24,%zmm27
   23a49:	62 52 b5 40 b5 f4    	vpmadd52huq %zmm12,%zmm25,%zmm14
   23a4f:	62 32 bd 40 b5 fc    	vpmadd52huq %zmm20,%zmm24,%zmm15
   23a55:	62 32 bd 40 b4 d5    	vpmadd52luq %zmm21,%zmm24,%zmm10
   23a5b:	62 22 bd 40 b4 f6    	vpmadd52luq %zmm22,%zmm24,%zmm30
   23a61:	62 22 bd 40 b4 fc    	vpmadd52luq %zmm20,%zmm24,%zmm31
   23a67:	62 91 dd 40 73 f3 04 	vpsllq $0x4,%zmm27,%zmm20
   23a6e:	62 d1 85 48 73 f7 04 	vpsllq $0x4,%zmm15,%zmm15
   23a75:	62 c1 dd 40 db e3    	vpandq %zmm11,%zmm20,%zmm20
   23a7b:	62 51 dd 40 d4 f6    	vpaddq %zmm14,%zmm20,%zmm14
   23a81:	62 91 dd 40 73 d3 30 	vpsrlq $0x30,%zmm27,%zmm20
   23a88:	62 d1 c5 40 73 f2 04 	vpsllq $0x4,%zmm10,%zmm23
   23a8f:	62 91 cd 40 73 d6 30 	vpsrlq $0x30,%zmm30,%zmm22
   23a96:	62 91 bd 40 73 f7 04 	vpsllq $0x4,%zmm31,%zmm24
   23a9d:	62 91 d5 40 73 f6 04 	vpsllq $0x4,%zmm30,%zmm21
   23aa4:	62 d1 ad 48 73 d2 30 	vpsrlq $0x30,%zmm10,%zmm10
   23aab:	62 91 b5 40 73 d7 30 	vpsrlq $0x30,%zmm31,%zmm25
   23ab2:	62 c1 c5 40 db fb    	vpandq %zmm11,%zmm23,%zmm23
   23ab8:	62 c1 d5 40 db eb    	vpandq %zmm11,%zmm21,%zmm21
   23abe:	62 11 85 48 d4 f9    	vpaddq %zmm25,%zmm15,%zmm15
   23ac4:	62 a1 c5 40 d4 f6    	vpaddq %zmm22,%zmm23,%zmm22
   23aca:	62 c1 bd 40 db fb    	vpandq %zmm11,%zmm24,%zmm23
   23ad0:	62 d1 bd 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm24
   23ad7:	62 71 8d 48 db f0    	vpandq %zmm0,%zmm14,%zmm14
   23add:	62 51 fd 40 d4 ff    	vpaddq %zmm15,%zmm16,%zmm15
   23ae3:	62 a1 bd 40 d4 e4    	vpaddq %zmm20,%zmm24,%zmm20
   23ae9:	62 51 c5 40 d4 d2    	vpaddq %zmm10,%zmm23,%zmm10
   23aef:	62 51 cd 48 d4 f6    	vpaddq %zmm14,%zmm6,%zmm14
   23af5:	62 a1 dd 40 d4 e5    	vpaddq %zmm21,%zmm20,%zmm20
   23afb:	62 d1 c5 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm23
   23b02:	62 71 8d 48 db f0    	vpandq %zmm0,%zmm14,%zmm14
   23b08:	62 b1 d5 40 73 d4 34 	vpsrlq $0x34,%zmm20,%zmm21
   23b0f:	62 e1 dd 40 db e0    	vpandq %zmm0,%zmm20,%zmm20
   23b15:	62 01 8d 48 d4 d4    	vpaddq %zmm28,%zmm14,%zmm26
   23b1b:	62 a1 d5 40 d4 ee    	vpaddq %zmm22,%zmm21,%zmm21
   23b21:	62 91 a5 40 72 e2 34 	vpsraq $0x34,%zmm26,%zmm27
   23b28:	62 b1 cd 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm22
   23b2f:	62 e1 d5 40 db e8    	vpandq %zmm0,%zmm21,%zmm21
   23b35:	62 51 cd 40 d4 d2    	vpaddq %zmm10,%zmm22,%zmm10
   23b3b:	62 a1 bd 48 d4 ed    	vpaddq %zmm21,%zmm8,%zmm21
   23b41:	62 d1 cd 40 73 d2 34 	vpsrlq $0x34,%zmm10,%zmm22
   23b48:	62 71 ad 48 db d0    	vpandq %zmm0,%zmm10,%zmm10
   23b4e:	62 31 85 48 d4 fe    	vpaddq %zmm22,%zmm15,%zmm15
   23b54:	62 e1 c5 40 d4 f7    	vpaddq %zmm7,%zmm23,%zmm22
   23b5a:	62 51 b5 48 d4 d2    	vpaddq %zmm10,%zmm9,%zmm10
   23b60:	62 a1 cd 40 d4 e4    	vpaddq %zmm20,%zmm22,%zmm20
   23b66:	62 b1 cd 40 73 d4 34 	vpsrlq $0x34,%zmm20,%zmm22
   23b6d:	62 e1 dd 40 db e0    	vpandq %zmm0,%zmm20,%zmm20
   23b73:	62 61 dd 50 fb 6c 24 	vpsubq 0x8(%rsp){1to8},%zmm20,%zmm29
   23b7a:	01
   23b7b:	62 a1 cd 40 d4 ed    	vpaddq %zmm21,%zmm22,%zmm21
   23b81:	62 b1 cd 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm22
   23b88:	62 e1 d5 40 db e8    	vpandq %zmm0,%zmm21,%zmm21
   23b8e:	62 e1 d5 50 fb 7c 24 	vpsubq 0x10(%rsp){1to8},%zmm21,%zmm23
   23b95:	02
   23b96:	62 51 cd 40 d4 d2    	vpaddq %zmm10,%zmm22,%zmm10
   23b9c:	62 d1 cd 40 73 d2 34 	vpsrlq $0x34,%zmm10,%zmm22
   23ba3:	62 71 ad 48 db d0    	vpandq %zmm0,%zmm10,%zmm10
   23ba9:	62 61 ad 58 fb 44 24 	vpsubq 0x18(%rsp){1to8},%zmm10,%zmm24
   23bb0:	03
   23bb1:	62 51 cd 40 d4 ff    	vpaddq %zmm15,%zmm22,%zmm15
   23bb7:	62 61 85 58 fb 4c 24 	vpsubq 0x20(%rsp){1to8},%zmm15,%zmm25
   23bbe:	04
   23bbf:	62 81 95 40 d4 f3    	vpaddq %zmm27,%zmm29,%zmm22
   23bc5:	62 b1 a5 40 72 e6 34 	vpsraq $0x34,%zmm22,%zmm27
   23bcc:	62 81 c5 40 d4 fb    	vpaddq %zmm27,%zmm23,%zmm23
   23bd2:	62 b1 a5 40 72 e7 34 	vpsraq $0x34,%zmm23,%zmm27
   23bd9:	62 01 bd 40 d4 c3    	vpaddq %zmm27,%zmm24,%zmm24
   23bdf:	62 91 a5 40 72 e0 34 	vpsraq $0x34,%zmm24,%zmm27
   23be6:	62 01 b5 40 d4 cb    	vpaddq %zmm27,%zmm25,%zmm25
   23bec:	62 b3 b5 40 1e ca 01 	vpcmpltuq %zmm18,%zmm25,%k1
   23bf3:	62 71 ad 41 db f0    	vpandq %zmm0,%zmm26,%zmm14{%k1}
   23bf9:	62 e1 cd 41 db e0    	vpandq %zmm0,%zmm22,%zmm20{%k1}
   23bff:	62 e1 c5 41 db e8    	vpandq %zmm0,%zmm23,%zmm21{%k1}
   23c05:	62 71 bd 41 db d0    	vpandq %zmm0,%zmm24,%zmm10{%k1}
   23c0b:	62 71 b5 41 db f8    	vpandq %zmm0,%zmm25,%zmm15{%k1}
   23c11:	62 71 fd 48 7f 74 3e 	vmovdqa64 %zmm14,-0x100(%rsi,%rdi,1)
   23c18:	fc
   23c19:	62 e1 fd 48 7f 64 3e 	vmovdqa64 %zmm20,-0xc0(%rsi,%rdi,1)
   23c20:	fd
   23c21:	62 e1 fd 48 7f 6c 3e 	vmovdqa64 %zmm21,-0x80(%rsi,%rdi,1)
   23c28:	fe
   23c29:	62 71 fd 48 7f 54 3e 	vmovdqa64 %zmm10,-0x40(%rsi,%rdi,1)
   23c30:	ff
   23c31:	c5 7d 6f 15 a7 fb fd 	vmovdqa -0x20459(%rip),%ymm10        # 37e0 <__abi_tag+0x34e4>
   23c38:	ff
   23c39:	62 71 fd 48 7f 3c 3e 	vmovdqa64 %zmm15,(%rsi,%rdi,1)
   23c40:	48 81 c7 40 01 00 00 	add    $0x140,%rdi
   23c47:	48 81 ff 00 06 00 00 	cmp    $0x600,%rdi
   23c4e:	0f 85 7c fa ff ff    	jne    236d0 <probe::streams::<zopro_bn254::FqConfig, true, false>+0xc0>
   23c54:	48 ff c0             	inc    %rax
   23c57:	4c 39 c0             	cmp    %r8,%rax
   23c5a:	0f 85 60 fa ff ff    	jne    236c0 <probe::streams::<zopro_bn254::FqConfig, true, false>+0xb0>
   23c60:	ba 00 05 00 00       	mov    $0x500,%edx
   23c65:	48 89 df             	mov    %rbx,%rdi
   23c68:	c5 f8 77             	vzeroupper
   23c6b:	ff 15 4f 37 07 00    	call   *0x7374f(%rip)        # 973c0 <memcpy@GLIBC_2.14>
   23c71:	48 83 c4 30          	add    $0x30,%rsp
   23c75:	5b                   	pop    %rbx
   23c76:	c3                   	ret
   23c77:	cc                   	int3
   23c78:	cc                   	int3
   23c79:	cc                   	int3
   23c7a:	cc                   	int3
   23c7b:	cc                   	int3
   23c7c:	cc                   	int3
   23c7d:	cc                   	int3
   23c7e:	cc                   	int3
   23c7f:	cc                   	int3
