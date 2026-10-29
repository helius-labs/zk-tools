
/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000245e60 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>>:
  245e60:	push   %rbp
  245e61:	push   %r15
  245e63:	push   %r14
  245e65:	push   %r13
  245e67:	push   %r12
  245e69:	push   %rbx
  245e6a:	sub    $0x138,%rsp
  245e71:	mov    0x170(%rsp),%rbp
  245e79:	movq   $0x0,0x50(%rsp)
  245e82:	movq   $0x8,0x58(%rsp)
  245e8b:	mov    %r8,0xa0(%rsp)
  245e93:	mov    %rcx,0x90(%rsp)
  245e9b:	mov    %rsi,0x98(%rsp)
  245ea3:	movq   $0x0,0x60(%rsp)
  245eac:	mov    %rdi,0x88(%rsp)
  245eb4:	mov    %rdx,0x28(%rsp)
  245eb9:	test   %rbp,%rbp
  245ebc:	je     24600c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1ac>
  245ec2:	lea    0x0(,%rbp,8),%r12
  245eca:	mov    %r9,%r13
  245ecd:	mov    $0x8,%edi
  245ed2:	mov    %r12,%rsi
  245ed5:	xor    %edx,%edx
  245ed7:	call   4b8940 <<zopro_core::alloc::linux::Allocator>::allocate>
  245edc:	test   %rax,%rax
  245edf:	je     246854 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x9f4>
  245ee5:	mov    %rbp,0x8(%rsp)
  245eea:	mov    %rax,0x10(%rsp)
  245eef:	movq   $0x0,(%rax)
  245ef6:	movq   $0x1,0x18(%rsp)
  245eff:	cmp    $0x1,%rbp
  245f03:	je     246046 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x1e6>
  245f09:	mov    0x0(%r13),%r14
  245f0d:	mov    $0x8,%r12d
  245f13:	mov    $0x1,%ebx
  245f18:	xor    %r15d,%r15d
  245f1b:	jmp    245f47 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xe7>
  245f1d:	nopl   (%rax)
  245f20:	mov    0x10(%rsp),%rax
  245f25:	mov    (%rsp),%r14
  245f29:	add    $0x8,%r13
  245f2d:	mov    %r15,(%rax,%rbx,8)
  245f31:	inc    %rbx
  245f34:	mov    %rbx,0x18(%rsp)
  245f39:	cmp    $0x3,%rbp
  245f3d:	lea    -0x1(%rbp),%rbp
  245f41:	jb     246078 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x218>
  245f47:	mov    0x8(%r13),%rax
  245f4b:	mov    %rax,(%rsp)
  245f4f:	sub    %r14,%rax
  245f52:	jbe    245ff2 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x192>
  245f58:	mov    %rax,%rbx
  245f5b:	and    $0x3ff,%eax
  245f60:	shr    $0xa,%rbx
  245f64:	mov    %rbp,0x30(%rsp)
  245f69:	mov    %r15,%rbp
  245f6c:	mov    %r13,0x20(%rsp)
  245f71:	cmp    $0x1,%rax
  245f75:	mov    %r12,%rax
  245f78:	sbb    $0xffffffffffffffff,%rbx
  245f7c:	shl    $0x4,%rbp
  245f80:	or     $0x8,%rbp
  245f84:	jmp    245fb0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x150>
  245f86:	cs nopw 0x0(%rax,%rax,1)
  245f90:	mov    %r14,-0x8(%rax,%rbp,1)
  245f95:	mov    %r13,(%rax,%rbp,1)
  245f99:	inc    %r15
  245f9c:	dec    %rbx
  245f9f:	add    $0x10,%rbp
  245fa3:	mov    %r12,%r14
  245fa6:	mov    %r15,0x60(%rsp)
  245fab:	test   %rbx,%rbx
  245fae:	je     245fe0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x180>
  245fb0:	mov    (%rsp),%rcx
  245fb4:	lea    0x400(%r14),%r12
  245fbb:	mov    %r12,%r13
  245fbe:	cmp    %r12,%rcx
  245fc1:	cmovb  %rcx,%r13
  245fc5:	cmp    0x50(%rsp),%r15
  245fca:	jne    245f90 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x130>
  245fcc:	lea    0x50(%rsp),%rdi
  245fd1:	call   2b46f0 <<alloc::raw_vec::RawVec<zopro_core::preparation::Reservation>>::grow_one>
  245fd6:	mov    0x58(%rsp),%rax
  245fdb:	jmp    245f90 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x130>
  245fdd:	nopl   (%rax)
  245fe0:	mov    0x18(%rsp),%rbx
  245fe5:	mov    0x20(%rsp),%r13
  245fea:	mov    0x30(%rsp),%rbp
  245fef:	mov    %rax,%r12
  245ff2:	cmp    0x8(%rsp),%rbx
  245ff7:	jne    245f20 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xc0>
  245ffd:	lea    0x8(%rsp),%rdi
  246002:	call   f3f90 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  246007:	jmp    245f20 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xc0>
  24600c:	movq   $0x0,0x8(%rsp)
  246015:	movq   $0x8,0x10(%rsp)
  24601e:	movq   $0x0,0x18(%rsp)
  246027:	lea    0x8(%rsp),%rdi
  24602c:	call   f3f90 <<alloc::raw_vec::RawVec<usize>>::grow_one>
  246031:	mov    0x10(%rsp),%rax
  246036:	movq   $0x0,(%rax)
  24603d:	movq   $0x1,0x18(%rsp)
  246046:	movq   $0x0,0x38(%rsp)
  24604f:	movq   $0x8,0x40(%rsp)
  246058:	movq   $0x0,0x48(%rsp)
  246061:	movq   $0x0,0x80(%rsp)
  24606d:	mov    $0x8,%r12d
  246073:	xor    %r13d,%r13d
  246076:	jmp    2460ae <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x24e>
  246078:	mov    0x50(%rsp),%r13
  24607d:	mov    0x58(%rsp),%r12
  246082:	movq   $0x0,0x38(%rsp)
  24608b:	movq   $0x8,0x40(%rsp)
  246094:	movq   $0x0,0x48(%rsp)
  24609d:	mov    %r15,0x80(%rsp)
  2460a5:	test   %r15,%r15
  2460a8:	jne    246796 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x936>
  2460ae:	mov    $0x8,%r14d
  2460b4:	mov    $0x1,%bl
  2460b6:	xor    %r15d,%r15d
  2460b9:	xor    %ebp,%ebp
  2460bb:	xor    %eax,%eax
  2460bd:	mov    0x90(%rsp),%rdx
  2460c5:	mov    0xa0(%rsp),%rsi
  2460cd:	mov    %r13,0xa8(%rsp)
  2460d5:	mov    %r12,0xb0(%rsp)
  2460dd:	mov    %r15,0xb8(%rsp)
  2460e5:	mov    0x98(%rsp),%rdi
  2460ed:	mov    %rax,0x30(%rsp)
  2460f2:	shl    $0x7,%rax
  2460f6:	lea    0xc0(%rsp),%rcx
  2460fe:	mov    %r15,0x118(%rsp)
  246106:	mov    %r13,0x68(%rsp)
  24610b:	mov    %r12,0x70(%rsp)
  246110:	movq   $0x0,0x78(%rsp)
  246119:	add    %r14,%rax
  24611c:	mov    %rcx,0x100(%rsp)
  246124:	mov    %rax,0x108(%rsp)
  24612c:	lea    0x68(%rsp),%rax
  246131:	mov    %r15,0x110(%rsp)
  246139:	mov    %rax,0xe0(%rsp)
  246141:	movq   $0x0,0xe8(%rsp)
  24614d:	mov    %r15,0xf0(%rsp)
  246155:	mov    %r15,0xf8(%rsp)
  24615d:	mov    %rdx,0xc0(%rsp)
  246165:	mov    %rsi,0xc8(%rsp)
  24616d:	mov    0x28(%rsp),%rsi
  246172:	mov    %rdi,0xd0(%rsp)
  24617a:	mov    %rsi,0xd8(%rsp)
  246182:	cmp    %r15,%r13
  246185:	jb     2467e6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x986>
  24618b:	mov    %fs:0xffffffffffffffc0,%rax
  246194:	test   %rax,%rax
  246197:	je     2461a1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x341>
  246199:	add    $0x110,%rax
  24619f:	jmp    2461a6 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x346>
  2461a1:	call   104130 <rayon_core::registry::global_registry>
  2461a6:	mov    (%rax),%rax
  2461a9:	mov    0x208(%rax),%rcx
  2461b0:	lea    0x100(%rsp),%rax
  2461b8:	xor    %edx,%edx
  2461ba:	lea    0x120(%rsp),%rdi
  2461c2:	mov    $0x1,%r8d
  2461c8:	mov    %r15,%rsi
  2461cb:	mov    %r12,%r9
  2461ce:	push   %rax
  2461cf:	push   %r15
  2461d1:	call   28ccf0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>
  2461d6:	add    $0x10,%rsp
  2461da:	cmp    %r15,0x78(%rsp)
  2461df:	sete   %al
  2461e2:	or     %al,%bl
  2461e4:	je     2461ef <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x38f>
  2461e6:	movq   $0x0,0x78(%rsp)
  2461ef:	mov    0x68(%rsp),%rax
  2461f4:	mov    %rbp,0x20(%rsp)
  2461f9:	test   %rax,%rax
  2461fc:	je     24623f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x3df>
  2461fe:	mov    0x70(%rsp),%rdi
  246203:	cmp    $0x1fff,%rax
  246209:	jbe    246237 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x3d7>
  24620b:	shl    $0x4,%rax
  24620f:	movabs $0x7ffffffffffffff9,%rcx
  246219:	mov    %rax,%rsi
  24621c:	or     $0x1,%rsi
  246220:	cmp    %rcx,%rsi
  246223:	jae    246861 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa01>
  246229:	cmpb   $0x1,(%rdi,%rax,1)
  24622d:	jne    246239 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x3d9>
  24622f:	call   *0x2a10cb(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  246235:	jmp    24623f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x3df>
  246237:	jmp    246239 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x3d9>
  246239:	call   *0x2a10c9(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  24623f:	mov    0x130(%rsp),%rax
  246247:	mov    %rax,0xe0(%rsp)
  24624f:	cmp    %r15,%rax
  246252:	jne    246800 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x9a0>
  246258:	mov    0x18(%rsp),%r12
  24625d:	mov    %r14,(%rsp)
  246261:	xor    %r14d,%r14d
  246264:	movabs $0x7ffffffffffffff9,%rcx
  24626e:	dec    %rcx
  246271:	lea    -0x1(%r12),%r10
  246276:	cmp    $0x2,%r12
  24627a:	mov    %r10,%r8
  24627d:	cmovb  %r14,%r8
  246281:	lea    0x0(,%r8,8),%rbx
  246289:	mov    %r8,%rax
  24628c:	shr    $0x3d,%rax
  246290:	setne  %al
  246293:	cmp    %rcx,%rbx
  246296:	seta   %cl
  246299:	or     %al,%cl
  24629b:	je     2462ad <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x44d>
  24629d:	mov    %r14,%rdi
  2462a0:	mov    %rbx,%rsi
  2462a3:	call   ed521 <alloc::raw_vec::handle_error>
  2462a8:	jmp    246873 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa13>
  2462ad:	mov    0x10(%rsp),%rbp
  2462b2:	test   %rbx,%rbx
  2462b5:	je     2462e1 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x481>
  2462b7:	mov    $0x8,%edi
  2462bc:	mov    $0x8,%r14d
  2462c2:	mov    %rbx,%rsi
  2462c5:	xor    %edx,%edx
  2462c7:	mov    %r8,%r13
  2462ca:	call   4b8940 <<zopro_core::alloc::linux::Allocator>::allocate>
  2462cf:	lea    -0x1(%r12),%r10
  2462d4:	mov    %r13,%rcx
  2462d7:	mov    %rax,%r13
  2462da:	test   %rax,%rax
  2462dd:	jne    2462e9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x489>
  2462df:	jmp    24629d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x43d>
  2462e1:	mov    $0x8,%r13d
  2462e7:	xor    %ecx,%ecx
  2462e9:	mov    0x30(%rsp),%rdx
  2462ee:	mov    %rcx,0x28(%rsp)
  2462f3:	add    %r15,%rdx
  2462f6:	cmp    $0x2,%r12
  2462fa:	jae    246349 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x4e9>
  2462fc:	xor    %r10d,%r10d
  2462ff:	mov    %edx,%ebx
  246301:	lea    0x0(,%rbx,4),%r14
  246309:	test   %rbx,%rbx
  24630c:	je     24661c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x7bc>
  246312:	mov    %r10,%r12
  246315:	mov    %rdx,%r15
  246318:	mov    $0x4,%edi
  24631d:	mov    %r14,%rsi
  246320:	xor    %edx,%edx
  246322:	vzeroupper
  246325:	call   4b8940 <<zopro_core::alloc::linux::Allocator>::allocate>
  24632a:	test   %rax,%rax
  24632d:	je     246866 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa06>
  246333:	mov    %rax,%rbp
  246336:	mov    %r12,%r10
  246339:	cmp    $0x8,%ebx
  24633c:	jae    246461 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x601>
  246342:	xor    %eax,%eax
  246344:	jmp    246540 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x6e0>
  246349:	mov    0x0(%rbp),%rcx
  24634d:	mov    %rdx,%r9
  246350:	cmp    $0x21,%r12
  246354:	jb     24636c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x50c>
  246356:	mov    %r13,%rax
  246359:	sub    %rbp,%rax
  24635c:	add    $0xfffffffffffffff8,%rax
  246360:	cmp    $0x100,%rax
  246366:	jae    246554 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x6f4>
  24636c:	xor    %eax,%eax
  24636e:	mov    %r12,%rdx
  246371:	sub    %rax,%rdx
  246374:	mov    %r12d,%esi
  246377:	and    $0x7,%esi
  24637a:	add    $0xfffffffffffffffe,%rdx
  24637e:	cmp    $0x1,%esi
  246381:	je     2463ae <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x54e>
  246383:	lea    -0x1(%r12),%esi
  246388:	and    $0x7,%esi
  24638b:	nopl   0x0(%rax,%rax,1)
  246390:	mov    %rcx,%rdi
  246393:	mov    0x8(%rbp),%rcx
  246397:	add    $0x8,%rbp
  24639b:	mov    %rcx,%r8
  24639e:	sub    %rdi,%r8
  2463a1:	mov    %r8,0x0(%r13,%rax,8)
  2463a6:	inc    %rax
  2463a9:	dec    %rsi
  2463ac:	jne    246390 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x530>
  2463ae:	cmp    $0x7,%rdx
  2463b2:	jb     246606 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x7a6>
  2463b8:	lea    0x38(%r13,%rax,8),%rdx
  2463bd:	not    %rax
  2463c0:	xor    %esi,%esi
  2463c2:	add    %r12,%rax
  2463c5:	data16 cs nopw 0x0(%rax,%rax,1)
  2463d0:	mov    0x8(%rbp,%rsi,8),%rdi
  2463d5:	mov    %rdi,%r8
  2463d8:	sub    %rcx,%r8
  2463db:	mov    %r8,-0x38(%rdx,%rsi,8)
  2463e0:	mov    0x10(%rbp,%rsi,8),%rcx
  2463e5:	mov    %rcx,%r8
  2463e8:	sub    %rdi,%r8
  2463eb:	mov    %r8,-0x30(%rdx,%rsi,8)
  2463f0:	mov    0x18(%rbp,%rsi,8),%rdi
  2463f5:	mov    %rdi,%r8
  2463f8:	sub    %rcx,%r8
  2463fb:	mov    %r8,-0x28(%rdx,%rsi,8)
  246400:	mov    0x20(%rbp,%rsi,8),%rcx
  246405:	mov    %rcx,%r8
  246408:	sub    %rdi,%r8
  24640b:	mov    %r8,-0x20(%rdx,%rsi,8)
  246410:	mov    0x28(%rbp,%rsi,8),%rdi
  246415:	mov    %rdi,%r8
  246418:	sub    %rcx,%r8
  24641b:	mov    %r8,-0x18(%rdx,%rsi,8)
  246420:	mov    0x30(%rbp,%rsi,8),%rcx
  246425:	mov    %rcx,%r8
  246428:	sub    %rdi,%r8
  24642b:	mov    %r8,-0x10(%rdx,%rsi,8)
  246430:	mov    0x38(%rbp,%rsi,8),%rdi
  246435:	mov    %rdi,%r8
  246438:	sub    %rcx,%r8
  24643b:	mov    %r8,-0x8(%rdx,%rsi,8)
  246440:	mov    0x40(%rbp,%rsi,8),%rcx
  246445:	mov    %rcx,%r8
  246448:	sub    %rdi,%r8
  24644b:	mov    %r8,(%rdx,%rsi,8)
  24644f:	add    $0x8,%rsi
  246453:	cmp    %rsi,%rax
  246456:	jne    2463d0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x570>
  24645c:	jmp    246606 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x7a6>
  246461:	cmp    $0x40,%ebx
  246464:	jae    24646d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x60d>
  246466:	xor    %eax,%eax
  246468:	jmp    2464fb <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x69b>
  24646d:	vmovdqa64 -0x239877(%rip),%zmm0        # cc00 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x11c0>
  246477:	vpbroadcastd -0x23a701(%rip),%zmm1        # bd80 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x340>
  246481:	vpbroadcastd -0x23a6e7(%rip),%zmm2        # bda4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x364>
  24648b:	vpbroadcastd -0x23a6dd(%rip),%zmm3        # bdb8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x378>
  246495:	vpbroadcastd -0x23a6db(%rip),%zmm4        # bdc4 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x384>
  24649f:	mov    %r15d,%eax
  2464a2:	and    $0xffffffc0,%eax
  2464a5:	xor    %ecx,%ecx
  2464a7:	nopw   0x0(%rax,%rax,1)
  2464b0:	vpaddd %zmm1,%zmm0,%zmm5
  2464b6:	vpaddd %zmm2,%zmm0,%zmm6
  2464bc:	vpaddd %zmm3,%zmm0,%zmm7
  2464c2:	vmovdqu64 %zmm0,0x0(%rbp,%rcx,4)
  2464ca:	vpaddd %zmm4,%zmm0,%zmm0
  2464d0:	vmovdqu64 %zmm5,0x40(%rbp,%rcx,4)
  2464d8:	vmovdqu64 %zmm6,0x80(%rbp,%rcx,4)
  2464e0:	vmovdqu64 %zmm7,0xc0(%rbp,%rcx,4)
  2464e8:	add    $0x40,%rcx
  2464ec:	cmp    %rcx,%rax
  2464ef:	jne    2464b0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x650>
  2464f1:	cmp    %eax,%ebx
  2464f3:	je     24654c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x6ec>
  2464f5:	test   $0x38,%r15b
  2464f9:	je     246540 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x6e0>
  2464fb:	mov    %rax,%rcx
  2464fe:	vpbroadcastd %ecx,%ymm0
  246504:	vpor   -0x234d4c(%rip),%ymm0,%ymm0        # 117c0 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x5d80>
  24650c:	vpbroadcastd -0x23a76d(%rip),%ymm1        # bda8 <serde_json::ser::Formatter::write_char_escape::HEX_DIGITS+0x368>
  246515:	mov    %r15d,%eax
  246518:	and    $0xfffffff8,%eax
  24651b:	nopl   0x0(%rax,%rax,1)
  246520:	vmovdqu %ymm0,0x0(%rbp,%rcx,4)
  246526:	add    $0x8,%rcx
  24652a:	vpaddd %ymm1,%ymm0,%ymm0
  24652e:	cmp    %rcx,%rax
  246531:	jne    246520 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x6c0>
  246533:	cmp    %eax,%ebx
  246535:	je     24654c <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x6ec>
  246537:	nopw   0x0(%rax,%rax,1)
  246540:	mov    %eax,0x0(%rbp,%rax,4)
  246544:	inc    %rax
  246547:	cmp    %rax,%rbx
  24654a:	jne    246540 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x6e0>
  24654c:	mov    %r15,%rdx
  24654f:	jmp    246621 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x7c1>
  246554:	mov    %r10,%rax
  246557:	and    $0xffffffffffffffe0,%rax
  24655b:	vpbroadcastq %rcx,%zmm0
  246561:	xor    %ecx,%ecx
  246563:	lea    0x0(%rbp,%rax,8),%rdx
  246568:	nopl   0x0(%rax,%rax,1)
  246570:	vmovdqu64 0x8(%rbp,%rcx,8),%zmm1
  24657b:	vmovdqu64 0x48(%rbp,%rcx,8),%zmm2
  246586:	vmovdqu64 0x88(%rbp,%rcx,8),%zmm3
  246591:	valignq $0x7,%zmm0,%zmm1,%zmm4
  246598:	vmovdqu64 0xc8(%rbp,%rcx,8),%zmm0
  2465a3:	valignq $0x7,%zmm1,%zmm2,%zmm5
  2465aa:	valignq $0x7,%zmm2,%zmm3,%zmm6
  2465b1:	vpsubq %zmm4,%zmm1,%zmm1
  2465b7:	vpsubq %zmm5,%zmm2,%zmm2
  2465bd:	vmovdqu64 %zmm1,0x0(%r13,%rcx,8)
  2465c5:	vmovdqu64 %zmm2,0x40(%r13,%rcx,8)
  2465cd:	valignq $0x7,%zmm3,%zmm0,%zmm7
  2465d4:	vpsubq %zmm6,%zmm3,%zmm3
  2465da:	vmovdqu64 %zmm3,0x80(%r13,%rcx,8)
  2465e2:	vpsubq %zmm7,%zmm0,%zmm4
  2465e8:	vmovdqu64 %zmm4,0xc0(%r13,%rcx,8)
  2465f0:	add    $0x20,%rcx
  2465f4:	cmp    %rcx,%rax
  2465f7:	jne    246570 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x710>
  2465fd:	cmp    %rax,%r10
  246600:	jne    246781 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x921>
  246606:	mov    %r9,%rdx
  246609:	mov    %edx,%ebx
  24660b:	lea    0x0(,%rbx,4),%r14
  246613:	test   %rbx,%rbx
  246616:	jne    246312 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x4b2>
  24661c:	mov    $0x4,%ebp
  246621:	mov    0x10(%rsp),%r9
  246626:	sub    $0x8,%rsp
  24662a:	mov    0x90(%rsp),%rdi
  246632:	mov    0x8(%rsp),%rsi
  246637:	mov    %rbp,%rcx
  24663a:	mov    %rbx,%r8
  24663d:	push   %r10
  24663f:	push   %r13
  246641:	push   0x30(%rsp)
  246645:	vzeroupper
  246648:	call   22b0f0 <zopro_core::msm::group_affine::<zopro_bn254::curve::G2Config>>
  24664d:	add    $0x20,%rsp
  246651:	test   %rbx,%rbx
  246654:	je     246683 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x823>
  246656:	cmp    $0x7fff,%ebx
  24665c:	jbe    246678 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x818>
  24665e:	cmpb   $0x1,0x0(%rbp,%r14,1)
  246664:	jne    24667a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x81a>
  246666:	or     $0x1,%r14
  24666a:	mov    %rbp,%rdi
  24666d:	mov    %r14,%rsi
  246670:	call   *0x2a0c8a(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  246676:	jmp    246683 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x823>
  246678:	jmp    24667a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x81a>
  24667a:	mov    %rbp,%rdi
  24667d:	call   *0x2a0c85(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  246683:	mov    0x28(%rsp),%rcx
  246688:	mov    (%rsp),%rbx
  24668c:	mov    0x20(%rsp),%r14
  246691:	test   %rcx,%rcx
  246694:	je     2466db <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x87b>
  246696:	cmp    $0x3fff,%rcx
  24669d:	jbe    2466d0 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x870>
  24669f:	shl    $0x3,%rcx
  2466a3:	movabs $0x7ffffffffffffff9,%rax
  2466ad:	mov    %rcx,%rsi
  2466b0:	or     $0x1,%rsi
  2466b4:	cmp    %rax,%rsi
  2466b7:	jae    246875 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa15>
  2466bd:	cmpb   $0x1,0x0(%r13,%rcx,1)
  2466c3:	jne    2466d2 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x872>
  2466c5:	mov    %r13,%rdi
  2466c8:	call   *0x2a0c32(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  2466ce:	jmp    2466db <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x87b>
  2466d0:	jmp    2466d2 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x872>
  2466d2:	mov    %r13,%rdi
  2466d5:	call   *0x2a0c2d(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  2466db:	test   %r14,%r14
  2466de:	je     246724 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x8c4>
  2466e0:	cmp    $0x3ff,%r14
  2466e7:	jbe    246719 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x8b9>
  2466e9:	shl    $0x7,%r14
  2466ed:	movabs $0x7ffffffffffffff9,%rax
  2466f7:	mov    %r14,%rsi
  2466fa:	or     $0x1,%rsi
  2466fe:	cmp    %rax,%rsi
  246701:	jae    24687a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa1a>
  246707:	cmpb   $0x1,(%rbx,%r14,1)
  24670c:	jne    24671b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x8bb>
  24670e:	mov    %rbx,%rdi
  246711:	call   *0x2a0be9(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  246717:	jmp    246724 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x8c4>
  246719:	jmp    24671b <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x8bb>
  24671b:	mov    %rbx,%rdi
  24671e:	call   *0x2a0be4(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  246724:	mov    0x8(%rsp),%rax
  246729:	test   %rax,%rax
  24672c:	je     24676f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x90f>
  24672e:	mov    0x10(%rsp),%rdi
  246733:	cmp    $0x3fff,%rax
  246739:	jbe    246767 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x907>
  24673b:	shl    $0x3,%rax
  24673f:	movabs $0x7ffffffffffffff9,%rcx
  246749:	mov    %rax,%rsi
  24674c:	or     $0x1,%rsi
  246750:	cmp    %rcx,%rsi
  246753:	jae    24687f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa1f>
  246759:	cmpb   $0x1,(%rdi,%rax,1)
  24675d:	jne    246769 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x909>
  24675f:	call   *0x2a0b9b(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  246765:	jmp    24676f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x90f>
  246767:	jmp    246769 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x909>
  246769:	call   *0x2a0b99(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  24676f:	add    $0x138,%rsp
  246776:	pop    %rbx
  246777:	pop    %r12
  246779:	pop    %r13
  24677b:	pop    %r14
  24677d:	pop    %r15
  24677f:	pop    %rbp
  246780:	ret
  246781:	vextracti32x4 $0x3,%zmm0,%xmm0
  246788:	mov    %rdx,%rbp
  24678b:	vpextrq $0x1,%xmm0,%rcx
  246791:	jmp    24636e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x50e>
  246796:	lea    0x38(%rsp),%rdi
  24679b:	mov    $0x8,%ecx
  2467a0:	mov    $0x80,%r8d
  2467a6:	xor    %esi,%esi
  2467a8:	mov    %r15,%rdx
  2467ab:	call   1b5e60 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
  2467b0:	mov    0x38(%rsp),%rbp
  2467b5:	mov    0x48(%rsp),%rcx
  2467ba:	mov    %rbp,%rax
  2467bd:	sub    %rcx,%rax
  2467c0:	cmp    %r15,%rax
  2467c3:	jae    246884 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa24>
  2467c9:	lea    -0x226e29(%rip),%rdi        # 1f9a7 <zmij::STATIC_DATA+0x55a7>
  2467d0:	lea    0x29e059(%rip),%rdx        # 4e4830 <__frame_dummy_init_array_entry+0x6210>
  2467d7:	mov    $0x2f,%esi
  2467dc:	call   f03d0 <core::panicking::panic>
  2467e1:	jmp    246873 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa13>
  2467e6:	lea    -0x226e46(%rip),%rdi        # 1f9a7 <zmij::STATIC_DATA+0x55a7>
  2467ed:	lea    0x29e4d4(%rip),%rdx        # 4e4cc8 <__frame_dummy_init_array_entry+0x66a8>
  2467f4:	mov    $0x2f,%esi
  2467f9:	call   f03d0 <core::panicking::panic>
  2467fe:	jmp    246873 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa13>
  246800:	lea    0x80(%rsp),%rax
  246808:	lea    -0x153ebf(%rip),%r8        # f2950 <<u64 as core::fmt::Display>::fmt>
  24680f:	lea    0xe0(%rsp),%rcx
  246817:	mov    %rax,0xa8(%rsp)
  24681f:	mov    %r8,0xb0(%rsp)
  246827:	mov    %rcx,0xb8(%rsp)
  24682f:	mov    %r8,0xc0(%rsp)
  246837:	lea    -0x22faec(%rip),%rdi        # 16d52 <<serde_json::value::Value as core::ops::index::Index<_>>::index::NULL+0x52d2>
  24683e:	lea    0x29c093(%rip),%rdx        # 4e28d8 <__frame_dummy_init_array_entry+0x42b8>
  246845:	lea    0xa8(%rsp),%rsi
  24684d:	call   ee300 <core::panicking::panic_fmt>
  246852:	jmp    246873 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa13>
  246854:	mov    $0x8,%edi
  246859:	mov    %r12,%rsi
  24685c:	call   ed521 <alloc::raw_vec::handle_error>
  246861:	call   110070 <std::process::abort>
  246866:	mov    $0x4,%edi
  24686b:	mov    %r14,%rsi
  24686e:	call   ed521 <alloc::raw_vec::handle_error>
  246873:	ud2
  246875:	call   110070 <std::process::abort>
  24687a:	call   110070 <std::process::abort>
  24687f:	call   110070 <std::process::abort>
  246884:	mov    0x40(%rsp),%r14
  246889:	xor    %ebx,%ebx
  24688b:	mov    %rcx,%rax
  24688e:	jmp    2460bd <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0x25d>
  246893:	jmp    2468e9 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa89>
  246895:	mov    %rax,%r15
  246898:	jmp    2468b4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa54>
  24689a:	jmp    2468da <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa7a>
  24689c:	mov    %rax,%r15
  24689f:	test   %rbx,%rbx
  2468a2:	je     2468b4 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa54>
  2468a4:	mov    $0x4,%edx
  2468a9:	mov    %rbp,%rdi
  2468ac:	mov    %r14,%rsi
  2468af:	call   e8820 <__rustc::__rust_dealloc>
  2468b4:	cmpq   $0x0,0x28(%rsp)
  2468ba:	je     246956 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaf6>
  2468c0:	mov    0x28(%rsp),%rsi
  2468c5:	shl    $0x3,%rsi
  2468c9:	mov    $0x8,%edx
  2468ce:	mov    %r13,%rdi
  2468d1:	call   e8820 <__rustc::__rust_dealloc>
  2468d6:	jmp    246956 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaf6>
  2468d8:	jmp    2468da <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xa7a>
  2468da:	mov    %rax,%r15
  2468dd:	mov    $0x1,%bl
  2468df:	jmp    2469a5 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb45>
  2468e4:	mov    %rax,%r15
  2468e7:	jmp    24692f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xacf>
  2468e9:	mov    %rax,%r15
  2468ec:	test   %r13,%r13
  2468ef:	je     24692f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xacf>
  2468f1:	shl    $0x4,%r13
  2468f5:	mov    $0x8,%edx
  2468fa:	mov    %r12,%rdi
  2468fd:	mov    %r13,%rsi
  246900:	jmp    24692a <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xaca>
  246902:	mov    %rax,%r15
  246905:	lea    0xe0(%rsp),%rdi
  24690d:	call   1f9400 <core::ptr::drop_glue::<rayon::vec::Drain<(usize, usize)>>>
  246912:	mov    0x68(%rsp),%rsi
  246917:	test   %rsi,%rsi
  24691a:	je     24692f <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xacf>
  24691c:	mov    0x70(%rsp),%rdi
  246921:	shl    $0x4,%rsi
  246925:	mov    $0x8,%edx
  24692a:	call   e8820 <__rustc::__rust_dealloc>
  24692f:	mov    0x38(%rsp),%rsi
  246934:	test   %rsi,%rsi
  246937:	je     2469a3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb43>
  246939:	mov    0x40(%rsp),%rdi
  24693e:	shl    $0x7,%rsi
  246942:	mov    $0x8,%edx
  246947:	call   e8820 <__rustc::__rust_dealloc>
  24694c:	jmp    2469a3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb43>
  24694e:	call   f1863 <core::panicking::panic_in_cleanup>
  246953:	mov    %rax,%r15
  246956:	mov    (%rsp),%rdi
  24695a:	mov    0x20(%rsp),%rcx
  24695f:	test   %rcx,%rcx
  246962:	je     2469a3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb43>
  246964:	cmp    $0x3ff,%rcx
  24696b:	jbe    24699d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb3d>
  24696d:	shl    $0x7,%rcx
  246971:	movabs $0x7ffffffffffffff9,%rax
  24697b:	mov    %rcx,%rsi
  24697e:	or     $0x1,%rsi
  246982:	cmp    %rax,%rsi
  246985:	jae    246a49 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xbe9>
  24698b:	cmpb   $0x1,(%rdi,%rcx,1)
  24698f:	jne    246a61 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xc01>
  246995:	call   *0x2a0965(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  24699b:	jmp    2469a3 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb43>
  24699d:	call   *0x2a0965(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  2469a3:	xor    %ebx,%ebx
  2469a5:	mov    0x8(%rsp),%rax
  2469aa:	test   %rax,%rax
  2469ad:	jne    2469b5 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb55>
  2469af:	test   %bl,%bl
  2469b1:	jne    2469fc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb9c>
  2469b3:	jmp    246a06 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xba6>
  2469b5:	mov    0x10(%rsp),%rdi
  2469ba:	cmp    $0x3fff,%rax
  2469c0:	jbe    2469f2 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb92>
  2469c2:	shl    $0x3,%rax
  2469c6:	movabs $0x7ffffffffffffff9,%rcx
  2469d0:	mov    %rax,%rsi
  2469d3:	or     $0x1,%rsi
  2469d7:	cmp    %rcx,%rsi
  2469da:	jae    246a4e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xbee>
  2469dc:	cmpb   $0x1,(%rdi,%rax,1)
  2469e0:	jne    246a66 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xc06>
  2469e6:	call   *0x2a0914(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  2469ec:	test   %bl,%bl
  2469ee:	jne    2469fc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb9c>
  2469f0:	jmp    246a06 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xba6>
  2469f2:	call   *0x2a0910(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  2469f8:	test   %bl,%bl
  2469fa:	je     246a06 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xba6>
  2469fc:	mov    0x50(%rsp),%rax
  246a01:	test   %rax,%rax
  246a04:	jne    246a0e <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xbae>
  246a06:	mov    %r15,%rdi
  246a09:	call   4dd5d0 <_Unwind_Resume@plt>
  246a0e:	mov    0x58(%rsp),%rdi
  246a13:	cmp    $0x1fff,%rax
  246a19:	jbe    246a53 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xbf3>
  246a1b:	shl    $0x4,%rax
  246a1f:	movabs $0x7ffffffffffffff9,%rcx
  246a29:	mov    %rax,%rsi
  246a2c:	or     $0x1,%rsi
  246a30:	cmp    %rcx,%rsi
  246a33:	jae    246a72 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xc12>
  246a35:	cmpb   $0x1,(%rdi,%rax,1)
  246a39:	jne    246a77 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xc17>
  246a3b:	call   *0x2a08bf(%rip)        # 4e7300 <munmap@GLIBC_2.2.5>
  246a41:	mov    %r15,%rdi
  246a44:	call   4dd5d0 <_Unwind_Resume@plt>
  246a49:	call   110070 <std::process::abort>
  246a4e:	call   110070 <std::process::abort>
  246a53:	call   *0x2a08af(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  246a59:	mov    %r15,%rdi
  246a5c:	call   4dd5d0 <_Unwind_Resume@plt>
  246a61:	jmp    24699d <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb3d>
  246a66:	call   *0x2a089c(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  246a6c:	test   %bl,%bl
  246a6e:	jne    2469fc <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xb9c>
  246a70:	jmp    246a06 <zopro_core::msm::reduce_indexed_buckets::<zopro_bn254::curve::G2Config>+0xba6>
  246a72:	call   110070 <std::process::abort>
  246a77:	call   *0x2a088b(%rip)        # 4e7308 <free@GLIBC_2.2.5>
  246a7d:	mov    %r15,%rdi
  246a80:	call   4dd5d0 <_Unwind_Resume@plt>
