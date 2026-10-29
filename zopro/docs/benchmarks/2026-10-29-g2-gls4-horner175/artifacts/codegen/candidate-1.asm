
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000485f90 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>>:
  485f90:	push   %rbx
  485f91:	sub    $0xc0,%rsp
  485f98:	vmovups (%rdx),%zmm0
  485f9e:	vmovups 0x38(%rdx),%zmm1
  485fa8:	mov    %fs:0x0,%rcx
  485fb1:	mov    %rsi,%rax
  485fb4:	mov    %rdi,%rbx
  485fb7:	lea    -0x28(%rcx),%rcx
  485fbe:	mov    %rcx,0x8(%rsp)
  485fc3:	vmovups %zmm0,0x10(%rsp)
  485fce:	vmovups %zmm1,0x48(%rsp)
  485fd9:	movq   $0x0,0x88(%rsp)
  485fe5:	lea    0xb14(%rip),%rsi        # 486b00 <<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)> as rayon_core::job::Job>::execute>
  485fec:	lea    0x8(%rsp),%rdx
  485ff1:	mov    %rax,%rdi
  485ff4:	vzeroupper
  485ff7:	call   105880 <<rayon_core::registry::Registry>::inject>
  485ffc:	mov    0x8(%rsp),%rdi
  486001:	call   105650 <<rayon_core::latch::LockLatch>::wait_and_reset>
  486006:	vmovdqu 0x90(%rsp),%xmm0
  48600f:	mov    0x88(%rsp),%rax
  486017:	cmp    $0x1,%rax
  48601b:	je     486033 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xa3>
  48601d:	cmp    $0x2,%rax
  486021:	jne    486051 <<rayon_core::registry::Registry>::in_worker_cold::<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::slice::IterProducer<[u64; 4]>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<[u64; 5]>, zopro_core::msm_affine::g2_borrowed::gls4::try_raw::{closure#0}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<[u64; 5]>, rayon::iter::collect::consumer::CollectResult<[u64; 5]>)>+0xc1>
  486023:	vmovq  %xmm0,%rdi
  486028:	vpextrq $0x1,%xmm0,%rsi
  48602e:	call   105640 <rayon_core::unwind::resume_unwinding>
  486033:	vmovups 0xa0(%rsp),%ymm1
  48603c:	vmovups %ymm1,0x10(%rbx)
  486041:	vmovdqu %xmm0,(%rbx)
  486045:	add    $0xc0,%rsp
  48604c:	pop    %rbx
  48604d:	vzeroupper
  486050:	ret
  486051:	lea    -0x466e0c(%rip),%rdi        # 1f24c <zmij::STATIC_DATA+0x4d4c>
  486058:	lea    0x69b11(%rip),%rdx        # 4efb70 <__frame_dummy_init_array_entry+0x66c0>
  48605f:	mov    $0x28,%esi
  486064:	call   f1580 <core::panicking::panic>
  486069:	mov    %rax,%rbx
  48606c:	lea    0x8(%rsp),%rdi
  486071:	call   1fed00 <core::ptr::drop_glue::<rayon_core::job::StackJob<rayon_core::latch::LatchRef<rayon_core::latch::LockLatch>, <rayon_core::registry::Registry>::in_worker_cold<rayon_core::join::join_context<rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#0}, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, zopro_core::msm_affine::msm_unsigned_raw<zopro_bn254::curve::G1Config>::{closure#1}>>::{closure#1}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>>::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>::{closure#0}::{closure#0}, (rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>>)>>>
  486076:	mov    %rbx,%rdi
  486079:	call   4e8460 <_Unwind_Resume@plt>
  48607e:	call   f2a13 <core::panicking::panic_in_cleanup>
