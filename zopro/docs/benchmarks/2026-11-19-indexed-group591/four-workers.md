# Direct A-query inputs on four Ryzen workers

The experimental prover can read the A scalars through the solver's existing wire indices and prepare the grouped representatives and exact residuals before the FFT. The key still supplies the point groups. Every proof reads the current witness. No request-specific values survive the proof.

During this correctness check the process had four pinned logical CPUs. Each of three inputs ran three warmups and one observed proof. On the original and input 57 the diagnostic independently gathered every A scalar, ran the full dense A MSM, normalized both results, and compared the affine points on each call. All eight comparisons matched. Scattered input 139 took the ordinary fallback on all four calls. Each process exported a fresh proof with the expected public outputs, and the independent verifier accepted all three proofs using the original key.

The original dense input to A contains 190,734 field elements, or 6,103,488 bytes. The indexed path assembles 33,394 canonical representatives and 4,064 correction points and scalars, with 1,458,752 bytes of payload. Its 12,481 temporary representative choices occupy a further 99,848 bytes. Both paths also use shared key storage and MSM scratch. The diagnostic adds a dense oracle, so its elapsed time and RSS do not quantify the benefit of removing the dense input in ordinary operation.

[Source change](artifacts/source.patch) · [Independent verification and path counts](artifacts/four-workers.json)
