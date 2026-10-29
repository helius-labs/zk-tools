
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000484650 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>:
  484650:	push   %r14
  484652:	push   %rbx
  484653:	sub    $0xb8,%rsp
  48465a:	vmovups (%rdx),%zmm0
  484660:	vmovups 0x38(%rdx),%zmm1
  48466a:	mov    %fs:0x0,%rcx
  484673:	lea    0x80(%rsp),%r14
  48467b:	mov    %rsi,%rax
  48467e:	mov    %rdi,%rbx
  484681:	lea    -0x28(%rcx),%rcx
  484688:	mov    %rcx,(%rsp)
  48468c:	vmovups %zmm0,0x8(%rsp)
  484697:	vmovups %zmm1,0x40(%rsp)
  48469f:	movq   $0x0,0x80(%rsp)
  4846ab:	lea    0x9be(%rip),%rsi        # 485070 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)> as rayon_core::job::Job>::execute>
  4846b2:	mov    %rsp,%rdx
  4846b5:	mov    %rax,%rdi
  4846b8:	vzeroupper
  4846bb:	call   105880 <<rayon_core::registry::Registry>::inject>
  4846c0:	mov    (%rsp),%rdi
  4846c4:	call   105650 <<rayon_core::latch::LockLatch>::wait_and_reset>
  4846c9:	vmovdqu 0x88(%rsp),%xmm0
  4846d2:	mov    0x80(%rsp),%rax
  4846da:	cmp    $0x1,%rax
  4846de:	je     4846f6 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>+0xa6>
  4846e0:	cmp    $0x2,%rax
  4846e4:	jne    484716 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>+0xc6>
  4846e6:	vmovq  %xmm0,%rdi
  4846eb:	vpextrq $0x1,%xmm0,%rsi
  4846f1:	call   105640 <rayon_core::unwind::resume_unwinding>
  4846f6:	vmovups 0x98(%rsp),%ymm1
  4846ff:	vmovups %ymm1,0x10(%rbx)
  484704:	vmovdqu %xmm0,(%rbx)
  484708:	add    $0xb8,%rsp
  48470f:	pop    %rbx
  484710:	pop    %r14
  484712:	vzeroupper
  484715:	ret
  484716:	lea    -0x4654d1(%rip),%rdi        # 1f24c <zmij::STATIC_DATA+0x4d4c>
  48471d:	lea    0x6b44c(%rip),%rdx        # 4efb70 <__frame_dummy_init_array_entry+0x66c0>
  484724:	mov    $0x28,%esi
  484729:	call   f1580 <core::panicking::panic>
  48472e:	mov    %rax,%rbx
  484731:	mov    %r14,%rdi
  484734:	call   295f50 <core::ptr::drop_glue::<rayon_core::job::JobResult<(rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>)>>>
  484739:	mov    %rbx,%rdi
  48473c:	call   4e8460 <_Unwind_Resume@plt>
  484741:	call   f2a13 <core::panicking::panic_in_cleanup>
