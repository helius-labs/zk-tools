# Interpretation of the configured memory cases

All eight cases passed with zero high/max/OOM/kill events. Cold cgroup peaks
were 60.17–62.97 MiB; warmed peaks including the 32 MiB ballast were
102.12–105.34 MiB. All SDKs used 32 workers. No memory.high throttling was
observed, so this does not establish successful operation through an active
reclaim/throttling episode or a minimum-RAM requirement. These short primary
cases and their cgroup accounting are distinct from full performance runs'
process high-water RSS. They establish correctness under their configured
limits only. No claim of a candidate memory saving follows from these values.

The full performance gate remains false solely for physical-16 identical-
baseline calibration. No latency rerun, changed threshold, adoption or new
AWS/Aeglos/phone/giant/swap comparison is made by this study. The full all-eight
CPU and Metal comparison remains the preceding qualification's measurements.
