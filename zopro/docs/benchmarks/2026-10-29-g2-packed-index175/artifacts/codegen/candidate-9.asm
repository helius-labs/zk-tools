
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000291740 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>>:
  291740:	push   %rbx
  291741:	sub    $0xf0,%rsp
  291748:	vmovups (%rdx),%zmm0
  29174e:	vmovups 0x40(%rdx),%zmm1
  291755:	vmovups 0x70(%rdx),%zmm2
  29175f:	mov    0x100(%rsi),%rcx
  291766:	lea    0x110(%rsi),%rax
  29176d:	mov    %rsi,%rbx
  291770:	mov    %rax,0xc8(%rsp)
  291778:	movq   $0x0,0xd0(%rsp)
  291784:	mov    %rcx,0xd8(%rsp)
  29178c:	movb   $0x1,0xe0(%rsp)
  291794:	vmovups %zmm1,0x40(%rsp)
  29179c:	vmovups %zmm0,(%rsp)
  2917a3:	vmovups %zmm2,0x70(%rsp)
  2917ae:	movq   $0x0,0xb0(%rsp)
  2917ba:	lea    0x8f(%rip),%rsi        # 291850 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())> as rayon_core::job::Job>::execute>
  2917c1:	mov    %rsp,%rdx
  2917c4:	vzeroupper
  2917c7:	call   104fe0 <<rayon_core::registry::Registry>::inject>
  2917cc:	mov    0xd0(%rsp),%rax
  2917d4:	cmp    $0x3,%rax
  2917d8:	jne    29180c <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>+0xcc>
  2917da:	mov    0xb0(%rsp),%rax
  2917e2:	cmp    $0x1,%rax
  2917e6:	je     291803 <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>+0xc3>
  2917e8:	cmp    $0x2,%rax
  2917ec:	jne    29181e <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>+0xde>
  2917ee:	mov    0xb8(%rsp),%rdi
  2917f6:	mov    0xc0(%rsp),%rsi
  2917fe:	call   104da0 <rayon_core::unwind::resume_unwinding>
  291803:	add    $0xf0,%rsp
  29180a:	pop    %rbx
  29180b:	ret
  29180c:	lea    0xd0(%rsp),%rsi
  291814:	mov    %rbx,%rdi
  291817:	call   feab0 <<rayon_core::registry::WorkerThread>::wait_until_cold>
  29181c:	jmp    2917da <<rayon_core::registry::Registry>::in_worker_cross::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>+0x9a>
  29181e:	lea    -0x2724d9(%rip),%rdi        # 1f34c <zmij::STATIC_DATA+0x4d4c>
  291825:	lea    0x256dcc(%rip),%rdx        # 4e85f8 <__frame_dummy_init_array_entry+0x6798>
  29182c:	mov    $0x28,%esi
  291831:	call   f0cd0 <core::panicking::panic>
  291836:	mov    %rax,%rbx
  291839:	mov    %rsp,%rdi
  29183c:	call   291aa0 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, <rayon_core::registry::Registry>::in_worker_cross<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}, ((), ())>>>
  291841:	mov    %rbx,%rdi
  291844:	call   4e0e10 <_Unwind_Resume@plt>
  291849:	call   f2163 <core::panicking::panic_in_cleanup>
