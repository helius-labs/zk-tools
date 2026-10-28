
/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000000fea40 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}>:
   fea40:	push   rbp
   fea41:	push   r15
   fea43:	push   r14
   fea45:	push   r13
   fea47:	push   r12
   fea49:	push   rbx
   fea4a:	sub    rsp,0x58
   fea4e:	mov    rax,QWORD PTR [rdi]
   fea51:	mov    rax,QWORD PTR [rax]
   fea54:	cmp    rsi,QWORD PTR [rax+0x100]
   fea5b:	je     ff1d0 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x790>
   fea61:	mov    rax,QWORD PTR [rdi+0x10]
   fea65:	mov    rbx,rdi
   fea68:	cmp    rsi,rax
   fea6b:	jae    ff4a1 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0xa61>
   fea71:	mov    rbp,QWORD PTR [rbx+0x18]
   fea75:	lea    rsi,[rsi+rsi*2]
   fea79:	shl    rsi,0x4
   fea7d:	add    rsi,QWORD PTR [rbx+0x8]
   fea81:	mov    rax,QWORD PTR [rbp+0x110]
   fea88:	cmp    BYTE PTR [rax+0x210],0x0
   fea8f:	je     fecb0 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x270>
   fea95:	mov    r15,QWORD PTR [rsi]
   fea98:	lea    r14,[rbp+0x118]
   fea9f:	cmp    r15,QWORD PTR [rbp+0x118]
   feaa6:	je     feccd <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x28d>
   feaac:	movzx  eax,BYTE PTR [rsi+0x8]
   feab0:	mov    BYTE PTR [rsp+0x20],al
   feab4:	mov    r13,QWORD PTR [r15+0x100]
   feabb:	cmp    BYTE PTR fs:0xffffffffffffffc8,0x1
   feac4:	jne    ff3d7 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x997>
   feaca:	mov    rax,QWORD PTR fs:0x0
   fead3:	lea    rax,[rax-0x40]
   feada:	mov    rax,QWORD PTR [rax]
   feadd:	mov    rcx,QWORD PTR [rax+0x818]
   feae4:	test   rcx,rcx
   feae7:	je     feaef <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0xaf>
   feae9:	lock or DWORD PTR [rsp-0x40],0x0
   feaef:	call   ff5c0 <crossbeam_epoch::default::with_handle::<crossbeam_epoch::default::pin::{closure#0}, crossbeam_epoch::guard::Guard>>
   feaf4:	mov    r12,QWORD PTR [r15+0x108]
   feafb:	sub    r12,r13
   feafe:	test   r12,r12
   feb01:	jle    fed5f <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x31f>
   feb07:	mov    QWORD PTR [rsp+0x18],r14
   feb0c:	lea    r14,[r12-0x1]
   feb11:	mov    r8d,0x1f
   feb17:	mov    QWORD PTR [rsp+0x8],rax
   feb1c:	shr    r14,1
   feb1f:	cmp    r14,0x1f
   feb23:	cmovb  r8,r14
   feb27:	test   r14,r14
   feb2a:	je     feb57 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x117>
   feb2c:	mov    rcx,QWORD PTR [rbp+0x118]
   feb33:	mov    rax,QWORD PTR [rcx+0x108]
   feb3a:	mov    rcx,QWORD PTR [rcx+0x100]
   feb41:	mov    rdx,QWORD PTR [rbp+0x128]
   feb48:	sub    rax,rcx
   feb4b:	sub    rdx,rax
   feb4e:	cmp    rdx,r8
   feb51:	jb     ff442 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0xa02>
   feb57:	mov    rax,QWORD PTR [rbp+0x118]
   feb5e:	mov    rsi,QWORD PTR [rbp+0x120]
   feb65:	mov    rdi,QWORD PTR [rbp+0x128]
   feb6c:	mov    rdx,rbp
   feb6f:	mov    rbp,QWORD PTR [rax+0x108]
   feb76:	mov    r9,QWORD PTR [r15+0x80]
   feb7d:	mov    QWORD PTR [rsp+0x38],r9
   feb82:	and    r9,0xfffffffffffffff8
   feb86:	mov    rcx,QWORD PTR [r9+0x8]
   feb8a:	mov    rax,QWORD PTR [r9]
   feb8d:	dec    rcx
   feb90:	and    rcx,r13
   feb93:	shl    rcx,0x4
   feb97:	mov    r10,QWORD PTR [rax+rcx*1]
   feb9b:	mov    rax,QWORD PTR [rax+rcx*1+0x8]
   feba0:	test   BYTE PTR [rsp+0x20],0x1
   feba5:	mov    QWORD PTR [rsp+0x30],rdx
   febaa:	mov    QWORD PTR [rsp+0x28],rax
   febaf:	je     fed67 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x327>
   febb5:	lea    rcx,[r13+0x1]
   febb9:	mov    rax,r13
   febbc:	mov    r12d,0x2
   febc2:	mov    QWORD PTR [rsp+0x20],rcx
   febc7:	lock cmpxchg QWORD PTR [r15+0x100],rcx
   febd0:	mov    rax,QWORD PTR [rsp+0x8]
   febd5:	jne    ff182 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x742>
   febdb:	test   r14,r14
   febde:	je     fee2d <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x3ed>
   febe4:	lea    rax,[r13+0x2]
   febe8:	not    r13
   febeb:	mov    QWORD PTR [rsp],r8
   febef:	xor    r8d,r8d
   febf2:	mov    r11,rbp
   febf5:	mov    QWORD PTR [rsp+0x10],rax
   febfa:	lock or DWORD PTR [rsp-0x40],0x0
   fec00:	mov    rax,QWORD PTR [r15+0x108]
   fec07:	lea    rcx,[rbp+r8*1+0x0]
   fec0c:	add    rax,r13
   fec0f:	test   rax,rax
   fec12:	jle    fec9e <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x25e>
   fec18:	mov    rax,QWORD PTR [rsp+0x20]
   fec1d:	mov    rdx,QWORD PTR [r9+0x8]
   fec21:	mov    r12,QWORD PTR [r9]
   fec24:	add    rax,r8
   fec27:	dec    rdx
   fec2a:	and    rdx,rax
   fec2d:	shl    rdx,0x4
   fec31:	mov    r14,QWORD PTR [r12+rdx*1]
   fec35:	mov    r12,QWORD PTR [r12+rdx*1+0x8]
   fec3a:	mov    rdx,QWORD PTR [r15+0x80]
   fec41:	cmp    rdx,QWORD PTR [rsp+0x38]
   fec46:	jne    fec9b <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x25b>
   fec48:	mov    rdx,QWORD PTR [rsp+0x10]
   fec4d:	add    rdx,r8
   fec50:	lock cmpxchg QWORD PTR [r15+0x100],rdx
   fec59:	jne    fec9b <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x25b>
   fec5b:	lea    rax,[rdi-0x1]
   fec5f:	inc    r8
   fec62:	inc    r11
   fec65:	dec    r13
   fec68:	and    rcx,rax
   fec6b:	shl    rcx,0x4
   fec6f:	mov    QWORD PTR [rsi+rcx*1],r10
   fec73:	mov    r10,r14
   fec76:	mov    rax,QWORD PTR [rsp+0x28]
   fec7b:	mov    QWORD PTR [rsi+rcx*1+0x8],rax
   fec80:	mov    QWORD PTR [rsp+0x28],r12
   fec85:	cmp    QWORD PTR [rsp],r8
   fec89:	jne    febfa <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x1ba>
   fec8f:	mov    r8,QWORD PTR [rsp]
   fec93:	mov    QWORD PTR [rsp+0x28],r12
   fec98:	mov    r10,r14
   fec9b:	mov    rcx,r11
   fec9e:	mov    rbp,rcx
   feca1:	mov    r14,QWORD PTR [rsp+0x18]
   feca6:	mov    rcx,QWORD PTR [rsp+0x8]
   fecab:	jmp    fee35 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x3f5>
   fecb0:	lea    rdi,[rsp+0x40]
   fecb5:	call   fe280 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal>
   fecba:	mov    r12,QWORD PTR [rsp+0x40]
   fecbf:	test   r12,r12
   fecc2:	jne    ff1b7 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x777>
   fecc8:	jmp    ff1d0 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x790>
   feccd:	mov    rdi,r14
   fecd0:	call   fe140 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::pop>
   fecd5:	mov    QWORD PTR [rsp+0x28],rdx
   fecda:	test   rax,rax
   fecdd:	je     feda9 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x369>
   fece3:	mov    r10,rax
   fece6:	mov    rcx,QWORD PTR [r14]
   fece9:	mov    rax,QWORD PTR [rcx+0x108]
   fecf0:	mov    rcx,QWORD PTR [rcx+0x100]
   fecf7:	mov    rdx,QWORD PTR [rsp+0x28]
   fecfc:	mov    QWORD PTR [rsp+0x40],0x1
   fed05:	mov    QWORD PTR [rsp+0x48],r10
   fed0a:	sub    rax,rcx
   fed0d:	mov    QWORD PTR [rsp+0x50],rdx
   fed12:	test   rax,rax
   fed15:	jle    fecba <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x27a>
   fed17:	mov    rdi,QWORD PTR [rbp+0x110]
   fed1e:	mov    esi,0xffffffff
   fed23:	movabs rdx,0x100000000
   fed2d:	cmp    rax,rsi
   fed30:	cmovb  rsi,rax
   fed34:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fed40:	mov    rax,QWORD PTR [rdi+0x1f0]
   fed47:	test   rax,rdx
   fed4a:	jne    fedc3 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x383>
   fed4c:	mov    rcx,rax
   fed4f:	or     rcx,rdx
   fed52:	lock cmpxchg QWORD PTR [rdi+0x1f0],rcx
   fed5b:	jne    fed40 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x300>
   fed5d:	jmp    fedc6 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x386>
   fed5f:	xor    r12d,r12d
   fed62:	jmp    ff182 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x742>
   fed67:	cmp    BYTE PTR [rdx+0x130],0x0
   fed6e:	mov    QWORD PTR [rsp],r8
   fed72:	mov    QWORD PTR [rsp+0x10],r10
   fed77:	je     fee06 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x3c6>
   fed7d:	test   r14,r14
   fed80:	je     ff138 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x6f8>
   fed86:	mov    rcx,QWORD PTR [rsp]
   fed8a:	dec    rdi
   fed8d:	mov    eax,ecx
   fed8f:	lea    r10,[rbp+rcx*1+0x0]
   fed94:	and    eax,0x3
   fed97:	cmp    r12,0x9
   fed9b:	jae    fee77 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x437>
   feda1:	xor    r8d,r8d
   feda4:	jmp    fefa4 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x564>
   feda9:	xor    r12d,r12d
   fedac:	mov    QWORD PTR [rsp+0x48],0x0
   fedb5:	test   r12,r12
   fedb8:	jne    ff1b7 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x777>
   fedbe:	jmp    ff1d0 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x790>
   fedc3:	mov    rcx,rax
   fedc6:	mov    rax,rcx
   fedc9:	and    rax,0xffff
   fedcf:	je     fecba <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x27a>
   fedd5:	shr    ecx,0x10
   fedd8:	sub    ecx,eax
   fedda:	sub    esi,ecx
   feddc:	jbe    fecba <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x27a>
   fede2:	cmp    eax,esi
   fede4:	cmovb  esi,eax
   fede7:	add    rdi,0x1d8
   fedee:	call   fe3f0 <<rayon_core::sleep::Sleep>::wake_any_threads>
   fedf3:	mov    r12,QWORD PTR [rsp+0x40]
   fedf8:	test   r12,r12
   fedfb:	jne    ff1b7 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x777>
   fee01:	jmp    ff1d0 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x790>
   fee06:	test   r14,r14
   fee09:	je     ff138 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x6f8>
   fee0f:	mov    rcx,QWORD PTR [rsp]
   fee13:	dec    rdi
   fee16:	mov    eax,ecx
   fee18:	and    eax,0x3
   fee1b:	cmp    r12,0x9
   fee1f:	jae    feffd <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x5bd>
   fee25:	xor    r10d,r10d
   fee28:	jmp    ff0f2 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x6b2>
   fee2d:	mov    r14,QWORD PTR [rsp+0x18]
   fee32:	mov    rcx,rax
   fee35:	mov    rax,QWORD PTR [rsp+0x30]
   fee3a:	cmp    BYTE PTR [rax+0x130],0x0
   fee41:	jne    ff380 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x940>
   fee47:	mov    rdx,r8
   fee4a:	shr    r8,1
   fee4d:	je     ff380 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x940>
   fee53:	mov    eax,r8d
   fee56:	dec    rdi
   fee59:	and    eax,0x3
   fee5c:	mov    QWORD PTR [rsp+0x10],r10
   fee61:	mov    QWORD PTR [rsp],rdx
   fee65:	cmp    rdx,0x8
   fee69:	jae    ff1e1 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x7a1>
   fee6f:	xor    r12d,r12d
   fee72:	jmp    ff31f <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x8df>
   fee77:	lea    r14,[rbp+rcx*1-0x1]
   fee7c:	mov    edx,ecx
   fee7e:	and    edx,0x1c
   fee81:	xor    r8d,r8d
   fee84:	mov    QWORD PTR [rsp+0x20],rdx
   fee89:	nop    DWORD PTR [rax+0x0]
   fee90:	mov    rdx,QWORD PTR [r9+0x8]
   fee94:	mov    rcx,QWORD PTR [r9]
   fee97:	mov    r11,r8
   fee9a:	lea    r8,[r13+r8*1+0x1]
   fee9f:	movabs r12,0xffffffffffffffe
   feea9:	dec    rdx
   feeac:	and    rdx,r8
   feeaf:	shl    rdx,0x4
   feeb3:	mov    r8,QWORD PTR [rcx+rdx*1]
   feeb7:	mov    rcx,QWORD PTR [rcx+rdx*1+0x8]
   feebc:	mov    rdx,r14
   feebf:	and    rdx,rdi
   feec2:	add    r14,0xfffffffffffffffc
   feec6:	shl    rdx,0x4
   feeca:	mov    QWORD PTR [rsi+rdx*1],r8
   feece:	mov    QWORD PTR [rsi+rdx*1+0x8],rcx
   feed3:	lea    rcx,[r13+r11*1+0x2]
   feed8:	mov    r8,QWORD PTR [r9+0x8]
   feedc:	mov    rdx,QWORD PTR [r9]
   feedf:	dec    r8
   feee2:	and    r8,rcx
   feee5:	shl    r8,0x4
   feee9:	mov    rcx,QWORD PTR [rdx+r8*1]
   feeed:	mov    rdx,QWORD PTR [rdx+r8*1+0x8]
   feef2:	mov    r8,r11
   feef5:	xor    r8,r12
   feef8:	mov    r12,r11
   feefb:	add    r8,r10
   feefe:	and    r8,rdi
   fef01:	shl    r8,0x4
   fef05:	mov    QWORD PTR [rsi+r8*1],rcx
   fef09:	mov    QWORD PTR [rsi+r8*1+0x8],rdx
   fef0e:	lea    r8,[r13+r11*1+0x3]
   fef13:	mov    rdx,QWORD PTR [r9+0x8]
   fef17:	mov    rcx,QWORD PTR [r9]
   fef1a:	dec    rdx
   fef1d:	and    rdx,r8
   fef20:	shl    rdx,0x4
   fef24:	mov    r8,QWORD PTR [rcx+rdx*1]
   fef28:	mov    rcx,QWORD PTR [rcx+rdx*1+0x8]
   fef2d:	movabs rdx,0xffffffffffffffd
   fef37:	xor    r12,rdx
   fef3a:	add    r12,r10
   fef3d:	and    r12,rdi
   fef40:	shl    r12,0x4
   fef44:	mov    QWORD PTR [rsi+r12*1],r8
   fef48:	mov    QWORD PTR [rsi+r12*1+0x8],rcx
   fef4d:	lea    rcx,[r13+r11*1+0x4]
   fef52:	lea    r8,[r11+0x4]
   fef56:	mov    r12,QWORD PTR [r9+0x8]
   fef5a:	mov    rdx,QWORD PTR [r9]
   fef5d:	dec    r12
   fef60:	and    r12,rcx
   fef63:	shl    r12,0x4
   fef67:	mov    rcx,QWORD PTR [rdx+r12*1]
   fef6b:	mov    rdx,QWORD PTR [rdx+r12*1+0x8]
   fef70:	movabs r12,0xffffffffffffffc
   fef7a:	xor    r11,r12
   fef7d:	add    r11,r10
   fef80:	and    r11,rdi
   fef83:	shl    r11,0x4
   fef87:	mov    QWORD PTR [rsi+r11*1],rcx
   fef8b:	mov    QWORD PTR [rsi+r11*1+0x8],rdx
   fef90:	cmp    QWORD PTR [rsp+0x20],r8
   fef95:	jne    fee90 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x450>
   fef9b:	test   rax,rax
   fef9e:	je     ff138 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x6f8>
   fefa4:	lea    rcx,[r8+r13*1+0x1]
   fefa9:	not    r8
   fefac:	neg    rax
   fefaf:	add    r8,r10
   fefb2:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fefc0:	mov    r10,QWORD PTR [r9+0x8]
   fefc4:	mov    rdx,QWORD PTR [r9]
   fefc7:	dec    r10
   fefca:	and    r10,rcx
   fefcd:	inc    rcx
   fefd0:	shl    r10,0x4
   fefd4:	mov    r11,QWORD PTR [rdx+r10*1]
   fefd8:	mov    rdx,QWORD PTR [rdx+r10*1+0x8]
   fefdd:	mov    r10,r8
   fefe0:	and    r10,rdi
   fefe3:	dec    r8
   fefe6:	shl    r10,0x4
   fefea:	inc    rax
   fefed:	mov    QWORD PTR [rsi+r10*1],r11
   feff1:	mov    QWORD PTR [rsi+r10*1+0x8],rdx
   feff6:	jne    fefc0 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x580>
   feff8:	jmp    ff138 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x6f8>
   feffd:	mov    r8,rcx
   ff000:	and    r8d,0x1c
   ff004:	xor    r10d,r10d
   ff007:	nop    WORD PTR [rax+rax*1+0x0]
   ff010:	mov    r11,QWORD PTR [r9+0x8]
   ff014:	mov    rdx,QWORD PTR [r9]
   ff017:	lea    rcx,[r13+r10*1+0x1]
   ff01c:	dec    r11
   ff01f:	and    r11,rcx
   ff022:	shl    r11,0x4
   ff026:	mov    rcx,QWORD PTR [rdx+r11*1]
   ff02a:	mov    rdx,QWORD PTR [rdx+r11*1+0x8]
   ff02f:	lea    r11,[rbp+r10*1+0x0]
   ff034:	and    r11,rdi
   ff037:	shl    r11,0x4
   ff03b:	mov    QWORD PTR [rsi+r11*1],rcx
   ff03f:	mov    QWORD PTR [rsi+r11*1+0x8],rdx
   ff044:	lea    rcx,[r13+r10*1+0x2]
   ff049:	mov    r11,QWORD PTR [r9+0x8]
   ff04d:	mov    rdx,QWORD PTR [r9]
   ff050:	dec    r11
   ff053:	and    r11,rcx
   ff056:	shl    r11,0x4
   ff05a:	mov    rcx,QWORD PTR [rdx+r11*1]
   ff05e:	mov    rdx,QWORD PTR [rdx+r11*1+0x8]
   ff063:	lea    r11,[rbp+r10*1+0x1]
   ff068:	and    r11,rdi
   ff06b:	shl    r11,0x4
   ff06f:	mov    QWORD PTR [rsi+r11*1],rcx
   ff073:	mov    QWORD PTR [rsi+r11*1+0x8],rdx
   ff078:	lea    rcx,[r13+r10*1+0x3]
   ff07d:	mov    r11,QWORD PTR [r9+0x8]
   ff081:	mov    rdx,QWORD PTR [r9]
   ff084:	dec    r11
   ff087:	and    r11,rcx
   ff08a:	shl    r11,0x4
   ff08e:	mov    rcx,QWORD PTR [rdx+r11*1]
   ff092:	mov    rdx,QWORD PTR [rdx+r11*1+0x8]
   ff097:	lea    r11,[rbp+r10*1+0x2]
   ff09c:	and    r11,rdi
   ff09f:	shl    r11,0x4
   ff0a3:	mov    QWORD PTR [rsi+r11*1],rcx
   ff0a7:	mov    QWORD PTR [rsi+r11*1+0x8],rdx
   ff0ac:	lea    rcx,[r13+r10*1+0x4]
   ff0b1:	lea    rdx,[rbp+r10*1+0x3]
   ff0b6:	add    r10,0x4
   ff0ba:	mov    r14,QWORD PTR [r9+0x8]
   ff0be:	mov    r11,QWORD PTR [r9]
   ff0c1:	and    rdx,rdi
   ff0c4:	shl    rdx,0x4
   ff0c8:	dec    r14
   ff0cb:	and    r14,rcx
   ff0ce:	shl    r14,0x4
   ff0d2:	mov    rcx,QWORD PTR [r11+r14*1]
   ff0d6:	mov    r11,QWORD PTR [r11+r14*1+0x8]
   ff0db:	mov    QWORD PTR [rsi+rdx*1],rcx
   ff0df:	mov    QWORD PTR [rsi+rdx*1+0x8],r11
   ff0e4:	cmp    r8,r10
   ff0e7:	jne    ff010 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x5d0>
   ff0ed:	test   rax,rax
   ff0f0:	je     ff138 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x6f8>
   ff0f2:	lea    rcx,[r10+r13*1+0x1]
   ff0f7:	add    r10,rbp
   ff0fa:	neg    rax
   ff0fd:	nop    DWORD PTR [rax]
   ff100:	mov    r8,QWORD PTR [r9+0x8]
   ff104:	mov    rdx,QWORD PTR [r9]
   ff107:	dec    r8
   ff10a:	and    r8,rcx
   ff10d:	inc    rcx
   ff110:	shl    r8,0x4
   ff114:	mov    r11,QWORD PTR [rdx+r8*1]
   ff118:	mov    rdx,QWORD PTR [rdx+r8*1+0x8]
   ff11d:	mov    r8,r10
   ff120:	and    r8,rdi
   ff123:	inc    r10
   ff126:	shl    r8,0x4
   ff12a:	inc    rax
   ff12d:	mov    QWORD PTR [rsi+r8*1],r11
   ff131:	mov    QWORD PTR [rsi+r8*1+0x8],rdx
   ff136:	jne    ff100 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x6c0>
   ff138:	mov    rax,QWORD PTR [r15+0x80]
   ff13f:	mov    r12d,0x2
   ff145:	cmp    rax,QWORD PTR [rsp+0x38]
   ff14a:	jne    ff17d <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x73d>
   ff14c:	mov    rdx,QWORD PTR [rsp]
   ff150:	mov    rax,r13
   ff153:	lea    rcx,[r13+rdx*1+0x1]
   ff158:	lock cmpxchg QWORD PTR [r15+0x100],rcx
   ff161:	mov    rax,QWORD PTR [rsp+0x8]
   ff166:	mov    r10,QWORD PTR [rsp+0x10]
   ff16b:	jne    ff182 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x742>
   ff16d:	mov    r14,QWORD PTR [rsp+0x18]
   ff172:	mov    rcx,rax
   ff175:	add    rbp,rdx
   ff178:	jmp    ff380 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x940>
   ff17d:	mov    rax,QWORD PTR [rsp+0x8]
   ff182:	test   rax,rax
   ff185:	je     ff1a9 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x769>
   ff187:	dec    QWORD PTR [rax+0x818]
   ff18e:	jne    ff1a9 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x769>
   ff190:	mov    QWORD PTR [rax+0x880],0x0
   ff19b:	cmp    QWORD PTR [rax+0x820],0x0
   ff1a3:	je     ff482 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0xa42>
   ff1a9:	mov    QWORD PTR [rsp+0x48],0x0
   ff1b2:	test   r12,r12
   ff1b5:	je     ff1d0 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x790>
   ff1b7:	cmp    r12,0x1
   ff1bb:	jne    ff1c9 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x789>
   ff1bd:	mov    rax,QWORD PTR [rsp+0x48]
   ff1c2:	mov    rdx,QWORD PTR [rsp+0x50]
   ff1c7:	jmp    ff1d2 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x792>
   ff1c9:	mov    rax,QWORD PTR [rbx+0x20]
   ff1cd:	mov    BYTE PTR [rax],0x1
   ff1d0:	xor    eax,eax
   ff1d2:	add    rsp,0x58
   ff1d6:	pop    rbx
   ff1d7:	pop    r12
   ff1d9:	pop    r13
   ff1db:	pop    r14
   ff1dd:	pop    r15
   ff1df:	pop    rbp
   ff1e0:	ret
   ff1e1:	mov    r9,rbp
   ff1e4:	and    r8,0xfffffffffffffffc
   ff1e8:	sub    r9,rdx
   ff1eb:	lea    r11,[rbp-0x1]
   ff1ef:	xor    r12d,r12d
   ff1f2:	lea    rcx,[r9+r12*1]
   ff1f6:	mov    r10,r11
   ff1f9:	and    r10,rdi
   ff1fc:	mov    r13,r12
   ff1ff:	add    r11,0xfffffffffffffffc
   ff203:	and    rcx,rdi
   ff206:	shl    r10,0x4
   ff20a:	shl    rcx,0x4
   ff20e:	mov    rdx,QWORD PTR [rsi+rcx*1]
   ff212:	mov    r12,QWORD PTR [rsi+rcx*1+0x8]
   ff217:	mov    r14,QWORD PTR [rsi+r10*1]
   ff21b:	mov    r15,QWORD PTR [rsi+r10*1+0x8]
   ff220:	mov    QWORD PTR [rsi+rcx*1],r14
   ff224:	mov    QWORD PTR [rsi+rcx*1+0x8],r15
   ff229:	movabs r15,0xffffffffffffffe
   ff233:	mov    QWORD PTR [rsi+r10*1],rdx
   ff237:	lea    rcx,[r9+r13*1+0x1]
   ff23c:	mov    rdx,r13
   ff23f:	mov    QWORD PTR [rsi+r10*1+0x8],r12
   ff244:	xor    rdx,r15
   ff247:	add    rdx,rbp
   ff24a:	and    rcx,rdi
   ff24d:	and    rdx,rdi
   ff250:	shl    rcx,0x4
   ff254:	shl    rdx,0x4
   ff258:	mov    r10,QWORD PTR [rsi+rcx*1]
   ff25c:	mov    r14,QWORD PTR [rsi+rcx*1+0x8]
   ff261:	mov    r15,QWORD PTR [rsi+rdx*1]
   ff265:	mov    r12,QWORD PTR [rsi+rdx*1+0x8]
   ff26a:	mov    QWORD PTR [rsi+rcx*1],r15
   ff26e:	mov    QWORD PTR [rsi+rcx*1+0x8],r12
   ff273:	mov    QWORD PTR [rsi+rdx*1],r10
   ff277:	mov    QWORD PTR [rsi+rdx*1+0x8],r14
   ff27c:	movabs r14,0xffffffffffffffd
   ff286:	lea    rcx,[r9+r13*1+0x2]
   ff28b:	mov    rdx,r13
   ff28e:	xor    rdx,r14
   ff291:	add    rdx,rbp
   ff294:	and    rcx,rdi
   ff297:	and    rdx,rdi
   ff29a:	shl    rcx,0x4
   ff29e:	shl    rdx,0x4
   ff2a2:	mov    r10,QWORD PTR [rsi+rcx*1]
   ff2a6:	mov    r14,QWORD PTR [rsi+rcx*1+0x8]
   ff2ab:	mov    r15,QWORD PTR [rsi+rdx*1]
   ff2af:	mov    r12,QWORD PTR [rsi+rdx*1+0x8]
   ff2b4:	mov    QWORD PTR [rsi+rcx*1],r15
   ff2b8:	mov    QWORD PTR [rsi+rcx*1+0x8],r12
   ff2bd:	mov    QWORD PTR [rsi+rdx*1],r10
   ff2c1:	movabs r10,0xffffffffffffffc
   ff2cb:	lea    rcx,[r9+r13*1+0x3]
   ff2d0:	lea    r12,[r13+0x4]
   ff2d4:	mov    QWORD PTR [rsi+rdx*1+0x8],r14
   ff2d9:	xor    r13,r10
   ff2dc:	add    r13,rbp
   ff2df:	and    rcx,rdi
   ff2e2:	and    r13,rdi
   ff2e5:	shl    rcx,0x4
   ff2e9:	shl    r13,0x4
   ff2ed:	mov    rdx,QWORD PTR [rsi+rcx*1]
   ff2f1:	mov    r10,QWORD PTR [rsi+rcx*1+0x8]
   ff2f6:	mov    r14,QWORD PTR [rsi+r13*1]
   ff2fa:	mov    r15,QWORD PTR [rsi+r13*1+0x8]
   ff2ff:	mov    QWORD PTR [rsi+rcx*1],r14
   ff303:	mov    QWORD PTR [rsi+rcx*1+0x8],r15
   ff308:	mov    QWORD PTR [rsi+r13*1],rdx
   ff30c:	mov    QWORD PTR [rsi+r13*1+0x8],r10
   ff311:	cmp    r8,r12
   ff314:	jne    ff1f2 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x7b2>
   ff31a:	test   rax,rax
   ff31d:	je     ff371 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x931>
   ff31f:	lea    rcx,[rbp+r12*1+0x0]
   ff324:	sub    rcx,QWORD PTR [rsp]
   ff328:	not    r12
   ff32b:	add    r12,rbp
   ff32e:	mov    rdx,rcx
   ff331:	mov    r10,r12
   ff334:	and    rdx,rdi
   ff337:	and    r10,rdi
   ff33a:	dec    r12
   ff33d:	inc    rcx
   ff340:	shl    rdx,0x4
   ff344:	shl    r10,0x4
   ff348:	dec    rax
   ff34b:	mov    r8,QWORD PTR [rsi+rdx*1]
   ff34f:	mov    r9,QWORD PTR [rsi+rdx*1+0x8]
   ff354:	mov    r11,QWORD PTR [rsi+r10*1]
   ff358:	mov    r14,QWORD PTR [rsi+r10*1+0x8]
   ff35d:	mov    QWORD PTR [rsi+rdx*1],r11
   ff361:	mov    QWORD PTR [rsi+rdx*1+0x8],r14
   ff366:	mov    QWORD PTR [rsi+r10*1],r8
   ff36a:	mov    QWORD PTR [rsi+r10*1+0x8],r9
   ff36f:	jne    ff32e <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x8ee>
   ff371:	mov    r14,QWORD PTR [rsp+0x18]
   ff376:	mov    rcx,QWORD PTR [rsp+0x8]
   ff37b:	mov    r10,QWORD PTR [rsp+0x10]
   ff380:	mov    rax,QWORD PTR [r14]
   ff383:	mov    QWORD PTR [rax+0x108],rbp
   ff38a:	test   rcx,rcx
   ff38d:	je     ff3cd <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x98d>
   ff38f:	dec    QWORD PTR [rcx+0x818]
   ff396:	mov    rbp,QWORD PTR [rsp+0x30]
   ff39b:	mov    rdi,rcx
   ff39e:	jne    fece6 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x2a6>
   ff3a4:	mov    QWORD PTR [rdi+0x880],0x0
   ff3af:	cmp    QWORD PTR [rdi+0x820],0x0
   ff3b7:	jne    fece6 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x2a6>
   ff3bd:	mov    r15,r10
   ff3c0:	call   f3200 <<crossbeam_epoch::internal::Local>::finalize>
   ff3c5:	mov    r10,r15
   ff3c8:	jmp    fece6 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x2a6>
   ff3cd:	mov    rbp,QWORD PTR [rsp+0x30]
   ff3d2:	jmp    fece6 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x2a6>
   ff3d7:	mov    rax,QWORD PTR fs:0x0
   ff3e0:	lea    rdi,[rax-0x40]
   ff3e7:	call   ff4e0 <<std::sys::thread_local::native::lazy::Storage<crossbeam_epoch::collector::LocalHandle, ()>>::get_or_init_slow::<crossbeam_epoch::default::HANDLE::__rust_std_internal_init_fn>>
   ff3ec:	test   rax,rax
   ff3ef:	jne    feada <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x9a>
   ff3f5:	mov    eax,DWORD PTR [rip+0x3d8915]        # 4d7d10 <crossbeam_epoch::default::collector::COLLECTOR+0x8>
   ff3fb:	test   eax,eax
   ff3fd:	jne    ff4b3 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0xa73>
   ff403:	mov    rdi,QWORD PTR [rip+0x3d88fe]        # 4d7d08 <crossbeam_epoch::default::collector::COLLECTOR>
   ff40a:	call   f2fb0 <<crossbeam_epoch::collector::Collector>::register>
   ff40f:	mov    rdx,QWORD PTR [rax+0x820]
   ff416:	mov    rcx,QWORD PTR [rax+0x818]
   ff41d:	lea    rsi,[rdx-0x1]
   ff421:	xor    rdx,0x1
   ff425:	or     rdx,rcx
   ff428:	mov    QWORD PTR [rax+0x820],rsi
   ff42f:	jne    feae4 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0xa4>
   ff435:	mov    rdi,rax
   ff438:	call   f3200 <<crossbeam_epoch::internal::Local>::finalize>
   ff43d:	jmp    feaef <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0xaf>
   ff442:	lea    rcx,[rax+r8*1-0x1]
   ff447:	mov    rdx,0xffffffffffffffff
   ff44e:	add    rax,r8
   ff451:	mov    esi,0x1
   ff456:	lzcnt  rcx,rcx
   ff45b:	shrx   rcx,rdx,rcx
   ff460:	inc    rcx
   ff463:	cmp    rax,0x2
   ff467:	cmovae rsi,rcx
   ff46b:	mov    rdi,QWORD PTR [rsp+0x18]
   ff470:	mov    QWORD PTR [rsp],r8
   ff474:	call   ff7b0 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::resize>
   ff479:	mov    r8,QWORD PTR [rsp]
   ff47d:	jmp    feb57 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x117>
   ff482:	mov    rdi,rax
   ff485:	call   f3200 <<crossbeam_epoch::internal::Local>::finalize>
   ff48a:	mov    QWORD PTR [rsp+0x48],0x0
   ff493:	test   r12,r12
   ff496:	jne    ff1b7 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x777>
   ff49c:	jmp    ff1d0 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x790>
   ff4a1:	lea    rdx,[rip+0x3cfb30]        # 4cefd8 <__frame_dummy_init_array_entry+0x1028>
   ff4a8:	mov    rdi,rsi
   ff4ab:	mov    rsi,rax
   ff4ae:	call   eeaab <core::panicking::panic_bounds_check>
   ff4b3:	call   f35d0 <<crossbeam_epoch::sync::once_lock::OnceLock<crossbeam_epoch::collector::Collector>>::initialize::<<crossbeam_epoch::collector::Collector>::new>>
   ff4b8:	jmp    ff403 <core::iter::adapters::filter::filter_try_fold::<usize, (), core::ops::control_flow::ControlFlow<rayon_core::job::JobRef>, <rayon_core::registry::WorkerThread>::steal::{closure#0}, core::iter::traits::iterator::Iterator::find_map::check<usize, rayon_core::job::JobRef, <rayon_core::registry::WorkerThread>::steal::{closure#1}>::{closure#0}>::{closure#0}+0x9c3>
   ff4bd:	mov    rbx,rax
   ff4c0:	mov    rdi,QWORD PTR [rsp+0x8]
   ff4c5:	call   ff590 <core::ptr::drop_glue::<crossbeam_epoch::guard::Guard>>
   ff4ca:	mov    rdi,rbx
   ff4cd:	call   4ccf60 <_Unwind_Resume@plt>
   ff4d2:	call   f0f63 <core::panicking::panic_in_cleanup>
