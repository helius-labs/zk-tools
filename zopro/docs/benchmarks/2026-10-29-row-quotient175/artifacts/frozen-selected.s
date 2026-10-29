0000000000019b00 <probe::chain::<zopro_bn254::FqConfig, true, true>>:
   19b00:	55                   	push   %rbp
   19b01:	48 89 e5             	mov    %rsp,%rbp
   19b04:	48 83 e4 c0          	and    $0xffffffffffffffc0,%rsp
   19b08:	48 81 ec 80 01 00 00 	sub    $0x180,%rsp
   19b0f:	62 f1 fd 48 6f 02    	vmovdqa64 (%rdx),%zmm0
   19b15:	62 f1 fd 48 6f 4a 01 	vmovdqa64 0x40(%rdx),%zmm1
   19b1c:	62 f1 fd 48 6f 52 02 	vmovdqa64 0x80(%rdx),%zmm2
   19b23:	62 f1 fd 48 6f 5a 03 	vmovdqa64 0xc0(%rdx),%zmm3
   19b2a:	62 f1 fd 48 6f 62 04 	vmovdqa64 0x100(%rdx),%zmm4
   19b31:	62 f1 fd 48 6f 29    	vmovdqa64 (%rcx),%zmm5
   19b37:	62 f1 fd 48 6f 71 01 	vmovdqa64 0x40(%rcx),%zmm6
   19b3e:	62 f1 fd 48 6f 79 02 	vmovdqa64 0x80(%rcx),%zmm7
   19b45:	62 71 fd 48 6f 41 03 	vmovdqa64 0xc0(%rcx),%zmm8
   19b4c:	62 71 fd 48 6f 49 04 	vmovdqa64 0x100(%rcx),%zmm9
   19b53:	62 61 fd 48 6f 06    	vmovdqa64 (%rsi),%zmm24
   19b59:	62 e1 fd 48 6f 7e 01 	vmovdqa64 0x40(%rsi),%zmm23
   19b60:	62 e1 fd 48 6f 76 02 	vmovdqa64 0x80(%rsi),%zmm22
   19b67:	62 e1 fd 48 6f 6e 03 	vmovdqa64 0xc0(%rsi),%zmm21
   19b6e:	62 e1 fd 48 6f 66 04 	vmovdqa64 0x100(%rsi),%zmm20
   19b75:	62 72 fd 48 59 1d 79 	vpbroadcastq -0x15c87(%rip),%zmm11        # 3ef8 <__abi_tag+0x3bfc>
   19b7c:	a3 fe ff
   19b7f:	62 72 fd 48 59 25 77 	vpbroadcastq -0x15d89(%rip),%zmm12        # 3e00 <__abi_tag+0x3b04>
   19b86:	a2 fe ff
   19b89:	62 72 fd 48 59 2d a5 	vpbroadcastq -0x15d5b(%rip),%zmm13        # 3e38 <__abi_tag+0x3b3c>
   19b90:	a2 fe ff
   19b93:	62 e2 fd 48 59 05 03 	vpbroadcastq -0x15cfd(%rip),%zmm16        # 3ea0 <__abi_tag+0x3ba4>
   19b9a:	a3 fe ff
   19b9d:	62 e1 7c 28 28 0d 39 	vmovaps -0x15fc7(%rip),%ymm17        # 3be0 <__abi_tag+0x38e4>
   19ba4:	a0 fe ff
   19ba7:	62 e2 fd 48 59 15 ff 	vpbroadcastq -0x15d01(%rip),%zmm18        # 3eb0 <__abi_tag+0x3bb4>
   19bae:	a2 fe ff
   19bb1:	62 e2 fd 48 59 1d 95 	vpbroadcastq -0x15d6b(%rip),%zmm19        # 3e50 <__abi_tag+0x3b54>
   19bb8:	a2 fe ff
   19bbb:	48 b8 31 e1 72 4e 64 	movabs $0x30644e72e131,%rax
   19bc2:	30 00 00
   19bc5:	48 b9 63 c2 e5 9c c8 	movabs $0x60c89ce5c263,%rcx
   19bcc:	60 00 00
   19bcf:	90                   	nop
   19bd0:	c5 7d 6f 15 08 9c fe 	vmovdqa -0x163f8(%rip),%ymm10        # 37e0 <__abi_tag+0x34e4>
   19bd7:	ff
   19bd8:	62 01 15 00 ef ed    	vpxord %xmm29,%xmm29,%xmm29
   19bde:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   19be4:	62 62 d5 40 b5 f8    	vpmadd52huq %zmm0,%zmm21,%zmm31
   19bea:	62 62 bd 40 b4 e8    	vpmadd52luq %zmm0,%zmm24,%zmm29
   19bf0:	c4 41 09 ef f6       	vpxor  %xmm14,%xmm14,%xmm14
   19bf5:	62 72 bd 40 b5 f0    	vpmadd52huq %zmm0,%zmm24,%zmm14
   19bfb:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   19c01:	62 62 c5 40 b5 f0    	vpmadd52huq %zmm0,%zmm23,%zmm30
   19c07:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   19c0c:	49 ff c8             	dec    %r8
   19c0f:	62 52 95 40 b4 fb    	vpmadd52luq %zmm11,%zmm29,%zmm15
   19c15:	62 62 dd 40 b4 f8    	vpmadd52luq %zmm0,%zmm20,%zmm31
   19c1b:	62 72 c5 40 b4 f0    	vpmadd52luq %zmm0,%zmm23,%zmm14
   19c21:	62 62 cd 40 b4 f0    	vpmadd52luq %zmm0,%zmm22,%zmm30
   19c27:	62 41 95 40 d4 ec    	vpaddq %zmm12,%zmm29,%zmm29
   19c2d:	62 91 95 40 73 d5 34 	vpsrlq $0x34,%zmm29,%zmm29
   19c34:	c5 7e 7f 14 24       	vmovdqu %ymm10,(%rsp)
   19c39:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
   19c3e:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   19c43:	62 72 cd 40 b5 d0    	vpmadd52huq %zmm0,%zmm22,%zmm10
   19c49:	62 42 85 48 b5 ed    	vpmadd52huq %zmm13,%zmm15,%zmm29
   19c4f:	62 62 fd 48 59 54 24 	vpbroadcastq 0x18(%rsp),%zmm26
   19c56:	03
   19c57:	62 62 fd 48 59 64 24 	vpbroadcastq 0x8(%rsp),%zmm28
   19c5e:	01
   19c5f:	62 62 fd 48 59 5c 24 	vpbroadcastq 0x10(%rsp),%zmm27
   19c66:	02
   19c67:	62 62 fd 48 59 4c 24 	vpbroadcastq 0x20(%rsp),%zmm25
   19c6e:	04
   19c6f:	62 e1 7c 28 11 0c 24 	vmovups %ymm17,(%rsp)
   19c76:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   19c7b:	62 72 d5 40 b4 d0    	vpmadd52luq %zmm0,%zmm21,%zmm10
   19c81:	62 02 85 48 b5 fa    	vpmadd52huq %zmm26,%zmm15,%zmm31
   19c87:	62 12 85 48 b4 f4    	vpmadd52luq %zmm28,%zmm15,%zmm14
   19c8d:	62 02 85 48 b5 f4    	vpmadd52huq %zmm28,%zmm15,%zmm30
   19c93:	62 12 85 48 b5 d3    	vpmadd52huq %zmm27,%zmm15,%zmm10
   19c99:	62 02 85 48 b4 f9    	vpmadd52luq %zmm25,%zmm15,%zmm31
   19c9f:	62 11 8d 48 d4 f5    	vpaddq %zmm29,%zmm14,%zmm14
   19ca5:	62 02 85 48 b4 f3    	vpmadd52luq %zmm27,%zmm15,%zmm30
   19cab:	62 12 85 48 b4 d2    	vpmadd52luq %zmm26,%zmm15,%zmm10
   19cb1:	62 01 15 00 ef ed    	vpxord %xmm29,%xmm29,%xmm29
   19cb7:	62 62 dd 40 b5 e8    	vpmadd52huq %zmm0,%zmm20,%zmm29
   19cbd:	62 72 bd 40 b4 f1    	vpmadd52luq %zmm1,%zmm24,%zmm14
   19cc3:	62 62 cd 40 b5 f9    	vpmadd52huq %zmm1,%zmm22,%zmm31
   19cc9:	62 62 bd 40 b5 f1    	vpmadd52huq %zmm1,%zmm24,%zmm30
   19ccf:	62 72 c5 40 b5 d1    	vpmadd52huq %zmm1,%zmm23,%zmm10
   19cd5:	62 02 85 48 b5 e9    	vpmadd52huq %zmm25,%zmm15,%zmm29
   19cdb:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   19ce0:	62 52 8d 48 b4 fb    	vpmadd52luq %zmm11,%zmm14,%zmm15
   19ce6:	62 51 8d 48 d4 f4    	vpaddq %zmm12,%zmm14,%zmm14
   19cec:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   19cf3:	62 62 d5 40 b4 f9    	vpmadd52luq %zmm1,%zmm21,%zmm31
   19cf9:	62 62 c5 40 b4 f1    	vpmadd52luq %zmm1,%zmm23,%zmm30
   19cff:	62 72 cd 40 b4 d1    	vpmadd52luq %zmm1,%zmm22,%zmm10
   19d05:	62 62 d5 40 b5 e9    	vpmadd52huq %zmm1,%zmm21,%zmm29
   19d0b:	62 52 85 48 b5 f5    	vpmadd52huq %zmm13,%zmm15,%zmm14
   19d11:	62 02 85 48 b5 fb    	vpmadd52huq %zmm27,%zmm15,%zmm31
   19d17:	62 12 85 48 b5 d4    	vpmadd52huq %zmm28,%zmm15,%zmm10
   19d1d:	62 02 85 48 b4 f4    	vpmadd52luq %zmm28,%zmm15,%zmm30
   19d23:	62 62 dd 40 b4 e9    	vpmadd52luq %zmm1,%zmm20,%zmm29
   19d29:	62 02 85 48 b4 fa    	vpmadd52luq %zmm26,%zmm15,%zmm31
   19d2f:	62 51 8d 40 d4 f6    	vpaddq %zmm14,%zmm30,%zmm14
   19d35:	62 12 85 48 b4 d3    	vpmadd52luq %zmm27,%zmm15,%zmm10
   19d3b:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   19d41:	62 62 dd 40 b5 f1    	vpmadd52huq %zmm1,%zmm20,%zmm30
   19d47:	62 02 85 48 b5 ea    	vpmadd52huq %zmm26,%zmm15,%zmm29
   19d4d:	62 72 bd 40 b4 f2    	vpmadd52luq %zmm2,%zmm24,%zmm14
   19d53:	62 62 c5 40 b5 fa    	vpmadd52huq %zmm2,%zmm23,%zmm31
   19d59:	62 72 bd 40 b5 d2    	vpmadd52huq %zmm2,%zmm24,%zmm10
   19d5f:	62 02 85 48 b4 e9    	vpmadd52luq %zmm25,%zmm15,%zmm29
   19d65:	62 02 85 48 b5 f1    	vpmadd52huq %zmm25,%zmm15,%zmm30
   19d6b:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   19d70:	62 52 8d 48 b4 fb    	vpmadd52luq %zmm11,%zmm14,%zmm15
   19d76:	62 51 8d 48 d4 f4    	vpaddq %zmm12,%zmm14,%zmm14
   19d7c:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   19d83:	62 62 cd 40 b4 fa    	vpmadd52luq %zmm2,%zmm22,%zmm31
   19d89:	62 72 c5 40 b4 d2    	vpmadd52luq %zmm2,%zmm23,%zmm10
   19d8f:	62 62 cd 40 b5 ea    	vpmadd52huq %zmm2,%zmm22,%zmm29
   19d95:	62 62 d5 40 b5 f2    	vpmadd52huq %zmm2,%zmm21,%zmm30
   19d9b:	62 52 85 48 b5 f5    	vpmadd52huq %zmm13,%zmm15,%zmm14
   19da1:	62 02 85 48 b5 fc    	vpmadd52huq %zmm28,%zmm15,%zmm31
   19da7:	62 12 85 48 b4 d4    	vpmadd52luq %zmm28,%zmm15,%zmm10
   19dad:	62 62 d5 40 b4 ea    	vpmadd52luq %zmm2,%zmm21,%zmm29
   19db3:	62 62 dd 40 b4 f2    	vpmadd52luq %zmm2,%zmm20,%zmm30
   19db9:	62 02 85 48 b4 fb    	vpmadd52luq %zmm27,%zmm15,%zmm31
   19dbf:	62 51 ad 48 d4 f6    	vpaddq %zmm14,%zmm10,%zmm14
   19dc5:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   19dca:	62 72 dd 40 b5 d2    	vpmadd52huq %zmm2,%zmm20,%zmm10
   19dd0:	62 02 85 48 b5 eb    	vpmadd52huq %zmm27,%zmm15,%zmm29
   19dd6:	62 02 85 48 b5 f2    	vpmadd52huq %zmm26,%zmm15,%zmm30
   19ddc:	62 72 bd 40 b4 f3    	vpmadd52luq %zmm3,%zmm24,%zmm14
   19de2:	62 62 bd 40 b5 fb    	vpmadd52huq %zmm3,%zmm24,%zmm31
   19de8:	62 02 85 48 b4 ea    	vpmadd52luq %zmm26,%zmm15,%zmm29
   19dee:	62 02 85 48 b4 f1    	vpmadd52luq %zmm25,%zmm15,%zmm30
   19df4:	62 12 85 48 b5 d1    	vpmadd52huq %zmm25,%zmm15,%zmm10
   19dfa:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   19dff:	62 52 8d 48 b4 fb    	vpmadd52luq %zmm11,%zmm14,%zmm15
   19e05:	62 51 8d 48 d4 f4    	vpaddq %zmm12,%zmm14,%zmm14
   19e0b:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   19e12:	62 62 c5 40 b4 fb    	vpmadd52luq %zmm3,%zmm23,%zmm31
   19e18:	62 72 d5 40 b5 d3    	vpmadd52huq %zmm3,%zmm21,%zmm10
   19e1e:	62 62 c5 40 b5 eb    	vpmadd52huq %zmm3,%zmm23,%zmm29
   19e24:	62 62 cd 40 b5 f3    	vpmadd52huq %zmm3,%zmm22,%zmm30
   19e2a:	62 52 85 48 b5 f5    	vpmadd52huq %zmm13,%zmm15,%zmm14
   19e30:	62 02 85 48 b4 fc    	vpmadd52luq %zmm28,%zmm15,%zmm31
   19e36:	62 72 dd 40 b4 d3    	vpmadd52luq %zmm3,%zmm20,%zmm10
   19e3c:	62 62 cd 40 b4 eb    	vpmadd52luq %zmm3,%zmm22,%zmm29
   19e42:	62 62 d5 40 b4 f3    	vpmadd52luq %zmm3,%zmm21,%zmm30
   19e48:	62 51 85 40 d4 f6    	vpaddq %zmm14,%zmm31,%zmm14
   19e4e:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   19e54:	62 62 dd 40 b5 fb    	vpmadd52huq %zmm3,%zmm20,%zmm31
   19e5a:	62 12 85 48 b5 d2    	vpmadd52huq %zmm26,%zmm15,%zmm10
   19e60:	62 02 85 48 b5 ec    	vpmadd52huq %zmm28,%zmm15,%zmm29
   19e66:	62 02 85 48 b5 f3    	vpmadd52huq %zmm27,%zmm15,%zmm30
   19e6c:	62 72 bd 40 b4 f4    	vpmadd52luq %zmm4,%zmm24,%zmm14
   19e72:	62 02 85 48 b5 f9    	vpmadd52huq %zmm25,%zmm15,%zmm31
   19e78:	62 12 85 48 b4 d1    	vpmadd52luq %zmm25,%zmm15,%zmm10
   19e7e:	62 02 85 48 b4 eb    	vpmadd52luq %zmm27,%zmm15,%zmm29
   19e84:	62 02 85 48 b4 f2    	vpmadd52luq %zmm26,%zmm15,%zmm30
   19e8a:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   19e8f:	62 72 dd 40 b5 fc    	vpmadd52huq %zmm4,%zmm20,%zmm15
   19e95:	62 62 d5 40 b5 fc    	vpmadd52huq %zmm4,%zmm21,%zmm31
   19e9b:	62 72 cd 40 b5 d4    	vpmadd52huq %zmm4,%zmm22,%zmm10
   19ea1:	62 62 c5 40 b5 f4    	vpmadd52huq %zmm4,%zmm23,%zmm30
   19ea7:	62 62 bd 40 b5 ec    	vpmadd52huq %zmm4,%zmm24,%zmm29
   19ead:	62 62 dd 40 b4 fc    	vpmadd52luq %zmm4,%zmm20,%zmm31
   19eb3:	62 a1 5d 00 ef e4    	vpxord %xmm20,%xmm20,%xmm20
   19eb9:	62 c2 8d 48 b4 e3    	vpmadd52luq %zmm11,%zmm14,%zmm20
   19ebf:	62 72 d5 40 b4 d4    	vpmadd52luq %zmm4,%zmm21,%zmm10
   19ec5:	62 62 cd 40 b4 f4    	vpmadd52luq %zmm4,%zmm22,%zmm30
   19ecb:	62 71 8d 58 d4 35 33 	vpaddq -0x160cd(%rip){1to8},%zmm14,%zmm14        # 3e08 <__abi_tag+0x3b0c>
   19ed2:	9f fe ff
   19ed5:	62 62 c5 40 b4 ec    	vpmadd52luq %zmm4,%zmm23,%zmm29
   19edb:	62 b1 d5 40 73 f4 04 	vpsllq $0x4,%zmm20,%zmm21
   19ee2:	62 e1 dd 50 db 25 1c 	vpandq -0x160e4(%rip){1to8},%zmm20,%zmm20        # 3e08 <__abi_tag+0x3b0c>
   19ee9:	9f fe ff
   19eec:	62 d1 8d 48 73 d6 30 	vpsrlq $0x30,%zmm14,%zmm14
   19ef3:	62 52 d5 40 b5 f5    	vpmadd52huq %zmm13,%zmm21,%zmm14
   19ef9:	62 12 dd 40 b5 d3    	vpmadd52huq %zmm27,%zmm20,%zmm10
   19eff:	62 02 dd 40 b5 f4    	vpmadd52huq %zmm28,%zmm20,%zmm30
   19f05:	62 02 dd 40 b5 fa    	vpmadd52huq %zmm26,%zmm20,%zmm31
   19f0b:	62 02 dd 40 b4 ec    	vpmadd52luq %zmm28,%zmm20,%zmm29
   19f11:	62 12 dd 40 b5 f9    	vpmadd52huq %zmm25,%zmm20,%zmm15
   19f17:	62 12 dd 40 b4 d2    	vpmadd52luq %zmm26,%zmm20,%zmm10
   19f1d:	62 02 dd 40 b4 f3    	vpmadd52luq %zmm27,%zmm20,%zmm30
   19f23:	62 02 dd 40 b4 f9    	vpmadd52luq %zmm25,%zmm20,%zmm31
   19f29:	62 91 dd 40 73 f5 04 	vpsllq $0x4,%zmm29,%zmm20
   19f30:	62 d1 85 48 73 f7 04 	vpsllq $0x4,%zmm15,%zmm15
   19f37:	62 a1 dd 40 db e0    	vpandq %zmm16,%zmm20,%zmm20
   19f3d:	62 51 dd 40 d4 f6    	vpaddq %zmm14,%zmm20,%zmm14
   19f43:	62 91 dd 40 73 d5 30 	vpsrlq $0x30,%zmm29,%zmm20
   19f4a:	62 d1 c5 40 73 f2 04 	vpsllq $0x4,%zmm10,%zmm23
   19f51:	62 91 cd 40 73 d6 30 	vpsrlq $0x30,%zmm30,%zmm22
   19f58:	62 d1 ad 48 73 d2 30 	vpsrlq $0x30,%zmm10,%zmm10
   19f5f:	62 91 d5 40 73 f6 04 	vpsllq $0x4,%zmm30,%zmm21
   19f66:	62 a1 c5 40 db f8    	vpandq %zmm16,%zmm23,%zmm23
   19f6c:	62 a1 d5 40 db e8    	vpandq %zmm16,%zmm21,%zmm21
   19f72:	62 a1 c5 40 d4 f6    	vpaddq %zmm22,%zmm23,%zmm22
   19f78:	62 91 c5 40 73 f7 04 	vpsllq $0x4,%zmm31,%zmm23
   19f7f:	62 a1 c5 40 db f8    	vpandq %zmm16,%zmm23,%zmm23
   19f85:	62 51 c5 40 d4 d2    	vpaddq %zmm10,%zmm23,%zmm10
   19f8b:	62 91 c5 40 73 d7 30 	vpsrlq $0x30,%zmm31,%zmm23
   19f92:	62 31 85 48 d4 ff    	vpaddq %zmm23,%zmm15,%zmm15
   19f98:	62 d1 c5 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm23
   19f9f:	62 51 8d 48 db f4    	vpandq %zmm12,%zmm14,%zmm14
   19fa5:	62 a1 c5 40 d4 e4    	vpaddq %zmm20,%zmm23,%zmm20
   19fab:	62 51 d5 48 d4 f6    	vpaddq %zmm14,%zmm5,%zmm14
   19fb1:	62 51 b5 48 d4 ff    	vpaddq %zmm15,%zmm9,%zmm15
   19fb7:	62 a1 dd 40 d4 e5    	vpaddq %zmm21,%zmm20,%zmm20
   19fbd:	62 41 8d 48 db c4    	vpandq %zmm12,%zmm14,%zmm24
   19fc3:	62 b1 d5 40 73 d4 34 	vpsrlq $0x34,%zmm20,%zmm21
   19fca:	62 c1 dd 40 db e4    	vpandq %zmm12,%zmm20,%zmm20
   19fd0:	62 21 bd 40 d4 d2    	vpaddq %zmm18,%zmm24,%zmm26
   19fd6:	62 a1 d5 40 d4 ee    	vpaddq %zmm22,%zmm21,%zmm21
   19fdc:	62 91 a5 40 72 e2 34 	vpsraq $0x34,%zmm26,%zmm27
   19fe3:	62 b1 cd 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm22
   19fea:	62 c1 d5 40 db ec    	vpandq %zmm12,%zmm21,%zmm21
   19ff0:	62 51 cd 40 d4 d2    	vpaddq %zmm10,%zmm22,%zmm10
   19ff6:	62 a1 c5 48 d4 ed    	vpaddq %zmm21,%zmm7,%zmm21
   19ffc:	62 d1 cd 40 73 d2 34 	vpsrlq $0x34,%zmm10,%zmm22
   1a003:	62 51 ad 48 db d4    	vpandq %zmm12,%zmm10,%zmm10
   1a009:	62 31 85 48 d4 fe    	vpaddq %zmm22,%zmm15,%zmm15
   1a00f:	62 d1 cd 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm22
   1a016:	62 51 bd 48 d4 d2    	vpaddq %zmm10,%zmm8,%zmm10
   1a01c:	62 e1 cd 40 d4 f6    	vpaddq %zmm6,%zmm22,%zmm22
   1a022:	62 a1 cd 40 d4 e4    	vpaddq %zmm20,%zmm22,%zmm20
   1a028:	62 b1 8d 48 73 d4 34 	vpsrlq $0x34,%zmm20,%zmm14
   1a02f:	62 c1 dd 40 db fc    	vpandq %zmm12,%zmm20,%zmm23
   1a035:	62 61 c5 50 fb 64 24 	vpsubq 0x8(%rsp){1to8},%zmm23,%zmm28
   1a03c:	01
   1a03d:	62 31 8d 48 d4 f5    	vpaddq %zmm21,%zmm14,%zmm14
   1a043:	62 d1 dd 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm20
   1a04a:	62 c1 8d 48 db f4    	vpandq %zmm12,%zmm14,%zmm22
   1a050:	62 51 dd 40 d4 d2    	vpaddq %zmm10,%zmm20,%zmm10
   1a056:	62 d1 8d 48 73 d2 34 	vpsrlq $0x34,%zmm10,%zmm14
   1a05d:	62 c1 ad 48 db ec    	vpandq %zmm12,%zmm10,%zmm21
   1a063:	62 c1 8d 48 d4 e7    	vpaddq %zmm15,%zmm14,%zmm20
   1a069:	62 71 cd 50 fb 74 24 	vpsubq 0x10(%rsp){1to8},%zmm22,%zmm14
   1a070:	02
   1a071:	62 71 d5 50 fb 7c 24 	vpsubq 0x18(%rsp){1to8},%zmm21,%zmm15
   1a078:	03
   1a079:	62 61 dd 50 fb 4c 24 	vpsubq 0x20(%rsp){1to8},%zmm20,%zmm25
   1a080:	04
   1a081:	62 11 9d 40 d4 d3    	vpaddq %zmm27,%zmm28,%zmm10
   1a087:	62 d1 a5 40 72 e2 34 	vpsraq $0x34,%zmm10,%zmm27
   1a08e:	62 11 8d 48 d4 f3    	vpaddq %zmm27,%zmm14,%zmm14
   1a094:	62 d1 a5 40 72 e6 34 	vpsraq $0x34,%zmm14,%zmm27
   1a09b:	62 11 85 48 d4 fb    	vpaddq %zmm27,%zmm15,%zmm15
   1a0a1:	62 d1 a5 40 72 e7 34 	vpsraq $0x34,%zmm15,%zmm27
   1a0a8:	62 01 b5 40 d4 cb    	vpaddq %zmm27,%zmm25,%zmm25
   1a0ae:	62 b3 b5 40 1e cb 01 	vpcmpltuq %zmm19,%zmm25,%k1
   1a0b5:	62 41 ad 41 db c4    	vpandq %zmm12,%zmm26,%zmm24{%k1}
   1a0bb:	62 c1 ad 49 db fc    	vpandq %zmm12,%zmm10,%zmm23{%k1}
   1a0c1:	62 c1 8d 49 db f4    	vpandq %zmm12,%zmm14,%zmm22{%k1}
   1a0c7:	62 c1 85 49 db ec    	vpandq %zmm12,%zmm15,%zmm21{%k1}
   1a0cd:	62 c1 b5 41 db e4    	vpandq %zmm12,%zmm25,%zmm20{%k1}
   1a0d3:	0f 85 f7 fa ff ff    	jne    19bd0 <probe::chain::<zopro_bn254::FqConfig, true, true>+0xd0>
   1a0d9:	62 61 fd 48 7f 06    	vmovdqa64 %zmm24,(%rsi)
   1a0df:	62 e1 fd 48 7f 7e 01 	vmovdqa64 %zmm23,0x40(%rsi)
   1a0e6:	62 e1 fd 48 7f 76 02 	vmovdqa64 %zmm22,0x80(%rsi)
   1a0ed:	62 e1 fd 48 7f 6e 03 	vmovdqa64 %zmm21,0xc0(%rsi)
   1a0f4:	62 e1 fd 48 7f 66 04 	vmovdqa64 %zmm20,0x100(%rsi)
   1a0fb:	48 89 e0             	mov    %rsp,%rax
   1a0fe:	62 f1 7c 48 28 46 04 	vmovaps 0x100(%rsi),%zmm0
   1a105:	62 f1 7c 48 28 2e    	vmovaps (%rsi),%zmm5
   1a10b:	62 f1 7c 48 28 4e 01 	vmovaps 0x40(%rsi),%zmm1
   1a112:	62 f1 7c 48 28 56 02 	vmovaps 0x80(%rsi),%zmm2
   1a119:	62 f1 7c 48 28 5e 03 	vmovaps 0xc0(%rsi),%zmm3
   1a120:	62 f1 7c 48 29 44 24 	vmovaps %zmm0,0x100(%rsp)
   1a127:	04
   1a128:	62 f1 7c 48 29 5c 24 	vmovaps %zmm3,0xc0(%rsp)
   1a12f:	03
   1a130:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x80(%rsp)
   1a137:	02
   1a138:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x40(%rsp)
   1a13f:	01
   1a140:	62 f1 7c 48 29 2c 24 	vmovaps %zmm5,(%rsp)
   1a147:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1a14e:	01
   1a14f:	62 f1 7c 48 28 44 24 	vmovaps 0x100(%rsp),%zmm0
   1a156:	04
   1a157:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1a15e:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1a165:	02
   1a166:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1a16d:	03
   1a16e:	62 f1 7c 48 28 2c 24 	vmovaps (%rsp),%zmm5
   1a175:	62 f1 7c 48 29 4f 01 	vmovaps %zmm1,0x40(%rdi)
   1a17c:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1a183:	04
   1a184:	62 f1 7c 48 29 5f 03 	vmovaps %zmm3,0xc0(%rdi)
   1a18b:	62 f1 7c 48 29 57 02 	vmovaps %zmm2,0x80(%rdi)
   1a192:	62 f1 7c 48 29 27    	vmovaps %zmm4,(%rdi)
   1a198:	62 f1 7c 48 29 47 04 	vmovaps %zmm0,0x100(%rdi)
   1a19f:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1a1a6:	02
   1a1a7:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1a1ae:	03
   1a1af:	62 f1 7c 48 29 6f 05 	vmovaps %zmm5,0x140(%rdi)
   1a1b6:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1a1bd:	62 f1 7c 48 29 4f 09 	vmovaps %zmm1,0x240(%rdi)
   1a1c4:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1a1cb:	01
   1a1cc:	62 f1 7c 48 29 57 07 	vmovaps %zmm2,0x1c0(%rdi)
   1a1d3:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1a1da:	02
   1a1db:	62 f1 7c 48 29 5f 08 	vmovaps %zmm3,0x200(%rdi)
   1a1e2:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1a1e9:	03
   1a1ea:	62 f1 7c 48 29 4f 06 	vmovaps %zmm1,0x180(%rdi)
   1a1f1:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1a1f8:	01
   1a1f9:	62 f1 7c 48 29 57 0c 	vmovaps %zmm2,0x300(%rdi)
   1a200:	62 f1 7c 48 28 54 24 	vmovaps 0x100(%rsp),%zmm2
   1a207:	04
   1a208:	62 f1 7c 48 29 67 0a 	vmovaps %zmm4,0x280(%rdi)
   1a20f:	62 f1 7c 48 29 5f 0d 	vmovaps %zmm3,0x340(%rdi)
   1a216:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1a21d:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1a224:	03
   1a225:	62 f1 7c 48 29 4f 0b 	vmovaps %zmm1,0x2c0(%rdi)
   1a22c:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1a233:	04
   1a234:	62 f1 7c 48 29 57 0e 	vmovaps %zmm2,0x380(%rdi)
   1a23b:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1a242:	02
   1a243:	62 f1 7c 48 29 5f 12 	vmovaps %zmm3,0x480(%rdi)
   1a24a:	62 f1 7c 48 29 67 0f 	vmovaps %zmm4,0x3c0(%rdi)
   1a251:	62 f1 7c 48 29 4f 13 	vmovaps %zmm1,0x4c0(%rdi)
   1a258:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1a25f:	01
   1a260:	62 f1 7c 48 29 57 11 	vmovaps %zmm2,0x440(%rdi)
   1a267:	62 f1 7c 48 29 4f 10 	vmovaps %zmm1,0x400(%rdi)
   1a26e:	48 89 ec             	mov    %rbp,%rsp
   1a271:	5d                   	pop    %rbp
   1a272:	c5 f8 77             	vzeroupper
   1a275:	c3                   	ret
   1a276:	cc                   	int3
   1a277:	cc                   	int3
   1a278:	cc                   	int3
   1a279:	cc                   	int3
   1a27a:	cc                   	int3
   1a27b:	cc                   	int3
   1a27c:	cc                   	int3
   1a27d:	cc                   	int3
   1a27e:	cc                   	int3
   1a27f:	cc                   	int3

000000000001b840 <probe::fused::<zopro_bn254::FqConfig, true, true>>:
   1b840:	55                   	push   %rbp
   1b841:	48 89 e5             	mov    %rsp,%rbp
   1b844:	48 83 e4 c0          	and    $0xffffffffffffffc0,%rsp
   1b848:	48 81 ec c0 01 00 00 	sub    $0x1c0,%rsp
   1b84f:	62 f1 fd 48 6f 02    	vmovdqa64 (%rdx),%zmm0
   1b855:	62 f1 fd 48 6f 4a 01 	vmovdqa64 0x40(%rdx),%zmm1
   1b85c:	62 f1 fd 48 6f 52 02 	vmovdqa64 0x80(%rdx),%zmm2
   1b863:	62 f1 fd 48 6f 5a 03 	vmovdqa64 0xc0(%rdx),%zmm3
   1b86a:	62 f1 fd 48 6f 62 04 	vmovdqa64 0x100(%rdx),%zmm4
   1b871:	62 f1 fd 48 6f 29    	vmovdqa64 (%rcx),%zmm5
   1b877:	62 f1 fd 48 6f 71 01 	vmovdqa64 0x40(%rcx),%zmm6
   1b87e:	62 f1 fd 48 6f 79 02 	vmovdqa64 0x80(%rcx),%zmm7
   1b885:	62 71 fd 48 6f 41 03 	vmovdqa64 0xc0(%rcx),%zmm8
   1b88c:	62 71 fd 48 6f 49 04 	vmovdqa64 0x100(%rcx),%zmm9
   1b893:	62 51 fd 48 6f 10    	vmovdqa64 (%r8),%zmm10
   1b899:	62 51 fd 48 6f 58 01 	vmovdqa64 0x40(%r8),%zmm11
   1b8a0:	62 51 fd 48 6f 60 02 	vmovdqa64 0x80(%r8),%zmm12
   1b8a7:	62 51 fd 48 6f 68 03 	vmovdqa64 0xc0(%r8),%zmm13
   1b8ae:	62 51 fd 48 6f 70 04 	vmovdqa64 0x100(%r8),%zmm14
   1b8b5:	62 61 fd 48 6f 36    	vmovdqa64 (%rsi),%zmm30
   1b8bb:	62 61 fd 48 6f 6e 01 	vmovdqa64 0x40(%rsi),%zmm29
   1b8c2:	62 61 fd 48 6f 66 02 	vmovdqa64 0x80(%rsi),%zmm28
   1b8c9:	62 61 fd 48 6f 5e 03 	vmovdqa64 0xc0(%rsi),%zmm27
   1b8d0:	62 61 fd 48 6f 56 04 	vmovdqa64 0x100(%rsi),%zmm26
   1b8d7:	48 b8 31 e1 72 4e 64 	movabs $0x30644e72e131,%rax
   1b8de:	30 00 00
   1b8e1:	48 b9 63 c2 e5 9c c8 	movabs $0x60c89ce5c263,%rcx
   1b8e8:	60 00 00
   1b8eb:	62 f1 fd 48 7f 44 24 	vmovdqa64 %zmm0,0x140(%rsp)
   1b8f2:	05
   1b8f3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   1b8fa:	84 00 00 00 00 00
   1b900:	62 f1 fd 48 6f 44 24 	vmovdqa64 0x140(%rsp),%zmm0
   1b907:	05
   1b908:	c5 7d 6f 3d d0 7e fe 	vmovdqa -0x18130(%rip),%ymm15        # 37e0 <__abi_tag+0x34e4>
   1b90f:	ff
   1b910:	62 a1 4d 00 ef f6    	vpxord %xmm22,%xmm22,%xmm22
   1b916:	62 a1 65 00 ef db    	vpxord %xmm19,%xmm19,%xmm19
   1b91c:	62 a1 5d 00 ef e4    	vpxord %xmm20,%xmm20,%xmm20
   1b922:	62 e2 fd 48 59 15 cc 	vpbroadcastq -0x17a34(%rip),%zmm18        # 3ef8 <__abi_tag+0x3bfc>
   1b929:	85 fe ff
   1b92c:	62 a1 55 00 ef ed    	vpxord %xmm21,%xmm21,%xmm21
   1b932:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1b938:	62 e2 fd 48 59 0d f6 	vpbroadcastq -0x17b0a(%rip),%zmm17        # 3e38 <__abi_tag+0x3b3c>
   1b93f:	84 fe ff
   1b942:	62 e2 9d 40 b5 f0    	vpmadd52huq %zmm0,%zmm28,%zmm22
   1b948:	c5 7e 7f 3c 24       	vmovdqu %ymm15,(%rsp)
   1b94d:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1b952:	62 72 8d 40 b4 f8    	vpmadd52luq %zmm0,%zmm30,%zmm15
   1b958:	62 e2 95 40 b5 d8    	vpmadd52huq %zmm0,%zmm29,%zmm19
   1b95e:	62 e2 a5 40 b5 e0    	vpmadd52huq %zmm0,%zmm27,%zmm20
   1b964:	62 e2 8d 40 b5 e8    	vpmadd52huq %zmm0,%zmm30,%zmm21
   1b96a:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
   1b96f:	62 62 fd 48 59 44 24 	vpbroadcastq 0x10(%rsp),%zmm24
   1b976:	02
   1b977:	62 62 fd 48 59 4c 24 	vpbroadcastq 0x8(%rsp),%zmm25
   1b97e:	01
   1b97f:	62 e2 fd 48 59 7c 24 	vpbroadcastq 0x18(%rsp),%zmm23
   1b986:	03
   1b987:	62 62 fd 48 59 7c 24 	vpbroadcastq 0x20(%rsp),%zmm31
   1b98e:	04
   1b98f:	62 e2 a5 40 b4 f0    	vpmadd52luq %zmm0,%zmm27,%zmm22
   1b995:	62 e2 9d 40 b4 d8    	vpmadd52luq %zmm0,%zmm28,%zmm19
   1b99b:	62 e2 ad 40 b4 e0    	vpmadd52luq %zmm0,%zmm26,%zmm20
   1b9a1:	62 52 d5 48 b4 fa    	vpmadd52luq %zmm10,%zmm5,%zmm15
   1b9a7:	62 e2 95 40 b4 e8    	vpmadd52luq %zmm0,%zmm29,%zmm21
   1b9ad:	62 c2 c5 48 b5 f2    	vpmadd52huq %zmm10,%zmm7,%zmm22
   1b9b3:	62 c2 cd 48 b5 da    	vpmadd52huq %zmm10,%zmm6,%zmm19
   1b9b9:	62 c2 bd 48 b5 e2    	vpmadd52huq %zmm10,%zmm8,%zmm20
   1b9bf:	62 a2 85 48 b4 c2    	vpmadd52luq %zmm18,%zmm15,%zmm16
   1b9c5:	62 e2 fd 48 59 15 31 	vpbroadcastq -0x17bcf(%rip),%zmm18        # 3e00 <__abi_tag+0x3b04>
   1b9cc:	84 fe ff
   1b9cf:	62 c2 d5 48 b5 ea    	vpmadd52huq %zmm10,%zmm5,%zmm21
   1b9d5:	62 c2 bd 48 b4 f2    	vpmadd52luq %zmm10,%zmm8,%zmm22
   1b9db:	62 c2 c5 48 b4 da    	vpmadd52luq %zmm10,%zmm7,%zmm19
   1b9e1:	62 c2 b5 48 b4 e2    	vpmadd52luq %zmm10,%zmm9,%zmm20
   1b9e7:	62 c2 cd 48 b4 ea    	vpmadd52luq %zmm10,%zmm6,%zmm21
   1b9ed:	62 82 fd 40 b5 f0    	vpmadd52huq %zmm24,%zmm16,%zmm22
   1b9f3:	62 31 85 48 d4 fa    	vpaddq %zmm18,%zmm15,%zmm15
   1b9f9:	62 82 fd 40 b5 d9    	vpmadd52huq %zmm25,%zmm16,%zmm19
   1b9ff:	62 a2 fd 40 b5 e7    	vpmadd52huq %zmm23,%zmm16,%zmm20
   1ba05:	62 82 fd 40 b4 e9    	vpmadd52luq %zmm25,%zmm16,%zmm21
   1ba0b:	62 d1 85 48 73 d7 34 	vpsrlq $0x34,%zmm15,%zmm15
   1ba12:	62 32 fd 40 b5 f9    	vpmadd52huq %zmm17,%zmm16,%zmm15
   1ba18:	62 a2 fd 40 b4 f7    	vpmadd52luq %zmm23,%zmm16,%zmm22
   1ba1e:	62 82 fd 40 b4 d8    	vpmadd52luq %zmm24,%zmm16,%zmm19
   1ba24:	62 82 fd 40 b4 e7    	vpmadd52luq %zmm31,%zmm16,%zmm20
   1ba2a:	62 c1 d5 40 d4 ef    	vpaddq %zmm15,%zmm21,%zmm21
   1ba30:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1ba35:	62 72 ad 40 b5 f8    	vpmadd52huq %zmm0,%zmm26,%zmm15
   1ba3b:	62 f2 fd 48 59 05 b3 	vpbroadcastq -0x17b4d(%rip),%zmm0        # 3ef8 <__abi_tag+0x3bfc>
   1ba42:	84 fe ff
   1ba45:	62 e2 8d 40 b4 e9    	vpmadd52luq %zmm1,%zmm30,%zmm21
   1ba4b:	62 e2 95 40 b5 f1    	vpmadd52huq %zmm1,%zmm29,%zmm22
   1ba51:	62 e2 8d 40 b5 d9    	vpmadd52huq %zmm1,%zmm30,%zmm19
   1ba57:	62 e2 9d 40 b5 e1    	vpmadd52huq %zmm1,%zmm28,%zmm20
   1ba5d:	62 52 b5 48 b5 fa    	vpmadd52huq %zmm10,%zmm9,%zmm15
   1ba63:	62 c2 d5 48 b4 eb    	vpmadd52luq %zmm11,%zmm5,%zmm21
   1ba69:	62 e2 9d 40 b4 f1    	vpmadd52luq %zmm1,%zmm28,%zmm22
   1ba6f:	62 e2 95 40 b4 d9    	vpmadd52luq %zmm1,%zmm29,%zmm19
   1ba75:	62 e2 a5 40 b4 e1    	vpmadd52luq %zmm1,%zmm27,%zmm20
   1ba7b:	62 12 fd 40 b5 ff    	vpmadd52huq %zmm31,%zmm16,%zmm15
   1ba81:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1ba87:	62 e2 d5 40 b4 c0    	vpmadd52luq %zmm0,%zmm21,%zmm16
   1ba8d:	62 a1 d5 40 d4 ea    	vpaddq %zmm18,%zmm21,%zmm21
   1ba93:	62 c2 cd 48 b5 f3    	vpmadd52huq %zmm11,%zmm6,%zmm22
   1ba99:	62 c2 d5 48 b5 db    	vpmadd52huq %zmm11,%zmm5,%zmm19
   1ba9f:	62 c2 c5 48 b5 e3    	vpmadd52huq %zmm11,%zmm7,%zmm20
   1baa5:	62 b1 d5 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm21
   1baac:	62 72 a5 40 b5 f9    	vpmadd52huq %zmm1,%zmm27,%zmm15
   1bab2:	62 a2 fd 40 b5 e9    	vpmadd52huq %zmm17,%zmm16,%zmm21
   1bab8:	62 c2 c5 48 b4 f3    	vpmadd52luq %zmm11,%zmm7,%zmm22
   1babe:	62 c2 cd 48 b4 db    	vpmadd52luq %zmm11,%zmm6,%zmm19
   1bac4:	62 c2 bd 48 b4 e3    	vpmadd52luq %zmm11,%zmm8,%zmm20
   1baca:	62 72 ad 40 b4 f9    	vpmadd52luq %zmm1,%zmm26,%zmm15
   1bad0:	62 82 fd 40 b5 f1    	vpmadd52huq %zmm25,%zmm16,%zmm22
   1bad6:	62 82 fd 40 b5 e0    	vpmadd52huq %zmm24,%zmm16,%zmm20
   1badc:	62 82 fd 40 b4 d9    	vpmadd52luq %zmm25,%zmm16,%zmm19
   1bae2:	62 52 bd 48 b5 fb    	vpmadd52huq %zmm11,%zmm8,%zmm15
   1bae8:	62 82 fd 40 b4 f0    	vpmadd52luq %zmm24,%zmm16,%zmm22
   1baee:	62 a1 e5 40 d4 ed    	vpaddq %zmm21,%zmm19,%zmm21
   1baf4:	62 a1 65 00 ef db    	vpxord %xmm19,%xmm19,%xmm19
   1bafa:	62 e2 ad 40 b5 d9    	vpmadd52huq %zmm1,%zmm26,%zmm19
   1bb00:	62 a2 fd 40 b4 e7    	vpmadd52luq %zmm23,%zmm16,%zmm20
   1bb06:	62 e2 8d 40 b4 ea    	vpmadd52luq %zmm2,%zmm30,%zmm21
   1bb0c:	62 52 b5 48 b4 fb    	vpmadd52luq %zmm11,%zmm9,%zmm15
   1bb12:	62 e2 8d 40 b5 f2    	vpmadd52huq %zmm2,%zmm30,%zmm22
   1bb18:	62 c2 b5 48 b5 db    	vpmadd52huq %zmm11,%zmm9,%zmm19
   1bb1e:	62 e2 95 40 b5 e2    	vpmadd52huq %zmm2,%zmm29,%zmm20
   1bb24:	62 c2 d5 48 b4 ec    	vpmadd52luq %zmm12,%zmm5,%zmm21
   1bb2a:	62 32 fd 40 b5 ff    	vpmadd52huq %zmm23,%zmm16,%zmm15
   1bb30:	62 e2 95 40 b4 f2    	vpmadd52luq %zmm2,%zmm29,%zmm22
   1bb36:	62 82 fd 40 b5 df    	vpmadd52huq %zmm31,%zmm16,%zmm19
   1bb3c:	62 e2 9d 40 b4 e2    	vpmadd52luq %zmm2,%zmm28,%zmm20
   1bb42:	62 12 fd 40 b4 ff    	vpmadd52luq %zmm31,%zmm16,%zmm15
   1bb48:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1bb4e:	62 e2 d5 40 b4 c0    	vpmadd52luq %zmm0,%zmm21,%zmm16
   1bb54:	62 a1 d5 40 d4 ea    	vpaddq %zmm18,%zmm21,%zmm21
   1bb5a:	62 b1 d5 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm21
   1bb61:	62 c2 d5 48 b5 f4    	vpmadd52huq %zmm12,%zmm5,%zmm22
   1bb67:	62 e2 a5 40 b5 da    	vpmadd52huq %zmm2,%zmm27,%zmm19
   1bb6d:	62 c2 cd 48 b5 e4    	vpmadd52huq %zmm12,%zmm6,%zmm20
   1bb73:	62 72 9d 40 b5 fa    	vpmadd52huq %zmm2,%zmm28,%zmm15
   1bb79:	62 a2 fd 40 b5 e9    	vpmadd52huq %zmm17,%zmm16,%zmm21
   1bb7f:	62 c2 cd 48 b4 f4    	vpmadd52luq %zmm12,%zmm6,%zmm22
   1bb85:	62 e2 ad 40 b4 da    	vpmadd52luq %zmm2,%zmm26,%zmm19
   1bb8b:	62 c2 c5 48 b4 e4    	vpmadd52luq %zmm12,%zmm7,%zmm20
   1bb91:	62 72 a5 40 b4 fa    	vpmadd52luq %zmm2,%zmm27,%zmm15
   1bb97:	62 82 fd 40 b4 f1    	vpmadd52luq %zmm25,%zmm16,%zmm22
   1bb9d:	62 c2 bd 48 b5 dc    	vpmadd52huq %zmm12,%zmm8,%zmm19
   1bba3:	62 82 fd 40 b5 e1    	vpmadd52huq %zmm25,%zmm16,%zmm20
   1bba9:	62 52 c5 48 b5 fc    	vpmadd52huq %zmm12,%zmm7,%zmm15
   1bbaf:	62 a1 cd 40 d4 ed    	vpaddq %zmm21,%zmm22,%zmm21
   1bbb5:	62 a1 4d 00 ef f6    	vpxord %xmm22,%xmm22,%xmm22
   1bbbb:	62 e2 ad 40 b5 f2    	vpmadd52huq %zmm2,%zmm26,%zmm22
   1bbc1:	62 c2 b5 48 b4 dc    	vpmadd52luq %zmm12,%zmm9,%zmm19
   1bbc7:	62 82 fd 40 b4 e0    	vpmadd52luq %zmm24,%zmm16,%zmm20
   1bbcd:	62 e2 8d 40 b4 eb    	vpmadd52luq %zmm3,%zmm30,%zmm21
   1bbd3:	62 52 bd 48 b4 fc    	vpmadd52luq %zmm12,%zmm8,%zmm15
   1bbd9:	62 c2 b5 48 b5 f4    	vpmadd52huq %zmm12,%zmm9,%zmm22
   1bbdf:	62 a2 fd 40 b5 df    	vpmadd52huq %zmm23,%zmm16,%zmm19
   1bbe5:	62 e2 8d 40 b5 e3    	vpmadd52huq %zmm3,%zmm30,%zmm20
   1bbeb:	62 c2 d5 48 b4 ed    	vpmadd52luq %zmm13,%zmm5,%zmm21
   1bbf1:	62 12 fd 40 b5 f8    	vpmadd52huq %zmm24,%zmm16,%zmm15
   1bbf7:	62 82 fd 40 b5 f7    	vpmadd52huq %zmm31,%zmm16,%zmm22
   1bbfd:	62 82 fd 40 b4 df    	vpmadd52luq %zmm31,%zmm16,%zmm19
   1bc03:	62 e2 95 40 b4 e3    	vpmadd52luq %zmm3,%zmm29,%zmm20
   1bc09:	62 32 fd 40 b4 ff    	vpmadd52luq %zmm23,%zmm16,%zmm15
   1bc0f:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1bc15:	62 e2 d5 40 b4 c0    	vpmadd52luq %zmm0,%zmm21,%zmm16
   1bc1b:	62 a1 d5 40 d4 ea    	vpaddq %zmm18,%zmm21,%zmm21
   1bc21:	62 b1 d5 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm21
   1bc28:	62 e2 a5 40 b5 f3    	vpmadd52huq %zmm3,%zmm27,%zmm22
   1bc2e:	62 e2 9d 40 b5 db    	vpmadd52huq %zmm3,%zmm28,%zmm19
   1bc34:	62 c2 d5 48 b5 e5    	vpmadd52huq %zmm13,%zmm5,%zmm20
   1bc3a:	62 72 95 40 b5 fb    	vpmadd52huq %zmm3,%zmm29,%zmm15
   1bc40:	62 a2 fd 40 b5 e9    	vpmadd52huq %zmm17,%zmm16,%zmm21
   1bc46:	62 e2 ad 40 b4 f3    	vpmadd52luq %zmm3,%zmm26,%zmm22
   1bc4c:	62 e2 a5 40 b4 db    	vpmadd52luq %zmm3,%zmm27,%zmm19
   1bc52:	62 c2 cd 48 b4 e5    	vpmadd52luq %zmm13,%zmm6,%zmm20
   1bc58:	62 72 9d 40 b4 fb    	vpmadd52luq %zmm3,%zmm28,%zmm15
   1bc5e:	62 c2 bd 48 b5 f5    	vpmadd52huq %zmm13,%zmm8,%zmm22
   1bc64:	62 c2 c5 48 b5 dd    	vpmadd52huq %zmm13,%zmm7,%zmm19
   1bc6a:	62 82 fd 40 b4 e1    	vpmadd52luq %zmm25,%zmm16,%zmm20
   1bc70:	62 52 cd 48 b5 fd    	vpmadd52huq %zmm13,%zmm6,%zmm15
   1bc76:	62 c2 b5 48 b4 f5    	vpmadd52luq %zmm13,%zmm9,%zmm22
   1bc7c:	62 a1 dd 40 d4 ed    	vpaddq %zmm21,%zmm20,%zmm21
   1bc82:	62 a1 5d 00 ef e4    	vpxord %xmm20,%xmm20,%xmm20
   1bc88:	62 e2 ad 40 b5 e3    	vpmadd52huq %zmm3,%zmm26,%zmm20
   1bc8e:	62 c2 bd 48 b4 dd    	vpmadd52luq %zmm13,%zmm8,%zmm19
   1bc94:	62 e2 8d 40 b4 ec    	vpmadd52luq %zmm4,%zmm30,%zmm21
   1bc9a:	62 52 c5 48 b4 fd    	vpmadd52luq %zmm13,%zmm7,%zmm15
   1bca0:	62 a2 fd 40 b5 f7    	vpmadd52huq %zmm23,%zmm16,%zmm22
   1bca6:	62 c2 b5 48 b5 e5    	vpmadd52huq %zmm13,%zmm9,%zmm20
   1bcac:	62 82 fd 40 b5 d8    	vpmadd52huq %zmm24,%zmm16,%zmm19
   1bcb2:	62 c2 d5 48 b4 ee    	vpmadd52luq %zmm14,%zmm5,%zmm21
   1bcb8:	62 12 fd 40 b5 f9    	vpmadd52huq %zmm25,%zmm16,%zmm15
   1bcbe:	62 82 fd 40 b4 f7    	vpmadd52luq %zmm31,%zmm16,%zmm22
   1bcc4:	62 a2 fd 40 b4 df    	vpmadd52luq %zmm23,%zmm16,%zmm19
   1bcca:	62 82 fd 40 b5 e7    	vpmadd52huq %zmm31,%zmm16,%zmm20
   1bcd0:	62 12 fd 40 b4 f8    	vpmadd52luq %zmm24,%zmm16,%zmm15
   1bcd6:	62 a1 7d 00 ef c0    	vpxord %xmm16,%xmm16,%xmm16
   1bcdc:	62 e2 d5 40 b4 c0    	vpmadd52luq %zmm0,%zmm21,%zmm16
   1bce2:	62 e1 d5 50 d4 2d 1c 	vpaddq -0x17ee4(%rip){1to8},%zmm21,%zmm21        # 3e08 <__abi_tag+0x3b0c>
   1bce9:	81 fe ff
   1bcec:	62 e2 9d 40 b5 f4    	vpmadd52huq %zmm4,%zmm28,%zmm22
   1bcf2:	62 e2 95 40 b5 dc    	vpmadd52huq %zmm4,%zmm29,%zmm19
   1bcf8:	62 e2 a5 40 b5 e4    	vpmadd52huq %zmm4,%zmm27,%zmm20
   1bcfe:	62 72 8d 40 b5 fc    	vpmadd52huq %zmm4,%zmm30,%zmm15
   1bd04:	62 e2 a5 40 b4 f4    	vpmadd52luq %zmm4,%zmm27,%zmm22
   1bd0a:	62 e2 9d 40 b4 dc    	vpmadd52luq %zmm4,%zmm28,%zmm19
   1bd10:	62 e2 ad 40 b4 e4    	vpmadd52luq %zmm4,%zmm26,%zmm20
   1bd16:	62 01 25 00 ef db    	vpxord %xmm27,%xmm27,%xmm27
   1bd1c:	62 62 ad 40 b5 dc    	vpmadd52huq %zmm4,%zmm26,%zmm27
   1bd22:	62 b1 ad 40 73 f0 04 	vpsllq $0x4,%zmm16,%zmm26
   1bd29:	62 e1 fd 50 db 05 d5 	vpandq -0x17f2b(%rip){1to8},%zmm16,%zmm16        # 3e08 <__abi_tag+0x3b0c>
   1bd30:	80 fe ff
   1bd33:	62 72 95 40 b4 fc    	vpmadd52luq %zmm4,%zmm29,%zmm15
   1bd39:	62 b1 d5 40 73 d5 30 	vpsrlq $0x30,%zmm21,%zmm21
   1bd40:	62 a2 ad 40 b5 e9    	vpmadd52huq %zmm17,%zmm26,%zmm21
   1bd46:	62 e2 fd 48 59 0d 50 	vpbroadcastq -0x17eb0(%rip),%zmm17        # 3ea0 <__abi_tag+0x3ba4>
   1bd4d:	81 fe ff
   1bd50:	62 c2 c5 48 b5 f6    	vpmadd52huq %zmm14,%zmm7,%zmm22
   1bd56:	62 c2 cd 48 b5 de    	vpmadd52huq %zmm14,%zmm6,%zmm19
   1bd5c:	62 c2 bd 48 b5 e6    	vpmadd52huq %zmm14,%zmm8,%zmm20
   1bd62:	62 42 b5 48 b5 de    	vpmadd52huq %zmm14,%zmm9,%zmm27
   1bd68:	62 52 d5 48 b5 fe    	vpmadd52huq %zmm14,%zmm5,%zmm15
   1bd6e:	62 c2 bd 48 b4 f6    	vpmadd52luq %zmm14,%zmm8,%zmm22
   1bd74:	62 c2 c5 48 b4 de    	vpmadd52luq %zmm14,%zmm7,%zmm19
   1bd7a:	62 c2 b5 48 b4 e6    	vpmadd52luq %zmm14,%zmm9,%zmm20
   1bd80:	62 02 fd 40 b5 df    	vpmadd52huq %zmm31,%zmm16,%zmm27
   1bd86:	62 52 cd 48 b4 fe    	vpmadd52luq %zmm14,%zmm6,%zmm15
   1bd8c:	62 82 fd 40 b5 f0    	vpmadd52huq %zmm24,%zmm16,%zmm22
   1bd92:	62 82 fd 40 b5 d9    	vpmadd52huq %zmm25,%zmm16,%zmm19
   1bd98:	62 a2 fd 40 b5 e7    	vpmadd52huq %zmm23,%zmm16,%zmm20
   1bd9e:	62 12 fd 40 b4 f9    	vpmadd52luq %zmm25,%zmm16,%zmm15
   1bda4:	62 a2 fd 40 b4 f7    	vpmadd52luq %zmm23,%zmm16,%zmm22
   1bdaa:	62 82 fd 40 b4 d8    	vpmadd52luq %zmm24,%zmm16,%zmm19
   1bdb0:	62 82 fd 40 b4 e7    	vpmadd52luq %zmm31,%zmm16,%zmm20
   1bdb6:	62 91 bd 40 73 f3 04 	vpsllq $0x4,%zmm27,%zmm24
   1bdbd:	62 d1 fd 40 73 f7 04 	vpsllq $0x4,%zmm15,%zmm16
   1bdc4:	62 d1 85 48 73 d7 30 	vpsrlq $0x30,%zmm15,%zmm15
   1bdcb:	62 a1 fd 40 db c1    	vpandq %zmm17,%zmm16,%zmm16
   1bdd1:	62 b1 c5 40 73 f6 04 	vpsllq $0x4,%zmm22,%zmm23
   1bdd8:	62 a1 fd 40 d4 c5    	vpaddq %zmm21,%zmm16,%zmm16
   1bdde:	62 b1 d5 40 73 d3 30 	vpsrlq $0x30,%zmm19,%zmm21
   1bde5:	62 b1 fd 48 73 f4 04 	vpsllq $0x4,%zmm20,%zmm0
   1bdec:	62 b1 e5 40 73 f3 04 	vpsllq $0x4,%zmm19,%zmm19
   1bdf3:	62 b1 cd 40 73 d6 30 	vpsrlq $0x30,%zmm22,%zmm22
   1bdfa:	62 b1 dd 40 73 d4 30 	vpsrlq $0x30,%zmm20,%zmm20
   1be01:	62 a1 c5 40 db f9    	vpandq %zmm17,%zmm23,%zmm23
   1be07:	62 a1 e5 40 db d9    	vpandq %zmm17,%zmm19,%zmm19
   1be0d:	62 a1 bd 40 d4 e4    	vpaddq %zmm20,%zmm24,%zmm20
   1be13:	62 21 fd 40 db f2    	vpandq %zmm18,%zmm16,%zmm30
   1be19:	62 a1 c5 40 d4 ed    	vpaddq %zmm21,%zmm23,%zmm21
   1be1f:	62 a1 fd 48 db f9    	vpandq %zmm17,%zmm0,%zmm23
   1be25:	62 b1 fd 48 73 d0 34 	vpsrlq $0x34,%zmm16,%zmm0
   1be2c:	62 51 fd 48 d4 ff    	vpaddq %zmm15,%zmm0,%zmm15
   1be32:	62 a1 c5 40 d4 f6    	vpaddq %zmm22,%zmm23,%zmm22
   1be38:	62 31 85 48 d4 fb    	vpaddq %zmm19,%zmm15,%zmm15
   1be3e:	62 d1 e5 40 73 d7 34 	vpsrlq $0x34,%zmm15,%zmm19
   1be45:	62 21 85 48 db ea    	vpandq %zmm18,%zmm15,%zmm29
   1be4b:	62 a1 e5 40 d4 dd    	vpaddq %zmm21,%zmm19,%zmm19
   1be51:	62 b1 d5 40 73 d3 34 	vpsrlq $0x34,%zmm19,%zmm21
   1be58:	62 21 e5 40 db e2    	vpandq %zmm18,%zmm19,%zmm28
   1be5e:	62 a1 d5 40 d4 ee    	vpaddq %zmm22,%zmm21,%zmm21
   1be64:	62 b1 cd 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm22
   1be6b:	62 21 d5 40 db da    	vpandq %zmm18,%zmm21,%zmm27
   1be71:	62 21 cd 40 d4 d4    	vpaddq %zmm20,%zmm22,%zmm26
   1be77:	62 f3 ad 50 1e 05 36 	vpcmpnleuq -0x17fca(%rip){1to8},%zmm26,%k0        # 3eb8 <__abi_tag+0x3bbc>
   1be7e:	80 fe ff 06
   1be82:	c5 f9 98 c0          	kortestb %k0,%k0
   1be86:	75 0e                	jne    1be96 <probe::fused::<zopro_bn254::FqConfig, true, true>+0x656>
   1be88:	49 ff c9             	dec    %r9
   1be8b:	0f 85 6f fa ff ff    	jne    1b900 <probe::fused::<zopro_bn254::FqConfig, true, true>+0xc0>
   1be91:	e9 ac 00 00 00       	jmp    1bf42 <probe::fused::<zopro_bn254::FqConfig, true, true>+0x702>
   1be96:	c5 7c 28 3d 42 7d fe 	vmovaps -0x182be(%rip),%ymm15        # 3be0 <__abi_tag+0x38e4>
   1be9d:	ff
   1be9e:	62 e1 8d 50 d4 2d 08 	vpaddq -0x17ff8(%rip){1to8},%zmm30,%zmm21        # 3eb0 <__abi_tag+0x3bb4>
   1bea5:	80 fe ff
   1bea8:	62 e2 fd 48 59 0d 4e 	vpbroadcastq -0x180b2(%rip),%zmm17        # 3e00 <__abi_tag+0x3b04>
   1beaf:	7f fe ff
   1beb2:	c5 7c 11 3c 24       	vmovups %ymm15,(%rsp)
   1beb7:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   1bebc:	62 b1 cd 40 72 e5 34 	vpsraq $0x34,%zmm21,%zmm22
   1bec3:	62 71 95 50 fb 7c 24 	vpsubq 0x8(%rsp){1to8},%zmm29,%zmm15
   1beca:	01
   1becb:	62 e1 9d 50 fb 44 24 	vpsubq 0x10(%rsp){1to8},%zmm28,%zmm16
   1bed2:	02
   1bed3:	62 e1 a5 50 fb 5c 24 	vpsubq 0x18(%rsp){1to8},%zmm27,%zmm19
   1beda:	03
   1bedb:	62 e1 ad 50 fb 64 24 	vpsubq 0x20(%rsp){1to8},%zmm26,%zmm20
   1bee2:	04
   1bee3:	62 31 85 48 d4 fe    	vpaddq %zmm22,%zmm15,%zmm15
   1bee9:	62 d1 cd 40 72 e7 34 	vpsraq $0x34,%zmm15,%zmm22
   1bef0:	62 a1 fd 40 d4 c6    	vpaddq %zmm22,%zmm16,%zmm16
   1bef6:	62 b1 cd 40 72 e0 34 	vpsraq $0x34,%zmm16,%zmm22
   1befd:	62 a1 e5 40 d4 de    	vpaddq %zmm22,%zmm19,%zmm19
   1bf03:	62 b1 cd 40 72 e3 34 	vpsraq $0x34,%zmm19,%zmm22
   1bf0a:	62 a1 dd 40 d4 e6    	vpaddq %zmm22,%zmm20,%zmm20
   1bf10:	62 f3 dd 50 1e 0d 35 	vpcmpltuq -0x180cb(%rip){1to8},%zmm20,%k1        # 3e50 <__abi_tag+0x3b54>
   1bf17:	7f fe ff 01
   1bf1b:	62 21 d5 41 db f1    	vpandq %zmm17,%zmm21,%zmm30{%k1}
   1bf21:	62 21 85 49 db e9    	vpandq %zmm17,%zmm15,%zmm29{%k1}
   1bf27:	62 21 fd 41 db e1    	vpandq %zmm17,%zmm16,%zmm28{%k1}
   1bf2d:	62 21 e5 41 db d9    	vpandq %zmm17,%zmm19,%zmm27{%k1}
   1bf33:	62 21 dd 41 db d1    	vpandq %zmm17,%zmm20,%zmm26{%k1}
   1bf39:	49 ff c9             	dec    %r9
   1bf3c:	0f 85 be f9 ff ff    	jne    1b900 <probe::fused::<zopro_bn254::FqConfig, true, true>+0xc0>
   1bf42:	62 61 fd 48 7f 36    	vmovdqa64 %zmm30,(%rsi)
   1bf48:	62 61 fd 48 7f 6e 01 	vmovdqa64 %zmm29,0x40(%rsi)
   1bf4f:	62 61 fd 48 7f 66 02 	vmovdqa64 %zmm28,0x80(%rsi)
   1bf56:	62 61 fd 48 7f 5e 03 	vmovdqa64 %zmm27,0xc0(%rsi)
   1bf5d:	62 61 fd 48 7f 56 04 	vmovdqa64 %zmm26,0x100(%rsi)
   1bf64:	48 89 e0             	mov    %rsp,%rax
   1bf67:	62 f1 7c 48 28 46 04 	vmovaps 0x100(%rsi),%zmm0
   1bf6e:	62 f1 7c 48 28 2e    	vmovaps (%rsi),%zmm5
   1bf74:	62 f1 7c 48 28 4e 01 	vmovaps 0x40(%rsi),%zmm1
   1bf7b:	62 f1 7c 48 28 56 02 	vmovaps 0x80(%rsi),%zmm2
   1bf82:	62 f1 7c 48 28 5e 03 	vmovaps 0xc0(%rsi),%zmm3
   1bf89:	62 f1 7c 48 29 44 24 	vmovaps %zmm0,0x100(%rsp)
   1bf90:	04
   1bf91:	62 f1 7c 48 29 5c 24 	vmovaps %zmm3,0xc0(%rsp)
   1bf98:	03
   1bf99:	62 f1 7c 48 29 54 24 	vmovaps %zmm2,0x80(%rsp)
   1bfa0:	02
   1bfa1:	62 f1 7c 48 29 4c 24 	vmovaps %zmm1,0x40(%rsp)
   1bfa8:	01
   1bfa9:	62 f1 7c 48 29 2c 24 	vmovaps %zmm5,(%rsp)
   1bfb0:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1bfb7:	01
   1bfb8:	62 f1 7c 48 28 44 24 	vmovaps 0x100(%rsp),%zmm0
   1bfbf:	04
   1bfc0:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1bfc7:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1bfce:	02
   1bfcf:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1bfd6:	03
   1bfd7:	62 f1 7c 48 28 2c 24 	vmovaps (%rsp),%zmm5
   1bfde:	62 f1 7c 48 29 4f 01 	vmovaps %zmm1,0x40(%rdi)
   1bfe5:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1bfec:	04
   1bfed:	62 f1 7c 48 29 5f 03 	vmovaps %zmm3,0xc0(%rdi)
   1bff4:	62 f1 7c 48 29 57 02 	vmovaps %zmm2,0x80(%rdi)
   1bffb:	62 f1 7c 48 29 27    	vmovaps %zmm4,(%rdi)
   1c001:	62 f1 7c 48 29 47 04 	vmovaps %zmm0,0x100(%rdi)
   1c008:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1c00f:	02
   1c010:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1c017:	03
   1c018:	62 f1 7c 48 29 6f 05 	vmovaps %zmm5,0x140(%rdi)
   1c01f:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1c026:	62 f1 7c 48 29 4f 09 	vmovaps %zmm1,0x240(%rdi)
   1c02d:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1c034:	01
   1c035:	62 f1 7c 48 29 57 07 	vmovaps %zmm2,0x1c0(%rdi)
   1c03c:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1c043:	02
   1c044:	62 f1 7c 48 29 5f 08 	vmovaps %zmm3,0x200(%rdi)
   1c04b:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1c052:	03
   1c053:	62 f1 7c 48 29 4f 06 	vmovaps %zmm1,0x180(%rdi)
   1c05a:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1c061:	01
   1c062:	62 f1 7c 48 29 57 0c 	vmovaps %zmm2,0x300(%rdi)
   1c069:	62 f1 7c 48 28 54 24 	vmovaps 0x100(%rsp),%zmm2
   1c070:	04
   1c071:	62 f1 7c 48 29 67 0a 	vmovaps %zmm4,0x280(%rdi)
   1c078:	62 f1 7c 48 29 5f 0d 	vmovaps %zmm3,0x340(%rdi)
   1c07f:	62 f1 7c 48 28 24 24 	vmovaps (%rsp),%zmm4
   1c086:	62 f1 7c 48 28 5c 24 	vmovaps 0xc0(%rsp),%zmm3
   1c08d:	03
   1c08e:	62 f1 7c 48 29 4f 0b 	vmovaps %zmm1,0x2c0(%rdi)
   1c095:	62 f1 7c 48 28 4c 24 	vmovaps 0x100(%rsp),%zmm1
   1c09c:	04
   1c09d:	62 f1 7c 48 29 57 0e 	vmovaps %zmm2,0x380(%rdi)
   1c0a4:	62 f1 7c 48 28 54 24 	vmovaps 0x80(%rsp),%zmm2
   1c0ab:	02
   1c0ac:	62 f1 7c 48 29 5f 12 	vmovaps %zmm3,0x480(%rdi)
   1c0b3:	62 f1 7c 48 29 67 0f 	vmovaps %zmm4,0x3c0(%rdi)
   1c0ba:	62 f1 7c 48 29 4f 13 	vmovaps %zmm1,0x4c0(%rdi)
   1c0c1:	62 f1 7c 48 28 4c 24 	vmovaps 0x40(%rsp),%zmm1
   1c0c8:	01
   1c0c9:	62 f1 7c 48 29 57 11 	vmovaps %zmm2,0x440(%rdi)
   1c0d0:	62 f1 7c 48 29 4f 10 	vmovaps %zmm1,0x400(%rdi)
   1c0d7:	48 89 ec             	mov    %rbp,%rsp
   1c0da:	5d                   	pop    %rbp
   1c0db:	c5 f8 77             	vzeroupper
   1c0de:	c3                   	ret
   1c0df:	cc                   	int3

000000000001dc10 <probe::streams::<zopro_bn254::FqConfig, true, true>>:
   1dc10:	53                   	push   %rbx
   1dc11:	48 83 ec 30          	sub    $0x30,%rsp
   1dc15:	62 e1 fd 48 6f 0a    	vmovdqa64 (%rdx),%zmm17
   1dc1b:	62 f1 fd 48 6f 4a 01 	vmovdqa64 0x40(%rdx),%zmm1
   1dc22:	62 f1 fd 48 6f 52 02 	vmovdqa64 0x80(%rdx),%zmm2
   1dc29:	62 f1 fd 48 6f 5a 03 	vmovdqa64 0xc0(%rdx),%zmm3
   1dc30:	62 f1 fd 48 6f 62 04 	vmovdqa64 0x100(%rdx),%zmm4
   1dc37:	62 f1 fd 48 6f 31    	vmovdqa64 (%rcx),%zmm6
   1dc3d:	62 f1 fd 48 6f 79 01 	vmovdqa64 0x40(%rcx),%zmm7
   1dc44:	62 71 fd 48 6f 41 02 	vmovdqa64 0x80(%rcx),%zmm8
   1dc4b:	62 71 fd 48 6f 49 03 	vmovdqa64 0xc0(%rcx),%zmm9
   1dc52:	62 e1 fd 48 6f 41 04 	vmovdqa64 0x100(%rcx),%zmm16
   1dc59:	c5 7d 6f 15 7f 5b fe 	vmovdqa -0x1a481(%rip),%ymm10        # 37e0 <__abi_tag+0x34e4>
   1dc60:	ff
   1dc61:	62 72 fd 48 59 2d 8d 	vpbroadcastq -0x19d73(%rip),%zmm13        # 3ef8 <__abi_tag+0x3bfc>
   1dc68:	62 fe ff
   1dc6b:	62 f2 fd 48 59 05 8b 	vpbroadcastq -0x19e75(%rip),%zmm0        # 3e00 <__abi_tag+0x3b04>
   1dc72:	61 fe ff
   1dc75:	62 72 fd 48 59 25 b9 	vpbroadcastq -0x19e47(%rip),%zmm12        # 3e38 <__abi_tag+0x3b3c>
   1dc7c:	61 fe ff
   1dc7f:	62 e2 fd 48 59 1d 7f 	vpbroadcastq -0x19e81(%rip),%zmm19        # 3e08 <__abi_tag+0x3b0c>
   1dc86:	61 fe ff
   1dc89:	62 72 fd 48 59 1d 0d 	vpbroadcastq -0x19df3(%rip),%zmm11        # 3ea0 <__abi_tag+0x3ba4>
   1dc90:	62 fe ff
   1dc93:	c5 fc 28 2d 45 5f fe 	vmovaps -0x1a0bb(%rip),%ymm5        # 3be0 <__abi_tag+0x38e4>
   1dc9a:	ff
   1dc9b:	62 62 fd 48 59 25 0b 	vpbroadcastq -0x19df5(%rip),%zmm28        # 3eb0 <__abi_tag+0x3bb4>
   1dca2:	62 fe ff
   1dca5:	48 89 fb             	mov    %rdi,%rbx
   1dca8:	31 c0                	xor    %eax,%eax
   1dcaa:	48 b9 31 e1 72 4e 64 	movabs $0x30644e72e131,%rcx
   1dcb1:	30 00 00
   1dcb4:	48 ba 63 c2 e5 9c c8 	movabs $0x60c89ce5c263,%rdx
   1dcbb:	60 00 00
   1dcbe:	66 90                	xchg   %ax,%ax
   1dcc0:	bf 00 01 00 00       	mov    $0x100,%edi
   1dcc5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   1dccc:	00 00 00 00
   1dcd0:	62 61 fd 48 6f 6c 3e 	vmovdqa64 -0x100(%rsi,%rdi,1),%zmm29
   1dcd7:	fc
   1dcd8:	62 61 fd 48 6f 4c 3e 	vmovdqa64 -0x40(%rsi,%rdi,1),%zmm25
   1dcdf:	ff
   1dce0:	62 61 fd 48 6f 54 3e 	vmovdqa64 -0x80(%rsi,%rdi,1),%zmm26
   1dce7:	fe
   1dce8:	62 e1 7c 28 28 d5    	vmovaps %ymm5,%ymm18
   1dcee:	62 91 fd 48 6f ec    	vmovdqa64 %zmm28,%zmm5
   1dcf4:	62 61 fd 48 6f 64 3e 	vmovdqa64 -0xc0(%rsi,%rdi,1),%zmm28
   1dcfb:	fd
   1dcfc:	62 61 fd 48 6f 04 3e 	vmovdqa64 (%rsi,%rdi,1),%zmm24
   1dd03:	c5 7e 7f 14 24       	vmovdqu %ymm10,(%rsp)
   1dd08:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   1dd0d:	62 01 25 00 ef db    	vpxord %xmm27,%xmm27,%xmm27
   1dd13:	c4 41 09 ef f6       	vpxor  %xmm14,%xmm14,%xmm14
   1dd18:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   1dd1e:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   1dd23:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   1dd29:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1dd2e:	62 e2 fd 48 59 7c 24 	vpbroadcastq 0x8(%rsp),%zmm23
   1dd35:	01
   1dd36:	62 e2 fd 48 59 6c 24 	vpbroadcastq 0x18(%rsp),%zmm21
   1dd3d:	03
   1dd3e:	62 e2 fd 48 59 74 24 	vpbroadcastq 0x10(%rsp),%zmm22
   1dd45:	02
   1dd46:	62 e2 fd 48 59 64 24 	vpbroadcastq 0x20(%rsp),%zmm20
   1dd4d:	04
   1dd4e:	62 32 95 40 b5 f1    	vpmadd52huq %zmm17,%zmm29,%zmm14
   1dd54:	62 22 b5 40 b5 f9    	vpmadd52huq %zmm17,%zmm25,%zmm31
   1dd5a:	62 22 95 40 b4 d9    	vpmadd52luq %zmm17,%zmm29,%zmm27
   1dd60:	62 32 ad 40 b5 d1    	vpmadd52huq %zmm17,%zmm26,%zmm10
   1dd66:	62 22 9d 40 b5 f1    	vpmadd52huq %zmm17,%zmm28,%zmm30
   1dd6c:	62 52 a5 40 b4 fd    	vpmadd52luq %zmm13,%zmm27,%zmm15
   1dd72:	62 32 9d 40 b4 f1    	vpmadd52luq %zmm17,%zmm28,%zmm14
   1dd78:	62 22 bd 40 b4 f9    	vpmadd52luq %zmm17,%zmm24,%zmm31
   1dd7e:	62 32 b5 40 b4 d1    	vpmadd52luq %zmm17,%zmm25,%zmm10
   1dd84:	62 22 ad 40 b4 f1    	vpmadd52luq %zmm17,%zmm26,%zmm30
   1dd8a:	62 61 a5 40 d4 d8    	vpaddq %zmm0,%zmm27,%zmm27
   1dd90:	62 91 a5 40 73 d3 34 	vpsrlq $0x34,%zmm27,%zmm27
   1dd97:	62 22 85 48 b5 fd    	vpmadd52huq %zmm21,%zmm15,%zmm31
   1dd9d:	62 42 85 48 b5 dc    	vpmadd52huq %zmm12,%zmm15,%zmm27
   1dda3:	62 32 85 48 b4 f7    	vpmadd52luq %zmm23,%zmm15,%zmm14
   1dda9:	62 32 85 48 b5 d6    	vpmadd52huq %zmm22,%zmm15,%zmm10
   1ddaf:	62 22 85 48 b5 f7    	vpmadd52huq %zmm23,%zmm15,%zmm30
   1ddb5:	62 11 8d 48 d4 f3    	vpaddq %zmm27,%zmm14,%zmm14
   1ddbb:	62 01 25 00 ef db    	vpxord %xmm27,%xmm27,%xmm27
   1ddc1:	62 22 bd 40 b5 d9    	vpmadd52huq %zmm17,%zmm24,%zmm27
   1ddc7:	62 22 85 48 b4 fc    	vpmadd52luq %zmm20,%zmm15,%zmm31
   1ddcd:	62 32 85 48 b4 d5    	vpmadd52luq %zmm21,%zmm15,%zmm10
   1ddd3:	62 22 85 48 b4 f6    	vpmadd52luq %zmm22,%zmm15,%zmm30
   1ddd9:	62 72 95 40 b4 f1    	vpmadd52luq %zmm1,%zmm29,%zmm14
   1dddf:	62 22 85 48 b5 dc    	vpmadd52huq %zmm20,%zmm15,%zmm27
   1dde5:	62 62 ad 40 b5 f9    	vpmadd52huq %zmm1,%zmm26,%zmm31
   1ddeb:	62 72 9d 40 b5 d1    	vpmadd52huq %zmm1,%zmm28,%zmm10
   1ddf1:	62 62 95 40 b5 f1    	vpmadd52huq %zmm1,%zmm29,%zmm30
   1ddf7:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1ddfc:	62 52 8d 48 b4 fd    	vpmadd52luq %zmm13,%zmm14,%zmm15
   1de02:	62 71 8d 48 d4 f0    	vpaddq %zmm0,%zmm14,%zmm14
   1de08:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   1de0f:	62 62 b5 40 b4 f9    	vpmadd52luq %zmm1,%zmm25,%zmm31
   1de15:	62 62 b5 40 b5 d9    	vpmadd52huq %zmm1,%zmm25,%zmm27
   1de1b:	62 72 ad 40 b4 d1    	vpmadd52luq %zmm1,%zmm26,%zmm10
   1de21:	62 62 9d 40 b4 f1    	vpmadd52luq %zmm1,%zmm28,%zmm30
   1de27:	62 52 85 48 b5 f4    	vpmadd52huq %zmm12,%zmm15,%zmm14
   1de2d:	62 62 bd 40 b4 d9    	vpmadd52luq %zmm1,%zmm24,%zmm27
   1de33:	62 22 85 48 b5 fe    	vpmadd52huq %zmm22,%zmm15,%zmm31
   1de39:	62 32 85 48 b5 d7    	vpmadd52huq %zmm23,%zmm15,%zmm10
   1de3f:	62 22 85 48 b4 f7    	vpmadd52luq %zmm23,%zmm15,%zmm30
   1de45:	62 22 85 48 b4 fd    	vpmadd52luq %zmm21,%zmm15,%zmm31
   1de4b:	62 22 85 48 b5 dd    	vpmadd52huq %zmm21,%zmm15,%zmm27
   1de51:	62 32 85 48 b4 d6    	vpmadd52luq %zmm22,%zmm15,%zmm10
   1de57:	62 51 8d 40 d4 f6    	vpaddq %zmm14,%zmm30,%zmm14
   1de5d:	62 01 0d 00 ef f6    	vpxord %xmm30,%xmm30,%xmm30
   1de63:	62 62 bd 40 b5 f1    	vpmadd52huq %zmm1,%zmm24,%zmm30
   1de69:	62 72 95 40 b4 f2    	vpmadd52luq %zmm2,%zmm29,%zmm14
   1de6f:	62 22 85 48 b4 dc    	vpmadd52luq %zmm20,%zmm15,%zmm27
   1de75:	62 62 9d 40 b5 fa    	vpmadd52huq %zmm2,%zmm28,%zmm31
   1de7b:	62 72 95 40 b5 d2    	vpmadd52huq %zmm2,%zmm29,%zmm10
   1de81:	62 22 85 48 b5 f4    	vpmadd52huq %zmm20,%zmm15,%zmm30
   1de87:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1de8c:	62 52 8d 48 b4 fd    	vpmadd52luq %zmm13,%zmm14,%zmm15
   1de92:	62 71 8d 48 d4 f0    	vpaddq %zmm0,%zmm14,%zmm14
   1de98:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   1de9f:	62 62 ad 40 b4 fa    	vpmadd52luq %zmm2,%zmm26,%zmm31
   1dea5:	62 62 ad 40 b5 da    	vpmadd52huq %zmm2,%zmm26,%zmm27
   1deab:	62 72 9d 40 b4 d2    	vpmadd52luq %zmm2,%zmm28,%zmm10
   1deb1:	62 62 b5 40 b5 f2    	vpmadd52huq %zmm2,%zmm25,%zmm30
   1deb7:	62 52 85 48 b5 f4    	vpmadd52huq %zmm12,%zmm15,%zmm14
   1debd:	62 62 b5 40 b4 da    	vpmadd52luq %zmm2,%zmm25,%zmm27
   1dec3:	62 22 85 48 b5 ff    	vpmadd52huq %zmm23,%zmm15,%zmm31
   1dec9:	62 32 85 48 b4 d7    	vpmadd52luq %zmm23,%zmm15,%zmm10
   1decf:	62 62 bd 40 b4 f2    	vpmadd52luq %zmm2,%zmm24,%zmm30
   1ded5:	62 22 85 48 b4 fe    	vpmadd52luq %zmm22,%zmm15,%zmm31
   1dedb:	62 22 85 48 b5 de    	vpmadd52huq %zmm22,%zmm15,%zmm27
   1dee1:	62 51 ad 48 d4 f6    	vpaddq %zmm14,%zmm10,%zmm14
   1dee7:	c4 41 29 ef d2       	vpxor  %xmm10,%xmm10,%xmm10
   1deec:	62 72 bd 40 b5 d2    	vpmadd52huq %zmm2,%zmm24,%zmm10
   1def2:	62 22 85 48 b5 f5    	vpmadd52huq %zmm21,%zmm15,%zmm30
   1def8:	62 72 95 40 b4 f3    	vpmadd52luq %zmm3,%zmm29,%zmm14
   1defe:	62 22 85 48 b4 dd    	vpmadd52luq %zmm21,%zmm15,%zmm27
   1df04:	62 62 95 40 b5 fb    	vpmadd52huq %zmm3,%zmm29,%zmm31
   1df0a:	62 32 85 48 b5 d4    	vpmadd52huq %zmm20,%zmm15,%zmm10
   1df10:	62 22 85 48 b4 f4    	vpmadd52luq %zmm20,%zmm15,%zmm30
   1df16:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1df1b:	62 52 8d 48 b4 fd    	vpmadd52luq %zmm13,%zmm14,%zmm15
   1df21:	62 71 8d 48 d4 f0    	vpaddq %zmm0,%zmm14,%zmm14
   1df27:	62 d1 8d 48 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm14
   1df2e:	62 62 9d 40 b4 fb    	vpmadd52luq %zmm3,%zmm28,%zmm31
   1df34:	62 62 9d 40 b5 db    	vpmadd52huq %zmm3,%zmm28,%zmm27
   1df3a:	62 72 b5 40 b5 d3    	vpmadd52huq %zmm3,%zmm25,%zmm10
   1df40:	62 62 ad 40 b5 f3    	vpmadd52huq %zmm3,%zmm26,%zmm30
   1df46:	62 52 85 48 b5 f4    	vpmadd52huq %zmm12,%zmm15,%zmm14
   1df4c:	62 62 ad 40 b4 db    	vpmadd52luq %zmm3,%zmm26,%zmm27
   1df52:	62 22 85 48 b4 ff    	vpmadd52luq %zmm23,%zmm15,%zmm31
   1df58:	62 72 bd 40 b4 d3    	vpmadd52luq %zmm3,%zmm24,%zmm10
   1df5e:	62 62 b5 40 b4 f3    	vpmadd52luq %zmm3,%zmm25,%zmm30
   1df64:	62 51 85 40 d4 f6    	vpaddq %zmm14,%zmm31,%zmm14
   1df6a:	62 01 05 00 ef ff    	vpxord %xmm31,%xmm31,%xmm31
   1df70:	62 62 bd 40 b5 fb    	vpmadd52huq %zmm3,%zmm24,%zmm31
   1df76:	62 22 85 48 b5 df    	vpmadd52huq %zmm23,%zmm15,%zmm27
   1df7c:	62 32 85 48 b5 d5    	vpmadd52huq %zmm21,%zmm15,%zmm10
   1df82:	62 22 85 48 b5 f6    	vpmadd52huq %zmm22,%zmm15,%zmm30
   1df88:	62 72 95 40 b4 f4    	vpmadd52luq %zmm4,%zmm29,%zmm14
   1df8e:	62 22 85 48 b4 de    	vpmadd52luq %zmm22,%zmm15,%zmm27
   1df94:	62 22 85 48 b5 fc    	vpmadd52huq %zmm20,%zmm15,%zmm31
   1df9a:	62 32 85 48 b4 d4    	vpmadd52luq %zmm20,%zmm15,%zmm10
   1dfa0:	62 22 85 48 b4 f5    	vpmadd52luq %zmm21,%zmm15,%zmm30
   1dfa6:	c4 41 01 ef ff       	vpxor  %xmm15,%xmm15,%xmm15
   1dfab:	62 72 bd 40 b5 fc    	vpmadd52huq %zmm4,%zmm24,%zmm15
   1dfb1:	62 62 95 40 b5 dc    	vpmadd52huq %zmm4,%zmm29,%zmm27
   1dfb7:	62 62 b5 40 b5 fc    	vpmadd52huq %zmm4,%zmm25,%zmm31
   1dfbd:	62 62 7d 48 5a 2d e9 	vbroadcasti32x4 -0x19717(%rip),%zmm29        # 48b0 <__abi_tag+0x45b4>
   1dfc4:	68 fe ff
   1dfc7:	62 72 ad 40 b5 d4    	vpmadd52huq %zmm4,%zmm26,%zmm10
   1dfcd:	62 62 9d 40 b5 f4    	vpmadd52huq %zmm4,%zmm28,%zmm30
   1dfd3:	62 62 bd 40 b4 fc    	vpmadd52luq %zmm4,%zmm24,%zmm31
   1dfd9:	62 01 3d 00 ef c0    	vpxord %xmm24,%xmm24,%xmm24
   1dfdf:	62 42 8d 48 b4 c5    	vpmadd52luq %zmm13,%zmm14,%zmm24
   1dfe5:	62 72 b5 40 b4 d4    	vpmadd52luq %zmm4,%zmm25,%zmm10
   1dfeb:	62 62 ad 40 b4 f4    	vpmadd52luq %zmm4,%zmm26,%zmm30
   1dff1:	62 62 9d 40 b4 dc    	vpmadd52luq %zmm4,%zmm28,%zmm27
   1dff7:	62 31 8d 48 d4 f3    	vpaddq %zmm19,%zmm14,%zmm14
   1dffd:	62 61 fd 48 6f e5    	vmovdqa64 %zmm5,%zmm28
   1e003:	62 b1 7c 28 28 ea    	vmovaps %ymm18,%ymm5
   1e009:	c5 fc 11 2c 24       	vmovups %ymm5,(%rsp)
   1e00e:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
   1e013:	62 e2 fd 48 59 15 33 	vpbroadcastq -0x1a1cd(%rip),%zmm18        # 3e50 <__abi_tag+0x3b54>
   1e01a:	5e fe ff
   1e01d:	62 d1 8d 48 73 d6 30 	vpsrlq $0x30,%zmm14,%zmm14
   1e024:	62 91 b5 40 73 f0 04 	vpsllq $0x4,%zmm24,%zmm25
   1e02b:	62 01 bd 40 db c5    	vpandq %zmm29,%zmm24,%zmm24
   1e031:	62 32 bd 40 b5 d6    	vpmadd52huq %zmm22,%zmm24,%zmm10
   1e037:	62 22 bd 40 b5 f7    	vpmadd52huq %zmm23,%zmm24,%zmm30
   1e03d:	62 22 bd 40 b5 fd    	vpmadd52huq %zmm21,%zmm24,%zmm31
   1e043:	62 22 bd 40 b4 df    	vpmadd52luq %zmm23,%zmm24,%zmm27
   1e049:	62 52 b5 40 b5 f4    	vpmadd52huq %zmm12,%zmm25,%zmm14
   1e04f:	62 32 bd 40 b5 fc    	vpmadd52huq %zmm20,%zmm24,%zmm15
   1e055:	62 32 bd 40 b4 d5    	vpmadd52luq %zmm21,%zmm24,%zmm10
   1e05b:	62 22 bd 40 b4 f6    	vpmadd52luq %zmm22,%zmm24,%zmm30
   1e061:	62 22 bd 40 b4 fc    	vpmadd52luq %zmm20,%zmm24,%zmm31
   1e067:	62 91 dd 40 73 f3 04 	vpsllq $0x4,%zmm27,%zmm20
   1e06e:	62 d1 85 48 73 f7 04 	vpsllq $0x4,%zmm15,%zmm15
   1e075:	62 c1 dd 40 db e3    	vpandq %zmm11,%zmm20,%zmm20
   1e07b:	62 51 dd 40 d4 f6    	vpaddq %zmm14,%zmm20,%zmm14
   1e081:	62 91 dd 40 73 d3 30 	vpsrlq $0x30,%zmm27,%zmm20
   1e088:	62 d1 c5 40 73 f2 04 	vpsllq $0x4,%zmm10,%zmm23
   1e08f:	62 91 cd 40 73 d6 30 	vpsrlq $0x30,%zmm30,%zmm22
   1e096:	62 91 bd 40 73 f7 04 	vpsllq $0x4,%zmm31,%zmm24
   1e09d:	62 91 d5 40 73 f6 04 	vpsllq $0x4,%zmm30,%zmm21
   1e0a4:	62 d1 ad 48 73 d2 30 	vpsrlq $0x30,%zmm10,%zmm10
   1e0ab:	62 91 b5 40 73 d7 30 	vpsrlq $0x30,%zmm31,%zmm25
   1e0b2:	62 c1 c5 40 db fb    	vpandq %zmm11,%zmm23,%zmm23
   1e0b8:	62 c1 d5 40 db eb    	vpandq %zmm11,%zmm21,%zmm21
   1e0be:	62 11 85 48 d4 f9    	vpaddq %zmm25,%zmm15,%zmm15
   1e0c4:	62 a1 c5 40 d4 f6    	vpaddq %zmm22,%zmm23,%zmm22
   1e0ca:	62 c1 bd 40 db fb    	vpandq %zmm11,%zmm24,%zmm23
   1e0d0:	62 d1 bd 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm24
   1e0d7:	62 71 8d 48 db f0    	vpandq %zmm0,%zmm14,%zmm14
   1e0dd:	62 51 fd 40 d4 ff    	vpaddq %zmm15,%zmm16,%zmm15
   1e0e3:	62 a1 bd 40 d4 e4    	vpaddq %zmm20,%zmm24,%zmm20
   1e0e9:	62 51 c5 40 d4 d2    	vpaddq %zmm10,%zmm23,%zmm10
   1e0ef:	62 51 cd 48 d4 f6    	vpaddq %zmm14,%zmm6,%zmm14
   1e0f5:	62 a1 dd 40 d4 e5    	vpaddq %zmm21,%zmm20,%zmm20
   1e0fb:	62 d1 c5 40 73 d6 34 	vpsrlq $0x34,%zmm14,%zmm23
   1e102:	62 71 8d 48 db f0    	vpandq %zmm0,%zmm14,%zmm14
   1e108:	62 b1 d5 40 73 d4 34 	vpsrlq $0x34,%zmm20,%zmm21
   1e10f:	62 e1 dd 40 db e0    	vpandq %zmm0,%zmm20,%zmm20
   1e115:	62 01 8d 48 d4 d4    	vpaddq %zmm28,%zmm14,%zmm26
   1e11b:	62 a1 d5 40 d4 ee    	vpaddq %zmm22,%zmm21,%zmm21
   1e121:	62 91 a5 40 72 e2 34 	vpsraq $0x34,%zmm26,%zmm27
   1e128:	62 b1 cd 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm22
   1e12f:	62 e1 d5 40 db e8    	vpandq %zmm0,%zmm21,%zmm21
   1e135:	62 51 cd 40 d4 d2    	vpaddq %zmm10,%zmm22,%zmm10
   1e13b:	62 a1 bd 48 d4 ed    	vpaddq %zmm21,%zmm8,%zmm21
   1e141:	62 d1 cd 40 73 d2 34 	vpsrlq $0x34,%zmm10,%zmm22
   1e148:	62 71 ad 48 db d0    	vpandq %zmm0,%zmm10,%zmm10
   1e14e:	62 31 85 48 d4 fe    	vpaddq %zmm22,%zmm15,%zmm15
   1e154:	62 e1 c5 40 d4 f7    	vpaddq %zmm7,%zmm23,%zmm22
   1e15a:	62 51 b5 48 d4 d2    	vpaddq %zmm10,%zmm9,%zmm10
   1e160:	62 a1 cd 40 d4 e4    	vpaddq %zmm20,%zmm22,%zmm20
   1e166:	62 b1 cd 40 73 d4 34 	vpsrlq $0x34,%zmm20,%zmm22
   1e16d:	62 e1 dd 40 db e0    	vpandq %zmm0,%zmm20,%zmm20
   1e173:	62 61 dd 50 fb 6c 24 	vpsubq 0x8(%rsp){1to8},%zmm20,%zmm29
   1e17a:	01
   1e17b:	62 a1 cd 40 d4 ed    	vpaddq %zmm21,%zmm22,%zmm21
   1e181:	62 b1 cd 40 73 d5 34 	vpsrlq $0x34,%zmm21,%zmm22
   1e188:	62 e1 d5 40 db e8    	vpandq %zmm0,%zmm21,%zmm21
   1e18e:	62 e1 d5 50 fb 7c 24 	vpsubq 0x10(%rsp){1to8},%zmm21,%zmm23
   1e195:	02
   1e196:	62 51 cd 40 d4 d2    	vpaddq %zmm10,%zmm22,%zmm10
   1e19c:	62 d1 cd 40 73 d2 34 	vpsrlq $0x34,%zmm10,%zmm22
   1e1a3:	62 71 ad 48 db d0    	vpandq %zmm0,%zmm10,%zmm10
   1e1a9:	62 61 ad 58 fb 44 24 	vpsubq 0x18(%rsp){1to8},%zmm10,%zmm24
   1e1b0:	03
   1e1b1:	62 51 cd 40 d4 ff    	vpaddq %zmm15,%zmm22,%zmm15
   1e1b7:	62 61 85 58 fb 4c 24 	vpsubq 0x20(%rsp){1to8},%zmm15,%zmm25
   1e1be:	04
   1e1bf:	62 81 95 40 d4 f3    	vpaddq %zmm27,%zmm29,%zmm22
   1e1c5:	62 b1 a5 40 72 e6 34 	vpsraq $0x34,%zmm22,%zmm27
   1e1cc:	62 81 c5 40 d4 fb    	vpaddq %zmm27,%zmm23,%zmm23
   1e1d2:	62 b1 a5 40 72 e7 34 	vpsraq $0x34,%zmm23,%zmm27
   1e1d9:	62 01 bd 40 d4 c3    	vpaddq %zmm27,%zmm24,%zmm24
   1e1df:	62 91 a5 40 72 e0 34 	vpsraq $0x34,%zmm24,%zmm27
   1e1e6:	62 01 b5 40 d4 cb    	vpaddq %zmm27,%zmm25,%zmm25
   1e1ec:	62 b3 b5 40 1e ca 01 	vpcmpltuq %zmm18,%zmm25,%k1
   1e1f3:	62 71 ad 41 db f0    	vpandq %zmm0,%zmm26,%zmm14{%k1}
   1e1f9:	62 e1 cd 41 db e0    	vpandq %zmm0,%zmm22,%zmm20{%k1}
   1e1ff:	62 e1 c5 41 db e8    	vpandq %zmm0,%zmm23,%zmm21{%k1}
   1e205:	62 71 bd 41 db d0    	vpandq %zmm0,%zmm24,%zmm10{%k1}
   1e20b:	62 71 b5 41 db f8    	vpandq %zmm0,%zmm25,%zmm15{%k1}
   1e211:	62 71 fd 48 7f 74 3e 	vmovdqa64 %zmm14,-0x100(%rsi,%rdi,1)
   1e218:	fc
   1e219:	62 e1 fd 48 7f 64 3e 	vmovdqa64 %zmm20,-0xc0(%rsi,%rdi,1)
   1e220:	fd
   1e221:	62 e1 fd 48 7f 6c 3e 	vmovdqa64 %zmm21,-0x80(%rsi,%rdi,1)
   1e228:	fe
   1e229:	62 71 fd 48 7f 54 3e 	vmovdqa64 %zmm10,-0x40(%rsi,%rdi,1)
   1e230:	ff
   1e231:	c5 7d 6f 15 a7 55 fe 	vmovdqa -0x1aa59(%rip),%ymm10        # 37e0 <__abi_tag+0x34e4>
   1e238:	ff
   1e239:	62 71 fd 48 7f 3c 3e 	vmovdqa64 %zmm15,(%rsi,%rdi,1)
   1e240:	48 81 c7 40 01 00 00 	add    $0x140,%rdi
   1e247:	48 81 ff 00 06 00 00 	cmp    $0x600,%rdi
   1e24e:	0f 85 7c fa ff ff    	jne    1dcd0 <probe::streams::<zopro_bn254::FqConfig, true, true>+0xc0>
   1e254:	48 ff c0             	inc    %rax
   1e257:	4c 39 c0             	cmp    %r8,%rax
   1e25a:	0f 85 60 fa ff ff    	jne    1dcc0 <probe::streams::<zopro_bn254::FqConfig, true, true>+0xb0>
   1e260:	ba 00 05 00 00       	mov    $0x500,%edx
   1e265:	48 89 df             	mov    %rbx,%rdi
   1e268:	c5 f8 77             	vzeroupper
   1e26b:	ff 15 3f 2a 07 00    	call   *0x72a3f(%rip)        # 90cb0 <memcpy@GLIBC_2.14>
   1e271:	48 83 c4 30          	add    $0x30,%rsp
   1e275:	5b                   	pop    %rbx
   1e276:	c3                   	ret
   1e277:	cc                   	int3
   1e278:	cc                   	int3
   1e279:	cc                   	int3
   1e27a:	cc                   	int3
   1e27b:	cc                   	int3
   1e27c:	cc                   	int3
   1e27d:	cc                   	int3
   1e27e:	cc                   	int3
   1e27f:	cc                   	int3
