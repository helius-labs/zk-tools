.intel_syntax noprefix
# Model coverage only. This file is never assembled or executed as a benchmark.
vpmadd52luq zmm0,zmm1,zmm2
vpmadd52huq zmm3,zmm1,zmm2
vpmadd52luq zmm4,zmm5,ZMMWORD PTR [rsp+0x40]
vpmadd52huq zmm6,zmm7,ZMMWORD PTR [rsp+0x80]
vpaddq zmm8,zmm0,zmm3
vpsrlq zmm9,zmm8,52
vpsraq zmm10,zmm8,52
vmovdqa64 ZMMWORD PTR [rsp+0xc0],zmm9
