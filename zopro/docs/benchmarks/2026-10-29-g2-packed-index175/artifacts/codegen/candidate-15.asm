
/home/ubuntu/src/zopro-perf-work/g2-packed-index175/native/candidate-sdk:     file format elf64-x86-64


Disassembly of section .text:

0000000000290fa0 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute>:
  290fa0:	push   %rbp
  290fa1:	push   %r15
  290fa3:	push   %r14
  290fa5:	push   %r13
  290fa7:	push   %r12
  290fa9:	push   %rbx
  290faa:	sub    $0x68,%rsp
  290fae:	mov    (%rdi),%rdx
  290fb1:	mov    0x8(%rdi),%rcx
  290fb5:	mov    0x10(%rdi),%rax
  290fb9:	movq   $0x0,(%rdi)
  290fc0:	test   %rdx,%rdx
  290fc3:	je     29113d <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x19d>
  290fc9:	mov    0x38(%rdi),%rsi
  290fcd:	mov    %rdi,%rbx
  290fd0:	lea    0x50(%rsp),%r10
  290fd5:	mov    %rsi,0x60(%rsp)
  290fda:	vmovups 0x18(%rdi),%ymm0
  290fdf:	vmovups %ymm0,0x40(%rsp)
  290fe5:	mov    %rdx,0x28(%rsp)
  290fea:	mov    %rcx,0x30(%rsp)
  290fef:	mov    %rax,0x38(%rsp)
  290ff4:	mov    (%rdx),%rsi
  290ff7:	mov    0x40(%rsp),%r9
  290ffc:	mov    0x8(%rax),%r8
  291000:	sub    (%rcx),%rsi
  291003:	mov    (%rax),%rcx
  291006:	mov    $0x1,%edx
  29100b:	lea    0x10(%rsp),%rdi
  291010:	push   %r10
  291012:	push   0x50(%rsp)
  291016:	vzeroupper
  291019:	call   28f6f0 <rayon::iter::plumbing::bridge_producer_consumer::helper::<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>>
  29101e:	add    $0x10,%rsp
  291022:	mov    0x10(%rsp),%rax
  291027:	mov    0x18(%rsp),%r15
  29102c:	mov    0x20(%rsp),%r13
  291031:	mov    $0x1,%ebp
  291036:	cmpl   $0x2,0x40(%rbx)
  29103a:	jb     2910c9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x129>
  291040:	mov    0x50(%rbx),%r14
  291044:	mov    %rax,0x8(%rsp)
  291049:	mov    0x48(%rbx),%r12
  29104d:	mov    (%r14),%rax
  291050:	test   %rax,%rax
  291053:	je     29105a <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0xba>
  291055:	mov    %r12,%rdi
  291058:	call   *%rax
  29105a:	mov    0x8(%r14),%rsi
  29105e:	mov    0x8(%rsp),%rax
  291063:	test   %rsi,%rsi
  291066:	je     2910c9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x129>
  291068:	mov    0x10(%r14),%rax
  29106c:	cmp    $0x20000,%rsi
  291073:	setae  %cl
  291076:	cmp    $0x1001,%rax
  29107c:	setb   %dl
  29107f:	test   %cl,%dl
  291081:	je     2910b9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x119>
  291083:	blsr   %eax,%ecx
  291088:	jne    29114b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x1ab>
  29108e:	movabs $0x8000000000000000,%rcx
  291098:	sub    %rax,%rcx
  29109b:	cmp    %rcx,%rsi
  29109e:	jae    29114b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x1ab>
  2910a4:	cmpb   $0x1,(%r12,%rsi,1)
  2910a9:	jne    2910bb <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x11b>
  2910ab:	inc    %rsi
  2910ae:	mov    %r12,%rdi
  2910b1:	call   *0x259b91(%rip)        # 4eac48 <munmap@GLIBC_2.2.5>
  2910b7:	jmp    2910c4 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x124>
  2910b9:	jmp    2910bb <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x11b>
  2910bb:	mov    %r12,%rdi
  2910be:	call   *0x259b8c(%rip)        # 4eac50 <free@GLIBC_2.2.5>
  2910c4:	mov    0x8(%rsp),%rax
  2910c9:	mov    %rbp,0x40(%rbx)
  2910cd:	mov    %rax,0x48(%rbx)
  2910d1:	mov    %r15,0x50(%rbx)
  2910d5:	mov    %r13,0x58(%rbx)
  2910d9:	mov    0x60(%rbx),%rax
  2910dd:	movzbl 0x78(%rbx),%ebp
  2910e1:	mov    (%rax),%rdi
  2910e4:	cmp    $0x1,%bpl
  2910e8:	jne    2910fc <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x15c>
  2910ea:	lock incq (%rdi)
  2910ee:	jle    2911ba <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x21a>
  2910f4:	mov    (%rax),%r14
  2910f7:	mov    %r14,%rdi
  2910fa:	jmp    2910fc <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x15c>
  2910fc:	mov    $0x3,%eax
  291101:	mov    0x70(%rbx),%rsi
  291105:	xchg   %rax,0x68(%rbx)
  291109:	cmp    $0x2,%rax
  29110d:	jne    29111b <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x17b>
  29110f:	add    $0x1d8,%rdi
  291116:	call   ff7c0 <<rayon_core::sleep::Sleep>::wake_specific_thread>
  29111b:	test   %bpl,%bpl
  29111e:	je     29112e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x18e>
  291120:	lock decq (%r14)
  291124:	jne    29112e <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x18e>
  291126:	mov    %r14,%rdi
  291129:	call   100580 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  29112e:	add    $0x68,%rsp
  291132:	pop    %rbx
  291133:	pop    %r12
  291135:	pop    %r13
  291137:	pop    %r14
  291139:	pop    %r15
  29113b:	pop    %rbp
  29113c:	ret
  29113d:	lea    0x25887c(%rip),%rdi        # 4e99c0 <__frame_dummy_init_array_entry+0x7b60>
  291144:	call   f1ff0 <core::option::unwrap_failed>
  291149:	jmp    2911ba <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x21a>
  29114b:	call   110970 <std::process::abort>
  291150:	mov    0x8(%r14),%rsi
  291154:	test   %rsi,%rsi
  291157:	je     291165 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x1c5>
  291159:	mov    0x10(%r14),%rdx
  29115d:	mov    %r12,%rdi
  291160:	call   e9120 <__rustc::__rust_dealloc>
  291165:	mov    0x8(%rsp),%rax
  29116a:	mov    %rbp,0x40(%rbx)
  29116e:	mov    %rax,0x48(%rbx)
  291172:	mov    %r15,0x50(%rbx)
  291176:	mov    %r13,0x58(%rbx)
  29117a:	jmp    2911b5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x215>
  29117c:	test   %bpl,%bpl
  29117f:	je     2911b5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x215>
  291181:	lock decq (%r14)
  291185:	jne    2911b5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x215>
  291187:	mov    %r14,%rdi
  29118a:	call   100580 <<alloc::sync::Arc<rayon_core::registry::Registry>>::drop_slow>
  29118f:	jmp    2911b5 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x215>
  291191:	call   f2163 <core::panicking::panic_in_cleanup>
  291196:	mov    %rax,%rdi
  291199:	call   14485a <std::panicking::catch_unwind::cleanup>
  29119e:	mov    $0x2,%ebp
  2911a3:	mov    %rdx,%r15
  2911a6:	cmpl   $0x2,0x40(%rbx)
  2911aa:	jae    291040 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0xa0>
  2911b0:	jmp    2910c9 <<rayon_core::job::StackJob<rayon_core::latch::SpinLatch, rayon_core::join::join_context::call_b<rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, rayon::iter::plumbing::bridge_producer_consumer::helper<rayon::vec::DrainProducer<(usize, usize)>, rayon::iter::map::MapConsumer<rayon::iter::collect::consumer::CollectConsumer<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>, zopro_core::msm::reduce_indexed_buckets<zopro_bn254::curve::G2Config>::{closure#0}>>::{closure#1}>::{closure#0}, rayon::iter::collect::consumer::CollectResult<zopro_bn254::curve::Affine<zopro_bn254::curve::G2Config>>> as rayon_core::job::Job>::execute+0x129>
  2911b5:	call   1052d0 <<rayon_core::unwind::AbortIfPanic as core::ops::drop::Drop>::drop>
  2911ba:	ud2
  2911bc:	call   f2163 <core::panicking::panic_in_cleanup>
