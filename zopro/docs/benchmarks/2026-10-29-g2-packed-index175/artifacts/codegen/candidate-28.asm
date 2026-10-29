
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000246760 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>>:
  246760:	push   %rbp
  246761:	push   %r15
  246763:	push   %r14
  246765:	push   %r13
  246767:	push   %r12
  246769:	push   %rbx
  24676a:	sub    $0x1e8,%rsp
  246771:	mov    0x220(%rsp),%r13
  246779:	mov    %rcx,0x118(%rsp)
  246781:	mov    %rdx,0x128(%rsp)
  246789:	mov    %rsi,0x120(%rsp)
  246791:	mov    %rdi,0x110(%rsp)
  246799:	mov    %r8,0x40(%rsp)
  24679e:	test   %r13,%r13
  2467a1:	je     2467d5 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x75>
  2467a3:	mov    %fs:0xffffffffffffffc0,%rax
  2467ac:	lea    -0x1(%r13),%r14
  2467b0:	mov    %r9,%r12
  2467b3:	mov    %r8,%rbx
  2467b6:	mov    %r14,0x180(%rsp)
  2467be:	test   %rax,%rax
  2467c1:	je     24683e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xde>
  2467c3:	add    $0x110,%rax
  2467c9:	xor    %esi,%esi
  2467cb:	test   %r14,%r14
  2467ce:	jne    24684e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xee>
  2467d0:	jmp    24699d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x23d>
  2467d5:	movq   $0x0,0xf0(%rsp)
  2467e1:	movq   $0x8,0xf8(%rsp)
  2467ed:	movq   $0x0,0x70(%rsp)
  2467f6:	movq   $0x8,0x78(%rsp)
  2467ff:	movq   $0x0,0x100(%rsp)
  24680b:	movq   $0x0,0x80(%rsp)
  246817:	lea    0x70(%rsp),%rdi
  24681c:	call   f4890 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  246821:	mov    0x78(%rsp),%rax
  246826:	movq   $0x0,(%rax)
  24682d:	movq   $0x1,0x80(%rsp)
  246839:	jmp    246a9e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x33e>
  24683e:	call   104a30 <rayon_core::registry::global_registry>
  246843:	xor    %esi,%esi
  246845:	test   %r14,%r14
  246848:	je     24699d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x23d>
  24684e:	movabs $0xffffffff00000000,%rcx
  246858:	lea    (%rbx,%rcx,1),%rdx
  24685c:	inc    %rcx
  24685f:	cmp    %rcx,%rdx
  246862:	jb     24699d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x23d>
  246868:	mov    (%rax),%rax
  24686b:	mov    0x208(%rax),%r8
  246872:	test   %r8,%r8
  246875:	je     24697e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x21e>
  24687b:	lea    0x3ff(%rbx),%rax
  246882:	movabs $0x200000000000000,%rdx
  24688c:	shr    $0xa,%rax
  246890:	lea    (%r14,%rax,1),%rcx
  246894:	cmp    %rbx,%rcx
  246897:	cmovae %rbx,%rcx
  24689b:	cmp    %rdx,%r13
  24689e:	ja     24699b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x23b>
  2468a4:	mov    %r14,%rsi
  2468a7:	shl    $0x4,%rsi
  2468ab:	lea    (%rcx,%rcx,4),%r9
  2468af:	mov    %rcx,%rdi
  2468b2:	shl    $0x7,%rdi
  2468b6:	mov    %r14,%rdx
  2468b9:	shl    $0x7,%rdx
  2468bd:	lea    (%rsi,%rax,8),%rax
  2468c1:	or     $0x40,%rdx
  2468c5:	lea    (%rax,%r9,4),%rax
  2468c9:	lea    0x190(%rdi,%rax,1),%r9
  2468d1:	add    %rdx,%r9
  2468d4:	jb     24699b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x23b>
  2468da:	cmp    $0x400,%rbx
  2468e1:	mov    $0x400,%edx
  2468e6:	mov    $0x400,%eax
  2468eb:	cmovb  %rbx,%rdx
  2468ef:	xor    %esi,%esi
  2468f1:	mov    %edx,%r10d
  2468f4:	shr    $0x4,%r10d
  2468f8:	imul   $0xb40,%r10,%r11
  2468ff:	add    $0x40,%r11
  246903:	test   %r10d,%r10d
  246906:	mov    %edx,%r10d
  246909:	cmove  %rsi,%r11
  24690d:	cmp    $0x400,%r14
  246914:	cmovb  %r14,%rax
  246918:	shl    $0x7,%edx
  24691b:	lea    (%rdx,%r10,8),%rdx
  24691f:	shl    $0x4,%eax
  246922:	add    %r11,%rdx
  246925:	lea    0x1f8(%rax,%rdx,1),%rax
  24692d:	mul    %r8
  246930:	jo     24699d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x23d>
  246932:	add    %r9,%rax
  246935:	jb     24699b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x23b>
  246937:	mov    %rcx,%rsi
  24693a:	shr    $0x4,%rsi
  24693e:	lea    0xfe(%r13),%rdx
  246945:	or     $0x40,%rdi
  246949:	imul   $0xb40,%rsi,%r8
  246950:	shr    $0x8,%rdx
  246954:	add    $0x40,%r8
  246958:	test   %rsi,%rsi
  24695b:	cmove  %rsi,%r8
  24695f:	imul   $0xf8,%rdx,%rdx
  246966:	xor    %esi,%esi
  246968:	lea    (%rdx,%rcx,8),%rcx
  24696c:	add    %rdi,%rcx
  24696f:	add    %r8,%rcx
  246972:	lea    0x40(%rax,%rcx,1),%rdx
  246977:	cmp    %rax,%rdx
  24697a:	setae  %sil
  24697e:	lea    0xa0(%rsp),%rdi
  246986:	call   21ba30 <<zopro_core::preparation::Reservation>::analysis>
  24698b:	cmpl   $0x1,0xa0(%rsp)
  246993:	jne    246a34 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2d4>
  246999:	jmp    2469b8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x258>
  24699b:	xor    %esi,%esi
  24699d:	lea    0xa0(%rsp),%rdi
  2469a5:	call   21ba30 <<zopro_core::preparation::Reservation>::analysis>
  2469aa:	cmpl   $0x1,0xa0(%rsp)
  2469b2:	jne    246a34 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2d4>
  2469b8:	mov    0xa8(%rsp),%r13
  2469c0:	mov    0xb0(%rsp),%r15
  2469c8:	cmpq   $0x0,(%r12)
  2469cd:	movq   $0x400,0x188(%rsp)
  2469d9:	jne    246a1c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2bc>
  2469db:	mov    0x220(%rsp),%rax
  2469e3:	cmp    %rbx,-0x8(%r12,%rax,8)
  2469e8:	jne    246a1c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2bc>
  2469ea:	mov    0x220(%rsp),%rdx
  2469f2:	lea    0x8(%r12),%rax
  2469f7:	xor    %ecx,%ecx
  2469f9:	nopl   0x0(%rax)
  246a00:	cmp    $0x2,%rdx
  246a04:	jb     247315 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xbb5>
  246a0a:	mov    (%rax),%rsi
  246a0d:	dec    %rdx
  246a10:	add    $0x8,%rax
  246a14:	cmp    %rsi,%rcx
  246a17:	mov    %rsi,%rcx
  246a1a:	jbe    246a00 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2a0>
  246a1c:	lock sub %r13,0x2a600c(%rip)        # 4eca30 <zopro_core::preparation::RESERVED>
  246a24:	mov    0x220(%rsp),%r13
  246a2c:	lock sub %r15,0x2a6004(%rip)        # 4eca38 <zopro_core::preparation::WORKSPACES>
  246a34:	lea    0x0(,%r13,8),%r14
  246a3c:	movq   $0x0,0xf0(%rsp)
  246a48:	movq   $0x8,0xf8(%rsp)
  246a54:	movq   $0x0,0x100(%rsp)
  246a60:	mov    $0x8,%edi
  246a65:	mov    %r14,%rsi
  246a68:	xor    %edx,%edx
  246a6a:	vzeroupper
  246a6d:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  246a72:	test   %rax,%rax
  246a75:	je     24804d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x18ed>
  246a7b:	mov    %r13,0x70(%rsp)
  246a80:	mov    %rax,0x78(%rsp)
  246a85:	movq   $0x0,(%rax)
  246a8c:	movq   $0x1,0x80(%rsp)
  246a98:	cmp    $0x1,%r13
  246a9c:	jne    246ad1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x371>
  246a9e:	movq   $0x0,0x48(%rsp)
  246aa7:	movq   $0x8,0x50(%rsp)
  246ab0:	movq   $0x0,0x58(%rsp)
  246ab9:	movq   $0x0,0x108(%rsp)
  246ac5:	mov    $0x8,%ebp
  246aca:	xor    %ebx,%ebx
  246acc:	jmp    246c2b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x4cb>
  246ad1:	mov    (%r12),%r15
  246ad5:	mov    $0x8,%ebx
  246ada:	mov    $0x1,%ebp
  246adf:	xor    %r14d,%r14d
  246ae2:	jmp    246b1b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x3bb>
  246ae4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  246af0:	mov    0x78(%rsp),%rax
  246af5:	mov    0x8(%rsp),%r15
  246afa:	add    $0x8,%r12
  246afe:	mov    %r14,(%rax,%rbp,8)
  246b02:	inc    %rbp
  246b05:	mov    %rbp,0x80(%rsp)
  246b0d:	cmp    $0x3,%r13
  246b11:	lea    -0x1(%r13),%r13
  246b15:	jb     246bef <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x48f>
  246b1b:	mov    0x8(%r12),%rax
  246b20:	mov    %rax,0x8(%rsp)
  246b25:	sub    %r15,%rax
  246b28:	jbe    246bd5 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x475>
  246b2e:	mov    %rax,%rbp
  246b31:	and    $0x3ff,%eax
  246b36:	shr    $0xa,%rbp
  246b3a:	mov    %r13,0x10(%rsp)
  246b3f:	mov    %r14,%r13
  246b42:	mov    %r12,0x18(%rsp)
  246b47:	cmp    $0x1,%rax
  246b4b:	mov    %rbx,%rax
  246b4e:	sbb    $0xffffffffffffffff,%rbp
  246b52:	shl    $0x4,%r13
  246b56:	or     $0x8,%r13
  246b5a:	jmp    246b83 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x423>
  246b5c:	nopl   0x0(%rax)
  246b60:	mov    %r15,-0x8(%rax,%r13,1)
  246b65:	mov    %r12,(%rax,%r13,1)
  246b69:	inc    %r14
  246b6c:	dec    %rbp
  246b6f:	add    $0x10,%r13
  246b73:	mov    %rbx,%r15
  246b76:	mov    %r14,0x100(%rsp)
  246b7e:	test   %rbp,%rbp
  246b81:	je     246bc0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x460>
  246b83:	mov    0x8(%rsp),%rcx
  246b88:	lea    0x400(%r15),%rbx
  246b8f:	mov    %rbx,%r12
  246b92:	cmp    %rbx,%rcx
  246b95:	cmovb  %rcx,%r12
  246b99:	cmp    0xf0(%rsp),%r14
  246ba1:	jne    246b60 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x400>
  246ba3:	lea    0xf0(%rsp),%rdi
  246bab:	call   2b7f10 <<alloc::raw_vec::RawVec<zopro_core::preparation::Reservation>>::grow_one>
  246bb0:	mov    0xf8(%rsp),%rax
  246bb8:	jmp    246b60 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x400>
  246bba:	nopw   0x0(%rax,%rax,1)
  246bc0:	mov    0x80(%rsp),%rbp
  246bc8:	mov    0x18(%rsp),%r12
  246bcd:	mov    0x10(%rsp),%r13
  246bd2:	mov    %rax,%rbx
  246bd5:	cmp    0x70(%rsp),%rbp
  246bda:	jne    246af0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x390>
  246be0:	lea    0x70(%rsp),%rdi
  246be5:	call   f4890 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  246bea:	jmp    246af0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x390>
  246bef:	mov    0xf0(%rsp),%rbx
  246bf7:	mov    0xf8(%rsp),%rbp
  246bff:	movq   $0x0,0x48(%rsp)
  246c08:	movq   $0x8,0x50(%rsp)
  246c11:	movq   $0x0,0x58(%rsp)
  246c1a:	mov    %r14,0x108(%rsp)
  246c22:	test   %r14,%r14
  246c25:	jne    24744c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xcec>
  246c2b:	mov    $0x8,%r12d
  246c31:	mov    $0x1,%r15b
  246c34:	xor    %r14d,%r14d
  246c37:	xor    %r13d,%r13d
  246c3a:	xor    %eax,%eax
  246c3c:	mov    0x118(%rsp),%rdx
  246c44:	mov    0x40(%rsp),%rsi
  246c49:	mov    %rbx,0xa0(%rsp)
  246c51:	mov    %rbp,0xa8(%rsp)
  246c59:	mov    %r14,0xb0(%rsp)
  246c61:	mov    0x120(%rsp),%rdi
  246c69:	mov    %rax,0x10(%rsp)
  246c6e:	shl    $0x7,%rax
  246c72:	lea    0xb8(%rsp),%rcx
  246c7a:	mov    %r14,0x1b8(%rsp)
  246c82:	mov    %rbx,0x20(%rsp)
  246c87:	mov    %rbp,0x28(%rsp)
  246c8c:	movq   $0x0,0x30(%rsp)
  246c95:	add    %r12,%rax
  246c98:	mov    %rcx,0x1a0(%rsp)
  246ca0:	mov    %rax,0x1a8(%rsp)
  246ca8:	lea    0x20(%rsp),%rax
  246cad:	mov    %r14,0x1b0(%rsp)
  246cb5:	mov    %rax,0x130(%rsp)
  246cbd:	movq   $0x0,0x138(%rsp)
  246cc9:	mov    %r14,0x140(%rsp)
  246cd1:	mov    %r14,0x148(%rsp)
  246cd9:	mov    %rdx,0xb8(%rsp)
  246ce1:	mov    %rsi,0xc0(%rsp)
  246ce9:	mov    0x128(%rsp),%rsi
  246cf1:	mov    %rdi,0xc8(%rsp)
  246cf9:	mov    %rsi,0xd0(%rsp)
  246d01:	cmp    %r14,%rbx
  246d04:	jb     24749c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xd3c>
  246d0a:	mov    %fs:0xffffffffffffffc0,%rax
  246d13:	test   %rax,%rax
  246d16:	je     246d20 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x5c0>
  246d18:	add    $0x110,%rax
  246d1e:	jmp    246d25 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x5c5>
  246d20:	call   104a30 <rayon_core::registry::global_registry>
  246d25:	mov    (%rax),%rax
  246d28:	mov    0x208(%rax),%rcx
  246d2f:	lea    0x1a0(%rsp),%rax
  246d37:	xor    %edx,%edx
  246d39:	lea    0x88(%rsp),%rdi
  246d41:	mov    $0x1,%r8d
  246d47:	mov    %r14,%rsi
  246d4a:	mov    %rbp,%r9
  246d4d:	push   %rax
  246d4e:	push   %r14
  246d50:	call   28f6f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>
  246d55:	add    $0x10,%rsp
  246d59:	cmp    %r14,0x30(%rsp)
  246d5e:	sete   %al
  246d61:	or     %al,%r15b
  246d64:	je     246d6f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x60f>
  246d66:	movq   $0x0,0x30(%rsp)
  246d6f:	mov    0x20(%rsp),%rax
  246d74:	mov    %r13,0x18(%rsp)
  246d79:	test   %rax,%rax
  246d7c:	je     246dbf <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x65f>
  246d7e:	mov    0x28(%rsp),%rdi
  246d83:	cmp    $0x1fff,%rax
  246d89:	jbe    246db7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x657>
  246d8b:	shl    $0x4,%rax
  246d8f:	movabs $0x7ffffffffffffff9,%rcx
  246d99:	mov    %rax,%rsi
  246d9c:	or     $0x1,%rsi
  246da0:	cmp    %rcx,%rsi
  246da3:	jae    248067 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1907>
  246da9:	cmpb   $0x1,(%rdi,%rax,1)
  246dad:	jne    246db9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x659>
  246daf:	call   *0x2a3e93(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  246db5:	jmp    246dbf <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x65f>
  246db7:	jmp    246db9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x659>
  246db9:	call   *0x2a3e91(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  246dbf:	mov    0x98(%rsp),%rax
  246dc7:	mov    %rax,0x130(%rsp)
  246dcf:	cmp    %r14,%rax
  246dd2:	jne    2474b9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xd59>
  246dd8:	mov    0x80(%rsp),%r15
  246de0:	xor    %ebp,%ebp
  246de2:	movabs $0x7ffffffffffffff9,%rcx
  246dec:	mov    %r12,0x8(%rsp)
  246df1:	dec    %rcx
  246df4:	lea    -0x1(%r15),%r10
  246df8:	cmp    $0x2,%r15
  246dfc:	mov    %r10,%r8
  246dff:	cmovb  %rbp,%r8
  246e03:	lea    0x0(,%r8,8),%rbx
  246e0b:	mov    %r8,%rax
  246e0e:	shr    $0x3d,%rax
  246e12:	setne  %al
  246e15:	cmp    %rcx,%rbx
  246e18:	seta   %cl
  246e1b:	or     %al,%cl
  246e1d:	je     246e2f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x6cf>
  246e1f:	mov    %rbp,%rdi
  246e22:	mov    %rbx,%rsi
  246e25:	call   ede21 <alloc::raw_vec::handle_error>
  246e2a:	jmp    24820b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1aab>
  246e2f:	mov    0x78(%rsp),%r13
  246e34:	test   %rbx,%rbx
  246e37:	je     246e61 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x701>
  246e39:	mov    $0x8,%edi
  246e3e:	mov    $0x8,%ebp
  246e43:	mov    %rbx,%rsi
  246e46:	xor    %edx,%edx
  246e48:	mov    %r8,%r12
  246e4b:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  246e50:	lea    -0x1(%r15),%r10
  246e54:	mov    %r12,%rcx
  246e57:	mov    %rax,%r12
  246e5a:	test   %rax,%rax
  246e5d:	jne    246e69 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x709>
  246e5f:	jmp    246e1f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x6bf>
  246e61:	mov    $0x8,%r12d
  246e67:	xor    %ecx,%ecx
  246e69:	mov    0x10(%rsp),%rdx
  246e6e:	mov    %rcx,0x40(%rsp)
  246e73:	add    %r14,%rdx
  246e76:	cmp    $0x2,%r15
  246e7a:	jae    246e9c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x73c>
  246e7c:	xor    %r10d,%r10d
  246e7f:	mov    %edx,%ebx
  246e81:	lea    0x0(,%rbx,4),%r13
  246e89:	test   %rbx,%rbx
  246e8c:	jne    24707b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x91b>
  246e92:	mov    $0x4,%ebp
  246e97:	jmp    24719f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa3f>
  246e9c:	mov    0x0(%r13),%rcx
  246ea0:	mov    %rdx,%r9
  246ea3:	cmp    $0x21,%r15
  246ea7:	jb     246ebf <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x75f>
  246ea9:	mov    %r12,%rax
  246eac:	sub    %r13,%rax
  246eaf:	add    $0xfffffffffffffff8,%rax
  246eb3:	cmp    $0x100,%rax
  246eb9:	jae    246fb1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x851>
  246ebf:	xor    %eax,%eax
  246ec1:	mov    %r15,%rdx
  246ec4:	sub    %rax,%rdx
  246ec7:	mov    %r15d,%esi
  246eca:	and    $0x7,%esi
  246ecd:	add    $0xfffffffffffffffe,%rdx
  246ed1:	cmp    $0x1,%esi
  246ed4:	je     246efd <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x79d>
  246ed6:	lea    -0x1(%r15),%esi
  246eda:	and    $0x7,%esi
  246edd:	nopl   (%rax)
  246ee0:	mov    %rcx,%rdi
  246ee3:	mov    0x8(%r13),%rcx
  246ee7:	add    $0x8,%r13
  246eeb:	mov    %rcx,%r8
  246eee:	sub    %rdi,%r8
  246ef1:	mov    %r8,(%r12,%rax,8)
  246ef5:	inc    %rax
  246ef8:	dec    %rsi
  246efb:	jne    246ee0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x780>
  246efd:	cmp    $0x7,%rdx
  246f01:	jb     247065 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x905>
  246f07:	lea    0x38(%r12,%rax,8),%rdx
  246f0c:	not    %rax
  246f0f:	xor    %esi,%esi
  246f11:	add    %r15,%rax
  246f14:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  246f20:	mov    0x8(%r13,%rsi,8),%rdi
  246f25:	mov    %rdi,%r8
  246f28:	sub    %rcx,%r8
  246f2b:	mov    %r8,-0x38(%rdx,%rsi,8)
  246f30:	mov    0x10(%r13,%rsi,8),%rcx
  246f35:	mov    %rcx,%r8
  246f38:	sub    %rdi,%r8
  246f3b:	mov    %r8,-0x30(%rdx,%rsi,8)
  246f40:	mov    0x18(%r13,%rsi,8),%rdi
  246f45:	mov    %rdi,%r8
  246f48:	sub    %rcx,%r8
  246f4b:	mov    %r8,-0x28(%rdx,%rsi,8)
  246f50:	mov    0x20(%r13,%rsi,8),%rcx
  246f55:	mov    %rcx,%r8
  246f58:	sub    %rdi,%r8
  246f5b:	mov    %r8,-0x20(%rdx,%rsi,8)
  246f60:	mov    0x28(%r13,%rsi,8),%rdi
  246f65:	mov    %rdi,%r8
  246f68:	sub    %rcx,%r8
  246f6b:	mov    %r8,-0x18(%rdx,%rsi,8)
  246f70:	mov    0x30(%r13,%rsi,8),%rcx
  246f75:	mov    %rcx,%r8
  246f78:	sub    %rdi,%r8
  246f7b:	mov    %r8,-0x10(%rdx,%rsi,8)
  246f80:	mov    0x38(%r13,%rsi,8),%rdi
  246f85:	mov    %rdi,%r8
  246f88:	sub    %rcx,%r8
  246f8b:	mov    %r8,-0x8(%rdx,%rsi,8)
  246f90:	mov    0x40(%r13,%rsi,8),%rcx
  246f95:	mov    %rcx,%r8
  246f98:	sub    %rdi,%r8
  246f9b:	mov    %r8,(%rdx,%rsi,8)
  246f9f:	add    $0x8,%rsi
  246fa3:	cmp    %rsi,%rax
  246fa6:	jne    246f20 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x7c0>
  246fac:	jmp    247065 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x905>
  246fb1:	mov    %r10,%rax
  246fb4:	and    $0xffffffffffffffe0,%rax
  246fb8:	vpbroadcastq %rcx,%zmm0
  246fbe:	xor    %ecx,%ecx
  246fc0:	lea    0x0(%r13,%rax,8),%rdx
  246fc5:	data16 cs nopw 0x0(%rax,%rax,1)
  246fd0:	vmovdqu64 0x8(%r13,%rcx,8),%zmm1
  246fdb:	vmovdqu64 0x48(%r13,%rcx,8),%zmm2
  246fe6:	vmovdqu64 0x88(%r13,%rcx,8),%zmm3
  246ff1:	valignq $0x7,%zmm0,%zmm1,%zmm4
  246ff8:	vmovdqu64 0xc8(%r13,%rcx,8),%zmm0
  247003:	valignq $0x7,%zmm1,%zmm2,%zmm5
  24700a:	valignq $0x7,%zmm2,%zmm3,%zmm6
  247011:	vpsubq %zmm4,%zmm1,%zmm1
  247017:	vpsubq %zmm5,%zmm2,%zmm2
  24701d:	vmovdqu64 %zmm1,(%r12,%rcx,8)
  247024:	vmovdqu64 %zmm2,0x40(%r12,%rcx,8)
  24702c:	valignq $0x7,%zmm3,%zmm0,%zmm7
  247033:	vpsubq %zmm6,%zmm3,%zmm3
  247039:	vmovdqu64 %zmm3,0x80(%r12,%rcx,8)
  247041:	vpsubq %zmm7,%zmm0,%zmm4
  247047:	vmovdqu64 %zmm4,0xc0(%r12,%rcx,8)
  24704f:	add    $0x20,%rcx
  247053:	cmp    %rcx,%rax
  247056:	jne    246fd0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x870>
  24705c:	cmp    %rax,%r10
  24705f:	jne    247300 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xba0>
  247065:	mov    %r9,%rdx
  247068:	mov    %edx,%ebx
  24706a:	lea    0x0(,%rbx,4),%r13
  247072:	test   %rbx,%rbx
  247075:	je     246e92 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x732>
  24707b:	mov    %r10,%r15
  24707e:	mov    %rdx,%r14
  247081:	mov    $0x4,%edi
  247086:	mov    %r13,%rsi
  247089:	xor    %edx,%edx
  24708b:	vzeroupper
  24708e:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  247093:	test   %rax,%rax
  247096:	je     24806c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x190c>
  24709c:	mov    %rax,%rbp
  24709f:	mov    %r15,%r10
  2470a2:	cmp    $0x8,%ebx
  2470a5:	jae    2470ae <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x94e>
  2470a7:	xor    %eax,%eax
  2470a9:	jmp    247190 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa30>
  2470ae:	cmp    $0x40,%ebx
  2470b1:	jae    2470ba <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x95a>
  2470b3:	xor    %eax,%eax
  2470b5:	jmp    24714b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x9eb>
  2470ba:	vmovdqa64 -0x23a3c4(%rip),%zmm0        # cd00 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x11c0>
  2470c4:	vpbroadcastd -0x23b24e(%rip),%zmm1        # be80 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x340>
  2470ce:	vpbroadcastd -0x23b234(%rip),%zmm2        # bea4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x364>
  2470d8:	vpbroadcastd -0x23b22a(%rip),%zmm3        # beb8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x378>
  2470e2:	vpbroadcastd -0x23b228(%rip),%zmm4        # bec4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x384>
  2470ec:	mov    %r14d,%eax
  2470ef:	and    $0xffffffc0,%eax
  2470f2:	xor    %ecx,%ecx
  2470f4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  247100:	vpaddd %zmm1,%zmm0,%zmm5
  247106:	vpaddd %zmm2,%zmm0,%zmm6
  24710c:	vpaddd %zmm3,%zmm0,%zmm7
  247112:	vmovdqu64 %zmm0,0x0(%rbp,%rcx,4)
  24711a:	vpaddd %zmm4,%zmm0,%zmm0
  247120:	vmovdqu64 %zmm5,0x40(%rbp,%rcx,4)
  247128:	vmovdqu64 %zmm6,0x80(%rbp,%rcx,4)
  247130:	vmovdqu64 %zmm7,0xc0(%rbp,%rcx,4)
  247138:	add    $0x40,%rcx
  24713c:	cmp    %rcx,%rax
  24713f:	jne    247100 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x9a0>
  247141:	cmp    %eax,%ebx
  247143:	je     24719c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa3c>
  247145:	test   $0x38,%r14b
  247149:	je     247190 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa30>
  24714b:	mov    %rax,%rcx
  24714e:	vpbroadcastd %ecx,%ymm0
  247154:	vpor   -0x23581c(%rip),%ymm0,%ymm0        # 11940 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5e00>
  24715c:	vpbroadcastd -0x23b2bd(%rip),%ymm1        # bea8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x368>
  247165:	mov    %r14d,%eax
  247168:	and    $0xfffffff8,%eax
  24716b:	nopl   0x0(%rax,%rax,1)
  247170:	vmovdqu %ymm0,0x0(%rbp,%rcx,4)
  247176:	add    $0x8,%rcx
  24717a:	vpaddd %ymm1,%ymm0,%ymm0
  24717e:	cmp    %rcx,%rax
  247181:	jne    247170 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa10>
  247183:	cmp    %eax,%ebx
  247185:	je     24719c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa3c>
  247187:	nopw   0x0(%rax,%rax,1)
  247190:	mov    %eax,0x0(%rbp,%rax,4)
  247194:	inc    %rax
  247197:	cmp    %rax,%rbx
  24719a:	jne    247190 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa30>
  24719c:	mov    %r14,%rdx
  24719f:	mov    0x78(%rsp),%r9
  2471a4:	sub    $0x8,%rsp
  2471a8:	mov    0x118(%rsp),%rdi
  2471b0:	mov    0x10(%rsp),%rsi
  2471b5:	mov    %rbp,%rcx
  2471b8:	mov    %rbx,%r8
  2471bb:	push   %r10
  2471bd:	push   %r12
  2471bf:	push   0x98(%rsp)
  2471c6:	vzeroupper
  2471c9:	call   22b9f0 <zopro_core::msm::group_affine::<zopro_bn254::curve::G2Config>>
  2471ce:	add    $0x20,%rsp
  2471d2:	test   %rbx,%rbx
  2471d5:	je     247204 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaa4>
  2471d7:	cmp    $0x7fff,%ebx
  2471dd:	jbe    2471f9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa99>
  2471df:	cmpb   $0x1,0x0(%rbp,%r13,1)
  2471e5:	jne    2471fb <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa9b>
  2471e7:	or     $0x1,%r13
  2471eb:	mov    %rbp,%rdi
  2471ee:	mov    %r13,%rsi
  2471f1:	call   *0x2a3a51(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  2471f7:	jmp    247204 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaa4>
  2471f9:	jmp    2471fb <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa9b>
  2471fb:	mov    %rbp,%rdi
  2471fe:	call   *0x2a3a4c(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247204:	mov    0x40(%rsp),%rcx
  247209:	mov    0x18(%rsp),%rbx
  24720e:	test   %rcx,%rcx
  247211:	je     247257 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaf7>
  247213:	cmp    $0x3fff,%rcx
  24721a:	jbe    24724c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaec>
  24721c:	shl    $0x3,%rcx
  247220:	movabs $0x7ffffffffffffff9,%rax
  24722a:	mov    %rcx,%rsi
  24722d:	or     $0x1,%rsi
  247231:	cmp    %rax,%rsi
  247234:	jae    248167 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1a07>
  24723a:	cmpb   $0x1,(%r12,%rcx,1)
  24723f:	jne    24724e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaee>
  247241:	mov    %r12,%rdi
  247244:	call   *0x2a39fe(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  24724a:	jmp    247257 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaf7>
  24724c:	jmp    24724e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaee>
  24724e:	mov    %r12,%rdi
  247251:	call   *0x2a39f9(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247257:	test   %rbx,%rbx
  24725a:	je     2472a3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb43>
  24725c:	cmp    $0x3ff,%rbx
  247263:	jbe    247296 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb36>
  247265:	mov    0x8(%rsp),%rdi
  24726a:	shl    $0x7,%rbx
  24726e:	movabs $0x7ffffffffffffff9,%rax
  247278:	mov    %rbx,%rsi
  24727b:	or     $0x1,%rsi
  24727f:	cmp    %rax,%rsi
  247282:	jae    24816c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1a0c>
  247288:	cmpb   $0x1,(%rdi,%rbx,1)
  24728c:	jne    24729d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb3d>
  24728e:	call   *0x2a39b4(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  247294:	jmp    2472a3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb43>
  247296:	mov    0x8(%rsp),%rdi
  24729b:	jmp    24729d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb3d>
  24729d:	call   *0x2a39ad(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  2472a3:	mov    0x70(%rsp),%rax
  2472a8:	test   %rax,%rax
  2472ab:	je     2472ee <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb8e>
  2472ad:	mov    0x78(%rsp),%rdi
  2472b2:	cmp    $0x3fff,%rax
  2472b8:	jbe    2472e6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb86>
  2472ba:	shl    $0x3,%rax
  2472be:	movabs $0x7ffffffffffffff9,%rcx
  2472c8:	mov    %rax,%rsi
  2472cb:	or     $0x1,%rsi
  2472cf:	cmp    %rcx,%rsi
  2472d2:	jae    248171 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1a11>
  2472d8:	cmpb   $0x1,(%rdi,%rax,1)
  2472dc:	jne    2472e8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb88>
  2472de:	call   *0x2a3964(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  2472e4:	jmp    2472ee <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb8e>
  2472e6:	jmp    2472e8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb88>
  2472e8:	call   *0x2a3962(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  2472ee:	add    $0x1e8,%rsp
  2472f5:	pop    %rbx
  2472f6:	pop    %r12
  2472f8:	pop    %r13
  2472fa:	pop    %r14
  2472fc:	pop    %r15
  2472fe:	pop    %rbp
  2472ff:	ret
  247300:	vextracti32x4 $0x3,%zmm0,%xmm0
  247307:	mov    %rdx,%r13
  24730a:	vpextrq $0x1,%xmm0,%rcx
  247310:	jmp    246ec1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x761>
  247315:	lea    0x3ff(%rbx),%rbp
  24731c:	mov    $0x8,%eax
  247321:	movq   $0x0,0x48(%rsp)
  24732a:	movq   $0x8,0x50(%rsp)
  247333:	movq   $0x0,0x58(%rsp)
  24733c:	shr    $0xa,%rbp
  247340:	mov    %rax,0x38(%rsp)
  247345:	lea    0x0(%rbp,%r14,1),%rcx
  24734a:	cmp    %rbx,%rcx
  24734d:	cmovae %rbx,%rcx
  247351:	test   %rcx,%rcx
  247354:	jne    2473fa <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xc9a>
  24735a:	movq   $0x8,0x90(%rsp)
  247366:	movq   $0x0,0x98(%rsp)
  247372:	mov    0x220(%rsp),%rcx
  24737a:	lea    0xa0(%rsp),%rdi
  247382:	mov    $0x8,%edx
  247387:	mov    $0x8,%r8d
  24738d:	mov    $0x8,%r9d
  247393:	xor    %esi,%esi
  247395:	call   1e1430 <<alloc::raw_vec::RawVecInner>::finish_grow>
  24739a:	cmpb   $0x0,0xa0(%rsp)
  2473a2:	je     24751b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xdbb>
  2473a8:	mov    0x48(%rsp),%rax
  2473ad:	test   %rax,%rax
  2473b0:	je     246a1c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2bc>
  2473b6:	mov    0x50(%rsp),%rdi
  2473bb:	cmp    $0x1fff,%rax
  2473c1:	jbe    247447 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xce7>
  2473c7:	shl    $0x4,%rax
  2473cb:	movabs $0x7ffffffffffffff9,%rcx
  2473d5:	mov    %rax,%rsi
  2473d8:	or     $0x1,%rsi
  2473dc:	cmp    %rcx,%rsi
  2473df:	jae    24820d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1aad>
  2473e5:	cmpb   $0x1,(%rdi,%rax,1)
  2473e9:	jne    247510 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xdb0>
  2473ef:	call   *0x2a3853(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  2473f5:	jmp    246a1c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2bc>
  2473fa:	lea    0xa0(%rsp),%rdi
  247402:	mov    $0x8,%edx
  247407:	mov    $0x8,%r8d
  24740d:	mov    $0x10,%r9d
  247413:	xor    %esi,%esi
  247415:	mov    %rcx,%rbx
  247418:	call   1e1430 <<alloc::raw_vec::RawVecInner>::finish_grow>
  24741d:	cmpb   $0x0,0xa0(%rsp)
  247425:	jne    246a1c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2bc>
  24742b:	mov    0xa8(%rsp),%rax
  247433:	mov    %rax,0x50(%rsp)
  247438:	mov    %rax,0x38(%rsp)
  24743d:	mov    %rbx,0x48(%rsp)
  247442:	jmp    24735a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xbfa>
  247447:	jmp    247510 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xdb0>
  24744c:	lea    0x48(%rsp),%rdi
  247451:	mov    $0x8,%ecx
  247456:	mov    $0x80,%r8d
  24745c:	xor    %esi,%esi
  24745e:	mov    %r14,%rdx
  247461:	call   1b6760 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  247466:	mov    0x48(%rsp),%r13
  24746b:	mov    0x58(%rsp),%rcx
  247470:	mov    %r13,%rax
  247473:	sub    %rcx,%rax
  247476:	cmp    %r14,%rax
  247479:	jae    248235 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ad5>
  24747f:	lea    -0x2278c9(%rip),%rdi        # 1fbbd <zmij::STATIC_DATA+0x55bd>
  247486:	lea    0x2a0ceb(%rip),%rdx        # 4e8178 <__frame_dummy_init_array_entry+0x6318>
  24748d:	mov    $0x2f,%esi
  247492:	call   f0cd0 <core::panicking::panic>
  247497:	jmp    24820b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1aab>
  24749c:	lea    -0x2278e6(%rip),%rdi        # 1fbbd <zmij::STATIC_DATA+0x55bd>
  2474a3:	lea    0x2a1166(%rip),%rdx        # 4e8610 <__frame_dummy_init_array_entry+0x67b0>
  2474aa:	mov    $0x2f,%esi
  2474af:	call   f0cd0 <core::panicking::panic>
  2474b4:	jmp    24820b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1aab>
  2474b9:	lea    0x108(%rsp),%rax
  2474c1:	lea    -0x154278(%rip),%r8        # f3250 <<u64 as core::fmt::Display>::fmt>
  2474c8:	lea    0x130(%rsp),%rcx
  2474d0:	mov    %rax,0xa0(%rsp)
  2474d8:	mov    %r8,0xa8(%rsp)
  2474e0:	mov    %rcx,0xb0(%rsp)
  2474e8:	mov    %r8,0xb8(%rsp)
  2474f0:	lea    -0x2305b2(%rip),%rdi        # 16f45 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x5345>
  2474f7:	lea    0x29ecda(%rip),%rdx        # 4e61d8 <__frame_dummy_init_array_entry+0x4378>
  2474fe:	lea    0xa0(%rsp),%rsi
  247506:	call   eec00 <core::panicking::panic_fmt>
  24750b:	jmp    24820b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1aab>
  247510:	call   *0x2a373a(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247516:	jmp    246a1c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2bc>
  24751b:	mov    0xa8(%rsp),%rbx
  247523:	mov    0x220(%rsp),%rax
  24752b:	inc    %rbp
  24752e:	movq   $0x0,0x20(%rsp)
  247537:	movq   $0x8,0x28(%rsp)
  247540:	mov    %r15,0x60(%rsp)
  247545:	mov    %r12,0x18(%rsp)
  24754a:	movq   $0x0,0x30(%rsp)
  247553:	mov    %rbx,0x90(%rsp)
  24755b:	mov    %rax,0x88(%rsp)
  247563:	lea    0xa0(%rsp),%rdi
  24756b:	mov    $0x8,%edx
  247570:	mov    $0x8,%r8d
  247576:	mov    $0x8,%r9d
  24757c:	xor    %esi,%esi
  24757e:	mov    %rbp,%rcx
  247581:	call   1e1430 <<alloc::raw_vec::RawVecInner>::finish_grow>
  247586:	cmpb   $0x0,0xa0(%rsp)
  24758e:	je     2475f2 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xe92>
  247590:	mov    0x88(%rsp),%rax
  247598:	mov    0x18(%rsp),%r12
  24759d:	mov    0x60(%rsp),%r15
  2475a2:	test   %rax,%rax
  2475a5:	je     2473a8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xc48>
  2475ab:	mov    0x90(%rsp),%rdi
  2475b3:	cmp    $0x3fff,%rax
  2475b9:	jbe    2479f4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1294>
  2475bf:	shl    $0x3,%rax
  2475c3:	movabs $0x7ffffffffffffff9,%rcx
  2475cd:	mov    %rax,%rsi
  2475d0:	or     $0x1,%rsi
  2475d4:	cmp    %rcx,%rsi
  2475d7:	jae    248212 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ab2>
  2475dd:	cmpb   $0x1,(%rdi,%rax,1)
  2475e1:	jne    2479f6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1296>
  2475e7:	call   *0x2a365b(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  2475ed:	jmp    2479fc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x129c>
  2475f2:	mov    0xa8(%rsp),%rax
  2475fa:	cmpq   $0x1,0x220(%rsp)
  247603:	mov    %rax,0x28(%rsp)
  247608:	mov    %rbp,0x20(%rsp)
  24760d:	movq   $0x0,(%rbx)
  247614:	movq   $0x1,0x98(%rsp)
  247620:	movq   $0x0,(%rax)
  247627:	movq   $0x1,0x30(%rsp)
  247630:	jne    24786e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x110e>
  247636:	movq   $0x0,0x8(%rsp)
  24763f:	cmpq   $0x0,0x40(%rsp)
  247645:	je     247673 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xf13>
  247647:	mov    0x30(%rsp),%rbx
  24764c:	cmp    0x20(%rsp),%rbx
  247651:	jne    24765d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xefd>
  247653:	lea    0x20(%rsp),%rdi
  247658:	call   f4890 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  24765d:	mov    0x28(%rsp),%rax
  247662:	mov    0x8(%rsp),%rcx
  247667:	mov    %rcx,(%rax,%rbx,8)
  24766b:	inc    %rbx
  24766e:	mov    %rbx,0x30(%rsp)
  247673:	lea    0x50(%rsp),%rax
  247678:	vmovdqu 0x88(%rsp),%xmm2
  247681:	vmovdqu 0x20(%rsp),%xmm1
  247687:	mov    0x48(%rsp),%rbp
  24768c:	mov    0x18(%rsp),%r12
  247691:	mov    0x60(%rsp),%r15
  247696:	vmovups (%rax),%xmm0
  24769a:	mov    0x98(%rsp),%rax
  2476a2:	vmovaps %xmm0,0xa0(%rsp)
  2476ab:	mov    %rax,0xc0(%rsp)
  2476b3:	mov    0x30(%rsp),%rax
  2476b8:	vmovdqa %xmm2,0xb0(%rsp)
  2476c1:	vmovdqu %xmm1,0xc8(%rsp)
  2476ca:	mov    %rax,0xd8(%rsp)
  2476d2:	vmovdqu64 0xa0(%rsp),%zmm0
  2476dd:	vmovdqu64 %zmm0,0x1a0(%rsp)
  2476e8:	cmp    $0xffffffffffffffff,%rbp
  2476ec:	je     246a1c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2bc>
  2476f2:	vmovdqu64 0x1a0(%rsp),%zmm0
  2476fd:	mov    %rbp,0x130(%rsp)
  247705:	mov    $0x8,%eax
  24770a:	mov    %rax,0x38(%rsp)
  24770f:	mov    %rax,0x8(%rsp)
  247714:	vmovdqu64 %zmm0,0x138(%rsp)
  24771f:	mov    0x140(%rsp),%r15
  247727:	test   %r15,%r15
  24772a:	jne    247ebd <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x175d>
  247730:	movq   $0x0,0x20(%rsp)
  247739:	movq   $0x8,0x28(%rsp)
  247742:	movq   $0x0,0x30(%rsp)
  24774b:	test   %r14,%r14
  24774e:	jne    247f03 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x17a3>
  247754:	mov    0x158(%rsp),%rdx
  24775c:	mov    0x150(%rsp),%rax
  247764:	cmp    $0x1,%rdx
  247768:	mov    %rdx,0x10(%rsp)
  24776d:	mov    %rax,0x68(%rsp)
  247772:	adc    $0xffffffffffffffff,%rdx
  247776:	cmp    %r14,%rdx
  247779:	ja     248176 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1a16>
  24777f:	xor    %r14d,%r14d
  247782:	cmpq   $0x2,0x10(%rsp)
  247788:	jb     2477ee <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x108e>
  24778a:	mov    0x68(%rsp),%rax
  24778f:	cmpq   $0x21,0x10(%rsp)
  247795:	mov    (%rax),%rdx
  247798:	jb     2477b8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1058>
  24779a:	mov    0x38(%rsp),%rax
  24779f:	lea    (%rax,%r14,8),%rax
  2477a3:	sub    0x68(%rsp),%rax
  2477a8:	add    $0xfffffffffffffff8,%rax
  2477ac:	cmp    $0x100,%rax
  2477b2:	jae    247a43 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x12e3>
  2477b8:	mov    0x10(%rsp),%rax
  2477bd:	mov    0x68(%rsp),%rcx
  2477c2:	mov    0x38(%rsp),%r8
  2477c7:	inc    %rax
  2477ca:	add    $0x8,%rcx
  2477ce:	mov    %rdx,%rsi
  2477d1:	mov    (%rcx),%rdx
  2477d4:	dec    %rax
  2477d7:	add    $0x8,%rcx
  2477db:	mov    %rdx,%rdi
  2477de:	sub    %rsi,%rdi
  2477e1:	mov    %rdi,(%r8,%r14,8)
  2477e5:	inc    %r14
  2477e8:	cmp    $0x2,%rax
  2477ec:	ja     2477ce <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x106e>
  2477ee:	mov    $0x4,%r12d
  2477f4:	mov    %r14,0x30(%rsp)
  2477f9:	test   %r15,%r15
  2477fc:	jne    247f3e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x17de>
  247802:	lea    -0x227ece(%rip),%rsi        # 1f93b <zmij::STATIC_DATA+0x533b>
  247809:	lea    0xa0(%rsp),%rdi
  247811:	mov    $0x16,%edx
  247816:	vzeroupper
  247819:	call   143720 <std::sys::env::unix::getenv>
  24781e:	mov    0xa0(%rsp),%rax
  247826:	cmp    $0xffffffffffffffff,%rax
  24782a:	je     247a0b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x12ab>
  247830:	test   %rax,%rax
  247833:	je     247b2d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x13cd>
  247839:	mov    0xa8(%rsp),%rdi
  247841:	cmp    $0x1ffff,%rax
  247847:	jbe    247a3e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x12de>
  24784d:	mov    %rax,%rsi
  247850:	inc    %rsi
  247853:	js     248230 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ad0>
  247859:	cmpb   $0x1,(%rdi,%rax,1)
  24785d:	jne    247b27 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x13c7>
  247863:	call   *0x2a33df(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  247869:	jmp    247b2d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x13cd>
  24786e:	mov    0x220(%rsp),%rax
  247876:	mov    0x18(%rsp),%rdx
  24787b:	xor    %r12d,%r12d
  24787e:	movq   $0x0,0x8(%rsp)
  247887:	jmp    2478c9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1169>
  247889:	mov    0x90(%rsp),%rax
  247891:	mov    0x8(%rsp),%rcx
  247896:	mov    0x178(%rsp),%rdx
  24789e:	mov    0x10(%rsp),%r12
  2478a3:	mov    %rcx,(%rax,%rbx,8)
  2478a7:	mov    0x68(%rsp),%rcx
  2478ac:	inc    %rbx
  2478af:	add    $0x8,%rdx
  2478b3:	mov    %rbx,0x98(%rsp)
  2478bb:	lea    -0x1(%rcx),%rax
  2478bf:	cmp    $0x3,%rcx
  2478c3:	jb     24763f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xedf>
  2478c9:	mov    0x8(%rdx),%rcx
  2478cd:	mov    %rdx,0x178(%rsp)
  2478d5:	mov    %rax,0x68(%rsp)
  2478da:	mov    %rcx,0x10(%rsp)
  2478df:	cmp    %rcx,%r12
  2478e2:	jae    2479cc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x126c>
  2478e8:	mov    0x8(%rsp),%r15
  2478ed:	shl    $0x4,%r15
  2478f1:	or     $0x8,%r15
  2478f5:	jmp    24791f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x11bf>
  2478f7:	mov    0x28(%rsp),%rax
  2478fc:	mov    0x8(%rsp),%rcx
  247901:	mov    %rcx,(%rax,%r12,8)
  247905:	inc    %r12
  247908:	mov    %r12,0x30(%rsp)
  24790d:	add    $0x10,%r15
  247911:	mov    %rbp,%r12
  247914:	cmp    0x10(%rsp),%rbx
  247919:	jae    2479cc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x126c>
  24791f:	cmp    $0xfffffffffffffbff,%r12
  247926:	ja     247a18 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x12b8>
  24792c:	mov    0x40(%rsp),%rax
  247931:	mov    0x10(%rsp),%rbp
  247936:	mov    %r12,%rbx
  247939:	and    $0xfffffffffffffc00,%rbx
  247940:	add    $0x400,%rbx
  247947:	cmp    %rbx,%rax
  24794a:	cmovb  %rax,%rbx
  24794e:	mov    0x8(%rsp),%rax
  247953:	cmp    %rbp,%rbx
  247956:	cmovb  %rbx,%rbp
  24795a:	cmp    0x48(%rsp),%rax
  24795f:	jne    247975 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1215>
  247961:	lea    0x48(%rsp),%rdi
  247966:	call   2b7f10 <<alloc::raw_vec::RawVec<zopro_core::preparation::Reservation>>::grow_one>
  24796b:	mov    0x50(%rsp),%rax
  247970:	mov    %rax,0x38(%rsp)
  247975:	mov    0x8(%rsp),%rcx
  24797a:	mov    0x38(%rsp),%rax
  24797f:	inc    %rcx
  247982:	cmp    0x10(%rsp),%rbx
  247987:	mov    %r12,-0x8(%rax,%r15,1)
  24798c:	mov    %rbp,(%rax,%r15,1)
  247990:	mov    %rcx,0x8(%rsp)
  247995:	mov    %rcx,0x58(%rsp)
  24799a:	setbe  %al
  24799d:	cmp    0x40(%rsp),%rbp
  2479a2:	setb   %cl
  2479a5:	test   %cl,%al
  2479a7:	je     24790d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x11ad>
  2479ad:	mov    0x30(%rsp),%r12
  2479b2:	cmp    0x20(%rsp),%r12
  2479b7:	jne    2478f7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1197>
  2479bd:	lea    0x20(%rsp),%rdi
  2479c2:	call   f4890 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  2479c7:	jmp    2478f7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1197>
  2479cc:	mov    0x98(%rsp),%rbx
  2479d4:	cmp    0x88(%rsp),%rbx
  2479dc:	jne    247889 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1129>
  2479e2:	lea    0x88(%rsp),%rdi
  2479ea:	call   f4890 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  2479ef:	jmp    247889 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1129>
  2479f4:	jmp    2479f6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1296>
  2479f6:	call   *0x2a3254(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  2479fc:	mov    0x18(%rsp),%r12
  247a01:	mov    0x60(%rsp),%r15
  247a06:	jmp    2473a8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xc48>
  247a0b:	mov    0x170(%rsp),%rbx
  247a13:	jmp    247c24 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x14c4>
  247a18:	mov    0x20(%rsp),%rsi
  247a1d:	test   %rsi,%rsi
  247a20:	je     247590 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xe30>
  247a26:	mov    0x28(%rsp),%rdi
  247a2b:	shl    $0x3,%rsi
  247a2f:	mov    $0x8,%edx
  247a34:	call   e9120 <__rustc::__rust_dealloc>
  247a39:	jmp    247590 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xe30>
  247a3e:	jmp    247b27 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x13c7>
  247a43:	mov    0x10(%rsp),%rax
  247a48:	vpbroadcastq %rdx,%zmm0
  247a4e:	mov    0x68(%rsp),%r9
  247a53:	mov    0x38(%rsp),%rdx
  247a58:	lea    0x0(,%r14,8),%r8
  247a60:	lea    -0x1(%rax),%rsi
  247a64:	lea    0xc0(%r8,%rdx,1),%rdx
  247a6c:	xor    %r8d,%r8d
  247a6f:	mov    %rsi,%rdi
  247a72:	and    $0xffffffffffffffe0,%rdi
  247a76:	lea    (%r9,%rdi,8),%rcx
  247a7a:	add    %rdi,%r14
  247a7d:	sub    %rdi,%rax
  247a80:	vmovdqu64 0x8(%r9,%r8,8),%zmm1
  247a8b:	vmovdqu64 0x48(%r9,%r8,8),%zmm2
  247a96:	vmovdqu64 0x88(%r9,%r8,8),%zmm3
  247aa1:	valignq $0x7,%zmm0,%zmm1,%zmm4
  247aa8:	vmovdqu64 0xc8(%r9,%r8,8),%zmm0
  247ab3:	valignq $0x7,%zmm1,%zmm2,%zmm5
  247aba:	valignq $0x7,%zmm2,%zmm3,%zmm6
  247ac1:	vpsubq %zmm4,%zmm1,%zmm1
  247ac7:	vpsubq %zmm5,%zmm2,%zmm2
  247acd:	vmovdqu64 %zmm1,-0xc0(%rdx,%r8,8)
  247ad5:	vmovdqu64 %zmm2,-0x80(%rdx,%r8,8)
  247add:	valignq $0x7,%zmm3,%zmm0,%zmm7
  247ae4:	vpsubq %zmm6,%zmm3,%zmm3
  247aea:	vmovdqu64 %zmm3,-0x40(%rdx,%r8,8)
  247af2:	vpsubq %zmm7,%zmm0,%zmm4
  247af8:	vmovdqu64 %zmm4,(%rdx,%r8,8)
  247aff:	add    $0x20,%r8
  247b03:	cmp    %r8,%rdi
  247b06:	jne    247a80 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1320>
  247b0c:	cmp    %rdi,%rsi
  247b0f:	je     2477ee <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x108e>
  247b15:	vextracti32x4 $0x3,%zmm0,%xmm0
  247b1c:	vpextrq $0x1,%xmm0,%rdx
  247b22:	jmp    2477c2 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1062>
  247b27:	call   *0x2a3123(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247b2d:	mov    0x170(%rsp),%rbx
  247b35:	mov    0x40(%rsp),%rax
  247b3a:	lea    0x190(%rsp),%rcx
  247b42:	lea    -0x1548f9(%rip),%r10        # f3250 <<u64 as core::fmt::Display>::fmt>
  247b49:	lea    0x198(%rsp),%r9
  247b51:	mov    %r15,0x108(%rsp)
  247b59:	mov    %rcx,0xa0(%rsp)
  247b61:	lea    0x180(%rsp),%rcx
  247b69:	mov    %r10,0xa8(%rsp)
  247b71:	mov    %rcx,0xb0(%rsp)
  247b79:	mov    %r10,0xb8(%rsp)
  247b81:	lea    0x2a4e38(%rip),%rcx        # 4ec9c0 <std::io::stdio::stderr::INSTANCE>
  247b88:	mov    %rcx,0xf0(%rsp)
  247b90:	lea    -0x1(%rbx),%r8
  247b94:	mov    %rax,0x190(%rsp)
  247b9c:	mov    %r8,0x198(%rsp)
  247ba4:	lea    0x188(%rsp),%r8
  247bac:	mov    %r8,0xc0(%rsp)
  247bb4:	lea    0x108(%rsp),%r8
  247bbc:	mov    %r10,0xc8(%rsp)
  247bc4:	mov    %r9,0xd0(%rsp)
  247bcc:	mov    %r10,0xd8(%rsp)
  247bd4:	mov    %r8,0xe0(%rsp)
  247bdc:	lea    -0x22d9cb(%rip),%r8        # 1a218 <itoa::DECIMAL_PAIRS+0xa8a>
  247be3:	mov    %r10,0xe8(%rsp)
  247beb:	mov    %r8,0x88(%rsp)
  247bf3:	movq   $0x6,0x90(%rsp)
  247bff:	lea    -0x23102e(%rip),%rsi        # 16bd8 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x4fd8>
  247c06:	lea    0xf0(%rsp),%rdi
  247c0e:	lea    0xa0(%rsp),%rdx
  247c16:	call   13f8d0 <<&std::io::stdio::Stderr as std::io::Write>::write_fmt>
  247c1b:	test   %rax,%rax
  247c1e:	jne    2481b0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1a50>
  247c24:	mov    0x138(%rsp),%r8
  247c2c:	mov    0x118(%rsp),%rdx
  247c34:	mov    0x120(%rsp),%rdi
  247c3c:	mov    0x128(%rsp),%rsi
  247c44:	mov    0x40(%rsp),%rcx
  247c49:	mov    %r15,%r9
  247c4c:	push   %r15
  247c4e:	push   0x10(%rsp)
  247c52:	push   %rbx
  247c53:	push   0x180(%rsp)
  247c5a:	call   28e7e0 <zopro_core::msm::packed_index::fill::<zopro_bn254::curve::G2Config>>
  247c5f:	add    $0x20,%rsp
  247c63:	sub    $0x8,%rsp
  247c67:	mov    0x10(%rsp),%rsi
  247c6c:	mov    0x70(%rsp),%r9
  247c71:	mov    %r15,%rdx
  247c74:	lea    0xa8(%rsp),%rdi
  247c7c:	mov    %r12,%rcx
  247c7f:	mov    %r15,%r8
  247c82:	push   %r14
  247c84:	push   0x48(%rsp)
  247c88:	push   0x28(%rsp)
  247c8c:	call   22b9f0 <zopro_core::msm::group_affine::<zopro_bn254::curve::G2Config>>
  247c91:	add    $0x20,%rsp
  247c95:	vmovdqu 0xa8(%rsp),%xmm0
  247c9e:	mov    0xa0(%rsp),%r14
  247ca6:	vmovdqa %xmm0,0x70(%rsp)
  247cac:	test   %r15,%r15
  247caf:	je     247ce3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1583>
  247cb1:	cmp    $0x7fff,%r15
  247cb8:	jbe    247cd8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1578>
  247cba:	cmpb   $0x1,(%r12,%r15,4)
  247cbf:	jne    247cda <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x157a>
  247cc1:	lea    0x0(,%r15,4),%rsi
  247cc9:	or     $0x1,%rsi
  247ccd:	mov    %r12,%rdi
  247cd0:	call   *0x2a2f72(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  247cd6:	jmp    247ce3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1583>
  247cd8:	jmp    247cda <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x157a>
  247cda:	mov    %r12,%rdi
  247cdd:	call   *0x2a2f6d(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247ce3:	mov    0x20(%rsp),%rax
  247ce8:	test   %rax,%rax
  247ceb:	je     247d38 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x15d8>
  247ced:	cmp    $0x3fff,%rax
  247cf3:	jbe    247d2b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x15cb>
  247cf5:	shl    $0x3,%rax
  247cf9:	movabs $0x7ffffffffffffff9,%rcx
  247d03:	mov    %rax,%rsi
  247d06:	or     $0x1,%rsi
  247d0a:	cmp    %rcx,%rsi
  247d0d:	jae    248217 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ab7>
  247d13:	mov    0x38(%rsp),%rcx
  247d18:	cmpb   $0x1,(%rcx,%rax,1)
  247d1c:	jne    247d2d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x15cd>
  247d1e:	mov    0x38(%rsp),%rdi
  247d23:	call   *0x2a2f1f(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  247d29:	jmp    247d38 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x15d8>
  247d2b:	jmp    247d2d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x15cd>
  247d2d:	mov    0x38(%rsp),%rdi
  247d32:	call   *0x2a2f18(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247d38:	test   %r15,%r15
  247d3b:	je     247d8a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x162a>
  247d3d:	cmp    $0x3ff,%r15
  247d44:	jbe    247d7d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x161d>
  247d46:	shl    $0x7,%r15
  247d4a:	movabs $0x7ffffffffffffff9,%rax
  247d54:	mov    %r15,%rsi
  247d57:	or     $0x1,%rsi
  247d5b:	cmp    %rax,%rsi
  247d5e:	jae    24821c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1abc>
  247d64:	mov    0x8(%rsp),%rax
  247d69:	cmpb   $0x1,(%rax,%r15,1)
  247d6e:	jne    247d7f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x161f>
  247d70:	mov    0x8(%rsp),%rdi
  247d75:	call   *0x2a2ecd(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  247d7b:	jmp    247d8a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x162a>
  247d7d:	jmp    247d7f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x161f>
  247d7f:	mov    0x8(%rsp),%rdi
  247d84:	call   *0x2a2ec6(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247d8a:	test   %rbp,%rbp
  247d8d:	je     247dd4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1674>
  247d8f:	mov    0x138(%rsp),%rdi
  247d97:	cmp    $0x1fff,%rbp
  247d9e:	jbe    247dcc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x166c>
  247da0:	shl    $0x4,%rbp
  247da4:	movabs $0x7ffffffffffffff9,%rax
  247dae:	mov    %rbp,%rsi
  247db1:	or     $0x1,%rsi
  247db5:	cmp    %rax,%rsi
  247db8:	jae    248221 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ac1>
  247dbe:	cmpb   $0x1,(%rdi,%rbp,1)
  247dc2:	jne    247dce <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x166e>
  247dc4:	call   *0x2a2e7e(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  247dca:	jmp    247dd4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1674>
  247dcc:	jmp    247dce <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x166e>
  247dce:	call   *0x2a2e7c(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247dd4:	mov    0x148(%rsp),%rax
  247ddc:	test   %rax,%rax
  247ddf:	je     247e25 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x16c5>
  247de1:	mov    0x150(%rsp),%rdi
  247de9:	cmp    $0x3fff,%rax
  247def:	jbe    247e1d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x16bd>
  247df1:	shl    $0x3,%rax
  247df5:	movabs $0x7ffffffffffffff9,%rcx
  247dff:	mov    %rax,%rsi
  247e02:	or     $0x1,%rsi
  247e06:	cmp    %rcx,%rsi
  247e09:	jae    248226 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ac6>
  247e0f:	cmpb   $0x1,(%rdi,%rax,1)
  247e13:	jne    247e1f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x16bf>
  247e15:	call   *0x2a2e2d(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  247e1b:	jmp    247e25 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x16c5>
  247e1d:	jmp    247e1f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x16bf>
  247e1f:	call   *0x2a2e2b(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247e25:	mov    0x160(%rsp),%rax
  247e2d:	test   %rax,%rax
  247e30:	je     247e76 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1716>
  247e32:	mov    0x168(%rsp),%rdi
  247e3a:	cmp    $0x3fff,%rax
  247e40:	jbe    247e6e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x170e>
  247e42:	shl    $0x3,%rax
  247e46:	movabs $0x7ffffffffffffff9,%rcx
  247e50:	mov    %rax,%rsi
  247e53:	or     $0x1,%rsi
  247e57:	cmp    %rcx,%rsi
  247e5a:	jae    24822b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1acb>
  247e60:	cmpb   $0x1,(%rdi,%rax,1)
  247e64:	jne    247e70 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1710>
  247e66:	call   *0x2a2ddc(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  247e6c:	jmp    247e76 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1716>
  247e6e:	jmp    247e70 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1710>
  247e70:	call   *0x2a2dda(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  247e76:	lock sub %r13,0x2a4bb2(%rip)        # 4eca30 <zopro_core::preparation::RESERVED>
  247e7e:	mov    0x220(%rsp),%r13
  247e86:	mov    0x60(%rsp),%rax
  247e8b:	lock sub %rax,0x2a4ba5(%rip)        # 4eca38 <zopro_core::preparation::WORKSPACES>
  247e93:	mov    0x18(%rsp),%r12
  247e98:	cmp    $0xffffffffffffffff,%r14
  247e9c:	je     246a34 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2d4>
  247ea2:	vmovdqa 0x70(%rsp),%xmm0
  247ea8:	mov    0x110(%rsp),%rax
  247eb0:	mov    %r14,(%rax)
  247eb3:	vmovdqu %xmm0,0x8(%rax)
  247eb8:	jmp    2472ee <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb8e>
  247ebd:	lea    0xa0(%rsp),%rdi
  247ec5:	mov    $0x8,%edx
  247eca:	mov    $0x8,%r8d
  247ed0:	mov    $0x80,%r9d
  247ed6:	xor    %esi,%esi
  247ed8:	mov    %r15,%rcx
  247edb:	vzeroupper
  247ede:	call   1e1430 <<alloc::raw_vec::RawVecInner>::finish_grow>
  247ee3:	cmpb   $0x0,0xa0(%rsp)
  247eeb:	jne    247fa5 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1845>
  247ef1:	mov    0xa8(%rsp),%rax
  247ef9:	mov    %rax,0x8(%rsp)
  247efe:	jmp    247730 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xfd0>
  247f03:	lea    0xa0(%rsp),%rdi
  247f0b:	mov    $0x8,%edx
  247f10:	mov    $0x8,%r8d
  247f16:	mov    $0x8,%r9d
  247f1c:	xor    %esi,%esi
  247f1e:	mov    %r14,%rcx
  247f21:	vzeroupper
  247f24:	call   1e1430 <<alloc::raw_vec::RawVecInner>::finish_grow>
  247f29:	cmpb   $0x0,0xa0(%rsp)
  247f31:	je     24801c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x18bc>
  247f37:	test   %r15,%r15
  247f3a:	jne    247f8f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x182f>
  247f3c:	jmp    247fa5 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1845>
  247f3e:	lea    0xa0(%rsp),%rdi
  247f46:	mov    $0x4,%edx
  247f4b:	mov    $0x4,%r8d
  247f51:	mov    $0x4,%r9d
  247f57:	xor    %esi,%esi
  247f59:	mov    %r15,%rcx
  247f5c:	vzeroupper
  247f5f:	call   1e1430 <<alloc::raw_vec::RawVecInner>::finish_grow>
  247f64:	cmpb   $0x0,0xa0(%rsp)
  247f6c:	je     248038 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x18d8>
  247f72:	mov    0x20(%rsp),%rsi
  247f77:	test   %rsi,%rsi
  247f7a:	je     247f8f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x182f>
  247f7c:	mov    0x28(%rsp),%rdi
  247f81:	shl    $0x3,%rsi
  247f85:	mov    $0x8,%edx
  247f8a:	call   e9120 <__rustc::__rust_dealloc>
  247f8f:	shl    $0x7,%r15
  247f93:	mov    0x8(%rsp),%rdi
  247f98:	mov    $0x8,%edx
  247f9d:	mov    %r15,%rsi
  247fa0:	call   e9120 <__rustc::__rust_dealloc>
  247fa5:	test   %rbp,%rbp
  247fa8:	je     247fc3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1863>
  247faa:	mov    0x138(%rsp),%rdi
  247fb2:	shl    $0x4,%rbp
  247fb6:	mov    $0x8,%edx
  247fbb:	mov    %rbp,%rsi
  247fbe:	call   e9120 <__rustc::__rust_dealloc>
  247fc3:	mov    0x148(%rsp),%rsi
  247fcb:	test   %rsi,%rsi
  247fce:	je     247fe6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1886>
  247fd0:	mov    0x150(%rsp),%rdi
  247fd8:	shl    $0x3,%rsi
  247fdc:	mov    $0x8,%edx
  247fe1:	call   e9120 <__rustc::__rust_dealloc>
  247fe6:	mov    0x160(%rsp),%rsi
  247fee:	mov    0x18(%rsp),%r12
  247ff3:	mov    0x60(%rsp),%r15
  247ff8:	test   %rsi,%rsi
  247ffb:	je     246a1c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2bc>
  248001:	mov    0x168(%rsp),%rdi
  248009:	shl    $0x3,%rsi
  24800d:	mov    $0x8,%edx
  248012:	call   e9120 <__rustc::__rust_dealloc>
  248017:	jmp    246a1c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x2bc>
  24801c:	mov    0xa8(%rsp),%rax
  248024:	mov    %rax,0x28(%rsp)
  248029:	mov    %rax,0x38(%rsp)
  24802e:	mov    %r14,0x20(%rsp)
  248033:	jmp    247754 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xff4>
  248038:	mov    0xa8(%rsp),%r12
  248040:	cmp    $0x8,%r15
  248044:	jae    24805a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x18fa>
  248046:	xor    %eax,%eax
  248048:	jmp    248560 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1e00>
  24804d:	mov    $0x8,%edi
  248052:	mov    %r14,%rsi
  248055:	call   ede21 <alloc::raw_vec::handle_error>
  24805a:	cmp    $0x40,%r15
  24805e:	jae    24807e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x191e>
  248060:	xor    %eax,%eax
  248062:	jmp    248124 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x19c4>
  248067:	call   110970 <std::process::abort>
  24806c:	mov    $0x4,%edi
  248071:	mov    %r13,%rsi
  248074:	call   ede21 <alloc::raw_vec::handle_error>
  248079:	jmp    24820b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1aab>
  24807e:	vmovdqa64 -0x23b388(%rip),%zmm0        # cd00 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x11c0>
  248088:	vpbroadcastd -0x23c212(%rip),%zmm1        # be80 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x340>
  248092:	vpbroadcastd -0x23c1f8(%rip),%zmm2        # bea4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x364>
  24809c:	vpbroadcastd -0x23c1ee(%rip),%zmm3        # beb8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x378>
  2480a6:	vpbroadcastd -0x23c1ec(%rip),%zmm4        # bec4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x384>
  2480b0:	movabs $0x800000000000000,%rax
  2480ba:	mov    %r15,%rcx
  2480bd:	shr    $0x6,%rcx
  2480c1:	xor    %edx,%edx
  2480c3:	add    $0xffffffffffffffc0,%rax
  2480c7:	shl    $0x8,%rcx
  2480cb:	and    %r15,%rax
  2480ce:	vpaddd %zmm1,%zmm0,%zmm5
  2480d4:	vpaddd %zmm2,%zmm0,%zmm6
  2480da:	vpaddd %zmm3,%zmm0,%zmm7
  2480e0:	vmovdqu64 %zmm0,(%r12,%rdx,1)
  2480e7:	vpaddd %zmm4,%zmm0,%zmm0
  2480ed:	vmovdqu64 %zmm5,0x40(%r12,%rdx,1)
  2480f5:	vmovdqu64 %zmm6,0x80(%r12,%rdx,1)
  2480fd:	vmovdqu64 %zmm7,0xc0(%r12,%rdx,1)
  248105:	add    $0x100,%rdx
  24810c:	cmp    %rdx,%rcx
  24810f:	jne    2480ce <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x196e>
  248111:	cmp    %rax,%r15
  248114:	je     247802 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x10a2>
  24811a:	test   $0x38,%r15b
  24811e:	je     248560 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1e00>
  248124:	mov    %rax,%rcx
  248127:	vpbroadcastd %ecx,%ymm0
  24812d:	vpor   -0x2367f5(%rip),%ymm0,%ymm0        # 11940 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5e00>
  248135:	vpbroadcastd -0x23c296(%rip),%ymm1        # bea8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x368>
  24813e:	movabs $0x800000000000000,%rax
  248148:	add    $0xfffffffffffffff8,%rax
  24814c:	and    %r15,%rax
  24814f:	vmovdqu %ymm0,(%r12,%rcx,4)
  248155:	add    $0x8,%rcx
  248159:	vpaddd %ymm1,%ymm0,%ymm0
  24815d:	cmp    %rcx,%rax
  248160:	jne    24814f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x19ef>
  248162:	jmp    248567 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1e07>
  248167:	call   110970 <std::process::abort>
  24816c:	call   110970 <std::process::abort>
  248171:	call   110970 <std::process::abort>
  248176:	lea    0x20(%rsp),%rdi
  24817b:	mov    $0x8,%ecx
  248180:	mov    $0x8,%r8d
  248186:	xor    %esi,%esi
  248188:	vzeroupper
  24818b:	call   1b6760 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  248190:	mov    0x28(%rsp),%rax
  248195:	mov    0x30(%rsp),%r14
  24819a:	mov    %rax,0x38(%rsp)
  24819f:	cmpq   $0x2,0x10(%rsp)
  2481a5:	jae    24778a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x102a>
  2481ab:	jmp    2477ee <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x108e>
  2481b0:	lea    0x88(%rsp),%r8
  2481b8:	lea    -0x15960f(%rip),%rcx        # eebb0 <<&str as core::fmt::Display>::fmt>
  2481bf:	mov    %rax,0x48(%rsp)
  2481c4:	mov    %r8,0x1a0(%rsp)
  2481cc:	mov    %rcx,0x1a8(%rsp)
  2481d4:	lea    0x48(%rsp),%r8
  2481d9:	lea    -0x1551f0(%rip),%rcx        # f2ff0 <<core::io::error::Error as core::fmt::Display>::fmt>
  2481e0:	mov    %r8,0x1b0(%rsp)
  2481e8:	mov    %rcx,0x1b8(%rsp)
  2481f0:	lea    -0x2329f6(%rip),%rdi        # 15801 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x3c01>
  2481f7:	lea    0x29b662(%rip),%rdx        # 4e3860 <__frame_dummy_init_array_entry+0x1a00>
  2481fe:	lea    0x1a0(%rsp),%rsi
  248206:	call   eec00 <core::panicking::panic_fmt>
  24820b:	ud2
  24820d:	call   110970 <std::process::abort>
  248212:	call   110970 <std::process::abort>
  248217:	call   110970 <std::process::abort>
  24821c:	call   110970 <std::process::abort>
  248221:	call   110970 <std::process::abort>
  248226:	call   110970 <std::process::abort>
  24822b:	call   110970 <std::process::abort>
  248230:	call   110970 <std::process::abort>
  248235:	mov    0x50(%rsp),%r12
  24823a:	xor    %r15d,%r15d
  24823d:	mov    %rcx,%rax
  248240:	jmp    246c3c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x4dc>
  248245:	mov    %rax,%r14
  248248:	jmp    248302 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ba2>
  24824d:	jmp    24825e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1afe>
  24824f:	mov    %rax,%r14
  248252:	jmp    2482e8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1b88>
  248257:	jmp    24825e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1afe>
  248259:	jmp    2483ec <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1c8c>
  24825e:	mov    0x20(%rsp),%rsi
  248263:	mov    %rax,%r14
  248266:	test   %rsi,%rsi
  248269:	je     24827e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1b1e>
  24826b:	mov    0x28(%rsp),%rdi
  248270:	shl    $0x3,%rsi
  248274:	mov    $0x8,%edx
  248279:	call   e9120 <__rustc::__rust_dealloc>
  24827e:	mov    0x88(%rsp),%rsi
  248286:	test   %rsi,%rsi
  248289:	je     2482a1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1b41>
  24828b:	mov    0x90(%rsp),%rdi
  248293:	shl    $0x3,%rsi
  248297:	mov    $0x8,%edx
  24829c:	call   e9120 <__rustc::__rust_dealloc>
  2482a1:	mov    0x48(%rsp),%rsi
  2482a6:	test   %rsi,%rsi
  2482a9:	je     248347 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1be7>
  2482af:	mov    0x50(%rsp),%rdi
  2482b4:	shl    $0x4,%rsi
  2482b8:	mov    $0x8,%edx
  2482bd:	call   e9120 <__rustc::__rust_dealloc>
  2482c2:	jmp    248347 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1be7>
  2482c7:	mov    %rax,%r14
  2482ca:	mov    0x48(%rsp),%rax
  2482cf:	mov    %eax,%ecx
  2482d1:	and    $0x3,%ecx
  2482d4:	lea    -0x2(%rcx),%edx
  2482d7:	cmp    $0x2,%edx
  2482da:	jb     2482e8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1b88>
  2482dc:	test   %rcx,%rcx
  2482df:	je     2482e8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1b88>
  2482e1:	lea    -0x1(%rax),%rdi
  2482e5:	call   *0x17(%rax)
  2482e8:	test   %r15,%r15
  2482eb:	je     248302 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ba2>
  2482ed:	lea    0x0(,%r15,4),%rsi
  2482f5:	mov    $0x4,%edx
  2482fa:	mov    %r12,%rdi
  2482fd:	call   e9120 <__rustc::__rust_dealloc>
  248302:	mov    0x20(%rsp),%rsi
  248307:	test   %rsi,%rsi
  24830a:	je     24831f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1bbf>
  24830c:	mov    0x28(%rsp),%rdi
  248311:	shl    $0x3,%rsi
  248315:	mov    $0x8,%edx
  24831a:	call   e9120 <__rustc::__rust_dealloc>
  24831f:	test   %r15,%r15
  248322:	je     24833a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1bda>
  248324:	shl    $0x7,%r15
  248328:	mov    0x8(%rsp),%rdi
  24832d:	mov    $0x8,%edx
  248332:	mov    %r15,%rsi
  248335:	call   e9120 <__rustc::__rust_dealloc>
  24833a:	lea    0x130(%rsp),%rdi
  248342:	call   28f5f0 <core::ptr::drop_glue::<zopro_core::msm::packed_index::Layout>>
  248347:	lock sub %r13,0x2a46e1(%rip)        # 4eca30 <zopro_core::preparation::RESERVED>
  24834f:	mov    0x60(%rsp),%rax
  248354:	lock sub %rax,0x2a46dc(%rip)        # 4eca38 <zopro_core::preparation::WORKSPACES>
  24835c:	mov    %r14,%rdi
  24835f:	call   4e0e10 <_Unwind_Resume@plt>
  248364:	call   f2163 <core::panicking::panic_in_cleanup>
  248369:	mov    %rax,%r14
  24836c:	jmp    248388 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1c28>
  24836e:	jmp    2483ae <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1c4e>
  248370:	mov    %rax,%r14
  248373:	test   %rbx,%rbx
  248376:	je     248388 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1c28>
  248378:	mov    $0x4,%edx
  24837d:	mov    %rbp,%rdi
  248380:	mov    %r13,%rsi
  248383:	call   e9120 <__rustc::__rust_dealloc>
  248388:	cmpq   $0x0,0x40(%rsp)
  24838e:	je     24842a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1cca>
  248394:	mov    0x40(%rsp),%rsi
  248399:	shl    $0x3,%rsi
  24839d:	mov    $0x8,%edx
  2483a2:	mov    %r12,%rdi
  2483a5:	call   e9120 <__rustc::__rust_dealloc>
  2483aa:	jmp    24842a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1cca>
  2483ac:	jmp    2483ae <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1c4e>
  2483ae:	mov    %rax,%r14
  2483b1:	mov    $0x1,%bl
  2483b3:	jmp    24847a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d1a>
  2483b8:	mov    %rax,%r14
  2483bb:	jmp    248408 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ca8>
  2483bd:	mov    %rax,%r14
  2483c0:	lea    0x130(%rsp),%rdi
  2483c8:	call   1f9d00 <core::ptr::drop_glue::<rayon::vec::Drain<(usize, usize)>>>
  2483cd:	mov    0x20(%rsp),%rsi
  2483d2:	test   %rsi,%rsi
  2483d5:	je     248408 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ca8>
  2483d7:	mov    0x28(%rsp),%rdi
  2483dc:	shl    $0x4,%rsi
  2483e0:	mov    $0x8,%edx
  2483e5:	jmp    248403 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ca3>
  2483e7:	call   f2163 <core::panicking::panic_in_cleanup>
  2483ec:	mov    %rax,%r14
  2483ef:	test   %rbx,%rbx
  2483f2:	je     248408 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ca8>
  2483f4:	shl    $0x4,%rbx
  2483f8:	mov    $0x8,%edx
  2483fd:	mov    %rbp,%rdi
  248400:	mov    %rbx,%rsi
  248403:	call   e9120 <__rustc::__rust_dealloc>
  248408:	mov    0x48(%rsp),%rsi
  24840d:	test   %rsi,%rsi
  248410:	je     248478 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d18>
  248412:	mov    0x50(%rsp),%rdi
  248417:	shl    $0x7,%rsi
  24841b:	mov    $0x8,%edx
  248420:	call   e9120 <__rustc::__rust_dealloc>
  248425:	jmp    248478 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d18>
  248427:	mov    %rax,%r14
  24842a:	mov    0x8(%rsp),%rdi
  24842f:	mov    0x18(%rsp),%rcx
  248434:	test   %rcx,%rcx
  248437:	je     248478 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d18>
  248439:	cmp    $0x3ff,%rcx
  248440:	jbe    248472 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d12>
  248442:	shl    $0x7,%rcx
  248446:	movabs $0x7ffffffffffffff9,%rax
  248450:	mov    %rcx,%rsi
  248453:	or     $0x1,%rsi
  248457:	cmp    %rax,%rsi
  24845a:	jae    248524 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1dc4>
  248460:	cmpb   $0x1,(%rdi,%rcx,1)
  248464:	jne    24853c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ddc>
  24846a:	call   *0x2a27d8(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  248470:	jmp    248478 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d18>
  248472:	call   *0x2a27d8(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  248478:	xor    %ebx,%ebx
  24847a:	mov    0x70(%rsp),%rax
  24847f:	test   %rax,%rax
  248482:	jne    24848a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d2a>
  248484:	test   %bl,%bl
  248486:	jne    2484d1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d71>
  248488:	jmp    2484de <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d7e>
  24848a:	mov    0x78(%rsp),%rdi
  24848f:	cmp    $0x3fff,%rax
  248495:	jbe    2484c7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d67>
  248497:	shl    $0x3,%rax
  24849b:	movabs $0x7ffffffffffffff9,%rcx
  2484a5:	mov    %rax,%rsi
  2484a8:	or     $0x1,%rsi
  2484ac:	cmp    %rcx,%rsi
  2484af:	jae    248529 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1dc9>
  2484b1:	cmpb   $0x1,(%rdi,%rax,1)
  2484b5:	jne    248541 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1de1>
  2484bb:	call   *0x2a2787(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  2484c1:	test   %bl,%bl
  2484c3:	jne    2484d1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d71>
  2484c5:	jmp    2484de <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d7e>
  2484c7:	call   *0x2a2783(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  2484cd:	test   %bl,%bl
  2484cf:	je     2484de <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d7e>
  2484d1:	mov    0xf0(%rsp),%rax
  2484d9:	test   %rax,%rax
  2484dc:	jne    2484e6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d86>
  2484de:	mov    %r14,%rdi
  2484e1:	call   4e0e10 <_Unwind_Resume@plt>
  2484e6:	mov    0xf8(%rsp),%rdi
  2484ee:	cmp    $0x1fff,%rax
  2484f4:	jbe    24852e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1dce>
  2484f6:	shl    $0x4,%rax
  2484fa:	movabs $0x7ffffffffffffff9,%rcx
  248504:	mov    %rax,%rsi
  248507:	or     $0x1,%rsi
  24850b:	cmp    %rcx,%rsi
  24850e:	jae    24854d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ded>
  248510:	cmpb   $0x1,(%rdi,%rax,1)
  248514:	jne    248552 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1df2>
  248516:	call   *0x2a272c(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  24851c:	mov    %r14,%rdi
  24851f:	call   4e0e10 <_Unwind_Resume@plt>
  248524:	call   110970 <std::process::abort>
  248529:	call   110970 <std::process::abort>
  24852e:	call   *0x2a271c(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  248534:	mov    %r14,%rdi
  248537:	call   4e0e10 <_Unwind_Resume@plt>
  24853c:	jmp    248472 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d12>
  248541:	call   *0x2a2709(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  248547:	test   %bl,%bl
  248549:	je     2484de <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d7e>
  24854b:	jmp    2484d1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1d71>
  24854d:	call   110970 <std::process::abort>
  248552:	call   *0x2a26f8(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  248558:	mov    %r14,%rdi
  24855b:	call   4e0e10 <_Unwind_Resume@plt>
  248560:	mov    %eax,(%r12,%rax,4)
  248564:	inc    %rax
  248567:	cmp    %rax,%r15
  24856a:	jne    248560 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1e00>
  24856c:	jmp    247802 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x10a2>
