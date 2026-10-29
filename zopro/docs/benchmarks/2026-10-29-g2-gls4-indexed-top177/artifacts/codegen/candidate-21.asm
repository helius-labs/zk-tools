
/home/ubuntu/src/zopro-perf-work/g2-gls4-indexed-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000467500 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>:
  467500:	push   %rbp
  467501:	push   %r15
  467503:	push   %r14
  467505:	push   %r13
  467507:	push   %r12
  467509:	push   %rbx
  46750a:	sub    $0x148,%rsp
  467511:	mov    0x188(%rsp),%rbx
  467519:	mov    0x180(%rsp),%r13
  467521:	mov    %rsi,0xb0(%rsp)
  467529:	shr    $1,%rsi
  46752c:	mov    %r9,0x8(%rsp)
  467531:	mov    %rdi,%r12
  467534:	mov    %r8,0xc8(%rsp)
  46753c:	cmp    %r8,%rsi
  46753f:	jae    467a63 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x563>
  467545:	mov    0x8(%rbx),%rax
  467549:	mov    0x10(%rbx),%rcx
  46754d:	mov    %r12,0x98(%rsp)
  467555:	mov    %rax,0x40(%rsp)
  46755a:	mov    %rcx,0x48(%rsp)
  46755f:	test   %r13,%r13
  467562:	je     467a8b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x58b>
  467568:	shl    $0x5,%r13
  46756c:	add    0x8(%rsp),%r13
  467571:	xor    %edx,%edx
  467573:	mov    %r13,0xa0(%rsp)
  46757b:	nopl   0x0(%rax,%rax,1)
  467580:	mov    0x8(%rsp),%rax
  467585:	mov    %rdx,0xa8(%rsp)
  46758d:	lea    -0x45b76c(%rip),%r11        # be28 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x368>
  467594:	xor    %r14d,%r14d
  467597:	mov    (%rax),%rcx
  46759a:	mov    0x8(%rax),%rsi
  46759e:	mov    0x10(%rax),%rdx
  4675a2:	mov    0x18(%rax),%rax
  4675a6:	mov    %rcx,0x30(%rsp)
  4675ab:	mov    %rsi,(%rsp)
  4675af:	mov    %rdx,0x18(%rsp)
  4675b4:	mov    %rax,0x10(%rsp)
  4675b9:	jmp    4675db <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0xdb>
  4675bb:	nopl   0x0(%rax,%rax,1)
  4675c0:	mov    %rsi,0x50(%rsp,%r14,1)
  4675c5:	mov    %rax,0x58(%rsp,%r14,1)
  4675ca:	add    $0x10,%r14
  4675ce:	inc    %r11
  4675d1:	cmp    $0x40,%r14
  4675d5:	je     467700 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x200>
  4675db:	lea    -0x447c32(%rip),%rax        # 1f9b0 <zmij::STATIC_DATA+0x5470>
  4675e2:	mov    0x30(%rsp),%rdi
  4675e7:	mov    (%rsp),%rbx
  4675eb:	mov    0x18(%rsp),%r15
  4675f0:	mov    (%rax,%r14,2),%rsi
  4675f4:	mov    0x8(%rax,%r14,2),%rcx
  4675f9:	mov    0x10(%rax,%r14,2),%rax
  4675fe:	mov    %rsi,%rdx
  467601:	mulx   %rdi,%rdx,%r8
  467606:	mov    %rdx,0x20(%rsp)
  46760b:	mov    %rcx,%rdx
  46760e:	mulx   %rdi,%r13,%r12
  467613:	mov    %rax,%rdx
  467616:	mulx   %rdi,%rbp,%rdx
  46761b:	add    %r8,%r13
  46761e:	mov    %rdx,0x28(%rsp)
  467623:	adc    %r12,%rbp
  467626:	mov    %rsi,%rdx
  467629:	mulx   %rbx,%r12,%r9
  46762e:	mov    %rcx,%rdx
  467631:	mulx   %rbx,%r10,%r8
  467636:	mov    %rsi,%rdx
  467639:	setb   %dil
  46763d:	add    %rbp,%r10
  467640:	adc    $0x0,%r8
  467644:	add    %r13,%r12
  467647:	adc    %r9,%r10
  46764a:	mulx   %r15,%r13,%r9
  46764f:	mov    %rcx,%rdx
  467652:	adc    $0x0,%r8
  467656:	add    %r10,%r13
  467659:	mulx   %r15,%r10,%rbp
  46765e:	mov    %rax,%rdx
  467661:	adc    %r9,%r10
  467664:	adc    $0x0,%rbp
  467668:	add    $0xff,%dil
  46766c:	mulx   %rbx,%rdi,%r9
  467671:	mov    %rsi,%rdx
  467674:	adc    0x28(%rsp),%rdi
  467679:	adc    $0x0,%r9
  46767d:	add    %r8,%rdi
  467680:	adc    $0x0,%r9
  467684:	add    %r10,%rdi
  467687:	mov    0x10(%rsp),%r10
  46768c:	adc    $0x0,%rbp
  467690:	mulx   %r10,%rbx,%r8
  467695:	mov    %rax,%rdx
  467698:	mulx   %r15,%rsi,%r15
  46769d:	mov    %rcx,%rdx
  4676a0:	mulx   %r10,%rcx,%rdx
  4676a5:	add    %rdi,%rbx
  4676a8:	adc    %r8,%rcx
  4676ab:	adc    $0x0,%rdx
  4676af:	add    %r9,%rsi
  4676b2:	adc    $0x0,%r15
  4676b6:	imul   %r10,%rax
  4676ba:	add    %rbp,%rsi
  4676bd:	adc    %r15,%rax
  4676c0:	add    %rcx,%rsi
  4676c3:	adc    %rdx,%rax
  4676c6:	cmpb   $0x1,(%r11)
  4676ca:	jne    4675c0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0xc0>
  4676d0:	mov    0x20(%rsp),%rdx
  4676d5:	xor    %ecx,%ecx
  4676d7:	or     %r12,%rdx
  4676da:	or     %r13,%rdx
  4676dd:	or     %rbx,%rdx
  4676e0:	setne  %cl
  4676e3:	neg    %rcx
  4676e6:	mov    %rcx,%rdx
  4676e9:	sub    %rsi,%rdx
  4676ec:	sbb    %rax,%rcx
  4676ef:	mov    %rdx,%rsi
  4676f2:	mov    %rcx,%rax
  4676f5:	jmp    4675c0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0xc0>
  4676fa:	nopw   0x0(%rax,%rax,1)
  467700:	mov    0x50(%rsp),%r12
  467705:	mov    0x58(%rsp),%rcx
  46770a:	movabs $0x762cda976b2dec1e,%rsi
  467714:	mov    0x68(%rsp),%rdi
  467719:	mov    0x60(%rsp),%r9
  46771e:	mov    0x78(%rsp),%r10
  467723:	mov    0x70(%rsp),%r11
  467728:	mov    0x88(%rsp),%r15
  467730:	mov    0x80(%rsp),%r13
  467738:	movabs $0x762cda976b2dec1d,%rbx
  467742:	mov    $0xffffffffffffffff,%rax
  467749:	mov    %rcx,0x18(%rsp)
  46774e:	imul   %rsi,%rcx
  467752:	mov    %r12,%rdx
  467755:	mulx   %rsi,%r8,%rsi
  46775a:	mov    %r9,%rdx
  46775d:	mov    %r9,0x10(%rsp)
  467762:	movabs $0x44e992b44a6909f2,%r9
  46776c:	mov    %r13,0x20(%rsp)
  467771:	mov    %r15,0x28(%rsp)
  467776:	sub    %r12,%rsi
  467779:	add    %rcx,%rsi
  46777c:	add    0x30(%rsp),%r8
  467781:	mov    %rdi,0x30(%rsp)
  467786:	mov    %rdi,%rcx
  467789:	movabs $0x44e992b44a6909f1,%rdi
  467793:	adc    (%rsp),%rsi
  467797:	imul   %rdi,%rcx
  46779b:	mulx   %rdi,%rbp,%rdi
  4677a0:	mov    %r11,(%rsp)
  4677a4:	mov    %r11,%rdx
  4677a7:	add    %rcx,%rdi
  4677aa:	add    %rbp,%r8
  4677ad:	mov    %r10,%rcx
  4677b0:	adc    %rdi,%rsi
  4677b3:	imul   %r9,%rcx
  4677b7:	mulx   %r9,%r9,%r11
  4677bc:	mov    %r13,%rdx
  4677bf:	mulx   %rbx,%r14,%rdx
  4677c4:	add    %rcx,%r11
  4677c7:	add    %r8,%r9
  4677ca:	mov    %r13,%rcx
  4677cd:	mov    %r15,%r8
  4677d0:	adc    %rsi,%r11
  4677d3:	imul   %rax,%rcx
  4677d7:	imul   %rbx,%r8
  4677db:	add    %rcx,%r8
  4677de:	add    %rdx,%r8
  4677e1:	add    %r9,%r14
  4677e4:	adc    %r11,%r8
  4677e7:	mov    %r8,%rdx
  4677ea:	sar    $0x3f,%rdx
  4677ee:	mov    %r8,%r11
  4677f1:	xor    %rdx,%r14
  4677f4:	xor    %rdx,%r11
  4677f7:	sub    %rdx,%r14
  4677fa:	sbb    %rdx,%r11
  4677fd:	cmp    $0x2,%r11
  467801:	jae    467c42 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  467807:	mov    0x20(%rsp),%rdx
  46780c:	mov    0x28(%rsp),%rsi
  467811:	movabs $0xbb166d4bb596f60e,%r9
  46781b:	movabs $0xbb166d4bb596f60f,%r13
  467825:	imul   %r9,%rsi
  467829:	mulx   %r9,%r15,%rdx
  46782e:	add    %rsi,%rcx
  467831:	mov    %r10,%rsi
  467834:	add    %rdx,%rcx
  467837:	mov    (%rsp),%rdx
  46783b:	add    %r12,%rdx
  46783e:	adc    0x18(%rsp),%rsi
  467843:	mulx   %r13,%r9,%rbx
  467848:	sub    %rdx,%rbx
  46784b:	imul   %r13,%rsi
  46784f:	add    %rbx,%rsi
  467852:	sub    %rbp,%r15
  467855:	sbb    %rdi,%rcx
  467858:	add    %r9,%r15
  46785b:	adc    %rsi,%rcx
  46785e:	mov    %rcx,%rdx
  467861:	sar    $0x3f,%rdx
  467865:	mov    %rcx,%r9
  467868:	xor    %rdx,%r15
  46786b:	xor    %rdx,%r9
  46786e:	sub    %rdx,%r15
  467871:	sbb    %rdx,%r9
  467874:	cmp    $0x1,%r9
  467878:	ja     467c42 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  46787e:	mov    $0xffffffffffffffff,%r13
  467885:	mov    %r12,%rbx
  467888:	movabs $0xbb166d4bb596f60e,%rsi
  467892:	mov    %rbp,0x38(%rsp)
  467897:	mov    %r12,%rdx
  46789a:	imul   %r13,%rbx
  46789e:	mov    0x18(%rsp),%r13
  4678a3:	mov    %rsi,%rbp
  4678a6:	mulx   %rsi,%rsi,%rdx
  4678ab:	imul   %rbp,%r13
  4678af:	movabs $0x44e992b44a6909f1,%rbp
  4678b9:	add    %rbx,%r13
  4678bc:	mov    0x28(%rsp),%rbx
  4678c1:	add    %rdx,%r13
  4678c4:	mov    0x20(%rsp),%rdx
  4678c9:	add    (%rsp),%rdx
  4678cd:	adc    %r10,%rbx
  4678d0:	mulx   %rbp,%rax,%rdx
  4678d5:	imul   %rbp,%rbx
  4678d9:	add    %rdx,%rbx
  4678dc:	sub    0x38(%rsp),%rsi
  4678e1:	sbb    %rdi,%r13
  4678e4:	add    %rax,%rsi
  4678e7:	adc    %rbx,%r13
  4678ea:	mov    %r13,%rdx
  4678ed:	sar    $0x3f,%rdx
  4678f1:	mov    %r13,%rdi
  4678f4:	xor    %rdx,%rsi
  4678f7:	xor    %rdx,%rdi
  4678fa:	sub    %rdx,%rsi
  4678fd:	sbb    %rdx,%rdi
  467900:	cmp    $0x1,%rdi
  467904:	ja     467c42 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  46790a:	mov    0x10(%rsp),%rdx
  46790f:	movabs $0x762cda976b2dec1d,%rbx
  467919:	mov    %r12,%rax
  46791c:	mov    $0xffffffffffffffff,%rbp
  467923:	mov    %r11,0x10(%rsp)
  467928:	mov    0x18(%rsp),%r11
  46792d:	mulx   %rbx,%r12,%rbx
  467932:	imul   %rbp,%rdx
  467936:	movabs $0x762cda976b2dec1d,%rbp
  467940:	mov    %r12,0x38(%rsp)
  467945:	mov    0x30(%rsp),%r12
  46794a:	imul   %rbp,%r12
  46794e:	add    %rdx,%r12
  467951:	mov    (%rsp),%rdx
  467955:	add    %rbx,%r12
  467958:	movabs $0x89d3256894d213e2,%rbx
  467962:	imul   %rbx,%r10
  467966:	mulx   %rbx,%rbp,%rdx
  46796b:	movabs $0x44e992b44a6909f1,%rbx
  467975:	add    %rdx,%r10
  467978:	sub    0x20(%rsp),%rax
  46797d:	sbb    0x28(%rsp),%r11
  467982:	mov    %rax,%rdx
  467985:	mulx   %rbx,%rdx,%rax
  46798a:	imul   %rbx,%r11
  46798e:	add    %rax,%r11
  467991:	add    0x38(%rsp),%rbp
  467996:	adc    %r12,%r10
  467999:	add    %rdx,%rbp
  46799c:	adc    %r11,%r10
  46799f:	mov    0x10(%rsp),%r11
  4679a4:	mov    %r10,%rdx
  4679a7:	sar    $0x3f,%rdx
  4679ab:	mov    %r10,%rax
  4679ae:	xor    %rdx,%rbp
  4679b1:	xor    %rdx,%rax
  4679b4:	sub    %rdx,%rbp
  4679b7:	sbb    %rdx,%rax
  4679ba:	cmp    $0x1,%rax
  4679be:	ja     467c42 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  4679c4:	mov    0xa8(%rsp),%rdx
  4679cc:	cmp    0x48(%rsp),%rdx
  4679d1:	je     467c5a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x75a>
  4679d7:	shr    $0x3b,%r8
  4679db:	shr    $0x3a,%rcx
  4679df:	shr    $0x38,%r10
  4679e3:	shr    $0x39,%r13
  4679e7:	add    %r9d,%r9d
  4679ea:	shl    $0x3,%eax
  4679ed:	shl    $0x2,%edi
  4679f0:	and    $0xfffffff0,%r8d
  4679f4:	and    $0xffffffe0,%ecx
  4679f7:	and    $0xffffff80,%r10d
  4679fb:	and    $0xffffffc0,%r13d
  4679ff:	and    $0x2,%r9d
  467a03:	and    $0x8,%eax
  467a06:	and    $0x4,%edi
  467a09:	or     %r8,%rcx
  467a0c:	or     %r13,%r9
  467a0f:	mov    0x40(%rsp),%r8
  467a14:	mov    0xa0(%rsp),%r13
  467a1c:	or     %rax,%rdi
  467a1f:	lea    (%rdx,%rdx,4),%rax
  467a23:	inc    %rdx
  467a26:	or     %rcx,%r10
  467a29:	mov    0x8(%rsp),%rcx
  467a2e:	or     %r11,%r10
  467a31:	or     %r10,%r9
  467a34:	or     %r9,%rdi
  467a37:	mov    %r14,(%r8,%rax,8)
  467a3b:	mov    %r15,0x8(%r8,%rax,8)
  467a40:	mov    %rsi,0x10(%r8,%rax,8)
  467a45:	mov    %rbp,0x18(%r8,%rax,8)
  467a4a:	mov    %rdi,0x20(%r8,%rax,8)
  467a4f:	add    $0x20,%rcx
  467a53:	mov    %rcx,0x8(%rsp)
  467a58:	cmp    %r13,%rcx
  467a5b:	jne    467580 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x80>
  467a61:	jmp    467a8d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x58d>
  467a63:	test   %dl,%dl
  467a65:	je     467a7d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x57d>
  467a67:	mov    %fs:0xffffffffffffffc0,%rax
  467a70:	test   %rax,%rax
  467a73:	je     467aaf <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x5af>
  467a75:	add    $0x110,%rax
  467a7b:	jmp    467ac0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x5c0>
  467a7d:	test   %rcx,%rcx
  467a80:	je     467545 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x45>
  467a86:	shr    $1,%rcx
  467a89:	jmp    467ad4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x5d4>
  467a8b:	xor    %edx,%edx
  467a8d:	mov    0x98(%rsp),%rax
  467a95:	mov    0x40(%rsp),%rcx
  467a9a:	mov    0x48(%rsp),%rsi
  467a9f:	mov    %rcx,(%rax)
  467aa2:	mov    %rsi,0x8(%rax)
  467aa6:	mov    %rdx,0x10(%rax)
  467aaa:	jmp    467c30 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x730>
  467aaf:	mov    %rsi,%r15
  467ab2:	mov    %rcx,%r14
  467ab5:	call   105490 <rayon_core::registry::global_registry>
  467aba:	mov    %r14,%rcx
  467abd:	mov    %r15,%rsi
  467ac0:	mov    (%rax),%rax
  467ac3:	shr    $1,%rcx
  467ac6:	mov    0x208(%rax),%rax
  467acd:	cmp    %rax,%rcx
  467ad0:	cmovbe %rax,%rcx
  467ad4:	sub    %rsi,%r13
  467ad7:	mov    %rcx,0xc0(%rsp)
  467adf:	mov    %rsi,0xb8(%rsp)
  467ae7:	jb     467c72 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x772>
  467aed:	mov    0x10(%rbx),%rax
  467af1:	sub    %rsi,%rax
  467af4:	jb     467c8a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x78a>
  467afa:	mov    0x8(%rbx),%r8
  467afe:	mov    0x8(%rsp),%rdx
  467b03:	mov    (%rbx),%rdi
  467b06:	lea    (%rsi,%rsi,4),%r9
  467b0a:	mov    %rsi,%rcx
  467b0d:	shl    $0x5,%rcx
  467b11:	lea    0xb0(%rsp),%r10
  467b19:	lea    0xc0(%rsp),%r11
  467b21:	mov    %r10,0xd0(%rsp)
  467b29:	lea    0xb8(%rsp),%r10
  467b31:	mov    %r10,0xd8(%rsp)
  467b39:	mov    %r11,0xe0(%rsp)
  467b41:	lea    (%r8,%r9,8),%r9
  467b45:	add    %rdx,%rcx
  467b48:	mov    %rcx,0xe8(%rsp)
  467b50:	mov    %r13,0xf0(%rsp)
  467b58:	mov    %rdi,0xf8(%rsp)
  467b60:	mov    %r9,0x100(%rsp)
  467b68:	mov    %rax,0x108(%rsp)
  467b70:	mov    %r10,0x110(%rsp)
  467b78:	mov    %r11,0x118(%rsp)
  467b80:	mov    %rdx,0x120(%rsp)
  467b88:	mov    %fs:0xffffffffffffffc0,%rdx
  467b91:	mov    %rsi,0x128(%rsp)
  467b99:	mov    %rdi,0x130(%rsp)
  467ba1:	mov    %r8,0x138(%rsp)
  467ba9:	mov    %rsi,0x140(%rsp)
  467bb1:	test   %rdx,%rdx
  467bb4:	jne    467bdd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x6dd>
  467bb6:	call   105490 <rayon_core::registry::global_registry>
  467bbb:	mov    (%rax),%rsi
  467bbe:	mov    %fs:0xffffffffffffffc0,%rdx
  467bc7:	test   %rdx,%rdx
  467bca:	je     467ca2 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x7a2>
  467bd0:	cmp    %rsi,0x110(%rdx)
  467bd7:	jne    467cbd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x7bd>
  467bdd:	lea    0x50(%rsp),%rdi
  467be2:	lea    0xd0(%rsp),%rsi
  467bea:	xor    %ecx,%ecx
  467bec:	call   489400 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}>
  467bf1:	mov    0x60(%rsp),%rcx
  467bf6:	mov    0x50(%rsp),%rax
  467bfb:	lea    (%rcx,%rcx,4),%rdx
  467bff:	lea    (%rax,%rdx,8),%rdx
  467c03:	cmp    0x68(%rsp),%rdx
  467c08:	jne    467c16 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x716>
  467c0a:	mov    0x70(%rsp),%rdx
  467c0f:	mov    0x78(%rsp),%rsi
  467c14:	jmp    467c1a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x71a>
  467c16:	xor    %edx,%edx
  467c18:	xor    %esi,%esi
  467c1a:	add    0x58(%rsp),%rdx
  467c1f:	add    %rcx,%rsi
  467c22:	mov    %rax,(%r12)
  467c26:	mov    %rdx,0x8(%r12)
  467c2b:	mov    %rsi,0x10(%r12)
  467c30:	add    $0x148,%rsp
  467c37:	pop    %rbx
  467c38:	pop    %r12
  467c3a:	pop    %r13
  467c3c:	pop    %r14
  467c3e:	pop    %r15
  467c40:	pop    %rbp
  467c41:	ret
  467c42:	lea    -0x3dd779(%rip),%rdi        # 8a4d0 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x100>
  467c49:	lea    0x8c568(%rip),%rdx        # 4f41b8 <__frame_dummy_init_array_entry+0x7998>
  467c50:	mov    $0x29,%esi
  467c55:	call   f1740 <core::panicking::panic>
  467c5a:	lea    -0x3dd676(%rip),%rdi        # 8a5eb <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x21b>
  467c61:	lea    0x8c730(%rip),%rdx        # 4f4398 <__frame_dummy_init_array_entry+0x7b78>
  467c68:	mov    $0x45,%esi
  467c6d:	call   ef670 <core::panicking::panic_fmt>
  467c72:	lea    -0x448143(%rip),%rdi        # 1fb36 <zmij::STATIC_DATA+0x55f6>
  467c79:	lea    0x8c730(%rip),%rdx        # 4f43b0 <__frame_dummy_init_array_entry+0x7b90>
  467c80:	mov    $0x13,%esi
  467c85:	call   ef670 <core::panicking::panic_fmt>
  467c8a:	lea    -0x3dd6c4(%rip),%rdi        # 8a5cd <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1fd>
  467c91:	lea    0x8c6d0(%rip),%rdx        # 4f4368 <__frame_dummy_init_array_entry+0x7b48>
  467c98:	mov    $0x1e,%esi
  467c9d:	call   f1740 <core::panicking::panic>
  467ca2:	sub    $0xffffffffffffff80,%rsi
  467ca6:	lea    0x50(%rsp),%rdi
  467cab:	lea    0xd0(%rsp),%rdx
  467cb3:	call   489300 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>
  467cb8:	jmp    467bf1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x6f1>
  467cbd:	sub    $0xffffffffffffff80,%rsi
  467cc1:	lea    0x50(%rsp),%rdi
  467cc6:	lea    0xd0(%rsp),%rcx
  467cce:	call   489890 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>
  467cd3:	jmp    467bf1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x6f1>
