
/home/ubuntu/src/zopro-perf-work/group-cpu211/build-run/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000474c40 <zopro_core::msm_affine::g2_query_exclusive>:
  474c40:	push   %rbp
  474c41:	push   %r15
  474c43:	push   %r14
  474c45:	push   %r13
  474c47:	push   %r12
  474c49:	push   %rbx
  474c4a:	sub    $0x188,%rsp
  474c51:	mov    %r8,%rbx
  474c54:	mov    %rcx,%r12
  474c57:	mov    %rdx,%r14
  474c5a:	mov    %rsi,%r15
  474c5d:	mov    %rdi,%r13
  474c60:	call   481c70 <zopro_core::boolean_phase_trace::group_begin>
  474c65:	mov    %r14,0x30(%rsp)
  474c6a:	mov    %rbx,0x50(%rsp)
  474c6f:	cmp    %rbx,%r14
  474c72:	jne    4752cc <zopro_core::msm_affine::g2_query_exclusive+0x68c>
  474c78:	lea    0x50(%rsp),%rdi
  474c7d:	lea    -0x437b34(%rip),%rsi        # 3d150 <zmij::STATIC_DATA+0x50d0>
  474c84:	mov    $0xf,%edx
  474c89:	mov    %r15,0x28(%rsp)
  474c8e:	mov    %rdi,0x18(%rsp)
  474c93:	call   15b470 <std::sys::env::unix::getenv>
  474c98:	mov    0x50(%rsp),%r15
  474c9d:	cmp    $0xffffffffffffffff,%r15
  474ca1:	je     474d6b <zopro_core::msm_affine::g2_query_exclusive+0x12b>
  474ca7:	mov    0x58(%rsp),%rbp
  474cac:	mov    0x60(%rsp),%rbx
  474cb1:	lea    0x50(%rsp),%rdi
  474cb6:	mov    %rbp,%rsi
  474cb9:	mov    %rbx,%rdx
  474cbc:	call   107820 <core::str::converts::from_utf8>
  474cc1:	cmpb   $0x0,0x50(%rsp)
  474cc6:	je     474cf9 <zopro_core::msm_affine::g2_query_exclusive+0xb9>
  474cc8:	test   %r15,%r15
  474ccb:	jle    474d6b <zopro_core::msm_affine::g2_query_exclusive+0x12b>
  474cd1:	cmp    $0x1ffff,%r15
  474cd8:	jbe    474d49 <zopro_core::msm_affine::g2_query_exclusive+0x109>
  474cda:	mov    %r15,%rsi
  474cdd:	inc    %rsi
  474ce0:	js     47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  474ce6:	cmpb   $0x1,0x0(%rbp,%r15,1)
  474cec:	jne    474d49 <zopro_core::msm_affine::g2_query_exclusive+0x109>
  474cee:	mov    %rbp,%rdi
  474cf1:	call   *0x9bcb1(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  474cf7:	jmp    474d6b <zopro_core::msm_affine::g2_query_exclusive+0x12b>
  474cf9:	mov    %r13,0x20(%rsp)
  474cfe:	mov    $0x1,%r13b
  474d01:	cmp    $0x1,%rbx
  474d05:	jne    474d0f <zopro_core::msm_affine::g2_query_exclusive+0xcf>
  474d07:	cmpb   $0x30,0x0(%rbp)
  474d0b:	setne  %r13b
  474d0f:	test   %r15,%r15
  474d12:	je     474d3a <zopro_core::msm_affine::g2_query_exclusive+0xfa>
  474d14:	cmp    $0x1ffff,%r15
  474d1b:	jbe    474d54 <zopro_core::msm_affine::g2_query_exclusive+0x114>
  474d1d:	mov    %r15,%rsi
  474d20:	inc    %rsi
  474d23:	js     47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  474d29:	cmpb   $0x1,0x0(%rbp,%r15,1)
  474d2f:	jne    474d54 <zopro_core::msm_affine::g2_query_exclusive+0x114>
  474d31:	mov    %rbp,%rdi
  474d34:	call   *0x9bc6e(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  474d3a:	test   %r13b,%r13b
  474d3d:	mov    0x20(%rsp),%r13
  474d42:	jne    474d6b <zopro_core::msm_affine::g2_query_exclusive+0x12b>
  474d44:	jmp    475063 <zopro_core::msm_affine::g2_query_exclusive+0x423>
  474d49:	mov    %rbp,%rdi
  474d4c:	call   *0x9bc5e(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  474d52:	jmp    474d6b <zopro_core::msm_affine::g2_query_exclusive+0x12b>
  474d54:	mov    %rbp,%rdi
  474d57:	call   *0x9bc53(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  474d5d:	test   %r13b,%r13b
  474d60:	mov    0x20(%rsp),%r13
  474d65:	je     475063 <zopro_core::msm_affine::g2_query_exclusive+0x423>
  474d6b:	lea    0x110(%rsp),%rdi
  474d73:	mov    %r12,%rsi
  474d76:	mov    %r14,%rdx
  474d79:	call   2317f0 <zopro_core::msm_affine::grouping::scalar_groups>
  474d7e:	call   481d70 <zopro_core::boolean_phase_trace::group_end>
  474d83:	mov    0x28(%rsp),%r15
  474d88:	mov    0x110(%rsp),%rax
  474d90:	lea    -0x400(%r14),%rcx
  474d97:	cmp    $0xfc00,%rcx
  474d9e:	jae    474eaf <zopro_core::msm_affine::g2_query_exclusive+0x26f>
  474da4:	cmp    $0xffffffffffffffff,%rax
  474da8:	je     474f4d <zopro_core::msm_affine::g2_query_exclusive+0x30d>
  474dae:	vmovups 0x110(%rsp),%zmm0
  474db9:	vmovups 0x148(%rsp),%zmm1
  474dc4:	movabs $0x7ffffffffffffff9,%rbx
  474dce:	vmovups %zmm0,0x50(%rsp)
  474dd9:	vmovups %zmm1,0x88(%rsp)
  474de4:	mov    0x60(%rsp),%r12
  474de9:	lea    -0x400(%r12),%rax
  474df1:	cmp    $0xfc00,%rax
  474df7:	jae    475074 <zopro_core::msm_affine::g2_query_exclusive+0x434>
  474dfd:	vmovaps 0xa0(%rsp),%xmm0
  474e06:	mov    0x90(%rsp),%rax
  474e0e:	mov    0x70(%rsp),%rcx
  474e13:	mov    0x78(%rsp),%r8
  474e18:	mov    0x88(%rsp),%r9
  474e20:	lea    0x38(%rsp),%rdi
  474e25:	vmovups %xmm0,0x8(%rsp)
  474e2b:	mov    %rax,(%rsp)
  474e2f:	mov    %r15,%rsi
  474e32:	mov    %r14,%rdx
  474e35:	vzeroupper
  474e38:	call   2451d0 <zopro_core::msm::group_affine::<zopro_bn254::curve::G2Config>>
  474e3d:	mov    0x40(%rsp),%r14
  474e42:	mov    0x48(%rsp),%rdx
  474e47:	mov    0x58(%rsp),%rcx
  474e4c:	mov    %r13,%rdi
  474e4f:	mov    %r14,%rsi
  474e52:	mov    %r12,%r8
  474e55:	call   481f10 <zopro_core::msm_affine::g2_phi::raw_exclusive>
  474e5a:	mov    0x38(%rsp),%rax
  474e5f:	test   %rax,%rax
  474e62:	je     47508a <zopro_core::msm_affine::g2_query_exclusive+0x44a>
  474e68:	cmp    $0x3ff,%rax
  474e6e:	jbe    4752b0 <zopro_core::msm_affine::g2_query_exclusive+0x670>
  474e74:	shl    $0x7,%rax
  474e78:	mov    %rax,%rsi
  474e7b:	or     $0x1,%rsi
  474e7f:	cmp    %rbx,%rsi
  474e82:	jae    47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  474e88:	cmpb   $0x1,(%r14,%rax,1)
  474e8d:	jne    4752b0 <zopro_core::msm_affine::g2_query_exclusive+0x670>
  474e93:	mov    %r14,%rdi
  474e96:	call   *0x9bb0c(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  474e9c:	mov    0x50(%rsp),%rax
  474ea1:	test   %rax,%rax
  474ea4:	jne    475098 <zopro_core::msm_affine::g2_query_exclusive+0x458>
  474eaa:	jmp    4751ae <zopro_core::msm_affine::g2_query_exclusive+0x56e>
  474eaf:	cmp    $0xffffffffffffffff,%rax
  474eb3:	je     47504a <zopro_core::msm_affine::g2_query_exclusive+0x40a>
  474eb9:	vmovups 0x148(%rsp),%zmm1
  474ec4:	vmovups 0x110(%rsp),%zmm0
  474ecf:	vmovups %zmm1,0x88(%rsp)
  474eda:	vmovups %zmm0,0x50(%rsp)
  474ee5:	lea    0x50(%rsp),%rcx
  474eea:	mov    %r13,%rdi
  474eed:	mov    %r15,%rsi
  474ef0:	mov    %r14,%rdx
  474ef3:	vzeroupper
  474ef6:	call   481210 <zopro_core::msm_affine::g2_query_grouped>
  474efb:	mov    0x50(%rsp),%rax
  474f00:	movabs $0x7ffffffffffffff9,%rbx
  474f0a:	test   %rax,%rax
  474f0d:	je     4750d8 <zopro_core::msm_affine::g2_query_exclusive+0x498>
  474f13:	mov    0x58(%rsp),%rdi
  474f18:	cmp    $0xfff,%rax
  474f1e:	jbe    4750d2 <zopro_core::msm_affine::g2_query_exclusive+0x492>
  474f24:	shl    $0x5,%rax
  474f28:	mov    %rax,%rsi
  474f2b:	or     $0x1,%rsi
  474f2f:	cmp    %rbx,%rsi
  474f32:	jae    47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  474f38:	cmpb   $0x1,(%rdi,%rax,1)
  474f3c:	jne    4750d2 <zopro_core::msm_affine::g2_query_exclusive+0x492>
  474f42:	call   *0x9ba60(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  474f48:	jmp    4750d8 <zopro_core::msm_affine::g2_query_exclusive+0x498>
  474f4d:	mov    0x9d735(%rip),%eax        # 512688 <zopro_core::boolean_phase_trace::ENABLED>
  474f53:	test   %eax,%eax
  474f55:	jne    4752e4 <zopro_core::msm_affine::g2_query_exclusive+0x6a4>
  474f5b:	cmpb   $0x1,0x9d72a(%rip)        # 51268c <zopro_core::boolean_phase_trace::ENABLED+0x4>
  474f62:	jne    474fa3 <zopro_core::msm_affine::g2_query_exclusive+0x363>
  474f64:	mov    0x9d716(%rip),%eax        # 512680 <zopro_core::boolean_phase_trace::ORIGIN+0x10>
  474f6a:	test   %eax,%eax
  474f6c:	jne    4752f8 <zopro_core::msm_affine::g2_query_exclusive+0x6b8>
  474f72:	lea    0x9d6f7(%rip),%rdi        # 512670 <zopro_core::boolean_phase_trace::ORIGIN>
  474f79:	call   151010 <<std::time::Instant>::elapsed>
  474f7e:	mov    $0x3b9aca00,%esi
  474f83:	mov    %edx,%ecx
  474f85:	mov    %rax,%rdx
  474f88:	mov    %ecx,%eax
  474f8a:	mulx   %rsi,%rsi,%rdx
  474f8f:	add    %rsi,%rax
  474f92:	adc    $0x0,%rdx
  474f96:	jne    47531b <zopro_core::msm_affine::g2_query_exclusive+0x6db>
  474f9c:	mov    %rax,0x9d8ad(%rip)        # 512850 <zopro_core::boolean_phase_trace::EVENTS+0x60>
  474fa3:	lea    0x50(%rsp),%rdi
  474fa8:	mov    %r15,%rsi
  474fab:	mov    %r14,%rdx
  474fae:	mov    %r12,%rcx
  474fb1:	mov    %r14,%r8
  474fb4:	call   481f10 <zopro_core::msm_affine::g2_phi::raw_exclusive>
  474fb9:	mov    0x9d6c9(%rip),%eax        # 512688 <zopro_core::boolean_phase_trace::ENABLED>
  474fbf:	test   %eax,%eax
  474fc1:	jne    4752ee <zopro_core::msm_affine::g2_query_exclusive+0x6ae>
  474fc7:	cmpb   $0x1,0x9d6be(%rip)        # 51268c <zopro_core::boolean_phase_trace::ENABLED+0x4>
  474fce:	jne    47500f <zopro_core::msm_affine::g2_query_exclusive+0x3cf>
  474fd0:	mov    0x9d6aa(%rip),%eax        # 512680 <zopro_core::boolean_phase_trace::ORIGIN+0x10>
  474fd6:	test   %eax,%eax
  474fd8:	jne    475302 <zopro_core::msm_affine::g2_query_exclusive+0x6c2>
  474fde:	lea    0x9d68b(%rip),%rdi        # 512670 <zopro_core::boolean_phase_trace::ORIGIN>
  474fe5:	call   151010 <<std::time::Instant>::elapsed>
  474fea:	mov    $0x3b9aca00,%esi
  474fef:	mov    %edx,%ecx
  474ff1:	mov    %rax,%rdx
  474ff4:	mov    %ecx,%eax
  474ff6:	mulx   %rsi,%rsi,%rdx
  474ffb:	add    %rsi,%rax
  474ffe:	adc    $0x0,%rdx
  475002:	jne    475311 <zopro_core::msm_affine::g2_query_exclusive+0x6d1>
  475008:	mov    %rax,0x9d849(%rip)        # 512858 <zopro_core::boolean_phase_trace::EVENTS+0x68>
  47500f:	vmovups 0x50(%rsp),%zmm0
  47501a:	vmovups 0x90(%rsp),%zmm1
  475025:	vmovups 0xd0(%rsp),%zmm2
  475030:	vmovups %zmm2,0x80(%r13)
  475037:	vmovups %zmm1,0x40(%r13)
  47503e:	vmovups %zmm0,0x0(%r13)
  475045:	jmp    47529b <zopro_core::msm_affine::g2_query_exclusive+0x65b>
  47504a:	mov    %r13,%rdi
  47504d:	mov    %r15,%rsi
  475050:	mov    %r14,%rdx
  475053:	mov    %r12,%rcx
  475056:	mov    %r14,%r8
  475059:	call   476c50 <zopro_core::msm_affine::g2_horner::raw>
  47505e:	jmp    47529b <zopro_core::msm_affine::g2_query_exclusive+0x65b>
  475063:	movq   $0xffffffffffffffff,0x110(%rsp)
  47506f:	jmp    474d7e <zopro_core::msm_affine::g2_query_exclusive+0x13e>
  475074:	lea    0x50(%rsp),%rcx
  475079:	mov    %r13,%rdi
  47507c:	mov    %r15,%rsi
  47507f:	mov    %r14,%rdx
  475082:	vzeroupper
  475085:	call   481210 <zopro_core::msm_affine::g2_query_grouped>
  47508a:	mov    0x50(%rsp),%rax
  47508f:	test   %rax,%rax
  475092:	je     4751ae <zopro_core::msm_affine::g2_query_exclusive+0x56e>
  475098:	mov    0x58(%rsp),%rdi
  47509d:	cmp    $0xfff,%rax
  4750a3:	jbe    4751a8 <zopro_core::msm_affine::g2_query_exclusive+0x568>
  4750a9:	shl    $0x5,%rax
  4750ad:	mov    %rax,%rsi
  4750b0:	or     $0x1,%rsi
  4750b4:	cmp    %rbx,%rsi
  4750b7:	jae    47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  4750bd:	cmpb   $0x1,(%rdi,%rax,1)
  4750c1:	jne    4751a8 <zopro_core::msm_affine::g2_query_exclusive+0x568>
  4750c7:	call   *0x9b8db(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  4750cd:	jmp    4751ae <zopro_core::msm_affine::g2_query_exclusive+0x56e>
  4750d2:	call   *0x9b8d8(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  4750d8:	mov    0x68(%rsp),%rax
  4750dd:	test   %rax,%rax
  4750e0:	je     47511b <zopro_core::msm_affine::g2_query_exclusive+0x4db>
  4750e2:	mov    0x70(%rsp),%rdi
  4750e7:	cmp    $0x7fff,%rax
  4750ed:	jbe    475115 <zopro_core::msm_affine::g2_query_exclusive+0x4d5>
  4750ef:	shl    $0x2,%rax
  4750f3:	lea    0x4(%rbx),%rcx
  4750f7:	mov    %rax,%rsi
  4750fa:	or     $0x1,%rsi
  4750fe:	cmp    %rcx,%rsi
  475101:	jae    47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  475107:	cmpb   $0x1,(%rdi,%rax,1)
  47510b:	jne    475115 <zopro_core::msm_affine::g2_query_exclusive+0x4d5>
  47510d:	call   *0x9b895(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  475113:	jmp    47511b <zopro_core::msm_affine::g2_query_exclusive+0x4db>
  475115:	call   *0x9b895(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  47511b:	mov    0x80(%rsp),%rax
  475123:	test   %rax,%rax
  475126:	je     475160 <zopro_core::msm_affine::g2_query_exclusive+0x520>
  475128:	mov    0x88(%rsp),%rdi
  475130:	cmp    $0x3fff,%rax
  475136:	jbe    47515a <zopro_core::msm_affine::g2_query_exclusive+0x51a>
  475138:	shl    $0x3,%rax
  47513c:	mov    %rax,%rsi
  47513f:	or     $0x1,%rsi
  475143:	cmp    %rbx,%rsi
  475146:	jae    47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  47514c:	cmpb   $0x1,(%rdi,%rax,1)
  475150:	jne    47515a <zopro_core::msm_affine::g2_query_exclusive+0x51a>
  475152:	call   *0x9b850(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  475158:	jmp    475160 <zopro_core::msm_affine::g2_query_exclusive+0x520>
  47515a:	call   *0x9b850(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  475160:	mov    0x98(%rsp),%rax
  475168:	test   %rax,%rax
  47516b:	je     47527b <zopro_core::msm_affine::g2_query_exclusive+0x63b>
  475171:	mov    0xa0(%rsp),%rdi
  475179:	cmp    $0x3fff,%rax
  47517f:	jbe    475275 <zopro_core::msm_affine::g2_query_exclusive+0x635>
  475185:	shl    $0x3,%rax
  475189:	mov    %rax,%rsi
  47518c:	or     $0x1,%rsi
  475190:	cmp    %rbx,%rsi
  475193:	jae    47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  475199:	cmpb   $0x1,(%rdi,%rax,1)
  47519d:	jne    475275 <zopro_core::msm_affine::g2_query_exclusive+0x635>
  4751a3:	jmp    47526d <zopro_core::msm_affine::g2_query_exclusive+0x62d>
  4751a8:	call   *0x9b802(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  4751ae:	mov    0x68(%rsp),%rax
  4751b3:	test   %rax,%rax
  4751b6:	je     4751f1 <zopro_core::msm_affine::g2_query_exclusive+0x5b1>
  4751b8:	mov    0x70(%rsp),%rdi
  4751bd:	cmp    $0x7fff,%rax
  4751c3:	jbe    4751eb <zopro_core::msm_affine::g2_query_exclusive+0x5ab>
  4751c5:	shl    $0x2,%rax
  4751c9:	lea    0x4(%rbx),%rcx
  4751cd:	mov    %rax,%rsi
  4751d0:	or     $0x1,%rsi
  4751d4:	cmp    %rcx,%rsi
  4751d7:	jae    47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  4751dd:	cmpb   $0x1,(%rdi,%rax,1)
  4751e1:	jne    4751eb <zopro_core::msm_affine::g2_query_exclusive+0x5ab>
  4751e3:	call   *0x9b7bf(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  4751e9:	jmp    4751f1 <zopro_core::msm_affine::g2_query_exclusive+0x5b1>
  4751eb:	call   *0x9b7bf(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  4751f1:	mov    0x80(%rsp),%rax
  4751f9:	test   %rax,%rax
  4751fc:	je     475236 <zopro_core::msm_affine::g2_query_exclusive+0x5f6>
  4751fe:	mov    0x88(%rsp),%rdi
  475206:	cmp    $0x3fff,%rax
  47520c:	jbe    475230 <zopro_core::msm_affine::g2_query_exclusive+0x5f0>
  47520e:	shl    $0x3,%rax
  475212:	mov    %rax,%rsi
  475215:	or     $0x1,%rsi
  475219:	cmp    %rbx,%rsi
  47521c:	jae    47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  475222:	cmpb   $0x1,(%rdi,%rax,1)
  475226:	jne    475230 <zopro_core::msm_affine::g2_query_exclusive+0x5f0>
  475228:	call   *0x9b77a(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  47522e:	jmp    475236 <zopro_core::msm_affine::g2_query_exclusive+0x5f6>
  475230:	call   *0x9b77a(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  475236:	mov    0x98(%rsp),%rax
  47523e:	test   %rax,%rax
  475241:	je     47527b <zopro_core::msm_affine::g2_query_exclusive+0x63b>
  475243:	mov    0xa0(%rsp),%rdi
  47524b:	cmp    $0x3fff,%rax
  475251:	jbe    475275 <zopro_core::msm_affine::g2_query_exclusive+0x635>
  475253:	shl    $0x3,%rax
  475257:	mov    %rax,%rsi
  47525a:	or     $0x1,%rsi
  47525e:	cmp    %rbx,%rsi
  475261:	jae    47530c <zopro_core::msm_affine::g2_query_exclusive+0x6cc>
  475267:	cmpb   $0x1,(%rdi,%rax,1)
  47526b:	jne    475275 <zopro_core::msm_affine::g2_query_exclusive+0x635>
  47526d:	call   *0x9b735(%rip)        # 5109a8 <munmap@GLIBC_2.2.5>
  475273:	jmp    47527b <zopro_core::msm_affine::g2_query_exclusive+0x63b>
  475275:	call   *0x9b735(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  47527b:	mov    0xb8(%rsp),%rax
  475283:	mov    0xc0(%rsp),%rcx
  47528b:	lock sub %rax,0x9d695(%rip)        # 512928 <zopro_core::preparation::RESERVED>
  475293:	lock sub %rcx,0x9d695(%rip)        # 512930 <zopro_core::preparation::WORKSPACES>
  47529b:	add    $0x188,%rsp
  4752a2:	pop    %rbx
  4752a3:	pop    %r12
  4752a5:	pop    %r13
  4752a7:	pop    %r14
  4752a9:	pop    %r15
  4752ab:	pop    %rbp
  4752ac:	vzeroupper
  4752af:	ret
  4752b0:	mov    %r14,%rdi
  4752b3:	call   *0x9b6f7(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  4752b9:	mov    0x50(%rsp),%rax
  4752be:	test   %rax,%rax
  4752c1:	jne    475098 <zopro_core::msm_affine::g2_query_exclusive+0x458>
  4752c7:	jmp    4751ae <zopro_core::msm_affine::g2_query_exclusive+0x56e>
  4752cc:	lea    0x99905(%rip),%r8        # 50ebd8 <__frame_dummy_init_array_entry+0x79f8>
  4752d3:	lea    0x30(%rsp),%rdi
  4752d8:	lea    0x50(%rsp),%rsi
  4752dd:	xor    %edx,%edx
  4752df:	call   106e62 <core::panicking::assert_failed::<usize, usize>>
  4752e4:	call   481e66 <<std::sync::once_lock::OnceLock<bool>>::initialize::<<std::sync::once_lock::OnceLock<bool>>::get_or_init<zopro_core::boolean_phase_trace::enabled::{closure#0}>::{closure#0}, !>>
  4752e9:	jmp    474f5b <zopro_core::msm_affine::g2_query_exclusive+0x31b>
  4752ee:	call   481e66 <<std::sync::once_lock::OnceLock<bool>>::initialize::<<std::sync::once_lock::OnceLock<bool>>::get_or_init<zopro_core::boolean_phase_trace::enabled::{closure#0}>::{closure#0}, !>>
  4752f3:	jmp    474fc7 <zopro_core::msm_affine::g2_query_exclusive+0x387>
  4752f8:	call   481eb7 <<std::sync::once_lock::OnceLock<std::time::Instant>>::initialize::<<std::sync::once_lock::OnceLock<std::time::Instant>>::get_or_init<<std::time::Instant>::now>::{closure#0}, !>>
  4752fd:	jmp    474f72 <zopro_core::msm_affine::g2_query_exclusive+0x332>
  475302:	call   481eb7 <<std::sync::once_lock::OnceLock<std::time::Instant>>::initialize::<<std::sync::once_lock::OnceLock<std::time::Instant>>::get_or_init<<std::time::Instant>::now>::{closure#0}, !>>
  475307:	jmp    474fde <zopro_core::msm_affine::g2_query_exclusive+0x39e>
  47530c:	call   128310 <std::process::abort>
  475311:	lea    0x38(%rsp),%rax
  475316:	mov    %rax,0x18(%rsp)
  47531b:	mov    0x18(%rsp),%rdx
  475320:	movb   $0x2,(%rdx)
  475323:	lea    -0x3ccde5(%rip),%rdi        # a8545 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x5fd>
  47532a:	lea    0x98377(%rip),%rcx        # 50d6a8 <__frame_dummy_init_array_entry+0x64c8>
  475331:	lea    0x99930(%rip),%r8        # 50ec68 <__frame_dummy_init_array_entry+0x7a88>
  475338:	mov    $0x2b,%esi
  47533d:	call   108d70 <core::result::unwrap_failed>
  475342:	ud2
  475344:	mov    0x38(%rsp),%rsi
  475349:	mov    %rax,%rbx
  47534c:	test   %rsi,%rsi
  47534f:	je     47537c <zopro_core::msm_affine::g2_query_exclusive+0x73c>
  475351:	shl    $0x7,%rsi
  475355:	mov    $0x8,%edx
  47535a:	mov    %r14,%rdi
  47535d:	call   fa0a0 <__rustc::__rust_dealloc>
  475362:	jmp    47537c <zopro_core::msm_affine::g2_query_exclusive+0x73c>
  475364:	mov    %rax,%rbx
  475367:	jmp    47537c <zopro_core::msm_affine::g2_query_exclusive+0x73c>
  475369:	mov    %rax,%rbx
  47536c:	jmp    47537c <zopro_core::msm_affine::g2_query_exclusive+0x73c>
  47536e:	cmpq   $0xffffffffffffffff,0x110(%rsp)
  475377:	mov    %rax,%rbx
  47537a:	je     475389 <zopro_core::msm_affine::g2_query_exclusive+0x749>
  47537c:	lea    0x110(%rsp),%rdi
  475384:	call   1d8920 <core::ptr::drop_glue::<zopro_core::msm_affine::grouping::ScalarGroups>>
  475389:	mov    %rbx,%rdi
  47538c:	call   506190 <_Unwind_Resume@plt>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
