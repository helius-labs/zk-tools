
/home/ubuntu/src/zopro-perf-work/service-trace175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004bf220 <zopro_core::service_trace::clear>:
  4bf220:	push   %rbx
  4bf221:	sub    $0x20,%rsp
  4bf225:	mov    %fs:0xffffffffffffffd8,%rax
  4bf22e:	mov    %rax,0x8(%rsp)
  4bf233:	test   %rax,%rax
  4bf236:	jne    4bf376 <zopro_core::service_trace::clear+0x156>
  4bf23c:	movzbl %fs:0xffffffffffffff88,%eax
  4bf245:	movq   $0x0,%fs:0xffffffffffffffe0
  4bf252:	test   %eax,%eax
  4bf254:	jne    4bf38e <zopro_core::service_trace::clear+0x16e>
  4bf25a:	cmpq   $0x0,%fs:0xffffffffffffff68
  4bf264:	jne    4bf3c8 <zopro_core::service_trace::clear+0x1a8>
  4bf26a:	movq   $0xffffffffffffffff,%fs:0xffffffffffffff68
  4bf277:	movq   $0x0,%fs:0xffffffffffffff80
  4bf284:	mov    %fs:0xffffffffffffff70,%rsi
  4bf28d:	cmp    $0x400,%rsi
  4bf294:	jb     4bf2a7 <zopro_core::service_trace::clear+0x87>
  4bf296:	xor    %eax,%eax
  4bf298:	mov    %rax,%fs:0xffffffffffffff68
  4bf2a1:	add    $0x20,%rsp
  4bf2a5:	pop    %rbx
  4bf2a6:	ret
  4bf2a7:	mov    %fs:0xffffffffffffff78,%rdx
  4bf2b0:	lea    0x8(%rsp),%rdi
  4bf2b5:	mov    $0x400,%ecx
  4bf2ba:	mov    $0x8,%r8d
  4bf2c0:	mov    $0x50,%r9d
  4bf2c6:	call   1e5cf0 <<alloc::raw_vec::RawVecInner>::finish_grow>
  4bf2cb:	cmpl   $0x1,0x8(%rsp)
  4bf2d0:	je     4bf3e0 <zopro_core::service_trace::clear+0x1c0>
  4bf2d6:	mov    0x10(%rsp),%rbx
  4bf2db:	mov    %rbx,%fs:0xffffffffffffff78
  4bf2e4:	movq   $0x400,%fs:0xffffffffffffff70
  4bf2f1:	mov    %fs:0xffffffffffffff80,%rax
  4bf2fa:	cmp    $0x3ff,%rax
  4bf300:	ja     4bf34b <zopro_core::service_trace::clear+0x12b>
  4bf302:	lea    (%rax,%rax,4),%rdi
  4bf306:	shl    $0x4,%rdi
  4bf30a:	add    %rbx,%rdi
  4bf30d:	cmp    $0x3ff,%rax
  4bf313:	je     4bf337 <zopro_core::service_trace::clear+0x117>
  4bf315:	shl    $0x4,%rax
  4bf319:	mov    $0x13fb0,%edx
  4bf31e:	xor    %esi,%esi
  4bf320:	lea    (%rax,%rax,4),%rax
  4bf324:	sub    %rax,%rdx
  4bf327:	call   *0x30adb(%rip)        # 4efe08 <memset@GLIBC_2.2.5>
  4bf32d:	add    $0x13fb0,%rbx
  4bf334:	mov    %rbx,%rdi
  4bf337:	vxorps %xmm0,%xmm0,%xmm0
  4bf33b:	vmovups %zmm0,0x10(%rdi)
  4bf345:	vmovups %zmm0,(%rdi)
  4bf34b:	movq   $0x0,%fs:0xffffffffffffff80
  4bf358:	mov    %fs:0xffffffffffffff68,%rax
  4bf361:	inc    %rax
  4bf364:	mov    %rax,%fs:0xffffffffffffff68
  4bf36d:	add    $0x20,%rsp
  4bf371:	pop    %rbx
  4bf372:	vzeroupper
  4bf375:	ret
  4bf376:	lea    -0x4987d5(%rip),%rsi        # 26ba8 <GCC_except_table3180+0x168>
  4bf37d:	lea    0x2d37c(%rip),%rdx        # 4ec700 <__frame_dummy_init_array_entry+0x5a78>
  4bf384:	lea    0x8(%rsp),%rdi
  4bf389:	call   f3d87 <core::panicking::assert_failed::<u64, u64>>
  4bf38e:	cmp    $0x1,%eax
  4bf391:	jne    4bf3d4 <zopro_core::service_trace::clear+0x1b4>
  4bf393:	mov    %fs:0x0,%rax
  4bf39c:	lea    -0x2d3743(%rip),%rsi        # 1ebc60 <std::sys::thread_local::native::eager::destroy::<core::cell::RefCell<alloc::vec::Vec<[u64; 10]>>>>
  4bf3a3:	lea    -0x98(%rax),%rdi
  4bf3aa:	call   13c3f0 <std::sys::thread_local::destructors::linux_like::register>
  4bf3af:	movb   $0x0,%fs:0xffffffffffffff88
  4bf3b8:	cmpq   $0x0,%fs:0xffffffffffffff68
  4bf3c2:	je     4bf26a <zopro_core::service_trace::clear+0x4a>
  4bf3c8:	lea    0x2d349(%rip),%rdi        # 4ec718 <__frame_dummy_init_array_entry+0x5a90>
  4bf3cf:	call   f5d10 <core::cell::panic_already_borrowed>
  4bf3d4:	lea    0x2a20d(%rip),%rdi        # 4e95e8 <__frame_dummy_init_array_entry+0x2960>
  4bf3db:	call   1425b0 <std::thread::local::panic_access_error>
  4bf3e0:	mov    0x10(%rsp),%rdi
  4bf3e5:	mov    0x18(%rsp),%rsi
  4bf3ea:	call   f1bb1 <alloc::raw_vec::handle_error>
  4bf3ef:	ud2
  4bf3f1:	incq   %fs:0xffffffffffffff68
  4bf3fa:	mov    %rax,%rdi
  4bf3fd:	call   4e5c10 <_Unwind_Resume@plt>
