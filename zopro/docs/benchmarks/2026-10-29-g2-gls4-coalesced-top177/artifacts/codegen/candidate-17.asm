
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000489540 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}>:
  489540:	push   %rbp
  489541:	push   %r15
  489543:	push   %r14
  489545:	push   %r13
  489547:	push   %r12
  489549:	push   %rbx
  48954a:	sub    $0x148,%rsp
  489551:	vmovups (%rsi),%zmm0
  489557:	mov    %ecx,(%rsp)
  48955a:	mov    0x100(%rdx),%rcx
  489561:	lea    0x110(%rdx),%rax
  489568:	mov    %rdi,0x8(%rsp)
  48956d:	mov    %rsi,%r12
  489570:	mov    %rdx,%r14
  489573:	mov    %rax,0x70(%rsp)
  489578:	movq   $0x0,0x78(%rsp)
  489581:	mov    %rcx,0x80(%rsp)
  489589:	movb   $0x0,0x88(%rsp)
  489591:	vmovups %zmm0,0x10(%rsp)
  48959c:	movq   $0x0,0x50(%rsp)
  4895a5:	mov    0x118(%rdx),%rax
  4895ac:	mov    0x108(%rax),%rbx
  4895b3:	mov    0x100(%rax),%r15
  4895ba:	mov    0x118(%rdx),%rcx
  4895c1:	mov    0x108(%rcx),%rbp
  4895c8:	mov    0x100(%rcx),%rax
  4895cf:	mov    0x128(%rdx),%rsi
  4895d6:	mov    %rbp,%rcx
  4895d9:	sub    %rax,%rcx
  4895dc:	cmp    %rsi,%rcx
  4895df:	jge    4898f8 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3b8>
  4895e5:	mov    0x120(%r14),%rax
  4895ec:	dec    %rsi
  4895ef:	sub    %r15,%rbx
  4895f2:	lea    0x787(%rip),%r13        # 489d80 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute>
  4895f9:	lea    0x10(%rsp),%r15
  4895fe:	movabs $0x100000000,%rcx
  489608:	and    %rbp,%rsi
  48960b:	inc    %rbp
  48960e:	shl    $0x4,%rsi
  489612:	mov    %r13,(%rax,%rsi,1)
  489616:	mov    %r15,0x8(%rax,%rsi,1)
  48961b:	mov    0x118(%r14),%rdx
  489622:	mov    %rbp,0x108(%rdx)
  489629:	mov    0x110(%r14),%rsi
  489630:	lea    0x1d8(%rsi),%rdi
  489637:	nopw   0x0(%rax,%rax,1)
  489640:	mov    0x1f0(%rsi),%rax
  489647:	test   %rcx,%rax
  48964a:	jne    48966a <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x12a>
  48964c:	mov    %rax,%rdx
  48964f:	or     %rcx,%rdx
  489652:	lock cmpxchg %rdx,0x1f0(%rsi)
  48965b:	jne    489640 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x100>
  48965d:	mov    %rdx,%rax
  489660:	and    $0xffff,%rax
  489666:	jne    489678 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x138>
  489668:	jmp    48968c <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x14c>
  48966a:	mov    %rax,%rdx
  48966d:	mov    %rdx,%rax
  489670:	and    $0xffff,%rax
  489676:	je     48968c <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x14c>
  489678:	test   %rbx,%rbx
  48967b:	jg     4898e6 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3a6>
  489681:	shr    $0x10,%edx
  489684:	cmp    %eax,%edx
  489686:	je     4898e6 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3a6>
  48968c:	mov    0x48(%r12),%rdx
  489691:	mov    0x40(%r12),%rax
  489696:	mov    0x50(%r12),%r9
  48969b:	lea    0x60(%r12),%r10
  4896a0:	mov    (%rax),%rsi
  4896a3:	mov    (%rdx),%rcx
  4896a6:	mov    0x8(%rdx),%r8
  4896aa:	movzbl (%rsp),%ebp
  4896ae:	lea    0xc0(%rsp),%rdi
  4896b6:	mov    %ebp,%edx
  4896b8:	push   %r10
  4896ba:	push   0x58(%r12)
  4896bf:	vzeroupper
  4896c2:	call   4675f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  4896c7:	add    $0x10,%rsp
  4896cb:	mov    0xc8(%rsp),%rax
  4896d3:	mov    0xc0(%rsp),%r12
  4896db:	mov    0xd0(%rsp),%rbx
  4896e3:	mov    %rax,(%rsp)
  4896e7:	nopw   0x0(%rax,%rax,1)
  4896f0:	mov    0x78(%rsp),%rax
  4896f5:	cmp    $0x3,%rax
  4896f9:	je     48972f <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x1ef>
  4896fb:	mov    %r14,%rdi
  4896fe:	call   1c3a80 <<rayon_core::registry::WorkerThread>::take_local_job>
  489703:	test   %rax,%rax
  489706:	je     489720 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x1e0>
  489708:	mov    %r15,%rcx
  48970b:	mov    %rax,%rsi
  48970e:	xor    %rdx,%rcx
  489711:	xor    %r13,%rsi
  489714:	or     %rcx,%rsi
  489717:	je     489774 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x234>
  489719:	mov    %rdx,%rdi
  48971c:	call   *%rax
  48971e:	jmp    4896f0 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x1b0>
  489720:	mov    0x78(%rsp),%rax
  489725:	cmp    $0x3,%rax
  489729:	jne    48992e <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3ee>
  48972f:	vmovdqu 0x58(%rsp),%xmm0
  489735:	mov    0x50(%rsp),%rax
  48973a:	cmp    $0x1,%rax
  48973e:	je     48975a <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x21a>
  489740:	cmp    $0x2,%rax
  489744:	jne    489916 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3d6>
  48974a:	vmovq  %xmm0,%rdi
  48974f:	vpextrq $0x1,%xmm0,%rsi
  489755:	call   1058c0 <rayon_core::unwind::resume_unwinding>
  48975a:	mov    0x8(%rsp),%rcx
  48975f:	mov    (%rsp),%rdx
  489763:	mov    0x68(%rsp),%rax
  489768:	mov    %r12,(%rcx)
  48976b:	mov    %rdx,0x8(%rcx)
  48976f:	jmp    4898c7 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x387>
  489774:	vmovups 0x10(%rsp),%zmm0
  48977f:	vmovups 0x50(%rsp),%zmm1
  48978a:	vmovups %zmm0,0xc0(%rsp)
  489792:	vmovups %zmm1,0x100(%rsp)
  48979a:	mov    0xc0(%rsp),%rax
  4897a2:	test   %rax,%rax
  4897a5:	je     489940 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x400>
  4897ab:	vmovups 0x38(%rsp),%xmm0
  4897b1:	mov    0x48(%rsp),%rsi
  4897b6:	mov    0xc8(%rsp),%rcx
  4897be:	mov    0xd0(%rsp),%rdx
  4897c6:	mov    0xe0(%rsp),%r10
  4897ce:	mov    0xd8(%rsp),%r9
  4897d6:	mov    %rsi,0xa0(%rsp)
  4897de:	vmovaps %xmm0,0x90(%rsp)
  4897e7:	mov    (%rax),%rsi
  4897ea:	mov    0x8(%rdx),%r8
  4897ee:	sub    (%rcx),%rsi
  4897f1:	mov    (%rdx),%rcx
  4897f4:	lea    0x90(%rsp),%rax
  4897fc:	mov    %ebp,%edx
  4897fe:	lea    0xa8(%rsp),%rdi
  489806:	push   %rax
  489807:	push   %r10
  489809:	vzeroupper
  48980c:	call   4675f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  489811:	add    $0x10,%rsp
  489815:	cmpq   $0x2,0x100(%rsp)
  48981e:	jb     4898a6 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x366>
  489824:	mov    0x110(%rsp),%r15
  48982c:	mov    0x108(%rsp),%r14
  489834:	mov    (%r15),%rax
  489837:	test   %rax,%rax
  48983a:	je     489841 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x301>
  48983c:	mov    %r14,%rdi
  48983f:	call   *%rax
  489841:	mov    0x8(%r15),%rsi
  489845:	test   %rsi,%rsi
  489848:	je     4898a6 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x366>
  48984a:	mov    0x10(%r15),%rax
  48984e:	cmp    $0x20000,%rsi
  489855:	setae  %cl
  489858:	cmp    $0x1001,%rax
  48985e:	setb   %dl
  489861:	test   %cl,%dl
  489863:	je     48989b <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x35b>
  489865:	blsr   %eax,%ecx
  48986a:	jne    489951 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x411>
  489870:	movabs $0x8000000000000000,%rcx
  48987a:	sub    %rax,%rcx
  48987d:	cmp    %rcx,%rsi
  489880:	jae    489951 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x411>
  489886:	cmpb   $0x1,(%r14,%rsi,1)
  48988b:	jne    48989d <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x35d>
  48988d:	inc    %rsi
  489890:	mov    %r14,%rdi
  489893:	call   *0x6be4f(%rip)        # 4f56e8 <munmap@GLIBC_2.2.5>
  489899:	jmp    4898a6 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x366>
  48989b:	jmp    48989d <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x35d>
  48989d:	mov    %r14,%rdi
  4898a0:	call   *0x6be4a(%rip)        # 4f56f0 <free@GLIBC_2.2.5>
  4898a6:	mov    0x8(%rsp),%rcx
  4898ab:	mov    (%rsp),%rax
  4898af:	vmovdqu 0xa8(%rsp),%xmm0
  4898b8:	mov    %r12,(%rcx)
  4898bb:	mov    %rax,0x8(%rcx)
  4898bf:	mov    0xb8(%rsp),%rax
  4898c7:	mov    %rbx,0x10(%rcx)
  4898cb:	vmovdqu %xmm0,0x18(%rcx)
  4898d0:	mov    %rax,0x28(%rcx)
  4898d4:	add    $0x148,%rsp
  4898db:	pop    %rbx
  4898dc:	pop    %r12
  4898de:	pop    %r13
  4898e0:	pop    %r14
  4898e2:	pop    %r15
  4898e4:	pop    %rbp
  4898e5:	ret
  4898e6:	mov    $0x1,%esi
  4898eb:	vzeroupper
  4898ee:	call   ffe10 <<rayon_core::sleep::Sleep>::wake_any_threads>
  4898f3:	jmp    48968c <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x14c>
  4898f8:	lea    0x118(%r14),%rdi
  4898ff:	add    %rsi,%rsi
  489902:	vzeroupper
  489905:	call   1c3530 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::resize>
  48990a:	mov    0x128(%r14),%rsi
  489911:	jmp    4895e5 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0xa5>
  489916:	lea    -0x46a691(%rip),%rdi        # 1f28c <zmij::STATIC_DATA+0x4d4c>
  48991d:	lea    0x696fc(%rip),%rdx        # 4f3020 <__frame_dummy_init_array_entry+0x66c0>
  489924:	mov    $0x28,%esi
  489929:	call   f1800 <core::panicking::panic>
  48992e:	lea    0x78(%rsp),%rsi
  489933:	mov    %r14,%rdi
  489936:	call   ff5d0 <<rayon_core::registry::WorkerThread>::wait_until_cold>
  48993b:	jmp    48972f <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x1ef>
  489940:	lea    0x69499(%rip),%rdi        # 4f2de0 <__frame_dummy_init_array_entry+0x6480>
  489947:	vzeroupper
  48994a:	call   f2b20 <core::option::unwrap_failed>
  48994f:	jmp    489993 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x453>
  489951:	call   111490 <std::process::abort>
  489956:	jmp    489997 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x457>
  489958:	mov    0x8(%r15),%rsi
  48995c:	mov    %rax,%rbx
  48995f:	test   %rsi,%rsi
  489962:	je     4899bb <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x47b>
  489964:	mov    0x10(%r15),%rdx
  489968:	mov    %r14,%rdi
  48996b:	call   e9c50 <__rustc::__rust_dealloc>
  489970:	mov    %rbx,%rdi
  489973:	call   4eb910 <_Unwind_Resume@plt>
  489978:	mov    %rax,%rdi
  48997b:	call   14537a <std::panicking::catch_unwind::cleanup>
  489980:	mov    %rdx,%rcx
  489983:	lea    0x70(%rsp),%rsi
  489988:	mov    %r14,%rdi
  48998b:	mov    %rax,%rdx
  48998e:	call   105d70 <rayon_core::join::join_recover_from_panic>
  489993:	ud2
  489995:	jmp    489997 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x457>
  489997:	mov    %rax,%rbx
  48999a:	lea    0x10(%rsp),%rdi
  48999f:	call   2215a0 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 4]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<zopro_bn254::fp::Fp<zopro_bn254::FrConfig>>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 4]>, <zopro_bn254::fp::Fp<zopro_bn254::FrConfig>>::to_canonical>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 4]>>>>
  4899a4:	jmp    4899bb <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x47b>
  4899a6:	call   f2c93 <core::panicking::panic_in_cleanup>
  4899ab:	mov    %rax,%rbx
  4899ae:	lea    0x100(%rsp),%rdi
  4899b6:	call   2214f0 <core::ptr::drop_glue::<core::cell::UnsafeCell<rayon_core::job::JobResult<rayon::iter::collect::consumer::CollectResult<[u64; 4]>>>>>
  4899bb:	mov    %rbx,%rdi
  4899be:	call   4eb910 <_Unwind_Resume@plt>
  4899c3:	call   f2c93 <core::panicking::panic_in_cleanup>
