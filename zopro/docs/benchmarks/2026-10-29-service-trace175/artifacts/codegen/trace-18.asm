
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001d19f0 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop>:
  1d19f0:	push   %rbp
  1d19f1:	push   %r15
  1d19f3:	push   %r14
  1d19f5:	push   %r13
  1d19f7:	push   %r12
  1d19f9:	push   %rbx
  1d19fa:	sub    $0x28,%rsp
  1d19fe:	cmpl   $0x1,(%rdi)
  1d1a01:	jne    1d1b28 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x138>
  1d1a07:	mov    0x8(%rdi),%r14
  1d1a0b:	cmpb   $0x0,0x20(%rdi)
  1d1a0f:	mov    %rdi,%rbx
  1d1a12:	je     1d1a4a <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x5a>
  1d1a14:	lea    0x10(%rsp),%rsi
  1d1a19:	mov    $0x3,%edi
  1d1a1e:	vxorps %xmm0,%xmm0,%xmm0
  1d1a22:	vmovaps %xmm0,0x10(%rsp)
  1d1a28:	call   *0x31e4fa(%rip)        # 4eff28 <clock_gettime@GLIBC_2.17>
  1d1a2e:	mov    %eax,0xc(%rsp)
  1d1a32:	test   %eax,%eax
  1d1a34:	jne    1d1bc3 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x1d3>
  1d1a3a:	imul   $0x3b9aca00,0x10(%rsp),%r15
  1d1a43:	add    0x18(%rsp),%r15
  1d1a48:	jmp    1d1a4d <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x5d>
  1d1a4a:	xor    %r15d,%r15d
  1d1a4d:	mov    0x31efcd(%rip),%eax        # 4f0a20 <zopro_core::service_trace::CLOCK+0x10>
  1d1a53:	test   %eax,%eax
  1d1a55:	jne    1d1b37 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x147>
  1d1a5b:	lea    0x31efae(%rip),%rdi        # 4f0a10 <zopro_core::service_trace::CLOCK>
  1d1a62:	call   13d820 <<std::time::Instant>::elapsed>
  1d1a67:	mov    0x31f102(%rip),%r12        # 4f0b70 <zopro_core::service_trace::ORIGIN>
  1d1a6e:	movzbl %fs:0xffffffffffffff88,%ecx
  1d1a77:	test   %ecx,%ecx
  1d1a79:	jne    1d1b73 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x183>
  1d1a7f:	cmpq   $0x0,%fs:0xffffffffffffff68
  1d1a89:	jne    1d1bb7 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x1c7>
  1d1a8f:	movq   $0xffffffffffffffff,%fs:0xffffffffffffff68
  1d1a9c:	mov    %fs:0xffffffffffffff80,%rsi
  1d1aa5:	cmp    %rsi,%r14
  1d1aa8:	jae    1d1be7 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x1f7>
  1d1aae:	mov    %fs:0xffffffffffffff78,%rsi
  1d1ab7:	lea    (%r14,%r14,4),%rcx
  1d1abb:	shl    $0x4,%rcx
  1d1abf:	cmpq   $0x0,0x30(%rsi,%rcx,1)
  1d1ac5:	lea    0x30(%rsi,%rcx,1),%rdi
  1d1aca:	jne    1d1b43 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x153>
  1d1acc:	imul   $0x3b9aca00,%rax,%rax
  1d1ad3:	mov    %edx,%edx
  1d1ad5:	add    %rax,%rdx
  1d1ad8:	sub    %r12,%rdx
  1d1adb:	mov    %rdx,(%rdi)
  1d1ade:	mov    %fs:0xffffffffffffff80,%rsi
  1d1ae7:	cmp    %rsi,%r14
  1d1aea:	jae    1d1bf0 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x200>
  1d1af0:	mov    %fs:0xffffffffffffff78,%rax
  1d1af9:	mov    %r15,0x48(%rax,%rcx,1)
  1d1afe:	incq   %fs:0xffffffffffffff68
  1d1b07:	mov    %fs:0xffffffffffffffd8,%rcx
  1d1b10:	mov    0x18(%rbx),%rax
  1d1b14:	mov    %rcx,0x10(%rsp)
  1d1b19:	cmp    0x10(%rbx),%rcx
  1d1b1d:	jne    1d1b5b <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x16b>
  1d1b1f:	mov    %rax,%fs:0xffffffffffffffd8
  1d1b28:	add    $0x28,%rsp
  1d1b2c:	pop    %rbx
  1d1b2d:	pop    %r12
  1d1b2f:	pop    %r13
  1d1b31:	pop    %r14
  1d1b33:	pop    %r15
  1d1b35:	pop    %rbp
  1d1b36:	ret
  1d1b37:	lea    0x31c59a(%rip),%rdi        # 4ee0d8 <__frame_dummy_init_array_entry+0x7450>
  1d1b3e:	call   f5cf0 <core::option::unwrap_failed>
  1d1b43:	lea    -0x1aafa2(%rip),%rsi        # 26ba8 <GCC_except_table3180+0x168>
  1d1b4a:	lea    0x31ad37(%rip),%rdx        # 4ec888 <__frame_dummy_init_array_entry+0x5c00>
  1d1b51:	call   f3d87 <core::panicking::assert_failed::<u64, u64>>
  1d1b56:	jmp    1d1bff <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x20f>
  1d1b5b:	add    $0x10,%rbx
  1d1b5f:	lea    0x31ad52(%rip),%rdx        # 4ec8b8 <__frame_dummy_init_array_entry+0x5c30>
  1d1b66:	lea    0x10(%rsp),%rdi
  1d1b6b:	mov    %rbx,%rsi
  1d1b6e:	call   f3d87 <core::panicking::assert_failed::<u64, u64>>
  1d1b73:	cmp    $0x1,%ecx
  1d1b76:	jne    1d1bdb <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x1eb>
  1d1b78:	mov    %fs:0x0,%rcx
  1d1b81:	lea    0x1a0d8(%rip),%rsi        # 1ebc60 <std::sys::thread_local::native::eager::destroy::<core::cell::RefCell<alloc::vec::Vec<[u64; 10]>>>>
  1d1b88:	mov    %edx,%ebp
  1d1b8a:	mov    %rax,%r13
  1d1b8d:	lea    -0x98(%rcx),%rdi
  1d1b94:	call   13c3f0 <std::sys::thread_local::destructors::linux_like::register>
  1d1b99:	movb   $0x0,%fs:0xffffffffffffff88
  1d1ba2:	mov    %r13,%rax
  1d1ba5:	mov    %ebp,%edx
  1d1ba7:	cmpq   $0x0,%fs:0xffffffffffffff68
  1d1bb1:	je     1d1a8f <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x9f>
  1d1bb7:	lea    0x31ace2(%rip),%rdi        # 4ec8a0 <__frame_dummy_init_array_entry+0x5c18>
  1d1bbe:	call   f5d10 <core::cell::panic_already_borrowed>
  1d1bc3:	lea    -0x1a9d9a(%rip),%rsi        # 27e30 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x100>
  1d1bca:	lea    0x31c4ef(%rip),%rdx        # 4ee0c0 <__frame_dummy_init_array_entry+0x7438>
  1d1bd1:	lea    0xc(%rsp),%rdi
  1d1bd6:	call   13cdd1 <core::panicking::assert_failed::<i32, i32>>
  1d1bdb:	lea    0x317a06(%rip),%rdi        # 4e95e8 <__frame_dummy_init_array_entry+0x2960>
  1d1be2:	call   1425b0 <std::thread::local::panic_access_error>
  1d1be7:	lea    0x31ac6a(%rip),%rdx        # 4ec858 <__frame_dummy_init_array_entry+0x5bd0>
  1d1bee:	jmp    1d1bf7 <<zopro_core::service_trace::Scope as core::ops::drop::Drop>::drop+0x207>
  1d1bf0:	lea    0x31ac79(%rip),%rdx        # 4ec870 <__frame_dummy_init_array_entry+0x5be8>
  1d1bf7:	mov    %r14,%rdi
  1d1bfa:	call   f39ab <core::panicking::panic_bounds_check>
  1d1bff:	ud2
  1d1c01:	incq   %fs:0xffffffffffffff68
  1d1c0a:	mov    %rax,%rdi
  1d1c0d:	call   4e5c10 <_Unwind_Resume@plt>
