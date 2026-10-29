
/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000004675f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>:
  4675f0:	push   %rbp
  4675f1:	push   %r15
  4675f3:	push   %r14
  4675f5:	push   %r13
  4675f7:	push   %r12
  4675f9:	push   %rbx
  4675fa:	sub    $0x148,%rsp
  467601:	mov    0x188(%rsp),%rbx
  467609:	mov    0x180(%rsp),%r13
  467611:	mov    %rsi,0xb0(%rsp)
  467619:	shr    $1,%rsi
  46761c:	mov    %r9,0x8(%rsp)
  467621:	mov    %rdi,%r12
  467624:	mov    %r8,0xc8(%rsp)
  46762c:	cmp    %r8,%rsi
  46762f:	jae    467b53 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x563>
  467635:	mov    0x8(%rbx),%rax
  467639:	mov    0x10(%rbx),%rcx
  46763d:	mov    %r12,0x98(%rsp)
  467645:	mov    %rax,0x40(%rsp)
  46764a:	mov    %rcx,0x48(%rsp)
  46764f:	test   %r13,%r13
  467652:	je     467b7b <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x58b>
  467658:	shl    $0x5,%r13
  46765c:	add    0x8(%rsp),%r13
  467661:	xor    %edx,%edx
  467663:	mov    %r13,0xa0(%rsp)
  46766b:	nopl   0x0(%rax,%rax,1)
  467670:	mov    0x8(%rsp),%rax
  467675:	mov    %rdx,0xa8(%rsp)
  46767d:	lea    -0x45b85c(%rip),%r11        # be28 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x368>
  467684:	xor    %r14d,%r14d
  467687:	mov    (%rax),%rcx
  46768a:	mov    0x8(%rax),%rsi
  46768e:	mov    0x10(%rax),%rdx
  467692:	mov    0x18(%rax),%rax
  467696:	mov    %rcx,0x30(%rsp)
  46769b:	mov    %rsi,(%rsp)
  46769f:	mov    %rdx,0x18(%rsp)
  4676a4:	mov    %rax,0x10(%rsp)
  4676a9:	jmp    4676cb <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0xdb>
  4676ab:	nopl   0x0(%rax,%rax,1)
  4676b0:	mov    %rsi,0x50(%rsp,%r14,1)
  4676b5:	mov    %rax,0x58(%rsp,%r14,1)
  4676ba:	add    $0x10,%r14
  4676be:	inc    %r11
  4676c1:	cmp    $0x40,%r14
  4676c5:	je     4677f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x200>
  4676cb:	lea    -0x447d22(%rip),%rax        # 1f9b0 <zmij::STATIC_DATA+0x5470>
  4676d2:	mov    0x30(%rsp),%rdi
  4676d7:	mov    (%rsp),%rbx
  4676db:	mov    0x18(%rsp),%r15
  4676e0:	mov    (%rax,%r14,2),%rsi
  4676e4:	mov    0x8(%rax,%r14,2),%rcx
  4676e9:	mov    0x10(%rax,%r14,2),%rax
  4676ee:	mov    %rsi,%rdx
  4676f1:	mulx   %rdi,%rdx,%r8
  4676f6:	mov    %rdx,0x20(%rsp)
  4676fb:	mov    %rcx,%rdx
  4676fe:	mulx   %rdi,%r13,%r12
  467703:	mov    %rax,%rdx
  467706:	mulx   %rdi,%rbp,%rdx
  46770b:	add    %r8,%r13
  46770e:	mov    %rdx,0x28(%rsp)
  467713:	adc    %r12,%rbp
  467716:	mov    %rsi,%rdx
  467719:	mulx   %rbx,%r12,%r9
  46771e:	mov    %rcx,%rdx
  467721:	mulx   %rbx,%r10,%r8
  467726:	mov    %rsi,%rdx
  467729:	setb   %dil
  46772d:	add    %rbp,%r10
  467730:	adc    $0x0,%r8
  467734:	add    %r13,%r12
  467737:	adc    %r9,%r10
  46773a:	mulx   %r15,%r13,%r9
  46773f:	mov    %rcx,%rdx
  467742:	adc    $0x0,%r8
  467746:	add    %r10,%r13
  467749:	mulx   %r15,%r10,%rbp
  46774e:	mov    %rax,%rdx
  467751:	adc    %r9,%r10
  467754:	adc    $0x0,%rbp
  467758:	add    $0xff,%dil
  46775c:	mulx   %rbx,%rdi,%r9
  467761:	mov    %rsi,%rdx
  467764:	adc    0x28(%rsp),%rdi
  467769:	adc    $0x0,%r9
  46776d:	add    %r8,%rdi
  467770:	adc    $0x0,%r9
  467774:	add    %r10,%rdi
  467777:	mov    0x10(%rsp),%r10
  46777c:	adc    $0x0,%rbp
  467780:	mulx   %r10,%rbx,%r8
  467785:	mov    %rax,%rdx
  467788:	mulx   %r15,%rsi,%r15
  46778d:	mov    %rcx,%rdx
  467790:	mulx   %r10,%rcx,%rdx
  467795:	add    %rdi,%rbx
  467798:	adc    %r8,%rcx
  46779b:	adc    $0x0,%rdx
  46779f:	add    %r9,%rsi
  4677a2:	adc    $0x0,%r15
  4677a6:	imul   %r10,%rax
  4677aa:	add    %rbp,%rsi
  4677ad:	adc    %r15,%rax
  4677b0:	add    %rcx,%rsi
  4677b3:	adc    %rdx,%rax
  4677b6:	cmpb   $0x1,(%r11)
  4677ba:	jne    4676b0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0xc0>
  4677c0:	mov    0x20(%rsp),%rdx
  4677c5:	xor    %ecx,%ecx
  4677c7:	or     %r12,%rdx
  4677ca:	or     %r13,%rdx
  4677cd:	or     %rbx,%rdx
  4677d0:	setne  %cl
  4677d3:	neg    %rcx
  4677d6:	mov    %rcx,%rdx
  4677d9:	sub    %rsi,%rdx
  4677dc:	sbb    %rax,%rcx
  4677df:	mov    %rdx,%rsi
  4677e2:	mov    %rcx,%rax
  4677e5:	jmp    4676b0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0xc0>
  4677ea:	nopw   0x0(%rax,%rax,1)
  4677f0:	mov    0x50(%rsp),%r12
  4677f5:	mov    0x58(%rsp),%rcx
  4677fa:	movabs $0x762cda976b2dec1e,%rsi
  467804:	mov    0x68(%rsp),%rdi
  467809:	mov    0x60(%rsp),%r9
  46780e:	mov    0x78(%rsp),%r10
  467813:	mov    0x70(%rsp),%r11
  467818:	mov    0x88(%rsp),%r15
  467820:	mov    0x80(%rsp),%r13
  467828:	movabs $0x762cda976b2dec1d,%rbx
  467832:	mov    $0xffffffffffffffff,%rax
  467839:	mov    %rcx,0x18(%rsp)
  46783e:	imul   %rsi,%rcx
  467842:	mov    %r12,%rdx
  467845:	mulx   %rsi,%r8,%rsi
  46784a:	mov    %r9,%rdx
  46784d:	mov    %r9,0x10(%rsp)
  467852:	movabs $0x44e992b44a6909f2,%r9
  46785c:	mov    %r13,0x20(%rsp)
  467861:	mov    %r15,0x28(%rsp)
  467866:	sub    %r12,%rsi
  467869:	add    %rcx,%rsi
  46786c:	add    0x30(%rsp),%r8
  467871:	mov    %rdi,0x30(%rsp)
  467876:	mov    %rdi,%rcx
  467879:	movabs $0x44e992b44a6909f1,%rdi
  467883:	adc    (%rsp),%rsi
  467887:	imul   %rdi,%rcx
  46788b:	mulx   %rdi,%rbp,%rdi
  467890:	mov    %r11,(%rsp)
  467894:	mov    %r11,%rdx
  467897:	add    %rcx,%rdi
  46789a:	add    %rbp,%r8
  46789d:	mov    %r10,%rcx
  4678a0:	adc    %rdi,%rsi
  4678a3:	imul   %r9,%rcx
  4678a7:	mulx   %r9,%r9,%r11
  4678ac:	mov    %r13,%rdx
  4678af:	mulx   %rbx,%r14,%rdx
  4678b4:	add    %rcx,%r11
  4678b7:	add    %r8,%r9
  4678ba:	mov    %r13,%rcx
  4678bd:	mov    %r15,%r8
  4678c0:	adc    %rsi,%r11
  4678c3:	imul   %rax,%rcx
  4678c7:	imul   %rbx,%r8
  4678cb:	add    %rcx,%r8
  4678ce:	add    %rdx,%r8
  4678d1:	add    %r9,%r14
  4678d4:	adc    %r11,%r8
  4678d7:	mov    %r8,%rdx
  4678da:	sar    $0x3f,%rdx
  4678de:	mov    %r8,%r11
  4678e1:	xor    %rdx,%r14
  4678e4:	xor    %rdx,%r11
  4678e7:	sub    %rdx,%r14
  4678ea:	sbb    %rdx,%r11
  4678ed:	cmp    $0x2,%r11
  4678f1:	jae    467d32 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  4678f7:	mov    0x20(%rsp),%rdx
  4678fc:	mov    0x28(%rsp),%rsi
  467901:	movabs $0xbb166d4bb596f60e,%r9
  46790b:	movabs $0xbb166d4bb596f60f,%r13
  467915:	imul   %r9,%rsi
  467919:	mulx   %r9,%r15,%rdx
  46791e:	add    %rsi,%rcx
  467921:	mov    %r10,%rsi
  467924:	add    %rdx,%rcx
  467927:	mov    (%rsp),%rdx
  46792b:	add    %r12,%rdx
  46792e:	adc    0x18(%rsp),%rsi
  467933:	mulx   %r13,%r9,%rbx
  467938:	sub    %rdx,%rbx
  46793b:	imul   %r13,%rsi
  46793f:	add    %rbx,%rsi
  467942:	sub    %rbp,%r15
  467945:	sbb    %rdi,%rcx
  467948:	add    %r9,%r15
  46794b:	adc    %rsi,%rcx
  46794e:	mov    %rcx,%rdx
  467951:	sar    $0x3f,%rdx
  467955:	mov    %rcx,%r9
  467958:	xor    %rdx,%r15
  46795b:	xor    %rdx,%r9
  46795e:	sub    %rdx,%r15
  467961:	sbb    %rdx,%r9
  467964:	cmp    $0x1,%r9
  467968:	ja     467d32 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  46796e:	mov    $0xffffffffffffffff,%r13
  467975:	mov    %r12,%rbx
  467978:	movabs $0xbb166d4bb596f60e,%rsi
  467982:	mov    %rbp,0x38(%rsp)
  467987:	mov    %r12,%rdx
  46798a:	imul   %r13,%rbx
  46798e:	mov    0x18(%rsp),%r13
  467993:	mov    %rsi,%rbp
  467996:	mulx   %rsi,%rsi,%rdx
  46799b:	imul   %rbp,%r13
  46799f:	movabs $0x44e992b44a6909f1,%rbp
  4679a9:	add    %rbx,%r13
  4679ac:	mov    0x28(%rsp),%rbx
  4679b1:	add    %rdx,%r13
  4679b4:	mov    0x20(%rsp),%rdx
  4679b9:	add    (%rsp),%rdx
  4679bd:	adc    %r10,%rbx
  4679c0:	mulx   %rbp,%rax,%rdx
  4679c5:	imul   %rbp,%rbx
  4679c9:	add    %rdx,%rbx
  4679cc:	sub    0x38(%rsp),%rsi
  4679d1:	sbb    %rdi,%r13
  4679d4:	add    %rax,%rsi
  4679d7:	adc    %rbx,%r13
  4679da:	mov    %r13,%rdx
  4679dd:	sar    $0x3f,%rdx
  4679e1:	mov    %r13,%rdi
  4679e4:	xor    %rdx,%rsi
  4679e7:	xor    %rdx,%rdi
  4679ea:	sub    %rdx,%rsi
  4679ed:	sbb    %rdx,%rdi
  4679f0:	cmp    $0x1,%rdi
  4679f4:	ja     467d32 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  4679fa:	mov    0x10(%rsp),%rdx
  4679ff:	movabs $0x762cda976b2dec1d,%rbx
  467a09:	mov    %r12,%rax
  467a0c:	mov    $0xffffffffffffffff,%rbp
  467a13:	mov    %r11,0x10(%rsp)
  467a18:	mov    0x18(%rsp),%r11
  467a1d:	mulx   %rbx,%r12,%rbx
  467a22:	imul   %rbp,%rdx
  467a26:	movabs $0x762cda976b2dec1d,%rbp
  467a30:	mov    %r12,0x38(%rsp)
  467a35:	mov    0x30(%rsp),%r12
  467a3a:	imul   %rbp,%r12
  467a3e:	add    %rdx,%r12
  467a41:	mov    (%rsp),%rdx
  467a45:	add    %rbx,%r12
  467a48:	movabs $0x89d3256894d213e2,%rbx
  467a52:	imul   %rbx,%r10
  467a56:	mulx   %rbx,%rbp,%rdx
  467a5b:	movabs $0x44e992b44a6909f1,%rbx
  467a65:	add    %rdx,%r10
  467a68:	sub    0x20(%rsp),%rax
  467a6d:	sbb    0x28(%rsp),%r11
  467a72:	mov    %rax,%rdx
  467a75:	mulx   %rbx,%rdx,%rax
  467a7a:	imul   %rbx,%r11
  467a7e:	add    %rax,%r11
  467a81:	add    0x38(%rsp),%rbp
  467a86:	adc    %r12,%r10
  467a89:	add    %rdx,%rbp
  467a8c:	adc    %r11,%r10
  467a8f:	mov    0x10(%rsp),%r11
  467a94:	mov    %r10,%rdx
  467a97:	sar    $0x3f,%rdx
  467a9b:	mov    %r10,%rax
  467a9e:	xor    %rdx,%rbp
  467aa1:	xor    %rdx,%rax
  467aa4:	sub    %rdx,%rbp
  467aa7:	sbb    %rdx,%rax
  467aaa:	cmp    $0x1,%rax
  467aae:	ja     467d32 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x742>
  467ab4:	mov    0xa8(%rsp),%rdx
  467abc:	cmp    0x48(%rsp),%rdx
  467ac1:	je     467d4a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x75a>
  467ac7:	shr    $0x3b,%r8
  467acb:	shr    $0x3a,%rcx
  467acf:	shr    $0x38,%r10
  467ad3:	shr    $0x39,%r13
  467ad7:	add    %r9d,%r9d
  467ada:	shl    $0x3,%eax
  467add:	shl    $0x2,%edi
  467ae0:	and    $0xfffffff0,%r8d
  467ae4:	and    $0xffffffe0,%ecx
  467ae7:	and    $0xffffff80,%r10d
  467aeb:	and    $0xffffffc0,%r13d
  467aef:	and    $0x2,%r9d
  467af3:	and    $0x8,%eax
  467af6:	and    $0x4,%edi
  467af9:	or     %r8,%rcx
  467afc:	or     %r13,%r9
  467aff:	mov    0x40(%rsp),%r8
  467b04:	mov    0xa0(%rsp),%r13
  467b0c:	or     %rax,%rdi
  467b0f:	lea    (%rdx,%rdx,4),%rax
  467b13:	inc    %rdx
  467b16:	or     %rcx,%r10
  467b19:	mov    0x8(%rsp),%rcx
  467b1e:	or     %r11,%r10
  467b21:	or     %r10,%r9
  467b24:	or     %r9,%rdi
  467b27:	mov    %r14,(%r8,%rax,8)
  467b2b:	mov    %r15,0x8(%r8,%rax,8)
  467b30:	mov    %rsi,0x10(%r8,%rax,8)
  467b35:	mov    %rbp,0x18(%r8,%rax,8)
  467b3a:	mov    %rdi,0x20(%r8,%rax,8)
  467b3f:	add    $0x20,%rcx
  467b43:	mov    %rcx,0x8(%rsp)
  467b48:	cmp    %r13,%rcx
  467b4b:	jne    467670 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x80>
  467b51:	jmp    467b7d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x58d>
  467b53:	test   %dl,%dl
  467b55:	je     467b6d <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x57d>
  467b57:	mov    %fs:0xffffffffffffffc0,%rax
  467b60:	test   %rax,%rax
  467b63:	je     467b9f <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x5af>
  467b65:	add    $0x110,%rax
  467b6b:	jmp    467bb0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x5c0>
  467b6d:	test   %rcx,%rcx
  467b70:	je     467635 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x45>
  467b76:	shr    $1,%rcx
  467b79:	jmp    467bc4 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x5d4>
  467b7b:	xor    %edx,%edx
  467b7d:	mov    0x98(%rsp),%rax
  467b85:	mov    0x40(%rsp),%rcx
  467b8a:	mov    0x48(%rsp),%rsi
  467b8f:	mov    %rcx,(%rax)
  467b92:	mov    %rsi,0x8(%rax)
  467b96:	mov    %rdx,0x10(%rax)
  467b9a:	jmp    467d20 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x730>
  467b9f:	mov    %rsi,%r15
  467ba2:	mov    %rcx,%r14
  467ba5:	call   105550 <rayon_core::registry::global_registry>
  467baa:	mov    %r14,%rcx
  467bad:	mov    %r15,%rsi
  467bb0:	mov    (%rax),%rax
  467bb3:	shr    $1,%rcx
  467bb6:	mov    0x208(%rax),%rax
  467bbd:	cmp    %rax,%rcx
  467bc0:	cmovbe %rax,%rcx
  467bc4:	sub    %rsi,%r13
  467bc7:	mov    %rcx,0xc0(%rsp)
  467bcf:	mov    %rsi,0xb8(%rsp)
  467bd7:	jb     467d62 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x772>
  467bdd:	mov    0x10(%rbx),%rax
  467be1:	sub    %rsi,%rax
  467be4:	jb     467d7a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x78a>
  467bea:	mov    0x8(%rbx),%r8
  467bee:	mov    0x8(%rsp),%rdx
  467bf3:	mov    (%rbx),%rdi
  467bf6:	lea    (%rsi,%rsi,4),%r9
  467bfa:	mov    %rsi,%rcx
  467bfd:	shl    $0x5,%rcx
  467c01:	lea    0xb0(%rsp),%r10
  467c09:	lea    0xc0(%rsp),%r11
  467c11:	mov    %r10,0xd0(%rsp)
  467c19:	lea    0xb8(%rsp),%r10
  467c21:	mov    %r10,0xd8(%rsp)
  467c29:	mov    %r11,0xe0(%rsp)
  467c31:	lea    (%r8,%r9,8),%r9
  467c35:	add    %rdx,%rcx
  467c38:	mov    %rcx,0xe8(%rsp)
  467c40:	mov    %r13,0xf0(%rsp)
  467c48:	mov    %rdi,0xf8(%rsp)
  467c50:	mov    %r9,0x100(%rsp)
  467c58:	mov    %rax,0x108(%rsp)
  467c60:	mov    %r10,0x110(%rsp)
  467c68:	mov    %r11,0x118(%rsp)
  467c70:	mov    %rdx,0x120(%rsp)
  467c78:	mov    %fs:0xffffffffffffffc0,%rdx
  467c81:	mov    %rsi,0x128(%rsp)
  467c89:	mov    %rdi,0x130(%rsp)
  467c91:	mov    %r8,0x138(%rsp)
  467c99:	mov    %rsi,0x140(%rsp)
  467ca1:	test   %rdx,%rdx
  467ca4:	jne    467ccd <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x6dd>
  467ca6:	call   105550 <rayon_core::registry::global_registry>
  467cab:	mov    (%rax),%rsi
  467cae:	mov    %fs:0xffffffffffffffc0,%rdx
  467cb7:	test   %rdx,%rdx
  467cba:	je     467d92 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x7a2>
  467cc0:	cmp    %rsi,0x110(%rdx)
  467cc7:	jne    467dad <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x7bd>
  467ccd:	lea    0x50(%rsp),%rdi
  467cd2:	lea    0xd0(%rsp),%rsi
  467cda:	xor    %ecx,%ecx
  467cdc:	call   489540 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}>
  467ce1:	mov    0x60(%rsp),%rcx
  467ce6:	mov    0x50(%rsp),%rax
  467ceb:	lea    (%rcx,%rcx,4),%rdx
  467cef:	lea    (%rax,%rdx,8),%rdx
  467cf3:	cmp    0x68(%rsp),%rdx
  467cf8:	jne    467d06 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x716>
  467cfa:	mov    0x70(%rsp),%rdx
  467cff:	mov    0x78(%rsp),%rsi
  467d04:	jmp    467d0a <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x71a>
  467d06:	xor    %edx,%edx
  467d08:	xor    %esi,%esi
  467d0a:	add    0x58(%rsp),%rdx
  467d0f:	add    %rcx,%rsi
  467d12:	mov    %rax,(%r12)
  467d16:	mov    %rdx,0x8(%r12)
  467d1b:	mov    %rsi,0x10(%r12)
  467d20:	add    $0x148,%rsp
  467d27:	pop    %rbx
  467d28:	pop    %r12
  467d2a:	pop    %r13
  467d2c:	pop    %r14
  467d2e:	pop    %r15
  467d30:	pop    %rbp
  467d31:	ret
  467d32:	lea    -0x3dd869(%rip),%rdi        # 8a4d0 <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x100>
  467d39:	lea    0x8c5b8(%rip),%rdx        # 4f42f8 <__frame_dummy_init_array_entry+0x7998>
  467d40:	mov    $0x29,%esi
  467d45:	call   f1800 <core::panicking::panic>
  467d4a:	lea    -0x3dd766(%rip),%rdi        # 8a5eb <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x21b>
  467d51:	lea    0x8c780(%rip),%rdx        # 4f44d8 <__frame_dummy_init_array_entry+0x7b78>
  467d58:	mov    $0x45,%esi
  467d5d:	call   ef730 <core::panicking::panic_fmt>
  467d62:	lea    -0x448233(%rip),%rdi        # 1fb36 <zmij::STATIC_DATA+0x55f6>
  467d69:	lea    0x8c780(%rip),%rdx        # 4f44f0 <__frame_dummy_init_array_entry+0x7b90>
  467d70:	mov    $0x13,%esi
  467d75:	call   ef730 <core::panicking::panic_fmt>
  467d7a:	lea    -0x3dd7b4(%rip),%rdi        # 8a5cd <core::unicode::unicode_data::white_space::WHITESPACE_MAP+0x1fd>
  467d81:	lea    0x8c720(%rip),%rdx        # 4f44a8 <__frame_dummy_init_array_entry+0x7b48>
  467d88:	mov    $0x1e,%esi
  467d8d:	call   f1800 <core::panicking::panic>
  467d92:	sub    $0xffffffffffffff80,%rsi
  467d96:	lea    0x50(%rsp),%rdi
  467d9b:	lea    0xd0(%rsp),%rdx
  467da3:	call   489440 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>
  467da8:	jmp    467ce1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x6f1>
  467dad:	sub    $0xffffffffffffff80,%rsi
  467db1:	lea    0x50(%rsp),%rdi
  467db6:	lea    0xd0(%rsp),%rcx
  467dbe:	call   4899d0 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>
  467dc3:	jmp    467ce1 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>+0x6f1>
