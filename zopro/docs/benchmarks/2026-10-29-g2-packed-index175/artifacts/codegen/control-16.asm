
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/control-sdk:     file format elf64-x86-64


Disassembly of section .text:

00000000001c2ed0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>>:
  1c2ed0:	push   %rbp
  1c2ed1:	push   %r15
  1c2ed3:	push   %r14
  1c2ed5:	push   %r13
  1c2ed7:	push   %r12
  1c2ed9:	push   %rbx
  1c2eda:	sub    $0x138,%rsp
  1c2ee1:	mov    0x170(%rsp),%rbp
  1c2ee9:	movq   $0x0,0x50(%rsp)
  1c2ef2:	movq   $0x8,0x58(%rsp)
  1c2efb:	mov    %r8,0xa0(%rsp)
  1c2f03:	mov    %rcx,0x90(%rsp)
  1c2f0b:	mov    %rsi,0x98(%rsp)
  1c2f13:	movq   $0x0,0x60(%rsp)
  1c2f1c:	mov    %rdi,0x88(%rsp)
  1c2f24:	mov    %rdx,0x28(%rsp)
  1c2f29:	test   %rbp,%rbp
  1c2f2c:	je     1c307c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x1ac>
  1c2f32:	lea    0x0(,%rbp,8),%r12
  1c2f3a:	mov    %r9,%r13
  1c2f3d:	mov    $0x8,%edi
  1c2f42:	mov    %r12,%rsi
  1c2f45:	xor    %edx,%edx
  1c2f47:	call   4b8940 <<zopro_core::alloc::linux::Allocator>::allocate>
  1c2f4c:	test   %rax,%rax
  1c2f4f:	je     1c38c4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x9f4>
  1c2f55:	mov    %rbp,0x8(%rsp)
  1c2f5a:	mov    %rax,0x10(%rsp)
  1c2f5f:	movq   $0x0,(%rax)
  1c2f66:	movq   $0x1,0x18(%rsp)
  1c2f6f:	cmp    $0x1,%rbp
  1c2f73:	je     1c30b6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x1e6>
  1c2f79:	mov    0x0(%r13),%r14
  1c2f7d:	mov    $0x8,%r12d
  1c2f83:	mov    $0x1,%ebx
  1c2f88:	xor    %r15d,%r15d
  1c2f8b:	jmp    1c2fb7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xe7>
  1c2f8d:	nopl   (%rax)
  1c2f90:	mov    0x10(%rsp),%rax
  1c2f95:	mov    (%rsp),%r14
  1c2f99:	add    $0x8,%r13
  1c2f9d:	mov    %r15,(%rax,%rbx,8)
  1c2fa1:	inc    %rbx
  1c2fa4:	mov    %rbx,0x18(%rsp)
  1c2fa9:	cmp    $0x3,%rbp
  1c2fad:	lea    -0x1(%rbp),%rbp
  1c2fb1:	jb     1c30e8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x218>
  1c2fb7:	mov    0x8(%r13),%rax
  1c2fbb:	mov    %rax,(%rsp)
  1c2fbf:	sub    %r14,%rax
  1c2fc2:	jbe    1c3062 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x192>
  1c2fc8:	mov    %rax,%rbx
  1c2fcb:	and    $0x7ff,%eax
  1c2fd0:	shr    $0xb,%rbx
  1c2fd4:	mov    %rbp,0x30(%rsp)
  1c2fd9:	mov    %r15,%rbp
  1c2fdc:	mov    %r13,0x20(%rsp)
  1c2fe1:	cmp    $0x1,%rax
  1c2fe5:	mov    %r12,%rax
  1c2fe8:	sbb    $0xffffffffffffffff,%rbx
  1c2fec:	shl    $0x4,%rbp
  1c2ff0:	or     $0x8,%rbp
  1c2ff4:	jmp    1c3020 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x150>
  1c2ff6:	cs nopw 0x0(%rax,%rax,1)
  1c3000:	mov    %r14,-0x8(%rax,%rbp,1)
  1c3005:	mov    %r13,(%rax,%rbp,1)
  1c3009:	inc    %r15
  1c300c:	dec    %rbx
  1c300f:	add    $0x10,%rbp
  1c3013:	mov    %r12,%r14
  1c3016:	mov    %r15,0x60(%rsp)
  1c301b:	test   %rbx,%rbx
  1c301e:	je     1c3050 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x180>
  1c3020:	mov    (%rsp),%rcx
  1c3024:	lea    0x800(%r14),%r12
  1c302b:	mov    %r12,%r13
  1c302e:	cmp    %r12,%rcx
  1c3031:	cmovb  %rcx,%r13
  1c3035:	cmp    0x50(%rsp),%r15
  1c303a:	jne    1c3000 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x130>
  1c303c:	lea    0x50(%rsp),%rdi
  1c3041:	call   2b46f0 <<alloc::raw_vec::RawVec<zopro_core::preparation::Reservation>>::grow_one>
  1c3046:	mov    0x58(%rsp),%rax
  1c304b:	jmp    1c3000 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x130>
  1c304d:	nopl   (%rax)
  1c3050:	mov    0x18(%rsp),%rbx
  1c3055:	mov    0x20(%rsp),%r13
  1c305a:	mov    0x30(%rsp),%rbp
  1c305f:	mov    %rax,%r12
  1c3062:	cmp    0x8(%rsp),%rbx
  1c3067:	jne    1c2f90 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc0>
  1c306d:	lea    0x8(%rsp),%rdi
  1c3072:	call   f3f90 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  1c3077:	jmp    1c2f90 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc0>
  1c307c:	movq   $0x0,0x8(%rsp)
  1c3085:	movq   $0x8,0x10(%rsp)
  1c308e:	movq   $0x0,0x18(%rsp)
  1c3097:	lea    0x8(%rsp),%rdi
  1c309c:	call   f3f90 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  1c30a1:	mov    0x10(%rsp),%rax
  1c30a6:	movq   $0x0,(%rax)
  1c30ad:	movq   $0x1,0x18(%rsp)
  1c30b6:	movq   $0x0,0x38(%rsp)
  1c30bf:	movq   $0x8,0x40(%rsp)
  1c30c8:	movq   $0x0,0x48(%rsp)
  1c30d1:	movq   $0x0,0x80(%rsp)
  1c30dd:	mov    $0x8,%r12d
  1c30e3:	xor    %r13d,%r13d
  1c30e6:	jmp    1c311e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x24e>
  1c30e8:	mov    0x50(%rsp),%r13
  1c30ed:	mov    0x58(%rsp),%r12
  1c30f2:	movq   $0x0,0x38(%rsp)
  1c30fb:	movq   $0x8,0x40(%rsp)
  1c3104:	movq   $0x0,0x48(%rsp)
  1c310d:	mov    %r15,0x80(%rsp)
  1c3115:	test   %r15,%r15
  1c3118:	jne    1c3806 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x936>
  1c311e:	mov    $0x8,%r14d
  1c3124:	mov    $0x1,%bl
  1c3126:	xor    %r15d,%r15d
  1c3129:	xor    %ebp,%ebp
  1c312b:	xor    %eax,%eax
  1c312d:	mov    0x90(%rsp),%rdx
  1c3135:	mov    0xa0(%rsp),%rsi
  1c313d:	mov    %r13,0xa8(%rsp)
  1c3145:	mov    %r12,0xb0(%rsp)
  1c314d:	mov    %r15,0xb8(%rsp)
  1c3155:	mov    0x98(%rsp),%rdi
  1c315d:	mov    %rax,0x30(%rsp)
  1c3162:	shl    $0x6,%rax
  1c3166:	lea    0xc0(%rsp),%rcx
  1c316e:	mov    %r15,0x118(%rsp)
  1c3176:	mov    %r13,0x68(%rsp)
  1c317b:	mov    %r12,0x70(%rsp)
  1c3180:	movq   $0x0,0x78(%rsp)
  1c3189:	add    %r14,%rax
  1c318c:	mov    %rcx,0x100(%rsp)
  1c3194:	mov    %rax,0x108(%rsp)
  1c319c:	lea    0x68(%rsp),%rax
  1c31a1:	mov    %r15,0x110(%rsp)
  1c31a9:	mov    %rax,0xe0(%rsp)
  1c31b1:	movq   $0x0,0xe8(%rsp)
  1c31bd:	mov    %r15,0xf0(%rsp)
  1c31c5:	mov    %r15,0xf8(%rsp)
  1c31cd:	mov    %rdx,0xc0(%rsp)
  1c31d5:	mov    %rsi,0xc8(%rsp)
  1c31dd:	mov    0x28(%rsp),%rsi
  1c31e2:	mov    %rdi,0xd0(%rsp)
  1c31ea:	mov    %rsi,0xd8(%rsp)
  1c31f2:	cmp    %r15,%r13
  1c31f5:	jb     1c3856 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x986>
  1c31fb:	mov    %fs:0xffffffffffffffc0,%rax
  1c3204:	test   %rax,%rax
  1c3207:	je     1c3211 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x341>
  1c3209:	add    $0x110,%rax
  1c320f:	jmp    1c3216 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x346>
  1c3211:	call   104130 <rayon_core::registry::global_registry>
  1c3216:	mov    (%rax),%rax
  1c3219:	mov    0x208(%rax),%rcx
  1c3220:	lea    0x100(%rsp),%rax
  1c3228:	xor    %edx,%edx
  1c322a:	lea    0x120(%rsp),%rdi
  1c3232:	mov    $0x1,%r8d
  1c3238:	mov    %r15,%rsi
  1c323b:	mov    %r12,%r9
  1c323e:	push   %rax
  1c323f:	push   %r15
  1c3241:	call   1f94a0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G1Config>::{closure#0}>>>
  1c3246:	add    $0x10,%rsp
  1c324a:	cmp    %r15,0x78(%rsp)
  1c324f:	sete   %al
  1c3252:	or     %al,%bl
  1c3254:	je     1c325f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x38f>
  1c3256:	movq   $0x0,0x78(%rsp)
  1c325f:	mov    0x68(%rsp),%rax
  1c3264:	mov    %rbp,0x20(%rsp)
  1c3269:	test   %rax,%rax
  1c326c:	je     1c32af <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3df>
  1c326e:	mov    0x70(%rsp),%rdi
  1c3273:	cmp    $0x1fff,%rax
  1c3279:	jbe    1c32a7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3d7>
  1c327b:	shl    $0x4,%rax
  1c327f:	movabs $0x7ffffffffffffff9,%rcx
  1c3289:	mov    %rax,%rsi
  1c328c:	or     $0x1,%rsi
  1c3290:	cmp    %rcx,%rsi
  1c3293:	jae    1c38d1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa01>
  1c3299:	cmpb   $0x1,(%rdi,%rax,1)
  1c329d:	jne    1c32a9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3d9>
  1c329f:	call   *0x32405b(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1c32a5:	jmp    1c32af <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3df>
  1c32a7:	jmp    1c32a9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x3d9>
  1c32a9:	call   *0x324059(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c32af:	mov    0x130(%rsp),%rax
  1c32b7:	mov    %rax,0xe0(%rsp)
  1c32bf:	cmp    %r15,%rax
  1c32c2:	jne    1c3870 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x9a0>
  1c32c8:	mov    0x18(%rsp),%r12
  1c32cd:	mov    %r14,(%rsp)
  1c32d1:	xor    %r14d,%r14d
  1c32d4:	movabs $0x7ffffffffffffff9,%rcx
  1c32de:	dec    %rcx
  1c32e1:	lea    -0x1(%r12),%r10
  1c32e6:	cmp    $0x2,%r12
  1c32ea:	mov    %r10,%r8
  1c32ed:	cmovb  %r14,%r8
  1c32f1:	lea    0x0(,%r8,8),%rbx
  1c32f9:	mov    %r8,%rax
  1c32fc:	shr    $0x3d,%rax
  1c3300:	setne  %al
  1c3303:	cmp    %rcx,%rbx
  1c3306:	seta   %cl
  1c3309:	or     %al,%cl
  1c330b:	je     1c331d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x44d>
  1c330d:	mov    %r14,%rdi
  1c3310:	mov    %rbx,%rsi
  1c3313:	call   ed521 <alloc::raw_vec::handle_error>
  1c3318:	jmp    1c38e3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa13>
  1c331d:	mov    0x10(%rsp),%rbp
  1c3322:	test   %rbx,%rbx
  1c3325:	je     1c3351 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x481>
  1c3327:	mov    $0x8,%edi
  1c332c:	mov    $0x8,%r14d
  1c3332:	mov    %rbx,%rsi
  1c3335:	xor    %edx,%edx
  1c3337:	mov    %r8,%r13
  1c333a:	call   4b8940 <<zopro_core::alloc::linux::Allocator>::allocate>
  1c333f:	lea    -0x1(%r12),%r10
  1c3344:	mov    %r13,%rcx
  1c3347:	mov    %rax,%r13
  1c334a:	test   %rax,%rax
  1c334d:	jne    1c3359 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x489>
  1c334f:	jmp    1c330d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x43d>
  1c3351:	mov    $0x8,%r13d
  1c3357:	xor    %ecx,%ecx
  1c3359:	mov    0x30(%rsp),%rdx
  1c335e:	mov    %rcx,0x28(%rsp)
  1c3363:	add    %r15,%rdx
  1c3366:	cmp    $0x2,%r12
  1c336a:	jae    1c33b9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x4e9>
  1c336c:	xor    %r10d,%r10d
  1c336f:	mov    %edx,%ebx
  1c3371:	lea    0x0(,%rbx,4),%r14
  1c3379:	test   %rbx,%rbx
  1c337c:	je     1c368c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x7bc>
  1c3382:	mov    %r10,%r12
  1c3385:	mov    %rdx,%r15
  1c3388:	mov    $0x4,%edi
  1c338d:	mov    %r14,%rsi
  1c3390:	xor    %edx,%edx
  1c3392:	vzeroupper
  1c3395:	call   4b8940 <<zopro_core::alloc::linux::Allocator>::allocate>
  1c339a:	test   %rax,%rax
  1c339d:	je     1c38d6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa06>
  1c33a3:	mov    %rax,%rbp
  1c33a6:	mov    %r12,%r10
  1c33a9:	cmp    $0x8,%ebx
  1c33ac:	jae    1c34d1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x601>
  1c33b2:	xor    %eax,%eax
  1c33b4:	jmp    1c35b0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6e0>
  1c33b9:	mov    0x0(%rbp),%rcx
  1c33bd:	mov    %rdx,%r9
  1c33c0:	cmp    $0x21,%r12
  1c33c4:	jb     1c33dc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x50c>
  1c33c6:	mov    %r13,%rax
  1c33c9:	sub    %rbp,%rax
  1c33cc:	add    $0xfffffffffffffff8,%rax
  1c33d0:	cmp    $0x100,%rax
  1c33d6:	jae    1c35c4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6f4>
  1c33dc:	xor    %eax,%eax
  1c33de:	mov    %r12,%rdx
  1c33e1:	sub    %rax,%rdx
  1c33e4:	mov    %r12d,%esi
  1c33e7:	and    $0x7,%esi
  1c33ea:	add    $0xfffffffffffffffe,%rdx
  1c33ee:	cmp    $0x1,%esi
  1c33f1:	je     1c341e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x54e>
  1c33f3:	lea    -0x1(%r12),%esi
  1c33f8:	and    $0x7,%esi
  1c33fb:	nopl   0x0(%rax,%rax,1)
  1c3400:	mov    %rcx,%rdi
  1c3403:	mov    0x8(%rbp),%rcx
  1c3407:	add    $0x8,%rbp
  1c340b:	mov    %rcx,%r8
  1c340e:	sub    %rdi,%r8
  1c3411:	mov    %r8,0x0(%r13,%rax,8)
  1c3416:	inc    %rax
  1c3419:	dec    %rsi
  1c341c:	jne    1c3400 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x530>
  1c341e:	cmp    $0x7,%rdx
  1c3422:	jb     1c3676 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x7a6>
  1c3428:	lea    0x38(%r13,%rax,8),%rdx
  1c342d:	not    %rax
  1c3430:	xor    %esi,%esi
  1c3432:	add    %r12,%rax
  1c3435:	data16 cs nopw 0x0(%rax,%rax,1)
  1c3440:	mov    0x8(%rbp,%rsi,8),%rdi
  1c3445:	mov    %rdi,%r8
  1c3448:	sub    %rcx,%r8
  1c344b:	mov    %r8,-0x38(%rdx,%rsi,8)
  1c3450:	mov    0x10(%rbp,%rsi,8),%rcx
  1c3455:	mov    %rcx,%r8
  1c3458:	sub    %rdi,%r8
  1c345b:	mov    %r8,-0x30(%rdx,%rsi,8)
  1c3460:	mov    0x18(%rbp,%rsi,8),%rdi
  1c3465:	mov    %rdi,%r8
  1c3468:	sub    %rcx,%r8
  1c346b:	mov    %r8,-0x28(%rdx,%rsi,8)
  1c3470:	mov    0x20(%rbp,%rsi,8),%rcx
  1c3475:	mov    %rcx,%r8
  1c3478:	sub    %rdi,%r8
  1c347b:	mov    %r8,-0x20(%rdx,%rsi,8)
  1c3480:	mov    0x28(%rbp,%rsi,8),%rdi
  1c3485:	mov    %rdi,%r8
  1c3488:	sub    %rcx,%r8
  1c348b:	mov    %r8,-0x18(%rdx,%rsi,8)
  1c3490:	mov    0x30(%rbp,%rsi,8),%rcx
  1c3495:	mov    %rcx,%r8
  1c3498:	sub    %rdi,%r8
  1c349b:	mov    %r8,-0x10(%rdx,%rsi,8)
  1c34a0:	mov    0x38(%rbp,%rsi,8),%rdi
  1c34a5:	mov    %rdi,%r8
  1c34a8:	sub    %rcx,%r8
  1c34ab:	mov    %r8,-0x8(%rdx,%rsi,8)
  1c34b0:	mov    0x40(%rbp,%rsi,8),%rcx
  1c34b5:	mov    %rcx,%r8
  1c34b8:	sub    %rdi,%r8
  1c34bb:	mov    %r8,(%rdx,%rsi,8)
  1c34bf:	add    $0x8,%rsi
  1c34c3:	cmp    %rsi,%rax
  1c34c6:	jne    1c3440 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x570>
  1c34cc:	jmp    1c3676 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x7a6>
  1c34d1:	cmp    $0x40,%ebx
  1c34d4:	jae    1c34dd <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x60d>
  1c34d6:	xor    %eax,%eax
  1c34d8:	jmp    1c356b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x69b>
  1c34dd:	vmovdqa64 -0x1b72e7(%rip),%zmm0        # c200 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x7c0>
  1c34e7:	vpbroadcastd -0x1b7771(%rip),%zmm1        # bd80 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x340>
  1c34f1:	vpbroadcastd -0x1b7757(%rip),%zmm2        # bda4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x364>
  1c34fb:	vpbroadcastd -0x1b774d(%rip),%zmm3        # bdb8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x378>
  1c3505:	vpbroadcastd -0x1b774b(%rip),%zmm4        # bdc4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x384>
  1c350f:	mov    %r15d,%eax
  1c3512:	and    $0xffffffc0,%eax
  1c3515:	xor    %ecx,%ecx
  1c3517:	nopw   0x0(%rax,%rax,1)
  1c3520:	vpaddd %zmm1,%zmm0,%zmm5
  1c3526:	vpaddd %zmm2,%zmm0,%zmm6
  1c352c:	vpaddd %zmm3,%zmm0,%zmm7
  1c3532:	vmovdqu64 %zmm0,0x0(%rbp,%rcx,4)
  1c353a:	vpaddd %zmm4,%zmm0,%zmm0
  1c3540:	vmovdqu64 %zmm5,0x40(%rbp,%rcx,4)
  1c3548:	vmovdqu64 %zmm6,0x80(%rbp,%rcx,4)
  1c3550:	vmovdqu64 %zmm7,0xc0(%rbp,%rcx,4)
  1c3558:	add    $0x40,%rcx
  1c355c:	cmp    %rcx,%rax
  1c355f:	jne    1c3520 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x650>
  1c3561:	cmp    %eax,%ebx
  1c3563:	je     1c35bc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6ec>
  1c3565:	test   $0x38,%r15b
  1c3569:	je     1c35b0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6e0>
  1c356b:	mov    %rax,%rcx
  1c356e:	vpbroadcastd %ecx,%ymm0
  1c3574:	vpor   -0x1b1dbc(%rip),%ymm0,%ymm0        # 117c0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d80>
  1c357c:	vpbroadcastd -0x1b77dd(%rip),%ymm1        # bda8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x368>
  1c3585:	mov    %r15d,%eax
  1c3588:	and    $0xfffffff8,%eax
  1c358b:	nopl   0x0(%rax,%rax,1)
  1c3590:	vmovdqu %ymm0,0x0(%rbp,%rcx,4)
  1c3596:	add    $0x8,%rcx
  1c359a:	vpaddd %ymm1,%ymm0,%ymm0
  1c359e:	cmp    %rcx,%rax
  1c35a1:	jne    1c3590 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6c0>
  1c35a3:	cmp    %eax,%ebx
  1c35a5:	je     1c35bc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6ec>
  1c35a7:	nopw   0x0(%rax,%rax,1)
  1c35b0:	mov    %eax,0x0(%rbp,%rax,4)
  1c35b4:	inc    %rax
  1c35b7:	cmp    %rax,%rbx
  1c35ba:	jne    1c35b0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x6e0>
  1c35bc:	mov    %r15,%rdx
  1c35bf:	jmp    1c3691 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x7c1>
  1c35c4:	mov    %r10,%rax
  1c35c7:	and    $0xffffffffffffffe0,%rax
  1c35cb:	vpbroadcastq %rcx,%zmm0
  1c35d1:	xor    %ecx,%ecx
  1c35d3:	lea    0x0(%rbp,%rax,8),%rdx
  1c35d8:	nopl   0x0(%rax,%rax,1)
  1c35e0:	vmovdqu64 0x8(%rbp,%rcx,8),%zmm1
  1c35eb:	vmovdqu64 0x48(%rbp,%rcx,8),%zmm2
  1c35f6:	vmovdqu64 0x88(%rbp,%rcx,8),%zmm3
  1c3601:	valignq $0x7,%zmm0,%zmm1,%zmm4
  1c3608:	vmovdqu64 0xc8(%rbp,%rcx,8),%zmm0
  1c3613:	valignq $0x7,%zmm1,%zmm2,%zmm5
  1c361a:	valignq $0x7,%zmm2,%zmm3,%zmm6
  1c3621:	vpsubq %zmm4,%zmm1,%zmm1
  1c3627:	vpsubq %zmm5,%zmm2,%zmm2
  1c362d:	vmovdqu64 %zmm1,0x0(%r13,%rcx,8)
  1c3635:	vmovdqu64 %zmm2,0x40(%r13,%rcx,8)
  1c363d:	valignq $0x7,%zmm3,%zmm0,%zmm7
  1c3644:	vpsubq %zmm6,%zmm3,%zmm3
  1c364a:	vmovdqu64 %zmm3,0x80(%r13,%rcx,8)
  1c3652:	vpsubq %zmm7,%zmm0,%zmm4
  1c3658:	vmovdqu64 %zmm4,0xc0(%r13,%rcx,8)
  1c3660:	add    $0x20,%rcx
  1c3664:	cmp    %rcx,%rax
  1c3667:	jne    1c35e0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x710>
  1c366d:	cmp    %rax,%r10
  1c3670:	jne    1c37f1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x921>
  1c3676:	mov    %r9,%rdx
  1c3679:	mov    %edx,%ebx
  1c367b:	lea    0x0(,%rbx,4),%r14
  1c3683:	test   %rbx,%rbx
  1c3686:	jne    1c3382 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x4b2>
  1c368c:	mov    $0x4,%ebp
  1c3691:	mov    0x10(%rsp),%r9
  1c3696:	sub    $0x8,%rsp
  1c369a:	mov    0x90(%rsp),%rdi
  1c36a2:	mov    0x8(%rsp),%rsi
  1c36a7:	mov    %rbp,%rcx
  1c36aa:	mov    %rbx,%r8
  1c36ad:	push   %r10
  1c36af:	push   %r13
  1c36b1:	push   0x30(%rsp)
  1c36b5:	vzeroupper
  1c36b8:	call   1f9fa0 <zopro_core::msm::group_affine::<zopro_bn254::curve::G1Config>>
  1c36bd:	add    $0x20,%rsp
  1c36c1:	test   %rbx,%rbx
  1c36c4:	je     1c36f3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x823>
  1c36c6:	cmp    $0x7fff,%ebx
  1c36cc:	jbe    1c36e8 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x818>
  1c36ce:	cmpb   $0x1,0x0(%rbp,%r14,1)
  1c36d4:	jne    1c36ea <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x81a>
  1c36d6:	or     $0x1,%r14
  1c36da:	mov    %rbp,%rdi
  1c36dd:	mov    %r14,%rsi
  1c36e0:	call   *0x323c1a(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1c36e6:	jmp    1c36f3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x823>
  1c36e8:	jmp    1c36ea <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x81a>
  1c36ea:	mov    %rbp,%rdi
  1c36ed:	call   *0x323c15(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c36f3:	mov    0x28(%rsp),%rcx
  1c36f8:	mov    (%rsp),%rbx
  1c36fc:	mov    0x20(%rsp),%r14
  1c3701:	test   %rcx,%rcx
  1c3704:	je     1c374b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x87b>
  1c3706:	cmp    $0x3fff,%rcx
  1c370d:	jbe    1c3740 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x870>
  1c370f:	shl    $0x3,%rcx
  1c3713:	movabs $0x7ffffffffffffff9,%rax
  1c371d:	mov    %rcx,%rsi
  1c3720:	or     $0x1,%rsi
  1c3724:	cmp    %rax,%rsi
  1c3727:	jae    1c38e5 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa15>
  1c372d:	cmpb   $0x1,0x0(%r13,%rcx,1)
  1c3733:	jne    1c3742 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x872>
  1c3735:	mov    %r13,%rdi
  1c3738:	call   *0x323bc2(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1c373e:	jmp    1c374b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x87b>
  1c3740:	jmp    1c3742 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x872>
  1c3742:	mov    %r13,%rdi
  1c3745:	call   *0x323bbd(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c374b:	test   %r14,%r14
  1c374e:	je     1c3794 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8c4>
  1c3750:	cmp    $0x7ff,%r14
  1c3757:	jbe    1c3789 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8b9>
  1c3759:	shl    $0x6,%r14
  1c375d:	movabs $0x7ffffffffffffff9,%rax
  1c3767:	mov    %r14,%rsi
  1c376a:	or     $0x1,%rsi
  1c376e:	cmp    %rax,%rsi
  1c3771:	jae    1c38ea <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa1a>
  1c3777:	cmpb   $0x1,(%rbx,%r14,1)
  1c377c:	jne    1c378b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8bb>
  1c377e:	mov    %rbx,%rdi
  1c3781:	call   *0x323b79(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1c3787:	jmp    1c3794 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8c4>
  1c3789:	jmp    1c378b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x8bb>
  1c378b:	mov    %rbx,%rdi
  1c378e:	call   *0x323b74(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c3794:	mov    0x8(%rsp),%rax
  1c3799:	test   %rax,%rax
  1c379c:	je     1c37df <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x90f>
  1c379e:	mov    0x10(%rsp),%rdi
  1c37a3:	cmp    $0x3fff,%rax
  1c37a9:	jbe    1c37d7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x907>
  1c37ab:	shl    $0x3,%rax
  1c37af:	movabs $0x7ffffffffffffff9,%rcx
  1c37b9:	mov    %rax,%rsi
  1c37bc:	or     $0x1,%rsi
  1c37c0:	cmp    %rcx,%rsi
  1c37c3:	jae    1c38ef <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa1f>
  1c37c9:	cmpb   $0x1,(%rdi,%rax,1)
  1c37cd:	jne    1c37d9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x909>
  1c37cf:	call   *0x323b2b(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1c37d5:	jmp    1c37df <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x90f>
  1c37d7:	jmp    1c37d9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x909>
  1c37d9:	call   *0x323b29(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c37df:	add    $0x138,%rsp
  1c37e6:	pop    %rbx
  1c37e7:	pop    %r12
  1c37e9:	pop    %r13
  1c37eb:	pop    %r14
  1c37ed:	pop    %r15
  1c37ef:	pop    %rbp
  1c37f0:	ret
  1c37f1:	vextracti32x4 $0x3,%zmm0,%xmm0
  1c37f8:	mov    %rdx,%rbp
  1c37fb:	vpextrq $0x1,%xmm0,%rcx
  1c3801:	jmp    1c33de <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x50e>
  1c3806:	lea    0x38(%rsp),%rdi
  1c380b:	mov    $0x8,%ecx
  1c3810:	mov    $0x40,%r8d
  1c3816:	xor    %esi,%esi
  1c3818:	mov    %r15,%rdx
  1c381b:	call   1b5e60 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  1c3820:	mov    0x38(%rsp),%rbp
  1c3825:	mov    0x48(%rsp),%rcx
  1c382a:	mov    %rbp,%rax
  1c382d:	sub    %rcx,%rax
  1c3830:	cmp    %r15,%rax
  1c3833:	jae    1c38f4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa24>
  1c3839:	lea    -0x1a3e99(%rip),%rdi        # 1f9a7 <zmij::STATIC_DATA+0x55a7>
  1c3840:	lea    0x320fe9(%rip),%rdx        # 4e4830 <__frame_dummy_init_array_entry+0x6210>
  1c3847:	mov    $0x2f,%esi
  1c384c:	call   f03d0 <core::panicking::panic>
  1c3851:	jmp    1c38e3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa13>
  1c3856:	lea    -0x1a3eb6(%rip),%rdi        # 1f9a7 <zmij::STATIC_DATA+0x55a7>
  1c385d:	lea    0x321464(%rip),%rdx        # 4e4cc8 <__frame_dummy_init_array_entry+0x66a8>
  1c3864:	mov    $0x2f,%esi
  1c3869:	call   f03d0 <core::panicking::panic>
  1c386e:	jmp    1c38e3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa13>
  1c3870:	lea    0x80(%rsp),%rax
  1c3878:	lea    -0xd0f2f(%rip),%r8        # f2950 <<u64 as core::fmt::Display>::fmt>
  1c387f:	lea    0xe0(%rsp),%rcx
  1c3887:	mov    %rax,0xa8(%rsp)
  1c388f:	mov    %r8,0xb0(%rsp)
  1c3897:	mov    %rcx,0xb8(%rsp)
  1c389f:	mov    %r8,0xc0(%rsp)
  1c38a7:	lea    -0x1acb5c(%rip),%rdi        # 16d52 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x52d2>
  1c38ae:	lea    0x31f023(%rip),%rdx        # 4e28d8 <__frame_dummy_init_array_entry+0x42b8>
  1c38b5:	lea    0xa8(%rsp),%rsi
  1c38bd:	call   ee300 <core::panicking::panic_fmt>
  1c38c2:	jmp    1c38e3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa13>
  1c38c4:	mov    $0x8,%edi
  1c38c9:	mov    %r12,%rsi
  1c38cc:	call   ed521 <alloc::raw_vec::handle_error>
  1c38d1:	call   110070 <std::process::abort>
  1c38d6:	mov    $0x4,%edi
  1c38db:	mov    %r14,%rsi
  1c38de:	call   ed521 <alloc::raw_vec::handle_error>
  1c38e3:	ud2
  1c38e5:	call   110070 <std::process::abort>
  1c38ea:	call   110070 <std::process::abort>
  1c38ef:	call   110070 <std::process::abort>
  1c38f4:	mov    0x40(%rsp),%r14
  1c38f9:	xor    %ebx,%ebx
  1c38fb:	mov    %rcx,%rax
  1c38fe:	jmp    1c312d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0x25d>
  1c3903:	jmp    1c3959 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa89>
  1c3905:	mov    %rax,%r15
  1c3908:	jmp    1c3924 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa54>
  1c390a:	jmp    1c394a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa7a>
  1c390c:	mov    %rax,%r15
  1c390f:	test   %rbx,%rbx
  1c3912:	je     1c3924 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa54>
  1c3914:	mov    $0x4,%edx
  1c3919:	mov    %rbp,%rdi
  1c391c:	mov    %r14,%rsi
  1c391f:	call   e8820 <__rustc::__rust_dealloc>
  1c3924:	cmpq   $0x0,0x28(%rsp)
  1c392a:	je     1c39c6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xaf6>
  1c3930:	mov    0x28(%rsp),%rsi
  1c3935:	shl    $0x3,%rsi
  1c3939:	mov    $0x8,%edx
  1c393e:	mov    %r13,%rdi
  1c3941:	call   e8820 <__rustc::__rust_dealloc>
  1c3946:	jmp    1c39c6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xaf6>
  1c3948:	jmp    1c394a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xa7a>
  1c394a:	mov    %rax,%r15
  1c394d:	mov    $0x1,%bl
  1c394f:	jmp    1c3a15 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb45>
  1c3954:	mov    %rax,%r15
  1c3957:	jmp    1c399f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xacf>
  1c3959:	mov    %rax,%r15
  1c395c:	test   %r13,%r13
  1c395f:	je     1c399f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xacf>
  1c3961:	shl    $0x4,%r13
  1c3965:	mov    $0x8,%edx
  1c396a:	mov    %r12,%rdi
  1c396d:	mov    %r13,%rsi
  1c3970:	jmp    1c399a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xaca>
  1c3972:	mov    %rax,%r15
  1c3975:	lea    0xe0(%rsp),%rdi
  1c397d:	call   1f9400 <core::ptr::drop_glue::<rayon::vec::Drain<(usize, usize)>>>
  1c3982:	mov    0x68(%rsp),%rsi
  1c3987:	test   %rsi,%rsi
  1c398a:	je     1c399f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xacf>
  1c398c:	mov    0x70(%rsp),%rdi
  1c3991:	shl    $0x4,%rsi
  1c3995:	mov    $0x8,%edx
  1c399a:	call   e8820 <__rustc::__rust_dealloc>
  1c399f:	mov    0x38(%rsp),%rsi
  1c39a4:	test   %rsi,%rsi
  1c39a7:	je     1c3a13 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb43>
  1c39a9:	mov    0x40(%rsp),%rdi
  1c39ae:	shl    $0x6,%rsi
  1c39b2:	mov    $0x8,%edx
  1c39b7:	call   e8820 <__rustc::__rust_dealloc>
  1c39bc:	jmp    1c3a13 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb43>
  1c39be:	call   f1863 <core::panicking::panic_in_cleanup>
  1c39c3:	mov    %rax,%r15
  1c39c6:	mov    (%rsp),%rdi
  1c39ca:	mov    0x20(%rsp),%rcx
  1c39cf:	test   %rcx,%rcx
  1c39d2:	je     1c3a13 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb43>
  1c39d4:	cmp    $0x7ff,%rcx
  1c39db:	jbe    1c3a0d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb3d>
  1c39dd:	shl    $0x6,%rcx
  1c39e1:	movabs $0x7ffffffffffffff9,%rax
  1c39eb:	mov    %rcx,%rsi
  1c39ee:	or     $0x1,%rsi
  1c39f2:	cmp    %rax,%rsi
  1c39f5:	jae    1c3ab9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xbe9>
  1c39fb:	cmpb   $0x1,(%rdi,%rcx,1)
  1c39ff:	jne    1c3ad1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc01>
  1c3a05:	call   *0x3238f5(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1c3a0b:	jmp    1c3a13 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb43>
  1c3a0d:	call   *0x3238f5(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c3a13:	xor    %ebx,%ebx
  1c3a15:	mov    0x8(%rsp),%rax
  1c3a1a:	test   %rax,%rax
  1c3a1d:	jne    1c3a25 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb55>
  1c3a1f:	test   %bl,%bl
  1c3a21:	jne    1c3a6c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb9c>
  1c3a23:	jmp    1c3a76 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xba6>
  1c3a25:	mov    0x10(%rsp),%rdi
  1c3a2a:	cmp    $0x3fff,%rax
  1c3a30:	jbe    1c3a62 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb92>
  1c3a32:	shl    $0x3,%rax
  1c3a36:	movabs $0x7ffffffffffffff9,%rcx
  1c3a40:	mov    %rax,%rsi
  1c3a43:	or     $0x1,%rsi
  1c3a47:	cmp    %rcx,%rsi
  1c3a4a:	jae    1c3abe <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xbee>
  1c3a4c:	cmpb   $0x1,(%rdi,%rax,1)
  1c3a50:	jne    1c3ad6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc06>
  1c3a56:	call   *0x3238a4(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1c3a5c:	test   %bl,%bl
  1c3a5e:	jne    1c3a6c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb9c>
  1c3a60:	jmp    1c3a76 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xba6>
  1c3a62:	call   *0x3238a0(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c3a68:	test   %bl,%bl
  1c3a6a:	je     1c3a76 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xba6>
  1c3a6c:	mov    0x50(%rsp),%rax
  1c3a71:	test   %rax,%rax
  1c3a74:	jne    1c3a7e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xbae>
  1c3a76:	mov    %r15,%rdi
  1c3a79:	call   4dd5d0 <_Unwind_Resume@plt>
  1c3a7e:	mov    0x58(%rsp),%rdi
  1c3a83:	cmp    $0x1fff,%rax
  1c3a89:	jbe    1c3ac3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xbf3>
  1c3a8b:	shl    $0x4,%rax
  1c3a8f:	movabs $0x7ffffffffffffff9,%rcx
  1c3a99:	mov    %rax,%rsi
  1c3a9c:	or     $0x1,%rsi
  1c3aa0:	cmp    %rcx,%rsi
  1c3aa3:	jae    1c3ae2 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc12>
  1c3aa5:	cmpb   $0x1,(%rdi,%rax,1)
  1c3aa9:	jne    1c3ae7 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xc17>
  1c3aab:	call   *0x32384f(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  1c3ab1:	mov    %r15,%rdi
  1c3ab4:	call   4dd5d0 <_Unwind_Resume@plt>
  1c3ab9:	call   110070 <std::process::abort>
  1c3abe:	call   110070 <std::process::abort>
  1c3ac3:	call   *0x32383f(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c3ac9:	mov    %r15,%rdi
  1c3acc:	call   4dd5d0 <_Unwind_Resume@plt>
  1c3ad1:	jmp    1c3a0d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb3d>
  1c3ad6:	call   *0x32382c(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c3adc:	test   %bl,%bl
  1c3ade:	jne    1c3a6c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xb9c>
  1c3ae0:	jmp    1c3a76 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G1Config>+0xba6>
  1c3ae2:	call   110070 <std::process::abort>
  1c3ae7:	call   *0x32381b(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  1c3aed:	mov    %r15,%rdi
  1c3af0:	call   4dd5d0 <_Unwind_Resume@plt>
