
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004659a0 <zopro_core::msm_affine::g2_phi::raw_exclusive>:
  4659a0:	push   %rbp
  4659a1:	push   %r15
  4659a3:	push   %r14
  4659a5:	push   %r13
  4659a7:	push   %r12
  4659a9:	push   %rbx
  4659aa:	sub    $0x358,%rsp
  4659b1:	mov    %rdx,0x288(%rsp)
  4659b9:	mov    %r8,0x50(%rsp)
  4659be:	cmp    %r8,%rdx
  4659c1:	jne    466b11 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1171>
  4659c7:	lea    -0x400(%rdx),%rax
  4659ce:	mov    %rcx,%r15
  4659d1:	mov    %rdx,%rbx
  4659d4:	cmp    $0xfc00,%rax
  4659da:	jae    4659ff <zopro_core::msm_affine::g2_phi::raw_exclusive+0x5f>
  4659dc:	mov    %fs:0xffffffffffffffc0,%rax
  4659e5:	mov    %rsi,0x8(%rsp)
  4659ea:	mov    %rdi,0x180(%rsp)
  4659f2:	test   %rax,%rax
  4659f5:	je     465a1e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x7e>
  4659f7:	add    $0x110,%rax
  4659fd:	jmp    465a23 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x83>
  4659ff:	mov    %rbx,%rdx
  465a02:	mov    %r15,%rcx
  465a05:	mov    %rbx,%r8
  465a08:	add    $0x358,%rsp
  465a0f:	pop    %rbx
  465a10:	pop    %r12
  465a12:	pop    %r13
  465a14:	pop    %r14
  465a16:	pop    %r15
  465a18:	pop    %rbp
  465a19:	jmp    45a980 <zopro_core::msm_affine::g2_horner::raw>
  465a1e:	call   105550 <rayon_core::registry::global_registry>
  465a23:	mov    (%rax),%rax
  465a26:	mov    0x208(%rax),%r13
  465a2d:	lea    -0x4453a1(%rip),%rsi        # 20693 <zmij::STATIC_DATA+0x6153>
  465a34:	lea    0x50(%rsp),%rdi
  465a39:	mov    $0x15,%edx
  465a3e:	call   144240 <std::sys::env::unix::getenv>
  465a43:	mov    0x50(%rsp),%rbp
  465a48:	cmp    $0xffffffffffffffff,%rbp
  465a4c:	je     465b0c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  465a52:	mov    0x58(%rsp),%r14
  465a57:	mov    0x60(%rsp),%r12
  465a5c:	lea    0x50(%rsp),%rdi
  465a61:	mov    %r14,%rsi
  465a64:	mov    %r12,%rdx
  465a67:	call   f1530 <core::str::converts::from_utf8>
  465a6c:	cmpb   $0x0,0x50(%rsp)
  465a71:	je     465aa3 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x103>
  465a73:	test   %rbp,%rbp
  465a76:	jle    465b0c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  465a7c:	cmp    $0x1ffff,%rbp
  465a83:	jbe    465ae5 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x145>
  465a85:	mov    %rbp,%rsi
  465a88:	inc    %rsi
  465a8b:	js     466b71 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x11d1>
  465a91:	cmpb   $0x1,(%r14,%rbp,1)
  465a96:	jne    465ae9 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x149>
  465a98:	mov    %r14,%rdi
  465a9b:	call   *0x8fc47(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  465aa1:	jmp    465b0c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  465aa3:	mov    %r13,0x10(%rsp)
  465aa8:	mov    $0x1,%r13b
  465aab:	cmp    $0x1,%r12
  465aaf:	jne    465ab9 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x119>
  465ab1:	cmpb   $0x30,(%r14)
  465ab5:	setne  %r13b
  465ab9:	test   %rbp,%rbp
  465abc:	je     465afd <zopro_core::msm_affine::g2_phi::raw_exclusive+0x15d>
  465abe:	cmp    $0x1ffff,%rbp
  465ac5:	jbe    465ae7 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x147>
  465ac7:	mov    %rbp,%rsi
  465aca:	inc    %rsi
  465acd:	js     466b76 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x11d6>
  465ad3:	cmpb   $0x1,(%r14,%rbp,1)
  465ad8:	jne    465af4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x154>
  465ada:	mov    %r14,%rdi
  465add:	call   *0x8fc05(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  465ae3:	jmp    465afd <zopro_core::msm_affine::g2_phi::raw_exclusive+0x15d>
  465ae5:	jmp    465ae9 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x149>
  465ae7:	jmp    465af4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x154>
  465ae9:	mov    %r14,%rdi
  465aec:	call   *0x8fbfe(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  465af2:	jmp    465b0c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  465af4:	mov    %r14,%rdi
  465af7:	call   *0x8fbf3(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  465afd:	mov    $0x96,%ebp
  465b02:	test   %r13b,%r13b
  465b05:	mov    0x10(%rsp),%r13
  465b0a:	je     465b11 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x171>
  465b0c:	mov    $0x98,%ebp
  465b11:	lea    0x3(%rbx),%r12
  465b15:	shr    $0x2,%r12
  465b19:	lea    0x50(%rsp),%r14
  465b1e:	mov    $0xb,%edx
  465b23:	mov    %r12,%rsi
  465b26:	mov    %r14,%rdi
  465b29:	call   4199d0 <zopro_core::msm_affine::workspace::top::<zopro_bn254::curve::G2Config>>
  465b2e:	cmpb   $0x0,0x50(%rsp)
  465b33:	je     465bca <zopro_core::msm_affine::g2_phi::raw_exclusive+0x22a>
  465b39:	mov    0x58(%rsp),%rax
  465b3e:	movabs $0x3ffffffffff9ff3f,%rcx
  465b48:	cmp    %rcx,%rax
  465b4b:	ja     465bca <zopro_core::msm_affine::g2_phi::raw_exclusive+0x22a>
  465b4d:	lea    0x483100(,%rax,4),%rax
  465b55:	cmp    $0x302e00,%rax
  465b5b:	jb     465bca <zopro_core::msm_affine::g2_phi::raw_exclusive+0x22a>
  465b5d:	lea    (%rbx,%rbx,4),%rcx
  465b61:	lea    0x40(,%rcx,8),%rcx
  465b69:	add    %rcx,%rax
  465b6c:	jb     465bca <zopro_core::msm_affine::g2_phi::raw_exclusive+0x22a>
  465b6e:	mov    %rbx,%rcx
  465b71:	shr    $0x4,%rcx
  465b75:	mov    %rbx,%rdx
  465b78:	shl    $0x7,%rdx
  465b7c:	imul   $0xb40,%rcx,%rcx
  465b83:	lea    (%rdx,%rbx,8),%rdx
  465b87:	lea    0x1fc0(%rcx,%rdx,1),%rcx
  465b8f:	add    %rax,%rcx
  465b92:	jb     465bca <zopro_core::msm_affine::g2_phi::raw_exclusive+0x22a>
  465b94:	cmp    $0xfffffffffffdffff,%rcx
  465b9b:	ja     465bca <zopro_core::msm_affine::g2_phi::raw_exclusive+0x22a>
  465b9d:	mov    %ebp,%edx
  465b9f:	shl    $0x7,%edx
  465ba2:	mov    %ebp,%eax
  465ba4:	shl    $0x6,%eax
  465ba7:	lea    (%rdx,%rbp,8),%edx
  465baa:	shr    $0x3,%ebp
  465bad:	or     $0x40,%eax
  465bb0:	imul   $0xb40,%rbp,%rsi
  465bb7:	add    %eax,%edx
  465bb9:	lea    0x93d58(%rsi,%rdx,2),%rax
  465bc1:	mul    %r13
  465bc4:	jno    466af6 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1156>
  465bca:	xor    %esi,%esi
  465bcc:	lea    0x190(%rsp),%r13
  465bd4:	mov    %r13,%rdi
  465bd7:	call   21c550 <<zopro_core::preparation::Reservation>::analysis>
  465bdc:	cmpl   $0x1,0x190(%rsp)
  465be4:	je     465cea <zopro_core::msm_affine::g2_phi::raw_exclusive+0x34a>
  465bea:	mov    %fs:0xffffffffffffffc0,%rax
  465bf3:	movq   $0xc,0x188(%rsp)
  465bff:	movq   $0x2,0x148(%rsp)
  465c0b:	test   %rax,%rax
  465c0e:	je     465c18 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x278>
  465c10:	add    $0x110,%rax
  465c16:	jmp    465c1d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x27d>
  465c18:	call   105550 <rayon_core::registry::global_registry>
  465c1d:	mov    (%rax),%rax
  465c20:	mov    0x208(%rax),%rdx
  465c27:	sub    $0x8,%rsp
  465c2b:	lea    0x27(%rsp),%rax
  465c30:	mov    $0x1,%ecx
  465c35:	mov    %rbx,%rdi
  465c38:	xor    %esi,%esi
  465c3a:	mov    %r15,%r8
  465c3d:	mov    %rbx,%r9
  465c40:	push   %rax
  465c41:	call   4680e0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterMutProducer<[u64; 4]>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::g2_phi::hb_raw::{closure#0}>>>
  465c46:	add    $0x10,%rsp
  465c4a:	mov    %rbx,%rax
  465c4d:	shr    $1,%rax
  465c50:	mov    %rbx,%rcx
  465c53:	movq   $0x0,0x118(%rsp)
  465c5f:	movq   $0x8,0x120(%rsp)
  465c6b:	movq   $0x0,0x128(%rsp)
  465c77:	movq   $0xb,0x158(%rsp)
  465c83:	sub    %rax,%rcx
  465c86:	mov    %rcx,0x150(%rsp)
  465c8e:	lea    0x118(%rsp),%rdi
  465c96:	mov    $0xb,%edx
  465c9b:	mov    $0x8,%ecx
  465ca0:	mov    $0xc0,%r8d
  465ca6:	xor    %esi,%esi
  465ca8:	call   1b7280 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  465cad:	mov    0x118(%rsp),%r12
  465cb5:	mov    0x128(%rsp),%rbp
  465cbd:	mov    %r12,%rax
  465cc0:	sub    %rbp,%rax
  465cc3:	cmp    $0xb,%rax
  465cc7:	jae    466b7b <zopro_core::msm_affine::g2_phi::raw_exclusive+0x11db>
  465ccd:	lea    -0x446124(%rip),%rdi        # 1fbb0 <zmij::STATIC_DATA+0x5670>
  465cd4:	lea    0x8cec5(%rip),%rdx        # 4f2ba0 <__frame_dummy_init_array_entry+0x6240>
  465cdb:	mov    $0x2f,%esi
  465ce0:	call   f1800 <core::panicking::panic>
  465ce5:	jmp    46753d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b9d>
  465cea:	mov    0x198(%rsp),%rax
  465cf2:	mov    0x1a0(%rsp),%rbp
  465cfa:	mov    %rax,0x10(%rsp)
  465cff:	lea    0x50(%rsp),%rdi
  465d04:	mov    $0x8,%edx
  465d09:	mov    $0x8,%r8d
  465d0f:	mov    $0x28,%r9d
  465d15:	xor    %esi,%esi
  465d17:	mov    %rbx,%rcx
  465d1a:	call   1e1f50 <<alloc::raw_vec::RawVecInner>::finish_grow>
  465d1f:	cmpb   $0x0,0x50(%rsp)
  465d24:	je     465d40 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x3a0>
  465d26:	mov    0x10(%rsp),%rax
  465d2b:	lock sub %rax,0x9179d(%rip)        # 4f74d0 <zopro_core::preparation::RESERVED>
  465d33:	lock sub %rbp,0x9179d(%rip)        # 4f74d8 <zopro_core::preparation::WORKSPACES>
  465d3b:	jmp    465bea <zopro_core::msm_affine::g2_phi::raw_exclusive+0x24a>
  465d40:	lea    0x40(%rsp),%rax
  465d45:	mov    %rbp,%r14
  465d48:	mov    0x58(%rsp),%rbp
  465d4d:	mov    %rbx,0x68(%rsp)
  465d52:	mov    %r15,0x30(%rsp)
  465d57:	mov    %rbx,0x118(%rsp)
  465d5f:	mov    %rbx,0x38(%rsp)
  465d64:	mov    %rax,0x50(%rsp)
  465d69:	mov    %fs:0xffffffffffffffc0,%rax
  465d72:	mov    %rbp,0x58(%rsp)
  465d77:	mov    %rbx,0x60(%rsp)
  465d7c:	test   %rax,%rax
  465d7f:	je     465d89 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x3e9>
  465d81:	add    $0x110,%rax
  465d87:	jmp    465d8e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x3ee>
  465d89:	call   105550 <rayon_core::registry::global_registry>
  465d8e:	mov    (%rax),%rax
  465d91:	mov    0x208(%rax),%rcx
  465d98:	lea    0x190(%rsp),%r13
  465da0:	lea    0x50(%rsp),%rax
  465da5:	xor    %edx,%edx
  465da7:	mov    $0x1,%r8d
  465dad:	mov    %rbx,%rsi
  465db0:	mov    %r15,%r9
  465db3:	mov    %r13,%rdi
  465db6:	push   %rax
  465db7:	push   %rbx
  465db8:	call   4675f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  465dbd:	add    $0x10,%rsp
  465dc1:	mov    0x1a0(%rsp),%rax
  465dc9:	mov    %rax,0x30(%rsp)
  465dce:	cmp    %rbx,%rax
  465dd1:	jne    466b2c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x118c>
  465dd7:	lea    -0x44536d(%rip),%rsi        # 20a71 <zmij::STATIC_DATA+0x6531>
  465dde:	lea    0x50(%rsp),%rdi
  465de3:	mov    $0x17,%edx
  465de8:	call   144240 <std::sys::env::unix::getenv>
  465ded:	mov    0x50(%rsp),%rsi
  465df2:	mov    %r14,0x28(%rsp)
  465df7:	mov    %rbp,0x20(%rsp)
  465dfc:	cmp    $0xffffffffffffffff,%rsi
  465e00:	je     465e40 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x4a0>
  465e02:	test   %rsi,%rsi
  465e05:	je     465e16 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x476>
  465e07:	mov    0x58(%rsp),%rdi
  465e0c:	mov    $0x1,%edx
  465e11:	call   e9c50 <__rustc::__rust_dealloc>
  465e16:	lea    -0x37209d(%rip),%rax        # f3d80 <<u64 as core::fmt::Display>::fmt>
  465e1d:	mov    %r13,0x50(%rsp)
  465e22:	mov    %rbx,0x190(%rsp)
  465e2a:	mov    %rax,0x58(%rsp)
  465e2f:	lea    -0x44f002(%rip),%rdi        # 16e34 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x5334>
  465e36:	lea    0x50(%rsp),%rsi
  465e3b:	call   140e90 <std::io::stdio::_eprint>
  465e40:	vmovups -0x454148(%rip),%ymm0        # 11d00 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x200>
  465e48:	vmovups -0x454270(%rip),%ymm2        # 11be0 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0xe0>
  465e50:	mov    %r12,0x148(%rsp)
  465e58:	lea    0x190(%rsp),%rax
  465e60:	lea    0x30(%rsp),%rcx
  465e65:	lea    0x50(%rsp),%rsi
  465e6a:	movabs $0x87d20782e4866389,%r12
  465e74:	vmovups -0x4454d4(%rip),%ymm1        # 209a8 <zmij::STATIC_DATA+0x6468>
  465e7c:	lea    0x190(%rsp),%r13
  465e84:	vmovups %ymm0,0x30(%rsp)
  465e8a:	vmovups %ymm0,0x190(%rsp)
  465e93:	vmovups %ymm2,0x50(%rsp)
  465e99:	xor    %r11d,%r11d
  465e9c:	xor    %r10d,%r10d
  465e9f:	xor    %r9d,%r9d
  465ea2:	xor    %edi,%edi
  465ea4:	xor    %r15d,%r15d
  465ea7:	mov    (%rcx),%rdx
  465eaa:	xor    %r14d,%r14d
  465ead:	mulx   (%rax),%r8,%rbp
  465eb2:	adcx   %r8,%r11
  465eb8:	adox   %rbp,%r10
  465ebe:	mulx   0x8(%rax),%r8,%rbp
  465ec4:	adcx   %r8,%r10
  465eca:	adox   %rbp,%r9
  465ed0:	mulx   0x10(%rax),%r8,%rbp
  465ed6:	adcx   %r8,%r9
  465edc:	adox   %rbp,%rdi
  465ee2:	mulx   0x18(%rax),%r8,%rbp
  465ee8:	adcx   %r8,%rdi
  465eee:	adox   %rbp,%r15
  465ef4:	adcx   %r14,%r15
  465efa:	mov    %r11,%rdx
  465efd:	imul   %r12,%rdx
  465f01:	xor    %r14d,%r14d
  465f04:	mulx   (%rsi),%r8,%rbp
  465f09:	adcx   %r8,%r11
  465f0f:	adox   %rbp,%r10
  465f15:	mulx   0x8(%rsi),%r8,%rbp
  465f1b:	adcx   %r8,%r10
  465f21:	adox   %rbp,%r9
  465f27:	mulx   0x10(%rsi),%r8,%rbp
  465f2d:	adcx   %r8,%r9
  465f33:	adox   %rbp,%rdi
  465f39:	mulx   0x18(%rsi),%r8,%rbp
  465f3f:	adcx   %r8,%rdi
  465f45:	adox   %rbp,%r15
  465f4b:	adcx   %r14,%r15
  465f51:	mov    0x8(%rcx),%rdx
  465f55:	xor    %r14d,%r14d
  465f58:	mulx   (%rax),%r8,%rbp
  465f5d:	adcx   %r8,%r10
  465f63:	adox   %rbp,%r9
  465f69:	mulx   0x8(%rax),%r8,%rbp
  465f6f:	adcx   %r8,%r9
  465f75:	adox   %rbp,%rdi
  465f7b:	mulx   0x10(%rax),%r8,%rbp
  465f81:	adcx   %r8,%rdi
  465f87:	adox   %rbp,%r15
  465f8d:	mulx   0x18(%rax),%r8,%rbp
  465f93:	adcx   %r8,%r15
  465f99:	adox   %rbp,%r11
  465f9f:	adcx   %r14,%r11
  465fa5:	mov    %r10,%rdx
  465fa8:	imul   %r12,%rdx
  465fac:	xor    %r14d,%r14d
  465faf:	mulx   (%rsi),%r8,%rbp
  465fb4:	adcx   %r8,%r10
  465fba:	adox   %rbp,%r9
  465fc0:	mulx   0x8(%rsi),%r8,%rbp
  465fc6:	adcx   %r8,%r9
  465fcc:	adox   %rbp,%rdi
  465fd2:	mulx   0x10(%rsi),%r8,%rbp
  465fd8:	adcx   %r8,%rdi
  465fde:	adox   %rbp,%r15
  465fe4:	mulx   0x18(%rsi),%r8,%rbp
  465fea:	adcx   %r8,%r15
  465ff0:	adox   %rbp,%r11
  465ff6:	adcx   %r14,%r11
  465ffc:	mov    0x10(%rcx),%rdx
  466000:	xor    %r14d,%r14d
  466003:	mulx   (%rax),%r8,%rbp
  466008:	adcx   %r8,%r9
  46600e:	adox   %rbp,%rdi
  466014:	mulx   0x8(%rax),%r8,%rbp
  46601a:	adcx   %r8,%rdi
  466020:	adox   %rbp,%r15
  466026:	mulx   0x10(%rax),%r8,%rbp
  46602c:	adcx   %r8,%r15
  466032:	adox   %rbp,%r11
  466038:	mulx   0x18(%rax),%r8,%rbp
  46603e:	adcx   %r8,%r11
  466044:	adox   %rbp,%r10
  46604a:	adcx   %r14,%r10
  466050:	mov    %r9,%rdx
  466053:	imul   %r12,%rdx
  466057:	xor    %r14d,%r14d
  46605a:	mulx   (%rsi),%r8,%rbp
  46605f:	adcx   %r8,%r9
  466065:	adox   %rbp,%rdi
  46606b:	mulx   0x8(%rsi),%r8,%rbp
  466071:	adcx   %r8,%rdi
  466077:	adox   %rbp,%r15
  46607d:	mulx   0x10(%rsi),%r8,%rbp
  466083:	adcx   %r8,%r15
  466089:	adox   %rbp,%r11
  46608f:	mulx   0x18(%rsi),%r8,%rbp
  466095:	adcx   %r8,%r11
  46609b:	adox   %rbp,%r10
  4660a1:	adcx   %r14,%r10
  4660a7:	mov    0x18(%rcx),%rdx
  4660ab:	xor    %r14d,%r14d
  4660ae:	mulx   (%rax),%r8,%rbp
  4660b3:	adcx   %r8,%rdi
  4660b9:	adox   %rbp,%r15
  4660bf:	mulx   0x8(%rax),%r8,%rbp
  4660c5:	adcx   %r8,%r15
  4660cb:	adox   %rbp,%r11
  4660d1:	mulx   0x10(%rax),%r8,%rbp
  4660d7:	adcx   %r8,%r11
  4660dd:	adox   %rbp,%r10
  4660e3:	mulx   0x18(%rax),%r8,%rbp
  4660e9:	adcx   %r8,%r10
  4660ef:	adox   %rbp,%r9
  4660f5:	adcx   %r14,%r9
  4660fb:	mov    %rdi,%rdx
  4660fe:	imul   %r12,%rdx
  466102:	xor    %r14d,%r14d
  466105:	mulx   (%rsi),%r8,%rbp
  46610a:	adcx   %r8,%rdi
  466110:	adox   %rbp,%r15
  466116:	mulx   0x8(%rsi),%r8,%rbp
  46611c:	adcx   %r8,%r15
  466122:	adox   %rbp,%r11
  466128:	mulx   0x10(%rsi),%r8,%rbp
  46612e:	adcx   %r8,%r11
  466134:	adox   %rbp,%r10
  46613a:	mulx   0x18(%rsi),%r8,%rbp
  466140:	adcx   %r8,%r10
  466146:	adox   %rbp,%r9
  46614c:	adcx   %r14,%r9
  466152:	movabs $0x687e956e978e3573,%rax
  46615c:	movabs $0x3c208c16d87cfd47,%r8
  466166:	vmovups %ymm1,0x30(%rsp)
  46616c:	vmovups %ymm2,0x50(%rsp)
  466172:	lea    0x30(%rsp),%r14
  466177:	vmovups -0x4457b7(%rip),%ymm1        # 209c8 <zmij::STATIC_DATA+0x6488>
  46617f:	lea    (%r11,%rax,1),%rsi
  466183:	cmp    %r8,%r15
  466186:	movabs $0x97816a916871ca8d,%rax
  466190:	sbb    $0x0,%rsi
  466194:	setb   %cl
  466197:	cmp    %rax,%r11
  46619a:	movabs $0x47afba497e7ea7a3,%rax
  4661a4:	setb   %dl
  4661a7:	or     %cl,%dl
  4661a9:	lea    (%r10,%rax,1),%rcx
  4661ad:	movabs $0xb85045b68181585d,%rax
  4661b7:	movzbl %dl,%edx
  4661ba:	sub    %rdx,%rcx
  4661bd:	setb   %dl
  4661c0:	cmp    %rax,%r10
  4661c3:	movabs $0xcf9bb18d1ece5fd7,%rax
  4661cd:	setb   %dil
  4661d1:	lea    (%r9,%rax,1),%r12
  4661d5:	movabs $0x30644e72e131a029,%rax
  4661df:	or     %dl,%dil
  4661e2:	movzbl %dil,%edx
  4661e6:	sub    %rdx,%r12
  4661e9:	setb   %dl
  4661ec:	cmp    %rax,%r9
  4661ef:	movabs $0xc3df73e9278302b9,%rax
  4661f9:	setb   %dil
  4661fd:	add    %r15,%rax
  466200:	or     %dl,%dil
  466203:	cmovne %r15,%rax
  466207:	cmovne %r11,%rsi
  46620b:	cmovne %r10,%rcx
  46620f:	cmovne %r9,%r12
  466213:	lea    0x50(%rsp),%r11
  466218:	mov    %rax,0x138(%rsp)
  466220:	mov    %rax,0x190(%rsp)
  466228:	mov    %rsi,0x140(%rsp)
  466230:	mov    %rsi,0x198(%rsp)
  466238:	mov    %rcx,0x1a0(%rsp)
  466240:	mov    %rcx,0x130(%rsp)
  466248:	mov    %r12,0x1a8(%rsp)
  466250:	movabs $0x87d20782e4866389,%rax
  46625a:	xor    %esi,%esi
  46625c:	xor    %edi,%edi
  46625e:	xor    %r8d,%r8d
  466261:	xor    %r15d,%r15d
  466264:	xor    %ecx,%ecx
  466266:	mov    0x0(%r13),%rdx
  46626a:	xor    %ebp,%ebp
  46626c:	mulx   (%r14),%r9,%r10
  466271:	adcx   %r9,%rsi
  466277:	adox   %r10,%rdi
  46627d:	mulx   0x8(%r14),%r9,%r10
  466283:	adcx   %r9,%rdi
  466289:	adox   %r10,%r8
  46628f:	mulx   0x10(%r14),%r9,%r10
  466295:	adcx   %r9,%r8
  46629b:	adox   %r10,%r15
  4662a1:	mulx   0x18(%r14),%r9,%r10
  4662a7:	adcx   %r9,%r15
  4662ad:	adox   %r10,%rcx
  4662b3:	adcx   %rbp,%rcx
  4662b9:	mov    %rsi,%rdx
  4662bc:	imul   %rax,%rdx
  4662c0:	xor    %ebp,%ebp
  4662c2:	mulx   (%r11),%r9,%r10
  4662c7:	adcx   %r9,%rsi
  4662cd:	adox   %r10,%rdi
  4662d3:	mulx   0x8(%r11),%r9,%r10
  4662d9:	adcx   %r9,%rdi
  4662df:	adox   %r10,%r8
  4662e5:	mulx   0x10(%r11),%r9,%r10
  4662eb:	adcx   %r9,%r8
  4662f1:	adox   %r10,%r15
  4662f7:	mulx   0x18(%r11),%r9,%r10
  4662fd:	adcx   %r9,%r15
  466303:	adox   %r10,%rcx
  466309:	adcx   %rbp,%rcx
  46630f:	mov    0x8(%r13),%rdx
  466313:	xor    %ebp,%ebp
  466315:	mulx   (%r14),%r9,%r10
  46631a:	adcx   %r9,%rdi
  466320:	adox   %r10,%r8
  466326:	mulx   0x8(%r14),%r9,%r10
  46632c:	adcx   %r9,%r8
  466332:	adox   %r10,%r15
  466338:	mulx   0x10(%r14),%r9,%r10
  46633e:	adcx   %r9,%r15
  466344:	adox   %r10,%rcx
  46634a:	mulx   0x18(%r14),%r9,%r10
  466350:	adcx   %r9,%rcx
  466356:	adox   %r10,%rsi
  46635c:	adcx   %rbp,%rsi
  466362:	mov    %rdi,%rdx
  466365:	imul   %rax,%rdx
  466369:	xor    %ebp,%ebp
  46636b:	mulx   (%r11),%r9,%r10
  466370:	adcx   %r9,%rdi
  466376:	adox   %r10,%r8
  46637c:	mulx   0x8(%r11),%r9,%r10
  466382:	adcx   %r9,%r8
  466388:	adox   %r10,%r15
  46638e:	mulx   0x10(%r11),%r9,%r10
  466394:	adcx   %r9,%r15
  46639a:	adox   %r10,%rcx
  4663a0:	mulx   0x18(%r11),%r9,%r10
  4663a6:	adcx   %r9,%rcx
  4663ac:	adox   %r10,%rsi
  4663b2:	adcx   %rbp,%rsi
  4663b8:	mov    0x10(%r13),%rdx
  4663bc:	xor    %ebp,%ebp
  4663be:	mulx   (%r14),%r9,%r10
  4663c3:	adcx   %r9,%r8
  4663c9:	adox   %r10,%r15
  4663cf:	mulx   0x8(%r14),%r9,%r10
  4663d5:	adcx   %r9,%r15
  4663db:	adox   %r10,%rcx
  4663e1:	mulx   0x10(%r14),%r9,%r10
  4663e7:	adcx   %r9,%rcx
  4663ed:	adox   %r10,%rsi
  4663f3:	mulx   0x18(%r14),%r9,%r10
  4663f9:	adcx   %r9,%rsi
  4663ff:	adox   %r10,%rdi
  466405:	adcx   %rbp,%rdi
  46640b:	mov    %r8,%rdx
  46640e:	imul   %rax,%rdx
  466412:	xor    %ebp,%ebp
  466414:	mulx   (%r11),%r9,%r10
  466419:	adcx   %r9,%r8
  46641f:	adox   %r10,%r15
  466425:	mulx   0x8(%r11),%r9,%r10
  46642b:	adcx   %r9,%r15
  466431:	adox   %r10,%rcx
  466437:	mulx   0x10(%r11),%r9,%r10
  46643d:	adcx   %r9,%rcx
  466443:	adox   %r10,%rsi
  466449:	mulx   0x18(%r11),%r9,%r10
  46644f:	adcx   %r9,%rsi
  466455:	adox   %r10,%rdi
  46645b:	adcx   %rbp,%rdi
  466461:	mov    0x18(%r13),%rdx
  466465:	xor    %ebp,%ebp
  466467:	mulx   (%r14),%r9,%r10
  46646c:	adcx   %r9,%r15
  466472:	adox   %r10,%rcx
  466478:	mulx   0x8(%r14),%r9,%r10
  46647e:	adcx   %r9,%rcx
  466484:	adox   %r10,%rsi
  46648a:	mulx   0x10(%r14),%r9,%r10
  466490:	adcx   %r9,%rsi
  466496:	adox   %r10,%rdi
  46649c:	mulx   0x18(%r14),%r9,%r10
  4664a2:	adcx   %r9,%rdi
  4664a8:	adox   %r10,%r8
  4664ae:	adcx   %rbp,%r8
  4664b4:	mov    %r15,%rdx
  4664b7:	imul   %rax,%rdx
  4664bb:	xor    %ebp,%ebp
  4664bd:	mulx   (%r11),%r9,%r10
  4664c2:	adcx   %r9,%r15
  4664c8:	adox   %r10,%rcx
  4664ce:	mulx   0x8(%r11),%r9,%r10
  4664d4:	adcx   %r9,%rcx
  4664da:	adox   %r10,%rsi
  4664e0:	mulx   0x10(%r11),%r9,%r10
  4664e6:	adcx   %r9,%rsi
  4664ec:	adox   %r10,%rdi
  4664f2:	mulx   0x18(%r11),%r9,%r10
  4664f8:	adcx   %r9,%rdi
  4664fe:	adox   %r10,%r8
  466504:	adcx   %rbp,%r8
  46650a:	movabs $0x687e956e978e3573,%rax
  466514:	movabs $0x3c208c16d87cfd47,%r11
  46651e:	mov    %r8,0x170(%rsp)
  466526:	mov    %rdi,0x178(%rsp)
  46652e:	mov    %rcx,0x270(%rsp)
  466536:	mov    %rsi,0x278(%rsp)
  46653e:	movabs $0x97816a916871ca8d,%rdi
  466548:	vmovups %ymm1,0x30(%rsp)
  46654e:	vmovups %ymm2,0x50(%rsp)
  466554:	movabs $0x87d20782e4866389,%r14
  46655e:	lea    0x50(%rsp),%r13
  466563:	cmp    %r11,%rcx
  466566:	lea    (%rsi,%rax,1),%rax
  46656a:	sbb    $0x0,%rax
  46656e:	mov    %rax,0x280(%rsp)
  466576:	setb   %dl
  466579:	cmp    %rdi,%rsi
  46657c:	movabs $0x47afba497e7ea7a3,%rax
  466586:	mov    0x178(%rsp),%rsi
  46658e:	setb   %r9b
  466592:	or     %dl,%r9b
  466595:	movzbl %r9b,%edx
  466599:	movabs $0xb85045b68181585d,%r9
  4665a3:	add    %rsi,%rax
  4665a6:	sub    %rdx,%rax
  4665a9:	mov    %rax,0x268(%rsp)
  4665b1:	setb   %dl
  4665b4:	cmp    %r9,%rsi
  4665b7:	movabs $0xcf9bb18d1ece5fd7,%rax
  4665c1:	mov    0x170(%rsp),%rsi
  4665c9:	setb   %r10b
  4665cd:	or     %dl,%r10b
  4665d0:	movzbl %r10b,%edx
  4665d4:	add    %rsi,%rax
  4665d7:	sub    %rdx,%rax
  4665da:	mov    %rax,0x258(%rsp)
  4665e2:	mov    0x138(%rsp),%rax
  4665ea:	mov    0x140(%rsp),%rdi
  4665f2:	mov    0x130(%rsp),%rsi
  4665fa:	setb   0x138(%rsp)
  466602:	mov    %rax,0x190(%rsp)
  46660a:	mov    %rdi,0x198(%rsp)
  466612:	movabs $0xc3df73e9278302b9,%rdi
  46661c:	mov    %rsi,0x1a0(%rsp)
  466624:	mov    %r12,0x1a8(%rsp)
  46662c:	lea    0x30(%rsp),%rsi
  466631:	add    %rdi,%rcx
  466634:	mov    %rcx,0x140(%rsp)
  46663c:	lea    0x190(%rsp),%rcx
  466644:	xor    %r11d,%r11d
  466647:	xor    %eax,%eax
  466649:	xor    %r9d,%r9d
  46664c:	xor    %ebp,%ebp
  46664e:	xor    %r10d,%r10d
  466651:	mov    (%rcx),%rdx
  466654:	xor    %r12d,%r12d
  466657:	mulx   (%rsi),%r8,%r15
  46665c:	adcx   %r8,%r11
  466662:	adox   %r15,%rax
  466668:	mulx   0x8(%rsi),%r8,%r15
  46666e:	adcx   %r8,%rax
  466674:	adox   %r15,%r9
  46667a:	mulx   0x10(%rsi),%r8,%r15
  466680:	adcx   %r8,%r9
  466686:	adox   %r15,%rbp
  46668c:	mulx   0x18(%rsi),%r8,%r15
  466692:	adcx   %r8,%rbp
  466698:	adox   %r15,%r10
  46669e:	adcx   %r12,%r10
  4666a4:	mov    %r11,%rdx
  4666a7:	imul   %r14,%rdx
  4666ab:	xor    %r12d,%r12d
  4666ae:	mulx   0x0(%r13),%r8,%r15
  4666b4:	adcx   %r8,%r11
  4666ba:	adox   %r15,%rax
  4666c0:	mulx   0x8(%r13),%r8,%r15
  4666c6:	adcx   %r8,%rax
  4666cc:	adox   %r15,%r9
  4666d2:	mulx   0x10(%r13),%r8,%r15
  4666d8:	adcx   %r8,%r9
  4666de:	adox   %r15,%rbp
  4666e4:	mulx   0x18(%r13),%r8,%r15
  4666ea:	adcx   %r8,%rbp
  4666f0:	adox   %r15,%r10
  4666f6:	adcx   %r12,%r10
  4666fc:	mov    0x8(%rcx),%rdx
  466700:	xor    %r12d,%r12d
  466703:	mulx   (%rsi),%r8,%r15
  466708:	adcx   %r8,%rax
  46670e:	adox   %r15,%r9
  466714:	mulx   0x8(%rsi),%r8,%r15
  46671a:	adcx   %r8,%r9
  466720:	adox   %r15,%rbp
  466726:	mulx   0x10(%rsi),%r8,%r15
  46672c:	adcx   %r8,%rbp
  466732:	adox   %r15,%r10
  466738:	mulx   0x18(%rsi),%r8,%r15
  46673e:	adcx   %r8,%r10
  466744:	adox   %r15,%r11
  46674a:	adcx   %r12,%r11
  466750:	mov    %rax,%rdx
  466753:	imul   %r14,%rdx
  466757:	xor    %r12d,%r12d
  46675a:	mulx   0x0(%r13),%r8,%r15
  466760:	adcx   %r8,%rax
  466766:	adox   %r15,%r9
  46676c:	mulx   0x8(%r13),%r8,%r15
  466772:	adcx   %r8,%r9
  466778:	adox   %r15,%rbp
  46677e:	mulx   0x10(%r13),%r8,%r15
  466784:	adcx   %r8,%rbp
  46678a:	adox   %r15,%r10
  466790:	mulx   0x18(%r13),%r8,%r15
  466796:	adcx   %r8,%r10
  46679c:	adox   %r15,%r11
  4667a2:	adcx   %r12,%r11
  4667a8:	mov    0x10(%rcx),%rdx
  4667ac:	xor    %r12d,%r12d
  4667af:	mulx   (%rsi),%r8,%r15
  4667b4:	adcx   %r8,%r9
  4667ba:	adox   %r15,%rbp
  4667c0:	mulx   0x8(%rsi),%r8,%r15
  4667c6:	adcx   %r8,%rbp
  4667cc:	adox   %r15,%r10
  4667d2:	mulx   0x10(%rsi),%r8,%r15
  4667d8:	adcx   %r8,%r10
  4667de:	adox   %r15,%r11
  4667e4:	mulx   0x18(%rsi),%r8,%r15
  4667ea:	adcx   %r8,%r11
  4667f0:	adox   %r15,%rax
  4667f6:	adcx   %r12,%rax
  4667fc:	mov    %r9,%rdx
  4667ff:	imul   %r14,%rdx
  466803:	xor    %r12d,%r12d
  466806:	mulx   0x0(%r13),%r8,%r15
  46680c:	adcx   %r8,%r9
  466812:	adox   %r15,%rbp
  466818:	mulx   0x8(%r13),%r8,%r15
  46681e:	adcx   %r8,%rbp
  466824:	adox   %r15,%r10
  46682a:	mulx   0x10(%r13),%r8,%r15
  466830:	adcx   %r8,%r10
  466836:	adox   %r15,%r11
  46683c:	mulx   0x18(%r13),%r8,%r15
  466842:	adcx   %r8,%r11
  466848:	adox   %r15,%rax
  46684e:	adcx   %r12,%rax
  466854:	mov    0x18(%rcx),%rdx
  466858:	xor    %r12d,%r12d
  46685b:	mulx   (%rsi),%r8,%r15
  466860:	adcx   %r8,%rbp
  466866:	adox   %r15,%r10
  46686c:	mulx   0x8(%rsi),%r8,%r15
  466872:	adcx   %r8,%r10
  466878:	adox   %r15,%r11
  46687e:	mulx   0x10(%rsi),%r8,%r15
  466884:	adcx   %r8,%r11
  46688a:	adox   %r15,%rax
  466890:	mulx   0x18(%rsi),%r8,%r15
  466896:	adcx   %r8,%rax
  46689c:	adox   %r15,%r9
  4668a2:	adcx   %r12,%r9
  4668a8:	mov    %rbp,%rdx
  4668ab:	imul   %r14,%rdx
  4668af:	xor    %r12d,%r12d
  4668b2:	mulx   0x0(%r13),%r8,%r15
  4668b8:	adcx   %r8,%rbp
  4668be:	adox   %r15,%r10
  4668c4:	mulx   0x8(%r13),%r8,%r15
  4668ca:	adcx   %r8,%r10
  4668d0:	adox   %r15,%r11
  4668d6:	mulx   0x10(%r13),%r8,%r15
  4668dc:	adcx   %r8,%r11
  4668e2:	adox   %r15,%rax
  4668e8:	mulx   0x18(%r13),%r8,%r15
  4668ee:	adcx   %r8,%rax
  4668f4:	adox   %r15,%r9
  4668fa:	adcx   %r12,%r9
  466900:	mov    %rax,0x260(%rsp)
  466908:	mov    %r9,0x130(%rsp)
  466910:	movabs $0x687e956e978e3573,%rax
  46691a:	movabs $0x3c208c16d87cfd47,%rdx
  466924:	add    %r10,%rdi
  466927:	movabs $0x97816a916871ca8d,%r8
  466931:	movabs $0x47afba497e7ea7a3,%rcx
  46693b:	movabs $0xcf9bb18d1ece5fd7,%rsi
  466945:	movabs $0x30644e72e131a029,%r14
  46694f:	mov    %r11,%r13
  466952:	movq   $0x0,0x158(%rsp)
  46695e:	movq   $0x8,0x160(%rsp)
  46696a:	movq   $0x0,0x168(%rsp)
  466976:	movq   $0x6,0x150(%rsp)
  466982:	add    %r11,%rax
  466985:	cmp    %rdx,%r10
  466988:	mov    0x260(%rsp),%rbp
  466990:	mov    0x130(%rsp),%r12
  466998:	mov    0x170(%rsp),%r15
  4669a0:	mov    0x268(%rsp),%r9
  4669a8:	sbb    $0x0,%rax
  4669ac:	setb   %dl
  4669af:	cmp    %r8,%r11
  4669b2:	mov    0x258(%rsp),%r11
  4669ba:	setb   %r8b
  4669be:	or     %dl,%r8b
  4669c1:	movzbl %r8b,%edx
  4669c5:	add    %rbp,%rcx
  4669c8:	movabs $0xb85045b68181585d,%r8
  4669d2:	sub    %rdx,%rcx
  4669d5:	setb   %dl
  4669d8:	cmp    %r8,%rbp
  4669db:	setb   %r8b
  4669df:	add    %r12,%rsi
  4669e2:	or     %dl,%r8b
  4669e5:	movzbl %r8b,%edx
  4669e9:	sub    %rdx,%rsi
  4669ec:	setb   %dl
  4669ef:	cmp    %r14,%r12
  4669f2:	setb   %r8b
  4669f6:	or     %dl,%r8b
  4669f9:	cmp    %r14,%r15
  4669fc:	mov    0x140(%rsp),%r14
  466a04:	setb   %dl
  466a07:	or     0x138(%rsp),%dl
  466a0e:	mov    0x280(%rsp),%rdx
  466a16:	cmovne 0x270(%rsp),%r14
  466a1f:	cmovne 0x278(%rsp),%rdx
  466a28:	cmovne 0x178(%rsp),%r9
  466a31:	cmovne %r15,%r11
  466a35:	test   %r8b,%r8b
  466a38:	cmovne %r13,%rax
  466a3c:	cmovne %r10,%rdi
  466a40:	cmovne %rbp,%rcx
  466a44:	cmovne %r12,%rsi
  466a48:	mov    %r14,0x190(%rsp)
  466a50:	mov    %rdx,0x198(%rsp)
  466a58:	mov    %r9,0x1a0(%rsp)
  466a60:	mov    %r11,0x1a8(%rsp)
  466a68:	mov    %rdi,0x1b0(%rsp)
  466a70:	mov    %rax,0x1b8(%rsp)
  466a78:	mov    %rcx,0x1c0(%rsp)
  466a80:	mov    %rsi,0x1c8(%rsp)
  466a88:	mov    0x28(%rsp),%r14
  466a8d:	mov    0x10(%rsp),%r12
  466a92:	mov    0x20(%rsp),%rbp
  466a97:	lea    0x158(%rsp),%rdi
  466a9f:	mov    $0x6,%edx
  466aa4:	mov    $0x8,%ecx
  466aa9:	mov    $0xc0,%r8d
  466aaf:	xor    %esi,%esi
  466ab1:	vzeroupper
  466ab4:	call   1b7280 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  466ab9:	mov    0x158(%rsp),%r13
  466ac1:	mov    0x168(%rsp),%r15
  466ac9:	mov    %r13,%rax
  466acc:	sub    %r15,%rax
  466acf:	cmp    $0x5,%rax
  466ad3:	ja     466ed2 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1532>
  466ad9:	lea    -0x446f30(%rip),%rdi        # 1fbb0 <zmij::STATIC_DATA+0x5670>
  466ae0:	lea    0x8c0b9(%rip),%rdx        # 4f2ba0 <__frame_dummy_init_array_entry+0x6240>
  466ae7:	mov    $0x2f,%esi
  466aec:	call   f1800 <core::panicking::panic>
  466af1:	jmp    46753d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b9d>
  466af6:	add    $0x20000,%rcx
  466afd:	mov    %rax,%rdx
  466b00:	xor    %esi,%esi
  466b02:	add    %rcx,%rdx
  466b05:	cmp    %rcx,%rdx
  466b08:	setae  %sil
  466b0c:	jmp    465bcc <zopro_core::msm_affine::g2_phi::raw_exclusive+0x22c>
  466b11:	lea    0x8d1c8(%rip),%r8        # 4f3ce0 <__frame_dummy_init_array_entry+0x7380>
  466b18:	lea    0x288(%rsp),%rdi
  466b20:	lea    0x50(%rsp),%rsi
  466b25:	xor    %edx,%edx
  466b27:	call   f0b82 <core::panicking::assert_failed::<usize, usize>>
  466b2c:	lea    0x118(%rsp),%rax
  466b34:	lea    -0x372dbb(%rip),%r8        # f3d80 <<u64 as core::fmt::Display>::fmt>
  466b3b:	lea    0x30(%rsp),%rcx
  466b40:	mov    %rax,0x50(%rsp)
  466b45:	mov    %r8,0x58(%rsp)
  466b4a:	mov    %rcx,0x60(%rsp)
  466b4f:	mov    %r8,0x68(%rsp)
  466b54:	lea    -0x44fd4c(%rip),%rdi        # 16e0f <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x530f>
  466b5b:	lea    0x8a0b6(%rip),%rdx        # 4f0c18 <__frame_dummy_init_array_entry+0x42b8>
  466b62:	lea    0x50(%rsp),%rsi
  466b67:	call   ef730 <core::panicking::panic_fmt>
  466b6c:	jmp    46753d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b9d>
  466b71:	call   111490 <std::process::abort>
  466b76:	call   111490 <std::process::abort>
  466b7b:	mov    0x8(%rsp),%rdx
  466b80:	mov    0x120(%rsp),%rcx
  466b88:	lea    0x148(%rsp),%rax
  466b90:	lea    0x150(%rsp),%rsi
  466b98:	movq   $0xb,0x1a8(%rsp)
  466ba4:	mov    %r14,0x190(%rsp)
  466bac:	mov    %rax,0x50(%rsp)
  466bb1:	mov    %fs:0xffffffffffffffc0,%rax
  466bba:	mov    %rsi,0x58(%rsp)
  466bbf:	lea    0x188(%rsp),%rsi
  466bc7:	mov    %rdx,0x60(%rsp)
  466bcc:	lea    0x0(%rbp,%rbp,2),%rdx
  466bd1:	mov    %rbx,0x68(%rsp)
  466bd6:	mov    %r15,0x70(%rsp)
  466bdb:	mov    %rbx,0x78(%rsp)
  466be0:	mov    %rsi,0x80(%rsp)
  466be8:	movq   $0x0,0x88(%rsp)
  466bf4:	movq   $0xb,0x90(%rsp)
  466c00:	mov    %rcx,0x8(%rsp)
  466c05:	shl    $0x6,%rdx
  466c09:	add    %rcx,%rdx
  466c0c:	mov    %rdx,0x198(%rsp)
  466c14:	movq   $0xb,0x1a0(%rsp)
  466c20:	test   %rax,%rax
  466c23:	je     466c2d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x128d>
  466c25:	add    $0x110,%rax
  466c2b:	jmp    466c32 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1292>
  466c2d:	call   105550 <rayon_core::registry::global_registry>
  466c32:	mov    (%rax),%rax
  466c35:	mov    0x208(%rax),%rcx
  466c3c:	xor    %edx,%edx
  466c3e:	lea    0x30(%rsp),%rdi
  466c43:	mov    $0xb,%esi
  466c48:	mov    $0x1,%r8d
  466c4e:	xor    %r9d,%r9d
  466c51:	push   %r13
  466c53:	push   $0xb
  466c55:	call   468550 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_phi::hb_raw::{closure#1}>>>
  466c5a:	add    $0x10,%rsp
  466c5e:	mov    0x40(%rsp),%rax
  466c63:	mov    %rax,0x190(%rsp)
  466c6b:	cmp    $0xb,%rax
  466c6f:	jne    4674b1 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b11>
  466c75:	vmovups -0x4476ef(%rip),%zmm0        # 1f590 <zmij::STATIC_DATA+0x5050>
  466c7f:	vmovups -0x447739(%rip),%zmm2        # 1f550 <zmij::STATIC_DATA+0x5010>
  466c89:	vmovups -0x447783(%rip),%zmm1        # 1f510 <zmij::STATIC_DATA+0x4fd0>
  466c93:	mov    0x188(%rsp),%r13
  466c9b:	shl    $0x6,%rbp
  466c9f:	lea    0x0(%rbp,%rbp,2),%r15
  466ca4:	vmovups %zmm0,0x210(%rsp)
  466caf:	vmovups %zmm2,0x1d0(%rsp)
  466cba:	vmovups %zmm1,0x190(%rsp)
  466cc5:	test   %r13,%r13
  466cc8:	je     466da7 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1407>
  466cce:	mov    0x8(%rsp),%rax
  466cd3:	lea    0x50(%rsp),%r14
  466cd8:	lea    0x840(%rax,%r15,1),%rbx
  466ce0:	lea    0x190(%rsp),%r15
  466ce8:	mov    %r13,%rbp
  466ceb:	mov    %r14,%rdi
  466cee:	mov    %r15,%rsi
  466cf1:	vzeroupper
  466cf4:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466cf9:	vmovups 0x50(%rsp),%zmm0
  466d04:	vmovups 0x90(%rsp),%zmm1
  466d0f:	vmovups 0xd0(%rsp),%zmm2
  466d1a:	dec    %rbp
  466d1d:	vmovups %zmm2,0x210(%rsp)
  466d28:	vmovups %zmm1,0x1d0(%rsp)
  466d33:	vmovups %zmm0,0x190(%rsp)
  466d3e:	jne    466ceb <zopro_core::msm_affine::g2_phi::raw_exclusive+0x134b>
  466d40:	add    $0xffffffffffffff40,%rbx
  466d47:	mov    %r14,%rdi
  466d4a:	mov    %r15,%rsi
  466d4d:	mov    %rbx,%rdx
  466d50:	vzeroupper
  466d53:	call   1852b0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  466d58:	vmovups 0x50(%rsp),%zmm0
  466d63:	vmovups 0x90(%rsp),%zmm1
  466d6e:	vmovups 0xd0(%rsp),%zmm2
  466d79:	vmovups %zmm2,0x210(%rsp)
  466d84:	vmovups %zmm1,0x1d0(%rsp)
  466d8f:	vmovups %zmm0,0x190(%rsp)
  466d9a:	cmp    %rbx,0x8(%rsp)
  466d9f:	jne    466ce8 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1348>
  466da5:	jmp    466e25 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1485>
  466da7:	mov    0x8(%rsp),%rax
  466dac:	lea    0x50(%rsp),%rbx
  466db1:	lea    0x190(%rsp),%r14
  466db9:	lea    0x780(%rax),%r13
  466dc0:	lea    0x0(%r13,%r15,1),%rdx
  466dc5:	mov    %rbx,%rdi
  466dc8:	mov    %r14,%rsi
  466dcb:	vzeroupper
  466dce:	call   1852b0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  466dd3:	vmovups 0x50(%rsp),%zmm0
  466dde:	vmovups 0x90(%rsp),%zmm1
  466de9:	vmovups 0xd0(%rsp),%zmm2
  466df4:	add    $0xffffffffffffff40,%r15
  466dfb:	vmovups %zmm2,0x210(%rsp)
  466e06:	vmovups %zmm1,0x1d0(%rsp)
  466e11:	vmovups %zmm0,0x190(%rsp)
  466e1c:	cmp    $0xfffffffffffff7c0,%r15
  466e23:	jne    466dc0 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1420>
  466e25:	vmovups 0x190(%rsp),%zmm0
  466e30:	vmovups 0x1d0(%rsp),%zmm1
  466e3b:	vmovups 0x210(%rsp),%zmm2
  466e46:	mov    0x180(%rsp),%rax
  466e4e:	vmovups %zmm2,0x80(%rax)
  466e55:	vmovups %zmm1,0x40(%rax)
  466e5c:	vmovups %zmm0,(%rax)
  466e62:	test   %r12,%r12
  466e65:	je     466ebd <zopro_core::msm_affine::g2_phi::raw_exclusive+0x151d>
  466e67:	cmp    $0x2aa,%r12
  466e6e:	jbe    466ead <zopro_core::msm_affine::g2_phi::raw_exclusive+0x150d>
  466e70:	shl    $0x6,%r12
  466e74:	movabs $0x7ffffffffffffff8,%rcx
  466e7e:	lea    (%r12,%r12,2),%rax
  466e82:	mov    %rax,%rsi
  466e85:	or     $0x1,%rsi
  466e89:	cmp    %rcx,%rsi
  466e8c:	ja     46753f <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b9f>
  466e92:	mov    0x8(%rsp),%rcx
  466e97:	cmpb   $0x1,(%rcx,%rax,1)
  466e9b:	jne    466eaf <zopro_core::msm_affine::g2_phi::raw_exclusive+0x150f>
  466e9d:	mov    0x8(%rsp),%rdi
  466ea2:	vzeroupper
  466ea5:	call   *0x8e83d(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  466eab:	jmp    466ebd <zopro_core::msm_affine::g2_phi::raw_exclusive+0x151d>
  466ead:	jmp    466eaf <zopro_core::msm_affine::g2_phi::raw_exclusive+0x150f>
  466eaf:	mov    0x8(%rsp),%rdi
  466eb4:	vzeroupper
  466eb7:	call   *0x8e833(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  466ebd:	add    $0x358,%rsp
  466ec4:	pop    %rbx
  466ec5:	pop    %r12
  466ec7:	pop    %r13
  466ec9:	pop    %r14
  466ecb:	pop    %r15
  466ecd:	pop    %rbp
  466ece:	vzeroupper
  466ed1:	ret
  466ed2:	mov    0x8(%rsp),%rsi
  466ed7:	mov    0x160(%rsp),%rcx
  466edf:	lea    0x148(%rsp),%rax
  466ee7:	lea    0x190(%rsp),%rdx
  466eef:	movq   $0x6,0x48(%rsp)
  466ef8:	mov    %rax,0x50(%rsp)
  466efd:	mov    %fs:0xffffffffffffffc0,%rax
  466f06:	mov    %rsi,0x58(%rsp)
  466f0b:	lea    (%r15,%r15,2),%rsi
  466f0f:	mov    %rbx,0x60(%rsp)
  466f14:	mov    %rbp,0x68(%rsp)
  466f19:	mov    %rbx,0x70(%rsp)
  466f1e:	mov    %rdx,0x78(%rsp)
  466f23:	lea    0x50(%rsp),%rdx
  466f28:	movq   $0x0,0x80(%rsp)
  466f34:	movq   $0x6,0x88(%rsp)
  466f40:	mov    %rcx,0x8(%rsp)
  466f45:	mov    %rdx,0x30(%rsp)
  466f4a:	shl    $0x6,%rsi
  466f4e:	add    %rcx,%rsi
  466f51:	mov    %rsi,0x38(%rsp)
  466f56:	movq   $0x6,0x40(%rsp)
  466f5f:	test   %rax,%rax
  466f62:	je     466f6c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x15cc>
  466f64:	add    $0x110,%rax
  466f6a:	jmp    466f80 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x15e0>
  466f6c:	mov    0x28(%rsp),%r14
  466f71:	mov    0x10(%rsp),%r12
  466f76:	mov    0x20(%rsp),%rbp
  466f7b:	call   105550 <rayon_core::registry::global_registry>
  466f80:	mov    (%rax),%rax
  466f83:	mov    0x208(%rax),%rcx
  466f8a:	lea    0x30(%rsp),%rax
  466f8f:	xor    %edx,%edx
  466f91:	lea    0x118(%rsp),%rdi
  466f99:	mov    $0x6,%esi
  466f9e:	mov    $0x1,%r8d
  466fa4:	xor    %r9d,%r9d
  466fa7:	push   %rax
  466fa8:	push   $0x6
  466faa:	mov    0x38(%rsp),%r14
  466faf:	mov    0x20(%rsp),%r12
  466fb4:	mov    0x30(%rsp),%rbp
  466fb9:	call   467dd0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>>
  466fbe:	add    $0x10,%rsp
  466fc2:	mov    0x128(%rsp),%rax
  466fca:	mov    %rax,0x30(%rsp)
  466fcf:	cmp    $0x6,%rax
  466fd3:	jne    4674ee <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b4e>
  466fd9:	vmovups -0x447ad3(%rip),%zmm0        # 1f510 <zmij::STATIC_DATA+0x4fd0>
  466fe3:	vmovups -0x447a9d(%rip),%zmm2        # 1f550 <zmij::STATIC_DATA+0x5010>
  466fed:	vmovups -0x447a67(%rip),%zmm1        # 1f590 <zmij::STATIC_DATA+0x5050>
  466ff7:	mov    0x8(%rsp),%rcx
  466ffc:	shl    $0x6,%r15
  467000:	mov    $0xfffffffffffffb80,%r14
  467007:	lea    0x50(%rsp),%rbp
  46700c:	lea    (%r15,%r15,2),%rax
  467010:	lea    0x290(%rsp),%r15
  467018:	sub    %rax,%r14
  46701b:	lea    0x3c0(%rcx,%rax,1),%r12
  467023:	vmovups %zmm0,0x290(%rsp)
  46702e:	vmovups %zmm2,0x2d0(%rsp)
  467039:	vmovups %zmm1,0x310(%rsp)
  467044:	mov    %rbp,%rdi
  467047:	mov    %r15,%rsi
  46704a:	vzeroupper
  46704d:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467052:	vmovups 0x50(%rsp),%zmm0
  46705d:	vmovups 0x90(%rsp),%zmm1
  467068:	vmovups 0xd0(%rsp),%zmm2
  467073:	vmovups %zmm2,0x310(%rsp)
  46707e:	vmovups %zmm1,0x2d0(%rsp)
  467089:	vmovups %zmm0,0x290(%rsp)
  467094:	mov    %rbp,%rdi
  467097:	mov    %r15,%rsi
  46709a:	vzeroupper
  46709d:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  4670a2:	vmovups 0x50(%rsp),%zmm0
  4670ad:	vmovups 0x90(%rsp),%zmm1
  4670b8:	vmovups 0xd0(%rsp),%zmm2
  4670c3:	vmovups %zmm2,0x310(%rsp)
  4670ce:	vmovups %zmm1,0x2d0(%rsp)
  4670d9:	vmovups %zmm0,0x290(%rsp)
  4670e4:	mov    %rbp,%rdi
  4670e7:	mov    %r15,%rsi
  4670ea:	vzeroupper
  4670ed:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  4670f2:	vmovups 0x50(%rsp),%zmm0
  4670fd:	vmovups 0x90(%rsp),%zmm1
  467108:	vmovups 0xd0(%rsp),%zmm2
  467113:	vmovups %zmm2,0x310(%rsp)
  46711e:	vmovups %zmm1,0x2d0(%rsp)
  467129:	vmovups %zmm0,0x290(%rsp)
  467134:	mov    %rbp,%rdi
  467137:	mov    %r15,%rsi
  46713a:	vzeroupper
  46713d:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467142:	vmovups 0x50(%rsp),%zmm0
  46714d:	vmovups 0x90(%rsp),%zmm1
  467158:	vmovups 0xd0(%rsp),%zmm2
  467163:	vmovups %zmm2,0x310(%rsp)
  46716e:	vmovups %zmm1,0x2d0(%rsp)
  467179:	vmovups %zmm0,0x290(%rsp)
  467184:	mov    %rbp,%rdi
  467187:	mov    %r15,%rsi
  46718a:	vzeroupper
  46718d:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467192:	vmovups 0x50(%rsp),%zmm0
  46719d:	vmovups 0x90(%rsp),%zmm1
  4671a8:	vmovups 0xd0(%rsp),%zmm2
  4671b3:	vmovups %zmm2,0x310(%rsp)
  4671be:	vmovups %zmm1,0x2d0(%rsp)
  4671c9:	vmovups %zmm0,0x290(%rsp)
  4671d4:	mov    %rbp,%rdi
  4671d7:	mov    %r15,%rsi
  4671da:	vzeroupper
  4671dd:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  4671e2:	vmovups 0x50(%rsp),%zmm0
  4671ed:	vmovups 0x90(%rsp),%zmm1
  4671f8:	vmovups 0xd0(%rsp),%zmm2
  467203:	vmovups %zmm2,0x310(%rsp)
  46720e:	vmovups %zmm1,0x2d0(%rsp)
  467219:	vmovups %zmm0,0x290(%rsp)
  467224:	mov    %rbp,%rdi
  467227:	mov    %r15,%rsi
  46722a:	vzeroupper
  46722d:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467232:	vmovups 0x50(%rsp),%zmm0
  46723d:	vmovups 0x90(%rsp),%zmm1
  467248:	vmovups 0xd0(%rsp),%zmm2
  467253:	vmovups %zmm2,0x310(%rsp)
  46725e:	vmovups %zmm1,0x2d0(%rsp)
  467269:	vmovups %zmm0,0x290(%rsp)
  467274:	mov    %rbp,%rdi
  467277:	mov    %r15,%rsi
  46727a:	vzeroupper
  46727d:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467282:	vmovups 0x50(%rsp),%zmm0
  46728d:	vmovups 0x90(%rsp),%zmm1
  467298:	vmovups 0xd0(%rsp),%zmm2
  4672a3:	vmovups %zmm2,0x310(%rsp)
  4672ae:	vmovups %zmm1,0x2d0(%rsp)
  4672b9:	vmovups %zmm0,0x290(%rsp)
  4672c4:	mov    %rbp,%rdi
  4672c7:	mov    %r15,%rsi
  4672ca:	vzeroupper
  4672cd:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  4672d2:	vmovups 0x50(%rsp),%zmm0
  4672dd:	vmovups 0x90(%rsp),%zmm1
  4672e8:	vmovups 0xd0(%rsp),%zmm2
  4672f3:	vmovups %zmm2,0x310(%rsp)
  4672fe:	vmovups %zmm1,0x2d0(%rsp)
  467309:	vmovups %zmm0,0x290(%rsp)
  467314:	mov    %rbp,%rdi
  467317:	mov    %r15,%rsi
  46731a:	vzeroupper
  46731d:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467322:	vmovups 0x50(%rsp),%zmm0
  46732d:	vmovups 0x90(%rsp),%zmm1
  467338:	vmovups 0xd0(%rsp),%zmm2
  467343:	vmovups %zmm2,0x310(%rsp)
  46734e:	vmovups %zmm1,0x2d0(%rsp)
  467359:	vmovups %zmm0,0x290(%rsp)
  467364:	mov    %rbp,%rdi
  467367:	mov    %r15,%rsi
  46736a:	vzeroupper
  46736d:	call   191880 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467372:	vmovups 0x50(%rsp),%zmm0
  46737d:	vmovups 0x90(%rsp),%zmm1
  467388:	vmovups 0xd0(%rsp),%zmm2
  467393:	vmovups %zmm2,0x310(%rsp)
  46739e:	vmovups %zmm1,0x2d0(%rsp)
  4673a9:	vmovups %zmm0,0x290(%rsp)
  4673b4:	mov    %rbp,%rdi
  4673b7:	mov    %r15,%rsi
  4673ba:	mov    %r12,%rdx
  4673bd:	vzeroupper
  4673c0:	call   1852b0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  4673c5:	vmovups 0x50(%rsp),%zmm0
  4673d0:	vmovups 0x90(%rsp),%zmm1
  4673db:	vmovups 0xd0(%rsp),%zmm2
  4673e6:	add    $0xffffffffffffff40,%r12
  4673ed:	add    $0xc0,%r14
  4673f4:	vmovups %zmm2,0x310(%rsp)
  4673ff:	vmovups %zmm1,0x2d0(%rsp)
  46740a:	vmovups %zmm0,0x290(%rsp)
  467415:	jne    467044 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16a4>
  46741b:	test   %r13,%r13
  46741e:	je     46743b <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1a9b>
  467420:	shl    $0x6,%r13
  467424:	lea    0x0(%r13,%r13,2),%rsi
  467429:	mov    0x8(%rsp),%rdi
  46742e:	mov    $0x8,%edx
  467433:	vzeroupper
  467436:	call   e9c50 <__rustc::__rust_dealloc>
  46743b:	shl    $0x3,%rbx
  46743f:	lea    (%rbx,%rbx,4),%rsi
  467443:	mov    0x20(%rsp),%rdi
  467448:	mov    $0x8,%edx
  46744d:	vzeroupper
  467450:	call   e9c50 <__rustc::__rust_dealloc>
  467455:	mov    0x10(%rsp),%rax
  46745a:	lock sub %rax,0x9006e(%rip)        # 4f74d0 <zopro_core::preparation::RESERVED>
  467462:	mov    0x28(%rsp),%rax
  467467:	lock sub %rax,0x90069(%rip)        # 4f74d8 <zopro_core::preparation::WORKSPACES>
  46746f:	vmovups 0x290(%rsp),%zmm0
  46747a:	vmovups 0x2d0(%rsp),%zmm1
  467485:	vmovups 0x310(%rsp),%zmm2
  467490:	mov    0x180(%rsp),%rax
  467498:	vmovups %zmm2,0x80(%rax)
  46749f:	vmovups %zmm1,0x40(%rax)
  4674a6:	vmovups %zmm0,(%rax)
  4674ac:	jmp    466ebd <zopro_core::msm_affine::g2_phi::raw_exclusive+0x151d>
  4674b1:	lea    0x158(%rsp),%rax
  4674b9:	lea    -0x373740(%rip),%rcx        # f3d80 <<u64 as core::fmt::Display>::fmt>
  4674c0:	mov    %rax,0x50(%rsp)
  4674c5:	mov    %rcx,0x58(%rsp)
  4674ca:	mov    %r13,0x60(%rsp)
  4674cf:	mov    %rcx,0x68(%rsp)
  4674d4:	lea    -0x4506cc(%rip),%rdi        # 16e0f <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x530f>
  4674db:	lea    0x89736(%rip),%rdx        # 4f0c18 <__frame_dummy_init_array_entry+0x42b8>
  4674e2:	lea    0x50(%rsp),%rsi
  4674e7:	call   ef730 <core::panicking::panic_fmt>
  4674ec:	jmp    46753d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b9d>
  4674ee:	lea    0x150(%rsp),%rax
  4674f6:	lea    -0x37377d(%rip),%r8        # f3d80 <<u64 as core::fmt::Display>::fmt>
  4674fd:	lea    0x30(%rsp),%rcx
  467502:	mov    %rax,0x50(%rsp)
  467507:	mov    %r8,0x58(%rsp)
  46750c:	mov    %rcx,0x60(%rsp)
  467511:	mov    %r8,0x68(%rsp)
  467516:	mov    0x28(%rsp),%r14
  46751b:	mov    0x10(%rsp),%r12
  467520:	mov    0x20(%rsp),%rbp
  467525:	lea    -0x45071d(%rip),%rdi        # 16e0f <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x530f>
  46752c:	lea    0x896e5(%rip),%rdx        # 4f0c18 <__frame_dummy_init_array_entry+0x42b8>
  467533:	lea    0x50(%rsp),%rsi
  467538:	call   ef730 <core::panicking::panic_fmt>
  46753d:	ud2
  46753f:	vzeroupper
  467542:	call   111490 <std::process::abort>
  467547:	mov    0x10(%rsp),%r12
  46754c:	mov    %rax,%r15
  46754f:	jmp    46757b <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1bdb>
  467551:	mov    %rax,%r15
  467554:	mov    0x158(%rsp),%rax
  46755c:	test   %rax,%rax
  46755f:	je     46757b <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1bdb>
  467561:	mov    0x160(%rsp),%rdi
  467569:	shl    $0x6,%rax
  46756d:	lea    (%rax,%rax,2),%rsi
  467571:	mov    $0x8,%edx
  467576:	call   e9c50 <__rustc::__rust_dealloc>
  46757b:	shl    $0x3,%rbx
  46757f:	lea    (%rbx,%rbx,4),%rsi
  467583:	mov    $0x8,%edx
  467588:	mov    %rbp,%rdi
  46758b:	call   e9c50 <__rustc::__rust_dealloc>
  467590:	lock sub %r12,0x8ff38(%rip)        # 4f74d0 <zopro_core::preparation::RESERVED>
  467598:	lock sub %r14,0x8ff38(%rip)        # 4f74d8 <zopro_core::preparation::WORKSPACES>
  4675a0:	mov    %r15,%rdi
  4675a3:	call   4eb910 <_Unwind_Resume@plt>
  4675a8:	mov    %rax,%r15
  4675ab:	mov    0x118(%rsp),%rax
  4675b3:	test   %rax,%rax
  4675b6:	jne    4675c0 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1c20>
  4675b8:	mov    %r15,%rdi
  4675bb:	call   4eb910 <_Unwind_Resume@plt>
  4675c0:	mov    0x120(%rsp),%rdi
  4675c8:	shl    $0x6,%rax
  4675cc:	lea    (%rax,%rax,2),%rsi
  4675d0:	mov    $0x8,%edx
  4675d5:	call   e9c50 <__rustc::__rust_dealloc>
  4675da:	mov    %r15,%rdi
  4675dd:	call   4eb910 <_Unwind_Resume@plt>
