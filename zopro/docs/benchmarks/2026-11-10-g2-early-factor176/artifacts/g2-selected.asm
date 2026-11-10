push   rbp
mov    rbp,rsp
push   r15
push   r14
push   r13
push   r12
push   rbx
and    rsp,0xffffffffffffffc0
sub    rsp,0xfc0
mov    QWORD PTR [rsp],0x0
sub    rsp,0xb00
vmovdqa ymm0,YMMWORD PTR [rip+0xfffffffffff29a91]        # a060 <GCC_except_table830+0x1b04>
movabs rax,0x1f16424e1bb7
vpbroadcastq zmm20,QWORD PTR [rip+0xfffffffffff292d5]        # 98b8 <GCC_except_table830+0x135c>
movabs r15,0x7fffffffffffff8
mov    rbx,r8
shr    rbx,0x3
movabs r12,0x30644e72e131
mov    QWORD PTR [rdi+0x10],0x0
mov    QWORD PTR [rsp+0x30],rcx
mov    r13,rdx
mov    QWORD PTR [rsp+0x38],rsi
mov    QWORD PTR [rsp+0x18],r8
mov    QWORD PTR [rsp+0x28],rdi
and    r15,r8
mov    QWORD PTR [rsp+0xf80],rbx
mov    QWORD PTR [rsp+0xf40],r15
vmovdqu YMMWORD PTR [rsp+0x5c0],ymm0
mov    QWORD PTR [rsp+0x5e0],rax
vmovdqa64 ZMMWORD PTR [rsp+0x14c0],zmm20
vpbroadcastq zmm19,QWORD PTR [rsp+0x5c8]
vpbroadcastq zmm18,QWORD PTR [rsp+0x5d0]
vpbroadcastq zmm17,QWORD PTR [rsp+0x5d8]
vpbroadcastq zmm16,QWORD PTR [rsp+0x5e0]
vmovdqa64 ZMMWORD PTR [rsp+0x1500],zmm19
vmovdqa64 ZMMWORD PTR [rsp+0x1540],zmm18
vmovdqa64 ZMMWORD PTR [rsp+0x1580],zmm17
vmovdqa64 ZMMWORD PTR [rsp+0x15c0],zmm16
cmp    rbx,QWORD PTR [rdi]
ja     ec725 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc185>
test   rbx,rbx
je     e17b4 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1214>
vpbroadcastq zmm20,QWORD PTR [rip+0xfffffffffff29207]        # 98b8 <GCC_except_table830+0x135c>
vmovdqa64 zmm22,ZMMWORD PTR [rip+0xfffffffffff28cc5]        # 9380 <GCC_except_table830+0xe24>
vmovdqa64 zmm23,ZMMWORD PTR [rip+0xfffffffffff28cfb]        # 93c0 <GCC_except_table830+0xe64>
vpbroadcastq zmm24,QWORD PTR [rip+0xfffffffffff29079]        # 9748 <GCC_except_table830+0x11ec>
vpbroadcastq zmm25,QWORD PTR [rip+0xfffffffffff290c7]        # 97a0 <GCC_except_table830+0x1244>
vpbroadcastq zmm28,QWORD PTR [rip+0xfffffffffff29145]        # 9828 <GCC_except_table830+0x12cc>
vpbroadcastq zmm29,QWORD PTR [rip+0xfffffffffff291ab]        # 9898 <GCC_except_table830+0x133c>
vpbroadcastq zmm30,QWORD PTR [rip+0xfffffffffff29251]        # 9948 <GCC_except_table830+0x13ec>
mov    rax,QWORD PTR [rsp+0x30]
xor    r14d,r14d
lea    rbx,[rax+0x78]
jmp    e1353 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xdb3>
nop    DWORD PTR [rax+rax*1+0x0]
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x200]
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x280]
vinserti64x4 zmm0,zmm13,YMMWORD PTR [rsp+0x980],0x1
vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x2c0]
vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x300]
vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x440]
vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x9c0]
vmovdqa64 zmm10,ZMMWORD PTR [rsp+0x240]
vmovdqa64 zmm11,ZMMWORD PTR [rsp+0x4c0]
vmovdqa64 zmm12,ZMMWORD PTR [rsp+0x480]
vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x940]
mov    rax,QWORD PTR [rdi+0x8]
lea    rcx,[r15+r15*4]
inc    r15
add    r14,0x8
sub    rbx,0xffffffffffffff80
vinserti64x4 zmm2,zmm1,YMMWORD PTR [rsp+0x380],0x1
vinserti64x4 zmm3,zmm4,YMMWORD PTR [rsp+0x340],0x1
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x900]
shl    rcx,0x7
vpunpcklqdq zmm1,zmm0,zmm2
vpunpckhqdq zmm0,zmm0,zmm2
vpunpcklqdq zmm2,zmm3,zmm4
vpunpckhqdq zmm3,zmm3,zmm4
vmovdqa64 zmm5,zmm0
vpermt2q zmm5,zmm22,zmm3
vpermt2q zmm0,zmm23,zmm3
vpunpcklqdq zmm3,zmm6,zmm7
vpunpckhqdq zmm6,zmm6,zmm7
vpunpcklqdq zmm7,zmm8,zmm9
vpunpckhqdq zmm8,zmm8,zmm9
vmovdqa64 zmm4,zmm1
vpermt2q zmm4,zmm22,zmm2
vpermt2q zmm1,zmm23,zmm2
vmovdqa64 zmm9,zmm3
vpermt2q zmm9,zmm22,zmm7
vpermt2q zmm3,zmm23,zmm7
vmovdqa64 zmm7,zmm6
vpermt2q zmm7,zmm22,zmm8
vpermt2q zmm6,zmm23,zmm8
vpunpcklqdq zmm8,zmm10,zmm11
vpunpckhqdq zmm10,zmm10,zmm11
vpunpcklqdq zmm11,zmm12,zmm13
vpunpckhqdq zmm12,zmm12,zmm13
vmovdqa64 zmm13,zmm8
vpermt2q zmm13,zmm22,zmm11
vpermt2q zmm8,zmm23,zmm11
vmovdqa64 zmm11,zmm10
vpermt2q zmm11,zmm22,zmm12
vpermt2q zmm10,zmm23,zmm12
vpandq zmm2,zmm4,zmm24
vpshldq zmm4,zmm5,zmm4,0xc
vpshldq zmm5,zmm1,zmm5,0x18
vpshldq zmm1,zmm0,zmm1,0x24
vpsrlq zmm0,zmm0,0x10
vpandq zmm12,zmm13,zmm24
vpandq zmm1,zmm1,zmm24
vpandq zmm4,zmm4,zmm24
vpandq zmm5,zmm5,zmm24
vpsubq zmm2,zmm12,zmm2
vpshldq zmm12,zmm11,zmm13,0xc
vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x180]
vpshldq zmm11,zmm8,zmm11,0x18
vpshldq zmm8,zmm10,zmm8,0x24
vpaddq zmm2,zmm2,zmm25
vpandq zmm8,zmm8,zmm24
vpandq zmm12,zmm12,zmm24
vpandq zmm11,zmm11,zmm24
vpsubq zmm1,zmm8,zmm1
vpsrlq zmm8,zmm10,0x10
vpsubq zmm4,zmm12,zmm4
vpunpckhqdq zmm10,zmm14,zmm15
vpsubq zmm5,zmm11,zmm5
vpandq zmm11,zmm9,zmm24
vpshldq zmm9,zmm7,zmm9,0xc
vpshldq zmm7,zmm3,zmm7,0x18
vpshldq zmm3,zmm6,zmm3,0x24
vpsrlq zmm6,zmm6,0x10
vpsubq zmm0,zmm8,zmm0
vpunpcklqdq zmm8,zmm14,zmm15
vmovdqa64 zmm15,zmm10
vpaddq zmm1,zmm1,zmm28
vmovdqa64 zmm14,zmm8
vpandq zmm9,zmm9,zmm24
vpandq zmm7,zmm7,zmm24
vpandq zmm3,zmm3,zmm24
vpaddq zmm0,zmm0,zmm29
vpunpcklqdq zmm12,zmm21,zmm13
vpunpckhqdq zmm13,zmm21,zmm13
vpermt2q zmm14,zmm22,zmm12
vpermt2q zmm15,zmm22,zmm13
vpermt2q zmm8,zmm23,zmm12
vpermt2q zmm10,zmm23,zmm13
vpandq zmm12,zmm14,zmm24
vpsubq zmm11,zmm12,zmm11
vpshldq zmm12,zmm15,zmm14,0xc
vpbroadcastq zmm14,QWORD PTR [rip+0xfffffffffff29015]        # 9990 <GCC_except_table830+0x1434>
vpandq zmm12,zmm12,zmm24
vpsubq zmm12,zmm12,zmm9
vpshldq zmm9,zmm8,zmm15,0x18
vpbroadcastq zmm15,QWORD PTR [rip+0xfffffffffff28e28]        # 97c0 <GCC_except_table830+0x1264>
vpandq zmm9,zmm9,zmm24
vpsubq zmm13,zmm9,zmm7
vpshldq zmm7,zmm10,zmm8,0x24
vpandq zmm9,zmm2,zmm24
vpandq zmm7,zmm7,zmm24
vpsubq zmm3,zmm7,zmm3
vpsrlq zmm7,zmm10,0x10
vpsubq zmm10,zmm7,zmm6
vpsraq zmm6,zmm2,0x34
vpaddq zmm3,zmm3,zmm28
vpaddq zmm4,zmm4,zmm6
vpaddq zmm5,zmm5,zmm15
vpaddq zmm21,zmm10,zmm29
vpxor  xmm10,xmm10,xmm10
vpmadd52luq zmm10,zmm9,zmm9
vpaddq zmm2,zmm4,zmm14
vpsraq zmm4,zmm2,0x34
vpandq zmm6,zmm2,zmm24
vpaddq zmm2,zmm5,zmm4
vpsraq zmm4,zmm2,0x34
vpandq zmm7,zmm2,zmm24
vpaddq zmm1,zmm1,zmm4
vpsraq zmm2,zmm1,0x34
vpandq zmm8,zmm1,zmm24
vpaddq zmm0,zmm0,zmm2
vpaddq zmm2,zmm11,zmm25
vpaddq zmm11,zmm13,zmm15
vpxor  xmm13,xmm13,xmm13
vpmadd52huq zmm13,zmm7,zmm9
vpxor  xmm15,xmm15,xmm15
vpsraq zmm1,zmm2,0x34
vpandq zmm4,zmm0,zmm24
vpandq zmm5,zmm2,zmm24
vpaddq zmm1,zmm12,zmm1
vpxor  xmm12,xmm12,xmm12
vpmadd52huq zmm12,zmm6,zmm9
vpmadd52luq zmm10,zmm5,zmm5
vpaddq zmm0,zmm1,zmm14
vpxor  xmm14,xmm14,xmm14
vpmadd52huq zmm14,zmm9,zmm9
vpsraq zmm2,zmm0,0x34
vpandq zmm1,zmm0,zmm24
vpmadd52luq zmm13,zmm8,zmm9
vpaddq zmm0,zmm11,zmm2
vpsraq zmm11,zmm0,0x34
vpandq zmm2,zmm0,zmm24
vpmadd52luq zmm12,zmm7,zmm9
vpmadd52luq zmm15,zmm10,zmm30
vpaddq zmm10,zmm10,zmm24
vpaddq zmm0,zmm3,zmm11
vpmadd52luq zmm14,zmm6,zmm9
vpsrlq zmm10,zmm10,0x34
vpsraq zmm11,zmm0,0x34
vpandq zmm3,zmm0,zmm24
vpmadd52huq zmm13,zmm2,zmm5
vpaddq zmm0,zmm21,zmm11
vpxor  xmm11,xmm11,xmm11
vpmadd52huq zmm11,zmm8,zmm9
vmovdqa64 zmm21,ZMMWORD PTR [rsp+0x1c0]
vpmadd52huq zmm12,zmm1,zmm5
vpandq zmm0,zmm0,zmm24
vpmadd52huq zmm10,zmm15,zmm25
vpmadd52huq zmm14,zmm5,zmm5
vpmadd52luq zmm13,zmm3,zmm5
vpmadd52luq zmm11,zmm4,zmm9
vpmadd52luq zmm12,zmm2,zmm5
vpmadd52luq zmm14,zmm1,zmm5
vpmadd52huq zmm13,zmm15,zmm26
vpmadd52huq zmm11,zmm3,zmm5
vpmadd52huq zmm12,zmm15,zmm27
vpmadd52luq zmm14,zmm15,zmm27
vpmadd52luq zmm13,zmm15,zmm21
vpmadd52luq zmm11,zmm0,zmm5
vpmadd52luq zmm12,zmm15,zmm26
vpaddq zmm14,zmm14,zmm10
vpxor  xmm10,xmm10,xmm10
vpmadd52huq zmm10,zmm4,zmm9
vpmadd52huq zmm13,zmm6,zmm6
vpmadd52luq zmm14,zmm9,zmm6
vpmadd52huq zmm11,zmm15,zmm21
vpmadd52huq zmm12,zmm9,zmm6
vpmadd52huq zmm10,zmm0,zmm5
vpmadd52luq zmm13,zmm7,zmm6
vpmadd52luq zmm14,zmm5,zmm1
vpmadd52luq zmm11,zmm15,zmm31
vpmadd52luq zmm12,zmm6,zmm6
vpmadd52huq zmm10,zmm15,zmm31
vpxor  xmm15,xmm15,xmm15
vpmadd52huq zmm13,zmm1,zmm1
vpmadd52luq zmm15,zmm14,zmm30
vpaddq zmm14,zmm14,zmm24
vpmadd52huq zmm11,zmm7,zmm6
vpsrlq zmm14,zmm14,0x34
vpmadd52huq zmm12,zmm5,zmm1
vpmadd52huq zmm10,zmm8,zmm6
vpmadd52luq zmm13,zmm2,zmm1
vpmadd52huq zmm14,zmm15,zmm25
vpmadd52luq zmm11,zmm8,zmm6
vpmadd52luq zmm12,zmm1,zmm1
vpmadd52luq zmm10,zmm4,zmm6
vpmadd52huq zmm13,zmm15,zmm27
vpmadd52huq zmm11,zmm2,zmm1
vpmadd52luq zmm12,zmm15,zmm27
vpmadd52huq zmm10,zmm3,zmm1
vpmadd52luq zmm13,zmm15,zmm26
vpmadd52luq zmm11,zmm3,zmm1
vpaddq zmm14,zmm12,zmm14
vpxor  xmm12,xmm12,xmm12
vpmadd52huq zmm12,zmm4,zmm6
vpmadd52luq zmm10,zmm0,zmm1
vpmadd52luq zmm14,zmm9,zmm7
vpmadd52huq zmm13,zmm9,zmm7
vpmadd52huq zmm11,zmm15,zmm26
vpmadd52huq zmm12,zmm0,zmm1
vpmadd52huq zmm10,zmm15,zmm21
vpmadd52luq zmm14,zmm5,zmm2
vpmadd52luq zmm13,zmm6,zmm7
vpmadd52luq zmm11,zmm15,zmm21
vpmadd52huq zmm12,zmm15,zmm31
vpmadd52luq zmm10,zmm15,zmm31
vpxor  xmm15,xmm15,xmm15
vpmadd52luq zmm15,zmm14,zmm30
vpaddq zmm14,zmm14,zmm24
vpmadd52huq zmm13,zmm5,zmm2
vpsrlq zmm14,zmm14,0x34
vpmadd52huq zmm11,zmm6,zmm7
vpmadd52huq zmm12,zmm8,zmm7
vpmadd52huq zmm10,zmm7,zmm7
vpmadd52huq zmm14,zmm15,zmm25
vpmadd52luq zmm13,zmm1,zmm2
vpmadd52luq zmm11,zmm7,zmm7
vpmadd52luq zmm12,zmm4,zmm7
vpmadd52luq zmm10,zmm8,zmm7
vpmadd52luq zmm13,zmm15,zmm27
vpmadd52huq zmm11,zmm1,zmm2
vpmadd52huq zmm12,zmm3,zmm2
vpmadd52huq zmm10,zmm2,zmm2
vpaddq zmm14,zmm13,zmm14
vpxor  xmm13,xmm13,xmm13
vpmadd52huq zmm13,zmm4,zmm7
vpmadd52luq zmm11,zmm2,zmm2
vpmadd52luq zmm14,zmm9,zmm8
vpmadd52luq zmm12,zmm0,zmm2
vpmadd52luq zmm10,zmm3,zmm2
vpmadd52huq zmm13,zmm0,zmm2
vpmadd52huq zmm11,zmm15,zmm27
vpmadd52luq zmm14,zmm5,zmm3
vpmadd52huq zmm12,zmm15,zmm21
vpmadd52huq zmm10,zmm15,zmm26
vpmadd52huq zmm13,zmm15,zmm31
vpmadd52luq zmm11,zmm15,zmm26
vpmadd52luq zmm12,zmm15,zmm31
vpmadd52luq zmm10,zmm15,zmm21
vpxor  xmm15,xmm15,xmm15
vpmadd52luq zmm15,zmm14,zmm30
vpaddq zmm14,zmm14,zmm24
vpmadd52huq zmm13,zmm8,zmm8
vpsrlq zmm14,zmm14,0x34
vpmadd52huq zmm11,zmm9,zmm8
vpmadd52huq zmm12,zmm7,zmm8
vpmadd52huq zmm10,zmm6,zmm8
vpmadd52huq zmm14,zmm15,zmm25
vpmadd52luq zmm13,zmm4,zmm8
vpmadd52luq zmm11,zmm6,zmm8
vpmadd52luq zmm12,zmm8,zmm8
vpmadd52luq zmm10,zmm7,zmm8
vpmadd52huq zmm13,zmm3,zmm3
vpmadd52huq zmm11,zmm5,zmm3
vpmadd52huq zmm12,zmm2,zmm3
vpmadd52huq zmm10,zmm1,zmm3
vpmadd52luq zmm13,zmm0,zmm3
vpmadd52luq zmm11,zmm1,zmm3
vpmadd52luq zmm12,zmm3,zmm3
vpmadd52luq zmm10,zmm2,zmm3
vpmadd52huq zmm13,zmm15,zmm21
vpmadd52luq zmm11,zmm15,zmm27
vpmadd52huq zmm12,zmm15,zmm26
vpmadd52huq zmm10,zmm15,zmm27
vpmadd52luq zmm13,zmm15,zmm31
vpaddq zmm14,zmm11,zmm14
vpxor  xmm11,xmm11,xmm11
vpmadd52huq zmm11,zmm4,zmm8
vpmadd52luq zmm12,zmm15,zmm21
vpmadd52luq zmm14,zmm9,zmm4
vpmadd52luq zmm10,zmm15,zmm26
vpmadd52huq zmm13,zmm7,zmm4
vpmadd52huq zmm11,zmm0,zmm3
vpmadd52huq zmm12,zmm6,zmm4
vpmadd52luq zmm14,zmm5,zmm0
vpmadd52huq zmm10,zmm9,zmm4
vpmadd52luq zmm13,zmm8,zmm4
vpmadd52huq zmm11,zmm15,zmm31
vpmadd52luq zmm12,zmm7,zmm4
vpxor  xmm7,xmm7,xmm7
vpmadd52luq zmm10,zmm6,zmm4
vpxor  xmm6,xmm6,xmm6
vpmadd52huq zmm6,zmm4,zmm4
vpmadd52huq zmm13,zmm2,zmm0
vpmadd52huq zmm11,zmm8,zmm4
vpxor  xmm8,xmm8,xmm8
vpmadd52huq zmm12,zmm1,zmm0
vpmadd52huq zmm10,zmm5,zmm0
vpmadd52huq zmm6,zmm0,zmm0
vpmadd52luq zmm13,zmm3,zmm0
vpmadd52luq zmm11,zmm4,zmm4
vpmadd52luq zmm12,zmm2,zmm0
vpmadd52luq zmm10,zmm1,zmm0
vpaddq zmm1,zmm14,zmm24
vpsrlq zmm1,zmm1,0x34
vpmadd52huq zmm11,zmm3,zmm0
vmovaps zmm3,ZMMWORD PTR [rsp+0x15c0]
vpmadd52luq zmm11,zmm0,zmm0
vpxor  xmm0,xmm0,xmm0
vpmadd52luq zmm0,zmm14,zmm30
vpxor  xmm14,xmm14,xmm14
vpmadd52huq zmm12,zmm0,zmm27
vpmadd52huq zmm13,zmm0,zmm26
vpmadd52huq zmm1,zmm0,zmm25
vpmadd52luq zmm10,zmm0,zmm27
vpmadd52huq zmm11,zmm0,zmm21
vpmadd52huq zmm6,zmm0,zmm31
vmovaps ZMMWORD PTR [rax+rcx*1+0x100],zmm3
vmovaps zmm3,ZMMWORD PTR [rsp+0x1540]
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x1580]
vpmadd52luq zmm12,zmm0,zmm26
vpaddq zmm1,zmm10,zmm1
vpmadd52luq zmm13,zmm0,zmm21
vpmadd52luq zmm11,zmm0,zmm31
vpsrlq zmm0,zmm1,0x34
vpandq zmm5,zmm1,zmm24
vpmadd52huq zmm8,zmm17,zmm5
vpmadd52luq zmm7,zmm20,zmm5
vpaddq zmm0,zmm0,zmm12
vpxor  xmm12,xmm12,xmm12
vpmadd52huq zmm12,zmm19,zmm5
vpsrlq zmm1,zmm0,0x34
vpandq zmm10,zmm0,zmm24
vpaddq zmm1,zmm1,zmm13
vpxor  xmm13,xmm13,xmm13
vpmadd52huq zmm13,zmm20,zmm5
vpmadd52luq zmm8,zmm16,zmm5
vpmadd52luq zmm14,zmm7,zmm30
vpsrlq zmm0,zmm1,0x34
vpandq zmm9,zmm1,zmm24
vmovaps zmm1,ZMMWORD PTR [rsp+0x14c0]
vpaddq zmm2,zmm0,zmm11
vpxor  xmm11,xmm11,xmm11
vpmadd52huq zmm11,zmm18,zmm5
vpmadd52luq zmm12,zmm18,zmm5
vpsrlq zmm0,zmm2,0x34
vpaddq zmm0,zmm0,zmm6
vpandq zmm6,zmm2,zmm24
vmovaps zmm2,ZMMWORD PTR [rsp+0x1500]
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0xc0],zmm4
vmovaps ZMMWORD PTR [rax+rcx*1+0x80],zmm3
vpmadd52luq zmm13,zmm19,zmm5
vpxor  xmm4,xmm4,xmm4
vpmadd52huq zmm4,zmm16,zmm5
vpmadd52luq zmm11,zmm17,zmm5
vmovaps ZMMWORD PTR [rax+rcx*1],zmm1
vmovaps ZMMWORD PTR [rax+rcx*1+0x40],zmm2
vmovaps ymm2,YMMWORD PTR [rip+0xfffffffffff28e94]        # 9e60 <GCC_except_table830+0x1904>
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x140],zmm5
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x180],zmm10
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x1c0],zmm9
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x200],zmm6
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x240],zmm0
vpaddq zmm5,zmm7,zmm24
vpxor  xmm7,xmm7,xmm7
vpmadd52huq zmm7,zmm16,zmm10
mov    QWORD PTR [rdi+0x10],r15
mov    r15,QWORD PTR [rsp+0xf40]
vpsrlq zmm5,zmm5,0x34
vpmadd52huq zmm5,zmm14,zmm25
vmovups YMMWORD PTR [rsp+0x5c0],ymm2
mov    QWORD PTR [rsp+0x5e0],r12
vpbroadcastq zmm3,QWORD PTR [rsp+0x5c8]
vpbroadcastq zmm2,QWORD PTR [rsp+0x5d0]
vpbroadcastq zmm1,QWORD PTR [rsp+0x5d8]
vpbroadcastq zmm15,QWORD PTR [rsp+0x5e0]
vpmadd52huq zmm12,zmm14,zmm3
vpmadd52huq zmm11,zmm14,zmm2
vpmadd52huq zmm8,zmm14,zmm1
vpmadd52luq zmm13,zmm14,zmm3
vpmadd52huq zmm4,zmm14,zmm15
vpmadd52luq zmm12,zmm14,zmm2
vpmadd52luq zmm11,zmm14,zmm1
vpmadd52luq zmm8,zmm14,zmm15
vpaddq zmm13,zmm13,zmm5
vpmadd52huq zmm4,zmm17,zmm10
vpmadd52luq zmm13,zmm20,zmm10
vpmadd52huq zmm12,zmm20,zmm10
vpmadd52huq zmm11,zmm19,zmm10
vpmadd52huq zmm8,zmm18,zmm10
vpmadd52luq zmm4,zmm16,zmm10
vpmadd52luq zmm12,zmm19,zmm10
vpmadd52luq zmm11,zmm18,zmm10
vpmadd52luq zmm8,zmm17,zmm10
vpxor  xmm10,xmm10,xmm10
vpmadd52luq zmm10,zmm13,zmm30
vpaddq zmm13,zmm13,zmm24
vpsrlq zmm13,zmm13,0x34
vpmadd52huq zmm11,zmm10,zmm3
vpmadd52huq zmm8,zmm10,zmm2
vpmadd52huq zmm4,zmm10,zmm1
vpmadd52huq zmm13,zmm10,zmm25
vpmadd52luq zmm12,zmm10,zmm3
vpmadd52huq zmm7,zmm10,zmm15
vpmadd52luq zmm11,zmm10,zmm2
vpmadd52luq zmm8,zmm10,zmm1
vpmadd52luq zmm4,zmm10,zmm15
vpaddq zmm12,zmm12,zmm13
vpmadd52huq zmm7,zmm17,zmm9
vpxor  xmm10,xmm10,xmm10
vpmadd52huq zmm10,zmm16,zmm9
vpmadd52luq zmm12,zmm20,zmm9
vpmadd52huq zmm11,zmm20,zmm9
vpmadd52huq zmm8,zmm19,zmm9
vpmadd52huq zmm4,zmm18,zmm9
vpmadd52luq zmm7,zmm16,zmm9
vpmadd52luq zmm11,zmm19,zmm9
vpmadd52luq zmm8,zmm18,zmm9
vpmadd52luq zmm4,zmm17,zmm9
vpxor  xmm9,xmm9,xmm9
vpmadd52luq zmm9,zmm12,zmm30
vpaddq zmm12,zmm12,zmm24
vpsrlq zmm12,zmm12,0x34
vpmadd52huq zmm8,zmm9,zmm3
vpmadd52huq zmm4,zmm9,zmm2
vpmadd52huq zmm7,zmm9,zmm1
vpmadd52huq zmm12,zmm9,zmm25
vpmadd52luq zmm11,zmm9,zmm3
vpmadd52huq zmm10,zmm9,zmm15
vpmadd52luq zmm8,zmm9,zmm2
vpmadd52luq zmm4,zmm9,zmm1
vpmadd52luq zmm7,zmm9,zmm15
vpaddq zmm11,zmm11,zmm12
vpmadd52huq zmm10,zmm17,zmm6
vpxor  xmm9,xmm9,xmm9
vpmadd52huq zmm9,zmm16,zmm6
vpmadd52luq zmm11,zmm20,zmm6
vpmadd52huq zmm8,zmm20,zmm6
vpmadd52huq zmm4,zmm19,zmm6
vpmadd52huq zmm7,zmm18,zmm6
vpmadd52luq zmm10,zmm16,zmm6
vpmadd52luq zmm8,zmm19,zmm6
vpmadd52luq zmm4,zmm18,zmm6
vpmadd52luq zmm7,zmm17,zmm6
vpxor  xmm6,xmm6,xmm6
vpmadd52luq zmm6,zmm11,zmm30
vpaddq zmm11,zmm11,zmm24
vpsrlq zmm11,zmm11,0x34
vpmadd52huq zmm4,zmm6,zmm3
vpmadd52huq zmm7,zmm6,zmm2
vpmadd52huq zmm10,zmm6,zmm1
vpmadd52huq zmm11,zmm6,zmm25
vpmadd52luq zmm8,zmm6,zmm3
vpmadd52huq zmm9,zmm6,zmm15
vpmadd52luq zmm4,zmm6,zmm2
vpmadd52luq zmm7,zmm6,zmm1
vpmadd52luq zmm10,zmm6,zmm15
vpaddq zmm8,zmm8,zmm11
vpmadd52huq zmm9,zmm17,zmm0
vpxor  xmm6,xmm6,xmm6
vpmadd52huq zmm6,zmm16,zmm0
vpmadd52luq zmm8,zmm20,zmm0
vpmadd52huq zmm4,zmm20,zmm0
vpmadd52huq zmm7,zmm19,zmm0
vpmadd52huq zmm10,zmm18,zmm0
vpmadd52luq zmm9,zmm16,zmm0
vpmadd52luq zmm4,zmm19,zmm0
vpmadd52luq zmm7,zmm18,zmm0
vpmadd52luq zmm10,zmm17,zmm0
vpxor  xmm0,xmm0,xmm0
vpmadd52luq zmm0,zmm8,zmm30
vpmadd52huq zmm7,zmm0,zmm3
vpmadd52huq zmm10,zmm0,zmm2
vpmadd52huq zmm9,zmm0,zmm1
vpmadd52luq zmm4,zmm0,zmm3
vpmadd52huq zmm6,zmm0,zmm15
vpmadd52luq zmm7,zmm0,zmm2
vpaddq zmm2,zmm8,zmm24
vpmadd52luq zmm10,zmm0,zmm1
vpmadd52luq zmm9,zmm0,zmm15
vpsrlq zmm1,zmm2,0x34
vpmadd52huq zmm1,zmm0,zmm25
vpaddq zmm0,zmm4,zmm1
vpsrlq zmm1,zmm0,0x34
vpandq zmm20,zmm0,zmm24
vpaddq zmm1,zmm1,zmm7
vmovdqa64 ZMMWORD PTR [rsp+0x14c0],zmm20
vpsrlq zmm2,zmm1,0x34
vpandq zmm19,zmm1,zmm24
vpaddq zmm2,zmm2,zmm10
vmovdqa64 ZMMWORD PTR [rsp+0x1500],zmm19
vpsrlq zmm3,zmm2,0x34
vpandq zmm18,zmm2,zmm24
vpaddq zmm3,zmm3,zmm9
vmovdqa64 ZMMWORD PTR [rsp+0x1540],zmm18
vpsrlq zmm4,zmm3,0x34
vpandq zmm17,zmm3,zmm24
vpaddq zmm16,zmm4,zmm6
vmovdqa64 ZMMWORD PTR [rsp+0x1580],zmm17
vmovdqa64 ZMMWORD PTR [rsp+0x15c0],zmm16
cmp    r15,r14
je     e175e <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x11be>
mov    rax,QWORD PTR [rsp+0x18]
mov    r8d,0x0
sub    rax,r14
cmovae r8,rax
jbe    ec863 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc2c3>
mov    edi,DWORD PTR [rbx-0x78]
and    edi,0x7fffffff
cmp    r13,rdi
jbe    ec8fb <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc35b>
cmp    r8,0x1
je     ec7d9 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc239>
mov    eax,DWORD PTR [rbx-0x68]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec8f8 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc358>
cmp    r8,0x2
je     ec7f3 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc253>
mov    esi,DWORD PTR [rbx-0x58]
and    esi,0x7fffffff
cmp    r13,rsi
jbe    ec8e3 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc343>
cmp    r8,0x3
je     ec80e <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc26e>
mov    ecx,DWORD PTR [rbx-0x48]
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    ec8ce <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc32e>
cmp    r8,0x4
je     ec7be <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc21e>
mov    r9d,DWORD PTR [rbx-0x38]
and    r9d,0x7fffffff
cmp    r13,r9
jbe    ec8b9 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc319>
cmp    r8,0x5
je     ec844 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc2a4>
mov    edx,DWORD PTR [rbx-0x28]
and    edx,0x7fffffff
cmp    r13,rdx
jbe    ec8a4 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc304>
cmp    r8,0x6
je     ec85f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc2bf>
mov    r10d,DWORD PTR [rbx-0x18]
and    r10d,0x7fffffff
cmp    r13,r10
jbe    ec88f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc2ef>
cmp    r8,0x7
je     ec829 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc289>
mov    r8d,DWORD PTR [rbx-0x8]
and    r8d,0x7fffffff
cmp    r13,r8
jbe    ec87a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc2da>
mov    r11,QWORD PTR [rsp+0x38]
shl    rax,0x7
shl    rsi,0x7
shl    r9,0x7
shl    rcx,0x7
shl    rdi,0x7
shl    r10,0x7
shl    rdx,0x7
shl    r8,0x7
vmovups ymm14,YMMWORD PTR [r11+rax*1]
vmovups ymm1,YMMWORD PTR [r11+rsi*1]
vmovups ymm2,YMMWORD PTR [r11+r9*1]
vmovdqu ymm13,YMMWORD PTR [r11+rdi*1]
add    rdi,r11
lea    rsi,[r11+rsi*1]
lea    rax,[r11+rax*1]
vmovups ymm3,YMMWORD PTR [r11+r10*1]
vmovups ymm26,YMMWORD PTR [r11+rdx*1]
vmovups ymm0,YMMWORD PTR [rdi+0x20]
mov    rdi,QWORD PTR [rsp+0x28]
vinsertf64x4 zmm0,zmm0,YMMWORD PTR [rax+0x20],0x1
lea    rax,[r11+rcx*1]
mov    r15,QWORD PTR [rdi+0x10]
vmovaps YMMWORD PTR [rsp+0x980],ymm14
vmovups ymm14,YMMWORD PTR [r11+rcx*1]
vmovaps ZMMWORD PTR [rsp+0x200],zmm1
vmovups ymm1,YMMWORD PTR [rsi+0x20]
lea    rsi,[r11+r9*1]
vmovaps ZMMWORD PTR [rsp+0x280],zmm2
vmovaps YMMWORD PTR [rsp+0x340],ymm26
vinsertf64x4 zmm15,zmm1,YMMWORD PTR [rax+0x20],0x1
vmovups ymm2,YMMWORD PTR [rsi+0x20]
lea    rax,[r11+rdx*1]
lea    rsi,[r11+r10*1]
vmovaps ymm1,YMMWORD PTR [rip+0xfffffffffff2895e]        # 9e60 <GCC_except_table830+0x1904>
vmovups ymm4,YMMWORD PTR [rsi+0x20]
mov    rsi,QWORD PTR [rbx-0x70]
vmovaps ZMMWORD PTR [rsp+0x2c0],zmm0
vmovups ymm5,YMMWORD PTR [rsi]
vmovdqu ymm6,YMMWORD PTR [rsi+0x20]
mov    rsi,QWORD PTR [rbx-0x50]
vmovaps YMMWORD PTR [rsp+0x380],ymm14
vinsertf64x4 zmm14,zmm2,YMMWORD PTR [rax+0x20],0x1
vinsertf64x4 zmm2,zmm3,YMMWORD PTR [r11+r8*1],0x1
mov    rax,QWORD PTR [rbx-0x60]
vinsertf64x4 zmm3,zmm4,YMMWORD PTR [r11+r8*1+0x20],0x1
vmovaps ZMMWORD PTR [rsp+0x300],zmm15
vmovups ymm7,YMMWORD PTR [rsi]
vmovdqu ymm8,YMMWORD PTR [rsi+0x20]
mov    rsi,QWORD PTR [rbx-0x30]
vmovaps ZMMWORD PTR [rsp+0x440],zmm14
vmovaps ZMMWORD PTR [rsp+0x900],zmm2
vinsertf64x4 zmm2,zmm5,YMMWORD PTR [rax],0x1
vinserti64x4 zmm14,zmm6,YMMWORD PTR [rax+0x20],0x1
mov    rax,QWORD PTR [rbx-0x40]
vmovaps ZMMWORD PTR [rsp+0x9c0],zmm3
vmovups ymm9,YMMWORD PTR [rsi]
vmovdqu ymm10,YMMWORD PTR [rsi+0x20]
mov    rsi,QWORD PTR [rbx-0x10]
vinsertf64x4 zmm3,zmm7,YMMWORD PTR [rax],0x1
vinserti64x4 zmm15,zmm8,YMMWORD PTR [rax+0x20],0x1
mov    rax,QWORD PTR [rbx-0x20]
vmovups ymm11,YMMWORD PTR [rsi]
vmovups ymm12,YMMWORD PTR [rsi+0x20]
vmovaps ZMMWORD PTR [rsp+0x240],zmm2
vinsertf64x4 zmm2,zmm9,YMMWORD PTR [rax],0x1
vinserti64x4 zmm21,zmm10,YMMWORD PTR [rax+0x20],0x1
mov    rax,QWORD PTR [rbx]
vmovaps ZMMWORD PTR [rsp+0x4c0],zmm3
vinsertf64x4 zmm3,zmm11,YMMWORD PTR [rax],0x1
vmovaps ZMMWORD PTR [rsp+0x480],zmm2
vinsertf64x4 zmm2,zmm12,YMMWORD PTR [rax+0x20],0x1
vmovups YMMWORD PTR [rsp+0x5c0],ymm1
mov    QWORD PTR [rsp+0x5e0],r12
vbroadcastsd zmm0,QWORD PTR [rsp+0x5d8]
vpbroadcastq zmm27,QWORD PTR [rsp+0x5c8]
vpbroadcastq zmm26,QWORD PTR [rsp+0x5d0]
vpbroadcastq zmm31,QWORD PTR [rsp+0x5e0]
vmovaps ZMMWORD PTR [rsp+0x940],zmm3
vmovaps ZMMWORD PTR [rsp+0x180],zmm2
vmovaps ZMMWORD PTR [rsp+0x1c0],zmm0
cmp    r15,QWORD PTR [rdi]
jne    e0710 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x170>
vmovdqa64 ZMMWORD PTR [rsp+0x880],zmm16
vmovdqa64 ZMMWORD PTR [rsp+0xa80],zmm17
vmovdqa64 ZMMWORD PTR [rsp+0xa40],zmm18
vmovdqa64 ZMMWORD PTR [rsp+0xa00],zmm19
vmovdqa64 ZMMWORD PTR [rsp+0xac0],zmm20
vmovdqa64 ZMMWORD PTR [rsp+0x840],zmm31
vmovdqa64 ZMMWORD PTR [rsp+0x400],zmm26
vmovdqa64 ZMMWORD PTR [rsp+0x3c0],zmm27
vmovdqa64 ZMMWORD PTR [rsp+0x580],zmm13
vmovdqa64 ZMMWORD PTR [rsp+0xb00],zmm14
vmovdqa64 ZMMWORD PTR [rsp+0xbc0],zmm15
vmovdqa64 ZMMWORD PTR [rsp+0xf00],zmm21
vzeroupper
call   bf2c0 <_RNvMs4_NtCs6i54tJFfzR_5alloc7raw_vecINtB5_6RawVecAINtNtNtCs9RL81RXvecK_11zopro_bn2545ifma818persistent_quarter4WideKy2_Ej2_E8grow_oneCsh5u81WWlmio_10zopro_core>
vmovdqa64 zmm21,ZMMWORD PTR [rsp+0xf00]
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0xbc0]
vmovdqa64 zmm14,ZMMWORD PTR [rsp+0xb00]
vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x580]
vmovdqa64 zmm27,ZMMWORD PTR [rsp+0x3c0]
vmovdqa64 zmm26,ZMMWORD PTR [rsp+0x400]
vmovdqa64 zmm31,ZMMWORD PTR [rsp+0x840]
vpbroadcastq zmm30,QWORD PTR [rip+0xfffffffffff28258]        # 9948 <GCC_except_table830+0x13ec>
vpbroadcastq zmm29,QWORD PTR [rip+0xfffffffffff2819e]        # 9898 <GCC_except_table830+0x133c>
vpbroadcastq zmm28,QWORD PTR [rip+0xfffffffffff28124]        # 9828 <GCC_except_table830+0x12cc>
vpbroadcastq zmm25,QWORD PTR [rip+0xfffffffffff28092]        # 97a0 <GCC_except_table830+0x1244>
vpbroadcastq zmm24,QWORD PTR [rip+0xfffffffffff28030]        # 9748 <GCC_except_table830+0x11ec>
vmovdqa64 zmm23,ZMMWORD PTR [rip+0xfffffffffff27c9e]        # 93c0 <GCC_except_table830+0xe64>
vmovdqa64 zmm22,ZMMWORD PTR [rip+0xfffffffffff27c54]        # 9380 <GCC_except_table830+0xe24>
vmovdqa64 zmm20,ZMMWORD PTR [rsp+0xac0]
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0xa00]
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0xa40]
vmovdqa64 zmm17,ZMMWORD PTR [rsp+0xa80]
vmovdqa64 zmm16,ZMMWORD PTR [rsp+0x880]
mov    rdi,QWORD PTR [rsp+0x28]
jmp    e0710 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x170>
mov    rcx,QWORD PTR [rsp+0x30]
mov    rbx,QWORD PTR [rsp+0xf80]
xor    r14d,r14d
mov    QWORD PTR [rdi+0x28],0x0
mov    QWORD PTR [rsp+0x20],r13
cmp    r15,QWORD PTR [rsp+0x18]
jne    e17d0 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1230>
lea    rdi,[rsp+0x40]
lea    rsi,[rsp+0x14c0]
vzeroupper
call   d7b50 <_RNvMs0_NtNtCs9RL81RXvecK_11zopro_bn2545ifma818persistent_quarterINtB5_4WideKy2_E15inverse_nonzero>
mov    r9,QWORD PTR [rsp+0x28]
mov    r10,QWORD PTR [rsp+0x38]
mov    r8,QWORD PTR [rsp+0x30]
test   r14b,r14b
je     e7b65 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x75c5>
jmp    ec6e4 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc144>
mov    rcx,QWORD PTR [rsp+0x30]
mov    r14b,0x1
mov    QWORD PTR [rdi+0x28],0x0
mov    QWORD PTR [rsp+0x20],r13
cmp    r15,QWORD PTR [rsp+0x18]
je     e1782 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x11e2>
vmovaps ymm8,YMMWORD PTR [rip+0xfffffffffff28688]        # 9e60 <GCC_except_table830+0x1904>
vpsllq zmm0,zmm20,0x3
vpbroadcastq zmm6,QWORD PTR [rip+0xfffffffffff28147]        # 9930 <GCC_except_table830+0x13d4>
vpbroadcastq zmm7,QWORD PTR [rip+0xfffffffffff27f5d]        # 9750 <GCC_except_table830+0x11f4>
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff27f4b]        # 9748 <GCC_except_table830+0x11ec>
shl    rbx,0x7
mov    DWORD PTR [rsp+0x1340],r14d
mov    QWORD PTR [rsp+0xa80],r15
vpaddq zmm0,zmm0,zmm20
vpandq zmm0,zmm0,QWORD BCST [rip+0xfffffffffff27f47]        # 9768 <GCC_except_table830+0x120c>
lea    rax,[rbx+rcx*1+0x8]
vpmadd52luq zmm20,zmm0,QWORD BCST [rip+0xfffffffffff27f70]        # 97a0 <GCC_except_table830+0x1244>
vmovups YMMWORD PTR [rsp+0x5c0],ymm8
mov    QWORD PTR [rsp+0x5e0],r12
vpmullq zmm1,zmm0,QWORD BCST [rsp+0x5c8]
vpmullq zmm2,zmm0,QWORD BCST [rsp+0x5d0]
vpmullq zmm3,zmm0,QWORD BCST [rsp+0x5d8]
vpmullq zmm4,zmm0,QWORD BCST [rsp+0x5e0]
vmovups YMMWORD PTR [rsp+0x5c0],ymm8
mov    QWORD PTR [rsp+0x5e0],r12
vpsrlq zmm0,zmm20,0x34
vpsrlq zmm10,zmm20,0x4
vpaddq zmm1,zmm19,zmm1
vpaddq zmm2,zmm18,zmm2
vpaddq zmm3,zmm17,zmm3
vpaddq zmm4,zmm16,zmm4
vpaddq zmm0,zmm1,zmm0
vpsrlq zmm1,zmm0,0x34
vpsllq zmm5,zmm0,0x30
vpsrlq zmm0,zmm0,0x4
vpaddq zmm1,zmm2,zmm1
vpandq zmm5,zmm5,zmm6
vpsrlq zmm2,zmm1,0x34
vpsllq zmm9,zmm1,0x30
vpternlogq zmm5,zmm10,zmm7,0xf8
vpaddq zmm2,zmm3,zmm2
vpsrlq zmm3,zmm2,0x34
vpsrlq zmm10,zmm2,0x4
vpaddq zmm3,zmm4,zmm3
vpandq zmm4,zmm9,zmm6
vpsrlq zmm9,zmm1,0x4
vpsllq zmm1,zmm2,0x30
vpandq zmm1,zmm1,zmm6
vpternlogq zmm4,zmm0,zmm7,0xf8
vpsubq zmm12,zmm4,QWORD BCST [rsp+0x5c8]
vpsrlq zmm0,zmm3,0x4
vpternlogq zmm1,zmm9,zmm7,0xf8
vpsllq zmm9,zmm3,0x30
vpandq zmm3,zmm0,zmm7
vpsubq zmm3,zmm3,QWORD BCST [rsp+0x5e0]
vpandq zmm2,zmm9,zmm6
vpaddq zmm9,zmm5,QWORD BCST [rip+0xfffffffffff27f64]        # 98c0 <GCC_except_table830+0x1364>
vpternlogq zmm2,zmm10,zmm7,0xf8
vpsubq zmm7,zmm1,QWORD BCST [rsp+0x5d0]
vpsubq zmm8,zmm2,QWORD BCST [rsp+0x5d8]
vpsraq zmm10,zmm9,0x34
vpaddq zmm6,zmm12,zmm10
vpsraq zmm10,zmm6,0x34
vpaddq zmm7,zmm7,zmm10
vpsraq zmm10,zmm7,0x34
vpaddq zmm8,zmm8,zmm10
vpsraq zmm10,zmm8,0x34
vpaddq zmm3,zmm3,zmm10
vpcmpltuq k1,zmm3,QWORD BCST [rip+0xfffffffffff27e30]        # 97e8 <GCC_except_table830+0x128c>
vpandq zmm1{k1},zmm7,zmm11
vpandq zmm4{k1},zmm6,zmm11
vpandq zmm2{k1},zmm8,zmm11
vmovdqa64 zmm0{k1},zmm3
vpandq zmm5{k1},zmm9,zmm11
vpsllq zmm0,zmm0,0x10
vpsllq zmm3,zmm4,0x34
vpsllq zmm7,zmm1,0x28
vpsrlq zmm4,zmm4,0xc
vpsllq zmm6,zmm2,0x1c
vpsrlq zmm1,zmm1,0x18
vpsrlq zmm2,zmm2,0x24
vporq  zmm1,zmm6,zmm1
vporq  zmm3,zmm3,zmm5
vporq  zmm4,zmm7,zmm4
vporq  zmm0,zmm0,zmm2
vmovdqa64 zmm6,ZMMWORD PTR [rip+0xfffffffffff27a17]        # 9440 <GCC_except_table830+0xee4>
vpunpcklqdq zmm2,zmm3,zmm4
vpunpckhqdq zmm3,zmm3,zmm4
vpunpcklqdq zmm4,zmm1,zmm0
vpunpckhqdq zmm0,zmm1,zmm0
vmovdqa64 zmm1,ZMMWORD PTR [rip+0xfffffffffff279b5]        # 9400 <GCC_except_table830+0xea4>
vmovdqa64 zmm5,zmm2
vpermt2q zmm2,zmm6,zmm4
vpermt2q zmm5,zmm1,zmm4
vpermi2q zmm1,zmm3,zmm0
vpermt2q zmm3,zmm6,zmm0
vinserti64x4 zmm0,zmm5,ymm1,0x1
vinserti64x4 zmm4,zmm2,ymm3,0x1
vshufi64x2 zmm1,zmm5,zmm1,0xee
vshufi64x2 zmm2,zmm2,zmm3,0xee
vmovdqu64 ZMMWORD PTR [rsp+0x1600],zmm0
vmovdqu64 ZMMWORD PTR [rsp+0x1640],zmm1
vmovdqu64 ZMMWORD PTR [rsp+0x1680],zmm4
vmovdqu64 ZMMWORD PTR [rsp+0x16c0],zmm2
jmp    e1e09 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1869>
nop    WORD PTR [rax+rax*1+0x0]
movabs rcx,0xc3df73e9278302b9
add    r8,rcx
mov    QWORD PTR [rsp+rax*1+0x1600],r8
mov    r8,QWORD PTR [rsp+0x1000]
mov    QWORD PTR [rsp+rax*1+0x1608],rdx
mov    QWORD PTR [rsp+rax*1+0x1610],r13
mov    QWORD PTR [rsp+rax*1+0x1618],r14
mov    r9,QWORD PTR [rsp+0x1180]
mov    rax,QWORD PTR [rsp+0x1380]
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
mov    r15,QWORD PTR [rsp+0xa40]
mov    r14,r11
mov    rbx,rsi
mov    r12,r10
mov    QWORD PTR [rsp+0x40],r8
mov    r8,QWORD PTR [rsp+0x13c0]
mov    QWORD PTR [rsp+0x48],r9
lea    r9,[rsp+0x40]
mov    QWORD PTR [rsp+0x50],r8
mov    QWORD PTR [rsp+0x58],rax
lea    r8,[rsp+0x5c0]
add    r12,QWORD PTR [r9]
adc    r14,QWORD PTR [r9+0x8]
adc    r15,QWORD PTR [r9+0x10]
adc    rbx,QWORD PTR [r9+0x18]
mov    rax,r12
mov    rcx,r14
mov    rdx,r15
mov    rsi,rbx
sub    rax,QWORD PTR [r8]
sbb    rcx,QWORD PTR [r8+0x8]
sbb    rdx,QWORD PTR [r8+0x10]
sbb    rsi,QWORD PTR [r8+0x18]
cmovae r12,rax
cmovae r14,rcx
cmovae r15,rdx
cmovae rbx,rsi
mov    QWORD PTR [rsp+0x840],r14
mov    QWORD PTR [rsp+0x880],rbx
mov    QWORD PTR [rsp+0x3c0],r12
mov    QWORD PTR [rsp+0x400],r15
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
mov    rax,QWORD PTR [rsp+0xac0]
mov    r14,QWORD PTR [rsp+0x10c0]
mov    rbx,QWORD PTR [rsp+0x1080]
mov    r12,QWORD PTR [rsp+0x440]
mov    r15,QWORD PTR [rsp+0x9c0]
mov    QWORD PTR [rsp+0x40],rax
mov    rax,QWORD PTR [rsp+0x1040]
mov    QWORD PTR [rsp+0x48],r14
mov    QWORD PTR [rsp+0x50],rbx
mov    r14,QWORD PTR [rsp+0x940]
mov    rbx,QWORD PTR [rsp+0x980]
mov    QWORD PTR [rsp+0x58],rax
add    r12,QWORD PTR [r9]
adc    r14,QWORD PTR [r9+0x8]
adc    r15,QWORD PTR [r9+0x10]
adc    rbx,QWORD PTR [r9+0x18]
mov    rax,r12
mov    rcx,r14
mov    rdx,r15
mov    rsi,rbx
sub    rax,QWORD PTR [r8]
sbb    rcx,QWORD PTR [r8+0x8]
sbb    rdx,QWORD PTR [r8+0x10]
sbb    rsi,QWORD PTR [r8+0x18]
cmovae r12,rax
cmovae r14,rcx
cmovae r15,rdx
cmovae rbx,rsi
mov    r13,QWORD PTR [rdi+0x28]
cmp    r13,QWORD PTR [rdi+0x18]
je     e4894 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x42f4>
mov    rcx,QWORD PTR [rdi+0x20]
mov    rdx,QWORD PTR [rsp+0xa40]
mov    r8,QWORD PTR [rsp+0xa00]
lea    rax,[r13+r13*4+0x0]
mov    rsi,QWORD PTR [rsp+0x440]
inc    r13
shl    rax,0x6
mov    QWORD PTR [rcx+rax*1],r10
mov    QWORD PTR [rcx+rax*1+0x8],r11
mov    QWORD PTR [rcx+rax*1+0x10],rdx
mov    QWORD PTR [rcx+rax*1+0x18],r8
mov    r8,QWORD PTR [rsp+0x940]
mov    QWORD PTR [rcx+rax*1+0x20],rsi
mov    rsi,QWORD PTR [rsp+0x9c0]
mov    rdx,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rcx+rax*1+0x28],r8
mov    r8,QWORD PTR [rsp+0x980]
mov    QWORD PTR [rcx+rax*1+0x30],rsi
mov    rsi,QWORD PTR [rsp+0x840]
mov    QWORD PTR [rcx+rax*1+0x38],r8
mov    r8,QWORD PTR [rsp+0x400]
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x1400]
vmovdqu64 ZMMWORD PTR [rcx+rax*1+0x40],zmm0
mov    QWORD PTR [rcx+rax*1+0x80],rdx
mov    QWORD PTR [rcx+rax*1+0x88],rsi
mov    rsi,QWORD PTR [rsp+0x880]
mov    QWORD PTR [rcx+rax*1+0x90],r8
mov    r8,QWORD PTR [rsp+0x1c0]
vpxor  xmm0,xmm0,xmm0
mov    QWORD PTR [rcx+rax*1+0x98],rsi
mov    rsi,QWORD PTR [rsp+0x180]
mov    QWORD PTR [rcx+rax*1+0xa0],r12
mov    QWORD PTR [rcx+rax*1+0xa8],r14
mov    QWORD PTR [rcx+rax*1+0xb0],r15
mov    QWORD PTR [rcx+rax*1+0xb8],rbx
mov    QWORD PTR [rcx+rax*1+0xc0],r8
mov    r8,QWORD PTR [rsp+0x200]
mov    QWORD PTR [rcx+rax*1+0xc8],rsi
mov    rsi,QWORD PTR [rsp+0x280]
mov    QWORD PTR [rcx+rax*1+0xd0],r8
mov    r8,QWORD PTR [rsp+0x240]
mov    QWORD PTR [rcx+rax*1+0xd8],rsi
mov    rsi,QWORD PTR [rsp+0x2c0]
vmovdqu YMMWORD PTR [rcx+rax*1+0xe0],ymm0
mov    QWORD PTR [rcx+rax*1+0x100],r8
mov    r8,QWORD PTR [rsp+0x300]
mov    QWORD PTR [rcx+rax*1+0x108],rsi
mov    rsi,QWORD PTR [rsp+0x4c0]
mov    QWORD PTR [rcx+rax*1+0x110],r8
mov    r8,QWORD PTR [rsp+0x480]
mov    QWORD PTR [rcx+rax*1+0x118],rsi
mov    rsi,QWORD PTR [rsp+0x380]
mov    QWORD PTR [rcx+rax*1+0x120],r8
mov    r8,QWORD PTR [rsp+0x340]
mov    QWORD PTR [rcx+rax*1+0x128],rsi
mov    rsi,QWORD PTR [rsp+0x900]
mov    QWORD PTR [rcx+rax*1+0x130],r8
mov    QWORD PTR [rcx+rax*1+0x138],rsi
mov    rcx,QWORD PTR [rsp+0xa80]
mov    rax,QWORD PTR [rsp+0xf00]
mov    QWORD PTR [rdi+0x28],r13
inc    rcx
add    rax,0x10
mov    QWORD PTR [rsp+0xa80],rcx
cmp    rcx,QWORD PTR [rsp+0x18]
jae    e48ba <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x431a>
mov    rdx,QWORD PTR [rax]
mov    ecx,DWORD PTR [rax-0x8]
mov    QWORD PTR [rsp+0xf00],rax
vmovups zmm0,ZMMWORD PTR [rdx]
mov    QWORD PTR [rsp+0x280],rcx
vmovaps ZMMWORD PTR [rsp+0x5c0],zmm0
test   ecx,ecx
js     e1e70 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x18d0>
mov    rsi,QWORD PTR [rdx+0x68]
mov    rdi,QWORD PTR [rdx+0x70]
mov    r12,QWORD PTR [rdx+0x40]
mov    rax,QWORD PTR [rdx+0x48]
mov    r14,QWORD PTR [rdx+0x50]
mov    r11,QWORD PTR [rdx+0x58]
mov    rcx,QWORD PTR [rdx+0x60]
mov    rdx,QWORD PTR [rdx+0x78]
mov    QWORD PTR [rsp+0x180],rsi
mov    QWORD PTR [rsp+0x200],rdi
mov    QWORD PTR [rsp+0x1c0],rdx
jmp    e1f6d <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x19cd>
xchg   ax,ax
lea    rax,[rdx+0x40]
vmovdqu ymm2,YMMWORD PTR [rip+0xfffffffffff280e4]        # 9f60 <GCC_except_table830+0x1a04>
xor    r12d,r12d
xor    r14d,r14d
xor    r11d,r11d
lea    rsi,[rsp+0xc00]
lea    r9,[rsp+0x40]
lea    r13,[rsp+0xc00]
vmovups ymm0,YMMWORD PTR [rax]
xor    eax,eax
vmovdqa YMMWORD PTR [rsp+0xc00],ymm2
vmovaps YMMWORD PTR [rsp+0x40],ymm0
sub    r12,QWORD PTR [r9]
sbb    rax,QWORD PTR [r9+0x8]
sbb    r14,QWORD PTR [r9+0x10]
sbb    r11,QWORD PTR [r9+0x18]
sbb    rbx,rbx
mov    rcx,QWORD PTR [rsi]
mov    rdi,QWORD PTR [rsi+0x8]
mov    r8,QWORD PTR [rsi+0x10]
mov    r10,QWORD PTR [rsi+0x18]
and    rcx,rbx
and    rdi,rbx
and    r8,rbx
and    r10,rbx
add    r12,rcx
adc    rax,rdi
adc    r14,r8
adc    r11,r10
mov    QWORD PTR [rsp+0x240],r11
xor    ecx,ecx
xor    esi,esi
xor    r9d,r9d
xor    r8d,r8d
lea    r11,[rsp+0x40]
vmovdqu ymm1,YMMWORD PTR [rdx+0x60]
vmovdqa YMMWORD PTR [rsp+0xc00],ymm2
vmovdqa YMMWORD PTR [rsp+0x40],ymm1
sub    rcx,QWORD PTR [r11]
sbb    rsi,QWORD PTR [r11+0x8]
sbb    r9,QWORD PTR [r11+0x10]
sbb    r8,QWORD PTR [r11+0x18]
sbb    r15,r15
mov    rdx,QWORD PTR [r13+0x0]
mov    rdi,QWORD PTR [r13+0x8]
mov    r10,QWORD PTR [r13+0x10]
mov    rbx,QWORD PTR [r13+0x18]
and    rdx,r15
and    rdi,r15
and    r10,r15
and    rbx,r15
add    rcx,rdx
adc    rsi,rdi
adc    r9,r10
adc    r8,rbx
mov    QWORD PTR [rsp+0x200],r9
mov    QWORD PTR [rsp+0x180],rsi
mov    QWORD PTR [rsp+0x1c0],r8
mov    r11,QWORD PTR [rsp+0x240]
mov    r15d,r12d
and    r15d,0x1
mov    rdi,r14
shld   rdi,rax,0x3e
shld   rax,r12,0x3e
movabs rdx,0x4f082305b61f3f52
shl    r12,0x3e
xor    r13d,r13d
movabs rbx,0x65e05aa45a1c72a3
vmovaps zmm0,ZMMWORD PTR [rsp+0x5c0]
neg    r15
sar    r12,0x3f
mov    r9,r15
and    r9,rdx
mov    rsi,r12
mov    r10,r15
mov    r8,r15
add    r9,rax
movabs rax,0x9e10460b6c3e7ea4
setb   r13b
and    rsi,rax
and    r10,rbx
movabs rax,0xcbc0b548b438e546
mov    rbx,r12
xor    edx,edx
and    rbx,rax
add    r10,rdi
movabs rax,0x6e14116da0605617
setb   dl
add    r10,rbx
mov    rbx,r12
adc    rdx,0x0
and    r8,rax
movabs rax,0xdc2822db40c0ac2e
shrd   r14,r11,0x2
xor    edi,edi
and    rbx,rax
movabs rax,0xc19139cb84c680a
vmovaps ZMMWORD PTR [rsp+0xc00],zmm0
add    rbx,r8
movabs r8,0x183227397098d014
setb   dil
add    rbx,r14
adc    rdi,0x0
and    r15,rax
and    r12,r8
shr    r11,0x2
mov    rax,QWORD PTR [rsp+0x180]
add    r12,r15
add    r12,r11
add    rsi,r9
mov    r11d,ecx
movabs r9,0xcbc0b548b438e546
mov    QWORD PTR [rsp+0x240],rsi
mov    rsi,QWORD PTR [rsp+0x200]
adc    r10,r13
movabs r13,0x65e05aa45a1c72a3
adc    rbx,rdx
movabs rdx,0x4f082305b61f3f52
adc    r12,rdi
and    r11d,0x1
neg    r11
mov    r15,r11
and    r15,rdx
xor    edx,edx
mov    r14,r11
mov    rdi,rsi
shld   rdi,rax,0x3e
shld   rax,rcx,0x3e
shl    rcx,0x3e
sar    rcx,0x3f
add    r15,rax
movabs rax,0x9e10460b6c3e7ea4
mov    r8,rcx
setb   dl
and    r14,r13
mov    r13,rcx
and    r13,r9
mov    r9,QWORD PTR [rsp+0x1c0]
and    r8,rax
mov    rax,r11
mov    QWORD PTR [rsp+0x180],rdx
xor    edx,edx
add    r14,rdi
movabs rdi,0x6e14116da0605617
setb   dl
add    r14,r13
mov    r13,rcx
adc    rdx,0x0
and    rax,rdi
movabs rdi,0xdc2822db40c0ac2e
and    r13,rdi
xor    edi,edi
shrd   rsi,r9,0x2
add    r13,rax
movabs rax,0xc19139cb84c680a
setb   dil
add    r13,rsi
movabs rsi,0x183227397098d014
adc    rdi,0x0
and    r11,rax
and    rcx,rsi
shr    r9,0x2
mov    rsi,QWORD PTR [rsp+0x280]
mov    rax,QWORD PTR [rsp+0x240]
add    rcx,r11
add    rcx,r9
add    r8,r15
adc    r14,QWORD PTR [rsp+0x180]
mov    QWORD PTR [rsp+0xc40],rax
mov    QWORD PTR [rsp+0xc48],r10
mov    QWORD PTR [rsp+0xc50],rbx
mov    QWORD PTR [rsp+0xc58],r12
mov    QWORD PTR [rsp+0xc60],r8
adc    r13,rdx
adc    rcx,rdi
and    esi,0x7fffffff
mov    QWORD PTR [rsp+0xc68],r14
mov    QWORD PTR [rsp+0xc70],r13
mov    QWORD PTR [rsp+0xc78],rcx
cmp    QWORD PTR [rsp+0x20],rsi
jbe    ec90d <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc36d>
shl    rsi,0x7
add    rsi,QWORD PTR [rsp+0x38]
lea    r13,[rsp+0x5c0]
mov    rdi,r13
vzeroupper
call   ece90 <_RNvMs_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB4_8G2Bucket6scaled>
mov    rcx,QWORD PTR [rsp+0x5f0]
mov    rdx,QWORD PTR [rsp+0x5e8]
mov    rax,QWORD PTR [rsp+0x5e0]
mov    rbx,QWORD PTR [rsp+0x5c0]
mov    r14,QWORD PTR [rsp+0x5c8]
mov    r15,QWORD PTR [rsp+0x5d0]
mov    r12,QWORD PTR [rsp+0x5d8]
lea    rsi,[rsp+0xc00]
mov    rdi,r13
mov    QWORD PTR [rsp+0x9c0],rcx
lea    rcx,[rsp+0x600]
mov    QWORD PTR [rsp+0x940],rdx
mov    rdx,QWORD PTR [rsp+0x5f8]
mov    QWORD PTR [rsp+0x440],rax
vmovups zmm0,ZMMWORD PTR [rcx]
mov    QWORD PTR [rsp+0x980],rdx
vmovaps ZMMWORD PTR [rsp+0x1400],zmm0
vzeroupper
call   ece90 <_RNvMs_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB4_8G2Bucket6scaled>
mov    rdi,QWORD PTR [rsp+0x5e0]
vmovdqu ymm1,YMMWORD PTR [rip+0xfffffffffff27ce8]        # 9f60 <GCC_except_table830+0x1a04>
mov    rax,QWORD PTR [rsp+0x5c0]
mov    rcx,QWORD PTR [rsp+0x5c8]
mov    r8,QWORD PTR [rsp+0x5d0]
mov    rsi,QWORD PTR [rsp+0x5d8]
mov    QWORD PTR [rsp+0x40],rbx
mov    QWORD PTR [rsp+0xb00],rbx
mov    QWORD PTR [rsp+0x48],r14
mov    QWORD PTR [rsp+0xbc0],r14
mov    QWORD PTR [rsp+0x50],r15
mov    QWORD PTR [rsp+0xa40],r15
mov    r11,QWORD PTR [rsp+0x5e8]
mov    r13,QWORD PTR [rsp+0x5f0]
mov    r9,QWORD PTR [rsp+0x5f8]
mov    QWORD PTR [rsp+0xa00],r12
mov    QWORD PTR [rsp+0x58],r12
lea    r14,[rsp+0x40]
lea    r10,[rsp+0x5c0]
mov    QWORD PTR [rsp+0xac0],rdi
mov    rdi,QWORD PTR [rsp+0x600]
mov    QWORD PTR [rsp+0x1180],rcx
mov    QWORD PTR [rsp+0x1380],rsi
mov    QWORD PTR [rsp+0x1000],rax
mov    QWORD PTR [rsp+0x13c0],r8
mov    r12,rcx
mov    r15,rsi
mov    rbx,rax
mov    QWORD PTR [rsp+0x2c0],rdi
mov    rdi,QWORD PTR [rsp+0x608]
mov    QWORD PTR [rsp+0x300],rdi
mov    rdi,QWORD PTR [rsp+0x610]
mov    QWORD PTR [rsp+0x4c0],rdi
mov    rdi,QWORD PTR [rsp+0x618]
mov    QWORD PTR [rsp+0x480],rdi
mov    rdi,QWORD PTR [rsp+0x620]
mov    QWORD PTR [rsp+0x340],rdi
mov    rdi,QWORD PTR [rsp+0x628]
mov    QWORD PTR [rsp+0x900],rdi
mov    rdi,QWORD PTR [rsp+0x630]
mov    QWORD PTR [rsp+0x880],rdi
mov    rdi,QWORD PTR [rsp+0x638]
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
mov    QWORD PTR [rsp+0x240],rdi
sub    rbx,QWORD PTR [r14]
sbb    r12,QWORD PTR [r14+0x8]
sbb    r8,QWORD PTR [r14+0x10]
sbb    r15,QWORD PTR [r14+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r10]
mov    rcx,QWORD PTR [r10+0x8]
mov    rdx,QWORD PTR [r10+0x10]
mov    rsi,QWORD PTR [r10+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    rbx,rax
adc    r12,rcx
adc    r8,rdx
adc    r15,rsi
mov    QWORD PTR [rsp+0x380],r15
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
mov    QWORD PTR [rsp+0x10c0],r11
mov    QWORD PTR [rsp+0x1040],r9
mov    QWORD PTR [rsp+0x1080],r13
lea    r14,[rsp+0x5c0]
mov    rax,QWORD PTR [rsp+0x440]
mov    rdi,QWORD PTR [rsp+0x940]
mov    r10,QWORD PTR [rsp+0x9c0]
mov    QWORD PTR [rsp+0x40],rax
mov    QWORD PTR [rsp+0x48],rdi
mov    rax,QWORD PTR [rsp+0xac0]
mov    rdi,QWORD PTR [rsp+0x980]
mov    QWORD PTR [rsp+0x50],r10
lea    r10,[rsp+0x40]
mov    QWORD PTR [rsp+0x58],rdi
mov    QWORD PTR [rsp+0x180],rax
mov    rdi,r9
mov    r15,QWORD PTR [rsp+0x180]
sub    r15,QWORD PTR [r10]
sbb    r11,QWORD PTR [r10+0x8]
sbb    r13,QWORD PTR [r10+0x10]
sbb    rdi,QWORD PTR [r10+0x18]
sbb    r9,r9
mov    rax,QWORD PTR [r14]
mov    rcx,QWORD PTR [r14+0x8]
mov    rdx,QWORD PTR [r14+0x10]
mov    rsi,QWORD PTR [r14+0x18]
and    rax,r9
and    rcx,r9
and    rdx,r9
and    rsi,r9
add    r15,rax
adc    r11,rcx
adc    r13,rdx
adc    rdi,rsi
mov    QWORD PTR [rsp+0x500],rbx
mov    QWORD PTR [rsp+0x508],r12
mov    QWORD PTR [rsp+0x510],r8
mov    QWORD PTR [rsp+0xec0],r13
mov    QWORD PTR [rsp+0x400],r8
mov    QWORD PTR [rsp+0x180],r15
mov    QWORD PTR [rsp+0x280],r11
mov    QWORD PTR [rsp+0x200],rdi
mov    QWORD PTR [rsp+0x3c0],rbx
mov    QWORD PTR [rsp+0x840],r12
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
lea    r8,[rsp+0x500]
movabs r13,0x87d20782e4866389
mov    rax,QWORD PTR [rsp+0x380]
mov    QWORD PTR [rsp+0x518],rax
mov    rcx,QWORD PTR [rsp+0x508]
mov    rax,QWORD PTR [rsp+0x500]
mov    rsi,QWORD PTR [rsp+0x510]
mov    QWORD PTR [rsp+0x48],rcx
mov    rcx,QWORD PTR [rsp+0x518]
mov    QWORD PTR [rsp+0x50],rsi
mov    QWORD PTR [rsp+0x40],rax
lea    rsi,[rsp+0x40]
mov    QWORD PTR [rsp+0x58],rcx
lea    rcx,[rsp+0x5c0]
xor    ebx,ebx
xor    eax,eax
xor    r11d,r11d
xor    r9d,r9d
xor    edi,edi
mov    rdx,QWORD PTR [r8]
xor    r12d,r12d
mulx   r15,r10,QWORD PTR [rsi]
adcx   rbx,r10
adox   rax,r15
mulx   r15,r10,QWORD PTR [rsi+0x8]
adcx   rax,r10
adox   r11,r15
mulx   r15,r10,QWORD PTR [rsi+0x10]
adcx   r11,r10
adox   r9,r15
mulx   r15,r10,QWORD PTR [rsi+0x18]
adcx   r9,r10
adox   rdi,r15
adcx   rdi,r12
mov    rdx,rbx
imul   rdx,r13
xor    r12d,r12d
mulx   r15,r10,QWORD PTR [rcx]
adcx   rbx,r10
adox   rax,r15
mulx   r15,r10,QWORD PTR [rcx+0x8]
adcx   rax,r10
adox   r11,r15
mulx   r15,r10,QWORD PTR [rcx+0x10]
adcx   r11,r10
adox   r9,r15
mulx   r15,r10,QWORD PTR [rcx+0x18]
adcx   r9,r10
adox   rdi,r15
adcx   rdi,r12
mov    rdx,QWORD PTR [r8+0x8]
xor    r12d,r12d
mulx   r15,r10,QWORD PTR [rsi]
adcx   rax,r10
adox   r11,r15
mulx   r15,r10,QWORD PTR [rsi+0x8]
adcx   r11,r10
adox   r9,r15
mulx   r15,r10,QWORD PTR [rsi+0x10]
adcx   r9,r10
adox   rdi,r15
mulx   r15,r10,QWORD PTR [rsi+0x18]
adcx   rdi,r10
adox   rbx,r15
adcx   rbx,r12
mov    rdx,rax
imul   rdx,r13
xor    r12d,r12d
mulx   r15,r10,QWORD PTR [rcx]
adcx   rax,r10
adox   r11,r15
mulx   r15,r10,QWORD PTR [rcx+0x8]
adcx   r11,r10
adox   r9,r15
mulx   r15,r10,QWORD PTR [rcx+0x10]
adcx   r9,r10
adox   rdi,r15
mulx   r15,r10,QWORD PTR [rcx+0x18]
adcx   rdi,r10
adox   rbx,r15
adcx   rbx,r12
mov    rdx,QWORD PTR [r8+0x10]
xor    r12d,r12d
mulx   r15,r10,QWORD PTR [rsi]
adcx   r11,r10
adox   r9,r15
mulx   r15,r10,QWORD PTR [rsi+0x8]
adcx   r9,r10
adox   rdi,r15
mulx   r15,r10,QWORD PTR [rsi+0x10]
adcx   rdi,r10
adox   rbx,r15
mulx   r15,r10,QWORD PTR [rsi+0x18]
adcx   rbx,r10
adox   rax,r15
adcx   rax,r12
mov    rdx,r11
imul   rdx,r13
xor    r12d,r12d
mulx   r15,r10,QWORD PTR [rcx]
adcx   r11,r10
adox   r9,r15
mulx   r15,r10,QWORD PTR [rcx+0x8]
adcx   r9,r10
adox   rdi,r15
mulx   r15,r10,QWORD PTR [rcx+0x10]
adcx   rdi,r10
adox   rbx,r15
mulx   r15,r10,QWORD PTR [rcx+0x18]
adcx   rbx,r10
adox   rax,r15
adcx   rax,r12
mov    rdx,QWORD PTR [r8+0x18]
xor    r12d,r12d
mulx   r15,r10,QWORD PTR [rsi]
adcx   r9,r10
adox   rdi,r15
mulx   r15,r10,QWORD PTR [rsi+0x8]
adcx   rdi,r10
adox   rbx,r15
mulx   r15,r10,QWORD PTR [rsi+0x10]
adcx   rbx,r10
adox   rax,r15
mulx   r15,r10,QWORD PTR [rsi+0x18]
adcx   rax,r10
adox   r11,r15
adcx   r11,r12
mov    rdx,r9
imul   rdx,r13
xor    r12d,r12d
mulx   r15,r10,QWORD PTR [rcx]
adcx   r9,r10
adox   rdi,r15
mulx   r15,r10,QWORD PTR [rcx+0x8]
adcx   rdi,r10
adox   rbx,r15
mulx   r15,r10,QWORD PTR [rcx+0x10]
adcx   rbx,r10
adox   rax,r15
mulx   r15,r10,QWORD PTR [rcx+0x18]
adcx   rax,r10
adox   r11,r15
adcx   r11,r12
mov    QWORD PTR [rsp+0x1c0],rdi
movabs rdi,0x97816a916871ca8d
mov    QWORD PTR [rsp+0x580],r11
movabs r8,0xcf9bb18d1ece5fd7
mov    QWORD PTR [rsp+0xe80],rbx
mov    QWORD PTR [rsp+0xfc0],rax
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
movabs r12,0x87d20782e4866389
mov    rcx,QWORD PTR [rsp+0x180]
mov    rdx,QWORD PTR [rsp+0x280]
mov    rsi,QWORD PTR [rsp+0xec0]
mov    QWORD PTR [rsp+0x8e0],rcx
mov    QWORD PTR [rsp+0x8e8],rdx
mov    rdx,QWORD PTR [rsp+0x200]
mov    QWORD PTR [rsp+0x8f0],rsi
mov    rcx,QWORD PTR [rsp+0x580]
lea    rsi,[rsp+0x5c0]
mov    QWORD PTR [rsp+0x8f8],rdx
movabs rdx,0x3c208c16d87cfd47
cmp    QWORD PTR [rsp+0x1c0],rdx
movabs rdx,0x687e956e978e3573
lea    rdx,[rbx+rdx*1]
sbb    rdx,0x0
mov    QWORD PTR [rsp+0x180],rdx
setb   dl
cmp    rbx,rdi
setb   dil
or     dil,dl
movzx  edx,dil
movabs rdi,0x47afba497e7ea7a3
add    rdi,rax
sub    rdi,rdx
mov    QWORD PTR [rsp+0x200],rdi
movabs rdi,0xb85045b68181585d
setb   dl
cmp    rax,rdi
lea    rax,[rsp+0x40]
setb   dil
or     dil,dl
movzx  edx,dil
lea    rdi,[rcx+r8*1]
lea    r8,[rsp+0x8e0]
sub    rdi,rdx
mov    rdx,QWORD PTR [rsp+0x8e0]
mov    QWORD PTR [rsp+0x280],rdi
setb   BYTE PTR [rsp+0xec0]
mov    QWORD PTR [rsp+0x40],rdx
mov    rdx,QWORD PTR [rsp+0x8e8]
mov    QWORD PTR [rsp+0x48],rdx
mov    rdx,QWORD PTR [rsp+0x8f0]
mov    QWORD PTR [rsp+0x50],rdx
mov    rdx,QWORD PTR [rsp+0x8f8]
mov    QWORD PTR [rsp+0x58],rdx
xor    r15d,r15d
xor    r14d,r14d
xor    r13d,r13d
xor    r9d,r9d
xor    edi,edi
mov    rdx,QWORD PTR [r8]
xor    ebx,ebx
mulx   r11,r10,QWORD PTR [rax]
adcx   r15,r10
adox   r14,r11
mulx   r11,r10,QWORD PTR [rax+0x8]
adcx   r14,r10
adox   r13,r11
mulx   r11,r10,QWORD PTR [rax+0x10]
adcx   r13,r10
adox   r9,r11
mulx   r11,r10,QWORD PTR [rax+0x18]
adcx   r9,r10
adox   rdi,r11
adcx   rdi,rbx
mov    rdx,r15
imul   rdx,r12
xor    ebx,ebx
mulx   r11,r10,QWORD PTR [rsi]
adcx   r15,r10
adox   r14,r11
mulx   r11,r10,QWORD PTR [rsi+0x8]
adcx   r14,r10
adox   r13,r11
mulx   r11,r10,QWORD PTR [rsi+0x10]
adcx   r13,r10
adox   r9,r11
mulx   r11,r10,QWORD PTR [rsi+0x18]
adcx   r9,r10
adox   rdi,r11
adcx   rdi,rbx
mov    rdx,QWORD PTR [r8+0x8]
xor    ebx,ebx
mulx   r11,r10,QWORD PTR [rax]
adcx   r14,r10
adox   r13,r11
mulx   r11,r10,QWORD PTR [rax+0x8]
adcx   r13,r10
adox   r9,r11
mulx   r11,r10,QWORD PTR [rax+0x10]
adcx   r9,r10
adox   rdi,r11
mulx   r11,r10,QWORD PTR [rax+0x18]
adcx   rdi,r10
adox   r15,r11
adcx   r15,rbx
mov    rdx,r14
imul   rdx,r12
xor    ebx,ebx
mulx   r11,r10,QWORD PTR [rsi]
adcx   r14,r10
adox   r13,r11
mulx   r11,r10,QWORD PTR [rsi+0x8]
adcx   r13,r10
adox   r9,r11
mulx   r11,r10,QWORD PTR [rsi+0x10]
adcx   r9,r10
adox   rdi,r11
mulx   r11,r10,QWORD PTR [rsi+0x18]
adcx   rdi,r10
adox   r15,r11
adcx   r15,rbx
mov    rdx,QWORD PTR [r8+0x10]
xor    ebx,ebx
mulx   r11,r10,QWORD PTR [rax]
adcx   r13,r10
adox   r9,r11
mulx   r11,r10,QWORD PTR [rax+0x8]
adcx   r9,r10
adox   rdi,r11
mulx   r11,r10,QWORD PTR [rax+0x10]
adcx   rdi,r10
adox   r15,r11
mulx   r11,r10,QWORD PTR [rax+0x18]
adcx   r15,r10
adox   r14,r11
adcx   r14,rbx
mov    rdx,r13
imul   rdx,r12
xor    ebx,ebx
mulx   r11,r10,QWORD PTR [rsi]
adcx   r13,r10
adox   r9,r11
mulx   r11,r10,QWORD PTR [rsi+0x8]
adcx   r9,r10
adox   rdi,r11
mulx   r11,r10,QWORD PTR [rsi+0x10]
adcx   rdi,r10
adox   r15,r11
mulx   r11,r10,QWORD PTR [rsi+0x18]
adcx   r15,r10
adox   r14,r11
adcx   r14,rbx
mov    rdx,QWORD PTR [r8+0x18]
xor    ebx,ebx
mulx   r11,r10,QWORD PTR [rax]
adcx   r9,r10
adox   rdi,r11
mulx   r11,r10,QWORD PTR [rax+0x8]
adcx   rdi,r10
adox   r15,r11
mulx   r11,r10,QWORD PTR [rax+0x10]
adcx   r15,r10
adox   r14,r11
mulx   r11,r10,QWORD PTR [rax+0x18]
adcx   r14,r10
adox   r13,r11
adcx   r13,rbx
mov    rdx,r9
imul   rdx,r12
xor    ebx,ebx
mulx   r11,r10,QWORD PTR [rsi]
adcx   r9,r10
adox   rdi,r11
mulx   r11,r10,QWORD PTR [rsi+0x8]
adcx   rdi,r10
adox   r15,r11
mulx   r11,r10,QWORD PTR [rsi+0x10]
adcx   r15,r10
adox   r14,r11
mulx   r11,r10,QWORD PTR [rsi+0x18]
adcx   r14,r10
adox   r13,r11
adcx   r13,rbx
movabs rdx,0x3c208c16d87cfd47
movabs rax,0x687e956e978e3573
movabs r8,0x30644e72e131a029
cmp    rdi,rdx
lea    rdx,[r15+rax*1]
movabs rax,0x97816a916871ca8d
sbb    rdx,0x0
setb   r9b
cmp    r15,rax
movabs rax,0x47afba497e7ea7a3
setb   r10b
lea    rbx,[r14+rax*1]
movabs rax,0xb85045b68181585d
or     r10b,r9b
movzx  r9d,r10b
sub    rbx,r9
setb   r9b
cmp    r14,rax
movabs rax,0xcf9bb18d1ece5fd7
setb   r10b
lea    r11,[r13+rax*1+0x0]
or     r10b,r9b
movzx  r9d,r10b
sub    r11,r9
setb   r9b
cmp    r13,r8
setb   r10b
cmp    rcx,r8
setb   r8b
or     r8b,BYTE PTR [rsp+0xec0]
jne    e2cf0 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x2750>
mov    rcx,QWORD PTR [rsp+0x1c0]
movabs rax,0xc3df73e9278302b9
add    rcx,rax
or     r10b,r9b
je     e2d25 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x2785>
mov    rdx,r15
mov    rbx,r14
mov    r11,r13
jmp    e2d32 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x2792>
nop    WORD PTR [rax+rax*1+0x0]
mov    rsi,QWORD PTR [rsp+0xe80]
mov    rax,QWORD PTR [rsp+0xfc0]
mov    QWORD PTR [rsp+0x280],rcx
mov    rcx,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0x180],rsi
mov    QWORD PTR [rsp+0x200],rax
or     r10b,r9b
jne    e2cdf <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x273f>
movabs rax,0xc3df73e9278302b9
add    rdi,rax
mov    QWORD PTR [rsp+0x40],rdi
mov    QWORD PTR [rsp+0x48],rdx
mov    QWORD PTR [rsp+0x50],rbx
mov    QWORD PTR [rsp+0x58],r11
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
lea    r14,[rsp+0x5c0]
lea    rbx,[rsp+0x40]
mov    rdi,rcx
mov    r8,QWORD PTR [rsp+0x180]
mov    r9,QWORD PTR [rsp+0x200]
mov    r10,QWORD PTR [rsp+0x280]
xor    r15d,r15d
lea    r13,[rsp+0x5c0]
add    rdi,QWORD PTR [rbx]
adc    r8,QWORD PTR [rbx+0x8]
adc    r9,QWORD PTR [rbx+0x10]
adc    r10,QWORD PTR [rbx+0x18]
mov    rax,rdi
mov    rcx,r8
mov    rdx,r9
mov    rsi,r10
sub    rax,QWORD PTR [r14]
sbb    rcx,QWORD PTR [r14+0x8]
sbb    rdx,QWORD PTR [r14+0x10]
sbb    rsi,QWORD PTR [r14+0x18]
cmovae rdi,rax
cmovae r8,rcx
cmovae r9,rdx
cmovae r10,rsi
mov    QWORD PTR [rsp+0x280],r10
mov    QWORD PTR [rsp+0x200],r9
mov    QWORD PTR [rsp+0x180],r8
mov    QWORD PTR [rsp+0x1c0],rdi
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
vmovaps ymm0,YMMWORD PTR [rsp+0x1400]
mov    r8,QWORD PTR [rsp+0x480]
mov    r9,QWORD PTR [rsp+0x4c0]
mov    r10,QWORD PTR [rsp+0x300]
mov    r11,QWORD PTR [rsp+0x2c0]
vmovaps YMMWORD PTR [rsp+0x40],ymm0
sub    r11,QWORD PTR [rbx]
sbb    r10,QWORD PTR [rbx+0x8]
sbb    r9,QWORD PTR [rbx+0x10]
sbb    r8,QWORD PTR [rbx+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r14]
mov    rcx,QWORD PTR [r14+0x8]
mov    rdx,QWORD PTR [r14+0x10]
mov    rsi,QWORD PTR [r14+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    r11,rax
adc    r10,rcx
adc    r9,rdx
adc    r8,rsi
lea    rax,[rsp+0x1420]
mov    QWORD PTR [rsp+0x480],r8
mov    QWORD PTR [rsp+0x2c0],r11
mov    QWORD PTR [rsp+0x300],r10
mov    QWORD PTR [rsp+0x4c0],r9
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
vmovups ymm0,YMMWORD PTR [rax]
mov    r11,QWORD PTR [rsp+0x340]
mov    r10,QWORD PTR [rsp+0x900]
mov    r9,QWORD PTR [rsp+0x880]
mov    r8,QWORD PTR [rsp+0x240]
vmovaps YMMWORD PTR [rsp+0x40],ymm0
sub    r11,QWORD PTR [rbx]
sbb    r10,QWORD PTR [rbx+0x8]
sbb    r9,QWORD PTR [rbx+0x10]
sbb    r8,QWORD PTR [rbx+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r14]
mov    rcx,QWORD PTR [r14+0x8]
mov    rdx,QWORD PTR [r14+0x10]
mov    rsi,QWORD PTR [r14+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    r11,rax
adc    r10,rcx
adc    r9,rdx
adc    r8,rsi
mov    QWORD PTR [rsp+0x240],r8
mov    QWORD PTR [rsp+0x340],r11
mov    QWORD PTR [rsp+0x900],r10
mov    QWORD PTR [rsp+0x880],r9
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
mov    r12,r11
mov    rbx,r10
mov    r11,r9
xor    r10d,r10d
xor    r9d,r9d
xor    edi,edi
lea    r14,[rsp+0x40]
vmovups ymm0,YMMWORD PTR [rsp+0x8e0]
vmovaps YMMWORD PTR [rsp+0x40],ymm0
vxorps xmm0,xmm0,xmm0
sub    r10,QWORD PTR [r14]
sbb    r9,QWORD PTR [r14+0x8]
sbb    rdi,QWORD PTR [r14+0x10]
sbb    r15,QWORD PTR [r14+0x18]
sbb    r8,r8
mov    rax,QWORD PTR [r13+0x0]
mov    rcx,QWORD PTR [r13+0x8]
mov    rdx,QWORD PTR [r13+0x10]
mov    rsi,QWORD PTR [r13+0x18]
and    rax,r8
and    rcx,r8
and    rdx,r8
and    rsi,r8
add    r10,rax
adc    r9,rcx
adc    rdi,rdx
adc    r15,rsi
mov    QWORD PTR [rsp+0x580],r15
mov    QWORD PTR [rsp+0x40],r12
mov    QWORD PTR [rsp+0x48],rbx
mov    QWORD PTR [rsp+0x50],r11
mov    QWORD PTR [rsp+0xec0],r10
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
mov    r15,rdi
mov    r13,r9
mov    rdi,r10
xor    ecx,ecx
xor    edx,edx
xor    esi,esi
lea    r11,[rsp+0x5c0]
lea    rbx,[rsp+0x40]
mov    rax,QWORD PTR [rsp+0x240]
mov    QWORD PTR [rsp+0x58],rax
xor    eax,eax
sub    rax,QWORD PTR [rbx]
sbb    rcx,QWORD PTR [rbx+0x8]
sbb    rdx,QWORD PTR [rbx+0x10]
sbb    rsi,QWORD PTR [rbx+0x18]
sbb    r12,r12
mov    r8,QWORD PTR [r11]
mov    r9,QWORD PTR [r11+0x8]
mov    r10,QWORD PTR [r11+0x10]
mov    r14,QWORD PTR [r11+0x18]
and    r8,r12
and    r9,r12
and    r10,r12
and    r14,r12
add    rax,r8
adc    rcx,r9
adc    rdx,r10
adc    rsi,r14
movabs r12,0x87d20782e4866389
lea    r11,[rsp+0x5c0]
lea    rbx,[rsp+0x40]
mov    r8,QWORD PTR [rsp+0x2c0]
mov    QWORD PTR [rsp+0x40],r8
mov    r8,QWORD PTR [rsp+0x300]
mov    QWORD PTR [rsp+0x48],r8
mov    r8,QWORD PTR [rsp+0x4c0]
mov    QWORD PTR [rsp+0x50],r8
mov    r8,QWORD PTR [rsp+0x480]
mov    QWORD PTR [rsp+0x58],r8
mov    QWORD PTR [rsp+0x60],rax
mov    QWORD PTR [rsp+0x68],rcx
mov    rcx,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rsp+0x70],rdx
mov    QWORD PTR [rsp+0x78],rsi
lea    rsi,[rsp+0x80]
vmovdqu YMMWORD PTR [rsi],ymm1
mov    rsi,QWORD PTR [rsp+0x840]
mov    QWORD PTR [rsp+0x5c0],rcx
mov    rcx,QWORD PTR [rsp+0x400]
mov    QWORD PTR [rsp+0x5c8],rsi
mov    rsi,QWORD PTR [rsp+0x380]
mov    QWORD PTR [rsp+0x5d0],rcx
mov    rcx,QWORD PTR [rsp+0x580]
mov    QWORD PTR [rsp+0x5d8],rsi
mov    QWORD PTR [rsp+0x5e0],rdi
mov    QWORD PTR [rsp+0x5e8],r13
mov    QWORD PTR [rsp+0x5f0],r15
mov    rdi,r15
lea    r15,[rsp+0x600]
mov    QWORD PTR [rsp+0x5f8],rcx
mov    QWORD PTR [rsp+0x600],r12
vmovups XMMWORD PTR [r15+0x8],xmm0
mov    QWORD PTR [r15+0x18],0x0
xor    r14d,r14d
xor    eax,eax
xor    esi,esi
xor    ecx,ecx
xor    r12d,r12d
mov    rdx,QWORD PTR [r11]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x8]
adcx   rax,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x10]
adcx   rsi,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x18]
adcx   rcx,r8
adox   r12,r9
adcx   r12,r10
mov    rdx,QWORD PTR [r11+0x20]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx+0x20]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x28]
adcx   rax,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x30]
adcx   rsi,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x38]
adcx   rcx,r8
adox   r12,r9
adcx   r12,r10
mov    rdx,r14
imul   rdx,QWORD PTR [r11+0x40]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx+0x40]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x48]
adcx   rax,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x50]
adcx   rsi,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x58]
adcx   rcx,r8
adox   r12,r9
adcx   r12,r10
mov    rdx,QWORD PTR [r11+0x8]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx]
adcx   rax,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x8]
adcx   rsi,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x10]
adcx   rcx,r8
adox   r12,r9
mulx   r9,r8,QWORD PTR [rbx+0x18]
adcx   r12,r8
adox   r14,r9
adcx   r14,r10
mov    rdx,QWORD PTR [r11+0x28]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx+0x20]
adcx   rax,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x28]
adcx   rsi,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x30]
adcx   rcx,r8
adox   r12,r9
mulx   r9,r8,QWORD PTR [rbx+0x38]
adcx   r12,r8
adox   r14,r9
adcx   r14,r10
mov    rdx,rax
imul   rdx,QWORD PTR [r11+0x40]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx+0x40]
adcx   rax,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x48]
adcx   rsi,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x50]
adcx   rcx,r8
adox   r12,r9
mulx   r9,r8,QWORD PTR [rbx+0x58]
adcx   r12,r8
adox   r14,r9
adcx   r14,r10
mov    rdx,QWORD PTR [r11+0x10]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx]
adcx   rsi,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x8]
adcx   rcx,r8
adox   r12,r9
mulx   r9,r8,QWORD PTR [rbx+0x10]
adcx   r12,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x18]
adcx   r14,r8
adox   rax,r9
adcx   rax,r10
mov    rdx,QWORD PTR [r11+0x30]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx+0x20]
adcx   rsi,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x28]
adcx   rcx,r8
adox   r12,r9
mulx   r9,r8,QWORD PTR [rbx+0x30]
adcx   r12,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x38]
adcx   r14,r8
adox   rax,r9
adcx   rax,r10
mov    rdx,rsi
imul   rdx,QWORD PTR [r11+0x40]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx+0x40]
adcx   rsi,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x48]
adcx   rcx,r8
adox   r12,r9
mulx   r9,r8,QWORD PTR [rbx+0x50]
adcx   r12,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x58]
adcx   r14,r8
adox   rax,r9
adcx   rax,r10
mov    rdx,QWORD PTR [r11+0x18]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx]
adcx   rcx,r8
adox   r12,r9
mulx   r9,r8,QWORD PTR [rbx+0x8]
adcx   r12,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x10]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x18]
adcx   rax,r8
adox   rsi,r9
adcx   rsi,r10
mov    rdx,QWORD PTR [r11+0x38]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx+0x20]
adcx   rcx,r8
adox   r12,r9
mulx   r9,r8,QWORD PTR [rbx+0x28]
adcx   r12,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x30]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x38]
adcx   rax,r8
adox   rsi,r9
adcx   rsi,r10
mov    rdx,rcx
imul   rdx,QWORD PTR [r11+0x40]
xor    r10d,r10d
mulx   r9,r8,QWORD PTR [rbx+0x40]
adcx   rcx,r8
adox   r12,r9
mulx   r9,r8,QWORD PTR [rbx+0x48]
adcx   r12,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x50]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x58]
adcx   rax,r8
adox   rsi,r9
adcx   rsi,r10
movabs rdx,0x97816a916871ca8d
mov    QWORD PTR [rsp+0xfc0],rsi
lea    r11,[rsp+0x40]
mov    rcx,QWORD PTR [rsp+0x2c0]
mov    r9,QWORD PTR [rsp+0xec0]
mov    QWORD PTR [rsp+0x2c0],r12
mov    QWORD PTR [rsp+0x40],rcx
mov    rcx,QWORD PTR [rsp+0x300]
mov    QWORD PTR [rsp+0x300],r14
mov    QWORD PTR [rsp+0x5c0],r9
mov    r9,QWORD PTR [rsp+0x580]
mov    QWORD PTR [rsp+0x5c8],r13
mov    QWORD PTR [rsp+0x5d0],rdi
lea    r13,[rsp+0x5c0]
mov    QWORD PTR [rsp+0x48],rcx
mov    rcx,QWORD PTR [rsp+0x4c0]
mov    QWORD PTR [rsp+0x5d8],r9
mov    r9,QWORD PTR [rsp+0x840]
mov    QWORD PTR [rsp+0x4c0],rax
mov    QWORD PTR [rsp+0x50],rcx
mov    rcx,QWORD PTR [rsp+0x480]
mov    QWORD PTR [rsp+0x58],rcx
mov    rcx,QWORD PTR [rsp+0x340]
mov    QWORD PTR [rsp+0x60],rcx
mov    rcx,QWORD PTR [rsp+0x900]
mov    QWORD PTR [rsp+0x68],rcx
mov    rcx,QWORD PTR [rsp+0x880]
mov    QWORD PTR [rsp+0x70],rcx
movabs rcx,0x3c208c16d87cfd47
cmp    r12,rcx
movabs rcx,0x687e956e978e3573
lea    r8,[r14+rcx*1]
sbb    r8,0x0
setb   cl
cmp    r14,rdx
setb   dl
or     dl,cl
movzx  ecx,dl
movabs rdx,0x47afba497e7ea7a3
lea    r14,[rax+rdx*1]
movabs rdx,0xb85045b68181585d
sub    r14,rcx
setb   cl
cmp    rax,rdx
setb   dl
or     dl,cl
movabs rcx,0xcf9bb18d1ece5fd7
add    rcx,rsi
mov    rsi,QWORD PTR [rsp+0x240]
movzx  edx,dl
sub    rcx,rdx
setb   BYTE PTR [rsp+0x240]
mov    QWORD PTR [rsp+0x78],rsi
lea    rsi,[rsp+0x80]
vmovdqu YMMWORD PTR [rsi],ymm1
mov    rsi,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rsp+0x5e0],rsi
mov    rsi,QWORD PTR [rsp+0x400]
mov    QWORD PTR [rsp+0x5e8],r9
mov    r9,QWORD PTR [rsp+0x380]
mov    QWORD PTR [rsp+0x5f0],rsi
movabs rsi,0x87d20782e4866389
mov    QWORD PTR [rsp+0x5f8],r9
mov    QWORD PTR [rsp+0x600],rsi
vmovups XMMWORD PTR [r15+0x8],xmm0
mov    QWORD PTR [r15+0x18],0x0
xor    ebx,ebx
xor    r15d,r15d
xor    esi,esi
xor    r9d,r9d
xor    eax,eax
mov    rdx,QWORD PTR [r13+0x0]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11]
adcx   rbx,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [r11+0x8]
adcx   r15,r10
adox   rsi,r12
mulx   r12,r10,QWORD PTR [r11+0x10]
adcx   rsi,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [r11+0x18]
adcx   r9,r10
adox   rax,r12
adcx   rax,rdi
mov    rdx,QWORD PTR [r13+0x20]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11+0x20]
adcx   rbx,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [r11+0x28]
adcx   r15,r10
adox   rsi,r12
mulx   r12,r10,QWORD PTR [r11+0x30]
adcx   rsi,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [r11+0x38]
adcx   r9,r10
adox   rax,r12
adcx   rax,rdi
mov    rdx,rbx
imul   rdx,QWORD PTR [r13+0x40]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11+0x40]
adcx   rbx,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [r11+0x48]
adcx   r15,r10
adox   rsi,r12
mulx   r12,r10,QWORD PTR [r11+0x50]
adcx   rsi,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [r11+0x58]
adcx   r9,r10
adox   rax,r12
adcx   rax,rdi
mov    rdx,QWORD PTR [r13+0x8]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11]
adcx   r15,r10
adox   rsi,r12
mulx   r12,r10,QWORD PTR [r11+0x8]
adcx   rsi,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [r11+0x10]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [r11+0x18]
adcx   rax,r10
adox   rbx,r12
adcx   rbx,rdi
mov    rdx,QWORD PTR [r13+0x28]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11+0x20]
adcx   r15,r10
adox   rsi,r12
mulx   r12,r10,QWORD PTR [r11+0x28]
adcx   rsi,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [r11+0x30]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [r11+0x38]
adcx   rax,r10
adox   rbx,r12
adcx   rbx,rdi
mov    rdx,r15
imul   rdx,QWORD PTR [r13+0x40]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11+0x40]
adcx   r15,r10
adox   rsi,r12
mulx   r12,r10,QWORD PTR [r11+0x48]
adcx   rsi,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [r11+0x50]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [r11+0x58]
adcx   rax,r10
adox   rbx,r12
adcx   rbx,rdi
mov    rdx,QWORD PTR [r13+0x10]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11]
adcx   rsi,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [r11+0x8]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [r11+0x10]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [r11+0x18]
adcx   rbx,r10
adox   r15,r12
adcx   r15,rdi
mov    rdx,QWORD PTR [r13+0x30]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11+0x20]
adcx   rsi,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [r11+0x28]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [r11+0x30]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [r11+0x38]
adcx   rbx,r10
adox   r15,r12
adcx   r15,rdi
mov    rdx,rsi
imul   rdx,QWORD PTR [r13+0x40]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11+0x40]
adcx   rsi,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [r11+0x48]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [r11+0x50]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [r11+0x58]
adcx   rbx,r10
adox   r15,r12
adcx   r15,rdi
mov    rdx,QWORD PTR [r13+0x18]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [r11+0x8]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [r11+0x10]
adcx   rbx,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [r11+0x18]
adcx   r15,r10
adox   rsi,r12
adcx   rsi,rdi
mov    rdx,QWORD PTR [r13+0x38]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11+0x20]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [r11+0x28]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [r11+0x30]
adcx   rbx,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [r11+0x38]
adcx   r15,r10
adox   rsi,r12
adcx   rsi,rdi
mov    rdx,r9
imul   rdx,QWORD PTR [r13+0x40]
xor    edi,edi
mulx   r12,r10,QWORD PTR [r11+0x40]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [r11+0x48]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [r11+0x50]
adcx   rbx,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [r11+0x58]
adcx   r15,r10
adox   rsi,r12
adcx   rsi,rdi
movabs rdx,0x687e956e978e3573
movabs rdi,0x3c208c16d87cfd47
mov    QWORD PTR [rsp+0x480],rax
mov    QWORD PTR [rsp+0x380],rsi
mov    QWORD PTR [rsp+0x580],rbx
mov    QWORD PTR [rsp+0x3c0],r15
cmp    rax,rdi
lea    rdx,[rbx+rdx*1]
movabs rax,0x97816a916871ca8d
sbb    rdx,0x0
setb   r9b
cmp    rbx,rax
movabs rax,0x47afba497e7ea7a3
mov    QWORD PTR [rsp+0x900],rdx
setb   dil
add    rax,r15
or     dil,r9b
movzx  edx,dil
movabs rdi,0xb85045b68181585d
sub    rax,rdx
setb   dl
cmp    r15,rdi
mov    QWORD PTR [rsp+0x400],rax
setb   dil
or     dil,dl
movzx  edx,dil
movabs rdi,0xcf9bb18d1ece5fd7
add    rdi,rsi
mov    rsi,QWORD PTR [rsp+0xfc0]
sub    rdi,rdx
movabs rdx,0x30644e72e131a029
setb   BYTE PTR [rsp+0x340]
mov    QWORD PTR [rsp+0xe80],rdi
cmp    rsi,rdx
setb   dl
or     dl,BYTE PTR [rsp+0x240]
jne    e3ba0 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x3600>
movabs rax,0xc3df73e9278302b9
add    QWORD PTR [rsp+0x2c0],rax
jmp    e3bb3 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x3613>
nop    DWORD PTR [rax+0x0]
mov    r8,QWORD PTR [rsp+0x300]
mov    r14,QWORD PTR [rsp+0x4c0]
mov    rcx,rsi
mov    rax,QWORD PTR [rsp+0xa80]
and    eax,0x7
mov    rdx,QWORD PTR [rsp+0x2c0]
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
lea    r13,[rsp+0x5c0]
lea    rbx,[rsp+0x40]
lea    r15,[rsp+0x500]
shl    eax,0x5
vmovups ymm0,YMMWORD PTR [rsp+rax*1+0x1600]
mov    QWORD PTR [rsp+0x500],rdx
mov    QWORD PTR [rsp+0x508],r8
mov    QWORD PTR [rsp+0x510],r14
mov    QWORD PTR [rsp+0x518],rcx
movabs r14,0x87d20782e4866389
vmovaps YMMWORD PTR [rsp+0x40],ymm0
xor    r12d,r12d
xor    r10d,r10d
xor    r9d,r9d
xor    ecx,ecx
xor    r8d,r8d
mov    rdx,QWORD PTR [rbx]
xor    r11d,r11d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   r12,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   r10,rsi
adox   r9,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   rcx,rsi
adox   r8,rdi
adcx   r8,r11
mov    rdx,r12
imul   rdx,r14
xor    r11d,r11d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   r12,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   r10,rsi
adox   r9,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   rcx,rsi
adox   r8,rdi
adcx   r8,r11
mov    rdx,QWORD PTR [rbx+0x8]
xor    r11d,r11d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   r10,rsi
adox   r9,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   r8,rsi
adox   r12,rdi
adcx   r12,r11
mov    rdx,r10
imul   rdx,r14
xor    r11d,r11d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   r10,rsi
adox   r9,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   r8,rsi
adox   r12,rdi
adcx   r12,r11
mov    rdx,QWORD PTR [rbx+0x10]
xor    r11d,r11d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   r8,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   r12,rsi
adox   r10,rdi
adcx   r10,r11
mov    rdx,r9
imul   rdx,r14
xor    r11d,r11d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   r8,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   r12,rsi
adox   r10,rdi
adcx   r10,r11
mov    rdx,QWORD PTR [rbx+0x18]
xor    r11d,r11d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   r8,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   r12,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   r10,rsi
adox   r9,rdi
adcx   r9,r11
mov    rdx,rcx
imul   rdx,r14
xor    r11d,r11d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   r8,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   r12,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   r10,rsi
adox   r9,rdi
adcx   r9,r11
movabs rcx,0x687e956e978e3573
mov    QWORD PTR [rsp+0x240],r8
mov    QWORD PTR [rsp+0xfc0],r12
mov    QWORD PTR [rsp+0xec0],r10
mov    QWORD PTR [rsp+0x880],r9
lea    rdx,[r12+rcx*1]
movabs rcx,0x3c208c16d87cfd47
cmp    r8,rcx
mov    r8,QWORD PTR [rsp+0x380]
sbb    rdx,0x0
mov    QWORD PTR [rsp+0x2c0],rdx
movabs rdx,0x97816a916871ca8d
setb   cl
cmp    r12,rdx
setb   dl
or     dl,cl
movabs rcx,0x47afba497e7ea7a3
lea    rsi,[r10+rcx*1]
movzx  ecx,dl
movabs rdx,0xb85045b68181585d
sub    rsi,rcx
setb   cl
cmp    r10,rdx
mov    QWORD PTR [rsp+0x300],rsi
setb   dl
or     dl,cl
movabs rcx,0xcf9bb18d1ece5fd7
lea    rsi,[r9+rcx*1]
movzx  ecx,dl
sub    rsi,rcx
movabs rcx,0x30644e72e131a029
setb   BYTE PTR [rsp+0x840]
cmp    r8,rcx
mov    QWORD PTR [rsp+0x4c0],rsi
setb   cl
or     cl,BYTE PTR [rsp+0x340]
jne    e3ff0 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x3a50>
mov    rdx,QWORD PTR [rsp+0x480]
movabs rcx,0xc3df73e9278302b9
mov    rsi,QWORD PTR [rsp+0x400]
mov    r8,QWORD PTR [rsp+0xe80]
add    rdx,rcx
mov    rcx,rdx
mov    rdx,QWORD PTR [rsp+0x900]
jmp    e4008 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x3a68>
data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
mov    rdx,QWORD PTR [rsp+0x580]
mov    rsi,QWORD PTR [rsp+0x3c0]
mov    rcx,QWORD PTR [rsp+0x480]
vmovups ymm0,YMMWORD PTR [rsp+rax*1+0x1600]
mov    QWORD PTR [rsp+0x500],rcx
mov    QWORD PTR [rsp+0x508],rdx
mov    QWORD PTR [rsp+0x510],rsi
mov    QWORD PTR [rsp+0x518],r8
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
lea    r15,[rsp+0x5c0]
lea    r12,[rsp+0x40]
lea    r13,[rsp+0x500]
vmovaps YMMWORD PTR [rsp+0x40],ymm0
xor    r11d,r11d
xor    r10d,r10d
xor    r9d,r9d
xor    ecx,ecx
xor    ebx,ebx
mov    rdx,QWORD PTR [r12]
xor    r8d,r8d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   r11,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   r10,rsi
adox   r9,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   rcx,rsi
adox   rbx,rdi
adcx   rbx,r8
mov    rdx,r11
imul   rdx,r14
xor    r8d,r8d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   r11,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   r10,rsi
adox   r9,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   rcx,rsi
adox   rbx,rdi
adcx   rbx,r8
mov    rdx,QWORD PTR [r12+0x8]
xor    r8d,r8d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   r10,rsi
adox   r9,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   rcx,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   rbx,rsi
adox   r11,rdi
adcx   r11,r8
mov    rdx,r10
imul   rdx,r14
xor    r8d,r8d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   r10,rsi
adox   r9,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   rcx,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   rbx,rsi
adox   r11,rdi
adcx   r11,r8
mov    rdx,QWORD PTR [r12+0x10]
xor    r8d,r8d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   rcx,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   rbx,rsi
adox   r11,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   r11,rsi
adox   r10,rdi
adcx   r10,r8
mov    rdx,r9
imul   rdx,r14
xor    r8d,r8d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   r9,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   rcx,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   rbx,rsi
adox   r11,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   r11,rsi
adox   r10,rdi
adcx   r10,r8
mov    rdx,QWORD PTR [r12+0x18]
xor    r8d,r8d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   rcx,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   rbx,rsi
adox   r11,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   r11,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   r10,rsi
adox   r9,rdi
adcx   r9,r8
mov    rdx,rcx
imul   rdx,r14
xor    r8d,r8d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   rcx,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   rbx,rsi
adox   r11,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   r11,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   r10,rsi
adox   r9,rdi
adcx   r9,r8
movabs rcx,0x687e956e978e3573
movabs r13,0x97816a916871ca8d
mov    QWORD PTR [rsp+0x580],rax
mov    QWORD PTR [rsp+0x480],rbx
mov    QWORD PTR [rsp+0x1100],r11
mov    QWORD PTR [rsp+0x1140],r10
vmovdqa YMMWORD PTR [rsp+0x5c0],ymm1
lea    rdx,[r11+rcx*1]
movabs rcx,0x3c208c16d87cfd47
vmovups ymm0,YMMWORD PTR [rsp+rax*1+0x1600]
mov    r8,QWORD PTR [rsp+0x1c0]
mov    rax,QWORD PTR [rsp+0x180]
cmp    rbx,rcx
sbb    rdx,0x0
setb   cl
cmp    r11,r13
mov    QWORD PTR [rsp+0x380],rdx
lea    r13,[rsp+0x5c0]
lea    r11,[rsp+0x500]
setb   dl
or     dl,cl
movabs rcx,0x47afba497e7ea7a3
mov    QWORD PTR [rsp+0x40],r8
mov    QWORD PTR [rsp+0x48],rax
mov    r8,QWORD PTR [rsp+0x200]
mov    rax,QWORD PTR [rsp+0x280]
lea    rsi,[r10+rcx*1]
movzx  ecx,dl
movabs rdx,0xb85045b68181585d
sub    rsi,rcx
setb   cl
cmp    r10,rdx
mov    QWORD PTR [rsp+0x340],rsi
vmovaps YMMWORD PTR [rsp+0x500],ymm0
lea    r10,[rsp+0x40]
setb   dl
or     dl,cl
movabs rcx,0xcf9bb18d1ece5fd7
mov    QWORD PTR [rsp+0x50],r8
mov    QWORD PTR [rsp+0x58],rax
movabs r8,0x87d20782e4866389
lea    rsi,[r9+rcx*1]
movzx  ecx,dl
sub    rsi,rcx
mov    QWORD PTR [rsp+0x900],rsi
setb   BYTE PTR [rsp+0x400]
xor    eax,eax
xor    r15d,r15d
xor    ebx,ebx
xor    ecx,ecx
xor    r12d,r12d
mov    rdx,QWORD PTR [r10]
xor    r14d,r14d
mulx   rdi,rsi,QWORD PTR [r11]
adcx   rax,rsi
adox   r15,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x8]
adcx   r15,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x10]
adcx   rbx,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x18]
adcx   rcx,rsi
adox   r12,rdi
adcx   r12,r14
mov    rdx,rax
imul   rdx,r8
xor    r14d,r14d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   rax,rsi
adox   r15,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   r15,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   rbx,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   rcx,rsi
adox   r12,rdi
adcx   r12,r14
mov    rdx,QWORD PTR [r10+0x8]
xor    r14d,r14d
mulx   rdi,rsi,QWORD PTR [r11]
adcx   r15,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x8]
adcx   rbx,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x10]
adcx   rcx,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x18]
adcx   r12,rsi
adox   rax,rdi
adcx   rax,r14
mov    rdx,r15
imul   rdx,r8
xor    r14d,r14d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   r15,rsi
adox   rbx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   rbx,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   rcx,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   r12,rsi
adox   rax,rdi
adcx   rax,r14
mov    rdx,QWORD PTR [r10+0x10]
xor    r14d,r14d
mulx   rdi,rsi,QWORD PTR [r11]
adcx   rbx,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x8]
adcx   rcx,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x10]
adcx   r12,rsi
adox   rax,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x18]
adcx   rax,rsi
adox   r15,rdi
adcx   r15,r14
mov    rdx,rbx
imul   rdx,r8
xor    r14d,r14d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   rbx,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   rcx,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   r12,rsi
adox   rax,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   rax,rsi
adox   r15,rdi
adcx   r15,r14
mov    rdx,QWORD PTR [r10+0x18]
xor    r14d,r14d
mulx   rdi,rsi,QWORD PTR [r11]
adcx   rcx,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x8]
adcx   r12,rsi
adox   rax,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x10]
adcx   rax,rsi
adox   r15,rdi
mulx   rdi,rsi,QWORD PTR [r11+0x18]
adcx   r15,rsi
adox   rbx,rdi
adcx   rbx,r14
mov    rdx,rcx
imul   rdx,r8
xor    r14d,r14d
mulx   rdi,rsi,QWORD PTR [r13+0x0]
adcx   rcx,rsi
adox   r12,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x8]
adcx   r12,rsi
adox   rax,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x10]
adcx   rax,rsi
adox   r15,rdi
mulx   rdi,rsi,QWORD PTR [r13+0x18]
adcx   r15,rsi
adox   rbx,rdi
adcx   rbx,r14
mov    r8,r12
mov    r12,rbx
mov    rbx,r15
mov    r15,rax
movabs rax,0x687e956e978e3573
movabs r10,0x3c208c16d87cfd47
mov    QWORD PTR [rsp+0xe80],r9
lea    rdx,[r15+rax*1]
cmp    r8,r10
movabs rax,0x97816a916871ca8d
sbb    rdx,0x0
setb   cl
cmp    r15,rax
setb   sil
or     sil,cl
movzx  ecx,sil
movabs rsi,0x47afba497e7ea7a3
lea    r13,[rbx+rsi*1]
movabs rsi,0xb85045b68181585d
sub    r13,rcx
setb   cl
cmp    rbx,rsi
setb   sil
or     sil,cl
movzx  ecx,sil
movabs rsi,0xcf9bb18d1ece5fd7
lea    r14,[r12+rsi*1]
movabs rsi,0x30644e72e131a029
sub    r14,rcx
setb   BYTE PTR [rsp+0x3c0]
cmp    r12,rsi
setb   cl
cmp    r9,rsi
mov    r9,QWORD PTR [rsp+0x880]
setb   dil
cmp    r9,rsi
setb   sil
or     sil,BYTE PTR [rsp+0x840]
jne    e4810 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x4270>
movabs rsi,0xc3df73e9278302b9
add    QWORD PTR [rsp+0x240],rsi
mov    rax,QWORD PTR [rsp+0x580]
or     dil,BYTE PTR [rsp+0x400]
je     e484a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x42aa>
mov    rsi,QWORD PTR [rsp+0x1100]
mov    r9,QWORD PTR [rsp+0x1140]
mov    rdi,QWORD PTR [rsp+0xe80]
mov    QWORD PTR [rsp+0x380],rsi
mov    QWORD PTR [rsp+0x340],r9
mov    QWORD PTR [rsp+0x900],rdi
jmp    e485c <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x42bc>
nop    DWORD PTR [rax+rax*1+0x0]
mov    rsi,QWORD PTR [rsp+0xfc0]
mov    rax,QWORD PTR [rsp+0xec0]
mov    QWORD PTR [rsp+0x4c0],r9
mov    QWORD PTR [rsp+0x2c0],rsi
mov    QWORD PTR [rsp+0x300],rax
mov    rax,QWORD PTR [rsp+0x580]
or     dil,BYTE PTR [rsp+0x400]
jne    e47d6 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x4236>
movabs rsi,0xc3df73e9278302b9
add    QWORD PTR [rsp+0x480],rsi
mov    rdi,QWORD PTR [rsp+0x28]
mov    r10,QWORD PTR [rsp+0xb00]
mov    r11,QWORD PTR [rsp+0xbc0]
mov    rsi,QWORD PTR [rsp+0xa00]
or     cl,BYTE PTR [rsp+0x3c0]
je     e1ab0 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1510>
mov    rdx,r15
mov    r13,rbx
mov    r14,r12
jmp    e1abd <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x151d>
add    rdi,0x18
vzeroupper
call   acf10 <_RNvMs4_NtCs6i54tJFfzR_5alloc7raw_vecINtB5_6RawVecANtNtCs9RL81RXvecK_11zopro_bn2543fq23Fq2j5_E8grow_oneCsh5u81WWlmio_10zopro_core>
mov    r11,QWORD PTR [rsp+0xbc0]
mov    r10,QWORD PTR [rsp+0xb00]
mov    rdi,QWORD PTR [rsp+0x28]
jmp    e1c37 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1697>
lea    rdi,[rsp+0x5c0]
lea    rsi,[rsp+0x1600]
vzeroupper
call   d96c0 <_RNvNtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernels12host_inverse>
mov    rcx,QWORD PTR [rsp+0x18]
mov    rdx,QWORD PTR [rsp+0x30]
vmovups ymm2,YMMWORD PTR [rip+0xfffffffffff2567c]        # 9f60 <GCC_except_table830+0x1a04>
mov    r9,QWORD PTR [rsp+0x28]
vpxor  xmm0,xmm0,xmm0
mov    rax,rcx
shl    rax,0x4
lea    r12,[rcx-0x1]
lea    rsi,[rax+rdx*1-0x10]
mov    rdx,QWORD PTR [rsp+0xf80]
lea    rax,[rdx*8+0x0]
neg    rax
mov    QWORD PTR [rsp+0x1180],rax
mov    rax,rcx
shl    rax,0x6
lea    r8,[rax+rax*4]
mov    rax,rdx
shl    rax,0x9
lea    rax,[rax+rax*4]
sub    r8,rax
mov    rax,QWORD PTR [rsp+0xf40]
add    r8,0xfffffffffffffec8
neg    rax
lea    rax,[rcx+rax*1-0x1]
shl    rax,0x6
lea    rcx,[rax+rax*4]
mov    rax,QWORD PTR [rsp+0x1180]
mov    QWORD PTR [rsp+0x10c0],rsi
mov    rsi,QWORD PTR [r9+0x28]
add    rax,r12
cmp    rax,rsi
jae    ec924 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc384>
mov    rax,QWORD PTR [r9+0x20]
mov    QWORD PTR [rsp+0x1000],rcx
mov    QWORD PTR [rsp+0x1080],r12
and    r12d,0x7
mov    QWORD PTR [rsp+0x1040],r8
lea    r9,[rsp+0xb40]
lea    rsi,[rsp+0x40]
lea    r11,[rsp+0xc00]
movabs rbx,0x87d20782e4866389
shl    r12d,0x5
mov    rcx,QWORD PTR [rax+rcx*1]
vmovups zmm1,ZMMWORD PTR [rax+r8*1+0x38]
mov    QWORD PTR [rsp+0xa00],rcx
mov    rcx,QWORD PTR [rax+r8*1]
mov    QWORD PTR [rsp+0x980],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x8]
mov    QWORD PTR [rsp+0x940],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x10]
mov    QWORD PTR [rsp+0x9c0],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x18]
mov    QWORD PTR [rsp+0xa80],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x20]
mov    QWORD PTR [rsp+0x900],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x28]
mov    QWORD PTR [rsp+0xa40],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x30]
vmovaps ZMMWORD PTR [rsp+0x11c0],zmm1
vmovups zmm1,ZMMWORD PTR [rax+r8*1+0x78]
mov    QWORD PTR [rsp+0x880],rcx
vmovaps ZMMWORD PTR [rsp+0x12c0],zmm1
vmovups ymm1,YMMWORD PTR [rax+r8*1+0xb8]
vmovaps YMMWORD PTR [rsp+0x1240],ymm1
vmovups zmm1,ZMMWORD PTR [rax+r8*1+0xf8]
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
vmovaps ZMMWORD PTR [rsp+0x500],zmm1
vmovaps ymm1,YMMWORD PTR [rsp+0x500]
vmovaps YMMWORD PTR [rsp+0xb40],ymm1
vmovups ymm1,YMMWORD PTR [rsp+r12*1+0x5c0]
vmovaps YMMWORD PTR [rsp+0x40],ymm1
xor    r14d,r14d
xor    r13d,r13d
xor    r10d,r10d
xor    eax,eax
xor    r15d,r15d
mov    rdx,QWORD PTR [rsi]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r9]
adcx   r14,rcx
adox   r13,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x8]
adcx   r13,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x10]
adcx   r10,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x18]
adcx   rax,rcx
adox   r15,rdi
adcx   r15,r8
mov    rdx,r14
imul   rdx,rbx
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r11]
adcx   r14,rcx
adox   r13,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x8]
adcx   r13,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x10]
adcx   r10,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x18]
adcx   rax,rcx
adox   r15,rdi
adcx   r15,r8
mov    rdx,QWORD PTR [rsi+0x8]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r9]
adcx   r13,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x8]
adcx   r10,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x10]
adcx   rax,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x18]
adcx   r15,rcx
adox   r14,rdi
adcx   r14,r8
mov    rdx,r13
imul   rdx,rbx
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r11]
adcx   r13,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x8]
adcx   r10,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x10]
adcx   rax,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x18]
adcx   r15,rcx
adox   r14,rdi
adcx   r14,r8
mov    rdx,QWORD PTR [rsi+0x10]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r9]
adcx   r10,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x8]
adcx   rax,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x10]
adcx   r15,rcx
adox   r14,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x18]
adcx   r14,rcx
adox   r13,rdi
adcx   r13,r8
mov    rdx,r10
imul   rdx,rbx
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r11]
adcx   r10,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x8]
adcx   rax,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x10]
adcx   r15,rcx
adox   r14,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x18]
adcx   r14,rcx
adox   r13,rdi
adcx   r13,r8
mov    rdx,QWORD PTR [rsi+0x18]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r9]
adcx   rax,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x8]
adcx   r15,rcx
adox   r14,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x10]
adcx   r14,rcx
adox   r13,rdi
mulx   rdi,rcx,QWORD PTR [r9+0x18]
adcx   r13,rcx
adox   r10,rdi
adcx   r10,r8
mov    rdx,rax
imul   rdx,rbx
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r11]
adcx   rax,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x8]
adcx   r15,rcx
adox   r14,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x10]
adcx   r14,rcx
adox   r13,rdi
mulx   rdi,rcx,QWORD PTR [r11+0x18]
adcx   r13,rcx
adox   r10,rdi
adcx   r10,r8
movabs rax,0x3c208c16d87cfd47
mov    QWORD PTR [rsp+0x400],r10
mov    QWORD PTR [rsp+0x480],r15
mov    QWORD PTR [rsp+0x440],r14
mov    QWORD PTR [rsp+0x380],r13
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
mov    r11,rbx
mov    r9,rsi
cmp    r15,rax
movabs rax,0x687e956e978e3573
lea    r15,[rsp+0xc00]
lea    rcx,[r14+rax*1]
sbb    rcx,0x0
mov    QWORD PTR [rsp+0x300],rcx
movabs rcx,0x97816a916871ca8d
setb   al
cmp    r14,rcx
lea    r14,[rsp+0xb40]
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0x47afba497e7ea7a3
lea    rdx,[r13+rcx*1+0x0]
movabs rcx,0xb85045b68181585d
sub    rdx,rax
setb   al
cmp    r13,rcx
mov    QWORD PTR [rsp+0x180],rdx
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0xcf9bb18d1ece5fd7
lea    rdx,[r10+rcx*1]
sub    rdx,rax
lea    rax,[rsp+0x520]
vmovups ymm1,YMMWORD PTR [rax]
mov    QWORD PTR [rsp+0x200],rdx
setb   BYTE PTR [rsp+0xac0]
vmovaps YMMWORD PTR [rsp+0xb40],ymm1
vmovups ymm1,YMMWORD PTR [rsp+r12*1+0x5c0]
vmovaps YMMWORD PTR [rsp+0x40],ymm1
xor    esi,esi
xor    ebx,ebx
xor    r13d,r13d
xor    eax,eax
xor    r10d,r10d
mov    rdx,QWORD PTR [r9]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r14]
adcx   rsi,rcx
adox   rbx,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x8]
adcx   rbx,rcx
adox   r13,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x10]
adcx   r13,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x18]
adcx   rax,rcx
adox   r10,rdi
adcx   r10,r8
mov    rdx,rsi
imul   rdx,r11
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r15]
adcx   rsi,rcx
adox   rbx,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x8]
adcx   rbx,rcx
adox   r13,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x10]
adcx   r13,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x18]
adcx   rax,rcx
adox   r10,rdi
adcx   r10,r8
mov    rdx,QWORD PTR [r9+0x8]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r14]
adcx   rbx,rcx
adox   r13,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x8]
adcx   r13,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x10]
adcx   rax,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x18]
adcx   r10,rcx
adox   rsi,rdi
adcx   rsi,r8
mov    rdx,rbx
imul   rdx,r11
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r15]
adcx   rbx,rcx
adox   r13,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x8]
adcx   r13,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x10]
adcx   rax,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x18]
adcx   r10,rcx
adox   rsi,rdi
adcx   rsi,r8
mov    rdx,QWORD PTR [r9+0x10]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r14]
adcx   r13,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x8]
adcx   rax,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x10]
adcx   r10,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x18]
adcx   rsi,rcx
adox   rbx,rdi
adcx   rbx,r8
mov    rdx,r13
imul   rdx,r11
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r15]
adcx   r13,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x8]
adcx   rax,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x10]
adcx   r10,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x18]
adcx   rsi,rcx
adox   rbx,rdi
adcx   rbx,r8
mov    rdx,QWORD PTR [r9+0x18]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r14]
adcx   rax,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x8]
adcx   r10,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x10]
adcx   rsi,rcx
adox   rbx,rdi
mulx   rdi,rcx,QWORD PTR [r14+0x18]
adcx   rbx,rcx
adox   r13,rdi
adcx   r13,r8
mov    rdx,rax
imul   rdx,r11
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r15]
adcx   rax,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x8]
adcx   r10,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x10]
adcx   rsi,rcx
adox   rbx,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x18]
adcx   rbx,rcx
adox   r13,rdi
adcx   r13,r8
movabs rax,0x3c208c16d87cfd47
movabs rcx,0x97816a916871ca8d
mov    QWORD PTR [rsp+0x4c0],r13
mov    QWORD PTR [rsp+0x840],r10
mov    QWORD PTR [rsp+0x340],rsi
mov    QWORD PTR [rsp+0x3c0],rbx
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
lea    r8,[rsp+0xb40]
cmp    r10,rax
movabs rax,0x687e956e978e3573
vmovups ymm1,YMMWORD PTR [rsp+r12*1+0x5c0]
lea    rax,[rsi+rax*1]
sbb    rax,0x0
mov    QWORD PTR [rsp+0x240],rax
setb   al
cmp    rsi,rcx
lea    rsi,[rsp+0x40]
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0x47afba497e7ea7a3
add    rcx,rbx
sub    rcx,rax
vmovaps YMMWORD PTR [rsp+0xb40],ymm1
vmovaps ymm1,YMMWORD PTR [rsp+0x1240]
mov    QWORD PTR [rsp+0x2c0],rcx
movabs rcx,0xb85045b68181585d
setb   al
cmp    rbx,rcx
lea    rbx,[rsp+0xc00]
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0xcf9bb18d1ece5fd7
add    rcx,r13
movabs r13,0x87d20782e4866389
sub    rcx,rax
mov    QWORD PTR [rsp+0x1c0],rcx
vmovaps YMMWORD PTR [rsp+0x40],ymm1
setb   BYTE PTR [rsp+0xb00]
xor    r15d,r15d
xor    r10d,r10d
xor    r11d,r11d
xor    eax,eax
xor    r9d,r9d
mov    rdx,QWORD PTR [rsi]
xor    r14d,r14d
mulx   rdi,rcx,QWORD PTR [r8]
adcx   r15,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x8]
adcx   r10,rcx
adox   r11,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x10]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x18]
adcx   rax,rcx
adox   r9,rdi
adcx   r9,r14
mov    rdx,r15
imul   rdx,r13
xor    r14d,r14d
mulx   rdi,rcx,QWORD PTR [rbx]
adcx   r15,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x8]
adcx   r10,rcx
adox   r11,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x10]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x18]
adcx   rax,rcx
adox   r9,rdi
adcx   r9,r14
mov    rdx,QWORD PTR [rsi+0x8]
xor    r14d,r14d
mulx   rdi,rcx,QWORD PTR [r8]
adcx   r10,rcx
adox   r11,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x8]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x10]
adcx   rax,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x18]
adcx   r9,rcx
adox   r15,rdi
adcx   r15,r14
mov    rdx,r10
imul   rdx,r13
xor    r14d,r14d
mulx   rdi,rcx,QWORD PTR [rbx]
adcx   r10,rcx
adox   r11,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x8]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x10]
adcx   rax,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x18]
adcx   r9,rcx
adox   r15,rdi
adcx   r15,r14
mov    rdx,QWORD PTR [rsi+0x10]
xor    r14d,r14d
mulx   rdi,rcx,QWORD PTR [r8]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x8]
adcx   rax,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x10]
adcx   r9,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x18]
adcx   r15,rcx
adox   r10,rdi
adcx   r10,r14
mov    rdx,r11
imul   rdx,r13
xor    r14d,r14d
mulx   rdi,rcx,QWORD PTR [rbx]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x8]
adcx   rax,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x10]
adcx   r9,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x18]
adcx   r15,rcx
adox   r10,rdi
adcx   r10,r14
mov    rdx,QWORD PTR [rsi+0x18]
xor    r14d,r14d
mulx   rdi,rcx,QWORD PTR [r8]
adcx   rax,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x8]
adcx   r9,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x10]
adcx   r15,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r8+0x18]
adcx   r10,rcx
adox   r11,rdi
adcx   r11,r14
mov    rdx,rax
imul   rdx,r13
xor    r14d,r14d
mulx   rdi,rcx,QWORD PTR [rbx]
adcx   rax,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x8]
adcx   r9,rcx
adox   r15,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x10]
adcx   r15,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x18]
adcx   r10,rcx
adox   r11,rdi
adcx   r11,r14
movabs rax,0x3c208c16d87cfd47
movabs rcx,0x97816a916871ca8d
mov    QWORD PTR [rsp+0x280],r11
mov    r13,r15
movabs r8,0x30644e72e131a029
mov    QWORD PTR [rsp+0x580],r10
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
cmp    r9,rax
movabs rax,0x687e956e978e3573
lea    rdi,[r15+rax*1]
sbb    rdi,0x0
setb   al
cmp    r15,rcx
mov    r15,QWORD PTR [rsp+0x280]
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0x47afba497e7ea7a3
lea    rsi,[r10+rcx*1]
movabs rcx,0xb85045b68181585d
sub    rsi,rax
setb   al
cmp    r10,rcx
movabs r10,0xc3df73e9278302b9
setb   cl
or     cl,al
movabs rax,0xcf9bb18d1ece5fd7
lea    rdx,[r15+rax*1]
movzx  ecx,cl
mov    rax,QWORD PTR [rsp+0x400]
sub    rdx,rcx
setb   r14b
cmp    r15,r8
mov    r15,QWORD PTR [rsp+0x300]
setb   cl
or     cl,r14b
cmp    QWORD PTR [rsp+0x4c0],r8
setb   r14b
or     r14b,BYTE PTR [rsp+0xb00]
cmp    rax,r8
mov    r8,QWORD PTR [rsp+0x480]
setb   r11b
or     r11b,BYTE PTR [rsp+0xac0]
mov    r11,QWORD PTR [rsp+0x180]
lea    rbx,[r8+r10*1]
cmovne r15,QWORD PTR [rsp+0x440]
cmovne r11,QWORD PTR [rsp+0x380]
cmovne rbx,r8
mov    r8,QWORD PTR [rsp+0x200]
mov    QWORD PTR [rsp+0xb40],rbx
mov    QWORD PTR [rsp+0xac0],rbx
lea    rbx,[rsp+0x40]
mov    QWORD PTR [rsp+0xb48],r15
mov    QWORD PTR [rsp+0x300],r15
mov    QWORD PTR [rsp+0x180],r11
mov    QWORD PTR [rsp+0xb50],r11
lea    r15,[rsp+0xc00]
cmovne r8,rax
mov    rax,QWORD PTR [rsp+0x840]
test   r14b,r14b
mov    r14,QWORD PTR [rsp+0x2c0]
cmovne r14,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rsp+0x200],r8
mov    QWORD PTR [rsp+0xb58],r8
lea    r10,[rax+r10*1]
cmovne r10,rax
mov    rax,QWORD PTR [rsp+0x240]
mov    QWORD PTR [rsp+0x2c0],r14
mov    QWORD PTR [rsp+0x380],r10
mov    r10,QWORD PTR [rsp+0x1c0]
cmovne rax,QWORD PTR [rsp+0x340]
cmovne r10,QWORD PTR [rsp+0x4c0]
test   cl,cl
movabs rcx,0xc3df73e9278302b9
cmovne rdi,r13
mov    r13,QWORD PTR [rsp+0x380]
cmovne rsi,QWORD PTR [rsp+0x580]
cmovne rdx,QWORD PTR [rsp+0x280]
lea    rcx,[r9+rcx*1]
cmovne rcx,r9
mov    QWORD PTR [rsp+r12*1+0x5c0],rcx
mov    QWORD PTR [rsp+r12*1+0x5c8],rdi
mov    QWORD PTR [rsp+0x240],rax
mov    QWORD PTR [rsp+0x1c0],r10
mov    QWORD PTR [rsp+r12*1+0x5d0],rsi
mov    QWORD PTR [rsp+r12*1+0x5d8],rdx
mov    QWORD PTR [rsp+0x40],r13
mov    QWORD PTR [rsp+0x48],rax
mov    QWORD PTR [rsp+0x50],r14
mov    QWORD PTR [rsp+0x58],r10
lea    rax,[rsp+0xb40]
movabs r12,0x87d20782e4866389
xor    r10d,r10d
xor    r14d,r14d
xor    r11d,r11d
xor    ecx,ecx
xor    r8d,r8d
mov    rdx,QWORD PTR [rbx]
xor    r9d,r9d
mulx   rdi,rsi,QWORD PTR [rax]
adcx   r10,rsi
adox   r14,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x8]
adcx   r14,rsi
adox   r11,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x10]
adcx   r11,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x18]
adcx   rcx,rsi
adox   r8,rdi
adcx   r8,r9
mov    rdx,r10
imul   rdx,r12
xor    r9d,r9d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   r10,rsi
adox   r14,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   r14,rsi
adox   r11,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   r11,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   rcx,rsi
adox   r8,rdi
adcx   r8,r9
mov    rdx,QWORD PTR [rbx+0x8]
xor    r9d,r9d
mulx   rdi,rsi,QWORD PTR [rax]
adcx   r14,rsi
adox   r11,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x8]
adcx   r11,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x10]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x18]
adcx   r8,rsi
adox   r10,rdi
adcx   r10,r9
mov    rdx,r14
imul   rdx,r12
xor    r9d,r9d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   r14,rsi
adox   r11,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   r11,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   r8,rsi
adox   r10,rdi
adcx   r10,r9
mov    rdx,QWORD PTR [rbx+0x10]
xor    r9d,r9d
mulx   rdi,rsi,QWORD PTR [rax]
adcx   r11,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x8]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x10]
adcx   r8,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x18]
adcx   r10,rsi
adox   r14,rdi
adcx   r14,r9
mov    rdx,r11
imul   rdx,r12
xor    r9d,r9d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   r11,rsi
adox   rcx,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   r8,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   r10,rsi
adox   r14,rdi
adcx   r14,r9
mov    rdx,QWORD PTR [rbx+0x18]
xor    r9d,r9d
mulx   rdi,rsi,QWORD PTR [rax]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x8]
adcx   r8,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x10]
adcx   r10,rsi
adox   r14,rdi
mulx   rdi,rsi,QWORD PTR [rax+0x18]
adcx   r14,rsi
adox   r11,rdi
adcx   r11,r9
mov    rdx,rcx
imul   rdx,r12
xor    r9d,r9d
mulx   rdi,rsi,QWORD PTR [r15]
adcx   rcx,rsi
adox   r8,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x8]
adcx   r8,rsi
adox   r10,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x10]
adcx   r10,rsi
adox   r14,rdi
mulx   rdi,rsi,QWORD PTR [r15+0x18]
adcx   r14,rsi
adox   r11,rdi
adcx   r11,r9
movabs rax,0x3c208c16d87cfd47
mov    QWORD PTR [rsp+0x440],r14
mov    QWORD PTR [rsp+0x40],r13
mov    QWORD PTR [rsp+0x340],r8
mov    QWORD PTR [rsp+0x400],r11
mov    QWORD PTR [rsp+0x840],r10
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
lea    r15,[rsp+0xc00]
lea    r12,[rsp+0x40]
cmp    r8,rax
movabs rax,0x687e956e978e3573
mov    r14,QWORD PTR [rsp+0x440]
mov    r13,QWORD PTR [rsp+0x2c0]
mov    r8,QWORD PTR [rsp+0xac0]
mov    rdi,QWORD PTR [rsp+0x300]
mov    r9,QWORD PTR [rsp+0x180]
lea    rax,[r10+rax*1]
sbb    rax,0x0
mov    QWORD PTR [rsp+0x4c0],rax
movabs rax,0x97816a916871ca8d
setb   cl
cmp    r10,rax
movabs rax,0x47afba497e7ea7a3
setb   dl
add    rax,r14
mov    rsi,r8
or     dl,cl
movzx  ecx,dl
sub    rax,rcx
mov    QWORD PTR [rsp+0x280],rax
movabs rax,0xb85045b68181585d
setb   cl
cmp    r14,rax
movabs rax,0xcf9bb18d1ece5fd7
setb   dl
add    rax,r11
or     dl,cl
movzx  ecx,dl
mov    rdx,QWORD PTR [rsp+0x200]
sub    rax,rcx
mov    rcx,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0x480],rax
mov    rax,QWORD PTR [rsp+0x240]
setb   BYTE PTR [rsp+0x3c0]
mov    QWORD PTR [rsp+0x48],rax
mov    QWORD PTR [rsp+0x50],r13
mov    QWORD PTR [rsp+0x58],rcx
mov    rcx,rdi
add    rsi,QWORD PTR [r12]
adc    rcx,QWORD PTR [r12+0x8]
adc    r9,QWORD PTR [r12+0x10]
adc    rdx,QWORD PTR [r12+0x18]
mov    r10,rsi
mov    r11,rcx
mov    rbx,r9
mov    r14,rdx
sub    r10,QWORD PTR [r15]
sbb    r11,QWORD PTR [r15+0x8]
sbb    rbx,QWORD PTR [r15+0x10]
sbb    r14,QWORD PTR [r15+0x18]
cmovae rsi,r10
cmovae rcx,r11
cmovae r9,rbx
cmovae rdx,r14
mov    QWORD PTR [rsp+0x580],rdx
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
mov    rbx,r8
lea    r12,[rsp+0xc00]
lea    rdx,[rsp+0x40]
mov    r10,QWORD PTR [rsp+0x380]
mov    r11,QWORD PTR [rsp+0x200]
mov    r14,QWORD PTR [rsp+0x180]
mov    QWORD PTR [rsp+0x40],r10
mov    QWORD PTR [rsp+0x48],rax
mov    rax,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0x50],r13
mov    r10,rdi
mov    QWORD PTR [rsp+0x58],rax
sub    rbx,QWORD PTR [rdx]
sbb    r10,QWORD PTR [rdx+0x8]
sbb    r14,QWORD PTR [rdx+0x10]
sbb    r11,QWORD PTR [rdx+0x18]
sbb    r8,r8
mov    r15,QWORD PTR [r12]
mov    rdi,QWORD PTR [r12+0x8]
mov    rax,QWORD PTR [r12+0x10]
mov    r13,QWORD PTR [r12+0x18]
and    r15,r8
and    rdi,r8
and    rax,r8
and    r13,r8
add    rbx,r15
adc    r10,rdi
adc    r14,rax
adc    r11,r13
mov    QWORD PTR [rsp+0x40],rbx
mov    QWORD PTR [rsp+0xb40],rsi
mov    QWORD PTR [rsp+0xb48],rcx
mov    QWORD PTR [rsp+0x48],r10
mov    QWORD PTR [rsp+0x50],r14
mov    QWORD PTR [rsp+0xb50],r9
mov    QWORD PTR [rsp+0x58],r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
lea    r12,[rsp+0x40]
lea    rbx,[rsp+0xb40]
movabs r14,0x87d20782e4866389
lea    r15,[rsp+0xc00]
mov    rax,QWORD PTR [rsp+0x580]
mov    QWORD PTR [rsp+0xb58],rax
xor    r9d,r9d
xor    r10d,r10d
xor    r11d,r11d
xor    eax,eax
xor    esi,esi
mov    rdx,QWORD PTR [r12]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [rbx]
adcx   r9,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x8]
adcx   r10,rcx
adox   r11,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x10]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x18]
adcx   rax,rcx
adox   rsi,rdi
adcx   rsi,r8
mov    rdx,r9
imul   rdx,r14
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r15]
adcx   r9,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x8]
adcx   r10,rcx
adox   r11,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x10]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x18]
adcx   rax,rcx
adox   rsi,rdi
adcx   rsi,r8
mov    rdx,QWORD PTR [r12+0x8]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [rbx]
adcx   r10,rcx
adox   r11,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x8]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x10]
adcx   rax,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x18]
adcx   rsi,rcx
adox   r9,rdi
adcx   r9,r8
mov    rdx,r10
imul   rdx,r14
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r15]
adcx   r10,rcx
adox   r11,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x8]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x10]
adcx   rax,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x18]
adcx   rsi,rcx
adox   r9,rdi
adcx   r9,r8
mov    rdx,QWORD PTR [r12+0x10]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [rbx]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x8]
adcx   rax,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x10]
adcx   rsi,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x18]
adcx   r9,rcx
adox   r10,rdi
adcx   r10,r8
mov    rdx,r11
imul   rdx,r14
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r15]
adcx   r11,rcx
adox   rax,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x8]
adcx   rax,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x10]
adcx   rsi,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x18]
adcx   r9,rcx
adox   r10,rdi
adcx   r10,r8
mov    rdx,QWORD PTR [r12+0x18]
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [rbx]
adcx   rax,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x8]
adcx   rsi,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x10]
adcx   r9,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [rbx+0x18]
adcx   r10,rcx
adox   r11,rdi
adcx   r11,r8
mov    rdx,rax
imul   rdx,r14
xor    r8d,r8d
mulx   rdi,rcx,QWORD PTR [r15]
adcx   rax,rcx
adox   rsi,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x8]
adcx   rsi,rcx
adox   r9,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x10]
adcx   r9,rcx
adox   r10,rdi
mulx   rdi,rcx,QWORD PTR [r15+0x18]
adcx   r10,rcx
adox   r11,rdi
adcx   r11,r8
movabs rax,0x3c208c16d87cfd47
movabs rdx,0x30644e72e131a029
mov    r14,r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
cmp    rsi,rax
movabs rax,0x687e956e978e3573
mov    r12,QWORD PTR [rsp+0x400]
mov    rdi,QWORD PTR [rsp+0x340]
mov    rbx,QWORD PTR [rsp+0x480]
lea    rcx,[r9+rax*1]
sbb    rcx,0x0
mov    r15,rcx
movabs rcx,0x97816a916871ca8d
setb   al
cmp    r9,rcx
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0x47afba497e7ea7a3
lea    r8,[r10+rcx*1]
movabs rcx,0xb85045b68181585d
sub    r8,rax
setb   al
cmp    r10,rcx
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0xcf9bb18d1ece5fd7
add    rcx,r11
sub    rcx,rax
setb   al
cmp    r11,rdx
mov    r13,rcx
setb   cl
or     cl,al
cmp    r12,rdx
movabs rdx,0xc3df73e9278302b9
setb   al
or     al,BYTE PTR [rsp+0x3c0]
lea    rax,[rdi+rdx*1]
cmovne rax,rdi
mov    rdi,QWORD PTR [rsp+0x280]
cmovne rbx,r12
mov    r11,rax
mov    rax,QWORD PTR [rsp+0x4c0]
cmovne rdi,QWORD PTR [rsp+0x440]
mov    QWORD PTR [rsp+0x40],r11
cmovne rax,QWORD PTR [rsp+0x840]
test   cl,cl
lea    rcx,[rsi+rdx*1]
cmovne rcx,rsi
cmovne r15,r9
cmovne r8,r10
cmovne r13,r14
lea    r14,[rsp+0x40]
lea    r9,[rsp+0xc00]
mov    r12,rcx
mov    QWORD PTR [rsp+0x48],rax
mov    QWORD PTR [rsp+0x50],rdi
mov    QWORD PTR [rsp+0x58],rbx
mov    r10,rax
add    r11,QWORD PTR [r14]
adc    r10,QWORD PTR [r14+0x8]
adc    rdi,QWORD PTR [r14+0x10]
adc    rbx,QWORD PTR [r14+0x18]
mov    rax,r11
mov    rcx,r10
mov    rdx,rdi
mov    rsi,rbx
sub    rax,QWORD PTR [r9]
sbb    rcx,QWORD PTR [r9+0x8]
sbb    rdx,QWORD PTR [r9+0x10]
sbb    rsi,QWORD PTR [r9+0x18]
cmovae r11,rax
cmovae r10,rcx
cmovae rdi,rdx
cmovae rbx,rsi
mov    QWORD PTR [rsp+0x440],r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
mov    r9,rdi
lea    r14,[rsp+0xc00]
lea    r11,[rsp+0x40]
vmovaps ymm1,YMMWORD PTR [rsp+0x12c0]
vmovaps YMMWORD PTR [rsp+0x40],ymm1
sub    r12,QWORD PTR [r11]
sbb    r15,QWORD PTR [r11+0x8]
sbb    r8,QWORD PTR [r11+0x10]
sbb    r13,QWORD PTR [r11+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r14]
mov    rcx,QWORD PTR [r14+0x8]
mov    rdx,QWORD PTR [r14+0x10]
mov    rsi,QWORD PTR [r14+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    r12,rax
adc    r15,rcx
adc    r8,rdx
adc    r13,rsi
lea    rax,[rsp+0x12e0]
mov    QWORD PTR [rsp+0x340],r13
mov    QWORD PTR [rsp+0x840],r8
mov    QWORD PTR [rsp+0x3c0],r12
mov    QWORD PTR [rsp+0x400],r15
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
mov    r13,r12
mov    r11,r10
lea    r10,[rsp+0xc00]
lea    r12,[rsp+0x40]
vmovups ymm1,YMMWORD PTR [rax]
mov    r14,QWORD PTR [rsp+0x440]
vmovaps YMMWORD PTR [rsp+0x40],ymm1
sub    r14,QWORD PTR [r12]
sbb    r11,QWORD PTR [r12+0x8]
sbb    r9,QWORD PTR [r12+0x10]
sbb    rbx,QWORD PTR [r12+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r10]
mov    rcx,QWORD PTR [r10+0x8]
mov    rdx,QWORD PTR [r10+0x10]
mov    rsi,QWORD PTR [r10+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    r14,rax
adc    r11,rcx
adc    r9,rdx
adc    rbx,rsi
mov    QWORD PTR [rsp+0x40],r13
mov    QWORD PTR [rsp+0x48],r15
mov    QWORD PTR [rsp+0x280],r9
mov    QWORD PTR [rsp+0x50],r8
mov    QWORD PTR [rsp+0x4c0],r11
mov    QWORD PTR [rsp+0x440],r14
mov    QWORD PTR [rsp+0x480],rbx
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
lea    r13,[rsp+0xc00]
lea    r15,[rsp+0x40]
mov    rax,QWORD PTR [rsp+0x340]
mov    r10,QWORD PTR [rsp+0xa00]
mov    r8,QWORD PTR [rsp+0x940]
mov    r9,QWORD PTR [rsp+0x9c0]
mov    r12,QWORD PTR [rsp+0x980]
mov    QWORD PTR [rsp+0x58],rax
sub    r10,QWORD PTR [r15]
sbb    r12,QWORD PTR [r15+0x8]
sbb    r8,QWORD PTR [r15+0x10]
sbb    r9,QWORD PTR [r15+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r13+0x0]
mov    rcx,QWORD PTR [r13+0x8]
mov    rdx,QWORD PTR [r13+0x10]
mov    rsi,QWORD PTR [r13+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    r10,rax
adc    r12,rcx
adc    r8,rdx
adc    r9,rsi
mov    QWORD PTR [rsp+0x40],r14
mov    QWORD PTR [rsp+0x980],r12
mov    QWORD PTR [rsp+0x9c0],r9
mov    QWORD PTR [rsp+0x940],r8
mov    QWORD PTR [rsp+0xa00],r10
mov    QWORD PTR [rsp+0x48],r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
lea    r13,[rsp+0xc00]
lea    r12,[rsp+0x40]
mov    r15,r10
mov    rax,QWORD PTR [rsp+0x280]
mov    r14,QWORD PTR [rsp+0xa40]
mov    r8,QWORD PTR [rsp+0x900]
mov    r9,QWORD PTR [rsp+0x880]
mov    QWORD PTR [rsp+0x50],rax
mov    QWORD PTR [rsp+0x58],rbx
mov    rbx,QWORD PTR [rsp+0xa80]
sub    rbx,QWORD PTR [r12]
sbb    r8,QWORD PTR [r12+0x8]
sbb    r14,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r13+0x0]
mov    rcx,QWORD PTR [r13+0x8]
mov    rdx,QWORD PTR [r13+0x10]
mov    rsi,QWORD PTR [r13+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    rbx,rax
adc    r8,rcx
adc    r14,rdx
adc    r9,rsi
mov    QWORD PTR [rsp+0xa40],r14
mov    QWORD PTR [rsp+0xa80],rbx
mov    QWORD PTR [rsp+0x880],r9
mov    QWORD PTR [rsp+0x900],r8
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
xor    ecx,ecx
xor    edx,edx
xor    esi,esi
mov    rax,QWORD PTR [rsp+0x380]
mov    QWORD PTR [rsp+0x40],rax
mov    rax,QWORD PTR [rsp+0x240]
mov    QWORD PTR [rsp+0x48],rax
mov    rax,QWORD PTR [rsp+0x2c0]
mov    QWORD PTR [rsp+0x50],rax
mov    rax,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0x58],rax
xor    eax,eax
sub    rcx,QWORD PTR [r12]
sbb    rdx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    rax,QWORD PTR [r12+0x18]
sbb    r11,r11
mov    rdi,QWORD PTR [r13+0x0]
mov    r8,QWORD PTR [r13+0x8]
mov    r9,QWORD PTR [r13+0x10]
mov    r10,QWORD PTR [r13+0x18]
and    rdi,r11
and    r8,r11
and    r9,r11
and    r10,r11
add    rcx,rdi
adc    rdx,r8
adc    rsi,r9
adc    rax,r10
lea    r9,[rsp+0x80]
mov    QWORD PTR [rsp+0xc00],r15
lea    r10,[rsp+0xc48]
lea    r12,[rsp+0xc00]
lea    r11,[rsp+0x40]
lea    r13,[rsp+0xc00]
mov    rdi,QWORD PTR [rsp+0xac0]
mov    r8,QWORD PTR [rsp+0x980]
mov    QWORD PTR [rsp+0x40],rdi
mov    rdi,QWORD PTR [rsp+0x300]
mov    QWORD PTR [rsp+0xc08],r8
mov    r8,QWORD PTR [rsp+0x9c0]
mov    QWORD PTR [rsp+0x48],rdi
mov    rdi,QWORD PTR [rsp+0x180]
mov    QWORD PTR [rsp+0x50],rdi
mov    rdi,QWORD PTR [rsp+0x200]
mov    QWORD PTR [rsp+0x58],rdi
mov    QWORD PTR [rsp+0x60],rcx
mov    QWORD PTR [rsp+0x68],rdx
mov    QWORD PTR [rsp+0x70],rsi
mov    QWORD PTR [rsp+0x78],rax
vmovups YMMWORD PTR [r9],ymm2
mov    r9,QWORD PTR [rsp+0x940]
mov    QWORD PTR [rsp+0xc10],r9
mov    QWORD PTR [rsp+0xc18],r8
mov    QWORD PTR [rsp+0xc20],rbx
mov    rbx,QWORD PTR [rsp+0x900]
movabs r8,0x87d20782e4866389
mov    QWORD PTR [rsp+0xc28],rbx
mov    QWORD PTR [rsp+0xc30],r14
mov    r14,QWORD PTR [rsp+0x880]
mov    QWORD PTR [rsp+0xc38],r14
mov    QWORD PTR [rsp+0xc40],r8
vmovdqu XMMWORD PTR [r10],xmm0
mov    QWORD PTR [r10+0x10],0x0
xor    r10d,r10d
xor    r8d,r8d
xor    r15d,r15d
xor    eax,eax
xor    r9d,r9d
mov    rdx,QWORD PTR [r12]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11]
adcx   r10,rcx
adox   r8,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x8]
adcx   r8,rcx
adox   r15,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x10]
adcx   r15,rcx
adox   rax,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x18]
adcx   rax,rcx
adox   r9,rsi
adcx   r9,rdi
mov    rdx,QWORD PTR [r12+0x20]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11+0x20]
adcx   r10,rcx
adox   r8,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x28]
adcx   r8,rcx
adox   r15,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x30]
adcx   r15,rcx
adox   rax,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x38]
adcx   rax,rcx
adox   r9,rsi
adcx   r9,rdi
mov    rdx,r10
imul   rdx,QWORD PTR [r12+0x40]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11+0x40]
adcx   r10,rcx
adox   r8,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x48]
adcx   r8,rcx
adox   r15,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x50]
adcx   r15,rcx
adox   rax,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x58]
adcx   rax,rcx
adox   r9,rsi
adcx   r9,rdi
mov    rdx,QWORD PTR [r12+0x8]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11]
adcx   r8,rcx
adox   r15,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x8]
adcx   r15,rcx
adox   rax,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x10]
adcx   rax,rcx
adox   r9,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x18]
adcx   r9,rcx
adox   r10,rsi
adcx   r10,rdi
mov    rdx,QWORD PTR [r12+0x28]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11+0x20]
adcx   r8,rcx
adox   r15,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x28]
adcx   r15,rcx
adox   rax,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x30]
adcx   rax,rcx
adox   r9,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x38]
adcx   r9,rcx
adox   r10,rsi
adcx   r10,rdi
mov    rdx,r8
imul   rdx,QWORD PTR [r12+0x40]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11+0x40]
adcx   r8,rcx
adox   r15,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x48]
adcx   r15,rcx
adox   rax,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x50]
adcx   rax,rcx
adox   r9,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x58]
adcx   r9,rcx
adox   r10,rsi
adcx   r10,rdi
mov    rdx,QWORD PTR [r12+0x10]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11]
adcx   r15,rcx
adox   rax,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x8]
adcx   rax,rcx
adox   r9,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x10]
adcx   r9,rcx
adox   r10,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x18]
adcx   r10,rcx
adox   r8,rsi
adcx   r8,rdi
mov    rdx,QWORD PTR [r12+0x30]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11+0x20]
adcx   r15,rcx
adox   rax,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x28]
adcx   rax,rcx
adox   r9,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x30]
adcx   r9,rcx
adox   r10,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x38]
adcx   r10,rcx
adox   r8,rsi
adcx   r8,rdi
mov    rdx,r15
imul   rdx,QWORD PTR [r12+0x40]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11+0x40]
adcx   r15,rcx
adox   rax,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x48]
adcx   rax,rcx
adox   r9,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x50]
adcx   r9,rcx
adox   r10,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x58]
adcx   r10,rcx
adox   r8,rsi
adcx   r8,rdi
mov    rdx,QWORD PTR [r12+0x18]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11]
adcx   rax,rcx
adox   r9,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x8]
adcx   r9,rcx
adox   r10,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x10]
adcx   r10,rcx
adox   r8,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x18]
adcx   r8,rcx
adox   r15,rsi
adcx   r15,rdi
mov    rdx,QWORD PTR [r12+0x38]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11+0x20]
adcx   rax,rcx
adox   r9,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x28]
adcx   r9,rcx
adox   r10,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x30]
adcx   r10,rcx
adox   r8,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x38]
adcx   r8,rcx
adox   r15,rsi
adcx   r15,rdi
mov    rdx,rax
imul   rdx,QWORD PTR [r12+0x40]
xor    edi,edi
mulx   rsi,rcx,QWORD PTR [r11+0x40]
adcx   rax,rcx
adox   r9,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x48]
adcx   r9,rcx
adox   r10,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x50]
adcx   r10,rcx
adox   r8,rsi
mulx   rsi,rcx,QWORD PTR [r11+0x58]
adcx   r8,rcx
adox   r15,rsi
adcx   r15,rdi
movabs rdi,0x3c208c16d87cfd47
mov    QWORD PTR [rsp+0x580],r15
mov    QWORD PTR [rsp+0xbc0],r9
mov    QWORD PTR [rsp+0xb00],r10
mov    QWORD PTR [rsp+0xf00],r8
lea    r11,[rsp+0x40]
mov    rax,QWORD PTR [rsp+0xac0]
mov    rsi,QWORD PTR [rsp+0x300]
mov    rcx,QWORD PTR [rsp+0x180]
cmp    r9,rdi
mov    QWORD PTR [rsp+0x40],rax
mov    QWORD PTR [rsp+0x48],rsi
mov    QWORD PTR [rsp+0x50],rcx
mov    rsi,QWORD PTR [rsp+0x200]
mov    rcx,QWORD PTR [rsp+0xa80]
movabs rax,0x687e956e978e3573
lea    rax,[r10+rax*1]
sbb    rax,0x0
setb   dl
mov    QWORD PTR [rsp+0xc00],rcx
mov    QWORD PTR [rsp+0x58],rsi
mov    rsi,QWORD PTR [rsp+0xa40]
mov    rcx,QWORD PTR [rsp+0xa00]
mov    QWORD PTR [rsp+0xc08],rbx
lea    rbx,[rsp+0xc00]
mov    QWORD PTR [rsp+0xc10],rsi
mov    QWORD PTR [rsp+0xc18],r14
mov    QWORD PTR [rsp+0xc20],rcx
mov    rsi,QWORD PTR [rsp+0x980]
mov    rcx,QWORD PTR [rsp+0x940]
mov    r14,rax
mov    QWORD PTR [rsp+0xc28],rsi
mov    QWORD PTR [rsp+0xc30],rcx
movabs rcx,0x97816a916871ca8d
mov    rsi,QWORD PTR [rsp+0x9c0]
cmp    r10,rcx
setb   cl
or     cl,dl
mov    rdx,QWORD PTR [rsp+0x580]
movzx  eax,cl
movabs rcx,0x47afba497e7ea7a3
add    rcx,r8
mov    QWORD PTR [rsp+0xc38],rsi
mov    rsi,QWORD PTR [rsp+0x240]
sub    rcx,rax
mov    QWORD PTR [rsp+0x180],rcx
movabs rcx,0xb85045b68181585d
setb   al
cmp    r8,rcx
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0xcf9bb18d1ece5fd7
add    rcx,rdx
sub    rcx,rax
mov    r15,rcx
mov    rcx,QWORD PTR [rsp+0x380]
mov    QWORD PTR [rsp+0x60],rcx
mov    rcx,QWORD PTR [rsp+0x2c0]
mov    QWORD PTR [rsp+0x68],rsi
mov    rsi,QWORD PTR [rsp+0x1c0]
setb   BYTE PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0x70],rcx
lea    rcx,[rsp+0x80]
mov    QWORD PTR [rsp+0x78],rsi
movabs rsi,0x87d20782e4866389
vmovups YMMWORD PTR [rcx],ymm2
lea    rcx,[rsp+0xc48]
mov    QWORD PTR [rsp+0xc40],rsi
vmovdqu XMMWORD PTR [rcx],xmm0
mov    QWORD PTR [rcx+0x10],0x0
xor    esi,esi
xor    edi,edi
xor    eax,eax
xor    r8d,r8d
xor    ecx,ecx
mov    rdx,QWORD PTR [rbx]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11]
adcx   rsi,r9
adox   rdi,r10
mulx   r10,r9,QWORD PTR [r11+0x8]
adcx   rdi,r9
adox   rax,r10
mulx   r10,r9,QWORD PTR [r11+0x10]
adcx   rax,r9
adox   r8,r10
mulx   r10,r9,QWORD PTR [r11+0x18]
adcx   r8,r9
adox   rcx,r10
adcx   rcx,r12
mov    rdx,QWORD PTR [rbx+0x20]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11+0x20]
adcx   rsi,r9
adox   rdi,r10
mulx   r10,r9,QWORD PTR [r11+0x28]
adcx   rdi,r9
adox   rax,r10
mulx   r10,r9,QWORD PTR [r11+0x30]
adcx   rax,r9
adox   r8,r10
mulx   r10,r9,QWORD PTR [r11+0x38]
adcx   r8,r9
adox   rcx,r10
adcx   rcx,r12
mov    rdx,rsi
imul   rdx,QWORD PTR [rbx+0x40]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11+0x40]
adcx   rsi,r9
adox   rdi,r10
mulx   r10,r9,QWORD PTR [r11+0x48]
adcx   rdi,r9
adox   rax,r10
mulx   r10,r9,QWORD PTR [r11+0x50]
adcx   rax,r9
adox   r8,r10
mulx   r10,r9,QWORD PTR [r11+0x58]
adcx   r8,r9
adox   rcx,r10
adcx   rcx,r12
mov    rdx,QWORD PTR [rbx+0x8]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11]
adcx   rdi,r9
adox   rax,r10
mulx   r10,r9,QWORD PTR [r11+0x8]
adcx   rax,r9
adox   r8,r10
mulx   r10,r9,QWORD PTR [r11+0x10]
adcx   r8,r9
adox   rcx,r10
mulx   r10,r9,QWORD PTR [r11+0x18]
adcx   rcx,r9
adox   rsi,r10
adcx   rsi,r12
mov    rdx,QWORD PTR [rbx+0x28]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11+0x20]
adcx   rdi,r9
adox   rax,r10
mulx   r10,r9,QWORD PTR [r11+0x28]
adcx   rax,r9
adox   r8,r10
mulx   r10,r9,QWORD PTR [r11+0x30]
adcx   r8,r9
adox   rcx,r10
mulx   r10,r9,QWORD PTR [r11+0x38]
adcx   rcx,r9
adox   rsi,r10
adcx   rsi,r12
mov    rdx,rdi
imul   rdx,QWORD PTR [rbx+0x40]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11+0x40]
adcx   rdi,r9
adox   rax,r10
mulx   r10,r9,QWORD PTR [r11+0x48]
adcx   rax,r9
adox   r8,r10
mulx   r10,r9,QWORD PTR [r11+0x50]
adcx   r8,r9
adox   rcx,r10
mulx   r10,r9,QWORD PTR [r11+0x58]
adcx   rcx,r9
adox   rsi,r10
adcx   rsi,r12
mov    rdx,QWORD PTR [rbx+0x10]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11]
adcx   rax,r9
adox   r8,r10
mulx   r10,r9,QWORD PTR [r11+0x8]
adcx   r8,r9
adox   rcx,r10
mulx   r10,r9,QWORD PTR [r11+0x10]
adcx   rcx,r9
adox   rsi,r10
mulx   r10,r9,QWORD PTR [r11+0x18]
adcx   rsi,r9
adox   rdi,r10
adcx   rdi,r12
mov    rdx,QWORD PTR [rbx+0x30]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11+0x20]
adcx   rax,r9
adox   r8,r10
mulx   r10,r9,QWORD PTR [r11+0x28]
adcx   r8,r9
adox   rcx,r10
mulx   r10,r9,QWORD PTR [r11+0x30]
adcx   rcx,r9
adox   rsi,r10
mulx   r10,r9,QWORD PTR [r11+0x38]
adcx   rsi,r9
adox   rdi,r10
adcx   rdi,r12
mov    rdx,rax
imul   rdx,QWORD PTR [rbx+0x40]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11+0x40]
adcx   rax,r9
adox   r8,r10
mulx   r10,r9,QWORD PTR [r11+0x48]
adcx   r8,r9
adox   rcx,r10
mulx   r10,r9,QWORD PTR [r11+0x50]
adcx   rcx,r9
adox   rsi,r10
mulx   r10,r9,QWORD PTR [r11+0x58]
adcx   rsi,r9
adox   rdi,r10
adcx   rdi,r12
mov    rdx,QWORD PTR [rbx+0x18]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11]
adcx   r8,r9
adox   rcx,r10
mulx   r10,r9,QWORD PTR [r11+0x8]
adcx   rcx,r9
adox   rsi,r10
mulx   r10,r9,QWORD PTR [r11+0x10]
adcx   rsi,r9
adox   rdi,r10
mulx   r10,r9,QWORD PTR [r11+0x18]
adcx   rdi,r9
adox   rax,r10
adcx   rax,r12
mov    rdx,QWORD PTR [rbx+0x38]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11+0x20]
adcx   r8,r9
adox   rcx,r10
mulx   r10,r9,QWORD PTR [r11+0x28]
adcx   rcx,r9
adox   rsi,r10
mulx   r10,r9,QWORD PTR [r11+0x30]
adcx   rsi,r9
adox   rdi,r10
mulx   r10,r9,QWORD PTR [r11+0x38]
adcx   rdi,r9
adox   rax,r10
adcx   rax,r12
mov    rdx,r8
imul   rdx,QWORD PTR [rbx+0x40]
xor    r12d,r12d
mulx   r10,r9,QWORD PTR [r11+0x40]
adcx   r8,r9
adox   rcx,r10
mulx   r10,r9,QWORD PTR [r11+0x48]
adcx   rcx,r9
adox   rsi,r10
mulx   r10,r9,QWORD PTR [r11+0x50]
adcx   rsi,r9
adox   rdi,r10
mulx   r10,r9,QWORD PTR [r11+0x58]
adcx   rdi,r9
adox   rax,r10
adcx   rax,r12
movabs rdx,0x3c208c16d87cfd47
mov    QWORD PTR [rsp+0x2c0],rdi
mov    QWORD PTR [rsp+0x300],rax
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
cmp    rcx,rdx
movabs rdx,0x687e956e978e3573
vmovaps ymm1,YMMWORD PTR [rsp+0x11c0]
lea    rdx,[rsi+rdx*1]
sbb    rdx,0x0
mov    r11,rdx
movabs rdx,0x97816a916871ca8d
setb   r8b
cmp    rsi,rdx
movabs rdx,0x47afba497e7ea7a3
setb   r9b
lea    r12,[rdi+rdx*1]
movabs rdx,0xb85045b68181585d
or     r9b,r8b
movabs r8,0x30644e72e131a029
movzx  r9d,r9b
sub    r12,r9
vmovaps YMMWORD PTR [rsp+0x40],ymm1
setb   r9b
cmp    rdi,rdx
movabs rdx,0xcf9bb18d1ece5fd7
setb   r10b
lea    rdi,[rax+rdx*1]
mov    rdx,QWORD PTR [rsp+0xbc0]
or     r10b,r9b
movzx  r9d,r10b
sub    rdi,r9
setb   r9b
cmp    rax,r8
mov    rax,QWORD PTR [rsp+0x580]
setb   r10b
or     r10b,r9b
cmp    rax,r8
movabs r8,0xc3df73e9278302b9
setb   r9b
or     r9b,BYTE PTR [rsp+0x1c0]
lea    rbx,[rdx+r8*1]
lea    r8,[rcx+r8*1]
cmovne r14,QWORD PTR [rsp+0xb00]
cmovne rbx,rdx
mov    rdx,QWORD PTR [rsp+0x180]
cmovne r15,rax
cmovne rdx,QWORD PTR [rsp+0xf00]
test   r10b,r10b
mov    QWORD PTR [rsp+0x1c0],r15
lea    r15,[rsp+0x40]
cmovne r11,rsi
cmovne r8,rcx
cmovne r12,QWORD PTR [rsp+0x2c0]
cmovne rdi,QWORD PTR [rsp+0x300]
mov    QWORD PTR [rsp+0x240],r11
mov    QWORD PTR [rsp+0x200],r14
mov    r14,QWORD PTR [rsp+0x1c0]
mov    r11,QWORD PTR [rsp+0x200]
sub    rbx,QWORD PTR [r15]
sbb    r11,QWORD PTR [r15+0x8]
sbb    rdx,QWORD PTR [r15+0x10]
sbb    r14,QWORD PTR [r15+0x18]
sbb    r10,r10
mov    rax,QWORD PTR [r13+0x0]
mov    rcx,QWORD PTR [r13+0x8]
mov    rsi,QWORD PTR [r13+0x10]
mov    r9,QWORD PTR [r13+0x18]
and    rax,r10
and    rcx,r10
and    rsi,r10
and    r9,r10
add    rbx,rax
adc    r11,rcx
adc    rdx,rsi
adc    r14,r9
lea    rax,[rsp+0x11e0]
mov    QWORD PTR [rsp+0x200],r11
mov    QWORD PTR [rsp+0x180],rdx
mov    QWORD PTR [rsp+0x1c0],r14
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
lea    rdx,[rsp+0xc00]
lea    r13,[rsp+0x40]
vmovdqu ymm1,YMMWORD PTR [rax]
mov    r11,QWORD PTR [rsp+0x240]
vmovdqa YMMWORD PTR [rsp+0x40],ymm1
sub    r8,QWORD PTR [r13+0x0]
sbb    r11,QWORD PTR [r13+0x8]
sbb    r12,QWORD PTR [r13+0x10]
sbb    rdi,QWORD PTR [r13+0x18]
sbb    r10,r10
mov    rax,QWORD PTR [rdx]
mov    rcx,QWORD PTR [rdx+0x8]
mov    rsi,QWORD PTR [rdx+0x10]
mov    r9,QWORD PTR [rdx+0x18]
and    rax,r10
and    rcx,r10
and    rsi,r10
and    r9,r10
add    r8,rax
adc    r11,rcx
adc    r12,rsi
adc    rdi,r9
mov    QWORD PTR [rsp+0x240],r11
mov    QWORD PTR [rsp+0x940],rdi
mov    QWORD PTR [rsp+0x300],r12
mov    r12,rdx
mov    QWORD PTR [rsp+0x2c0],r8
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
mov    r8,r13
mov    r10,QWORD PTR [rsp+0x3c0]
mov    rdx,QWORD PTR [rsp+0x400]
mov    rdi,QWORD PTR [rsp+0x840]
mov    r11,QWORD PTR [rsp+0x340]
mov    QWORD PTR [rsp+0x40],r10
mov    QWORD PTR [rsp+0x48],rdx
mov    QWORD PTR [rsp+0x50],rdi
mov    QWORD PTR [rsp+0x58],r11
add    r10,QWORD PTR [r13+0x0]
adc    rdx,QWORD PTR [r13+0x8]
adc    rdi,QWORD PTR [r13+0x10]
adc    r11,QWORD PTR [r13+0x18]
mov    rax,r10
mov    rcx,rdx
mov    rsi,rdi
mov    r9,r11
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae r10,rax
cmovae rdx,rcx
cmovae rdi,rsi
cmovae r11,r9
mov    QWORD PTR [rsp+0x40],r10
mov    QWORD PTR [rsp+0x48],rdx
mov    QWORD PTR [rsp+0x50],rdi
mov    QWORD PTR [rsp+0x58],r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    r10,QWORD PTR [r13+0x0]
adc    rdx,QWORD PTR [r13+0x8]
adc    rdi,QWORD PTR [r13+0x10]
adc    r11,QWORD PTR [r13+0x18]
mov    rax,r10
mov    rcx,rdx
mov    rsi,rdi
mov    r9,r11
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae r10,rax
cmovae rdx,rcx
cmovae rdi,rsi
cmovae r11,r9
mov    QWORD PTR [rsp+0x40],r10
mov    QWORD PTR [rsp+0x48],rdx
mov    QWORD PTR [rsp+0x50],rdi
mov    QWORD PTR [rsp+0x58],r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    r10,QWORD PTR [r13+0x0]
adc    rdx,QWORD PTR [r13+0x8]
adc    rdi,QWORD PTR [r13+0x10]
adc    r11,QWORD PTR [r13+0x18]
mov    rax,r10
mov    rcx,rdx
mov    rsi,rdi
mov    r9,r11
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae r10,rax
cmovae rdx,rcx
cmovae rdi,rsi
cmovae r11,r9
mov    QWORD PTR [rsp+0x40],r10
mov    QWORD PTR [rsp+0x48],rdx
mov    QWORD PTR [rsp+0x50],rdi
mov    QWORD PTR [rsp+0x58],r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    r10,QWORD PTR [r13+0x0]
adc    rdx,QWORD PTR [r13+0x8]
adc    rdi,QWORD PTR [r13+0x10]
adc    r11,QWORD PTR [r13+0x18]
mov    rax,r10
mov    rcx,rdx
mov    rsi,rdi
mov    r9,r11
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae r10,rax
cmovae rdx,rcx
cmovae rdi,rsi
cmovae r11,r9
mov    QWORD PTR [rsp+0x3c0],r10
mov    QWORD PTR [rsp+0x840],rdi
mov    QWORD PTR [rsp+0x340],r11
mov    QWORD PTR [rsp+0x400],rdx
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
mov    r13,QWORD PTR [rsp+0x440]
mov    rdi,QWORD PTR [rsp+0x4c0]
mov    r10,QWORD PTR [rsp+0x280]
mov    r11,QWORD PTR [rsp+0x480]
mov    QWORD PTR [rsp+0x40],r13
mov    QWORD PTR [rsp+0x48],rdi
mov    QWORD PTR [rsp+0x50],r10
mov    QWORD PTR [rsp+0x58],r11
add    r13,QWORD PTR [r8]
adc    rdi,QWORD PTR [r8+0x8]
adc    r10,QWORD PTR [r8+0x10]
adc    r11,QWORD PTR [r8+0x18]
mov    rax,r13
mov    rcx,rdi
mov    rsi,r10
mov    r9,r11
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae r13,rax
cmovae rdi,rcx
cmovae r10,rsi
cmovae r11,r9
mov    QWORD PTR [rsp+0x40],r13
mov    QWORD PTR [rsp+0x48],rdi
mov    QWORD PTR [rsp+0x50],r10
mov    QWORD PTR [rsp+0x58],r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    r13,QWORD PTR [r8]
adc    rdi,QWORD PTR [r8+0x8]
adc    r10,QWORD PTR [r8+0x10]
adc    r11,QWORD PTR [r8+0x18]
mov    rax,r13
mov    rcx,rdi
mov    rsi,r10
mov    r9,r11
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae r13,rax
cmovae rdi,rcx
cmovae r10,rsi
cmovae r11,r9
mov    QWORD PTR [rsp+0x40],r13
mov    QWORD PTR [rsp+0x48],rdi
mov    QWORD PTR [rsp+0x50],r10
mov    QWORD PTR [rsp+0x58],r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    r13,QWORD PTR [r8]
adc    rdi,QWORD PTR [r8+0x8]
adc    r10,QWORD PTR [r8+0x10]
adc    r11,QWORD PTR [r8+0x18]
mov    rax,r13
mov    rcx,rdi
mov    rsi,r10
mov    r9,r11
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae r13,rax
cmovae rdi,rcx
cmovae r10,rsi
cmovae r11,r9
mov    QWORD PTR [rsp+0x40],r13
mov    QWORD PTR [rsp+0x48],rdi
mov    QWORD PTR [rsp+0x50],r10
mov    QWORD PTR [rsp+0x58],r11
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    r13,QWORD PTR [r8]
adc    rdi,QWORD PTR [r8+0x8]
adc    r10,QWORD PTR [r8+0x10]
adc    r11,QWORD PTR [r8+0x18]
mov    rax,r13
mov    rcx,rdi
mov    rsi,r10
mov    r9,r11
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae r13,rax
cmovae rdi,rcx
cmovae r10,rsi
cmovae r11,r9
mov    QWORD PTR [rsp+0x480],r11
mov    QWORD PTR [rsp+0x280],r10
mov    QWORD PTR [rsp+0x4c0],rdi
mov    QWORD PTR [rsp+0x440],r13
mov    QWORD PTR [rsp+0x40],rbx
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
mov    rdi,r8
mov    r11,QWORD PTR [rsp+0x200]
mov    rdx,QWORD PTR [rsp+0x180]
mov    r10,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0x48],r11
mov    QWORD PTR [rsp+0x50],rdx
mov    QWORD PTR [rsp+0x58],r10
add    rbx,QWORD PTR [r8]
adc    r11,QWORD PTR [r8+0x8]
adc    rdx,QWORD PTR [r8+0x10]
adc    r10,QWORD PTR [r8+0x18]
mov    rax,rbx
mov    rcx,r11
mov    rsi,rdx
mov    r9,r10
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae rbx,rax
cmovae r11,rcx
cmovae rdx,rsi
cmovae r10,r9
mov    QWORD PTR [rsp+0x40],rbx
mov    QWORD PTR [rsp+0x48],r11
mov    QWORD PTR [rsp+0x50],rdx
mov    QWORD PTR [rsp+0x58],r10
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    rbx,QWORD PTR [r8]
adc    r11,QWORD PTR [r8+0x8]
adc    rdx,QWORD PTR [r8+0x10]
adc    r10,QWORD PTR [r8+0x18]
mov    rax,rbx
mov    rcx,r11
mov    rsi,rdx
mov    r9,r10
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae rbx,rax
cmovae r11,rcx
cmovae rdx,rsi
cmovae r10,r9
mov    QWORD PTR [rsp+0x40],rbx
mov    QWORD PTR [rsp+0x48],r11
mov    QWORD PTR [rsp+0x50],rdx
mov    QWORD PTR [rsp+0x58],r10
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    rbx,QWORD PTR [r8]
adc    r11,QWORD PTR [r8+0x8]
adc    rdx,QWORD PTR [r8+0x10]
adc    r10,QWORD PTR [r8+0x18]
mov    rax,rbx
mov    rcx,r11
mov    rsi,rdx
mov    r9,r10
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae rbx,rax
cmovae r11,rcx
cmovae rdx,rsi
cmovae r10,r9
mov    QWORD PTR [rsp+0x40],rbx
mov    QWORD PTR [rsp+0x48],r11
mov    QWORD PTR [rsp+0x50],rdx
mov    QWORD PTR [rsp+0x58],r10
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
mov    r8,r10
add    rbx,QWORD PTR [rdi]
adc    r11,QWORD PTR [rdi+0x8]
adc    rdx,QWORD PTR [rdi+0x10]
adc    r8,QWORD PTR [rdi+0x18]
mov    rax,rbx
mov    rcx,r11
mov    rsi,rdx
mov    r9,r8
sub    rax,QWORD PTR [r12]
sbb    rcx,QWORD PTR [r12+0x8]
sbb    rsi,QWORD PTR [r12+0x10]
sbb    r9,QWORD PTR [r12+0x18]
cmovae rbx,rax
cmovae r11,rcx
cmovae rdx,rsi
cmovae r8,r9
movabs rax,0x7841182db0f9fa8e
movabs rcx,0x70a08b6d0302b0bb
mov    r13,r11
mov    QWORD PTR [rsp+0x1c0],rbx
mov    r14,r8
cmp    rbx,rax
movabs rax,0x2f02d522d0e3951a
mov    rbx,rdx
setb   sil
cmp    r11,rax
movabs r11,0x60c89ce5c2634053
sete   r10b
setb   al
cmp    rdx,rcx
sete   r9b
setb   cl
cmp    r8,r11
sete   r11b
jb     e759f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x6fff>
and    sil,r10b
or     al,sil
and    al,r9b
or     cl,al
and    cl,r11b
je     ec6f6 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc156>
mov    rdx,QWORD PTR [rsp+0x2c0]
mov    r15,QWORD PTR [rsp+0x240]
mov    r8,QWORD PTR [rsp+0x300]
mov    r12,QWORD PTR [rsp+0x940]
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
lea    rdi,[rsp+0x40]
lea    r10,[rsp+0xc00]
movabs r11,0x60c89ce5c2634053
mov    QWORD PTR [rsp+0x40],rdx
mov    QWORD PTR [rsp+0x48],r15
mov    QWORD PTR [rsp+0x50],r8
mov    QWORD PTR [rsp+0x58],r12
add    rdx,QWORD PTR [rdi]
adc    r15,QWORD PTR [rdi+0x8]
adc    r8,QWORD PTR [rdi+0x10]
adc    r12,QWORD PTR [rdi+0x18]
mov    rax,rdx
mov    rcx,r15
mov    rsi,r8
mov    r9,r12
sub    rax,QWORD PTR [r10]
sbb    rcx,QWORD PTR [r10+0x8]
sbb    rsi,QWORD PTR [r10+0x10]
sbb    r9,QWORD PTR [r10+0x18]
cmovae rdx,rax
cmovae r15,rcx
cmovae r8,rsi
cmovae r12,r9
mov    QWORD PTR [rsp+0x40],rdx
mov    QWORD PTR [rsp+0x48],r15
mov    QWORD PTR [rsp+0x50],r8
mov    QWORD PTR [rsp+0x58],r12
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    rdx,QWORD PTR [rdi]
adc    r15,QWORD PTR [rdi+0x8]
adc    r8,QWORD PTR [rdi+0x10]
adc    r12,QWORD PTR [rdi+0x18]
mov    rax,rdx
mov    rcx,r15
mov    rsi,r8
mov    r9,r12
sub    rax,QWORD PTR [r10]
sbb    rcx,QWORD PTR [r10+0x8]
sbb    rsi,QWORD PTR [r10+0x10]
sbb    r9,QWORD PTR [r10+0x18]
cmovae rdx,rax
cmovae r15,rcx
cmovae r8,rsi
cmovae r12,r9
mov    QWORD PTR [rsp+0x40],rdx
mov    QWORD PTR [rsp+0x48],r15
mov    QWORD PTR [rsp+0x50],r8
mov    QWORD PTR [rsp+0x58],r12
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    rdx,QWORD PTR [rdi]
adc    r15,QWORD PTR [rdi+0x8]
adc    r8,QWORD PTR [rdi+0x10]
adc    r12,QWORD PTR [rdi+0x18]
mov    rax,rdx
mov    rcx,r15
mov    rsi,r8
mov    r9,r12
sub    rax,QWORD PTR [r10]
sbb    rcx,QWORD PTR [r10+0x8]
sbb    rsi,QWORD PTR [r10+0x10]
sbb    r9,QWORD PTR [r10+0x18]
cmovae rdx,rax
cmovae r15,rcx
cmovae r8,rsi
cmovae r12,r9
mov    QWORD PTR [rsp+0x40],rdx
mov    QWORD PTR [rsp+0x48],r15
mov    QWORD PTR [rsp+0x50],r8
mov    QWORD PTR [rsp+0x58],r12
vmovaps YMMWORD PTR [rsp+0xc00],ymm2
add    rdx,QWORD PTR [rdi]
adc    r15,QWORD PTR [rdi+0x8]
adc    r8,QWORD PTR [rdi+0x10]
adc    r12,QWORD PTR [rdi+0x18]
mov    rax,rdx
mov    rcx,r15
mov    rsi,r8
mov    r9,r12
sub    rax,QWORD PTR [r10]
sbb    rcx,QWORD PTR [r10+0x8]
sbb    rsi,QWORD PTR [r10+0x10]
sbb    r9,QWORD PTR [r10+0x18]
cmovae rdx,rax
cmovae r15,rcx
cmovae r8,rsi
cmovae r12,r9
movabs rax,0x7841182db0f9fa8e
movabs rcx,0x70a08b6d0302b0bb
cmp    rdx,rax
movabs rax,0x2f02d522d0e3951a
setb   sil
cmp    r15,rax
sete   r10b
setb   al
cmp    r8,rcx
sete   r9b
setb   cl
cmp    r12,r11
sete   r11b
jb     e7788 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x71e8>
and    sil,r10b
or     al,sil
and    al,r9b
or     cl,al
and    cl,r11b
je     ec6f6 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc156>
mov    rdi,QWORD PTR [rsp+0x1080]
cmp    rdi,QWORD PTR [rsp+0x18]
jae    ec94d <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3ad>
mov    rsi,QWORD PTR [rsp+0x10c0]
mov    r10,QWORD PTR [rsp+0x38]
mov    r9,QWORD PTR [rsp+0x28]
mov    eax,DWORD PTR [rsi]
and    eax,0x7fffffff
cmp    QWORD PTR [rsp+0x20],rax
jbe    ec936 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc396>
mov    rcx,QWORD PTR [rsp+0x3c0]
shl    rax,0x7
add    rsi,0xfffffffffffffff0
mov    QWORD PTR [r10+rax*1],rcx
mov    rcx,QWORD PTR [rsp+0x400]
mov    QWORD PTR [r10+rax*1+0x8],rcx
mov    rcx,QWORD PTR [rsp+0x840]
mov    QWORD PTR [r10+rax*1+0x10],rcx
mov    rcx,QWORD PTR [rsp+0x340]
mov    QWORD PTR [r10+rax*1+0x18],rcx
mov    rcx,QWORD PTR [rsp+0x440]
mov    QWORD PTR [r10+rax*1+0x20],rcx
mov    rcx,QWORD PTR [rsp+0x4c0]
mov    QWORD PTR [r10+rax*1+0x28],rcx
mov    rcx,QWORD PTR [rsp+0x280]
mov    QWORD PTR [r10+rax*1+0x30],rcx
mov    rcx,QWORD PTR [rsp+0x480]
mov    QWORD PTR [r10+rax*1+0x38],rcx
mov    rcx,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [r10+rax*1+0x40],rcx
mov    rcx,QWORD PTR [rsp+0x1000]
mov    QWORD PTR [r10+rax*1+0x48],r13
mov    QWORD PTR [r10+rax*1+0x50],rbx
mov    QWORD PTR [r10+rax*1+0x58],r14
mov    QWORD PTR [r10+rax*1+0x60],rdx
mov    QWORD PTR [r10+rax*1+0x68],r15
mov    QWORD PTR [r10+rax*1+0x70],r8
mov    r8,QWORD PTR [rsp+0x1040]
mov    r15,QWORD PTR [rsp+0xf40]
mov    QWORD PTR [r10+rax*1+0x78],r12
lea    r12,[rdi-0x1]
add    r8,0xfffffffffffffec0
add    rcx,0xfffffffffffffec0
cmp    r15,rdi
jb     e4950 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x43b0>
vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x5c0]
vmovdqu64 zmm1,ZMMWORD PTR [rsp+0x600]
vmovdqu64 zmm2,ZMMWORD PTR [rsp+0x640]
vmovdqu64 zmm3,ZMMWORD PTR [rsp+0x680]
vmovdqa64 zmm7,ZMMWORD PTR [rip+0xfffffffffff21ac5]        # 9380 <GCC_except_table830+0xe24>
vmovdqa64 zmm6,ZMMWORD PTR [rip+0xfffffffffff21afb]        # 93c0 <GCC_except_table830+0xe64>
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff21e79]        # 9748 <GCC_except_table830+0x11ec>
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff21fb7]        # 9890 <GCC_except_table830+0x1334>
vmovaps ymm10,YMMWORD PTR [rip+0xfffffffffff2257f]        # 9e60 <GCC_except_table830+0x1904>
movabs r12,0x30644e72e131
mov    r13,QWORD PTR [rsp+0x20]
mov    rbx,QWORD PTR [rsp+0xf80]
mov    r14d,DWORD PTR [rsp+0x1340]
vpunpcklqdq zmm4,zmm0,zmm1
vpunpcklqdq zmm8,zmm2,zmm3
vpunpckhqdq zmm0,zmm0,zmm1
vpunpckhqdq zmm2,zmm2,zmm3
vmovups YMMWORD PTR [rsp+0xc00],ymm10
mov    QWORD PTR [rsp+0xc20],r12
vmovdqa64 zmm5,zmm4
vpermt2q zmm5,zmm7,zmm8
vpermi2q zmm7,zmm0,zmm2
vpermt2q zmm4,zmm6,zmm8
vpermt2q zmm0,zmm6,zmm2
vpshldq zmm3,zmm4,zmm7,0x18
vpshldq zmm4,zmm0,zmm4,0x24
vpsrlq zmm0,zmm0,0xc
vpshldq zmm2,zmm7,zmm5,0xc
vpandq zmm1,zmm5,zmm11
vpsllq zmm5,zmm5,0x4
vpsrlq zmm1,zmm1,0x30
vpandq zmm5,zmm5,zmm9
vpandq zmm8,zmm4,zmm11
vpandq zmm7,zmm3,zmm11
vpandq zmm6,zmm2,zmm11
vpsllq zmm2,zmm2,0x4
vpsllq zmm3,zmm3,0x4
vpsllq zmm4,zmm4,0x4
vpsrlq zmm12,zmm8,0x30
vpsrlq zmm13,zmm7,0x30
vpsrlq zmm14,zmm6,0x30
vpternlogq zmm1,zmm2,zmm9,0xf8
vpternlogq zmm12,zmm0,zmm9,0xf8
vpxor  xmm0,xmm0,xmm0
vpmadd52huq zmm0,zmm12,QWORD BCST [rip+0xfffffffffff21e43]        # 9818 <GCC_except_table830+0x12bc>
vpternlogq zmm14,zmm3,zmm9,0xf8
vpternlogq zmm13,zmm4,zmm9,0xf8
vpmullq zmm7,zmm0,QWORD BCST [rsp+0xc18]
vpmullq zmm9,zmm0,QWORD BCST [rsp+0xc10]
vpmullq zmm6,zmm0,QWORD BCST [rsp+0xc08]
vpsubq zmm4,zmm13,zmm7
vpbroadcastq zmm7,QWORD PTR [rip+0xfffffffffff21eac]        # 98c0 <GCC_except_table830+0x1364>
vpsubq zmm3,zmm14,zmm9
vpmullq zmm9,zmm0,QWORD BCST [rsp+0xc20]
vpsubq zmm1,zmm1,zmm6
vmovups YMMWORD PTR [rsp+0xc00],ymm10
mov    QWORD PTR [rsp+0xc20],r12
vpmullq zmm0,zmm0,zmm7
vpsubq zmm2,zmm12,zmm9
vpaddq zmm0,zmm0,zmm5
vpsraq zmm5,zmm0,0x34
vpandq zmm0,zmm0,zmm11
vpaddq zmm1,zmm1,zmm5
vpaddq zmm7,zmm0,zmm7
vpsraq zmm5,zmm1,0x34
vpandq zmm1,zmm1,zmm11
vpsubq zmm12,zmm1,QWORD BCST [rsp+0xc08]
vpsraq zmm10,zmm7,0x34
vpaddq zmm3,zmm3,zmm5
vpsraq zmm5,zmm3,0x34
vpandq zmm3,zmm3,zmm11
vpsubq zmm6,zmm3,QWORD BCST [rsp+0xc10]
vpaddq zmm4,zmm4,zmm5
vpsraq zmm5,zmm4,0x34
vpandq zmm4,zmm4,zmm11
vpsubq zmm8,zmm4,QWORD BCST [rsp+0xc18]
vpaddq zmm2,zmm2,zmm5
vpandq zmm2,zmm2,zmm11
vpsubq zmm9,zmm2,QWORD BCST [rsp+0xc20]
vpaddq zmm5,zmm12,zmm10
vpsraq zmm10,zmm5,0x34
vpaddq zmm6,zmm6,zmm10
vpsraq zmm10,zmm6,0x34
vpaddq zmm8,zmm8,zmm10
vpsraq zmm10,zmm8,0x34
vpaddq zmm9,zmm9,zmm10
vpcmpltuq k1,zmm9,QWORD BCST [rip+0xfffffffffff21cd7]        # 97e8 <GCC_except_table830+0x128c>
vpandq zmm0{k1},zmm7,zmm11
vpandq zmm1{k1},zmm5,zmm11
vpandq zmm3{k1},zmm6,zmm11
vpandq zmm4{k1},zmm8,zmm11
vpandq zmm2{k1},zmm9,zmm11
vmovdqa64 ZMMWORD PTR [rsp+0x40],zmm0
vmovdqa64 ZMMWORD PTR [rsp+0x80],zmm1
vmovdqa64 ZMMWORD PTR [rsp+0xc0],zmm3
vmovdqa64 ZMMWORD PTR [rsp+0x100],zmm4
vmovdqa64 ZMMWORD PTR [rsp+0x140],zmm2
mov    r8,QWORD PTR [rsp+0x30]
test   r14b,r14b
jne    ec6e4 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc144>
vmovdqa ymm2,YMMWORD PTR [rip+0xfffffffffff222f3]        # 9e60 <GCC_except_table830+0x1904>
vpbroadcastq zmm21,QWORD PTR [rip+0xfffffffffff21dd1]        # 9948 <GCC_except_table830+0x13ec>
vpbroadcastq zmm30,QWORD PTR [rip+0xfffffffffff21bc7]        # 9748 <GCC_except_table830+0x11ec>
vpbroadcastq zmm28,QWORD PTR [rip+0xfffffffffff21c15]        # 97a0 <GCC_except_table830+0x1244>
mov    r14,QWORD PTR [rsp+0x18]
sub    r14,r15
add    r14,0x8
nop    WORD PTR [rax+rax*1+0x0]
mov    rsi,QWORD PTR [r9+0x10]
lea    rdi,[rbx-0x1]
cmp    rdi,rsi
jae    eca2e <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc48e>
mov    rcx,QWORD PTR [r9+0x8]
mov    rsi,QWORD PTR [r9+0x10]
vmovdqu YMMWORD PTR [rsp+0x5c0],ymm2
mov    QWORD PTR [rsp+0x5e0],r12
cmp    rdi,rsi
jae    eca1f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc47f>
mov    rdx,QWORD PTR [r9+0x8]
lea    rax,[rdi+rdi*4]
vpbroadcastq zmm22,QWORD PTR [rsp+0x5c8]
vpbroadcastq zmm24,QWORD PTR [rsp+0x5d0]
vpbroadcastq zmm29,QWORD PTR [rsp+0x5d8]
vpbroadcastq zmm27,QWORD PTR [rsp+0x5e0]
vmovdqa64 zmm11,ZMMWORD PTR [rsp+0x80]
vmovdqa64 zmm10,ZMMWORD PTR [rsp+0xc0]
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x100]
vmovdqa64 zmm14,ZMMWORD PTR [rsp+0x40]
vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x140]
vpxord xmm18,xmm18,xmm18
vpxord xmm19,xmm19,xmm19
vpxord xmm16,xmm16,xmm16
vpxord xmm17,xmm17,xmm17
vpxor  xmm15,xmm15,xmm15
vpxor  xmm13,xmm13,xmm13
mov    rsi,QWORD PTR [rsp+0x18]
mov    QWORD PTR [rsp+0x480],rdi
lea    rdi,[rbx*8-0x8]
shl    rax,0x7
vmovdqa64 zmm9,ZMMWORD PTR [rcx+rax*1]
vmovdqa64 zmm8,ZMMWORD PTR [rcx+rax*1+0x40]
vmovdqa64 zmm26,ZMMWORD PTR [rcx+rax*1+0x80]
vmovdqa64 zmm1,ZMMWORD PTR [rcx+rax*1+0xc0]
vmovdqa64 zmm5,ZMMWORD PTR [rcx+rax*1+0x100]
vmovdqu YMMWORD PTR [rsp+0x5c0],ymm2
mov    QWORD PTR [rsp+0x5e0],r12
vpbroadcastq zmm12,QWORD PTR [rsp+0x5c8]
vpbroadcastq zmm7,QWORD PTR [rsp+0x5d0]
vpbroadcastq zmm4,QWORD PTR [rsp+0x5d8]
vpbroadcastq zmm2,QWORD PTR [rsp+0x5e0]
vmovdqa64 zmm3,ZMMWORD PTR [rdx+rax*1+0x140]
vmovdqa64 zmm20,ZMMWORD PTR [rdx+rax*1+0x1c0]
sub    rsi,rdi
vpmadd52huq zmm19,zmm14,zmm3
vpmadd52huq zmm16,zmm11,zmm3
vpmadd52huq zmm17,zmm10,zmm3
vpmadd52huq zmm15,zmm0,zmm3
vpmadd52luq zmm18,zmm14,zmm3
vpmadd52huq zmm13,zmm6,zmm3
vpmadd52luq zmm19,zmm11,zmm3
vpmadd52luq zmm16,zmm10,zmm3
vpmadd52luq zmm17,zmm0,zmm3
vpmadd52luq zmm15,zmm6,zmm3
vpxor  xmm3,xmm3,xmm3
vpmadd52luq zmm3,zmm18,zmm21
vpaddq zmm18,zmm18,zmm30
vpsrlq zmm18,zmm18,0x34
vpmadd52huq zmm18,zmm3,zmm28
vpmadd52luq zmm19,zmm3,zmm12
vpmadd52huq zmm16,zmm3,zmm12
vpmadd52huq zmm17,zmm3,zmm7
vpmadd52huq zmm15,zmm3,zmm4
vpmadd52huq zmm13,zmm3,zmm2
vpaddq zmm18,zmm19,zmm18
vmovdqa64 zmm19,ZMMWORD PTR [rdx+rax*1+0x180]
vpmadd52luq zmm16,zmm3,zmm7
vpmadd52luq zmm17,zmm3,zmm4
vpmadd52luq zmm15,zmm3,zmm2
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm16,zmm14,zmm19
vpmadd52huq zmm17,zmm11,zmm19
vpmadd52huq zmm15,zmm10,zmm19
vpmadd52huq zmm13,zmm0,zmm19
vpmadd52luq zmm18,zmm14,zmm19
vpmadd52huq zmm3,zmm6,zmm19
vpmadd52luq zmm16,zmm11,zmm19
vpmadd52luq zmm17,zmm10,zmm19
vpmadd52luq zmm15,zmm0,zmm19
vpmadd52luq zmm13,zmm6,zmm19
vpxord xmm19,xmm19,xmm19
vpmadd52luq zmm19,zmm18,zmm21
vpaddq zmm18,zmm18,zmm30
vpsrlq zmm18,zmm18,0x34
vpmadd52huq zmm15,zmm19,zmm7
vpmadd52huq zmm17,zmm19,zmm12
vpmadd52huq zmm18,zmm19,zmm28
vpmadd52luq zmm16,zmm19,zmm12
vpmadd52huq zmm13,zmm19,zmm4
vpmadd52huq zmm3,zmm19,zmm2
vpmadd52luq zmm15,zmm19,zmm4
vpaddq zmm18,zmm16,zmm18
vpmadd52luq zmm17,zmm19,zmm7
vpmadd52luq zmm13,zmm19,zmm2
vpmadd52huq zmm3,zmm0,zmm20
vpxord xmm19,xmm19,xmm19
vpxord xmm16,xmm16,xmm16
vpmadd52huq zmm16,zmm6,zmm20
vpmadd52luq zmm18,zmm14,zmm20
vpmadd52huq zmm15,zmm11,zmm20
vpmadd52huq zmm17,zmm14,zmm20
vpmadd52huq zmm13,zmm10,zmm20
vpmadd52luq zmm3,zmm6,zmm20
vpmadd52luq zmm19,zmm18,zmm21
vpaddq zmm18,zmm18,zmm30
vpsrlq zmm18,zmm18,0x34
vpmadd52luq zmm15,zmm10,zmm20
vpmadd52luq zmm17,zmm11,zmm20
vpmadd52luq zmm13,zmm0,zmm20
vmovdqa64 zmm20,ZMMWORD PTR [rdx+rax*1+0x200]
vpmadd52huq zmm18,zmm19,zmm28
vpmadd52huq zmm3,zmm19,zmm4
vpmadd52huq zmm16,zmm19,zmm2
vpmadd52huq zmm15,zmm19,zmm12
vpmadd52luq zmm17,zmm19,zmm12
vpmadd52huq zmm13,zmm19,zmm7
vpmadd52luq zmm3,zmm19,zmm2
vpmadd52luq zmm15,zmm19,zmm7
vpaddq zmm18,zmm17,zmm18
vpmadd52luq zmm13,zmm19,zmm4
vpmadd52huq zmm16,zmm0,zmm20
vpxord xmm19,xmm19,xmm19
vpxord xmm17,xmm17,xmm17
vpmadd52huq zmm17,zmm6,zmm20
vpmadd52luq zmm18,zmm14,zmm20
vpmadd52huq zmm3,zmm10,zmm20
vpmadd52huq zmm15,zmm14,zmm20
vpmadd52huq zmm13,zmm11,zmm20
vpmadd52luq zmm16,zmm6,zmm20
vpmadd52luq zmm19,zmm18,zmm21
vpaddq zmm18,zmm18,zmm30
vpmadd52luq zmm3,zmm0,zmm20
vpsrlq zmm18,zmm18,0x34
vpmadd52luq zmm15,zmm11,zmm20
vpmadd52luq zmm13,zmm10,zmm20
vpmadd52huq zmm18,zmm19,zmm28
vpmadd52huq zmm3,zmm19,zmm7
vpmadd52huq zmm16,zmm19,zmm4
vpmadd52huq zmm17,zmm19,zmm2
vpmadd52luq zmm15,zmm19,zmm12
vpmadd52huq zmm13,zmm19,zmm12
vpmadd52luq zmm3,zmm19,zmm4
vpmadd52luq zmm16,zmm19,zmm2
vpaddq zmm15,zmm15,zmm18
vmovdqa64 zmm18,ZMMWORD PTR [rdx+rax*1+0x240]
vpmadd52luq zmm13,zmm19,zmm7
mov    eax,0x0
cmovb  rsi,rax
vpmadd52huq zmm13,zmm14,zmm18
vpmadd52huq zmm3,zmm11,zmm18
vpmadd52huq zmm16,zmm10,zmm18
vpmadd52huq zmm17,zmm0,zmm18
vpmadd52luq zmm15,zmm14,zmm18
vpxor  xmm14,xmm14,xmm14
vpmadd52huq zmm14,zmm6,zmm18
vpmadd52luq zmm13,zmm11,zmm18
vpmadd52luq zmm3,zmm10,zmm18
vpmadd52luq zmm16,zmm0,zmm18
vpmadd52luq zmm17,zmm6,zmm18
vpxord xmm18,xmm18,xmm18
vpmadd52luq zmm18,zmm15,zmm21
vpmadd52huq zmm16,zmm18,zmm7
vpmadd52huq zmm17,zmm18,zmm4
vpmadd52huq zmm3,zmm18,zmm12
vpmadd52huq zmm14,zmm18,zmm2
vpmadd52luq zmm13,zmm18,zmm12
vpmadd52luq zmm16,zmm18,zmm4
vpaddq zmm4,zmm15,zmm30
vpmadd52luq zmm17,zmm18,zmm2
vpmadd52luq zmm3,zmm18,zmm7
vpsrlq zmm2,zmm4,0x34
vpmadd52huq zmm2,zmm18,zmm28
vpaddq zmm2,zmm13,zmm2
vpsrlq zmm4,zmm2,0x34
vpandq zmm2,zmm2,zmm30
vpaddq zmm3,zmm4,zmm3
vpsrlq zmm4,zmm3,0x34
vpaddq zmm7,zmm4,zmm16
vpsrlq zmm4,zmm7,0x34
vpaddq zmm12,zmm4,zmm17
vpsrlq zmm4,zmm12,0x34
vpaddq zmm13,zmm4,zmm14
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x40]
vmovdqa64 ZMMWORD PTR [rsp+0x40],zmm2
vpandq zmm2,zmm3,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0x80],zmm2
vpandq zmm2,zmm7,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0xc0],zmm2
vpandq zmm2,zmm12,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0x100],zmm2
vmovdqa64 ZMMWORD PTR [rsp+0x140],zmm13
jbe    eca0b <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc46b>
mov    r11,rdi
shl    r11,0x4
mov    ebx,DWORD PTR [r8+r11*1]
mov    ecx,ebx
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    eca42 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc4a2>
shl    rcx,0x7
mov    r9,rdi
or     r9,0x1
vmovdqu64 zmm2,ZMMWORD PTR [r10+rcx*1+0x40]
vmovdqu64 ymm23,YMMWORD PTR [r10+rcx*1]
vmovdqu64 ymm20,YMMWORD PTR [r10+rcx*1+0x20]
vmovdqa64 ZMMWORD PTR [rsp+0x5c0],zmm2
cmp    rsi,0x1
je     eca08 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc468>
shl    r9,0x4
mov    ecx,DWORD PTR [r8+r9*1]
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    eca42 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc4a2>
shl    rcx,0x7
mov    r15,rdi
lea    rax,[rsp+0x600]
or     r15,0x2
vmovups ymm2,YMMWORD PTR [r10+rcx*1]
vmovups YMMWORD PTR [rax],ymm2
cmp    rsi,0x2
je     ec9f1 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc451>
shl    r15,0x4
mov    eax,DWORD PTR [r8+r15*1]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec7a6 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc206>
shl    rax,0x7
vmovdqu64 ymm25,YMMWORD PTR [r10+rcx*1+0x20]
vmovdqu ymm14,YMMWORD PTR [r10+rcx*1+0x40]
vmovdqu ymm12,YMMWORD PTR [r10+rcx*1+0x60]
mov    r12,rdi
or     r12,0x3
vmovdqu64 zmm2,ZMMWORD PTR [r10+rax*1+0x40]
vmovdqu64 ymm16,YMMWORD PTR [r10+rax*1]
vmovdqu ymm3,YMMWORD PTR [r10+rax*1+0x20]
vmovdqa64 ZMMWORD PTR [rsp+0xc00],zmm2
cmp    rsi,0x3
je     ec9da <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc43a>
shl    r12,0x4
mov    ecx,DWORD PTR [r8+r12*1]
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    eca42 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc4a2>
shl    rcx,0x7
mov    rdx,rdi
lea    rax,[rsp+0xc40]
or     rdx,0x4
vmovups ymm2,YMMWORD PTR [r10+rcx*1]
vmovups YMMWORD PTR [rax],ymm2
cmp    rsi,0x4
je     ec9c3 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc423>
shl    rdx,0x4
mov    eax,DWORD PTR [r8+rdx*1]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec7a6 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc206>
shl    rax,0x7
vmovdqu64 ymm31,YMMWORD PTR [r10+rcx*1+0x20]
vmovdqu64 ymm17,YMMWORD PTR [r10+rcx*1+0x40]
vmovdqu ymm2,YMMWORD PTR [r10+rcx*1+0x60]
vmovdqu64 zmm7,ZMMWORD PTR [r10+rax*1+0x40]
vmovdqu64 ymm28,YMMWORD PTR [r10+rax*1]
vmovdqu ymm15,YMMWORD PTR [r10+rax*1+0x20]
mov    rax,rdi
or     rax,0x5
vmovdqa64 ZMMWORD PTR [rsp+0x500],zmm7
cmp    rsi,0x5
je     ec9ac <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc40c>
shl    rax,0x4
mov    ecx,DWORD PTR [r8+rax*1]
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    eca42 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc4a2>
shl    rcx,0x7
vmovups ymm7,YMMWORD PTR [r10+rcx*1]
lea    r10,[rsp+0x540]
vmovups YMMWORD PTR [r10],ymm7
mov    r10,rdi
or     r10,0x6
cmp    rsi,0x6
je     ec995 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3f5>
shl    r10,0x4
mov    r8d,DWORD PTR [r8+r10*1]
and    r8d,0x7fffffff
cmp    r13,r8
jbe    ec961 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3c1>
mov    r13,QWORD PTR [rsp+0x38]
shl    r8,0x7
vmovups ymm7,YMMWORD PTR [r13+r8*1+0x0]
vmovdqu64 zmm13,ZMMWORD PTR [r13+r8*1+0x40]
vmovdqu64 ymm21,YMMWORD PTR [r13+rcx*1+0x20]
vmovdqu64 ymm18,YMMWORD PTR [r13+rcx*1+0x40]
vmovdqu64 ymm19,YMMWORD PTR [r13+rcx*1+0x60]
vmovaps ZMMWORD PTR [rsp+0x1c0],zmm7
vmovdqu ymm7,YMMWORD PTR [r13+r8*1+0x20]
mov    r13,rdi
or     r13,0x7
vmovdqa64 ZMMWORD PTR [rsp+0x12c0],zmm13
cmp    rsi,0x7
je     ec97e <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3de>
mov    r8,QWORD PTR [rsp+0x30]
shl    r13,0x4
vmovdqa64 YMMWORD PTR [rsp+0x180],ymm21
vmovdqa64 YMMWORD PTR [rsp+0x200],ymm18
mov    QWORD PTR [rsp+0x4c0],r14
mov    ecx,DWORD PTR [r8+r13*1]
and    ecx,0x7fffffff
cmp    QWORD PTR [rsp+0x20],rcx
jbe    eca3d <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc49d>
mov    rsi,QWORD PTR [rsp+0x38]
shl    rcx,0x7
lea    r14,[rsp+0x1300]
vmovups ymm13,YMMWORD PTR [rsi+rcx*1]
vmovdqu64 ymm18,YMMWORD PTR [rsi+rcx*1+0x40]
vmovups ymm21,YMMWORD PTR [rsi+rcx*1+0x60]
vinserti64x4 zmm7,zmm7,YMMWORD PTR [rsi+rcx*1+0x20],0x1
lea    rsi,[rdi+0x8]
vmovups YMMWORD PTR [r14],ymm13
vmovups zmm13,ZMMWORD PTR [rsp+0x5e0]
vmovups ZMMWORD PTR [rsp+0x11e0],zmm13
vmovaps zmm13,ZMMWORD PTR [rsp+0x5c0]
vmovaps ZMMWORD PTR [rsp+0x11c0],zmm13
vmovaps zmm13,ZMMWORD PTR [rsp+0xc00]
vmovaps ZMMWORD PTR [rsp+0xb40],zmm13
vmovups zmm13,ZMMWORD PTR [rsp+0xc20]
vmovups ZMMWORD PTR [rsp+0xb60],zmm13
vmovaps zmm13,ZMMWORD PTR [rsp+0x500]
vmovaps ZMMWORD PTR [rsp+0x1240],zmm13
vmovups zmm13,ZMMWORD PTR [rsp+0x520]
vmovups ZMMWORD PTR [rsp+0x1260],zmm13
vmovaps zmm13,ZMMWORD PTR [rsp+0x12c0]
vmovaps ZMMWORD PTR [rsp+0x1440],zmm13
vmovdqu64 zmm13,ZMMWORD PTR [rsp+0x12e0]
vmovdqu64 ZMMWORD PTR [rsp+0x1460],zmm13
cmp    rsi,QWORD PTR [rsp+0x18]
ja     ec711 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc171>
vmovaps YMMWORD PTR [rsp+0x940],ymm21
vmovdqa64 zmm21,zmm27
vpbroadcastq zmm27,QWORD PTR [rip+0xfffffffffff21579]        # 9948 <GCC_except_table830+0x13ec>
vpxor  xmm13,xmm13,xmm13
vpmadd52huq zmm13,zmm8,zmm4
vmovdqa64 YMMWORD PTR [rsp+0x980],ymm17
vpxord xmm17,xmm17,xmm17
vmovdqa YMMWORD PTR [rsp+0x280],ymm2
vpxor  xmm2,xmm2,xmm2
vpmadd52huq zmm2,zmm9,zmm4
vmovdqa64 ZMMWORD PTR [rsp+0x340],zmm20
vmovdqa64 zmm20,zmm29
vpxord xmm29,xmm29,xmm29
vpmadd52huq zmm29,zmm26,zmm4
vmovdqa YMMWORD PTR [rsp+0x380],ymm14
vpxor  xmm14,xmm14,xmm14
vpmadd52huq zmm14,zmm1,zmm4
vpmadd52luq zmm17,zmm9,zmm4
vmovdqa64 YMMWORD PTR [rsp+0x440],ymm19
vpxord xmm19,xmm19,xmm19
vmovdqa YMMWORD PTR [rsp+0x9c0],ymm12
vpmadd52huq zmm19,zmm5,zmm4
add    r9,r8
add    r15,r8
add    r12,r8
add    rdx,r8
add    rax,r8
add    r10,r8
add    r11,r8
add    r13,r8
vmovdqa64 ZMMWORD PTR [rsp+0x2c0],zmm24
vmovdqa64 ZMMWORD PTR [rsp+0x1040],zmm21
vmovdqa64 ZMMWORD PTR [rsp+0x240],zmm22
vmovdqa64 ZMMWORD PTR [rsp+0x300],zmm20
mov    rdi,QWORD PTR [r9+0x8]
mov    r9,QWORD PTR [r15+0x8]
mov    r12,QWORD PTR [r12+0x8]
mov    rdx,QWORD PTR [rdx+0x8]
mov    rcx,QWORD PTR [rax+0x8]
mov    r8,QWORD PTR [r11+0x8]
mov    rsi,QWORD PTR [r10+0x8]
mov    r15,QWORD PTR [r13+0x8]
mov    ax,0x1111
kmovd  k1,eax
mov    al,0x11
vpmadd52luq zmm2,zmm8,zmm4
vpmadd52luq zmm13,zmm26,zmm4
vpmadd52luq zmm29,zmm1,zmm4
vpmadd52luq zmm14,zmm5,zmm4
vpxor  xmm4,xmm4,xmm4
vpmadd52luq zmm4,zmm17,zmm27
vmovdqa64 zmm12,zmm27
vpbroadcastq zmm27,QWORD PTR [rip+0xfffffffffff212bf]        # 97a0 <GCC_except_table830+0x1244>
vpaddq zmm17,zmm17,zmm30
vpsrlq zmm17,zmm17,0x34
vpmadd52huq zmm14,zmm4,zmm20
vpmadd52huq zmm13,zmm4,zmm22
vpmadd52huq zmm29,zmm4,zmm24
vpmadd52luq zmm2,zmm4,zmm22
vpmadd52huq zmm19,zmm4,zmm21
vpmadd52huq zmm17,zmm4,zmm27
vpmadd52luq zmm14,zmm4,zmm21
vpmadd52luq zmm13,zmm4,zmm24
vpmadd52luq zmm29,zmm4,zmm20
vpmadd52huq zmm19,zmm1,zmm11
vpxor  xmm4,xmm4,xmm4
vpaddq zmm2,zmm2,zmm17
vpmadd52huq zmm14,zmm26,zmm11
vpmadd52huq zmm13,zmm9,zmm11
vpmadd52huq zmm29,zmm8,zmm11
vpmadd52luq zmm19,zmm5,zmm11
vpxord xmm17,xmm17,xmm17
vpmadd52huq zmm17,zmm5,zmm11
vpmadd52luq zmm2,zmm9,zmm11
vpmadd52luq zmm14,zmm1,zmm11
vpmadd52luq zmm13,zmm8,zmm11
vpmadd52luq zmm29,zmm26,zmm11
vpxor  xmm11,xmm11,xmm11
vpmadd52huq zmm11,zmm5,zmm10
vpmadd52luq zmm4,zmm2,zmm12
vpaddq zmm2,zmm2,zmm30
vpsrlq zmm2,zmm2,0x34
vpmadd52huq zmm14,zmm4,zmm24
vpmadd52huq zmm19,zmm4,zmm20
vpmadd52huq zmm29,zmm4,zmm22
vpmadd52huq zmm2,zmm4,zmm27
vpmadd52luq zmm13,zmm4,zmm22
vpmadd52huq zmm17,zmm4,zmm21
vpmadd52luq zmm14,zmm4,zmm20
vpmadd52luq zmm19,zmm4,zmm21
vpmadd52luq zmm29,zmm4,zmm24
vpaddq zmm2,zmm13,zmm2
vpmadd52huq zmm17,zmm1,zmm10
vpxor  xmm4,xmm4,xmm4
vpmadd52luq zmm2,zmm9,zmm10
vpmadd52huq zmm14,zmm8,zmm10
vpmadd52huq zmm19,zmm26,zmm10
vpmadd52huq zmm29,zmm9,zmm10
vpmadd52luq zmm17,zmm5,zmm10
vpmadd52luq zmm4,zmm2,zmm12
vpaddq zmm2,zmm2,zmm30
vpsrlq zmm2,zmm2,0x34
vpmadd52luq zmm14,zmm26,zmm10
vpmadd52luq zmm19,zmm1,zmm10
vpmadd52luq zmm29,zmm8,zmm10
vpmadd52huq zmm17,zmm4,zmm20
vpmadd52huq zmm2,zmm4,zmm27
vpmadd52huq zmm11,zmm4,zmm21
vpmadd52huq zmm14,zmm4,zmm22
vpmadd52huq zmm19,zmm4,zmm24
vpmadd52luq zmm29,zmm4,zmm22
vpmadd52luq zmm17,zmm4,zmm21
vpmadd52huq zmm11,zmm1,zmm0
vpmadd52luq zmm14,zmm4,zmm24
vpmadd52luq zmm19,zmm4,zmm20
vpaddq zmm2,zmm29,zmm2
vpxor  xmm4,xmm4,xmm4
vpmadd52huq zmm4,zmm5,zmm0
vmovdqa64 zmm29,zmm27
vpmadd52huq zmm17,zmm26,zmm0
vpmadd52luq zmm2,zmm9,zmm0
vpmadd52luq zmm11,zmm5,zmm0
vpmadd52huq zmm14,zmm9,zmm0
vpmadd52huq zmm19,zmm8,zmm0
vpmadd52luq zmm17,zmm1,zmm0
vpmadd52luq zmm14,zmm8,zmm0
vpmadd52luq zmm19,zmm26,zmm0
vpxor  xmm0,xmm0,xmm0
vpmadd52luq zmm0,zmm2,zmm12
vpaddq zmm2,zmm2,zmm30
vpsrlq zmm2,zmm2,0x34
vpmadd52huq zmm19,zmm0,zmm22
vpmadd52huq zmm17,zmm0,zmm24
vpmadd52huq zmm11,zmm0,zmm20
vpmadd52huq zmm4,zmm0,zmm21
vpmadd52huq zmm2,zmm0,zmm27
vpmadd52luq zmm14,zmm0,zmm22
vmovdqa64 zmm27,ZMMWORD PTR [rip+0xfffffffffff20ce2]        # 93c0 <GCC_except_table830+0xe64>
vpmadd52luq zmm19,zmm0,zmm24
vpmadd52luq zmm11,zmm0,zmm21
vpmadd52luq zmm17,zmm0,zmm20
vpmadd52huq zmm4,zmm1,zmm6
vpaddq zmm2,zmm14,zmm2
vpxor  xmm0,xmm0,xmm0
vpmadd52huq zmm0,zmm5,zmm6
vpbroadcastq zmm24,QWORD PTR [rip+0xfffffffffff21218]        # 9928 <GCC_except_table830+0x13cc>
vpbroadcastq zmm21,QWORD PTR [rip+0xfffffffffff210de]        # 97f8 <GCC_except_table830+0x129c>
vpmadd52luq zmm2,zmm9,zmm6
vpmadd52huq zmm19,zmm9,zmm6
vpmadd52huq zmm17,zmm8,zmm6
vpmadd52huq zmm11,zmm26,zmm6
vpmadd52luq zmm4,zmm5,zmm6
vmovdqa64 ZMMWORD PTR [rsp+0xfc0],zmm0
vinserti64x4 zmm0,zmm23,YMMWORD PTR [rsp+0x1200],0x1
vmovdqa64 ZMMWORD PTR [rsp+0x1000],zmm2
vinserti64x4 zmm2,zmm28,YMMWORD PTR [rsp+0x1280],0x1
vpmadd52luq zmm19,zmm8,zmm6
vpmadd52luq zmm11,zmm1,zmm6
vpmadd52luq zmm17,zmm26,zmm6
vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x1c0]
vinserti64x4 zmm1,zmm16,YMMWORD PTR [rsp+0xb80],0x1
vmovdqa64 ZMMWORD PTR [rsp+0xec0],zmm4
vmovdqa64 zmm8,ZMMWORD PTR [rip+0xfffffffffff20beb]        # 9380 <GCC_except_table830+0xe24>
vinserti64x4 zmm4,zmm6,YMMWORD PTR [rsp+0x1480],0x1
vmovdqa64 ZMMWORD PTR [rsp+0x1380],zmm11
vmovdqa64 ZMMWORD PTR [rsp+0x13c0],zmm17
vmovdqa64 ZMMWORD PTR [rsp+0x1180],zmm19
vpunpcklqdq zmm26,zmm0,zmm1
vpunpckhqdq zmm14,zmm0,zmm1
vpunpcklqdq zmm0,zmm2,zmm4
vmovdqa64 zmm9,zmm26
vpunpckhqdq zmm1,zmm2,zmm4
vinserti64x4 zmm2,zmm15,YMMWORD PTR [rsp+0x180],0x1
vmovdqa64 zmm28,zmm14
vpermt2q zmm9,zmm8,zmm0
vpermt2q zmm26,zmm27,zmm0
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x340]
vpermt2q zmm28,zmm8,zmm1
vpermt2q zmm14,zmm27,zmm1
vinserti64x4 zmm1,zmm3,ymm31,0x1
vinserti64x4 zmm0,zmm0,ymm25,0x1
vpbroadcastq zmm25,QWORD PTR [rip+0xfffffffffff21162]        # 9980 <GCC_except_table830+0x1424>
vpunpcklqdq zmm3,zmm0,zmm1
vpunpckhqdq zmm1,zmm0,zmm1
vpunpcklqdq zmm0,zmm2,zmm7
vpunpckhqdq zmm2,zmm2,zmm7
vmovdqa64 zmm4,zmm3
vpermt2q zmm3,zmm27,zmm0
vmovdqa64 zmm5,zmm1
vpermt2q zmm1,zmm27,zmm2
vpermt2q zmm5,zmm8,zmm2
vpermt2q zmm4,zmm8,zmm0
vmovdqu ymm0,YMMWORD PTR [r8]
vmovdqu ymm2,YMMWORD PTR [r8+0x20]
vinserti64x4 zmm0,zmm0,YMMWORD PTR [rdi],0x1
vinserti64x4 zmm2,zmm2,YMMWORD PTR [rdi+0x20],0x1
vmovdqa64 ZMMWORD PTR [rsp+0xbc0],zmm3
vmovdqu ymm3,YMMWORD PTR [r9]
vmovdqa64 ZMMWORD PTR [rsp+0x1c0],zmm1
vmovdqa64 ZMMWORD PTR [rsp+0xf40],zmm5
vmovdqa64 ZMMWORD PTR [rsp+0x3c0],zmm4
vinserti64x4 zmm1,zmm3,YMMWORD PTR [r12],0x1
vmovdqu ymm3,YMMWORD PTR [rdx]
vinserti64x4 zmm5,zmm3,YMMWORD PTR [rcx],0x1
vmovdqu ymm3,YMMWORD PTR [rsi]
vinserti64x4 zmm6,zmm3,YMMWORD PTR [r15],0x1
vpunpcklqdq zmm3,zmm0,zmm1
vpunpckhqdq zmm15,zmm0,zmm1
vmovdqa64 zmm16,zmm3
vmovdqa64 zmm12,zmm15
vpunpcklqdq zmm0,zmm5,zmm6
vpunpckhqdq zmm4,zmm5,zmm6
vpermt2q zmm16,zmm8,zmm0
vpermt2q zmm3,zmm27,zmm0
vmovdqu ymm0,YMMWORD PTR [r9+0x20]
vpermt2q zmm12,zmm8,zmm4
vpermt2q zmm15,zmm27,zmm4
vinserti64x4 zmm0,zmm0,YMMWORD PTR [r12+0x20],0x1
vpunpcklqdq zmm5,zmm2,zmm0
vpunpckhqdq zmm1,zmm2,zmm0
vmovdqu ymm0,YMMWORD PTR [rdx+0x20]
vmovdqu ymm2,YMMWORD PTR [rsi+0x20]
vinserti64x4 zmm0,zmm0,YMMWORD PTR [rcx+0x20],0x1
vinserti64x4 zmm2,zmm2,YMMWORD PTR [r15+0x20],0x1
vpunpcklqdq zmm4,zmm0,zmm2
vpunpckhqdq zmm0,zmm0,zmm2
vmovdqa64 zmm2,zmm5
vpermt2q zmm2,zmm8,zmm4
vpermt2q zmm5,zmm27,zmm4
vmovdqa64 zmm4,zmm1
vpermt2q zmm4,zmm8,zmm0
vpermt2q zmm1,zmm27,zmm0
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x11c0]
vinserti64x4 zmm0,zmm0,YMMWORD PTR [rsp+0x380],0x1
vmovdqa64 ZMMWORD PTR [rsp+0xf00],zmm4
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x1240]
vmovdqa64 ZMMWORD PTR [rsp+0x180],zmm1
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0xb40]
vmovdqa64 ZMMWORD PTR [rsp+0xb00],zmm2
vmovdqa64 ZMMWORD PTR [rsp+0x10c0],zmm5
vpandq zmm5,zmm16,zmm30
vinserti64x4 zmm7,zmm4,YMMWORD PTR [rsp+0x200],0x1
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x1440]
vinserti64x4 zmm2,zmm1,YMMWORD PTR [rsp+0x980],0x1
vmovdqa64 ZMMWORD PTR [rsp+0xa40],zmm5
vinserti64x4 zmm10,zmm4,ymm18,0x1
vpunpcklqdq zmm4,zmm0,zmm2
vpunpckhqdq zmm1,zmm0,zmm2
vpunpcklqdq zmm0,zmm7,zmm10
vpunpckhqdq zmm2,zmm7,zmm10
vmovdqa64 zmm13,zmm4
vmovdqa64 zmm7,zmm1
vpermt2q zmm13,zmm8,zmm0
vpermt2q zmm4,zmm27,zmm0
vmovdqa64 zmm0,zmm1
vpermt2q zmm0,zmm27,zmm2
vpermt2q zmm7,zmm8,zmm2
vpandq zmm1,zmm9,zmm30
vpshldq zmm9,zmm28,zmm9,0xc
vmovdqa64 zmm31,zmm0
vmovdqa64 ZMMWORD PTR [rsp+0x380],zmm0
vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x11e0]
vinserti64x4 zmm2,zmm0,YMMWORD PTR [rsp+0x9c0],0x1
vmovdqu64 zmm0,ZMMWORD PTR [rsp+0xb60]
vinserti64x4 zmm10,zmm0,YMMWORD PTR [rsp+0x280],0x1
vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x1260]
vmovdqa64 ZMMWORD PTR [rsp+0x280],zmm1
vinserti64x4 zmm11,zmm0,YMMWORD PTR [rsp+0x440],0x1
vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x1460]
vinserti64x4 zmm17,zmm0,YMMWORD PTR [rsp+0x940],0x1
vpandq zmm0,zmm9,zmm30
vpshldq zmm9,zmm12,zmm16,0xc
vpshldq zmm12,zmm3,zmm12,0x18
vpshldq zmm3,zmm15,zmm3,0x24
vmovdqa64 ZMMWORD PTR [rsp+0x200],zmm0
vpandq zmm16,zmm9,zmm30
vpsubq zmm9,zmm5,zmm1
vmovdqa64 ZMMWORD PTR [rsp+0xa80],zmm16
vpsubq zmm16,zmm16,zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20ecc]        # 9990 <GCC_except_table830+0x1434>
vpaddq zmm1,zmm9,zmm29
vmovdqa64 ZMMWORD PTR [rsp+0xf80],zmm1
vpunpcklqdq zmm6,zmm2,zmm10
vpunpckhqdq zmm19,zmm2,zmm10
vmovdqa64 zmm10,zmm19
vpunpcklqdq zmm2,zmm11,zmm17
vpunpckhqdq zmm17,zmm11,zmm17
vmovdqa64 zmm11,zmm6
vpermt2q zmm10,zmm8,zmm17
vpermt2q zmm19,zmm27,zmm17
vpsraq zmm17,zmm1,0x34
vpermt2q zmm11,zmm8,zmm2
vpermt2q zmm6,zmm27,zmm2
vpandq zmm2,zmm12,zmm30
vpbroadcastq zmm12,QWORD PTR [rip+0xfffffffffff20d5b]        # 9880 <GCC_except_table830+0x1324>
vpaddq zmm16,zmm16,zmm17
vpshldq zmm17,zmm26,zmm28,0x18
vmovdqa64 ZMMWORD PTR [rsp+0xa00],zmm2
vpaddq zmm5,zmm16,zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20c76]        # 97c0 <GCC_except_table830+0x1264>
vpandq zmm1,zmm17,zmm30
vpsraq zmm17,zmm5,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x1100],zmm5
vpbroadcastq zmm5,QWORD PTR [rip+0xfffffffffff20cff]        # 9868 <GCC_except_table830+0x130c>
vpsubq zmm16,zmm2,zmm1
vmovdqa64 ZMMWORD PTR [rsp+0x340],zmm1
vpandq zmm1,zmm3,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0xac0],zmm1
vpaddq zmm16,zmm16,zmm0
vpaddq zmm2,zmm16,zmm17
vpshldq zmm16,zmm14,zmm26,0x24
vmovdqa64 ZMMWORD PTR [rsp+0x1140],zmm2
vpandq zmm0,zmm16,zmm30
vpsraq zmm16,zmm2,0x34
vpsubq zmm3,zmm1,zmm0
vmovdqa64 ZMMWORD PTR [rsp+0x900],zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20c63]        # 9828 <GCC_except_table830+0x12cc>
vpsrlq zmm1,zmm15,0x10
vmovdqa64 ZMMWORD PTR [rsp+0x840],zmm1
vpaddq zmm3,zmm3,zmm0
vpsrlq zmm0,zmm14,0x10
vpaddq zmm2,zmm3,zmm16
vpsubq zmm3,zmm1,zmm0
vmovdqa64 ZMMWORD PTR [rsp+0x880],zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20c99]        # 9898 <GCC_except_table830+0x133c>
vmovdqu64 ymm16,YMMWORD PTR [rsi+0x40]
vpxor  xmm1,xmm1,xmm1
vpsraq zmm14,zmm2,0x34
vinserti64x4 zmm16,zmm16,YMMWORD PTR [r15+0x40],0x1
vmovdqa64 ZMMWORD PTR [rsp+0xe80],zmm2
vpbroadcastq zmm2,QWORD PTR [rip+0xfffffffffff20cb5]        # 98e0 <GCC_except_table830+0x1384>
vpaddq zmm3,zmm3,zmm0
vpaddq zmm0,zmm3,zmm14
vmovdqu32 zmm3{k1}{z},ZMMWORD PTR [r11+0x10]
kmovd  k1,eax
vmovdqa64 ZMMWORD PTR [rsp+0x1080],zmm0
vpmovsxbd xmm0,DWORD PTR [rip+0xfffffffffff216b6]        # a30c <GCC_except_table830+0x1db0>
vpinsrd xmm14,xmm0,DWORD PTR [r11+0x70],0x2
vpermd zmm3,zmm0,zmm3
vmovdqa ymm0,YMMWORD PTR [rip+0xfffffffffff21395]        # a000 <GCC_except_table830+0x1aa4>
vpinsrd xmm14,xmm14,ebx,0x3
vinserti128 ymm3,ymm3,xmm14,0x1
vmovdqu32 ymm14{k1}{z},YMMWORD PTR [r11+0x50]
vpermt2d ymm3,ymm0,ymm14
vpsrlvd ymm3,ymm3,YMMWORD PTR [rip+0xfffffffffff21270]        # 9f00 <GCC_except_table830+0x19a4>
vpand  ymm3,ymm3,YMMWORD PTR [rip+0xfffffffffff21288]        # 9f20 <GCC_except_table830+0x19c4>
vextracti128 xmm14,ymm3,0x1
vpor   xmm3,xmm14,xmm3
vpshufd xmm14,xmm3,0xee
vpor   xmm3,xmm14,xmm3
vpshufd xmm14,xmm3,0x55
vpor   xmm3,xmm14,xmm3
vmovdqu ymm14,YMMWORD PTR [r9+0x40]
vmovd  eax,xmm3
vmovdqu ymm3,YMMWORD PTR [r8+0x40]
vinserti64x4 zmm14,zmm14,YMMWORD PTR [r12+0x40],0x1
vinserti64x4 zmm3,zmm3,YMMWORD PTR [rdi+0x40],0x1
kmovd  k1,eax
lea    rax,[rsp+0xc00]
vpunpcklqdq zmm15,zmm3,zmm14
vpunpckhqdq zmm14,zmm3,zmm14
vmovdqu ymm3,YMMWORD PTR [rdx+0x40]
vinserti64x4 zmm3,zmm3,YMMWORD PTR [rcx+0x40],0x1
vmovdqa64 zmm18,zmm15
vpunpcklqdq zmm17,zmm3,zmm16
vpunpckhqdq zmm16,zmm3,zmm16
vmovdqa64 zmm3,zmm14
vpermt2q zmm15,zmm27,zmm17
vpermt2q zmm3,zmm27,zmm16
vpermt2q zmm14,zmm8,zmm16
vpermt2q zmm18,zmm8,zmm17
vpshldq zmm23,zmm3,zmm15,0x24
vpsllq zmm17,zmm3,0x22
vpshldq zmm15,zmm15,zmm14,0x18
vpandq zmm20,zmm18,zmm24
vpshldq zmm14,zmm14,zmm18,0xc
vpsrlq zmm9,zmm18,0x2
vpsrlq zmm3,zmm3,0x12
vpandq zmm17,zmm17,zmm5
vpmullq zmm0,zmm20,zmm25
vpsrlq zmm22,zmm23,0x2
vpsllq zmm16,zmm23,0x32
vpsrlq zmm23,zmm15,0x2
vpsllq zmm15,zmm15,0x32
vpternlogq zmm22,zmm17,zmm2,0xec
vpmullq zmm17,zmm20,zmm21
vpandq zmm16,zmm16,zmm5
vpandq zmm15,zmm15,zmm5
vpternlogq zmm23,zmm16,zmm2,0xec
vpsrlq zmm16,zmm14,0x2
vpsllq zmm14,zmm14,0x32
vpternlogq zmm16,zmm15,zmm2,0xec
vpandq zmm14,zmm14,zmm5
vpaddq zmm22,zmm17,zmm22
vpaddq zmm17,zmm0,zmm16
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20add]        # 98b0 <GCC_except_table830+0x1354>
vpternlogq zmm9,zmm14,zmm2,0xec
vpsubq zmm17{k1},zmm1,zmm17
vpsubq zmm22{k1},zmm1,zmm22
vpmullq zmm0,zmm20,zmm0
vpaddq zmm14,zmm0,zmm9
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff2095c]        # 9758 <GCC_except_table830+0x11fc>
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20ad2]        # 98d8 <GCC_except_table830+0x137c>
vpsubq zmm14{k1},zmm1,zmm14
vpblendmq zmm15{k1},zmm0,zmm9
vpandq zmm0,zmm13,zmm30
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff2098e]        # 97b0 <GCC_except_table830+0x1254>
vpshldq zmm13,zmm7,zmm13,0xc
vpshldq zmm7,zmm4,zmm7,0x18
vpshldq zmm4,zmm31,zmm4,0x24
vpaddq zmm14,zmm14,zmm15
vmovdqa64 ZMMWORD PTR [rsp+0x440],zmm0
vpsubq zmm14,zmm14,zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20b23]        # 9978 <GCC_except_table830+0x141c>
vpsraq zmm18,zmm14,0x34
vpblendmq zmm16{k1},zmm0,zmm9
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff209a4]        # 9810 <GCC_except_table830+0x12b4>
vpandq zmm0,zmm13,zmm30
vpaddq zmm17,zmm17,zmm16
vmovdqa64 ZMMWORD PTR [rsp+0x940],zmm0
vpaddq zmm17,zmm17,zmm18
vpsubq zmm13,zmm17,zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20aba]        # 9950 <GCC_except_table830+0x13f4>
vpmadd52luq zmm23,zmm20,zmm9
vpblendmq zmm17{k1},zmm0,zmm12
vpsraq zmm0,zmm13,0x34
vpbroadcastq zmm12,QWORD PTR [rip+0xfffffffffff20a4d]        # 9900 <GCC_except_table830+0x13a4>
vpsubq zmm23{k1},zmm1,zmm23
vpaddq zmm18,zmm23,zmm17
vpaddq zmm18,zmm18,zmm0
vpandq zmm0,zmm7,zmm30
vpsubq zmm7,zmm18,zmm0
vmovdqa64 ZMMWORD PTR [rsp+0x9c0],zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20995]        # 9878 <GCC_except_table830+0x131c>
vpsraq zmm23,zmm7,0x34
vpblendmq zmm18{k1},zmm0,zmm12
vpbroadcastq zmm12,QWORD PTR [rip+0xfffffffffff20866]        # 9760 <GCC_except_table830+0x1204>
vpandq zmm0,zmm4,zmm30
vpaddq zmm22,zmm22,zmm18
vmovdqa64 ZMMWORD PTR [rsp+0x980],zmm0
vpaddq zmm22,zmm22,zmm23
vmovdqu64 ymm23,YMMWORD PTR [r9+0x60]
vpsubq zmm4,zmm22,zmm0
vpbroadcastq zmm22,QWORD PTR [rip+0xfffffffffff209bd]        # 98e8 <GCC_except_table830+0x138c>
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff208bb]        # 97f0 <GCC_except_table830+0x1294>
vinserti64x4 zmm23,zmm23,YMMWORD PTR [r12+0x60],0x1
movabs r12,0x30644e72e131
vpmadd52luq zmm3,zmm20,zmm12
vpblendmq zmm20{k1},zmm0,zmm22
vpsraq zmm22,zmm4,0x34
vpsubq zmm3{k1},zmm1,zmm3
vpaddq zmm3,zmm3,zmm20
vpaddq zmm3,zmm3,zmm22
vmovdqu64 ymm22,YMMWORD PTR [r8+0x60]
vinserti64x4 zmm22,zmm22,YMMWORD PTR [rdi+0x60],0x1
lea    rdi,[rsp+0x700]
vpunpcklqdq zmm26,zmm22,zmm23
vpunpckhqdq zmm28,zmm22,zmm23
vmovdqu64 ymm23,YMMWORD PTR [rsi+0x60]
vmovdqu64 ymm22,YMMWORD PTR [rdx+0x60]
mov    sil,0x1
vinserti64x4 zmm22,zmm22,YMMWORD PTR [rcx+0x60],0x1
vinserti64x4 zmm23,zmm23,YMMWORD PTR [r15+0x60],0x1
vmovdqa64 zmm0,zmm26
lea    rcx,[rsp+0x5c0]
vpunpcklqdq zmm29,zmm22,zmm23
vpunpckhqdq zmm31,zmm22,zmm23
vmovdqa64 zmm22,zmm28
vpermt2q zmm22,zmm27,zmm31
vpermt2q zmm26,zmm27,zmm29
vpermt2q zmm0,zmm8,zmm29
vpermt2q zmm28,zmm8,zmm31
vpshldq zmm29,zmm22,zmm26,0x24
vpsllq zmm23,zmm22,0x22
vpshldq zmm26,zmm26,zmm28,0x18
vpshldq zmm8,zmm28,zmm0,0xc
vpandq zmm23,zmm23,zmm5
vpsrlq zmm27,zmm29,0x2
vpsrlq zmm28,zmm8,0x2
vpternlogq zmm27,zmm23,zmm2,0xec
vpandq zmm23,zmm0,zmm24
vpsrlq zmm0,zmm0,0x2
vpmullq zmm24,zmm23,zmm21
vpsllq zmm21,zmm29,0x32
vpsrlq zmm29,zmm26,0x2
vpsllq zmm26,zmm26,0x32
vpandq zmm26,zmm26,zmm5
vpternlogq zmm28,zmm26,zmm2,0xec
vpmullq zmm26,zmm23,zmm25
vpaddq zmm24,zmm24,zmm27
vpandq zmm27,zmm21,zmm5
vpbroadcastq zmm21,QWORD PTR [rip+0xfffffffffff208d7]        # 9948 <GCC_except_table830+0x13ec>
vpternlogq zmm29,zmm27,zmm2,0xec
vpsllq zmm27,zmm8,0x32
vpshldq zmm8,zmm6,zmm10,0x18
vpsubq zmm24{k1},zmm1,zmm24
vpandq zmm27,zmm27,zmm5
vpaddq zmm26,zmm26,zmm28
vpmadd52luq zmm29,zmm23,zmm9
vpsrlq zmm5,zmm22,0x12
vpandq zmm22,zmm30,ZMMWORD PTR [rsp+0xf80]
vpxor  xmm9,xmm9,xmm9
vpandq zmm28,zmm7,zmm30
vpternlogq zmm0,zmm27,zmm2,0xec
vpmullq zmm2,zmm23,QWORD BCST [rip+0xfffffffffff207e7]        # 98b0 <GCC_except_table830+0x1354>
vpsubq zmm26{k1},zmm1,zmm26
vpmadd52luq zmm5,zmm23,zmm12
vpandq zmm12,zmm30,ZMMWORD PTR [rsp+0x1100]
vpxord xmm23,xmm23,xmm23
vpsubq zmm29{k1},zmm1,zmm29
vpsubq zmm5{k1},zmm1,zmm5
vpaddq zmm0,zmm2,zmm0
vpandq zmm2,zmm11,zmm30
vpshldq zmm11,zmm10,zmm11,0xc
vmovdqa64 zmm10,ZMMWORD PTR [rsp+0xb00]
vpsubq zmm0{k1},zmm1,zmm0
vmovdqa64 ZMMWORD PTR [rsp+0x1800],zmm2
vpshldq zmm1,zmm19,zmm6,0x24
vpxor  xmm6,xmm6,xmm6
vpmadd52huq zmm6,zmm28,zmm22
vpaddq zmm0,zmm0,zmm15
vpaddq zmm15,zmm26,zmm16
vpsubq zmm26,zmm0,zmm2
vpandq zmm2,zmm11,zmm30
vpaddq zmm11,zmm24,zmm18
vpandq zmm1,zmm1,zmm30
vmovdqa64 zmm24,ZMMWORD PTR [rsp+0xf00]
vpsraq zmm0,zmm26,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x17c0],zmm2
vmovdqa64 ZMMWORD PTR [rsp+0x1740],zmm1
vpaddq zmm0,zmm15,zmm0
vpaddq zmm15,zmm29,zmm17
vpandq zmm29,zmm26,zmm30
vpxord xmm26,xmm26,xmm26
vpsubq zmm27,zmm0,zmm2
vpandq zmm2,zmm8,zmm30
vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x3c0]
vmovdqa64 ZMMWORD PTR [rsp+0x1840],zmm29
vpsraq zmm0,zmm27,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x1780],zmm2
vpaddq zmm0,zmm15,zmm0
vpandq zmm15,zmm27,zmm30
vpsubq zmm16,zmm0,zmm2
vpaddq zmm2,zmm5,zmm20
vpbroadcastq zmm5,QWORD PTR [rip+0xfffffffffff205cf]        # 97a0 <GCC_except_table830+0x1244>
vpsraq zmm0,zmm16,0x34
vpaddq zmm0,zmm11,zmm0
vpsubq zmm17,zmm0,zmm1
vpandq zmm1,zmm10,zmm30
vpsraq zmm0,zmm17,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x580],zmm1
vpandq zmm20,zmm17,zmm30
vpandq zmm17,zmm16,zmm30
vpaddq zmm18,zmm2,zmm0
vpandq zmm0,zmm8,zmm30
vpandq zmm2,zmm14,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0x400],zmm0
vpsubq zmm0,zmm1,zmm0
vmovdqa ymm1,YMMWORD PTR [rip+0xfffffffffff20c33]        # 9e60 <GCC_except_table830+0x1904>
vpmadd52huq zmm9,zmm2,zmm22
vpaddq zmm11,zmm0,zmm5
vpandq zmm0,zmm13,zmm30
vpandq zmm31,zmm11,zmm30
vpmadd52luq zmm9,zmm0,zmm22
vmovdqu YMMWORD PTR [rsp+0x5c0],ymm1
vpxor  xmm1,xmm1,xmm1
vpmadd52luq zmm1,zmm2,zmm22
mov    QWORD PTR [rsp+0x5e0],r12
vpmadd52huq zmm9,zmm29,zmm31
vpbroadcastq zmm25,QWORD PTR [rsp+0x5c8]
vpbroadcastq zmm27,QWORD PTR [rsp+0x5d8]
vpbroadcastq zmm16,QWORD PTR [rsp+0x5e0]
vpmadd52luq zmm1,zmm29,zmm31
vpmadd52luq zmm9,zmm15,zmm31
vpmadd52luq zmm26,zmm1,zmm21
vpaddq zmm1,zmm1,zmm30
vpsrlq zmm1,zmm1,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x1a00],zmm16
vmovdqa64 ZMMWORD PTR [rsp+0x1a40],zmm27
vpmadd52huq zmm1,zmm26,zmm5
vpmadd52luq zmm9,zmm26,zmm25
vpsrlq zmm5,zmm19,0x10
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0xf40]
vmovdqa64 ZMMWORD PTR [rsp+0x1700],zmm5
vpaddq zmm13,zmm9,zmm1
vpsrlq zmm1,ZMMWORD PTR [rsp+0x380],0x10
vpandq zmm9,zmm4,zmm30
vpxor  xmm4,xmm4,xmm4
vpmadd52luq zmm6,zmm9,zmm22
vpmadd52huq zmm4,zmm9,zmm22
vpmadd52luq zmm13,zmm2,zmm12
vpmadd52huq zmm6,zmm17,zmm31
vmovdqa64 ZMMWORD PTR [rsp+0x380],zmm1
vpsubq zmm1,zmm3,zmm1
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff20430]        # 9758 <GCC_except_table830+0x11fc>
vpandq zmm7,zmm1,zmm30
vpmadd52luq zmm6,zmm20,zmm31
vpmadd52luq zmm4,zmm7,zmm22
vpmadd52huq zmm23,zmm7,zmm22
vpmadd52huq zmm4,zmm20,zmm31
vpsubq zmm3,zmm3,zmm2
vmovdqa64 ZMMWORD PTR [rsp+0x19c0],zmm3
vpsraq zmm3,zmm3,0x34
vpsubq zmm3,zmm3,zmm0
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff20445]        # 97b0 <GCC_except_table830+0x1254>
vmovdqa64 ZMMWORD PTR [rsp+0x1980],zmm3
vpsraq zmm3,zmm3,0x34
vpsubq zmm3,zmm3,zmm28
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff204f6]        # 9880 <GCC_except_table830+0x1324>
vmovdqa64 ZMMWORD PTR [rsp+0x1940],zmm3
vpsraq zmm3,zmm3,0x34
vpsubq zmm3,zmm3,zmm9
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff20557]        # 9900 <GCC_except_table830+0x13a4>
vmovdqa64 ZMMWORD PTR [rsp+0x18c0],zmm3
vpsraq zmm3,zmm3,0x34
vpsubq zmm3,zmm3,zmm1
vpsubq zmm1,zmm18,zmm5
vpbroadcastq zmm18,QWORD PTR [rsp+0x5d0]
vmovdqa64 ZMMWORD PTR [rsp+0x1900],zmm3
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm0,zmm22
vpandq zmm14,zmm1,zmm30
vpshldq zmm1,zmm19,zmm8,0xc
vpmadd52luq zmm4,zmm14,zmm31
vpmadd52huq zmm23,zmm14,zmm31
vpandq zmm5,zmm1,zmm30
vpshldq zmm1,zmm24,zmm10,0xc
vmovdqa64 ZMMWORD PTR [rsp+0x3c0],zmm5
vpmadd52luq zmm3,zmm28,zmm22
vpandq zmm8,zmm1,zmm30
vpsraq zmm1,zmm11,0x34
vpmadd52huq zmm4,zmm26,zmm27
vpaddq zmm1,zmm1,zmm8
vpmadd52huq zmm23,zmm26,zmm16
vmovdqa64 ZMMWORD PTR [rsp+0xb00],zmm8
vpsubq zmm1,zmm1,zmm5
vpaddq zmm10,zmm1,QWORD BCST [rip+0xfffffffffff20544]        # 9990 <GCC_except_table830+0x1434>
vpmadd52huq zmm6,zmm26,zmm18
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm3,zmm15,zmm31
vpmadd52luq zmm4,zmm26,zmm16
vpmadd52huq zmm23,zmm9,zmm12
vpmadd52luq zmm6,zmm26,zmm27
vpmadd52luq zmm3,zmm17,zmm31
vpmadd52huq zmm4,zmm28,zmm12
vpmadd52luq zmm23,zmm7,zmm12
vpandq zmm8,zmm10,zmm30
vpmadd52huq zmm6,zmm0,zmm12
vpmadd52luq zmm13,zmm29,zmm8
vpmadd52huq zmm3,zmm26,zmm25
vpmadd52luq zmm4,zmm9,zmm12
vpmadd52huq zmm23,zmm20,zmm8
vpmadd52luq zmm6,zmm28,zmm12
vpmadd52luq zmm1,zmm13,zmm21
vpxord xmm21,xmm21,xmm21
vpmadd52huq zmm21,zmm7,zmm12
vpaddq zmm11,zmm13,zmm30
vmovdqa64 zmm13,zmm25
vmovdqa64 ZMMWORD PTR [rsp+0xf80],zmm13
vpmadd52luq zmm3,zmm26,zmm18
vmovdqa64 zmm26,zmm18
vpsrlq zmm11,zmm11,0x34
vpmadd52huq zmm11,zmm1,QWORD BCST [rip+0xfffffffffff202b3]        # 97a0 <GCC_except_table830+0x1244>
vmovdqa64 ZMMWORD PTR [rsp+0x1340],zmm26
vpmadd52huq zmm4,zmm17,zmm8
vpmadd52luq zmm23,zmm14,zmm8
vpmadd52huq zmm6,zmm15,zmm8
vpmadd52huq zmm21,zmm14,zmm8
vpmadd52huq zmm3,zmm2,zmm12
vpmadd52luq zmm4,zmm20,zmm8
vpmadd52huq zmm23,zmm1,zmm27
vpmadd52luq zmm6,zmm17,zmm8
vpmadd52huq zmm21,zmm1,zmm16
vpmadd52luq zmm3,zmm0,zmm12
vpmadd52huq zmm4,zmm1,zmm18
vpmadd52luq zmm23,zmm1,zmm16
vpmadd52huq zmm6,zmm1,zmm25
vpmadd52huq zmm3,zmm29,zmm8
vpmadd52luq zmm4,zmm1,zmm27
vpmadd52luq zmm6,zmm1,zmm18
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0xbc0]
vpmadd52luq zmm3,zmm15,zmm8
vpmadd52luq zmm3,zmm1,zmm25
vpandq zmm25,zmm30,ZMMWORD PTR [rsp+0x1140]
vpshldq zmm1,zmm18,zmm19,0x18
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0x10c0]
vpaddq zmm11,zmm3,zmm11
vpandq zmm3,zmm1,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0xf40],zmm3
vpmadd52huq zmm6,zmm2,zmm25
vpmadd52luq zmm11,zmm2,zmm25
vpmadd52huq zmm4,zmm0,zmm25
vpmadd52huq zmm23,zmm28,zmm25
vpmadd52huq zmm21,zmm9,zmm25
vpshldq zmm1,zmm19,zmm24,0x18
vpmadd52luq zmm6,zmm0,zmm25
vpmadd52luq zmm4,zmm28,zmm25
vpmadd52luq zmm23,zmm9,zmm25
vpmadd52luq zmm21,zmm7,zmm25
vpandq zmm5,zmm1,zmm30
vpsraq zmm1,zmm10,0x34
vpxor  xmm10,xmm10,xmm10
vpaddq zmm1,zmm1,zmm5
vmovdqa64 ZMMWORD PTR [rsp+0xf00],zmm5
vmovdqa64 zmm5,ZMMWORD PTR [rsp+0x1c0]
vpsubq zmm1,zmm1,zmm3
vpaddq zmm3,zmm1,QWORD BCST [rip+0xfffffffffff201b7]        # 97c0 <GCC_except_table830+0x1264>
vpshldq zmm5,zmm5,zmm18,0x24
vpandq zmm24,zmm3,zmm30
vpsraq zmm3,zmm3,0x34
vpmadd52luq zmm11,zmm29,zmm24
vpmadd52luq zmm10,zmm11,QWORD BCST [rip+0xfffffffffff2031b]        # 9948 <GCC_except_table830+0x13ec>
vpmadd52huq zmm6,zmm29,zmm24
vpmadd52huq zmm4,zmm15,zmm24
vpmadd52huq zmm23,zmm17,zmm24
vpmadd52huq zmm21,zmm20,zmm24
vpaddq zmm1,zmm11,zmm30
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff2014b]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52luq zmm6,zmm15,zmm24
vpmadd52luq zmm4,zmm17,zmm24
vpmadd52luq zmm23,zmm20,zmm24
vpmadd52luq zmm21,zmm14,zmm24
vpsrlq zmm1,zmm1,0x34
vpmadd52luq zmm6,zmm10,zmm13
vpmadd52huq zmm4,zmm10,zmm13
vpmadd52huq zmm23,zmm10,zmm26
vpmadd52huq zmm21,zmm10,zmm27
vpmadd52huq zmm1,zmm10,zmm11
vpmadd52luq zmm4,zmm10,zmm26
vpmadd52luq zmm23,zmm10,zmm27
vpmadd52luq zmm21,zmm10,zmm16
vpaddq zmm6,zmm6,zmm1
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm1,zmm7,zmm25
vpmadd52huq zmm1,zmm14,zmm24
vpmadd52huq zmm1,zmm10,zmm16
vpandq zmm10,zmm5,zmm30
vmovdqa64 zmm5,ZMMWORD PTR [rsp+0x180]
vmovdqa64 ZMMWORD PTR [rsp+0xbc0],zmm10
vpshldq zmm5,zmm5,zmm19,0x24
vpandq zmm19,zmm30,ZMMWORD PTR [rsp+0xe80]
vpandq zmm5,zmm5,zmm30
vpaddq zmm3,zmm3,zmm5
vmovdqa64 ZMMWORD PTR [rsp+0x10c0],zmm5
vpsubq zmm3,zmm3,zmm10
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff2011f]        # 9828 <GCC_except_table830+0x12cc>
vmovdqa64 zmm10,ZMMWORD PTR [rsp+0x1900]
vpaddq zmm10,zmm10,QWORD BCST [rip+0xfffffffffff201cd]        # 98e8 <GCC_except_table830+0x138c>
vpmadd52huq zmm4,zmm2,zmm19
vpmadd52luq zmm6,zmm2,zmm19
vpmadd52huq zmm1,zmm9,zmm19
vpmadd52huq zmm23,zmm0,zmm19
vpmadd52huq zmm21,zmm28,zmm19
vpmadd52luq zmm4,zmm0,zmm19
vpandq zmm18,zmm3,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0x1140],zmm3
vpxor  xmm3,xmm3,xmm3
vpmadd52luq zmm1,zmm7,zmm19
vpmadd52luq zmm23,zmm28,zmm19
vpmadd52luq zmm21,zmm9,zmm19
vpmadd52luq zmm6,zmm29,zmm18
vpmadd52luq zmm3,zmm6,QWORD BCST [rip+0xfffffffffff201d5]        # 9948 <GCC_except_table830+0x13ec>
vpandq zmm10,zmm10,zmm30
vpmadd52huq zmm4,zmm29,zmm18
vpmadd52huq zmm1,zmm20,zmm18
vpmadd52huq zmm23,zmm15,zmm18
vpmadd52huq zmm21,zmm17,zmm18
vpaddq zmm5,zmm6,zmm30
vpandq zmm6,zmm30,ZMMWORD PTR [rsp+0x1980]
vpsrlq zmm5,zmm5,0x34
vpmadd52luq zmm4,zmm15,zmm18
vpmadd52luq zmm1,zmm14,zmm18
vpmadd52luq zmm23,zmm17,zmm18
vpmadd52luq zmm21,zmm20,zmm18
vpmadd52huq zmm5,zmm3,zmm11
vpxor  xmm11,xmm11,xmm11
vpmadd52luq zmm11,zmm29,zmm22
vpmadd52luq zmm4,zmm3,zmm13
vpmadd52huq zmm1,zmm3,zmm27
vpmadd52huq zmm23,zmm3,zmm13
vpmadd52huq zmm21,zmm3,zmm26
vmovaps ymm13,YMMWORD PTR [rip+0xfffffffffff20671]        # 9e60 <GCC_except_table830+0x1904>
vpaddq zmm4,zmm4,zmm5
vpxor  xmm5,xmm5,xmm5
vpmadd52huq zmm5,zmm7,zmm19
vpmadd52luq zmm1,zmm3,zmm16
vpmadd52luq zmm23,zmm3,zmm26
vpmadd52luq zmm21,zmm3,zmm27
vpxord xmm27,xmm27,xmm27
vpmadd52huq zmm27,zmm29,zmm22
vpxord xmm26,xmm26,xmm26
vpmadd52huq zmm26,zmm15,zmm22
vpmadd52huq zmm5,zmm14,zmm18
vpmadd52luq zmm26,zmm17,zmm22
vmovups YMMWORD PTR [rsp+0x5c0],ymm13
vpmadd52luq zmm27,zmm15,zmm22
vpbroadcastq zmm13,QWORD PTR [rip+0xfffffffffff200fa]        # 9948 <GCC_except_table830+0x13ec>
mov    QWORD PTR [rsp+0x5e0],r12
vpmadd52huq zmm5,zmm3,zmm16
vpandq zmm16,zmm30,ZMMWORD PTR [rsp+0x1080]
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm17,zmm22
vpmadd52huq zmm26,zmm6,zmm31
vpmadd52luq zmm3,zmm20,zmm22
vpmadd52huq zmm23,zmm2,zmm16
vpmadd52luq zmm4,zmm2,zmm16
vpmadd52huq zmm21,zmm0,zmm16
vpmadd52huq zmm1,zmm28,zmm16
vpmadd52huq zmm5,zmm9,zmm16
vpxor  xmm2,xmm2,xmm2
vpmadd52huq zmm2,zmm14,zmm22
vpmadd52luq zmm23,zmm0,zmm16
vpxor  xmm0,xmm0,xmm0
vpmadd52huq zmm0,zmm7,zmm16
vmovdqa64 ZMMWORD PTR [rsp+0x1880],zmm4
vpandq zmm4,zmm30,ZMMWORD PTR [rsp+0x19c0]
vpmadd52luq zmm1,zmm9,zmm16
vpmadd52luq zmm5,zmm7,zmm16
vpandq zmm7,zmm30,ZMMWORD PTR [rsp+0x1940]
vpandq zmm9,zmm30,ZMMWORD PTR [rsp+0x18c0]
vpmadd52huq zmm2,zmm10,zmm31
vpmadd52luq zmm21,zmm28,zmm16
vpbroadcastq zmm28,QWORD PTR [rsp+0x5e0]
vmovdqa64 ZMMWORD PTR [rsp+0x1100],zmm0
vpxor  xmm0,xmm0,xmm0
vpmadd52huq zmm0,zmm20,zmm22
vpmadd52luq zmm0,zmm14,zmm22
vpmadd52huq zmm27,zmm4,zmm31
vpmadd52huq zmm3,zmm7,zmm31
vpmadd52luq zmm11,zmm4,zmm31
vpmadd52luq zmm26,zmm7,zmm31
vpbroadcastq zmm22,QWORD PTR [rip+0xfffffffffff1fe71]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52huq zmm0,zmm9,zmm31
vpmadd52luq zmm27,zmm6,zmm31
vpmadd52luq zmm3,zmm9,zmm31
vpmadd52luq zmm0,zmm10,zmm31
vpxord xmm31,xmm31,xmm31
vpmadd52luq zmm31,zmm11,zmm13
vpbroadcastq zmm13,QWORD PTR [rsp+0x5c8]
vpaddq zmm11,zmm11,zmm30
vpsrlq zmm11,zmm11,0x34
vpmadd52huq zmm11,zmm31,zmm22
vpbroadcastq zmm22,QWORD PTR [rsp+0x5d0]
vpmadd52huq zmm2,zmm31,zmm28
vpmadd52luq zmm27,zmm31,zmm13
vpmadd52huq zmm26,zmm31,zmm13
vpmadd52huq zmm2,zmm20,zmm12
vpaddq zmm27,zmm27,zmm11
vpbroadcastq zmm11,QWORD PTR [rsp+0x5d8]
vpmadd52huq zmm3,zmm31,zmm22
vpmadd52luq zmm26,zmm31,zmm22
vpmadd52luq zmm2,zmm14,zmm12
vpmadd52luq zmm27,zmm29,zmm12
vpmadd52huq zmm26,zmm29,zmm12
vpmadd52huq zmm2,zmm9,zmm8
vmovdqa64 ymm29,YMMWORD PTR [rip+0xfffffffffff2048d]        # 9e60 <GCC_except_table830+0x1904>
vpmadd52luq zmm27,zmm4,zmm8
vpmadd52huq zmm0,zmm31,zmm11
vpmadd52luq zmm3,zmm31,zmm11
vpmadd52luq zmm26,zmm15,zmm12
vpmadd52luq zmm2,zmm10,zmm8
vpmadd52luq zmm0,zmm31,zmm28
vpmadd52huq zmm3,zmm15,zmm12
vpxord xmm31,xmm31,xmm31
vpmadd52huq zmm31,zmm14,zmm12
vpmadd52huq zmm26,zmm4,zmm8
vpmadd52huq zmm0,zmm17,zmm12
vpmadd52luq zmm3,zmm17,zmm12
vpmadd52luq zmm26,zmm6,zmm8
vpmadd52huq zmm31,zmm10,zmm8
vpmadd52luq zmm0,zmm20,zmm12
vpbroadcastq zmm12,QWORD PTR [rip+0xfffffffffff1ff11]        # 9948 <GCC_except_table830+0x13ec>
vpmadd52huq zmm3,zmm6,zmm8
vpmadd52huq zmm0,zmm7,zmm8
vpmadd52luq zmm3,zmm7,zmm8
vpmadd52luq zmm0,zmm9,zmm8
vpxor  xmm8,xmm8,xmm8
vpmadd52luq zmm8,zmm27,zmm12
vpaddq zmm12,zmm27,zmm30
vpbroadcastq zmm27,QWORD PTR [rip+0xfffffffffff1fd36]        # 97a0 <GCC_except_table830+0x1244>
vpsrlq zmm12,zmm12,0x34
vpmadd52huq zmm3,zmm8,zmm13
vpmadd52huq zmm0,zmm8,zmm22
vpmadd52huq zmm2,zmm8,zmm11
vpmadd52huq zmm31,zmm8,zmm28
vpmadd52luq zmm26,zmm8,zmm13
vpmadd52huq zmm12,zmm8,zmm27
vmovdqa64 zmm27,ZMMWORD PTR [rsp+0x1840]
vpmadd52luq zmm3,zmm8,zmm22
vpmadd52luq zmm0,zmm8,zmm11
vpmadd52luq zmm2,zmm8,zmm28
vpmadd52huq zmm31,zmm20,zmm25
vpxor  xmm8,xmm8,xmm8
vpmadd52huq zmm8,zmm14,zmm25
vpmadd52huq zmm0,zmm15,zmm25
vpmadd52huq zmm2,zmm17,zmm25
vpaddq zmm12,zmm26,zmm12
vpmadd52luq zmm31,zmm14,zmm25
vpbroadcastq zmm26,QWORD PTR [rip+0xfffffffffff1fe66]        # 9948 <GCC_except_table830+0x13ec>
vpmadd52huq zmm8,zmm10,zmm24
vpmadd52huq zmm3,zmm27,zmm25
vpmadd52luq zmm12,zmm27,zmm25
vpmadd52luq zmm0,zmm17,zmm25
vpmadd52luq zmm2,zmm20,zmm25
vpmadd52huq zmm31,zmm9,zmm24
vpmadd52luq zmm3,zmm15,zmm25
vpmadd52huq zmm0,zmm6,zmm24
vpmadd52huq zmm2,zmm7,zmm24
vpmadd52luq zmm12,zmm4,zmm24
vpmadd52luq zmm31,zmm10,zmm24
vpbroadcastq zmm25,QWORD PTR [rip+0xfffffffffff1fc72]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52huq zmm3,zmm4,zmm24
vpmadd52luq zmm0,zmm7,zmm24
vpmadd52luq zmm2,zmm9,zmm24
vpmadd52luq zmm3,zmm6,zmm24
vpxord xmm24,xmm24,xmm24
vpmadd52luq zmm24,zmm12,zmm26
vpaddq zmm12,zmm12,zmm30
vpsrlq zmm12,zmm12,0x34
vpmadd52huq zmm2,zmm24,zmm22
vpmadd52huq zmm31,zmm24,zmm11
vpmadd52huq zmm0,zmm24,zmm13
vpmadd52huq zmm8,zmm24,zmm28
vpmadd52huq zmm12,zmm24,zmm25
vpmadd52luq zmm3,zmm24,zmm13
vpmadd52luq zmm2,zmm24,zmm11
vpmadd52luq zmm31,zmm24,zmm28
vpmadd52luq zmm0,zmm24,zmm22
vpmadd52huq zmm8,zmm20,zmm19
vpaddq zmm3,zmm3,zmm12
vpxord xmm24,xmm24,xmm24
vpmadd52huq zmm24,zmm14,zmm19
vpxor  xmm12,xmm12,xmm12
vpmadd52luq zmm3,zmm27,zmm19
vpmadd52huq zmm2,zmm15,zmm19
vpmadd52huq zmm31,zmm17,zmm19
vpmadd52huq zmm0,zmm27,zmm19
vpmadd52luq zmm8,zmm14,zmm19
vpmadd52huq zmm24,zmm10,zmm18
vpmadd52luq zmm3,zmm4,zmm18
vpmadd52luq zmm2,zmm17,zmm19
vpmadd52luq zmm31,zmm20,zmm19
vpmadd52luq zmm0,zmm15,zmm19
vpmadd52huq zmm8,zmm9,zmm18
vpsraq zmm19,ZMMWORD PTR [rsp+0x1140],0x34
vpmadd52luq zmm12,zmm3,zmm26
vpaddq zmm3,zmm3,zmm30
vpsrlq zmm3,zmm3,0x34
vpmadd52huq zmm2,zmm6,zmm18
vpmadd52huq zmm31,zmm7,zmm18
vpmadd52huq zmm0,zmm4,zmm18
vpmadd52luq zmm8,zmm10,zmm18
vpmadd52huq zmm3,zmm12,zmm25
vpmadd52huq zmm24,zmm12,zmm28
vpmadd52luq zmm2,zmm7,zmm18
vpmadd52luq zmm31,zmm9,zmm18
vpmadd52luq zmm0,zmm6,zmm18
vpmadd52huq zmm8,zmm12,zmm11
vmovdqa64 zmm18,zmm25
vpsrlq zmm25,ZMMWORD PTR [rsp+0x180],0x10
vpmadd52huq zmm24,zmm20,zmm16
vpmadd52huq zmm2,zmm12,zmm13
vpmadd52huq zmm31,zmm12,zmm22
vpmadd52luq zmm0,zmm12,zmm13
vpmadd52luq zmm8,zmm12,zmm28
vpmadd52luq zmm24,zmm14,zmm16
vpmadd52luq zmm2,zmm12,zmm22
vpmadd52luq zmm31,zmm12,zmm11
vpsrlq zmm12,ZMMWORD PTR [rsp+0x1c0],0x10
vpaddq zmm0,zmm0,zmm3
vpaddq zmm3,zmm19,zmm25
vmovdqa64 ZMMWORD PTR [rsp+0xe80],zmm25
vmovdqa64 zmm25,ZMMWORD PTR [rsp+0x1880]
vpmadd52huq zmm8,zmm17,zmm16
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0xfc0]
vpmadd52luq zmm0,zmm27,zmm16
vpmadd52huq zmm2,zmm27,zmm16
vpmadd52huq zmm31,zmm15,zmm16
vpmadd52luq zmm8,zmm20,zmm16
vpsubq zmm3,zmm3,zmm12
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff1fbb2]        # 9898 <GCC_except_table830+0x133c>
vmovdqa64 ZMMWORD PTR [rsp+0x1080],zmm12
vpmadd52luq zmm2,zmm15,zmm16
vpmadd52luq zmm31,zmm17,zmm16
vpandq zmm12,zmm3,zmm30
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm14,zmm16
vmovdqa64 zmm16,ZMMWORD PTR [rsp+0x1180]
vpmadd52huq zmm23,zmm27,zmm12
vpmadd52luq zmm25,zmm27,zmm12
vmovdqa64 zmm27,ZMMWORD PTR [rsp+0x1100]
vpmadd52huq zmm21,zmm15,zmm12
vpmadd52huq zmm1,zmm17,zmm12
vpmadd52huq zmm5,zmm20,zmm12
vpmadd52huq zmm2,zmm4,zmm12
vpmadd52huq zmm31,zmm6,zmm12
vpmadd52huq zmm8,zmm7,zmm12
vpmadd52huq zmm24,zmm9,zmm12
vpmadd52luq zmm0,zmm4,zmm12
vpxor  xmm4,xmm4,xmm4
vpmadd52huq zmm3,zmm10,zmm12
vpmadd52luq zmm23,zmm15,zmm12
vpmadd52luq zmm21,zmm17,zmm12
vpmadd52luq zmm2,zmm6,zmm12
vpmadd52luq zmm31,zmm7,zmm12
vpmadd52luq zmm8,zmm9,zmm12
vpmadd52luq zmm24,zmm10,zmm12
vpmadd52luq zmm1,zmm20,zmm12
vpmadd52luq zmm5,zmm14,zmm12
vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x2c0]
vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x240]
vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x13c0]
vmovdqa64 zmm10,ZMMWORD PTR [rsp+0x300]
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0xec0]
vmovdqa64 zmm17,zmm18
vpmadd52huq zmm27,zmm14,zmm12
vmovdqa64 zmm12,ZMMWORD PTR [rsp+0x1000]
vmovdqa64 zmm14,ZMMWORD PTR [rsp+0x1380]
vpmadd52luq zmm4,zmm12,zmm26
vpmadd52huq zmm14,zmm4,zmm9
vpmadd52huq zmm6,zmm4,zmm7
vpmadd52huq zmm15,zmm4,zmm10
vpmadd52luq zmm16,zmm4,zmm7
vpmadd52luq zmm14,zmm4,zmm10
vpmadd52luq zmm6,zmm4,zmm9
vmovdqa64 zmm9,zmm14
vmovdqa64 zmm14,ZMMWORD PTR [rsp+0x1040]
vmovdqa64 zmm7,zmm6
vpaddq zmm6,zmm12,zmm30
vpxor  xmm12,xmm12,xmm12
vpmadd52luq zmm12,zmm25,zmm26
vpsrlq zmm6,zmm6,0x34
vpmadd52huq zmm6,zmm4,zmm18
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0x1340]
vpmadd52luq zmm15,zmm4,zmm14
vpmadd52huq zmm19,zmm4,zmm14
vpaddq zmm4,zmm16,zmm6
vpxord xmm16,xmm16,xmm16
vpmadd52luq zmm16,zmm0,zmm26
vpaddq zmm0,zmm0,zmm30
vpsrlq zmm6,zmm4,0x34
vpsrlq zmm0,zmm0,0x34
vpaddq zmm6,zmm6,zmm7
vpmadd52huq zmm1,zmm12,zmm18
vpsrlq zmm7,zmm6,0x34
vmovdqa64 zmm10,zmm15
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0xf80]
vpaddq zmm7,zmm7,zmm9
vpmadd52huq zmm31,zmm16,zmm13
vpmadd52huq zmm8,zmm16,zmm22
vpmadd52luq zmm2,zmm16,zmm13
vpmadd52huq zmm0,zmm16,zmm17
vpmadd52huq zmm24,zmm16,zmm11
vpmadd52huq zmm3,zmm16,zmm28
vpsrlq zmm9,zmm7,0x34
vpaddq zmm10,zmm9,zmm10
vpsrlq zmm9,zmm10,0x34
vpaddq zmm9,zmm9,zmm19
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0x1a00]
vpmadd52luq zmm31,zmm16,zmm22
vpaddq zmm0,zmm2,zmm0
vpmadd52luq zmm8,zmm16,zmm11
vpmadd52luq zmm24,zmm16,zmm28
vmovdqa64 zmm28,zmm17
vpsrlq zmm2,zmm0,0x34
vpandq zmm0,zmm0,zmm30
vpmadd52huq zmm21,zmm12,zmm15
vpmadd52luq zmm23,zmm12,zmm15
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0x1a40]
vpaddq zmm2,zmm2,zmm31
vpsrlq zmm11,zmm2,0x34
vpaddq zmm8,zmm11,zmm8
vpsrlq zmm11,zmm8,0x34
vpmadd52luq zmm21,zmm12,zmm18
vmovdqa64 zmm18,zmm27
vpmadd52huq zmm18,zmm12,zmm19
vpaddq zmm11,zmm11,zmm24
vpsrlq zmm13,zmm11,0x34
vpaddq zmm3,zmm13,zmm3
vpmadd52huq zmm5,zmm12,zmm15
vpmadd52luq zmm1,zmm12,zmm15
vpaddq zmm15,zmm25,zmm30
vpsrlq zmm14,zmm15,0x34
vpmadd52huq zmm14,zmm12,zmm17
vpmadd52luq zmm5,zmm12,zmm19
vpaddq zmm12,zmm23,zmm14
vmovdqa64 zmm23,zmm26
vpsrlq zmm14,zmm12,0x34
vpandq zmm12,zmm12,zmm30
vpaddq zmm14,zmm14,zmm21
vmovdqa64 ZMMWORD PTR [rsp+0xc00],zmm12
vpsrlq zmm15,zmm14,0x34
vpandq zmm12,zmm14,zmm30
vpaddq zmm1,zmm15,zmm1
vmovdqa64 ZMMWORD PTR [rsp+0xc40],zmm12
vpsrlq zmm15,zmm1,0x34
vpandq zmm1,zmm1,zmm30
vpaddq zmm5,zmm15,zmm5
vmovdqa64 ZMMWORD PTR [rsp+0xc80],zmm1
vpsrlq zmm15,zmm5,0x34
vpandq zmm1,zmm5,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0xcc0],zmm1
vpaddq zmm15,zmm15,zmm18
vpandq zmm1,zmm2,zmm30
vpandq zmm2,zmm8,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0xd00],zmm15
vmovdqa64 ZMMWORD PTR [rsp+0xd40],zmm0
vmovdqa64 ZMMWORD PTR [rsp+0xd80],zmm1
vpandq zmm1,zmm11,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0xdc0],zmm2
vpandq zmm0,zmm4,zmm30
vpandq zmm2,zmm7,zmm30
vpandq zmm4,zmm10,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0xe00],zmm1
vpandq zmm1,zmm6,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0xe40],zmm3
nop    DWORD PTR [rax+0x0]
vmovdqa64 zmm15,ZMMWORD PTR [rax]
vmovdqa64 zmm14,ZMMWORD PTR [rax+0x40]
vmovdqa64 zmm13,ZMMWORD PTR [rax+0x80]
vmovdqa64 zmm10,ZMMWORD PTR [rax+0xc0]
vmovdqa64 zmm11,ZMMWORD PTR [rax+0x100]
vmovdqu64 YMMWORD PTR [rsp+0x500],ymm29
mov    QWORD PTR [rsp+0x520],r12
vpxor  xmm3,xmm3,xmm3
vpxor  xmm12,xmm12,xmm12
vpxord xmm19,xmm19,xmm19
vpxord xmm17,xmm17,xmm17
vpxord xmm20,xmm20,xmm20
vpxord xmm21,xmm21,xmm21
vpxord xmm16,xmm16,xmm16
mov    edx,esi
add    rax,0x140
xor    esi,esi
vpbroadcastq zmm8,QWORD PTR [rsp+0x508]
vpbroadcastq zmm7,QWORD PTR [rsp+0x510]
vpbroadcastq zmm6,QWORD PTR [rsp+0x518]
vpbroadcastq zmm5,QWORD PTR [rsp+0x520]
vpmadd52huq zmm12,zmm14,zmm0
vpmadd52huq zmm19,zmm13,zmm0
vpmadd52huq zmm17,zmm10,zmm0
vpmadd52luq zmm3,zmm15,zmm0
vpmadd52huq zmm20,zmm15,zmm0
vpmadd52huq zmm16,zmm11,zmm0
vpmadd52luq zmm21,zmm3,zmm23
vpmadd52luq zmm12,zmm13,zmm0
vpmadd52luq zmm19,zmm10,zmm0
vpmadd52luq zmm17,zmm11,zmm0
vpmadd52luq zmm20,zmm14,zmm0
vpaddq zmm3,zmm3,zmm30
vpsrlq zmm3,zmm3,0x34
vpmadd52huq zmm12,zmm21,zmm8
vpmadd52huq zmm19,zmm21,zmm7
vpmadd52huq zmm17,zmm21,zmm6
vpmadd52huq zmm3,zmm21,zmm28
vpmadd52luq zmm20,zmm21,zmm8
vpmadd52huq zmm16,zmm21,zmm5
vpmadd52luq zmm12,zmm21,zmm7
vpmadd52luq zmm19,zmm21,zmm6
vpmadd52luq zmm17,zmm21,zmm5
vpaddq zmm20,zmm20,zmm3
vpmadd52huq zmm16,zmm10,zmm1
vpxord xmm21,xmm21,xmm21
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm11,zmm1
vpmadd52luq zmm20,zmm15,zmm1
vpmadd52huq zmm12,zmm15,zmm1
vpmadd52huq zmm19,zmm14,zmm1
vpmadd52huq zmm17,zmm13,zmm1
vpmadd52luq zmm16,zmm11,zmm1
vpmadd52luq zmm21,zmm20,zmm23
vpaddq zmm20,zmm20,zmm30
vpsrlq zmm20,zmm20,0x34
vpmadd52luq zmm12,zmm14,zmm1
vpmadd52luq zmm19,zmm13,zmm1
vpmadd52luq zmm17,zmm10,zmm1
vpmadd52huq zmm16,zmm21,zmm6
vpmadd52huq zmm20,zmm21,zmm28
vpmadd52huq zmm3,zmm21,zmm5
vpmadd52huq zmm19,zmm21,zmm8
vpmadd52huq zmm17,zmm21,zmm7
vpmadd52luq zmm12,zmm21,zmm8
vpmadd52luq zmm16,zmm21,zmm5
vpmadd52huq zmm3,zmm10,zmm2
vpaddq zmm20,zmm12,zmm20
vpmadd52luq zmm19,zmm21,zmm7
vpmadd52luq zmm17,zmm21,zmm6
vpxord xmm21,xmm21,xmm21
vpxor  xmm12,xmm12,xmm12
vpmadd52huq zmm12,zmm11,zmm2
vpmadd52luq zmm20,zmm15,zmm2
vpmadd52huq zmm16,zmm13,zmm2
vpmadd52luq zmm3,zmm11,zmm2
vpmadd52huq zmm19,zmm15,zmm2
vpmadd52huq zmm17,zmm14,zmm2
vpmadd52luq zmm21,zmm20,zmm23
vpmadd52luq zmm16,zmm10,zmm2
vpaddq zmm20,zmm20,zmm30
vpsrlq zmm20,zmm20,0x34
vpmadd52luq zmm19,zmm14,zmm2
vpmadd52luq zmm17,zmm13,zmm2
vpmadd52huq zmm3,zmm21,zmm6
vpmadd52huq zmm16,zmm21,zmm7
vpmadd52huq zmm20,zmm21,zmm28
vpmadd52huq zmm12,zmm21,zmm5
vpmadd52huq zmm17,zmm21,zmm8
vpmadd52luq zmm19,zmm21,zmm8
vpmadd52luq zmm3,zmm21,zmm5
vpmadd52luq zmm16,zmm21,zmm6
vpmadd52huq zmm12,zmm10,zmm4
vpaddq zmm20,zmm19,zmm20
vpmadd52luq zmm17,zmm21,zmm7
vpxord xmm21,xmm21,xmm21
vpxord xmm19,xmm19,xmm19
vpmadd52huq zmm19,zmm11,zmm4
vpmadd52huq zmm3,zmm13,zmm4
vpmadd52luq zmm20,zmm15,zmm4
vpmadd52huq zmm16,zmm14,zmm4
vpmadd52luq zmm12,zmm11,zmm4
vpmadd52huq zmm17,zmm15,zmm4
vpmadd52luq zmm21,zmm20,zmm23
vpmadd52luq zmm3,zmm10,zmm4
vpmadd52luq zmm16,zmm13,zmm4
vpaddq zmm20,zmm20,zmm30
vpsrlq zmm20,zmm20,0x34
vpmadd52luq zmm17,zmm14,zmm4
vpmadd52huq zmm3,zmm21,zmm7
vpmadd52huq zmm12,zmm21,zmm6
vpmadd52huq zmm16,zmm21,zmm8
vpmadd52huq zmm20,zmm21,zmm28
vpmadd52huq zmm19,zmm21,zmm5
vpmadd52luq zmm17,zmm21,zmm8
vpmadd52luq zmm3,zmm21,zmm6
vpmadd52luq zmm12,zmm21,zmm5
vpmadd52luq zmm16,zmm21,zmm7
vpmadd52huq zmm19,zmm10,zmm9
vpaddq zmm17,zmm17,zmm20
vpmadd52huq zmm3,zmm14,zmm9
vpmadd52huq zmm12,zmm13,zmm9
vpmadd52luq zmm17,zmm15,zmm9
vpmadd52huq zmm16,zmm15,zmm9
vpmadd52luq zmm19,zmm11,zmm9
vpmadd52luq zmm3,zmm13,zmm9
vpmadd52luq zmm12,zmm10,zmm9
vpxor  xmm10,xmm10,xmm10
vpmadd52luq zmm10,zmm17,zmm23
vpmadd52luq zmm16,zmm14,zmm9
vpxor  xmm13,xmm13,xmm13
vpmadd52huq zmm13,zmm11,zmm9
vpaddq zmm11,zmm17,zmm30
vpsrlq zmm11,zmm11,0x34
vpmadd52huq zmm3,zmm10,zmm8
vpmadd52huq zmm12,zmm10,zmm7
vpmadd52huq zmm11,zmm10,zmm28
vpmadd52luq zmm16,zmm10,zmm8
vpmadd52huq zmm19,zmm10,zmm6
vpmadd52huq zmm13,zmm10,zmm5
vpmadd52luq zmm3,zmm10,zmm7
vpaddq zmm11,zmm16,zmm11
vpmadd52luq zmm12,zmm10,zmm6
vpmadd52luq zmm19,zmm10,zmm5
vpsrlq zmm5,zmm11,0x34
vpaddq zmm3,zmm5,zmm3
vpandq zmm5,zmm11,zmm30
vpsrlq zmm6,zmm3,0x34
vpandq zmm3,zmm3,zmm30
vmovdqa64 ZMMWORD PTR [rcx],zmm5
vpaddq zmm6,zmm6,zmm12
vmovdqa64 ZMMWORD PTR [rcx+0x40],zmm3
vpsrlq zmm7,zmm6,0x34
vpandq zmm6,zmm6,zmm30
vpaddq zmm7,zmm7,zmm19
vmovdqa64 ZMMWORD PTR [rcx+0x80],zmm6
vpsrlq zmm8,zmm7,0x34
vpandq zmm7,zmm7,zmm30
vpaddq zmm3,zmm8,zmm13
vmovdqa64 ZMMWORD PTR [rcx+0xc0],zmm7
vmovdqa64 ZMMWORD PTR [rcx+0x100],zmm3
mov    rcx,rdi
test   dl,0x1
jne    ea050 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x9ab0>
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0x5c0]
vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x700]
vmovdqa64 zmm26,ZMMWORD PTR [rsp+0x600]
vmovdqa64 zmm27,ZMMWORD PTR [rsp+0x740]
vmovdqa64 zmm11,ZMMWORD PTR [rsp+0x780]
vmovdqa64 zmm25,ZMMWORD PTR [rsp+0x640]
vmovdqa64 zmm31,ZMMWORD PTR [rsp+0x7c0]
vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x680]
vmovdqa64 zmm16,ZMMWORD PTR [rsp+0x800]
vmovdqa64 zmm14,ZMMWORD PTR [rsp+0x6c0]
vpbroadcastq zmm6,QWORD PTR [rip+0xfffffffffff1f443]        # 98d8 <GCC_except_table830+0x137c>
vmovdqu64 YMMWORD PTR [rsp+0x5c0],ymm29
mov    QWORD PTR [rsp+0x5e0],r12
vpxor  xmm10,xmm10,xmm10
vpxor  xmm12,xmm12,xmm12
vpxord xmm17,xmm17,xmm17
vpbroadcastq zmm15,QWORD PTR [rip+0xfffffffffff1f3b9]        # 9878 <GCC_except_table830+0x131c>
vpxord xmm21,xmm21,xmm21
vpxord xmm20,xmm20,xmm20
vpxord xmm24,xmm24,xmm24
movabs rax,0x60c89ce5c263
mov    r14,QWORD PTR [rsp+0x4c0]
mov    r13,QWORD PTR [rsp+0x20]
mov    r8,QWORD PTR [rsp+0x30]
mov    r10,QWORD PTR [rsp+0x38]
vpaddq zmm1,zmm8,zmm18
vpaddq zmm2,zmm27,zmm26
vpaddq zmm4,zmm31,zmm13
vpaddq zmm5,zmm16,zmm14
vmovdqa64 ZMMWORD PTR [rsp+0x1c0],zmm8
vmovdqa64 ZMMWORD PTR [rsp+0x180],zmm11
vmovdqa64 ZMMWORD PTR [rsp+0x240],zmm27
vmovdqa64 ZMMWORD PTR [rsp+0x2c0],zmm31
vmovdqa64 ZMMWORD PTR [rsp+0x300],zmm16
vmovdqa64 ZMMWORD PTR [rsp+0x1040],zmm18
vmovdqa64 ZMMWORD PTR [rsp+0x1000],zmm26
vpsrlq zmm0,zmm1,0x34
vpandq zmm9,zmm1,zmm30
vpbroadcastq zmm1,QWORD PTR [rsp+0x5c8]
vpaddq zmm3,zmm0,zmm2
vpaddq zmm2,zmm11,zmm25
vpsrlq zmm0,zmm3,0x34
vpaddq zmm0,zmm0,zmm2
vpsrlq zmm2,zmm0,0x34
vpaddq zmm2,zmm2,zmm4
vpsrlq zmm4,zmm2,0x34
vpaddq zmm7,zmm4,zmm5
vpsubq zmm4,zmm18,zmm8
vpsubq zmm8,zmm26,zmm27
vpaddq zmm4,zmm4,zmm6
vpbroadcastq zmm6,QWORD PTR [rip+0xfffffffffff1f3cf]        # 9978 <GCC_except_table830+0x141c>
vpaddq zmm5,zmm8,zmm6
vpandq zmm6,zmm4,zmm30
vpsraq zmm8,zmm4,0x34
vpxor  xmm4,xmm4,xmm4
vpmadd52luq zmm10,zmm9,zmm6
vpmadd52huq zmm12,zmm9,zmm6
vpaddq zmm5,zmm5,zmm8
vpandq zmm8,zmm3,zmm30
vpmadd52luq zmm4,zmm10,zmm23
vpmadd52luq zmm12,zmm8,zmm6
vpaddq zmm3,zmm10,zmm30
vpbroadcastq zmm10,QWORD PTR [rip+0xfffffffffff1f35c]        # 9950 <GCC_except_table830+0x13f4>
vpsrlq zmm3,zmm3,0x34
vpmadd52luq zmm12,zmm4,zmm1
vpmadd52huq zmm3,zmm4,zmm28
vpaddq zmm12,zmm12,zmm3
vpsubq zmm3,zmm25,zmm11
vpsraq zmm11,zmm5,0x34
vpaddq zmm3,zmm3,zmm10
vpandq zmm10,zmm0,zmm30
vpbroadcastq zmm0,QWORD PTR [rsp+0x5d0]
vpaddq zmm19,zmm3,zmm11
vpandq zmm3,zmm5,zmm30
vpxor  xmm5,xmm5,xmm5
vpmadd52huq zmm5,zmm8,zmm6
vpmadd52huq zmm21,zmm10,zmm6
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff1f199]        # 97f0 <GCC_except_table830+0x1294>
vpmadd52luq zmm12,zmm9,zmm3
vpmadd52luq zmm5,zmm10,zmm6
vpmadd52luq zmm17,zmm12,zmm23
vpaddq zmm12,zmm12,zmm30
vpsrlq zmm12,zmm12,0x34
vpmadd52huq zmm5,zmm4,zmm1
vpmadd52huq zmm12,zmm17,zmm28
vpmadd52luq zmm5,zmm4,zmm0
vpmadd52huq zmm5,zmm9,zmm3
vpmadd52luq zmm5,zmm8,zmm3
vpmadd52luq zmm5,zmm17,zmm1
vpaddq zmm5,zmm5,zmm12
vpsubq zmm12,zmm13,zmm31
vpaddq zmm12,zmm12,zmm15
vpsraq zmm15,zmm19,0x34
vpandq zmm19,zmm19,zmm30
vpaddq zmm12,zmm12,zmm15
vpandq zmm15,zmm2,zmm30
vpbroadcastq zmm2,QWORD PTR [rsp+0x5d8]
vpmadd52luq zmm5,zmm9,zmm19
vpmadd52luq zmm21,zmm15,zmm6
vpmadd52huq zmm24,zmm15,zmm6
vpmadd52luq zmm20,zmm5,zmm23
vpaddq zmm5,zmm5,zmm30
vpmadd52huq zmm21,zmm4,zmm0
vpsrlq zmm5,zmm5,0x34
vpmadd52luq zmm24,zmm7,zmm6
vpmadd52huq zmm5,zmm20,zmm28
vpmadd52luq zmm21,zmm4,zmm2
vpmadd52huq zmm24,zmm4,zmm2
vpmadd52huq zmm21,zmm8,zmm3
vpmadd52luq zmm21,zmm10,zmm3
vpmadd52huq zmm21,zmm17,zmm1
vpmadd52luq zmm21,zmm17,zmm0
vpmadd52huq zmm21,zmm9,zmm19
vpmadd52luq zmm21,zmm8,zmm19
vpmadd52luq zmm21,zmm20,zmm1
vpaddq zmm22,zmm21,zmm5
vpsubq zmm5,zmm14,zmm16
vpsraq zmm21,zmm12,0x34
vpandq zmm12,zmm12,zmm30
vpaddq zmm16,zmm16,zmm16
vpaddq zmm5,zmm5,zmm11
vpmadd52luq zmm22,zmm9,zmm12
vpaddq zmm11,zmm5,zmm21
vpxor  xmm5,xmm5,xmm5
vpmadd52huq zmm5,zmm7,zmm6
vpbroadcastq zmm6,QWORD PTR [rsp+0x5e0]
vpxord xmm21,xmm21,xmm21
vmovdqu64 YMMWORD PTR [rsp+0x5c0],ymm29
mov    QWORD PTR [rsp+0x5e0],r12
vpmadd52luq zmm21,zmm22,zmm23
vpmadd52huq zmm5,zmm4,zmm6
vpmadd52luq zmm24,zmm4,zmm6
vpaddq zmm4,zmm22,zmm30
vpandq zmm22,zmm11,zmm30
vmovdqa64 zmm11,zmm23
vpsrlq zmm4,zmm4,0x34
vpmadd52huq zmm4,zmm21,zmm28
vpxord xmm28,xmm28,xmm28
vpmadd52huq zmm5,zmm15,zmm3
vpmadd52huq zmm24,zmm10,zmm3
vpmadd52luq zmm5,zmm7,zmm3
vpmadd52luq zmm24,zmm15,zmm3
vpmadd52huq zmm5,zmm17,zmm2
vpmadd52huq zmm24,zmm17,zmm0
vpmadd52luq zmm5,zmm17,zmm6
vpmadd52luq zmm24,zmm17,zmm2
vpmadd52huq zmm5,zmm10,zmm19
vpmadd52huq zmm24,zmm8,zmm19
vpmadd52luq zmm5,zmm15,zmm19
vpmadd52luq zmm24,zmm10,zmm19
vpmadd52huq zmm5,zmm20,zmm0
vpmadd52huq zmm24,zmm20,zmm1
vpmadd52luq zmm5,zmm20,zmm2
vpmadd52luq zmm24,zmm20,zmm0
vpmadd52huq zmm5,zmm8,zmm12
vpmadd52huq zmm24,zmm9,zmm12
vpmadd52luq zmm5,zmm10,zmm12
vpmadd52luq zmm24,zmm8,zmm12
vpmadd52huq zmm5,zmm21,zmm1
vpmadd52luq zmm24,zmm21,zmm1
vpmadd52luq zmm5,zmm21,zmm0
vpaddq zmm4,zmm24,zmm4
vpmadd52luq zmm4,zmm9,zmm22
vpmadd52huq zmm5,zmm9,zmm22
vpxor  xmm9,xmm9,xmm9
vpmadd52huq zmm9,zmm7,zmm3
vpsllq zmm3,ZMMWORD PTR [rsp+0x1c0],0x1
vpmadd52huq zmm9,zmm17,zmm6
vpmadd52luq zmm5,zmm8,zmm22
vpxord xmm17,xmm17,xmm17
vpmadd52huq zmm9,zmm15,zmm19
vpmadd52luq zmm9,zmm7,zmm19
vpmadd52huq zmm9,zmm20,zmm2
vpmadd52luq zmm9,zmm20,zmm6
vpmadd52huq zmm9,zmm10,zmm12
vpmadd52luq zmm9,zmm15,zmm12
vpmadd52huq zmm9,zmm21,zmm0
vpmadd52luq zmm9,zmm21,zmm2
vpmadd52huq zmm9,zmm8,zmm22
vpxor  xmm8,xmm8,xmm8
vpmadd52huq zmm8,zmm7,zmm19
vpsllq zmm19,ZMMWORD PTR [rsp+0x180],0x1
vpmadd52huq zmm8,zmm20,zmm6
vpmadd52luq zmm9,zmm10,zmm22
vpxord xmm20,xmm20,xmm20
vpmadd52huq zmm8,zmm15,zmm12
vpmadd52luq zmm8,zmm7,zmm12
vpmadd52huq zmm8,zmm21,zmm2
vpmadd52luq zmm8,zmm21,zmm6
vpmadd52huq zmm8,zmm10,zmm22
vpxor  xmm10,xmm10,xmm10
vpmadd52huq zmm10,zmm7,zmm12
vpaddq zmm12,zmm27,zmm27
vpaddq zmm27,zmm31,zmm31
vpxord xmm31,xmm31,xmm31
vpmadd52huq zmm10,zmm21,zmm6
vpmadd52luq zmm8,zmm15,zmm22
vpmadd52huq zmm10,zmm15,zmm22
vpxor  xmm15,xmm15,xmm15
vpmadd52huq zmm15,zmm7,zmm22
vpmadd52luq zmm10,zmm7,zmm22
vpandq zmm22,zmm3,QWORD BCST [rip+0xfffffffffff1efca]        # 9920 <GCC_except_table830+0x13c4>
vpsrlq zmm7,zmm3,0x34
vpbroadcastq zmm3,QWORD PTR [rsp+0x5c8]
vpaddq zmm7,zmm7,zmm12
vpxor  xmm12,xmm12,xmm12
vpsrlq zmm21,zmm7,0x34
vpandq zmm24,zmm7,zmm30
vpbroadcastq zmm7,QWORD PTR [rsp+0x5d0]
vpmadd52luq zmm12,zmm18,zmm22
vpmadd52huq zmm17,zmm18,zmm22
vpmadd52huq zmm31,zmm13,zmm22
vpmadd52luq zmm28,zmm12,zmm23
vpaddq zmm12,zmm12,zmm30
vpmadd52luq zmm17,zmm26,zmm22
vpxord xmm23,xmm23,xmm23
vpmadd52luq zmm31,zmm14,zmm22
vpsrlq zmm12,zmm12,0x34
vpmadd52huq zmm12,zmm28,QWORD BCST [rip+0xfffffffffff1edd4]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52luq zmm17,zmm28,zmm3
vpaddq zmm12,zmm17,zmm12
vpaddq zmm17,zmm21,zmm19
vpxord xmm19,xmm19,xmm19
vpmadd52huq zmm19,zmm26,zmm22
vpmadd52luq zmm12,zmm18,zmm24
vpmadd52luq zmm19,zmm25,zmm22
vpmadd52luq zmm23,zmm12,zmm11
vpaddq zmm12,zmm12,zmm30
vpsrlq zmm12,zmm12,0x34
vpmadd52huq zmm12,zmm23,QWORD BCST [rip+0xfffffffffff1ed8d]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52huq zmm19,zmm28,zmm3
vpmadd52luq zmm19,zmm28,zmm7
vpmadd52huq zmm19,zmm18,zmm24
vpmadd52luq zmm19,zmm26,zmm24
vpmadd52luq zmm19,zmm23,zmm3
vpaddq zmm21,zmm19,zmm12
vpsrlq zmm12,zmm17,0x34
vpandq zmm19,zmm17,zmm30
vpxord xmm17,xmm17,xmm17
vpmadd52huq zmm17,zmm25,zmm22
vpaddq zmm27,zmm12,zmm27
vpbroadcastq zmm12,QWORD PTR [rsp+0x5d8]
vpmadd52luq zmm21,zmm18,zmm19
vpmadd52luq zmm17,zmm13,zmm22
vpmadd52luq zmm20,zmm21,zmm11
vpaddq zmm21,zmm21,zmm30
vpsrlq zmm21,zmm21,0x34
vpmadd52huq zmm21,zmm20,QWORD BCST [rip+0xfffffffffff1ed16]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52huq zmm17,zmm28,zmm7
vpmadd52huq zmm31,zmm28,zmm12
vpmadd52luq zmm17,zmm28,zmm12
vpmadd52huq zmm17,zmm26,zmm24
vpmadd52luq zmm17,zmm25,zmm24
vpmadd52huq zmm17,zmm23,zmm3
vpmadd52luq zmm17,zmm23,zmm7
vpmadd52huq zmm17,zmm18,zmm19
vpmadd52luq zmm17,zmm26,zmm19
vpmadd52luq zmm17,zmm20,zmm3
vpaddq zmm29,zmm17,zmm21
vpsrlq zmm17,zmm27,0x34
vpandq zmm27,zmm27,zmm30
vpaddq zmm21,zmm17,zmm16
vpbroadcastq zmm16,QWORD PTR [rsp+0x5e0]
vpxord xmm17,xmm17,xmm17
vpmadd52huq zmm17,zmm14,zmm22
vpmadd52luq zmm29,zmm18,zmm27
vmovdqa64 zmm22,ZMMWORD PTR [rsp+0x900]
vpmadd52luq zmm31,zmm28,zmm16
vpmadd52huq zmm17,zmm28,zmm16
vpxord xmm28,xmm28,xmm28
vpmadd52luq zmm28,zmm29,zmm11
vpaddq zmm29,zmm29,zmm30
vpsrlq zmm29,zmm29,0x34
vpmadd52huq zmm29,zmm28,QWORD BCST [rip+0xfffffffffff1ec6d]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52huq zmm31,zmm25,zmm24
vpmadd52huq zmm17,zmm13,zmm24
vpmadd52luq zmm31,zmm13,zmm24
vpmadd52luq zmm17,zmm14,zmm24
vpmadd52huq zmm31,zmm23,zmm7
vpmadd52huq zmm17,zmm23,zmm12
vpmadd52luq zmm31,zmm23,zmm12
vpmadd52luq zmm17,zmm23,zmm16
vpmadd52huq zmm31,zmm26,zmm19
vpmadd52huq zmm17,zmm25,zmm19
vpmadd52luq zmm31,zmm25,zmm19
vpmadd52luq zmm17,zmm13,zmm19
vpmadd52huq zmm31,zmm20,zmm3
vpmadd52huq zmm17,zmm20,zmm7
vpmadd52luq zmm31,zmm20,zmm7
vpmadd52luq zmm17,zmm20,zmm12
vpmadd52huq zmm31,zmm18,zmm27
vpmadd52huq zmm17,zmm26,zmm27
vpmadd52luq zmm31,zmm26,zmm27
vpmadd52luq zmm17,zmm25,zmm27
vpmadd52luq zmm31,zmm28,zmm3
vpmadd52huq zmm17,zmm28,zmm3
vpaddq zmm29,zmm31,zmm29
vpxord xmm31,xmm31,xmm31
vpmadd52huq zmm31,zmm14,zmm24
vpmadd52luq zmm17,zmm28,zmm7
vpbroadcastq zmm24,QWORD PTR [rip+0xfffffffffff1ecff]        # 98d8 <GCC_except_table830+0x137c>
vpmadd52luq zmm29,zmm18,zmm21
vpmadd52huq zmm31,zmm23,zmm16
vpmadd52huq zmm17,zmm18,zmm21
vpxord xmm23,xmm23,xmm23
vpmadd52luq zmm23,zmm29,zmm11
vpmadd52huq zmm31,zmm13,zmm19
vpmadd52luq zmm17,zmm26,zmm21
vpmadd52luq zmm31,zmm14,zmm19
vpmadd52luq zmm17,zmm23,zmm3
vpmadd52huq zmm31,zmm20,zmm12
vpmadd52luq zmm31,zmm20,zmm16
vpmadd52huq zmm31,zmm25,zmm27
vpmadd52luq zmm31,zmm13,zmm27
vpmadd52huq zmm31,zmm28,zmm7
vpmadd52luq zmm31,zmm28,zmm12
vpmadd52huq zmm31,zmm26,zmm21
vpbroadcastq zmm26,QWORD PTR [rip+0xfffffffffff1ed0d]        # 9950 <GCC_except_table830+0x13f4>
vpmadd52luq zmm31,zmm25,zmm21
vpmadd52huq zmm31,zmm23,zmm3
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm14,zmm19
vpaddq zmm19,zmm29,zmm30
vpsrlq zmm19,zmm19,0x34
vpmadd52huq zmm19,zmm23,QWORD BCST [rip+0xfffffffffff1eb30]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52huq zmm3,zmm20,zmm16
vpmadd52luq zmm31,zmm23,zmm7
vpmadd52huq zmm3,zmm13,zmm27
vpaddq zmm20,zmm17,zmm19
vpmadd52luq zmm3,zmm14,zmm27
vpsrlq zmm17,zmm20,0x34
vpaddq zmm19,zmm17,zmm31
vpsrlq zmm17,zmm19,0x34
vpmadd52huq zmm3,zmm28,zmm12
vpmadd52luq zmm3,zmm28,zmm16
vpmadd52huq zmm3,zmm25,zmm21
vpmadd52luq zmm3,zmm13,zmm21
vpmadd52huq zmm3,zmm23,zmm7
vpxor  xmm7,xmm7,xmm7
vpmadd52huq zmm7,zmm14,zmm27
vpbroadcastq zmm27,QWORD PTR [rip+0xfffffffffff1eca4]        # 9978 <GCC_except_table830+0x141c>
vpmadd52huq zmm7,zmm28,zmm16
vpmadd52luq zmm3,zmm23,zmm12
vpmadd52huq zmm7,zmm13,zmm21
vpaddq zmm17,zmm17,zmm3
vpsrlq zmm3,zmm17,0x34
vpmadd52luq zmm7,zmm14,zmm21
vpmadd52huq zmm7,zmm23,zmm12
vpxor  xmm12,xmm12,xmm12
vpmadd52huq zmm12,zmm14,zmm21
vmovdqa64 zmm21,ZMMWORD PTR [rsp+0x340]
vpmadd52luq zmm7,zmm23,zmm16
vpmadd52huq zmm12,zmm23,zmm16
vmovdqa64 zmm23,ZMMWORD PTR [rsp+0x880]
vpaddq zmm18,zmm3,zmm7
vpaddq zmm7,zmm21,ZMMWORD PTR [rsp+0xa00]
vpsrlq zmm3,zmm18,0x34
vpaddq zmm28,zmm3,zmm12
vpxor  xmm12,xmm12,xmm12
vpmadd52luq zmm12,zmm4,zmm11
vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x280]
vmovaps ymm11,YMMWORD PTR [rip+0xfffffffffff1f284]        # 9fe0 <GCC_except_table830+0x1a84>
vpmadd52huq zmm9,zmm12,zmm1
vpmadd52luq zmm5,zmm12,zmm1
vpaddq zmm1,zmm4,zmm30
vpmadd52huq zmm8,zmm12,zmm0
vpmadd52huq zmm10,zmm12,zmm2
vpmadd52huq zmm15,zmm12,zmm6
vpmadd52luq zmm9,zmm12,zmm0
vpsrlq zmm0,zmm1,0x34
vpmadd52huq zmm0,zmm12,QWORD BCST [rip+0xfffffffffff1ea09]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52luq zmm10,zmm12,zmm6
vpaddq zmm6,zmm3,ZMMWORD PTR [rsp+0xa40]
vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x200]
vpmadd52luq zmm8,zmm12,zmm2
vpbroadcastq zmm12,QWORD PTR [rip+0xfffffffffff1ea33]        # 97f0 <GCC_except_table830+0x1294>
vmovups YMMWORD PTR [rsp+0x5c0],ymm11
mov    QWORD PTR [rsp+0x5e0],rax
vpaddq zmm3,zmm3,ZMMWORD PTR [rsp+0xa80]
vpaddq zmm0,zmm5,zmm0
vpsrlq zmm1,zmm0,0x34
vpandq zmm0,zmm0,zmm30
vpsubq zmm0,zmm0,zmm6
vpaddq zmm1,zmm1,zmm9
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff1e9e9]        # 97e8 <GCC_except_table830+0x128c>
vpaddq zmm0,zmm0,zmm24
vpsrlq zmm2,zmm1,0x34
vpandq zmm1,zmm1,zmm30
vpsraq zmm6,zmm0,0x34
vpaddq zmm2,zmm2,zmm8
vpbroadcastq zmm8,QWORD PTR [rip+0xfffffffffff1ea7f]        # 98a8 <GCC_except_table830+0x134c>
vpandq zmm29,zmm0,zmm30
vpaddq zmm1,zmm6,zmm1
vpsrlq zmm4,zmm2,0x34
vpandq zmm2,zmm2,zmm30
vpsubq zmm1,zmm1,zmm3
vpaddq zmm3,zmm22,ZMMWORD PTR [rsp+0xac0]
vpaddq zmm4,zmm4,zmm10
vpbroadcastq zmm10,QWORD PTR [rip+0xfffffffffff1ea60]        # 98c0 <GCC_except_table830+0x1364>
vpaddq zmm1,zmm1,zmm27
vpsrlq zmm5,zmm4,0x34
vpandq zmm4,zmm4,zmm30
vpsraq zmm6,zmm1,0x34
vpandq zmm31,zmm1,zmm30
vpaddq zmm5,zmm5,zmm15
vpsubq zmm0,zmm31,QWORD BCST [rsp+0x5c8]
vmovaps ymm15,YMMWORD PTR [rip+0xfffffffffff1efc7]        # 9e60 <GCC_except_table830+0x1904>
vpaddq zmm2,zmm6,zmm2
vpsubq zmm2,zmm2,zmm7
vpaddq zmm7,zmm23,ZMMWORD PTR [rsp+0x840]
vpaddq zmm2,zmm2,zmm26
vpsraq zmm6,zmm2,0x34
vpaddq zmm1,zmm29,zmm8
vpaddq zmm4,zmm6,zmm4
vpsubq zmm4,zmm4,zmm3
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff1e9a2]        # 9878 <GCC_except_table830+0x131c>
vpaddq zmm4,zmm4,zmm3
vpsraq zmm6,zmm4,0x34
vpandq zmm8,zmm4,zmm30
vpsubq zmm4,zmm8,QWORD BCST [rsp+0x5d8]
vpaddq zmm5,zmm6,zmm5
vpsraq zmm6,zmm1,0x34
vpsubq zmm5,zmm5,zmm7
vpandq zmm7,zmm2,zmm30
vpsubq zmm2,zmm7,QWORD BCST [rsp+0x5d0]
vpaddq zmm0,zmm0,zmm6
vpaddq zmm5,zmm5,zmm12
vpsraq zmm6,zmm0,0x34
vpandq zmm16,zmm5,zmm30
vpsubq zmm5,zmm16,QWORD BCST [rsp+0x5e0]
vmovups YMMWORD PTR [rsp+0x5c0],ymm15
mov    QWORD PTR [rsp+0x5e0],r12
vpaddq zmm2,zmm6,zmm2
vpsraq zmm6,zmm2,0x34
vpaddq zmm4,zmm6,zmm4
vpsraq zmm6,zmm4,0x34
vpaddq zmm5,zmm6,zmm5
vpcmpltuq k1,zmm5,zmm9
vpandq zmm31{k1},zmm0,zmm30
vpsubq zmm0,zmm31,QWORD BCST [rsp+0x5c8]
vpandq zmm7{k1},zmm2,zmm30
vpandq zmm16{k1},zmm5,zmm30
vpsubq zmm5,zmm7,QWORD BCST [rsp+0x5d0]
vpandq zmm29{k1},zmm1,zmm30
vpandq zmm8{k1},zmm4,zmm30
vpaddq zmm1,zmm29,zmm10
vpsraq zmm2,zmm1,0x34
vpaddq zmm0,zmm0,zmm2
vpsraq zmm4,zmm0,0x34
vpaddq zmm2,zmm4,zmm5
vpsubq zmm4,zmm8,QWORD BCST [rsp+0x5d8]
vpsraq zmm5,zmm2,0x34
vpaddq zmm4,zmm5,zmm4
vpsubq zmm5,zmm16,QWORD BCST [rsp+0x5e0]
vmovups YMMWORD PTR [rsp+0x5c0],ymm11
mov    QWORD PTR [rsp+0x5e0],rax
vpsraq zmm6,zmm4,0x34
vpaddq zmm5,zmm6,zmm5
vpcmpltuq k1,zmm5,zmm9
vpandq zmm7{k1},zmm2,zmm30
vpandq zmm2,zmm20,zmm30
vmovdqa64 zmm20,ZMMWORD PTR [rsp+0x400]
vpandq zmm29{k1},zmm1,zmm30
vpandq zmm31{k1},zmm0,zmm30
vpandq zmm8{k1},zmm4,zmm30
vmovdqa64 zmm16{k1},zmm5
vmovdqa64 ZMMWORD PTR [rsp+0x900],zmm16
vpaddq zmm1,zmm20,ZMMWORD PTR [rsp+0x580]
vmovdqa64 ZMMWORD PTR [rsp+0xa80],zmm29
vmovdqa64 ZMMWORD PTR [rsp+0x340],zmm8
vmovdqa64 ZMMWORD PTR [rsp+0xa40],zmm7
vpsubq zmm0,zmm2,zmm1
vpandq zmm1,zmm19,zmm30
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0x3c0]
vpaddq zmm4,zmm19,ZMMWORD PTR [rsp+0xb00]
vpaddq zmm0,zmm0,zmm24
vpsraq zmm2,zmm0,0x34
vpaddq zmm1,zmm2,zmm1
vpandq zmm2,zmm17,zmm30
vmovdqa64 zmm17,ZMMWORD PTR [rsp+0xf40]
vpsubq zmm1,zmm1,zmm4
vpaddq zmm1,zmm1,zmm27
vmovdqa64 zmm27,ZMMWORD PTR [rsp+0xbc0]
vpsraq zmm4,zmm1,0x34
vpaddq zmm6,zmm27,ZMMWORD PTR [rsp+0x10c0]
vpandq zmm24,zmm1,zmm30
vpaddq zmm2,zmm4,zmm2
vpaddq zmm4,zmm17,ZMMWORD PTR [rsp+0xf00]
vpsubq zmm2,zmm2,zmm4
vpandq zmm4,zmm18,zmm30
vpandq zmm18,zmm0,zmm30
vpaddq zmm1,zmm18,QWORD BCST [rip+0xfffffffffff1e7b4]        # 98a8 <GCC_except_table830+0x134c>
vpsubq zmm0,zmm24,QWORD BCST [rsp+0x5c8]
vpaddq zmm2,zmm2,zmm26
vpsraq zmm5,zmm2,0x34
vpandq zmm26,zmm2,zmm30
vpsubq zmm2,zmm26,QWORD BCST [rsp+0x5d0]
vpaddq zmm4,zmm5,zmm4
vpsubq zmm4,zmm4,zmm6
vpaddq zmm4,zmm4,zmm3
vpsraq zmm5,zmm4,0x34
vpandq zmm11,zmm4,zmm30
vpsubq zmm4,zmm11,QWORD BCST [rsp+0x5d8]
vpaddq zmm3,zmm5,zmm28
vmovdqa64 zmm28,ZMMWORD PTR [rsp+0x1080]
vpaddq zmm5,zmm28,ZMMWORD PTR [rsp+0xe80]
vpsubq zmm5,zmm3,zmm5
vpsraq zmm3,zmm1,0x34
vpaddq zmm5,zmm5,zmm12
vpaddq zmm0,zmm0,zmm3
vpandq zmm12,zmm5,zmm30
vpsubq zmm5,zmm12,QWORD BCST [rsp+0x5e0]
vpsraq zmm6,zmm0,0x34
vmovups YMMWORD PTR [rsp+0x5c0],ymm15
mov    QWORD PTR [rsp+0x5e0],r12
vpaddq zmm2,zmm6,zmm2
vpsraq zmm6,zmm2,0x34
vpaddq zmm4,zmm6,zmm4
vpsraq zmm6,zmm4,0x34
vpaddq zmm5,zmm6,zmm5
vpcmpltuq k1,zmm5,zmm9
vpandq zmm24{k1},zmm0,zmm30
vpsubq zmm0,zmm24,QWORD BCST [rsp+0x5c8]
vpandq zmm26{k1},zmm2,zmm30
vpsubq zmm3,zmm26,QWORD BCST [rsp+0x5d0]
vpandq zmm18{k1},zmm1,zmm30
vpandq zmm11{k1},zmm4,zmm30
vpandq zmm12{k1},zmm5,zmm30
vpaddq zmm1,zmm18,zmm10
vpbroadcastq zmm10,QWORD PTR [rip+0xfffffffffff1e61e]        # 9828 <GCC_except_table830+0x12cc>
vpsraq zmm2,zmm1,0x34
vpaddq zmm0,zmm0,zmm2
vpsraq zmm4,zmm0,0x34
vpaddq zmm2,zmm4,zmm3
vpsubq zmm4,zmm11,QWORD BCST [rsp+0x5d8]
vpsubq zmm3,zmm12,QWORD BCST [rsp+0x5e0]
vmovups YMMWORD PTR [rsp+0x5c0],ymm15
mov    QWORD PTR [rsp+0x5e0],r12
vpsraq zmm5,zmm2,0x34
vpbroadcastq zmm15,QWORD PTR [rsp+0x5e0]
vpaddq zmm4,zmm5,zmm4
vpsraq zmm6,zmm4,0x34
vpaddq zmm5,zmm6,zmm3
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff1e716]        # 9990 <GCC_except_table830+0x1434>
vpcmpltuq k1,zmm5,zmm9
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff1e60d]        # 9898 <GCC_except_table830+0x133c>
vpandq zmm18{k1},zmm1,zmm30
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x280]
vpandq zmm24{k1},zmm0,zmm30
vpandq zmm26{k1},zmm2,zmm30
vpaddq zmm2,zmm3,ZMMWORD PTR [rsp+0x200]
vmovdqa64 zmm12{k1},zmm5
vpbroadcastq zmm5,QWORD PTR [rip+0xfffffffffff1e503]        # 97c0 <GCC_except_table830+0x1264>
vpandq zmm11{k1},zmm4,zmm30
vmovdqa64 ZMMWORD PTR [rsp+0x280],zmm31
vmovdqa64 ZMMWORD PTR [rsp+0x400],zmm12
vmovdqa64 ZMMWORD PTR [rsp+0xa00],zmm24
vmovdqa64 ZMMWORD PTR [rsp+0xac0],zmm26
vmovdqa64 ZMMWORD PTR [rsp+0x880],zmm18
vmovdqa64 ZMMWORD PTR [rsp+0x840],zmm11
vpaddq zmm4,zmm19,zmm3
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff1e5d5]        # 98d8 <GCC_except_table830+0x137c>
vpsubq zmm0,zmm1,zmm29
vpaddq zmm0,zmm0,QWORD BCST [rip+0xfffffffffff1e48d]        # 97a0 <GCC_except_table830+0x1244>
vpsraq zmm1,zmm0,0x34
vpandq zmm0,zmm0,zmm30
vpsubq zmm1,zmm1,zmm31
vpaddq zmm6,zmm1,zmm2
vpaddq zmm2,zmm21,zmm5
vpbroadcastq zmm21,QWORD PTR [rsp+0x5c8]
vpsraq zmm1,zmm6,0x34
vpandq zmm6,zmm6,zmm30
vpsubq zmm1,zmm1,zmm7
vpaddq zmm29,zmm1,zmm2
vpaddq zmm2,zmm22,zmm10
vpsraq zmm1,zmm29,0x34
vpsubq zmm1,zmm1,zmm8
vpaddq zmm22,zmm1,zmm2
vpaddq zmm2,zmm23,zmm9
vpbroadcastq zmm23,QWORD PTR [rsp+0x5d8]
vpsraq zmm1,zmm22,0x34
vpandq zmm22,zmm22,zmm30
vpsubq zmm1,zmm1,zmm16
vmovdqa64 zmm16,ZMMWORD PTR [rsp+0x1000]
vpaddq zmm1,zmm1,zmm2
vmovdqa64 ZMMWORD PTR [rsp+0x200],zmm1
vpsubq zmm1,zmm20,zmm18
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1e3e7]        # 97a0 <GCC_except_table830+0x1244>
vpbroadcastq zmm18,QWORD PTR [rsp+0x5d0]
vpsraq zmm2,zmm1,0x34
vpsubq zmm2,zmm2,zmm24
vpaddq zmm8,zmm2,zmm4
vpaddq zmm4,zmm17,zmm5
vpandq zmm5,zmm1,zmm30
vpxor  xmm1,xmm1,xmm1
vpsraq zmm2,zmm8,0x34
vpsubq zmm2,zmm2,zmm26
vmovdqa64 zmm26,ZMMWORD PTR [rsp+0x1040]
vpaddq zmm24,zmm2,zmm4
vpaddq zmm4,zmm27,zmm10
vpbroadcastq zmm10,QWORD PTR [rip+0xfffffffffff1e536]        # 9948 <GCC_except_table830+0x13ec>
vpsraq zmm2,zmm24,0x34
vpandq zmm24,zmm24,zmm30
vpsubq zmm2,zmm2,zmm11
vmovdqa64 zmm11,zmm25
vpaddq zmm27,zmm2,zmm4
vpaddq zmm4,zmm28,zmm9
vpxor  xmm9,xmm9,xmm9
vpmadd52huq zmm9,zmm25,zmm0
vpsraq zmm2,zmm27,0x34
vpandq zmm27,zmm27,zmm30
vpsubq zmm2,zmm2,zmm12
vpandq zmm12,zmm8,zmm30
vpaddq zmm31,zmm2,zmm4
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x1c0]
vpmadd52luq zmm1,zmm26,zmm0
vpmadd52luq zmm9,zmm13,zmm0
vpandq zmm31,zmm31,zmm30
vmovdqa64 zmm8,zmm10
vpsubq zmm2,zmm3,zmm4
vpxor  xmm3,xmm3,xmm3
vpsraq zmm4,zmm2,0x34
vpandq zmm20,zmm2,zmm30
vpsubq zmm2,zmm4,ZMMWORD PTR [rsp+0x240]
vpxor  xmm4,xmm4,xmm4
vpmadd52huq zmm4,zmm26,zmm0
vpaddq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff1e4c4]        # 9978 <GCC_except_table830+0x141c>
vpmadd52luq zmm1,zmm20,zmm5
vpmadd52luq zmm4,zmm16,zmm0
vpmadd52luq zmm3,zmm1,zmm10
vpaddq zmm1,zmm1,zmm30
vpsrlq zmm1,zmm1,0x34
vpmadd52huq zmm1,zmm3,QWORD BCST [rip+0xfffffffffff1e2c3]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52huq zmm4,zmm20,zmm5
vpandq zmm17,zmm2,zmm30
vpmadd52luq zmm4,zmm17,zmm5
vpmadd52luq zmm4,zmm3,zmm21
vpaddq zmm19,zmm4,zmm1
vpsraq zmm1,zmm2,0x34
vpsubq zmm1,zmm1,ZMMWORD PTR [rsp+0x180]
vpxor  xmm4,xmm4,xmm4
vpmadd52huq zmm4,zmm16,zmm0
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1e432]        # 9950 <GCC_except_table830+0x13f4>
vpmadd52luq zmm19,zmm26,zmm6
vpmadd52luq zmm4,zmm25,zmm0
vpxord xmm25,xmm25,xmm25
vpmadd52huq zmm25,zmm14,zmm0
vpmadd52luq zmm19,zmm20,zmm12
vpmadd52huq zmm4,zmm17,zmm5
vpandq zmm28,zmm1,zmm30
vpsraq zmm2,zmm1,0x34
vpsubq zmm1,zmm2,ZMMWORD PTR [rsp+0x2c0]
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1e317]        # 9878 <GCC_except_table830+0x131c>
vpmadd52huq zmm9,zmm28,zmm5
vpmadd52luq zmm4,zmm28,zmm5
vpmadd52huq zmm4,zmm3,zmm21
vpandq zmm7,zmm1,zmm30
vpsraq zmm2,zmm1,0x34
vpsubq zmm1,zmm2,ZMMWORD PTR [rsp+0x300]
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1e25e]        # 97f0 <GCC_except_table830+0x1294>
vpmadd52luq zmm9,zmm7,zmm5
vpmadd52luq zmm4,zmm3,zmm18
vpmadd52huq zmm9,zmm3,zmm18
vpmadd52huq zmm4,zmm26,zmm6
vpandq zmm2,zmm1,zmm30
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm1,zmm13,zmm0
vpmadd52luq zmm9,zmm3,zmm23
vpmadd52huq zmm25,zmm2,zmm5
vpmadd52luq zmm4,zmm16,zmm6
vpmadd52luq zmm1,zmm14,zmm0
vpmadd52huq zmm9,zmm16,zmm6
vpmadd52huq zmm25,zmm3,zmm15
vpmadd52huq zmm4,zmm20,zmm12
vpmadd52huq zmm1,zmm7,zmm5
vpmadd52luq zmm9,zmm11,zmm6
vpmadd52huq zmm25,zmm13,zmm6
vpmadd52luq zmm4,zmm17,zmm12
vpmadd52luq zmm1,zmm2,zmm5
vpmadd52huq zmm9,zmm17,zmm12
vpmadd52luq zmm25,zmm14,zmm6
vpmadd52huq zmm1,zmm3,zmm23
vpmadd52luq zmm9,zmm28,zmm12
vpmadd52huq zmm25,zmm7,zmm12
vpmadd52luq zmm1,zmm3,zmm15
vpxor  xmm3,xmm3,xmm3
vpmadd52luq zmm3,zmm19,zmm10
vpaddq zmm19,zmm19,zmm30
vpandq zmm10,zmm29,zmm30
vpandq zmm29,zmm30,ZMMWORD PTR [rsp+0x200]
vpsrlq zmm19,zmm19,0x34
vpmadd52huq zmm19,zmm3,QWORD BCST [rip+0xfffffffffff1e14b]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52luq zmm25,zmm2,zmm12
vpmadd52huq zmm1,zmm11,zmm6
vpmadd52luq zmm4,zmm3,zmm21
vpmadd52huq zmm9,zmm3,zmm21
vpmadd52huq zmm25,zmm3,zmm23
vpmadd52luq zmm1,zmm13,zmm6
vpmadd52luq zmm9,zmm3,zmm18
vpmadd52luq zmm25,zmm3,zmm15
vpaddq zmm19,zmm4,zmm19
vpxor  xmm4,xmm4,xmm4
vpmadd52huq zmm4,zmm14,zmm6
vpmadd52huq zmm1,zmm28,zmm12
vpmadd52huq zmm9,zmm26,zmm10
vpmadd52luq zmm19,zmm26,zmm10
vpmadd52huq zmm25,zmm11,zmm10
vpmadd52huq zmm4,zmm2,zmm12
vpmadd52luq zmm1,zmm7,zmm12
vpmadd52luq zmm9,zmm16,zmm10
vpmadd52luq zmm19,zmm20,zmm24
vpmadd52luq zmm25,zmm13,zmm10
vpmadd52huq zmm4,zmm3,zmm15
vpmadd52huq zmm1,zmm3,zmm18
vpmadd52huq zmm9,zmm20,zmm24
vpmadd52huq zmm25,zmm28,zmm24
vpmadd52huq zmm4,zmm13,zmm10
vpmadd52luq zmm1,zmm3,zmm23
vpxor  xmm3,xmm3,xmm3
vpmadd52luq zmm3,zmm19,zmm8
vpaddq zmm19,zmm19,zmm30
vpmadd52luq zmm9,zmm17,zmm24
vpsrlq zmm19,zmm19,0x34
vpmadd52huq zmm19,zmm3,QWORD BCST [rip+0xfffffffffff1e08a]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52luq zmm25,zmm7,zmm24
vpmadd52luq zmm4,zmm14,zmm10
vpmadd52huq zmm1,zmm16,zmm10
vpmadd52luq zmm9,zmm3,zmm21
vpmadd52huq zmm25,zmm3,zmm18
vpmadd52huq zmm4,zmm7,zmm24
vpmadd52luq zmm1,zmm11,zmm10
vpmadd52luq zmm25,zmm3,zmm23
vpmadd52luq zmm4,zmm2,zmm24
vpaddq zmm19,zmm9,zmm19
vpxor  xmm9,xmm9,xmm9
vpmadd52huq zmm9,zmm14,zmm10
vpmadd52huq zmm1,zmm17,zmm24
vpmadd52luq zmm19,zmm26,zmm22
vpmadd52huq zmm25,zmm16,zmm22
vpmadd52huq zmm4,zmm3,zmm23
vpmadd52huq zmm9,zmm2,zmm24
vpmadd52luq zmm1,zmm28,zmm24
vpmadd52luq zmm19,zmm20,zmm27
vpmadd52luq zmm25,zmm11,zmm22
vpmadd52luq zmm4,zmm3,zmm15
vpmadd52huq zmm9,zmm3,zmm15
vpmadd52huq zmm1,zmm3,zmm21
vpmadd52huq zmm25,zmm17,zmm27
vpmadd52huq zmm4,zmm11,zmm22
vpmadd52huq zmm9,zmm13,zmm22
vpmadd52luq zmm1,zmm3,zmm18
vpxor  xmm3,xmm3,xmm3
vpmadd52luq zmm3,zmm19,zmm8
vpaddq zmm19,zmm19,zmm30
vpsrlq zmm19,zmm19,0x34
vpmadd52huq zmm19,zmm3,QWORD BCST [rip+0xfffffffffff1dfc8]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52luq zmm25,zmm28,zmm27
vpmadd52luq zmm4,zmm13,zmm22
vpmadd52luq zmm9,zmm14,zmm22
vpmadd52huq zmm1,zmm26,zmm22
vpmadd52huq zmm25,zmm3,zmm21
vpmadd52huq zmm4,zmm28,zmm27
vpmadd52huq zmm9,zmm7,zmm27
vpmadd52luq zmm1,zmm16,zmm22
vpmadd52luq zmm25,zmm3,zmm18
vpmadd52luq zmm4,zmm7,zmm27
vpmadd52luq zmm9,zmm2,zmm27
vpmadd52huq zmm1,zmm20,zmm27
vpmadd52huq zmm25,zmm26,zmm29
vpmadd52huq zmm4,zmm3,zmm18
vpmadd52huq zmm9,zmm3,zmm23
vpmadd52luq zmm1,zmm17,zmm27
vpmadd52luq zmm25,zmm16,zmm29
vpmadd52luq zmm4,zmm3,zmm23
vpmadd52luq zmm9,zmm3,zmm15
vpmadd52luq zmm1,zmm3,zmm21
vpmadd52huq zmm25,zmm20,zmm31
vpmadd52huq zmm4,zmm16,zmm29
vpmadd52huq zmm9,zmm11,zmm29
vpaddq zmm19,zmm1,zmm19
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm1,zmm14,zmm22
vpmadd52luq zmm19,zmm26,zmm29
vpmadd52luq zmm25,zmm17,zmm31
vpmadd52luq zmm4,zmm11,zmm29
vpmadd52luq zmm9,zmm13,zmm29
vpmadd52huq zmm1,zmm2,zmm27
vpmadd52luq zmm19,zmm20,zmm31
vpmadd52huq zmm4,zmm17,zmm31
vpmadd52huq zmm9,zmm28,zmm31
vmovaps ymm17,YMMWORD PTR [rip+0xfffffffffff1e5b4]        # 9e60 <GCC_except_table830+0x1904>
vpmadd52huq zmm1,zmm3,zmm15
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm14,zmm29
vpmadd52luq zmm4,zmm28,zmm31
vpbroadcastq zmm28,QWORD PTR [rip+0xfffffffffff1ded4]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52luq zmm9,zmm7,zmm31
vpmadd52huq zmm1,zmm13,zmm29
vpmadd52huq zmm3,zmm2,zmm31
vpmadd52luq zmm1,zmm14,zmm29
vpmadd52huq zmm1,zmm7,zmm31
vpaddq zmm7,zmm19,zmm30
vpsrlq zmm7,zmm7,0x34
vpmadd52luq zmm1,zmm2,zmm31
vpxor  xmm2,xmm2,xmm2
vpmadd52luq zmm2,zmm19,zmm8
vpmadd52huq zmm4,zmm2,zmm21
vpmadd52huq zmm9,zmm2,zmm18
vpmadd52huq zmm1,zmm2,zmm23
vpmadd52luq zmm25,zmm2,zmm21
vpmadd52huq zmm7,zmm2,zmm28
vpmadd52huq zmm3,zmm2,zmm15
vpmadd52luq zmm4,zmm2,zmm18
vpmadd52luq zmm9,zmm2,zmm23
vpmadd52luq zmm1,zmm2,zmm15
vpaddq zmm2,zmm25,zmm7
vpsrlq zmm7,zmm2,0x34
vpandq zmm2,zmm2,zmm30
vpsubq zmm2,zmm2,ZMMWORD PTR [rsp+0x440]
vpaddq zmm4,zmm7,zmm4
vpsrlq zmm7,zmm4,0x34
vpandq zmm4,zmm4,zmm30
vpsubq zmm4,zmm4,ZMMWORD PTR [rsp+0x940]
vpaddq zmm7,zmm7,zmm9
vpsrlq zmm9,zmm7,0x34
vpandq zmm7,zmm7,zmm30
vpaddq zmm1,zmm9,zmm1
vpsrlq zmm9,zmm1,0x34
vpandq zmm1,zmm1,zmm30
vpsubq zmm1,zmm1,ZMMWORD PTR [rsp+0x980]
vpaddq zmm3,zmm9,zmm3
vpaddq zmm9,zmm2,QWORD BCST [rip+0xfffffffffff1df27]        # 98d8 <GCC_except_table830+0x137c>
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1debd]        # 9878 <GCC_except_table830+0x131c>
vpsubq zmm3,zmm3,ZMMWORD PTR [rsp+0x380]
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff1de23]        # 97f0 <GCC_except_table830+0x1294>
vpsraq zmm2,zmm9,0x34
vpaddq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff1df9a]        # 9978 <GCC_except_table830+0x141c>
vpaddq zmm2,zmm4,zmm2
vpsubq zmm4,zmm7,ZMMWORD PTR [rsp+0x9c0]
vpsraq zmm7,zmm2,0x34
vpandq zmm8,zmm2,zmm30
vmovdqa ymm2,YMMWORD PTR [rip+0xfffffffffff1e5df]        # 9fe0 <GCC_except_table830+0x1a84>
vpaddq zmm4,zmm4,zmm7
vpaddq zmm4,zmm4,QWORD BCST [rip+0xfffffffffff1df3f]        # 9950 <GCC_except_table830+0x13f4>
vmovdqu YMMWORD PTR [rsp+0x5c0],ymm2
mov    QWORD PTR [rsp+0x5e0],rax
vpsubq zmm23,zmm8,QWORD BCST [rsp+0x5c8]
vpsraq zmm7,zmm4,0x34
vpaddq zmm1,zmm1,zmm7
vpsraq zmm7,zmm1,0x34
vpaddq zmm3,zmm3,zmm7
vpandq zmm7,zmm4,zmm30
vpandq zmm4,zmm1,zmm30
vpsubq zmm19,zmm7,QWORD BCST [rsp+0x5d0]
vpsubq zmm18,zmm4,QWORD BCST [rsp+0x5d8]
vpandq zmm1,zmm3,zmm30
vpsubq zmm15,zmm1,QWORD BCST [rsp+0x5e0]
vmovups YMMWORD PTR [rsp+0x5c0],ymm17
mov    QWORD PTR [rsp+0x5e0],r12
vpbroadcastq zmm25,QWORD PTR [rsp+0x5c8]
vpbroadcastq zmm21,QWORD PTR [rsp+0x5d0]
vpbroadcastq zmm20,QWORD PTR [rsp+0x5d8]
vpbroadcastq zmm17,QWORD PTR [rsp+0x5e0]
vmovdqu YMMWORD PTR [rsp+0x5c0],ymm2
mov    QWORD PTR [rsp+0x5e0],rax
test   r14,r14
je     ec78f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1ef>
mov    eax,DWORD PTR [r11]
vmovdqa64 ZMMWORD PTR [rsp+0xf40],zmm23
vmovdqa64 ZMMWORD PTR [rsp+0x580],zmm19
vmovdqa64 ZMMWORD PTR [rsp+0x3c0],zmm18
vmovdqa64 ZMMWORD PTR [rsp+0x380],zmm15
vmovdqa64 ZMMWORD PTR [rsp+0x980],zmm9
vmovdqa64 ZMMWORD PTR [rsp+0x9c0],zmm8
vmovdqa64 ZMMWORD PTR [rsp+0x940],zmm7
vmovdqa64 ZMMWORD PTR [rsp+0x440],zmm4
vmovdqa64 ZMMWORD PTR [rsp+0x200],zmm3
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec77a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1da>
vpxor  xmm9,xmm9,xmm9
vpmadd52huq zmm9,zmm26,zmm5
vpxor  xmm4,xmm4,xmm4
vpmadd52huq zmm4,zmm16,zmm5
vpxor  xmm7,xmm7,xmm7
vpmadd52huq zmm7,zmm11,zmm5
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm1,zmm13,zmm5
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0x1c0]
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0x240]
vmovdqa64 zmm23,ZMMWORD PTR [rsp+0x180]
vmovdqa64 zmm28,ZMMWORD PTR [rsp+0x2c0]
vpxor  xmm3,xmm3,xmm3
vpmadd52luq zmm3,zmm26,zmm5
vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x300]
vpbroadcastq zmm15,QWORD PTR [rip+0xfffffffffff1ddba]        # 9948 <GCC_except_table830+0x13ec>
vpxor  xmm2,xmm2,xmm2
vpmadd52huq zmm2,zmm14,zmm5
shl    rax,0x7
vpmadd52luq zmm9,zmm16,zmm5
vpmadd52luq zmm4,zmm11,zmm5
vpmadd52luq zmm7,zmm13,zmm5
vpmadd52luq zmm1,zmm14,zmm5
vpbroadcastq zmm5,QWORD PTR [rip+0xfffffffffff1dbe2]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52huq zmm9,zmm19,zmm0
vpmadd52huq zmm4,zmm18,zmm0
vpmadd52huq zmm7,zmm23,zmm0
vpmadd52huq zmm1,zmm28,zmm0
vpmadd52luq zmm3,zmm19,zmm0
vpmadd52huq zmm2,zmm8,zmm0
vpmadd52luq zmm9,zmm18,zmm0
vpmadd52luq zmm4,zmm23,zmm0
vpmadd52luq zmm7,zmm28,zmm0
vpmadd52luq zmm1,zmm8,zmm0
vpxor  xmm0,xmm0,xmm0
vpmadd52luq zmm0,zmm3,zmm15
vpaddq zmm3,zmm3,zmm30
vpsrlq zmm3,zmm3,0x34
vpmadd52huq zmm4,zmm0,zmm25
vpmadd52huq zmm7,zmm0,zmm21
vpmadd52huq zmm1,zmm0,zmm20
vpmadd52huq zmm2,zmm0,zmm17
vpmadd52huq zmm3,zmm0,zmm5
vpmadd52luq zmm9,zmm0,zmm25
vpxor  xmm5,xmm5,xmm5
vpmadd52luq zmm4,zmm0,zmm21
vpmadd52luq zmm7,zmm0,zmm20
vpmadd52luq zmm1,zmm0,zmm17
vpmadd52huq zmm2,zmm13,zmm12
vpaddq zmm3,zmm9,zmm3
vpxor  xmm0,xmm0,xmm0
vpmadd52huq zmm0,zmm14,zmm12
vpmadd52luq zmm3,zmm26,zmm12
vpmadd52huq zmm4,zmm26,zmm12
vpmadd52huq zmm7,zmm16,zmm12
vpmadd52huq zmm1,zmm11,zmm12
vpmadd52luq zmm2,zmm14,zmm12
vpmadd52huq zmm0,zmm8,zmm6
vpmadd52luq zmm3,zmm19,zmm6
vpmadd52luq zmm4,zmm16,zmm12
vpmadd52luq zmm7,zmm11,zmm12
vpmadd52luq zmm1,zmm13,zmm12
vpmadd52huq zmm2,zmm28,zmm6
vpmadd52luq zmm5,zmm3,zmm15
vpaddq zmm3,zmm3,zmm30
vpsrlq zmm3,zmm3,0x34
vpmadd52huq zmm4,zmm19,zmm6
vpmadd52huq zmm7,zmm18,zmm6
vpmadd52huq zmm1,zmm23,zmm6
vpmadd52luq zmm2,zmm8,zmm6
vpmadd52huq zmm0,zmm5,zmm17
vpmadd52luq zmm4,zmm18,zmm6
vpmadd52luq zmm7,zmm23,zmm6
vpmadd52luq zmm1,zmm28,zmm6
vpbroadcastq zmm6,QWORD PTR [rip+0xfffffffffff1dab0]        # 97a0 <GCC_except_table830+0x1244>
vpmadd52huq zmm2,zmm5,zmm20
vpmadd52huq zmm0,zmm13,zmm24
vpmadd52luq zmm2,zmm5,zmm17
vpmadd52huq zmm7,zmm5,zmm25
vpmadd52huq zmm1,zmm5,zmm21
vpmadd52luq zmm4,zmm5,zmm25
vpmadd52luq zmm0,zmm14,zmm24
vpmadd52huq zmm3,zmm5,zmm6
vpmadd52huq zmm2,zmm11,zmm24
vpmadd52luq zmm7,zmm5,zmm21
vpmadd52luq zmm1,zmm5,zmm20
vpxor  xmm5,xmm5,xmm5
vpmadd52huq zmm0,zmm28,zmm10
vpaddq zmm3,zmm4,zmm3
vpmadd52luq zmm2,zmm13,zmm24
vpmadd52huq zmm7,zmm26,zmm24
vpmadd52huq zmm1,zmm16,zmm24
vpxor  xmm4,xmm4,xmm4
vpmadd52huq zmm4,zmm14,zmm24
vpmadd52luq zmm3,zmm26,zmm24
vpmadd52luq zmm0,zmm8,zmm10
vpmadd52huq zmm2,zmm23,zmm10
vpmadd52luq zmm7,zmm16,zmm24
vpmadd52luq zmm1,zmm11,zmm24
vpmadd52huq zmm4,zmm8,zmm10
vmovdqa64 zmm24,ZMMWORD PTR [rsp+0x940]
vpmadd52luq zmm3,zmm19,zmm10
vpmadd52luq zmm2,zmm28,zmm10
vpmadd52huq zmm7,zmm19,zmm10
vpmadd52huq zmm1,zmm18,zmm10
vpmadd52luq zmm5,zmm3,zmm15
vpaddq zmm3,zmm3,zmm30
vpsrlq zmm3,zmm3,0x34
vpmadd52luq zmm7,zmm18,zmm10
vpmadd52luq zmm1,zmm23,zmm10
vpmadd52huq zmm2,zmm5,zmm21
vpmadd52huq zmm3,zmm5,zmm6
vpmadd52huq zmm0,zmm5,zmm20
vpmadd52huq zmm4,zmm5,zmm17
vpmadd52huq zmm1,zmm5,zmm25
vpmadd52luq zmm7,zmm5,zmm25
vpmadd52luq zmm2,zmm5,zmm20
vpmadd52luq zmm0,zmm5,zmm17
vpmadd52huq zmm4,zmm13,zmm27
vpaddq zmm3,zmm7,zmm3
vpmadd52luq zmm1,zmm5,zmm21
vpxor  xmm7,xmm7,xmm7
vpxor  xmm5,xmm5,xmm5
vpmadd52huq zmm5,zmm14,zmm27
vpmadd52huq zmm2,zmm16,zmm27
vpmadd52luq zmm3,zmm26,zmm27
vpmadd52huq zmm0,zmm11,zmm27
vpmadd52luq zmm4,zmm14,zmm27
vpmadd52huq zmm1,zmm26,zmm27
vpmadd52huq zmm5,zmm8,zmm22
vpmadd52luq zmm2,zmm11,zmm27
vpmadd52luq zmm3,zmm19,zmm22
vpmadd52luq zmm0,zmm13,zmm27
vpmadd52huq zmm4,zmm28,zmm22
vpmadd52luq zmm1,zmm16,zmm27
vmovdqa64 zmm27,zmm6
vpmadd52huq zmm2,zmm18,zmm22
vpmadd52luq zmm7,zmm3,zmm15
vpmadd52huq zmm0,zmm23,zmm22
vpaddq zmm3,zmm3,zmm30
vpmadd52luq zmm4,zmm8,zmm22
vpsrlq zmm3,zmm3,0x34
vpmadd52huq zmm1,zmm19,zmm22
vpmadd52luq zmm2,zmm23,zmm22
vpmadd52huq zmm3,zmm7,zmm6
vpmadd52luq zmm0,zmm28,zmm22
vpmadd52huq zmm5,zmm7,zmm17
vpmadd52huq zmm4,zmm7,zmm20
vpmadd52luq zmm1,zmm18,zmm22
vmovdqa64 zmm22,ZMMWORD PTR [rsp+0x200]
vpmadd52huq zmm2,zmm7,zmm25
vpmadd52huq zmm0,zmm7,zmm21
vpmadd52huq zmm5,zmm13,zmm31
vpmadd52luq zmm4,zmm7,zmm17
vpmadd52luq zmm1,zmm7,zmm25
vpmadd52luq zmm2,zmm7,zmm21
vpmadd52luq zmm0,zmm7,zmm20
vpmadd52luq zmm5,zmm14,zmm31
vpxor  xmm7,xmm7,xmm7
vpmadd52huq zmm7,zmm14,zmm31
vpbroadcastq zmm14,QWORD PTR [rip+0xfffffffffff1d9b4]        # 98a8 <GCC_except_table830+0x134c>
vpmadd52huq zmm4,zmm11,zmm31
vpaddq zmm6,zmm1,zmm3
vpmadd52huq zmm2,zmm26,zmm31
vpmadd52luq zmm6,zmm26,zmm31
vpmadd52huq zmm0,zmm16,zmm31
vpmadd52luq zmm4,zmm13,zmm31
vpmadd52huq zmm5,zmm28,zmm29
vpmadd52huq zmm7,zmm8,zmm29
vmovdqa64 zmm26,ZMMWORD PTR [rsp+0x9c0]
vpmadd52luq zmm2,zmm16,zmm31
vpmadd52luq zmm6,zmm19,zmm29
vpmadd52luq zmm0,zmm11,zmm31
vpbroadcastq zmm16,QWORD PTR [rip+0xfffffffffff1d8a0]        # 97e8 <GCC_except_table830+0x128c>
vpmadd52huq zmm4,zmm23,zmm29
vpmadd52luq zmm5,zmm8,zmm29
vpmadd52huq zmm2,zmm19,zmm29
vpandq zmm19,zmm30,ZMMWORD PTR [rsp+0x980]
vpmadd52huq zmm0,zmm18,zmm29
vpmadd52luq zmm4,zmm28,zmm29
vpmadd52luq zmm0,zmm23,zmm29
vpmadd52luq zmm2,zmm18,zmm29
vmovdqa64 zmm23,ZMMWORD PTR [rsp+0x440]
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0x840]
vpaddq zmm3,zmm19,zmm14
vpsraq zmm9,zmm3,0x34
vpaddq zmm9,zmm9,ZMMWORD PTR [rsp+0xf40]
vpsraq zmm10,zmm9,0x34
vpaddq zmm10,zmm10,ZMMWORD PTR [rsp+0x580]
vpsraq zmm11,zmm10,0x34
vpaddq zmm11,zmm11,ZMMWORD PTR [rsp+0x3c0]
vpsraq zmm12,zmm11,0x34
vpaddq zmm12,zmm12,ZMMWORD PTR [rsp+0x380]
vpcmpltuq k1,zmm12,zmm16
vpandq zmm19{k1},zmm3,zmm30
vpxor  xmm3,xmm3,xmm3
vpmadd52luq zmm3,zmm6,zmm15
vpaddq zmm6,zmm6,zmm30
vpandq zmm26{k1},zmm9,zmm30
vpandq zmm23{k1},zmm11,zmm30
vpandq zmm24{k1},zmm10,zmm30
vmovdqa64 zmm22{k1},zmm12
vpsrlq zmm6,zmm6,0x34
vpsrlq zmm1,zmm26,0xc
vpmadd52huq zmm0,zmm3,zmm25
vpmadd52huq zmm4,zmm3,zmm21
vpmadd52luq zmm2,zmm3,zmm25
vpmadd52huq zmm6,zmm3,zmm27
vpmadd52huq zmm5,zmm3,zmm20
vpmadd52huq zmm7,zmm3,zmm17
vpmadd52luq zmm0,zmm3,zmm21
vpmadd52luq zmm4,zmm3,zmm20
vpaddq zmm2,zmm2,zmm6
vpmadd52luq zmm5,zmm3,zmm17
vpsrlq zmm3,zmm2,0x34
vpandq zmm2,zmm2,zmm30
vpsubq zmm2,zmm2,ZMMWORD PTR [rsp+0x1800]
vpaddq zmm0,zmm3,zmm0
vpsrlq zmm3,zmm0,0x34
vpaddq zmm3,zmm3,zmm4
vpandq zmm4,zmm0,zmm30
vpsubq zmm4,zmm4,ZMMWORD PTR [rsp+0x17c0]
vpsrlq zmm0,zmm3,0x34
vpandq zmm3,zmm3,zmm30
vpsubq zmm3,zmm3,ZMMWORD PTR [rsp+0x1780]
vpaddq zmm0,zmm0,zmm5
vpsrlq zmm5,zmm0,0x34
vpandq zmm6,zmm0,zmm30
vpaddq zmm0,zmm2,QWORD BCST [rip+0xfffffffffff1d825]        # 98d8 <GCC_except_table830+0x137c>
vpaddq zmm5,zmm5,zmm7
vpsubq zmm7,zmm6,ZMMWORD PTR [rsp+0x1740]
vpsubq zmm6,zmm5,ZMMWORD PTR [rsp+0x1700]
vpsraq zmm2,zmm0,0x34
vpaddq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff1d89e]        # 9978 <GCC_except_table830+0x141c>
vpandq zmm0,zmm0,zmm30
vpaddq zmm2,zmm4,zmm2
vpsraq zmm4,zmm2,0x34
vpandq zmm2,zmm2,zmm30
vpsubq zmm8,zmm2,QWORD BCST [rsp+0x5c8]
vpaddq zmm3,zmm3,zmm4
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff1d842]        # 9950 <GCC_except_table830+0x13f4>
vpandq zmm9,zmm3,zmm30
vpsraq zmm4,zmm3,0x34
vpaddq zmm3,zmm7,QWORD BCST [rip+0xfffffffffff1d753]        # 9878 <GCC_except_table830+0x131c>
vpsubq zmm5,zmm9,QWORD BCST [rsp+0x5d0]
vpaddq zmm7,zmm0,zmm14
vpsraq zmm10,zmm7,0x34
vpaddq zmm3,zmm3,zmm4
vpsraq zmm4,zmm3,0x34
vpandq zmm11,zmm3,zmm30
vpaddq zmm3,zmm6,QWORD BCST [rip+0xfffffffffff1d696]        # 97f0 <GCC_except_table830+0x1294>
vpsubq zmm6,zmm11,QWORD BCST [rsp+0x5d8]
vpaddq zmm13,zmm3,zmm4
vpaddq zmm4,zmm8,zmm10
vpandq zmm3,zmm13,zmm30
vpsubq zmm3,zmm3,QWORD BCST [rsp+0x5e0]
vpsraq zmm10,zmm4,0x34
vpaddq zmm5,zmm5,zmm10
vpsraq zmm10,zmm5,0x34
vpaddq zmm6,zmm6,zmm10
vpsraq zmm10,zmm6,0x34
vpaddq zmm3,zmm3,zmm10
vpcmpltuq k1,zmm3,zmm16
vpandq zmm9{k1},zmm5,zmm30
vmovdqa64 zmm5,ZMMWORD PTR [rsp+0x280]
vmovdqa64 zmm13{k1},zmm3
vpandq zmm11{k1},zmm6,zmm30
vpandq zmm2{k1},zmm4,zmm30
vpandq zmm0{k1},zmm7,zmm30
vmovdqa64 zmm7,ZMMWORD PTR [rsp+0xac0]
vpsllq zmm3,zmm5,0x34
vporq  zmm10,zmm3,ZMMWORD PTR [rsp+0xa80]
vpsrlq zmm3,zmm5,0xc
vmovdqa64 zmm5,ZMMWORD PTR [rsp+0xa40]
vpsllq zmm6,zmm5,0x28
vpsrlq zmm5,zmm5,0x18
vporq  zmm12,zmm6,zmm3
vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x340]
vpsllq zmm4,zmm6,0x1c
vpsrlq zmm3,zmm6,0x24
vmovdqa64 zmm6,ZMMWORD PTR [rsp+0xa00]
vporq  zmm14,zmm4,zmm5
vpsllq zmm5,ZMMWORD PTR [rsp+0x900],0x10
vpunpcklqdq zmm4,zmm10,zmm12
vpsllq zmm8,zmm6,0x34
vporq  zmm16,zmm8,ZMMWORD PTR [rsp+0x880]
vpsllq zmm8,zmm7,0x28
vpsrlq zmm7,zmm7,0x18
vporq  zmm15,zmm5,zmm3
vpsrlq zmm3,zmm6,0xc
vpsllq zmm6,zmm18,0x1c
vporq  zmm17,zmm8,zmm3
vpsllq zmm8,ZMMWORD PTR [rsp+0x400],0x10
vporq  zmm3,zmm6,zmm7
vpsrlq zmm7,zmm18,0x24
vpunpcklqdq zmm5,zmm14,zmm15
vinserti32x4 ymm27,ymm4,xmm5,0x1
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm27
vpunpcklqdq zmm6,zmm16,zmm17
vporq  zmm18,zmm8,zmm7
vpsllq zmm8,zmm26,0x34
vpunpcklqdq zmm7,zmm3,zmm18
vporq  zmm19,zmm8,zmm19
vpsllq zmm8,zmm24,0x28
vporq  zmm20,zmm8,zmm1
vinserti32x4 ymm28,ymm6,xmm7,0x1
vpsrlq zmm1,zmm24,0x18
vpsllq zmm8,zmm23,0x1c
vporq  zmm21,zmm8,zmm1
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm28
vpsrlq zmm1,zmm23,0x24
vpsllq zmm8,zmm22,0x10
vpbroadcastq zmm28,QWORD PTR [rip+0xfffffffffff1d495]        # 97a0 <GCC_except_table830+0x1244>
vporq  zmm22,zmm8,zmm1
vpsllq zmm1,zmm2,0x34
vpunpcklqdq zmm8,zmm19,zmm20
vporq  zmm23,zmm1,zmm0
vpsrlq zmm0,zmm2,0xc
vpsllq zmm1,zmm9,0x28
vpsllq zmm2,zmm13,0x10
vporq  zmm24,zmm1,zmm0
vpsrlq zmm0,zmm9,0x18
vpsllq zmm1,zmm11,0x1c
vporq  zmm25,zmm1,zmm0
vpsrlq zmm1,zmm11,0x24
vpunpcklqdq zmm0,zmm21,zmm22
vpunpcklqdq zmm9,zmm23,zmm24
vporq  zmm26,zmm2,zmm1
vinserti128 ymm2,ymm8,xmm0,0x1
vpunpcklqdq zmm1,zmm25,zmm26
vmovdqu YMMWORD PTR [r10+rax*1+0x40],ymm2
vinserti128 ymm2,ymm9,xmm1,0x1
vmovdqu YMMWORD PTR [r10+rax*1+0x60],ymm2
cmp    r14,0x1
je     ec78f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1ef>
mov    eax,DWORD PTR [r11+0x10]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec77a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1da>
vpunpckhqdq zmm10,zmm10,zmm12
vpunpckhqdq zmm12,zmm14,zmm15
vpunpckhqdq zmm15,zmm3,zmm18
vpunpckhqdq zmm11,zmm16,zmm17
vpunpckhqdq zmm14,zmm19,zmm20
vpunpckhqdq zmm2,zmm21,zmm22
vpunpckhqdq zmm13,zmm23,zmm24
vpunpckhqdq zmm3,zmm25,zmm26
shl    rax,0x7
vinserti32x4 ymm27,ymm10,xmm12,0x1
vinserti32x4 ymm16,ymm11,xmm15,0x1
vinserti32x4 ymm17,ymm14,xmm2,0x1
vinserti32x4 ymm18,ymm13,xmm3,0x1
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm27
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm16
vmovdqu64 YMMWORD PTR [r10+rax*1+0x40],ymm17
vmovdqu64 YMMWORD PTR [r10+rax*1+0x60],ymm18
cmp    r14,0x2
je     ec78f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1ef>
mov    eax,DWORD PTR [r11+0x20]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec77a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1da>
vshufi64x2 ymm16,ymm4,ymm5,0x3
vshufi64x2 ymm17,ymm6,ymm7,0x3
vshufi64x2 ymm18,ymm8,ymm0,0x3
vshufi64x2 ymm19,ymm9,ymm1,0x3
shl    rax,0x7
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm16
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm17
vmovdqu64 YMMWORD PTR [r10+rax*1+0x40],ymm18
vmovdqu64 YMMWORD PTR [r10+rax*1+0x60],ymm19
cmp    r14,0x3
je     ec78f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1ef>
mov    eax,DWORD PTR [r11+0x30]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec77a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1da>
vshufi64x2 ymm16,ymm10,ymm12,0x3
vshufi64x2 ymm17,ymm11,ymm15,0x3
vshufi64x2 ymm18,ymm14,ymm2,0x3
vshufi64x2 ymm19,ymm13,ymm3,0x3
shl    rax,0x7
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm16
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm17
vmovdqu64 YMMWORD PTR [r10+rax*1+0x40],ymm18
vmovdqu64 YMMWORD PTR [r10+rax*1+0x60],ymm19
cmp    r14,0x4
je     ec78f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1ef>
mov    eax,DWORD PTR [r11+0x40]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec77a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1da>
vextracti64x4 ymm5,zmm5,0x1
vextracti64x4 ymm4,zmm4,0x1
vextracti64x4 ymm7,zmm7,0x1
vextracti64x4 ymm6,zmm6,0x1
vextracti32x4 xmm18,zmm0,0x2
vextracti64x4 ymm8,zmm8,0x1
vextracti32x4 xmm19,zmm1,0x2
vextracti64x4 ymm9,zmm9,0x1
shl    rax,0x7
vinserti32x4 ymm16,ymm4,xmm5,0x1
vinserti32x4 ymm17,ymm6,xmm7,0x1
vinserti32x4 ymm18,ymm8,xmm18,0x1
vinserti32x4 ymm19,ymm9,xmm19,0x1
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm16
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm17
vmovdqu64 YMMWORD PTR [r10+rax*1+0x40],ymm18
vmovdqu64 YMMWORD PTR [r10+rax*1+0x60],ymm19
cmp    r14,0x5
je     ec78f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1ef>
mov    eax,DWORD PTR [r11+0x50]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec77a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1da>
vextracti64x4 ymm12,zmm12,0x1
vextracti64x4 ymm10,zmm10,0x1
vextracti64x4 ymm15,zmm15,0x1
vextracti64x4 ymm11,zmm11,0x1
vextracti32x4 xmm18,zmm2,0x2
vextracti64x4 ymm14,zmm14,0x1
vextracti32x4 xmm19,zmm3,0x2
vextracti64x4 ymm13,zmm13,0x1
shl    rax,0x7
vinserti32x4 ymm16,ymm10,xmm12,0x1
vinserti32x4 ymm17,ymm11,xmm15,0x1
vinserti32x4 ymm18,ymm14,xmm18,0x1
vinserti32x4 ymm19,ymm13,xmm19,0x1
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm16
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm17
vmovdqu64 YMMWORD PTR [r10+rax*1+0x40],ymm18
vmovdqu64 YMMWORD PTR [r10+rax*1+0x60],ymm19
cmp    r14,0x6
je     ec78f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1ef>
mov    eax,DWORD PTR [r11+0x60]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec77a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1da>
vextracti64x4 ymm0,zmm0,0x1
vextracti64x4 ymm1,zmm1,0x1
vperm2i128 ymm4,ymm4,ymm5,0x31
vperm2i128 ymm5,ymm6,ymm7,0x31
shl    rax,0x7
vperm2i128 ymm0,ymm8,ymm0,0x31
vperm2i128 ymm1,ymm9,ymm1,0x31
vmovdqu YMMWORD PTR [r10+rax*1],ymm4
vmovdqu YMMWORD PTR [r10+rax*1+0x20],ymm5
vmovdqu YMMWORD PTR [r10+rax*1+0x40],ymm0
vmovdqu YMMWORD PTR [r10+rax*1+0x60],ymm1
cmp    r14,0x7
je     ec78f <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1ef>
mov    eax,DWORD PTR [r11+0x70]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec77a <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1da>
vextracti64x4 ymm2,zmm2,0x1
vperm2i128 ymm0,ymm10,ymm12,0x31
vperm2i128 ymm1,ymm11,ymm15,0x31
shl    rax,0x7
vextracti64x4 ymm3,zmm3,0x1
vpbroadcastq zmm21,QWORD PTR [rip+0xfffffffffff1d2b0]        # 9948 <GCC_except_table830+0x13ec>
mov    r9,QWORD PTR [rsp+0x28]
add    r14,0x8
vperm2i128 ymm2,ymm14,ymm2,0x31
vperm2i128 ymm3,ymm13,ymm3,0x31
vmovdqu YMMWORD PTR [r10+rax*1],ymm0
vmovdqu YMMWORD PTR [r10+rax*1+0x20],ymm1
vmovdqu YMMWORD PTR [r10+rax*1+0x40],ymm2
vmovdqa ymm2,YMMWORD PTR [rip+0xfffffffffff1d797]        # 9e60 <GCC_except_table830+0x1904>
vmovdqu YMMWORD PTR [r10+rax*1+0x60],ymm3
mov    rax,QWORD PTR [rsp+0x480]
mov    rbx,rax
test   rax,rax
jne    e7ba0 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x7600>
lea    rsp,[rbp-0x28]
pop    rbx
pop    r12
pop    r13
pop    r14
pop    r15
pop    rbp
vzeroupper
ret
lea    rdi,[rip+0xfffffffffff23bf3]        # 102f0 <_RNvNtNtNtNtNtCsgxBkk5gSRhY_4core3num3imp7flt2dec8strategy6dragon9POW5TO256+0x27c>
lea    rdx,[rip+0x455bc]        # 131cc0 <__frame_dummy_init_array_entry+0xcd0>
mov    esi,0x5d
vzeroupper
call   3b660 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
mov    rdx,QWORD PTR [rsp+0x18]
lea    rcx,[rip+0x45e43]        # 132560 <__frame_dummy_init_array_entry+0x1570>
vzeroupper
call   3b590 <_RNvNtNtCsgxBkk5gSRhY_4core5slice5index16slice_index_fail>
mov    edx,0x280
mov    rsi,rbx
vmovdqa64 ZMMWORD PTR [rsp+0x880],zmm16
vmovdqa64 ZMMWORD PTR [rsp+0xa80],zmm17
vmovdqa64 ZMMWORD PTR [rsp+0xa40],zmm18
vmovdqa64 ZMMWORD PTR [rsp+0xa00],zmm19
vzeroupper
call   48070 <_RINvNvMs2_NtCs6i54tJFfzR_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsh5u81WWlmio_10zopro_core>
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0xa00]
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0xa40]
vmovdqa64 zmm17,ZMMWORD PTR [rsp+0xa80]
vmovdqa64 zmm16,ZMMWORD PTR [rsp+0x880]
jmp    e06a7 <_RNvMs1_NtNtCsh5u81WWlmio_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x107>
lea    rdx,[rip+0x46277]        # 1329f8 <__frame_dummy_init_array_entry+0x1a08>
mov    rdi,rax
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x46245]        # 1329e0 <__frame_dummy_init_array_entry+0x19f0>
mov    rsi,rdi
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rcx,rax
lea    rdx,[rip+0x45d98]        # 132548 <__frame_dummy_init_array_entry+0x1558>
mov    rdi,rcx
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x4
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45d32]        # 132500 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
inc    r14
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45d18]        # 132500 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x2
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45cfd]        # 132500 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x3
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45ce2]        # 132500 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x7
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45cc7]        # 132500 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x5
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45cac]        # 132500 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x6
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45c91]        # 132500 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r8
lea    rdx,[rip+0x45c94]        # 132518 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r10
lea    rdx,[rip+0x45c7f]        # 132518 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rdx
lea    rdx,[rip+0x45c6a]        # 132518 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r9
lea    rdx,[rip+0x45c55]        # 132518 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rcx
lea    rdx,[rip+0x45c40]        # 132518 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rsi
lea    rdx,[rip+0x45c2b]        # 132518 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rax
lea    rdx,[rip+0x45c16]        # 132518 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rsi
mov    rsi,QWORD PTR [rsp+0x20]
lea    rdx,[rip+0x4607c]        # 132998 <__frame_dummy_init_array_entry+0x19a8>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
lea    rdx,[rip+0x46025]        # 132950 <__frame_dummy_init_array_entry+0x1960>
mov    rdi,rax
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rsi,QWORD PTR [rsp+0x20]
lea    rdx,[rip+0x4603e]        # 132980 <__frame_dummy_init_array_entry+0x1990>
mov    rdi,rax
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x4600f]        # 132968 <__frame_dummy_init_array_entry+0x1978>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r13,QWORD PTR [rsp+0x20]
mov    rcx,r8
lea    rdx,[rip+0x45bd8]        # 132548 <__frame_dummy_init_array_entry+0x1558>
mov    rdi,rcx
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r13
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45ba3]        # 132530 <__frame_dummy_init_array_entry+0x1540>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r10
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b8c]        # 132530 <__frame_dummy_init_array_entry+0x1540>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rax
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b75]        # 132530 <__frame_dummy_init_array_entry+0x1540>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rdx
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b5e]        # 132530 <__frame_dummy_init_array_entry+0x1540>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r12
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b47]        # 132530 <__frame_dummy_init_array_entry+0x1540>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r15
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b30]        # 132530 <__frame_dummy_init_array_entry+0x1540>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r9
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b19]        # 132530 <__frame_dummy_init_array_entry+0x1540>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
lea    rdx,[rip+0x45fa2]        # 1329c8 <__frame_dummy_init_array_entry+0x19d8>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
lea    rdx,[rip+0x45f7b]        # 1329b0 <__frame_dummy_init_array_entry+0x19c0>
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r13,QWORD PTR [rsp+0x20]
lea    rdx,[rip+0x45aff]        # 132548 <__frame_dummy_init_array_entry+0x1558>
mov    rdi,rcx
mov    rsi,r13
vzeroupper
call   3a63b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>