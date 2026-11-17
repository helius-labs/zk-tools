  1d6712:	call   *0x33a298(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  1d6718:	movzbl 0xf(%rsp),%eax
  1d671d:	test   %al,%al
  1d671f:	je     1d67d6 <zopro_core::msm_affine::msm_canonical::<zopro_bn254::curve::G1Config>+0x1f6>
  1d6725:	lea    0xa0(%rsp),%rdi
  1d672d:	mov    %rbp,%rsi
  1d6730:	mov    %r13,%rdx
  1d6733:	call   2317f0 <zopro_core::msm_affine::grouping::scalar_groups>
  1d6738:	cmpq   $0xffffffffffffffff,0xa0(%rsp)
  1d6741:	je     1d67d6 <zopro_core::msm_affine::msm_canonical::<zopro_bn254::curve::G1Config>+0x1f6>
  1d6747:	vmovups 0xd8(%rsp),%zmm1
  1d6752:	vmovups 0xa0(%rsp),%zmm0
--
  1d6a20:	call   *0x339f8a(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  1d6a26:	mov    %r13d,%eax
  1d6a29:	test   %al,%al
  1d6a2b:	je     1d6a97 <zopro_core::msm_affine::grouping::group_witness+0x167>
  1d6a2d:	lea    0x18(%rsp),%rdi
  1d6a32:	mov    %r15,%rsi
  1d6a35:	mov    %r14,%rdx
  1d6a38:	call   2317f0 <zopro_core::msm_affine::grouping::scalar_groups>
  1d6a3d:	cmpq   $0xffffffffffffffff,0x18(%rsp)
  1d6a43:	je     1d6a97 <zopro_core::msm_affine::grouping::group_witness+0x167>
  1d6a45:	vmovups 0x50(%rsp),%zmm1
  1d6a50:	vmovups 0x18(%rsp),%zmm0
--
  241749:	call   *0x2cf261(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  24174f:	test   %bpl,%bpl
  241752:	mov    0x10(%rsp),%rbp
  241757:	je     241804 <zopro_core::msm_g2+0xf84>
  24175d:	lea    0x40(%rsp),%rdi
  241762:	mov    %r14,%rsi
  241765:	mov    %rbx,%rdx
  241768:	call   2317f0 <zopro_core::msm_affine::grouping::scalar_groups>
  24176d:	cmpq   $0xffffffffffffffff,0x40(%rsp)
  241773:	je     241804 <zopro_core::msm_g2+0xf84>
  241779:	vmovdqu64 0x78(%rsp),%zmm1
  241784:	vmovdqu64 0x40(%rsp),%zmm0
--
  474d57:	call   *0x9bc53(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  474d5d:	test   %r13b,%r13b
  474d60:	mov    0x20(%rsp),%r13
  474d65:	je     475063 <zopro_core::msm_affine::g2_query_exclusive+0x423>
  474d6b:	lea    0x110(%rsp),%rdi
  474d73:	mov    %r12,%rsi
  474d76:	mov    %r14,%rdx
  474d79:	call   2317f0 <zopro_core::msm_affine::grouping::scalar_groups>
  474d7e:	call   481d70 <zopro_core::boolean_phase_trace::group_end>
  474d83:	mov    0x28(%rsp),%r15
  474d88:	mov    0x110(%rsp),%rax
  474d90:	lea    -0x400(%r14),%rcx
--
  49c9e0:	call   *0x73fca(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  49c9e6:	mov    %r12d,%eax
  49c9e9:	test   %al,%al
  49c9eb:	je     49cbf6 <rayon_core::join::join_context::call_b::<(zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>, zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>), rayon_core::join::join::call<(zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>, zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>), <zopro_core::Key>::cpu_msms::{closure#3}>::{closure#0}>::{closure#0}+0x426>
  49c9f1:	lea    0x18(%rsp),%rdi
  49c9f6:	mov    %r15,%rsi
  49c9f9:	mov    %r14,%rdx
  49c9fc:	call   2317f0 <zopro_core::msm_affine::grouping::scalar_groups>
  49ca01:	mov    %fs:0xffffffffffffffc0,%rdx
  49ca0a:	lea    0x18(%rsp),%rax
  49ca0f:	mov    %rax,0x90(%rsp)
  49ca17:	mov    %rbx,0x98(%rsp)
--
  4b7828:	call   *0x59182(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  4b782e:	mov    %r13d,%eax
  4b7831:	test   %al,%al
  4b7833:	je     4b788d <rayon_core::join::join_context::call_b::<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>, rayon_core::join::join::call<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>, <zopro_core::Key>::solve_canonical_wires::{closure#3}>::{closure#0}>::{closure#0}+0x1dd>
  4b7835:	lea    0x10(%rsp),%rdi
  4b783a:	mov    %r15,%rsi
  4b783d:	mov    %r14,%rdx
  4b7840:	call   2317f0 <zopro_core::msm_affine::grouping::scalar_groups>
  4b7845:	mov    0x10(%rsp),%r13
  4b784a:	cmp    $0xffffffffffffffff,%r13
  4b784e:	je     4b788d <rayon_core::join::join_context::call_b::<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>, rayon_core::join::join::call<zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>, <zopro_core::Key>::solve_canonical_wires::{closure#3}>::{closure#0}>::{closure#0}+0x1dd>
  4b7850:	mov    0x7a0(%r12),%rbx
--
  4b7cc6:	call   *0x58ce4(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  4b7ccc:	test   %r12b,%r12b
  4b7ccf:	mov    0x20(%rsp),%r12
  4b7cd4:	je     4b81ad <<zopro_core::Key>::canonical_b2_query+0x72d>
  4b7cda:	lea    0x138(%rsp),%rdi
  4b7ce2:	mov    %r15,%rsi
  4b7ce5:	mov    %r14,%rdx
  4b7ce8:	call   2317f0 <zopro_core::msm_affine::grouping::scalar_groups>
  4b7ced:	call   481d70 <zopro_core::boolean_phase_trace::group_end>
  4b7cf2:	mov    0x30(%rsp),%r13
  4b7cf7:	mov    0x40(%rsp),%rbx
  4b7cfc:	mov    0x138(%rsp),%rax
--
  4c31c2:	call   *0x4d7e8(%rip)        # 5109b0 <free@GLIBC_2.2.5>
  4c31c8:	mov    %ebx,%eax
  4c31ca:	test   %al,%al
  4c31cc:	je     4c33d7 <rayon_core::join::join_context::call_b::<(zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>, zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>), rayon_core::join::join::call<(zopro_bn254::curve::Projective<zopro_bn254::curve::G1Config>, zopro_bn254::curve::Projective<zopro_bn254::curve::G2Config>), <zopro_core::Key>::solve_canonical_wires::{closure#5}>::{closure#0}>::{closure#0}+0x3c7>
  4c31d2:	lea    0x18(%rsp),%rdi
  4c31d7:	mov    %r15,%rsi
  4c31da:	mov    %r14,%rdx
  4c31dd:	call   2317f0 <zopro_core::msm_affine::grouping::scalar_groups>
  4c31e2:	mov    %fs:0xffffffffffffffc0,%rdx
  4c31eb:	lea    0x18(%rsp),%rax
  4c31f0:	mov    %rax,0x90(%rsp)
  4c31f8:	mov    %r12,0x98(%rsp)
