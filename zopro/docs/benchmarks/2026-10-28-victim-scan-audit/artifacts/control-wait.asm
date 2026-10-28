
/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000000fd920 <<rayon_core::registry::WorkerThread>::wait_until_cold>:
   fd920:	push   rbp
   fd921:	push   r15
   fd923:	push   r14
   fd925:	push   r13
   fd927:	push   r12
   fd929:	push   rbx
   fd92a:	sub    rsp,0x58
   fd92e:	mov    rax,QWORD PTR [rsi]
   fd931:	cmp    rax,0x3
   fd935:	je     fdf57 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x637>
   fd93b:	mov    r14,QWORD PTR [rip+0x3d85f6]        # 4d5f38 <sched_yield@GLIBC_2.2.5>
   fd942:	movabs r13,0x100000000
   fd94c:	mov    r15,rsi
   fd94f:	mov    rbx,rdi
   fd952:	mov    QWORD PTR [rsp],rdi
   fd956:	mov    QWORD PTR [rsp+0x8],rsi
   fd95b:	mov    rdi,rbx
   fd95e:	call   fdfa0 <<rayon_core::registry::WorkerThread>::take_local_job>
   fd963:	test   rax,rax
   fd966:	je     fd980 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x60>
   fd968:	mov    rdi,rdx
   fd96b:	call   rax
   fd96d:	jmp    fdf15 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5f5>
   fd972:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fd980:	mov    rcx,QWORD PTR [rbx+0x110]
   fd987:	mov    rax,QWORD PTR [rbx+0x100]
   fd98e:	lock add QWORD PTR [rcx+0x1f0],0x10000
   fd99a:	mov    QWORD PTR [rsp+0x20],rax
   fd99f:	mov    DWORD PTR [rsp+0x30],0x0
   fd9a7:	mov    QWORD PTR [rsp+0x28],0xffffffffffffffff
   fd9b0:	jmp    fd9cd <<rayon_core::registry::WorkerThread>::wait_until_cold+0xad>
   fd9b2:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fd9c0:	call   r14
   fd9c3:	inc    ebx
   fd9c5:	mov    DWORD PTR [rsp+0x30],ebx
   fd9c9:	mov    rbx,QWORD PTR [rsp]
   fd9cd:	mov    rax,QWORD PTR [r15]
   fd9d0:	cmp    rax,0x3
   fd9d4:	je     fdf24 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x604>
   fd9da:	mov    rdi,rbx
   fd9dd:	call   fdfa0 <<rayon_core::registry::WorkerThread>::take_local_job>
   fd9e2:	mov    r12,rax
   fd9e5:	test   rax,rax
   fd9e8:	je     fda10 <<rayon_core::registry::WorkerThread>::wait_until_cold+0xf0>
   fd9ea:	mov    rbp,rdx
   fd9ed:	mov    rdi,QWORD PTR [rbx+0x110]
   fd9f4:	test   r12,r12
   fd9f7:	je     fde2b <<rayon_core::registry::WorkerThread>::wait_until_cold+0x50b>
   fd9fd:	jmp    fdee3 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5c3>
   fda02:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fda10:	mov    r15,QWORD PTR [rbx+0x110]
   fda17:	mov    r13,QWORD PTR [r15+0x208]
   fda1e:	cmp    r13,0x2
   fda22:	jb     fdbbf <<rayon_core::registry::WorkerThread>::wait_until_cold+0x29f>
   fda28:	mov    rax,QWORD PTR [r15+0x200]
   fda2f:	mov    QWORD PTR [rsp+0x18],rax
   fda34:	mov    rax,QWORD PTR [rbx+0x108]
   fda3b:	mov    QWORD PTR [rsp+0x10],0x0
   fda44:	mov    rcx,rax
   fda47:	shr    rcx,0xc
   fda4b:	xor    rcx,rax
   fda4e:	mov    rdx,rcx
   fda51:	shl    rdx,0x19
   fda55:	xor    rdx,rcx
   fda58:	movabs rcx,0x2545f4914f6cdd1d
   fda62:	mov    rax,rdx
   fda65:	shr    rax,0x1b
   fda69:	xor    rax,rdx
   fda6c:	xor    edx,edx
   fda6e:	mov    QWORD PTR [rbx+0x108],rax
   fda75:	imul   rax,rcx
   fda79:	div    r13
   fda7c:	mov    r15,rdx
   fda7f:	mov    QWORD PTR [rsp+0x50],rdx
   fda84:	jmp    fda9f <<rayon_core::registry::WorkerThread>::wait_until_cold+0x17f>
   fda86:	mov    al,0x1
   fda88:	mov    QWORD PTR [rsp+0x10],rax
   fda8d:	nop    DWORD PTR [rax]
   fda90:	xor    r12d,r12d
   fda93:	inc    r15
   fda96:	test   r12,r12
   fda99:	jne    fde0c <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4ec>
   fda9f:	cmp    r15,r13
   fdaa2:	je     fdb00 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x1e0>
   fdaa4:	cmp    r15,QWORD PTR [rbx+0x100]
   fdaab:	je     fda90 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x170>
   fdaad:	cmp    r15,r13
   fdab0:	jae    fdf66 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x646>
   fdab6:	lea    rsi,[r15+r15*2]
   fdaba:	shl    rsi,0x4
   fdabe:	add    rsi,QWORD PTR [rsp+0x18]
   fdac3:	lea    rdi,[rsp+0x38]
   fdac8:	call   fe7b0 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal>
   fdacd:	mov    rax,QWORD PTR [rsp+0x38]
   fdad2:	test   rax,rax
   fdad5:	je     fda90 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x170>
   fdad7:	cmp    eax,0x1
   fdada:	jne    fda86 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x166>
   fdadc:	mov    r12,QWORD PTR [rsp+0x40]
   fdae1:	mov    rbp,QWORD PTR [rsp+0x48]
   fdae6:	inc    r15
   fdae9:	test   r12,r12
   fdaec:	je     fda9f <<rayon_core::registry::WorkerThread>::wait_until_cold+0x17f>
   fdaee:	jmp    fde0c <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4ec>
   fdaf3:	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdb00:	mov    rbx,QWORD PTR [rsp+0x18]
   fdb05:	xor    r15d,r15d
   fdb08:	jmp    fdb33 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x213>
   fdb0a:	mov    al,0x1
   fdb0c:	mov    QWORD PTR [rsp+0x10],rax
   fdb11:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdb20:	xor    r12d,r12d
   fdb23:	add    rbx,0x30
   fdb27:	inc    r15
   fdb2a:	test   r12,r12
   fdb2d:	jne    fde08 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4e8>
   fdb33:	cmp    QWORD PTR [rsp+0x50],r15
   fdb38:	je     fdb90 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x270>
   fdb3a:	mov    rax,QWORD PTR [rsp]
   fdb3e:	cmp    r15,QWORD PTR [rax+0x100]
   fdb45:	je     fdb20 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x200>
   fdb47:	cmp    r15,r13
   fdb4a:	jae    fdf66 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x646>
   fdb50:	lea    rdi,[rsp+0x38]
   fdb55:	mov    rsi,rbx
   fdb58:	call   fe7b0 <<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal>
   fdb5d:	mov    rax,QWORD PTR [rsp+0x38]
   fdb62:	test   rax,rax
   fdb65:	je     fdb20 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x200>
   fdb67:	cmp    eax,0x1
   fdb6a:	jne    fdb0a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x1ea>
   fdb6c:	mov    r12,QWORD PTR [rsp+0x40]
   fdb71:	mov    rbp,QWORD PTR [rsp+0x48]
   fdb76:	add    rbx,0x30
   fdb7a:	inc    r15
   fdb7d:	test   r12,r12
   fdb80:	je     fdb33 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x213>
   fdb82:	jmp    fde08 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4e8>
   fdb87:	nop    WORD PTR [rax+rax*1+0x0]
   fdb90:	test   BYTE PTR [rsp+0x10],0x1
   fdb95:	mov    rbx,QWORD PTR [rsp]
   fdb99:	jne    fda34 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x114>
   fdb9f:	mov    r15,QWORD PTR [rbx+0x110]
   fdba6:	jmp    fdbbf <<rayon_core::registry::WorkerThread>::wait_until_cold+0x29f>
   fdba8:	nop    DWORD PTR [rax+rax*1+0x0]
   fdbb0:	lock cmpxchg QWORD PTR [r15+0x80],r12
   fdbb9:	je     fdc7c <<rayon_core::registry::WorkerThread>::wait_until_cold+0x35c>
   fdbbf:	mov    rax,QWORD PTR [r15+0x80]
   fdbc6:	mov    r13,QWORD PTR [r15+0x88]
   fdbcd:	mov    rcx,rax
   fdbd0:	shr    rcx,1
   fdbd3:	mov    ebx,ecx
   fdbd5:	and    ebx,0x3f
   fdbd8:	cmp    rbx,0x3f
   fdbdc:	jne    fdc40 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x320>
   fdbde:	xor    ebp,ebp
   fdbe0:	jmp    fdc11 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x2f1>
   fdbe2:	data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdbf0:	inc    ebp
   fdbf2:	mov    rax,QWORD PTR [r15+0x80]
   fdbf9:	mov    r13,QWORD PTR [r15+0x88]
   fdc00:	mov    rcx,rax
   fdc03:	shr    rcx,1
   fdc06:	mov    ebx,ecx
   fdc08:	and    ebx,0x3f
   fdc0b:	cmp    rbx,0x3f
   fdc0f:	jne    fdc40 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x320>
   fdc11:	cmp    ebp,0x6
   fdc14:	ja     fdc30 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x310>
   fdc16:	mov    eax,0x1
   fdc1b:	nop    DWORD PTR [rax+rax*1+0x0]
   fdc20:	shrx   ecx,eax,ebp
   fdc25:	inc    eax
   fdc27:	pause
   fdc29:	test   ecx,ecx
   fdc2b:	je     fdc20 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x300>
   fdc2d:	jmp    fdbf0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x2d0>
   fdc2f:	nop
   fdc30:	call   r14
   fdc33:	cmp    ebp,0xb
   fdc36:	jb     fdbf0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x2d0>
   fdc38:	jmp    fdbf2 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x2d2>
   fdc3a:	nop    WORD PTR [rax+rax*1+0x0]
   fdc40:	lea    r12,[rax+0x2]
   fdc44:	test   al,0x1
   fdc46:	jne    fdbb0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x290>
   fdc4c:	lock or DWORD PTR [rsp-0x40],0x0
   fdc52:	mov    rdx,QWORD PTR [r15+0x100]
   fdc59:	mov    rsi,rdx
   fdc5c:	shr    rsi,1
   fdc5f:	cmp    rcx,rsi
   fdc62:	je     fdd0a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x3ea>
   fdc68:	xor    rdx,rax
   fdc6b:	xor    ecx,ecx
   fdc6d:	cmp    rdx,0x7f
   fdc71:	seta   cl
   fdc74:	or     r12,rcx
   fdc77:	jmp    fdbb0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x290>
   fdc7c:	cmp    ebx,0x3e
   fdc7f:	jne    fdd1b <<rayon_core::registry::WorkerThread>::wait_until_cold+0x3fb>
   fdc85:	mov    rax,QWORD PTR [r13+0x0]
   fdc89:	test   rax,rax
   fdc8c:	je     fde5d <<rayon_core::registry::WorkerThread>::wait_until_cold+0x53d>
   fdc92:	mov    rcx,QWORD PTR [rax]
   fdc95:	and    r12,0xfffffffffffffffe
   fdc99:	xor    edx,edx
   fdc9b:	mov    QWORD PTR [r15+0x88],rax
   fdca2:	test   rcx,rcx
   fdca5:	setne  dl
   fdca8:	lea    rcx,[r12+rdx*1+0x2]
   fdcad:	mov    QWORD PTR [r15+0x80],rcx
   fdcb4:	mov    rax,QWORD PTR [r13+0x5e8]
   fdcbb:	mov    r15,QWORD PTR [rsp+0x8]
   fdcc0:	test   al,0x1
   fdcc2:	jne    fdd7a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x45a>
   fdcc8:	xor    ebp,ebp
   fdcca:	jmp    fdce1 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x3c1>
   fdccc:	nop    DWORD PTR [rax+0x0]
   fdcd0:	inc    ebp
   fdcd2:	mov    rax,QWORD PTR [r13+0x5e8]
   fdcd9:	test   al,0x1
   fdcdb:	jne    fdd7a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x45a>
   fdce1:	cmp    ebp,0x6
   fdce4:	ja     fdd00 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x3e0>
   fdce6:	mov    eax,0x1
   fdceb:	nop    DWORD PTR [rax+rax*1+0x0]
   fdcf0:	shrx   ecx,eax,ebp
   fdcf5:	inc    eax
   fdcf7:	pause
   fdcf9:	test   ecx,ecx
   fdcfb:	je     fdcf0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x3d0>
   fdcfd:	jmp    fdcd0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x3b0>
   fdcff:	nop
   fdd00:	call   r14
   fdd03:	cmp    ebp,0xb
   fdd06:	jb     fdcd0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x3b0>
   fdd08:	jmp    fdcd2 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x3b2>
   fdd0a:	mov    r15,QWORD PTR [rsp+0x8]
   fdd0f:	mov    rbx,QWORD PTR [rsp]
   fdd13:	xor    r12d,r12d
   fdd16:	jmp    fde11 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4f1>
   fdd1b:	lea    rax,[rbx+rbx*2]
   fdd1f:	lea    r15,[r13+rax*8+0x8]
   fdd24:	mov    rax,QWORD PTR [r13+rax*8+0x18]
   fdd29:	test   al,0x1
   fdd2b:	jne    fdd8a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x46a>
   fdd2d:	xor    ebp,ebp
   fdd2f:	jmp    fdd4a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x42a>
   fdd31:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdd40:	inc    ebp
   fdd42:	mov    rax,QWORD PTR [r15+0x10]
   fdd46:	test   al,0x1
   fdd48:	jne    fdd8a <<rayon_core::registry::WorkerThread>::wait_until_cold+0x46a>
   fdd4a:	cmp    ebp,0x6
   fdd4d:	ja     fdd70 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x450>
   fdd4f:	mov    eax,0x1
   fdd54:	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdd60:	shrx   ecx,eax,ebp
   fdd65:	inc    eax
   fdd67:	pause
   fdd69:	test   ecx,ecx
   fdd6b:	je     fdd60 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x440>
   fdd6d:	jmp    fdd40 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x420>
   fdd6f:	nop
   fdd70:	call   r14
   fdd73:	cmp    ebp,0xb
   fdd76:	jb     fdd40 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x420>
   fdd78:	jmp    fdd42 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x422>
   fdd7a:	mov    r12,QWORD PTR [r13+0x5d8]
   fdd81:	mov    rbp,QWORD PTR [r13+0x5e0]
   fdd88:	jmp    fddc0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4a0>
   fdd8a:	mov    r12,QWORD PTR [r15]
   fdd8d:	mov    rbp,QWORD PTR [r15+0x8]
   fdd91:	mov    rax,QWORD PTR [r15+0x10]
   fdd95:	data16 cs nop WORD PTR [rax+rax*1+0x0]
   fdda0:	mov    rcx,rax
   fdda3:	or     rcx,0x2
   fdda7:	lock cmpxchg QWORD PTR [r15+0x10],rcx
   fddad:	jne    fdda0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x480>
   fddaf:	mov    r15,QWORD PTR [rsp+0x8]
   fddb4:	test   al,0x4
   fddb6:	je     fddf3 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4d3>
   fddb8:	nop    DWORD PTR [rax+rax*1+0x0]
   fddc0:	test   rbx,rbx
   fddc3:	je     fddf9 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4d9>
   fddc5:	dec    rbx
   fddc8:	lea    rax,[rbx+rbx*2]
   fddcc:	mov    rcx,QWORD PTR [r13+rax*8+0x18]
   fddd1:	test   cl,0x2
   fddd4:	jne    fddc0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4a0>
   fddd6:	lea    rcx,[r13+rax*8+0x0]
   fdddb:	mov    rax,QWORD PTR [rcx+0x18]
   fdddf:	nop
   fdde0:	mov    rdx,rax
   fdde3:	or     rdx,0x4
   fdde7:	lock cmpxchg QWORD PTR [rcx+0x18],rdx
   fdded:	jne    fdde0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4c0>
   fddef:	test   al,0x2
   fddf1:	jne    fddc0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4a0>
   fddf3:	mov    rbx,QWORD PTR [rsp]
   fddf7:	jmp    fde11 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4f1>
   fddf9:	mov    rdi,r13
   fddfc:	call   QWORD PTR [rip+0x3d8096]        # 4d5e98 <free@GLIBC_2.2.5>
   fde02:	mov    rbx,QWORD PTR [rsp]
   fde06:	jmp    fde11 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x4f1>
   fde08:	mov    rbx,QWORD PTR [rsp]
   fde0c:	mov    r15,QWORD PTR [rsp+0x8]
   fde11:	movabs r13,0x100000000
   fde1b:	mov    rdi,QWORD PTR [rbx+0x110]
   fde22:	test   r12,r12
   fde25:	jne    fdee3 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5c3>
   fde2b:	mov    ebx,DWORD PTR [rsp+0x30]
   fde2f:	lea    rax,[rdi+0x1d8]
   fde36:	cmp    ebx,0x20
   fde39:	jb     fd9c0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0xa0>
   fde3f:	je     fdeaa <<rayon_core::registry::WorkerThread>::wait_until_cold+0x58a>
   fde41:	mov    rbx,QWORD PTR [rsp]
   fde45:	lea    rsi,[rsp+0x20]
   fde4a:	mov    rdi,rax
   fde4d:	mov    rdx,r15
   fde50:	mov    rcx,rbx
   fde53:	call   fe1b0 <<rayon_core::sleep::Sleep>::sleep::<<rayon_core::registry::WorkerThread>::wait_until_cold::{closure#0}>>
   fde58:	jmp    fd9cd <<rayon_core::registry::WorkerThread>::wait_until_cold+0xad>
   fde5d:	xor    ebp,ebp
   fde5f:	jmp    fde7f <<rayon_core::registry::WorkerThread>::wait_until_cold+0x55f>
   fde61:	data16 data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   fde70:	inc    ebp
   fde72:	mov    rax,QWORD PTR [r13+0x0]
   fde76:	test   rax,rax
   fde79:	jne    fdc92 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x372>
   fde7f:	cmp    ebp,0x6
   fde82:	ja     fdea0 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x580>
   fde84:	mov    eax,0x1
   fde89:	nop    DWORD PTR [rax+0x0]
   fde90:	shrx   ecx,eax,ebp
   fde95:	inc    eax
   fde97:	pause
   fde99:	test   ecx,ecx
   fde9b:	je     fde90 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x570>
   fde9d:	jmp    fde70 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x550>
   fde9f:	nop
   fdea0:	call   r14
   fdea3:	cmp    ebp,0xb
   fdea6:	jb     fde70 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x550>
   fdea8:	jmp    fde72 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x552>
   fdeaa:	mov    rbx,QWORD PTR [rsp]
   fdeae:	mov    rax,QWORD PTR [rdi+0x1f0]
   fdeb5:	test   rax,r13
   fdeb8:	je     fdecb <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5ab>
   fdeba:	lea    rcx,[rax+r13*1]
   fdebe:	lock cmpxchg QWORD PTR [rdi+0x1f0],rcx
   fdec7:	jne    fdeae <<rayon_core::registry::WorkerThread>::wait_until_cold+0x58e>
   fdec9:	jmp    fdece <<rayon_core::registry::WorkerThread>::wait_until_cold+0x5ae>
   fdecb:	mov    rcx,rax
   fdece:	shr    rcx,0x20
   fded2:	mov    QWORD PTR [rsp+0x28],rcx
   fded7:	inc    DWORD PTR [rsp+0x30]
   fdedb:	call   r14
   fdede:	jmp    fd9cd <<rayon_core::registry::WorkerThread>::wait_until_cold+0xad>
   fdee3:	mov    rax,0xffffffffffff0000
   fdeea:	lock xadd QWORD PTR [rdi+0x1f0],rax
   fdef3:	movzx  esi,ax
   fdef6:	mov    eax,0x2
   fdefb:	cmp    rsi,0x2
   fdeff:	cmovae rsi,rax
   fdf03:	add    rdi,0x1d8
   fdf0a:	call   fe160 <<rayon_core::sleep::Sleep>::wake_any_threads>
   fdf0f:	mov    rdi,rbp
   fdf12:	call   r12
   fdf15:	mov    rax,QWORD PTR [r15]
   fdf18:	cmp    rax,0x3
   fdf1c:	jne    fd95b <<rayon_core::registry::WorkerThread>::wait_until_cold+0x3b>
   fdf22:	jmp    fdf57 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x637>
   fdf24:	mov    rdi,QWORD PTR [rbx+0x110]
   fdf2b:	mov    rax,0xffffffffffff0000
   fdf32:	mov    esi,0x2
   fdf37:	lock xadd QWORD PTR [rdi+0x1f0],rax
   fdf40:	movzx  eax,ax
   fdf43:	cmp    rax,0x2
   fdf47:	cmovb  rsi,rax
   fdf4b:	add    rdi,0x1d8
   fdf52:	call   fe160 <<rayon_core::sleep::Sleep>::wake_any_threads>
   fdf57:	add    rsp,0x58
   fdf5b:	pop    rbx
   fdf5c:	pop    r12
   fdf5e:	pop    r13
   fdf60:	pop    r14
   fdf62:	pop    r15
   fdf64:	pop    rbp
   fdf65:	ret
   fdf66:	lea    rdx,[rip+0x3d047b]        # 4ce3e8 <__frame_dummy_init_array_entry+0x1028>
   fdf6d:	mov    rdi,r15
   fdf70:	mov    rsi,r13
   fdf73:	call   eeb1b <core::panicking::panic_bounds_check>
   fdf78:	ud2
   fdf7a:	jmp    fdf82 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x662>
   fdf7c:	jmp    fdf82 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x662>
   fdf7e:	jmp    fdf82 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x662>
   fdf80:	jmp    fdf82 <<rayon_core::registry::WorkerThread>::wait_until_cold+0x662>
   fdf82:	lea    rdi,[rip+0xfffffffffff1a795]        # 1871e <miniz_oxide::inflate::core::reverse_bits::REVERSED_BITS_LOOKUP+0x6e2>
   fdf89:	mov    esi,0x57
   fdf8e:	call   13f1e0 <std::io::stdio::_eprint>
   fdf93:	call   10f7e0 <std::process::abort>
   fdf98:	call   f0fd3 <core::panicking::panic_in_cleanup>
