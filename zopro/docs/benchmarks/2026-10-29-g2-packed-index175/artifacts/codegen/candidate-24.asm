
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001f9da0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>>:
  1f9da0:	push   %rbp
  1f9da1:	push   %r15
  1f9da3:	push   %r14
  1f9da5:	push   %r13
  1f9da7:	push   %r12
  1f9da9:	push   %rbx
  1f9daa:	sub    $0x3a8,%rsp
  1f9db1:	mov    0x3e8(%rsp),%rbx
  1f9db9:	mov    0x3e0(%rsp),%r13
  1f9dc1:	mov    %rsi,0x28(%rsp)
  1f9dc6:	shr    $1,%rsi
  1f9dc9:	mov    %rcx,0x30(%rsp)
  1f9dce:	mov    %r9,%r15
  1f9dd1:	mov    %rdi,0x58(%rsp)
  1f9dd6:	mov    %r8,0x38(%rsp)
  1f9ddb:	cmp    %r8,%rsi
  1f9dde:	jae    1fa3e3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x643>
  1f9de4:	mov    (%rbx),%rcx
  1f9de7:	mov    0x8(%rbx),%rsi
  1f9deb:	mov    0x10(%rbx),%rdx
  1f9def:	movabs $0x7fffffffffffffc1,%rax
  1f9df9:	shl    $0x4,%r13
  1f9dfd:	xor    %ebx,%ebx
  1f9dff:	add    %r15,%r13
  1f9e02:	add    $0x37,%rax
  1f9e06:	mov    %rax,0xc8(%rsp)
  1f9e0e:	mov    %r13,0x40(%rsp)
  1f9e13:	mov    %rcx,0x60(%rsp)
  1f9e18:	movabs $0xffffffffffffffd,%rcx
  1f9e22:	mov    %rsi,0x48(%rsp)
  1f9e27:	mov    %rdx,0x50(%rsp)
  1f9e2c:	cmp    %r13,%r15
  1f9e2f:	je     1fa654 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8b4>
  1f9e35:	mov    0x60(%rsp),%rax
  1f9e3a:	mov    (%r15),%r14
  1f9e3d:	mov    0x8(%r15),%r12
  1f9e41:	mov    0x8(%rax),%rdx
  1f9e45:	cmp    %r14,%r12
  1f9e48:	jb     1fa670 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8d0>
  1f9e4e:	cmp    %rdx,%r12
  1f9e51:	ja     1fa670 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8d0>
  1f9e57:	shl    $0x2,%r12
  1f9e5b:	shl    $0x2,%r14
  1f9e5f:	mov    %rbx,0xc0(%rsp)
  1f9e67:	mov    %r12,%rbp
  1f9e6a:	sub    %r14,%rbp
  1f9e6d:	mov    %rbp,%r13
  1f9e70:	shl    $0x4,%r13
  1f9e74:	cmp    %rcx,%rbp
  1f9e77:	setb   %al
  1f9e7a:	cmp    0xc8(%rsp),%r13
  1f9e82:	setbe  %cl
  1f9e85:	test   %cl,%al
  1f9e87:	je     1fa3db <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x63b>
  1f9e8d:	mov    0x60(%rsp),%rax
  1f9e92:	mov    %r15,0x10(%rsp)
  1f9e97:	shr    $0x2,%rbp
  1f9e9b:	mov    (%rax),%r15
  1f9e9e:	mov    0x10(%rax),%rbx
  1f9ea2:	mov    0x18(%rax),%rax
  1f9ea6:	add    %r14,%r15
  1f9ea9:	mov    %rax,0xb8(%rsp)
  1f9eb1:	test   %r13,%r13
  1f9eb4:	je     1f9f55 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x1b5>
  1f9eba:	mov    $0x8,%edi
  1f9ebf:	mov    %r13,%rsi
  1f9ec2:	xor    %edx,%edx
  1f9ec4:	vzeroupper
  1f9ec7:	call   4bc180 <<zopro_core::alloc::linux::Allocator>::allocate>
  1f9ecc:	mov    %rbp,0x20(%rsp)
  1f9ed1:	mov    %rax,0x18(%rsp)
  1f9ed6:	test   %rax,%rax
  1f9ed9:	je     1fa852 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xab2>
  1f9edf:	cmp    %r14,%r12
  1f9ee2:	jne    1f9f71 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x1d1>
  1f9ee8:	mov    0x40(%rsp),%r13
  1f9eed:	xor    %ebp,%ebp
  1f9eef:	mov    %rbp,0x68(%rsp)
  1f9ef4:	mov    $0x1d,%edx
  1f9ef9:	lea    0x70(%rsp),%rdi
  1f9efe:	lea    -0x16f8ce(%rip),%rsi        # 8a637 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x22f>
  1f9f05:	vzeroupper
  1f9f08:	call   143720 <std::sys::env::unix::getenv>
  1f9f0d:	mov    0x70(%rsp),%r15
  1f9f12:	cmp    $0xffffffffffffffff,%r15
  1f9f16:	je     1fa017 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x277>
  1f9f1c:	mov    0x78(%rsp),%rbx
  1f9f21:	mov    0x80(%rsp),%r14
  1f9f29:	lea    0x70(%rsp),%rdi
  1f9f2e:	mov    %rbx,%rsi
  1f9f31:	mov    %r14,%rdx
  1f9f34:	call   f0a00 <core::str::converts::from_utf8>
  1f9f39:	cmpb   $0x0,0x70(%rsp)
  1f9f3e:	je     1fa01c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x27c>
  1f9f44:	mov    $0x1,%r12b
  1f9f47:	test   %r15,%r15
  1f9f4a:	jg     1fa031 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x291>
  1f9f50:	jmp    1fa061 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2c1>
  1f9f55:	mov    $0x8,%eax
  1f9f5a:	movq   $0x0,0x20(%rsp)
  1f9f63:	mov    %rax,0x18(%rsp)
  1f9f68:	cmp    %r14,%r12
  1f9f6b:	je     1f9ee8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x148>
  1f9f71:	mov    0x18(%rsp),%rax
  1f9f76:	mov    0x40(%rsp),%r13
  1f9f7b:	mov    0xb8(%rsp),%rsi
  1f9f83:	lea    0x90(%rsp),%rdx
  1f9f8b:	xor    %ecx,%ecx
  1f9f8d:	jmp    1f9fc7 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x227>
  1f9f8f:	nop
  1f9f90:	add    %rbx,%rdi
  1f9f93:	vmovups (%rdi),%ymm0
  1f9f97:	vmovups %ymm0,0x70(%rsp)
  1f9f9d:	vmovdqu 0x20(%rdi),%ymm0
  1f9fa2:	vmovdqu %ymm0,(%rdx)
  1f9fa6:	vmovdqu64 0x70(%rsp),%zmm0
  1f9fb1:	inc    %rcx
  1f9fb4:	vmovdqu64 %zmm0,(%rax)
  1f9fba:	add    $0x40,%rax
  1f9fbe:	cmp    %rcx,%rbp
  1f9fc1:	je     1f9eef <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x14f>
  1f9fc7:	mov    (%r15,%rcx,4),%edi
  1f9fcb:	cmp    %rdi,%rsi
  1f9fce:	jbe    1fa824 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa84>
  1f9fd4:	shl    $0x6,%rdi
  1f9fd8:	vmovups (%rbx,%rdi,1),%zmm0
  1f9fdf:	vmovups %zmm0,0x110(%rsp)
  1f9fea:	vmovdqu (%rbx,%rdi,1),%ymm0
  1f9fef:	vptest %ymm0,%ymm0
  1f9ff4:	jne    1f9f90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x1f0>
  1f9ff6:	vmovdqu 0x130(%rsp),%ymm0
  1f9fff:	vptest %ymm0,%ymm0
  1fa004:	jne    1f9f90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x1f0>
  1fa006:	vpxor  %xmm0,%xmm0,%xmm0
  1fa00a:	vmovdqu64 %zmm0,0x70(%rsp)
  1fa015:	jmp    1f9fa6 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x206>
  1fa017:	mov    $0x1,%r12b
  1fa01a:	jmp    1fa061 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2c1>
  1fa01c:	mov    $0x1,%r12b
  1fa01f:	cmp    $0x1,%r14
  1fa023:	jne    1fa02c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x28c>
  1fa025:	cmpb   $0x30,(%rbx)
  1fa028:	setne  %r12b
  1fa02c:	test   %r15,%r15
  1fa02f:	je     1fa061 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2c1>
  1fa031:	cmp    $0x1ffff,%r15
  1fa038:	jbe    1fa058 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2b8>
  1fa03a:	mov    %r15,%rsi
  1fa03d:	inc    %rsi
  1fa040:	js     1fa835 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1fa046:	cmpb   $0x1,(%rbx,%r15,1)
  1fa04b:	jne    1fa058 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2b8>
  1fa04d:	mov    %rbx,%rdi
  1fa050:	call   *0x2f0bf2(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1fa056:	jmp    1fa061 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x2c1>
  1fa058:	mov    %rbx,%rdi
  1fa05b:	call   *0x2f0bef(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1fa061:	mov    0x10(%rsp),%r15
  1fa066:	mov    %r12b,0x358(%rsp)
  1fa06e:	lea    0x120(%rsp),%rax
  1fa076:	lea    0x110(%rsp),%rbx
  1fa07e:	lea    -0x1eed6d(%rip),%r14        # b318 <__abi_tag+0xb01c>
  1fa085:	lea    0x68(%rsp),%r12
  1fa08a:	movq   $0x0,0x110(%rsp)
  1fa096:	movq   $0x8,0x118(%rsp)
  1fa0a2:	vpxor  %xmm0,%xmm0,%xmm0
  1fa0a6:	vmovdqu %xmm0,(%rax)
  1fa0aa:	movq   $0x40,0x130(%rsp)
  1fa0b6:	vmovdqu %xmm0,0x18(%rax)
  1fa0bb:	movq   $0x40,0x148(%rsp)
  1fa0c7:	vpxor  %xmm0,%xmm0,%xmm0
  1fa0cb:	vmovdqu64 %zmm0,0x1f0(%rax)
  1fa0d5:	vmovdqu64 %zmm0,0x1b0(%rax)
  1fa0df:	vmovdqu64 %zmm0,0x170(%rax)
  1fa0e9:	vmovdqu64 %zmm0,0x130(%rax)
  1fa0f3:	vmovdqu64 %zmm0,0xf0(%rax)
  1fa0fd:	vmovdqu64 %zmm0,0xb0(%rax)
  1fa107:	vmovdqu64 %zmm0,0x70(%rax)
  1fa111:	vmovdqu64 %zmm0,0x30(%rax)
  1fa11b:	movq   $0x0,0x230(%rax)
  1fa126:	add    $0x10,%r15
  1fa12a:	nopw   0x0(%rax,%rax,1)
  1fa130:	cmpq   $0x1,0x68(%rsp)
  1fa136:	jbe    1fa160 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x3c0>
  1fa138:	mov    0x18(%rsp),%rdi
  1fa13d:	mov    $0x1,%ecx
  1fa142:	mov    $0x1,%r9d
  1fa148:	mov    %rbx,(%rsp)
  1fa14c:	mov    %rbp,%rsi
  1fa14f:	mov    %r14,%rdx
  1fa152:	mov    %r12,%r8
  1fa155:	vzeroupper
  1fa158:	call   1e14e0 <zopro_core::msm::reduce_layer::<zopro_bn254::curve::G1Config>>
  1fa15d:	jmp    1fa130 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x390>
  1fa15f:	nop
  1fa160:	test   %rbp,%rbp
  1fa163:	je     1fa83d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa9d>
  1fa169:	mov    0x18(%rsp),%r14
  1fa16e:	mov    0xc0(%rsp),%rbx
  1fa176:	mov    0x20(%rsp),%r12
  1fa17b:	vmovups (%r14),%zmm0
  1fa181:	vmovups %zmm0,0x70(%rsp)
  1fa18c:	vmovdqu (%r14),%ymm0
  1fa191:	vptest %ymm0,%ymm0
  1fa196:	jne    1fa1ac <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x40c>
  1fa198:	vmovdqu 0x90(%rsp),%ymm0
  1fa1a1:	vptest %ymm0,%ymm0
  1fa1a6:	je     1fa3b6 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x616>
  1fa1ac:	vmovups (%r14),%ymm0
  1fa1b1:	lea    0xf0(%rsp),%rax
  1fa1b9:	vmovups %ymm0,0xd0(%rsp)
  1fa1c2:	vmovdqu 0x20(%r14),%ymm0
  1fa1c8:	vmovdqu %ymm0,(%rax)
  1fa1cc:	mov    0x110(%rsp),%rax
  1fa1d4:	test   %rax,%rax
  1fa1d7:	je     1fa229 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x489>
  1fa1d9:	mov    0x118(%rsp),%rdi
  1fa1e1:	cmp    $0x3fff,%rax
  1fa1e7:	jbe    1fa220 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x480>
  1fa1e9:	movabs $0x7fffffffffffffc1,%rcx
  1fa1f3:	shl    $0x3,%rax
  1fa1f7:	mov    %rax,%rsi
  1fa1fa:	or     $0x1,%rsi
  1fa1fe:	add    $0x38,%rcx
  1fa202:	cmp    %rcx,%rsi
  1fa205:	jae    1fa835 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1fa20b:	cmpb   $0x1,(%rdi,%rax,1)
  1fa20f:	jne    1fa220 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x480>
  1fa211:	vzeroupper
  1fa214:	call   *0x2f0a2e(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1fa21a:	jmp    1fa229 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x489>
  1fa21c:	nopl   0x0(%rax)
  1fa220:	vzeroupper
  1fa223:	call   *0x2f0a27(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1fa229:	mov    0x128(%rsp),%rax
  1fa231:	test   %rax,%rax
  1fa234:	je     1fa280 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x4e0>
  1fa236:	mov    0x130(%rsp),%rdi
  1fa23e:	cmp    $0x51,%rax
  1fa242:	jbe    1fa290 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x4f0>
  1fa244:	imul   $0x640,%rax,%rax
  1fa24b:	movabs $0x7fffffffffffffc1,%rcx
  1fa255:	mov    %rax,%rsi
  1fa258:	or     $0x1,%rsi
  1fa25c:	cmp    %rcx,%rsi
  1fa25f:	jae    1fa835 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1fa265:	cmpb   $0x1,(%rdi,%rax,1)
  1fa269:	jne    1fa290 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x4f0>
  1fa26b:	vzeroupper
  1fa26e:	call   *0x2f09d4(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1fa274:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  1fa280:	mov    0x140(%rsp),%rax
  1fa288:	test   %rax,%rax
  1fa28b:	jne    1fa2a6 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x506>
  1fa28d:	jmp    1fa2f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x550>
  1fa28f:	nop
  1fa290:	vzeroupper
  1fa293:	call   *0x2f09b7(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1fa299:	mov    0x140(%rsp),%rax
  1fa2a1:	test   %rax,%rax
  1fa2a4:	je     1fa2f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x550>
  1fa2a6:	mov    0x148(%rsp),%rdi
  1fa2ae:	cmp    $0xcc,%rax
  1fa2b4:	jbe    1fa300 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x560>
  1fa2b6:	shl    $0x7,%rax
  1fa2ba:	movabs $0x7fffffffffffffc1,%rcx
  1fa2c4:	lea    (%rax,%rax,4),%rax
  1fa2c8:	mov    %rax,%rsi
  1fa2cb:	or     $0x1,%rsi
  1fa2cf:	cmp    %rcx,%rsi
  1fa2d2:	jae    1fa835 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1fa2d8:	cmpb   $0x1,(%rdi,%rax,1)
  1fa2dc:	jne    1fa300 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x560>
  1fa2de:	vzeroupper
  1fa2e1:	call   *0x2f0961(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1fa2e7:	nopw   0x0(%rax,%rax,1)
  1fa2f0:	test   %r12,%r12
  1fa2f3:	jne    1fa30e <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x56e>
  1fa2f5:	jmp    1fa35c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5bc>
  1fa2f7:	nopw   0x0(%rax,%rax,1)
  1fa300:	vzeroupper
  1fa303:	call   *0x2f0947(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1fa309:	test   %r12,%r12
  1fa30c:	je     1fa35c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5bc>
  1fa30e:	cmp    $0x7ff,%r12
  1fa315:	jbe    1fa350 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5b0>
  1fa317:	movabs $0x7fffffffffffffc1,%rax
  1fa321:	shl    $0x6,%r12
  1fa325:	mov    %r12,%rsi
  1fa328:	or     $0x1,%rsi
  1fa32c:	add    $0x38,%rax
  1fa330:	cmp    %rax,%rsi
  1fa333:	jae    1fa835 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xa95>
  1fa339:	cmpb   $0x1,(%r14,%r12,1)
  1fa33e:	jne    1fa350 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5b0>
  1fa340:	mov    %r14,%rdi
  1fa343:	vzeroupper
  1fa346:	call   *0x2f08fc(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  1fa34c:	jmp    1fa35c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x5bc>
  1fa34e:	xchg   %ax,%ax
  1fa350:	mov    %r14,%rdi
  1fa353:	vzeroupper
  1fa356:	call   *0x2f08f4(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  1fa35c:	vmovups 0xd0(%rsp),%zmm0
  1fa367:	vmovups %zmm0,0x360(%rsp)
  1fa372:	cmp    0x50(%rsp),%rbx
  1fa377:	je     1fa685 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8e5>
  1fa37d:	vmovdqu64 0x360(%rsp),%zmm0
  1fa388:	mov    0x48(%rsp),%rcx
  1fa38d:	mov    %rbx,%rax
  1fa390:	shl    $0x6,%rax
  1fa394:	inc    %rbx
  1fa397:	vmovdqu64 %zmm0,(%rcx,%rax,1)
  1fa39e:	movabs $0xffffffffffffffd,%rcx
  1fa3a8:	cmp    %r13,%r15
  1fa3ab:	jne    1f9e35 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x95>
  1fa3b1:	jmp    1fa654 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x8b4>
  1fa3b6:	vpxor  %xmm0,%xmm0,%xmm0
  1fa3ba:	vmovdqu64 %zmm0,0xd0(%rsp)
  1fa3c5:	mov    0x110(%rsp),%rax
  1fa3cd:	test   %rax,%rax
  1fa3d0:	jne    1fa1d9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x439>
  1fa3d6:	jmp    1fa229 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x489>
  1fa3db:	vzeroupper
  1fa3de:	call   edf50 <alloc::raw_vec::capacity_overflow>
  1fa3e3:	mov    %r15,0x10(%rsp)
  1fa3e8:	test   %dl,%dl
  1fa3ea:	je     1fa402 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x662>
  1fa3ec:	mov    %fs:0xffffffffffffffc0,%rax
  1fa3f5:	test   %rax,%rax
  1fa3f8:	je     1fa410 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x670>
  1fa3fa:	add    $0x110,%rax
  1fa400:	jmp    1fa421 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x681>
  1fa402:	test   %rcx,%rcx
  1fa405:	je     1f9de4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x44>
  1fa40b:	shr    $1,%rcx
  1fa40e:	jmp    1fa435 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x695>
  1fa410:	mov    %rsi,%r14
  1fa413:	mov    %rcx,%r15
  1fa416:	call   104a30 <rayon_core::registry::global_registry>
  1fa41b:	mov    %r15,%rcx
  1fa41e:	mov    %r14,%rsi
  1fa421:	mov    (%rax),%rax
  1fa424:	shr    $1,%rcx
  1fa427:	mov    0x208(%rax),%rax
  1fa42e:	cmp    %rax,%rcx
  1fa431:	cmovbe %rax,%rcx
  1fa435:	sub    %rsi,%r13
  1fa438:	mov    %rcx,0x30(%rsp)
  1fa43d:	mov    %rsi,0xd0(%rsp)
  1fa445:	jb     1fa6a0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x900>
  1fa44b:	mov    0x10(%rbx),%r15
  1fa44f:	sub    %rsi,%r15
  1fa452:	jb     1fa6b8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x918>
  1fa458:	mov    (%rbx),%r14
  1fa45b:	mov    0x10(%rsp),%rdi
  1fa460:	mov    0x8(%rbx),%rbx
  1fa464:	mov    %fs:0xffffffffffffffc0,%rdx
  1fa46d:	mov    %rsi,%r12
  1fa470:	shl    $0x6,%r12
  1fa474:	mov    %r13,%rbp
  1fa477:	mov    %rsi,%r13
  1fa47a:	shl    $0x4,%r13
  1fa47e:	add    %rdi,%r13
  1fa481:	add    %rbx,%r12
  1fa484:	test   %rdx,%rdx
  1fa487:	je     1fa51c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x77c>
  1fa48d:	lea    0x28(%rsp),%rax
  1fa492:	lea    0xd0(%rsp),%r8
  1fa49a:	lea    0x30(%rsp),%rcx
  1fa49f:	mov    %rax,0x110(%rsp)
  1fa4a7:	mov    %r8,0x118(%rsp)
  1fa4af:	mov    %rcx,0x120(%rsp)
  1fa4b7:	mov    %r13,0x128(%rsp)
  1fa4bf:	mov    %rbp,0x130(%rsp)
  1fa4c7:	mov    %r14,0x138(%rsp)
  1fa4cf:	mov    %r12,0x140(%rsp)
  1fa4d7:	mov    %r15,0x148(%rsp)
  1fa4df:	mov    %r8,0x150(%rsp)
  1fa4e7:	mov    %rcx,0x158(%rsp)
  1fa4ef:	mov    %rdi,0x160(%rsp)
  1fa4f7:	mov    %rsi,0x168(%rsp)
  1fa4ff:	mov    %r14,0x170(%rsp)
  1fa507:	mov    %rbx,0x178(%rsp)
  1fa50f:	mov    %rsi,0x180(%rsp)
  1fa517:	jmp    1fa5dc <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x83c>
  1fa51c:	mov    %rsi,0x18(%rsp)
  1fa521:	call   104a30 <rayon_core::registry::global_registry>
  1fa526:	mov    (%rax),%rsi
  1fa529:	mov    %fs:0xffffffffffffffc0,%rdx
  1fa532:	test   %rdx,%rdx
  1fa535:	je     1fa6d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x930>
  1fa53b:	mov    0x18(%rsp),%rax
  1fa540:	lea    0x28(%rsp),%rcx
  1fa545:	cmp    %rsi,0x110(%rdx)
  1fa54c:	jne    1fa77f <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x9df>
  1fa552:	lea    0xd0(%rsp),%rdi
  1fa55a:	lea    0x30(%rsp),%rsi
  1fa55f:	mov    %rcx,0x110(%rsp)
  1fa567:	mov    %rdi,0x118(%rsp)
  1fa56f:	mov    %rsi,0x120(%rsp)
  1fa577:	mov    %r13,0x128(%rsp)
  1fa57f:	mov    %rbp,0x130(%rsp)
  1fa587:	mov    %r14,0x138(%rsp)
  1fa58f:	mov    %r12,0x140(%rsp)
  1fa597:	mov    %r15,0x148(%rsp)
  1fa59f:	mov    %rdi,0x150(%rsp)
  1fa5a7:	mov    %rsi,0x158(%rsp)
  1fa5af:	mov    0x10(%rsp),%rsi
  1fa5b4:	mov    %rsi,0x160(%rsp)
  1fa5bc:	mov    %rax,0x168(%rsp)
  1fa5c4:	mov    %r14,0x170(%rsp)
  1fa5cc:	mov    %rbx,0x178(%rsp)
  1fa5d4:	mov    %rax,0x180(%rsp)
  1fa5dc:	lea    0x70(%rsp),%rdi
  1fa5e1:	lea    0x110(%rsp),%rsi
  1fa5e9:	xor    %ecx,%ecx
  1fa5eb:	call   1fc4d0 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}>
  1fa5f0:	mov    0x80(%rsp),%rcx
  1fa5f8:	mov    0x70(%rsp),%rax
  1fa5fd:	mov    %rcx,%rdx
  1fa600:	shl    $0x6,%rdx
  1fa604:	add    %rax,%rdx
  1fa607:	cmp    0x88(%rsp),%rdx
  1fa60f:	jne    1fa623 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x883>
  1fa611:	mov    0x90(%rsp),%rdx
  1fa619:	mov    0x98(%rsp),%rsi
  1fa621:	jmp    1fa627 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x887>
  1fa623:	xor    %edx,%edx
  1fa625:	xor    %esi,%esi
  1fa627:	mov    0x58(%rsp),%rdi
  1fa62c:	add    0x78(%rsp),%rdx
  1fa631:	add    %rcx,%rsi
  1fa634:	mov    %rax,(%rdi)
  1fa637:	mov    %rdx,0x8(%rdi)
  1fa63b:	mov    %rsi,0x10(%rdi)
  1fa63f:	add    $0x3a8,%rsp
  1fa646:	pop    %rbx
  1fa647:	pop    %r12
  1fa649:	pop    %r13
  1fa64b:	pop    %r14
  1fa64d:	pop    %r15
  1fa64f:	pop    %rbp
  1fa650:	vzeroupper
  1fa653:	ret
  1fa654:	mov    0x58(%rsp),%rax
  1fa659:	mov    0x48(%rsp),%rcx
  1fa65e:	mov    0x50(%rsp),%rdx
  1fa663:	mov    %rcx,(%rax)
  1fa666:	mov    %rdx,0x8(%rax)
  1fa66a:	mov    %rbx,0x10(%rax)
  1fa66e:	jmp    1fa63f <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x89f>
  1fa670:	lea    0x2ebe91(%rip),%rcx        # 4e6508 <__frame_dummy_init_array_entry+0x46a8>
  1fa677:	mov    %r14,%rdi
  1fa67a:	mov    %r12,%rsi
  1fa67d:	vzeroupper
  1fa680:	call   f0c00 <core::slice::index::slice_index_fail>
  1fa685:	lea    -0x170092(%rip),%rdi        # 8a5fa <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1f2>
  1fa68c:	lea    0x2ef3a5(%rip),%rdx        # 4e9a38 <__frame_dummy_init_array_entry+0x7bd8>
  1fa693:	mov    $0x45,%esi
  1fa698:	vzeroupper
  1fa69b:	call   eec00 <core::panicking::panic_fmt>
  1fa6a0:	lea    -0x1dab1d(%rip),%rdi        # 1fb8a <zmij::STATIC_DATA+0x558a>
  1fa6a7:	lea    0x2ef3da(%rip),%rdx        # 4e9a88 <__frame_dummy_init_array_entry+0x7c28>
  1fa6ae:	mov    $0x13,%esi
  1fa6b3:	call   eec00 <core::panicking::panic_fmt>
  1fa6b8:	lea    -0x1700e3(%rip),%rdi        # 8a5dc <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1d4>
  1fa6bf:	lea    0x2ef342(%rip),%rdx        # 4e9a08 <__frame_dummy_init_array_entry+0x7ba8>
  1fa6c6:	mov    $0x1e,%esi
  1fa6cb:	call   f0cd0 <core::panicking::panic>
  1fa6d0:	lea    0x28(%rsp),%rax
  1fa6d5:	lea    0xd0(%rsp),%rdx
  1fa6dd:	lea    0x30(%rsp),%rcx
  1fa6e2:	sub    $0xffffffffffffff80,%rsi
  1fa6e6:	lea    0x70(%rsp),%rdi
  1fa6eb:	mov    %rax,0x110(%rsp)
  1fa6f3:	mov    %rdx,0x118(%rsp)
  1fa6fb:	mov    %rcx,0x120(%rsp)
  1fa703:	mov    %r13,0x128(%rsp)
  1fa70b:	mov    %rbp,0x130(%rsp)
  1fa713:	mov    %r14,0x138(%rsp)
  1fa71b:	mov    %r12,0x140(%rsp)
  1fa723:	mov    %r15,0x148(%rsp)
  1fa72b:	mov    %rdx,0x150(%rsp)
  1fa733:	mov    %rcx,0x158(%rsp)
  1fa73b:	mov    0x10(%rsp),%rcx
  1fa740:	mov    0x18(%rsp),%rbp
  1fa745:	lea    0x110(%rsp),%rdx
  1fa74d:	mov    %rcx,0x160(%rsp)
  1fa755:	mov    %rbp,0x168(%rsp)
  1fa75d:	mov    %r14,0x170(%rsp)
  1fa765:	mov    %rbx,0x178(%rsp)
  1fa76d:	mov    %rbp,0x180(%rsp)
  1fa775:	call   1fc960 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>>
  1fa77a:	jmp    1fa5f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x850>
  1fa77f:	lea    0xd0(%rsp),%r8
  1fa787:	lea    0x30(%rsp),%rdi
  1fa78c:	mov    %rcx,0x110(%rsp)
  1fa794:	sub    $0xffffffffffffff80,%rsi
  1fa798:	lea    0x110(%rsp),%rcx
  1fa7a0:	mov    %r8,0x118(%rsp)
  1fa7a8:	mov    %rdi,0x120(%rsp)
  1fa7b0:	mov    %r13,0x128(%rsp)
  1fa7b8:	mov    %rbp,0x130(%rsp)
  1fa7c0:	mov    %r14,0x138(%rsp)
  1fa7c8:	mov    %r12,0x140(%rsp)
  1fa7d0:	mov    %r15,0x148(%rsp)
  1fa7d8:	mov    %r8,0x150(%rsp)
  1fa7e0:	mov    %rdi,0x158(%rsp)
  1fa7e8:	mov    0x10(%rsp),%rdi
  1fa7ed:	mov    %rdi,0x160(%rsp)
  1fa7f5:	lea    0x70(%rsp),%rdi
  1fa7fa:	mov    %rax,0x168(%rsp)
  1fa802:	mov    %r14,0x170(%rsp)
  1fa80a:	mov    %rbx,0x178(%rsp)
  1fa812:	mov    %rax,0x180(%rsp)
  1fa81a:	call   1fca60 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>)>>
  1fa81f:	jmp    1fa5f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0x850>
  1fa824:	lea    0x2ec8c5(%rip),%rdx        # 4e70f0 <__frame_dummy_init_array_entry+0x5290>
  1fa82b:	vzeroupper
  1fa82e:	call   efcab <core::panicking::panic_bounds_check>
  1fa833:	jmp    1fa850 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xab0>
  1fa835:	vzeroupper
  1fa838:	call   110970 <std::process::abort>
  1fa83d:	lea    0x2ebcac(%rip),%rdx        # 4e64f0 <__frame_dummy_init_array_entry+0x4690>
  1fa844:	xor    %edi,%edi
  1fa846:	xor    %esi,%esi
  1fa848:	vzeroupper
  1fa84b:	call   efcab <core::panicking::panic_bounds_check>
  1fa850:	ud2
  1fa852:	mov    $0x8,%edi
  1fa857:	mov    %r13,%rsi
  1fa85a:	call   ede21 <alloc::raw_vec::handle_error>
  1fa85f:	jmp    1fa863 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xac3>
  1fa861:	jmp    1fa868 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xac8>
  1fa863:	mov    %rax,%rbx
  1fa866:	jmp    1fa878 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xad8>
  1fa868:	lea    0x110(%rsp),%rdi
  1fa870:	mov    %rax,%rbx
  1fa873:	call   1e71f0 <core::ptr::drop_glue::<zopro_core::msm::Scratch<zopro_bn254::curve::G1Config>>>
  1fa878:	cmpq   $0x0,0x20(%rsp)
  1fa87e:	je     1fa898 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>+0xaf8>
  1fa880:	mov    0x20(%rsp),%rsi
  1fa885:	mov    0x18(%rsp),%rdi
  1fa88a:	mov    $0x8,%edx
  1fa88f:	shl    $0x6,%rsi
  1fa893:	call   e9120 <__rustc::__rust_dealloc>
  1fa898:	mov    %rbx,%rdi
  1fa89b:	call   4e0e10 <_Unwind_Resume@plt>
