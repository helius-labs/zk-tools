
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000467ab0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>>:
  467ab0:	push   %rbp
  467ab1:	push   %r15
  467ab3:	push   %r14
  467ab5:	push   %r13
  467ab7:	push   %r12
  467ab9:	push   %rbx
  467aba:	sub    $0x178,%rsp
  467ac1:	mov    0x1b8(%rsp),%r15
  467ac9:	mov    0x1b0(%rsp),%r13
  467ad1:	mov    %rsi,0x18(%rsp)
  467ad6:	shr    $1,%rsi
  467ad9:	mov    %r9,%r14
  467adc:	mov    %rdi,%rbp
  467adf:	mov    %r8,0x30(%rsp)
  467ae4:	cmp    %r8,%rsi
  467ae7:	jae    467b73 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xc3>
  467aed:	mov    %rbp,0x10(%rsp)
  467af2:	mov    0x8(%r15),%rbp
  467af6:	mov    0x10(%r15),%rbx
  467afa:	sub    %r14,%r13
  467afd:	mov    %rbp,0x8(%rsp)
  467b02:	jbe    467b8d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xdd>
  467b08:	mov    (%r15),%rax
  467b0b:	lea    0xb8(%rsp),%r12
  467b13:	xor    %r15d,%r15d
  467b16:	mov    %rax,(%rsp)
  467b1a:	mov    (%rsp),%rsi
  467b1e:	lea    (%r14,%r15,1),%rdx
  467b22:	mov    %r12,%rdi
  467b25:	vzeroupper
  467b28:	call   47f0d0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut>
  467b2d:	cmp    %r15,%rbx
  467b30:	je     467d3b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x28b>
  467b36:	vmovups (%r12),%zmm0
  467b3d:	vmovups 0x40(%r12),%zmm1
  467b45:	vmovups 0x80(%r12),%zmm2
  467b4d:	inc    %r15
  467b50:	vmovups %zmm2,0x80(%rbp)
  467b57:	vmovups %zmm1,0x40(%rbp)
  467b5e:	vmovups %zmm0,0x0(%rbp)
  467b65:	add    $0xc0,%rbp
  467b6c:	cmp    %r15,%r13
  467b6f:	jne    467b1a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x6a>
  467b71:	jmp    467b90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xe0>
  467b73:	test   %dl,%dl
  467b75:	je     467baa <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0xfa>
  467b77:	mov    %fs:0xffffffffffffffc0,%rax
  467b80:	test   %rax,%rax
  467b83:	je     467bb8 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x108>
  467b85:	add    $0x110,%rax
  467b8b:	jmp    467bc9 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x119>
  467b8d:	xor    %r15d,%r15d
  467b90:	mov    0x10(%rsp),%rax
  467b95:	mov    0x8(%rsp),%rcx
  467b9a:	mov    %rcx,(%rax)
  467b9d:	mov    %rbx,0x8(%rax)
  467ba1:	mov    %r15,0x10(%rax)
  467ba5:	jmp    467d26 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x276>
  467baa:	test   %rcx,%rcx
  467bad:	je     467aed <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x3d>
  467bb3:	shr    $1,%rcx
  467bb6:	jmp    467bdd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x12d>
  467bb8:	mov    %rsi,%rbx
  467bbb:	mov    %rcx,%r12
  467bbe:	call   1052d0 <rayon_core::registry::global_registry>
  467bc3:	mov    %r12,%rcx
  467bc6:	mov    %rbx,%rsi
  467bc9:	mov    (%rax),%rax
  467bcc:	shr    $1,%rcx
  467bcf:	mov    0x208(%rax),%rax
  467bd6:	cmp    %rax,%rcx
  467bd9:	cmovbe %rax,%rcx
  467bdd:	mov    %rcx,0x28(%rsp)
  467be2:	xor    %eax,%eax
  467be4:	mov    %r13,%rcx
  467be7:	sub    %r14,%rcx
  467bea:	mov    %rsi,0x20(%rsp)
  467bef:	cmovae %rcx,%rax
  467bf3:	cmp    %rax,%rsi
  467bf6:	ja     467d53 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2a3>
  467bfc:	mov    0x10(%r15),%rax
  467c00:	sub    %rsi,%rax
  467c03:	jb     467d6b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2bb>
  467c09:	mov    0x8(%r15),%rdi
  467c0d:	mov    (%r15),%rdx
  467c10:	lea    (%rsi,%rsi,2),%r8
  467c14:	lea    0x18(%rsp),%r9
  467c19:	lea    (%rsi,%r14,1),%rcx
  467c1d:	lea    0x28(%rsp),%r10
  467c22:	mov    %r9,0x38(%rsp)
  467c27:	lea    0x20(%rsp),%r9
  467c2c:	shl    $0x6,%r8
  467c30:	mov    %r9,0x40(%rsp)
  467c35:	mov    %r10,0x48(%rsp)
  467c3a:	add    %rdi,%r8
  467c3d:	mov    %rdx,0x50(%rsp)
  467c42:	mov    %r8,0x58(%rsp)
  467c47:	mov    %rax,0x60(%rsp)
  467c4c:	mov    %rcx,0x68(%rsp)
  467c51:	mov    %r13,0x70(%rsp)
  467c56:	mov    %r9,0x78(%rsp)
  467c5b:	mov    %r10,0x80(%rsp)
  467c63:	mov    %rdx,0x88(%rsp)
  467c6b:	mov    %fs:0xffffffffffffffc0,%rdx
  467c74:	mov    %rdi,0x90(%rsp)
  467c7c:	mov    %rsi,0x98(%rsp)
  467c84:	mov    %r14,0xa0(%rsp)
  467c8c:	mov    %rcx,0xa8(%rsp)
  467c94:	test   %rdx,%rdx
  467c97:	jne    467cc0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x210>
  467c99:	call   1052d0 <rayon_core::registry::global_registry>
  467c9e:	mov    (%rax),%rsi
  467ca1:	mov    %fs:0xffffffffffffffc0,%rdx
  467caa:	test   %rdx,%rdx
  467cad:	je     467d83 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2d3>
  467cb3:	cmp    %rsi,0x110(%rdx)
  467cba:	jne    467d9e <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x2ee>
  467cc0:	lea    0xb0(%rsp),%rdi
  467cc8:	lea    0x38(%rsp),%rsi
  467ccd:	xor    %ecx,%ecx
  467ccf:	call   4852e0 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}>
  467cd4:	mov    0xc0(%rsp),%rcx
  467cdc:	mov    0xb0(%rsp),%rax
  467ce4:	lea    (%rcx,%rcx,2),%rdx
  467ce8:	shl    $0x6,%rdx
  467cec:	add    %rax,%rdx
  467cef:	cmp    0xc8(%rsp),%rdx
  467cf7:	jne    467d0b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x25b>
  467cf9:	mov    0xd0(%rsp),%rdx
  467d01:	mov    0xd8(%rsp),%rsi
  467d09:	jmp    467d0f <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x25f>
  467d0b:	xor    %edx,%edx
  467d0d:	xor    %esi,%esi
  467d0f:	add    0xb8(%rsp),%rdx
  467d17:	add    %rcx,%rsi
  467d1a:	mov    %rax,0x0(%rbp)
  467d1e:	mov    %rdx,0x8(%rbp)
  467d22:	mov    %rsi,0x10(%rbp)
  467d26:	add    $0x178,%rsp
  467d2d:	pop    %rbx
  467d2e:	pop    %r12
  467d30:	pop    %r13
  467d32:	pop    %r14
  467d34:	pop    %r15
  467d36:	pop    %rbp
  467d37:	vzeroupper
  467d3a:	ret
  467d3b:	lea    -0x3dd797(%rip),%rdi        # 8a5ab <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x21b>
  467d42:	lea    0x8927f(%rip),%rdx        # 4f0fc8 <__frame_dummy_init_array_entry+0x7b18>
  467d49:	mov    $0x45,%esi
  467d4e:	call   ef4b0 <core::panicking::panic_fmt>
  467d53:	lea    -0x44f151(%rip),%rdi        # 18c09 <miniz_oxide::inflate::core::reverse_bits::REVERSED_BITS_LOOKUP+0x611>
  467d5a:	lea    0x82687(%rip),%rdx        # 4ea3e8 <__frame_dummy_init_array_entry+0xf38>
  467d61:	mov    $0x2b,%esi
  467d66:	call   f1580 <core::panicking::panic>
  467d6b:	lea    -0x3dd7e5(%rip),%rdi        # 8a58d <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1fd>
  467d72:	lea    0x8921f(%rip),%rdx        # 4f0f98 <__frame_dummy_init_array_entry+0x7ae8>
  467d79:	mov    $0x1e,%esi
  467d7e:	call   f1580 <core::panicking::panic>
  467d83:	sub    $0xffffffffffffff80,%rsi
  467d87:	lea    0xb0(%rsp),%rdi
  467d8f:	lea    0x38(%rsp),%rdx
  467d94:	call   4851e0 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>>
  467d99:	jmp    467cd4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x224>
  467d9e:	sub    $0xffffffffffffff80,%rsi
  467da2:	lea    0xb0(%rsp),%rdi
  467daa:	lea    0x38(%rsp),%rcx
  467daf:	call   4857a0 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>)>>
  467db4:	jmp    467cd4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>+0x224>
