
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000002911d0 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>>:
  2911d0:	push   %rbx
  2911d1:	sub    $0xd0,%rsp
  2911d8:	vmovups (%rsi),%zmm0
  2911de:	vmovups 0x40(%rsi),%zmm1
  2911e5:	vmovups 0x70(%rsi),%zmm2
  2911ef:	mov    %fs:0x0,%rax
  2911f8:	lea    -0x28(%rax),%rax
  2911ff:	mov    %rax,0xb0(%rsp)
  291207:	vmovups %zmm1,0x40(%rsp)
  29120f:	vmovups %zmm0,(%rsp)
  291216:	vmovups %zmm2,0x70(%rsp)
  291221:	movq   $0x0,0xb8(%rsp)
  29122d:	lea    0xb2c(%rip),%rsi        # 291d60 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())> as rayon_core::job::Job>::execute>
  291234:	mov    %rsp,%rdx
  291237:	vzeroupper
  29123a:	call   104fe0 <<rayon_core::registry::Registry>::inject>
  29123f:	mov    0xb0(%rsp),%rdi
  291247:	call   104db0 <<rayon_core::latch::LockLatch>::wait_and_reset>
  29124c:	mov    0xb8(%rsp),%rax
  291254:	cmp    $0x1,%rax
  291258:	je     291275 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>+0xa5>
  29125a:	cmp    $0x2,%rax
  29125e:	jne    29127e <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>+0xae>
  291260:	mov    0xc0(%rsp),%rdi
  291268:	mov    0xc8(%rsp),%rsi
  291270:	call   104da0 <rayon_core::unwind::resume_unwinding>
  291275:	add    $0xd0,%rsp
  29127c:	pop    %rbx
  29127d:	ret
  29127e:	lea    -0x271f39(%rip),%rdi        # 1f34c <zmij::STATIC_DATA+0x4d4c>
  291285:	lea    0x25736c(%rip),%rdx        # 4e85f8 <__frame_dummy_init_array_entry+0x6798>
  29128c:	mov    $0x28,%esi
  291291:	call   f0cd0 <core::panicking::panic>
  291296:	mov    %rax,%rbx
  291299:	mov    %rsp,%rdi
  29129c:	call   291f30 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#0}>::{closure#0}, rayon_core::join::join::call<(), zopro_core::msm::packed_index::fill<zopro_bn254::curve::G2Config>::{closure#1}>::{closure#0}, (), ()>::{closure#0}, ((), ())>::{closure#0}::{closure#0}, ((), ())>>>
  2912a1:	mov    %rbx,%rdi
  2912a4:	call   4e0e10 <_Unwind_Resume@plt>
  2912a9:	call   f2163 <core::panicking::panic_in_cleanup>
