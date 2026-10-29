
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000488790 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}>:
  488790:	push   %rbp
  488791:	push   %r15
  488793:	push   %r14
  488795:	push   %r13
  488797:	push   %r12
  488799:	push   %rbx
  48879a:	sub    $0x168,%rsp
  4887a1:	vmovups (%rsi),%zmm0
  4887a7:	mov    %ecx,0x8(%rsp)
  4887ab:	mov    0x100(%rdx),%rcx
  4887b2:	lea    0x110(%rdx),%rax
  4887b9:	mov    %rdi,0x18(%rsp)
  4887be:	mov    %rsi,%r12
  4887c1:	mov    %rdx,%r14
  4887c4:	mov    %rax,0xa0(%rsp)
  4887cc:	movq   $0x0,0xa8(%rsp)
  4887d8:	mov    %rcx,0xb0(%rsp)
  4887e0:	movb   $0x0,0xb8(%rsp)
  4887e8:	vmovups %zmm0,0x40(%rsp)
  4887f0:	movq   $0x0,0x80(%rsp)
  4887fc:	mov    0x118(%rdx),%rax
  488803:	mov    0x108(%rax),%rbx
  48880a:	mov    0x100(%rax),%r15
  488811:	mov    0x118(%rdx),%rcx
  488818:	mov    0x108(%rcx),%rbp
  48881f:	mov    0x100(%rcx),%rax
  488826:	mov    0x128(%rdx),%rsi
  48882d:	mov    %rbp,%rcx
  488830:	sub    %rax,%rcx
  488833:	cmp    %rsi,%rcx
  488836:	jge    488b76 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x3e6>
  48883c:	mov    0x120(%r14),%rax
  488843:	dec    %rsi
  488846:	sub    %r15,%rbx
  488849:	lea    0x7b0(%rip),%r15        # 489000 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute>
  488850:	lea    0x40(%rsp),%r13
  488855:	movabs $0x100000000,%rcx
  48885f:	and    %rbp,%rsi
  488862:	inc    %rbp
  488865:	shl    $0x4,%rsi
  488869:	mov    %r15,(%rax,%rsi,1)
  48886d:	mov    %r13,0x8(%rax,%rsi,1)
  488872:	mov    0x118(%r14),%rdx
  488879:	mov    %rbp,0x108(%rdx)
  488880:	mov    0x110(%r14),%rsi
  488887:	lea    0x1d8(%rsi),%rdi
  48888e:	xchg   %ax,%ax
  488890:	mov    0x1f0(%rsi),%rax
  488897:	test   %rcx,%rax
  48889a:	jne    4888ba <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x12a>
  48889c:	mov    %rax,%rdx
  48889f:	or     %rcx,%rdx
  4888a2:	lock cmpxchg %rdx,0x1f0(%rsi)
  4888ab:	jne    488890 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x100>
  4888ad:	mov    %rdx,%rax
  4888b0:	and    $0xffff,%rax
  4888b6:	jne    4888c8 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x138>
  4888b8:	jmp    4888dc <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x14c>
  4888ba:	mov    %rax,%rdx
  4888bd:	mov    %rdx,%rax
  4888c0:	and    $0xffff,%rax
  4888c6:	je     4888dc <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x14c>
  4888c8:	test   %rbx,%rbx
  4888cb:	jg     488b64 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x3d4>
  4888d1:	shr    $0x10,%edx
  4888d4:	cmp    %eax,%edx
  4888d6:	je     488b64 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x3d4>
  4888dc:	vmovups 0x58(%r12),%xmm0
  4888e3:	mov    0x40(%r12),%rax
  4888e8:	mov    0x48(%r12),%rdx
  4888ed:	mov    0x50(%r12),%rdi
  4888f2:	mov    0x68(%r12),%r9
  4888f7:	mov    0x70(%r12),%r10
  4888fc:	vmovups %xmm0,0xe8(%rsp)
  488905:	mov    (%rax),%rsi
  488908:	mov    (%rdx),%rcx
  48890b:	mov    0x8(%rdx),%r8
  48890f:	mov    %rdi,0xe0(%rsp)
  488917:	movzbl 0x8(%rsp),%ebp
  48891c:	lea    0x20(%rsp),%r12
  488921:	lea    0xe0(%rsp),%rax
  488929:	mov    %r12,%rdi
  48892c:	mov    %ebp,%edx
  48892e:	push   %rax
  48892f:	push   %r10
  488931:	vzeroupper
  488934:	call   467dd0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>>
  488939:	add    $0x10,%rsp
  48893d:	mov    0x20(%rsp),%rax
  488942:	mov    0x28(%rsp),%rcx
  488947:	mov    0x30(%rsp),%rbx
  48894c:	mov    %rax,0x10(%rsp)
  488951:	mov    %rcx,0x8(%rsp)
  488956:	cs nopw 0x0(%rax,%rax,1)
  488960:	mov    0xa8(%rsp),%rax
  488968:	cmp    $0x3,%rax
  48896c:	je     4889a5 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x215>
  48896e:	mov    %r14,%rdi
  488971:	call   1c3a80 <<rayon_core::registry::WorkerThread>::take_local_job>
  488976:	test   %rax,%rax
  488979:	je     488993 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x203>
  48897b:	mov    %r13,%rcx
  48897e:	mov    %rax,%rsi
  488981:	xor    %rdx,%rcx
  488984:	xor    %r15,%rsi
  488987:	or     %rcx,%rsi
  48898a:	je     4889f9 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x269>
  48898c:	mov    %rdx,%rdi
  48898f:	call   *%rax
  488991:	jmp    488960 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x1d0>
  488993:	mov    0xa8(%rsp),%rax
  48899b:	cmp    $0x3,%rax
  48899f:	jne    488bac <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x41c>
  4889a5:	vmovdqu 0x88(%rsp),%xmm0
  4889ae:	mov    0x80(%rsp),%rax
  4889b6:	cmp    $0x1,%rax
  4889ba:	je     4889d6 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x246>
  4889bc:	cmp    $0x2,%rax
  4889c0:	jne    488b94 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x404>
  4889c6:	vmovq  %xmm0,%rdi
  4889cb:	vpextrq $0x1,%xmm0,%rsi
  4889d1:	call   1058c0 <rayon_core::unwind::resume_unwinding>
  4889d6:	mov    0x18(%rsp),%rcx
  4889db:	mov    0x10(%rsp),%rdx
  4889e0:	mov    0x8(%rsp),%rsi
  4889e5:	mov    0x98(%rsp),%rax
  4889ed:	mov    %rdx,(%rcx)
  4889f0:	mov    %rsi,0x8(%rcx)
  4889f4:	jmp    488b45 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x3b5>
  4889f9:	vmovups 0x40(%rsp),%zmm0
  488a01:	vmovups 0x80(%rsp),%zmm1
  488a09:	vmovups %zmm0,0xe0(%rsp)
  488a14:	vmovups %zmm1,0x120(%rsp)
  488a1f:	mov    0xe0(%rsp),%rax
  488a27:	test   %rax,%rax
  488a2a:	je     488bc1 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x431>
  488a30:	vmovups 0x58(%rsp),%xmm0
  488a36:	mov    0x68(%rsp),%rsi
  488a3b:	mov    0xe8(%rsp),%rcx
  488a43:	mov    0xf0(%rsp),%rdx
  488a4b:	mov    0x118(%rsp),%r10
  488a53:	mov    0x110(%rsp),%r9
  488a5b:	mov    %rsi,0x30(%rsp)
  488a60:	vmovaps %xmm0,0x20(%rsp)
  488a66:	mov    (%rax),%rsi
  488a69:	mov    0x8(%rdx),%r8
  488a6d:	sub    (%rcx),%rsi
  488a70:	mov    (%rdx),%rcx
  488a73:	mov    %ebp,%edx
  488a75:	lea    0xc8(%rsp),%rdi
  488a7d:	push   %r12
  488a7f:	push   %r10
  488a81:	vzeroupper
  488a84:	call   467dd0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>>
  488a89:	add    $0x10,%rsp
  488a8d:	cmpq   $0x2,0x120(%rsp)
  488a96:	jb     488b1e <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x38e>
  488a9c:	mov    0x130(%rsp),%r15
  488aa4:	mov    0x128(%rsp),%r14
  488aac:	mov    (%r15),%rax
  488aaf:	test   %rax,%rax
  488ab2:	je     488ab9 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x329>
  488ab4:	mov    %r14,%rdi
  488ab7:	call   *%rax
  488ab9:	mov    0x8(%r15),%rsi
  488abd:	test   %rsi,%rsi
  488ac0:	je     488b1e <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x38e>
  488ac2:	mov    0x10(%r15),%rax
  488ac6:	cmp    $0x20000,%rsi
  488acd:	setae  %cl
  488ad0:	cmp    $0x1001,%rax
  488ad6:	setb   %dl
  488ad9:	test   %cl,%dl
  488adb:	je     488b13 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x383>
  488add:	blsr   %eax,%ecx
  488ae2:	jne    488bd2 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x442>
  488ae8:	movabs $0x8000000000000000,%rcx
  488af2:	sub    %rax,%rcx
  488af5:	cmp    %rcx,%rsi
  488af8:	jae    488bd2 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x442>
  488afe:	cmpb   $0x1,(%r14,%rsi,1)
  488b03:	jne    488b15 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x385>
  488b05:	inc    %rsi
  488b08:	mov    %r14,%rdi
  488b0b:	call   *0x6cbd7(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  488b11:	jmp    488b1e <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x38e>
  488b13:	jmp    488b15 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x385>
  488b15:	mov    %r14,%rdi
  488b18:	call   *0x6cbd2(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  488b1e:	mov    0x18(%rsp),%rcx
  488b23:	mov    0x10(%rsp),%rax
  488b28:	vmovdqu 0xc8(%rsp),%xmm0
  488b31:	mov    0x8(%rsp),%rdx
  488b36:	mov    %rax,(%rcx)
  488b39:	mov    0xd8(%rsp),%rax
  488b41:	mov    %rdx,0x8(%rcx)
  488b45:	mov    %rbx,0x10(%rcx)
  488b49:	vmovdqu %xmm0,0x18(%rcx)
  488b4e:	mov    %rax,0x28(%rcx)
  488b52:	add    $0x168,%rsp
  488b59:	pop    %rbx
  488b5a:	pop    %r12
  488b5c:	pop    %r13
  488b5e:	pop    %r14
  488b60:	pop    %r15
  488b62:	pop    %rbp
  488b63:	ret
  488b64:	mov    $0x1,%esi
  488b69:	vzeroupper
  488b6c:	call   ffe10 <<rayon_core::sleep::Sleep>::wake_any_threads>
  488b71:	jmp    4888dc <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x14c>
  488b76:	lea    0x118(%r14),%rdi
  488b7d:	add    %rsi,%rsi
  488b80:	vzeroupper
  488b83:	call   1c3530 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::resize>
  488b88:	mov    0x128(%r14),%rsi
  488b8f:	jmp    48883c <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0xac>
  488b94:	lea    -0x46990f(%rip),%rdi        # 1f28c <zmij::STATIC_DATA+0x4d4c>
  488b9b:	lea    0x6a47e(%rip),%rdx        # 4f3020 <__frame_dummy_init_array_entry+0x66c0>
  488ba2:	mov    $0x28,%esi
  488ba7:	call   f1800 <core::panicking::panic>
  488bac:	lea    0xa8(%rsp),%rsi
  488bb4:	mov    %r14,%rdi
  488bb7:	call   ff5d0 <<rayon_core::registry::WorkerThread>::wait_until_cold>
  488bbc:	jmp    4889a5 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x215>
  488bc1:	lea    0x6a218(%rip),%rdi        # 4f2de0 <__frame_dummy_init_array_entry+0x6480>
  488bc8:	vzeroupper
  488bcb:	call   f2b20 <core::option::unwrap_failed>
  488bd0:	jmp    488c17 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x487>
  488bd2:	call   111490 <std::process::abort>
  488bd7:	jmp    488c1b <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x48b>
  488bd9:	mov    0x8(%r15),%rsi
  488bdd:	mov    %rax,%rbx
  488be0:	test   %rsi,%rsi
  488be3:	je     488c3f <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x4af>
  488be5:	mov    0x10(%r15),%rdx
  488be9:	mov    %r14,%rdi
  488bec:	call   e9c50 <__rustc::__rust_dealloc>
  488bf1:	mov    %rbx,%rdi
  488bf4:	call   4eb910 <_Unwind_Resume@plt>
  488bf9:	mov    %rax,%rdi
  488bfc:	call   14537a <std::panicking::catch_unwind::cleanup>
  488c01:	mov    %rdx,%rcx
  488c04:	lea    0xa0(%rsp),%rsi
  488c0c:	mov    %r14,%rdi
  488c0f:	mov    %rax,%rdx
  488c12:	call   105d70 <rayon_core::join::join_recover_from_panic>
  488c17:	ud2
  488c19:	jmp    488c1b <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x48b>
  488c1b:	mov    %rax,%rbx
  488c1e:	lea    0x40(%rsp),%rdi
  488c23:	call   2215a0 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 4]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<zopro_bn254::fp::Fp<zopro_bn254::FrConfig>>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 4]>, <zopro_bn254::fp::Fp<zopro_bn254::FrConfig>>::to_canonical>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 4]>>>>
  488c28:	jmp    488c3f <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>>>::{closure#0}+0x4af>
  488c2a:	call   f2c93 <core::panicking::panic_in_cleanup>
  488c2f:	mov    %rax,%rbx
  488c32:	lea    0x120(%rsp),%rdi
  488c3a:	call   2214f0 <core::ptr::drop_glue::<core::cell::UnsafeCell<rayon_core::job::JobResult<rayon::iter::collect::consumer::CollectResult<[u64; 4]>>>>>
  488c3f:	mov    %rbx,%rdi
  488c42:	call   4eb910 <_Unwind_Resume@plt>
  488c47:	call   f2c93 <core::panicking::panic_in_cleanup>
