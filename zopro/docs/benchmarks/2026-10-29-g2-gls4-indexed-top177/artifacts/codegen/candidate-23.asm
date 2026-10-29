
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004658e0 <zopro_core::msm_affine::g2_phi::raw_exclusive>:
  4658e0:	push   %rbp
  4658e1:	push   %r15
  4658e3:	push   %r14
  4658e5:	push   %r13
  4658e7:	push   %r12
  4658e9:	push   %rbx
  4658ea:	sub    $0x358,%rsp
  4658f1:	mov    %rdx,0x288(%rsp)
  4658f9:	mov    %r8,0x50(%rsp)
  4658fe:	cmp    %r8,%rdx
  465901:	jne    466a24 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1144>
  465907:	lea    -0x400(%rdx),%rax
  46590e:	mov    %rcx,%r15
  465911:	mov    %rdx,%rbx
  465914:	cmp    $0xfc00,%rax
  46591a:	jae    46593f <zopro_core::msm_affine::g2_phi::raw_exclusive+0x5f>
  46591c:	mov    %fs:0xffffffffffffffc0,%rax
  465925:	mov    %rsi,0x8(%rsp)
  46592a:	mov    %rdi,0x180(%rsp)
  465932:	test   %rax,%rax
  465935:	je     46595e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x7e>
  465937:	add    $0x110,%rax
  46593d:	jmp    465963 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x83>
  46593f:	mov    %rbx,%rdx
  465942:	mov    %r15,%rcx
  465945:	mov    %rbx,%r8
  465948:	add    $0x358,%rsp
  46594f:	pop    %rbx
  465950:	pop    %r12
  465952:	pop    %r13
  465954:	pop    %r14
  465956:	pop    %r15
  465958:	pop    %rbp
  465959:	jmp    45a8c0 <zopro_core::msm_affine::g2_horner::raw>
  46595e:	call   105490 <rayon_core::registry::global_registry>
  465963:	mov    (%rax),%rax
  465966:	mov    0x208(%rax),%r13
  46596d:	lea    -0x4452e1(%rip),%rsi        # 20693 <zmij::STATIC_DATA+0x6153>
  465974:	lea    0x50(%rsp),%rdi
  465979:	mov    $0x15,%edx
  46597e:	call   144180 <std::sys::env::unix::getenv>
  465983:	mov    0x50(%rsp),%rbp
  465988:	cmp    $0xffffffffffffffff,%rbp
  46598c:	je     465a4c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  465992:	mov    0x58(%rsp),%r14
  465997:	mov    0x60(%rsp),%r12
  46599c:	lea    0x50(%rsp),%rdi
  4659a1:	mov    %r14,%rsi
  4659a4:	mov    %r12,%rdx
  4659a7:	call   f1470 <core::str::converts::from_utf8>
  4659ac:	cmpb   $0x0,0x50(%rsp)
  4659b1:	je     4659e3 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x103>
  4659b3:	test   %rbp,%rbp
  4659b6:	jle    465a4c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  4659bc:	cmp    $0x1ffff,%rbp
  4659c3:	jbe    465a25 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x145>
  4659c5:	mov    %rbp,%rsi
  4659c8:	inc    %rsi
  4659cb:	js     466a84 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x11a4>
  4659d1:	cmpb   $0x1,(%r14,%rbp,1)
  4659d6:	jne    465a29 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x149>
  4659d8:	mov    %r14,%rdi
  4659db:	call   *0x8fbc7(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  4659e1:	jmp    465a4c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  4659e3:	mov    %r13,0x10(%rsp)
  4659e8:	mov    $0x1,%r13b
  4659eb:	cmp    $0x1,%r12
  4659ef:	jne    4659f9 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x119>
  4659f1:	cmpb   $0x30,(%r14)
  4659f5:	setne  %r13b
  4659f9:	test   %rbp,%rbp
  4659fc:	je     465a3d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x15d>
  4659fe:	cmp    $0x1ffff,%rbp
  465a05:	jbe    465a27 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x147>
  465a07:	mov    %rbp,%rsi
  465a0a:	inc    %rsi
  465a0d:	js     466a89 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x11a9>
  465a13:	cmpb   $0x1,(%r14,%rbp,1)
  465a18:	jne    465a34 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x154>
  465a1a:	mov    %r14,%rdi
  465a1d:	call   *0x8fb85(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  465a23:	jmp    465a3d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x15d>
  465a25:	jmp    465a29 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x149>
  465a27:	jmp    465a34 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x154>
  465a29:	mov    %r14,%rdi
  465a2c:	call   *0x8fb7e(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  465a32:	jmp    465a4c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  465a34:	mov    %r14,%rdi
  465a37:	call   *0x8fb73(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  465a3d:	mov    $0x96,%ebp
  465a42:	test   %r13b,%r13b
  465a45:	mov    0x10(%rsp),%r13
  465a4a:	je     465a51 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x171>
  465a4c:	mov    $0x98,%ebp
  465a51:	lea    0x3(%rbx),%r12
  465a55:	shr    $0x2,%r12
  465a59:	lea    0x50(%rsp),%r14
  465a5e:	mov    $0xb,%edx
  465a63:	mov    %r12,%rsi
  465a66:	mov    %r14,%rdi
  465a69:	call   419910 <zopro_core::msm_affine::workspace::top::<zopro_bn254::curve::G2Config>>
  465a6e:	cmpb   $0x0,0x50(%rsp)
  465a73:	je     465ae4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x204>
  465a75:	mov    0x58(%rsp),%rax
  465a7a:	movabs $0x3ffffffffff9ff3f,%rcx
  465a84:	cmp    %rcx,%rax
  465a87:	ja     465ae4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x204>
  465a89:	lea    0x483100(,%rax,4),%rcx
  465a91:	cmp    $0x302e00,%rcx
  465a98:	jb     465ae4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x204>
  465a9a:	lea    (%rbx,%rbx,4),%rax
  465a9e:	lea    0x40(,%rax,8),%rax
  465aa6:	add    %rax,%rcx
  465aa9:	jb     465ae4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x204>
  465aab:	mov    %ebp,%edx
  465aad:	shl    $0x7,%edx
  465ab0:	mov    %ebp,%eax
  465ab2:	shl    $0x6,%eax
  465ab5:	mov    0x60(%rsp),%rsi
  465aba:	lea    (%rdx,%rbp,8),%edx
  465abd:	or     $0x40,%eax
  465ac0:	shr    $0x3,%ebp
  465ac3:	add    %eax,%edx
  465ac5:	imul   $0xb40,%rbp,%rax
  465acc:	lea    0x93d58(%rax,%rdx,2),%rax
  465ad4:	cmp    %rax,%rsi
  465ad7:	cmova  %rsi,%rax
  465adb:	mul    %r13
  465ade:	jno    465c5a <zopro_core::msm_affine::g2_phi::raw_exclusive+0x37a>
  465ae4:	xor    %esi,%esi
  465ae6:	lea    0x190(%rsp),%r13
  465aee:	mov    %r13,%rdi
  465af1:	call   21c490 <<zopro_core::preparation::Reservation>::analysis>
  465af6:	cmpl   $0x1,0x190(%rsp)
  465afe:	je     465c04 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x324>
  465b04:	mov    %fs:0xffffffffffffffc0,%rax
  465b0d:	movq   $0xc,0x188(%rsp)
  465b19:	movq   $0x2,0x148(%rsp)
  465b25:	test   %rax,%rax
  465b28:	je     465b32 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x252>
  465b2a:	add    $0x110,%rax
  465b30:	jmp    465b37 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x257>
  465b32:	call   105490 <rayon_core::registry::global_registry>
  465b37:	mov    (%rax),%rax
  465b3a:	mov    0x208(%rax),%rdx
  465b41:	sub    $0x8,%rsp
  465b45:	lea    0x27(%rsp),%rax
  465b4a:	mov    $0x1,%ecx
  465b4f:	mov    %rbx,%rdi
  465b52:	xor    %esi,%esi
  465b54:	mov    %r15,%r8
  465b57:	mov    %rbx,%r9
  465b5a:	push   %rax
  465b5b:	call   467ff0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterMutProducer<[u64; 4]>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::g2_phi::hb_raw::{closure#0}>>>
  465b60:	add    $0x10,%rsp
  465b64:	mov    %rbx,%rax
  465b67:	shr    $1,%rax
  465b6a:	mov    %rbx,%rcx
  465b6d:	movq   $0x0,0x118(%rsp)
  465b79:	movq   $0x8,0x120(%rsp)
  465b85:	movq   $0x0,0x128(%rsp)
  465b91:	movq   $0xb,0x158(%rsp)
  465b9d:	sub    %rax,%rcx
  465ba0:	mov    %rcx,0x150(%rsp)
  465ba8:	lea    0x118(%rsp),%rdi
  465bb0:	mov    $0xb,%edx
  465bb5:	mov    $0x8,%ecx
  465bba:	mov    $0xc0,%r8d
  465bc0:	xor    %esi,%esi
  465bc2:	call   1b71c0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  465bc7:	mov    0x118(%rsp),%r12
  465bcf:	mov    0x128(%rsp),%rbp
  465bd7:	mov    %r12,%rax
  465bda:	sub    %rbp,%rax
  465bdd:	cmp    $0xb,%rax
  465be1:	jae    466a8e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x11ae>
  465be7:	lea    -0x44603e(%rip),%rdi        # 1fbb0 <zmij::STATIC_DATA+0x5670>
  465bee:	lea    0x8ce6b(%rip),%rdx        # 4f2a60 <__frame_dummy_init_array_entry+0x6240>
  465bf5:	mov    $0x2f,%esi
  465bfa:	call   f1740 <core::panicking::panic>
  465bff:	jmp    467450 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b70>
  465c04:	mov    0x198(%rsp),%rax
  465c0c:	mov    0x1a0(%rsp),%rbp
  465c14:	mov    %rax,0x10(%rsp)
  465c19:	lea    0x50(%rsp),%rdi
  465c1e:	mov    $0x8,%edx
  465c23:	mov    $0x8,%r8d
  465c29:	mov    $0x28,%r9d
  465c2f:	xor    %esi,%esi
  465c31:	mov    %rbx,%rcx
  465c34:	call   1e1e90 <<alloc::raw_vec::RawVecInner>::finish_grow>
  465c39:	cmpb   $0x0,0x50(%rsp)
  465c3e:	je     465c6e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x38e>
  465c40:	mov    0x10(%rsp),%rax
  465c45:	lock sub %rax,0x91743(%rip)        # 4f7390 <zopro_core::preparation::RESERVED>
  465c4d:	lock sub %rbp,0x91743(%rip)        # 4f7398 <zopro_core::preparation::WORKSPACES>
  465c55:	jmp    465b04 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x224>
  465c5a:	mov    %rax,%rdx
  465c5d:	add    %rcx,%rdx
  465c60:	xor    %esi,%esi
  465c62:	cmp    %rcx,%rdx
  465c65:	setae  %sil
  465c69:	jmp    465ae6 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x206>
  465c6e:	lea    0x40(%rsp),%rax
  465c73:	mov    %rbp,%r14
  465c76:	mov    0x58(%rsp),%rbp
  465c7b:	mov    %rbx,0x68(%rsp)
  465c80:	mov    %r15,0x30(%rsp)
  465c85:	mov    %rbx,0x118(%rsp)
  465c8d:	mov    %rbx,0x38(%rsp)
  465c92:	mov    %rax,0x50(%rsp)
  465c97:	mov    %fs:0xffffffffffffffc0,%rax
  465ca0:	mov    %rbp,0x58(%rsp)
  465ca5:	mov    %rbx,0x60(%rsp)
  465caa:	test   %rax,%rax
  465cad:	je     465cb7 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x3d7>
  465caf:	add    $0x110,%rax
  465cb5:	jmp    465cbc <zopro_core::msm_affine::g2_phi::raw_exclusive+0x3dc>
  465cb7:	call   105490 <rayon_core::registry::global_registry>
  465cbc:	mov    (%rax),%rax
  465cbf:	mov    0x208(%rax),%rcx
  465cc6:	lea    0x190(%rsp),%r13
  465cce:	lea    0x50(%rsp),%rax
  465cd3:	xor    %edx,%edx
  465cd5:	mov    $0x1,%r8d
  465cdb:	mov    %rbx,%rsi
  465cde:	mov    %r15,%r9
  465ce1:	mov    %r13,%rdi
  465ce4:	push   %rax
  465ce5:	push   %rbx
  465ce6:	call   467500 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  465ceb:	add    $0x10,%rsp
  465cef:	mov    0x1a0(%rsp),%rax
  465cf7:	mov    %rax,0x30(%rsp)
  465cfc:	cmp    %rbx,%rax
  465cff:	jne    466a3f <zopro_core::msm_affine::g2_phi::raw_exclusive+0x115f>
  465d05:	lea    -0x44529b(%rip),%rsi        # 20a71 <zmij::STATIC_DATA+0x6531>
  465d0c:	lea    0x50(%rsp),%rdi
  465d11:	mov    $0x17,%edx
  465d16:	call   144180 <std::sys::env::unix::getenv>
  465d1b:	mov    0x50(%rsp),%rsi
  465d20:	mov    %r14,0x28(%rsp)
  465d25:	mov    %rbp,0x20(%rsp)
  465d2a:	cmp    $0xffffffffffffffff,%rsi
  465d2e:	je     465d6e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x48e>
  465d30:	test   %rsi,%rsi
  465d33:	je     465d44 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x464>
  465d35:	mov    0x58(%rsp),%rdi
  465d3a:	mov    $0x1,%edx
  465d3f:	call   e9b90 <__rustc::__rust_dealloc>
  465d44:	lea    -0x37208b(%rip),%rax        # f3cc0 <<u64 as core::fmt::Display>::fmt>
  465d4b:	mov    %r13,0x50(%rsp)
  465d50:	mov    %rbx,0x190(%rsp)
  465d58:	mov    %rax,0x58(%rsp)
  465d5d:	lea    -0x4505fe(%rip),%rdi        # 15766 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x3c66>
  465d64:	lea    0x50(%rsp),%rsi
  465d69:	call   140dd0 <std::io::stdio::_eprint>
  465d6e:	vmovups -0x454076(%rip),%ymm0        # 11d00 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x200>
  465d76:	vmovups -0x45419e(%rip),%ymm2        # 11be0 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0xe0>
  465d7e:	mov    %r12,0x148(%rsp)
  465d86:	lea    0x190(%rsp),%rax
  465d8e:	lea    0x30(%rsp),%rcx
  465d93:	lea    0x50(%rsp),%rsi
  465d98:	movabs $0x87d20782e4866389,%r12
  465da2:	vmovups -0x445402(%rip),%ymm1        # 209a8 <zmij::STATIC_DATA+0x6468>
  465daa:	lea    0x190(%rsp),%r13
  465db2:	vmovups %ymm0,0x30(%rsp)
  465db8:	vmovups %ymm0,0x190(%rsp)
  465dc1:	vmovups %ymm2,0x50(%rsp)
  465dc7:	xor    %r11d,%r11d
  465dca:	xor    %r10d,%r10d
  465dcd:	xor    %r9d,%r9d
  465dd0:	xor    %edi,%edi
  465dd2:	xor    %r15d,%r15d
  465dd5:	mov    (%rcx),%rdx
  465dd8:	xor    %r14d,%r14d
  465ddb:	mulx   (%rax),%r8,%rbp
  465de0:	adcx   %r8,%r11
  465de6:	adox   %rbp,%r10
  465dec:	mulx   0x8(%rax),%r8,%rbp
  465df2:	adcx   %r8,%r10
  465df8:	adox   %rbp,%r9
  465dfe:	mulx   0x10(%rax),%r8,%rbp
  465e04:	adcx   %r8,%r9
  465e0a:	adox   %rbp,%rdi
  465e10:	mulx   0x18(%rax),%r8,%rbp
  465e16:	adcx   %r8,%rdi
  465e1c:	adox   %rbp,%r15
  465e22:	adcx   %r14,%r15
  465e28:	mov    %r11,%rdx
  465e2b:	imul   %r12,%rdx
  465e2f:	xor    %r14d,%r14d
  465e32:	mulx   (%rsi),%r8,%rbp
  465e37:	adcx   %r8,%r11
  465e3d:	adox   %rbp,%r10
  465e43:	mulx   0x8(%rsi),%r8,%rbp
  465e49:	adcx   %r8,%r10
  465e4f:	adox   %rbp,%r9
  465e55:	mulx   0x10(%rsi),%r8,%rbp
  465e5b:	adcx   %r8,%r9
  465e61:	adox   %rbp,%rdi
  465e67:	mulx   0x18(%rsi),%r8,%rbp
  465e6d:	adcx   %r8,%rdi
  465e73:	adox   %rbp,%r15
  465e79:	adcx   %r14,%r15
  465e7f:	mov    0x8(%rcx),%rdx
  465e83:	xor    %r14d,%r14d
  465e86:	mulx   (%rax),%r8,%rbp
  465e8b:	adcx   %r8,%r10
  465e91:	adox   %rbp,%r9
  465e97:	mulx   0x8(%rax),%r8,%rbp
  465e9d:	adcx   %r8,%r9
  465ea3:	adox   %rbp,%rdi
  465ea9:	mulx   0x10(%rax),%r8,%rbp
  465eaf:	adcx   %r8,%rdi
  465eb5:	adox   %rbp,%r15
  465ebb:	mulx   0x18(%rax),%r8,%rbp
  465ec1:	adcx   %r8,%r15
  465ec7:	adox   %rbp,%r11
  465ecd:	adcx   %r14,%r11
  465ed3:	mov    %r10,%rdx
  465ed6:	imul   %r12,%rdx
  465eda:	xor    %r14d,%r14d
  465edd:	mulx   (%rsi),%r8,%rbp
  465ee2:	adcx   %r8,%r10
  465ee8:	adox   %rbp,%r9
  465eee:	mulx   0x8(%rsi),%r8,%rbp
  465ef4:	adcx   %r8,%r9
  465efa:	adox   %rbp,%rdi
  465f00:	mulx   0x10(%rsi),%r8,%rbp
  465f06:	adcx   %r8,%rdi
  465f0c:	adox   %rbp,%r15
  465f12:	mulx   0x18(%rsi),%r8,%rbp
  465f18:	adcx   %r8,%r15
  465f1e:	adox   %rbp,%r11
  465f24:	adcx   %r14,%r11
  465f2a:	mov    0x10(%rcx),%rdx
  465f2e:	xor    %r14d,%r14d
  465f31:	mulx   (%rax),%r8,%rbp
  465f36:	adcx   %r8,%r9
  465f3c:	adox   %rbp,%rdi
  465f42:	mulx   0x8(%rax),%r8,%rbp
  465f48:	adcx   %r8,%rdi
  465f4e:	adox   %rbp,%r15
  465f54:	mulx   0x10(%rax),%r8,%rbp
  465f5a:	adcx   %r8,%r15
  465f60:	adox   %rbp,%r11
  465f66:	mulx   0x18(%rax),%r8,%rbp
  465f6c:	adcx   %r8,%r11
  465f72:	adox   %rbp,%r10
  465f78:	adcx   %r14,%r10
  465f7e:	mov    %r9,%rdx
  465f81:	imul   %r12,%rdx
  465f85:	xor    %r14d,%r14d
  465f88:	mulx   (%rsi),%r8,%rbp
  465f8d:	adcx   %r8,%r9
  465f93:	adox   %rbp,%rdi
  465f99:	mulx   0x8(%rsi),%r8,%rbp
  465f9f:	adcx   %r8,%rdi
  465fa5:	adox   %rbp,%r15
  465fab:	mulx   0x10(%rsi),%r8,%rbp
  465fb1:	adcx   %r8,%r15
  465fb7:	adox   %rbp,%r11
  465fbd:	mulx   0x18(%rsi),%r8,%rbp
  465fc3:	adcx   %r8,%r11
  465fc9:	adox   %rbp,%r10
  465fcf:	adcx   %r14,%r10
  465fd5:	mov    0x18(%rcx),%rdx
  465fd9:	xor    %r14d,%r14d
  465fdc:	mulx   (%rax),%r8,%rbp
  465fe1:	adcx   %r8,%rdi
  465fe7:	adox   %rbp,%r15
  465fed:	mulx   0x8(%rax),%r8,%rbp
  465ff3:	adcx   %r8,%r15
  465ff9:	adox   %rbp,%r11
  465fff:	mulx   0x10(%rax),%r8,%rbp
  466005:	adcx   %r8,%r11
  46600b:	adox   %rbp,%r10
  466011:	mulx   0x18(%rax),%r8,%rbp
  466017:	adcx   %r8,%r10
  46601d:	adox   %rbp,%r9
  466023:	adcx   %r14,%r9
  466029:	mov    %rdi,%rdx
  46602c:	imul   %r12,%rdx
  466030:	xor    %r14d,%r14d
  466033:	mulx   (%rsi),%r8,%rbp
  466038:	adcx   %r8,%rdi
  46603e:	adox   %rbp,%r15
  466044:	mulx   0x8(%rsi),%r8,%rbp
  46604a:	adcx   %r8,%r15
  466050:	adox   %rbp,%r11
  466056:	mulx   0x10(%rsi),%r8,%rbp
  46605c:	adcx   %r8,%r11
  466062:	adox   %rbp,%r10
  466068:	mulx   0x18(%rsi),%r8,%rbp
  46606e:	adcx   %r8,%r10
  466074:	adox   %rbp,%r9
  46607a:	adcx   %r14,%r9
  466080:	movabs $0x687e956e978e3573,%rax
  46608a:	movabs $0x3c208c16d87cfd47,%r8
  466094:	vmovups %ymm1,0x30(%rsp)
  46609a:	vmovups %ymm2,0x50(%rsp)
  4660a0:	lea    0x30(%rsp),%r14
  4660a5:	vmovups -0x4456e5(%rip),%ymm1        # 209c8 <zmij::STATIC_DATA+0x6488>
  4660ad:	lea    (%r11,%rax,1),%rsi
  4660b1:	cmp    %r8,%r15
  4660b4:	movabs $0x97816a916871ca8d,%rax
  4660be:	sbb    $0x0,%rsi
  4660c2:	setb   %cl
  4660c5:	cmp    %rax,%r11
  4660c8:	movabs $0x47afba497e7ea7a3,%rax
  4660d2:	setb   %dl
  4660d5:	or     %cl,%dl
  4660d7:	lea    (%r10,%rax,1),%rcx
  4660db:	movabs $0xb85045b68181585d,%rax
  4660e5:	movzbl %dl,%edx
  4660e8:	sub    %rdx,%rcx
  4660eb:	setb   %dl
  4660ee:	cmp    %rax,%r10
  4660f1:	movabs $0xcf9bb18d1ece5fd7,%rax
  4660fb:	setb   %dil
  4660ff:	lea    (%r9,%rax,1),%r12
  466103:	movabs $0x30644e72e131a029,%rax
  46610d:	or     %dl,%dil
  466110:	movzbl %dil,%edx
  466114:	sub    %rdx,%r12
  466117:	setb   %dl
  46611a:	cmp    %rax,%r9
  46611d:	movabs $0xc3df73e9278302b9,%rax
  466127:	setb   %dil
  46612b:	add    %r15,%rax
  46612e:	or     %dl,%dil
  466131:	cmovne %r15,%rax
  466135:	cmovne %r11,%rsi
  466139:	cmovne %r10,%rcx
  46613d:	cmovne %r9,%r12
  466141:	lea    0x50(%rsp),%r11
  466146:	mov    %rax,0x138(%rsp)
  46614e:	mov    %rax,0x190(%rsp)
  466156:	mov    %rsi,0x140(%rsp)
  46615e:	mov    %rsi,0x198(%rsp)
  466166:	mov    %rcx,0x1a0(%rsp)
  46616e:	mov    %rcx,0x130(%rsp)
  466176:	mov    %r12,0x1a8(%rsp)
  46617e:	movabs $0x87d20782e4866389,%rax
  466188:	xor    %esi,%esi
  46618a:	xor    %edi,%edi
  46618c:	xor    %r8d,%r8d
  46618f:	xor    %r15d,%r15d
  466192:	xor    %ecx,%ecx
  466194:	mov    0x0(%r13),%rdx
  466198:	xor    %ebp,%ebp
  46619a:	mulx   (%r14),%r9,%r10
  46619f:	adcx   %r9,%rsi
  4661a5:	adox   %r10,%rdi
  4661ab:	mulx   0x8(%r14),%r9,%r10
  4661b1:	adcx   %r9,%rdi
  4661b7:	adox   %r10,%r8
  4661bd:	mulx   0x10(%r14),%r9,%r10
  4661c3:	adcx   %r9,%r8
  4661c9:	adox   %r10,%r15
  4661cf:	mulx   0x18(%r14),%r9,%r10
  4661d5:	adcx   %r9,%r15
  4661db:	adox   %r10,%rcx
  4661e1:	adcx   %rbp,%rcx
  4661e7:	mov    %rsi,%rdx
  4661ea:	imul   %rax,%rdx
  4661ee:	xor    %ebp,%ebp
  4661f0:	mulx   (%r11),%r9,%r10
  4661f5:	adcx   %r9,%rsi
  4661fb:	adox   %r10,%rdi
  466201:	mulx   0x8(%r11),%r9,%r10
  466207:	adcx   %r9,%rdi
  46620d:	adox   %r10,%r8
  466213:	mulx   0x10(%r11),%r9,%r10
  466219:	adcx   %r9,%r8
  46621f:	adox   %r10,%r15
  466225:	mulx   0x18(%r11),%r9,%r10
  46622b:	adcx   %r9,%r15
  466231:	adox   %r10,%rcx
  466237:	adcx   %rbp,%rcx
  46623d:	mov    0x8(%r13),%rdx
  466241:	xor    %ebp,%ebp
  466243:	mulx   (%r14),%r9,%r10
  466248:	adcx   %r9,%rdi
  46624e:	adox   %r10,%r8
  466254:	mulx   0x8(%r14),%r9,%r10
  46625a:	adcx   %r9,%r8
  466260:	adox   %r10,%r15
  466266:	mulx   0x10(%r14),%r9,%r10
  46626c:	adcx   %r9,%r15
  466272:	adox   %r10,%rcx
  466278:	mulx   0x18(%r14),%r9,%r10
  46627e:	adcx   %r9,%rcx
  466284:	adox   %r10,%rsi
  46628a:	adcx   %rbp,%rsi
  466290:	mov    %rdi,%rdx
  466293:	imul   %rax,%rdx
  466297:	xor    %ebp,%ebp
  466299:	mulx   (%r11),%r9,%r10
  46629e:	adcx   %r9,%rdi
  4662a4:	adox   %r10,%r8
  4662aa:	mulx   0x8(%r11),%r9,%r10
  4662b0:	adcx   %r9,%r8
  4662b6:	adox   %r10,%r15
  4662bc:	mulx   0x10(%r11),%r9,%r10
  4662c2:	adcx   %r9,%r15
  4662c8:	adox   %r10,%rcx
  4662ce:	mulx   0x18(%r11),%r9,%r10
  4662d4:	adcx   %r9,%rcx
  4662da:	adox   %r10,%rsi
  4662e0:	adcx   %rbp,%rsi
  4662e6:	mov    0x10(%r13),%rdx
  4662ea:	xor    %ebp,%ebp
  4662ec:	mulx   (%r14),%r9,%r10
  4662f1:	adcx   %r9,%r8
  4662f7:	adox   %r10,%r15
  4662fd:	mulx   0x8(%r14),%r9,%r10
  466303:	adcx   %r9,%r15
  466309:	adox   %r10,%rcx
  46630f:	mulx   0x10(%r14),%r9,%r10
  466315:	adcx   %r9,%rcx
  46631b:	adox   %r10,%rsi
  466321:	mulx   0x18(%r14),%r9,%r10
  466327:	adcx   %r9,%rsi
  46632d:	adox   %r10,%rdi
  466333:	adcx   %rbp,%rdi
  466339:	mov    %r8,%rdx
  46633c:	imul   %rax,%rdx
  466340:	xor    %ebp,%ebp
  466342:	mulx   (%r11),%r9,%r10
  466347:	adcx   %r9,%r8
  46634d:	adox   %r10,%r15
  466353:	mulx   0x8(%r11),%r9,%r10
  466359:	adcx   %r9,%r15
  46635f:	adox   %r10,%rcx
  466365:	mulx   0x10(%r11),%r9,%r10
  46636b:	adcx   %r9,%rcx
  466371:	adox   %r10,%rsi
  466377:	mulx   0x18(%r11),%r9,%r10
  46637d:	adcx   %r9,%rsi
  466383:	adox   %r10,%rdi
  466389:	adcx   %rbp,%rdi
  46638f:	mov    0x18(%r13),%rdx
  466393:	xor    %ebp,%ebp
  466395:	mulx   (%r14),%r9,%r10
  46639a:	adcx   %r9,%r15
  4663a0:	adox   %r10,%rcx
  4663a6:	mulx   0x8(%r14),%r9,%r10
  4663ac:	adcx   %r9,%rcx
  4663b2:	adox   %r10,%rsi
  4663b8:	mulx   0x10(%r14),%r9,%r10
  4663be:	adcx   %r9,%rsi
  4663c4:	adox   %r10,%rdi
  4663ca:	mulx   0x18(%r14),%r9,%r10
  4663d0:	adcx   %r9,%rdi
  4663d6:	adox   %r10,%r8
  4663dc:	adcx   %rbp,%r8
  4663e2:	mov    %r15,%rdx
  4663e5:	imul   %rax,%rdx
  4663e9:	xor    %ebp,%ebp
  4663eb:	mulx   (%r11),%r9,%r10
  4663f0:	adcx   %r9,%r15
  4663f6:	adox   %r10,%rcx
  4663fc:	mulx   0x8(%r11),%r9,%r10
  466402:	adcx   %r9,%rcx
  466408:	adox   %r10,%rsi
  46640e:	mulx   0x10(%r11),%r9,%r10
  466414:	adcx   %r9,%rsi
  46641a:	adox   %r10,%rdi
  466420:	mulx   0x18(%r11),%r9,%r10
  466426:	adcx   %r9,%rdi
  46642c:	adox   %r10,%r8
  466432:	adcx   %rbp,%r8
  466438:	movabs $0x687e956e978e3573,%rax
  466442:	movabs $0x3c208c16d87cfd47,%r11
  46644c:	mov    %r8,0x170(%rsp)
  466454:	mov    %rdi,0x178(%rsp)
  46645c:	mov    %rcx,0x270(%rsp)
  466464:	mov    %rsi,0x278(%rsp)
  46646c:	movabs $0x97816a916871ca8d,%rdi
  466476:	vmovups %ymm1,0x30(%rsp)
  46647c:	vmovups %ymm2,0x50(%rsp)
  466482:	movabs $0x87d20782e4866389,%r14
  46648c:	lea    0x50(%rsp),%r13
  466491:	cmp    %r11,%rcx
  466494:	lea    (%rsi,%rax,1),%rax
  466498:	sbb    $0x0,%rax
  46649c:	mov    %rax,0x280(%rsp)
  4664a4:	setb   %dl
  4664a7:	cmp    %rdi,%rsi
  4664aa:	movabs $0x47afba497e7ea7a3,%rax
  4664b4:	mov    0x178(%rsp),%rsi
  4664bc:	setb   %r9b
  4664c0:	or     %dl,%r9b
  4664c3:	movzbl %r9b,%edx
  4664c7:	movabs $0xb85045b68181585d,%r9
  4664d1:	add    %rsi,%rax
  4664d4:	sub    %rdx,%rax
  4664d7:	mov    %rax,0x268(%rsp)
  4664df:	setb   %dl
  4664e2:	cmp    %r9,%rsi
  4664e5:	movabs $0xcf9bb18d1ece5fd7,%rax
  4664ef:	mov    0x170(%rsp),%rsi
  4664f7:	setb   %r10b
  4664fb:	or     %dl,%r10b
  4664fe:	movzbl %r10b,%edx
  466502:	add    %rsi,%rax
  466505:	sub    %rdx,%rax
  466508:	mov    %rax,0x258(%rsp)
  466510:	mov    0x138(%rsp),%rax
  466518:	mov    0x140(%rsp),%rdi
  466520:	mov    0x130(%rsp),%rsi
  466528:	setb   0x138(%rsp)
  466530:	mov    %rax,0x190(%rsp)
  466538:	mov    %rdi,0x198(%rsp)
  466540:	movabs $0xc3df73e9278302b9,%rdi
  46654a:	mov    %rsi,0x1a0(%rsp)
  466552:	mov    %r12,0x1a8(%rsp)
  46655a:	lea    0x30(%rsp),%rsi
  46655f:	add    %rdi,%rcx
  466562:	mov    %rcx,0x140(%rsp)
  46656a:	lea    0x190(%rsp),%rcx
  466572:	xor    %r11d,%r11d
  466575:	xor    %eax,%eax
  466577:	xor    %r9d,%r9d
  46657a:	xor    %ebp,%ebp
  46657c:	xor    %r10d,%r10d
  46657f:	mov    (%rcx),%rdx
  466582:	xor    %r12d,%r12d
  466585:	mulx   (%rsi),%r8,%r15
  46658a:	adcx   %r8,%r11
  466590:	adox   %r15,%rax
  466596:	mulx   0x8(%rsi),%r8,%r15
  46659c:	adcx   %r8,%rax
  4665a2:	adox   %r15,%r9
  4665a8:	mulx   0x10(%rsi),%r8,%r15
  4665ae:	adcx   %r8,%r9
  4665b4:	adox   %r15,%rbp
  4665ba:	mulx   0x18(%rsi),%r8,%r15
  4665c0:	adcx   %r8,%rbp
  4665c6:	adox   %r15,%r10
  4665cc:	adcx   %r12,%r10
  4665d2:	mov    %r11,%rdx
  4665d5:	imul   %r14,%rdx
  4665d9:	xor    %r12d,%r12d
  4665dc:	mulx   0x0(%r13),%r8,%r15
  4665e2:	adcx   %r8,%r11
  4665e8:	adox   %r15,%rax
  4665ee:	mulx   0x8(%r13),%r8,%r15
  4665f4:	adcx   %r8,%rax
  4665fa:	adox   %r15,%r9
  466600:	mulx   0x10(%r13),%r8,%r15
  466606:	adcx   %r8,%r9
  46660c:	adox   %r15,%rbp
  466612:	mulx   0x18(%r13),%r8,%r15
  466618:	adcx   %r8,%rbp
  46661e:	adox   %r15,%r10
  466624:	adcx   %r12,%r10
  46662a:	mov    0x8(%rcx),%rdx
  46662e:	xor    %r12d,%r12d
  466631:	mulx   (%rsi),%r8,%r15
  466636:	adcx   %r8,%rax
  46663c:	adox   %r15,%r9
  466642:	mulx   0x8(%rsi),%r8,%r15
  466648:	adcx   %r8,%r9
  46664e:	adox   %r15,%rbp
  466654:	mulx   0x10(%rsi),%r8,%r15
  46665a:	adcx   %r8,%rbp
  466660:	adox   %r15,%r10
  466666:	mulx   0x18(%rsi),%r8,%r15
  46666c:	adcx   %r8,%r10
  466672:	adox   %r15,%r11
  466678:	adcx   %r12,%r11
  46667e:	mov    %rax,%rdx
  466681:	imul   %r14,%rdx
  466685:	xor    %r12d,%r12d
  466688:	mulx   0x0(%r13),%r8,%r15
  46668e:	adcx   %r8,%rax
  466694:	adox   %r15,%r9
  46669a:	mulx   0x8(%r13),%r8,%r15
  4666a0:	adcx   %r8,%r9
  4666a6:	adox   %r15,%rbp
  4666ac:	mulx   0x10(%r13),%r8,%r15
  4666b2:	adcx   %r8,%rbp
  4666b8:	adox   %r15,%r10
  4666be:	mulx   0x18(%r13),%r8,%r15
  4666c4:	adcx   %r8,%r10
  4666ca:	adox   %r15,%r11
  4666d0:	adcx   %r12,%r11
  4666d6:	mov    0x10(%rcx),%rdx
  4666da:	xor    %r12d,%r12d
  4666dd:	mulx   (%rsi),%r8,%r15
  4666e2:	adcx   %r8,%r9
  4666e8:	adox   %r15,%rbp
  4666ee:	mulx   0x8(%rsi),%r8,%r15
  4666f4:	adcx   %r8,%rbp
  4666fa:	adox   %r15,%r10
  466700:	mulx   0x10(%rsi),%r8,%r15
  466706:	adcx   %r8,%r10
  46670c:	adox   %r15,%r11
  466712:	mulx   0x18(%rsi),%r8,%r15
  466718:	adcx   %r8,%r11
  46671e:	adox   %r15,%rax
  466724:	adcx   %r12,%rax
  46672a:	mov    %r9,%rdx
  46672d:	imul   %r14,%rdx
  466731:	xor    %r12d,%r12d
  466734:	mulx   0x0(%r13),%r8,%r15
  46673a:	adcx   %r8,%r9
  466740:	adox   %r15,%rbp
  466746:	mulx   0x8(%r13),%r8,%r15
  46674c:	adcx   %r8,%rbp
  466752:	adox   %r15,%r10
  466758:	mulx   0x10(%r13),%r8,%r15
  46675e:	adcx   %r8,%r10
  466764:	adox   %r15,%r11
  46676a:	mulx   0x18(%r13),%r8,%r15
  466770:	adcx   %r8,%r11
  466776:	adox   %r15,%rax
  46677c:	adcx   %r12,%rax
  466782:	mov    0x18(%rcx),%rdx
  466786:	xor    %r12d,%r12d
  466789:	mulx   (%rsi),%r8,%r15
  46678e:	adcx   %r8,%rbp
  466794:	adox   %r15,%r10
  46679a:	mulx   0x8(%rsi),%r8,%r15
  4667a0:	adcx   %r8,%r10
  4667a6:	adox   %r15,%r11
  4667ac:	mulx   0x10(%rsi),%r8,%r15
  4667b2:	adcx   %r8,%r11
  4667b8:	adox   %r15,%rax
  4667be:	mulx   0x18(%rsi),%r8,%r15
  4667c4:	adcx   %r8,%rax
  4667ca:	adox   %r15,%r9
  4667d0:	adcx   %r12,%r9
  4667d6:	mov    %rbp,%rdx
  4667d9:	imul   %r14,%rdx
  4667dd:	xor    %r12d,%r12d
  4667e0:	mulx   0x0(%r13),%r8,%r15
  4667e6:	adcx   %r8,%rbp
  4667ec:	adox   %r15,%r10
  4667f2:	mulx   0x8(%r13),%r8,%r15
  4667f8:	adcx   %r8,%r10
  4667fe:	adox   %r15,%r11
  466804:	mulx   0x10(%r13),%r8,%r15
  46680a:	adcx   %r8,%r11
  466810:	adox   %r15,%rax
  466816:	mulx   0x18(%r13),%r8,%r15
  46681c:	adcx   %r8,%rax
  466822:	adox   %r15,%r9
  466828:	adcx   %r12,%r9
  46682e:	mov    %rax,0x260(%rsp)
  466836:	mov    %r9,0x130(%rsp)
  46683e:	movabs $0x687e956e978e3573,%rax
  466848:	movabs $0x3c208c16d87cfd47,%rdx
  466852:	add    %r10,%rdi
  466855:	movabs $0x97816a916871ca8d,%r8
  46685f:	movabs $0x47afba497e7ea7a3,%rcx
  466869:	movabs $0xcf9bb18d1ece5fd7,%rsi
  466873:	movabs $0x30644e72e131a029,%r14
  46687d:	mov    %r11,%r13
  466880:	movq   $0x0,0x158(%rsp)
  46688c:	movq   $0x8,0x160(%rsp)
  466898:	movq   $0x0,0x168(%rsp)
  4668a4:	movq   $0x6,0x150(%rsp)
  4668b0:	add    %r11,%rax
  4668b3:	cmp    %rdx,%r10
  4668b6:	mov    0x260(%rsp),%rbp
  4668be:	mov    0x130(%rsp),%r12
  4668c6:	mov    0x170(%rsp),%r15
  4668ce:	mov    0x268(%rsp),%r9
  4668d6:	sbb    $0x0,%rax
  4668da:	setb   %dl
  4668dd:	cmp    %r8,%r11
  4668e0:	mov    0x258(%rsp),%r11
  4668e8:	setb   %r8b
  4668ec:	or     %dl,%r8b
  4668ef:	movzbl %r8b,%edx
  4668f3:	add    %rbp,%rcx
  4668f6:	movabs $0xb85045b68181585d,%r8
  466900:	sub    %rdx,%rcx
  466903:	setb   %dl
  466906:	cmp    %r8,%rbp
  466909:	setb   %r8b
  46690d:	add    %r12,%rsi
  466910:	or     %dl,%r8b
  466913:	movzbl %r8b,%edx
  466917:	sub    %rdx,%rsi
  46691a:	setb   %dl
  46691d:	cmp    %r14,%r12
  466920:	setb   %r8b
  466924:	or     %dl,%r8b
  466927:	cmp    %r14,%r15
  46692a:	mov    0x140(%rsp),%r14
  466932:	setb   %dl
  466935:	or     0x138(%rsp),%dl
  46693c:	mov    0x280(%rsp),%rdx
  466944:	cmovne 0x270(%rsp),%r14
  46694d:	cmovne 0x278(%rsp),%rdx
  466956:	cmovne 0x178(%rsp),%r9
  46695f:	cmovne %r15,%r11
  466963:	test   %r8b,%r8b
  466966:	cmovne %r13,%rax
  46696a:	cmovne %r10,%rdi
  46696e:	cmovne %rbp,%rcx
  466972:	cmovne %r12,%rsi
  466976:	mov    %r14,0x190(%rsp)
  46697e:	mov    %rdx,0x198(%rsp)
  466986:	mov    %r9,0x1a0(%rsp)
  46698e:	mov    %r11,0x1a8(%rsp)
  466996:	mov    %rdi,0x1b0(%rsp)
  46699e:	mov    %rax,0x1b8(%rsp)
  4669a6:	mov    %rcx,0x1c0(%rsp)
  4669ae:	mov    %rsi,0x1c8(%rsp)
  4669b6:	mov    0x28(%rsp),%r14
  4669bb:	mov    0x10(%rsp),%r12
  4669c0:	mov    0x20(%rsp),%rbp
  4669c5:	lea    0x158(%rsp),%rdi
  4669cd:	mov    $0x6,%edx
  4669d2:	mov    $0x8,%ecx
  4669d7:	mov    $0xc0,%r8d
  4669dd:	xor    %esi,%esi
  4669df:	vzeroupper
  4669e2:	call   1b71c0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  4669e7:	mov    0x158(%rsp),%r13
  4669ef:	mov    0x168(%rsp),%r15
  4669f7:	mov    %r13,%rax
  4669fa:	sub    %r15,%rax
  4669fd:	cmp    $0x5,%rax
  466a01:	ja     466de5 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1505>
  466a07:	lea    -0x446e5e(%rip),%rdi        # 1fbb0 <zmij::STATIC_DATA+0x5670>
  466a0e:	lea    0x8c04b(%rip),%rdx        # 4f2a60 <__frame_dummy_init_array_entry+0x6240>
  466a15:	mov    $0x2f,%esi
  466a1a:	call   f1740 <core::panicking::panic>
  466a1f:	jmp    467450 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b70>
  466a24:	lea    0x8d175(%rip),%r8        # 4f3ba0 <__frame_dummy_init_array_entry+0x7380>
  466a2b:	lea    0x288(%rsp),%rdi
  466a33:	lea    0x50(%rsp),%rsi
  466a38:	xor    %edx,%edx
  466a3a:	call   f0ac2 <core::panicking::assert_failed::<usize, usize>>
  466a3f:	lea    0x118(%rsp),%rax
  466a47:	lea    -0x372d8e(%rip),%r8        # f3cc0 <<u64 as core::fmt::Display>::fmt>
  466a4e:	lea    0x30(%rsp),%rcx
  466a53:	mov    %rax,0x50(%rsp)
  466a58:	mov    %r8,0x58(%rsp)
  466a5d:	mov    %rcx,0x60(%rsp)
  466a62:	mov    %r8,0x68(%rsp)
  466a67:	lea    -0x44fc2f(%rip),%rdi        # 16e3f <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x533f>
  466a6e:	lea    0x8a063(%rip),%rdx        # 4f0ad8 <__frame_dummy_init_array_entry+0x42b8>
  466a75:	lea    0x50(%rsp),%rsi
  466a7a:	call   ef670 <core::panicking::panic_fmt>
  466a7f:	jmp    467450 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b70>
  466a84:	call   1113d0 <std::process::abort>
  466a89:	call   1113d0 <std::process::abort>
  466a8e:	mov    0x8(%rsp),%rdx
  466a93:	mov    0x120(%rsp),%rcx
  466a9b:	lea    0x148(%rsp),%rax
  466aa3:	lea    0x150(%rsp),%rsi
  466aab:	movq   $0xb,0x1a8(%rsp)
  466ab7:	mov    %r14,0x190(%rsp)
  466abf:	mov    %rax,0x50(%rsp)
  466ac4:	mov    %fs:0xffffffffffffffc0,%rax
  466acd:	mov    %rsi,0x58(%rsp)
  466ad2:	lea    0x188(%rsp),%rsi
  466ada:	mov    %rdx,0x60(%rsp)
  466adf:	lea    0x0(%rbp,%rbp,2),%rdx
  466ae4:	mov    %rbx,0x68(%rsp)
  466ae9:	mov    %r15,0x70(%rsp)
  466aee:	mov    %rbx,0x78(%rsp)
  466af3:	mov    %rsi,0x80(%rsp)
  466afb:	movq   $0x0,0x88(%rsp)
  466b07:	movq   $0xb,0x90(%rsp)
  466b13:	mov    %rcx,0x8(%rsp)
  466b18:	shl    $0x6,%rdx
  466b1c:	add    %rcx,%rdx
  466b1f:	mov    %rdx,0x198(%rsp)
  466b27:	movq   $0xb,0x1a0(%rsp)
  466b33:	test   %rax,%rax
  466b36:	je     466b40 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1260>
  466b38:	add    $0x110,%rax
  466b3e:	jmp    466b45 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1265>
  466b40:	call   105490 <rayon_core::registry::global_registry>
  466b45:	mov    (%rax),%rax
  466b48:	mov    0x208(%rax),%rcx
  466b4f:	xor    %edx,%edx
  466b51:	lea    0x30(%rsp),%rdi
  466b56:	mov    $0xb,%esi
  466b5b:	mov    $0x1,%r8d
  466b61:	xor    %r9d,%r9d
  466b64:	push   %r13
  466b66:	push   $0xb
  466b68:	call   468460 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_phi::hb_raw::{closure#1}>>>
  466b6d:	add    $0x10,%rsp
  466b71:	mov    0x40(%rsp),%rax
  466b76:	mov    %rax,0x190(%rsp)
  466b7e:	cmp    $0xb,%rax
  466b82:	jne    4673c4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1ae4>
  466b88:	vmovups -0x447602(%rip),%zmm0        # 1f590 <zmij::STATIC_DATA+0x5050>
  466b92:	vmovups -0x44764c(%rip),%zmm2        # 1f550 <zmij::STATIC_DATA+0x5010>
  466b9c:	vmovups -0x447696(%rip),%zmm1        # 1f510 <zmij::STATIC_DATA+0x4fd0>
  466ba6:	mov    0x188(%rsp),%r13
  466bae:	shl    $0x6,%rbp
  466bb2:	lea    0x0(%rbp,%rbp,2),%r15
  466bb7:	vmovups %zmm0,0x210(%rsp)
  466bc2:	vmovups %zmm2,0x1d0(%rsp)
  466bcd:	vmovups %zmm1,0x190(%rsp)
  466bd8:	test   %r13,%r13
  466bdb:	je     466cba <zopro_core::msm_affine::g2_phi::raw_exclusive+0x13da>
  466be1:	mov    0x8(%rsp),%rax
  466be6:	lea    0x50(%rsp),%r14
  466beb:	lea    0x840(%rax,%r15,1),%rbx
  466bf3:	lea    0x190(%rsp),%r15
  466bfb:	mov    %r13,%rbp
  466bfe:	mov    %r14,%rdi
  466c01:	mov    %r15,%rsi
  466c04:	vzeroupper
  466c07:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466c0c:	vmovups 0x50(%rsp),%zmm0
  466c17:	vmovups 0x90(%rsp),%zmm1
  466c22:	vmovups 0xd0(%rsp),%zmm2
  466c2d:	dec    %rbp
  466c30:	vmovups %zmm2,0x210(%rsp)
  466c3b:	vmovups %zmm1,0x1d0(%rsp)
  466c46:	vmovups %zmm0,0x190(%rsp)
  466c51:	jne    466bfe <zopro_core::msm_affine::g2_phi::raw_exclusive+0x131e>
  466c53:	add    $0xffffffffffffff40,%rbx
  466c5a:	mov    %r14,%rdi
  466c5d:	mov    %r15,%rsi
  466c60:	mov    %rbx,%rdx
  466c63:	vzeroupper
  466c66:	call   1851f0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  466c6b:	vmovups 0x50(%rsp),%zmm0
  466c76:	vmovups 0x90(%rsp),%zmm1
  466c81:	vmovups 0xd0(%rsp),%zmm2
  466c8c:	vmovups %zmm2,0x210(%rsp)
  466c97:	vmovups %zmm1,0x1d0(%rsp)
  466ca2:	vmovups %zmm0,0x190(%rsp)
  466cad:	cmp    %rbx,0x8(%rsp)
  466cb2:	jne    466bfb <zopro_core::msm_affine::g2_phi::raw_exclusive+0x131b>
  466cb8:	jmp    466d38 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1458>
  466cba:	mov    0x8(%rsp),%rax
  466cbf:	lea    0x50(%rsp),%rbx
  466cc4:	lea    0x190(%rsp),%r14
  466ccc:	lea    0x780(%rax),%r13
  466cd3:	lea    0x0(%r13,%r15,1),%rdx
  466cd8:	mov    %rbx,%rdi
  466cdb:	mov    %r14,%rsi
  466cde:	vzeroupper
  466ce1:	call   1851f0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  466ce6:	vmovups 0x50(%rsp),%zmm0
  466cf1:	vmovups 0x90(%rsp),%zmm1
  466cfc:	vmovups 0xd0(%rsp),%zmm2
  466d07:	add    $0xffffffffffffff40,%r15
  466d0e:	vmovups %zmm2,0x210(%rsp)
  466d19:	vmovups %zmm1,0x1d0(%rsp)
  466d24:	vmovups %zmm0,0x190(%rsp)
  466d2f:	cmp    $0xfffffffffffff7c0,%r15
  466d36:	jne    466cd3 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x13f3>
  466d38:	vmovups 0x190(%rsp),%zmm0
  466d43:	vmovups 0x1d0(%rsp),%zmm1
  466d4e:	vmovups 0x210(%rsp),%zmm2
  466d59:	mov    0x180(%rsp),%rax
  466d61:	vmovups %zmm2,0x80(%rax)
  466d68:	vmovups %zmm1,0x40(%rax)
  466d6f:	vmovups %zmm0,(%rax)
  466d75:	test   %r12,%r12
  466d78:	je     466dd0 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14f0>
  466d7a:	cmp    $0x2aa,%r12
  466d81:	jbe    466dc0 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14e0>
  466d83:	shl    $0x6,%r12
  466d87:	movabs $0x7ffffffffffffff8,%rcx
  466d91:	lea    (%r12,%r12,2),%rax
  466d95:	mov    %rax,%rsi
  466d98:	or     $0x1,%rsi
  466d9c:	cmp    %rcx,%rsi
  466d9f:	ja     467452 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b72>
  466da5:	mov    0x8(%rsp),%rcx
  466daa:	cmpb   $0x1,(%rcx,%rax,1)
  466dae:	jne    466dc2 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14e2>
  466db0:	mov    0x8(%rsp),%rdi
  466db5:	vzeroupper
  466db8:	call   *0x8e7ea(%rip)        # 4f55a8 <munmap@GLIBC_2.2.5>
  466dbe:	jmp    466dd0 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14f0>
  466dc0:	jmp    466dc2 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14e2>
  466dc2:	mov    0x8(%rsp),%rdi
  466dc7:	vzeroupper
  466dca:	call   *0x8e7e0(%rip)        # 4f55b0 <free@GLIBC_2.2.5>
  466dd0:	add    $0x358,%rsp
  466dd7:	pop    %rbx
  466dd8:	pop    %r12
  466dda:	pop    %r13
  466ddc:	pop    %r14
  466dde:	pop    %r15
  466de0:	pop    %rbp
  466de1:	vzeroupper
  466de4:	ret
  466de5:	mov    0x8(%rsp),%rsi
  466dea:	mov    0x160(%rsp),%rcx
  466df2:	lea    0x148(%rsp),%rax
  466dfa:	lea    0x190(%rsp),%rdx
  466e02:	movq   $0x6,0x48(%rsp)
  466e0b:	mov    %rax,0x50(%rsp)
  466e10:	mov    %fs:0xffffffffffffffc0,%rax
  466e19:	mov    %rsi,0x58(%rsp)
  466e1e:	lea    (%r15,%r15,2),%rsi
  466e22:	mov    %rbx,0x60(%rsp)
  466e27:	mov    %rbp,0x68(%rsp)
  466e2c:	mov    %rbx,0x70(%rsp)
  466e31:	mov    %rdx,0x78(%rsp)
  466e36:	lea    0x50(%rsp),%rdx
  466e3b:	movq   $0x0,0x80(%rsp)
  466e47:	movq   $0x6,0x88(%rsp)
  466e53:	mov    %rcx,0x8(%rsp)
  466e58:	mov    %rdx,0x30(%rsp)
  466e5d:	shl    $0x6,%rsi
  466e61:	add    %rcx,%rsi
  466e64:	mov    %rsi,0x38(%rsp)
  466e69:	movq   $0x6,0x40(%rsp)
  466e72:	test   %rax,%rax
  466e75:	je     466e7f <zopro_core::msm_affine::g2_phi::raw_exclusive+0x159f>
  466e77:	add    $0x110,%rax
  466e7d:	jmp    466e93 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x15b3>
  466e7f:	mov    0x28(%rsp),%r14
  466e84:	mov    0x10(%rsp),%r12
  466e89:	mov    0x20(%rsp),%rbp
  466e8e:	call   105490 <rayon_core::registry::global_registry>
  466e93:	mov    (%rax),%rax
  466e96:	mov    0x208(%rax),%rcx
  466e9d:	lea    0x30(%rsp),%rax
  466ea2:	xor    %edx,%edx
  466ea4:	lea    0x118(%rsp),%rdi
  466eac:	mov    $0x6,%esi
  466eb1:	mov    $0x1,%r8d
  466eb7:	xor    %r9d,%r9d
  466eba:	push   %rax
  466ebb:	push   $0x6
  466ebd:	mov    0x38(%rsp),%r14
  466ec2:	mov    0x20(%rsp),%r12
  466ec7:	mov    0x30(%rsp),%rbp
  466ecc:	call   467ce0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>>
  466ed1:	add    $0x10,%rsp
  466ed5:	mov    0x128(%rsp),%rax
  466edd:	mov    %rax,0x30(%rsp)
  466ee2:	cmp    $0x6,%rax
  466ee6:	jne    467401 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b21>
  466eec:	vmovups -0x4479e6(%rip),%zmm0        # 1f510 <zmij::STATIC_DATA+0x4fd0>
  466ef6:	vmovups -0x4479b0(%rip),%zmm2        # 1f550 <zmij::STATIC_DATA+0x5010>
  466f00:	vmovups -0x44797a(%rip),%zmm1        # 1f590 <zmij::STATIC_DATA+0x5050>
  466f0a:	mov    0x8(%rsp),%rcx
  466f0f:	shl    $0x6,%r15
  466f13:	mov    $0xfffffffffffffb80,%r14
  466f1a:	lea    0x50(%rsp),%rbp
  466f1f:	lea    (%r15,%r15,2),%rax
  466f23:	lea    0x290(%rsp),%r15
  466f2b:	sub    %rax,%r14
  466f2e:	lea    0x3c0(%rcx,%rax,1),%r12
  466f36:	vmovups %zmm0,0x290(%rsp)
  466f41:	vmovups %zmm2,0x2d0(%rsp)
  466f4c:	vmovups %zmm1,0x310(%rsp)
  466f57:	mov    %rbp,%rdi
  466f5a:	mov    %r15,%rsi
  466f5d:	vzeroupper
  466f60:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466f65:	vmovups 0x50(%rsp),%zmm0
  466f70:	vmovups 0x90(%rsp),%zmm1
  466f7b:	vmovups 0xd0(%rsp),%zmm2
  466f86:	vmovups %zmm2,0x310(%rsp)
  466f91:	vmovups %zmm1,0x2d0(%rsp)
  466f9c:	vmovups %zmm0,0x290(%rsp)
  466fa7:	mov    %rbp,%rdi
  466faa:	mov    %r15,%rsi
  466fad:	vzeroupper
  466fb0:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466fb5:	vmovups 0x50(%rsp),%zmm0
  466fc0:	vmovups 0x90(%rsp),%zmm1
  466fcb:	vmovups 0xd0(%rsp),%zmm2
  466fd6:	vmovups %zmm2,0x310(%rsp)
  466fe1:	vmovups %zmm1,0x2d0(%rsp)
  466fec:	vmovups %zmm0,0x290(%rsp)
  466ff7:	mov    %rbp,%rdi
  466ffa:	mov    %r15,%rsi
  466ffd:	vzeroupper
  467000:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467005:	vmovups 0x50(%rsp),%zmm0
  467010:	vmovups 0x90(%rsp),%zmm1
  46701b:	vmovups 0xd0(%rsp),%zmm2
  467026:	vmovups %zmm2,0x310(%rsp)
  467031:	vmovups %zmm1,0x2d0(%rsp)
  46703c:	vmovups %zmm0,0x290(%rsp)
  467047:	mov    %rbp,%rdi
  46704a:	mov    %r15,%rsi
  46704d:	vzeroupper
  467050:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467055:	vmovups 0x50(%rsp),%zmm0
  467060:	vmovups 0x90(%rsp),%zmm1
  46706b:	vmovups 0xd0(%rsp),%zmm2
  467076:	vmovups %zmm2,0x310(%rsp)
  467081:	vmovups %zmm1,0x2d0(%rsp)
  46708c:	vmovups %zmm0,0x290(%rsp)
  467097:	mov    %rbp,%rdi
  46709a:	mov    %r15,%rsi
  46709d:	vzeroupper
  4670a0:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  4670a5:	vmovups 0x50(%rsp),%zmm0
  4670b0:	vmovups 0x90(%rsp),%zmm1
  4670bb:	vmovups 0xd0(%rsp),%zmm2
  4670c6:	vmovups %zmm2,0x310(%rsp)
  4670d1:	vmovups %zmm1,0x2d0(%rsp)
  4670dc:	vmovups %zmm0,0x290(%rsp)
  4670e7:	mov    %rbp,%rdi
  4670ea:	mov    %r15,%rsi
  4670ed:	vzeroupper
  4670f0:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  4670f5:	vmovups 0x50(%rsp),%zmm0
  467100:	vmovups 0x90(%rsp),%zmm1
  46710b:	vmovups 0xd0(%rsp),%zmm2
  467116:	vmovups %zmm2,0x310(%rsp)
  467121:	vmovups %zmm1,0x2d0(%rsp)
  46712c:	vmovups %zmm0,0x290(%rsp)
  467137:	mov    %rbp,%rdi
  46713a:	mov    %r15,%rsi
  46713d:	vzeroupper
  467140:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467145:	vmovups 0x50(%rsp),%zmm0
  467150:	vmovups 0x90(%rsp),%zmm1
  46715b:	vmovups 0xd0(%rsp),%zmm2
  467166:	vmovups %zmm2,0x310(%rsp)
  467171:	vmovups %zmm1,0x2d0(%rsp)
  46717c:	vmovups %zmm0,0x290(%rsp)
  467187:	mov    %rbp,%rdi
  46718a:	mov    %r15,%rsi
  46718d:	vzeroupper
  467190:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467195:	vmovups 0x50(%rsp),%zmm0
  4671a0:	vmovups 0x90(%rsp),%zmm1
  4671ab:	vmovups 0xd0(%rsp),%zmm2
  4671b6:	vmovups %zmm2,0x310(%rsp)
  4671c1:	vmovups %zmm1,0x2d0(%rsp)
  4671cc:	vmovups %zmm0,0x290(%rsp)
  4671d7:	mov    %rbp,%rdi
  4671da:	mov    %r15,%rsi
  4671dd:	vzeroupper
  4671e0:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  4671e5:	vmovups 0x50(%rsp),%zmm0
  4671f0:	vmovups 0x90(%rsp),%zmm1
  4671fb:	vmovups 0xd0(%rsp),%zmm2
  467206:	vmovups %zmm2,0x310(%rsp)
  467211:	vmovups %zmm1,0x2d0(%rsp)
  46721c:	vmovups %zmm0,0x290(%rsp)
  467227:	mov    %rbp,%rdi
  46722a:	mov    %r15,%rsi
  46722d:	vzeroupper
  467230:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467235:	vmovups 0x50(%rsp),%zmm0
  467240:	vmovups 0x90(%rsp),%zmm1
  46724b:	vmovups 0xd0(%rsp),%zmm2
  467256:	vmovups %zmm2,0x310(%rsp)
  467261:	vmovups %zmm1,0x2d0(%rsp)
  46726c:	vmovups %zmm0,0x290(%rsp)
  467277:	mov    %rbp,%rdi
  46727a:	mov    %r15,%rsi
  46727d:	vzeroupper
  467280:	call   1917c0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467285:	vmovups 0x50(%rsp),%zmm0
  467290:	vmovups 0x90(%rsp),%zmm1
  46729b:	vmovups 0xd0(%rsp),%zmm2
  4672a6:	vmovups %zmm2,0x310(%rsp)
  4672b1:	vmovups %zmm1,0x2d0(%rsp)
  4672bc:	vmovups %zmm0,0x290(%rsp)
  4672c7:	mov    %rbp,%rdi
  4672ca:	mov    %r15,%rsi
  4672cd:	mov    %r12,%rdx
  4672d0:	vzeroupper
  4672d3:	call   1851f0 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  4672d8:	vmovups 0x50(%rsp),%zmm0
  4672e3:	vmovups 0x90(%rsp),%zmm1
  4672ee:	vmovups 0xd0(%rsp),%zmm2
  4672f9:	add    $0xffffffffffffff40,%r12
  467300:	add    $0xc0,%r14
  467307:	vmovups %zmm2,0x310(%rsp)
  467312:	vmovups %zmm1,0x2d0(%rsp)
  46731d:	vmovups %zmm0,0x290(%rsp)
  467328:	jne    466f57 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1677>
  46732e:	test   %r13,%r13
  467331:	je     46734e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1a6e>
  467333:	shl    $0x6,%r13
  467337:	lea    0x0(%r13,%r13,2),%rsi
  46733c:	mov    0x8(%rsp),%rdi
  467341:	mov    $0x8,%edx
  467346:	vzeroupper
  467349:	call   e9b90 <__rustc::__rust_dealloc>
  46734e:	shl    $0x3,%rbx
  467352:	lea    (%rbx,%rbx,4),%rsi
  467356:	mov    0x20(%rsp),%rdi
  46735b:	mov    $0x8,%edx
  467360:	vzeroupper
  467363:	call   e9b90 <__rustc::__rust_dealloc>
  467368:	mov    0x10(%rsp),%rax
  46736d:	lock sub %rax,0x9001b(%rip)        # 4f7390 <zopro_core::preparation::RESERVED>
  467375:	mov    0x28(%rsp),%rax
  46737a:	lock sub %rax,0x90016(%rip)        # 4f7398 <zopro_core::preparation::WORKSPACES>
  467382:	vmovups 0x290(%rsp),%zmm0
  46738d:	vmovups 0x2d0(%rsp),%zmm1
  467398:	vmovups 0x310(%rsp),%zmm2
  4673a3:	mov    0x180(%rsp),%rax
  4673ab:	vmovups %zmm2,0x80(%rax)
  4673b2:	vmovups %zmm1,0x40(%rax)
  4673b9:	vmovups %zmm0,(%rax)
  4673bf:	jmp    466dd0 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14f0>
  4673c4:	lea    0x158(%rsp),%rax
  4673cc:	lea    -0x373713(%rip),%rcx        # f3cc0 <<u64 as core::fmt::Display>::fmt>
  4673d3:	mov    %rax,0x50(%rsp)
  4673d8:	mov    %rcx,0x58(%rsp)
  4673dd:	mov    %r13,0x60(%rsp)
  4673e2:	mov    %rcx,0x68(%rsp)
  4673e7:	lea    -0x4505af(%rip),%rdi        # 16e3f <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x533f>
  4673ee:	lea    0x896e3(%rip),%rdx        # 4f0ad8 <__frame_dummy_init_array_entry+0x42b8>
  4673f5:	lea    0x50(%rsp),%rsi
  4673fa:	call   ef670 <core::panicking::panic_fmt>
  4673ff:	jmp    467450 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b70>
  467401:	lea    0x150(%rsp),%rax
  467409:	lea    -0x373750(%rip),%r8        # f3cc0 <<u64 as core::fmt::Display>::fmt>
  467410:	lea    0x30(%rsp),%rcx
  467415:	mov    %rax,0x50(%rsp)
  46741a:	mov    %r8,0x58(%rsp)
  46741f:	mov    %rcx,0x60(%rsp)
  467424:	mov    %r8,0x68(%rsp)
  467429:	mov    0x28(%rsp),%r14
  46742e:	mov    0x10(%rsp),%r12
  467433:	mov    0x20(%rsp),%rbp
  467438:	lea    -0x450600(%rip),%rdi        # 16e3f <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x533f>
  46743f:	lea    0x89692(%rip),%rdx        # 4f0ad8 <__frame_dummy_init_array_entry+0x42b8>
  467446:	lea    0x50(%rsp),%rsi
  46744b:	call   ef670 <core::panicking::panic_fmt>
  467450:	ud2
  467452:	vzeroupper
  467455:	call   1113d0 <std::process::abort>
  46745a:	mov    0x10(%rsp),%r12
  46745f:	mov    %rax,%r15
  467462:	jmp    46748e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1bae>
  467464:	mov    %rax,%r15
  467467:	mov    0x158(%rsp),%rax
  46746f:	test   %rax,%rax
  467472:	je     46748e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1bae>
  467474:	mov    0x160(%rsp),%rdi
  46747c:	shl    $0x6,%rax
  467480:	lea    (%rax,%rax,2),%rsi
  467484:	mov    $0x8,%edx
  467489:	call   e9b90 <__rustc::__rust_dealloc>
  46748e:	shl    $0x3,%rbx
  467492:	lea    (%rbx,%rbx,4),%rsi
  467496:	mov    $0x8,%edx
  46749b:	mov    %rbp,%rdi
  46749e:	call   e9b90 <__rustc::__rust_dealloc>
  4674a3:	lock sub %r12,0x8fee5(%rip)        # 4f7390 <zopro_core::preparation::RESERVED>
  4674ab:	lock sub %r14,0x8fee5(%rip)        # 4f7398 <zopro_core::preparation::WORKSPACES>
  4674b3:	mov    %r15,%rdi
  4674b6:	call   4eb7d0 <_Unwind_Resume@plt>
  4674bb:	mov    %rax,%r15
  4674be:	mov    0x118(%rsp),%rax
  4674c6:	test   %rax,%rax
  4674c9:	jne    4674d3 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1bf3>
  4674cb:	mov    %r15,%rdi
  4674ce:	call   4eb7d0 <_Unwind_Resume@plt>
  4674d3:	mov    0x120(%rsp),%rdi
  4674db:	shl    $0x6,%rax
  4674df:	lea    (%rax,%rax,2),%rsi
  4674e3:	mov    $0x8,%edx
  4674e8:	call   e9b90 <__rustc::__rust_dealloc>
  4674ed:	mov    %r15,%rdi
  4674f0:	call   4eb7d0 <_Unwind_Resume@plt>
