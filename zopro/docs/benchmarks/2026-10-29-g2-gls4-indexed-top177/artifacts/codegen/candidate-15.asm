
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000467ce0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>>:
  467ce0:	push   %rbp
  467ce1:	push   %r15
  467ce3:	push   %r14
  467ce5:	push   %r13
  467ce7:	push   %r12
  467ce9:	push   %rbx
  467cea:	sub    $0x178,%rsp
  467cf1:	mov    0x1b8(%rsp),%r15
  467cf9:	mov    0x1b0(%rsp),%r13
  467d01:	mov    %rsi,0x18(%rsp)
  467d06:	shr    $1,%rsi
  467d09:	mov    %r9,%r14
  467d0c:	mov    %rdi,%rbp
  467d0f:	mov    %r8,0x30(%rsp)
  467d14:	cmp    %r8,%rsi
  467d17:	jae    467da3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xc3>
  467d1d:	mov    %rbp,0x10(%rsp)
  467d22:	mov    0x8(%r15),%rbp
  467d26:	mov    0x10(%r15),%rbx
  467d2a:	sub    %r14,%r13
  467d2d:	mov    %rbp,0x8(%rsp)
  467d32:	jbe    467dbd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xdd>
  467d38:	mov    (%r15),%rax
  467d3b:	lea    0xb8(%rsp),%r12
  467d43:	xor    %r15d,%r15d
  467d46:	mov    %rax,(%rsp)
  467d4a:	mov    (%rsp),%rsi
  467d4e:	lea    (%r14,%r15,1),%rdx
  467d52:	mov    %r12,%rdi
  467d55:	vzeroupper
  467d58:	call   47f300 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut>
  467d5d:	cmp    %r15,%rbx
  467d60:	je     467f6b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x28b>
  467d66:	vmovups (%r12),%zmm0
  467d6d:	vmovups 0x40(%r12),%zmm1
  467d75:	vmovups 0x80(%r12),%zmm2
  467d7d:	inc    %r15
  467d80:	vmovups %zmm2,0x80(%rbp)
  467d87:	vmovups %zmm1,0x40(%rbp)
  467d8e:	vmovups %zmm0,0x0(%rbp)
  467d95:	add    $0xc0,%rbp
  467d9c:	cmp    %r15,%r13
  467d9f:	jne    467d4a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x6a>
  467da1:	jmp    467dc0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xe0>
  467da3:	test   %dl,%dl
  467da5:	je     467dda <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xfa>
  467da7:	mov    %fs:0xffffffffffffffc0,%rax
  467db0:	test   %rax,%rax
  467db3:	je     467de8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x108>
  467db5:	add    $0x110,%rax
  467dbb:	jmp    467df9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x119>
  467dbd:	xor    %r15d,%r15d
  467dc0:	mov    0x10(%rsp),%rax
  467dc5:	mov    0x8(%rsp),%rcx
  467dca:	mov    %rcx,(%rax)
  467dcd:	mov    %rbx,0x8(%rax)
  467dd1:	mov    %r15,0x10(%rax)
  467dd5:	jmp    467f56 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x276>
  467dda:	test   %rcx,%rcx
  467ddd:	je     467d1d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x3d>
  467de3:	shr    $1,%rcx
  467de6:	jmp    467e0d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x12d>
  467de8:	mov    %rsi,%rbx
  467deb:	mov    %rcx,%r12
  467dee:	call   105490 <rayon_core::registry::global_registry>
  467df3:	mov    %r12,%rcx
  467df6:	mov    %rbx,%rsi
  467df9:	mov    (%rax),%rax
  467dfc:	shr    $1,%rcx
  467dff:	mov    0x208(%rax),%rax
  467e06:	cmp    %rax,%rcx
  467e09:	cmovbe %rax,%rcx
  467e0d:	mov    %rcx,0x28(%rsp)
  467e12:	xor    %eax,%eax
  467e14:	mov    %r13,%rcx
  467e17:	sub    %r14,%rcx
  467e1a:	mov    %rsi,0x20(%rsp)
  467e1f:	cmovae %rcx,%rax
  467e23:	cmp    %rax,%rsi
  467e26:	ja     467f83 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2a3>
  467e2c:	mov    0x10(%r15),%rax
  467e30:	sub    %rsi,%rax
  467e33:	jb     467f9b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2bb>
  467e39:	mov    0x8(%r15),%rdi
  467e3d:	mov    (%r15),%rdx
  467e40:	lea    (%rsi,%rsi,2),%r8
  467e44:	lea    0x18(%rsp),%r9
  467e49:	lea    (%rsi,%r14,1),%rcx
  467e4d:	lea    0x28(%rsp),%r10
  467e52:	mov    %r9,0x38(%rsp)
  467e57:	lea    0x20(%rsp),%r9
  467e5c:	shl    $0x6,%r8
  467e60:	mov    %r9,0x40(%rsp)
  467e65:	mov    %r10,0x48(%rsp)
  467e6a:	add    %rdi,%r8
  467e6d:	mov    %rdx,0x50(%rsp)
  467e72:	mov    %r8,0x58(%rsp)
  467e77:	mov    %rax,0x60(%rsp)
  467e7c:	mov    %rcx,0x68(%rsp)
  467e81:	mov    %r13,0x70(%rsp)
  467e86:	mov    %r9,0x78(%rsp)
  467e8b:	mov    %r10,0x80(%rsp)
  467e93:	mov    %rdx,0x88(%rsp)
  467e9b:	mov    %fs:0xffffffffffffffc0,%rdx
  467ea4:	mov    %rdi,0x90(%rsp)
  467eac:	mov    %rsi,0x98(%rsp)
  467eb4:	mov    %r14,0xa0(%rsp)
  467ebc:	mov    %rcx,0xa8(%rsp)
  467ec4:	test   %rdx,%rdx
  467ec7:	jne    467ef0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x210>
  467ec9:	call   105490 <rayon_core::registry::global_registry>
  467ece:	mov    (%rax),%rsi
  467ed1:	mov    %fs:0xffffffffffffffc0,%rdx
  467eda:	test   %rdx,%rdx
  467edd:	je     467fb3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2d3>
  467ee3:	cmp    %rsi,0x110(%rdx)
  467eea:	jne    467fce <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2ee>
  467ef0:	lea    0xb0(%rsp),%rdi
  467ef8:	lea    0x38(%rsp),%rsi
  467efd:	xor    %ecx,%ecx
  467eff:	call   488650 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}>
  467f04:	mov    0xc0(%rsp),%rcx
  467f0c:	mov    0xb0(%rsp),%rax
  467f14:	lea    (%rcx,%rcx,2),%rdx
  467f18:	shl    $0x6,%rdx
  467f1c:	add    %rax,%rdx
  467f1f:	cmp    0xc8(%rsp),%rdx
  467f27:	jne    467f3b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x25b>
  467f29:	mov    0xd0(%rsp),%rdx
  467f31:	mov    0xd8(%rsp),%rsi
  467f39:	jmp    467f3f <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x25f>
  467f3b:	xor    %edx,%edx
  467f3d:	xor    %esi,%esi
  467f3f:	add    0xb8(%rsp),%rdx
  467f47:	add    %rcx,%rsi
  467f4a:	mov    %rax,0x0(%rbp)
  467f4e:	mov    %rdx,0x8(%rbp)
  467f52:	mov    %rsi,0x10(%rbp)
  467f56:	add    $0x178,%rsp
  467f5d:	pop    %rbx
  467f5e:	pop    %r12
  467f60:	pop    %r13
  467f62:	pop    %r14
  467f64:	pop    %r15
  467f66:	pop    %rbp
  467f67:	vzeroupper
  467f6a:	ret
  467f6b:	lea    -0x3dd987(%rip),%rdi        # 8a5eb <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x21b>
  467f72:	lea    0x8c41f(%rip),%rdx        # 4f4398 <__frame_dummy_init_array_entry+0x7b78>
  467f79:	mov    $0x45,%esi
  467f7e:	call   ef670 <core::panicking::panic_fmt>
  467f83:	lea    -0x44f341(%rip),%rdi        # 18c49 <miniz_oxide::inflate::core::reverse_bits::REVERSED_BITS_LOOKUP+0x611>
  467f8a:	lea    0x857c7(%rip),%rdx        # 4ed758 <__frame_dummy_init_array_entry+0xf38>
  467f91:	mov    $0x2b,%esi
  467f96:	call   f1740 <core::panicking::panic>
  467f9b:	lea    -0x3dd9d5(%rip),%rdi        # 8a5cd <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1fd>
  467fa2:	lea    0x8c3bf(%rip),%rdx        # 4f4368 <__frame_dummy_init_array_entry+0x7b48>
  467fa9:	mov    $0x1e,%esi
  467fae:	call   f1740 <core::panicking::panic>
  467fb3:	sub    $0xffffffffffffff80,%rsi
  467fb7:	lea    0xb0(%rsp),%rdi
  467fbf:	lea    0x38(%rsp),%rdx
  467fc4:	call   488550 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>>
  467fc9:	jmp    467f04 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x224>
  467fce:	sub    $0xffffffffffffff80,%rsi
  467fd2:	lea    0xb0(%rsp),%rdi
  467fda:	lea    0x38(%rsp),%rcx
  467fdf:	call   488b10 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>>
  467fe4:	jmp    467f04 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x224>
