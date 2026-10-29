
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047f9f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>>:
  47f9f0:	push   %rbp
  47f9f1:	push   %r15
  47f9f3:	push   %r14
  47f9f5:	push   %r13
  47f9f7:	push   %r12
  47f9f9:	push   %rbx
  47f9fa:	sub    $0xe8,%rsp
  47fa01:	mov    0x128(%rsp),%r15
  47fa09:	mov    0x120(%rsp),%rbx
  47fa11:	mov    %rsi,0x50(%rsp)
  47fa16:	shr    $1,%rsi
  47fa19:	mov    %rcx,0x58(%rsp)
  47fa1e:	mov    %r9,0x8(%rsp)
  47fa23:	mov    %rdi,%r13
  47fa26:	mov    %r8,0x60(%rsp)
  47fa2b:	cmp    %r8,%rsi
  47fa2e:	jae    47fab2 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xc2>
  47fa34:	mov    0x8(%rsp),%r14
  47fa39:	mov    0x8(%r15),%rax
  47fa3d:	mov    0x10(%r15),%rcx
  47fa41:	mov    %r13,0x48(%rsp)
  47fa46:	sub    %rbx,%r14
  47fa49:	mov    %rcx,(%rsp)
  47fa4d:	mov    %rax,0x10(%rsp)
  47fa52:	jae    47facc <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xdc>
  47fa54:	mov    (%r15),%r13
  47fa57:	lea    0x10(%rax),%rbp
  47fa5b:	xor    %r12d,%r12d
  47fa5e:	xchg   %ax,%ax
  47fa60:	mov    0x8(%rsp),%rax
  47fa65:	lea    (%rax,%r12,1),%rdx
  47fa69:	lea    0x18(%rsp),%rdi
  47fa6e:	mov    %r13,%rsi
  47fa71:	call   47fdb0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut>
  47fa76:	mov    0x18(%rsp),%r15
  47fa7b:	cmp    $0xffffffffffffffff,%r15
  47fa7f:	je     47facf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xdf>
  47fa81:	mov    0x20(%rsp),%rbx
  47fa86:	cmp    %r12,(%rsp)
  47fa8a:	je     47fcf1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x301>
  47fa90:	mov    0x28(%rsp),%rax
  47fa95:	inc    %r12
  47fa98:	mov    %r15,-0x10(%rbp)
  47fa9c:	mov    %rbx,-0x8(%rbp)
  47faa0:	mov    %rax,0x0(%rbp)
  47faa4:	add    $0x18,%rbp
  47faa8:	mov    %r14,%rax
  47faab:	add    %r12,%rax
  47faae:	jne    47fa60 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x70>
  47fab0:	jmp    47facf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xdf>
  47fab2:	test   %dl,%dl
  47fab4:	je     47faed <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xfd>
  47fab6:	mov    %fs:0xffffffffffffffc0,%rax
  47fabf:	test   %rax,%rax
  47fac2:	je     47fafb <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x10b>
  47fac4:	add    $0x110,%rax
  47faca:	jmp    47fb0c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x11c>
  47facc:	xor    %r12d,%r12d
  47facf:	mov    0x48(%rsp),%rax
  47fad4:	mov    0x10(%rsp),%rcx
  47fad9:	mov    (%rsp),%rdx
  47fadd:	mov    %rcx,(%rax)
  47fae0:	mov    %rdx,0x8(%rax)
  47fae4:	mov    %r12,0x10(%rax)
  47fae8:	jmp    47fcdf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ef>
  47faed:	test   %rcx,%rcx
  47faf0:	je     47fa34 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x44>
  47faf6:	shr    $1,%rcx
  47faf9:	jmp    47fb20 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x130>
  47fafb:	mov    %rsi,%r14
  47fafe:	mov    %rcx,%r12
  47fb01:	call   105550 <rayon_core::registry::global_registry>
  47fb06:	mov    %r12,%rcx
  47fb09:	mov    %r14,%rsi
  47fb0c:	mov    (%rax),%rax
  47fb0f:	shr    $1,%rcx
  47fb12:	mov    0x208(%rax),%rax
  47fb19:	cmp    %rax,%rcx
  47fb1c:	cmovbe %rax,%rcx
  47fb20:	mov    %rcx,0x58(%rsp)
  47fb25:	xor    %eax,%eax
  47fb27:	mov    %rbx,%rcx
  47fb2a:	sub    0x8(%rsp),%rcx
  47fb2f:	mov    %rsi,0x68(%rsp)
  47fb34:	cmovae %rcx,%rax
  47fb38:	cmp    %rax,%rsi
  47fb3b:	ja     47fd0b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x31b>
  47fb41:	mov    0x10(%r15),%rax
  47fb45:	sub    %rsi,%rax
  47fb48:	jb     47fd23 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x333>
  47fb4e:	mov    0x8(%r15),%rdi
  47fb52:	mov    0x8(%rsp),%r11
  47fb57:	mov    (%r15),%rdx
  47fb5a:	lea    (%rsi,%rsi,2),%r8
  47fb5e:	lea    0x50(%rsp),%r9
  47fb63:	lea    0x58(%rsp),%r10
  47fb68:	mov    %r9,0x70(%rsp)
  47fb6d:	lea    0x68(%rsp),%r9
  47fb72:	mov    %r9,0x78(%rsp)
  47fb77:	mov    %r10,0x80(%rsp)
  47fb7f:	lea    (%rdi,%r8,8),%r8
  47fb83:	lea    (%rsi,%r11,1),%rcx
  47fb87:	mov    %rdx,0x88(%rsp)
  47fb8f:	mov    %r8,0x90(%rsp)
  47fb97:	mov    %rax,0x98(%rsp)
  47fb9f:	mov    %rcx,0xa0(%rsp)
  47fba7:	mov    %rbx,0xa8(%rsp)
  47fbaf:	mov    %r9,0xb0(%rsp)
  47fbb7:	mov    %r10,0xb8(%rsp)
  47fbbf:	mov    %rdx,0xc0(%rsp)
  47fbc7:	mov    %fs:0xffffffffffffffc0,%rdx
  47fbd0:	mov    %rdi,0xc8(%rsp)
  47fbd8:	mov    %rsi,0xd0(%rsp)
  47fbe0:	mov    %r11,0xd8(%rsp)
  47fbe8:	mov    %rcx,0xe0(%rsp)
  47fbf0:	test   %rdx,%rdx
  47fbf3:	jne    47fc1c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x22c>
  47fbf5:	call   105550 <rayon_core::registry::global_registry>
  47fbfa:	mov    (%rax),%rsi
  47fbfd:	mov    %fs:0xffffffffffffffc0,%rdx
  47fc06:	test   %rdx,%rdx
  47fc09:	je     47fd3b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x34b>
  47fc0f:	cmp    %rsi,0x110(%rdx)
  47fc16:	jne    47fd53 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x363>
  47fc1c:	lea    0x18(%rsp),%rdi
  47fc21:	lea    0x70(%rsp),%rsi
  47fc26:	xor    %ecx,%ecx
  47fc28:	call   487c00 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}>
  47fc2d:	mov    0x28(%rsp),%rcx
  47fc32:	vmovdqu 0x20(%rsp),%xmm0
  47fc38:	mov    0x18(%rsp),%rax
  47fc3d:	mov    0x30(%rsp),%rbx
  47fc42:	lea    (%rcx,%rcx,2),%rcx
  47fc46:	lea    (%rax,%rcx,8),%rcx
  47fc4a:	cmp    %rbx,%rcx
  47fc4d:	je     47fccf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2df>
  47fc53:	mov    0x40(%rsp),%r14
  47fc58:	mov    %rax,0x0(%r13)
  47fc5c:	vmovdqu %xmm0,0x8(%r13)
  47fc62:	test   %r14,%r14
  47fc65:	je     47fcdf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ef>
  47fc67:	mov    0x75a7a(%rip),%r12        # 4f56e8 <munmap@GLIBC_2.2.5>
  47fc6e:	mov    0x75a7b(%rip),%r13        # 4f56f0 <free@GLIBC_2.2.5>
  47fc75:	add    $0x8,%rbx
  47fc79:	movabs $0x7ffffffffffffff8,%r15
  47fc83:	jmp    47fc9c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ac>
  47fc85:	data16 cs nopw 0x0(%rax,%rax,1)
  47fc90:	call   *%r13
  47fc93:	add    $0x18,%rbx
  47fc97:	dec    %r14
  47fc9a:	je     47fcdf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ef>
  47fc9c:	mov    -0x8(%rbx),%rax
  47fca0:	test   %rax,%rax
  47fca3:	je     47fc93 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a3>
  47fca5:	mov    (%rbx),%rdi
  47fca8:	cmp    $0x3ff,%rax
  47fcae:	jbe    47fc90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a0>
  47fcb0:	shl    $0x7,%rax
  47fcb4:	mov    %rax,%rsi
  47fcb7:	or     $0x1,%rsi
  47fcbb:	cmp    %r15,%rsi
  47fcbe:	ja     47fd6b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x37b>
  47fcc4:	cmpb   $0x1,(%rdi,%rax,1)
  47fcc8:	jne    47fc90 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a0>
  47fcca:	call   *%r12
  47fccd:	jmp    47fc93 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a3>
  47fccf:	vpaddq 0x38(%rsp),%xmm0,%xmm0
  47fcd5:	mov    %rax,0x0(%r13)
  47fcd9:	vmovdqu %xmm0,0x8(%r13)
  47fcdf:	add    $0xe8,%rsp
  47fce6:	pop    %rbx
  47fce7:	pop    %r12
  47fce9:	pop    %r13
  47fceb:	pop    %r14
  47fced:	pop    %r15
  47fcef:	pop    %rbp
  47fcf0:	ret
  47fcf1:	lea    -0x3f570d(%rip),%rdi        # 8a5eb <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x21b>
  47fcf8:	lea    0x747d9(%rip),%rdx        # 4f44d8 <__frame_dummy_init_array_entry+0x7b78>
  47fcff:	mov    $0x45,%esi
  47fd04:	call   ef730 <core::panicking::panic_fmt>
  47fd09:	ud2
  47fd0b:	lea    -0x4670c9(%rip),%rdi        # 18c49 <miniz_oxide::inflate::core::reverse_bits::REVERSED_BITS_LOOKUP+0x611>
  47fd12:	lea    0x6db7f(%rip),%rdx        # 4ed898 <__frame_dummy_init_array_entry+0xf38>
  47fd19:	mov    $0x2b,%esi
  47fd1e:	call   f1800 <core::panicking::panic>
  47fd23:	lea    -0x3f575d(%rip),%rdi        # 8a5cd <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1fd>
  47fd2a:	lea    0x74777(%rip),%rdx        # 4f44a8 <__frame_dummy_init_array_entry+0x7b48>
  47fd31:	mov    $0x1e,%esi
  47fd36:	call   f1800 <core::panicking::panic>
  47fd3b:	sub    $0xffffffffffffff80,%rsi
  47fd3f:	lea    0x18(%rsp),%rdi
  47fd44:	lea    0x70(%rsp),%rdx
  47fd49:	call   487b00 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>
  47fd4e:	jmp    47fc2d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x23d>
  47fd53:	sub    $0xffffffffffffff80,%rsi
  47fd57:	lea    0x18(%rsp),%rdi
  47fd5c:	lea    0x70(%rsp),%rcx
  47fd61:	call   4880a0 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>
  47fd66:	jmp    47fc2d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x23d>
  47fd6b:	call   111490 <std::process::abort>
  47fd70:	mov    %rax,%r14
  47fd73:	jmp    47fd95 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x3a5>
  47fd75:	mov    %rax,%r14
  47fd78:	test   %r15,%r15
  47fd7b:	je     47fd91 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x3a1>
  47fd7d:	shl    $0x7,%r15
  47fd81:	mov    $0x8,%edx
  47fd86:	mov    %rbx,%rdi
  47fd89:	mov    %r15,%rsi
  47fd8c:	call   e9c50 <__rustc::__rust_dealloc>
  47fd91:	mov    (%rsp),%r12
  47fd95:	mov    0x10(%rsp),%rdi
  47fd9a:	mov    %r12,%rsi
  47fd9d:	call   294930 <core::ptr::drop_glue::<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47fda2:	mov    %r14,%rdi
  47fda5:	call   4eb910 <_Unwind_Resume@plt>
