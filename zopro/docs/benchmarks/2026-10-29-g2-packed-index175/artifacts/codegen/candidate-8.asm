
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000028f5f0 <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>>:
  28f5f0:	push   %rbx
  28f5f1:	mov    (%rdi),%rax
  28f5f4:	mov    %rdi,%rbx
  28f5f7:	test   %rax,%rax
  28f5fa:	je     28f632 <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0x42>
  28f5fc:	mov    0x8(%rbx),%rdi
  28f600:	cmp    $0x1fff,%rax
  28f606:	jbe    28f67f <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0x8f>
  28f608:	shl    $0x4,%rax
  28f60c:	movabs $0x7ffffffffffffff9,%rcx
  28f616:	mov    %rax,%rsi
  28f619:	or     $0x1,%rsi
  28f61d:	cmp    %rcx,%rsi
  28f620:	jae    28f6eb <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0xfb>
  28f626:	cmpb   $0x1,(%rdi,%rax,1)
  28f62a:	jne    28f67f <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0x8f>
  28f62c:	call   *0x25b616(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f632:	mov    0x18(%rbx),%rax
  28f636:	test   %rax,%rax
  28f639:	jne    28f68e <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0x9e>
  28f63b:	mov    0x30(%rbx),%rax
  28f63f:	test   %rax,%rax
  28f642:	je     28f6e2 <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0xf2>
  28f648:	mov    0x38(%rbx),%rdi
  28f64c:	cmp    $0x3fff,%rax
  28f652:	jbe    28f6e4 <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0xf4>
  28f658:	shl    $0x3,%rax
  28f65c:	movabs $0x7ffffffffffffff9,%rcx
  28f666:	mov    %rax,%rsi
  28f669:	or     $0x1,%rsi
  28f66d:	cmp    %rcx,%rsi
  28f670:	jae    28f6eb <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0xfb>
  28f672:	cmpb   $0x1,(%rdi,%rax,1)
  28f676:	jne    28f6e4 <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0xf4>
  28f678:	pop    %rbx
  28f679:	jmp    *0x25b5c9(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f67f:	call   *0x25b5cb(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f685:	mov    0x18(%rbx),%rax
  28f689:	test   %rax,%rax
  28f68c:	je     28f63b <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0x4b>
  28f68e:	mov    0x20(%rbx),%rdi
  28f692:	cmp    $0x3fff,%rax
  28f698:	jbe    28f6cf <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0xdf>
  28f69a:	shl    $0x3,%rax
  28f69e:	movabs $0x7ffffffffffffff9,%rcx
  28f6a8:	mov    %rax,%rsi
  28f6ab:	or     $0x1,%rsi
  28f6af:	cmp    %rcx,%rsi
  28f6b2:	jae    28f6eb <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0xfb>
  28f6b4:	cmpb   $0x1,(%rdi,%rax,1)
  28f6b8:	jne    28f6cf <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0xdf>
  28f6ba:	call   *0x25b588(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  28f6c0:	mov    0x30(%rbx),%rax
  28f6c4:	test   %rax,%rax
  28f6c7:	jne    28f648 <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0x58>
  28f6cd:	jmp    28f6e2 <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0xf2>
  28f6cf:	call   *0x25b57b(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f6d5:	mov    0x30(%rbx),%rax
  28f6d9:	test   %rax,%rax
  28f6dc:	jne    28f648 <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>+0x58>
  28f6e2:	pop    %rbx
  28f6e3:	ret
  28f6e4:	pop    %rbx
  28f6e5:	jmp    *0x25b565(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  28f6eb:	call   110970 <std::process::abort>
