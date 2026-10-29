
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/control-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000028ccf0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>:
  28ccf0:	push   %rbp
  28ccf1:	push   %r15
  28ccf3:	push   %r14
  28ccf5:	push   %r13
  28ccf7:	push   %r12
  28ccf9:	push   %rbx
  28ccfa:	sub    $0x6d8,%rsp
  28cd01:	mov    0x718(%rsp),%rbx
  28cd09:	mov    0x710(%rsp),%rbp
  28cd11:	mov    %rsi,0x28(%rsp)
  28cd16:	shr    $1,%rsi
  28cd19:	mov    %rcx,0x30(%rsp)
  28cd1e:	mov    %r9,%r12
  28cd21:	mov    %rdi,0x50(%rsp)
  28cd26:	mov    %r8,0x38(%rsp)
  28cd2b:	cmp    %r8,%rsi
  28cd2e:	jae    28d462 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x772>
  28cd34:	mov    (%rbx),%rcx
  28cd37:	mov    0x8(%rbx),%rsi
  28cd3b:	mov    0x10(%rbx),%rdx
  28cd3f:	movabs $0x7fffffffffffffc1,%rax
  28cd49:	shl    $0x4,%rbp
  28cd4d:	xor    %ebx,%ebx
  28cd4f:	add    %r12,%rbp
  28cd52:	add    $0x37,%rax
  28cd56:	mov    %rax,0x78(%rsp)
  28cd5b:	mov    %rbp,0x18(%rsp)
  28cd60:	mov    %rcx,0x58(%rsp)
  28cd65:	movabs $0x7fffffffffffffd,%rcx
  28cd6f:	mov    %rsi,0x40(%rsp)
  28cd74:	mov    %rdx,0x48(%rsp)
  28cd79:	cmp    %rbp,%r12
  28cd7c:	je     28d6dd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9ed>
  28cd82:	mov    0x58(%rsp),%rax
  28cd87:	mov    (%r12),%r14
  28cd8b:	mov    0x8(%r12),%r13
  28cd90:	mov    0x8(%rax),%rdx
  28cd94:	cmp    %r14,%r13
  28cd97:	jb     28d6f9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa09>
  28cd9d:	cmp    %rdx,%r13
  28cda0:	ja     28d6f9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa09>
  28cda6:	shl    $0x2,%r13
  28cdaa:	shl    $0x2,%r14
  28cdae:	mov    %rbx,0x70(%rsp)
  28cdb3:	mov    %r13,%r15
  28cdb6:	sub    %r14,%r15
  28cdb9:	mov    %r15,%rbp
  28cdbc:	shl    $0x5,%rbp
  28cdc0:	cmp    %rcx,%r15
  28cdc3:	setb   %al
  28cdc6:	cmp    0x78(%rsp),%rbp
  28cdcb:	setbe  %cl
  28cdce:	test   %cl,%al
  28cdd0:	je     28d45a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x76a>
  28cdd6:	mov    0x58(%rsp),%rax
  28cddb:	mov    %r12,0x8(%rsp)
  28cde0:	shr    $0x2,%r15
  28cde4:	mov    (%rax),%r12
  28cde7:	mov    0x10(%rax),%rbx
  28cdeb:	mov    0x18(%rax),%rax
  28cdef:	add    %r14,%r12
  28cdf2:	mov    %rax,0x68(%rsp)
  28cdf7:	test   %rbp,%rbp
  28cdfa:	je     28ced3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x1e3>
  28ce00:	mov    $0x8,%edi
  28ce05:	mov    %rbp,%rsi
  28ce08:	xor    %edx,%edx
  28ce0a:	vzeroupper
  28ce0d:	call   4b8940 <<zopro_core::alloc::linux::Allocator>::allocate>
  28ce12:	mov    %r15,0x10(%rsp)
  28ce17:	mov    %rax,0x20(%rsp)
  28ce1c:	test   %rax,%rax
  28ce1f:	je     28d8e3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbf3>
  28ce25:	cmp    %r14,%r13
  28ce28:	jne    28ceef <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x1ff>
  28ce2e:	xor    %r15d,%r15d
  28ce31:	lea    0x160(%rsp),%r13
  28ce39:	lea    0x120(%rsp),%r14
  28ce41:	mov    %r15,0x60(%rsp)
  28ce46:	mov    $0x1d,%edx
  28ce4b:	lea    0x80(%rsp),%rdi
  28ce53:	lea    -0x202a3b(%rip),%rsi        # 8a41f <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x22f>
  28ce5a:	vzeroupper
  28ce5d:	call   142e20 <std::sys::env::unix::getenv>
  28ce62:	lea    0x200(%rsp),%rbx
  28ce6a:	mov    0x80(%rsp),%r12
  28ce72:	cmp    $0xffffffffffffffff,%r12
  28ce76:	je     28d029 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x339>
  28ce7c:	mov    0x88(%rsp),%rbx
  28ce84:	mov    0x90(%rsp),%r14
  28ce8c:	lea    0x80(%rsp),%rdi
  28ce94:	mov    %rbx,%rsi
  28ce97:	mov    %r14,%rdx
  28ce9a:	call   f0100 <core::str::converts::from_utf8>
  28ce9f:	cmpb   $0x0,0x80(%rsp)
  28cea7:	je     28d033 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x343>
  28cead:	mov    $0x1,%bpl
  28ceb0:	test   %r12,%r12
  28ceb3:	jle    28d419 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x729>
  28ceb9:	lea    0x120(%rsp),%r14
  28cec1:	cmp    $0x1ffff,%r12
  28cec8:	ja     28d059 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x369>
  28cece:	jmp    28d077 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x387>
  28ced3:	mov    $0x8,%eax
  28ced8:	movq   $0x0,0x10(%rsp)
  28cee1:	mov    %rax,0x20(%rsp)
  28cee6:	cmp    %r14,%r13
  28cee9:	je     28ce2e <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x13e>
  28ceef:	mov    0x20(%rsp),%rax
  28cef4:	mov    0x68(%rsp),%rsi
  28cef9:	lea    0x160(%rsp),%r13
  28cf01:	lea    0x120(%rsp),%r14
  28cf09:	lea    0xc0(%rsp),%rdx
  28cf11:	xor    %ecx,%ecx
  28cf13:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  28cf20:	mov    (%r12,%rcx,4),%edi
  28cf24:	cmp    %rdi,%rsi
  28cf27:	jbe    28d8b8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbc8>
  28cf2d:	shl    $0x7,%rdi
  28cf31:	vmovups (%rbx,%rdi,1),%zmm0
  28cf38:	vmovups 0x40(%rbx,%rdi,1),%zmm1
  28cf40:	vmovups %zmm1,0x240(%rsp)
  28cf48:	vmovups %zmm0,0x200(%rsp)
  28cf50:	vmovdqu (%rbx,%rdi,1),%ymm0
  28cf55:	vptest %ymm0,%ymm0
  28cf5a:	jne    28cf90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x2a0>
  28cf5c:	vmovdqu 0x220(%rsp),%ymm0
  28cf65:	vptest %ymm0,%ymm0
  28cf6a:	jne    28cf90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x2a0>
  28cf6c:	vmovdqu 0x240(%rsp),%ymm0
  28cf75:	vptest %ymm0,%ymm0
  28cf7a:	jne    28cf90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x2a0>
  28cf7c:	vmovdqu 0x260(%rsp),%ymm0
  28cf85:	vptest %ymm0,%ymm0
  28cf8a:	je     28d013 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x323>
  28cf90:	add    %rbx,%rdi
  28cf93:	vmovups 0x20(%rdi),%ymm0
  28cf98:	vmovups %ymm0,0x0(%r13)
  28cf9e:	vmovups (%rdi),%ymm0
  28cfa2:	vmovups %ymm0,0x140(%rsp)
  28cfab:	vmovups 0x60(%rdi),%ymm0
  28cfb0:	vmovups 0x140(%rsp),%zmm2
  28cfb8:	vmovups %ymm0,(%r14)
  28cfbd:	vmovdqu 0x40(%rdi),%ymm0
  28cfc2:	vmovups %zmm2,0x80(%rsp)
  28cfca:	vmovdqu %ymm0,0x100(%rsp)
  28cfd3:	vmovups 0x100(%rsp),%zmm1
  28cfdb:	vmovups %zmm1,(%rdx)
  28cfe1:	vmovdqu64 0x80(%rsp),%zmm0
  28cfe9:	vmovups 0xc0(%rsp),%zmm1
  28cff1:	inc    %rcx
  28cff4:	vmovups %zmm1,0x40(%rax)
  28cffb:	vmovdqu64 %zmm0,(%rax)
  28d001:	sub    $0xffffffffffffff80,%rax
  28d005:	cmp    %rcx,%r15
  28d008:	jne    28cf20 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x230>
  28d00e:	jmp    28ce41 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x151>
  28d013:	vpxor  %xmm0,%xmm0,%xmm0
  28d017:	vmovdqu64 %zmm0,0xc0(%rsp)
  28d01f:	vmovdqu64 %zmm0,0x80(%rsp)
  28d027:	jmp    28cfe1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x2f1>
  28d029:	mov    0x8(%rsp),%r12
  28d02e:	mov    $0x1,%bpl
  28d031:	jmp    28d08d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x39d>
  28d033:	mov    $0x1,%bpl
  28d036:	cmp    $0x1,%r14
  28d03a:	jne    28d043 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x353>
  28d03c:	cmpb   $0x30,(%rbx)
  28d03f:	setne  %bpl
  28d043:	lea    0x120(%rsp),%r14
  28d04b:	test   %r12,%r12
  28d04e:	je     28d080 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x390>
  28d050:	cmp    $0x1ffff,%r12
  28d057:	jbe    28d077 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x387>
  28d059:	mov    %r12,%rsi
  28d05c:	inc    %rsi
  28d05f:	js     28d8c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28d065:	cmpb   $0x1,(%rbx,%r12,1)
  28d06a:	jne    28d077 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x387>
  28d06c:	mov    %rbx,%rdi
  28d06f:	call   *0x25a28b(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  28d075:	jmp    28d080 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x390>
  28d077:	mov    %rbx,%rdi
  28d07a:	call   *0x25a288(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  28d080:	mov    0x8(%rsp),%r12
  28d085:	lea    0x200(%rsp),%rbx
  28d08d:	lea    0x210(%rsp),%rax
  28d095:	mov    $0x408,%edx
  28d09a:	lea    0x240(%rsp),%rdi
  28d0a2:	mov    %bpl,0x648(%rsp)
  28d0aa:	movq   $0x0,0x200(%rsp)
  28d0b6:	xor    %esi,%esi
  28d0b8:	add    $0x10,%r12
  28d0bc:	movq   $0x8,0x208(%rsp)
  28d0c8:	vpxor  %xmm0,%xmm0,%xmm0
  28d0cc:	vmovdqu %xmm0,(%rax)
  28d0d0:	movq   $0x40,0x220(%rsp)
  28d0dc:	vmovdqu %xmm0,0x18(%rax)
  28d0e1:	movq   $0x40,0x238(%rsp)
  28d0ed:	call   *0x25a295(%rip)        # 4e7388 <memset@GLIBC_2.2.5>
  28d0f3:	mov    0x18(%rsp),%rbp
  28d0f8:	nopl   0x0(%rax,%rax,1)
  28d100:	cmpq   $0x1,0x60(%rsp)
  28d106:	jbe    28d140 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x450>
  28d108:	mov    0x20(%rsp),%rdi
  28d10d:	mov    $0x1,%ecx
  28d112:	mov    $0x1,%r9d
  28d118:	lea    -0x281f07(%rip),%rdx        # b218 <__abi_tag+0xaf1c>
  28d11f:	lea    0x60(%rsp),%r8
  28d124:	mov    %r15,%rsi
  28d127:	mov    %rbx,(%rsp)
  28d12b:	call   22c280 <zopro_core::msm::reduce_layer::<zopro_bn254::curve::G2Config>>
  28d130:	jmp    28d100 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x410>
  28d132:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  28d140:	test   %r15,%r15
  28d143:	je     28d8d1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbe1>
  28d149:	mov    0x20(%rsp),%r15
  28d14e:	mov    0x70(%rsp),%rbx
  28d153:	vmovups (%r15),%zmm0
  28d159:	vmovups 0x40(%r15),%zmm1
  28d160:	vmovups %zmm1,0xc0(%rsp)
  28d168:	vmovups %zmm0,0x80(%rsp)
  28d170:	vmovdqu (%r15),%ymm0
  28d175:	vptest %ymm0,%ymm0
  28d17a:	jne    28d1b0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x4c0>
  28d17c:	vmovdqu 0xa0(%rsp),%ymm0
  28d185:	vptest %ymm0,%ymm0
  28d18a:	jne    28d1b0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x4c0>
  28d18c:	vmovdqu 0xc0(%rsp),%ymm0
  28d195:	vptest %ymm0,%ymm0
  28d19a:	jne    28d1b0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x4c0>
  28d19c:	vmovdqu 0xe0(%rsp),%ymm0
  28d1a5:	vptest %ymm0,%ymm0
  28d1aa:	je     28d42b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x73b>
  28d1b0:	vmovups 0x20(%r15),%ymm0
  28d1b6:	lea    0x1c0(%rsp),%rax
  28d1be:	vmovups %ymm0,0x0(%r13)
  28d1c4:	vmovups (%r15),%ymm0
  28d1c9:	vmovups %ymm0,0x140(%rsp)
  28d1d2:	vmovups 0x60(%r15),%ymm0
  28d1d8:	vmovups 0x140(%rsp),%zmm2
  28d1e0:	vmovups %ymm0,(%r14)
  28d1e5:	vmovdqu 0x40(%r15),%ymm0
  28d1eb:	vmovups %zmm2,0x180(%rsp)
  28d1f3:	vmovdqu %ymm0,0x100(%rsp)
  28d1fc:	vmovups 0x100(%rsp),%zmm1
  28d204:	vmovups %zmm1,(%rax)
  28d20a:	mov    0x10(%rsp),%r14
  28d20f:	mov    0x200(%rsp),%rax
  28d217:	test   %rax,%rax
  28d21a:	je     28d269 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x579>
  28d21c:	mov    0x208(%rsp),%rdi
  28d224:	cmp    $0x3fff,%rax
  28d22a:	jbe    28d260 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x570>
  28d22c:	movabs $0x7fffffffffffffc1,%rcx
  28d236:	shl    $0x3,%rax
  28d23a:	mov    %rax,%rsi
  28d23d:	or     $0x1,%rsi
  28d241:	add    $0x38,%rcx
  28d245:	cmp    %rcx,%rsi
  28d248:	jae    28d8c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28d24e:	cmpb   $0x1,(%rdi,%rax,1)
  28d252:	jne    28d260 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x570>
  28d254:	vzeroupper
  28d257:	call   *0x25a0a3(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  28d25d:	jmp    28d269 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x579>
  28d25f:	nop
  28d260:	vzeroupper
  28d263:	call   *0x25a09f(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  28d269:	mov    0x218(%rsp),%rax
  28d271:	test   %rax,%rax
  28d274:	je     28d2c0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x5d0>
  28d276:	mov    0x220(%rsp),%rdi
  28d27e:	cmp    $0x2d,%rax
  28d282:	jbe    28d2d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x5e0>
  28d284:	imul   $0xb40,%rax,%rax
  28d28b:	movabs $0x7fffffffffffffc1,%rcx
  28d295:	mov    %rax,%rsi
  28d298:	or     $0x1,%rsi
  28d29c:	cmp    %rcx,%rsi
  28d29f:	jae    28d8c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28d2a5:	cmpb   $0x1,(%rdi,%rax,1)
  28d2a9:	jne    28d2d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x5e0>
  28d2ab:	vzeroupper
  28d2ae:	call   *0x25a04c(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  28d2b4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
  28d2c0:	mov    0x230(%rsp),%rax
  28d2c8:	test   %rax,%rax
  28d2cb:	jne    28d2e6 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x5f6>
  28d2cd:	jmp    28d330 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x640>
  28d2cf:	nop
  28d2d0:	vzeroupper
  28d2d3:	call   *0x25a02f(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  28d2d9:	mov    0x230(%rsp),%rax
  28d2e1:	test   %rax,%rax
  28d2e4:	je     28d330 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x640>
  28d2e6:	mov    0x238(%rsp),%rdi
  28d2ee:	cmp    $0xcc,%rax
  28d2f4:	jbe    28d340 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x650>
  28d2f6:	shl    $0x7,%rax
  28d2fa:	movabs $0x7fffffffffffffc1,%rcx
  28d304:	lea    (%rax,%rax,4),%rax
  28d308:	mov    %rax,%rsi
  28d30b:	or     $0x1,%rsi
  28d30f:	cmp    %rcx,%rsi
  28d312:	jae    28d8c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28d318:	cmpb   $0x1,(%rdi,%rax,1)
  28d31c:	jne    28d340 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x650>
  28d31e:	vzeroupper
  28d321:	call   *0x259fd9(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  28d327:	nopw   0x0(%rax,%rax,1)
  28d330:	test   %r14,%r14
  28d333:	jne    28d34e <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x65e>
  28d335:	jmp    28d39c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6ac>
  28d337:	nopw   0x0(%rax,%rax,1)
  28d340:	vzeroupper
  28d343:	call   *0x259fbf(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  28d349:	test   %r14,%r14
  28d34c:	je     28d39c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6ac>
  28d34e:	cmp    $0x3ff,%r14
  28d355:	jbe    28d390 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6a0>
  28d357:	movabs $0x7fffffffffffffc1,%rax
  28d361:	shl    $0x7,%r14
  28d365:	mov    %r14,%rsi
  28d368:	or     $0x1,%rsi
  28d36c:	add    $0x38,%rax
  28d370:	cmp    %rax,%rsi
  28d373:	jae    28d8c9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbd9>
  28d379:	cmpb   $0x1,(%r15,%r14,1)
  28d37e:	jne    28d390 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6a0>
  28d380:	mov    %r15,%rdi
  28d383:	vzeroupper
  28d386:	call   *0x259f74(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  28d38c:	jmp    28d39c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x6ac>
  28d38e:	xchg   %ax,%ax
  28d390:	mov    %r15,%rdi
  28d393:	vzeroupper
  28d396:	call   *0x259f6c(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  28d39c:	vmovups 0x180(%rsp),%zmm0
  28d3a4:	vmovups 0x1c0(%rsp),%zmm1
  28d3ac:	vmovups %zmm1,0x690(%rsp)
  28d3b7:	vmovups %zmm0,0x650(%rsp)
  28d3c2:	cmp    0x48(%rsp),%rbx
  28d3c7:	je     28d70e <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa1e>
  28d3cd:	vmovdqu64 0x650(%rsp),%zmm0
  28d3d8:	vmovups 0x690(%rsp),%zmm1
  28d3e3:	mov    0x40(%rsp),%rcx
  28d3e8:	mov    %rbx,%rax
  28d3eb:	shl    $0x7,%rax
  28d3ef:	inc    %rbx
  28d3f2:	vmovups %zmm1,0x40(%rcx,%rax,1)
  28d3fa:	vmovdqu64 %zmm0,(%rcx,%rax,1)
  28d401:	movabs $0x7fffffffffffffd,%rcx
  28d40b:	cmp    %rbp,%r12
  28d40e:	jne    28cd82 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x92>
  28d414:	jmp    28d6dd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9ed>
  28d419:	mov    0x8(%rsp),%r12
  28d41e:	lea    0x120(%rsp),%r14
  28d426:	jmp    28d085 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x395>
  28d42b:	vpxor  %xmm0,%xmm0,%xmm0
  28d42f:	vmovdqu64 %zmm0,0x1c0(%rsp)
  28d437:	vmovdqu64 %zmm0,0x180(%rsp)
  28d43f:	mov    0x10(%rsp),%r14
  28d444:	mov    0x200(%rsp),%rax
  28d44c:	test   %rax,%rax
  28d44f:	jne    28d21c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x52c>
  28d455:	jmp    28d269 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x579>
  28d45a:	vzeroupper
  28d45d:	call   ed650 <alloc::raw_vec::capacity_overflow>
  28d462:	test   %dl,%dl
  28d464:	je     28d47c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x78c>
  28d466:	mov    %fs:0xffffffffffffffc0,%rax
  28d46f:	test   %rax,%rax
  28d472:	je     28d48a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x79a>
  28d474:	add    $0x110,%rax
  28d47a:	jmp    28d49b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x7ab>
  28d47c:	test   %rcx,%rcx
  28d47f:	je     28cd34 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x44>
  28d485:	shr    $1,%rcx
  28d488:	jmp    28d4af <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x7bf>
  28d48a:	mov    %rsi,%r14
  28d48d:	mov    %rcx,%r15
  28d490:	call   104130 <rayon_core::registry::global_registry>
  28d495:	mov    %r15,%rcx
  28d498:	mov    %r14,%rsi
  28d49b:	mov    (%rax),%rax
  28d49e:	shr    $1,%rcx
  28d4a1:	mov    0x208(%rax),%rax
  28d4a8:	cmp    %rax,%rcx
  28d4ab:	cmovbe %rax,%rcx
  28d4af:	sub    %rsi,%rbp
  28d4b2:	mov    %rcx,0x30(%rsp)
  28d4b7:	mov    %rsi,0x180(%rsp)
  28d4bf:	jb     28d729 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa39>
  28d4c5:	mov    0x10(%rbx),%r15
  28d4c9:	sub    %rsi,%r15
  28d4cc:	jb     28d741 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa51>
  28d4d2:	mov    (%rbx),%r14
  28d4d5:	mov    0x8(%rbx),%rbx
  28d4d9:	mov    %fs:0xffffffffffffffc0,%rdx
  28d4e2:	mov    %rsi,%r13
  28d4e5:	shl    $0x4,%r13
  28d4e9:	mov    %r12,%rdi
  28d4ec:	add    %r12,%r13
  28d4ef:	mov    %rsi,%r12
  28d4f2:	shl    $0x7,%r12
  28d4f6:	add    %rbx,%r12
  28d4f9:	test   %rdx,%rdx
  28d4fc:	je     28d591 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x8a1>
  28d502:	lea    0x28(%rsp),%rax
  28d507:	lea    0x180(%rsp),%r8
  28d50f:	lea    0x30(%rsp),%rcx
  28d514:	mov    %rax,0x200(%rsp)
  28d51c:	mov    %r8,0x208(%rsp)
  28d524:	mov    %rcx,0x210(%rsp)
  28d52c:	mov    %r13,0x218(%rsp)
  28d534:	mov    %rbp,0x220(%rsp)
  28d53c:	mov    %r14,0x228(%rsp)
  28d544:	mov    %r12,0x230(%rsp)
  28d54c:	mov    %r15,0x238(%rsp)
  28d554:	mov    %r8,0x240(%rsp)
  28d55c:	mov    %rcx,0x248(%rsp)
  28d564:	mov    %rdi,0x250(%rsp)
  28d56c:	mov    %rsi,0x258(%rsp)
  28d574:	mov    %r14,0x260(%rsp)
  28d57c:	mov    %rbx,0x268(%rsp)
  28d584:	mov    %rsi,0x270(%rsp)
  28d58c:	jmp    28d65c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x96c>
  28d591:	mov    %rbp,0x18(%rsp)
  28d596:	mov    %rdi,0x8(%rsp)
  28d59b:	mov    %rsi,%rbp
  28d59e:	call   104130 <rayon_core::registry::global_registry>
  28d5a3:	mov    (%rax),%rsi
  28d5a6:	mov    %fs:0xffffffffffffffc0,%rdx
  28d5af:	test   %rdx,%rdx
  28d5b2:	je     28d759 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xa69>
  28d5b8:	lea    0x28(%rsp),%rcx
  28d5bd:	mov    %rbp,%rax
  28d5c0:	cmp    %rsi,0x110(%rdx)
  28d5c7:	jne    28d80b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xb1b>
  28d5cd:	mov    0x18(%rsp),%rdi
  28d5d2:	mov    %rcx,0x200(%rsp)
  28d5da:	lea    0x180(%rsp),%rcx
  28d5e2:	lea    0x30(%rsp),%rsi
  28d5e7:	mov    %rcx,0x208(%rsp)
  28d5ef:	mov    %rsi,0x210(%rsp)
  28d5f7:	mov    %r13,0x218(%rsp)
  28d5ff:	mov    %rdi,0x220(%rsp)
  28d607:	mov    %r14,0x228(%rsp)
  28d60f:	mov    %r12,0x230(%rsp)
  28d617:	mov    %r15,0x238(%rsp)
  28d61f:	mov    %rcx,0x240(%rsp)
  28d627:	mov    %rsi,0x248(%rsp)
  28d62f:	mov    0x8(%rsp),%rsi
  28d634:	mov    %rsi,0x250(%rsp)
  28d63c:	mov    %rax,0x258(%rsp)
  28d644:	mov    %r14,0x260(%rsp)
  28d64c:	mov    %rbx,0x268(%rsp)
  28d654:	mov    %rax,0x270(%rsp)
  28d65c:	lea    0x80(%rsp),%rdi
  28d664:	lea    0x200(%rsp),%rsi
  28d66c:	xor    %ecx,%ecx
  28d66e:	call   28d940 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}>
  28d673:	mov    0x90(%rsp),%rcx
  28d67b:	mov    0x80(%rsp),%rax
  28d683:	mov    %rcx,%rdx
  28d686:	shl    $0x7,%rdx
  28d68a:	add    %rax,%rdx
  28d68d:	cmp    0x98(%rsp),%rdx
  28d695:	jne    28d6a9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9b9>
  28d697:	mov    0xa0(%rsp),%rdx
  28d69f:	mov    0xa8(%rsp),%rsi
  28d6a7:	jmp    28d6ad <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9bd>
  28d6a9:	xor    %edx,%edx
  28d6ab:	xor    %esi,%esi
  28d6ad:	mov    0x50(%rsp),%rdi
  28d6b2:	add    0x88(%rsp),%rdx
  28d6ba:	add    %rcx,%rsi
  28d6bd:	mov    %rax,(%rdi)
  28d6c0:	mov    %rdx,0x8(%rdi)
  28d6c4:	mov    %rsi,0x10(%rdi)
  28d6c8:	add    $0x6d8,%rsp
  28d6cf:	pop    %rbx
  28d6d0:	pop    %r12
  28d6d2:	pop    %r13
  28d6d4:	pop    %r14
  28d6d6:	pop    %r15
  28d6d8:	pop    %rbp
  28d6d9:	vzeroupper
  28d6dc:	ret
  28d6dd:	mov    0x50(%rsp),%rax
  28d6e2:	mov    0x40(%rsp),%rcx
  28d6e7:	mov    0x48(%rsp),%rdx
  28d6ec:	mov    %rcx,(%rax)
  28d6ef:	mov    %rdx,0x8(%rax)
  28d6f3:	mov    %rbx,0x10(%rax)
  28d6f7:	jmp    28d6c8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x9d8>
  28d6f9:	lea    0x255508(%rip),%rcx        # 4e2c08 <__frame_dummy_init_array_entry+0x45e8>
  28d700:	mov    %r14,%rdi
  28d703:	mov    %r13,%rsi
  28d706:	vzeroupper
  28d709:	call   f0300 <core::slice::index::slice_index_fail>
  28d70e:	lea    -0x203333(%rip),%rdi        # 8a3e2 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1f2>
  28d715:	lea    0x2589d4(%rip),%rdx        # 4e60f0 <__frame_dummy_init_array_entry+0x7ad0>
  28d71c:	mov    $0x45,%esi
  28d721:	vzeroupper
  28d724:	call   ee300 <core::panicking::panic_fmt>
  28d729:	lea    -0x26ddbc(%rip),%rdi        # 1f974 <zmij::STATIC_DATA+0x5574>
  28d730:	lea    0x258a09(%rip),%rdx        # 4e6140 <__frame_dummy_init_array_entry+0x7b20>
  28d737:	mov    $0x13,%esi
  28d73c:	call   ee300 <core::panicking::panic_fmt>
  28d741:	lea    -0x203384(%rip),%rdi        # 8a3c4 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1d4>
  28d748:	lea    0x258971(%rip),%rdx        # 4e60c0 <__frame_dummy_init_array_entry+0x7aa0>
  28d74f:	mov    $0x1e,%esi
  28d754:	call   f03d0 <core::panicking::panic>
  28d759:	mov    0x18(%rsp),%rdx
  28d75e:	lea    0x28(%rsp),%rax
  28d763:	lea    0x30(%rsp),%rcx
  28d768:	sub    $0xffffffffffffff80,%rsi
  28d76c:	lea    0x80(%rsp),%rdi
  28d774:	mov    %rax,0x200(%rsp)
  28d77c:	lea    0x180(%rsp),%rax
  28d784:	mov    %rax,0x208(%rsp)
  28d78c:	mov    %rcx,0x210(%rsp)
  28d794:	mov    %r13,0x218(%rsp)
  28d79c:	mov    %rdx,0x220(%rsp)
  28d7a4:	mov    %r14,0x228(%rsp)
  28d7ac:	mov    %r12,0x230(%rsp)
  28d7b4:	mov    %r15,0x238(%rsp)
  28d7bc:	mov    %rax,0x240(%rsp)
  28d7c4:	mov    %rcx,0x248(%rsp)
  28d7cc:	mov    0x8(%rsp),%rcx
  28d7d1:	lea    0x200(%rsp),%rdx
  28d7d9:	mov    %rcx,0x250(%rsp)
  28d7e1:	mov    %rbp,0x258(%rsp)
  28d7e9:	mov    %r14,0x260(%rsp)
  28d7f1:	mov    %rbx,0x268(%rsp)
  28d7f9:	mov    %rbp,0x270(%rsp)
  28d801:	call   28ddd0 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>>
  28d806:	jmp    28d673 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x983>
  28d80b:	mov    0x18(%rsp),%r8
  28d810:	mov    %rcx,0x200(%rsp)
  28d818:	lea    0x180(%rsp),%rcx
  28d820:	lea    0x30(%rsp),%rdi
  28d825:	sub    $0xffffffffffffff80,%rsi
  28d829:	mov    %rcx,0x208(%rsp)
  28d831:	mov    %rdi,0x210(%rsp)
  28d839:	mov    %r13,0x218(%rsp)
  28d841:	mov    %r8,0x220(%rsp)
  28d849:	mov    %r14,0x228(%rsp)
  28d851:	mov    %r12,0x230(%rsp)
  28d859:	mov    %r15,0x238(%rsp)
  28d861:	mov    %rcx,0x240(%rsp)
  28d869:	mov    %rdi,0x248(%rsp)
  28d871:	mov    0x8(%rsp),%rdi
  28d876:	lea    0x200(%rsp),%rcx
  28d87e:	mov    %rdi,0x250(%rsp)
  28d886:	lea    0x80(%rsp),%rdi
  28d88e:	mov    %rax,0x258(%rsp)
  28d896:	mov    %r14,0x260(%rsp)
  28d89e:	mov    %rbx,0x268(%rsp)
  28d8a6:	mov    %rax,0x270(%rsp)
  28d8ae:	call   28ded0 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>)>>
  28d8b3:	jmp    28d673 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0x983>
  28d8b8:	lea    0x255ee9(%rip),%rdx        # 4e37a8 <__frame_dummy_init_array_entry+0x5188>
  28d8bf:	vzeroupper
  28d8c2:	call   ef3ab <core::panicking::panic_bounds_check>
  28d8c7:	jmp    28d8e1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xbf1>
  28d8c9:	vzeroupper
  28d8cc:	call   110070 <std::process::abort>
  28d8d1:	lea    0x255318(%rip),%rdx        # 4e2bf0 <__frame_dummy_init_array_entry+0x45d0>
  28d8d8:	xor    %edi,%edi
  28d8da:	xor    %esi,%esi
  28d8dc:	call   ef3ab <core::panicking::panic_bounds_check>
  28d8e1:	ud2
  28d8e3:	mov    $0x8,%edi
  28d8e8:	mov    %rbp,%rsi
  28d8eb:	call   ed521 <alloc::raw_vec::handle_error>
  28d8f0:	jmp    28d8f4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xc04>
  28d8f2:	jmp    28d8f9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xc09>
  28d8f4:	mov    %rax,%rbx
  28d8f7:	jmp    28d909 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xc19>
  28d8f9:	lea    0x200(%rsp),%rdi
  28d901:	mov    %rax,%rbx
  28d904:	call   2838c0 <core::ptr::drop_glue::<zopro_core::msm::Scratch<zopro_bn254::curve::G2Config>>>
  28d909:	cmpq   $0x0,0x10(%rsp)
  28d90f:	je     28d929 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>+0xc39>
  28d911:	mov    0x10(%rsp),%rsi
  28d916:	mov    0x20(%rsp),%rdi
  28d91b:	mov    $0x8,%edx
  28d920:	shl    $0x7,%rsi
  28d924:	call   e8820 <__rustc::__rust_dealloc>
  28d929:	mov    %rbx,%rdi
  28d92c:	call   4dd5d0 <_Unwind_Resume@plt>
