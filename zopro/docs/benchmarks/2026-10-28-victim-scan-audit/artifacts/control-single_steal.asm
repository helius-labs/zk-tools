
/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000000fe7b0 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal>:
   fe7b0:	push   r15
   fe7b2:	push   r14
   fe7b4:	push   rbx
   fe7b5:	mov    r15,QWORD PTR [rsi]
   fe7b8:	mov    rbx,rdi
   fe7bb:	mov    r14,QWORD PTR [r15+0x100]
   fe7c2:	cmp    BYTE PTR fs:0xffffffffffffffd0,0x1
   fe7cb:	jne    fe89c <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xec>
   fe7d1:	mov    rax,QWORD PTR fs:0x0
   fe7da:	lea    rax,[rax-0x38]
   fe7e1:	mov    rax,QWORD PTR [rax]
   fe7e4:	mov    rcx,QWORD PTR [rax+0x818]
   fe7eb:	test   rcx,rcx
   fe7ee:	je     fe7f6 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x46>
   fe7f0:	lock or DWORD PTR [rsp-0x40],0x0
   fe7f6:	call   fe9d0 <crossbeam_epoch::default::with_handle::<crossbeam_epoch::default::pin::{closure#0}, crossbeam_epoch::guard::Guard>>
   fe7fb:	mov    rcx,QWORD PTR [r15+0x108]
   fe802:	mov    rdi,rax
   fe805:	sub    rcx,r14
   fe808:	test   rcx,rcx
   fe80b:	jle    fe86e <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xbe>
   fe80d:	mov    rax,QWORD PTR [r15+0x80]
   fe814:	mov    rcx,rax
   fe817:	and    rcx,0xfffffffffffffff8
   fe81b:	mov    rsi,QWORD PTR [rcx]
   fe81e:	mov    rcx,QWORD PTR [rcx+0x8]
   fe822:	dec    rcx
   fe825:	and    rcx,r14
   fe828:	shl    rcx,0x4
   fe82c:	mov    rdx,QWORD PTR [rsi+rcx*1]
   fe830:	mov    rsi,QWORD PTR [rsi+rcx*1+0x8]
   fe835:	mov    r8,QWORD PTR [r15+0x80]
   fe83c:	mov    ecx,0x2
   fe841:	cmp    r8,rax
   fe844:	jne    fe870 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xc0>
   fe846:	lea    r8,[r14+0x1]
   fe84a:	mov    rax,r14
   fe84d:	lock cmpxchg QWORD PTR [r15+0x100],r8
   fe856:	jne    fe870 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xc0>
   fe858:	mov    QWORD PTR [rbx+0x8],rdx
   fe85c:	mov    QWORD PTR [rbx+0x10],rsi
   fe860:	mov    QWORD PTR [rbx],0x1
   fe867:	test   rdi,rdi
   fe86a:	jne    fe878 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xc8>
   fe86c:	jmp    fe896 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xe6>
   fe86e:	xor    ecx,ecx
   fe870:	mov    QWORD PTR [rbx],rcx
   fe873:	test   rdi,rdi
   fe876:	je     fe896 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xe6>
   fe878:	dec    QWORD PTR [rdi+0x818]
   fe87f:	jne    fe896 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0xe6>
   fe881:	mov    QWORD PTR [rdi+0x880],0x0
   fe88c:	cmp    QWORD PTR [rdi+0x820],0x0
   fe894:	je     fe903 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x153>
   fe896:	pop    rbx
   fe897:	pop    r14
   fe899:	pop    r15
   fe89b:	ret
   fe89c:	mov    rax,QWORD PTR fs:0x0
   fe8a5:	lea    rdi,[rax-0x38]
   fe8ac:	call   fe920 <<std::sys::thread_local::native::lazy::Storage<crossbeam_epoch::collector::LocalHandle, ()>>::get_or_init_slow::<crossbeam_epoch::default::HANDLE::__rust_std_internal_init_fn>>
   fe8b1:	test   rax,rax
   fe8b4:	jne    fe7e1 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x31>
   fe8ba:	mov    eax,DWORD PTR [rip+0x3d8860]        # 4d7120 <crossbeam_epoch::default::collector::COLLECTOR+0x8>
   fe8c0:	test   eax,eax
   fe8c2:	jne    fe90d <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x15d>
   fe8c4:	mov    rdi,QWORD PTR [rip+0x3d884d]        # 4d7118 <crossbeam_epoch::default::collector::COLLECTOR>
   fe8cb:	call   f3020 <<crossbeam_epoch::collector::Collector>::register>
   fe8d0:	mov    rdx,QWORD PTR [rax+0x820]
   fe8d7:	mov    rcx,QWORD PTR [rax+0x818]
   fe8de:	lea    rsi,[rdx-0x1]
   fe8e2:	xor    rdx,0x1
   fe8e6:	or     rdx,rcx
   fe8e9:	mov    QWORD PTR [rax+0x820],rsi
   fe8f0:	jne    fe7eb <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x3b>
   fe8f6:	mov    rdi,rax
   fe8f9:	call   f3270 <<crossbeam_epoch::internal::Local>::finalize>
   fe8fe:	jmp    fe7f6 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x46>
   fe903:	pop    rbx
   fe904:	pop    r14
   fe906:	pop    r15
   fe908:	jmp    f3270 <<crossbeam_epoch::internal::Local>::finalize>
   fe90d:	call   f3640 <<crossbeam_epoch::sync::once_lock::OnceLock<crossbeam_epoch::collector::Collector>>::initialize::<<crossbeam_epoch::collector::Collector>::new>>
   fe912:	jmp    fe8c4 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal+0x114>
