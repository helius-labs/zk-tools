
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047f900 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>>:
  47f900:	push   %rbp
  47f901:	push   %r15
  47f903:	push   %r14
  47f905:	push   %r13
  47f907:	push   %r12
  47f909:	push   %rbx
  47f90a:	sub    $0xe8,%rsp
  47f911:	mov    0x128(%rsp),%r15
  47f919:	mov    0x120(%rsp),%rbx
  47f921:	mov    %rsi,0x50(%rsp)
  47f926:	shr    $1,%rsi
  47f929:	mov    %rcx,0x58(%rsp)
  47f92e:	mov    %r9,0x8(%rsp)
  47f933:	mov    %rdi,%r13
  47f936:	mov    %r8,0x60(%rsp)
  47f93b:	cmp    %r8,%rsi
  47f93e:	jae    47f9c2 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xc2>
  47f944:	mov    0x8(%rsp),%r14
  47f949:	mov    0x8(%r15),%rax
  47f94d:	mov    0x10(%r15),%rcx
  47f951:	mov    %r13,0x48(%rsp)
  47f956:	sub    %rbx,%r14
  47f959:	mov    %rcx,(%rsp)
  47f95d:	mov    %rax,0x10(%rsp)
  47f962:	jae    47f9dc <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xdc>
  47f964:	mov    (%r15),%r13
  47f967:	lea    0x10(%rax),%rbp
  47f96b:	xor    %r12d,%r12d
  47f96e:	xchg   %ax,%ax
  47f970:	mov    0x8(%rsp),%rax
  47f975:	lea    (%rax,%r12,1),%rdx
  47f979:	lea    0x18(%rsp),%rdi
  47f97e:	mov    %r13,%rsi
  47f981:	call   47fcc0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut>
  47f986:	mov    0x18(%rsp),%r15
  47f98b:	cmp    $0xffffffffffffffff,%r15
  47f98f:	je     47f9df <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xdf>
  47f991:	mov    0x20(%rsp),%rbx
  47f996:	cmp    %r12,(%rsp)
  47f99a:	je     47fc01 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x301>
  47f9a0:	mov    0x28(%rsp),%rax
  47f9a5:	inc    %r12
  47f9a8:	mov    %r15,-0x10(%rbp)
  47f9ac:	mov    %rbx,-0x8(%rbp)
  47f9b0:	mov    %rax,0x0(%rbp)
  47f9b4:	add    $0x18,%rbp
  47f9b8:	mov    %r14,%rax
  47f9bb:	add    %r12,%rax
  47f9be:	jne    47f970 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x70>
  47f9c0:	jmp    47f9df <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xdf>
  47f9c2:	test   %dl,%dl
  47f9c4:	je     47f9fd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0xfd>
  47f9c6:	mov    %fs:0xffffffffffffffc0,%rax
  47f9cf:	test   %rax,%rax
  47f9d2:	je     47fa0b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x10b>
  47f9d4:	add    $0x110,%rax
  47f9da:	jmp    47fa1c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x11c>
  47f9dc:	xor    %r12d,%r12d
  47f9df:	mov    0x48(%rsp),%rax
  47f9e4:	mov    0x10(%rsp),%rcx
  47f9e9:	mov    (%rsp),%rdx
  47f9ed:	mov    %rcx,(%rax)
  47f9f0:	mov    %rdx,0x8(%rax)
  47f9f4:	mov    %r12,0x10(%rax)
  47f9f8:	jmp    47fbef <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ef>
  47f9fd:	test   %rcx,%rcx
  47fa00:	je     47f944 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x44>
  47fa06:	shr    $1,%rcx
  47fa09:	jmp    47fa30 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x130>
  47fa0b:	mov    %rsi,%r14
  47fa0e:	mov    %rcx,%r12
  47fa11:	call   105490 <rayon_core::registry::global_registry>
  47fa16:	mov    %r12,%rcx
  47fa19:	mov    %r14,%rsi
  47fa1c:	mov    (%rax),%rax
  47fa1f:	shr    $1,%rcx
  47fa22:	mov    0x208(%rax),%rax
  47fa29:	cmp    %rax,%rcx
  47fa2c:	cmovbe %rax,%rcx
  47fa30:	mov    %rcx,0x58(%rsp)
  47fa35:	xor    %eax,%eax
  47fa37:	mov    %rbx,%rcx
  47fa3a:	sub    0x8(%rsp),%rcx
  47fa3f:	mov    %rsi,0x68(%rsp)
  47fa44:	cmovae %rcx,%rax
  47fa48:	cmp    %rax,%rsi
  47fa4b:	ja     47fc1b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x31b>
  47fa51:	mov    0x10(%r15),%rax
  47fa55:	sub    %rsi,%rax
  47fa58:	jb     47fc33 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x333>
  47fa5e:	mov    0x8(%r15),%rdi
  47fa62:	mov    0x8(%rsp),%r11
  47fa67:	mov    (%r15),%rdx
  47fa6a:	lea    (%rsi,%rsi,2),%r8
  47fa6e:	lea    0x50(%rsp),%r9
  47fa73:	lea    0x58(%rsp),%r10
  47fa78:	mov    %r9,0x70(%rsp)
  47fa7d:	lea    0x68(%rsp),%r9
  47fa82:	mov    %r9,0x78(%rsp)
  47fa87:	mov    %r10,0x80(%rsp)
  47fa8f:	lea    (%rdi,%r8,8),%r8
  47fa93:	lea    (%rsi,%r11,1),%rcx
  47fa97:	mov    %rdx,0x88(%rsp)
  47fa9f:	mov    %r8,0x90(%rsp)
  47faa7:	mov    %rax,0x98(%rsp)
  47faaf:	mov    %rcx,0xa0(%rsp)
  47fab7:	mov    %rbx,0xa8(%rsp)
  47fabf:	mov    %r9,0xb0(%rsp)
  47fac7:	mov    %r10,0xb8(%rsp)
  47facf:	mov    %rdx,0xc0(%rsp)
  47fad7:	mov    %fs:0xffffffffffffffc0,%rdx
  47fae0:	mov    %rdi,0xc8(%rsp)
  47fae8:	mov    %rsi,0xd0(%rsp)
  47faf0:	mov    %r11,0xd8(%rsp)
  47faf8:	mov    %rcx,0xe0(%rsp)
  47fb00:	test   %rdx,%rdx
  47fb03:	jne    47fb2c <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x22c>
  47fb05:	call   105490 <rayon_core::registry::global_registry>
  47fb0a:	mov    (%rax),%rsi
  47fb0d:	mov    %fs:0xffffffffffffffc0,%rdx
  47fb16:	test   %rdx,%rdx
  47fb19:	je     47fc4b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x34b>
  47fb1f:	cmp    %rsi,0x110(%rdx)
  47fb26:	jne    47fc63 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x363>
  47fb2c:	lea    0x18(%rsp),%rdi
  47fb31:	lea    0x70(%rsp),%rsi
  47fb36:	xor    %ecx,%ecx
  47fb38:	call   487ac0 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}>
  47fb3d:	mov    0x28(%rsp),%rcx
  47fb42:	vmovdqu 0x20(%rsp),%xmm0
  47fb48:	mov    0x18(%rsp),%rax
  47fb4d:	mov    0x30(%rsp),%rbx
  47fb52:	lea    (%rcx,%rcx,2),%rcx
  47fb56:	lea    (%rax,%rcx,8),%rcx
  47fb5a:	cmp    %rbx,%rcx
  47fb5d:	je     47fbdf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2df>
  47fb63:	mov    0x40(%rsp),%r14
  47fb68:	mov    %rax,0x0(%r13)
  47fb6c:	vmovdqu %xmm0,0x8(%r13)
  47fb72:	test   %r14,%r14
  47fb75:	je     47fbef <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ef>
  47fb77:	mov    0x75a2a(%rip),%r12        # 4f55a8 <munmap@GLIBC_2.2.5>
  47fb7e:	mov    0x75a2b(%rip),%r13        # 4f55b0 <free@GLIBC_2.2.5>
  47fb85:	add    $0x8,%rbx
  47fb89:	movabs $0x7ffffffffffffff8,%r15
  47fb93:	jmp    47fbac <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ac>
  47fb95:	data16 cs nopw 0x0(%rax,%rax,1)
  47fba0:	call   *%r13
  47fba3:	add    $0x18,%rbx
  47fba7:	dec    %r14
  47fbaa:	je     47fbef <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2ef>
  47fbac:	mov    -0x8(%rbx),%rax
  47fbb0:	test   %rax,%rax
  47fbb3:	je     47fba3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a3>
  47fbb5:	mov    (%rbx),%rdi
  47fbb8:	cmp    $0x3ff,%rax
  47fbbe:	jbe    47fba0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a0>
  47fbc0:	shl    $0x7,%rax
  47fbc4:	mov    %rax,%rsi
  47fbc7:	or     $0x1,%rsi
  47fbcb:	cmp    %r15,%rsi
  47fbce:	ja     47fc7b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x37b>
  47fbd4:	cmpb   $0x1,(%rdi,%rax,1)
  47fbd8:	jne    47fba0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a0>
  47fbda:	call   *%r12
  47fbdd:	jmp    47fba3 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x2a3>
  47fbdf:	vpaddq 0x38(%rsp),%xmm0,%xmm0
  47fbe5:	mov    %rax,0x0(%r13)
  47fbe9:	vmovdqu %xmm0,0x8(%r13)
  47fbef:	add    $0xe8,%rsp
  47fbf6:	pop    %rbx
  47fbf7:	pop    %r12
  47fbf9:	pop    %r13
  47fbfb:	pop    %r14
  47fbfd:	pop    %r15
  47fbff:	pop    %rbp
  47fc00:	ret
  47fc01:	lea    -0x3f561d(%rip),%rdi        # 8a5eb <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x21b>
  47fc08:	lea    0x74789(%rip),%rdx        # 4f4398 <__frame_dummy_init_array_entry+0x7b78>
  47fc0f:	mov    $0x45,%esi
  47fc14:	call   ef670 <core::panicking::panic_fmt>
  47fc19:	ud2
  47fc1b:	lea    -0x466fd9(%rip),%rdi        # 18c49 <miniz_oxide::inflate::core::reverse_bits::REVERSED_BITS_LOOKUP+0x611>
  47fc22:	lea    0x6db2f(%rip),%rdx        # 4ed758 <__frame_dummy_init_array_entry+0xf38>
  47fc29:	mov    $0x2b,%esi
  47fc2e:	call   f1740 <core::panicking::panic>
  47fc33:	lea    -0x3f566d(%rip),%rdi        # 8a5cd <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1fd>
  47fc3a:	lea    0x74727(%rip),%rdx        # 4f4368 <__frame_dummy_init_array_entry+0x7b48>
  47fc41:	mov    $0x1e,%esi
  47fc46:	call   f1740 <core::panicking::panic>
  47fc4b:	sub    $0xffffffffffffff80,%rsi
  47fc4f:	lea    0x18(%rsp),%rdi
  47fc54:	lea    0x70(%rsp),%rdx
  47fc59:	call   4879c0 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>
  47fc5e:	jmp    47fb3d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x23d>
  47fc63:	sub    $0xffffffffffffff80,%rsi
  47fc67:	lea    0x18(%rsp),%rdi
  47fc6c:	lea    0x70(%rsp),%rcx
  47fc71:	call   487f60 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>
  47fc76:	jmp    47fb3d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x23d>
  47fc7b:	call   1113d0 <std::process::abort>
  47fc80:	mov    %rax,%r14
  47fc83:	jmp    47fca5 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x3a5>
  47fc85:	mov    %rax,%r14
  47fc88:	test   %r15,%r15
  47fc8b:	je     47fca1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>+0x3a1>
  47fc8d:	shl    $0x7,%r15
  47fc91:	mov    $0x8,%edx
  47fc96:	mov    %rbx,%rdi
  47fc99:	mov    %r15,%rsi
  47fc9c:	call   e9b90 <__rustc::__rust_dealloc>
  47fca1:	mov    (%rsp),%r12
  47fca5:	mov    0x10(%rsp),%rdi
  47fcaa:	mov    %r12,%rsi
  47fcad:	call   294870 <core::ptr::drop_glue::<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47fcb2:	mov    %r14,%rdi
  47fcb5:	call   4eb7d0 <_Unwind_Resume@plt>
