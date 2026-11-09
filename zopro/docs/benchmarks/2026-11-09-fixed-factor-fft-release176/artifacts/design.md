# Release consumer qualification before measurement

Use the exact 178-source bounded FFT qualification snapshot and its qualified
normal-release field. Build OFF and ON cores with debug assertions/overflow
checks disabled; omit all fixed-FFT diagnostics. Only the fixed-FFT compile flag
differs. Link the same new consumer harness into each arm, with fat LTO. Link a
third arm to the frozen accepted core and original field as a calibration
reference. No timing follows until all three actual binaries qualify.

New Arkworks sparse inputs (seed 283+domain+workers, changed terms/coefficient
values) check full coset and Euler outputs at domains 32768/65536/262144 and
1/16/32 workers, 512KiB stacks. Each arm compares 2,162,688 outputs. This does
not replay the parent model/native/complete-transform population. Decode the
same OFF/ON consumer ELF that executed those oracles. Verify diagnostic code
is absent and review actual dif_serial/dit_serial bodies before freezing a
matched trial; retain code-size/stack-operand observations as static evidence.

The CLI supports later bench/profile work, but no performance selection or
capture recipe is authorized by this qualification alone. Freeze counts,
contexts, order, calibration/nonregression/gain gates and profiler scope after
review and before any measurement. Actual cloned inputs and factor expansion
must remain in the timed complete Euler call. Plan setup is reported separately
where the existing key preparation is outside repeated proofs. A process
per-context resource/profile result must state its scope. No isolated-factor
timing, speed extrapolation, subset rescue or reroll. Any adoption still requires
verified SDK all8+54, independent Gnark validation, resources and portability.
