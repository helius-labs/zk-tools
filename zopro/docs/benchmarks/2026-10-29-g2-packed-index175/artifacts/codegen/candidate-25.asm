
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001c37d0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>>:
  1c37d0:	push   %rbp
  1c37d1:	push   %r15
  1c37d3:	push   %r14
  1c37d5:	push   %r13
  1c37d7:	push   %r12
  1c37d9:	push   %rbx
  1c37da:	sub    $0x138,%rsp
  1c37e1:	mov    0x170(%rsp),%rbp
  1c37e9:	movq   $0x0,0x50(%rsp)
  1c37f2:	movq   $0x8,0x58(%rsp)
  1c37fb:	mov    %r8,0xa0(%rsp)
  1c3803:	mov    %rcx,0x90(%rsp)
  1c380b:	mov    %rsi,0x98(%rsp)
  1c3813:	movq   $0x0,0x60(%rsp)
  1c381c:	mov    %rdi,0x88(%rsp)
  1c3824:	mov    %rdx,0x28(%rsp)
  1c3829:	test   %rbp,%rbp
  1c382c:	je     1c397c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x1ac>
  1c3832:	lea    0x0(,%rbp,8),%r12
  1c383a:	mov    %r9,%r13
  1c383d:	mov    $0x8,%edi
  1c3842:	mov    %r12,%rsi
  1c3845:	xor    %edx,%edx
  1c3847:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  1c384c:	test   %rax,%rax
  1c384f:	je     1c41c4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x9f4>
  1c3855:	mov    %rbp,0x8(%rsp)
  1c385a:	mov    %rax,0x10(%rsp)
  1c385f:	movq   $0x0,(%rax)
  1c3866:	movq   $0x1,0x18(%rsp)
  1c386f:	cmp    $0x1,%rbp
  1c3873:	je     1c39b6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x1e6>
  1c3879:	mov    0x0(%r13),%r14
  1c387d:	mov    $0x8,%r12d
  1c3883:	mov    $0x1,%ebx
  1c3888:	xor    %r15d,%r15d
  1c388b:	jmp    1c38b7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xe7>
  1c388d:	nopl   (%rax)
  1c3890:	mov    0x10(%rsp),%rax
  1c3895:	mov    (%rsp),%r14
  1c3899:	add    $0x8,%r13
  1c389d:	mov    %r15,(%rax,%rbx,8)
  1c38a1:	inc    %rbx
  1c38a4:	mov    %rbx,0x18(%rsp)
  1c38a9:	cmp    $0x3,%rbp
  1c38ad:	lea    -0x1(%rbp),%rbp
  1c38b1:	jb     1c39e8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x218>
  1c38b7:	mov    0x8(%r13),%rax
  1c38bb:	mov    %rax,(%rsp)
  1c38bf:	sub    %r14,%rax
  1c38c2:	jbe    1c3962 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x192>
  1c38c8:	mov    %rax,%rbx
  1c38cb:	and    $0x7ff,%eax
  1c38d0:	shr    $0xb,%rbx
  1c38d4:	mov    %rbp,0x30(%rsp)
  1c38d9:	mov    %r15,%rbp
  1c38dc:	mov    %r13,0x20(%rsp)
  1c38e1:	cmp    $0x1,%rax
  1c38e5:	mov    %r12,%rax
  1c38e8:	sbb    $0xffffffffffffffff,%rbx
  1c38ec:	shl    $0x4,%rbp
  1c38f0:	or     $0x8,%rbp
  1c38f4:	jmp    1c3920 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x150>
  1c38f6:	cs nopw 0x0(%rax,%rax,1)
  1c3900:	mov    %r14,-0x8(%rax,%rbp,1)
  1c3905:	mov    %r13,(%rax,%rbp,1)
  1c3909:	inc    %r15
  1c390c:	dec    %rbx
  1c390f:	add    $0x10,%rbp
  1c3913:	mov    %r12,%r14
  1c3916:	mov    %r15,0x60(%rsp)
  1c391b:	test   %rbx,%rbx
  1c391e:	je     1c3950 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x180>
  1c3920:	mov    (%rsp),%rcx
  1c3924:	lea    0x800(%r14),%r12
  1c392b:	mov    %r12,%r13
  1c392e:	cmp    %r12,%rcx
  1c3931:	cmovb  %rcx,%r13
  1c3935:	cmp    0x50(%rsp),%r15
  1c393a:	jne    1c3900 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x130>
  1c393c:	lea    0x50(%rsp),%rdi
  1c3941:	call   2b7f10 <<alloc::raw_vec::RawVec<zopro_core::preparation::Reservation>>::grow_one>
  1c3946:	mov    0x58(%rsp),%rax
  1c394b:	jmp    1c3900 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x130>
  1c394d:	nopl   (%rax)
  1c3950:	mov    0x18(%rsp),%rbx
  1c3955:	mov    0x20(%rsp),%r13
  1c395a:	mov    0x30(%rsp),%rbp
  1c395f:	mov    %rax,%r12
  1c3962:	cmp    0x8(%rsp),%rbx
  1c3967:	jne    1c3890 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc0>
  1c396d:	lea    0x8(%rsp),%rdi
  1c3972:	call   f4890 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  1c3977:	jmp    1c3890 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc0>
  1c397c:	movq   $0x0,0x8(%rsp)
  1c3985:	movq   $0x8,0x10(%rsp)
  1c398e:	movq   $0x0,0x18(%rsp)
  1c3997:	lea    0x8(%rsp),%rdi
  1c399c:	call   f4890 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  1c39a1:	mov    0x10(%rsp),%rax
  1c39a6:	movq   $0x0,(%rax)
  1c39ad:	movq   $0x1,0x18(%rsp)
  1c39b6:	movq   $0x0,0x38(%rsp)
  1c39bf:	movq   $0x8,0x40(%rsp)
  1c39c8:	movq   $0x0,0x48(%rsp)
  1c39d1:	movq   $0x0,0x80(%rsp)
  1c39dd:	mov    $0x8,%r12d
  1c39e3:	xor    %r13d,%r13d
  1c39e6:	jmp    1c3a1e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x24e>
  1c39e8:	mov    0x50(%rsp),%r13
  1c39ed:	mov    0x58(%rsp),%r12
  1c39f2:	movq   $0x0,0x38(%rsp)
  1c39fb:	movq   $0x8,0x40(%rsp)
  1c3a04:	movq   $0x0,0x48(%rsp)
  1c3a0d:	mov    %r15,0x80(%rsp)
  1c3a15:	test   %r15,%r15
  1c3a18:	jne    1c4106 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x936>
  1c3a1e:	mov    $0x8,%r14d
  1c3a24:	mov    $0x1,%bl
  1c3a26:	xor    %r15d,%r15d
  1c3a29:	xor    %ebp,%ebp
  1c3a2b:	xor    %eax,%eax
  1c3a2d:	mov    0x90(%rsp),%rdx
  1c3a35:	mov    0xa0(%rsp),%rsi
  1c3a3d:	mov    %r13,0xa8(%rsp)
  1c3a45:	mov    %r12,0xb0(%rsp)
  1c3a4d:	mov    %r15,0xb8(%rsp)
  1c3a55:	mov    0x98(%rsp),%rdi
  1c3a5d:	mov    %rax,0x30(%rsp)
  1c3a62:	shl    $0x6,%rax
  1c3a66:	lea    0xc0(%rsp),%rcx
  1c3a6e:	mov    %r15,0x118(%rsp)
  1c3a76:	mov    %r13,0x68(%rsp)
  1c3a7b:	mov    %r12,0x70(%rsp)
  1c3a80:	movq   $0x0,0x78(%rsp)
  1c3a89:	add    %r14,%rax
  1c3a8c:	mov    %rcx,0x100(%rsp)
  1c3a94:	mov    %rax,0x108(%rsp)
  1c3a9c:	lea    0x68(%rsp),%rax
  1c3aa1:	mov    %r15,0x110(%rsp)
  1c3aa9:	mov    %rax,0xe0(%rsp)
  1c3ab1:	movq   $0x0,0xe8(%rsp)
  1c3abd:	mov    %r15,0xf0(%rsp)
  1c3ac5:	mov    %r15,0xf8(%rsp)
  1c3acd:	mov    %rdx,0xc0(%rsp)
  1c3ad5:	mov    %rsi,0xc8(%rsp)
  1c3add:	mov    0x28(%rsp),%rsi
  1c3ae2:	mov    %rdi,0xd0(%rsp)
  1c3aea:	mov    %rsi,0xd8(%rsp)
  1c3af2:	cmp    %r15,%r13
  1c3af5:	jb     1c4156 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x986>
  1c3afb:	mov    %fs:0xffffffffffffffc0,%rax
  1c3b04:	test   %rax,%rax
  1c3b07:	je     1c3b11 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x341>
  1c3b09:	add    $0x110,%rax
  1c3b0f:	jmp    1c3b16 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x346>
  1c3b11:	call   104a30 <rayon_core::registry::global_registry>
  1c3b16:	mov    (%rax),%rax
  1c3b19:	mov    0x208(%rax),%rcx
  1c3b20:	lea    0x100(%rsp),%rax
  1c3b28:	xor    %edx,%edx
  1c3b2a:	lea    0x120(%rsp),%rdi
  1c3b32:	mov    $0x1,%r8d
  1c3b38:	mov    %r15,%rsi
  1c3b3b:	mov    %r12,%r9
  1c3b3e:	push   %rax
  1c3b3f:	push   %r15
  1c3b41:	call   1f9da0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>>
  1c3b46:	add    $0x10,%rsp
  1c3b4a:	cmp    %r15,0x78(%rsp)
  1c3b4f:	sete   %al
  1c3b52:	or     %al,%bl
  1c3b54:	je     1c3b5f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x38f>
  1c3b56:	movq   $0x0,0x78(%rsp)
  1c3b5f:	mov    0x68(%rsp),%rax
  1c3b64:	mov    %rbp,0x20(%rsp)
  1c3b69:	test   %rax,%rax
  1c3b6c:	je     1c3baf <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3df>
  1c3b6e:	mov    0x70(%rsp),%rdi
  1c3b73:	cmp    $0x1fff,%rax
  1c3b79:	jbe    1c3ba7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3d7>
  1c3b7b:	shl    $0x4,%rax
  1c3b7f:	movabs $0x7ffffffffffffff9,%rcx
  1c3b89:	mov    %rax,%rsi
  1c3b8c:	or     $0x1,%rsi
  1c3b90:	cmp    %rcx,%rsi
  1c3b93:	jae    1c41d1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa01>
  1c3b99:	cmpb   $0x1,(%rdi,%rax,1)
  1c3b9d:	jne    1c3ba9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3d9>
  1c3b9f:	call   *0x3270a3(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1c3ba5:	jmp    1c3baf <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3df>
  1c3ba7:	jmp    1c3ba9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3d9>
  1c3ba9:	call   *0x3270a1(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c3baf:	mov    0x130(%rsp),%rax
  1c3bb7:	mov    %rax,0xe0(%rsp)
  1c3bbf:	cmp    %r15,%rax
  1c3bc2:	jne    1c4170 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x9a0>
  1c3bc8:	mov    0x18(%rsp),%r12
  1c3bcd:	mov    %r14,(%rsp)
  1c3bd1:	xor    %r14d,%r14d
  1c3bd4:	movabs $0x7ffffffffffffff9,%rcx
  1c3bde:	dec    %rcx
  1c3be1:	lea    -0x1(%r12),%r10
  1c3be6:	cmp    $0x2,%r12
  1c3bea:	mov    %r10,%r8
  1c3bed:	cmovb  %r14,%r8
  1c3bf1:	lea    0x0(,%r8,8),%rbx
  1c3bf9:	mov    %r8,%rax
  1c3bfc:	shr    $0x3d,%rax
  1c3c00:	setne  %al
  1c3c03:	cmp    %rcx,%rbx
  1c3c06:	seta   %cl
  1c3c09:	or     %al,%cl
  1c3c0b:	je     1c3c1d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x44d>
  1c3c0d:	mov    %r14,%rdi
  1c3c10:	mov    %rbx,%rsi
  1c3c13:	call   ede21 <alloc::raw_vec::handle_error>
  1c3c18:	jmp    1c41e3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa13>
  1c3c1d:	mov    0x10(%rsp),%rbp
  1c3c22:	test   %rbx,%rbx
  1c3c25:	je     1c3c51 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x481>
  1c3c27:	mov    $0x8,%edi
  1c3c2c:	mov    $0x8,%r14d
  1c3c32:	mov    %rbx,%rsi
  1c3c35:	xor    %edx,%edx
  1c3c37:	mov    %r8,%r13
  1c3c3a:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  1c3c3f:	lea    -0x1(%r12),%r10
  1c3c44:	mov    %r13,%rcx
  1c3c47:	mov    %rax,%r13
  1c3c4a:	test   %rax,%rax
  1c3c4d:	jne    1c3c59 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x489>
  1c3c4f:	jmp    1c3c0d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x43d>
  1c3c51:	mov    $0x8,%r13d
  1c3c57:	xor    %ecx,%ecx
  1c3c59:	mov    0x30(%rsp),%rdx
  1c3c5e:	mov    %rcx,0x28(%rsp)
  1c3c63:	add    %r15,%rdx
  1c3c66:	cmp    $0x2,%r12
  1c3c6a:	jae    1c3cb9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x4e9>
  1c3c6c:	xor    %r10d,%r10d
  1c3c6f:	mov    %edx,%ebx
  1c3c71:	lea    0x0(,%rbx,4),%r14
  1c3c79:	test   %rbx,%rbx
  1c3c7c:	je     1c3f8c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x7bc>
  1c3c82:	mov    %r10,%r12
  1c3c85:	mov    %rdx,%r15
  1c3c88:	mov    $0x4,%edi
  1c3c8d:	mov    %r14,%rsi
  1c3c90:	xor    %edx,%edx
  1c3c92:	vzeroupper
  1c3c95:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  1c3c9a:	test   %rax,%rax
  1c3c9d:	je     1c41d6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa06>
  1c3ca3:	mov    %rax,%rbp
  1c3ca6:	mov    %r12,%r10
  1c3ca9:	cmp    $0x8,%ebx
  1c3cac:	jae    1c3dd1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x601>
  1c3cb2:	xor    %eax,%eax
  1c3cb4:	jmp    1c3eb0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6e0>
  1c3cb9:	mov    0x0(%rbp),%rcx
  1c3cbd:	mov    %rdx,%r9
  1c3cc0:	cmp    $0x21,%r12
  1c3cc4:	jb     1c3cdc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x50c>
  1c3cc6:	mov    %r13,%rax
  1c3cc9:	sub    %rbp,%rax
  1c3ccc:	add    $0xfffffffffffffff8,%rax
  1c3cd0:	cmp    $0x100,%rax
  1c3cd6:	jae    1c3ec4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6f4>
  1c3cdc:	xor    %eax,%eax
  1c3cde:	mov    %r12,%rdx
  1c3ce1:	sub    %rax,%rdx
  1c3ce4:	mov    %r12d,%esi
  1c3ce7:	and    $0x7,%esi
  1c3cea:	add    $0xfffffffffffffffe,%rdx
  1c3cee:	cmp    $0x1,%esi
  1c3cf1:	je     1c3d1e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x54e>
  1c3cf3:	lea    -0x1(%r12),%esi
  1c3cf8:	and    $0x7,%esi
  1c3cfb:	nopl   0x0(%rax,%rax,1)
  1c3d00:	mov    %rcx,%rdi
  1c3d03:	mov    0x8(%rbp),%rcx
  1c3d07:	add    $0x8,%rbp
  1c3d0b:	mov    %rcx,%r8
  1c3d0e:	sub    %rdi,%r8
  1c3d11:	mov    %r8,0x0(%r13,%rax,8)
  1c3d16:	inc    %rax
  1c3d19:	dec    %rsi
  1c3d1c:	jne    1c3d00 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x530>
  1c3d1e:	cmp    $0x7,%rdx
  1c3d22:	jb     1c3f76 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x7a6>
  1c3d28:	lea    0x38(%r13,%rax,8),%rdx
  1c3d2d:	not    %rax
  1c3d30:	xor    %esi,%esi
  1c3d32:	add    %r12,%rax
  1c3d35:	data16 cs nopw 0x0(%rax,%rax,1)
  1c3d40:	mov    0x8(%rbp,%rsi,8),%rdi
  1c3d45:	mov    %rdi,%r8
  1c3d48:	sub    %rcx,%r8
  1c3d4b:	mov    %r8,-0x38(%rdx,%rsi,8)
  1c3d50:	mov    0x10(%rbp,%rsi,8),%rcx
  1c3d55:	mov    %rcx,%r8
  1c3d58:	sub    %rdi,%r8
  1c3d5b:	mov    %r8,-0x30(%rdx,%rsi,8)
  1c3d60:	mov    0x18(%rbp,%rsi,8),%rdi
  1c3d65:	mov    %rdi,%r8
  1c3d68:	sub    %rcx,%r8
  1c3d6b:	mov    %r8,-0x28(%rdx,%rsi,8)
  1c3d70:	mov    0x20(%rbp,%rsi,8),%rcx
  1c3d75:	mov    %rcx,%r8
  1c3d78:	sub    %rdi,%r8
  1c3d7b:	mov    %r8,-0x20(%rdx,%rsi,8)
  1c3d80:	mov    0x28(%rbp,%rsi,8),%rdi
  1c3d85:	mov    %rdi,%r8
  1c3d88:	sub    %rcx,%r8
  1c3d8b:	mov    %r8,-0x18(%rdx,%rsi,8)
  1c3d90:	mov    0x30(%rbp,%rsi,8),%rcx
  1c3d95:	mov    %rcx,%r8
  1c3d98:	sub    %rdi,%r8
  1c3d9b:	mov    %r8,-0x10(%rdx,%rsi,8)
  1c3da0:	mov    0x38(%rbp,%rsi,8),%rdi
  1c3da5:	mov    %rdi,%r8
  1c3da8:	sub    %rcx,%r8
  1c3dab:	mov    %r8,-0x8(%rdx,%rsi,8)
  1c3db0:	mov    0x40(%rbp,%rsi,8),%rcx
  1c3db5:	mov    %rcx,%r8
  1c3db8:	sub    %rdi,%r8
  1c3dbb:	mov    %r8,(%rdx,%rsi,8)
  1c3dbf:	add    $0x8,%rsi
  1c3dc3:	cmp    %rsi,%rax
  1c3dc6:	jne    1c3d40 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x570>
  1c3dcc:	jmp    1c3f76 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x7a6>
  1c3dd1:	cmp    $0x40,%ebx
  1c3dd4:	jae    1c3ddd <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x60d>
  1c3dd6:	xor    %eax,%eax
  1c3dd8:	jmp    1c3e6b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x69b>
  1c3ddd:	vmovdqa64 -0x1b7ae7(%rip),%zmm0        # c300 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x7c0>
  1c3de7:	vpbroadcastd -0x1b7f71(%rip),%zmm1        # be80 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x340>
  1c3df1:	vpbroadcastd -0x1b7f57(%rip),%zmm2        # bea4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x364>
  1c3dfb:	vpbroadcastd -0x1b7f4d(%rip),%zmm3        # beb8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x378>
  1c3e05:	vpbroadcastd -0x1b7f4b(%rip),%zmm4        # bec4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x384>
  1c3e0f:	mov    %r15d,%eax
  1c3e12:	and    $0xffffffc0,%eax
  1c3e15:	xor    %ecx,%ecx
  1c3e17:	nopw   0x0(%rax,%rax,1)
  1c3e20:	vpaddd %zmm1,%zmm0,%zmm5
  1c3e26:	vpaddd %zmm2,%zmm0,%zmm6
  1c3e2c:	vpaddd %zmm3,%zmm0,%zmm7
  1c3e32:	vmovdqu64 %zmm0,0x0(%rbp,%rcx,4)
  1c3e3a:	vpaddd %zmm4,%zmm0,%zmm0
  1c3e40:	vmovdqu64 %zmm5,0x40(%rbp,%rcx,4)
  1c3e48:	vmovdqu64 %zmm6,0x80(%rbp,%rcx,4)
  1c3e50:	vmovdqu64 %zmm7,0xc0(%rbp,%rcx,4)
  1c3e58:	add    $0x40,%rcx
  1c3e5c:	cmp    %rcx,%rax
  1c3e5f:	jne    1c3e20 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x650>
  1c3e61:	cmp    %eax,%ebx
  1c3e63:	je     1c3ebc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6ec>
  1c3e65:	test   $0x38,%r15b
  1c3e69:	je     1c3eb0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6e0>
  1c3e6b:	mov    %rax,%rcx
  1c3e6e:	vpbroadcastd %ecx,%ymm0
  1c3e74:	vpor   -0x1b253c(%rip),%ymm0,%ymm0        # 11940 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5e00>
  1c3e7c:	vpbroadcastd -0x1b7fdd(%rip),%ymm1        # bea8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x368>
  1c3e85:	mov    %r15d,%eax
  1c3e88:	and    $0xfffffff8,%eax
  1c3e8b:	nopl   0x0(%rax,%rax,1)
  1c3e90:	vmovdqu %ymm0,0x0(%rbp,%rcx,4)
  1c3e96:	add    $0x8,%rcx
  1c3e9a:	vpaddd %ymm1,%ymm0,%ymm0
  1c3e9e:	cmp    %rcx,%rax
  1c3ea1:	jne    1c3e90 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6c0>
  1c3ea3:	cmp    %eax,%ebx
  1c3ea5:	je     1c3ebc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6ec>
  1c3ea7:	nopw   0x0(%rax,%rax,1)
  1c3eb0:	mov    %eax,0x0(%rbp,%rax,4)
  1c3eb4:	inc    %rax
  1c3eb7:	cmp    %rax,%rbx
  1c3eba:	jne    1c3eb0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6e0>
  1c3ebc:	mov    %r15,%rdx
  1c3ebf:	jmp    1c3f91 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x7c1>
  1c3ec4:	mov    %r10,%rax
  1c3ec7:	and    $0xffffffffffffffe0,%rax
  1c3ecb:	vpbroadcastq %rcx,%zmm0
  1c3ed1:	xor    %ecx,%ecx
  1c3ed3:	lea    0x0(%rbp,%rax,8),%rdx
  1c3ed8:	nopl   0x0(%rax,%rax,1)
  1c3ee0:	vmovdqu64 0x8(%rbp,%rcx,8),%zmm1
  1c3eeb:	vmovdqu64 0x48(%rbp,%rcx,8),%zmm2
  1c3ef6:	vmovdqu64 0x88(%rbp,%rcx,8),%zmm3
  1c3f01:	valignq $0x7,%zmm0,%zmm1,%zmm4
  1c3f08:	vmovdqu64 0xc8(%rbp,%rcx,8),%zmm0
  1c3f13:	valignq $0x7,%zmm1,%zmm2,%zmm5
  1c3f1a:	valignq $0x7,%zmm2,%zmm3,%zmm6
  1c3f21:	vpsubq %zmm4,%zmm1,%zmm1
  1c3f27:	vpsubq %zmm5,%zmm2,%zmm2
  1c3f2d:	vmovdqu64 %zmm1,0x0(%r13,%rcx,8)
  1c3f35:	vmovdqu64 %zmm2,0x40(%r13,%rcx,8)
  1c3f3d:	valignq $0x7,%zmm3,%zmm0,%zmm7
  1c3f44:	vpsubq %zmm6,%zmm3,%zmm3
  1c3f4a:	vmovdqu64 %zmm3,0x80(%r13,%rcx,8)
  1c3f52:	vpsubq %zmm7,%zmm0,%zmm4
  1c3f58:	vmovdqu64 %zmm4,0xc0(%r13,%rcx,8)
  1c3f60:	add    $0x20,%rcx
  1c3f64:	cmp    %rcx,%rax
  1c3f67:	jne    1c3ee0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x710>
  1c3f6d:	cmp    %rax,%r10
  1c3f70:	jne    1c40f1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x921>
  1c3f76:	mov    %r9,%rdx
  1c3f79:	mov    %edx,%ebx
  1c3f7b:	lea    0x0(,%rbx,4),%r14
  1c3f83:	test   %rbx,%rbx
  1c3f86:	jne    1c3c82 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x4b2>
  1c3f8c:	mov    $0x4,%ebp
  1c3f91:	mov    0x10(%rsp),%r9
  1c3f96:	sub    $0x8,%rsp
  1c3f9a:	mov    0x90(%rsp),%rdi
  1c3fa2:	mov    0x8(%rsp),%rsi
  1c3fa7:	mov    %rbp,%rcx
  1c3faa:	mov    %rbx,%r8
  1c3fad:	push   %r10
  1c3faf:	push   %r13
  1c3fb1:	push   0x30(%rsp)
  1c3fb5:	vzeroupper
  1c3fb8:	call   1fa8a0 <zopro_core::msm::group_affine::<zopro_bn254::curve::G1Config>>
  1c3fbd:	add    $0x20,%rsp
  1c3fc1:	test   %rbx,%rbx
  1c3fc4:	je     1c3ff3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x823>
  1c3fc6:	cmp    $0x7fff,%ebx
  1c3fcc:	jbe    1c3fe8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x818>
  1c3fce:	cmpb   $0x1,0x0(%rbp,%r14,1)
  1c3fd4:	jne    1c3fea <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x81a>
  1c3fd6:	or     $0x1,%r14
  1c3fda:	mov    %rbp,%rdi
  1c3fdd:	mov    %r14,%rsi
  1c3fe0:	call   *0x326c62(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1c3fe6:	jmp    1c3ff3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x823>
  1c3fe8:	jmp    1c3fea <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x81a>
  1c3fea:	mov    %rbp,%rdi
  1c3fed:	call   *0x326c5d(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c3ff3:	mov    0x28(%rsp),%rcx
  1c3ff8:	mov    (%rsp),%rbx
  1c3ffc:	mov    0x20(%rsp),%r14
  1c4001:	test   %rcx,%rcx
  1c4004:	je     1c404b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x87b>
  1c4006:	cmp    $0x3fff,%rcx
  1c400d:	jbe    1c4040 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x870>
  1c400f:	shl    $0x3,%rcx
  1c4013:	movabs $0x7ffffffffffffff9,%rax
  1c401d:	mov    %rcx,%rsi
  1c4020:	or     $0x1,%rsi
  1c4024:	cmp    %rax,%rsi
  1c4027:	jae    1c41e5 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa15>
  1c402d:	cmpb   $0x1,0x0(%r13,%rcx,1)
  1c4033:	jne    1c4042 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x872>
  1c4035:	mov    %r13,%rdi
  1c4038:	call   *0x326c0a(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1c403e:	jmp    1c404b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x87b>
  1c4040:	jmp    1c4042 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x872>
  1c4042:	mov    %r13,%rdi
  1c4045:	call   *0x326c05(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c404b:	test   %r14,%r14
  1c404e:	je     1c4094 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8c4>
  1c4050:	cmp    $0x7ff,%r14
  1c4057:	jbe    1c4089 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8b9>
  1c4059:	shl    $0x6,%r14
  1c405d:	movabs $0x7ffffffffffffff9,%rax
  1c4067:	mov    %r14,%rsi
  1c406a:	or     $0x1,%rsi
  1c406e:	cmp    %rax,%rsi
  1c4071:	jae    1c41ea <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa1a>
  1c4077:	cmpb   $0x1,(%rbx,%r14,1)
  1c407c:	jne    1c408b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8bb>
  1c407e:	mov    %rbx,%rdi
  1c4081:	call   *0x326bc1(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1c4087:	jmp    1c4094 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8c4>
  1c4089:	jmp    1c408b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8bb>
  1c408b:	mov    %rbx,%rdi
  1c408e:	call   *0x326bbc(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c4094:	mov    0x8(%rsp),%rax
  1c4099:	test   %rax,%rax
  1c409c:	je     1c40df <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x90f>
  1c409e:	mov    0x10(%rsp),%rdi
  1c40a3:	cmp    $0x3fff,%rax
  1c40a9:	jbe    1c40d7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x907>
  1c40ab:	shl    $0x3,%rax
  1c40af:	movabs $0x7ffffffffffffff9,%rcx
  1c40b9:	mov    %rax,%rsi
  1c40bc:	or     $0x1,%rsi
  1c40c0:	cmp    %rcx,%rsi
  1c40c3:	jae    1c41ef <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa1f>
  1c40c9:	cmpb   $0x1,(%rdi,%rax,1)
  1c40cd:	jne    1c40d9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x909>
  1c40cf:	call   *0x326b73(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1c40d5:	jmp    1c40df <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x90f>
  1c40d7:	jmp    1c40d9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x909>
  1c40d9:	call   *0x326b71(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c40df:	add    $0x138,%rsp
  1c40e6:	pop    %rbx
  1c40e7:	pop    %r12
  1c40e9:	pop    %r13
  1c40eb:	pop    %r14
  1c40ed:	pop    %r15
  1c40ef:	pop    %rbp
  1c40f0:	ret
  1c40f1:	vextracti32x4 $0x3,%zmm0,%xmm0
  1c40f8:	mov    %rdx,%rbp
  1c40fb:	vpextrq $0x1,%xmm0,%rcx
  1c4101:	jmp    1c3cde <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x50e>
  1c4106:	lea    0x38(%rsp),%rdi
  1c410b:	mov    $0x8,%ecx
  1c4110:	mov    $0x40,%r8d
  1c4116:	xor    %esi,%esi
  1c4118:	mov    %r15,%rdx
  1c411b:	call   1b6760 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  1c4120:	mov    0x38(%rsp),%rbp
  1c4125:	mov    0x48(%rsp),%rcx
  1c412a:	mov    %rbp,%rax
  1c412d:	sub    %rcx,%rax
  1c4130:	cmp    %r15,%rax
  1c4133:	jae    1c41f4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa24>
  1c4139:	lea    -0x1a4583(%rip),%rdi        # 1fbbd <zmij::STATIC_DATA+0x55bd>
  1c4140:	lea    0x324031(%rip),%rdx        # 4e8178 <__frame_dummy_init_array_entry+0x6318>
  1c4147:	mov    $0x2f,%esi
  1c414c:	call   f0cd0 <core::panicking::panic>
  1c4151:	jmp    1c41e3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa13>
  1c4156:	lea    -0x1a45a0(%rip),%rdi        # 1fbbd <zmij::STATIC_DATA+0x55bd>
  1c415d:	lea    0x3244ac(%rip),%rdx        # 4e8610 <__frame_dummy_init_array_entry+0x67b0>
  1c4164:	mov    $0x2f,%esi
  1c4169:	call   f0cd0 <core::panicking::panic>
  1c416e:	jmp    1c41e3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa13>
  1c4170:	lea    0x80(%rsp),%rax
  1c4178:	lea    -0xd0f2f(%rip),%r8        # f3250 <<u64 as core::fmt::Display>::fmt>
  1c417f:	lea    0xe0(%rsp),%rcx
  1c4187:	mov    %rax,0xa8(%rsp)
  1c418f:	mov    %r8,0xb0(%rsp)
  1c4197:	mov    %rcx,0xb8(%rsp)
  1c419f:	mov    %r8,0xc0(%rsp)
  1c41a7:	lea    -0x1ad269(%rip),%rdi        # 16f45 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x5345>
  1c41ae:	lea    0x322023(%rip),%rdx        # 4e61d8 <__frame_dummy_init_array_entry+0x4378>
  1c41b5:	lea    0xa8(%rsp),%rsi
  1c41bd:	call   eec00 <core::panicking::panic_fmt>
  1c41c2:	jmp    1c41e3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa13>
  1c41c4:	mov    $0x8,%edi
  1c41c9:	mov    %r12,%rsi
  1c41cc:	call   ede21 <alloc::raw_vec::handle_error>
  1c41d1:	call   110970 <std::process::abort>
  1c41d6:	mov    $0x4,%edi
  1c41db:	mov    %r14,%rsi
  1c41de:	call   ede21 <alloc::raw_vec::handle_error>
  1c41e3:	ud2
  1c41e5:	call   110970 <std::process::abort>
  1c41ea:	call   110970 <std::process::abort>
  1c41ef:	call   110970 <std::process::abort>
  1c41f4:	mov    0x40(%rsp),%r14
  1c41f9:	xor    %ebx,%ebx
  1c41fb:	mov    %rcx,%rax
  1c41fe:	jmp    1c3a2d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x25d>
  1c4203:	jmp    1c4259 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa89>
  1c4205:	mov    %rax,%r15
  1c4208:	jmp    1c4224 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa54>
  1c420a:	jmp    1c424a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa7a>
  1c420c:	mov    %rax,%r15
  1c420f:	test   %rbx,%rbx
  1c4212:	je     1c4224 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa54>
  1c4214:	mov    $0x4,%edx
  1c4219:	mov    %rbp,%rdi
  1c421c:	mov    %r14,%rsi
  1c421f:	call   e9120 <__rustc::__rust_dealloc>
  1c4224:	cmpq   $0x0,0x28(%rsp)
  1c422a:	je     1c42c6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xaf6>
  1c4230:	mov    0x28(%rsp),%rsi
  1c4235:	shl    $0x3,%rsi
  1c4239:	mov    $0x8,%edx
  1c423e:	mov    %r13,%rdi
  1c4241:	call   e9120 <__rustc::__rust_dealloc>
  1c4246:	jmp    1c42c6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xaf6>
  1c4248:	jmp    1c424a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa7a>
  1c424a:	mov    %rax,%r15
  1c424d:	mov    $0x1,%bl
  1c424f:	jmp    1c4315 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb45>
  1c4254:	mov    %rax,%r15
  1c4257:	jmp    1c429f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xacf>
  1c4259:	mov    %rax,%r15
  1c425c:	test   %r13,%r13
  1c425f:	je     1c429f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xacf>
  1c4261:	shl    $0x4,%r13
  1c4265:	mov    $0x8,%edx
  1c426a:	mov    %r12,%rdi
  1c426d:	mov    %r13,%rsi
  1c4270:	jmp    1c429a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xaca>
  1c4272:	mov    %rax,%r15
  1c4275:	lea    0xe0(%rsp),%rdi
  1c427d:	call   1f9d00 <core::ptr::drop_glue::<rayon::vec::Drain<(usize, usize)>>>
  1c4282:	mov    0x68(%rsp),%rsi
  1c4287:	test   %rsi,%rsi
  1c428a:	je     1c429f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xacf>
  1c428c:	mov    0x70(%rsp),%rdi
  1c4291:	shl    $0x4,%rsi
  1c4295:	mov    $0x8,%edx
  1c429a:	call   e9120 <__rustc::__rust_dealloc>
  1c429f:	mov    0x38(%rsp),%rsi
  1c42a4:	test   %rsi,%rsi
  1c42a7:	je     1c4313 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb43>
  1c42a9:	mov    0x40(%rsp),%rdi
  1c42ae:	shl    $0x6,%rsi
  1c42b2:	mov    $0x8,%edx
  1c42b7:	call   e9120 <__rustc::__rust_dealloc>
  1c42bc:	jmp    1c4313 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb43>
  1c42be:	call   f2163 <core::panicking::panic_in_cleanup>
  1c42c3:	mov    %rax,%r15
  1c42c6:	mov    (%rsp),%rdi
  1c42ca:	mov    0x20(%rsp),%rcx
  1c42cf:	test   %rcx,%rcx
  1c42d2:	je     1c4313 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb43>
  1c42d4:	cmp    $0x7ff,%rcx
  1c42db:	jbe    1c430d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb3d>
  1c42dd:	shl    $0x6,%rcx
  1c42e1:	movabs $0x7ffffffffffffff9,%rax
  1c42eb:	mov    %rcx,%rsi
  1c42ee:	or     $0x1,%rsi
  1c42f2:	cmp    %rax,%rsi
  1c42f5:	jae    1c43b9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xbe9>
  1c42fb:	cmpb   $0x1,(%rdi,%rcx,1)
  1c42ff:	jne    1c43d1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc01>
  1c4305:	call   *0x32693d(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1c430b:	jmp    1c4313 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb43>
  1c430d:	call   *0x32693d(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c4313:	xor    %ebx,%ebx
  1c4315:	mov    0x8(%rsp),%rax
  1c431a:	test   %rax,%rax
  1c431d:	jne    1c4325 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb55>
  1c431f:	test   %bl,%bl
  1c4321:	jne    1c436c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb9c>
  1c4323:	jmp    1c4376 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xba6>
  1c4325:	mov    0x10(%rsp),%rdi
  1c432a:	cmp    $0x3fff,%rax
  1c4330:	jbe    1c4362 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb92>
  1c4332:	shl    $0x3,%rax
  1c4336:	movabs $0x7ffffffffffffff9,%rcx
  1c4340:	mov    %rax,%rsi
  1c4343:	or     $0x1,%rsi
  1c4347:	cmp    %rcx,%rsi
  1c434a:	jae    1c43be <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xbee>
  1c434c:	cmpb   $0x1,(%rdi,%rax,1)
  1c4350:	jne    1c43d6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc06>
  1c4356:	call   *0x3268ec(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1c435c:	test   %bl,%bl
  1c435e:	jne    1c436c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb9c>
  1c4360:	jmp    1c4376 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xba6>
  1c4362:	call   *0x3268e8(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c4368:	test   %bl,%bl
  1c436a:	je     1c4376 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xba6>
  1c436c:	mov    0x50(%rsp),%rax
  1c4371:	test   %rax,%rax
  1c4374:	jne    1c437e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xbae>
  1c4376:	mov    %r15,%rdi
  1c4379:	call   4e0e10 <_Unwind_Resume@plt>
  1c437e:	mov    0x58(%rsp),%rdi
  1c4383:	cmp    $0x1fff,%rax
  1c4389:	jbe    1c43c3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xbf3>
  1c438b:	shl    $0x4,%rax
  1c438f:	movabs $0x7ffffffffffffff9,%rcx
  1c4399:	mov    %rax,%rsi
  1c439c:	or     $0x1,%rsi
  1c43a0:	cmp    %rcx,%rsi
  1c43a3:	jae    1c43e2 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc12>
  1c43a5:	cmpb   $0x1,(%rdi,%rax,1)
  1c43a9:	jne    1c43e7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc17>
  1c43ab:	call   *0x326897(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1c43b1:	mov    %r15,%rdi
  1c43b4:	call   4e0e10 <_Unwind_Resume@plt>
  1c43b9:	call   110970 <std::process::abort>
  1c43be:	call   110970 <std::process::abort>
  1c43c3:	call   *0x326887(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c43c9:	mov    %r15,%rdi
  1c43cc:	call   4e0e10 <_Unwind_Resume@plt>
  1c43d1:	jmp    1c430d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb3d>
  1c43d6:	call   *0x326874(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c43dc:	test   %bl,%bl
  1c43de:	jne    1c436c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb9c>
  1c43e0:	jmp    1c4376 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xba6>
  1c43e2:	call   110970 <std::process::abort>
  1c43e7:	call   *0x326863(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1c43ed:	mov    %r15,%rdi
  1c43f0:	call   4e0e10 <_Unwind_Resume@plt>
