The first three core builds succeeded. The first final-consumer build failed
because the existing Arkworks dependency libraries lacked bitcode for fat LTO.
The successful cores were retained. Before any caller execution or capture, an
untimed Arkworks helper generated complete canonical input hashes and reference
MSM outputs. The final driver regenerates its native inputs and checks every
point/scalar via that hash. It uses the exact frozen production-compatible JSON
and SHA libraries for LTO, with Arkworks outside the timed executable. Only the
three remaining consumer builds were repaired; no core or field rebuild or
successful science replay was needed. The initial driver and failed receipt
remain retained. Protocol/analysis/review readers are frozen after preparation
and before any preflight. Profile length1024 was selected prospectively, before
any caller clock or sample observation; there is no extension after capture.
