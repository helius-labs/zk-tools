
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000486090 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}>:
  486090:	push   %rbp
  486091:	push   %r15
  486093:	push   %r14
  486095:	push   %r13
  486097:	push   %r12
  486099:	push   %rbx
  48609a:	sub    $0x148,%rsp
  4860a1:	vmovups (%rsi),%zmm0
  4860a7:	mov    %ecx,(%rsp)
  4860aa:	mov    0x100(%rdx),%rcx
  4860b1:	lea    0x110(%rdx),%rax
  4860b8:	mov    %rdi,0x8(%rsp)
  4860bd:	mov    %rsi,%r12
  4860c0:	mov    %rdx,%r14
  4860c3:	mov    %rax,0x70(%rsp)
  4860c8:	movq   $0x0,0x78(%rsp)
  4860d1:	mov    %rcx,0x80(%rsp)
  4860d9:	movb   $0x0,0x88(%rsp)
  4860e1:	vmovups %zmm0,0x10(%rsp)
  4860ec:	movq   $0x0,0x50(%rsp)
  4860f5:	mov    0x118(%rdx),%rax
  4860fc:	mov    0x108(%rax),%rbx
  486103:	mov    0x100(%rax),%r15
  48610a:	mov    0x118(%rdx),%rcx
  486111:	mov    0x108(%rcx),%rbp
  486118:	mov    0x100(%rcx),%rax
  48611f:	mov    0x128(%rdx),%rsi
  486126:	mov    %rbp,%rcx
  486129:	sub    %rax,%rcx
  48612c:	cmp    %rsi,%rcx
  48612f:	jge    486448 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3b8>
  486135:	mov    0x120(%r14),%rax
  48613c:	dec    %rsi
  48613f:	sub    %r15,%rbx
  486142:	lea    0x787(%rip),%r13        # 4868d0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>> as rayon_core::job::Job>::execute>
  486149:	lea    0x10(%rsp),%r15
  48614e:	movabs $0x100000000,%rcx
  486158:	and    %rbp,%rsi
  48615b:	inc    %rbp
  48615e:	shl    $0x4,%rsi
  486162:	mov    %r13,(%rax,%rsi,1)
  486166:	mov    %r15,0x8(%rax,%rsi,1)
  48616b:	mov    0x118(%r14),%rdx
  486172:	mov    %rbp,0x108(%rdx)
  486179:	mov    0x110(%r14),%rsi
  486180:	lea    0x1d8(%rsi),%rdi
  486187:	nopw   0x0(%rax,%rax,1)
  486190:	mov    0x1f0(%rsi),%rax
  486197:	test   %rcx,%rax
  48619a:	jne    4861ba <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x12a>
  48619c:	mov    %rax,%rdx
  48619f:	or     %rcx,%rdx
  4861a2:	lock cmpxchg %rdx,0x1f0(%rsi)
  4861ab:	jne    486190 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x100>
  4861ad:	mov    %rdx,%rax
  4861b0:	and    $0xffff,%rax
  4861b6:	jne    4861c8 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x138>
  4861b8:	jmp    4861dc <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x14c>
  4861ba:	mov    %rax,%rdx
  4861bd:	mov    %rdx,%rax
  4861c0:	and    $0xffff,%rax
  4861c6:	je     4861dc <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x14c>
  4861c8:	test   %rbx,%rbx
  4861cb:	jg     486436 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3a6>
  4861d1:	shr    $0x10,%edx
  4861d4:	cmp    %eax,%edx
  4861d6:	je     486436 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3a6>
  4861dc:	mov    0x48(%r12),%rdx
  4861e1:	mov    0x40(%r12),%rax
  4861e6:	mov    0x50(%r12),%r9
  4861eb:	lea    0x60(%r12),%r10
  4861f0:	mov    (%rax),%rsi
  4861f3:	mov    (%rdx),%rcx
  4861f6:	mov    0x8(%rdx),%r8
  4861fa:	movzbl (%rsp),%ebp
  4861fe:	lea    0xc0(%rsp),%rdi
  486206:	mov    %ebp,%edx
  486208:	push   %r10
  48620a:	push   0x58(%r12)
  48620f:	vzeroupper
  486212:	call   4672d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  486217:	add    $0x10,%rsp
  48621b:	mov    0xc8(%rsp),%rax
  486223:	mov    0xc0(%rsp),%r12
  48622b:	mov    0xd0(%rsp),%rbx
  486233:	mov    %rax,(%rsp)
  486237:	nopw   0x0(%rax,%rax,1)
  486240:	mov    0x78(%rsp),%rax
  486245:	cmp    $0x3,%rax
  486249:	je     48627f <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x1ef>
  48624b:	mov    %r14,%rdi
  48624e:	call   1c3800 <<rayon_core::registry::WorkerThread>::take_local_job>
  486253:	test   %rax,%rax
  486256:	je     486270 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x1e0>
  486258:	mov    %r15,%rcx
  48625b:	mov    %rax,%rsi
  48625e:	xor    %rdx,%rcx
  486261:	xor    %r13,%rsi
  486264:	or     %rcx,%rsi
  486267:	je     4862c4 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x234>
  486269:	mov    %rdx,%rdi
  48626c:	call   *%rax
  48626e:	jmp    486240 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x1b0>
  486270:	mov    0x78(%rsp),%rax
  486275:	cmp    $0x3,%rax
  486279:	jne    48647e <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3ee>
  48627f:	vmovdqu 0x58(%rsp),%xmm0
  486285:	mov    0x50(%rsp),%rax
  48628a:	cmp    $0x1,%rax
  48628e:	je     4862aa <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x21a>
  486290:	cmp    $0x2,%rax
  486294:	jne    486466 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x3d6>
  48629a:	vmovq  %xmm0,%rdi
  48629f:	vpextrq $0x1,%xmm0,%rsi
  4862a5:	call   105640 <rayon_core::unwind::resume_unwinding>
  4862aa:	mov    0x8(%rsp),%rcx
  4862af:	mov    (%rsp),%rdx
  4862b3:	mov    0x68(%rsp),%rax
  4862b8:	mov    %r12,(%rcx)
  4862bb:	mov    %rdx,0x8(%rcx)
  4862bf:	jmp    486417 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x387>
  4862c4:	vmovups 0x10(%rsp),%zmm0
  4862cf:	vmovups 0x50(%rsp),%zmm1
  4862da:	vmovups %zmm0,0xc0(%rsp)
  4862e2:	vmovups %zmm1,0x100(%rsp)
  4862ea:	mov    0xc0(%rsp),%rax
  4862f2:	test   %rax,%rax
  4862f5:	je     486490 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x400>
  4862fb:	vmovups 0x38(%rsp),%xmm0
  486301:	mov    0x48(%rsp),%rsi
  486306:	mov    0xc8(%rsp),%rcx
  48630e:	mov    0xd0(%rsp),%rdx
  486316:	mov    0xe0(%rsp),%r10
  48631e:	mov    0xd8(%rsp),%r9
  486326:	mov    %rsi,0xa0(%rsp)
  48632e:	vmovaps %xmm0,0x90(%rsp)
  486337:	mov    (%rax),%rsi
  48633a:	mov    0x8(%rdx),%r8
  48633e:	sub    (%rcx),%rsi
  486341:	mov    (%rdx),%rcx
  486344:	lea    0x90(%rsp),%rax
  48634c:	mov    %ebp,%edx
  48634e:	lea    0xa8(%rsp),%rdi
  486356:	push   %rax
  486357:	push   %r10
  486359:	vzeroupper
  48635c:	call   4672d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>>
  486361:	add    $0x10,%rsp
  486365:	cmpq   $0x2,0x100(%rsp)
  48636e:	jb     4863f6 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x366>
  486374:	mov    0x110(%rsp),%r15
  48637c:	mov    0x108(%rsp),%r14
  486384:	mov    (%r15),%rax
  486387:	test   %rax,%rax
  48638a:	je     486391 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x301>
  48638c:	mov    %r14,%rdi
  48638f:	call   *%rax
  486391:	mov    0x8(%r15),%rsi
  486395:	test   %rsi,%rsi
  486398:	je     4863f6 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x366>
  48639a:	mov    0x10(%r15),%rax
  48639e:	cmp    $0x20000,%rsi
  4863a5:	setae  %cl
  4863a8:	cmp    $0x1001,%rax
  4863ae:	setb   %dl
  4863b1:	test   %cl,%dl
  4863b3:	je     4863eb <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x35b>
  4863b5:	blsr   %eax,%ecx
  4863ba:	jne    4864a1 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x411>
  4863c0:	movabs $0x8000000000000000,%rcx
  4863ca:	sub    %rax,%rcx
  4863cd:	cmp    %rcx,%rsi
  4863d0:	jae    4864a1 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x411>
  4863d6:	cmpb   $0x1,(%r14,%rsi,1)
  4863db:	jne    4863ed <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x35d>
  4863dd:	inc    %rsi
  4863e0:	mov    %r14,%rdi
  4863e3:	call   *0x6bdef(%rip)        # 4f21d8 <munmap@GLIBC_2.2.5>
  4863e9:	jmp    4863f6 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x366>
  4863eb:	jmp    4863ed <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x35d>
  4863ed:	mov    %r14,%rdi
  4863f0:	call   *0x6bdea(%rip)        # 4f21e0 <free@GLIBC_2.2.5>
  4863f6:	mov    0x8(%rsp),%rcx
  4863fb:	mov    (%rsp),%rax
  4863ff:	vmovdqu 0xa8(%rsp),%xmm0
  486408:	mov    %r12,(%rcx)
  48640b:	mov    %rax,0x8(%rcx)
  48640f:	mov    0xb8(%rsp),%rax
  486417:	mov    %rbx,0x10(%rcx)
  48641b:	vmovdqu %xmm0,0x18(%rcx)
  486420:	mov    %rax,0x28(%rcx)
  486424:	add    $0x148,%rsp
  48642b:	pop    %rbx
  48642c:	pop    %r12
  48642e:	pop    %r13
  486430:	pop    %r14
  486432:	pop    %r15
  486434:	pop    %rbp
  486435:	ret
  486436:	mov    $0x1,%esi
  48643b:	vzeroupper
  48643e:	call   ffb90 <<rayon_core::sleep::Sleep>::wake_any_threads>
  486443:	jmp    4861dc <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x14c>
  486448:	lea    0x118(%r14),%rdi
  48644f:	add    %rsi,%rsi
  486452:	vzeroupper
  486455:	call   1c32b0 <<crossbeam_deque::deque::Worker<rayon_core::job::JobRef>>::resize>
  48645a:	mov    0x128(%r14),%rsi
  486461:	jmp    486135 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0xa5>
  486466:	lea    -0x467221(%rip),%rdi        # 1f24c <zmij::STATIC_DATA+0x4d4c>
  48646d:	lea    0x696fc(%rip),%rdx        # 4efb70 <__frame_dummy_init_array_entry+0x66c0>
  486474:	mov    $0x28,%esi
  486479:	call   f1580 <core::panicking::panic>
  48647e:	lea    0x78(%rsp),%rsi
  486483:	mov    %r14,%rdi
  486486:	call   ff350 <<rayon_core::registry::WorkerThread>::wait_until_cold>
  48648b:	jmp    48627f <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x1ef>
  486490:	lea    0x69499(%rip),%rdi        # 4ef930 <__frame_dummy_init_array_entry+0x6480>
  486497:	vzeroupper
  48649a:	call   f28a0 <core::option::unwrap_failed>
  48649f:	jmp    4864e3 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x453>
  4864a1:	call   111210 <std::process::abort>
  4864a6:	jmp    4864e7 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x457>
  4864a8:	mov    0x8(%r15),%rsi
  4864ac:	mov    %rax,%rbx
  4864af:	test   %rsi,%rsi
  4864b2:	je     48650b <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x47b>
  4864b4:	mov    0x10(%r15),%rdx
  4864b8:	mov    %r14,%rdi
  4864bb:	call   e99d0 <__rustc::__rust_dealloc>
  4864c0:	mov    %rbx,%rdi
  4864c3:	call   4e8460 <_Unwind_Resume@plt>
  4864c8:	mov    %rax,%rdi
  4864cb:	call   1450fa <std::panicking::catch_unwind::cleanup>
  4864d0:	mov    %rdx,%rcx
  4864d3:	lea    0x70(%rsp),%rsi
  4864d8:	mov    %r14,%rdi
  4864db:	mov    %rax,%rdx
  4864de:	call   105af0 <rayon_core::join::join_recover_from_panic>
  4864e3:	ud2
  4864e5:	jmp    4864e7 <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x457>
  4864e7:	mov    %rax,%rbx
  4864ea:	lea    0x10(%rsp),%rdi
  4864ef:	call   221320 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<[u64; 4]>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<zopro_bn254::fp::Fp<zopro_bn254::FrConfig>>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 4]>, <zopro_bn254::fp::Fp<zopro_bn254::FrConfig>>::to_canonical>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<[u64; 4]>>>>
  4864f4:	jmp    48650b <rayon_core::join::join_context::<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}+0x47b>
  4864f6:	call   f2a13 <core::panicking::panic_in_cleanup>
  4864fb:	mov    %rax,%rbx
  4864fe:	lea    0x100(%rsp),%rdi
  486506:	call   221270 <core::ptr::drop_glue::<core::cell::UnsafeCell<rayon_core::job::JobResult<rayon::iter::collect::consumer::CollectResult<[u64; 4]>>>>>
  48650b:	mov    %rbx,%rdi
  48650e:	call   4e8460 <_Unwind_Resume@plt>
  486513:	call   f2a13 <core::panicking::panic_in_cleanup>
