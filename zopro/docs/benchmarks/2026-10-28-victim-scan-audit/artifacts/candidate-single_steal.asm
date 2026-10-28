
/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000000fe280 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal>:
   fe280:	push   r15
   fe282:	push   r14
   fe284:	push   rbx
   fe285:	mov    r15,QWORD PTR [rsi]
   fe288:	mov    rbx,rdi
   fe28b:	mov    r14,QWORD PTR [r15+0x100]
   fe292:	cmp    BYTE PTR fs:0xffffffffffffffc8,0x1
   fe29b:	jne    fe36c <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xec>
   fe2a1:	mov    rax,QWORD PTR fs:0x0
   fe2aa:	lea    rax,[rax-0x40]
   fe2b1:	mov    rax,QWORD PTR [rax]
   fe2b4:	mov    rcx,QWORD PTR [rax+0x818]
   fe2bb:	test   rcx,rcx
   fe2be:	je     fe2c6 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x46>
   fe2c0:	lock or DWORD PTR [rsp-0x40],0x0
   fe2c6:	call   ff5c0 <crossbeam_epoch::default::with_handle::<crossbeam_epoch::default::pin::{closure#0}, crossbeam_epoch::guard::Guard>>
   fe2cb:	mov    rcx,QWORD PTR [r15+0x108]
   fe2d2:	mov    rdi,rax
   fe2d5:	sub    rcx,r14
   fe2d8:	test   rcx,rcx
   fe2db:	jle    fe33e <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xbe>
   fe2dd:	mov    rax,QWORD PTR [r15+0x80]
   fe2e4:	mov    rcx,rax
   fe2e7:	and    rcx,0xfffffffffffffff8
   fe2eb:	mov    rsi,QWORD PTR [rcx]
   fe2ee:	mov    rcx,QWORD PTR [rcx+0x8]
   fe2f2:	dec    rcx
   fe2f5:	and    rcx,r14
   fe2f8:	shl    rcx,0x4
   fe2fc:	mov    rdx,QWORD PTR [rsi+rcx*1]
   fe300:	mov    rsi,QWORD PTR [rsi+rcx*1+0x8]
   fe305:	mov    r8,QWORD PTR [r15+0x80]
   fe30c:	mov    ecx,0x2
   fe311:	cmp    r8,rax
   fe314:	jne    fe340 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xc0>
   fe316:	lea    r8,[r14+0x1]
   fe31a:	mov    rax,r14
   fe31d:	lock cmpxchg QWORD PTR [r15+0x100],r8
   fe326:	jne    fe340 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xc0>
   fe328:	mov    QWORD PTR [rbx+0x8],rdx
   fe32c:	mov    QWORD PTR [rbx+0x10],rsi
   fe330:	mov    QWORD PTR [rbx],0x1
   fe337:	test   rdi,rdi
   fe33a:	jne    fe348 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xc8>
   fe33c:	jmp    fe366 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xe6>
   fe33e:	xor    ecx,ecx
   fe340:	mov    QWORD PTR [rbx],rcx
   fe343:	test   rdi,rdi
   fe346:	je     fe366 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xe6>
   fe348:	dec    QWORD PTR [rdi+0x818]
   fe34f:	jne    fe366 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xe6>
   fe351:	mov    QWORD PTR [rdi+0x880],0x0
   fe35c:	cmp    QWORD PTR [rdi+0x820],0x0
   fe364:	je     fe3d3 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x153>
   fe366:	pop    rbx
   fe367:	pop    r14
   fe369:	pop    r15
   fe36b:	ret
   fe36c:	mov    rax,QWORD PTR fs:0x0
   fe375:	lea    rdi,[rax-0x40]
   fe37c:	call   ff4e0 <<std::sys::thread_local::native::lazy::Storage<crossbeam_epoch::collector::LocalHandle, ()>>::get_or_init_slow::<crossbeam_epoch::default::HANDLE::__rust_std_internal_init_fn>>
   fe381:	test   rax,rax
   fe384:	jne    fe2b1 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x31>
   fe38a:	mov    eax,DWORD PTR [rip+0x3d9980]        # 4d7d10 <crossbeam_epoch::default::collector::COLLECTOR+0x8>
   fe390:	test   eax,eax
   fe392:	jne    fe3dd <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x15d>
   fe394:	mov    rdi,QWORD PTR [rip+0x3d996d]        # 4d7d08 <crossbeam_epoch::default::collector::COLLECTOR>
   fe39b:	call   f2fb0 <<crossbeam_epoch::collector::Collector>::register>
   fe3a0:	mov    rdx,QWORD PTR [rax+0x820]
   fe3a7:	mov    rcx,QWORD PTR [rax+0x818]
   fe3ae:	lea    rsi,[rdx-0x1]
   fe3b2:	xor    rdx,0x1
   fe3b6:	or     rdx,rcx
   fe3b9:	mov    QWORD PTR [rax+0x820],rsi
   fe3c0:	jne    fe2bb <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x3b>
   fe3c6:	mov    rdi,rax
   fe3c9:	call   f3200 <<crossbeam_epoch::internal::Local>::finalize>
   fe3ce:	jmp    fe2c6 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x46>
   fe3d3:	pop    rbx
   fe3d4:	pop    r14
   fe3d6:	pop    r15
   fe3d8:	jmp    f3200 <<crossbeam_epoch::internal::Local>::finalize>
   fe3dd:	call   f35d0 <<crossbeam_epoch::sync::once_lock::OnceLock<crossbeam_epoch::collector::Collector>>::initialize::<<crossbeam_epoch::collector::Collector>::new>>
   fe3e2:	jmp    fe394 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x114>
