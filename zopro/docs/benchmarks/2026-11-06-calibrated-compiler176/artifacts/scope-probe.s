.intel_syntax noprefix
# Model applicability only; never executed as a benchmark.
# Selected register instructions.
vpmadd52luq zmm0,zmm1,zmm2
vpmadd52huq zmm3,zmm4,zmm5
vpaddq zmm6,zmm7,zmm8
vpsubq zmm9,zmm10,zmm11
# Existing zero-idiom handling must remain.
vpsubq zmm12,zmm13,zmm13
# Unselected register widths, masks, memory and shifts.
vpmadd52luq ymm0,ymm1,ymm2
vpmadd52huq xmm3,xmm4,xmm5
vpmadd52luq zmm6{k1},zmm7,zmm8
vpmadd52huq zmm9{k2}{z},zmm10,zmm11
vpmadd52luq zmm12,zmm13,ZMMWORD PTR [rsp+0x40]
vpmadd52huq zmm14,zmm15,ZMMWORD PTR [rsp+0x80]
vpaddq zmm16{k3},zmm17,zmm18
vpsubq zmm19{k4},zmm20,zmm21
vpaddq ymm3,ymm4,ymm5
vpsrlq zmm22,zmm23,52
vpsrlq zmm24,zmm25,51
vpsraq zmm26,zmm27,52
vpandq zmm28,zmm29,zmm30
