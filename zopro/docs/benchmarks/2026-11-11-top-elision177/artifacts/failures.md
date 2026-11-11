The first separate reader stopped on exact equality between an independently
summed integer sample mean and a parsed JSON decimal: 3794855.8235294116 versus
3794855.823529411 ns. Its source is retained as review-original.rs. The correction
allows only JSON floating-point roundtrip error (0.00001 ns for means, 1e-14 for
ratios, 1e-12 ms for differences). Raw integer samples, recorded results, protocol
and measurement are unchanged. No capture or successful native build was repeated.
