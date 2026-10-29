
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001d1670 <zopro_core::service_trace::scope>:
  1d1670:	push   %rbp
  1d1671:	push   %r15
  1d1673:	push   %r14
  1d1675:	push   %r13
  1d1677:	push   %r12
  1d1679:	push   %rbx
  1d167a:	sub    $0x58,%rsp
  1d167e:	movzbl 0x31f4f3(%rip),%eax        # 4f0b78 <zopro_core::service_trace::ENABLED>
  1d1685:	mov    %rdi,%rbx
  1d1688:	test   %al,%al
  1d168a:	je     1d16a9 <zopro_core::service_trace::scope+0x39>
  1d168c:	mov    %fs:0xffffffffffffffb0,%rax
  1d1695:	mov    %r9d,%ebp
  1d1698:	test   %rax,%rax
  1d169b:	je     1d16be <zopro_core::service_trace::scope+0x4e>
  1d169d:	mov    0x100(%rax),%r15
  1d16a4:	inc    %r15
  1d16a7:	jmp    1d16c1 <zopro_core::service_trace::scope+0x51>
  1d16a9:	movq   $0x0,(%rbx)
  1d16b0:	vxorps %xmm0,%xmm0,%xmm0
  1d16b4:	vmovups %xmm0,0x10(%rbx)
  1d16b9:	jmp    1d17b6 <zopro_core::service_trace::scope+0x146>
  1d16be:	xor    %r15d,%r15d
  1d16c1:	mov    %fs:0x0,%rax
  1d16ca:	mov    %fs:0xffffffffffffffd8,%rdi
  1d16d3:	lea    -0x98(%rax),%r13
  1d16da:	movzbl %fs:0xffffffffffffff88,%eax
  1d16e3:	test   %eax,%eax
  1d16e5:	jne    1d18e8 <zopro_core::service_trace::scope+0x278>
  1d16eb:	movabs $0x7fffffffffffffff,%rax
  1d16f5:	cmp    %rax,%fs:0xffffffffffffff68
  1d16fe:	jae    1d194a <zopro_core::service_trace::scope+0x2da>
  1d1704:	cmpq   $0x400,%fs:0xffffffffffffff80
  1d1711:	jae    1d179a <zopro_core::service_trace::scope+0x12a>
  1d1717:	mov    0x31f303(%rip),%eax        # 4f0a20 <zopro_core::service_trace::CLOCK+0x10>
  1d171d:	test   %eax,%eax
  1d171f:	jne    1d1956 <zopro_core::service_trace::scope+0x2e6>
  1d1725:	mov    %rdi,(%rsp)
  1d1729:	lea    0x31f2e0(%rip),%rdi        # 4f0a10 <zopro_core::service_trace::CLOCK>
  1d1730:	mov    %r13,0x28(%rsp)
  1d1735:	mov    %rsi,0x8(%rsp)
  1d173a:	mov    %rdx,0x10(%rsp)
  1d173f:	mov    %rcx,%r14
  1d1742:	mov    %r8,%r13
  1d1745:	call   13d820 <<std::time::Instant>::elapsed>
  1d174a:	mov    %rax,0x48(%rsp)
  1d174f:	mov    %edx,0x20(%rsp)
  1d1753:	mov    0x31f416(%rip),%rax        # 4f0b70 <zopro_core::service_trace::ORIGIN>
  1d175a:	mov    %rax,0x50(%rsp)
  1d175f:	test   %bpl,%bpl
  1d1762:	je     1d17bf <zopro_core::service_trace::scope+0x14f>
  1d1764:	lea    0x30(%rsp),%rsi
  1d1769:	mov    $0x3,%edi
  1d176e:	vxorps %xmm0,%xmm0,%xmm0
  1d1772:	vmovaps %xmm0,0x30(%rsp)
  1d1778:	call   *0x31e7aa(%rip)        # 4eff28 <clock_gettime@GLIBC_2.17>
  1d177e:	mov    %eax,0x24(%rsp)
  1d1782:	test   %eax,%eax
  1d1784:	jne    1d198d <zopro_core::service_trace::scope+0x31d>
  1d178a:	imul   $0x3b9aca00,0x30(%rsp),%r12
  1d1793:	add    0x38(%rsp),%r12
  1d1798:	jmp    1d17c2 <zopro_core::service_trace::scope+0x152>
  1d179a:	incq   %fs:0xffffffffffffffe0
  1d17a3:	movq   $0x0,(%rbx)
  1d17aa:	movq   $0x0,0x10(%rbx)
  1d17b2:	mov    %rdi,0x18(%rbx)
  1d17b6:	movb   $0x0,0x20(%rbx)
  1d17ba:	jmp    1d18d9 <zopro_core::service_trace::scope+0x269>
  1d17bf:	xor    %r12d,%r12d
  1d17c2:	movzbl %fs:0xffffffffffffff88,%eax
  1d17cb:	test   %eax,%eax
  1d17cd:	jne    1d197c <zopro_core::service_trace::scope+0x30c>
  1d17d3:	cmpq   $0x0,%fs:0xffffffffffffff68
  1d17dd:	jne    1d19cf <zopro_core::service_trace::scope+0x35f>
  1d17e3:	movq   $0xffffffffffffffff,%fs:0xffffffffffffff68
  1d17f0:	mov    %fs:0xffffffffffffff70,%rax
  1d17f9:	cmp    $0x3ff,%rax
  1d17ff:	jbe    1d1962 <zopro_core::service_trace::scope+0x2f2>
  1d1805:	mov    %ebp,0x1c(%rsp)
  1d1809:	mov    %r13,%rbp
  1d180c:	mov    %fs:0xffffffffffffff80,%r13
  1d1815:	cmp    %rax,%r13
  1d1818:	jne    1d1828 <zopro_core::service_trace::scope+0x1b8>
  1d181a:	mov    0x28(%rsp),%rdi
  1d181f:	add    $0x8,%rdi
  1d1823:	call   1ebcb0 <<alloc::raw_vec::RawVec<[u64; 10]>>::grow_one>
  1d1828:	imul   $0x3b9aca00,0x48(%rsp),%rax
  1d1831:	mov    0x20(%rsp),%ecx
  1d1835:	mov    %fs:0xffffffffffffff78,%rsi
  1d183e:	mov    0x8(%rsp),%r8
  1d1843:	mov    0x10(%rsp),%r9
  1d1848:	lea    0x0(%r13,%r13,4),%rdi
  1d184d:	mov    %r15,%r10
  1d1850:	shl    $0x20,%r10
  1d1854:	lea    0x1(%r13),%rdx
  1d1858:	or     %rdx,%r10
  1d185b:	shl    $0x4,%rdi
  1d185f:	mov    %r10,(%rsi,%rdi,1)
  1d1863:	mov    %r8,0x8(%rsi,%rdi,1)
  1d1868:	mov    %r9,0x10(%rsi,%rdi,1)
  1d186d:	mov    %r14,0x18(%rsi,%rdi,1)
  1d1872:	mov    %rbp,0x20(%rsi,%rdi,1)
  1d1877:	add    %rax,%rcx
  1d187a:	sub    0x50(%rsp),%rcx
  1d187f:	mov    (%rsp),%rax
  1d1883:	mov    %rcx,0x28(%rsi,%rdi,1)
  1d1888:	movq   $0x0,0x30(%rsi,%rdi,1)
  1d1891:	mov    %r15,0x38(%rsi,%rdi,1)
  1d1896:	mov    %r12,0x40(%rsi,%rdi,1)
  1d189b:	movq   $0x0,0x48(%rsi,%rdi,1)
  1d18a4:	mov    %rdx,%fs:0xffffffffffffff80
  1d18ad:	incq   %fs:0xffffffffffffff68
  1d18b6:	movq   $0x1,(%rbx)
  1d18bd:	mov    %r13,0x8(%rbx)
  1d18c1:	mov    %r10,0x10(%rbx)
  1d18c5:	mov    %rax,0x18(%rbx)
  1d18c9:	mov    0x1c(%rsp),%eax
  1d18cd:	mov    %r10,%fs:0xffffffffffffffd8
  1d18d6:	mov    %al,0x20(%rbx)
  1d18d9:	add    $0x58,%rsp
  1d18dd:	pop    %rbx
  1d18de:	pop    %r12
  1d18e0:	pop    %r13
  1d18e2:	pop    %r14
  1d18e4:	pop    %r15
  1d18e6:	pop    %rbp
  1d18e7:	ret
  1d18e8:	mov    %rdi,(%rsp)
  1d18ec:	mov    %rsi,0x8(%rsp)
  1d18f1:	mov    %rdx,0x10(%rsp)
  1d18f6:	cmp    $0x1,%eax
  1d18f9:	jne    1d1981 <zopro_core::service_trace::scope+0x311>
  1d18ff:	lea    0x1a35a(%rip),%rsi        # 1ebc60 <std::sys::thread_local::native::eager::destroy::<core::cell::RefCell<alloc::vec::Vec<[u64; 10]>>>>
  1d1906:	mov    %r13,%rdi
  1d1909:	mov    %rcx,%r14
  1d190c:	mov    %r8,%r12
  1d190f:	call   13c3f0 <std::sys::thread_local::destructors::linux_like::register>
  1d1914:	mov    0x10(%rsp),%rdx
  1d1919:	mov    0x8(%rsp),%rsi
  1d191e:	mov    (%rsp),%rdi
  1d1922:	movb   $0x0,%fs:0xffffffffffffff88
  1d192b:	mov    %r12,%r8
  1d192e:	mov    %r14,%rcx
  1d1931:	movabs $0x7fffffffffffffff,%rax
  1d193b:	cmp    %rax,%fs:0xffffffffffffff68
  1d1944:	jb     1d1704 <zopro_core::service_trace::scope+0x94>
  1d194a:	lea    0x31ae0f(%rip),%rdi        # 4ec760 <__frame_dummy_init_array_entry+0x5ad8>
  1d1951:	call   f5d60 <core::cell::panic_already_mutably_borrowed>
  1d1956:	lea    0x31c77b(%rip),%rdi        # 4ee0d8 <__frame_dummy_init_array_entry+0x7450>
  1d195d:	call   f5cf0 <core::option::unwrap_failed>
  1d1962:	lea    -0x196596(%rip),%rdi        # 3b3d3 <zmij::STATIC_DATA+0x54d3>
  1d1969:	lea    0x31adc0(%rip),%rdx        # 4ec730 <__frame_dummy_init_array_entry+0x5aa8>
  1d1970:	mov    $0x27,%esi
  1d1975:	call   f49d0 <core::panicking::panic>
  1d197a:	ud2
  1d197c:	cmp    $0x2,%eax
  1d197f:	jne    1d19a5 <zopro_core::service_trace::scope+0x335>
  1d1981:	lea    0x317c60(%rip),%rdi        # 4e95e8 <__frame_dummy_init_array_entry+0x2960>
  1d1988:	call   1425b0 <std::thread::local::panic_access_error>
  1d198d:	lea    -0x1a9b64(%rip),%rsi        # 27e30 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x100>
  1d1994:	lea    0x31c725(%rip),%rdx        # 4ee0c0 <__frame_dummy_init_array_entry+0x7438>
  1d199b:	lea    0x24(%rsp),%rdi
  1d19a0:	call   13cdd1 <core::panicking::assert_failed::<i32, i32>>
  1d19a5:	mov    0x28(%rsp),%rdi
  1d19aa:	lea    0x1a2af(%rip),%rsi        # 1ebc60 <std::sys::thread_local::native::eager::destroy::<core::cell::RefCell<alloc::vec::Vec<[u64; 10]>>>>
  1d19b1:	call   13c3f0 <std::sys::thread_local::destructors::linux_like::register>
  1d19b6:	movb   $0x0,%fs:0xffffffffffffff88
  1d19bf:	cmpq   $0x0,%fs:0xffffffffffffff68
  1d19c9:	je     1d17e3 <zopro_core::service_trace::scope+0x173>
  1d19cf:	lea    0x31ad72(%rip),%rdi        # 4ec748 <__frame_dummy_init_array_entry+0x5ac0>
  1d19d6:	call   f5d10 <core::cell::panic_already_borrowed>
  1d19db:	incq   %fs:0xffffffffffffff68
  1d19e4:	mov    %rax,%rdi
  1d19e7:	call   4e5c10 <_Unwind_Resume@plt>
