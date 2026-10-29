
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000467dd0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>>:
  467dd0:	push   %rbp
  467dd1:	push   %r15
  467dd3:	push   %r14
  467dd5:	push   %r13
  467dd7:	push   %r12
  467dd9:	push   %rbx
  467dda:	sub    $0x178,%rsp
  467de1:	mov    0x1b8(%rsp),%r15
  467de9:	mov    0x1b0(%rsp),%r13
  467df1:	mov    %rsi,0x18(%rsp)
  467df6:	shr    $1,%rsi
  467df9:	mov    %r9,%r14
  467dfc:	mov    %rdi,%rbp
  467dff:	mov    %r8,0x30(%rsp)
  467e04:	cmp    %r8,%rsi
  467e07:	jae    467e93 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xc3>
  467e0d:	mov    %rbp,0x10(%rsp)
  467e12:	mov    0x8(%r15),%rbp
  467e16:	mov    0x10(%r15),%rbx
  467e1a:	sub    %r14,%r13
  467e1d:	mov    %rbp,0x8(%rsp)
  467e22:	jbe    467ead <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xdd>
  467e28:	mov    (%r15),%rax
  467e2b:	lea    0xb8(%rsp),%r12
  467e33:	xor    %r15d,%r15d
  467e36:	mov    %rax,(%rsp)
  467e3a:	mov    (%rsp),%rsi
  467e3e:	lea    (%r14,%r15,1),%rdx
  467e42:	mov    %r12,%rdi
  467e45:	vzeroupper
  467e48:	call   47f3f0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut>
  467e4d:	cmp    %r15,%rbx
  467e50:	je     46805b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x28b>
  467e56:	vmovups (%r12),%zmm0
  467e5d:	vmovups 0x40(%r12),%zmm1
  467e65:	vmovups 0x80(%r12),%zmm2
  467e6d:	inc    %r15
  467e70:	vmovups %zmm2,0x80(%rbp)
  467e77:	vmovups %zmm1,0x40(%rbp)
  467e7e:	vmovups %zmm0,0x0(%rbp)
  467e85:	add    $0xc0,%rbp
  467e8c:	cmp    %r15,%r13
  467e8f:	jne    467e3a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x6a>
  467e91:	jmp    467eb0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xe0>
  467e93:	test   %dl,%dl
  467e95:	je     467eca <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xfa>
  467e97:	mov    %fs:0xffffffffffffffc0,%rax
  467ea0:	test   %rax,%rax
  467ea3:	je     467ed8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x108>
  467ea5:	add    $0x110,%rax
  467eab:	jmp    467ee9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x119>
  467ead:	xor    %r15d,%r15d
  467eb0:	mov    0x10(%rsp),%rax
  467eb5:	mov    0x8(%rsp),%rcx
  467eba:	mov    %rcx,(%rax)
  467ebd:	mov    %rbx,0x8(%rax)
  467ec1:	mov    %r15,0x10(%rax)
  467ec5:	jmp    468046 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x276>
  467eca:	test   %rcx,%rcx
  467ecd:	je     467e0d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x3d>
  467ed3:	shr    $1,%rcx
  467ed6:	jmp    467efd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x12d>
  467ed8:	mov    %rsi,%rbx
  467edb:	mov    %rcx,%r12
  467ede:	call   105550 <rayon_core::registry::global_registry>
  467ee3:	mov    %r12,%rcx
  467ee6:	mov    %rbx,%rsi
  467ee9:	mov    (%rax),%rax
  467eec:	shr    $1,%rcx
  467eef:	mov    0x208(%rax),%rax
  467ef6:	cmp    %rax,%rcx
  467ef9:	cmovbe %rax,%rcx
  467efd:	mov    %rcx,0x28(%rsp)
  467f02:	xor    %eax,%eax
  467f04:	mov    %r13,%rcx
  467f07:	sub    %r14,%rcx
  467f0a:	mov    %rsi,0x20(%rsp)
  467f0f:	cmovae %rcx,%rax
  467f13:	cmp    %rax,%rsi
  467f16:	ja     468073 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2a3>
  467f1c:	mov    0x10(%r15),%rax
  467f20:	sub    %rsi,%rax
  467f23:	jb     46808b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2bb>
  467f29:	mov    0x8(%r15),%rdi
  467f2d:	mov    (%r15),%rdx
  467f30:	lea    (%rsi,%rsi,2),%r8
  467f34:	lea    0x18(%rsp),%r9
  467f39:	lea    (%rsi,%r14,1),%rcx
  467f3d:	lea    0x28(%rsp),%r10
  467f42:	mov    %r9,0x38(%rsp)
  467f47:	lea    0x20(%rsp),%r9
  467f4c:	shl    $0x6,%r8
  467f50:	mov    %r9,0x40(%rsp)
  467f55:	mov    %r10,0x48(%rsp)
  467f5a:	add    %rdi,%r8
  467f5d:	mov    %rdx,0x50(%rsp)
  467f62:	mov    %r8,0x58(%rsp)
  467f67:	mov    %rax,0x60(%rsp)
  467f6c:	mov    %rcx,0x68(%rsp)
  467f71:	mov    %r13,0x70(%rsp)
  467f76:	mov    %r9,0x78(%rsp)
  467f7b:	mov    %r10,0x80(%rsp)
  467f83:	mov    %rdx,0x88(%rsp)
  467f8b:	mov    %fs:0xffffffffffffffc0,%rdx
  467f94:	mov    %rdi,0x90(%rsp)
  467f9c:	mov    %rsi,0x98(%rsp)
  467fa4:	mov    %r14,0xa0(%rsp)
  467fac:	mov    %rcx,0xa8(%rsp)
  467fb4:	test   %rdx,%rdx
  467fb7:	jne    467fe0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x210>
  467fb9:	call   105550 <rayon_core::registry::global_registry>
  467fbe:	mov    (%rax),%rsi
  467fc1:	mov    %fs:0xffffffffffffffc0,%rdx
  467fca:	test   %rdx,%rdx
  467fcd:	je     4680a3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2d3>
  467fd3:	cmp    %rsi,0x110(%rdx)
  467fda:	jne    4680be <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2ee>
  467fe0:	lea    0xb0(%rsp),%rdi
  467fe8:	lea    0x38(%rsp),%rsi
  467fed:	xor    %ecx,%ecx
  467fef:	call   488790 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}>
  467ff4:	mov    0xc0(%rsp),%rcx
  467ffc:	mov    0xb0(%rsp),%rax
  468004:	lea    (%rcx,%rcx,2),%rdx
  468008:	shl    $0x6,%rdx
  46800c:	add    %rax,%rdx
  46800f:	cmp    0xc8(%rsp),%rdx
  468017:	jne    46802b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x25b>
  468019:	mov    0xd0(%rsp),%rdx
  468021:	mov    0xd8(%rsp),%rsi
  468029:	jmp    46802f <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x25f>
  46802b:	xor    %edx,%edx
  46802d:	xor    %esi,%esi
  46802f:	add    0xb8(%rsp),%rdx
  468037:	add    %rcx,%rsi
  46803a:	mov    %rax,0x0(%rbp)
  46803e:	mov    %rdx,0x8(%rbp)
  468042:	mov    %rsi,0x10(%rbp)
  468046:	add    $0x178,%rsp
  46804d:	pop    %rbx
  46804e:	pop    %r12
  468050:	pop    %r13
  468052:	pop    %r14
  468054:	pop    %r15
  468056:	pop    %rbp
  468057:	vzeroupper
  46805a:	ret
  46805b:	lea    -0x3dda77(%rip),%rdi        # 8a5eb <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x21b>
  468062:	lea    0x8c46f(%rip),%rdx        # 4f44d8 <__frame_dummy_init_array_entry+0x7b78>
  468069:	mov    $0x45,%esi
  46806e:	call   ef730 <core::panicking::panic_fmt>
  468073:	lea    -0x44f431(%rip),%rdi        # 18c49 <miniz_oxide::inflate::core::reverse_bits::REVERSED_BITS_LOOKUP+0x611>
  46807a:	lea    0x85817(%rip),%rdx        # 4ed898 <__frame_dummy_init_array_entry+0xf38>
  468081:	mov    $0x2b,%esi
  468086:	call   f1800 <core::panicking::panic>
  46808b:	lea    -0x3ddac5(%rip),%rdi        # 8a5cd <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1fd>
  468092:	lea    0x8c40f(%rip),%rdx        # 4f44a8 <__frame_dummy_init_array_entry+0x7b48>
  468099:	mov    $0x1e,%esi
  46809e:	call   f1800 <core::panicking::panic>
  4680a3:	sub    $0xffffffffffffff80,%rsi
  4680a7:	lea    0xb0(%rsp),%rdi
  4680af:	lea    0x38(%rsp),%rdx
  4680b4:	call   488690 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>>
  4680b9:	jmp    467ff4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x224>
  4680be:	sub    $0xffffffffffffff80,%rsi
  4680c2:	lea    0xb0(%rsp),%rdi
  4680ca:	lea    0x38(%rsp),%rcx
  4680cf:	call   488c50 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>>
  4680d4:	jmp    467ff4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x224>
