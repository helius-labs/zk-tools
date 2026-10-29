
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004be670 <zopro_core::service_trace::api>:
  4be670:	push   %rbx
  4be671:	sub    $0x30,%rsp
  4be675:	mov    %rdi,%rbx
  4be678:	lea    0x8(%rsp),%rdi
  4be67d:	xor    %esi,%esi
  4be67f:	xor    %edx,%edx
  4be681:	xor    %ecx,%ecx
  4be683:	xor    %r8d,%r8d
  4be686:	xor    %r9d,%r9d
  4be689:	call   1d1670 <zopro_core::service_trace::scope>
  4be68e:	mov    0x18(%rsp),%rax
  4be693:	mov    %rax,0x324ce(%rip)        # 4f0b68 <zopro_core::service_trace::API>
  4be69a:	vmovups 0x8(%rsp),%ymm0
  4be6a0:	mov    0x28(%rsp),%rax
  4be6a5:	mov    %rax,0x20(%rbx)
  4be6a9:	vmovups %ymm0,(%rbx)
  4be6ad:	add    $0x30,%rsp
  4be6b1:	pop    %rbx
  4be6b2:	vzeroupper
  4be6b5:	ret
