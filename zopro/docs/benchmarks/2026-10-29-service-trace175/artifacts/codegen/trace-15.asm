
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004c03c0 <zopro_core::service_trace::snapshot::{closure#0}>:
  4c03c0:	push   %r15
  4c03c2:	push   %r14
  4c03c4:	push   %r13
  4c03c6:	push   %r12
  4c03c8:	push   %rbx
  4c03c9:	sub    $0x10,%rsp
  4c03cd:	mov    %fs:0xffffffffffffffd8,%rax
  4c03d6:	mov    %rax,0x8(%rsp)
  4c03db:	test   %rax,%rax
  4c03de:	jne    4c04b6 <zopro_core::service_trace::snapshot::{closure#0}+0xf6>
  4c03e4:	movzbl %fs:0xffffffffffffff88,%eax
  4c03ed:	mov    %rdi,%rbx
  4c03f0:	test   %eax,%eax
  4c03f2:	jne    4c04ce <zopro_core::service_trace::snapshot::{closure#0}+0x10e>
  4c03f8:	mov    %fs:0xffffffffffffff68,%rax
  4c0401:	movabs $0x7fffffffffffffff,%rcx
  4c040b:	cmp    %rcx,%rax
  4c040e:	jae    4c0514 <zopro_core::service_trace::snapshot::{closure#0}+0x154>
  4c0414:	lea    0x1(%rax),%rcx
  4c0418:	mov    %rcx,%fs:0xffffffffffffff68
  4c0421:	mov    %fs:0xffffffffffffff80,%r13
  4c042a:	test   %r13,%r13
  4c042d:	je     4c0481 <zopro_core::service_trace::snapshot::{closure#0}+0xc1>
  4c042f:	mov    %fs:0xffffffffffffff78,%r12
  4c0438:	mov    %r13,%rax
  4c043b:	shl    $0x4,%rax
  4c043f:	mov    $0x8,%edi
  4c0444:	xor    %edx,%edx
  4c0446:	lea    (%rax,%rax,4),%r15
  4c044a:	mov    %r15,%rsi
  4c044d:	call   4c0830 <<zopro_core::alloc::linux::Allocator>::allocate>
  4c0452:	test   %rax,%rax
  4c0455:	je     4c052c <zopro_core::service_trace::snapshot::{closure#0}+0x16c>
  4c045b:	mov    %rax,%r14
  4c045e:	mov    %rax,%rdi
  4c0461:	mov    %r12,%rsi
  4c0464:	mov    %r15,%rdx
  4c0467:	call   *0x2f8eb(%rip)        # 4efd58 <memcpy@GLIBC_2.14>
  4c046d:	decq   %fs:0xffffffffffffff68
  4c0476:	cmp    $0xffffffffffffffff,%r13
  4c047a:	jne    4c0490 <zopro_core::service_trace::snapshot::{closure#0}+0xd0>
  4c047c:	jmp    4c0520 <zopro_core::service_trace::snapshot::{closure#0}+0x160>
  4c0481:	mov    %rax,%fs:0xffffffffffffff68
  4c048a:	mov    $0x8,%r14d
  4c0490:	mov    %fs:0xffffffffffffffe0,%rax
  4c0499:	mov    %r13,(%rbx)
  4c049c:	mov    %r14,0x8(%rbx)
  4c04a0:	mov    %r13,0x10(%rbx)
  4c04a4:	mov    %rax,0x18(%rbx)
  4c04a8:	add    $0x10,%rsp
  4c04ac:	pop    %rbx
  4c04ad:	pop    %r12
  4c04af:	pop    %r13
  4c04b1:	pop    %r14
  4c04b3:	pop    %r15
  4c04b5:	ret
  4c04b6:	lea    -0x499915(%rip),%rsi        # 26ba8 <GCC_except_table3180+0x168>
  4c04bd:	lea    0x2bd44(%rip),%rdx        # 4ec208 <__frame_dummy_init_array_entry+0x5580>
  4c04c4:	lea    0x8(%rsp),%rdi
  4c04c9:	call   f3d87 <core::panicking::assert_failed::<u64, u64>>
  4c04ce:	cmp    $0x1,%eax
  4c04d1:	jne    4c0520 <zopro_core::service_trace::snapshot::{closure#0}+0x160>
  4c04d3:	mov    %fs:0x0,%rax
  4c04dc:	lea    -0x2d4883(%rip),%rsi        # 1ebc60 <std::sys::thread_local::native::eager::destroy::<core::cell::RefCell<alloc::vec::Vec<[u64; 10]>>>>
  4c04e3:	lea    -0x98(%rax),%rdi
  4c04ea:	call   13c3f0 <std::sys::thread_local::destructors::linux_like::register>
  4c04ef:	movb   $0x0,%fs:0xffffffffffffff88
  4c04f8:	mov    %fs:0xffffffffffffff68,%rax
  4c0501:	movabs $0x7fffffffffffffff,%rcx
  4c050b:	cmp    %rcx,%rax
  4c050e:	jb     4c0414 <zopro_core::service_trace::snapshot::{closure#0}+0x54>
  4c0514:	lea    0x2bd05(%rip),%rdi        # 4ec220 <__frame_dummy_init_array_entry+0x5598>
  4c051b:	call   f5d60 <core::cell::panic_already_mutably_borrowed>
  4c0520:	lea    0x290c1(%rip),%rdi        # 4e95e8 <__frame_dummy_init_array_entry+0x2960>
  4c0527:	call   1425b0 <std::thread::local::panic_access_error>
  4c052c:	mov    $0x8,%edi
  4c0531:	mov    %r15,%rsi
  4c0534:	call   f1bb1 <alloc::raw_vec::handle_error>
  4c0539:	ud2
  4c053b:	decq   %fs:0xffffffffffffff68
  4c0544:	mov    %rax,%rdi
  4c0547:	call   4e5c10 <_Unwind_Resume@plt>
