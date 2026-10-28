
/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000000fd8b0 <<rayon_core::registry::WorkerThread>::wait_until_cold>:
   fd8b0:	push   rbp
   fd8b1:	push   r15
   fd8b3:	push   r14
   fd8b5:	push   r13
   fd8b7:	push   r12
   fd8b9:	push   rbx
   fd8ba:	sub    rsp,0xd8
   fd8c1:	mov    QWORD PTR [rsp+0x8],rdi
   fd8c6:	mov    rax,QWORD PTR [rsi]
   fd8c9:	cmp    rax,0x3
   fd8cd:	je     fe106 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x856>
   fd8d3:	mov    rax,QWORD PTR [rsp+0x8]
   fd8d8:	mov    rbp,QWORD PTR [rip+0x3d9249]        # 4d6b28 <sched_yield@GLIBC_2.2.5>
   fd8df:	movabs r13,0x100000000
   fd8e9:	mov    r15,rsi
   fd8ec:	mov    QWORD PTR [rsp+0x10],rsi
   fd8f1:	lea    rcx,[rax+0x118]
   fd8f8:	add    rax,0x138
   fd8fe:	mov    QWORD PTR [rsp+0x28],rcx
   fd903:	mov    rcx,QWORD PTR fs:0x0
   fd90c:	mov    QWORD PTR [rsp+0x38],rax
   fd911:	lea    rax,[rcx-0x40]
   fd918:	mov    QWORD PTR [rsp+0x30],rax
   fd91d:	mov    rdi,QWORD PTR [rsp+0x28]
   fd922:	call   fe140 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::pop>
   fd927:	test   rax,rax
   fd92a:	jne    fd960 <<rayon_core::registry::WorkerThread>::wait_until_cold+0xb0>
   fd92c:	nop    DWORD PTR [rax+0x0]
   fd930:	mov    rsi,QWORD PTR [rsp+0x38]
   fd935:	lea    rdi,[rsp+0x60]
   fd93a:	call   fe280 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal>
   fd93f:	mov    rax,QWORD PTR [rsp+0x60]
   fd944:	cmp    rax,0x2
   fd948:	je     fd930 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x80>
   fd94a:	test   rax,rax
   fd94d:	jne    fd96f <<rayon_core::registry::WorkerThread>::wait_until_cold+0xbf>
   fd94f:	xor    eax,eax
   fd951:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fd960:	test   rax,rax
   fd963:	je     fd980 <<rayon_core::registry::WorkerThread>::wait_until_cold+0xd0>
   fd965:	mov    rdi,rdx
   fd968:	call   rax
   fd96a:	jmp    fe0bf <<rayon_core::registry::WorkerThread>::wait_until_cold+0x80f>
   fd96f:	mov    rax,QWORD PTR [rsp+0x68]
   fd974:	mov    rdx,QWORD PTR [rsp+0x70]
   fd979:	test   rax,rax
   fd97c:	jne    fd965 <<rayon_core::registry::WorkerThread>::wait_until_cold+0xb5>
   fd97e:	xchg   ax,ax
   fd980:	mov    rcx,QWORD PTR [rsp+0x8]
   fd985:	mov    rax,QWORD PTR [rcx+0x100]
   fd98c:	mov    rcx,QWORD PTR [rcx+0x110]
   fd993:	lock add QWORD PTR [rcx+0x1f0],0x10000
   fd99f:	mov    QWORD PTR [rsp+0x40],rax
   fd9a4:	mov    DWORD PTR [rsp+0x50],0x0
   fd9ac:	mov    QWORD PTR [rsp+0x48],0xffffffffffffffff
   fd9b5:	jmp    fd9c8 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x118>
   fd9b7:	nop    WORD PTR [rax+rax*1+0x0]
   fd9c0:	call   rbp
   fd9c2:	inc    ebx
   fd9c4:	mov    DWORD PTR [rsp+0x50],ebx
   fd9c8:	mov    rax,QWORD PTR [r15]
   fd9cb:	cmp    rax,0x3
   fd9cf:	je     fe0ce <<rayon_core::registry::WorkerThread>::wait_until_cold+0x81e>
   fd9d5:	mov    rdi,QWORD PTR [rsp+0x28]
   fd9da:	call   fe140 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::pop>
   fd9df:	mov    r12,rax
   fd9e2:	test   rax,rax
   fd9e5:	je     fda09 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x159>
   fd9e7:	mov    r14,rdx
   fd9ea:	test   r12,r12
   fd9ed:	jne    fdfcf <<rayon_core::registry::WorkerThread>::wait_until_cold+0x71f>
   fd9f3:	jmp    fdbf0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x340>
   fd9f8:	call   f3200 <<crossbeam_epoch::internal::Local>::finalize>
   fd9fd:	nop    DWORD PTR [rax]
   fda00:	cmp    ebx,0x2
   fda03:	jne    fdbb5 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x305>
   fda09:	mov    rax,QWORD PTR [rsp+0x38]
   fda0e:	mov    r13,QWORD PTR [rax]
   fda11:	mov    r15,QWORD PTR [r13+0x100]
   fda18:	cmp    BYTE PTR fs:0xffffffffffffffc8,0x1
   fda21:	mov    rax,QWORD PTR [rsp+0x30]
   fda26:	jne    fdb36 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x286>
   fda2c:	mov    rax,QWORD PTR [rax]
   fda2f:	mov    rcx,QWORD PTR [rax+0x818]
   fda36:	test   rcx,rcx
   fda39:	je     fda41 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x191>
   fda3b:	lock or DWORD PTR [rsp-0x40],0x0
   fda41:	call   ff5c0 <crossbeam_epoch::default::with_handle::<crossbeam_epoch::default::pin::{closure#0}, crossbeam_epoch::guard::Guard>>
   fda46:	mov    rcx,QWORD PTR [r13+0x108]
   fda4d:	mov    rdi,rax
   fda50:	sub    rcx,r15
   fda53:	test   rcx,rcx
   fda56:	jle    fdb00 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x250>
   fda5c:	mov    rax,QWORD PTR [r13+0x80]
   fda63:	mov    ebx,0x2
   fda68:	mov    rcx,rax
   fda6b:	and    rcx,0xfffffffffffffff8
   fda6f:	mov    rdx,QWORD PTR [rcx]
   fda72:	mov    rcx,QWORD PTR [rcx+0x8]
   fda76:	dec    rcx
   fda79:	and    rcx,r15
   fda7c:	shl    rcx,0x4
   fda80:	mov    r12,QWORD PTR [rdx+rcx*1]
   fda84:	mov    r14,QWORD PTR [rdx+rcx*1+0x8]
   fda89:	mov    rdx,QWORD PTR [r13+0x80]
   fda90:	cmp    rdx,rax
   fda93:	jne    fdb02 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x252>
   fda95:	lea    rcx,[r15+0x1]
   fda99:	mov    rax,r15
   fda9c:	lock cmpxchg QWORD PTR [r13+0x100],rcx
   fdaa5:	jne    fdb02 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x252>
   fdaa7:	mov    r15,QWORD PTR [rsp+0x10]
   fdaac:	movabs r13,0x100000000
   fdab6:	test   rdi,rdi
   fdab9:	je     fdba0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x2f0>
   fdabf:	dec    QWORD PTR [rdi+0x818]
   fdac6:	jne    fdba0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x2f0>
   fdacc:	mov    QWORD PTR [rdi+0x880],0x0
   fdad7:	cmp    QWORD PTR [rdi+0x820],0x0
   fdadf:	jne    fdba0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x2f0>
   fdae5:	mov    ebx,0x1
   fdaea:	mov    QWORD PTR [rsp+0x20],r14
   fdaef:	mov    QWORD PTR [rsp+0x18],r12
   fdaf4:	jmp    fd9f8 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x148>
   fdaf9:	nop    DWORD PTR [rax+0x0]
   fdb00:	xor    ebx,ebx
   fdb02:	test   rdi,rdi
   fdb05:	je     fda00 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x150>
   fdb0b:	dec    QWORD PTR [rdi+0x818]
   fdb12:	jne    fda00 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x150>
   fdb18:	mov    QWORD PTR [rdi+0x880],0x0
   fdb23:	cmp    QWORD PTR [rdi+0x820],0x0
   fdb2b:	jne    fda00 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x150>
   fdb31:	jmp    fd9f8 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x148>
   fdb36:	mov    rdi,QWORD PTR [rsp+0x30]
   fdb3b:	call   ff4e0 <<std::sys::thread_local::native::lazy::Storage<crossbeam_epoch::collector::LocalHandle, ()>>::get_or_init_slow::<crossbeam_epoch::default::HANDLE::__rust_std_internal_init_fn>>
   fdb40:	test   rax,rax
   fdb43:	jne    fda2c <<rayon_core::registry::WorkerThread>::wait_until_cold+0x17c>
   fdb49:	mov    eax,DWORD PTR [rip+0x3da1c1]        # 4d7d10 <crossbeam_epoch::default::collector::COLLECTOR+0x8>
   fdb4f:	test   eax,eax
   fdb51:	jne    fdb92 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x2e2>
   fdb53:	mov    rdi,QWORD PTR [rip+0x3da1ae]        # 4d7d08 <crossbeam_epoch::default::collector::COLLECTOR>
   fdb5a:	call   f2fb0 <<crossbeam_epoch::collector::Collector>::register>
   fdb5f:	mov    rdx,QWORD PTR [rax+0x820]
   fdb66:	mov    rcx,QWORD PTR [rax+0x818]
   fdb6d:	lea    rsi,[rdx-0x1]
   fdb71:	xor    rdx,0x1
   fdb75:	or     rdx,rcx
   fdb78:	mov    QWORD PTR [rax+0x820],rsi
   fdb7f:	jne    fda36 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x186>
   fdb85:	mov    rdi,rax
   fdb88:	call   f3200 <<crossbeam_epoch::internal::Local>::finalize>
   fdb8d:	jmp    fda41 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x191>
   fdb92:	call   f35d0 <<crossbeam_epoch::sync::once_lock::OnceLock<crossbeam_epoch::collector::Collector>>::initialize::<<crossbeam_epoch::collector::Collector>::new>>
   fdb97:	jmp    fdb53 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x2a3>
   fdb99:	nop    DWORD PTR [rax+0x0]
   fdba0:	mov    QWORD PTR [rsp+0x20],r14
   fdba5:	mov    QWORD PTR [rsp+0x18],r12
   fdbaa:	test   r12,r12
   fdbad:	jne    fdfcf <<rayon_core::registry::WorkerThread>::wait_until_cold+0x71f>
   fdbb3:	jmp    fdbf0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x340>
   fdbb5:	cmp    ebx,0x1
   fdbb8:	jne    fdbc6 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x316>
   fdbba:	mov    r14,QWORD PTR [rsp+0x20]
   fdbbf:	mov    r12,QWORD PTR [rsp+0x18]
   fdbc4:	jmp    fdbc9 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x319>
   fdbc6:	xor    r12d,r12d
   fdbc9:	mov    r15,QWORD PTR [rsp+0x10]
   fdbce:	movabs r13,0x100000000
   fdbd8:	test   r12,r12
   fdbdb:	jne    fdfcf <<rayon_core::registry::WorkerThread>::wait_until_cold+0x71f>
   fdbe1:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdbf0:	mov    rax,QWORD PTR [rsp+0x8]
   fdbf5:	mov    r14,QWORD PTR [rax+0x110]
   fdbfc:	mov    rbx,QWORD PTR [r14+0x208]
   fdc03:	cmp    rbx,0x2
   fdc07:	jb     fdd8f <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4df>
   fdc0d:	mov    rdi,QWORD PTR [r14+0x200]
   fdc14:	vmovq  xmm1,rbx
   fdc19:	vmovdqa XMMWORD PTR [rsp+0xc0],xmm1
   fdc22:	mov    QWORD PTR [rsp+0x58],rdi
   fdc27:	mov    rsi,QWORD PTR [rsp+0x8]
   fdc2c:	mov    BYTE PTR [rsp+0x7],0x0
   fdc31:	mov    rax,QWORD PTR [rsi+0x108]
   fdc38:	mov    rcx,rax
   fdc3b:	shr    rcx,0xc
   fdc3f:	xor    rcx,rax
   fdc42:	mov    rdx,rcx
   fdc45:	shl    rdx,0x19
   fdc49:	xor    rdx,rcx
   fdc4c:	movabs rcx,0x2545f4914f6cdd1d
   fdc56:	mov    rax,rdx
   fdc59:	shr    rax,0x1b
   fdc5d:	xor    rax,rdx
   fdc60:	xor    edx,edx
   fdc62:	mov    QWORD PTR [rsi+0x108],rax
   fdc69:	imul   rax,rcx
   fdc6d:	div    rbx
   fdc70:	lea    rax,[rsp+0x90]
   fdc78:	mov    QWORD PTR [rsp+0x60],0x1
   fdc81:	vmovq  xmm0,rdx
   fdc86:	mov    QWORD PTR [rsp+0x98],rax
   fdc8e:	lea    rax,[rsp+0x7]
   fdc93:	mov    QWORD PTR [rsp+0xa0],rdi
   fdc9b:	mov    QWORD PTR [rsp+0xa8],rbx
   fdca3:	mov    QWORD PTR [rsp+0xb0],rsi
   fdcab:	mov    r13,rdx
   fdcae:	mov    r15,rdx
   fdcb1:	vpunpcklqdq xmm0,xmm0,xmm1
   fdcb5:	vpmovsxbq xmm1,WORD PTR [rip+0xfffffffffff0e088]        # bd46 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x506>
   fdcbe:	mov    QWORD PTR [rsp+0xb8],rax
   fdcc6:	vmovdqu XMMWORD PTR [rsp+0x68],xmm0
   fdccc:	vmovups XMMWORD PTR [rsp+0x78],xmm1
   fdcd2:	mov    QWORD PTR [rsp+0x88],rdx
   fdcda:	mov    QWORD PTR [rsp+0x90],rsi
   fdce2:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdcf0:	lea    rdi,[rsp+0x98]
   fdcf8:	mov    rsi,r15
   fdcfb:	call   fea40 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}>
   fdd00:	mov    r12,rax
   fdd03:	test   rax,rax
   fdd06:	jne    fde4c <<rayon_core::registry::WorkerThread>::wait_until_cold+0x59c>
   fdd0c:	inc    r15
   fdd0f:	cmp    r15,rbx
   fdd12:	jne    fdcf0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x440>
   fdd14:	xor    r15d,r15d
   fdd17:	nop    WORD PTR [rax+rax*1+0x0]
   fdd20:	cmp    r13,r15
   fdd23:	je     fdd50 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4a0>
   fdd25:	lea    rdi,[rsp+0x98]
   fdd2d:	mov    rsi,r15
   fdd30:	call   fea40 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}>
   fdd35:	inc    r15
   fdd38:	mov    r12,rax
   fdd3b:	test   rax,rax
   fdd3e:	je     fdd20 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x470>
   fdd40:	jmp    fde4c <<rayon_core::registry::WorkerThread>::wait_until_cold+0x59c>
   fdd45:	data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdd50:	vmovaps xmm1,XMMWORD PTR [rsp+0xc0]
   fdd59:	cmp    BYTE PTR [rsp+0x7],0x0
   fdd5e:	mov    rdi,QWORD PTR [rsp+0x58]
   fdd63:	jne    fdc27 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x377>
   fdd69:	mov    rax,QWORD PTR [rsp+0x8]
   fdd6e:	mov    r14,QWORD PTR [rax+0x110]
   fdd75:	jmp    fdd8f <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4df>
   fdd77:	nop    WORD PTR [rax+rax*1+0x0]
   fdd80:	lock cmpxchg QWORD PTR [r14+0x80],r12
   fdd89:	je     fde54 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5a4>
   fdd8f:	mov    rax,QWORD PTR [r14+0x80]
   fdd96:	mov    r15,QWORD PTR [r14+0x88]
   fdd9d:	mov    rcx,rax
   fdda0:	shr    rcx,1
   fdda3:	mov    r13d,ecx
   fdda6:	and    r13d,0x3f
   fddaa:	cmp    r13,0x3f
   fddae:	jne    fde10 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x560>
   fddb0:	xor    ebx,ebx
   fddb2:	jmp    fdde3 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x533>
   fddb4:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fddc0:	inc    ebx
   fddc2:	mov    rax,QWORD PTR [r14+0x80]
   fddc9:	mov    r15,QWORD PTR [r14+0x88]
   fddd0:	mov    rcx,rax
   fddd3:	shr    rcx,1
   fddd6:	mov    r13d,ecx
   fddd9:	and    r13d,0x3f
   fdddd:	cmp    r13,0x3f
   fdde1:	jne    fde10 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x560>
   fdde3:	cmp    ebx,0x6
   fdde6:	ja     fde00 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x550>
   fdde8:	mov    eax,0x1
   fdded:	nop    DWORD PTR [rax]
   fddf0:	shrx   ecx,eax,ebx
   fddf5:	inc    eax
   fddf7:	pause
   fddf9:	test   ecx,ecx
   fddfb:	je     fddf0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x540>
   fddfd:	jmp    fddc0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x510>
   fddff:	nop
   fde00:	call   rbp
   fde02:	cmp    ebx,0xb
   fde05:	jb     fddc0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x510>
   fde07:	jmp    fddc2 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x512>
   fde09:	nop    DWORD PTR [rax+0x0]
   fde10:	lea    r12,[rax+0x2]
   fde14:	test   al,0x1
   fde16:	jne    fdd80 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4d0>
   fde1c:	lock or DWORD PTR [rsp-0x40],0x0
   fde22:	mov    rdx,QWORD PTR [r14+0x100]
   fde29:	mov    rsi,rdx
   fde2c:	shr    rsi,1
   fde2f:	cmp    rcx,rsi
   fde32:	je     fded9 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x629>
   fde38:	xor    rdx,rax
   fde3b:	xor    ecx,ecx
   fde3d:	cmp    rdx,0x7f
   fde41:	seta   cl
   fde44:	or     r12,rcx
   fde47:	jmp    fdd80 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4d0>
   fde4c:	mov    r14,rdx
   fde4f:	jmp    fdfc0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x710>
   fde54:	cmp    r13d,0x3e
   fde58:	jne    fdee1 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x631>
   fde5e:	mov    rax,QWORD PTR [r15]
   fde61:	test   rax,rax
   fde64:	je     fe014 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x764>
   fde6a:	mov    rcx,QWORD PTR [rax]
   fde6d:	and    r12,0xfffffffffffffffe
   fde71:	xor    edx,edx
   fde73:	mov    QWORD PTR [r14+0x88],rax
   fde7a:	test   rcx,rcx
   fde7d:	setne  dl
   fde80:	lea    rcx,[r12+rdx*1+0x2]
   fde85:	mov    QWORD PTR [r14+0x80],rcx
   fde8c:	mov    rax,QWORD PTR [r15+0x5e8]
   fde93:	test   al,0x1
   fde95:	jne    fdf3a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x68a>
   fde9b:	xor    ebx,ebx
   fde9d:	jmp    fdeb1 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x601>
   fde9f:	nop
   fdea0:	inc    ebx
   fdea2:	mov    rax,QWORD PTR [r15+0x5e8]
   fdea9:	test   al,0x1
   fdeab:	jne    fdf3a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x68a>
   fdeb1:	cmp    ebx,0x6
   fdeb4:	ja     fded0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x620>
   fdeb6:	mov    eax,0x1
   fdebb:	nop    DWORD PTR [rax+rax*1+0x0]
   fdec0:	shrx   ecx,eax,ebx
   fdec5:	inc    eax
   fdec7:	pause
   fdec9:	test   ecx,ecx
   fdecb:	je     fdec0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x610>
   fdecd:	jmp    fdea0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5f0>
   fdecf:	nop
   fded0:	call   rbp
   fded2:	cmp    ebx,0xb
   fded5:	jb     fdea0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5f0>
   fded7:	jmp    fdea2 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5f2>
   fded9:	xor    r12d,r12d
   fdedc:	jmp    fdfc0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x710>
   fdee1:	lea    rax,[r13+r13*2+0x0]
   fdee6:	lea    rbx,[r15+rax*8+0x8]
   fdeeb:	mov    rax,QWORD PTR [r15+rax*8+0x18]
   fdef0:	test   al,0x1
   fdef2:	jne    fdf4a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x69a>
   fdef4:	xor    r14d,r14d
   fdef7:	jmp    fdf0b <<rayon_core::registry::WorkerThread>::wait_until_cold+0x65b>
   fdef9:	nop    DWORD PTR [rax+0x0]
   fdf00:	inc    r14d
   fdf03:	mov    rax,QWORD PTR [rbx+0x10]
   fdf07:	test   al,0x1
   fdf09:	jne    fdf4a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x69a>
   fdf0b:	cmp    r14d,0x6
   fdf0f:	ja     fdf30 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x680>
   fdf11:	mov    eax,0x1
   fdf16:	cs nop WORD PTR [rax+rax*1+0x0]
   fdf20:	shrx   ecx,eax,r14d
   fdf25:	inc    eax
   fdf27:	pause
   fdf29:	test   ecx,ecx
   fdf2b:	je     fdf20 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x670>
   fdf2d:	jmp    fdf00 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x650>
   fdf2f:	nop
   fdf30:	call   rbp
   fdf32:	cmp    r14d,0xb
   fdf36:	jb     fdf00 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x650>
   fdf38:	jmp    fdf03 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x653>
   fdf3a:	mov    r12,QWORD PTR [r15+0x5d8]
   fdf41:	mov    r14,QWORD PTR [r15+0x5e0]
   fdf48:	jmp    fdf80 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x6d0>
   fdf4a:	mov    r12,QWORD PTR [rbx]
   fdf4d:	mov    r14,QWORD PTR [rbx+0x8]
   fdf51:	mov    rax,QWORD PTR [rbx+0x10]
   fdf55:	data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdf60:	mov    rcx,rax
   fdf63:	or     rcx,0x2
   fdf67:	lock cmpxchg QWORD PTR [rbx+0x10],rcx
   fdf6d:	jne    fdf60 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x6b0>
   fdf6f:	test   al,0x4
   fdf71:	je     fdfc0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x710>
   fdf73:	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdf80:	test   r13,r13
   fdf83:	je     fdfb5 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x705>
   fdf85:	dec    r13
   fdf88:	lea    rax,[r13+r13*2+0x0]
   fdf8d:	mov    rcx,QWORD PTR [r15+rax*8+0x18]
   fdf92:	test   cl,0x2
   fdf95:	jne    fdf80 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x6d0>
   fdf97:	lea    rcx,[r15+rax*8]
   fdf9b:	mov    rax,QWORD PTR [rcx+0x18]
   fdf9f:	nop
   fdfa0:	mov    rdx,rax
   fdfa3:	or     rdx,0x4
   fdfa7:	lock cmpxchg QWORD PTR [rcx+0x18],rdx
   fdfad:	jne    fdfa0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x6f0>
   fdfaf:	test   al,0x2
   fdfb1:	jne    fdf80 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x6d0>
   fdfb3:	jmp    fdfc0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x710>
   fdfb5:	mov    rdi,r15
   fdfb8:	call   QWORD PTR [rip+0x3d8aca]        # 4d6a88 <free@GLIBC_2.2.5>
   fdfbe:	xchg   ax,ax
   fdfc0:	mov    r15,QWORD PTR [rsp+0x10]
   fdfc5:	movabs r13,0x100000000
   fdfcf:	mov    rax,QWORD PTR [rsp+0x8]
   fdfd4:	mov    rdi,QWORD PTR [rax+0x110]
   fdfdb:	test   r12,r12
   fdfde:	jne    fe08d <<rayon_core::registry::WorkerThread>::wait_until_cold+0x7dd>
   fdfe4:	mov    ebx,DWORD PTR [rsp+0x50]
   fdfe8:	lea    rax,[rdi+0x1d8]
   fdfef:	cmp    ebx,0x20
   fdff2:	jb     fd9c0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x110>
   fdff8:	je     fe059 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x7a9>
   fdffa:	mov    rcx,QWORD PTR [rsp+0x8]
   fdfff:	lea    rsi,[rsp+0x40]
   fe004:	mov    rdi,rax
   fe007:	mov    rdx,r15
   fe00a:	call   fe440 <<rayon_core::sleep::Sleep>::sleep::<<rayon_core::registry::WorkerThread>::wait_until_cold::{closure#0}>>
   fe00f:	jmp    fd9c8 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x118>
   fe014:	xor    ebx,ebx
   fe016:	jmp    fe02e <<rayon_core::registry::WorkerThread>::wait_until_cold+0x77e>
   fe018:	nop    DWORD PTR [rax+rax*1+0x0]
   fe020:	inc    ebx
   fe022:	mov    rax,QWORD PTR [r15]
   fe025:	test   rax,rax
   fe028:	jne    fde6a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5ba>
   fe02e:	cmp    ebx,0x6
   fe031:	ja     fe050 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x7a0>
   fe033:	mov    eax,0x1
   fe038:	nop    DWORD PTR [rax+rax*1+0x0]
   fe040:	shrx   ecx,eax,ebx
   fe045:	inc    eax
   fe047:	pause
   fe049:	test   ecx,ecx
   fe04b:	je     fe040 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x790>
   fe04d:	jmp    fe020 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x770>
   fe04f:	nop
   fe050:	call   rbp
   fe052:	cmp    ebx,0xb
   fe055:	jb     fe020 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x770>
   fe057:	jmp    fe022 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x772>
   fe059:	mov    rax,QWORD PTR [rdi+0x1f0]
   fe060:	test   rax,r13
   fe063:	je     fe076 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x7c6>
   fe065:	lea    rcx,[rax+r13*1]
   fe069:	lock cmpxchg QWORD PTR [rdi+0x1f0],rcx
   fe072:	jne    fe059 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x7a9>
   fe074:	jmp    fe079 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x7c9>
   fe076:	mov    rcx,rax
   fe079:	shr    rcx,0x20
   fe07d:	mov    QWORD PTR [rsp+0x48],rcx
   fe082:	inc    DWORD PTR [rsp+0x50]
   fe086:	call   rbp
   fe088:	jmp    fd9c8 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x118>
   fe08d:	mov    rax,0xffffffffffff0000
   fe094:	lock xadd QWORD PTR [rdi+0x1f0],rax
   fe09d:	movzx  esi,ax
   fe0a0:	mov    eax,0x2
   fe0a5:	cmp    rsi,0x2
   fe0a9:	cmovae rsi,rax
   fe0ad:	add    rdi,0x1d8
   fe0b4:	call   fe3f0 <<rayon_core::sleep::Sleep>::wake_any_threads>
   fe0b9:	mov    rdi,r14
   fe0bc:	call   r12
   fe0bf:	mov    rax,QWORD PTR [r15]
   fe0c2:	cmp    rax,0x3
   fe0c6:	jne    fd91d <<rayon_core::registry::WorkerThread>::wait_until_cold+0x6d>
   fe0cc:	jmp    fe106 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x856>
   fe0ce:	mov    rax,QWORD PTR [rsp+0x8]
   fe0d3:	mov    esi,0x2
   fe0d8:	mov    rdi,QWORD PTR [rax+0x110]
   fe0df:	mov    rax,0xffffffffffff0000
   fe0e6:	lock xadd QWORD PTR [rdi+0x1f0],rax
   fe0ef:	movzx  eax,ax
   fe0f2:	cmp    rax,0x2
   fe0f6:	cmovb  rsi,rax
   fe0fa:	add    rdi,0x1d8
   fe101:	call   fe3f0 <<rayon_core::sleep::Sleep>::wake_any_threads>
   fe106:	add    rsp,0xd8
   fe10d:	pop    rbx
   fe10e:	pop    r12
   fe110:	pop    r13
   fe112:	pop    r14
   fe114:	pop    r15
   fe116:	pop    rbp
   fe117:	ret
   fe118:	jmp    fe124 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x874>
   fe11a:	jmp    fe124 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x874>
   fe11c:	jmp    fe124 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x874>
   fe11e:	jmp    fe124 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x874>
   fe120:	jmp    fe124 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x874>
   fe122:	jmp    fe124 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x874>
   fe124:	lea    rdi,[rip+0xfffffffffff1a506]        # 18631 <miniz_oxide::inflate::core::reverse_bits::REVERSED_BITS_LOOKUP+0x6fd>
   fe12b:	mov    esi,0x57
   fe130:	call   13fdd0 <std::io::stdio::_eprint>
   fe135:	call   1103d0 <std::process::abort>
   fe13a:	call   f0f63 <core::panicking::panic_in_cleanup>
