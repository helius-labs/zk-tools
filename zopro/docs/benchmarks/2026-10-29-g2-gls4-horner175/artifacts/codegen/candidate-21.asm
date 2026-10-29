
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004672d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>:
  4672d0:	push   %rbp
  4672d1:	push   %r15
  4672d3:	push   %r14
  4672d5:	push   %r13
  4672d7:	push   %r12
  4672d9:	push   %rbx
  4672da:	sub    $0x148,%rsp
  4672e1:	mov    0x188(%rsp),%rbx
  4672e9:	mov    0x180(%rsp),%r13
  4672f1:	mov    %rsi,0xb0(%rsp)
  4672f9:	shr    $1,%rsi
  4672fc:	mov    %r9,0x8(%rsp)
  467301:	mov    %rdi,%r12
  467304:	mov    %r8,0xc8(%rsp)
  46730c:	cmp    %r8,%rsi
  46730f:	jae    467833 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x563>
  467315:	mov    0x8(%rbx),%rax
  467319:	mov    0x10(%rbx),%rcx
  46731d:	mov    %r12,0x98(%rsp)
  467325:	mov    %rax,0x40(%rsp)
  46732a:	mov    %rcx,0x48(%rsp)
  46732f:	test   %r13,%r13
  467332:	je     46785b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x58b>
  467338:	shl    $0x5,%r13
  46733c:	add    0x8(%rsp),%r13
  467341:	xor    %edx,%edx
  467343:	mov    %r13,0xa0(%rsp)
  46734b:	nopl   0x0(%rax,%rax,1)
  467350:	mov    0x8(%rsp),%rax
  467355:	mov    %rdx,0xa8(%rsp)
  46735d:	lea    -0x45b57c(%rip),%r11        # bde8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x368>
  467364:	xor    %r14d,%r14d
  467367:	mov    (%rax),%rcx
  46736a:	mov    0x8(%rax),%rsi
  46736e:	mov    0x10(%rax),%rdx
  467372:	mov    0x18(%rax),%rax
  467376:	mov    %rcx,0x30(%rsp)
  46737b:	mov    %rsi,(%rsp)
  46737f:	mov    %rdx,0x18(%rsp)
  467384:	mov    %rax,0x10(%rsp)
  467389:	jmp    4673ab <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0xdb>
  46738b:	nopl   0x0(%rax,%rax,1)
  467390:	mov    %rsi,0x50(%rsp,%r14,1)
  467395:	mov    %rax,0x58(%rsp,%r14,1)
  46739a:	add    $0x10,%r14
  46739e:	inc    %r11
  4673a1:	cmp    $0x40,%r14
  4673a5:	je     4674d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x200>
  4673ab:	lea    -0x447a42(%rip),%rax        # 1f970 <zmij::STATIC_DATA+0x5470>
  4673b2:	mov    0x30(%rsp),%rdi
  4673b7:	mov    (%rsp),%rbx
  4673bb:	mov    0x18(%rsp),%r15
  4673c0:	mov    (%rax,%r14,2),%rsi
  4673c4:	mov    0x8(%rax,%r14,2),%rcx
  4673c9:	mov    0x10(%rax,%r14,2),%rax
  4673ce:	mov    %rsi,%rdx
  4673d1:	mulx   %rdi,%rdx,%r8
  4673d6:	mov    %rdx,0x20(%rsp)
  4673db:	mov    %rcx,%rdx
  4673de:	mulx   %rdi,%r13,%r12
  4673e3:	mov    %rax,%rdx
  4673e6:	mulx   %rdi,%rbp,%rdx
  4673eb:	add    %r8,%r13
  4673ee:	mov    %rdx,0x28(%rsp)
  4673f3:	adc    %r12,%rbp
  4673f6:	mov    %rsi,%rdx
  4673f9:	mulx   %rbx,%r12,%r9
  4673fe:	mov    %rcx,%rdx
  467401:	mulx   %rbx,%r10,%r8
  467406:	mov    %rsi,%rdx
  467409:	setb   %dil
  46740d:	add    %rbp,%r10
  467410:	adc    $0x0,%r8
  467414:	add    %r13,%r12
  467417:	adc    %r9,%r10
  46741a:	mulx   %r15,%r13,%r9
  46741f:	mov    %rcx,%rdx
  467422:	adc    $0x0,%r8
  467426:	add    %r10,%r13
  467429:	mulx   %r15,%r10,%rbp
  46742e:	mov    %rax,%rdx
  467431:	adc    %r9,%r10
  467434:	adc    $0x0,%rbp
  467438:	add    $0xff,%dil
  46743c:	mulx   %rbx,%rdi,%r9
  467441:	mov    %rsi,%rdx
  467444:	adc    0x28(%rsp),%rdi
  467449:	adc    $0x0,%r9
  46744d:	add    %r8,%rdi
  467450:	adc    $0x0,%r9
  467454:	add    %r10,%rdi
  467457:	mov    0x10(%rsp),%r10
  46745c:	adc    $0x0,%rbp
  467460:	mulx   %r10,%rbx,%r8
  467465:	mov    %rax,%rdx
  467468:	mulx   %r15,%rsi,%r15
  46746d:	mov    %rcx,%rdx
  467470:	mulx   %r10,%rcx,%rdx
  467475:	add    %rdi,%rbx
  467478:	adc    %r8,%rcx
  46747b:	adc    $0x0,%rdx
  46747f:	add    %r9,%rsi
  467482:	adc    $0x0,%r15
  467486:	imul   %r10,%rax
  46748a:	add    %rbp,%rsi
  46748d:	adc    %r15,%rax
  467490:	add    %rcx,%rsi
  467493:	adc    %rdx,%rax
  467496:	cmpb   $0x1,(%r11)
  46749a:	jne    467390 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0xc0>
  4674a0:	mov    0x20(%rsp),%rdx
  4674a5:	xor    %ecx,%ecx
  4674a7:	or     %r12,%rdx
  4674aa:	or     %r13,%rdx
  4674ad:	or     %rbx,%rdx
  4674b0:	setne  %cl
  4674b3:	neg    %rcx
  4674b6:	mov    %rcx,%rdx
  4674b9:	sub    %rsi,%rdx
  4674bc:	sbb    %rax,%rcx
  4674bf:	mov    %rdx,%rsi
  4674c2:	mov    %rcx,%rax
  4674c5:	jmp    467390 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0xc0>
  4674ca:	nopw   0x0(%rax,%rax,1)
  4674d0:	mov    0x50(%rsp),%r12
  4674d5:	mov    0x58(%rsp),%rcx
  4674da:	movabs $0x762cda976b2dec1e,%rsi
  4674e4:	mov    0x68(%rsp),%rdi
  4674e9:	mov    0x60(%rsp),%r9
  4674ee:	mov    0x78(%rsp),%r10
  4674f3:	mov    0x70(%rsp),%r11
  4674f8:	mov    0x88(%rsp),%r15
  467500:	mov    0x80(%rsp),%r13
  467508:	movabs $0x762cda976b2dec1d,%rbx
  467512:	mov    $0xffffffffffffffff,%rax
  467519:	mov    %rcx,0x18(%rsp)
  46751e:	imul   %rsi,%rcx
  467522:	mov    %r12,%rdx
  467525:	mulx   %rsi,%r8,%rsi
  46752a:	mov    %r9,%rdx
  46752d:	mov    %r9,0x10(%rsp)
  467532:	movabs $0x44e992b44a6909f2,%r9
  46753c:	mov    %r13,0x20(%rsp)
  467541:	mov    %r15,0x28(%rsp)
  467546:	sub    %r12,%rsi
  467549:	add    %rcx,%rsi
  46754c:	add    0x30(%rsp),%r8
  467551:	mov    %rdi,0x30(%rsp)
  467556:	mov    %rdi,%rcx
  467559:	movabs $0x44e992b44a6909f1,%rdi
  467563:	adc    (%rsp),%rsi
  467567:	imul   %rdi,%rcx
  46756b:	mulx   %rdi,%rbp,%rdi
  467570:	mov    %r11,(%rsp)
  467574:	mov    %r11,%rdx
  467577:	add    %rcx,%rdi
  46757a:	add    %rbp,%r8
  46757d:	mov    %r10,%rcx
  467580:	adc    %rdi,%rsi
  467583:	imul   %r9,%rcx
  467587:	mulx   %r9,%r9,%r11
  46758c:	mov    %r13,%rdx
  46758f:	mulx   %rbx,%r14,%rdx
  467594:	add    %rcx,%r11
  467597:	add    %r8,%r9
  46759a:	mov    %r13,%rcx
  46759d:	mov    %r15,%r8
  4675a0:	adc    %rsi,%r11
  4675a3:	imul   %rax,%rcx
  4675a7:	imul   %rbx,%r8
  4675ab:	add    %rcx,%r8
  4675ae:	add    %rdx,%r8
  4675b1:	add    %r9,%r14
  4675b4:	adc    %r11,%r8
  4675b7:	mov    %r8,%rdx
  4675ba:	sar    $0x3f,%rdx
  4675be:	mov    %r8,%r11
  4675c1:	xor    %rdx,%r14
  4675c4:	xor    %rdx,%r11
  4675c7:	sub    %rdx,%r14
  4675ca:	sbb    %rdx,%r11
  4675cd:	cmp    $0x2,%r11
  4675d1:	jae    467a12 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  4675d7:	mov    0x20(%rsp),%rdx
  4675dc:	mov    0x28(%rsp),%rsi
  4675e1:	movabs $0xbb166d4bb596f60e,%r9
  4675eb:	movabs $0xbb166d4bb596f60f,%r13
  4675f5:	imul   %r9,%rsi
  4675f9:	mulx   %r9,%r15,%rdx
  4675fe:	add    %rsi,%rcx
  467601:	mov    %r10,%rsi
  467604:	add    %rdx,%rcx
  467607:	mov    (%rsp),%rdx
  46760b:	add    %r12,%rdx
  46760e:	adc    0x18(%rsp),%rsi
  467613:	mulx   %r13,%r9,%rbx
  467618:	sub    %rdx,%rbx
  46761b:	imul   %r13,%rsi
  46761f:	add    %rbx,%rsi
  467622:	sub    %rbp,%r15
  467625:	sbb    %rdi,%rcx
  467628:	add    %r9,%r15
  46762b:	adc    %rsi,%rcx
  46762e:	mov    %rcx,%rdx
  467631:	sar    $0x3f,%rdx
  467635:	mov    %rcx,%r9
  467638:	xor    %rdx,%r15
  46763b:	xor    %rdx,%r9
  46763e:	sub    %rdx,%r15
  467641:	sbb    %rdx,%r9
  467644:	cmp    $0x1,%r9
  467648:	ja     467a12 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  46764e:	mov    $0xffffffffffffffff,%r13
  467655:	mov    %r12,%rbx
  467658:	movabs $0xbb166d4bb596f60e,%rsi
  467662:	mov    %rbp,0x38(%rsp)
  467667:	mov    %r12,%rdx
  46766a:	imul   %r13,%rbx
  46766e:	mov    0x18(%rsp),%r13
  467673:	mov    %rsi,%rbp
  467676:	mulx   %rsi,%rsi,%rdx
  46767b:	imul   %rbp,%r13
  46767f:	movabs $0x44e992b44a6909f1,%rbp
  467689:	add    %rbx,%r13
  46768c:	mov    0x28(%rsp),%rbx
  467691:	add    %rdx,%r13
  467694:	mov    0x20(%rsp),%rdx
  467699:	add    (%rsp),%rdx
  46769d:	adc    %r10,%rbx
  4676a0:	mulx   %rbp,%rax,%rdx
  4676a5:	imul   %rbp,%rbx
  4676a9:	add    %rdx,%rbx
  4676ac:	sub    0x38(%rsp),%rsi
  4676b1:	sbb    %rdi,%r13
  4676b4:	add    %rax,%rsi
  4676b7:	adc    %rbx,%r13
  4676ba:	mov    %r13,%rdx
  4676bd:	sar    $0x3f,%rdx
  4676c1:	mov    %r13,%rdi
  4676c4:	xor    %rdx,%rsi
  4676c7:	xor    %rdx,%rdi
  4676ca:	sub    %rdx,%rsi
  4676cd:	sbb    %rdx,%rdi
  4676d0:	cmp    $0x1,%rdi
  4676d4:	ja     467a12 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  4676da:	mov    0x10(%rsp),%rdx
  4676df:	movabs $0x762cda976b2dec1d,%rbx
  4676e9:	mov    %r12,%rax
  4676ec:	mov    $0xffffffffffffffff,%rbp
  4676f3:	mov    %r11,0x10(%rsp)
  4676f8:	mov    0x18(%rsp),%r11
  4676fd:	mulx   %rbx,%r12,%rbx
  467702:	imul   %rbp,%rdx
  467706:	movabs $0x762cda976b2dec1d,%rbp
  467710:	mov    %r12,0x38(%rsp)
  467715:	mov    0x30(%rsp),%r12
  46771a:	imul   %rbp,%r12
  46771e:	add    %rdx,%r12
  467721:	mov    (%rsp),%rdx
  467725:	add    %rbx,%r12
  467728:	movabs $0x89d3256894d213e2,%rbx
  467732:	imul   %rbx,%r10
  467736:	mulx   %rbx,%rbp,%rdx
  46773b:	movabs $0x44e992b44a6909f1,%rbx
  467745:	add    %rdx,%r10
  467748:	sub    0x20(%rsp),%rax
  46774d:	sbb    0x28(%rsp),%r11
  467752:	mov    %rax,%rdx
  467755:	mulx   %rbx,%rdx,%rax
  46775a:	imul   %rbx,%r11
  46775e:	add    %rax,%r11
  467761:	add    0x38(%rsp),%rbp
  467766:	adc    %r12,%r10
  467769:	add    %rdx,%rbp
  46776c:	adc    %r11,%r10
  46776f:	mov    0x10(%rsp),%r11
  467774:	mov    %r10,%rdx
  467777:	sar    $0x3f,%rdx
  46777b:	mov    %r10,%rax
  46777e:	xor    %rdx,%rbp
  467781:	xor    %rdx,%rax
  467784:	sub    %rdx,%rbp
  467787:	sbb    %rdx,%rax
  46778a:	cmp    $0x1,%rax
  46778e:	ja     467a12 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  467794:	mov    0xa8(%rsp),%rdx
  46779c:	cmp    0x48(%rsp),%rdx
  4677a1:	je     467a2a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x75a>
  4677a7:	shr    $0x3b,%r8
  4677ab:	shr    $0x3a,%rcx
  4677af:	shr    $0x38,%r10
  4677b3:	shr    $0x39,%r13
  4677b7:	add    %r9d,%r9d
  4677ba:	shl    $0x3,%eax
  4677bd:	shl    $0x2,%edi
  4677c0:	and    $0xfffffff0,%r8d
  4677c4:	and    $0xffffffe0,%ecx
  4677c7:	and    $0xffffff80,%r10d
  4677cb:	and    $0xffffffc0,%r13d
  4677cf:	and    $0x2,%r9d
  4677d3:	and    $0x8,%eax
  4677d6:	and    $0x4,%edi
  4677d9:	or     %r8,%rcx
  4677dc:	or     %r13,%r9
  4677df:	mov    0x40(%rsp),%r8
  4677e4:	mov    0xa0(%rsp),%r13
  4677ec:	or     %rax,%rdi
  4677ef:	lea    (%rdx,%rdx,4),%rax
  4677f3:	inc    %rdx
  4677f6:	or     %rcx,%r10
  4677f9:	mov    0x8(%rsp),%rcx
  4677fe:	or     %r11,%r10
  467801:	or     %r10,%r9
  467804:	or     %r9,%rdi
  467807:	mov    %r14,(%r8,%rax,8)
  46780b:	mov    %r15,0x8(%r8,%rax,8)
  467810:	mov    %rsi,0x10(%r8,%rax,8)
  467815:	mov    %rbp,0x18(%r8,%rax,8)
  46781a:	mov    %rdi,0x20(%r8,%rax,8)
  46781f:	add    $0x20,%rcx
  467823:	mov    %rcx,0x8(%rsp)
  467828:	cmp    %r13,%rcx
  46782b:	jne    467350 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x80>
  467831:	jmp    46785d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x58d>
  467833:	test   %dl,%dl
  467835:	je     46784d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x57d>
  467837:	mov    %fs:0xffffffffffffffc0,%rax
  467840:	test   %rax,%rax
  467843:	je     46787f <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x5af>
  467845:	add    $0x110,%rax
  46784b:	jmp    467890 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x5c0>
  46784d:	test   %rcx,%rcx
  467850:	je     467315 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x45>
  467856:	shr    $1,%rcx
  467859:	jmp    4678a4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x5d4>
  46785b:	xor    %edx,%edx
  46785d:	mov    0x98(%rsp),%rax
  467865:	mov    0x40(%rsp),%rcx
  46786a:	mov    0x48(%rsp),%rsi
  46786f:	mov    %rcx,(%rax)
  467872:	mov    %rsi,0x8(%rax)
  467876:	mov    %rdx,0x10(%rax)
  46787a:	jmp    467a00 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x730>
  46787f:	mov    %rsi,%r15
  467882:	mov    %rcx,%r14
  467885:	call   1052d0 <rayon_core::registry::global_registry>
  46788a:	mov    %r14,%rcx
  46788d:	mov    %r15,%rsi
  467890:	mov    (%rax),%rax
  467893:	shr    $1,%rcx
  467896:	mov    0x208(%rax),%rax
  46789d:	cmp    %rax,%rcx
  4678a0:	cmovbe %rax,%rcx
  4678a4:	sub    %rsi,%r13
  4678a7:	mov    %rcx,0xc0(%rsp)
  4678af:	mov    %rsi,0xb8(%rsp)
  4678b7:	jb     467a42 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x772>
  4678bd:	mov    0x10(%rbx),%rax
  4678c1:	sub    %rsi,%rax
  4678c4:	jb     467a5a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x78a>
  4678ca:	mov    0x8(%rbx),%r8
  4678ce:	mov    0x8(%rsp),%rdx
  4678d3:	mov    (%rbx),%rdi
  4678d6:	lea    (%rsi,%rsi,4),%r9
  4678da:	mov    %rsi,%rcx
  4678dd:	shl    $0x5,%rcx
  4678e1:	lea    0xb0(%rsp),%r10
  4678e9:	lea    0xc0(%rsp),%r11
  4678f1:	mov    %r10,0xd0(%rsp)
  4678f9:	lea    0xb8(%rsp),%r10
  467901:	mov    %r10,0xd8(%rsp)
  467909:	mov    %r11,0xe0(%rsp)
  467911:	lea    (%r8,%r9,8),%r9
  467915:	add    %rdx,%rcx
  467918:	mov    %rcx,0xe8(%rsp)
  467920:	mov    %r13,0xf0(%rsp)
  467928:	mov    %rdi,0xf8(%rsp)
  467930:	mov    %r9,0x100(%rsp)
  467938:	mov    %rax,0x108(%rsp)
  467940:	mov    %r10,0x110(%rsp)
  467948:	mov    %r11,0x118(%rsp)
  467950:	mov    %rdx,0x120(%rsp)
  467958:	mov    %fs:0xffffffffffffffc0,%rdx
  467961:	mov    %rsi,0x128(%rsp)
  467969:	mov    %rdi,0x130(%rsp)
  467971:	mov    %r8,0x138(%rsp)
  467979:	mov    %rsi,0x140(%rsp)
  467981:	test   %rdx,%rdx
  467984:	jne    4679ad <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x6dd>
  467986:	call   1052d0 <rayon_core::registry::global_registry>
  46798b:	mov    (%rax),%rsi
  46798e:	mov    %fs:0xffffffffffffffc0,%rdx
  467997:	test   %rdx,%rdx
  46799a:	je     467a72 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x7a2>
  4679a0:	cmp    %rsi,0x110(%rdx)
  4679a7:	jne    467a8d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x7bd>
  4679ad:	lea    0x50(%rsp),%rdi
  4679b2:	lea    0xd0(%rsp),%rsi
  4679ba:	xor    %ecx,%ecx
  4679bc:	call   486090 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}>
  4679c1:	mov    0x60(%rsp),%rcx
  4679c6:	mov    0x50(%rsp),%rax
  4679cb:	lea    (%rcx,%rcx,4),%rdx
  4679cf:	lea    (%rax,%rdx,8),%rdx
  4679d3:	cmp    0x68(%rsp),%rdx
  4679d8:	jne    4679e6 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x716>
  4679da:	mov    0x70(%rsp),%rdx
  4679df:	mov    0x78(%rsp),%rsi
  4679e4:	jmp    4679ea <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x71a>
  4679e6:	xor    %edx,%edx
  4679e8:	xor    %esi,%esi
  4679ea:	add    0x58(%rsp),%rdx
  4679ef:	add    %rcx,%rsi
  4679f2:	mov    %rax,(%r12)
  4679f6:	mov    %rdx,0x8(%r12)
  4679fb:	mov    %rsi,0x10(%r12)
  467a00:	add    $0x148,%rsp
  467a07:	pop    %rbx
  467a08:	pop    %r12
  467a0a:	pop    %r13
  467a0c:	pop    %r14
  467a0e:	pop    %r15
  467a10:	pop    %rbp
  467a11:	ret
  467a12:	lea    -0x3dd589(%rip),%rdi        # 8a490 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x100>
  467a19:	lea    0x893c8(%rip),%rdx        # 4f0de8 <__frame_dummy_init_array_entry+0x7938>
  467a20:	mov    $0x29,%esi
  467a25:	call   f1580 <core::panicking::panic>
  467a2a:	lea    -0x3dd486(%rip),%rdi        # 8a5ab <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x21b>
  467a31:	lea    0x89590(%rip),%rdx        # 4f0fc8 <__frame_dummy_init_array_entry+0x7b18>
  467a38:	mov    $0x45,%esi
  467a3d:	call   ef4b0 <core::panicking::panic_fmt>
  467a42:	lea    -0x447f53(%rip),%rdi        # 1faf6 <zmij::STATIC_DATA+0x55f6>
  467a49:	lea    0x89590(%rip),%rdx        # 4f0fe0 <__frame_dummy_init_array_entry+0x7b30>
  467a50:	mov    $0x13,%esi
  467a55:	call   ef4b0 <core::panicking::panic_fmt>
  467a5a:	lea    -0x3dd4d4(%rip),%rdi        # 8a58d <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1fd>
  467a61:	lea    0x89530(%rip),%rdx        # 4f0f98 <__frame_dummy_init_array_entry+0x7ae8>
  467a68:	mov    $0x1e,%esi
  467a6d:	call   f1580 <core::panicking::panic>
  467a72:	sub    $0xffffffffffffff80,%rsi
  467a76:	lea    0x50(%rsp),%rdi
  467a7b:	lea    0xd0(%rsp),%rdx
  467a83:	call   485f90 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>
  467a88:	jmp    4679c1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x6f1>
  467a8d:	sub    $0xffffffffffffff80,%rsi
  467a91:	lea    0x50(%rsp),%rdi
  467a96:	lea    0xd0(%rsp),%rcx
  467a9e:	call   486520 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>
  467aa3:	jmp    4679c1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x6f1>
