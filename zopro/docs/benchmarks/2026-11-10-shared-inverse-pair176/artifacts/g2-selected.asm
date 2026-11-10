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
sub    rsp,0x9c0
vmovdqa ymm0,YMMWORD PTR [rip+0xfffffffffff29a61]        # a060 <GCC_except_table830+0x1b1c>
movabs rax,0x1f16424e1bb7
vpbroadcastq zmm20,QWORD PTR [rip+0xfffffffffff292a5]        # 98b8 <GCC_except_table830+0x1374>
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
mov    QWORD PTR [rsp+0x1140],rbx
mov    QWORD PTR [rsp+0x1100],r15
vmovdqu YMMWORD PTR [rsp+0x600],ymm0
mov    QWORD PTR [rsp+0x620],rax
vmovdqa64 ZMMWORD PTR [rsp+0x14c0],zmm20
vpbroadcastq zmm19,QWORD PTR [rsp+0x608]
vpbroadcastq zmm18,QWORD PTR [rsp+0x610]
vpbroadcastq zmm17,QWORD PTR [rsp+0x618]
vpbroadcastq zmm16,QWORD PTR [rsp+0x620]
vmovdqa64 ZMMWORD PTR [rsp+0x1500],zmm19
vmovdqa64 ZMMWORD PTR [rsp+0x1540],zmm18
vmovdqa64 ZMMWORD PTR [rsp+0x1580],zmm17
vmovdqa64 ZMMWORD PTR [rsp+0x15c0],zmm16
cmp    rbx,QWORD PTR [rdi]
ja     ec7db <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc20b>
test   rbx,rbx
je     e17e4 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1214>
vpbroadcastq zmm20,QWORD PTR [rip+0xfffffffffff291d7]        # 98b8 <GCC_except_table830+0x1374>
vmovdqa64 zmm22,ZMMWORD PTR [rip+0xfffffffffff28c95]        # 9380 <GCC_except_table830+0xe3c>
vmovdqa64 zmm23,ZMMWORD PTR [rip+0xfffffffffff28ccb]        # 93c0 <GCC_except_table830+0xe7c>
vpbroadcastq zmm24,QWORD PTR [rip+0xfffffffffff29049]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm25,QWORD PTR [rip+0xfffffffffff29097]        # 97a0 <GCC_except_table830+0x125c>
vpbroadcastq zmm28,QWORD PTR [rip+0xfffffffffff29115]        # 9828 <GCC_except_table830+0x12e4>
vpbroadcastq zmm29,QWORD PTR [rip+0xfffffffffff2917b]        # 9898 <GCC_except_table830+0x1354>
vpbroadcastq zmm30,QWORD PTR [rip+0xfffffffffff29221]        # 9948 <GCC_except_table830+0x1404>
mov    rax,QWORD PTR [rsp+0x30]
xor    r14d,r14d
lea    rbx,[rax+0x78]
jmp    e1383 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xdb3>
nop    DWORD PTR [rax+rax*1+0x0]
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x240]
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x300]
vinserti64x4 zmm0,zmm13,YMMWORD PTR [rsp+0x940],0x1
vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x280]
vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x400]
vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x4c0]
vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x9c0]
vmovdqa64 zmm10,ZMMWORD PTR [rsp+0x200]
vmovdqa64 zmm11,ZMMWORD PTR [rsp+0x500]
vmovdqa64 zmm12,ZMMWORD PTR [rsp+0x2c0]
vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x980]
mov    rax,QWORD PTR [rdi+0x8]
lea    rcx,[r15+r15*4]
inc    r15
add    r14,0x8
sub    rbx,0xffffffffffffff80
vinserti64x4 zmm2,zmm1,YMMWORD PTR [rsp+0x3c0],0x1
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
vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x1c0]
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
vpbroadcastq zmm14,QWORD PTR [rip+0xfffffffffff28fe5]        # 9990 <GCC_except_table830+0x144c>
vpandq zmm12,zmm12,zmm24
vpsubq zmm12,zmm12,zmm9
vpshldq zmm9,zmm8,zmm15,0x18
vpbroadcastq zmm15,QWORD PTR [rip+0xfffffffffff28df8]        # 97c0 <GCC_except_table830+0x127c>
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
vmovdqa64 zmm21,ZMMWORD PTR [rsp+0x180]
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
vmovaps ymm2,YMMWORD PTR [rip+0xfffffffffff28e64]        # 9e60 <GCC_except_table830+0x191c>
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x140],zmm5
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x180],zmm10
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x1c0],zmm9
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x200],zmm6
vmovdqa64 ZMMWORD PTR [rax+rcx*1+0x240],zmm0
vpaddq zmm5,zmm7,zmm24
vpxor  xmm7,xmm7,xmm7
vpmadd52huq zmm7,zmm16,zmm10
mov    QWORD PTR [rdi+0x10],r15
mov    r15,QWORD PTR [rsp+0x1100]
vpsrlq zmm5,zmm5,0x34
vpmadd52huq zmm5,zmm14,zmm25
vmovups YMMWORD PTR [rsp+0x600],ymm2
mov    QWORD PTR [rsp+0x620],r12
vpbroadcastq zmm3,QWORD PTR [rsp+0x608]
vpbroadcastq zmm2,QWORD PTR [rsp+0x610]
vpbroadcastq zmm1,QWORD PTR [rsp+0x618]
vpbroadcastq zmm15,QWORD PTR [rsp+0x620]
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
je     e178e <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x11be>
mov    rax,QWORD PTR [rsp+0x18]
mov    r8d,0x0
sub    rax,r14
cmovae r8,rax
jbe    ec91b <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc34b>
mov    edi,DWORD PTR [rbx-0x78]
and    edi,0x7fffffff
cmp    r13,rdi
jbe    ec9b3 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3e3>
cmp    r8,0x1
je     ec8ac <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc2dc>
mov    eax,DWORD PTR [rbx-0x68]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec9b0 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3e0>
cmp    r8,0x2
je     ec876 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc2a6>
mov    esi,DWORD PTR [rbx-0x58]
and    esi,0x7fffffff
cmp    r13,rsi
jbe    ec99b <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3cb>
cmp    r8,0x3
je     ec8c6 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc2f6>
mov    ecx,DWORD PTR [rbx-0x48]
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    ec986 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3b6>
cmp    r8,0x4
je     ec891 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc2c1>
mov    r9d,DWORD PTR [rbx-0x38]
and    r9d,0x7fffffff
cmp    r13,r9
jbe    ec971 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3a1>
cmp    r8,0x5
je     ec8e1 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc311>
mov    edx,DWORD PTR [rbx-0x28]
and    edx,0x7fffffff
cmp    r13,rdx
jbe    ec95c <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc38c>
cmp    r8,0x6
je     ec8fc <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc32c>
mov    r10d,DWORD PTR [rbx-0x18]
and    r10d,0x7fffffff
cmp    r13,r10
jbe    ec947 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc377>
cmp    r8,0x7
je     ec917 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc347>
mov    r8d,DWORD PTR [rbx-0x8]
and    r8d,0x7fffffff
cmp    r13,r8
jbe    ec932 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc362>
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
vmovaps YMMWORD PTR [rsp+0x940],ymm14
vmovups ymm14,YMMWORD PTR [r11+rcx*1]
vmovaps ZMMWORD PTR [rsp+0x240],zmm1
vmovups ymm1,YMMWORD PTR [rsi+0x20]
lea    rsi,[r11+r9*1]
vmovaps ZMMWORD PTR [rsp+0x300],zmm2
vmovaps YMMWORD PTR [rsp+0x340],ymm26
vinsertf64x4 zmm15,zmm1,YMMWORD PTR [rax+0x20],0x1
vmovups ymm2,YMMWORD PTR [rsi+0x20]
lea    rax,[r11+rdx*1]
lea    rsi,[r11+r10*1]
vmovaps ymm1,YMMWORD PTR [rip+0xfffffffffff2892e]        # 9e60 <GCC_except_table830+0x191c>
vmovups ymm4,YMMWORD PTR [rsi+0x20]
mov    rsi,QWORD PTR [rbx-0x70]
vmovaps ZMMWORD PTR [rsp+0x280],zmm0
vmovups ymm5,YMMWORD PTR [rsi]
vmovdqu ymm6,YMMWORD PTR [rsi+0x20]
mov    rsi,QWORD PTR [rbx-0x50]
vmovaps YMMWORD PTR [rsp+0x3c0],ymm14
vinsertf64x4 zmm14,zmm2,YMMWORD PTR [rax+0x20],0x1
vinsertf64x4 zmm2,zmm3,YMMWORD PTR [r11+r8*1],0x1
mov    rax,QWORD PTR [rbx-0x60]
vinsertf64x4 zmm3,zmm4,YMMWORD PTR [r11+r8*1+0x20],0x1
vmovaps ZMMWORD PTR [rsp+0x400],zmm15
vmovups ymm7,YMMWORD PTR [rsi]
vmovdqu ymm8,YMMWORD PTR [rsi+0x20]
mov    rsi,QWORD PTR [rbx-0x30]
vmovaps ZMMWORD PTR [rsp+0x4c0],zmm14
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
vmovaps ZMMWORD PTR [rsp+0x200],zmm2
vinsertf64x4 zmm2,zmm9,YMMWORD PTR [rax],0x1
vinserti64x4 zmm21,zmm10,YMMWORD PTR [rax+0x20],0x1
mov    rax,QWORD PTR [rbx]
vmovaps ZMMWORD PTR [rsp+0x500],zmm3
vinsertf64x4 zmm3,zmm11,YMMWORD PTR [rax],0x1
vmovaps ZMMWORD PTR [rsp+0x2c0],zmm2
vinsertf64x4 zmm2,zmm12,YMMWORD PTR [rax+0x20],0x1
vmovups YMMWORD PTR [rsp+0x600],ymm1
mov    QWORD PTR [rsp+0x620],r12
vbroadcastsd zmm0,QWORD PTR [rsp+0x618]
vpbroadcastq zmm27,QWORD PTR [rsp+0x608]
vpbroadcastq zmm26,QWORD PTR [rsp+0x610]
vpbroadcastq zmm31,QWORD PTR [rsp+0x620]
vmovaps ZMMWORD PTR [rsp+0x980],zmm3
vmovaps ZMMWORD PTR [rsp+0x1c0],zmm2
vmovaps ZMMWORD PTR [rsp+0x180],zmm0
cmp    r15,QWORD PTR [rdi]
jne    e0740 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x170>
vmovdqa64 ZMMWORD PTR [rsp+0x880],zmm16
vmovdqa64 ZMMWORD PTR [rsp+0xac0],zmm17
vmovdqa64 ZMMWORD PTR [rsp+0xa80],zmm18
vmovdqa64 ZMMWORD PTR [rsp+0xa40],zmm19
vmovdqa64 ZMMWORD PTR [rsp+0xb00],zmm20
vmovdqa64 ZMMWORD PTR [rsp+0x5c0],zmm31
vmovdqa64 ZMMWORD PTR [rsp+0x480],zmm26
vmovdqa64 ZMMWORD PTR [rsp+0x440],zmm27
vmovdqa64 ZMMWORD PTR [rsp+0x380],zmm13
vmovdqa64 ZMMWORD PTR [rsp+0xa00],zmm14
vmovdqa64 ZMMWORD PTR [rsp+0xc00],zmm15
vmovdqa64 ZMMWORD PTR [rsp+0xf40],zmm21
vzeroupper
call   bf2f0 <_RNvMs4_NtCs6i54tJFfzR_5alloc7raw_vecINtB5_6RawVecAINtNtNtCsgXZuNZuAPQq_11zopro_bn2545ifma818persistent_quarter4WideKy2_Ej2_E8grow_oneCse6ToTOequVP_10zopro_core>
vmovdqa64 zmm21,ZMMWORD PTR [rsp+0xf40]
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0xc00]
vmovdqa64 zmm14,ZMMWORD PTR [rsp+0xa00]
vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x380]
vmovdqa64 zmm27,ZMMWORD PTR [rsp+0x440]
vmovdqa64 zmm26,ZMMWORD PTR [rsp+0x480]
vmovdqa64 zmm31,ZMMWORD PTR [rsp+0x5c0]
vpbroadcastq zmm30,QWORD PTR [rip+0xfffffffffff28228]        # 9948 <GCC_except_table830+0x1404>
vpbroadcastq zmm29,QWORD PTR [rip+0xfffffffffff2816e]        # 9898 <GCC_except_table830+0x1354>
vpbroadcastq zmm28,QWORD PTR [rip+0xfffffffffff280f4]        # 9828 <GCC_except_table830+0x12e4>
vpbroadcastq zmm25,QWORD PTR [rip+0xfffffffffff28062]        # 97a0 <GCC_except_table830+0x125c>
vpbroadcastq zmm24,QWORD PTR [rip+0xfffffffffff28000]        # 9748 <GCC_except_table830+0x1204>
vmovdqa64 zmm23,ZMMWORD PTR [rip+0xfffffffffff27c6e]        # 93c0 <GCC_except_table830+0xe7c>
vmovdqa64 zmm22,ZMMWORD PTR [rip+0xfffffffffff27c24]        # 9380 <GCC_except_table830+0xe3c>
vmovdqa64 zmm20,ZMMWORD PTR [rsp+0xb00]
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0xa40]
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0xa80]
vmovdqa64 zmm17,ZMMWORD PTR [rsp+0xac0]
vmovdqa64 zmm16,ZMMWORD PTR [rsp+0x880]
mov    rdi,QWORD PTR [rsp+0x28]
jmp    e0740 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x170>
mov    rcx,QWORD PTR [rsp+0x30]
mov    rbx,QWORD PTR [rsp+0x1140]
xor    r14d,r14d
mov    QWORD PTR [rdi+0x28],0x0
mov    QWORD PTR [rsp+0x20],r13
cmp    r15,QWORD PTR [rsp+0x18]
jne    e1800 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1230>
lea    rdi,[rsp+0x40]
lea    rsi,[rsp+0x14c0]
vzeroupper
call   d7b80 <_RNvMs0_NtNtCsgXZuNZuAPQq_11zopro_bn2545ifma818persistent_quarterINtB5_4WideKy2_E15inverse_nonzero>
mov    r9,QWORD PTR [rsp+0x28]
mov    r10,QWORD PTR [rsp+0x38]
test   r14b,r14b
mov    r14,QWORD PTR [rsp+0x30]
je     e7ba5 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x75d5>
jmp    ec797 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1c7>
mov    rcx,QWORD PTR [rsp+0x30]
mov    r14b,0x1
mov    QWORD PTR [rdi+0x28],0x0
mov    QWORD PTR [rsp+0x20],r13
cmp    r15,QWORD PTR [rsp+0x18]
je     e17b2 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x11e2>
vmovaps ymm8,YMMWORD PTR [rip+0xfffffffffff28658]        # 9e60 <GCC_except_table830+0x191c>
vpsllq zmm0,zmm20,0x3
vpbroadcastq zmm6,QWORD PTR [rip+0xfffffffffff28117]        # 9930 <GCC_except_table830+0x13ec>
vpbroadcastq zmm7,QWORD PTR [rip+0xfffffffffff27f2d]        # 9750 <GCC_except_table830+0x120c>
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff27f1b]        # 9748 <GCC_except_table830+0x1204>
shl    rbx,0x7
mov    DWORD PTR [rsp+0x1380],r14d
mov    QWORD PTR [rsp+0xac0],r15
vpaddq zmm0,zmm0,zmm20
vpandq zmm0,zmm0,QWORD BCST [rip+0xfffffffffff27f17]        # 9768 <GCC_except_table830+0x1224>
lea    rax,[rbx+rcx*1+0x8]
vpmadd52luq zmm20,zmm0,QWORD BCST [rip+0xfffffffffff27f40]        # 97a0 <GCC_except_table830+0x125c>
vmovups YMMWORD PTR [rsp+0x600],ymm8
mov    QWORD PTR [rsp+0x620],r12
vpmullq zmm1,zmm0,QWORD BCST [rsp+0x608]
vpmullq zmm2,zmm0,QWORD BCST [rsp+0x610]
vpmullq zmm3,zmm0,QWORD BCST [rsp+0x618]
vpmullq zmm4,zmm0,QWORD BCST [rsp+0x620]
vmovups YMMWORD PTR [rsp+0x600],ymm8
mov    QWORD PTR [rsp+0x620],r12
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
vpsubq zmm12,zmm4,QWORD BCST [rsp+0x608]
vpsrlq zmm0,zmm3,0x4
vpternlogq zmm1,zmm9,zmm7,0xf8
vpsllq zmm9,zmm3,0x30
vpandq zmm3,zmm0,zmm7
vpsubq zmm3,zmm3,QWORD BCST [rsp+0x620]
vpandq zmm2,zmm9,zmm6
vpaddq zmm9,zmm5,QWORD BCST [rip+0xfffffffffff27f34]        # 98c0 <GCC_except_table830+0x137c>
vpternlogq zmm2,zmm10,zmm7,0xf8
vpsubq zmm7,zmm1,QWORD BCST [rsp+0x610]
vpsubq zmm8,zmm2,QWORD BCST [rsp+0x618]
vpsraq zmm10,zmm9,0x34
vpaddq zmm6,zmm12,zmm10
vpsraq zmm10,zmm6,0x34
vpaddq zmm7,zmm7,zmm10
vpsraq zmm10,zmm7,0x34
vpaddq zmm8,zmm8,zmm10
vpsraq zmm10,zmm8,0x34
vpaddq zmm3,zmm3,zmm10
vpcmpltuq k1,zmm3,QWORD BCST [rip+0xfffffffffff27e00]        # 97e8 <GCC_except_table830+0x12a4>
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
vmovdqa64 zmm6,ZMMWORD PTR [rip+0xfffffffffff279e7]        # 9440 <GCC_except_table830+0xefc>
vpunpcklqdq zmm2,zmm3,zmm4
vpunpckhqdq zmm3,zmm3,zmm4
vpunpcklqdq zmm4,zmm1,zmm0
vpunpckhqdq zmm0,zmm1,zmm0
vmovdqa64 zmm1,ZMMWORD PTR [rip+0xfffffffffff27985]        # 9400 <GCC_except_table830+0xebc>
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
jmp    e1e39 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1869>
nop    WORD PTR [rax+rax*1+0x0]
movabs rcx,0xc3df73e9278302b9
add    r8,rcx
mov    rdi,QWORD PTR [rsp+0xf80]
mov    QWORD PTR [rsp+rax*1+0x1600],r8
mov    QWORD PTR [rsp+rax*1+0x1608],rdx
mov    QWORD PTR [rsp+rax*1+0x1610],r13
mov    QWORD PTR [rsp+rax*1+0x1618],r14
mov    r8,QWORD PTR [rsp+0x1000]
mov    rax,QWORD PTR [rsp+0xfc0]
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
mov    r15,QWORD PTR [rsp+0xa80]
mov    r14,r11
mov    rbx,rsi
mov    r12,r10
mov    QWORD PTR [rsp+0x40],rdi
mov    rdi,QWORD PTR [rsp+0x11c0]
mov    QWORD PTR [rsp+0x48],r8
lea    r8,[rsp+0x40]
mov    QWORD PTR [rsp+0x50],rdi
mov    QWORD PTR [rsp+0x58],rax
lea    rdi,[rsp+0x600]
add    r12,QWORD PTR [r8]
adc    r14,QWORD PTR [r8+0x8]
adc    r15,QWORD PTR [r8+0x10]
adc    rbx,QWORD PTR [r8+0x18]
mov    rax,r12
mov    rcx,r14
mov    rdx,r15
mov    rsi,rbx
sub    rax,QWORD PTR [rdi]
sbb    rcx,QWORD PTR [rdi+0x8]
sbb    rdx,QWORD PTR [rdi+0x10]
sbb    rsi,QWORD PTR [rdi+0x18]
cmovae r12,rax
cmovae r14,rcx
cmovae r15,rdx
cmovae rbx,rsi
mov    QWORD PTR [rsp+0x5c0],r14
mov    QWORD PTR [rsp+0x880],rbx
mov    QWORD PTR [rsp+0x440],r12
mov    QWORD PTR [rsp+0x480],r15
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
mov    rax,QWORD PTR [rsp+0xb00]
mov    r14,QWORD PTR [rsp+0x10c0]
mov    rbx,QWORD PTR [rsp+0x1080]
mov    r12,QWORD PTR [rsp+0x4c0]
mov    r15,QWORD PTR [rsp+0x9c0]
mov    QWORD PTR [rsp+0x40],rax
mov    rax,QWORD PTR [rsp+0x1040]
mov    QWORD PTR [rsp+0x48],r14
mov    QWORD PTR [rsp+0x50],rbx
mov    r14,QWORD PTR [rsp+0x980]
mov    rbx,QWORD PTR [rsp+0x940]
mov    QWORD PTR [rsp+0x58],rax
add    r12,QWORD PTR [r8]
adc    r14,QWORD PTR [r8+0x8]
adc    r15,QWORD PTR [r8+0x10]
adc    rbx,QWORD PTR [r8+0x18]
mov    rax,r12
mov    rcx,r14
mov    rdx,r15
mov    rsi,rbx
sub    rax,QWORD PTR [rdi]
sbb    rcx,QWORD PTR [rdi+0x8]
sbb    rdx,QWORD PTR [rdi+0x10]
sbb    rsi,QWORD PTR [rdi+0x18]
cmovae r12,rax
cmovae r14,rcx
cmovae r15,rdx
cmovae rbx,rsi
mov    r13,QWORD PTR [r9+0x28]
cmp    r13,QWORD PTR [r9+0x18]
je     e48d4 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x4304>
mov    rcx,QWORD PTR [r9+0x20]
mov    rdx,QWORD PTR [rsp+0xa80]
mov    rdi,QWORD PTR [rsp+0xa40]
lea    rax,[r13+r13*4+0x0]
mov    rsi,QWORD PTR [rsp+0x4c0]
inc    r13
shl    rax,0x6
mov    QWORD PTR [rcx+rax*1],r10
mov    QWORD PTR [rcx+rax*1+0x8],r11
mov    QWORD PTR [rcx+rax*1+0x10],rdx
mov    QWORD PTR [rcx+rax*1+0x18],rdi
mov    rdi,QWORD PTR [rsp+0x980]
mov    QWORD PTR [rcx+rax*1+0x20],rsi
mov    rsi,QWORD PTR [rsp+0x9c0]
mov    rdx,QWORD PTR [rsp+0x440]
mov    QWORD PTR [rcx+rax*1+0x28],rdi
mov    rdi,QWORD PTR [rsp+0x940]
mov    QWORD PTR [rcx+rax*1+0x30],rsi
mov    rsi,QWORD PTR [rsp+0x5c0]
mov    QWORD PTR [rcx+rax*1+0x38],rdi
mov    rdi,QWORD PTR [rsp+0x480]
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x1400]
vmovdqu64 ZMMWORD PTR [rcx+rax*1+0x40],zmm0
mov    QWORD PTR [rcx+rax*1+0x80],rdx
mov    QWORD PTR [rcx+rax*1+0x88],rsi
mov    rsi,QWORD PTR [rsp+0x880]
mov    QWORD PTR [rcx+rax*1+0x90],rdi
mov    rdi,QWORD PTR [rsp+0x180]
vpxor  xmm0,xmm0,xmm0
mov    QWORD PTR [rcx+rax*1+0x98],rsi
mov    rsi,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rcx+rax*1+0xa0],r12
mov    QWORD PTR [rcx+rax*1+0xa8],r14
mov    QWORD PTR [rcx+rax*1+0xb0],r15
mov    QWORD PTR [rcx+rax*1+0xb8],rbx
mov    QWORD PTR [rcx+rax*1+0xc0],rdi
mov    rdi,QWORD PTR [rsp+0x240]
mov    QWORD PTR [rcx+rax*1+0xc8],rsi
mov    rsi,QWORD PTR [rsp+0x300]
mov    QWORD PTR [rcx+rax*1+0xd0],rdi
mov    rdi,QWORD PTR [rsp+0x200]
mov    QWORD PTR [rcx+rax*1+0xd8],rsi
mov    rsi,QWORD PTR [rsp+0x280]
vmovdqu YMMWORD PTR [rcx+rax*1+0xe0],ymm0
mov    QWORD PTR [rcx+rax*1+0x100],rdi
mov    rdi,QWORD PTR [rsp+0x400]
mov    QWORD PTR [rcx+rax*1+0x108],rsi
mov    rsi,QWORD PTR [rsp+0x500]
mov    QWORD PTR [rcx+rax*1+0x110],rdi
mov    rdi,QWORD PTR [rsp+0x2c0]
mov    QWORD PTR [rcx+rax*1+0x118],rsi
mov    rsi,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rcx+rax*1+0x120],rdi
mov    rdi,QWORD PTR [rsp+0x340]
mov    QWORD PTR [rcx+rax*1+0x128],rsi
mov    rsi,QWORD PTR [rsp+0x900]
mov    QWORD PTR [rcx+rax*1+0x130],rdi
mov    QWORD PTR [rcx+rax*1+0x138],rsi
mov    rcx,QWORD PTR [rsp+0xac0]
mov    rax,QWORD PTR [rsp+0xf40]
mov    QWORD PTR [r9+0x28],r13
inc    rcx
add    rax,0x10
mov    QWORD PTR [rsp+0xac0],rcx
cmp    rcx,QWORD PTR [rsp+0x18]
jae    e48fa <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x432a>
mov    rdx,QWORD PTR [rax]
mov    ecx,DWORD PTR [rax-0x8]
mov    QWORD PTR [rsp+0xf40],rax
vmovups zmm0,ZMMWORD PTR [rdx]
mov    QWORD PTR [rsp+0x300],rcx
vmovaps ZMMWORD PTR [rsp+0x600],zmm0
test   ecx,ecx
js     e1ea0 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x18d0>
mov    rsi,QWORD PTR [rdx+0x68]
mov    rdi,QWORD PTR [rdx+0x70]
mov    r12,QWORD PTR [rdx+0x40]
mov    rax,QWORD PTR [rdx+0x48]
mov    r14,QWORD PTR [rdx+0x50]
mov    r11,QWORD PTR [rdx+0x58]
mov    rcx,QWORD PTR [rdx+0x60]
mov    rdx,QWORD PTR [rdx+0x78]
mov    QWORD PTR [rsp+0x1c0],rsi
mov    QWORD PTR [rsp+0x240],rdi
mov    QWORD PTR [rsp+0x180],rdx
jmp    e1f9d <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x19cd>
xchg   ax,ax
lea    rax,[rdx+0x40]
vmovdqu ymm2,YMMWORD PTR [rip+0xfffffffffff280b4]        # 9f60 <GCC_except_table830+0x1a1c>
xor    r12d,r12d
xor    r14d,r14d
xor    r11d,r11d
lea    rsi,[rsp+0xc40]
lea    r9,[rsp+0x40]
lea    r13,[rsp+0xc40]
vmovups ymm0,YMMWORD PTR [rax]
xor    eax,eax
vmovdqa YMMWORD PTR [rsp+0xc40],ymm2
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
mov    QWORD PTR [rsp+0x200],r11
xor    ecx,ecx
xor    esi,esi
xor    r9d,r9d
xor    r8d,r8d
lea    r11,[rsp+0x40]
vmovdqu ymm1,YMMWORD PTR [rdx+0x60]
vmovdqa YMMWORD PTR [rsp+0xc40],ymm2
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
mov    QWORD PTR [rsp+0x240],r9
mov    QWORD PTR [rsp+0x1c0],rsi
mov    QWORD PTR [rsp+0x180],r8
mov    r11,QWORD PTR [rsp+0x200]
mov    r15d,r12d
and    r15d,0x1
mov    rdi,r14
shld   rdi,rax,0x3e
shld   rax,r12,0x3e
movabs rdx,0x4f082305b61f3f52
shl    r12,0x3e
xor    r13d,r13d
movabs rbx,0x65e05aa45a1c72a3
vmovaps zmm0,ZMMWORD PTR [rsp+0x600]
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
vmovaps ZMMWORD PTR [rsp+0xc40],zmm0
add    rbx,r8
movabs r8,0x183227397098d014
setb   dil
add    rbx,r14
adc    rdi,0x0
and    r15,rax
and    r12,r8
shr    r11,0x2
mov    rax,QWORD PTR [rsp+0x1c0]
add    r12,r15
add    r12,r11
add    rsi,r9
mov    r11d,ecx
movabs r9,0xcbc0b548b438e546
mov    QWORD PTR [rsp+0x200],rsi
mov    rsi,QWORD PTR [rsp+0x240]
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
mov    r9,QWORD PTR [rsp+0x180]
and    r8,rax
mov    rax,r11
mov    QWORD PTR [rsp+0x1c0],rdx
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
mov    rsi,QWORD PTR [rsp+0x300]
mov    rax,QWORD PTR [rsp+0x200]
add    rcx,r11
add    rcx,r9
add    r8,r15
adc    r14,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0xc80],rax
mov    QWORD PTR [rsp+0xc88],r10
mov    QWORD PTR [rsp+0xc90],rbx
mov    QWORD PTR [rsp+0xc98],r12
mov    QWORD PTR [rsp+0xca0],r8
adc    r13,rdx
adc    rcx,rdi
and    esi,0x7fffffff
mov    QWORD PTR [rsp+0xca8],r14
mov    QWORD PTR [rsp+0xcb0],r13
mov    QWORD PTR [rsp+0xcb8],rcx
cmp    QWORD PTR [rsp+0x20],rsi
jbe    ec9c5 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc3f5>
shl    rsi,0x7
add    rsi,QWORD PTR [rsp+0x38]
lea    r13,[rsp+0x600]
mov    rdi,r13
vzeroupper
call   ecf60 <_RNvMs_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB4_8G2Bucket6scaled>
mov    rcx,QWORD PTR [rsp+0x630]
mov    rdx,QWORD PTR [rsp+0x628]
mov    rax,QWORD PTR [rsp+0x620]
mov    rbx,QWORD PTR [rsp+0x600]
mov    r14,QWORD PTR [rsp+0x608]
mov    r15,QWORD PTR [rsp+0x610]
mov    r12,QWORD PTR [rsp+0x618]
lea    rsi,[rsp+0xc40]
mov    rdi,r13
mov    QWORD PTR [rsp+0x9c0],rcx
lea    rcx,[rsp+0x640]
mov    QWORD PTR [rsp+0x980],rdx
mov    rdx,QWORD PTR [rsp+0x638]
mov    QWORD PTR [rsp+0x4c0],rax
vmovups zmm0,ZMMWORD PTR [rcx]
mov    QWORD PTR [rsp+0x940],rdx
vmovaps ZMMWORD PTR [rsp+0x1400],zmm0
vzeroupper
call   ecf60 <_RNvMs_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB4_8G2Bucket6scaled>
mov    rdi,QWORD PTR [rsp+0x620]
vmovdqu ymm1,YMMWORD PTR [rip+0xfffffffffff27cb8]        # 9f60 <GCC_except_table830+0x1a1c>
mov    rax,QWORD PTR [rsp+0x600]
mov    rcx,QWORD PTR [rsp+0x608]
mov    r13,QWORD PTR [rsp+0x610]
mov    rsi,QWORD PTR [rsp+0x618]
mov    QWORD PTR [rsp+0x40],rbx
mov    QWORD PTR [rsp+0xa00],rbx
mov    QWORD PTR [rsp+0x48],r14
mov    QWORD PTR [rsp+0xc00],r14
mov    QWORD PTR [rsp+0x50],r15
mov    QWORD PTR [rsp+0xa80],r15
mov    r10,QWORD PTR [rsp+0x628]
mov    r11,QWORD PTR [rsp+0x630]
mov    r9,QWORD PTR [rsp+0x638]
mov    QWORD PTR [rsp+0xa40],r12
mov    QWORD PTR [rsp+0x58],r12
lea    rbx,[rsp+0x40]
lea    r8,[rsp+0x600]
mov    QWORD PTR [rsp+0xb00],rdi
mov    rdi,QWORD PTR [rsp+0x640]
mov    QWORD PTR [rsp+0x11c0],r13
mov    QWORD PTR [rsp+0x1000],rcx
mov    QWORD PTR [rsp+0xfc0],rsi
mov    QWORD PTR [rsp+0xf80],rax
mov    r12,rcx
mov    r15,rsi
mov    r14,rax
mov    QWORD PTR [rsp+0x280],rdi
mov    rdi,QWORD PTR [rsp+0x648]
mov    QWORD PTR [rsp+0x400],rdi
mov    rdi,QWORD PTR [rsp+0x650]
mov    QWORD PTR [rsp+0x500],rdi
mov    rdi,QWORD PTR [rsp+0x658]
mov    QWORD PTR [rsp+0x2c0],rdi
mov    rdi,QWORD PTR [rsp+0x660]
mov    QWORD PTR [rsp+0x340],rdi
mov    rdi,QWORD PTR [rsp+0x668]
mov    QWORD PTR [rsp+0x900],rdi
mov    rdi,QWORD PTR [rsp+0x670]
mov    QWORD PTR [rsp+0x880],rdi
mov    rdi,QWORD PTR [rsp+0x678]
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
mov    QWORD PTR [rsp+0x200],rdi
sub    r14,QWORD PTR [rbx]
sbb    r12,QWORD PTR [rbx+0x8]
sbb    r13,QWORD PTR [rbx+0x10]
sbb    r15,QWORD PTR [rbx+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r8]
mov    rcx,QWORD PTR [r8+0x8]
mov    rdx,QWORD PTR [r8+0x10]
mov    rsi,QWORD PTR [r8+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    r14,rax
adc    r12,rcx
adc    r13,rdx
adc    r15,rsi
mov    QWORD PTR [rsp+0x3c0],r13
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
mov    QWORD PTR [rsp+0x10c0],r10
mov    QWORD PTR [rsp+0x1040],r9
mov    QWORD PTR [rsp+0x1080],r11
lea    rbx,[rsp+0x40]
lea    r13,[rsp+0x600]
mov    rax,QWORD PTR [rsp+0x4c0]
mov    r8,QWORD PTR [rsp+0x980]
mov    rdi,QWORD PTR [rsp+0x9c0]
mov    QWORD PTR [rsp+0x40],rax
mov    rax,QWORD PTR [rsp+0x940]
mov    QWORD PTR [rsp+0x48],r8
mov    QWORD PTR [rsp+0x50],rdi
mov    r8,r9
mov    rdi,QWORD PTR [rsp+0xb00]
mov    QWORD PTR [rsp+0x58],rax
sub    rdi,QWORD PTR [rbx]
sbb    r10,QWORD PTR [rbx+0x8]
sbb    r11,QWORD PTR [rbx+0x10]
sbb    r8,QWORD PTR [rbx+0x18]
sbb    r9,r9
mov    rax,QWORD PTR [r13+0x0]
mov    rcx,QWORD PTR [r13+0x8]
mov    rdx,QWORD PTR [r13+0x10]
mov    rsi,QWORD PTR [r13+0x18]
and    rax,r9
and    rcx,r9
and    rdx,r9
and    rsi,r9
add    rdi,rax
adc    r10,rcx
adc    r11,rdx
adc    r8,rsi
mov    QWORD PTR [rsp+0x540],r14
mov    QWORD PTR [rsp+0x548],r12
mov    QWORD PTR [rsp+0x300],r11
mov    QWORD PTR [rsp+0x1c0],r8
mov    QWORD PTR [rsp+0x240],r10
mov    QWORD PTR [rsp+0x440],r14
mov    QWORD PTR [rsp+0x480],r12
mov    QWORD PTR [rsp+0x5c0],r15
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
mov    rbx,rdi
lea    r8,[rsp+0x540]
movabs r11,0x87d20782e4866389
mov    rax,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rsp+0x550],rax
mov    QWORD PTR [rsp+0x558],r15
mov    rcx,QWORD PTR [rsp+0x548]
mov    rax,QWORD PTR [rsp+0x540]
mov    rsi,QWORD PTR [rsp+0x550]
mov    QWORD PTR [rsp+0x48],rcx
mov    rcx,QWORD PTR [rsp+0x558]
mov    QWORD PTR [rsp+0x50],rsi
mov    QWORD PTR [rsp+0x40],rax
lea    rsi,[rsp+0x40]
mov    QWORD PTR [rsp+0x58],rcx
lea    rcx,[rsp+0x600]
xor    r13d,r13d
xor    eax,eax
xor    r10d,r10d
xor    r9d,r9d
xor    edi,edi
mov    rdx,QWORD PTR [r8]
xor    r12d,r12d
mulx   r15,r14,QWORD PTR [rsi]
adcx   r13,r14
adox   rax,r15
mulx   r15,r14,QWORD PTR [rsi+0x8]
adcx   rax,r14
adox   r10,r15
mulx   r15,r14,QWORD PTR [rsi+0x10]
adcx   r10,r14
adox   r9,r15
mulx   r15,r14,QWORD PTR [rsi+0x18]
adcx   r9,r14
adox   rdi,r15
adcx   rdi,r12
mov    rdx,r13
imul   rdx,r11
xor    r12d,r12d
mulx   r15,r14,QWORD PTR [rcx]
adcx   r13,r14
adox   rax,r15
mulx   r15,r14,QWORD PTR [rcx+0x8]
adcx   rax,r14
adox   r10,r15
mulx   r15,r14,QWORD PTR [rcx+0x10]
adcx   r10,r14
adox   r9,r15
mulx   r15,r14,QWORD PTR [rcx+0x18]
adcx   r9,r14
adox   rdi,r15
adcx   rdi,r12
mov    rdx,QWORD PTR [r8+0x8]
xor    r12d,r12d
mulx   r15,r14,QWORD PTR [rsi]
adcx   rax,r14
adox   r10,r15
mulx   r15,r14,QWORD PTR [rsi+0x8]
adcx   r10,r14
adox   r9,r15
mulx   r15,r14,QWORD PTR [rsi+0x10]
adcx   r9,r14
adox   rdi,r15
mulx   r15,r14,QWORD PTR [rsi+0x18]
adcx   rdi,r14
adox   r13,r15
adcx   r13,r12
mov    rdx,rax
imul   rdx,r11
xor    r12d,r12d
mulx   r15,r14,QWORD PTR [rcx]
adcx   rax,r14
adox   r10,r15
mulx   r15,r14,QWORD PTR [rcx+0x8]
adcx   r10,r14
adox   r9,r15
mulx   r15,r14,QWORD PTR [rcx+0x10]
adcx   r9,r14
adox   rdi,r15
mulx   r15,r14,QWORD PTR [rcx+0x18]
adcx   rdi,r14
adox   r13,r15
adcx   r13,r12
mov    rdx,QWORD PTR [r8+0x10]
xor    r12d,r12d
mulx   r15,r14,QWORD PTR [rsi]
adcx   r10,r14
adox   r9,r15
mulx   r15,r14,QWORD PTR [rsi+0x8]
adcx   r9,r14
adox   rdi,r15
mulx   r15,r14,QWORD PTR [rsi+0x10]
adcx   rdi,r14
adox   r13,r15
mulx   r15,r14,QWORD PTR [rsi+0x18]
adcx   r13,r14
adox   rax,r15
adcx   rax,r12
mov    rdx,r10
imul   rdx,r11
xor    r12d,r12d
mulx   r15,r14,QWORD PTR [rcx]
adcx   r10,r14
adox   r9,r15
mulx   r15,r14,QWORD PTR [rcx+0x8]
adcx   r9,r14
adox   rdi,r15
mulx   r15,r14,QWORD PTR [rcx+0x10]
adcx   rdi,r14
adox   r13,r15
mulx   r15,r14,QWORD PTR [rcx+0x18]
adcx   r13,r14
adox   rax,r15
adcx   rax,r12
mov    rdx,QWORD PTR [r8+0x18]
xor    r12d,r12d
mulx   r15,r14,QWORD PTR [rsi]
adcx   r9,r14
adox   rdi,r15
mulx   r15,r14,QWORD PTR [rsi+0x8]
adcx   rdi,r14
adox   r13,r15
mulx   r15,r14,QWORD PTR [rsi+0x10]
adcx   r13,r14
adox   rax,r15
mulx   r15,r14,QWORD PTR [rsi+0x18]
adcx   rax,r14
adox   r10,r15
adcx   r10,r12
mov    rdx,r9
imul   rdx,r11
xor    r12d,r12d
mulx   r15,r14,QWORD PTR [rcx]
adcx   r9,r14
adox   rdi,r15
mulx   r15,r14,QWORD PTR [rcx+0x8]
adcx   rdi,r14
adox   r13,r15
mulx   r15,r14,QWORD PTR [rcx+0x10]
adcx   r13,r14
adox   rax,r15
mulx   r15,r14,QWORD PTR [rcx+0x18]
adcx   rax,r14
adox   r10,r15
adcx   r10,r12
mov    QWORD PTR [rsp+0x180],rdi
mov    QWORD PTR [rsp+0x8e0],rbx
movabs rdi,0x97816a916871ca8d
mov    QWORD PTR [rsp+0x380],r10
movabs r8,0xcf9bb18d1ece5fd7
mov    QWORD PTR [rsp+0xf00],rax
mov    QWORD PTR [rsp+0xec0],r13
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
mov    rcx,QWORD PTR [rsp+0x240]
mov    rsi,QWORD PTR [rsp+0x300]
mov    rdx,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0x8e8],rcx
mov    QWORD PTR [rsp+0x8f0],rsi
mov    QWORD PTR [rsp+0x8f8],rdx
movabs rdx,0x3c208c16d87cfd47
mov    rcx,QWORD PTR [rsp+0x380]
lea    rsi,[rsp+0x600]
cmp    QWORD PTR [rsp+0x180],rdx
movabs rdx,0x687e956e978e3573
lea    rdx,[r13+rdx*1+0x0]
sbb    rdx,0x0
mov    QWORD PTR [rsp+0x1c0],rdx
setb   dl
cmp    r13,rdi
movabs r13,0x87d20782e4866389
setb   dil
or     dil,dl
movzx  edx,dil
movabs rdi,0x47afba497e7ea7a3
add    rdi,rax
sub    rdi,rdx
mov    QWORD PTR [rsp+0x240],rdi
movabs rdi,0xb85045b68181585d
setb   dl
cmp    rax,rdi
lea    rax,[rcx+r8*1]
lea    r8,[rsp+0x8e0]
setb   dil
or     dil,dl
movzx  edx,dil
sub    rax,rdx
mov    rdx,QWORD PTR [rsp+0x8e0]
mov    QWORD PTR [rsp+0x300],rax
setb   BYTE PTR [rsp+0xbc0]
lea    rax,[rsp+0x40]
mov    QWORD PTR [rsp+0x40],rdx
mov    rdx,QWORD PTR [rsp+0x8e8]
mov    QWORD PTR [rsp+0x48],rdx
mov    rdx,QWORD PTR [rsp+0x8f0]
mov    QWORD PTR [rsp+0x50],rdx
mov    rdx,QWORD PTR [rsp+0x8f8]
mov    QWORD PTR [rsp+0x58],rdx
xor    r14d,r14d
xor    ebx,ebx
xor    r10d,r10d
xor    r9d,r9d
xor    edi,edi
mov    rdx,QWORD PTR [r8]
xor    r12d,r12d
mulx   r15,r11,QWORD PTR [rax]
adcx   r14,r11
adox   rbx,r15
mulx   r15,r11,QWORD PTR [rax+0x8]
adcx   rbx,r11
adox   r10,r15
mulx   r15,r11,QWORD PTR [rax+0x10]
adcx   r10,r11
adox   r9,r15
mulx   r15,r11,QWORD PTR [rax+0x18]
adcx   r9,r11
adox   rdi,r15
adcx   rdi,r12
mov    rdx,r14
imul   rdx,r13
xor    r12d,r12d
mulx   r15,r11,QWORD PTR [rsi]
adcx   r14,r11
adox   rbx,r15
mulx   r15,r11,QWORD PTR [rsi+0x8]
adcx   rbx,r11
adox   r10,r15
mulx   r15,r11,QWORD PTR [rsi+0x10]
adcx   r10,r11
adox   r9,r15
mulx   r15,r11,QWORD PTR [rsi+0x18]
adcx   r9,r11
adox   rdi,r15
adcx   rdi,r12
mov    rdx,QWORD PTR [r8+0x8]
xor    r12d,r12d
mulx   r15,r11,QWORD PTR [rax]
adcx   rbx,r11
adox   r10,r15
mulx   r15,r11,QWORD PTR [rax+0x8]
adcx   r10,r11
adox   r9,r15
mulx   r15,r11,QWORD PTR [rax+0x10]
adcx   r9,r11
adox   rdi,r15
mulx   r15,r11,QWORD PTR [rax+0x18]
adcx   rdi,r11
adox   r14,r15
adcx   r14,r12
mov    rdx,rbx
imul   rdx,r13
xor    r12d,r12d
mulx   r15,r11,QWORD PTR [rsi]
adcx   rbx,r11
adox   r10,r15
mulx   r15,r11,QWORD PTR [rsi+0x8]
adcx   r10,r11
adox   r9,r15
mulx   r15,r11,QWORD PTR [rsi+0x10]
adcx   r9,r11
adox   rdi,r15
mulx   r15,r11,QWORD PTR [rsi+0x18]
adcx   rdi,r11
adox   r14,r15
adcx   r14,r12
mov    rdx,QWORD PTR [r8+0x10]
xor    r12d,r12d
mulx   r15,r11,QWORD PTR [rax]
adcx   r10,r11
adox   r9,r15
mulx   r15,r11,QWORD PTR [rax+0x8]
adcx   r9,r11
adox   rdi,r15
mulx   r15,r11,QWORD PTR [rax+0x10]
adcx   rdi,r11
adox   r14,r15
mulx   r15,r11,QWORD PTR [rax+0x18]
adcx   r14,r11
adox   rbx,r15
adcx   rbx,r12
mov    rdx,r10
imul   rdx,r13
xor    r12d,r12d
mulx   r15,r11,QWORD PTR [rsi]
adcx   r10,r11
adox   r9,r15
mulx   r15,r11,QWORD PTR [rsi+0x8]
adcx   r9,r11
adox   rdi,r15
mulx   r15,r11,QWORD PTR [rsi+0x10]
adcx   rdi,r11
adox   r14,r15
mulx   r15,r11,QWORD PTR [rsi+0x18]
adcx   r14,r11
adox   rbx,r15
adcx   rbx,r12
mov    rdx,QWORD PTR [r8+0x18]
xor    r12d,r12d
mulx   r15,r11,QWORD PTR [rax]
adcx   r9,r11
adox   rdi,r15
mulx   r15,r11,QWORD PTR [rax+0x8]
adcx   rdi,r11
adox   r14,r15
mulx   r15,r11,QWORD PTR [rax+0x10]
adcx   r14,r11
adox   rbx,r15
mulx   r15,r11,QWORD PTR [rax+0x18]
adcx   rbx,r11
adox   r10,r15
adcx   r10,r12
mov    rdx,r9
imul   rdx,r13
xor    r12d,r12d
mulx   r15,r11,QWORD PTR [rsi]
adcx   r9,r11
adox   rdi,r15
mulx   r15,r11,QWORD PTR [rsi+0x8]
adcx   rdi,r11
adox   r14,r15
mulx   r15,r11,QWORD PTR [rsi+0x10]
adcx   r14,r11
adox   rbx,r15
mulx   r15,r11,QWORD PTR [rsi+0x18]
adcx   rbx,r11
adox   r10,r15
adcx   r10,r12
movabs rdx,0x3c208c16d87cfd47
movabs rax,0x687e956e978e3573
movabs r8,0x30644e72e131a029
cmp    rdi,rdx
lea    rdx,[r14+rax*1]
movabs rax,0x97816a916871ca8d
sbb    rdx,0x0
setb   r9b
cmp    r14,rax
movabs rax,0x47afba497e7ea7a3
setb   r11b
or     r11b,r9b
movzx  r9d,r11b
lea    r11,[rbx+rax*1]
movabs rax,0xb85045b68181585d
sub    r11,r9
setb   r9b
cmp    rbx,rax
movabs rax,0xcf9bb18d1ece5fd7
setb   r15b
or     r15b,r9b
movzx  r9d,r15b
lea    r15,[r10+rax*1]
sub    r15,r9
setb   r12b
cmp    r10,r8
setb   r9b
cmp    rcx,r8
setb   r8b
or     r8b,BYTE PTR [rsp+0xbc0]
jne    e2d10 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x2740>
mov    rcx,QWORD PTR [rsp+0x180]
movabs rax,0xc3df73e9278302b9
add    rcx,rax
or     r9b,r12b
je     e2d45 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x2775>
mov    rdx,r14
mov    r11,rbx
mov    r15,r10
jmp    e2d52 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x2782>
cs nop WORD PTR [rax+rax*1+0x0]
mov    rsi,QWORD PTR [rsp+0xec0]
mov    rax,QWORD PTR [rsp+0xf00]
mov    QWORD PTR [rsp+0x300],rcx
mov    rcx,QWORD PTR [rsp+0x180]
mov    QWORD PTR [rsp+0x1c0],rsi
mov    QWORD PTR [rsp+0x240],rax
or     r9b,r12b
jne    e2cfb <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x272b>
movabs rax,0xc3df73e9278302b9
add    rdi,rax
mov    QWORD PTR [rsp+0x40],rdi
mov    QWORD PTR [rsp+0x48],rdx
mov    QWORD PTR [rsp+0x50],r11
mov    QWORD PTR [rsp+0x58],r15
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
lea    r11,[rsp+0x600]
lea    r14,[rsp+0x40]
mov    rdi,rcx
mov    r8,QWORD PTR [rsp+0x1c0]
mov    r9,QWORD PTR [rsp+0x240]
mov    r10,QWORD PTR [rsp+0x300]
xor    r12d,r12d
lea    r13,[rsp+0x600]
lea    r15,[rsp+0x40]
add    rdi,QWORD PTR [r14]
adc    r8,QWORD PTR [r14+0x8]
adc    r9,QWORD PTR [r14+0x10]
adc    r10,QWORD PTR [r14+0x18]
mov    rax,rdi
mov    rcx,r8
mov    rdx,r9
mov    rsi,r10
sub    rax,QWORD PTR [r11]
sbb    rcx,QWORD PTR [r11+0x8]
sbb    rdx,QWORD PTR [r11+0x10]
sbb    rsi,QWORD PTR [r11+0x18]
cmovae rdi,rax
cmovae r8,rcx
cmovae r9,rdx
cmovae r10,rsi
mov    QWORD PTR [rsp+0x300],r10
mov    QWORD PTR [rsp+0x240],r9
mov    QWORD PTR [rsp+0x1c0],r8
mov    QWORD PTR [rsp+0x180],rdi
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
vmovaps ymm0,YMMWORD PTR [rsp+0x1400]
mov    r8,QWORD PTR [rsp+0x2c0]
mov    r9,QWORD PTR [rsp+0x500]
mov    r10,QWORD PTR [rsp+0x400]
mov    rbx,QWORD PTR [rsp+0x280]
vmovaps YMMWORD PTR [rsp+0x40],ymm0
sub    rbx,QWORD PTR [r14]
sbb    r10,QWORD PTR [r14+0x8]
sbb    r9,QWORD PTR [r14+0x10]
sbb    r8,QWORD PTR [r14+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r11]
mov    rcx,QWORD PTR [r11+0x8]
mov    rdx,QWORD PTR [r11+0x10]
mov    rsi,QWORD PTR [r11+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    rbx,rax
adc    r10,rcx
adc    r9,rdx
adc    r8,rsi
lea    rax,[rsp+0x1420]
mov    QWORD PTR [rsp+0x2c0],r8
mov    QWORD PTR [rsp+0x280],rbx
mov    QWORD PTR [rsp+0x400],r10
mov    QWORD PTR [rsp+0x500],r9
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
lea    r14,[rsp+0x40]
vmovups ymm0,YMMWORD PTR [rax]
mov    rbx,QWORD PTR [rsp+0x340]
mov    r10,QWORD PTR [rsp+0x900]
mov    r9,QWORD PTR [rsp+0x880]
mov    r8,QWORD PTR [rsp+0x200]
vmovaps YMMWORD PTR [rsp+0x40],ymm0
sub    rbx,QWORD PTR [r14]
sbb    r10,QWORD PTR [r14+0x8]
sbb    r9,QWORD PTR [r14+0x10]
sbb    r8,QWORD PTR [r14+0x18]
sbb    rdi,rdi
mov    rax,QWORD PTR [r11]
mov    rcx,QWORD PTR [r11+0x8]
mov    rdx,QWORD PTR [r11+0x10]
mov    rsi,QWORD PTR [r11+0x18]
and    rax,rdi
and    rcx,rdi
and    rdx,rdi
and    rsi,rdi
add    rbx,rax
adc    r10,rcx
adc    r9,rdx
adc    r8,rsi
mov    QWORD PTR [rsp+0x200],r8
mov    QWORD PTR [rsp+0x340],rbx
mov    QWORD PTR [rsp+0x900],r10
mov    QWORD PTR [rsp+0x880],r9
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
mov    r14,rbx
mov    rbx,r10
mov    r11,r9
xor    r10d,r10d
xor    r9d,r9d
xor    edi,edi
vmovups ymm0,YMMWORD PTR [rsp+0x8e0]
vmovaps YMMWORD PTR [rsp+0x40],ymm0
vxorps xmm0,xmm0,xmm0
sub    r10,QWORD PTR [r15]
sbb    r9,QWORD PTR [r15+0x8]
sbb    rdi,QWORD PTR [r15+0x10]
sbb    r12,QWORD PTR [r15+0x18]
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
adc    r12,rsi
mov    QWORD PTR [rsp+0x40],r14
mov    QWORD PTR [rsp+0x48],rbx
mov    QWORD PTR [rsp+0x380],r12
mov    QWORD PTR [rsp+0xbc0],rdi
mov    QWORD PTR [rsp+0xf00],r10
mov    QWORD PTR [rsp+0x50],r11
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
mov    r13,rdi
mov    rdi,r9
xor    ecx,ecx
xor    edx,edx
xor    esi,esi
lea    rbx,[rsp+0x600]
lea    r15,[rsp+0x40]
mov    rax,QWORD PTR [rsp+0x200]
mov    QWORD PTR [rsp+0x58],rax
xor    eax,eax
sub    rax,QWORD PTR [r15]
sbb    rcx,QWORD PTR [r15+0x8]
sbb    rdx,QWORD PTR [r15+0x10]
sbb    rsi,QWORD PTR [r15+0x18]
sbb    r12,r12
mov    r8,QWORD PTR [rbx]
mov    r9,QWORD PTR [rbx+0x8]
mov    r11,QWORD PTR [rbx+0x10]
mov    r14,QWORD PTR [rbx+0x18]
and    r8,r12
and    r9,r12
and    r11,r12
and    r14,r12
add    rax,r8
adc    rcx,r9
adc    rdx,r11
adc    rsi,r14
lea    r15,[rsp+0x640]
lea    rbx,[rsp+0x40]
lea    r12,[rsp+0x600]
mov    r8,QWORD PTR [rsp+0x280]
mov    QWORD PTR [rsp+0x40],r8
mov    r8,QWORD PTR [rsp+0x400]
mov    QWORD PTR [rsp+0x48],r8
mov    r8,QWORD PTR [rsp+0x500]
mov    QWORD PTR [rsp+0x50],r8
mov    r8,QWORD PTR [rsp+0x2c0]
mov    QWORD PTR [rsp+0x58],r8
mov    QWORD PTR [rsp+0x60],rax
mov    QWORD PTR [rsp+0x68],rcx
mov    QWORD PTR [rsp+0x70],rdx
mov    QWORD PTR [rsp+0x78],rsi
mov    rsi,QWORD PTR [rsp+0x440]
lea    rcx,[rsp+0x80]
vmovdqu YMMWORD PTR [rcx],ymm1
mov    rcx,QWORD PTR [rsp+0x480]
mov    QWORD PTR [rsp+0x600],rsi
mov    rsi,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rsp+0x608],rcx
mov    rcx,QWORD PTR [rsp+0x5c0]
mov    QWORD PTR [rsp+0x610],rsi
mov    rsi,QWORD PTR [rsp+0x380]
mov    QWORD PTR [rsp+0x618],rcx
mov    QWORD PTR [rsp+0x620],r10
mov    QWORD PTR [rsp+0x628],rdi
mov    QWORD PTR [rsp+0x630],r13
movabs r13,0x87d20782e4866389
mov    r10,rdi
mov    QWORD PTR [rsp+0x638],rsi
mov    QWORD PTR [rsp+0x640],r13
vmovups XMMWORD PTR [r15+0x8],xmm0
mov    QWORD PTR [r15+0x18],0x0
xor    r14d,r14d
xor    eax,eax
xor    ecx,ecx
xor    esi,esi
xor    edi,edi
mov    rdx,QWORD PTR [r12]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x8]
adcx   rax,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x10]
adcx   rcx,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x18]
adcx   rsi,r8
adox   rdi,r9
adcx   rdi,r11
mov    rdx,QWORD PTR [r12+0x20]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx+0x20]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x28]
adcx   rax,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x30]
adcx   rcx,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x38]
adcx   rsi,r8
adox   rdi,r9
adcx   rdi,r11
mov    rdx,r14
imul   rdx,QWORD PTR [r12+0x40]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx+0x40]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x48]
adcx   rax,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x50]
adcx   rcx,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x58]
adcx   rsi,r8
adox   rdi,r9
adcx   rdi,r11
mov    rdx,QWORD PTR [r12+0x8]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx]
adcx   rax,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x8]
adcx   rcx,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x10]
adcx   rsi,r8
adox   rdi,r9
mulx   r9,r8,QWORD PTR [rbx+0x18]
adcx   rdi,r8
adox   r14,r9
adcx   r14,r11
mov    rdx,QWORD PTR [r12+0x28]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx+0x20]
adcx   rax,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x28]
adcx   rcx,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x30]
adcx   rsi,r8
adox   rdi,r9
mulx   r9,r8,QWORD PTR [rbx+0x38]
adcx   rdi,r8
adox   r14,r9
adcx   r14,r11
mov    rdx,rax
imul   rdx,QWORD PTR [r12+0x40]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx+0x40]
adcx   rax,r8
adox   rcx,r9
mulx   r9,r8,QWORD PTR [rbx+0x48]
adcx   rcx,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x50]
adcx   rsi,r8
adox   rdi,r9
mulx   r9,r8,QWORD PTR [rbx+0x58]
adcx   rdi,r8
adox   r14,r9
adcx   r14,r11
mov    rdx,QWORD PTR [r12+0x10]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx]
adcx   rcx,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x8]
adcx   rsi,r8
adox   rdi,r9
mulx   r9,r8,QWORD PTR [rbx+0x10]
adcx   rdi,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x18]
adcx   r14,r8
adox   rax,r9
adcx   rax,r11
mov    rdx,QWORD PTR [r12+0x30]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx+0x20]
adcx   rcx,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x28]
adcx   rsi,r8
adox   rdi,r9
mulx   r9,r8,QWORD PTR [rbx+0x30]
adcx   rdi,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x38]
adcx   r14,r8
adox   rax,r9
adcx   rax,r11
mov    rdx,rcx
imul   rdx,QWORD PTR [r12+0x40]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx+0x40]
adcx   rcx,r8
adox   rsi,r9
mulx   r9,r8,QWORD PTR [rbx+0x48]
adcx   rsi,r8
adox   rdi,r9
mulx   r9,r8,QWORD PTR [rbx+0x50]
adcx   rdi,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x58]
adcx   r14,r8
adox   rax,r9
adcx   rax,r11
mov    rdx,QWORD PTR [r12+0x18]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx]
adcx   rsi,r8
adox   rdi,r9
mulx   r9,r8,QWORD PTR [rbx+0x8]
adcx   rdi,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x10]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x18]
adcx   rax,r8
adox   rcx,r9
adcx   rcx,r11
mov    rdx,QWORD PTR [r12+0x38]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx+0x20]
adcx   rsi,r8
adox   rdi,r9
mulx   r9,r8,QWORD PTR [rbx+0x28]
adcx   rdi,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x30]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x38]
adcx   rax,r8
adox   rcx,r9
adcx   rcx,r11
mov    rdx,rsi
imul   rdx,QWORD PTR [r12+0x40]
xor    r11d,r11d
mulx   r9,r8,QWORD PTR [rbx+0x40]
adcx   rsi,r8
adox   rdi,r9
mulx   r9,r8,QWORD PTR [rbx+0x48]
adcx   rdi,r8
adox   r14,r9
mulx   r9,r8,QWORD PTR [rbx+0x50]
adcx   r14,r8
adox   rax,r9
mulx   r9,r8,QWORD PTR [rbx+0x58]
adcx   rax,r8
adox   rcx,r9
adcx   rcx,r11
mov    QWORD PTR [rsp+0xec0],rcx
lea    r11,[rsp+0x600]
mov    rdx,QWORD PTR [rsp+0x280]
mov    rsi,QWORD PTR [rsp+0x400]
mov    r8,QWORD PTR [rsp+0x500]
mov    QWORD PTR [rsp+0x500],r14
mov    r9,QWORD PTR [rsp+0xf00]
mov    QWORD PTR [rsp+0x280],rdi
mov    QWORD PTR [rsp+0x400],rax
mov    QWORD PTR [rsp+0x40],rdx
mov    QWORD PTR [rsp+0x48],rsi
mov    rsi,QWORD PTR [rsp+0x2c0]
mov    QWORD PTR [rsp+0x50],r8
mov    r8,QWORD PTR [rsp+0x340]
movabs rdx,0x687e956e978e3573
mov    QWORD PTR [rsp+0x600],r9
mov    QWORD PTR [rsp+0x608],r10
lea    r9,[rsp+0x80]
mov    QWORD PTR [rsp+0x58],rsi
mov    rsi,QWORD PTR [rsp+0x900]
mov    QWORD PTR [rsp+0x60],r8
mov    r8,QWORD PTR [rsp+0x880]
mov    QWORD PTR [rsp+0x68],rsi
movabs rsi,0x3c208c16d87cfd47
mov    QWORD PTR [rsp+0x70],r8
lea    r8,[r14+rdx*1]
cmp    rdi,rsi
movabs rsi,0x97816a916871ca8d
sbb    r8,0x0
setb   dl
cmp    r14,rsi
setb   sil
or     sil,dl
movzx  edx,sil
movabs rsi,0x47afba497e7ea7a3
lea    r14,[rax+rsi*1]
movabs rsi,0xb85045b68181585d
sub    r14,rdx
setb   dl
cmp    rax,rsi
setb   sil
or     sil,dl
movzx  edx,sil
movabs rsi,0xcf9bb18d1ece5fd7
add    rsi,rcx
mov    rcx,QWORD PTR [rsp+0x200]
sub    rsi,rdx
setb   BYTE PTR [rsp+0x200]
mov    QWORD PTR [rsp+0x78],rcx
mov    rcx,QWORD PTR [rsp+0xbc0]
vmovdqu YMMWORD PTR [r9],ymm1
mov    r9,QWORD PTR [rsp+0x440]
mov    QWORD PTR [rsp+0x610],rcx
mov    rcx,QWORD PTR [rsp+0x380]
mov    QWORD PTR [rsp+0x618],rcx
mov    rcx,QWORD PTR [rsp+0x480]
mov    QWORD PTR [rsp+0x620],r9
mov    r9,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rsp+0x628],rcx
mov    rcx,QWORD PTR [rsp+0x5c0]
mov    QWORD PTR [rsp+0x630],r9
mov    QWORD PTR [rsp+0x638],rcx
mov    QWORD PTR [rsp+0x640],r13
vmovups XMMWORD PTR [r15+0x8],xmm0
mov    QWORD PTR [r15+0x18],0x0
lea    rcx,[rsp+0x40]
xor    ebx,ebx
xor    r13d,r13d
xor    r15d,r15d
xor    r9d,r9d
xor    eax,eax
mov    rdx,QWORD PTR [r11]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx]
adcx   rbx,r10
adox   r13,r12
mulx   r12,r10,QWORD PTR [rcx+0x8]
adcx   r13,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [rcx+0x10]
adcx   r15,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [rcx+0x18]
adcx   r9,r10
adox   rax,r12
adcx   rax,rdi
mov    rdx,QWORD PTR [r11+0x20]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx+0x20]
adcx   rbx,r10
adox   r13,r12
mulx   r12,r10,QWORD PTR [rcx+0x28]
adcx   r13,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [rcx+0x30]
adcx   r15,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [rcx+0x38]
adcx   r9,r10
adox   rax,r12
adcx   rax,rdi
mov    rdx,rbx
imul   rdx,QWORD PTR [r11+0x40]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx+0x40]
adcx   rbx,r10
adox   r13,r12
mulx   r12,r10,QWORD PTR [rcx+0x48]
adcx   r13,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [rcx+0x50]
adcx   r15,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [rcx+0x58]
adcx   r9,r10
adox   rax,r12
adcx   rax,rdi
mov    rdx,QWORD PTR [r11+0x8]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx]
adcx   r13,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [rcx+0x8]
adcx   r15,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [rcx+0x10]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [rcx+0x18]
adcx   rax,r10
adox   rbx,r12
adcx   rbx,rdi
mov    rdx,QWORD PTR [r11+0x28]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx+0x20]
adcx   r13,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [rcx+0x28]
adcx   r15,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [rcx+0x30]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [rcx+0x38]
adcx   rax,r10
adox   rbx,r12
adcx   rbx,rdi
mov    rdx,r13
imul   rdx,QWORD PTR [r11+0x40]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx+0x40]
adcx   r13,r10
adox   r15,r12
mulx   r12,r10,QWORD PTR [rcx+0x48]
adcx   r15,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [rcx+0x50]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [rcx+0x58]
adcx   rax,r10
adox   rbx,r12
adcx   rbx,rdi
mov    rdx,QWORD PTR [r11+0x10]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx]
adcx   r15,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [rcx+0x8]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [rcx+0x10]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [rcx+0x18]
adcx   rbx,r10
adox   r13,r12
adcx   r13,rdi
mov    rdx,QWORD PTR [r11+0x30]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx+0x20]
adcx   r15,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [rcx+0x28]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [rcx+0x30]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [rcx+0x38]
adcx   rbx,r10
adox   r13,r12
adcx   r13,rdi
mov    rdx,r15
imul   rdx,QWORD PTR [r11+0x40]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx+0x40]
adcx   r15,r10
adox   r9,r12
mulx   r12,r10,QWORD PTR [rcx+0x48]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [rcx+0x50]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [rcx+0x58]
adcx   rbx,r10
adox   r13,r12
adcx   r13,rdi
mov    rdx,QWORD PTR [r11+0x18]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [rcx+0x8]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [rcx+0x10]
adcx   rbx,r10
adox   r13,r12
mulx   r12,r10,QWORD PTR [rcx+0x18]
adcx   r13,r10
adox   r15,r12
adcx   r15,rdi
mov    rdx,QWORD PTR [r11+0x38]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx+0x20]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [rcx+0x28]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [rcx+0x30]
adcx   rbx,r10
adox   r13,r12
mulx   r12,r10,QWORD PTR [rcx+0x38]
adcx   r13,r10
adox   r15,r12
adcx   r15,rdi
mov    rdx,r9
imul   rdx,QWORD PTR [r11+0x40]
xor    edi,edi
mulx   r12,r10,QWORD PTR [rcx+0x40]
adcx   r9,r10
adox   rax,r12
mulx   r12,r10,QWORD PTR [rcx+0x48]
adcx   rax,r10
adox   rbx,r12
mulx   r12,r10,QWORD PTR [rcx+0x50]
adcx   rbx,r10
adox   r13,r12
mulx   r12,r10,QWORD PTR [rcx+0x58]
adcx   r13,r10
adox   r15,r12
adcx   r15,rdi
movabs rcx,0x687e956e978e3573
movabs rdx,0x3c208c16d87cfd47
movabs rdi,0x97816a916871ca8d
movabs r10,0xcf9bb18d1ece5fd7
mov    QWORD PTR [rsp+0x380],rbx
mov    QWORD PTR [rsp+0x2c0],rax
mov    QWORD PTR [rsp+0x3c0],r15
mov    QWORD PTR [rsp+0x440],r13
add    rcx,rbx
cmp    rax,rdx
lea    r10,[r15+r10*1]
sbb    rcx,0x0
setb   dl
cmp    rbx,rdi
mov    QWORD PTR [rsp+0x900],rcx
setb   dil
or     dil,dl
movzx  edx,dil
movabs rdi,0x47afba497e7ea7a3
lea    r9,[r13+rdi*1+0x0]
movabs rdi,0xb85045b68181585d
sub    r9,rdx
setb   dl
cmp    r13,rdi
mov    QWORD PTR [rsp+0x480],r9
setb   dil
or     dil,dl
movzx  edx,dil
mov    rdi,QWORD PTR [rsp+0xec0]
sub    r10,rdx
movabs rdx,0x30644e72e131a029
setb   r11b
mov    QWORD PTR [rsp+0x1180],r10
mov    BYTE PTR [rsp+0x340],r11b
cmp    rdi,rdx
setb   dl
or     dl,BYTE PTR [rsp+0x200]
jne    e3be0 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x3610>
movabs rax,0xc3df73e9278302b9
add    QWORD PTR [rsp+0x280],rax
jmp    e3bf3 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x3623>
nop    DWORD PTR [rax+rax*1+0x0]
mov    r8,QWORD PTR [rsp+0x500]
mov    r14,QWORD PTR [rsp+0x400]
mov    rsi,rdi
mov    rax,QWORD PTR [rsp+0xac0]
and    eax,0x7
mov    rcx,QWORD PTR [rsp+0x280]
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
lea    r13,[rsp+0x600]
lea    rbx,[rsp+0x40]
lea    r15,[rsp+0x540]
shl    eax,0x5
vmovups ymm0,YMMWORD PTR [rsp+rax*1+0x1600]
mov    QWORD PTR [rsp+0x540],rcx
mov    QWORD PTR [rsp+0x548],r8
mov    QWORD PTR [rsp+0x550],r14
mov    QWORD PTR [rsp+0x558],rsi
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
movabs rdx,0x97816a916871ca8d
mov    rbx,r9
mov    QWORD PTR [rsp+0xf00],r12
mov    QWORD PTR [rsp+0xbc0],r10
mov    QWORD PTR [rsp+0x200],r8
mov    QWORD PTR [rsp+0x880],rbx
lea    rsi,[r12+rcx*1]
movabs rcx,0x3c208c16d87cfd47
cmp    r8,rcx
sbb    rsi,0x0
setb   cl
cmp    r12,rdx
mov    QWORD PTR [rsp+0x280],rsi
setb   dl
or     dl,cl
movabs rcx,0x47afba497e7ea7a3
lea    rdi,[r10+rcx*1]
movzx  ecx,dl
movabs rdx,0xb85045b68181585d
sub    rdi,rcx
setb   cl
cmp    r10,rdx
mov    QWORD PTR [rsp+0x400],rdi
setb   dl
or     dl,cl
movabs rcx,0xcf9bb18d1ece5fd7
lea    r11,[r9+rcx*1]
mov    r9,QWORD PTR [rsp+0x3c0]
movzx  ecx,dl
sub    r11,rcx
movabs rcx,0x30644e72e131a029
setb   dl
mov    QWORD PTR [rsp+0x500],r11
mov    BYTE PTR [rsp+0x5c0],dl
cmp    r9,rcx
setb   cl
or     cl,BYTE PTR [rsp+0x340]
jne    e4030 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x3a60>
mov    rdx,QWORD PTR [rsp+0x2c0]
movabs rcx,0xc3df73e9278302b9
mov    rsi,QWORD PTR [rsp+0x480]
mov    r9,QWORD PTR [rsp+0x1180]
add    rdx,rcx
mov    rcx,rdx
mov    rdx,QWORD PTR [rsp+0x900]
jmp    e4048 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x3a78>
cs nop WORD PTR [rax+rax*1+0x0]
mov    rdx,QWORD PTR [rsp+0x380]
mov    rsi,QWORD PTR [rsp+0x440]
mov    rcx,QWORD PTR [rsp+0x2c0]
vmovups ymm0,YMMWORD PTR [rsp+rax*1+0x1600]
mov    QWORD PTR [rsp+0x540],rcx
mov    QWORD PTR [rsp+0x548],rdx
mov    QWORD PTR [rsp+0x550],rsi
mov    QWORD PTR [rsp+0x558],r9
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
lea    r15,[rsp+0x600]
lea    r12,[rsp+0x40]
lea    r13,[rsp+0x540]
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
mov    QWORD PTR [rsp+0x380],rax
mov    QWORD PTR [rsp+0x2c0],rbx
mov    QWORD PTR [rsp+0x13c0],r11
mov    QWORD PTR [rsp+0x1180],r10
vmovdqa YMMWORD PTR [rsp+0x600],ymm1
lea    rdx,[r11+rcx*1]
movabs rcx,0x3c208c16d87cfd47
vmovups ymm0,YMMWORD PTR [rsp+rax*1+0x1600]
mov    r8,QWORD PTR [rsp+0x180]
mov    rax,QWORD PTR [rsp+0x1c0]
cmp    rbx,rcx
sbb    rdx,0x0
setb   cl
cmp    r11,r13
mov    QWORD PTR [rsp+0x3c0],rdx
lea    r13,[rsp+0x600]
lea    r11,[rsp+0x540]
setb   dl
or     dl,cl
movabs rcx,0x47afba497e7ea7a3
mov    QWORD PTR [rsp+0x40],r8
mov    QWORD PTR [rsp+0x48],rax
mov    r8,QWORD PTR [rsp+0x240]
mov    rax,QWORD PTR [rsp+0x300]
lea    rsi,[r10+rcx*1]
movzx  ecx,dl
movabs rdx,0xb85045b68181585d
sub    rsi,rcx
setb   cl
cmp    r10,rdx
mov    QWORD PTR [rsp+0x340],rsi
vmovaps YMMWORD PTR [rsp+0x540],ymm0
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
setb   BYTE PTR [rsp+0x480]
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
mov    QWORD PTR [rsp+0xec0],r9
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
setb   BYTE PTR [rsp+0x440]
cmp    r12,rsi
setb   dil
cmp    r9,rsi
mov    r9,QWORD PTR [rsp+0x880]
setb   cl
cmp    r9,rsi
setb   sil
or     sil,BYTE PTR [rsp+0x5c0]
jne    e4850 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x4280>
movabs rsi,0xc3df73e9278302b9
add    QWORD PTR [rsp+0x200],rsi
mov    rsi,QWORD PTR [rsp+0xa40]
mov    rax,QWORD PTR [rsp+0x380]
or     cl,BYTE PTR [rsp+0x480]
je     e4891 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x42c1>
mov    rcx,QWORD PTR [rsp+0x13c0]
mov    r10,QWORD PTR [rsp+0x1180]
mov    r9,QWORD PTR [rsp+0xec0]
mov    QWORD PTR [rsp+0x3c0],rcx
mov    QWORD PTR [rsp+0x340],r10
mov    QWORD PTR [rsp+0x900],r9
jmp    e48a3 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x42d3>
nop
mov    rsi,QWORD PTR [rsp+0xf00]
mov    rax,QWORD PTR [rsp+0xbc0]
mov    QWORD PTR [rsp+0x500],r9
mov    QWORD PTR [rsp+0x280],rsi
mov    QWORD PTR [rsp+0x400],rax
mov    rsi,QWORD PTR [rsp+0xa40]
mov    rax,QWORD PTR [rsp+0x380]
or     cl,BYTE PTR [rsp+0x480]
jne    e481d <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x424d>
movabs rcx,0xc3df73e9278302b9
add    QWORD PTR [rsp+0x2c0],rcx
mov    r9,QWORD PTR [rsp+0x28]
mov    r10,QWORD PTR [rsp+0xa00]
mov    r11,QWORD PTR [rsp+0xc00]
or     dil,BYTE PTR [rsp+0x440]
je     e1ae0 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1510>
mov    rdx,r15
mov    r13,rbx
mov    r14,r12
jmp    e1aed <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x151d>
lea    rdi,[r9+0x18]
vzeroupper
call   acf40 <_RNvMs4_NtCs6i54tJFfzR_5alloc7raw_vecINtB5_6RawVecANtNtCsgXZuNZuAPQq_11zopro_bn2543fq23Fq2j5_E8grow_oneCse6ToTOequVP_10zopro_core>
mov    r11,QWORD PTR [rsp+0xc00]
mov    r10,QWORD PTR [rsp+0xa00]
mov    r9,QWORD PTR [rsp+0x28]
jmp    e1c67 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x1697>
lea    rdi,[rsp+0x600]
lea    rsi,[rsp+0x1600]
vzeroupper
call   d96f0 <_RNvNtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernels12host_inverse>
mov    rcx,QWORD PTR [rsp+0x18]
mov    rdx,QWORD PTR [rsp+0x30]
vmovups ymm2,YMMWORD PTR [rip+0xfffffffffff2563c]        # 9f60 <GCC_except_table830+0x1a1c>
mov    r9,QWORD PTR [rsp+0x28]
vpxor  xmm0,xmm0,xmm0
mov    rax,rcx
shl    rax,0x4
lea    r12,[rcx-0x1]
lea    rsi,[rax+rdx*1-0x10]
mov    rdx,QWORD PTR [rsp+0x1140]
lea    rax,[rdx*8+0x0]
neg    rax
mov    QWORD PTR [rsp+0x1000],rax
mov    rax,rcx
shl    rax,0x6
lea    r8,[rax+rax*4]
mov    rax,rdx
shl    rax,0x9
lea    rax,[rax+rax*4]
sub    r8,rax
mov    rax,QWORD PTR [rsp+0x1100]
add    r8,0xfffffffffffffec8
neg    rax
lea    rax,[rcx+rax*1-0x1]
shl    rax,0x6
lea    rcx,[rax+rax*4]
mov    rax,QWORD PTR [rsp+0x1000]
mov    QWORD PTR [rsp+0x10c0],rsi
mov    rsi,QWORD PTR [r9+0x28]
add    rax,r12
cmp    rax,rsi
jae    ec9dc <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc40c>
mov    rax,QWORD PTR [r9+0x20]
mov    QWORD PTR [rsp+0xf80],rcx
mov    QWORD PTR [rsp+0x1080],r12
and    r12d,0x7
mov    QWORD PTR [rsp+0x1040],r8
lea    r9,[rsp+0xb40]
lea    rsi,[rsp+0x40]
lea    r11,[rsp+0xc40]
movabs rbx,0x87d20782e4866389
shl    r12d,0x5
mov    rcx,QWORD PTR [rax+rcx*1]
vmovups zmm1,ZMMWORD PTR [rax+r8*1+0x38]
mov    QWORD PTR [rsp+0xa40],rcx
mov    rcx,QWORD PTR [rax+r8*1]
mov    QWORD PTR [rsp+0x940],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x8]
mov    QWORD PTR [rsp+0x980],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x10]
mov    QWORD PTR [rsp+0x9c0],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x18]
mov    QWORD PTR [rsp+0xac0],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x20]
mov    QWORD PTR [rsp+0x900],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x28]
mov    QWORD PTR [rsp+0xa80],rcx
mov    rcx,QWORD PTR [rax+r8*1+0x30]
vmovaps ZMMWORD PTR [rsp+0x1200],zmm1
vmovups zmm1,ZMMWORD PTR [rax+r8*1+0x78]
mov    QWORD PTR [rsp+0x880],rcx
vmovaps ZMMWORD PTR [rsp+0x1300],zmm1
vmovups ymm1,YMMWORD PTR [rax+r8*1+0xb8]
vmovaps YMMWORD PTR [rsp+0x1280],ymm1
vmovups zmm1,ZMMWORD PTR [rax+r8*1+0xf8]
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
vmovaps ZMMWORD PTR [rsp+0x540],zmm1
vmovaps ymm1,YMMWORD PTR [rsp+0x540]
vmovaps YMMWORD PTR [rsp+0xb40],ymm1
vmovups ymm1,YMMWORD PTR [rsp+r12*1+0x600]
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
mov    QWORD PTR [rsp+0x480],r10
mov    QWORD PTR [rsp+0x2c0],r15
mov    QWORD PTR [rsp+0x4c0],r14
mov    QWORD PTR [rsp+0x3c0],r13
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
mov    r11,rbx
mov    r9,rsi
cmp    r15,rax
movabs rax,0x687e956e978e3573
lea    r15,[rsp+0xc40]
lea    rcx,[r14+rax*1]
sbb    rcx,0x0
mov    QWORD PTR [rsp+0x400],rcx
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
mov    QWORD PTR [rsp+0x1c0],rdx
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0xcf9bb18d1ece5fd7
lea    rdx,[r10+rcx*1]
sub    rdx,rax
lea    rax,[rsp+0x560]
vmovups ymm1,YMMWORD PTR [rax]
mov    QWORD PTR [rsp+0x240],rdx
setb   BYTE PTR [rsp+0xb00]
vmovaps YMMWORD PTR [rsp+0xb40],ymm1
vmovups ymm1,YMMWORD PTR [rsp+r12*1+0x600]
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
mov    QWORD PTR [rsp+0x500],r13
mov    QWORD PTR [rsp+0x5c0],r10
mov    QWORD PTR [rsp+0x340],rsi
mov    QWORD PTR [rsp+0x440],rbx
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
lea    r8,[rsp+0xb40]
cmp    r10,rax
movabs rax,0x687e956e978e3573
vmovups ymm1,YMMWORD PTR [rsp+r12*1+0x600]
lea    rax,[rsi+rax*1]
sbb    rax,0x0
mov    QWORD PTR [rsp+0x200],rax
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
vmovaps ymm1,YMMWORD PTR [rsp+0x1280]
mov    QWORD PTR [rsp+0x280],rcx
movabs rcx,0xb85045b68181585d
setb   al
cmp    rbx,rcx
lea    rbx,[rsp+0xc40]
setb   cl
or     cl,al
movzx  eax,cl
movabs rcx,0xcf9bb18d1ece5fd7
add    rcx,r13
movabs r13,0x87d20782e4866389
sub    rcx,rax
mov    QWORD PTR [rsp+0x180],rcx
vmovaps YMMWORD PTR [rsp+0x40],ymm1
setb   BYTE PTR [rsp+0xa00]
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
mov    QWORD PTR [rsp+0x300],r11
mov    r13,r15
movabs r8,0x30644e72e131a029
mov    QWORD PTR [rsp+0x380],r10
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
cmp    r9,rax
movabs rax,0x687e956e978e3573
lea    rdi,[r15+rax*1]
sbb    rdi,0x0
setb   al
cmp    r15,rcx
mov    r15,QWORD PTR [rsp+0x300]
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
mov    rax,QWORD PTR [rsp+0x480]
sub    rdx,rcx
setb   r14b
cmp    r15,r8
mov    r15,QWORD PTR [rsp+0x400]
setb   cl
or     cl,r14b
cmp    QWORD PTR [rsp+0x500],r8
setb   r14b
or     r14b,BYTE PTR [rsp+0xa00]
cmp    rax,r8
mov    r8,QWORD PTR [rsp+0x2c0]
setb   r11b
or     r11b,BYTE PTR [rsp+0xb00]
mov    r11,QWORD PTR [rsp+0x1c0]
lea    rbx,[r8+r10*1]
cmovne r15,QWORD PTR [rsp+0x4c0]
cmovne r11,QWORD PTR [rsp+0x3c0]
cmovne rbx,r8
mov    r8,QWORD PTR [rsp+0x240]
mov    QWORD PTR [rsp+0xb40],rbx
mov    QWORD PTR [rsp+0xb00],rbx
lea    rbx,[rsp+0x40]
mov    QWORD PTR [rsp+0xb48],r15
mov    QWORD PTR [rsp+0x400],r15
mov    QWORD PTR [rsp+0x1c0],r11
mov    QWORD PTR [rsp+0xb50],r11
lea    r15,[rsp+0xc40]
cmovne r8,rax
mov    rax,QWORD PTR [rsp+0x5c0]
test   r14b,r14b
mov    r14,QWORD PTR [rsp+0x280]
cmovne r14,QWORD PTR [rsp+0x440]
mov    QWORD PTR [rsp+0x240],r8
mov    QWORD PTR [rsp+0xb58],r8
lea    r10,[rax+r10*1]
cmovne r10,rax
mov    rax,QWORD PTR [rsp+0x200]
mov    QWORD PTR [rsp+0x280],r14
mov    QWORD PTR [rsp+0x3c0],r10
mov    r10,QWORD PTR [rsp+0x180]
cmovne rax,QWORD PTR [rsp+0x340]
cmovne r10,QWORD PTR [rsp+0x500]
test   cl,cl
movabs rcx,0xc3df73e9278302b9
cmovne rdi,r13
mov    r13,QWORD PTR [rsp+0x3c0]
cmovne rsi,QWORD PTR [rsp+0x380]
cmovne rdx,QWORD PTR [rsp+0x300]
lea    rcx,[r9+rcx*1]
cmovne rcx,r9
mov    QWORD PTR [rsp+r12*1+0x600],rcx
mov    QWORD PTR [rsp+r12*1+0x608],rdi
mov    QWORD PTR [rsp+0x200],rax
mov    QWORD PTR [rsp+0x180],r10
mov    QWORD PTR [rsp+r12*1+0x610],rsi
mov    QWORD PTR [rsp+r12*1+0x618],rdx
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
mov    QWORD PTR [rsp+0x4c0],r14
mov    QWORD PTR [rsp+0x40],r13
mov    QWORD PTR [rsp+0x340],r8
mov    QWORD PTR [rsp+0x480],r11
mov    QWORD PTR [rsp+0x5c0],r10
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
lea    r15,[rsp+0xc40]
lea    r12,[rsp+0x40]
cmp    r8,rax
movabs rax,0x687e956e978e3573
mov    r14,QWORD PTR [rsp+0x4c0]
mov    r13,QWORD PTR [rsp+0x280]
mov    r8,QWORD PTR [rsp+0xb00]
mov    rdi,QWORD PTR [rsp+0x400]
mov    r9,QWORD PTR [rsp+0x1c0]
lea    rax,[r10+rax*1]
sbb    rax,0x0
mov    QWORD PTR [rsp+0x500],rax
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
mov    QWORD PTR [rsp+0x300],rax
movabs rax,0xb85045b68181585d
setb   cl
cmp    r14,rax
movabs rax,0xcf9bb18d1ece5fd7
setb   dl
add    rax,r11
or     dl,cl
movzx  ecx,dl
mov    rdx,QWORD PTR [rsp+0x240]
sub    rax,rcx
mov    rcx,QWORD PTR [rsp+0x180]
mov    QWORD PTR [rsp+0x2c0],rax
mov    rax,QWORD PTR [rsp+0x200]
setb   BYTE PTR [rsp+0x440]
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
mov    QWORD PTR [rsp+0x380],rdx
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
mov    rbx,r8
lea    r12,[rsp+0xc40]
lea    rdx,[rsp+0x40]
mov    r10,QWORD PTR [rsp+0x3c0]
mov    r11,QWORD PTR [rsp+0x240]
mov    r14,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0x40],r10
mov    QWORD PTR [rsp+0x48],rax
mov    rax,QWORD PTR [rsp+0x180]
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
lea    r12,[rsp+0x40]
lea    rbx,[rsp+0xb40]
movabs r14,0x87d20782e4866389
lea    r15,[rsp+0xc40]
mov    rax,QWORD PTR [rsp+0x380]
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
cmp    rsi,rax
movabs rax,0x687e956e978e3573
mov    r12,QWORD PTR [rsp+0x480]
mov    rdi,QWORD PTR [rsp+0x340]
mov    rbx,QWORD PTR [rsp+0x2c0]
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
or     al,BYTE PTR [rsp+0x440]
lea    rax,[rdi+rdx*1]
cmovne rax,rdi
mov    rdi,QWORD PTR [rsp+0x300]
cmovne rbx,r12
mov    r11,rax
mov    rax,QWORD PTR [rsp+0x500]
cmovne rdi,QWORD PTR [rsp+0x4c0]
mov    QWORD PTR [rsp+0x40],r11
cmovne rax,QWORD PTR [rsp+0x5c0]
test   cl,cl
lea    rcx,[rsi+rdx*1]
cmovne rcx,rsi
cmovne r15,r9
cmovne r8,r10
cmovne r13,r14
lea    r14,[rsp+0x40]
lea    r9,[rsp+0xc40]
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
mov    QWORD PTR [rsp+0x4c0],r11
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
mov    r9,rdi
lea    r14,[rsp+0xc40]
lea    r11,[rsp+0x40]
vmovaps ymm1,YMMWORD PTR [rsp+0x1300]
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
lea    rax,[rsp+0x1320]
mov    QWORD PTR [rsp+0x340],r13
mov    QWORD PTR [rsp+0x5c0],r8
mov    QWORD PTR [rsp+0x440],r12
mov    QWORD PTR [rsp+0x480],r15
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
mov    r13,r12
mov    r11,r10
lea    r10,[rsp+0xc40]
lea    r12,[rsp+0x40]
vmovups ymm1,YMMWORD PTR [rax]
mov    r14,QWORD PTR [rsp+0x4c0]
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
mov    QWORD PTR [rsp+0x300],r9
mov    QWORD PTR [rsp+0x50],r8
mov    QWORD PTR [rsp+0x500],r11
mov    QWORD PTR [rsp+0x4c0],r14
mov    QWORD PTR [rsp+0x2c0],rbx
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
lea    r13,[rsp+0xc40]
lea    r15,[rsp+0x40]
mov    rax,QWORD PTR [rsp+0x340]
mov    r10,QWORD PTR [rsp+0xa40]
mov    r8,QWORD PTR [rsp+0x980]
mov    r9,QWORD PTR [rsp+0x9c0]
mov    r12,QWORD PTR [rsp+0x940]
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
mov    QWORD PTR [rsp+0x940],r12
mov    QWORD PTR [rsp+0x9c0],r9
mov    QWORD PTR [rsp+0x980],r8
mov    QWORD PTR [rsp+0xa40],r10
mov    QWORD PTR [rsp+0x48],r11
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
lea    r13,[rsp+0xc40]
lea    r12,[rsp+0x40]
mov    r15,r10
mov    rax,QWORD PTR [rsp+0x300]
mov    r14,QWORD PTR [rsp+0xa80]
mov    r8,QWORD PTR [rsp+0x900]
mov    r9,QWORD PTR [rsp+0x880]
mov    QWORD PTR [rsp+0x50],rax
mov    QWORD PTR [rsp+0x58],rbx
mov    rbx,QWORD PTR [rsp+0xac0]
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
mov    QWORD PTR [rsp+0xa80],r14
mov    QWORD PTR [rsp+0xac0],rbx
mov    QWORD PTR [rsp+0x880],r9
mov    QWORD PTR [rsp+0x900],r8
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
xor    ecx,ecx
xor    edx,edx
xor    esi,esi
mov    rax,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rsp+0x40],rax
mov    rax,QWORD PTR [rsp+0x200]
mov    QWORD PTR [rsp+0x48],rax
mov    rax,QWORD PTR [rsp+0x280]
mov    QWORD PTR [rsp+0x50],rax
mov    rax,QWORD PTR [rsp+0x180]
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
mov    QWORD PTR [rsp+0xc40],r15
lea    r10,[rsp+0xc88]
lea    r12,[rsp+0xc40]
lea    r11,[rsp+0x40]
lea    r13,[rsp+0xc40]
mov    rdi,QWORD PTR [rsp+0xb00]
mov    r8,QWORD PTR [rsp+0x940]
mov    QWORD PTR [rsp+0x40],rdi
mov    rdi,QWORD PTR [rsp+0x400]
mov    QWORD PTR [rsp+0xc48],r8
mov    r8,QWORD PTR [rsp+0x9c0]
mov    QWORD PTR [rsp+0x48],rdi
mov    rdi,QWORD PTR [rsp+0x1c0]
mov    QWORD PTR [rsp+0x50],rdi
mov    rdi,QWORD PTR [rsp+0x240]
mov    QWORD PTR [rsp+0x58],rdi
mov    QWORD PTR [rsp+0x60],rcx
mov    QWORD PTR [rsp+0x68],rdx
mov    QWORD PTR [rsp+0x70],rsi
mov    QWORD PTR [rsp+0x78],rax
vmovups YMMWORD PTR [r9],ymm2
mov    r9,QWORD PTR [rsp+0x980]
mov    QWORD PTR [rsp+0xc50],r9
mov    QWORD PTR [rsp+0xc58],r8
mov    QWORD PTR [rsp+0xc60],rbx
mov    rbx,QWORD PTR [rsp+0x900]
movabs r8,0x87d20782e4866389
mov    QWORD PTR [rsp+0xc68],rbx
mov    QWORD PTR [rsp+0xc70],r14
mov    r14,QWORD PTR [rsp+0x880]
mov    QWORD PTR [rsp+0xc78],r14
mov    QWORD PTR [rsp+0xc80],r8
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
mov    QWORD PTR [rsp+0x380],r15
mov    QWORD PTR [rsp+0xc00],r9
mov    QWORD PTR [rsp+0xa00],r10
mov    QWORD PTR [rsp+0xf40],r8
lea    r11,[rsp+0x40]
mov    rax,QWORD PTR [rsp+0xb00]
mov    rsi,QWORD PTR [rsp+0x400]
mov    rcx,QWORD PTR [rsp+0x1c0]
cmp    r9,rdi
mov    QWORD PTR [rsp+0x40],rax
mov    QWORD PTR [rsp+0x48],rsi
mov    QWORD PTR [rsp+0x50],rcx
mov    rsi,QWORD PTR [rsp+0x240]
mov    rcx,QWORD PTR [rsp+0xac0]
movabs rax,0x687e956e978e3573
lea    rax,[r10+rax*1]
sbb    rax,0x0
setb   dl
mov    QWORD PTR [rsp+0xc40],rcx
mov    QWORD PTR [rsp+0x58],rsi
mov    rsi,QWORD PTR [rsp+0xa80]
mov    rcx,QWORD PTR [rsp+0xa40]
mov    QWORD PTR [rsp+0xc48],rbx
lea    rbx,[rsp+0xc40]
mov    QWORD PTR [rsp+0xc50],rsi
mov    QWORD PTR [rsp+0xc58],r14
mov    QWORD PTR [rsp+0xc60],rcx
mov    rsi,QWORD PTR [rsp+0x940]
mov    rcx,QWORD PTR [rsp+0x980]
mov    r14,rax
mov    QWORD PTR [rsp+0xc68],rsi
mov    QWORD PTR [rsp+0xc70],rcx
movabs rcx,0x97816a916871ca8d
mov    rsi,QWORD PTR [rsp+0x9c0]
cmp    r10,rcx
setb   cl
or     cl,dl
mov    rdx,QWORD PTR [rsp+0x380]
movzx  eax,cl
movabs rcx,0x47afba497e7ea7a3
add    rcx,r8
mov    QWORD PTR [rsp+0xc78],rsi
mov    rsi,QWORD PTR [rsp+0x200]
sub    rcx,rax
mov    QWORD PTR [rsp+0x1c0],rcx
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
mov    rcx,QWORD PTR [rsp+0x3c0]
mov    QWORD PTR [rsp+0x60],rcx
mov    rcx,QWORD PTR [rsp+0x280]
mov    QWORD PTR [rsp+0x68],rsi
mov    rsi,QWORD PTR [rsp+0x180]
setb   BYTE PTR [rsp+0x180]
mov    QWORD PTR [rsp+0x70],rcx
lea    rcx,[rsp+0x80]
mov    QWORD PTR [rsp+0x78],rsi
movabs rsi,0x87d20782e4866389
vmovups YMMWORD PTR [rcx],ymm2
lea    rcx,[rsp+0xc88]
mov    QWORD PTR [rsp+0xc80],rsi
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
mov    QWORD PTR [rsp+0x280],rdi
mov    QWORD PTR [rsp+0x400],rax
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
cmp    rcx,rdx
movabs rdx,0x687e956e978e3573
vmovaps ymm1,YMMWORD PTR [rsp+0x1200]
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
mov    rdx,QWORD PTR [rsp+0xc00]
or     r10b,r9b
movzx  r9d,r10b
sub    rdi,r9
setb   r9b
cmp    rax,r8
mov    rax,QWORD PTR [rsp+0x380]
setb   r10b
or     r10b,r9b
cmp    rax,r8
movabs r8,0xc3df73e9278302b9
setb   r9b
or     r9b,BYTE PTR [rsp+0x180]
lea    rbx,[rdx+r8*1]
lea    r8,[rcx+r8*1]
cmovne r14,QWORD PTR [rsp+0xa00]
cmovne rbx,rdx
mov    rdx,QWORD PTR [rsp+0x1c0]
cmovne r15,rax
cmovne rdx,QWORD PTR [rsp+0xf40]
test   r10b,r10b
mov    QWORD PTR [rsp+0x180],r15
lea    r15,[rsp+0x40]
cmovne r11,rsi
cmovne r8,rcx
cmovne r12,QWORD PTR [rsp+0x280]
cmovne rdi,QWORD PTR [rsp+0x400]
mov    QWORD PTR [rsp+0x200],r11
mov    QWORD PTR [rsp+0x240],r14
mov    r14,QWORD PTR [rsp+0x180]
mov    r11,QWORD PTR [rsp+0x240]
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
lea    rax,[rsp+0x1220]
mov    QWORD PTR [rsp+0x240],r11
mov    QWORD PTR [rsp+0x1c0],rdx
mov    QWORD PTR [rsp+0x180],r14
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
lea    rdx,[rsp+0xc40]
lea    r13,[rsp+0x40]
vmovdqu ymm1,YMMWORD PTR [rax]
mov    r11,QWORD PTR [rsp+0x200]
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
mov    QWORD PTR [rsp+0x200],r11
mov    QWORD PTR [rsp+0x980],rdi
mov    QWORD PTR [rsp+0x400],r12
mov    r12,rdx
mov    QWORD PTR [rsp+0x280],r8
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
mov    r8,r13
mov    r10,QWORD PTR [rsp+0x440]
mov    rdx,QWORD PTR [rsp+0x480]
mov    rdi,QWORD PTR [rsp+0x5c0]
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
mov    QWORD PTR [rsp+0x440],r10
mov    QWORD PTR [rsp+0x5c0],rdi
mov    QWORD PTR [rsp+0x340],r11
mov    QWORD PTR [rsp+0x480],rdx
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
mov    r13,QWORD PTR [rsp+0x4c0]
mov    rdi,QWORD PTR [rsp+0x500]
mov    r10,QWORD PTR [rsp+0x300]
mov    r11,QWORD PTR [rsp+0x2c0]
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
mov    QWORD PTR [rsp+0x2c0],r11
mov    QWORD PTR [rsp+0x300],r10
mov    QWORD PTR [rsp+0x500],rdi
mov    QWORD PTR [rsp+0x4c0],r13
mov    QWORD PTR [rsp+0x40],rbx
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
mov    rdi,r8
mov    r11,QWORD PTR [rsp+0x240]
mov    rdx,QWORD PTR [rsp+0x1c0]
mov    r10,QWORD PTR [rsp+0x180]
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
mov    QWORD PTR [rsp+0x180],rbx
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
jb     e75df <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x700f>
and    sil,r10b
or     al,sil
and    al,r9b
or     cl,al
and    cl,r11b
je     ec7a9 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1d9>
mov    rdx,QWORD PTR [rsp+0x280]
mov    r15,QWORD PTR [rsp+0x200]
mov    r8,QWORD PTR [rsp+0x400]
mov    r12,QWORD PTR [rsp+0x980]
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
lea    rdi,[rsp+0x40]
lea    r10,[rsp+0xc40]
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
vmovaps YMMWORD PTR [rsp+0xc40],ymm2
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
jb     e77c8 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x71f8>
and    sil,r10b
or     al,sil
and    al,r9b
or     cl,al
and    cl,r11b
je     ec7a9 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1d9>
mov    rdi,QWORD PTR [rsp+0x1080]
cmp    rdi,QWORD PTR [rsp+0x18]
jae    eca05 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc435>
mov    rsi,QWORD PTR [rsp+0x10c0]
mov    r10,QWORD PTR [rsp+0x38]
mov    r9,QWORD PTR [rsp+0x28]
mov    eax,DWORD PTR [rsi]
and    eax,0x7fffffff
cmp    QWORD PTR [rsp+0x20],rax
jbe    ec9ee <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc41e>
mov    rcx,QWORD PTR [rsp+0x440]
shl    rax,0x7
add    rsi,0xfffffffffffffff0
mov    QWORD PTR [r10+rax*1],rcx
mov    rcx,QWORD PTR [rsp+0x480]
mov    QWORD PTR [r10+rax*1+0x8],rcx
mov    rcx,QWORD PTR [rsp+0x5c0]
mov    QWORD PTR [r10+rax*1+0x10],rcx
mov    rcx,QWORD PTR [rsp+0x340]
mov    QWORD PTR [r10+rax*1+0x18],rcx
mov    rcx,QWORD PTR [rsp+0x4c0]
mov    QWORD PTR [r10+rax*1+0x20],rcx
mov    rcx,QWORD PTR [rsp+0x500]
mov    QWORD PTR [r10+rax*1+0x28],rcx
mov    rcx,QWORD PTR [rsp+0x300]
mov    QWORD PTR [r10+rax*1+0x30],rcx
mov    rcx,QWORD PTR [rsp+0x2c0]
mov    QWORD PTR [r10+rax*1+0x38],rcx
mov    rcx,QWORD PTR [rsp+0x180]
mov    QWORD PTR [r10+rax*1+0x40],rcx
mov    rcx,QWORD PTR [rsp+0xf80]
mov    QWORD PTR [r10+rax*1+0x48],r13
mov    QWORD PTR [r10+rax*1+0x50],rbx
mov    QWORD PTR [r10+rax*1+0x58],r14
mov    QWORD PTR [r10+rax*1+0x60],rdx
mov    QWORD PTR [r10+rax*1+0x68],r15
mov    QWORD PTR [r10+rax*1+0x70],r8
mov    r8,QWORD PTR [rsp+0x1040]
mov    r15,QWORD PTR [rsp+0x1100]
mov    QWORD PTR [r10+rax*1+0x78],r12
lea    r12,[rdi-0x1]
add    r8,0xfffffffffffffec0
add    rcx,0xfffffffffffffec0
cmp    r15,rdi
jb     e4990 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x43c0>
vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x600]
vmovdqu64 zmm1,ZMMWORD PTR [rsp+0x640]
vmovdqu64 zmm2,ZMMWORD PTR [rsp+0x680]
vmovdqu64 zmm3,ZMMWORD PTR [rsp+0x6c0]
vmovdqa64 zmm7,ZMMWORD PTR [rip+0xfffffffffff21a85]        # 9380 <GCC_except_table830+0xe3c>
vmovdqa64 zmm6,ZMMWORD PTR [rip+0xfffffffffff21abb]        # 93c0 <GCC_except_table830+0xe7c>
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff21e39]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff21f77]        # 9890 <GCC_except_table830+0x134c>
vmovaps ymm10,YMMWORD PTR [rip+0xfffffffffff2253f]        # 9e60 <GCC_except_table830+0x191c>
movabs rax,0x30644e72e131
mov    r13,QWORD PTR [rsp+0x20]
mov    rbx,QWORD PTR [rsp+0x1140]
mov    r14d,DWORD PTR [rsp+0x1380]
vpunpcklqdq zmm4,zmm0,zmm1
vpunpcklqdq zmm8,zmm2,zmm3
vpunpckhqdq zmm0,zmm0,zmm1
vpunpckhqdq zmm2,zmm2,zmm3
vmovups YMMWORD PTR [rsp+0xc40],ymm10
mov    QWORD PTR [rsp+0xc60],rax
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
vpmadd52huq zmm0,zmm12,QWORD BCST [rip+0xfffffffffff21e03]        # 9818 <GCC_except_table830+0x12d4>
vpternlogq zmm14,zmm3,zmm9,0xf8
vpternlogq zmm13,zmm4,zmm9,0xf8
vpmullq zmm7,zmm0,QWORD BCST [rsp+0xc58]
vpmullq zmm9,zmm0,QWORD BCST [rsp+0xc50]
vpmullq zmm6,zmm0,QWORD BCST [rsp+0xc48]
vpsubq zmm4,zmm13,zmm7
vpbroadcastq zmm7,QWORD PTR [rip+0xfffffffffff21e6c]        # 98c0 <GCC_except_table830+0x137c>
vpsubq zmm3,zmm14,zmm9
vpmullq zmm9,zmm0,QWORD BCST [rsp+0xc60]
vpsubq zmm1,zmm1,zmm6
vmovups YMMWORD PTR [rsp+0xc40],ymm10
mov    QWORD PTR [rsp+0xc60],rax
vpmullq zmm0,zmm0,zmm7
vpsubq zmm2,zmm12,zmm9
vpaddq zmm0,zmm0,zmm5
vpsraq zmm5,zmm0,0x34
vpandq zmm0,zmm0,zmm11
vpaddq zmm1,zmm1,zmm5
vpaddq zmm7,zmm0,zmm7
vpsraq zmm5,zmm1,0x34
vpandq zmm1,zmm1,zmm11
vpsubq zmm12,zmm1,QWORD BCST [rsp+0xc48]
vpsraq zmm10,zmm7,0x34
vpaddq zmm3,zmm3,zmm5
vpsraq zmm5,zmm3,0x34
vpandq zmm3,zmm3,zmm11
vpsubq zmm6,zmm3,QWORD BCST [rsp+0xc50]
vpaddq zmm4,zmm4,zmm5
vpsraq zmm5,zmm4,0x34
vpandq zmm4,zmm4,zmm11
vpsubq zmm8,zmm4,QWORD BCST [rsp+0xc58]
vpaddq zmm2,zmm2,zmm5
vpandq zmm2,zmm2,zmm11
vpsubq zmm9,zmm2,QWORD BCST [rsp+0xc60]
vpaddq zmm5,zmm12,zmm10
vpsraq zmm10,zmm5,0x34
vpaddq zmm6,zmm6,zmm10
vpsraq zmm10,zmm6,0x34
vpaddq zmm8,zmm8,zmm10
vpsraq zmm10,zmm8,0x34
vpaddq zmm9,zmm9,zmm10
vpcmpltuq k1,zmm9,QWORD BCST [rip+0xfffffffffff21c97]        # 97e8 <GCC_except_table830+0x12a4>
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
test   r14b,r14b
mov    r14,QWORD PTR [rsp+0x30]
jne    ec797 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1c7>
vpbroadcastq zmm15,QWORD PTR [rip+0xfffffffffff21b99]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm16,QWORD PTR [rip+0xfffffffffff21be7]        # 97a0 <GCC_except_table830+0x125c>
mov    r12,QWORD PTR [rsp+0x18]
sub    r12,r15
add    r12,0x8
data16 cs nop WORD PTR [rax+rax*1+0x0]
mov    rsi,QWORD PTR [r9+0x10]
lea    rdi,[rbx-0x1]
cmp    rdi,rsi
jae    ecaff <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc52f>
mov    rax,QWORD PTR [r9+0x8]
lea    rcx,[rdi+rdi*4]
vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x40]
vmovdqa64 zmm5,ZMMWORD PTR [rsp+0x80]
vmovdqa64 zmm8,ZMMWORD PTR [rsp+0xc0]
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x100]
vmovdqa64 zmm11,ZMMWORD PTR [rsp+0x140]
vpbroadcastq zmm23,QWORD PTR [rip+0xfffffffffff21d2d]        # 9948 <GCC_except_table830+0x1404>
vpxor  xmm10,xmm10,xmm10
vpxor  xmm13,xmm13,xmm13
vpxor  xmm3,xmm3,xmm3
vpxor  xmm7,xmm7,xmm7
vpxor  xmm2,xmm2,xmm2
vpbroadcastq zmm18,QWORD PTR [rip+0xfffffffffff21d55]        # 9990 <GCC_except_table830+0x144c>
vpxor  xmm0,xmm0,xmm0
vpbroadcastq zmm19,QWORD PTR [rip+0xfffffffffff21b77]        # 97c0 <GCC_except_table830+0x127c>
vpbroadcastq zmm20,QWORD PTR [rip+0xfffffffffff21bd5]        # 9828 <GCC_except_table830+0x12e4>
vpbroadcastq zmm22,QWORD PTR [rip+0xfffffffffff21c3b]        # 9898 <GCC_except_table830+0x1354>
mov    rdx,QWORD PTR [rsp+0x18]
lea    r11,[rbx*8-0x8]
mov    QWORD PTR [rsp+0x500],rdi
shl    rcx,0x7
vmovdqa64 zmm1,ZMMWORD PTR [rax+rcx*1+0x140]
vmovdqa64 zmm17,ZMMWORD PTR [rax+rcx*1]
vmovdqa64 zmm14,ZMMWORD PTR [rax+rcx*1+0x40]
vmovdqa64 zmm6,ZMMWORD PTR [rax+rcx*1+0x80]
vmovdqa64 zmm12,ZMMWORD PTR [rax+rcx*1+0xc0]
vmovdqa64 zmm21,ZMMWORD PTR [rax+rcx*1+0x100]
sub    rdx,r11
vpmadd52huq zmm13,zmm9,zmm1
vpmadd52huq zmm3,zmm5,zmm1
vpmadd52huq zmm7,zmm8,zmm1
vpmadd52huq zmm2,zmm4,zmm1
vpmadd52luq zmm10,zmm9,zmm1
vpmadd52huq zmm0,zmm11,zmm1
vpmadd52luq zmm13,zmm5,zmm1
vpmadd52luq zmm3,zmm8,zmm1
vpmadd52luq zmm7,zmm4,zmm1
vpmadd52luq zmm2,zmm11,zmm1
vpxor  xmm1,xmm1,xmm1
vpmadd52luq zmm1,zmm10,zmm23
vpaddq zmm10,zmm10,zmm15
vpsrlq zmm10,zmm10,0x34
vpmadd52huq zmm10,zmm1,zmm16
vpmadd52luq zmm13,zmm1,zmm18
vpmadd52huq zmm3,zmm1,zmm18
vpmadd52huq zmm7,zmm1,zmm19
vpmadd52huq zmm2,zmm1,zmm20
vpmadd52huq zmm0,zmm1,zmm22
vpaddq zmm10,zmm13,zmm10
vmovdqa64 zmm13,ZMMWORD PTR [rax+rcx*1+0x180]
vpmadd52luq zmm3,zmm1,zmm19
vpmadd52luq zmm7,zmm1,zmm20
vpmadd52luq zmm2,zmm1,zmm22
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm3,zmm9,zmm13
vpmadd52huq zmm7,zmm5,zmm13
vpmadd52huq zmm2,zmm8,zmm13
vpmadd52huq zmm0,zmm4,zmm13
vpmadd52luq zmm10,zmm9,zmm13
vpmadd52huq zmm1,zmm11,zmm13
vpmadd52luq zmm3,zmm5,zmm13
vpmadd52luq zmm7,zmm8,zmm13
vpmadd52luq zmm2,zmm4,zmm13
vpmadd52luq zmm0,zmm11,zmm13
vpxor  xmm13,xmm13,xmm13
vpmadd52luq zmm13,zmm10,zmm23
vpaddq zmm10,zmm10,zmm15
vpsrlq zmm10,zmm10,0x34
vpmadd52huq zmm7,zmm13,zmm18
vpmadd52huq zmm2,zmm13,zmm19
vpmadd52huq zmm0,zmm13,zmm20
vpmadd52huq zmm10,zmm13,zmm16
vpmadd52luq zmm3,zmm13,zmm18
vpmadd52huq zmm1,zmm13,zmm22
vpmadd52luq zmm7,zmm13,zmm19
vpmadd52luq zmm2,zmm13,zmm20
vpmadd52luq zmm0,zmm13,zmm22
vmovdqa64 zmm13,ZMMWORD PTR [rax+rcx*1+0x1c0]
vpaddq zmm10,zmm3,zmm10
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm7,zmm9,zmm13
vpmadd52huq zmm2,zmm5,zmm13
vpmadd52huq zmm0,zmm8,zmm13
vpmadd52huq zmm1,zmm4,zmm13
vpmadd52luq zmm10,zmm9,zmm13
vpmadd52huq zmm3,zmm11,zmm13
vpmadd52luq zmm7,zmm5,zmm13
vpmadd52luq zmm2,zmm8,zmm13
vpmadd52luq zmm0,zmm4,zmm13
vpmadd52luq zmm1,zmm11,zmm13
vpxor  xmm13,xmm13,xmm13
vpmadd52luq zmm13,zmm10,zmm23
vpaddq zmm10,zmm10,zmm15
vpsrlq zmm10,zmm10,0x34
vpmadd52huq zmm2,zmm13,zmm18
vpmadd52huq zmm0,zmm13,zmm19
vpmadd52huq zmm1,zmm13,zmm20
vpmadd52huq zmm10,zmm13,zmm16
vpmadd52luq zmm7,zmm13,zmm18
vpmadd52huq zmm3,zmm13,zmm22
vpmadd52luq zmm2,zmm13,zmm19
vpmadd52luq zmm0,zmm13,zmm20
vpmadd52luq zmm1,zmm13,zmm22
vmovdqa64 zmm13,ZMMWORD PTR [rax+rcx*1+0x200]
vpaddq zmm10,zmm7,zmm10
vpxor  xmm7,xmm7,xmm7
vpmadd52huq zmm2,zmm9,zmm13
vpmadd52huq zmm0,zmm5,zmm13
vpmadd52huq zmm1,zmm8,zmm13
vpmadd52huq zmm3,zmm4,zmm13
vpmadd52luq zmm10,zmm9,zmm13
vpmadd52huq zmm7,zmm11,zmm13
vpmadd52luq zmm2,zmm5,zmm13
vpmadd52luq zmm0,zmm8,zmm13
vpmadd52luq zmm1,zmm4,zmm13
vpmadd52luq zmm3,zmm11,zmm13
vpxor  xmm13,xmm13,xmm13
vpmadd52luq zmm13,zmm10,zmm23
vpaddq zmm10,zmm10,zmm15
vpsrlq zmm10,zmm10,0x34
vpmadd52huq zmm10,zmm13,zmm16
vpmadd52luq zmm2,zmm13,zmm18
vpmadd52huq zmm0,zmm13,zmm18
vpmadd52huq zmm1,zmm13,zmm19
vpmadd52huq zmm3,zmm13,zmm20
vpmadd52huq zmm7,zmm13,zmm22
vpaddq zmm2,zmm2,zmm10
vmovdqa64 zmm10,ZMMWORD PTR [rax+rcx*1+0x240]
vpmadd52luq zmm0,zmm13,zmm19
vpmadd52luq zmm1,zmm13,zmm20
vpmadd52luq zmm3,zmm13,zmm22
vpxor  xmm13,xmm13,xmm13
mov    eax,0x0
cmovb  rdx,rax
vpmadd52huq zmm0,zmm9,zmm10
vpmadd52huq zmm1,zmm5,zmm10
vpmadd52huq zmm3,zmm8,zmm10
vpmadd52huq zmm7,zmm4,zmm10
vpmadd52luq zmm2,zmm9,zmm10
vpmadd52huq zmm13,zmm11,zmm10
vpmadd52luq zmm0,zmm5,zmm10
vpmadd52luq zmm1,zmm8,zmm10
vpmadd52luq zmm3,zmm4,zmm10
vpmadd52luq zmm7,zmm11,zmm10
vpxor  xmm10,xmm10,xmm10
vpmadd52luq zmm10,zmm2,zmm23
vpaddq zmm2,zmm2,zmm15
vpsrlq zmm2,zmm2,0x34
vpmadd52huq zmm1,zmm10,zmm18
vpmadd52huq zmm3,zmm10,zmm19
vpmadd52huq zmm2,zmm10,zmm16
vpmadd52luq zmm0,zmm10,zmm18
vpmadd52huq zmm7,zmm10,zmm20
vpmadd52huq zmm13,zmm10,zmm22
vpmadd52luq zmm1,zmm10,zmm19
vpaddq zmm0,zmm0,zmm2
vpmadd52luq zmm3,zmm10,zmm20
vpmadd52luq zmm7,zmm10,zmm22
vpsrlq zmm2,zmm0,0x34
vpandq zmm0,zmm0,zmm15
vmovdqa64 ZMMWORD PTR [rsp+0x40],zmm0
vpaddq zmm1,zmm2,zmm1
vpsrlq zmm2,zmm1,0x34
vpandq zmm1,zmm1,zmm15
vpaddq zmm2,zmm2,zmm3
vmovdqa64 ZMMWORD PTR [rsp+0x80],zmm1
vpsrlq zmm3,zmm2,0x34
vpandq zmm2,zmm2,zmm15
vpaddq zmm3,zmm3,zmm7
vmovdqa64 ZMMWORD PTR [rsp+0xc0],zmm2
vpsrlq zmm7,zmm3,0x34
vpandq zmm3,zmm3,zmm15
vpaddq zmm7,zmm7,zmm13
vmovdqa64 ZMMWORD PTR [rsp+0x100],zmm3
vmovdqa64 ZMMWORD PTR [rsp+0x140],zmm7
jbe    ecab8 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc4e8>
mov    r8,r11
shl    r8,0x4
mov    ebx,DWORD PTR [r14+r8*1]
mov    ecx,ebx
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    ecb16 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc546>
shl    rcx,0x7
mov    r9,r11
or     r9,0x1
vmovdqu64 zmm0,ZMMWORD PTR [r10+rcx*1+0x40]
vmovdqu64 ymm25,YMMWORD PTR [r10+rcx*1]
vmovdqu64 ymm20,YMMWORD PTR [r10+rcx*1+0x20]
vmovdqa64 ZMMWORD PTR [rsp+0x600],zmm0
cmp    rdx,0x1
je     ecab5 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc4e5>
shl    r9,0x4
mov    ecx,DWORD PTR [r14+r9*1]
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    ecb16 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc546>
shl    rcx,0x7
mov    r15,r11
lea    rax,[rsp+0x640]
or     r15,0x2
vmovups ymm0,YMMWORD PTR [r10+rcx*1]
vmovups YMMWORD PTR [rax],ymm0
cmp    rdx,0x2
je     eca9b <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc4cb>
shl    r15,0x4
mov    QWORD PTR [rsp+0x400],r12
mov    eax,DWORD PTR [r14+r15*1]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ecae7 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc517>
shl    rax,0x7
vmovdqu64 ymm23,YMMWORD PTR [r10+rcx*1+0x20]
vmovdqu ymm7,YMMWORD PTR [r10+rcx*1+0x40]
vmovdqu ymm1,YMMWORD PTR [r10+rcx*1+0x60]
mov    r12,r11
or     r12,0x3
vmovdqu64 zmm0,ZMMWORD PTR [r10+rax*1+0x40]
vmovdqu ymm15,YMMWORD PTR [r10+rax*1]
vmovdqu64 ymm26,YMMWORD PTR [r10+rax*1+0x20]
vmovdqa64 ZMMWORD PTR [rsp+0xc40],zmm0
cmp    rdx,0x3
je     eca81 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc4b1>
shl    r12,0x4
mov    ecx,DWORD PTR [r14+r12*1]
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    ecb16 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc546>
shl    rcx,0x7
lea    rax,[rsp+0xc80]
vmovups ymm0,YMMWORD PTR [r10+rcx*1]
vmovups YMMWORD PTR [rax],ymm0
mov    rax,r11
or     rax,0x4
cmp    rdx,0x4
je     eca67 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc497>
shl    rax,0x4
mov    esi,DWORD PTR [r14+rax*1]
and    esi,0x7fffffff
cmp    r13,rsi
jbe    ecacf <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc4ff>
shl    rsi,0x7
vmovdqu64 ymm27,YMMWORD PTR [r10+rcx*1+0x20]
vmovdqu64 ymm16,YMMWORD PTR [r10+rcx*1+0x40]
vmovdqu ymm10,YMMWORD PTR [r10+rcx*1+0x60]
vmovups zmm0,ZMMWORD PTR [r10+rsi*1+0x40]
vmovdqu64 ymm18,YMMWORD PTR [r10+rsi*1]
vmovdqu64 ymm29,YMMWORD PTR [r10+rsi*1+0x20]
mov    r10,r11
or     r10,0x5
vmovaps ZMMWORD PTR [rsp+0x540],zmm0
cmp    rdx,0x5
je     eca4d <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc47d>
shl    r10,0x4
mov    ecx,DWORD PTR [r14+r10*1]
and    ecx,0x7fffffff
cmp    r13,rcx
jbe    ec85c <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc28c>
mov    rsi,QWORD PTR [rsp+0x38]
shl    rcx,0x7
mov    r13,r11
or     r13,0x6
vmovdqu ymm0,YMMWORD PTR [rsi+rcx*1]
lea    rsi,[rsp+0x580]
vmovdqu YMMWORD PTR [rsi],ymm0
cmp    rdx,0x6
je     eca33 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc463>
shl    r13,0x4
mov    esi,DWORD PTR [r14+r13*1]
and    esi,0x7fffffff
cmp    QWORD PTR [rsp+0x20],rsi
jbe    ecb0e <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc53e>
mov    rdi,QWORD PTR [rsp+0x38]
shl    rsi,0x7
vmovups zmm0,ZMMWORD PTR [rdi+rsi*1+0x40]
vmovdqu64 ymm31,YMMWORD PTR [rdi+rcx*1+0x20]
vmovdqu64 ymm24,YMMWORD PTR [rdi+rcx*1+0x40]
vmovdqu64 ymm19,YMMWORD PTR [rdi+rcx*1+0x60]
vmovdqu ymm2,YMMWORD PTR [rdi+rsi*1]
vmovdqu ymm3,YMMWORD PTR [rdi+rsi*1+0x20]
mov    rdi,r11
or     rdi,0x7
vmovaps ZMMWORD PTR [rsp+0x1300],zmm0
cmp    rdx,0x7
je     eca19 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc449>
mov    rcx,QWORD PTR [rsp+0x30]
shl    rdi,0x4
mov    ecx,DWORD PTR [rcx+rdi*1]
and    ecx,0x7fffffff
cmp    QWORD PTR [rsp+0x20],rcx
jbe    ec85c <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc28c>
vmovdqu64 zmm13,ZMMWORD PTR [rsp+0x620]
vmovaps zmm28,ZMMWORD PTR [rsp+0x600]
vmovaps zmm30,ZMMWORD PTR [rsp+0xc40]
mov    rdx,QWORD PTR [rsp+0x38]
shl    rcx,0x7
lea    rsi,[rsp+0x1340]
vmovups ymm0,YMMWORD PTR [rdx+rcx*1]
vmovups ymm22,YMMWORD PTR [rdx+rcx*1+0x60]
vinserti64x4 zmm3,zmm3,YMMWORD PTR [rdx+rcx*1+0x20],0x1
vmovdqu64 ZMMWORD PTR [rsp+0x1220],zmm13
vmovaps ZMMWORD PTR [rsp+0x1200],zmm28
vmovups zmm28,ZMMWORD PTR [rsp+0xc60]
vmovaps ZMMWORD PTR [rsp+0xb40],zmm30
vmovaps zmm30,ZMMWORD PTR [rsp+0x540]
vmovups YMMWORD PTR [rsi],ymm0
vmovdqu ymm0,YMMWORD PTR [rdx+rcx*1+0x40]
lea    rsi,[r11+0x8]
vmovups ZMMWORD PTR [rsp+0xb60],zmm28
vmovups zmm28,ZMMWORD PTR [rsp+0x560]
vmovaps ZMMWORD PTR [rsp+0x1280],zmm30
vmovdqa64 zmm30,ZMMWORD PTR [rsp+0x1300]
vmovups ZMMWORD PTR [rsp+0x12a0],zmm28
vmovdqu64 zmm28,ZMMWORD PTR [rsp+0x1320]
vmovdqa64 ZMMWORD PTR [rsp+0x1440],zmm30
vmovdqu64 ZMMWORD PTR [rsp+0x1460],zmm28
cmp    rsi,QWORD PTR [rsp+0x18]
ja     ec7c4 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc1f4>
vmovaps YMMWORD PTR [rsp+0x4c0],ymm22
vpbroadcastq zmm22,QWORD PTR [rip+0xfffffffffff215d9]        # 9948 <GCC_except_table830+0x1404>
vpxor  xmm13,xmm13,xmm13
vpmadd52huq zmm13,zmm5,zmm17
vpxord xmm28,xmm28,xmm28
vpmadd52huq zmm28,zmm8,zmm17
vmovdqa64 ZMMWORD PTR [rsp+0x180],zmm25
vpxord xmm25,xmm25,xmm25
vmovdqa64 YMMWORD PTR [rsp+0x940],ymm16
vpxord xmm16,xmm16,xmm16
vpmadd52huq zmm16,zmm9,zmm17
vmovdqa64 YMMWORD PTR [rsp+0x2c0],ymm19
vpxord xmm19,xmm19,xmm19
vpmadd52huq zmm19,zmm4,zmm17
vpmadd52luq zmm25,zmm9,zmm17
vpxord xmm30,xmm30,xmm30
vpmadd52huq zmm30,zmm11,zmm17
vmovdqa YMMWORD PTR [rsp+0x3c0],ymm7
vpbroadcastq zmm7,QWORD PTR [rip+0xfffffffffff213bf]        # 97a0 <GCC_except_table830+0x125c>
vmovdqa YMMWORD PTR [rsp+0x9c0],ymm1
vmovdqa YMMWORD PTR [rsp+0x980],ymm10
vmovdqa64 ZMMWORD PTR [rsp+0x1c0],zmm20
vinserti64x4 zmm2,zmm2,YMMWORD PTR [rsp+0x1480],0x1
mov    rcx,QWORD PTR [rsp+0x30]
vpmadd52luq zmm16,zmm5,zmm17
vpmadd52luq zmm13,zmm8,zmm17
vpmadd52luq zmm28,zmm4,zmm17
vpmadd52luq zmm19,zmm11,zmm17
vpxord xmm17,xmm17,xmm17
add    rax,rcx
add    r8,rcx
add    r9,rcx
add    r13,rcx
add    r15,rcx
add    r12,rcx
add    r10,rcx
add    rdi,rcx
mov    r11,QWORD PTR [r9+0x8]
mov    r9,QWORD PTR [r15+0x8]
mov    rdx,QWORD PTR [r8+0x8]
mov    rax,QWORD PTR [rax+0x8]
mov    rsi,QWORD PTR [r13+0x8]
mov    r15,QWORD PTR [rdi+0x8]
mov    r12,QWORD PTR [r12+0x8]
mov    rcx,QWORD PTR [r10+0x8]
mov    di,0x1111
kmovd  k1,edi
mov    dil,0x11
vpmadd52luq zmm17,zmm25,zmm22
vpaddq zmm25,zmm25,QWORD BCST [rip+0xfffffffffff212ca]        # 9748 <GCC_except_table830+0x1204>
vmovdqa64 zmm1,zmm7
vpsrlq zmm25,zmm25,0x34
vpmadd52huq zmm25,zmm17,zmm7
vpbroadcastq zmm7,QWORD PTR [rip+0xfffffffffff214f5]        # 9990 <GCC_except_table830+0x144c>
vpmadd52luq zmm16,zmm17,zmm7
vmovdqa64 zmm10,zmm7
vpmadd52huq zmm13,zmm17,zmm7
vpbroadcastq zmm7,QWORD PTR [rip+0xfffffffffff21371]        # 9828 <GCC_except_table830+0x12e4>
vpaddq zmm16,zmm16,zmm25
vpbroadcastq zmm25,QWORD PTR [rip+0xfffffffffff212f9]        # 97c0 <GCC_except_table830+0x127c>
vpmadd52luq zmm16,zmm9,zmm14
vmovdqa64 zmm20,zmm7
vpmadd52huq zmm19,zmm17,zmm7
vpmadd52huq zmm28,zmm17,zmm25
vpmadd52luq zmm13,zmm17,zmm25
vpmadd52luq zmm28,zmm17,zmm7
vpbroadcastq zmm7,QWORD PTR [rip+0xfffffffffff213a3]        # 9898 <GCC_except_table830+0x1354>
vpmadd52huq zmm13,zmm9,zmm14
vpmadd52huq zmm28,zmm5,zmm14
vpmadd52luq zmm13,zmm5,zmm14
vpmadd52luq zmm19,zmm17,zmm7
vpmadd52huq zmm30,zmm17,zmm7
vpxord xmm17,xmm17,xmm17
vpmadd52luq zmm28,zmm8,zmm14
vpmadd52huq zmm17,zmm11,zmm14
vpmadd52huq zmm19,zmm8,zmm14
vpmadd52huq zmm30,zmm4,zmm14
vpmadd52luq zmm19,zmm4,zmm14
vpmadd52luq zmm30,zmm11,zmm14
vpxor  xmm14,xmm14,xmm14
vpmadd52luq zmm14,zmm16,zmm22
vpaddq zmm16,zmm16,QWORD BCST [rip+0xfffffffffff211f6]        # 9748 <GCC_except_table830+0x1204>
vpmadd52huq zmm28,zmm14,zmm10
vpmadd52huq zmm19,zmm14,zmm25
vpmadd52huq zmm30,zmm14,zmm20
vpmadd52luq zmm13,zmm14,zmm10
vpmadd52huq zmm17,zmm14,zmm7
vpsrlq zmm16,zmm16,0x34
vpmadd52luq zmm28,zmm14,zmm25
vpmadd52luq zmm19,zmm14,zmm20
vpmadd52luq zmm30,zmm14,zmm7
vpmadd52huq zmm17,zmm4,zmm6
vpmadd52huq zmm16,zmm14,zmm1
vpxor  xmm14,xmm14,xmm14
vpmadd52huq zmm14,zmm11,zmm6
vpmadd52huq zmm28,zmm9,zmm6
vpmadd52huq zmm19,zmm5,zmm6
vpmadd52huq zmm30,zmm8,zmm6
vpmadd52luq zmm17,zmm11,zmm6
vpaddq zmm13,zmm13,zmm16
vpxord xmm16,xmm16,xmm16
vpmadd52huq zmm16,zmm11,zmm12
vpmadd52luq zmm13,zmm9,zmm6
vpmadd52luq zmm28,zmm5,zmm6
vpmadd52luq zmm19,zmm8,zmm6
vpmadd52luq zmm30,zmm4,zmm6
vpxor  xmm6,xmm6,xmm6
vpmadd52luq zmm6,zmm13,zmm22
vpaddq zmm13,zmm13,QWORD BCST [rip+0xfffffffffff21152]        # 9748 <GCC_except_table830+0x1204>
vpmadd52luq zmm28,zmm6,zmm10
vpmadd52huq zmm19,zmm6,zmm10
vpmadd52huq zmm30,zmm6,zmm25
vpmadd52huq zmm17,zmm6,zmm20
vpmadd52huq zmm14,zmm6,zmm7
vpsrlq zmm13,zmm13,0x34
vpmadd52luq zmm19,zmm6,zmm25
vpmadd52luq zmm30,zmm6,zmm20
vpmadd52luq zmm17,zmm6,zmm7
vpmadd52huq zmm14,zmm4,zmm12
vpmadd52huq zmm13,zmm6,zmm1
vpxor  xmm6,xmm6,xmm6
vpmadd52huq zmm19,zmm9,zmm12
vpmadd52huq zmm30,zmm5,zmm12
vpmadd52huq zmm17,zmm8,zmm12
vpmadd52luq zmm14,zmm11,zmm12
vpaddq zmm13,zmm28,zmm13
vmovdqa64 zmm28,zmm10
vpmadd52luq zmm13,zmm9,zmm12
vpmadd52luq zmm19,zmm5,zmm12
vpmadd52luq zmm30,zmm8,zmm12
vpmadd52luq zmm17,zmm4,zmm12
vpmadd52luq zmm6,zmm13,zmm22
vpaddq zmm13,zmm13,QWORD BCST [rip+0xfffffffffff210bf]        # 9748 <GCC_except_table830+0x1204>
vmovdqa64 zmm22,zmm1
vpmadd52luq zmm19,zmm6,zmm10
vpmadd52huq zmm30,zmm6,zmm10
vpmadd52huq zmm14,zmm6,zmm20
vpmadd52huq zmm17,zmm6,zmm25
vpmadd52huq zmm16,zmm6,zmm7
vpsrlq zmm12,zmm13,0x34
vpmadd52luq zmm30,zmm6,zmm25
vpmadd52luq zmm14,zmm6,zmm7
vpmadd52luq zmm17,zmm6,zmm20
vpmadd52huq zmm16,zmm4,zmm21
vpmadd52huq zmm12,zmm6,zmm1
vpmadd52huq zmm30,zmm9,zmm21
vpmadd52huq zmm14,zmm8,zmm21
vpmadd52huq zmm17,zmm5,zmm21
vpmadd52luq zmm16,zmm11,zmm21
vpaddq zmm1,zmm19,zmm12
vmovdqa64 zmm12,ZMMWORD PTR [rip+0xfffffffffff20cc6]        # 93c0 <GCC_except_table830+0xe7c>
vpmadd52luq zmm1,zmm9,zmm21
vmovdqu ymm9,YMMWORD PTR [rsi]
vinserti64x4 zmm9,zmm9,YMMWORD PTR [r15],0x1
vpmadd52luq zmm30,zmm5,zmm21
vpmadd52luq zmm14,zmm4,zmm21
vinserti64x4 zmm4,zmm15,YMMWORD PTR [rsp+0xb80],0x1
vinserti64x4 zmm5,zmm18,YMMWORD PTR [rsp+0x12c0],0x1
vmovdqa64 zmm15,ZMMWORD PTR [rip+0xfffffffffff20c4a]        # 9380 <GCC_except_table830+0xe3c>
vpmadd52luq zmm17,zmm8,zmm21
vmovdqu ymm8,YMMWORD PTR [rax+0x20]
vmovdqa64 ZMMWORD PTR [rsp+0x1000],zmm16
vmovdqa64 ZMMWORD PTR [rsp+0x300],zmm1
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm1,zmm11,zmm21
vmovdqu ymm11,YMMWORD PTR [rsi+0x20]
vmovdqa64 ZMMWORD PTR [rsp+0x200],zmm17
vmovdqa64 ZMMWORD PTR [rsp+0x280],zmm14
vmovdqa64 ZMMWORD PTR [rsp+0x240],zmm30
vpxord xmm30,xmm30,xmm30
vmovdqa64 ZMMWORD PTR [rsp+0xf80],zmm1
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x180]
vinserti64x4 zmm1,zmm1,YMMWORD PTR [rsp+0x1240],0x1
vpunpcklqdq zmm17,zmm1,zmm4
vpunpckhqdq zmm14,zmm1,zmm4
vpunpcklqdq zmm1,zmm5,zmm2
vpunpckhqdq zmm4,zmm5,zmm2
vmovdqa64 zmm5,zmm17
vmovdqa64 zmm2,zmm14
vpermt2q zmm5,zmm15,zmm1
vpermt2q zmm2,zmm15,zmm4
vpermt2q zmm17,zmm12,zmm1
vpermt2q zmm14,zmm12,zmm4
vinserti64x4 zmm4,zmm26,ymm27,0x1
vpandq zmm18,zmm5,QWORD BCST [rip+0xfffffffffff20f61]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm26,QWORD PTR [rip+0xfffffffffff20f6f]        # 9760 <GCC_except_table830+0x121c>
vpshldq zmm1,zmm2,zmm5,0xc
vpandq zmm13,zmm1,QWORD BCST [rip+0xfffffffffff20f46]        # 9748 <GCC_except_table830+0x1204>
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x1c0]
vinserti64x4 zmm5,zmm29,ymm31,0x1
vpshldq zmm2,zmm17,zmm2,0x18
vpbroadcastq zmm31,QWORD PTR [rip+0xfffffffffff2115e]        # 9980 <GCC_except_table830+0x143c>
vinserti64x4 zmm1,zmm1,ymm23,0x1
vpunpcklqdq zmm6,zmm1,zmm4
vpunpckhqdq zmm4,zmm1,zmm4
vpunpcklqdq zmm1,zmm5,zmm3
vpunpckhqdq zmm3,zmm5,zmm3
vmovdqa64 zmm5,zmm6
vpermt2q zmm6,zmm12,zmm1
vmovdqa64 zmm7,zmm4
vpermt2q zmm5,zmm15,zmm1
vmovdqu ymm1,YMMWORD PTR [rdx]
vpermt2q zmm7,zmm15,zmm3
vpermt2q zmm4,zmm12,zmm3
vmovdqu ymm3,YMMWORD PTR [r9]
vinserti64x4 zmm1,zmm1,YMMWORD PTR [r11],0x1
vinserti64x4 zmm3,zmm3,YMMWORD PTR [r12],0x1
vmovdqa64 ZMMWORD PTR [rsp+0xf40],zmm6
vmovdqu ymm6,YMMWORD PTR [rax]
vmovdqa64 ZMMWORD PTR [rsp+0x380],zmm5
vmovdqa64 ZMMWORD PTR [rsp+0x180],zmm4
vmovdqu ymm4,YMMWORD PTR [rdx+0x20]
vmovdqu ymm5,YMMWORD PTR [r9+0x20]
vmovdqa64 ZMMWORD PTR [rsp+0xa00],zmm7
vinserti64x4 zmm6,zmm6,YMMWORD PTR [rcx],0x1
vinserti64x4 zmm7,zmm5,YMMWORD PTR [r12+0x20],0x1
vpunpckhqdq zmm19,zmm1,zmm3
vpunpcklqdq zmm21,zmm1,zmm3
vmovdqa64 zmm3,zmm21
vmovdqa64 zmm23,zmm19
vpunpcklqdq zmm1,zmm6,zmm9
vpunpckhqdq zmm6,zmm6,zmm9
vpermt2q zmm3,zmm15,zmm1
vpermt2q zmm21,zmm12,zmm1
vinserti64x4 zmm1,zmm4,YMMWORD PTR [r11+0x20],0x1
vpermt2q zmm23,zmm15,zmm6
vpermt2q zmm19,zmm12,zmm6
vinserti64x4 zmm6,zmm8,YMMWORD PTR [rcx+0x20],0x1
vinserti64x4 zmm8,zmm11,YMMWORD PTR [r15+0x20],0x1
vpunpcklqdq zmm5,zmm1,zmm7
vpunpckhqdq zmm4,zmm1,zmm7
vpunpcklqdq zmm1,zmm6,zmm8
vpunpckhqdq zmm6,zmm6,zmm8
vmovdqa64 zmm7,zmm5
vpermt2q zmm7,zmm15,zmm1
vpermt2q zmm5,zmm12,zmm1
vmovdqa64 zmm1,zmm4
vpermt2q zmm1,zmm15,zmm6
vpermt2q zmm4,zmm12,zmm6
vmovdqa64 ZMMWORD PTR [rsp+0x10c0],zmm1
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x1200]
vmovdqa64 ZMMWORD PTR [rsp+0xc00],zmm7
vmovdqa64 ZMMWORD PTR [rsp+0x1c0],zmm4
vpandq zmm4,zmm3,QWORD BCST [rip+0xfffffffffff20dd2]        # 9748 <GCC_except_table830+0x1204>
vmovdqa64 ZMMWORD PTR [rsp+0x1080],zmm5
vpbroadcastq zmm5,QWORD PTR [rip+0xfffffffffff20ee0]        # 9868 <GCC_except_table830+0x1324>
vinserti64x4 zmm6,zmm1,YMMWORD PTR [rsp+0x3c0],0x1
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0xb40]
vmovdqa64 ZMMWORD PTR [rsp+0x3c0],zmm18
vinserti64x4 zmm7,zmm1,YMMWORD PTR [rsp+0x940],0x1
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x1280]
vmovdqa64 ZMMWORD PTR [rsp+0x940],zmm13
vmovdqa64 ZMMWORD PTR [rsp+0xa40],zmm4
vinserti64x4 zmm8,zmm1,ymm24,0x1
vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x1440]
vpunpckhqdq zmm20,zmm6,zmm7
vmovdqa64 zmm9,zmm20
vinserti64x4 zmm0,zmm1,ymm0,0x1
vpunpcklqdq zmm1,zmm6,zmm7
vpunpcklqdq zmm6,zmm8,zmm0
vmovdqa64 zmm11,zmm1
vpunpckhqdq zmm0,zmm8,zmm0
vpermt2q zmm11,zmm15,zmm6
vpermt2q zmm1,zmm12,zmm6
vmovdqu64 zmm6,ZMMWORD PTR [rsp+0xb60]
vpermt2q zmm9,zmm15,zmm0
vpermt2q zmm20,zmm12,zmm0
vmovdqu64 zmm0,ZMMWORD PTR [rsp+0x1220]
vinserti64x4 zmm7,zmm6,YMMWORD PTR [rsp+0x980],0x1
vmovdqu64 zmm6,ZMMWORD PTR [rsp+0x12a0]
vinserti64x4 zmm0,zmm0,YMMWORD PTR [rsp+0x9c0],0x1
vinserti64x4 zmm8,zmm6,YMMWORD PTR [rsp+0x2c0],0x1
vmovdqu64 zmm6,ZMMWORD PTR [rsp+0x1460]
vinserti64x4 zmm10,zmm6,YMMWORD PTR [rsp+0x4c0],0x1
vpunpcklqdq zmm6,zmm0,zmm7
vpunpckhqdq zmm16,zmm0,zmm7
vmovdqa64 zmm7,zmm16
vpunpcklqdq zmm0,zmm8,zmm10
vpunpckhqdq zmm10,zmm8,zmm10
vmovdqa64 zmm8,zmm6
vpermt2q zmm8,zmm15,zmm0
vpermt2q zmm6,zmm12,zmm0
vpshldq zmm0,zmm23,zmm3,0xc
vpandq zmm3,zmm0,QWORD BCST [rip+0xfffffffffff20ca3]        # 9748 <GCC_except_table830+0x1204>
vpsubq zmm0,zmm4,zmm18
vpermt2q zmm7,zmm15,zmm10
vpermt2q zmm16,zmm12,zmm10
vpaddq zmm4,zmm0,zmm22
vpsraq zmm10,zmm4,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x340],zmm4
vpbroadcastq zmm4,QWORD PTR [rip+0xfffffffffff20e0a]        # 98e0 <GCC_except_table830+0x139c>
vpsubq zmm0,zmm3,zmm13
vmovdqa64 ZMMWORD PTR [rsp+0xa80],zmm3
vpandq zmm3,zmm2,QWORD BCST [rip+0xfffffffffff20c5a]        # 9748 <GCC_except_table830+0x1204>
vpshldq zmm2,zmm21,zmm23,0x18
vpandq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff20c49]        # 9748 <GCC_except_table830+0x1204>
vmovdqu ymm13,YMMWORD PTR [rsi+0x40]
vpbroadcastq zmm23,QWORD PTR [rip+0xfffffffffff20e1a]        # 9928 <GCC_except_table830+0x13e4>
vpaddq zmm0,zmm0,zmm10
vinserti64x4 zmm13,zmm13,YMMWORD PTR [r15+0x40],0x1
vpaddq zmm0,zmm0,zmm28
vpsraq zmm10,zmm0,0x34
vmovdqa64 ZMMWORD PTR [rsp+0xbc0],zmm0
vmovdqa64 ZMMWORD PTR [rsp+0xb00],zmm2
vpsubq zmm2,zmm2,zmm3
vmovdqa64 ZMMWORD PTR [rsp+0x900],zmm3
vpaddq zmm2,zmm2,zmm25
vpaddq zmm3,zmm2,zmm10
vpshldq zmm2,zmm14,zmm17,0x24
vpandq zmm0,zmm2,QWORD BCST [rip+0xfffffffffff20be4]        # 9748 <GCC_except_table830+0x1204>
vpshldq zmm2,zmm19,zmm21,0x24
vpandq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff20bd3]        # 9748 <GCC_except_table830+0x1204>
vpsraq zmm10,zmm3,0x34
vmovdqa64 ZMMWORD PTR [rsp+0xfc0],zmm3
vmovdqa64 ZMMWORD PTR [rsp+0x5c0],zmm2
vpsubq zmm2,zmm2,zmm0
vpaddq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff20c8c]        # 9828 <GCC_except_table830+0x12e4>
vmovdqa64 ZMMWORD PTR [rsp+0x880],zmm0
vpsrlq zmm0,zmm14,0x10
vmovdqa64 ZMMWORD PTR [rsp+0xac0],zmm0
vpaddq zmm3,zmm2,zmm10
vpsrlq zmm2,zmm19,0x10
vmovdqa64 ZMMWORD PTR [rsp+0x480],zmm2
vpsubq zmm2,zmm2,zmm0
vpaddq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff20cc0]        # 9898 <GCC_except_table830+0x1354>
vpsraq zmm10,zmm3,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x11c0],zmm3
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff20c07]        # 97f8 <GCC_except_table830+0x12b4>
vpaddq zmm0,zmm2,zmm10
vmovdqu32 zmm2{k1}{z},ZMMWORD PTR [r8+0x10]
kmovd  k1,edi
vmovdqa64 ZMMWORD PTR [rsp+0x1040],zmm0
vpmovsxbd xmm0,DWORD PTR [rip+0xfffffffffff216f6]        # a30c <GCC_except_table830+0x1dc8>
vpinsrd xmm10,xmm0,DWORD PTR [r8+0x70],0x2
vpermd zmm2,zmm0,zmm2
vmovdqa ymm0,YMMWORD PTR [rip+0xfffffffffff213d5]        # a000 <GCC_except_table830+0x1abc>
vpinsrd xmm10,xmm10,ebx,0x3
vinserti128 ymm2,ymm2,xmm10,0x1
vmovdqu32 ymm10{k1}{z},YMMWORD PTR [r8+0x50]
vpermt2d ymm2,ymm0,ymm10
vpsrlvd ymm2,ymm2,YMMWORD PTR [rip+0xfffffffffff212b0]        # 9f00 <GCC_except_table830+0x19bc>
vpand  ymm2,ymm2,YMMWORD PTR [rip+0xfffffffffff212c8]        # 9f20 <GCC_except_table830+0x19dc>
vextracti128 xmm10,ymm2,0x1
vpor   xmm2,xmm10,xmm2
vpshufd xmm10,xmm2,0xee
vpor   xmm2,xmm10,xmm2
vpshufd xmm10,xmm2,0x55
vpor   xmm2,xmm10,xmm2
vmovdqu ymm10,YMMWORD PTR [r9+0x40]
vmovd  edi,xmm2
vmovdqu ymm2,YMMWORD PTR [rdx+0x40]
vinserti64x4 zmm10,zmm10,YMMWORD PTR [r12+0x40],0x1
vinserti64x4 zmm2,zmm2,YMMWORD PTR [r11+0x40],0x1
kmovd  k1,edi
movabs rdi,0x30644e72e131
vpunpcklqdq zmm14,zmm2,zmm10
vpunpckhqdq zmm2,zmm2,zmm10
vmovdqu ymm10,YMMWORD PTR [rax+0x40]
vinserti64x4 zmm10,zmm10,YMMWORD PTR [rcx+0x40],0x1
vmovdqa64 zmm19,zmm14
vpunpcklqdq zmm17,zmm10,zmm13
vpunpckhqdq zmm10,zmm10,zmm13
vmovdqa64 zmm13,zmm2
vpermt2q zmm14,zmm12,zmm17
vpermt2q zmm13,zmm12,zmm10
vpermt2q zmm2,zmm15,zmm10
vpermt2q zmm19,zmm15,zmm17
vpshldq zmm22,zmm13,zmm14,0x24
vpsllq zmm17,zmm13,0x22
vpshldq zmm14,zmm14,zmm2,0x18
vpandq zmm18,zmm19,zmm23
vpshldq zmm2,zmm2,zmm19,0xc
vpsrlq zmm13,zmm13,0x12
vpandq zmm17,zmm17,zmm5
vpmullq zmm0,zmm18,zmm31
vpmadd52luq zmm13,zmm18,zmm26
vpsrlq zmm21,zmm22,0x2
vpsllq zmm10,zmm22,0x32
vpsrlq zmm22,zmm14,0x2
vpternlogq zmm21,zmm17,zmm4,0xec
vpmullq zmm17,zmm18,zmm3
vpandq zmm10,zmm10,zmm5
vpsllq zmm3,zmm14,0x32
vpsrlq zmm14,zmm2,0x2
vpsllq zmm2,zmm2,0x32
vpternlogq zmm22,zmm10,zmm4,0xec
vpandq zmm10,zmm3,zmm5
vpandq zmm2,zmm2,zmm5
vpsrlq zmm3,zmm19,0x2
vpternlogq zmm14,zmm10,zmm4,0xec
vpternlogq zmm3,zmm2,zmm4,0xec
vpsubq zmm13{k1},zmm30,zmm13
vpaddq zmm10,zmm0,zmm14
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20b0f]        # 98b0 <GCC_except_table830+0x136c>
vpaddq zmm21,zmm17,zmm21
vpsubq zmm10{k1},zmm30,zmm10
vpsubq zmm21{k1},zmm30,zmm21
vpmullq zmm2,zmm18,zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20b15]        # 98d8 <GCC_except_table830+0x1394>
vpaddq zmm2,zmm2,zmm3
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff20985]        # 9758 <GCC_except_table830+0x1214>
vpsubq zmm2{k1},zmm30,zmm2
vpblendmq zmm14{k1},zmm0,zmm3
vpandq zmm0,zmm11,QWORD BCST [rip+0xfffffffffff2095f]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff209bd]        # 97b0 <GCC_except_table830+0x126c>
vpshldq zmm11,zmm9,zmm11,0xc
vpshldq zmm9,zmm1,zmm9,0x18
vpshldq zmm1,zmm20,zmm1,0x24
vpaddq zmm2,zmm2,zmm14
vpsubq zmm2,zmm2,zmm0
vmovdqa64 ZMMWORD PTR [rsp+0x2c0],zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20b52]        # 9978 <GCC_except_table830+0x1434>
vpsraq zmm19,zmm2,0x34
vpblendmq zmm17{k1},zmm0,zmm3
vpandq zmm0,zmm11,QWORD BCST [rip+0xfffffffffff2090b]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff20a39]        # 9880 <GCC_except_table830+0x133c>
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff209bf]        # 9810 <GCC_except_table830+0x12cc>
vpaddq zmm10,zmm10,zmm17
vpaddq zmm10,zmm10,zmm19
vmovdqa64 ZMMWORD PTR [rsp+0x4c0],zmm0
vpsubq zmm10,zmm10,zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff20adb]        # 9950 <GCC_except_table830+0x140c>
vpmadd52luq zmm22,zmm18,zmm3
vpbroadcastq zmm18,QWORD PTR [rip+0xfffffffffff20a63]        # 98e8 <GCC_except_table830+0x13a4>
vpsubq zmm22{k1},zmm30,zmm22
vpblendmq zmm11{k1},zmm0,zmm11
vpandq zmm0,zmm9,QWORD BCST [rip+0xfffffffffff208ad]        # 9748 <GCC_except_table830+0x1204>
vpaddq zmm19,zmm22,zmm11
vpsraq zmm22,zmm10,0x34
vpaddq zmm19,zmm19,zmm22
vpsubq zmm9,zmm19,zmm0
vmovdqa64 ZMMWORD PTR [rsp+0x980],zmm0
vpbroadcastq zmm19,QWORD PTR [rip+0xfffffffffff20a3a]        # 9900 <GCC_except_table830+0x13bc>
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff209a8]        # 9878 <GCC_except_table830+0x1334>
vpsraq zmm22,zmm9,0x34
vpblendmq zmm19{k1},zmm0,zmm19
vpandq zmm0,zmm1,QWORD BCST [rip+0xfffffffffff20861]        # 9748 <GCC_except_table830+0x1204>
vpaddq zmm21,zmm21,zmm19
vpaddq zmm21,zmm21,zmm22
vmovdqu64 ymm22,YMMWORD PTR [r9+0x60]
lea    r9,[rsp+0x740]
vinserti64x4 zmm22,zmm22,YMMWORD PTR [r12+0x60],0x1
vmovdqa64 ZMMWORD PTR [rsp+0x9c0],zmm0
vpsubq zmm1,zmm21,zmm0
vpbroadcastq zmm0,QWORD PTR [rip+0xfffffffffff208cd]        # 97f0 <GCC_except_table830+0x12ac>
vpblendmq zmm21{k1},zmm0,zmm18
vpsraq zmm18,zmm1,0x34
vpaddq zmm13,zmm13,zmm21
vpaddq zmm13,zmm13,zmm18
vmovdqu64 ymm18,YMMWORD PTR [rdx+0x60]
vinserti64x4 zmm18,zmm18,YMMWORD PTR [r11+0x60],0x1
vpunpcklqdq zmm24,zmm18,zmm22
vpunpckhqdq zmm25,zmm18,zmm22
vmovdqu64 ymm18,YMMWORD PTR [rax+0x60]
vmovdqu64 ymm22,YMMWORD PTR [rsi+0x60]
mov    sil,0x1
lea    rax,[rsp+0xc40]
vinserti64x4 zmm18,zmm18,YMMWORD PTR [rcx+0x60],0x1
vinserti64x4 zmm22,zmm22,YMMWORD PTR [r15+0x60],0x1
vmovdqa64 zmm29,zmm24
lea    rcx,[rsp+0x600]
vpunpcklqdq zmm27,zmm18,zmm22
vpunpckhqdq zmm28,zmm18,zmm22
vmovdqa64 zmm18,zmm25
vpermt2q zmm18,zmm12,zmm28
vpermt2q zmm24,zmm12,zmm27
vpermt2q zmm29,zmm15,zmm27
vpermt2q zmm25,zmm15,zmm28
vpshldq zmm27,zmm18,zmm24,0x24
vpsllq zmm22,zmm18,0x22
vpshldq zmm24,zmm24,zmm25,0x18
vpandq zmm22,zmm22,zmm5
vpsrlq zmm0,zmm27,0x2
vpsllq zmm12,zmm27,0x32
vpsrlq zmm27,zmm24,0x2
vpsllq zmm15,zmm24,0x32
vpternlogq zmm0,zmm22,zmm4,0xec
vpandq zmm22,zmm29,zmm23
vpmullq zmm23,zmm22,QWORD BCST [rip+0xfffffffffff207f2]        # 97f8 <GCC_except_table830+0x12b4>
vpaddq zmm0,zmm23,zmm0
vpandq zmm23,zmm12,zmm5
vpmullq zmm12,zmm22,zmm31
vmovdqa64 zmm31,ZMMWORD PTR [rsp+0xc00]
vpternlogq zmm27,zmm23,zmm4,0xec
vpandq zmm23,zmm15,zmm5
vpshldq zmm15,zmm25,zmm29,0xc
vpsubq zmm0{k1},zmm30,zmm0
vpmadd52luq zmm27,zmm22,zmm3
vpaddq zmm0,zmm0,zmm19
vpsllq zmm24,zmm15,0x32
vpsrlq zmm25,zmm15,0x2
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0x380]
vpandq zmm24,zmm24,zmm5
vpsrlq zmm5,zmm29,0x2
vpternlogq zmm25,zmm23,zmm4,0xec
vpternlogq zmm5,zmm24,zmm4,0xec
vpmullq zmm4,zmm22,QWORD BCST [rip+0xfffffffffff2082f]        # 98b0 <GCC_except_table830+0x136c>
vpaddq zmm23,zmm12,zmm25
vpandq zmm25,zmm2,QWORD BCST [rip+0xfffffffffff206b7]        # 9748 <GCC_except_table830+0x1204>
vpxor  xmm2,xmm2,xmm2
vpsubq zmm23{k1},zmm30,zmm23
vpsubq zmm27{k1},zmm30,zmm27
vpaddq zmm17,zmm23,zmm17
vpaddq zmm11,zmm27,zmm11
vpaddq zmm24,zmm4,zmm5
vpandq zmm4,zmm8,QWORD BCST [rip+0xfffffffffff2068b]        # 9748 <GCC_except_table830+0x1204>
vpshldq zmm8,zmm7,zmm8,0xc
vpandq zmm3,zmm8,QWORD BCST [rip+0xfffffffffff2067a]        # 9748 <GCC_except_table830+0x1204>
vpshldq zmm7,zmm6,zmm7,0x18
vpshldq zmm6,zmm16,zmm6,0x24
vpbroadcastq zmm5,QWORD PTR [rip+0xfffffffffff20672]        # 9758 <GCC_except_table830+0x1214>
vpsubq zmm24{k1},zmm30,zmm24
vpaddq zmm14,zmm24,zmm14
vpandq zmm24,zmm10,QWORD BCST [rip+0xfffffffffff2064c]        # 9748 <GCC_except_table830+0x1204>
vpsubq zmm23,zmm14,zmm4
vmovdqa64 ZMMWORD PTR [rsp+0x17c0],zmm3
vpandq zmm28,zmm23,QWORD BCST [rip+0xfffffffffff20634]        # 9748 <GCC_except_table830+0x1204>
vpsubq zmm5,zmm5,zmm25
vmovdqa64 ZMMWORD PTR [rsp+0x1800],zmm4
vpsrlq zmm4,zmm16,0x10
vpsraq zmm14,zmm23,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x1140],zmm5
vmovdqa64 zmm23,ZMMWORD PTR [rsp+0x10c0]
vmovdqa64 ZMMWORD PTR [rsp+0x1700],zmm4
vpaddq zmm14,zmm17,zmm14
vpsubq zmm27,zmm14,zmm3
vpandq zmm3,zmm7,QWORD BCST [rip+0xfffffffffff205ea]        # 9748 <GCC_except_table830+0x1204>
vpandq zmm19,zmm27,QWORD BCST [rip+0xfffffffffff205e0]        # 9748 <GCC_except_table830+0x1204>
vpsraq zmm8,zmm27,0x34
vmovdqa64 zmm27,ZMMWORD PTR [rsp+0x1080]
vpaddq zmm8,zmm11,zmm8
vpandq zmm11,zmm1,QWORD BCST [rip+0xfffffffffff205c1]        # 9748 <GCC_except_table830+0x1204>
vpandq zmm1,zmm9,QWORD BCST [rip+0xfffffffffff205b7]        # 9748 <GCC_except_table830+0x1204>
vpxor  xmm9,xmm9,xmm9
vpsubq zmm8,zmm8,zmm3
vmovdqa64 ZMMWORD PTR [rsp+0x1780],zmm3
vpandq zmm3,zmm6,QWORD BCST [rip+0xfffffffffff2059a]        # 9748 <GCC_except_table830+0x1204>
vpsraq zmm6,zmm5,0x34
vpsraq zmm7,zmm8,0x34
vpsubq zmm6,zmm6,zmm24
vpaddq zmm5,zmm6,QWORD BCST [rip+0xfffffffffff205e4]        # 97b0 <GCC_except_table830+0x126c>
vpaddq zmm0,zmm0,zmm7
vpsrlq zmm7,zmm18,0x12
vpmadd52luq zmm7,zmm22,zmm26
vpbroadcastq zmm26,QWORD PTR [rip+0xfffffffffff205b7]        # 97a0 <GCC_except_table830+0x125c>
vpxord xmm22,xmm22,xmm22
vpsubq zmm7{k1},zmm30,zmm7
vpsubq zmm17,zmm0,zmm3
vpaddq zmm7,zmm7,zmm21
vmovdqa64 ZMMWORD PTR [rsp+0x1740],zmm3
vpandq zmm3,zmm31,QWORD BCST [rip+0xfffffffffff20535]        # 9748 <GCC_except_table830+0x1204>
vpsraq zmm0,zmm17,0x34
vpsraq zmm6,zmm5,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x1380],zmm5
vpandq zmm17,zmm17,QWORD BCST [rip+0xfffffffffff20515]        # 9748 <GCC_except_table830+0x1204>
vpaddq zmm18,zmm7,zmm0
vpandq zmm0,zmm15,QWORD BCST [rip+0xfffffffffff20505]        # 9748 <GCC_except_table830+0x1204>
vpsubq zmm6,zmm6,zmm1
vpaddq zmm5,zmm6,QWORD BCST [rip+0xfffffffffff2062d]        # 9880 <GCC_except_table830+0x133c>
vmovdqa64 ZMMWORD PTR [rsp+0x1100],zmm3
vmovdqa64 ZMMWORD PTR [rsp+0x440],zmm0
vpsubq zmm0,zmm3,zmm0
vpxor  xmm3,xmm3,xmm3
vpsraq zmm6,zmm5,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x1900],zmm5
vpaddq zmm12,zmm0,zmm26
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x340]
vpandq zmm10,zmm12,QWORD BCST [rip+0xfffffffffff204b4]        # 9748 <GCC_except_table830+0x1204>
vpsubq zmm6,zmm6,zmm11
vpaddq zmm5,zmm6,QWORD BCST [rip+0xfffffffffff2065c]        # 9900 <GCC_except_table830+0x13bc>
vpandq zmm7,zmm0,QWORD BCST [rip+0xfffffffffff2049a]        # 9748 <GCC_except_table830+0x1204>
vmovdqa ymm0,YMMWORD PTR [rip+0xfffffffffff20baa]        # 9e60 <GCC_except_table830+0x191c>
vpsraq zmm6,zmm5,0x34
vmovdqa64 ZMMWORD PTR [rsp+0x1880],zmm5
vpmadd52huq zmm2,zmm25,zmm7
vmovdqu YMMWORD PTR [rsp+0x600],ymm0
vpxor  xmm0,xmm0,xmm0
vpmadd52luq zmm0,zmm25,zmm7
mov    QWORD PTR [rsp+0x620],rdi
vpmadd52huq zmm9,zmm1,zmm7
vpbroadcastq zmm29,QWORD PTR [rsp+0x608]
vpbroadcastq zmm30,QWORD PTR [rsp+0x610]
vpbroadcastq zmm14,QWORD PTR [rsp+0x620]
vpmadd52luq zmm2,zmm24,zmm7
vpmadd52luq zmm0,zmm28,zmm10
vpmadd52luq zmm3,zmm0,QWORD BCST [rip+0xfffffffffff20625]        # 9948 <GCC_except_table830+0x1404>
vpaddq zmm0,zmm0,QWORD BCST [rip+0xfffffffffff2041b]        # 9748 <GCC_except_table830+0x1204>
vpmadd52luq zmm9,zmm11,zmm7
vpmadd52huq zmm2,zmm28,zmm10
vmovdqa64 ZMMWORD PTR [rsp+0xf00],zmm29
vmovdqa64 ZMMWORD PTR [rsp+0x13c0],zmm14
vmovdqa64 ZMMWORD PTR [rsp+0xec0],zmm30
vpmadd52luq zmm2,zmm19,zmm10
vpsrlq zmm0,zmm0,0x34
vpmadd52huq zmm0,zmm3,zmm26
vpmadd52luq zmm2,zmm3,zmm29
vpaddq zmm2,zmm2,zmm0
vpsrlq zmm0,zmm20,0x10
vpandq zmm20,zmm8,QWORD BCST [rip+0xfffffffffff203c7]        # 9748 <GCC_except_table830+0x1204>
vpxor  xmm8,xmm8,xmm8
vpmadd52huq zmm8,zmm11,zmm7
vmovdqa64 ZMMWORD PTR [rsp+0x340],zmm0
vpsubq zmm0,zmm13,zmm0
vpandq zmm16,zmm0,QWORD BCST [rip+0xfffffffffff203a4]        # 9748 <GCC_except_table830+0x1204>
vpxor  xmm13,xmm13,xmm13
vpmadd52huq zmm13,zmm24,zmm7
vpsubq zmm5,zmm6,zmm0
vpsubq zmm0,zmm18,zmm4
vpandq zmm21,zmm0,QWORD BCST [rip+0xfffffffffff20383]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm18,QWORD PTR [rsp+0x618]
vmovdqa64 ZMMWORD PTR [rsp+0x18c0],zmm5
vmovdqa64 zmm5,ZMMWORD PTR [rsp+0xa00]
vpmadd52luq zmm13,zmm1,zmm7
vpmadd52huq zmm9,zmm20,zmm10
vpmadd52luq zmm8,zmm16,zmm7
vpmadd52huq zmm13,zmm19,zmm10
vpmadd52huq zmm22,zmm16,zmm7
vmovdqa64 ZMMWORD PTR [rsp+0x1180],zmm18
vpshldq zmm0,zmm5,zmm15,0xc
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0xf40]
vpmadd52luq zmm9,zmm17,zmm10
vpmadd52huq zmm8,zmm17,zmm10
vpmadd52luq zmm13,zmm20,zmm10
vpmadd52huq zmm22,zmm21,zmm10
vpmadd52huq zmm9,zmm3,zmm30
vpmadd52luq zmm8,zmm21,zmm10
vpmadd52huq zmm13,zmm3,zmm29
vpmadd52huq zmm22,zmm3,zmm14
vpmadd52luq zmm9,zmm3,zmm18
vpmadd52huq zmm8,zmm3,zmm18
vpmadd52luq zmm13,zmm3,zmm30
vpmadd52luq zmm8,zmm3,zmm14
vpandq zmm3,zmm0,QWORD BCST [rip+0xfffffffffff202e1]        # 9748 <GCC_except_table830+0x1204>
vpshldq zmm0,zmm23,zmm31,0xc
vpandq zmm4,zmm0,QWORD BCST [rip+0xfffffffffff202d0]        # 9748 <GCC_except_table830+0x1204>
vpsraq zmm0,zmm12,0x34
vpaddq zmm0,zmm0,zmm4
vmovdqa64 ZMMWORD PTR [rsp+0xc00],zmm4
vpxor  xmm4,xmm4,xmm4
vmovdqa64 ZMMWORD PTR [rsp+0x380],zmm3
vpsubq zmm0,zmm0,zmm3
vpaddq zmm12,zmm0,QWORD BCST [rip+0xfffffffffff204e7]        # 9990 <GCC_except_table830+0x144c>
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0xbc0]
vpandq zmm6,zmm0,QWORD BCST [rip+0xfffffffffff2028d]        # 9748 <GCC_except_table830+0x1204>
vpandq zmm31,zmm12,QWORD BCST [rip+0xfffffffffff20283]        # 9748 <GCC_except_table830+0x1204>
vpxor  xmm0,xmm0,xmm0
vpmadd52huq zmm9,zmm24,zmm6
vpmadd52huq zmm8,zmm1,zmm6
vpmadd52huq zmm22,zmm11,zmm6
vpmadd52luq zmm2,zmm25,zmm6
vpmadd52huq zmm13,zmm25,zmm6
vpmadd52huq zmm4,zmm16,zmm6
vpmadd52luq zmm9,zmm1,zmm6
vpmadd52luq zmm8,zmm11,zmm6
vpmadd52luq zmm22,zmm16,zmm6
vpmadd52luq zmm2,zmm28,zmm31
vpmadd52luq zmm0,zmm2,QWORD BCST [rip+0xfffffffffff20439]        # 9948 <GCC_except_table830+0x1404>
vpmadd52luq zmm13,zmm24,zmm6
vpaddq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff20229]        # 9748 <GCC_except_table830+0x1204>
vpmadd52huq zmm4,zmm21,zmm31
vpmadd52huq zmm9,zmm19,zmm31
vpmadd52huq zmm8,zmm20,zmm31
vpmadd52huq zmm22,zmm17,zmm31
vpmadd52huq zmm13,zmm28,zmm31
vpmadd52luq zmm9,zmm20,zmm31
vpmadd52luq zmm8,zmm17,zmm31
vpmadd52luq zmm22,zmm21,zmm31
vpmadd52luq zmm13,zmm19,zmm31
vpsrlq zmm2,zmm2,0x34
vpmadd52huq zmm2,zmm0,zmm26
vpmadd52huq zmm4,zmm0,zmm14
vpmadd52huq zmm9,zmm0,zmm29
vpmadd52huq zmm8,zmm0,zmm30
vpmadd52huq zmm22,zmm0,zmm18
vpmadd52luq zmm13,zmm0,zmm29
vpmadd52luq zmm9,zmm0,zmm30
vpmadd52luq zmm8,zmm0,zmm18
vpmadd52luq zmm22,zmm0,zmm14
vpshldq zmm0,zmm15,zmm5,0x18
vpandq zmm3,zmm0,QWORD BCST [rip+0xfffffffffff201a5]        # 9748 <GCC_except_table830+0x1204>
vpshldq zmm0,zmm27,zmm23,0x18
vpandq zmm23,zmm0,QWORD BCST [rip+0xfffffffffff20194]        # 9748 <GCC_except_table830+0x1204>
vpsraq zmm0,zmm12,0x34
vpaddq zmm2,zmm13,zmm2
vpxor  xmm13,xmm13,xmm13
vmovdqa64 zmm5,ZMMWORD PTR [rsp+0x1c0]
vpxor  xmm12,xmm12,xmm12
vpaddq zmm0,zmm0,zmm23
vmovdqa64 ZMMWORD PTR [rsp+0xa00],zmm3
vmovdqa64 ZMMWORD PTR [rsp+0x10c0],zmm23
vpshldq zmm5,zmm5,zmm27,0x24
vpandq zmm5,zmm5,QWORD BCST [rip+0xfffffffffff2014e]        # 9748 <GCC_except_table830+0x1204>
vpsubq zmm0,zmm0,zmm3
vmovdqa64 zmm3,ZMMWORD PTR [rsp+0xfc0]
vpaddq zmm0,zmm0,QWORD BCST [rip+0xfffffffffff201ae]        # 97c0 <GCC_except_table830+0x127c>
vpandq zmm23,zmm3,QWORD BCST [rip+0xfffffffffff2012c]        # 9748 <GCC_except_table830+0x1204>
vpandq zmm26,zmm0,QWORD BCST [rip+0xfffffffffff20122]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff20170]        # 97a0 <GCC_except_table830+0x125c>
vmovdqa64 ZMMWORD PTR [rsp+0x1080],zmm5
vpsraq zmm0,zmm0,0x34
vpmadd52luq zmm2,zmm25,zmm23
vpmadd52huq zmm9,zmm25,zmm23
vpmadd52huq zmm22,zmm1,zmm23
vpmadd52huq zmm4,zmm11,zmm23
vpmadd52huq zmm8,zmm24,zmm23
vpmadd52huq zmm12,zmm16,zmm23
vpaddq zmm0,zmm0,zmm5
vpxor  xmm5,xmm5,xmm5
vpmadd52luq zmm2,zmm28,zmm26
vpmadd52luq zmm13,zmm2,QWORD BCST [rip+0xfffffffffff202cb]        # 9948 <GCC_except_table830+0x1404>
vpaddq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff200c1]        # 9748 <GCC_except_table830+0x1204>
vpmadd52luq zmm9,zmm24,zmm23
vpmadd52luq zmm22,zmm11,zmm23
vpmadd52luq zmm4,zmm16,zmm23
vpmadd52luq zmm8,zmm1,zmm23
vpmadd52huq zmm12,zmm21,zmm26
vpmadd52huq zmm9,zmm28,zmm26
vpmadd52huq zmm22,zmm20,zmm26
vpmadd52huq zmm4,zmm17,zmm26
vpmadd52huq zmm8,zmm19,zmm26
vpsrlq zmm2,zmm2,0x34
vpmadd52luq zmm9,zmm19,zmm26
vpmadd52luq zmm22,zmm17,zmm26
vpmadd52luq zmm4,zmm21,zmm26
vpmadd52luq zmm8,zmm20,zmm26
vpmadd52huq zmm2,zmm13,zmm3
vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x180]
vpmadd52huq zmm12,zmm13,zmm14
vpmadd52luq zmm9,zmm13,zmm29
vpmadd52huq zmm22,zmm13,zmm30
vpmadd52huq zmm4,zmm13,zmm18
vpmadd52huq zmm8,zmm13,zmm29
vpaddq zmm2,zmm9,zmm2
vpmadd52luq zmm22,zmm13,zmm18
vpmadd52luq zmm4,zmm13,zmm14
vpmadd52luq zmm8,zmm13,zmm30
vpxor  xmm13,xmm13,xmm13
vpmadd52huq zmm13,zmm28,zmm7
vpshldq zmm9,zmm3,zmm15,0x24
vpandq zmm3,zmm9,QWORD BCST [rip+0xfffffffffff2000c]        # 9748 <GCC_except_table830+0x1204>
vpmadd52luq zmm13,zmm19,zmm7
vmovdqa64 ZMMWORD PTR [rsp+0xf40],zmm3
vpsubq zmm0,zmm0,zmm3
vpaddq zmm3,zmm0,QWORD BCST [rip+0xfffffffffff200ce]        # 9828 <GCC_except_table830+0x12e4>
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x11c0]
vpandq zmm27,zmm0,QWORD BCST [rip+0xfffffffffff1ffdc]        # 9748 <GCC_except_table830+0x1204>
vpandq zmm15,zmm3,QWORD BCST [rip+0xfffffffffff1ffd2]        # 9748 <GCC_except_table830+0x1204>
vpxor  xmm0,xmm0,xmm0
vmovdqa64 ZMMWORD PTR [rsp+0xfc0],zmm3
vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x1140]
vpmadd52huq zmm22,zmm24,zmm27
vpmadd52huq zmm4,zmm1,zmm27
vpmadd52huq zmm12,zmm11,zmm27
vpmadd52luq zmm2,zmm25,zmm27
vpmadd52huq zmm8,zmm25,zmm27
vpmadd52huq zmm5,zmm16,zmm27
vpmadd52luq zmm22,zmm1,zmm27
vpmadd52luq zmm4,zmm11,zmm27
vpmadd52luq zmm12,zmm16,zmm27
vpmadd52luq zmm2,zmm28,zmm15
vpmadd52luq zmm0,zmm2,QWORD BCST [rip+0xfffffffffff20178]        # 9948 <GCC_except_table830+0x1404>
vpmadd52luq zmm8,zmm24,zmm27
vpaddq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff1ff68]        # 9748 <GCC_except_table830+0x1204>
vpmadd52huq zmm5,zmm21,zmm15
vpmadd52huq zmm22,zmm19,zmm15
vpmadd52huq zmm4,zmm20,zmm15
vpmadd52huq zmm12,zmm17,zmm15
vpmadd52huq zmm8,zmm28,zmm15
vpmadd52luq zmm22,zmm20,zmm15
vpmadd52luq zmm4,zmm17,zmm15
vpmadd52luq zmm12,zmm21,zmm15
vpmadd52luq zmm8,zmm19,zmm15
vpsrlq zmm2,zmm2,0x34
vpmadd52huq zmm2,zmm0,QWORD BCST [rip+0xfffffffffff1ff79]        # 97a0 <GCC_except_table830+0x125c>
vpmadd52huq zmm5,zmm0,zmm14
vpmadd52huq zmm22,zmm0,zmm29
vpmadd52huq zmm4,zmm0,zmm30
vpmadd52huq zmm12,zmm0,zmm18
vpmadd52luq zmm8,zmm0,zmm29
vpmadd52luq zmm22,zmm0,zmm30
vpmadd52luq zmm4,zmm0,zmm18
vpmadd52luq zmm12,zmm0,zmm14
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x1040]
vpxord xmm18,xmm18,xmm18
vpmadd52huq zmm18,zmm19,zmm7
vpxord xmm30,xmm30,xmm30
vpandq zmm29,zmm0,QWORD BCST [rip+0xfffffffffff1fecd]        # 9748 <GCC_except_table830+0x1204>
vpaddq zmm2,zmm8,zmm2
vpmadd52luq zmm18,zmm20,zmm7
vpxor  xmm0,xmm0,xmm0
vpmadd52luq zmm2,zmm25,zmm29
vpmadd52huq zmm0,zmm16,zmm29
vpmadd52huq zmm4,zmm24,zmm29
vpmadd52huq zmm22,zmm25,zmm29
vpmadd52huq zmm12,zmm1,zmm29
vpxord xmm25,xmm25,xmm25
vpmadd52huq zmm25,zmm21,zmm7
vpmadd52huq zmm5,zmm11,zmm29
vmovdqa64 ZMMWORD PTR [rsp+0x1840],zmm2
vpxor  xmm2,xmm2,xmm2
vpmadd52huq zmm2,zmm20,zmm7
vmovdqa64 ZMMWORD PTR [rsp+0xbc0],zmm0
vpxor  xmm0,xmm0,xmm0
vpmadd52huq zmm0,zmm17,zmm7
vpmadd52luq zmm4,zmm1,zmm29
vpxor  xmm1,xmm1,xmm1
vpmadd52luq zmm1,zmm28,zmm7
vpmadd52luq zmm12,zmm11,zmm29
vpmadd52luq zmm22,zmm24,zmm29
vmovdqa64 zmm24,zmm19
vpmadd52luq zmm5,zmm16,zmm29
vpmadd52luq zmm2,zmm17,zmm7
vpmadd52luq zmm0,zmm21,zmm7
vpandq zmm7,zmm3,QWORD BCST [rip+0xfffffffffff1fe2b]        # 9748 <GCC_except_table830+0x1204>
vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x1380]
vpandq zmm8,zmm3,QWORD BCST [rip+0xfffffffffff1fe19]        # 9748 <GCC_except_table830+0x1204>
vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x1900]
vpandq zmm9,zmm3,QWORD BCST [rip+0xfffffffffff1fe07]        # 9748 <GCC_except_table830+0x1204>
vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x1880]
vpandq zmm14,zmm3,QWORD BCST [rip+0xfffffffffff1fdf5]        # 9748 <GCC_except_table830+0x1204>
vmovdqa64 zmm3,ZMMWORD PTR [rsp+0x18c0]
vpaddq zmm11,zmm3,QWORD BCST [rip+0xfffffffffff1ff83]        # 98e8 <GCC_except_table830+0x13a4>
vmovaps ymm3,YMMWORD PTR [rip+0xfffffffffff204f3]        # 9e60 <GCC_except_table830+0x191c>
vpandq zmm11,zmm11,QWORD BCST [rip+0xfffffffffff1fdd1]        # 9748 <GCC_except_table830+0x1204>
vpmadd52luq zmm1,zmm7,zmm10
vpmadd52luq zmm30,zmm1,QWORD BCST [rip+0xfffffffffff1ffc1]        # 9948 <GCC_except_table830+0x1404>
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1fdb7]        # 9748 <GCC_except_table830+0x1204>
vpmadd52huq zmm13,zmm7,zmm10
vpmadd52huq zmm18,zmm8,zmm10
vpmadd52huq zmm2,zmm9,zmm10
vpmadd52huq zmm0,zmm14,zmm10
vmovups YMMWORD PTR [rsp+0x600],ymm3
mov    QWORD PTR [rsp+0x620],rdi
vpbroadcastq zmm3,QWORD PTR [rip+0xfffffffffff1fddc]        # 97a0 <GCC_except_table830+0x125c>
vpmadd52luq zmm13,zmm8,zmm10
vpbroadcastq zmm19,QWORD PTR [rsp+0x608]
vpbroadcastq zmm16,QWORD PTR [rsp+0x610]
vpmadd52luq zmm18,zmm9,zmm10
vpmadd52huq zmm25,zmm11,zmm10
vpmadd52luq zmm2,zmm14,zmm10
vpmadd52luq zmm0,zmm11,zmm10
vpbroadcastq zmm10,QWORD PTR [rsp+0x620]
vpsrlq zmm1,zmm1,0x34
vpmadd52huq zmm1,zmm30,zmm3
vpmadd52luq zmm13,zmm30,zmm19
vpmadd52huq zmm18,zmm30,zmm19
vpmadd52huq zmm2,zmm30,zmm16
vpmadd52huq zmm25,zmm30,zmm10
vpaddq zmm3,zmm13,zmm1
vpbroadcastq zmm13,QWORD PTR [rsp+0x618]
vpmadd52luq zmm18,zmm30,zmm16
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm1,zmm21,zmm6
vpmadd52luq zmm3,zmm28,zmm6
vpmadd52huq zmm25,zmm17,zmm6
vpmadd52huq zmm18,zmm28,zmm6
vpmadd52huq zmm1,zmm11,zmm31
vpmadd52luq zmm3,zmm7,zmm31
vpmadd52luq zmm25,zmm21,zmm6
vpmadd52huq zmm0,zmm30,zmm13
vpmadd52luq zmm2,zmm30,zmm13
vpmadd52luq zmm18,zmm24,zmm6
vpmadd52huq zmm25,zmm14,zmm31
vpmadd52luq zmm0,zmm30,zmm10
vpmadd52huq zmm2,zmm24,zmm6
vpbroadcastq zmm30,QWORD PTR [rip+0xfffffffffff1fead]        # 9948 <GCC_except_table830+0x1404>
vpmadd52huq zmm18,zmm7,zmm31
vpmadd52luq zmm25,zmm11,zmm31
vpmadd52huq zmm0,zmm20,zmm6
vpmadd52luq zmm2,zmm20,zmm6
vpmadd52luq zmm18,zmm8,zmm31
vpmadd52luq zmm0,zmm17,zmm6
vpmadd52huq zmm2,zmm8,zmm31
vpxor  xmm6,xmm6,xmm6
vpmadd52luq zmm6,zmm3,zmm30
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff1fc6f]        # 9748 <GCC_except_table830+0x1204>
vpmadd52huq zmm0,zmm9,zmm31
vpmadd52luq zmm2,zmm9,zmm31
vpmadd52huq zmm25,zmm6,zmm13
vpmadd52huq zmm1,zmm6,zmm10
vpmadd52luq zmm18,zmm6,zmm19
vpmadd52luq zmm0,zmm14,zmm31
vpbroadcastq zmm31,QWORD PTR [rip+0xfffffffffff1fc99]        # 97a0 <GCC_except_table830+0x125c>
vpmadd52luq zmm25,zmm6,zmm10
vpsrlq zmm3,zmm3,0x34
vpmadd52huq zmm2,zmm6,zmm19
vpmadd52huq zmm1,zmm17,zmm23
vpmadd52huq zmm0,zmm6,zmm16
vpmadd52huq zmm25,zmm20,zmm23
vpmadd52luq zmm2,zmm6,zmm16
vpmadd52luq zmm1,zmm21,zmm23
vpmadd52huq zmm3,zmm6,zmm31
vpmadd52luq zmm0,zmm6,zmm13
vpmadd52luq zmm25,zmm17,zmm23
vpmadd52huq zmm2,zmm28,zmm23
vpmadd52huq zmm1,zmm14,zmm26
vpxord xmm31,xmm31,xmm31
vpmadd52huq zmm31,zmm21,zmm23
vpxor  xmm6,xmm6,xmm6
vpaddq zmm3,zmm18,zmm3
vpmadd52huq zmm0,zmm24,zmm23
vpmadd52huq zmm25,zmm9,zmm26
vpmadd52luq zmm2,zmm24,zmm23
vpmadd52luq zmm1,zmm11,zmm26
vpmadd52huq zmm31,zmm11,zmm26
vpsraq zmm18,ZMMWORD PTR [rsp+0xfc0],0x34
vpmadd52luq zmm3,zmm28,zmm23
vpmadd52luq zmm0,zmm20,zmm23
vpmadd52luq zmm25,zmm14,zmm26
vpmadd52huq zmm2,zmm7,zmm26
vpbroadcastq zmm23,QWORD PTR [rip+0xfffffffffff1fbeb]        # 97a0 <GCC_except_table830+0x125c>
vpmadd52luq zmm3,zmm7,zmm26
vpmadd52huq zmm0,zmm8,zmm26
vpmadd52luq zmm2,zmm8,zmm26
vpmadd52luq zmm6,zmm3,zmm30
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff1fb71]        # 9748 <GCC_except_table830+0x1204>
vpmadd52luq zmm0,zmm9,zmm26
vpbroadcastq zmm26,QWORD PTR [rip+0xfffffffffff1fcb1]        # 9898 <GCC_except_table830+0x1354>
vpmadd52huq zmm25,zmm6,zmm16
vpmadd52huq zmm1,zmm6,zmm13
vpmadd52huq zmm31,zmm6,zmm10
vpmadd52luq zmm2,zmm6,zmm19
vpmadd52huq zmm0,zmm6,zmm19
vpmadd52luq zmm25,zmm6,zmm13
vpmadd52luq zmm1,zmm6,zmm10
vpsrlq zmm3,zmm3,0x34
vpmadd52huq zmm31,zmm17,zmm27
vpmadd52huq zmm3,zmm6,zmm23
vpmadd52luq zmm0,zmm6,zmm16
vpxor  xmm6,xmm6,xmm6
vpmadd52huq zmm25,zmm24,zmm27
vpmadd52huq zmm1,zmm20,zmm27
vpmadd52luq zmm31,zmm21,zmm27
vpaddq zmm2,zmm2,zmm3
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm21,zmm27
vpmadd52luq zmm2,zmm28,zmm27
vpmadd52huq zmm0,zmm28,zmm27
vpmadd52luq zmm25,zmm20,zmm27
vpmadd52luq zmm1,zmm17,zmm27
vpmadd52huq zmm31,zmm14,zmm15
vpmadd52huq zmm3,zmm11,zmm15
vpmadd52luq zmm2,zmm7,zmm15
vpmadd52luq zmm0,zmm24,zmm27
vmovdqa64 zmm27,zmm23
vpmadd52huq zmm25,zmm8,zmm15
vpmadd52huq zmm1,zmm9,zmm15
vpmadd52luq zmm31,zmm11,zmm15
vpmadd52luq zmm6,zmm2,zmm30
vpaddq zmm2,zmm2,QWORD BCST [rip+0xfffffffffff1faa0]        # 9748 <GCC_except_table830+0x1204>
vpmadd52huq zmm0,zmm7,zmm15
vpmadd52luq zmm25,zmm9,zmm15
vpmadd52luq zmm1,zmm14,zmm15
vpmadd52huq zmm31,zmm6,zmm13
vpmadd52huq zmm3,zmm6,zmm10
vpmadd52luq zmm0,zmm8,zmm15
vpsrlq zmm15,ZMMWORD PTR [rsp+0x1c0],0x10
vpmadd52huq zmm25,zmm6,zmm19
vpmadd52huq zmm1,zmm6,zmm16
vpsrlq zmm2,zmm2,0x34
vpmadd52luq zmm31,zmm6,zmm10
vpmadd52huq zmm3,zmm17,zmm29
vpmadd52huq zmm2,zmm6,zmm23
vpmadd52luq zmm0,zmm6,zmm19
vmovdqa64 zmm23,ZMMWORD PTR [rsp+0x1840]
vpmadd52luq zmm25,zmm6,zmm16
vpmadd52luq zmm1,zmm6,zmm13
vpsrlq zmm6,ZMMWORD PTR [rsp+0x180],0x10
vpmadd52huq zmm31,zmm20,zmm29
vpmadd52luq zmm3,zmm21,zmm29
vpaddq zmm2,zmm0,zmm2
vpaddq zmm0,zmm18,zmm15
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0xbc0]
vmovdqa64 ZMMWORD PTR [rsp+0x11c0],zmm15
vpmadd52huq zmm1,zmm24,zmm29
vpmadd52huq zmm25,zmm28,zmm29
vpmadd52luq zmm2,zmm28,zmm29
vpmadd52luq zmm31,zmm17,zmm29
vpsubq zmm0,zmm0,zmm6
vpmadd52luq zmm1,zmm20,zmm29
vmovdqa64 ZMMWORD PTR [rsp+0x1040],zmm6
vpmadd52luq zmm25,zmm24,zmm29
vmovdqa64 zmm6,ZMMWORD PTR [rsp+0x300]
vpaddq zmm0,zmm0,zmm26
vpandq zmm0,zmm0,QWORD BCST [rip+0xfffffffffff1f9b9]        # 9748 <GCC_except_table830+0x1204>
vpmadd52huq zmm22,zmm28,zmm0
vpmadd52huq zmm12,zmm20,zmm0
vpmadd52huq zmm1,zmm8,zmm0
vpmadd52huq zmm4,zmm24,zmm0
vpmadd52huq zmm5,zmm17,zmm0
vpmadd52huq zmm25,zmm7,zmm0
vpmadd52huq zmm31,zmm9,zmm0
vpmadd52huq zmm3,zmm14,zmm0
vpmadd52luq zmm2,zmm7,zmm0
vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x240]
vpmadd52luq zmm23,zmm28,zmm0
vpmadd52huq zmm18,zmm21,zmm0
vpbroadcastq zmm28,QWORD PTR [rip+0xfffffffffff1f965]        # 9748 <GCC_except_table830+0x1204>
vpmadd52luq zmm22,zmm24,zmm0
vpmadd52luq zmm12,zmm17,zmm0
vpxord xmm17,xmm17,xmm17
vpmadd52huq zmm17,zmm21,zmm29
vpmadd52luq zmm1,zmm9,zmm0
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff1fb85]        # 9990 <GCC_except_table830+0x144c>
vmovdqa64 zmm24,ZMMWORD PTR [rsp+0x200]
vpmadd52luq zmm4,zmm20,zmm0
vpmadd52luq zmm5,zmm21,zmm0
vpmadd52luq zmm31,zmm14,zmm0
vpmadd52luq zmm25,zmm8,zmm0
vpmadd52luq zmm3,zmm11,zmm0
vmovdqa64 zmm21,ZMMWORD PTR [rsp+0x280]
vmovdqa64 zmm20,ZMMWORD PTR [rsp+0x1000]
vmovdqa64 zmm14,ZMMWORD PTR [rsp+0xf80]
vpmadd52huq zmm17,zmm11,zmm0
vpxor  xmm0,xmm0,xmm0
vpmadd52luq zmm0,zmm6,zmm30
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff1f9c5]        # 9828 <GCC_except_table830+0x12e4>
vpaddq zmm6,zmm6,zmm28
vpsrlq zmm6,zmm6,0x34
vpmadd52luq zmm7,zmm0,zmm9
vpmadd52huq zmm24,zmm0,zmm9
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff1f93a]        # 97c0 <GCC_except_table830+0x127c>
vpmadd52huq zmm6,zmm0,zmm27
vpmadd52huq zmm14,zmm0,zmm26
vpmadd52huq zmm20,zmm0,zmm11
vpaddq zmm6,zmm7,zmm6
vpsrlq zmm15,zmm6,0x34
vpmadd52huq zmm21,zmm0,zmm9
vpmadd52luq zmm24,zmm0,zmm9
vpmadd52luq zmm20,zmm0,zmm26
vpmadd52luq zmm21,zmm0,zmm11
vpaddq zmm7,zmm15,zmm24
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0xf00]
vpxor  xmm11,xmm11,xmm11
vpmadd52luq zmm11,zmm23,zmm30
vpsrlq zmm0,zmm7,0x34
vpaddq zmm8,zmm0,zmm21
vpsrlq zmm0,zmm8,0x34
vpaddq zmm9,zmm0,zmm20
vmovdqa64 zmm20,ZMMWORD PTR [rsp+0xec0]
vpsrlq zmm0,zmm9,0x34
vpmadd52huq zmm4,zmm11,zmm15
vpmadd52luq zmm22,zmm11,zmm15
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0x1180]
vpaddq zmm0,zmm0,zmm14
vpmadd52huq zmm12,zmm11,zmm20
vpmadd52luq zmm4,zmm11,zmm20
vmovdqa64 zmm20,ZMMWORD PTR [rsp+0x13c0]
vpmadd52huq zmm5,zmm11,zmm15
vpmadd52luq zmm12,zmm11,zmm15
vpaddq zmm15,zmm23,zmm28
vpsrlq zmm14,zmm15,0x34
vpmadd52huq zmm14,zmm11,zmm27
vpmadd52luq zmm5,zmm11,zmm20
vpmadd52huq zmm18,zmm11,zmm20
vpaddq zmm11,zmm22,zmm14
vpsrlq zmm14,zmm11,0x34
vpandq zmm11,zmm11,zmm28
vpaddq zmm4,zmm14,zmm4
vmovdqa64 ZMMWORD PTR [rsp+0xc40],zmm11
vpsrlq zmm14,zmm4,0x34
vpandq zmm4,zmm4,zmm28
vpaddq zmm12,zmm14,zmm12
vmovdqa64 ZMMWORD PTR [rsp+0xc80],zmm4
vpsrlq zmm14,zmm12,0x34
vpandq zmm4,zmm12,zmm28
vpaddq zmm5,zmm14,zmm5
vmovdqa64 ZMMWORD PTR [rsp+0xcc0],zmm4
vpsrlq zmm14,zmm5,0x34
vpandq zmm4,zmm5,zmm28
vpandq zmm5,zmm9,zmm28
vpaddq zmm14,zmm14,zmm18
vpxord xmm18,xmm18,xmm18
vpmadd52luq zmm18,zmm2,zmm30
vpaddq zmm2,zmm2,zmm28
vmovdqa64 ZMMWORD PTR [rsp+0xd00],zmm4
vpandq zmm4,zmm8,zmm28
vpsrlq zmm2,zmm2,0x34
vmovdqa64 ZMMWORD PTR [rsp+0xd40],zmm14
vpmadd52huq zmm1,zmm18,zmm19
vpmadd52huq zmm31,zmm18,zmm16
vpmadd52luq zmm25,zmm18,zmm19
vpmadd52huq zmm2,zmm18,zmm27
vpmadd52huq zmm3,zmm18,zmm13
vpmadd52huq zmm17,zmm18,zmm10
vpmadd52luq zmm1,zmm18,zmm16
vpmadd52luq zmm31,zmm18,zmm13
vpaddq zmm2,zmm25,zmm2
vpmadd52luq zmm3,zmm18,zmm10
vpsrlq zmm10,zmm2,0x34
vpandq zmm2,zmm2,zmm28
vmovdqa64 ZMMWORD PTR [rsp+0xd80],zmm2
vpaddq zmm1,zmm10,zmm1
vpsrlq zmm10,zmm1,0x34
vpandq zmm1,zmm1,zmm28
vpaddq zmm10,zmm10,zmm31
vmovdqa64 ymm31,YMMWORD PTR [rip+0xfffffffffff1fdf5]        # 9e60 <GCC_except_table830+0x191c>
vmovdqa64 ZMMWORD PTR [rsp+0xdc0],zmm1
vpsrlq zmm13,zmm10,0x34
vpandq zmm1,zmm10,zmm28
vpaddq zmm3,zmm13,zmm3
vmovdqa64 ZMMWORD PTR [rsp+0xe00],zmm1
vpsrlq zmm13,zmm3,0x34
vpandq zmm1,zmm3,zmm28
vpandq zmm3,zmm7,zmm28
vmovdqa64 ZMMWORD PTR [rsp+0xe40],zmm1
vpaddq zmm13,zmm13,zmm17
vpandq zmm1,zmm6,zmm28
vmovdqa64 ZMMWORD PTR [rsp+0xe80],zmm13
nop    DWORD PTR [rax]
vmovdqa64 zmm14,ZMMWORD PTR [rax]
vmovdqa64 zmm13,ZMMWORD PTR [rax+0x40]
vmovdqa64 zmm12,ZMMWORD PTR [rax+0x80]
vmovdqa64 zmm10,ZMMWORD PTR [rax+0xc0]
vmovdqa64 zmm11,ZMMWORD PTR [rax+0x100]
vmovdqu64 YMMWORD PTR [rsp+0x540],ymm31
mov    QWORD PTR [rsp+0x560],rdi
vpxor  xmm2,xmm2,xmm2
vpxord xmm19,xmm19,xmm19
vpxord xmm18,xmm18,xmm18
vpxord xmm17,xmm17,xmm17
vpxord xmm20,xmm20,xmm20
vpxord xmm21,xmm21,xmm21
vpxord xmm16,xmm16,xmm16
mov    edx,esi
add    rax,0x140
xor    esi,esi
vpbroadcastq zmm9,QWORD PTR [rsp+0x548]
vpbroadcastq zmm8,QWORD PTR [rsp+0x550]
vpbroadcastq zmm7,QWORD PTR [rsp+0x558]
vpbroadcastq zmm6,QWORD PTR [rsp+0x560]
vpmadd52huq zmm19,zmm13,zmm1
vpmadd52huq zmm18,zmm12,zmm1
vpmadd52huq zmm17,zmm10,zmm1
vpmadd52luq zmm2,zmm14,zmm1
vpmadd52huq zmm20,zmm14,zmm1
vpmadd52huq zmm16,zmm11,zmm1
vpmadd52luq zmm21,zmm2,zmm30
vpmadd52luq zmm19,zmm12,zmm1
vpmadd52luq zmm18,zmm10,zmm1
vpmadd52luq zmm17,zmm11,zmm1
vpmadd52luq zmm20,zmm13,zmm1
vpaddq zmm2,zmm2,zmm28
vpsrlq zmm2,zmm2,0x34
vpmadd52huq zmm19,zmm21,zmm9
vpmadd52huq zmm18,zmm21,zmm8
vpmadd52huq zmm17,zmm21,zmm7
vpmadd52huq zmm2,zmm21,zmm27
vpmadd52luq zmm20,zmm21,zmm9
vpmadd52huq zmm16,zmm21,zmm6
vpmadd52luq zmm19,zmm21,zmm8
vpmadd52luq zmm18,zmm21,zmm7
vpmadd52luq zmm17,zmm21,zmm6
vpaddq zmm20,zmm20,zmm2
vpmadd52huq zmm16,zmm10,zmm3
vpxord xmm21,xmm21,xmm21
vpxor  xmm2,xmm2,xmm2
vpmadd52huq zmm2,zmm11,zmm3
vpmadd52luq zmm20,zmm14,zmm3
vpmadd52huq zmm19,zmm14,zmm3
vpmadd52huq zmm18,zmm13,zmm3
vpmadd52huq zmm17,zmm12,zmm3
vpmadd52luq zmm16,zmm11,zmm3
vpmadd52luq zmm21,zmm20,zmm30
vpaddq zmm20,zmm20,zmm28
vpsrlq zmm20,zmm20,0x34
vpmadd52luq zmm19,zmm13,zmm3
vpmadd52luq zmm18,zmm12,zmm3
vpmadd52luq zmm17,zmm10,zmm3
vpmadd52huq zmm16,zmm21,zmm7
vpmadd52huq zmm20,zmm21,zmm27
vpmadd52huq zmm2,zmm21,zmm6
vpmadd52huq zmm18,zmm21,zmm9
vpmadd52huq zmm17,zmm21,zmm8
vpmadd52luq zmm19,zmm21,zmm9
vpmadd52luq zmm16,zmm21,zmm6
vpmadd52huq zmm2,zmm10,zmm4
vpaddq zmm20,zmm19,zmm20
vpmadd52luq zmm18,zmm21,zmm8
vpmadd52luq zmm17,zmm21,zmm7
vpxord xmm21,xmm21,xmm21
vpxord xmm19,xmm19,xmm19
vpmadd52huq zmm19,zmm11,zmm4
vpmadd52luq zmm20,zmm14,zmm4
vpmadd52huq zmm16,zmm12,zmm4
vpmadd52luq zmm2,zmm11,zmm4
vpmadd52huq zmm18,zmm14,zmm4
vpmadd52huq zmm17,zmm13,zmm4
vpmadd52luq zmm21,zmm20,zmm30
vpmadd52luq zmm16,zmm10,zmm4
vpaddq zmm20,zmm20,zmm28
vpsrlq zmm20,zmm20,0x34
vpmadd52luq zmm18,zmm13,zmm4
vpmadd52luq zmm17,zmm12,zmm4
vpmadd52huq zmm2,zmm21,zmm7
vpmadd52huq zmm16,zmm21,zmm8
vpmadd52huq zmm20,zmm21,zmm27
vpmadd52huq zmm19,zmm21,zmm6
vpmadd52huq zmm17,zmm21,zmm9
vpmadd52luq zmm18,zmm21,zmm9
vpmadd52luq zmm2,zmm21,zmm6
vpmadd52luq zmm16,zmm21,zmm7
vpmadd52huq zmm19,zmm10,zmm5
vpaddq zmm20,zmm18,zmm20
vpmadd52luq zmm17,zmm21,zmm8
vpxord xmm21,xmm21,xmm21
vpxord xmm18,xmm18,xmm18
vpmadd52huq zmm18,zmm11,zmm5
vpmadd52huq zmm2,zmm12,zmm5
vpmadd52luq zmm20,zmm14,zmm5
vpmadd52huq zmm16,zmm13,zmm5
vpmadd52luq zmm19,zmm11,zmm5
vpmadd52huq zmm17,zmm14,zmm5
vpmadd52luq zmm21,zmm20,zmm30
vpmadd52luq zmm2,zmm10,zmm5
vpmadd52luq zmm16,zmm12,zmm5
vpaddq zmm20,zmm20,zmm28
vpsrlq zmm20,zmm20,0x34
vpmadd52luq zmm17,zmm13,zmm5
vpmadd52huq zmm2,zmm21,zmm8
vpmadd52huq zmm19,zmm21,zmm7
vpmadd52huq zmm16,zmm21,zmm9
vpmadd52huq zmm20,zmm21,zmm27
vpmadd52huq zmm18,zmm21,zmm6
vpmadd52luq zmm17,zmm21,zmm9
vpmadd52luq zmm2,zmm21,zmm7
vpmadd52luq zmm19,zmm21,zmm6
vpmadd52luq zmm16,zmm21,zmm8
vpmadd52huq zmm18,zmm10,zmm0
vpaddq zmm17,zmm17,zmm20
vpmadd52huq zmm2,zmm13,zmm0
vpmadd52huq zmm19,zmm12,zmm0
vpmadd52luq zmm17,zmm14,zmm0
vpmadd52huq zmm16,zmm14,zmm0
vpmadd52luq zmm18,zmm11,zmm0
vpmadd52luq zmm2,zmm12,zmm0
vpmadd52luq zmm19,zmm10,zmm0
vpxor  xmm10,xmm10,xmm10
vpmadd52luq zmm10,zmm17,zmm30
vpmadd52luq zmm16,zmm13,zmm0
vpxor  xmm12,xmm12,xmm12
vpmadd52huq zmm12,zmm11,zmm0
vpaddq zmm11,zmm17,zmm28
vpsrlq zmm11,zmm11,0x34
vpmadd52huq zmm2,zmm10,zmm9
vpmadd52huq zmm19,zmm10,zmm8
vpmadd52huq zmm11,zmm10,zmm27
vpmadd52luq zmm16,zmm10,zmm9
vpmadd52huq zmm18,zmm10,zmm7
vpmadd52huq zmm12,zmm10,zmm6
vpmadd52luq zmm2,zmm10,zmm8
vpaddq zmm11,zmm16,zmm11
vpmadd52luq zmm19,zmm10,zmm7
vpmadd52luq zmm18,zmm10,zmm6
vpsrlq zmm6,zmm11,0x34
vpaddq zmm2,zmm6,zmm2
vpandq zmm6,zmm11,zmm28
vpsrlq zmm7,zmm2,0x34
vpandq zmm2,zmm2,zmm28
vmovdqa64 ZMMWORD PTR [rcx],zmm6
vpaddq zmm7,zmm7,zmm19
vmovdqa64 ZMMWORD PTR [rcx+0x40],zmm2
vpsrlq zmm8,zmm7,0x34
vpandq zmm7,zmm7,zmm28
vpaddq zmm8,zmm8,zmm18
vmovdqa64 ZMMWORD PTR [rcx+0x80],zmm7
vpsrlq zmm9,zmm8,0x34
vpandq zmm8,zmm8,zmm28
vpaddq zmm2,zmm9,zmm12
vmovdqa64 ZMMWORD PTR [rcx+0xc0],zmm8
vmovdqa64 ZMMWORD PTR [rcx+0x100],zmm2
mov    rcx,r9
test   dl,0x1
jne    ea0c0 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x9af0>
vmovdqa64 zmm26,ZMMWORD PTR [rsp+0x600]
vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x740]
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0x640]
vmovdqa64 zmm24,ZMMWORD PTR [rsp+0x780]
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0x680]
vmovdqa64 zmm13,ZMMWORD PTR [rsp+0x7c0]
vmovdqa64 zmm23,ZMMWORD PTR [rsp+0x6c0]
vmovdqa64 zmm25,ZMMWORD PTR [rsp+0x800]
vmovdqa64 zmm21,ZMMWORD PTR [rsp+0x700]
vmovdqa64 zmm29,ZMMWORD PTR [rsp+0x840]
vpbroadcastq zmm8,QWORD PTR [rip+0xfffffffffff1f3d1]        # 98d8 <GCC_except_table830+0x1394>
vmovdqu64 YMMWORD PTR [rsp+0x600],ymm31
mov    QWORD PTR [rsp+0x620],rdi
vpxor  xmm11,xmm11,xmm11
vpxor  xmm12,xmm12,xmm12
vpbroadcastq zmm17,QWORD PTR [rip+0xfffffffffff1f34d]        # 9878 <GCC_except_table830+0x1334>
vpxord xmm16,xmm16,xmm16
vpxord xmm22,xmm22,xmm22
movabs rax,0x60c89ce5c263
mov    r12,QWORD PTR [rsp+0x400]
mov    r13,QWORD PTR [rsp+0x20]
mov    r14,QWORD PTR [rsp+0x30]
mov    r10,QWORD PTR [rsp+0x38]
mov    r9,QWORD PTR [rsp+0x28]
vpaddq zmm0,zmm7,zmm26
vpaddq zmm2,zmm24,zmm15
vpaddq zmm6,zmm29,zmm21
vmovdqa64 ZMMWORD PTR [rsp+0x180],zmm7
vmovdqa64 ZMMWORD PTR [rsp+0x1c0],zmm13
vmovdqa64 ZMMWORD PTR [rsp+0x240],zmm21
vmovdqa64 ZMMWORD PTR [rsp+0x300],zmm24
vmovdqa64 ZMMWORD PTR [rsp+0x200],zmm25
vmovdqa64 ZMMWORD PTR [rsp+0x280],zmm29
vmovdqa64 ZMMWORD PTR [rsp+0xf80],zmm15
vmovdqa64 ZMMWORD PTR [rsp+0xbc0],zmm23
vmovdqa64 ZMMWORD PTR [rsp+0x1000],zmm26
vmovdqa64 ZMMWORD PTR [rsp+0xfc0],zmm18
vpsrlq zmm1,zmm0,0x34
vpaddq zmm5,zmm1,zmm2
vpaddq zmm2,zmm13,zmm18
vpsrlq zmm1,zmm5,0x34
vpandq zmm9,zmm5,zmm28
vpaddq zmm4,zmm1,zmm2
vpaddq zmm2,zmm25,zmm23
vpsrlq zmm1,zmm4,0x34
vpaddq zmm3,zmm1,zmm2
vpsubq zmm2,zmm26,zmm7
vpsrlq zmm1,zmm3,0x34
vpaddq zmm1,zmm1,zmm6
vpaddq zmm6,zmm2,zmm8
vpbroadcastq zmm8,QWORD PTR [rip+0xfffffffffff1f35d]        # 9978 <GCC_except_table830+0x1434>
vpsubq zmm2,zmm15,zmm24
vpaddq zmm7,zmm2,zmm8
vpsraq zmm2,zmm6,0x34
vpandq zmm8,zmm0,zmm28
vpandq zmm6,zmm6,zmm28
vpbroadcastq zmm0,QWORD PTR [rsp+0x608]
vpmadd52huq zmm12,zmm8,zmm6
vpmadd52luq zmm11,zmm8,zmm6
vpaddq zmm10,zmm7,zmm2
vpxor  xmm7,xmm7,xmm7
vpmadd52luq zmm7,zmm11,zmm30
vpmadd52luq zmm12,zmm9,zmm6
vpaddq zmm5,zmm11,zmm28
vpsubq zmm11,zmm18,zmm13
vpxor  xmm13,xmm13,xmm13
vpsrlq zmm5,zmm5,0x34
vmovdqa64 ZMMWORD PTR [rsp+0xf00],zmm0
vpmadd52luq zmm12,zmm7,zmm0
vpmadd52huq zmm5,zmm7,zmm27
vpaddq zmm5,zmm12,zmm5
vpbroadcastq zmm12,QWORD PTR [rip+0xfffffffffff1f2ad]        # 9950 <GCC_except_table830+0x140c>
vpaddq zmm11,zmm11,zmm12
vpsraq zmm12,zmm10,0x34
vpaddq zmm14,zmm11,zmm12
vpxor  xmm11,xmm11,xmm11
vpmadd52huq zmm11,zmm9,zmm6
vpandq zmm12,zmm10,zmm28
vpandq zmm10,zmm4,zmm28
vpbroadcastq zmm4,QWORD PTR [rsp+0x610]
vpmadd52luq zmm5,zmm8,zmm12
vpsraq zmm2,zmm14,0x34
vpandq zmm14,zmm14,zmm28
vpmadd52luq zmm11,zmm10,zmm6
vpmadd52luq zmm13,zmm5,zmm30
vpaddq zmm5,zmm5,zmm28
vpsrlq zmm5,zmm5,0x34
vpmadd52huq zmm11,zmm7,zmm0
vpmadd52huq zmm5,zmm13,zmm27
vpmadd52luq zmm11,zmm7,zmm4
vpmadd52huq zmm11,zmm8,zmm12
vpmadd52luq zmm11,zmm9,zmm12
vpmadd52luq zmm11,zmm13,zmm0
vpaddq zmm19,zmm11,zmm5
vpandq zmm11,zmm3,zmm28
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm10,zmm6
vpsubq zmm5,zmm23,zmm25
vpaddq zmm5,zmm5,zmm17
vpmadd52luq zmm19,zmm8,zmm14
vpmadd52huq zmm22,zmm11,zmm6
vpaddq zmm17,zmm5,zmm2
vpbroadcastq zmm5,QWORD PTR [rsp+0x618]
vpbroadcastq zmm2,QWORD PTR [rip+0xfffffffffff1f07f]        # 97f0 <GCC_except_table830+0x12ac>
vpmadd52luq zmm3,zmm11,zmm6
vpmadd52luq zmm16,zmm19,zmm30
vpaddq zmm19,zmm19,zmm28
vpmadd52luq zmm22,zmm1,zmm6
vpsrlq zmm19,zmm19,0x34
vpmadd52huq zmm3,zmm7,zmm4
vpmadd52huq zmm19,zmm16,zmm27
vpmadd52huq zmm22,zmm7,zmm5
vpmadd52luq zmm3,zmm7,zmm5
vpmadd52huq zmm3,zmm9,zmm12
vpmadd52luq zmm3,zmm10,zmm12
vpmadd52huq zmm3,zmm13,zmm0
vpmadd52luq zmm3,zmm13,zmm4
vpmadd52huq zmm3,zmm8,zmm14
vpmadd52luq zmm3,zmm9,zmm14
vpmadd52luq zmm3,zmm16,zmm0
vpaddq zmm20,zmm3,zmm19
vpsubq zmm3,zmm21,zmm29
vpsraq zmm19,zmm17,0x34
vpandq zmm17,zmm17,zmm28
vpaddq zmm3,zmm3,zmm2
vpmadd52luq zmm20,zmm8,zmm17
vmovdqa64 zmm2,zmm27
vpaddq zmm21,zmm3,zmm19
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm1,zmm6
vpbroadcastq zmm6,QWORD PTR [rsp+0x620]
vpxord xmm19,xmm19,xmm19
vmovdqu64 YMMWORD PTR [rsp+0x600],ymm31
mov    QWORD PTR [rsp+0x620],rdi
vpmadd52luq zmm19,zmm20,zmm30
vpmadd52huq zmm3,zmm7,zmm6
vpmadd52luq zmm22,zmm7,zmm6
vpaddq zmm7,zmm20,zmm28
vpandq zmm20,zmm21,zmm28
vpxord xmm21,xmm21,xmm21
vpsrlq zmm7,zmm7,0x34
vpmadd52huq zmm7,zmm19,zmm27
vpmadd52huq zmm3,zmm11,zmm12
vpmadd52huq zmm22,zmm10,zmm12
vpmadd52luq zmm3,zmm1,zmm12
vpmadd52luq zmm22,zmm11,zmm12
vpmadd52huq zmm3,zmm13,zmm5
vpmadd52huq zmm22,zmm13,zmm4
vpmadd52luq zmm3,zmm13,zmm6
vpmadd52luq zmm22,zmm13,zmm5
vpmadd52huq zmm3,zmm10,zmm14
vpmadd52huq zmm22,zmm9,zmm14
vpmadd52luq zmm3,zmm11,zmm14
vpmadd52luq zmm22,zmm10,zmm14
vpmadd52huq zmm3,zmm16,zmm4
vpmadd52huq zmm22,zmm16,zmm0
vpmadd52luq zmm3,zmm16,zmm5
vpmadd52luq zmm22,zmm16,zmm4
vpmadd52huq zmm3,zmm9,zmm17
vpmadd52huq zmm22,zmm8,zmm17
vpmadd52luq zmm3,zmm10,zmm17
vpmadd52luq zmm22,zmm9,zmm17
vpmadd52huq zmm3,zmm19,zmm0
vpmadd52luq zmm22,zmm19,zmm0
vpsllq zmm0,ZMMWORD PTR [rsp+0x180],0x1
vpmadd52luq zmm3,zmm19,zmm4
vpaddq zmm7,zmm22,zmm7
vpmadd52luq zmm7,zmm8,zmm20
vpmadd52huq zmm3,zmm8,zmm20
vpxor  xmm8,xmm8,xmm8
vpmadd52huq zmm8,zmm1,zmm12
vpsrlq zmm12,zmm0,0x34
vpmadd52huq zmm8,zmm13,zmm6
vpmadd52luq zmm3,zmm9,zmm20
vpaddq zmm13,zmm24,zmm24
vpxord xmm24,xmm24,xmm24
vpaddq zmm12,zmm12,zmm13
vpxor  xmm13,xmm13,xmm13
vpandq zmm22,zmm12,zmm28
vpmadd52huq zmm8,zmm11,zmm14
vpmadd52luq zmm8,zmm1,zmm14
vpmadd52huq zmm8,zmm16,zmm5
vpmadd52luq zmm8,zmm16,zmm6
vpmadd52huq zmm8,zmm10,zmm17
vpmadd52luq zmm8,zmm11,zmm17
vpmadd52huq zmm8,zmm19,zmm4
vpmadd52luq zmm8,zmm19,zmm5
vpmadd52huq zmm8,zmm9,zmm20
vpxor  xmm9,xmm9,xmm9
vpmadd52huq zmm9,zmm1,zmm14
vpxor  xmm14,xmm14,xmm14
vpmadd52huq zmm9,zmm16,zmm6
vpmadd52luq zmm8,zmm10,zmm20
vpsllq zmm16,ZMMWORD PTR [rsp+0x1c0],0x1
vpmadd52huq zmm9,zmm11,zmm17
vpmadd52luq zmm9,zmm1,zmm17
vpmadd52huq zmm9,zmm19,zmm5
vpmadd52luq zmm9,zmm19,zmm6
vpmadd52huq zmm9,zmm10,zmm20
vpxor  xmm10,xmm10,xmm10
vpmadd52huq zmm10,zmm1,zmm17
vpxord xmm17,xmm17,xmm17
vpmadd52huq zmm10,zmm19,zmm6
vpmadd52luq zmm9,zmm11,zmm20
vpmadd52huq zmm10,zmm11,zmm20
vpxor  xmm11,xmm11,xmm11
vpmadd52huq zmm11,zmm1,zmm20
vpmadd52luq zmm10,zmm1,zmm20
vpandq zmm20,zmm0,QWORD BCST [rip+0xfffffffffff1ef2a]        # 9920 <GCC_except_table830+0x13dc>
vpbroadcastq zmm1,QWORD PTR [rsp+0x608]
vpsrlq zmm0,zmm12,0x34
vpbroadcastq zmm12,QWORD PTR [rsp+0x610]
vpmadd52huq zmm14,zmm26,zmm20
vpmadd52luq zmm13,zmm26,zmm20
vpmadd52luq zmm24,zmm13,zmm30
vpmadd52luq zmm14,zmm15,zmm20
vpaddq zmm13,zmm13,zmm28
vpsrlq zmm13,zmm13,0x34
vpmadd52luq zmm14,zmm24,zmm1
vpmadd52huq zmm13,zmm24,zmm27
vpaddq zmm13,zmm14,zmm13
vpaddq zmm14,zmm0,zmm16
vpxord xmm16,xmm16,xmm16
vpmadd52huq zmm16,zmm15,zmm20
vpaddq zmm0,zmm25,zmm25
vpmadd52luq zmm13,zmm26,zmm22
vpmadd52luq zmm16,zmm18,zmm20
vpmadd52luq zmm21,zmm13,zmm30
vpaddq zmm13,zmm13,zmm28
vpsrlq zmm13,zmm13,0x34
vpmadd52huq zmm16,zmm24,zmm1
vpmadd52huq zmm13,zmm21,zmm27
vpmadd52luq zmm16,zmm24,zmm12
vpmadd52huq zmm16,zmm26,zmm22
vpmadd52luq zmm16,zmm15,zmm22
vpmadd52luq zmm16,zmm21,zmm1
vpaddq zmm19,zmm16,zmm13
vpsrlq zmm13,zmm14,0x34
vpandq zmm16,zmm14,zmm28
vpxor  xmm14,xmm14,xmm14
vpmadd52huq zmm14,zmm18,zmm20
vpaddq zmm25,zmm13,zmm0
vpbroadcastq zmm13,QWORD PTR [rsp+0x618]
vpmadd52luq zmm19,zmm26,zmm16
vpaddq zmm0,zmm29,zmm29
vpxord xmm29,xmm29,xmm29
vpmadd52huq zmm29,zmm23,zmm20
vpmadd52luq zmm14,zmm23,zmm20
vpmadd52luq zmm17,zmm19,zmm30
vpaddq zmm19,zmm19,zmm28
vpsrlq zmm19,zmm19,0x34
vpmadd52huq zmm14,zmm24,zmm12
vpmadd52huq zmm19,zmm17,zmm27
vmovdqa64 zmm27,zmm28
vpmadd52luq zmm14,zmm24,zmm13
vpmadd52huq zmm14,zmm15,zmm22
vpmadd52luq zmm14,zmm18,zmm22
vpmadd52huq zmm14,zmm21,zmm1
vpmadd52luq zmm14,zmm21,zmm12
vpmadd52huq zmm14,zmm26,zmm16
vpmadd52luq zmm14,zmm15,zmm16
vpmadd52luq zmm14,zmm17,zmm1
vpaddq zmm28,zmm14,zmm19
vpsrlq zmm14,zmm25,0x34
vpaddq zmm19,zmm14,zmm0
vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x240]
vpxor  xmm14,xmm14,xmm14
vpmadd52luq zmm29,zmm0,zmm20
vpmadd52huq zmm14,zmm0,zmm20
vpbroadcastq zmm20,QWORD PTR [rsp+0x620]
vpmadd52huq zmm29,zmm24,zmm13
vpmadd52luq zmm29,zmm24,zmm20
vpmadd52huq zmm14,zmm24,zmm20
vpandq zmm24,zmm25,zmm27
vpxord xmm25,xmm25,xmm25
vpmadd52luq zmm28,zmm26,zmm24
vpmadd52huq zmm29,zmm18,zmm22
vpmadd52huq zmm14,zmm23,zmm22
vpmadd52luq zmm25,zmm28,zmm30
vpaddq zmm28,zmm28,zmm27
vpsrlq zmm28,zmm28,0x34
vpmadd52luq zmm29,zmm23,zmm22
vpmadd52luq zmm14,zmm0,zmm22
vpmadd52huq zmm28,zmm25,zmm2
vpmadd52huq zmm29,zmm21,zmm12
vpmadd52huq zmm14,zmm21,zmm13
vpmadd52luq zmm29,zmm21,zmm13
vpmadd52luq zmm14,zmm21,zmm20
vpmadd52huq zmm29,zmm15,zmm16
vpmadd52huq zmm14,zmm18,zmm16
vpmadd52luq zmm29,zmm18,zmm16
vpmadd52luq zmm14,zmm23,zmm16
vpmadd52huq zmm29,zmm17,zmm1
vpmadd52huq zmm14,zmm17,zmm12
vpmadd52luq zmm29,zmm17,zmm12
vpmadd52luq zmm14,zmm17,zmm13
vpmadd52huq zmm29,zmm26,zmm24
vpmadd52huq zmm14,zmm15,zmm24
vpmadd52luq zmm29,zmm15,zmm24
vpmadd52luq zmm14,zmm18,zmm24
vpmadd52luq zmm29,zmm25,zmm1
vpmadd52huq zmm14,zmm25,zmm1
vpaddq zmm28,zmm29,zmm28
vpxord xmm29,xmm29,xmm29
vpmadd52huq zmm29,zmm0,zmm22
vpmadd52luq zmm14,zmm25,zmm12
vmovdqa64 zmm22,ZMMWORD PTR [rsp+0xac0]
vpmadd52luq zmm28,zmm26,zmm19
vpmadd52huq zmm29,zmm21,zmm20
vpmadd52huq zmm14,zmm26,zmm19
vpxord xmm21,xmm21,xmm21
vpbroadcastq zmm26,QWORD PTR [rip+0xfffffffffff1ec27]        # 98a8 <GCC_except_table830+0x1364>
vpmadd52luq zmm21,zmm28,zmm30
vpmadd52huq zmm29,zmm23,zmm16
vpmadd52luq zmm14,zmm15,zmm19
vpmadd52luq zmm29,zmm0,zmm16
vpmadd52luq zmm14,zmm21,zmm1
vpmadd52huq zmm29,zmm17,zmm13
vpmadd52luq zmm29,zmm17,zmm20
vpmadd52huq zmm29,zmm18,zmm24
vpmadd52luq zmm29,zmm23,zmm24
vpmadd52huq zmm29,zmm25,zmm12
vpmadd52luq zmm29,zmm25,zmm13
vpmadd52huq zmm29,zmm15,zmm19
vmovdqa64 zmm15,ZMMWORD PTR [rsp+0xf00]
vpmadd52luq zmm29,zmm18,zmm19
vpmadd52huq zmm29,zmm21,zmm1
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm1,zmm0,zmm16
vpmadd52huq zmm1,zmm17,zmm20
vpmadd52luq zmm29,zmm21,zmm12
vpxord xmm17,xmm17,xmm17
vpmadd52huq zmm17,zmm0,zmm19
vpmadd52huq zmm1,zmm23,zmm24
vpmadd52huq zmm17,zmm21,zmm20
vpmadd52luq zmm1,zmm0,zmm24
vpmadd52huq zmm1,zmm25,zmm13
vpmadd52luq zmm1,zmm25,zmm20
vpmadd52huq zmm1,zmm18,zmm19
vpmadd52luq zmm1,zmm23,zmm19
vpmadd52huq zmm1,zmm21,zmm12
vpxor  xmm12,xmm12,xmm12
vpmadd52huq zmm12,zmm0,zmm24
vpmadd52huq zmm12,zmm25,zmm20
vpmadd52luq zmm1,zmm21,zmm13
vpmadd52huq zmm12,zmm23,zmm19
vpbroadcastq zmm23,QWORD PTR [rip+0xfffffffffff1ebfa]        # 9950 <GCC_except_table830+0x140c>
vpmadd52luq zmm12,zmm0,zmm19
vpaddq zmm0,zmm28,zmm27
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0x3c0]
vpmadd52huq zmm12,zmm21,zmm13
vpsrlq zmm13,zmm0,0x34
vpmadd52huq zmm13,zmm21,zmm2
vpmadd52luq zmm12,zmm21,zmm20
vmovdqa64 zmm20,ZMMWORD PTR [rsp+0x900]
vmovdqa64 zmm21,ZMMWORD PTR [rsp+0x880]
vpaddq zmm16,zmm14,zmm13
vpsrlq zmm13,zmm16,0x34
vpaddq zmm14,zmm13,zmm29
vpsrlq zmm13,zmm14,0x34
vpaddq zmm13,zmm13,zmm1
vpsrlq zmm1,zmm13,0x34
vpaddq zmm12,zmm1,zmm12
vpsrlq zmm1,zmm12,0x34
vpaddq zmm0,zmm1,zmm17
vpxord xmm17,xmm17,xmm17
vpmadd52luq zmm17,zmm7,zmm30
vpaddq zmm1,zmm7,zmm27
vpaddq zmm7,zmm19,ZMMWORD PTR [rsp+0xa40]
vmovdqa64 ZMMWORD PTR [rsp+0xec0],zmm0
vpsrlq zmm0,zmm1,0x34
vpmadd52huq zmm8,zmm17,zmm15
vpmadd52huq zmm9,zmm17,zmm4
vpmadd52luq zmm3,zmm17,zmm15
vpmadd52huq zmm0,zmm17,zmm2
vpmadd52huq zmm10,zmm17,zmm5
vpmadd52huq zmm11,zmm17,zmm6
vpbroadcastq zmm15,QWORD PTR [rip+0xfffffffffff1eb54]        # 9978 <GCC_except_table830+0x1434>
vmovaps ymm2,YMMWORD PTR [rip+0xfffffffffff1f1b4]        # 9fe0 <GCC_except_table830+0x1a9c>
vpmadd52luq zmm8,zmm17,zmm4
vpaddq zmm0,zmm3,zmm0
vpmadd52luq zmm9,zmm17,zmm5
vpmadd52luq zmm10,zmm17,zmm6
vmovdqa64 zmm17,ZMMWORD PTR [rsp+0x940]
vpsrlq zmm3,zmm0,0x34
vpaddq zmm1,zmm17,ZMMWORD PTR [rsp+0xa80]
vpandq zmm0,zmm0,zmm27
vpsubq zmm0,zmm0,zmm7
vpaddq zmm3,zmm3,zmm8
vpaddq zmm8,zmm20,ZMMWORD PTR [rsp+0xb00]
vmovups YMMWORD PTR [rsp+0x600],ymm2
mov    QWORD PTR [rsp+0x620],rax
vpsrlq zmm4,zmm3,0x34
vpandq zmm3,zmm3,zmm27
vpaddq zmm4,zmm4,zmm9
vpbroadcastq zmm9,QWORD PTR [rip+0xfffffffffff1ea35]        # 98d8 <GCC_except_table830+0x1394>
vpsrlq zmm5,zmm4,0x34
vpandq zmm4,zmm4,zmm27
vpaddq zmm5,zmm5,zmm10
vpbroadcastq zmm10,QWORD PTR [rip+0xfffffffffff1e928]        # 97e8 <GCC_except_table830+0x12a4>
vpsrlq zmm6,zmm5,0x34
vpandq zmm5,zmm5,zmm27
vpaddq zmm6,zmm6,zmm11
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff1e9e3]        # 98c0 <GCC_except_table830+0x137c>
vpaddq zmm0,zmm0,zmm9
vpsraq zmm7,zmm0,0x34
vpandq zmm25,zmm0,zmm27
vpaddq zmm3,zmm7,zmm3
vpsubq zmm3,zmm3,zmm1
vpaddq zmm1,zmm21,ZMMWORD PTR [rsp+0x5c0]
vpaddq zmm3,zmm3,zmm15
vpsraq zmm7,zmm3,0x34
vpandq zmm28,zmm3,zmm27
vpsubq zmm0,zmm28,QWORD BCST [rsp+0x608]
vpaddq zmm3,zmm25,zmm26
vpaddq zmm4,zmm7,zmm4
vpsubq zmm4,zmm4,zmm8
vpaddq zmm8,zmm22,ZMMWORD PTR [rsp+0x480]
vpaddq zmm4,zmm4,zmm23
vpsraq zmm7,zmm4,0x34
vpandq zmm29,zmm4,zmm27
vpsubq zmm4,zmm29,QWORD BCST [rsp+0x610]
vpaddq zmm5,zmm7,zmm5
vpsubq zmm5,zmm5,zmm1
vpbroadcastq zmm1,QWORD PTR [rip+0xfffffffffff1e908]        # 9878 <GCC_except_table830+0x1334>
vpaddq zmm5,zmm5,zmm1
vpsraq zmm7,zmm5,0x34
vpandq zmm30,zmm5,zmm27
vpsubq zmm5,zmm30,QWORD BCST [rsp+0x618]
vpaddq zmm6,zmm7,zmm6
vpsraq zmm7,zmm3,0x34
vpsubq zmm6,zmm6,zmm8
vpbroadcastq zmm8,QWORD PTR [rip+0xfffffffffff1e845]        # 97f0 <GCC_except_table830+0x12ac>
vpaddq zmm0,zmm0,zmm7
vpsraq zmm7,zmm0,0x34
vpaddq zmm4,zmm7,zmm4
vpsraq zmm7,zmm4,0x34
vpaddq zmm5,zmm7,zmm5
vpsraq zmm7,zmm5,0x34
vpaddq zmm6,zmm6,zmm8
vpandq zmm18,zmm6,zmm27
vpsubq zmm6,zmm18,QWORD BCST [rsp+0x620]
vmovdqu64 YMMWORD PTR [rsp+0x600],ymm31
mov    QWORD PTR [rsp+0x620],rdi
vpaddq zmm6,zmm7,zmm6
vpcmpltuq k1,zmm6,zmm10
vpandq zmm28{k1},zmm0,zmm27
vpsubq zmm0,zmm28,QWORD BCST [rsp+0x608]
vpandq zmm18{k1},zmm6,zmm27
vpandq zmm29{k1},zmm4,zmm27
vpsubq zmm6,zmm29,QWORD BCST [rsp+0x610]
vpandq zmm30{k1},zmm5,zmm27
vpsubq zmm7,zmm30,QWORD BCST [rsp+0x618]
vpandq zmm25{k1},zmm3,zmm27
vpsubq zmm24,zmm18,QWORD BCST [rsp+0x620]
vmovups YMMWORD PTR [rsp+0x600],ymm2
mov    QWORD PTR [rsp+0x620],rax
vpaddq zmm3,zmm25,zmm11
vpsraq zmm4,zmm3,0x34
vpaddq zmm0,zmm0,zmm4
vpsraq zmm5,zmm0,0x34
vpaddq zmm4,zmm5,zmm6
vpsraq zmm6,zmm4,0x34
vpaddq zmm5,zmm6,zmm7
vpsraq zmm7,zmm5,0x34
vpaddq zmm6,zmm7,zmm24
vmovdqa64 zmm24,ZMMWORD PTR [rsp+0xf40]
vpcmpltuq k1,zmm6,zmm10
vpandq zmm29{k1},zmm4,zmm27
vpandq zmm4,zmm16,zmm27
vmovdqa64 zmm16,ZMMWORD PTR [rsp+0x440]
vpandq zmm25{k1},zmm3,zmm27
vpandq zmm28{k1},zmm0,zmm27
vpandq zmm30{k1},zmm5,zmm27
vmovdqa64 zmm18{k1},zmm6
vmovdqa64 ZMMWORD PTR [rsp+0x880],zmm18
vpaddq zmm3,zmm16,ZMMWORD PTR [rsp+0x1100]
vmovdqa64 ZMMWORD PTR [rsp+0x3c0],zmm28
vmovdqa64 ZMMWORD PTR [rsp+0x900],zmm30
vmovdqa64 ZMMWORD PTR [rsp+0xa40],zmm29
vmovdqa64 ZMMWORD PTR [rsp+0xa80],zmm25
vpsubq zmm0,zmm4,zmm3
vpandq zmm3,zmm14,zmm27
vmovdqa64 zmm14,ZMMWORD PTR [rsp+0x380]
vpaddq zmm5,zmm14,ZMMWORD PTR [rsp+0xc00]
vpaddq zmm0,zmm0,zmm9
vpaddq zmm9,zmm24,ZMMWORD PTR [rsp+0x1080]
vpsraq zmm4,zmm0,0x34
vpaddq zmm3,zmm4,zmm3
vpandq zmm4,zmm13,zmm27
vmovdqa64 zmm13,ZMMWORD PTR [rsp+0xa00]
vpaddq zmm6,zmm13,ZMMWORD PTR [rsp+0x10c0]
vpsubq zmm3,zmm3,zmm5
vpaddq zmm3,zmm3,zmm15
vpandq zmm15,zmm0,zmm27
vpsraq zmm5,zmm3,0x34
vpaddq zmm4,zmm5,zmm4
vpandq zmm5,zmm12,zmm27
vpbroadcastq zmm12,QWORD PTR [rip+0xfffffffffff1e716]        # 9898 <GCC_except_table830+0x1354>
vpsubq zmm4,zmm4,zmm6
vpaddq zmm4,zmm4,zmm23
vpsraq zmm6,zmm4,0x34
vpandq zmm23,zmm4,zmm27
vpsubq zmm4,zmm23,QWORD BCST [rsp+0x610]
vpaddq zmm5,zmm6,zmm5
vpsubq zmm5,zmm5,zmm9
vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x1040]
vpaddq zmm5,zmm5,zmm1
vpaddq zmm7,zmm9,ZMMWORD PTR [rsp+0x11c0]
vpsraq zmm6,zmm5,0x34
vpaddq zmm1,zmm6,ZMMWORD PTR [rsp+0xec0]
vpsubq zmm1,zmm1,zmm7
vpandq zmm7,zmm3,zmm27
vpsubq zmm0,zmm7,QWORD BCST [rsp+0x608]
vpaddq zmm3,zmm15,zmm26
vpandq zmm26,zmm5,zmm27
vpsubq zmm5,zmm26,QWORD BCST [rsp+0x618]
vpaddq zmm1,zmm1,zmm8
vpsraq zmm6,zmm3,0x34
vpandq zmm2,zmm1,zmm27
vpsubq zmm1,zmm2,QWORD BCST [rsp+0x620]
vmovdqu64 YMMWORD PTR [rsp+0x600],ymm31
mov    QWORD PTR [rsp+0x620],rdi
vpaddq zmm0,zmm0,zmm6
vpsraq zmm6,zmm0,0x34
vpaddq zmm4,zmm6,zmm4
vpsraq zmm6,zmm4,0x34
vpaddq zmm5,zmm6,zmm5
vpsraq zmm6,zmm5,0x34
vpaddq zmm1,zmm6,zmm1
vpcmpltuq k1,zmm1,zmm10
vpandq zmm7{k1},zmm0,zmm27
vpsubq zmm0,zmm7,QWORD BCST [rsp+0x608]
vpandq zmm26{k1},zmm5,zmm27
vpandq zmm23{k1},zmm4,zmm27
vpsubq zmm5,zmm23,QWORD BCST [rsp+0x610]
vpsubq zmm6,zmm26,QWORD BCST [rsp+0x618]
vpandq zmm15{k1},zmm3,zmm27
vpandq zmm2{k1},zmm1,zmm27
vpsubq zmm8,zmm2,QWORD BCST [rsp+0x620]
vmovdqu64 YMMWORD PTR [rsp+0x600],ymm31
mov    QWORD PTR [rsp+0x620],rdi
vpxord xmm31,xmm31,xmm31
vpaddq zmm1,zmm15,zmm11
vpbroadcastq zmm11,QWORD PTR [rip+0xfffffffffff1e551]        # 9828 <GCC_except_table830+0x12e4>
vpsraq zmm3,zmm1,0x34
vpaddq zmm0,zmm0,zmm3
vpsraq zmm4,zmm0,0x34
vpaddq zmm3,zmm4,zmm5
vpsraq zmm5,zmm3,0x34
vpaddq zmm4,zmm5,zmm6
vpsraq zmm6,zmm4,0x34
vpaddq zmm5,zmm6,zmm8
vpcmpltuq k1,zmm5,zmm10
vpbroadcastq zmm10,QWORD PTR [rip+0xfffffffffff1e4a4]        # 97c0 <GCC_except_table830+0x127c>
vpandq zmm15{k1},zmm1,zmm27
vpsubq zmm1,zmm19,zmm25
vpbroadcastq zmm19,QWORD PTR [rip+0xfffffffffff1e46e]        # 97a0 <GCC_except_table830+0x125c>
vmovdqa64 zmm2{k1},zmm5
vpbroadcastq zmm5,QWORD PTR [rip+0xfffffffffff1e64e]        # 9990 <GCC_except_table830+0x144c>
vpandq zmm7{k1},zmm0,zmm27
vpandq zmm23{k1},zmm3,zmm27
vpandq zmm26{k1},zmm4,zmm27
vpaddq zmm4,zmm22,zmm12
vmovdqa64 ZMMWORD PTR [rsp+0x440],zmm2
vpxord xmm25,xmm25,xmm25
vmovdqa64 ZMMWORD PTR [rsp+0xb00],zmm7
vmovdqa64 ZMMWORD PTR [rsp+0x5c0],zmm23
vmovdqa64 ZMMWORD PTR [rsp+0x480],zmm26
vmovdqa64 ZMMWORD PTR [rsp+0xac0],zmm15
vpaddq zmm1,zmm1,zmm19
vpaddq zmm3,zmm17,zmm5
vpaddq zmm5,zmm14,zmm5
vpsraq zmm0,zmm1,0x34
vpsubq zmm0,zmm0,zmm28
vpaddq zmm6,zmm0,zmm3
vpaddq zmm3,zmm20,zmm10
vpbroadcastq zmm20,QWORD PTR [rsp+0x608]
vpsraq zmm0,zmm6,0x34
vpandq zmm6,zmm6,zmm27
vpsubq zmm0,zmm0,zmm29
vpaddq zmm8,zmm0,zmm3
vpaddq zmm3,zmm21,zmm11
vpsraq zmm0,zmm8,0x34
vpsubq zmm0,zmm0,zmm30
vpxord xmm30,xmm30,xmm30
vpaddq zmm17,zmm0,zmm3
vpsraq zmm3,zmm17,0x34
vpsubq zmm3,zmm3,zmm18
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0x1000]
vpaddq zmm0,zmm3,zmm4
vmovdqa64 ZMMWORD PTR [rsp+0x940],zmm0
vpsubq zmm0,zmm16,zmm15
vpxor  xmm15,xmm15,xmm15
vpaddq zmm3,zmm0,zmm19
vmovdqa64 zmm0,zmm19
vpsraq zmm4,zmm3,0x34
vpsubq zmm4,zmm4,zmm7
vpbroadcastq zmm7,QWORD PTR [rip+0xfffffffffff1e491]        # 98d8 <GCC_except_table830+0x1394>
vpaddq zmm16,zmm4,zmm5
vpaddq zmm5,zmm13,zmm10
vpbroadcastq zmm13,QWORD PTR [rsp+0x618]
vpsraq zmm4,zmm16,0x34
vpandq zmm16,zmm16,zmm27
vpsubq zmm4,zmm4,zmm23
vmovdqa64 zmm23,ZMMWORD PTR [rsp+0xfc0]
vpaddq zmm28,zmm4,zmm5
vpaddq zmm5,zmm24,zmm11
vpbroadcastq zmm24,QWORD PTR [rsp+0x610]
vpsraq zmm4,zmm28,0x34
vpandq zmm28,zmm28,zmm27
vpsubq zmm4,zmm4,zmm26
vmovdqa64 zmm26,ZMMWORD PTR [rsp+0xbc0]
vpaddq zmm19,zmm4,zmm5
vpaddq zmm5,zmm9,zmm12
vmovdqa64 zmm9,ZMMWORD PTR [rsp+0xf80]
vpxor  xmm12,xmm12,xmm12
vpsraq zmm4,zmm19,0x34
vpandq zmm19,zmm19,zmm27
vpsubq zmm4,zmm4,zmm2
vmovdqa64 zmm2,ZMMWORD PTR [rsp+0x240]
vpaddq zmm21,zmm4,zmm5
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x180]
vpandq zmm21,zmm21,zmm27
vpsubq zmm4,zmm7,zmm4
vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x1c0]
vpsraq zmm5,zmm4,0x34
vpandq zmm10,zmm4,zmm27
vpsubq zmm4,zmm5,ZMMWORD PTR [rsp+0x300]
vpandq zmm5,zmm3,zmm27
vpxor  xmm3,xmm3,xmm3
vpaddq zmm11,zmm4,QWORD BCST [rip+0xfffffffffff1e44e]        # 9978 <GCC_except_table830+0x1434>
vpandq zmm4,zmm1,zmm27
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm3,zmm18,zmm4
vpmadd52luq zmm1,zmm18,zmm4
vpmadd52huq zmm15,zmm23,zmm4
vpmadd52huq zmm25,zmm9,zmm4
vpmadd52huq zmm12,zmm26,zmm4
vpmadd52huq zmm31,zmm2,zmm4
vpmadd52luq zmm3,zmm9,zmm4
vpmadd52luq zmm1,zmm10,zmm5
vpmadd52luq zmm30,zmm1,QWORD BCST [rip+0xfffffffffff1e3da]        # 9948 <GCC_except_table830+0x1404>
vpmadd52luq zmm15,zmm26,zmm4
vpmadd52luq zmm25,zmm23,zmm4
vpmadd52luq zmm12,zmm2,zmm4
vpandq zmm29,zmm11,zmm27
vpmadd52huq zmm3,zmm10,zmm5
vpaddq zmm1,zmm1,zmm27
vpmadd52huq zmm25,zmm29,zmm5
vpsrlq zmm1,zmm1,0x34
vpmadd52luq zmm3,zmm29,zmm5
vpmadd52huq zmm1,zmm30,zmm0
vpmadd52luq zmm3,zmm30,zmm20
vpaddq zmm14,zmm3,zmm1
vpsraq zmm1,zmm11,0x34
vpbroadcastq zmm11,QWORD PTR [rsp+0x620]
vpsubq zmm1,zmm1,zmm7
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1e377]        # 9950 <GCC_except_table830+0x140c>
vpmadd52luq zmm14,zmm18,zmm6
vpandq zmm7,zmm8,zmm27
vmovdqa64 zmm8,zmm0
vpmadd52luq zmm14,zmm10,zmm16
vpandq zmm22,zmm1,zmm27
vpsraq zmm3,zmm1,0x34
vpsubq zmm1,zmm3,ZMMWORD PTR [rsp+0x200]
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1e268]        # 9878 <GCC_except_table830+0x1334>
vpmadd52huq zmm15,zmm22,zmm5
vpmadd52luq zmm25,zmm22,zmm5
vpmadd52huq zmm25,zmm30,zmm20
vpsraq zmm3,zmm1,0x34
vpsubq zmm3,zmm3,ZMMWORD PTR [rsp+0x280]
vpandq zmm1,zmm1,zmm27
vpmadd52luq zmm25,zmm30,zmm24
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff1e1a9]        # 97f0 <GCC_except_table830+0x12ac>
vpmadd52luq zmm15,zmm1,zmm5
vpmadd52huq zmm12,zmm1,zmm5
vpmadd52huq zmm25,zmm18,zmm6
vpmadd52huq zmm15,zmm30,zmm24
vpmadd52luq zmm25,zmm9,zmm6
vpandq zmm3,zmm3,zmm27
vpmadd52luq zmm15,zmm30,zmm13
vpmadd52luq zmm12,zmm3,zmm5
vpmadd52huq zmm31,zmm3,zmm5
vpmadd52huq zmm25,zmm10,zmm16
vpmadd52huq zmm15,zmm9,zmm6
vpmadd52huq zmm12,zmm30,zmm13
vpmadd52huq zmm31,zmm30,zmm11
vpmadd52luq zmm25,zmm29,zmm16
vpmadd52luq zmm15,zmm23,zmm6
vpmadd52luq zmm12,zmm30,zmm11
vpxord xmm30,xmm30,xmm30
vpmadd52luq zmm30,zmm14,QWORD BCST [rip+0xfffffffffff1e291]        # 9948 <GCC_except_table830+0x1404>
vpmadd52huq zmm31,zmm26,zmm6
vpaddq zmm14,zmm14,zmm27
vpsrlq zmm14,zmm14,0x34
vpmadd52huq zmm15,zmm29,zmm16
vpmadd52huq zmm12,zmm23,zmm6
vpmadd52luq zmm31,zmm2,zmm6
vpmadd52luq zmm15,zmm22,zmm16
vpmadd52luq zmm12,zmm26,zmm6
vpmadd52huq zmm31,zmm1,zmm16
vpmadd52huq zmm14,zmm30,zmm0
vpmadd52luq zmm25,zmm30,zmm20
vpmadd52huq zmm15,zmm30,zmm20
vpmadd52huq zmm12,zmm22,zmm16
vpmadd52luq zmm31,zmm3,zmm16
vpaddq zmm14,zmm25,zmm14
vpxord xmm25,xmm25,xmm25
vpmadd52huq zmm25,zmm2,zmm6
vpmadd52luq zmm15,zmm30,zmm24
vpmadd52luq zmm14,zmm18,zmm7
vpmadd52luq zmm12,zmm1,zmm16
vpmadd52huq zmm31,zmm30,zmm13
vpmadd52huq zmm25,zmm3,zmm16
vpmadd52huq zmm15,zmm18,zmm7
vpmadd52luq zmm14,zmm10,zmm28
vpmadd52huq zmm12,zmm30,zmm24
vpmadd52luq zmm31,zmm30,zmm11
vpmadd52huq zmm25,zmm30,zmm11
vpmadd52luq zmm15,zmm9,zmm7
vpmadd52luq zmm12,zmm30,zmm13
vpxord xmm30,xmm30,xmm30
vpmadd52luq zmm30,zmm14,QWORD BCST [rip+0xfffffffffff1e1d2]        # 9948 <GCC_except_table830+0x1404>
vpaddq zmm14,zmm14,zmm27
vpmadd52huq zmm31,zmm23,zmm7
vpsrlq zmm14,zmm14,0x34
vpmadd52huq zmm25,zmm26,zmm7
vpmadd52huq zmm15,zmm10,zmm28
vpmadd52huq zmm12,zmm9,zmm7
vpmadd52luq zmm31,zmm26,zmm7
vpmadd52luq zmm25,zmm2,zmm7
vpmadd52luq zmm15,zmm29,zmm28
vpmadd52luq zmm12,zmm23,zmm7
vpmadd52huq zmm31,zmm22,zmm28
vpmadd52huq zmm14,zmm30,zmm0
vpmadd52huq zmm25,zmm1,zmm28
vpandq zmm0,zmm17,zmm27
vpandq zmm17,zmm27,ZMMWORD PTR [rsp+0x940]
vpmadd52luq zmm15,zmm30,zmm20
vpmadd52huq zmm12,zmm29,zmm28
vpmadd52luq zmm31,zmm1,zmm28
vpmadd52luq zmm25,zmm3,zmm28
vpaddq zmm14,zmm15,zmm14
vpxor  xmm15,xmm15,xmm15
vpmadd52huq zmm15,zmm2,zmm7
vpmadd52luq zmm12,zmm22,zmm28
vpmadd52huq zmm31,zmm30,zmm24
vpmadd52luq zmm14,zmm18,zmm0
vpmadd52huq zmm25,zmm30,zmm13
vpmadd52huq zmm15,zmm3,zmm28
vpmadd52huq zmm12,zmm30,zmm20
vpmadd52luq zmm31,zmm30,zmm13
vpmadd52luq zmm14,zmm10,zmm19
vpmadd52luq zmm25,zmm30,zmm11
vpmadd52huq zmm15,zmm30,zmm11
vpmadd52luq zmm12,zmm30,zmm24
vpxord xmm30,xmm30,xmm30
vpmadd52luq zmm30,zmm14,QWORD BCST [rip+0xfffffffffff1e0fa]        # 9948 <GCC_except_table830+0x1404>
vpmadd52huq zmm31,zmm9,zmm0
vpaddq zmm14,zmm14,zmm27
vpsrlq zmm14,zmm14,0x34
vpmadd52huq zmm25,zmm23,zmm0
vpmadd52huq zmm15,zmm26,zmm0
vpmadd52huq zmm12,zmm18,zmm0
vpmadd52luq zmm31,zmm23,zmm0
vpmadd52luq zmm25,zmm26,zmm0
vpmadd52luq zmm15,zmm2,zmm0
vpmadd52luq zmm12,zmm9,zmm0
vpmadd52huq zmm31,zmm29,zmm19
vpmadd52huq zmm14,zmm30,zmm8
vpmadd52huq zmm25,zmm22,zmm19
vpbroadcastq zmm8,QWORD PTR [rip+0xfffffffffff1e0a1]        # 9948 <GCC_except_table830+0x1404>
vpmadd52huq zmm15,zmm1,zmm19
vpmadd52huq zmm12,zmm10,zmm19
vpmadd52luq zmm31,zmm22,zmm19
vpmadd52luq zmm25,zmm1,zmm19
vpmadd52luq zmm15,zmm3,zmm19
vpmadd52luq zmm12,zmm29,zmm19
vpmadd52huq zmm31,zmm30,zmm20
vpmadd52huq zmm25,zmm30,zmm24
vpmadd52huq zmm15,zmm30,zmm13
vpmadd52luq zmm12,zmm30,zmm20
vpmadd52luq zmm31,zmm30,zmm24
vpmadd52luq zmm25,zmm30,zmm13
vpmadd52luq zmm15,zmm30,zmm11
vpaddq zmm14,zmm12,zmm14
vpxor  xmm12,xmm12,xmm12
vpmadd52huq zmm12,zmm2,zmm0
vpmadd52huq zmm31,zmm18,zmm17
vpmadd52luq zmm14,zmm18,zmm17
vpmadd52huq zmm25,zmm9,zmm17
vpmadd52huq zmm15,zmm23,zmm17
vpmadd52huq zmm12,zmm3,zmm19
vpmadd52luq zmm31,zmm9,zmm17
vpmadd52luq zmm14,zmm10,zmm21
vpmadd52luq zmm25,zmm23,zmm17
vpmadd52luq zmm15,zmm26,zmm17
vpmadd52huq zmm12,zmm30,zmm11
vpmadd52huq zmm31,zmm10,zmm21
vmovdqa64 zmm30,zmm9
vpaddq zmm10,zmm14,zmm27
vpmadd52huq zmm25,zmm29,zmm21
vpsrlq zmm10,zmm10,0x34
vpmadd52huq zmm15,zmm22,zmm21
vpmadd52huq zmm12,zmm26,zmm17
vpmadd52luq zmm31,zmm29,zmm21
vpmadd52luq zmm25,zmm22,zmm21
vpmadd52luq zmm15,zmm1,zmm21
vpmadd52luq zmm12,zmm2,zmm17
vpmadd52huq zmm12,zmm1,zmm21
vpxor  xmm1,xmm1,xmm1
vpmadd52huq zmm1,zmm2,zmm17
vpmadd52luq zmm12,zmm3,zmm21
vpmadd52huq zmm1,zmm3,zmm21
vpxor  xmm3,xmm3,xmm3
vpmadd52luq zmm3,zmm14,zmm8
vpmadd52huq zmm10,zmm3,QWORD BCST [rip+0xfffffffffff1ddeb]        # 97a0 <GCC_except_table830+0x125c>
vpmadd52huq zmm25,zmm3,zmm20
vpmadd52huq zmm15,zmm3,zmm24
vpmadd52huq zmm12,zmm3,zmm13
vpmadd52luq zmm31,zmm3,zmm20
vpmadd52huq zmm1,zmm3,zmm11
vpmadd52luq zmm25,zmm3,zmm24
vpmadd52luq zmm15,zmm3,zmm13
vpmadd52luq zmm12,zmm3,zmm11
vpaddq zmm3,zmm31,zmm10
vpsrlq zmm10,zmm3,0x34
vpandq zmm3,zmm3,zmm27
vpsubq zmm3,zmm3,ZMMWORD PTR [rsp+0x2c0]
vpaddq zmm14,zmm3,QWORD BCST [rip+0xfffffffffff1dece]        # 98d8 <GCC_except_table830+0x1394>
vpaddq zmm10,zmm10,zmm25
vpsrlq zmm11,zmm10,0x34
vpandq zmm10,zmm10,zmm27
vpsubq zmm10,zmm10,ZMMWORD PTR [rsp+0x4c0]
vpaddq zmm11,zmm11,zmm15
vpandq zmm8,zmm11,zmm27
vpsrlq zmm13,zmm11,0x34
vpaddq zmm12,zmm13,zmm12
vpsrlq zmm13,zmm12,0x34
vpaddq zmm1,zmm13,zmm1
vpsubq zmm1,zmm1,ZMMWORD PTR [rsp+0x340]
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1dd93]        # 97f0 <GCC_except_table830+0x12ac>
vpsraq zmm3,zmm14,0x34
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff1df0a]        # 9978 <GCC_except_table830+0x1434>
vpaddq zmm3,zmm10,zmm3
vpsubq zmm10,zmm8,ZMMWORD PTR [rsp+0x980]
vpsraq zmm11,zmm3,0x34
vpandq zmm13,zmm3,zmm27
vmovdqa ymm3,YMMWORD PTR [rip+0xfffffffffff1e54f]        # 9fe0 <GCC_except_table830+0x1a9c>
vpaddq zmm10,zmm10,zmm11
vpandq zmm11,zmm12,zmm27
vpsubq zmm11,zmm11,ZMMWORD PTR [rsp+0x9c0]
vpaddq zmm10,zmm10,QWORD BCST [rip+0xfffffffffff1dea1]        # 9950 <GCC_except_table830+0x140c>
vpaddq zmm11,zmm11,QWORD BCST [rip+0xfffffffffff1ddbf]        # 9878 <GCC_except_table830+0x1334>
vmovdqu YMMWORD PTR [rsp+0x600],ymm3
mov    QWORD PTR [rsp+0x620],rax
vpsubq zmm24,zmm13,QWORD BCST [rsp+0x608]
vpsraq zmm12,zmm10,0x34
vpaddq zmm11,zmm11,zmm12
vpsraq zmm12,zmm11,0x34
vpandq zmm9,zmm11,zmm27
vpsubq zmm15,zmm9,QWORD BCST [rsp+0x618]
vpaddq zmm8,zmm1,zmm12
vpandq zmm12,zmm10,zmm27
vmovaps ymm10,YMMWORD PTR [rip+0xfffffffffff1e352]        # 9e60 <GCC_except_table830+0x191c>
vpsubq zmm22,zmm12,QWORD BCST [rsp+0x610]
vpandq zmm1,zmm8,zmm27
vpsubq zmm11,zmm1,QWORD BCST [rsp+0x620]
vmovups YMMWORD PTR [rsp+0x600],ymm10
mov    QWORD PTR [rsp+0x620],rdi
vpbroadcastq zmm1,QWORD PTR [rsp+0x608]
vpbroadcastq zmm10,QWORD PTR [rsp+0x610]
vpbroadcastq zmm20,QWORD PTR [rsp+0x618]
vpbroadcastq zmm29,QWORD PTR [rsp+0x620]
vmovdqu YMMWORD PTR [rsp+0x600],ymm3
mov    QWORD PTR [rsp+0x620],rax
test   r12,r12
je     ec845 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc275>
mov    eax,DWORD PTR [r8]
vmovdqa64 ZMMWORD PTR [rsp+0xa00],zmm24
vmovdqa64 ZMMWORD PTR [rsp+0x1100],zmm22
vmovdqa64 ZMMWORD PTR [rsp+0x380],zmm15
vmovdqa64 ZMMWORD PTR [rsp+0x340],zmm11
vmovdqa64 ZMMWORD PTR [rsp+0x940],zmm14
vmovdqa64 ZMMWORD PTR [rsp+0x9c0],zmm13
vmovdqa64 ZMMWORD PTR [rsp+0x980],zmm12
vmovdqa64 ZMMWORD PTR [rsp+0x4c0],zmm9
vmovdqa64 ZMMWORD PTR [rsp+0x2c0],zmm8
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec830 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc260>
vpxor  xmm15,xmm15,xmm15
vpmadd52huq zmm15,zmm18,zmm5
vpxor  xmm13,xmm13,xmm13
vpmadd52huq zmm13,zmm30,zmm5
vpxor  xmm12,xmm12,xmm12
vpmadd52huq zmm12,zmm23,zmm5
vpxor  xmm11,xmm11,xmm11
vpmadd52huq zmm11,zmm26,zmm5
vmovdqa64 zmm27,ZMMWORD PTR [rsp+0x180]
vmovdqa64 zmm25,ZMMWORD PTR [rsp+0x300]
vmovdqa64 zmm31,ZMMWORD PTR [rsp+0x1c0]
vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x200]
vpxor  xmm14,xmm14,xmm14
vpmadd52luq zmm14,zmm18,zmm5
vpxor  xmm3,xmm3,xmm3
vpmadd52huq zmm3,zmm2,zmm5
vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x280]
vpbroadcastq zmm22,QWORD PTR [rip+0xfffffffffff1dafb]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm24,QWORD PTR [rip+0xfffffffffff1db49]        # 97a0 <GCC_except_table830+0x125c>
shl    rax,0x7
vpmadd52luq zmm15,zmm30,zmm5
vpmadd52luq zmm13,zmm23,zmm5
vpmadd52luq zmm12,zmm26,zmm5
vpmadd52luq zmm11,zmm2,zmm5
vpbroadcastq zmm5,QWORD PTR [rip+0xfffffffffff1dccb]        # 9948 <GCC_except_table830+0x1404>
vpmadd52huq zmm15,zmm27,zmm4
vpmadd52huq zmm13,zmm25,zmm4
vpmadd52huq zmm12,zmm31,zmm4
vpmadd52huq zmm11,zmm8,zmm4
vpmadd52luq zmm14,zmm27,zmm4
vpmadd52huq zmm3,zmm9,zmm4
vpmadd52luq zmm15,zmm25,zmm4
vpmadd52luq zmm13,zmm31,zmm4
vpmadd52luq zmm12,zmm8,zmm4
vpmadd52luq zmm11,zmm9,zmm4
vpxor  xmm4,xmm4,xmm4
vpmadd52luq zmm4,zmm14,zmm5
vpaddq zmm5,zmm14,zmm22
vpbroadcastq zmm14,QWORD PTR [rip+0xfffffffffff1dc75]        # 9948 <GCC_except_table830+0x1404>
vpsrlq zmm5,zmm5,0x34
vpmadd52huq zmm13,zmm4,zmm1
vpmadd52huq zmm12,zmm4,zmm10
vpmadd52huq zmm11,zmm4,zmm20
vpmadd52huq zmm3,zmm4,zmm29
vpmadd52huq zmm5,zmm4,zmm24
vpmadd52luq zmm15,zmm4,zmm1
vpmadd52luq zmm13,zmm4,zmm10
vpmadd52luq zmm12,zmm4,zmm20
vpmadd52luq zmm11,zmm4,zmm29
vpmadd52huq zmm3,zmm26,zmm16
vpaddq zmm5,zmm15,zmm5
vpxor  xmm4,xmm4,xmm4
vpmadd52huq zmm4,zmm2,zmm16
vpmadd52luq zmm5,zmm18,zmm16
vpmadd52huq zmm13,zmm18,zmm16
vpmadd52huq zmm12,zmm30,zmm16
vpmadd52huq zmm11,zmm23,zmm16
vpmadd52luq zmm3,zmm2,zmm16
vpmadd52huq zmm4,zmm9,zmm6
vpmadd52luq zmm5,zmm27,zmm6
vpmadd52luq zmm13,zmm30,zmm16
vpmadd52luq zmm12,zmm23,zmm16
vpmadd52luq zmm11,zmm26,zmm16
vpmadd52huq zmm3,zmm8,zmm6
vpbroadcastq zmm16,QWORD PTR [rip+0xfffffffffff1db36]        # 98a8 <GCC_except_table830+0x1364>
vpmadd52huq zmm13,zmm27,zmm6
vpmadd52huq zmm12,zmm25,zmm6
vpmadd52huq zmm11,zmm31,zmm6
vpmadd52luq zmm3,zmm9,zmm6
vpmadd52luq zmm13,zmm25,zmm6
vpmadd52luq zmm12,zmm31,zmm6
vpmadd52luq zmm11,zmm8,zmm6
vpxor  xmm6,xmm6,xmm6
vpmadd52luq zmm6,zmm5,zmm14
vpaddq zmm5,zmm5,zmm22
vpsrlq zmm5,zmm5,0x34
vpmadd52huq zmm11,zmm6,zmm10
vpmadd52huq zmm3,zmm6,zmm20
vpmadd52huq zmm4,zmm6,zmm29
vpmadd52huq zmm12,zmm6,zmm1
vpmadd52huq zmm5,zmm6,zmm24
vpmadd52luq zmm13,zmm6,zmm1
vpmadd52luq zmm11,zmm6,zmm20
vpmadd52luq zmm3,zmm6,zmm29
vpmadd52huq zmm4,zmm26,zmm28
vpaddq zmm13,zmm13,zmm5
vpmadd52luq zmm12,zmm6,zmm10
vpxor  xmm5,xmm5,xmm5
vpmadd52huq zmm5,zmm2,zmm28
vpxor  xmm6,xmm6,xmm6
vpmadd52luq zmm13,zmm18,zmm28
vpmadd52huq zmm11,zmm30,zmm28
vpmadd52huq zmm3,zmm23,zmm28
vpmadd52luq zmm4,zmm2,zmm28
vpmadd52huq zmm12,zmm18,zmm28
vpmadd52huq zmm5,zmm9,zmm7
vpmadd52luq zmm13,zmm27,zmm7
vpmadd52luq zmm11,zmm23,zmm28
vpmadd52luq zmm3,zmm26,zmm28
vpmadd52huq zmm4,zmm8,zmm7
vpmadd52luq zmm12,zmm30,zmm28
vpmadd52luq zmm6,zmm13,zmm14
vpmadd52huq zmm11,zmm25,zmm7
vpmadd52huq zmm3,zmm31,zmm7
vpmadd52luq zmm4,zmm9,zmm7
vpmadd52huq zmm12,zmm27,zmm7
vpmadd52huq zmm5,zmm6,zmm29
vpmadd52luq zmm11,zmm31,zmm7
vpmadd52luq zmm3,zmm8,zmm7
vpmadd52huq zmm4,zmm6,zmm20
vpmadd52luq zmm12,zmm25,zmm7
vpaddq zmm7,zmm13,zmm22
vpsrlq zmm7,zmm7,0x34
vpmadd52huq zmm5,zmm26,zmm19
vpmadd52huq zmm7,zmm6,zmm24
vpmadd52huq zmm11,zmm6,zmm1
vpmadd52huq zmm3,zmm6,zmm10
vpmadd52luq zmm4,zmm6,zmm29
vpmadd52luq zmm12,zmm6,zmm1
vpmadd52luq zmm5,zmm2,zmm19
vpmadd52luq zmm11,zmm6,zmm10
vpmadd52luq zmm3,zmm6,zmm20
vpmadd52huq zmm4,zmm23,zmm19
vpaddq zmm7,zmm12,zmm7
vpxor  xmm6,xmm6,xmm6
vpmadd52huq zmm6,zmm2,zmm19
vpmadd52luq zmm7,zmm18,zmm19
vpmadd52huq zmm5,zmm8,zmm0
vpmadd52huq zmm11,zmm18,zmm19
vpmadd52huq zmm3,zmm30,zmm19
vpmadd52luq zmm4,zmm26,zmm19
vpmadd52huq zmm6,zmm9,zmm0
vpmadd52luq zmm7,zmm27,zmm0
vpmadd52luq zmm5,zmm9,zmm0
vpmadd52luq zmm11,zmm30,zmm19
vpmadd52luq zmm3,zmm23,zmm19
vpmadd52huq zmm4,zmm31,zmm0
vmovdqa64 zmm19,zmm14
vpmadd52huq zmm11,zmm27,zmm0
vpmadd52huq zmm3,zmm25,zmm0
vpmadd52luq zmm4,zmm8,zmm0
vpmadd52luq zmm11,zmm25,zmm0
vpmadd52luq zmm3,zmm31,zmm0
vpxor  xmm0,xmm0,xmm0
vpmadd52luq zmm0,zmm7,zmm14
vpaddq zmm7,zmm7,zmm22
vpsrlq zmm7,zmm7,0x34
vpmadd52huq zmm5,zmm0,zmm20
vpmadd52huq zmm6,zmm0,zmm29
vpmadd52huq zmm7,zmm0,zmm24
vpmadd52luq zmm11,zmm0,zmm1
vpmadd52huq zmm3,zmm0,zmm1
vpmadd52huq zmm4,zmm0,zmm10
vpmadd52luq zmm5,zmm0,zmm29
vpmadd52huq zmm6,zmm26,zmm21
vpaddq zmm7,zmm11,zmm7
vpxor  xmm11,xmm11,xmm11
vpmadd52huq zmm11,zmm2,zmm21
vpmadd52luq zmm3,zmm0,zmm10
vpmadd52luq zmm4,zmm0,zmm20
vpmadd52luq zmm7,zmm18,zmm21
vpmadd52huq zmm5,zmm23,zmm21
vpmadd52luq zmm6,zmm2,zmm21
vpmadd52huq zmm3,zmm18,zmm21
vpmadd52huq zmm4,zmm30,zmm21
vpmadd52huq zmm11,zmm9,zmm17
vpmadd52luq zmm7,zmm27,zmm17
vpmadd52luq zmm5,zmm26,zmm21
vpandq zmm26,zmm22,ZMMWORD PTR [rsp+0x940]
vpmadd52luq zmm4,zmm23,zmm21
vpmadd52luq zmm3,zmm30,zmm21
vpmadd52huq zmm6,zmm8,zmm17
vmovdqa64 zmm23,ZMMWORD PTR [rsp+0x4c0]
vmovdqa64 zmm21,ZMMWORD PTR [rsp+0x9c0]
vpmadd52huq zmm3,zmm27,zmm17
vpmadd52huq zmm4,zmm25,zmm17
vpmadd52huq zmm5,zmm31,zmm17
vpmadd52luq zmm6,zmm9,zmm17
vmovdqa64 zmm9,ZMMWORD PTR [rsp+0x2c0]
vpaddq zmm2,zmm26,zmm16
vpmadd52luq zmm3,zmm25,zmm17
vpmadd52luq zmm5,zmm8,zmm17
vpmadd52luq zmm4,zmm31,zmm17
vpbroadcastq zmm17,QWORD PTR [rip+0xfffffffffff1d7a8]        # 97e8 <GCC_except_table830+0x12a4>
vmovdqa64 zmm25,ZMMWORD PTR [rsp+0x980]
vmovdqa64 zmm8,ZMMWORD PTR [rsp+0xa40]
vpsraq zmm12,zmm2,0x34
vpaddq zmm12,zmm12,ZMMWORD PTR [rsp+0xa00]
vpsraq zmm13,zmm12,0x34
vpaddq zmm13,zmm13,ZMMWORD PTR [rsp+0x1100]
vpsraq zmm14,zmm13,0x34
vpaddq zmm14,zmm14,ZMMWORD PTR [rsp+0x380]
vpsraq zmm15,zmm14,0x34
vpaddq zmm15,zmm15,ZMMWORD PTR [rsp+0x340]
vpcmpltuq k1,zmm15,zmm17
vpandq zmm26{k1},zmm2,zmm22
vpxor  xmm2,xmm2,xmm2
vpmadd52luq zmm2,zmm7,zmm19
vpaddq zmm7,zmm7,zmm22
vpandq zmm23{k1},zmm14,zmm22
vpandq zmm25{k1},zmm13,zmm22
vpandq zmm21{k1},zmm12,zmm22
vmovdqa64 zmm9{k1},zmm15
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0x480]
vpsrlq zmm0,zmm21,0xc
vpmadd52huq zmm4,zmm2,zmm1
vpmadd52luq zmm3,zmm2,zmm1
vpsrlq zmm1,zmm7,0x34
vpmadd52huq zmm5,zmm2,zmm10
vpmadd52huq zmm6,zmm2,zmm20
vpmadd52huq zmm11,zmm2,zmm29
vpmadd52huq zmm1,zmm2,zmm24
vpmadd52luq zmm4,zmm2,zmm10
vpmadd52luq zmm5,zmm2,zmm20
vpmadd52luq zmm6,zmm2,zmm29
vpaddq zmm1,zmm3,zmm1
vpsrlq zmm2,zmm1,0x34
vpandq zmm1,zmm1,zmm22
vpsubq zmm1,zmm1,ZMMWORD PTR [rsp+0x1800]
vpaddq zmm1,zmm1,QWORD BCST [rip+0xfffffffffff1d7a6]        # 98d8 <GCC_except_table830+0x1394>
vpaddq zmm2,zmm2,zmm4
vpsrlq zmm3,zmm2,0x34
vpandq zmm2,zmm2,zmm22
vpsubq zmm2,zmm2,ZMMWORD PTR [rsp+0x17c0]
vpaddq zmm3,zmm3,zmm5
vpsrlq zmm4,zmm3,0x34
vpandq zmm3,zmm3,zmm22
vpsubq zmm3,zmm3,ZMMWORD PTR [rsp+0x1780]
vpaddq zmm4,zmm4,zmm6
vpsrlq zmm5,zmm4,0x34
vpandq zmm4,zmm4,zmm22
vpsubq zmm4,zmm4,ZMMWORD PTR [rsp+0x1740]
vpaddq zmm5,zmm5,zmm11
vpsubq zmm5,zmm5,ZMMWORD PTR [rsp+0x1700]
vpsraq zmm6,zmm1,0x34
vpaddq zmm6,zmm6,QWORD BCST [rip+0xfffffffffff1d7d6]        # 9978 <GCC_except_table830+0x1434>
vpandq zmm1,zmm1,zmm22
vpaddq zmm7,zmm1,zmm16
vpsraq zmm12,zmm7,0x34
vpaddq zmm2,zmm2,zmm6
vpsraq zmm6,zmm2,0x34
vpandq zmm2,zmm2,zmm22
vpsubq zmm14,zmm2,QWORD BCST [rsp+0x608]
vpaddq zmm3,zmm3,zmm6
vpaddq zmm3,zmm3,QWORD BCST [rip+0xfffffffffff1d76d]        # 9950 <GCC_except_table830+0x140c>
vpsraq zmm6,zmm3,0x34
vpandq zmm10,zmm3,zmm22
vpaddq zmm3,zmm4,QWORD BCST [rip+0xfffffffffff1d67e]        # 9878 <GCC_except_table830+0x1334>
vpaddq zmm3,zmm3,zmm6
vpandq zmm11,zmm3,zmm22
vpsraq zmm4,zmm3,0x34
vpaddq zmm3,zmm5,QWORD BCST [rip+0xfffffffffff1d5d9]        # 97f0 <GCC_except_table830+0x12ac>
vpsubq zmm5,zmm10,QWORD BCST [rsp+0x610]
vpsubq zmm6,zmm11,QWORD BCST [rsp+0x618]
vpaddq zmm13,zmm3,zmm4
vpaddq zmm4,zmm14,zmm12
vpandq zmm3,zmm13,zmm22
vpsubq zmm3,zmm3,QWORD BCST [rsp+0x620]
vpsraq zmm12,zmm4,0x34
vpaddq zmm5,zmm5,zmm12
vpsraq zmm12,zmm5,0x34
vpaddq zmm6,zmm6,zmm12
vpsraq zmm12,zmm6,0x34
vpaddq zmm3,zmm3,zmm12
vpcmpltuq k1,zmm3,zmm17
vpandq zmm2{k1},zmm4,zmm22
vmovdqa64 zmm4,ZMMWORD PTR [rsp+0x3c0]
vpandq zmm1{k1},zmm7,zmm22
vmovdqa64 zmm7,ZMMWORD PTR [rsp+0x900]
vpandq zmm10{k1},zmm5,zmm22
vmovdqa64 zmm13{k1},zmm3
vpsllq zmm5,zmm8,0x28
vpsrlq zmm8,zmm8,0x18
vpandq zmm11{k1},zmm6,zmm22
vpsllq zmm3,zmm4,0x34
vpsrlq zmm4,zmm4,0xc
vpsrlq zmm6,zmm7,0x24
vporq  zmm3,zmm3,ZMMWORD PTR [rsp+0xa80]
vporq  zmm12,zmm5,zmm4
vpsllq zmm5,zmm7,0x1c
vmovdqa64 zmm7,ZMMWORD PTR [rsp+0xb00]
vporq  zmm14,zmm5,zmm8
vpsllq zmm5,ZMMWORD PTR [rsp+0x880],0x10
vmovdqa64 zmm8,ZMMWORD PTR [rsp+0x5c0]
vpunpcklqdq zmm4,zmm3,zmm12
vpsllq zmm16,zmm7,0x34
vporq  zmm16,zmm16,ZMMWORD PTR [rsp+0xac0]
vporq  zmm15,zmm5,zmm6
vpsrlq zmm6,zmm7,0xc
vpsllq zmm17,zmm8,0x28
vpsrlq zmm8,zmm8,0x18
vpsllq zmm7,zmm19,0x1c
vporq  zmm18,zmm7,zmm8
vpsllq zmm8,ZMMWORD PTR [rsp+0x440],0x10
vporq  zmm17,zmm17,zmm6
vpsrlq zmm6,zmm19,0x24
vpunpcklqdq zmm5,zmm14,zmm15
vinserti32x4 ymm28,ymm4,xmm5,0x1
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm28
vporq  zmm19,zmm8,zmm6
vpsllq zmm8,zmm21,0x34
vpunpcklqdq zmm6,zmm16,zmm17
vporq  zmm20,zmm8,zmm26
vpsllq zmm8,zmm25,0x28
vpunpcklqdq zmm7,zmm18,zmm19
vporq  zmm21,zmm8,zmm0
vpsrlq zmm0,zmm25,0x18
vpsllq zmm8,zmm23,0x1c
vinserti32x4 ymm29,ymm6,xmm7,0x1
vporq  zmm22,zmm8,zmm0
vpsrlq zmm0,zmm23,0x24
vpsllq zmm8,zmm9,0x10
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm29
vporq  zmm23,zmm8,zmm0
vpsllq zmm0,zmm2,0x34
vpunpcklqdq zmm8,zmm20,zmm21
vporq  zmm24,zmm0,zmm1
vpsrlq zmm0,zmm2,0xc
vpsllq zmm1,zmm10,0x28
vpsllq zmm2,zmm13,0x10
vporq  zmm25,zmm1,zmm0
vpsrlq zmm0,zmm10,0x18
vpsllq zmm1,zmm11,0x1c
vporq  zmm26,zmm1,zmm0
vpsrlq zmm1,zmm11,0x24
vpunpcklqdq zmm0,zmm22,zmm23
vpunpcklqdq zmm9,zmm24,zmm25
vporq  zmm27,zmm2,zmm1
vinserti128 ymm2,ymm8,xmm0,0x1
vpunpcklqdq zmm1,zmm26,zmm27
vmovdqu YMMWORD PTR [r10+rax*1+0x40],ymm2
vinserti128 ymm10,ymm9,xmm1,0x1
vmovdqu YMMWORD PTR [r10+rax*1+0x60],ymm10
cmp    r12,0x1
je     ec845 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc275>
mov    eax,DWORD PTR [r8+0x10]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec830 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc260>
vpunpckhqdq zmm10,zmm3,zmm12
vpunpckhqdq zmm12,zmm14,zmm15
vpunpckhqdq zmm11,zmm16,zmm17
vpunpckhqdq zmm15,zmm18,zmm19
vpunpckhqdq zmm14,zmm20,zmm21
vpunpckhqdq zmm2,zmm22,zmm23
vpunpckhqdq zmm13,zmm24,zmm25
vpunpckhqdq zmm3,zmm26,zmm27
shl    rax,0x7
vinserti32x4 ymm28,ymm10,xmm12,0x1
vinserti32x4 ymm16,ymm11,xmm15,0x1
vinserti32x4 ymm17,ymm14,xmm2,0x1
vinserti32x4 ymm18,ymm13,xmm3,0x1
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm28
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm16
vmovdqu64 YMMWORD PTR [r10+rax*1+0x40],ymm17
vmovdqu64 YMMWORD PTR [r10+rax*1+0x60],ymm18
cmp    r12,0x2
je     ec845 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc275>
mov    eax,DWORD PTR [r8+0x20]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec830 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc260>
vshufi64x2 ymm16,ymm4,ymm5,0x3
vshufi64x2 ymm17,ymm6,ymm7,0x3
vshufi64x2 ymm18,ymm8,ymm0,0x3
vshufi64x2 ymm19,ymm9,ymm1,0x3
shl    rax,0x7
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm16
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm17
vmovdqu64 YMMWORD PTR [r10+rax*1+0x40],ymm18
vmovdqu64 YMMWORD PTR [r10+rax*1+0x60],ymm19
cmp    r12,0x3
je     ec845 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc275>
mov    eax,DWORD PTR [r8+0x30]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec830 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc260>
vshufi64x2 ymm16,ymm10,ymm12,0x3
vshufi64x2 ymm17,ymm11,ymm15,0x3
vshufi64x2 ymm18,ymm14,ymm2,0x3
vshufi64x2 ymm19,ymm13,ymm3,0x3
shl    rax,0x7
vmovdqu64 YMMWORD PTR [r10+rax*1],ymm16
vmovdqu64 YMMWORD PTR [r10+rax*1+0x20],ymm17
vmovdqu64 YMMWORD PTR [r10+rax*1+0x40],ymm18
vmovdqu64 YMMWORD PTR [r10+rax*1+0x60],ymm19
cmp    r12,0x4
je     ec845 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc275>
mov    eax,DWORD PTR [r8+0x40]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec830 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc260>
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
cmp    r12,0x5
je     ec845 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc275>
mov    eax,DWORD PTR [r8+0x50]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec830 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc260>
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
cmp    r12,0x6
je     ec845 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc275>
mov    eax,DWORD PTR [r8+0x60]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec830 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc260>
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
cmp    r12,0x7
je     ec845 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc275>
mov    eax,DWORD PTR [r8+0x70]
and    eax,0x7fffffff
cmp    r13,rax
jbe    ec830 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0xc260>
vperm2i128 ymm1,ymm11,ymm15,0x31
vpbroadcastq zmm15,QWORD PTR [rip+0xfffffffffff1d012]        # 9748 <GCC_except_table830+0x1204>
vpbroadcastq zmm16,QWORD PTR [rip+0xfffffffffff1d060]        # 97a0 <GCC_except_table830+0x125c>
mov    rcx,QWORD PTR [rsp+0x500]
vextracti64x4 ymm2,zmm2,0x1
vextracti64x4 ymm3,zmm3,0x1
vperm2i128 ymm0,ymm10,ymm12,0x31
shl    rax,0x7
add    r12,0x8
vperm2i128 ymm2,ymm14,ymm2,0x31
vperm2i128 ymm3,ymm13,ymm3,0x31
vmovdqu YMMWORD PTR [r10+rax*1],ymm0
vmovdqu YMMWORD PTR [r10+rax*1+0x20],ymm1
vmovdqu YMMWORD PTR [r10+rax*1+0x40],ymm2
vmovdqu YMMWORD PTR [r10+rax*1+0x60],ymm3
mov    rbx,rcx
test   rcx,rcx
jne    e7bd0 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x7600>
lea    rsp,[rbp-0x28]
pop    rbx
pop    r12
pop    r13
pop    r14
pop    r15
pop    rbp
vzeroupper
ret
lea    rdi,[rip+0xfffffffffff23b78]        # 10328 <_RNvNtNtNtNtNtCsgxBkk5gSRhY_4core3num3imp7flt2dec8strategy6dragon9POW5TO256+0x27c>
lea    rdx,[rip+0x455d9]        # 131d90 <__frame_dummy_init_array_entry+0xcd0>
mov    esi,0x5d
vzeroupper
call   3b690 <_RNvNtCsgxBkk5gSRhY_4core9panicking5panic>
mov    rdx,QWORD PTR [rsp+0x18]
lea    rcx,[rip+0x45e60]        # 132630 <__frame_dummy_init_array_entry+0x1570>
mov    rdi,r11
vzeroupper
call   3b5c0 <_RNvNtNtCsgxBkk5gSRhY_4core5slice5index16slice_index_fail>
mov    edx,0x280
mov    rsi,rbx
vmovdqa64 ZMMWORD PTR [rsp+0x880],zmm16
vmovdqa64 ZMMWORD PTR [rsp+0xac0],zmm17
vmovdqa64 ZMMWORD PTR [rsp+0xa80],zmm18
vmovdqa64 ZMMWORD PTR [rsp+0xa40],zmm19
vzeroupper
call   480a0 <_RINvNvMs2_NtCs6i54tJFfzR_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECse6ToTOequVP_10zopro_core>
vmovdqa64 zmm19,ZMMWORD PTR [rsp+0xa40]
vmovdqa64 zmm18,ZMMWORD PTR [rsp+0xa80]
vmovdqa64 zmm17,ZMMWORD PTR [rsp+0xac0]
vmovdqa64 zmm16,ZMMWORD PTR [rsp+0x880]
jmp    e06d7 <_RNvMs1_NtNtCse6ToTOequVP_10zopro_core10msm_affine15quarter_kernelsNtB5_9G2Scratch5apply+0x107>
lea    rdx,[rip+0x46279]        # 132ab0 <__frame_dummy_init_array_entry+0x19f0>
mov    rdi,rax
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x46247]        # 132a98 <__frame_dummy_init_array_entry+0x19d8>
mov    rsi,rdi
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r13,QWORD PTR [rsp+0x20]
lea    rdx,[rip+0x45db0]        # 132618 <__frame_dummy_init_array_entry+0x1558>
mov    rdi,rcx
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x2
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45d4a]        # 1325d0 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x4
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45d2f]        # 1325d0 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
inc    r14
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45d15]        # 1325d0 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x3
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45cfa]        # 1325d0 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x5
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45cdf]        # 1325d0 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x6
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45cc4]        # 1325d0 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
add    r14,0x7
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45ca9]        # 1325d0 <__frame_dummy_init_array_entry+0x1510>
mov    rdi,r14
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r8
lea    rdx,[rip+0x45cac]        # 1325e8 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r10
lea    rdx,[rip+0x45c97]        # 1325e8 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rdx
lea    rdx,[rip+0x45c82]        # 1325e8 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,r9
lea    rdx,[rip+0x45c6d]        # 1325e8 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rcx
lea    rdx,[rip+0x45c58]        # 1325e8 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rsi
lea    rdx,[rip+0x45c43]        # 1325e8 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rax
lea    rdx,[rip+0x45c2e]        # 1325e8 <__frame_dummy_init_array_entry+0x1528>
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rdi,rsi
mov    rsi,QWORD PTR [rsp+0x20]
lea    rdx,[rip+0x46094]        # 132a68 <__frame_dummy_init_array_entry+0x19a8>
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
lea    rdx,[rip+0x4603d]        # 132a20 <__frame_dummy_init_array_entry+0x1960>
mov    rdi,rax
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rsi,QWORD PTR [rsp+0x20]
lea    rdx,[rip+0x46056]        # 132a50 <__frame_dummy_init_array_entry+0x1990>
mov    rdi,rax
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x46027]        # 132a38 <__frame_dummy_init_array_entry+0x1978>
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r11,rdi
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45bd8]        # 132600 <__frame_dummy_init_array_entry+0x1540>
mov    rdi,r11
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r11,r13
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45bbe]        # 132600 <__frame_dummy_init_array_entry+0x1540>
mov    rdi,r11
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r11,r10
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45ba4]        # 132600 <__frame_dummy_init_array_entry+0x1540>
mov    rdi,r11
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r11,rax
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b8a]        # 132600 <__frame_dummy_init_array_entry+0x1540>
mov    rdi,r11
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r11,r12
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b70]        # 132600 <__frame_dummy_init_array_entry+0x1540>
mov    rdi,r11
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r11,r15
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b56]        # 132600 <__frame_dummy_init_array_entry+0x1540>
mov    rdi,r11
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r11,r9
mov    rsi,QWORD PTR [rsp+0x18]
lea    rdx,[rip+0x45b3c]        # 132600 <__frame_dummy_init_array_entry+0x1540>
mov    rdi,r11
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rcx,rsi
lea    rdx,[rip+0x45b3f]        # 132618 <__frame_dummy_init_array_entry+0x1558>
mov    rdi,rcx
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    rcx,rax
lea    rdx,[rip+0x45b27]        # 132618 <__frame_dummy_init_array_entry+0x1558>
mov    rdi,rcx
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
lea    rdx,[rip+0x45f7a]        # 132a80 <__frame_dummy_init_array_entry+0x19c0>
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>
mov    r13,QWORD PTR [rsp+0x20]
mov    rcx,rsi
lea    rdx,[rip+0x45afb]        # 132618 <__frame_dummy_init_array_entry+0x1558>
mov    rdi,rcx
mov    rsi,r13
vzeroupper
call   3a66b <_RNvNtCsgxBkk5gSRhY_4core9panicking18panic_bounds_check>