
/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

000000000047f0d0 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut>:
  47f0d0:	push   %rbp
  47f0d1:	push   %r15
  47f0d3:	push   %r14
  47f0d5:	push   %r13
  47f0d7:	push   %r12
  47f0d9:	push   %rbx
  47f0da:	sub    $0xf8,%rsp
  47f0e1:	mov    %rdx,0xe8(%rsp)
  47f0e9:	mov    %rdi,%rbx
  47f0ec:	vmovups (%rsi),%xmm0
  47f0f0:	mov    0x10(%rsi),%r12
  47f0f4:	mov    0x18(%rsi),%r13
  47f0f8:	mov    0x20(%rsi),%rbp
  47f0fc:	mov    0x28(%rsi),%r14
  47f100:	movq   $0x0,0x38(%rsp)
  47f109:	movq   $0x8,0x40(%rsp)
  47f112:	movq   $0x0,0x48(%rsp)
  47f11b:	movq   $0x4,0xf0(%rsp)
  47f127:	vmovaps %xmm0,0x20(%rsp)
  47f12d:	lea    0x38(%rsp),%rdi
  47f132:	mov    $0x4,%edx
  47f137:	mov    $0x8,%ecx
  47f13c:	mov    $0x18,%r8d
  47f142:	xor    %esi,%esi
  47f144:	call   1b7000 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  47f149:	mov    0x38(%rsp),%rax
  47f14e:	mov    0x48(%rsp),%r15
  47f153:	sub    %r15,%rax
  47f156:	cmp    $0x3,%rax
  47f15a:	ja     47f179 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0xa9>
  47f15c:	lea    -0x45f5f3(%rip),%rdi        # 1fb70 <zmij::STATIC_DATA+0x5670>
  47f163:	lea    0x70586(%rip),%rdx        # 4ef6f0 <__frame_dummy_init_array_entry+0x6240>
  47f16a:	mov    $0x2f,%esi
  47f16f:	call   f1580 <core::panicking::panic>
  47f174:	jmp    47f605 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x535>
  47f179:	lea    (%r15,%r15,2),%rax
  47f17d:	vmovaps 0x20(%rsp),%xmm0
  47f183:	lea    0x90(%rsp),%rdx
  47f18b:	movq   $0x4,0x68(%rsp)
  47f194:	lea    0xe8(%rsp),%rcx
  47f19c:	mov    %rdx,0x50(%rsp)
  47f1a1:	shl    $0x3,%rax
  47f1a5:	add    0x40(%rsp),%rax
  47f1aa:	mov    %rax,0x58(%rsp)
  47f1af:	mov    %fs:0xffffffffffffffc0,%rax
  47f1b8:	movq   $0x4,0x60(%rsp)
  47f1c1:	vmovaps %xmm0,0x90(%rsp)
  47f1ca:	mov    %r12,0xa0(%rsp)
  47f1d2:	mov    %r13,0xa8(%rsp)
  47f1da:	mov    %rbp,0xb0(%rsp)
  47f1e2:	mov    %rcx,0xb8(%rsp)
  47f1ea:	mov    %r14,0xc0(%rsp)
  47f1f2:	movq   $0x0,0xc8(%rsp)
  47f1fe:	movq   $0x4,0xd0(%rsp)
  47f20a:	test   %rax,%rax
  47f20d:	je     47f217 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x147>
  47f20f:	add    $0x110,%rax
  47f215:	jmp    47f21c <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x14c>
  47f217:	call   1052d0 <rayon_core::registry::global_registry>
  47f21c:	mov    (%rax),%rax
  47f21f:	mov    0x208(%rax),%rcx
  47f226:	lea    0x50(%rsp),%r12
  47f22b:	xor    %edx,%edx
  47f22d:	lea    0x70(%rsp),%rdi
  47f232:	mov    $0x4,%esi
  47f237:	mov    $0x1,%r8d
  47f23d:	xor    %r9d,%r9d
  47f240:	push   %r12
  47f242:	push   $0x4
  47f244:	call   47f6d0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::range::IterProducer<usize>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0}::{closure#0}>>>
  47f249:	add    $0x10,%rsp
  47f24d:	mov    0x80(%rsp),%r14
  47f255:	mov    %r14,0x50(%rsp)
  47f25a:	cmp    $0x4,%r14
  47f25e:	jne    47f5b4 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4e4>
  47f264:	vmovups 0x38(%rsp),%xmm0
  47f26a:	add    $0x4,%r15
  47f26e:	mov    %r15,0x80(%rsp)
  47f276:	vmovaps %xmm0,0x70(%rsp)
  47f27c:	je     47f600 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x530>
  47f282:	mov    0x78(%rsp),%r13
  47f287:	dec    %r15
  47f28a:	mov    0x0(%r13),%rax
  47f28e:	mov    0x8(%r13),%rcx
  47f292:	mov    0x10(%r13),%r12
  47f296:	mov    %r13,0x10(%rsp)
  47f29b:	mov    %rax,0x18(%rsp)
  47f2a0:	lea    (%r15,%r15,2),%rax
  47f2a4:	mov    %rcx,0x20(%rsp)
  47f2a9:	vmovups 0x0(%r13,%rax,8),%xmm0
  47f2b0:	mov    0x10(%r13,%rax,8),%rcx
  47f2b5:	lea    0x0(%r13,%rax,8),%rdx
  47f2ba:	mov    %rdx,0x88(%rsp)
  47f2c2:	mov    %rcx,0x10(%r13)
  47f2c6:	vmovups %xmm0,0x0(%r13)
  47f2cc:	mov    %r15,0x80(%rsp)
  47f2d4:	mov    %r13,0x50(%rsp)
  47f2d9:	mov    0x70(%rsp),%rax
  47f2de:	mov    %rax,0x60(%rsp)
  47f2e3:	mov    %rax,0x30(%rsp)
  47f2e8:	mov    %rdx,0x68(%rsp)
  47f2ed:	je     47f432 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x362>
  47f2f3:	mov    0x10(%rsp),%rcx
  47f2f8:	lea    0x1ff(%r12),%rax
  47f300:	shr    $0x9,%rax
  47f304:	mov    %rax,0xe0(%rsp)
  47f30c:	lea    0x18(%rcx),%r13
  47f310:	jmp    47f331 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x261>
  47f312:	mov    %r15,%rdi
  47f315:	call   *0x72ec5(%rip)        # 4f21e0 <free@GLIBC_2.2.5>
  47f31b:	add    $0x18,%r13
  47f31f:	add    $0x18,%rbp
  47f323:	cmp    0x88(%rsp),%rbp
  47f32b:	je     47f4b6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e6>
  47f331:	mov    -0x18(%r13),%r14
  47f335:	cmp    $0xffffffffffffffff,%r14
  47f339:	je     47f432 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x362>
  47f33f:	lea    -0x18(%r13),%rbp
  47f343:	mov    -0x10(%r13),%r15
  47f347:	mov    0x10(%rbp),%rax
  47f34b:	mov    %r12,0x38(%rsp)
  47f350:	mov    %rax,0x90(%rsp)
  47f358:	cmp    %rax,%r12
  47f35b:	jne    47f592 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4c2>
  47f361:	mov    0x20(%rsp),%rax
  47f366:	mov    %rax,0x90(%rsp)
  47f36e:	mov    %fs:0xffffffffffffffc0,%rax
  47f377:	mov    %r12,0x98(%rsp)
  47f37f:	movq   $0x200,0xa0(%rsp)
  47f38b:	mov    %r15,0xa8(%rsp)
  47f393:	mov    %r12,0xb0(%rsp)
  47f39b:	movq   $0x200,0xb8(%rsp)
  47f3a7:	test   %rax,%rax
  47f3aa:	je     47f3b4 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x2e4>
  47f3ac:	add    $0x110,%rax
  47f3b2:	jmp    47f3b9 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x2e9>
  47f3b4:	call   1052d0 <rayon_core::registry::global_registry>
  47f3b9:	mov    (%rax),%rax
  47f3bc:	mov    0x208(%rax),%rdx
  47f3c3:	mov    0xe0(%rsp),%rdi
  47f3cb:	mov    $0x1,%ecx
  47f3d0:	lea    0x90(%rsp),%r8
  47f3d8:	lea    0xf(%rsp),%r9
  47f3dd:	xor    %esi,%esi
  47f3df:	call   250660 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::iter::zip::ZipProducer<rayon::slice::chunks::ChunksMutProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::slice::chunks::ChunksProducer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>, rayon::iter::for_each::ForEachConsumer<zopro_core::msm_affine::merge_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>
  47f3e4:	test   %r14,%r14
  47f3e7:	je     47f31b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x24b>
  47f3ed:	cmp    $0x3ff,%r14
  47f3f4:	jbe    47f312 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x242>
  47f3fa:	shl    $0x7,%r14
  47f3fe:	movabs $0x7ffffffffffffff9,%rax
  47f408:	mov    %r14,%rsi
  47f40b:	or     $0x1,%rsi
  47f40f:	cmp    %rax,%rsi
  47f412:	jae    47f60c <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x53c>
  47f418:	cmpb   $0x1,(%r15,%r14,1)
  47f41d:	jne    47f42d <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x35d>
  47f41f:	mov    %r15,%rdi
  47f422:	call   *0x72db0(%rip)        # 4f21d8 <munmap@GLIBC_2.2.5>
  47f428:	jmp    47f31b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x24b>
  47f42d:	jmp    47f312 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x242>
  47f432:	mov    0x88(%rsp),%rdx
  47f43a:	movabs $0xaaaaaaaaaaaaaaab,%rax
  47f444:	sub    %r13,%rdx
  47f447:	mulx   %rax,%r14,%r14
  47f44c:	mov    %r13,0x58(%rsp)
  47f451:	je     47f4b6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e6>
  47f453:	mov    0x72d7e(%rip),%r15        # 4f21d8 <munmap@GLIBC_2.2.5>
  47f45a:	mov    0x72d7f(%rip),%rbp        # 4f21e0 <free@GLIBC_2.2.5>
  47f461:	shr    $0x4,%r14
  47f465:	add    $0x8,%r13
  47f469:	jmp    47f476 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3a6>
  47f46b:	call   *%rbp
  47f46d:	add    $0x18,%r13
  47f471:	dec    %r14
  47f474:	je     47f4b6 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e6>
  47f476:	mov    -0x8(%r13),%rax
  47f47a:	test   %rax,%rax
  47f47d:	je     47f46d <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39d>
  47f47f:	mov    0x0(%r13),%rdi
  47f483:	cmp    $0x3ff,%rax
  47f489:	jbe    47f46b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39b>
  47f48b:	shl    $0x7,%rax
  47f48f:	movabs $0x7ffffffffffffff9,%rcx
  47f499:	mov    %rax,%rsi
  47f49c:	or     $0x1,%rsi
  47f4a0:	cmp    %rcx,%rsi
  47f4a3:	jae    47f607 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x537>
  47f4a9:	cmpb   $0x1,(%rdi,%rax,1)
  47f4ad:	jne    47f4b4 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x3e4>
  47f4af:	call   *%r15
  47f4b2:	jmp    47f46d <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39d>
  47f4b4:	jmp    47f46b <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x39b>
  47f4b6:	cmpq   $0x0,0x30(%rsp)
  47f4bc:	je     47f515 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x445>
  47f4be:	cmpq   $0x1555,0x30(%rsp)
  47f4c7:	jbe    47f508 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x438>
  47f4c9:	mov    0x30(%rsp),%rax
  47f4ce:	movabs $0x7ffffffffffffff9,%rcx
  47f4d8:	shl    $0x3,%rax
  47f4dc:	lea    (%rax,%rax,2),%rax
  47f4e0:	mov    %rax,%rsi
  47f4e3:	or     $0x1,%rsi
  47f4e7:	cmp    %rcx,%rsi
  47f4ea:	jae    47f611 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x541>
  47f4f0:	mov    0x10(%rsp),%rcx
  47f4f5:	cmpb   $0x1,(%rcx,%rax,1)
  47f4f9:	jne    47f50a <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x43a>
  47f4fb:	mov    0x10(%rsp),%rdi
  47f500:	call   *0x72cd2(%rip)        # 4f21d8 <munmap@GLIBC_2.2.5>
  47f506:	jmp    47f515 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x445>
  47f508:	jmp    47f50a <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x43a>
  47f50a:	mov    0x10(%rsp),%rdi
  47f50f:	call   *0x72ccb(%rip)        # 4f21e0 <free@GLIBC_2.2.5>
  47f515:	mov    0x20(%rsp),%rsi
  47f51a:	mov    %rbx,%rdi
  47f51d:	mov    %r12,%rdx
  47f520:	call   247c30 <zopro_core::msm::weighted_affine::<zopro_bn254::curve::G2Config>>
  47f525:	cmpq   $0x0,0x18(%rsp)
  47f52b:	je     47f580 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4b0>
  47f52d:	cmpq   $0x3ff,0x18(%rsp)
  47f536:	jbe    47f573 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4a3>
  47f538:	mov    0x18(%rsp),%rsi
  47f53d:	movabs $0x7ffffffffffffff9,%rax
  47f547:	shl    $0x7,%rsi
  47f54b:	mov    %rsi,%rcx
  47f54e:	or     $0x1,%rsi
  47f552:	cmp    %rax,%rsi
  47f555:	jae    47f616 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x546>
  47f55b:	mov    0x20(%rsp),%rax
  47f560:	cmpb   $0x1,(%rax,%rcx,1)
  47f564:	jne    47f575 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4a5>
  47f566:	mov    0x20(%rsp),%rdi
  47f56b:	call   *0x72c67(%rip)        # 4f21d8 <munmap@GLIBC_2.2.5>
  47f571:	jmp    47f580 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4b0>
  47f573:	jmp    47f575 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x4a5>
  47f575:	mov    0x20(%rsp),%rdi
  47f57a:	call   *0x72c60(%rip)        # 4f21e0 <free@GLIBC_2.2.5>
  47f580:	add    $0xf8,%rsp
  47f587:	pop    %rbx
  47f588:	pop    %r12
  47f58a:	pop    %r13
  47f58c:	pop    %r14
  47f58e:	pop    %r15
  47f590:	pop    %rbp
  47f591:	ret
  47f592:	mov    %r13,0x58(%rsp)
  47f597:	lea    0x6c94a(%rip),%r8        # 4ebee8 <__frame_dummy_init_array_entry+0x2a38>
  47f59e:	lea    0x38(%rsp),%rdi
  47f5a3:	lea    0x90(%rsp),%rsi
  47f5ab:	xor    %edx,%edx
  47f5ad:	call   f0902 <core::panicking::assert_failed::<usize, usize>>
  47f5b2:	jmp    47f605 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x535>
  47f5b4:	lea    0xf0(%rsp),%rax
  47f5bc:	lea    -0x38bac3(%rip),%rcx        # f3b00 <<u64 as core::fmt::Display>::fmt>
  47f5c3:	mov    %rax,0x90(%rsp)
  47f5cb:	mov    %rcx,0x98(%rsp)
  47f5d3:	mov    %r12,0xa0(%rsp)
  47f5db:	mov    %rcx,0xa8(%rsp)
  47f5e3:	lea    -0x4687eb(%rip),%rdi        # 16dff <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x533f>
  47f5ea:	lea    0x6e177(%rip),%rdx        # 4ed768 <__frame_dummy_init_array_entry+0x42b8>
  47f5f1:	lea    0x90(%rsp),%rsi
  47f5f9:	call   ef4b0 <core::panicking::panic_fmt>
  47f5fe:	jmp    47f605 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x535>
  47f600:	call   eec57 <<alloc::vec::Vec<_, _>>::swap_remove::assert_failed>
  47f605:	ud2
  47f607:	call   111210 <std::process::abort>
  47f60c:	call   111210 <std::process::abort>
  47f611:	call   111210 <std::process::abort>
  47f616:	call   111210 <std::process::abort>
  47f61b:	mov    %rax,%rbx
  47f61e:	jmp    47f689 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x5b9>
  47f620:	jmp    47f622 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x552>
  47f622:	mov    %rax,%rbx
  47f625:	mov    %r13,0x58(%rsp)
  47f62a:	jmp    47f666 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x596>
  47f62c:	mov    %rax,%rbx
  47f62f:	lea    0x70(%rsp),%rdi
  47f634:	call   2502f0 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f639:	mov    %rbx,%rdi
  47f63c:	call   4e8460 <_Unwind_Resume@plt>
  47f641:	mov    0x70(%rsp),%rdi
  47f646:	mov    %rax,%rbx
  47f649:	mov    %r14,%rsi
  47f64c:	call   2946b0 <core::ptr::drop_glue::<rayon::iter::collect::consumer::CollectResult<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f651:	lea    0x38(%rsp),%rdi
  47f656:	call   2502f0 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f65b:	mov    %rbx,%rdi
  47f65e:	call   4e8460 <_Unwind_Resume@plt>
  47f663:	mov    %rax,%rbx
  47f666:	test   %r14,%r14
  47f669:	je     47f67f <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x5af>
  47f66b:	shl    $0x7,%r14
  47f66f:	mov    $0x8,%edx
  47f674:	mov    %r15,%rdi
  47f677:	mov    %r14,%rsi
  47f67a:	call   e99d0 <__rustc::__rust_dealloc>
  47f67f:	lea    0x50(%rsp),%rdi
  47f684:	call   250560 <core::ptr::drop_glue::<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f689:	cmpq   $0x0,0x18(%rsp)
  47f68f:	je     47f6a9 <<&zopro_core::msm_affine::g2_borrowed::gls4::run_prepared::{closure#0} as core::ops::function::FnMut<(usize,)>>::call_mut+0x5d9>
  47f691:	mov    0x18(%rsp),%rsi
  47f696:	shl    $0x7,%rsi
  47f69a:	mov    0x20(%rsp),%rdi
  47f69f:	mov    $0x8,%edx
  47f6a4:	call   e99d0 <__rustc::__rust_dealloc>
  47f6a9:	mov    %rbx,%rdi
  47f6ac:	call   4e8460 <_Unwind_Resume@plt>
  47f6b1:	mov    %rax,%rbx
  47f6b4:	lea    0x38(%rsp),%rdi
  47f6b9:	call   2502f0 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>>>>
  47f6be:	mov    %rbx,%rdi
  47f6c1:	call   4e8460 <_Unwind_Resume@plt>
