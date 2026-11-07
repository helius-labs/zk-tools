# CPU usage scope correction

The SDK driver samples process CPU usage immediately before and after the observed request loop. Proof export happens afterward. The report's sentence saying the CPU summaries include export work is incorrect: they include the request loop and its bookkeeping, and exclude subsequent proof-file export.

No timing, CPU-time number, proof count, confidence interval or selection result changes. The original report and evidence archive remain unchanged as published. This correction follows inspection of the byte-identical driver reused in the larger canonical-wire experiment.
