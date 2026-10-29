
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000465700 <zopro_core::msm_affine::g2_phi::raw_exclusive>:
  465700:	push   %rbp
  465701:	push   %r15
  465703:	push   %r14
  465705:	push   %r13
  465707:	push   %r12
  465709:	push   %rbx
  46570a:	sub    $0x358,%rsp
  465711:	mov    %rdx,0x288(%rsp)
  465719:	mov    %r8,0x50(%rsp)
  46571e:	cmp    %r8,%rdx
  465721:	jne    4667f1 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x10f1>
  465727:	lea    -0x400(%rdx),%rax
  46572e:	mov    %rcx,%r15
  465731:	mov    %rdx,%rbx
  465734:	cmp    $0xfc00,%rax
  46573a:	jae    46575e <zopro_core::msm_affine::g2_phi::raw_exclusive+0x5e>
  46573c:	mov    %fs:0xffffffffffffffc0,%rax
  465745:	mov    %rsi,(%rsp)
  465749:	mov    %rdi,0x180(%rsp)
  465751:	test   %rax,%rax
  465754:	je     46577d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x7d>
  465756:	add    $0x110,%rax
  46575c:	jmp    465782 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x82>
  46575e:	mov    %rbx,%rdx
  465761:	mov    %r15,%rcx
  465764:	mov    %rbx,%r8
  465767:	add    $0x358,%rsp
  46576e:	pop    %rbx
  46576f:	pop    %r12
  465771:	pop    %r13
  465773:	pop    %r14
  465775:	pop    %r15
  465777:	pop    %rbp
  465778:	jmp    45a6e0 <zopro_core::msm_affine::g2_horner::raw>
  46577d:	call   1052d0 <rayon_core::registry::global_registry>
  465782:	mov    (%rax),%rax
  465785:	mov    0x208(%rax),%rbp
  46578c:	lea    -0x445140(%rip),%rsi        # 20653 <zmij::STATIC_DATA+0x6153>
  465793:	lea    0x50(%rsp),%rdi
  465798:	mov    $0x15,%edx
  46579d:	call   143fc0 <std::sys::env::unix::getenv>
  4657a2:	mov    0x50(%rsp),%r14
  4657a7:	cmp    $0xffffffffffffffff,%r14
  4657ab:	je     46586c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  4657b1:	mov    0x58(%rsp),%r12
  4657b6:	mov    0x60(%rsp),%r13
  4657bb:	lea    0x50(%rsp),%rdi
  4657c0:	mov    %r12,%rsi
  4657c3:	mov    %r13,%rdx
  4657c6:	call   f12b0 <core::str::converts::from_utf8>
  4657cb:	cmpb   $0x0,0x50(%rsp)
  4657d0:	je     465802 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x102>
  4657d2:	test   %r14,%r14
  4657d5:	jle    46586c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  4657db:	cmp    $0x1ffff,%r14
  4657e2:	jbe    465845 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x145>
  4657e4:	mov    %r14,%rsi
  4657e7:	inc    %rsi
  4657ea:	js     466858 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1158>
  4657f0:	cmpb   $0x1,(%r12,%r14,1)
  4657f5:	jne    465849 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x149>
  4657f7:	mov    %r12,%rdi
  4657fa:	call   *0x8c9d8(%rip)        # 4f21d8 <munmap@GLIBC_2.2.5>
  465800:	jmp    46586c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  465802:	mov    %r15,0x48(%rsp)
  465807:	mov    $0x1,%r15b
  46580a:	cmp    $0x1,%r13
  46580e:	jne    465819 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x119>
  465810:	cmpb   $0x30,(%r12)
  465815:	setne  %r15b
  465819:	test   %r14,%r14
  46581c:	je     46585d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x15d>
  46581e:	cmp    $0x1ffff,%r14
  465825:	jbe    465847 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x147>
  465827:	mov    %r14,%rsi
  46582a:	inc    %rsi
  46582d:	js     46685d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x115d>
  465833:	cmpb   $0x1,(%r12,%r14,1)
  465838:	jne    465854 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x154>
  46583a:	mov    %r12,%rdi
  46583d:	call   *0x8c995(%rip)        # 4f21d8 <munmap@GLIBC_2.2.5>
  465843:	jmp    46585d <zopro_core::msm_affine::g2_phi::raw_exclusive+0x15d>
  465845:	jmp    465849 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x149>
  465847:	jmp    465854 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x154>
  465849:	mov    %r12,%rdi
  46584c:	call   *0x8c98e(%rip)        # 4f21e0 <free@GLIBC_2.2.5>
  465852:	jmp    46586c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x16c>
  465854:	mov    %r12,%rdi
  465857:	call   *0x8c983(%rip)        # 4f21e0 <free@GLIBC_2.2.5>
  46585d:	mov    $0x96,%eax
  465862:	test   %r15b,%r15b
  465865:	mov    0x48(%rsp),%r15
  46586a:	je     465871 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x171>
  46586c:	mov    $0x98,%eax
  465871:	mov    %eax,%edx
  465873:	shl    $0x7,%edx
  465876:	mov    %eax,%ecx
  465878:	shl    $0x6,%ecx
  46587b:	lea    (%rdx,%rax,8),%edx
  46587e:	shr    $0x3,%eax
  465881:	or     $0x40,%ecx
  465884:	imul   $0xb40,%rax,%rax
  46588b:	add    %ecx,%edx
  46588d:	lea    0x93d58(%rax,%rdx,2),%rax
  465895:	mul    %rbp
  465898:	jo     46680c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x110c>
  46589e:	lea    (%rbx,%rbx,4),%rcx
  4658a2:	xor    %esi,%esi
  4658a4:	lea    0x302e40(,%rcx,8),%rdi
  4658ac:	lea    0x302e40(%rax,%rcx,8),%rdx
  4658b4:	cmp    %rdi,%rdx
  4658b7:	setae  %sil
  4658bb:	lea    0x190(%rsp),%r13
  4658c3:	mov    %r13,%rdi
  4658c6:	call   21c2d0 <<zopro_core::preparation::Reservation>::analysis>
  4658cb:	cmpl   $0x1,0x190(%rsp)
  4658d3:	je     4659d9 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x2d9>
  4658d9:	mov    %fs:0xffffffffffffffc0,%rax
  4658e2:	movq   $0xc,0x188(%rsp)
  4658ee:	movq   $0x2,0x150(%rsp)
  4658fa:	test   %rax,%rax
  4658fd:	je     465907 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x207>
  4658ff:	add    $0x110,%rax
  465905:	jmp    46590c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x20c>
  465907:	call   1052d0 <rayon_core::registry::global_registry>
  46590c:	mov    (%rax),%rax
  46590f:	mov    0x208(%rax),%rdx
  465916:	sub    $0x8,%rsp
  46591a:	lea    0x17(%rsp),%rax
  46591f:	mov    $0x1,%ecx
  465924:	mov    %rbx,%rdi
  465927:	xor    %esi,%esi
  465929:	mov    %r15,%r8
  46592c:	mov    %rbx,%r9
  46592f:	push   %rax
  465930:	call   467dc0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterMutProducer<[u64; 4]>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::g2_phi::hb_raw::{closure#0}>>>
  465935:	add    $0x10,%rsp
  465939:	mov    %rbx,%rax
  46593c:	shr    $1,%rax
  46593f:	mov    %rbx,%rcx
  465942:	movq   $0x0,0x118(%rsp)
  46594e:	movq   $0x8,0x120(%rsp)
  46595a:	movq   $0x0,0x128(%rsp)
  465966:	movq   $0xb,0x160(%rsp)
  465972:	sub    %rax,%rcx
  465975:	mov    %rcx,0x158(%rsp)
  46597d:	lea    0x118(%rsp),%rdi
  465985:	mov    $0xb,%edx
  46598a:	mov    $0x8,%ecx
  46598f:	mov    $0xc0,%r8d
  465995:	xor    %esi,%esi
  465997:	call   1b7000 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  46599c:	mov    0x118(%rsp),%r12
  4659a4:	mov    0x128(%rsp),%rbp
  4659ac:	mov    %r12,%rax
  4659af:	sub    %rbp,%rax
  4659b2:	cmp    $0xb,%rax
  4659b6:	jae    466862 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1162>
  4659bc:	lea    -0x445e53(%rip),%rdi        # 1fb70 <zmij::STATIC_DATA+0x5670>
  4659c3:	lea    0x89d26(%rip),%rdx        # 4ef6f0 <__frame_dummy_init_array_entry+0x6240>
  4659ca:	mov    $0x2f,%esi
  4659cf:	call   f1580 <core::panicking::panic>
  4659d4:	jmp    467219 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b19>
  4659d9:	mov    0x198(%rsp),%r14
  4659e1:	mov    0x1a0(%rsp),%rbp
  4659e9:	lea    0x50(%rsp),%r12
  4659ee:	mov    $0x8,%edx
  4659f3:	mov    $0x8,%r8d
  4659f9:	mov    $0x28,%r9d
  4659ff:	xor    %esi,%esi
  465a01:	mov    %rbx,%rcx
  465a04:	mov    %r12,%rdi
  465a07:	call   1e1cd0 <<alloc::raw_vec::RawVecInner>::finish_grow>
  465a0c:	cmpb   $0x0,0x50(%rsp)
  465a11:	je     465a28 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x328>
  465a13:	lock sub %r14,0x8e5a5(%rip)        # 4f3fc0 <zopro_core::preparation::RESERVED>
  465a1b:	lock sub %rbp,0x8e5a5(%rip)        # 4f3fc8 <zopro_core::preparation::WORKSPACES>
  465a23:	jmp    4658d9 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1d9>
  465a28:	lea    0x30(%rsp),%rax
  465a2d:	mov    0x58(%rsp),%rcx
  465a32:	mov    %rbx,0x68(%rsp)
  465a37:	mov    %r14,0x10(%rsp)
  465a3c:	mov    %r15,0x20(%rsp)
  465a41:	mov    %rbx,0x118(%rsp)
  465a49:	mov    %rbx,0x28(%rsp)
  465a4e:	mov    %rax,0x50(%rsp)
  465a53:	mov    %fs:0xffffffffffffffc0,%rax
  465a5c:	mov    %rcx,0x58(%rsp)
  465a61:	mov    %rcx,0x148(%rsp)
  465a69:	mov    %rbx,0x60(%rsp)
  465a6e:	test   %rax,%rax
  465a71:	je     465a7b <zopro_core::msm_affine::g2_phi::raw_exclusive+0x37b>
  465a73:	add    $0x110,%rax
  465a79:	jmp    465a80 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x380>
  465a7b:	call   1052d0 <rayon_core::registry::global_registry>
  465a80:	mov    (%rax),%rax
  465a83:	mov    0x208(%rax),%rcx
  465a8a:	lea    0x190(%rsp),%r13
  465a92:	xor    %edx,%edx
  465a94:	mov    $0x1,%r8d
  465a9a:	mov    %rbx,%rsi
  465a9d:	mov    %r15,%r9
  465aa0:	mov    %r13,%rdi
  465aa3:	push   %r12
  465aa5:	push   %rbx
  465aa6:	call   4672d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  465aab:	add    $0x10,%rsp
  465aaf:	mov    0x1a0(%rsp),%rax
  465ab7:	mov    %rax,0x20(%rsp)
  465abc:	cmp    %rbx,%rax
  465abf:	jne    466813 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1113>
  465ac5:	lea    -0x44509b(%rip),%rsi        # 20a31 <zmij::STATIC_DATA+0x6531>
  465acc:	lea    0x50(%rsp),%rdi
  465ad1:	mov    $0x17,%edx
  465ad6:	call   143fc0 <std::sys::env::unix::getenv>
  465adb:	mov    0x50(%rsp),%rsi
  465ae0:	mov    %rbp,0x18(%rsp)
  465ae5:	cmp    $0xffffffffffffffff,%rsi
  465ae9:	je     465b29 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x429>
  465aeb:	test   %rsi,%rsi
  465aee:	je     465aff <zopro_core::msm_affine::g2_phi::raw_exclusive+0x3ff>
  465af0:	mov    0x58(%rsp),%rdi
  465af5:	mov    $0x1,%edx
  465afa:	call   e99d0 <__rustc::__rust_dealloc>
  465aff:	lea    -0x372006(%rip),%rax        # f3b00 <<u64 as core::fmt::Display>::fmt>
  465b06:	mov    %r13,0x50(%rsp)
  465b0b:	mov    %rbx,0x190(%rsp)
  465b13:	mov    %rax,0x58(%rsp)
  465b18:	lea    -0x44ffd0(%rip),%rdi        # 15b4f <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x408f>
  465b1f:	lea    0x50(%rsp),%rsi
  465b24:	call   140c10 <std::io::stdio::_eprint>
  465b29:	vmovups -0x453e71(%rip),%ymm0        # 11cc0 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x200>
  465b31:	vmovups -0x453f99(%rip),%ymm2        # 11ba0 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0xe0>
  465b39:	lea    0x3(%rbx),%rax
  465b3d:	lea    0x20(%rsp),%rcx
  465b42:	lea    0x50(%rsp),%rsi
  465b47:	movabs $0x87d20782e4866389,%r12
  465b51:	vmovups -0x4451f1(%rip),%ymm1        # 20968 <zmij::STATIC_DATA+0x6468>
  465b59:	lea    0x190(%rsp),%r13
  465b61:	shr    $0x2,%rax
  465b65:	mov    %rax,0x150(%rsp)
  465b6d:	lea    0x190(%rsp),%rax
  465b75:	vmovups %ymm0,0x20(%rsp)
  465b7b:	vmovups %ymm0,0x190(%rsp)
  465b84:	vmovups %ymm2,0x50(%rsp)
  465b8a:	xor    %r10d,%r10d
  465b8d:	xor    %r9d,%r9d
  465b90:	xor    %r8d,%r8d
  465b93:	xor    %edi,%edi
  465b95:	xor    %r11d,%r11d
  465b98:	mov    (%rcx),%rdx
  465b9b:	xor    %r15d,%r15d
  465b9e:	mulx   (%rax),%rbp,%r14
  465ba3:	adcx   %rbp,%r10
  465ba9:	adox   %r14,%r9
  465baf:	mulx   0x8(%rax),%rbp,%r14
  465bb5:	adcx   %rbp,%r9
  465bbb:	adox   %r14,%r8
  465bc1:	mulx   0x10(%rax),%rbp,%r14
  465bc7:	adcx   %rbp,%r8
  465bcd:	adox   %r14,%rdi
  465bd3:	mulx   0x18(%rax),%rbp,%r14
  465bd9:	adcx   %rbp,%rdi
  465bdf:	adox   %r14,%r11
  465be5:	adcx   %r15,%r11
  465beb:	mov    %r10,%rdx
  465bee:	imul   %r12,%rdx
  465bf2:	xor    %r15d,%r15d
  465bf5:	mulx   (%rsi),%rbp,%r14
  465bfa:	adcx   %rbp,%r10
  465c00:	adox   %r14,%r9
  465c06:	mulx   0x8(%rsi),%rbp,%r14
  465c0c:	adcx   %rbp,%r9
  465c12:	adox   %r14,%r8
  465c18:	mulx   0x10(%rsi),%rbp,%r14
  465c1e:	adcx   %rbp,%r8
  465c24:	adox   %r14,%rdi
  465c2a:	mulx   0x18(%rsi),%rbp,%r14
  465c30:	adcx   %rbp,%rdi
  465c36:	adox   %r14,%r11
  465c3c:	adcx   %r15,%r11
  465c42:	mov    0x8(%rcx),%rdx
  465c46:	xor    %r15d,%r15d
  465c49:	mulx   (%rax),%rbp,%r14
  465c4e:	adcx   %rbp,%r9
  465c54:	adox   %r14,%r8
  465c5a:	mulx   0x8(%rax),%rbp,%r14
  465c60:	adcx   %rbp,%r8
  465c66:	adox   %r14,%rdi
  465c6c:	mulx   0x10(%rax),%rbp,%r14
  465c72:	adcx   %rbp,%rdi
  465c78:	adox   %r14,%r11
  465c7e:	mulx   0x18(%rax),%rbp,%r14
  465c84:	adcx   %rbp,%r11
  465c8a:	adox   %r14,%r10
  465c90:	adcx   %r15,%r10
  465c96:	mov    %r9,%rdx
  465c99:	imul   %r12,%rdx
  465c9d:	xor    %r15d,%r15d
  465ca0:	mulx   (%rsi),%rbp,%r14
  465ca5:	adcx   %rbp,%r9
  465cab:	adox   %r14,%r8
  465cb1:	mulx   0x8(%rsi),%rbp,%r14
  465cb7:	adcx   %rbp,%r8
  465cbd:	adox   %r14,%rdi
  465cc3:	mulx   0x10(%rsi),%rbp,%r14
  465cc9:	adcx   %rbp,%rdi
  465ccf:	adox   %r14,%r11
  465cd5:	mulx   0x18(%rsi),%rbp,%r14
  465cdb:	adcx   %rbp,%r11
  465ce1:	adox   %r14,%r10
  465ce7:	adcx   %r15,%r10
  465ced:	mov    0x10(%rcx),%rdx
  465cf1:	xor    %r15d,%r15d
  465cf4:	mulx   (%rax),%rbp,%r14
  465cf9:	adcx   %rbp,%r8
  465cff:	adox   %r14,%rdi
  465d05:	mulx   0x8(%rax),%rbp,%r14
  465d0b:	adcx   %rbp,%rdi
  465d11:	adox   %r14,%r11
  465d17:	mulx   0x10(%rax),%rbp,%r14
  465d1d:	adcx   %rbp,%r11
  465d23:	adox   %r14,%r10
  465d29:	mulx   0x18(%rax),%rbp,%r14
  465d2f:	adcx   %rbp,%r10
  465d35:	adox   %r14,%r9
  465d3b:	adcx   %r15,%r9
  465d41:	mov    %r8,%rdx
  465d44:	imul   %r12,%rdx
  465d48:	xor    %r15d,%r15d
  465d4b:	mulx   (%rsi),%rbp,%r14
  465d50:	adcx   %rbp,%r8
  465d56:	adox   %r14,%rdi
  465d5c:	mulx   0x8(%rsi),%rbp,%r14
  465d62:	adcx   %rbp,%rdi
  465d68:	adox   %r14,%r11
  465d6e:	mulx   0x10(%rsi),%rbp,%r14
  465d74:	adcx   %rbp,%r11
  465d7a:	adox   %r14,%r10
  465d80:	mulx   0x18(%rsi),%rbp,%r14
  465d86:	adcx   %rbp,%r10
  465d8c:	adox   %r14,%r9
  465d92:	adcx   %r15,%r9
  465d98:	mov    0x18(%rcx),%rdx
  465d9c:	xor    %r15d,%r15d
  465d9f:	mulx   (%rax),%rbp,%r14
  465da4:	adcx   %rbp,%rdi
  465daa:	adox   %r14,%r11
  465db0:	mulx   0x8(%rax),%rbp,%r14
  465db6:	adcx   %rbp,%r11
  465dbc:	adox   %r14,%r10
  465dc2:	mulx   0x10(%rax),%rbp,%r14
  465dc8:	adcx   %rbp,%r10
  465dce:	adox   %r14,%r9
  465dd4:	mulx   0x18(%rax),%rbp,%r14
  465dda:	adcx   %rbp,%r9
  465de0:	adox   %r14,%r8
  465de6:	adcx   %r15,%r8
  465dec:	mov    %rdi,%rdx
  465def:	imul   %r12,%rdx
  465df3:	xor    %r15d,%r15d
  465df6:	mulx   (%rsi),%rbp,%r14
  465dfb:	adcx   %rbp,%rdi
  465e01:	adox   %r14,%r11
  465e07:	mulx   0x8(%rsi),%rbp,%r14
  465e0d:	adcx   %rbp,%r11
  465e13:	adox   %r14,%r10
  465e19:	mulx   0x10(%rsi),%rbp,%r14
  465e1f:	adcx   %rbp,%r10
  465e25:	adox   %r14,%r9
  465e2b:	mulx   0x18(%rsi),%rbp,%r14
  465e31:	adcx   %rbp,%r9
  465e37:	adox   %r14,%r8
  465e3d:	adcx   %r15,%r8
  465e43:	movabs $0x687e956e978e3573,%rax
  465e4d:	movabs $0x3c208c16d87cfd47,%r14
  465e57:	vmovups %ymm1,0x20(%rsp)
  465e5d:	vmovups %ymm2,0x50(%rsp)
  465e63:	lea    0x20(%rsp),%r12
  465e68:	vmovups -0x4454e8(%rip),%ymm1        # 20988 <zmij::STATIC_DATA+0x6488>
  465e70:	lea    (%r10,%rax,1),%rsi
  465e74:	cmp    %r14,%r11
  465e77:	movabs $0x97816a916871ca8d,%rax
  465e81:	sbb    $0x0,%rsi
  465e85:	setb   %cl
  465e88:	cmp    %rax,%r10
  465e8b:	movabs $0x47afba497e7ea7a3,%rax
  465e95:	setb   %dl
  465e98:	or     %cl,%dl
  465e9a:	lea    (%r9,%rax,1),%rcx
  465e9e:	movabs $0xb85045b68181585d,%rax
  465ea8:	movzbl %dl,%edx
  465eab:	sub    %rdx,%rcx
  465eae:	setb   %dl
  465eb1:	cmp    %rax,%r9
  465eb4:	movabs $0xcf9bb18d1ece5fd7,%rax
  465ebe:	setb   %dil
  465ec2:	lea    (%r8,%rax,1),%r15
  465ec6:	movabs $0x30644e72e131a029,%rax
  465ed0:	or     %dl,%dil
  465ed3:	movzbl %dil,%edx
  465ed7:	sub    %rdx,%r15
  465eda:	setb   %dl
  465edd:	cmp    %rax,%r8
  465ee0:	movabs $0xc3df73e9278302b9,%rax
  465eea:	setb   %dil
  465eee:	add    %r11,%rax
  465ef1:	or     %dl,%dil
  465ef4:	cmovne %r11,%rax
  465ef8:	cmovne %r10,%rsi
  465efc:	cmovne %r9,%rcx
  465f00:	cmovne %r8,%r15
  465f04:	lea    0x50(%rsp),%r11
  465f09:	mov    %rax,0x138(%rsp)
  465f11:	mov    %rax,0x190(%rsp)
  465f19:	mov    %rsi,0x140(%rsp)
  465f21:	mov    %rsi,0x198(%rsp)
  465f29:	mov    %rcx,0x1a0(%rsp)
  465f31:	mov    %rcx,0x130(%rsp)
  465f39:	mov    %r15,0x1a8(%rsp)
  465f41:	movabs $0x87d20782e4866389,%rax
  465f4b:	xor    %esi,%esi
  465f4d:	xor    %edi,%edi
  465f4f:	xor    %ebp,%ebp
  465f51:	xor    %r14d,%r14d
  465f54:	xor    %ecx,%ecx
  465f56:	mov    0x0(%r13),%rdx
  465f5a:	xor    %r10d,%r10d
  465f5d:	mulx   (%r12),%r8,%r9
  465f63:	adcx   %r8,%rsi
  465f69:	adox   %r9,%rdi
  465f6f:	mulx   0x8(%r12),%r8,%r9
  465f76:	adcx   %r8,%rdi
  465f7c:	adox   %r9,%rbp
  465f82:	mulx   0x10(%r12),%r8,%r9
  465f89:	adcx   %r8,%rbp
  465f8f:	adox   %r9,%r14
  465f95:	mulx   0x18(%r12),%r8,%r9
  465f9c:	adcx   %r8,%r14
  465fa2:	adox   %r9,%rcx
  465fa8:	adcx   %r10,%rcx
  465fae:	mov    %rsi,%rdx
  465fb1:	imul   %rax,%rdx
  465fb5:	xor    %r10d,%r10d
  465fb8:	mulx   (%r11),%r8,%r9
  465fbd:	adcx   %r8,%rsi
  465fc3:	adox   %r9,%rdi
  465fc9:	mulx   0x8(%r11),%r8,%r9
  465fcf:	adcx   %r8,%rdi
  465fd5:	adox   %r9,%rbp
  465fdb:	mulx   0x10(%r11),%r8,%r9
  465fe1:	adcx   %r8,%rbp
  465fe7:	adox   %r9,%r14
  465fed:	mulx   0x18(%r11),%r8,%r9
  465ff3:	adcx   %r8,%r14
  465ff9:	adox   %r9,%rcx
  465fff:	adcx   %r10,%rcx
  466005:	mov    0x8(%r13),%rdx
  466009:	xor    %r10d,%r10d
  46600c:	mulx   (%r12),%r8,%r9
  466012:	adcx   %r8,%rdi
  466018:	adox   %r9,%rbp
  46601e:	mulx   0x8(%r12),%r8,%r9
  466025:	adcx   %r8,%rbp
  46602b:	adox   %r9,%r14
  466031:	mulx   0x10(%r12),%r8,%r9
  466038:	adcx   %r8,%r14
  46603e:	adox   %r9,%rcx
  466044:	mulx   0x18(%r12),%r8,%r9
  46604b:	adcx   %r8,%rcx
  466051:	adox   %r9,%rsi
  466057:	adcx   %r10,%rsi
  46605d:	mov    %rdi,%rdx
  466060:	imul   %rax,%rdx
  466064:	xor    %r10d,%r10d
  466067:	mulx   (%r11),%r8,%r9
  46606c:	adcx   %r8,%rdi
  466072:	adox   %r9,%rbp
  466078:	mulx   0x8(%r11),%r8,%r9
  46607e:	adcx   %r8,%rbp
  466084:	adox   %r9,%r14
  46608a:	mulx   0x10(%r11),%r8,%r9
  466090:	adcx   %r8,%r14
  466096:	adox   %r9,%rcx
  46609c:	mulx   0x18(%r11),%r8,%r9
  4660a2:	adcx   %r8,%rcx
  4660a8:	adox   %r9,%rsi
  4660ae:	adcx   %r10,%rsi
  4660b4:	mov    0x10(%r13),%rdx
  4660b8:	xor    %r10d,%r10d
  4660bb:	mulx   (%r12),%r8,%r9
  4660c1:	adcx   %r8,%rbp
  4660c7:	adox   %r9,%r14
  4660cd:	mulx   0x8(%r12),%r8,%r9
  4660d4:	adcx   %r8,%r14
  4660da:	adox   %r9,%rcx
  4660e0:	mulx   0x10(%r12),%r8,%r9
  4660e7:	adcx   %r8,%rcx
  4660ed:	adox   %r9,%rsi
  4660f3:	mulx   0x18(%r12),%r8,%r9
  4660fa:	adcx   %r8,%rsi
  466100:	adox   %r9,%rdi
  466106:	adcx   %r10,%rdi
  46610c:	mov    %rbp,%rdx
  46610f:	imul   %rax,%rdx
  466113:	xor    %r10d,%r10d
  466116:	mulx   (%r11),%r8,%r9
  46611b:	adcx   %r8,%rbp
  466121:	adox   %r9,%r14
  466127:	mulx   0x8(%r11),%r8,%r9
  46612d:	adcx   %r8,%r14
  466133:	adox   %r9,%rcx
  466139:	mulx   0x10(%r11),%r8,%r9
  46613f:	adcx   %r8,%rcx
  466145:	adox   %r9,%rsi
  46614b:	mulx   0x18(%r11),%r8,%r9
  466151:	adcx   %r8,%rsi
  466157:	adox   %r9,%rdi
  46615d:	adcx   %r10,%rdi
  466163:	mov    0x18(%r13),%rdx
  466167:	xor    %r10d,%r10d
  46616a:	mulx   (%r12),%r8,%r9
  466170:	adcx   %r8,%r14
  466176:	adox   %r9,%rcx
  46617c:	mulx   0x8(%r12),%r8,%r9
  466183:	adcx   %r8,%rcx
  466189:	adox   %r9,%rsi
  46618f:	mulx   0x10(%r12),%r8,%r9
  466196:	adcx   %r8,%rsi
  46619c:	adox   %r9,%rdi
  4661a2:	mulx   0x18(%r12),%r8,%r9
  4661a9:	adcx   %r8,%rdi
  4661af:	adox   %r9,%rbp
  4661b5:	adcx   %r10,%rbp
  4661bb:	mov    %r14,%rdx
  4661be:	imul   %rax,%rdx
  4661c2:	xor    %r10d,%r10d
  4661c5:	mulx   (%r11),%r8,%r9
  4661ca:	adcx   %r8,%r14
  4661d0:	adox   %r9,%rcx
  4661d6:	mulx   0x8(%r11),%r8,%r9
  4661dc:	adcx   %r8,%rcx
  4661e2:	adox   %r9,%rsi
  4661e8:	mulx   0x10(%r11),%r8,%r9
  4661ee:	adcx   %r8,%rsi
  4661f4:	adox   %r9,%rdi
  4661fa:	mulx   0x18(%r11),%r8,%r9
  466200:	adcx   %r8,%rdi
  466206:	adox   %r9,%rbp
  46620c:	adcx   %r10,%rbp
  466212:	movabs $0x687e956e978e3573,%rax
  46621c:	movabs $0x3c208c16d87cfd47,%r10
  466226:	mov    %rbp,0x178(%rsp)
  46622e:	mov    %rdi,0x48(%rsp)
  466233:	mov    %rcx,0x270(%rsp)
  46623b:	mov    %rsi,0x278(%rsp)
  466243:	movabs $0x97816a916871ca8d,%rdi
  46624d:	vmovups %ymm1,0x20(%rsp)
  466253:	vmovups %ymm2,0x50(%rsp)
  466259:	movabs $0x87d20782e4866389,%r12
  466263:	lea    0x50(%rsp),%rbp
  466268:	cmp    %r10,%rcx
  46626b:	lea    (%rsi,%rax,1),%rax
  46626f:	sbb    $0x0,%rax
  466273:	mov    %rax,0x280(%rsp)
  46627b:	setb   %dl
  46627e:	cmp    %rdi,%rsi
  466281:	movabs $0x47afba497e7ea7a3,%rax
  46628b:	mov    0x48(%rsp),%rsi
  466290:	setb   %r8b
  466294:	or     %dl,%r8b
  466297:	movzbl %r8b,%edx
  46629b:	movabs $0xb85045b68181585d,%r8
  4662a5:	add    %rsi,%rax
  4662a8:	sub    %rdx,%rax
  4662ab:	mov    %rax,0x268(%rsp)
  4662b3:	setb   %dl
  4662b6:	cmp    %r8,%rsi
  4662b9:	movabs $0xcf9bb18d1ece5fd7,%rax
  4662c3:	mov    0x178(%rsp),%rsi
  4662cb:	setb   %r9b
  4662cf:	or     %dl,%r9b
  4662d2:	movzbl %r9b,%edx
  4662d6:	add    %rsi,%rax
  4662d9:	sub    %rdx,%rax
  4662dc:	mov    %rax,0x258(%rsp)
  4662e4:	mov    0x138(%rsp),%rax
  4662ec:	mov    0x140(%rsp),%rdi
  4662f4:	mov    0x130(%rsp),%rsi
  4662fc:	setb   0x138(%rsp)
  466304:	mov    %rax,0x190(%rsp)
  46630c:	mov    %rdi,0x198(%rsp)
  466314:	movabs $0xc3df73e9278302b9,%rdi
  46631e:	mov    %rsi,0x1a0(%rsp)
  466326:	mov    %r15,0x1a8(%rsp)
  46632e:	lea    0x20(%rsp),%rsi
  466333:	add    %rdi,%rcx
  466336:	mov    %rcx,0x140(%rsp)
  46633e:	lea    0x190(%rsp),%rcx
  466346:	xor    %r11d,%r11d
  466349:	xor    %eax,%eax
  46634b:	xor    %r8d,%r8d
  46634e:	xor    %r10d,%r10d
  466351:	xor    %r9d,%r9d
  466354:	mov    (%rcx),%rdx
  466357:	xor    %r13d,%r13d
  46635a:	mulx   (%rsi),%r15,%r14
  46635f:	adcx   %r15,%r11
  466365:	adox   %r14,%rax
  46636b:	mulx   0x8(%rsi),%r15,%r14
  466371:	adcx   %r15,%rax
  466377:	adox   %r14,%r8
  46637d:	mulx   0x10(%rsi),%r15,%r14
  466383:	adcx   %r15,%r8
  466389:	adox   %r14,%r10
  46638f:	mulx   0x18(%rsi),%r15,%r14
  466395:	adcx   %r15,%r10
  46639b:	adox   %r14,%r9
  4663a1:	adcx   %r13,%r9
  4663a7:	mov    %r11,%rdx
  4663aa:	imul   %r12,%rdx
  4663ae:	xor    %r13d,%r13d
  4663b1:	mulx   0x0(%rbp),%r15,%r14
  4663b7:	adcx   %r15,%r11
  4663bd:	adox   %r14,%rax
  4663c3:	mulx   0x8(%rbp),%r15,%r14
  4663c9:	adcx   %r15,%rax
  4663cf:	adox   %r14,%r8
  4663d5:	mulx   0x10(%rbp),%r15,%r14
  4663db:	adcx   %r15,%r8
  4663e1:	adox   %r14,%r10
  4663e7:	mulx   0x18(%rbp),%r15,%r14
  4663ed:	adcx   %r15,%r10
  4663f3:	adox   %r14,%r9
  4663f9:	adcx   %r13,%r9
  4663ff:	mov    0x8(%rcx),%rdx
  466403:	xor    %r13d,%r13d
  466406:	mulx   (%rsi),%r15,%r14
  46640b:	adcx   %r15,%rax
  466411:	adox   %r14,%r8
  466417:	mulx   0x8(%rsi),%r15,%r14
  46641d:	adcx   %r15,%r8
  466423:	adox   %r14,%r10
  466429:	mulx   0x10(%rsi),%r15,%r14
  46642f:	adcx   %r15,%r10
  466435:	adox   %r14,%r9
  46643b:	mulx   0x18(%rsi),%r15,%r14
  466441:	adcx   %r15,%r9
  466447:	adox   %r14,%r11
  46644d:	adcx   %r13,%r11
  466453:	mov    %rax,%rdx
  466456:	imul   %r12,%rdx
  46645a:	xor    %r13d,%r13d
  46645d:	mulx   0x0(%rbp),%r15,%r14
  466463:	adcx   %r15,%rax
  466469:	adox   %r14,%r8
  46646f:	mulx   0x8(%rbp),%r15,%r14
  466475:	adcx   %r15,%r8
  46647b:	adox   %r14,%r10
  466481:	mulx   0x10(%rbp),%r15,%r14
  466487:	adcx   %r15,%r10
  46648d:	adox   %r14,%r9
  466493:	mulx   0x18(%rbp),%r15,%r14
  466499:	adcx   %r15,%r9
  46649f:	adox   %r14,%r11
  4664a5:	adcx   %r13,%r11
  4664ab:	mov    0x10(%rcx),%rdx
  4664af:	xor    %r13d,%r13d
  4664b2:	mulx   (%rsi),%r15,%r14
  4664b7:	adcx   %r15,%r8
  4664bd:	adox   %r14,%r10
  4664c3:	mulx   0x8(%rsi),%r15,%r14
  4664c9:	adcx   %r15,%r10
  4664cf:	adox   %r14,%r9
  4664d5:	mulx   0x10(%rsi),%r15,%r14
  4664db:	adcx   %r15,%r9
  4664e1:	adox   %r14,%r11
  4664e7:	mulx   0x18(%rsi),%r15,%r14
  4664ed:	adcx   %r15,%r11
  4664f3:	adox   %r14,%rax
  4664f9:	adcx   %r13,%rax
  4664ff:	mov    %r8,%rdx
  466502:	imul   %r12,%rdx
  466506:	xor    %r13d,%r13d
  466509:	mulx   0x0(%rbp),%r15,%r14
  46650f:	adcx   %r15,%r8
  466515:	adox   %r14,%r10
  46651b:	mulx   0x8(%rbp),%r15,%r14
  466521:	adcx   %r15,%r10
  466527:	adox   %r14,%r9
  46652d:	mulx   0x10(%rbp),%r15,%r14
  466533:	adcx   %r15,%r9
  466539:	adox   %r14,%r11
  46653f:	mulx   0x18(%rbp),%r15,%r14
  466545:	adcx   %r15,%r11
  46654b:	adox   %r14,%rax
  466551:	adcx   %r13,%rax
  466557:	mov    0x18(%rcx),%rdx
  46655b:	xor    %r13d,%r13d
  46655e:	mulx   (%rsi),%r15,%r14
  466563:	adcx   %r15,%r10
  466569:	adox   %r14,%r9
  46656f:	mulx   0x8(%rsi),%r15,%r14
  466575:	adcx   %r15,%r9
  46657b:	adox   %r14,%r11
  466581:	mulx   0x10(%rsi),%r15,%r14
  466587:	adcx   %r15,%r11
  46658d:	adox   %r14,%rax
  466593:	mulx   0x18(%rsi),%r15,%r14
  466599:	adcx   %r15,%rax
  46659f:	adox   %r14,%r8
  4665a5:	adcx   %r13,%r8
  4665ab:	mov    %r10,%rdx
  4665ae:	imul   %r12,%rdx
  4665b2:	xor    %r13d,%r13d
  4665b5:	mulx   0x0(%rbp),%r15,%r14
  4665bb:	adcx   %r15,%r10
  4665c1:	adox   %r14,%r9
  4665c7:	mulx   0x8(%rbp),%r15,%r14
  4665cd:	adcx   %r15,%r9
  4665d3:	adox   %r14,%r11
  4665d9:	mulx   0x10(%rbp),%r15,%r14
  4665df:	adcx   %r15,%r11
  4665e5:	adox   %r14,%rax
  4665eb:	mulx   0x18(%rbp),%r15,%r14
  4665f1:	adcx   %r15,%rax
  4665f7:	adox   %r14,%r8
  4665fd:	adcx   %r13,%r8
  466603:	mov    %rax,0x260(%rsp)
  46660b:	mov    %r8,0x130(%rsp)
  466613:	movabs $0x687e956e978e3573,%rax
  46661d:	movabs $0x3c208c16d87cfd47,%rdx
  466627:	add    %r9,%rdi
  46662a:	movabs $0x97816a916871ca8d,%r10
  466634:	movabs $0x47afba497e7ea7a3,%rcx
  46663e:	movabs $0xcf9bb18d1ece5fd7,%rsi
  466648:	movabs $0x30644e72e131a029,%r14
  466652:	mov    %r11,%rbp
  466655:	movq   $0x0,0x160(%rsp)
  466661:	movq   $0x8,0x168(%rsp)
  46666d:	movq   $0x0,0x170(%rsp)
  466679:	movq   $0x6,0x158(%rsp)
  466685:	add    %r11,%rax
  466688:	cmp    %rdx,%r9
  46668b:	mov    0x260(%rsp),%r13
  466693:	mov    0x130(%rsp),%r12
  46669b:	mov    0x178(%rsp),%r15
  4666a3:	mov    0x268(%rsp),%r8
  4666ab:	sbb    $0x0,%rax
  4666af:	setb   %dl
  4666b2:	cmp    %r10,%r11
  4666b5:	mov    0x258(%rsp),%r11
  4666bd:	setb   %r10b
  4666c1:	or     %dl,%r10b
  4666c4:	movzbl %r10b,%edx
  4666c8:	add    %r13,%rcx
  4666cb:	movabs $0xb85045b68181585d,%r10
  4666d5:	sub    %rdx,%rcx
  4666d8:	setb   %dl
  4666db:	cmp    %r10,%r13
  4666de:	setb   %r10b
  4666e2:	add    %r12,%rsi
  4666e5:	or     %dl,%r10b
  4666e8:	movzbl %r10b,%edx
  4666ec:	sub    %rdx,%rsi
  4666ef:	setb   %dl
  4666f2:	cmp    %r14,%r12
  4666f5:	setb   %r10b
  4666f9:	or     %dl,%r10b
  4666fc:	cmp    %r14,%r15
  4666ff:	mov    0x140(%rsp),%r14
  466707:	setb   %dl
  46670a:	or     0x138(%rsp),%dl
  466711:	mov    0x280(%rsp),%rdx
  466719:	cmovne 0x270(%rsp),%r14
  466722:	cmovne 0x278(%rsp),%rdx
  46672b:	cmovne 0x48(%rsp),%r8
  466731:	cmovne %r15,%r11
  466735:	test   %r10b,%r10b
  466738:	cmovne %rbp,%rax
  46673c:	cmovne %r9,%rdi
  466740:	cmovne %r13,%rcx
  466744:	cmovne %r12,%rsi
  466748:	mov    %r14,0x190(%rsp)
  466750:	mov    %rdx,0x198(%rsp)
  466758:	mov    %r8,0x1a0(%rsp)
  466760:	mov    %r11,0x1a8(%rsp)
  466768:	mov    %rdi,0x1b0(%rsp)
  466770:	mov    %rax,0x1b8(%rsp)
  466778:	mov    %rcx,0x1c0(%rsp)
  466780:	mov    %rsi,0x1c8(%rsp)
  466788:	mov    0x18(%rsp),%rbp
  46678d:	mov    0x10(%rsp),%r12
  466792:	lea    0x160(%rsp),%rdi
  46679a:	mov    $0x6,%edx
  46679f:	mov    $0x8,%ecx
  4667a4:	mov    $0xc0,%r8d
  4667aa:	xor    %esi,%esi
  4667ac:	vzeroupper
  4667af:	call   1b7000 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  4667b4:	mov    0x160(%rsp),%r13
  4667bc:	mov    0x170(%rsp),%r14
  4667c4:	mov    %r13,%rax
  4667c7:	sub    %r14,%rax
  4667ca:	cmp    $0x5,%rax
  4667ce:	ja     466bb6 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14b6>
  4667d4:	lea    -0x446c6b(%rip),%rdi        # 1fb70 <zmij::STATIC_DATA+0x5670>
  4667db:	lea    0x88f0e(%rip),%rdx        # 4ef6f0 <__frame_dummy_init_array_entry+0x6240>
  4667e2:	mov    $0x2f,%esi
  4667e7:	call   f1580 <core::panicking::panic>
  4667ec:	jmp    467219 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b19>
  4667f1:	lea    0x8a038(%rip),%r8        # 4f0830 <__frame_dummy_init_array_entry+0x7380>
  4667f8:	lea    0x288(%rsp),%rdi
  466800:	lea    0x50(%rsp),%rsi
  466805:	xor    %edx,%edx
  466807:	call   f0902 <core::panicking::assert_failed::<usize, usize>>
  46680c:	xor    %esi,%esi
  46680e:	jmp    4658bb <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1bb>
  466813:	lea    0x118(%rsp),%rax
  46681b:	lea    -0x372d22(%rip),%r8        # f3b00 <<u64 as core::fmt::Display>::fmt>
  466822:	lea    0x20(%rsp),%rcx
  466827:	mov    %rax,0x50(%rsp)
  46682c:	mov    %r8,0x58(%rsp)
  466831:	mov    %rcx,0x60(%rsp)
  466836:	mov    %r8,0x68(%rsp)
  46683b:	lea    -0x44fa43(%rip),%rdi        # 16dff <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x533f>
  466842:	lea    0x86f1f(%rip),%rdx        # 4ed768 <__frame_dummy_init_array_entry+0x42b8>
  466849:	lea    0x50(%rsp),%rsi
  46684e:	call   ef4b0 <core::panicking::panic_fmt>
  466853:	jmp    467219 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b19>
  466858:	call   111210 <std::process::abort>
  46685d:	call   111210 <std::process::abort>
  466862:	mov    (%rsp),%rsi
  466866:	mov    0x120(%rsp),%rcx
  46686e:	lea    0x150(%rsp),%rax
  466876:	lea    0x158(%rsp),%rdx
  46687e:	movq   $0xb,0x1a8(%rsp)
  46688a:	mov    %rax,0x50(%rsp)
  46688f:	mov    %fs:0xffffffffffffffc0,%rax
  466898:	mov    %rdx,0x58(%rsp)
  46689d:	lea    0x188(%rsp),%rdx
  4668a5:	mov    %rsi,0x60(%rsp)
  4668aa:	lea    0x0(%rbp,%rbp,2),%rsi
  4668af:	mov    %rbx,0x68(%rsp)
  4668b4:	mov    %r15,0x70(%rsp)
  4668b9:	mov    %rbx,0x78(%rsp)
  4668be:	mov    %rdx,0x80(%rsp)
  4668c6:	lea    0x50(%rsp),%rdx
  4668cb:	movq   $0x0,0x88(%rsp)
  4668d7:	movq   $0xb,0x90(%rsp)
  4668e3:	mov    %rcx,(%rsp)
  4668e7:	mov    %rdx,0x190(%rsp)
  4668ef:	shl    $0x6,%rsi
  4668f3:	add    %rcx,%rsi
  4668f6:	mov    %rsi,0x198(%rsp)
  4668fe:	movq   $0xb,0x1a0(%rsp)
  46690a:	test   %rax,%rax
  46690d:	je     466917 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1217>
  46690f:	add    $0x110,%rax
  466915:	jmp    46691c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x121c>
  466917:	call   1052d0 <rayon_core::registry::global_registry>
  46691c:	mov    (%rax),%rax
  46691f:	mov    0x208(%rax),%rcx
  466926:	xor    %edx,%edx
  466928:	lea    0x20(%rsp),%rdi
  46692d:	mov    $0xb,%esi
  466932:	mov    $0x1,%r8d
  466938:	xor    %r9d,%r9d
  46693b:	push   %r13
  46693d:	push   $0xb
  46693f:	call   468230 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_phi::hb_raw::{closure#1}>>>
  466944:	add    $0x10,%rsp
  466948:	mov    0x30(%rsp),%rax
  46694d:	mov    %rax,0x190(%rsp)
  466955:	cmp    $0xb,%rax
  466959:	jne    467192 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1a92>
  46695f:	vmovups -0x447419(%rip),%zmm0        # 1f550 <zmij::STATIC_DATA+0x5050>
  466969:	vmovups -0x447463(%rip),%zmm2        # 1f510 <zmij::STATIC_DATA+0x5010>
  466973:	vmovups -0x4474ad(%rip),%zmm1        # 1f4d0 <zmij::STATIC_DATA+0x4fd0>
  46697d:	mov    0x188(%rsp),%r13
  466985:	shl    $0x6,%rbp
  466989:	lea    0x0(%rbp,%rbp,2),%r15
  46698e:	vmovups %zmm0,0x210(%rsp)
  466999:	vmovups %zmm2,0x1d0(%rsp)
  4669a4:	vmovups %zmm1,0x190(%rsp)
  4669af:	test   %r13,%r13
  4669b2:	je     466a8f <zopro_core::msm_affine::g2_phi::raw_exclusive+0x138f>
  4669b8:	mov    (%rsp),%rax
  4669bc:	lea    0x50(%rsp),%r14
  4669c1:	lea    0x840(%rax,%r15,1),%rbx
  4669c9:	lea    0x190(%rsp),%r15
  4669d1:	mov    %r13,%rbp
  4669d4:	mov    %r14,%rdi
  4669d7:	mov    %r15,%rsi
  4669da:	vzeroupper
  4669dd:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  4669e2:	vmovups 0x50(%rsp),%zmm0
  4669ed:	vmovups 0x90(%rsp),%zmm1
  4669f8:	vmovups 0xd0(%rsp),%zmm2
  466a03:	dec    %rbp
  466a06:	vmovups %zmm2,0x210(%rsp)
  466a11:	vmovups %zmm1,0x1d0(%rsp)
  466a1c:	vmovups %zmm0,0x190(%rsp)
  466a27:	jne    4669d4 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x12d4>
  466a29:	add    $0xffffffffffffff40,%rbx
  466a30:	mov    %r14,%rdi
  466a33:	mov    %r15,%rsi
  466a36:	mov    %rbx,%rdx
  466a39:	vzeroupper
  466a3c:	call   185030 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  466a41:	vmovups 0x50(%rsp),%zmm0
  466a4c:	vmovups 0x90(%rsp),%zmm1
  466a57:	vmovups 0xd0(%rsp),%zmm2
  466a62:	vmovups %zmm2,0x210(%rsp)
  466a6d:	vmovups %zmm1,0x1d0(%rsp)
  466a78:	vmovups %zmm0,0x190(%rsp)
  466a83:	cmp    %rbx,(%rsp)
  466a87:	jne    4669d1 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x12d1>
  466a8d:	jmp    466b0c <zopro_core::msm_affine::g2_phi::raw_exclusive+0x140c>
  466a8f:	mov    (%rsp),%rax
  466a93:	lea    0x50(%rsp),%rbx
  466a98:	lea    0x190(%rsp),%r14
  466aa0:	lea    0x780(%rax),%r13
  466aa7:	lea    0x0(%r13,%r15,1),%rdx
  466aac:	mov    %rbx,%rdi
  466aaf:	mov    %r14,%rsi
  466ab2:	vzeroupper
  466ab5:	call   185030 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  466aba:	vmovups 0x50(%rsp),%zmm0
  466ac5:	vmovups 0x90(%rsp),%zmm1
  466ad0:	vmovups 0xd0(%rsp),%zmm2
  466adb:	add    $0xffffffffffffff40,%r15
  466ae2:	vmovups %zmm2,0x210(%rsp)
  466aed:	vmovups %zmm1,0x1d0(%rsp)
  466af8:	vmovups %zmm0,0x190(%rsp)
  466b03:	cmp    $0xfffffffffffff7c0,%r15
  466b0a:	jne    466aa7 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x13a7>
  466b0c:	vmovups 0x190(%rsp),%zmm0
  466b17:	vmovups 0x1d0(%rsp),%zmm1
  466b22:	vmovups 0x210(%rsp),%zmm2
  466b2d:	mov    0x180(%rsp),%rax
  466b35:	vmovups %zmm2,0x80(%rax)
  466b3c:	vmovups %zmm1,0x40(%rax)
  466b43:	vmovups %zmm0,(%rax)
  466b49:	test   %r12,%r12
  466b4c:	je     466ba1 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14a1>
  466b4e:	cmp    $0x2aa,%r12
  466b55:	jbe    466b92 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1492>
  466b57:	shl    $0x6,%r12
  466b5b:	movabs $0x7ffffffffffffff8,%rcx
  466b65:	lea    (%r12,%r12,2),%rax
  466b69:	mov    %rax,%rsi
  466b6c:	or     $0x1,%rsi
  466b70:	cmp    %rcx,%rsi
  466b73:	ja     46721b <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b1b>
  466b79:	mov    (%rsp),%rcx
  466b7d:	cmpb   $0x1,(%rcx,%rax,1)
  466b81:	jne    466b94 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1494>
  466b83:	mov    (%rsp),%rdi
  466b87:	vzeroupper
  466b8a:	call   *0x8b648(%rip)        # 4f21d8 <munmap@GLIBC_2.2.5>
  466b90:	jmp    466ba1 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14a1>
  466b92:	jmp    466b94 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1494>
  466b94:	mov    (%rsp),%rdi
  466b98:	vzeroupper
  466b9b:	call   *0x8b63f(%rip)        # 4f21e0 <free@GLIBC_2.2.5>
  466ba1:	add    $0x358,%rsp
  466ba8:	pop    %rbx
  466ba9:	pop    %r12
  466bab:	pop    %r13
  466bad:	pop    %r14
  466baf:	pop    %r15
  466bb1:	pop    %rbp
  466bb2:	vzeroupper
  466bb5:	ret
  466bb6:	mov    (%rsp),%rdx
  466bba:	mov    0x148(%rsp),%rsi
  466bc2:	mov    0x168(%rsp),%rcx
  466bca:	lea    0x150(%rsp),%rax
  466bd2:	movq   $0x6,0x38(%rsp)
  466bdb:	mov    %rax,0x50(%rsp)
  466be0:	mov    %fs:0xffffffffffffffc0,%rax
  466be9:	mov    %rdx,0x58(%rsp)
  466bee:	mov    %rbx,0x60(%rsp)
  466bf3:	mov    %rsi,0x68(%rsp)
  466bf8:	lea    (%r14,%r14,2),%rsi
  466bfc:	lea    0x190(%rsp),%rdx
  466c04:	mov    %rbx,0x70(%rsp)
  466c09:	mov    %rcx,(%rsp)
  466c0d:	mov    %rdx,0x78(%rsp)
  466c12:	lea    0x50(%rsp),%rdx
  466c17:	movq   $0x0,0x80(%rsp)
  466c23:	movq   $0x6,0x88(%rsp)
  466c2f:	shl    $0x6,%rsi
  466c33:	mov    %rdx,0x20(%rsp)
  466c38:	add    %rcx,%rsi
  466c3b:	mov    %rsi,0x28(%rsp)
  466c40:	movq   $0x6,0x30(%rsp)
  466c49:	test   %rax,%rax
  466c4c:	je     466c56 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1556>
  466c4e:	add    $0x110,%rax
  466c54:	jmp    466c65 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1565>
  466c56:	mov    0x18(%rsp),%rbp
  466c5b:	mov    0x10(%rsp),%r12
  466c60:	call   1052d0 <rayon_core::registry::global_registry>
  466c65:	mov    (%rax),%rax
  466c68:	mov    0x208(%rax),%rcx
  466c6f:	lea    0x20(%rsp),%rax
  466c74:	xor    %edx,%edx
  466c76:	lea    0x118(%rsp),%rdi
  466c7e:	mov    $0x6,%esi
  466c83:	mov    $0x1,%r8d
  466c89:	xor    %r9d,%r9d
  466c8c:	push   %rax
  466c8d:	push   $0x6
  466c8f:	mov    0x28(%rsp),%rbp
  466c94:	mov    0x20(%rsp),%r12
  466c99:	call   467ab0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>>
  466c9e:	add    $0x10,%rsp
  466ca2:	mov    0x128(%rsp),%rax
  466caa:	mov    %rax,0x20(%rsp)
  466caf:	cmp    $0x6,%rax
  466cb3:	jne    4671cf <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1acf>
  466cb9:	vmovups -0x4477f3(%rip),%zmm0        # 1f4d0 <zmij::STATIC_DATA+0x4fd0>
  466cc3:	vmovups -0x4477bd(%rip),%zmm2        # 1f510 <zmij::STATIC_DATA+0x5010>
  466ccd:	vmovups -0x447787(%rip),%zmm1        # 1f550 <zmij::STATIC_DATA+0x5050>
  466cd7:	mov    (%rsp),%rcx
  466cdb:	shl    $0x6,%r14
  466cdf:	lea    0x50(%rsp),%rbp
  466ce4:	lea    0x290(%rsp),%r15
  466cec:	lea    (%r14,%r14,2),%rax
  466cf0:	mov    $0xfffffffffffffb80,%r14
  466cf7:	sub    %rax,%r14
  466cfa:	lea    0x3c0(%rcx,%rax,1),%r12
  466d02:	vmovups %zmm0,0x290(%rsp)
  466d0d:	vmovups %zmm2,0x2d0(%rsp)
  466d18:	vmovups %zmm1,0x310(%rsp)
  466d23:	mov    %rbp,%rdi
  466d26:	mov    %r15,%rsi
  466d29:	vzeroupper
  466d2c:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466d31:	vmovups 0x50(%rsp),%zmm0
  466d3c:	vmovups 0x90(%rsp),%zmm1
  466d47:	vmovups 0xd0(%rsp),%zmm2
  466d52:	vmovups %zmm2,0x310(%rsp)
  466d5d:	vmovups %zmm1,0x2d0(%rsp)
  466d68:	vmovups %zmm0,0x290(%rsp)
  466d73:	mov    %rbp,%rdi
  466d76:	mov    %r15,%rsi
  466d79:	vzeroupper
  466d7c:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466d81:	vmovups 0x50(%rsp),%zmm0
  466d8c:	vmovups 0x90(%rsp),%zmm1
  466d97:	vmovups 0xd0(%rsp),%zmm2
  466da2:	vmovups %zmm2,0x310(%rsp)
  466dad:	vmovups %zmm1,0x2d0(%rsp)
  466db8:	vmovups %zmm0,0x290(%rsp)
  466dc3:	mov    %rbp,%rdi
  466dc6:	mov    %r15,%rsi
  466dc9:	vzeroupper
  466dcc:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466dd1:	vmovups 0x50(%rsp),%zmm0
  466ddc:	vmovups 0x90(%rsp),%zmm1
  466de7:	vmovups 0xd0(%rsp),%zmm2
  466df2:	vmovups %zmm2,0x310(%rsp)
  466dfd:	vmovups %zmm1,0x2d0(%rsp)
  466e08:	vmovups %zmm0,0x290(%rsp)
  466e13:	mov    %rbp,%rdi
  466e16:	mov    %r15,%rsi
  466e19:	vzeroupper
  466e1c:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466e21:	vmovups 0x50(%rsp),%zmm0
  466e2c:	vmovups 0x90(%rsp),%zmm1
  466e37:	vmovups 0xd0(%rsp),%zmm2
  466e42:	vmovups %zmm2,0x310(%rsp)
  466e4d:	vmovups %zmm1,0x2d0(%rsp)
  466e58:	vmovups %zmm0,0x290(%rsp)
  466e63:	mov    %rbp,%rdi
  466e66:	mov    %r15,%rsi
  466e69:	vzeroupper
  466e6c:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466e71:	vmovups 0x50(%rsp),%zmm0
  466e7c:	vmovups 0x90(%rsp),%zmm1
  466e87:	vmovups 0xd0(%rsp),%zmm2
  466e92:	vmovups %zmm2,0x310(%rsp)
  466e9d:	vmovups %zmm1,0x2d0(%rsp)
  466ea8:	vmovups %zmm0,0x290(%rsp)
  466eb3:	mov    %rbp,%rdi
  466eb6:	mov    %r15,%rsi
  466eb9:	vzeroupper
  466ebc:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466ec1:	vmovups 0x50(%rsp),%zmm0
  466ecc:	vmovups 0x90(%rsp),%zmm1
  466ed7:	vmovups 0xd0(%rsp),%zmm2
  466ee2:	vmovups %zmm2,0x310(%rsp)
  466eed:	vmovups %zmm1,0x2d0(%rsp)
  466ef8:	vmovups %zmm0,0x290(%rsp)
  466f03:	mov    %rbp,%rdi
  466f06:	mov    %r15,%rsi
  466f09:	vzeroupper
  466f0c:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466f11:	vmovups 0x50(%rsp),%zmm0
  466f1c:	vmovups 0x90(%rsp),%zmm1
  466f27:	vmovups 0xd0(%rsp),%zmm2
  466f32:	vmovups %zmm2,0x310(%rsp)
  466f3d:	vmovups %zmm1,0x2d0(%rsp)
  466f48:	vmovups %zmm0,0x290(%rsp)
  466f53:	mov    %rbp,%rdi
  466f56:	mov    %r15,%rsi
  466f59:	vzeroupper
  466f5c:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466f61:	vmovups 0x50(%rsp),%zmm0
  466f6c:	vmovups 0x90(%rsp),%zmm1
  466f77:	vmovups 0xd0(%rsp),%zmm2
  466f82:	vmovups %zmm2,0x310(%rsp)
  466f8d:	vmovups %zmm1,0x2d0(%rsp)
  466f98:	vmovups %zmm0,0x290(%rsp)
  466fa3:	mov    %rbp,%rdi
  466fa6:	mov    %r15,%rsi
  466fa9:	vzeroupper
  466fac:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  466fb1:	vmovups 0x50(%rsp),%zmm0
  466fbc:	vmovups 0x90(%rsp),%zmm1
  466fc7:	vmovups 0xd0(%rsp),%zmm2
  466fd2:	vmovups %zmm2,0x310(%rsp)
  466fdd:	vmovups %zmm1,0x2d0(%rsp)
  466fe8:	vmovups %zmm0,0x290(%rsp)
  466ff3:	mov    %rbp,%rdi
  466ff6:	mov    %r15,%rsi
  466ff9:	vzeroupper
  466ffc:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467001:	vmovups 0x50(%rsp),%zmm0
  46700c:	vmovups 0x90(%rsp),%zmm1
  467017:	vmovups 0xd0(%rsp),%zmm2
  467022:	vmovups %zmm2,0x310(%rsp)
  46702d:	vmovups %zmm1,0x2d0(%rsp)
  467038:	vmovups %zmm0,0x290(%rsp)
  467043:	mov    %rbp,%rdi
  467046:	mov    %r15,%rsi
  467049:	vzeroupper
  46704c:	call   191600 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::double>
  467051:	vmovups 0x50(%rsp),%zmm0
  46705c:	vmovups 0x90(%rsp),%zmm1
  467067:	vmovups 0xd0(%rsp),%zmm2
  467072:	vmovups %zmm2,0x310(%rsp)
  46707d:	vmovups %zmm1,0x2d0(%rsp)
  467088:	vmovups %zmm0,0x290(%rsp)
  467093:	mov    %rbp,%rdi
  467096:	mov    %r15,%rsi
  467099:	mov    %r12,%rdx
  46709c:	vzeroupper
  46709f:	call   185030 <<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>::add_projective>
  4670a4:	vmovups 0x50(%rsp),%zmm0
  4670af:	vmovups 0x90(%rsp),%zmm1
  4670ba:	vmovups 0xd0(%rsp),%zmm2
  4670c5:	add    $0xffffffffffffff40,%r12
  4670cc:	add    $0xc0,%r14
  4670d3:	vmovups %zmm2,0x310(%rsp)
  4670de:	vmovups %zmm1,0x2d0(%rsp)
  4670e9:	vmovups %zmm0,0x290(%rsp)
  4670f4:	jne    466d23 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1623>
  4670fa:	test   %r13,%r13
  4670fd:	je     467119 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1a19>
  4670ff:	shl    $0x6,%r13
  467103:	lea    0x0(%r13,%r13,2),%rsi
  467108:	mov    (%rsp),%rdi
  46710c:	mov    $0x8,%edx
  467111:	vzeroupper
  467114:	call   e99d0 <__rustc::__rust_dealloc>
  467119:	shl    $0x3,%rbx
  46711d:	lea    (%rbx,%rbx,4),%rsi
  467121:	mov    0x148(%rsp),%rdi
  467129:	mov    $0x8,%edx
  46712e:	vzeroupper
  467131:	call   e99d0 <__rustc::__rust_dealloc>
  467136:	mov    0x10(%rsp),%rax
  46713b:	lock sub %rax,0x8ce7d(%rip)        # 4f3fc0 <zopro_core::preparation::RESERVED>
  467143:	mov    0x18(%rsp),%rax
  467148:	lock sub %rax,0x8ce78(%rip)        # 4f3fc8 <zopro_core::preparation::WORKSPACES>
  467150:	vmovups 0x290(%rsp),%zmm0
  46715b:	vmovups 0x2d0(%rsp),%zmm1
  467166:	vmovups 0x310(%rsp),%zmm2
  467171:	mov    0x180(%rsp),%rax
  467179:	vmovups %zmm2,0x80(%rax)
  467180:	vmovups %zmm1,0x40(%rax)
  467187:	vmovups %zmm0,(%rax)
  46718d:	jmp    466ba1 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x14a1>
  467192:	lea    0x160(%rsp),%rax
  46719a:	lea    -0x3736a1(%rip),%rcx        # f3b00 <<u64 as core::fmt::Display>::fmt>
  4671a1:	mov    %rax,0x50(%rsp)
  4671a6:	mov    %rcx,0x58(%rsp)
  4671ab:	mov    %r13,0x60(%rsp)
  4671b0:	mov    %rcx,0x68(%rsp)
  4671b5:	lea    -0x4503bd(%rip),%rdi        # 16dff <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x533f>
  4671bc:	lea    0x865a5(%rip),%rdx        # 4ed768 <__frame_dummy_init_array_entry+0x42b8>
  4671c3:	lea    0x50(%rsp),%rsi
  4671c8:	call   ef4b0 <core::panicking::panic_fmt>
  4671cd:	jmp    467219 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b19>
  4671cf:	lea    0x158(%rsp),%rax
  4671d7:	lea    -0x3736de(%rip),%r8        # f3b00 <<u64 as core::fmt::Display>::fmt>
  4671de:	lea    0x20(%rsp),%rcx
  4671e3:	mov    %rax,0x50(%rsp)
  4671e8:	mov    %r8,0x58(%rsp)
  4671ed:	mov    %rcx,0x60(%rsp)
  4671f2:	mov    %r8,0x68(%rsp)
  4671f7:	mov    0x18(%rsp),%rbp
  4671fc:	mov    0x10(%rsp),%r12
  467201:	lea    -0x450409(%rip),%rdi        # 16dff <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x533f>
  467208:	lea    0x86559(%rip),%rdx        # 4ed768 <__frame_dummy_init_array_entry+0x42b8>
  46720f:	lea    0x50(%rsp),%rsi
  467214:	call   ef4b0 <core::panicking::panic_fmt>
  467219:	ud2
  46721b:	vzeroupper
  46721e:	call   111210 <std::process::abort>
  467223:	mov    0x10(%rsp),%r12
  467228:	mov    %rax,%r15
  46722b:	jmp    467257 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b57>
  46722d:	mov    %rax,%r15
  467230:	mov    0x160(%rsp),%rax
  467238:	test   %rax,%rax
  46723b:	je     467257 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1b57>
  46723d:	mov    0x168(%rsp),%rdi
  467245:	shl    $0x6,%rax
  467249:	lea    (%rax,%rax,2),%rsi
  46724d:	mov    $0x8,%edx
  467252:	call   e99d0 <__rustc::__rust_dealloc>
  467257:	shl    $0x3,%rbx
  46725b:	lea    (%rbx,%rbx,4),%rsi
  46725f:	mov    0x148(%rsp),%rdi
  467267:	mov    $0x8,%edx
  46726c:	call   e99d0 <__rustc::__rust_dealloc>
  467271:	lock sub %r12,0x8cd47(%rip)        # 4f3fc0 <zopro_core::preparation::RESERVED>
  467279:	lock sub %rbp,0x8cd47(%rip)        # 4f3fc8 <zopro_core::preparation::WORKSPACES>
  467281:	mov    %r15,%rdi
  467284:	call   4e8460 <_Unwind_Resume@plt>
  467289:	mov    %rax,%r15
  46728c:	mov    0x118(%rsp),%rax
  467294:	test   %rax,%rax
  467297:	jne    4672a1 <zopro_core::msm_affine::g2_phi::raw_exclusive+0x1ba1>
  467299:	mov    %r15,%rdi
  46729c:	call   4e8460 <_Unwind_Resume@plt>
  4672a1:	mov    0x120(%rsp),%rdi
  4672a9:	shl    $0x6,%rax
  4672ad:	lea    (%rax,%rax,2),%rsi
  4672b1:	mov    $0x8,%edx
  4672b6:	call   e99d0 <__rustc::__rust_dealloc>
  4672bb:	mov    %r15,%rdi
  4672be:	call   4e8460 <_Unwind_Resume@plt>
