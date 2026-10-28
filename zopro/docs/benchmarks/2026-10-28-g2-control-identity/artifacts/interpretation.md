
## Interpretation and limits

The original frozen control measures42.212ms/MSM and OFF61.549ms atmode0; equal-length hardlink aliases measure42.202ms and61.561ms. OFF/frozen paired median is1.45816 in original paths and1.45667 through aliases. Label-only ratios are1.00050 for frozen and0.99991 for OFF. Alias/original ratios are1.00045 and0.99942. Thus the large gap persists across the tested relabeling and path changes; those interventions do not recover it.

Retired instructions remain approximately574.55million/MSM across all six cases. Frozen/alias-frozen take about228.6–228.7million cycles; OFF/alias-OFF about336.4–336.5million. The cycle/CPU-time quotient is similar (roughly5.41–5.47GHz), so a simple lower-frequency account is inconsistent with these observations. Genericcache misses and branch misses are also similar, but they cannot rule out instruction/frontend, cache-level, address-aliasing or other dependency effects.

The six changed Rust panic-line fields and buildID remain associated with each executable, as do its backing inode and mapped code pages. Program headers also match by read-only inspection. This study does not isolate inode identity, physical placement, virtual layout or diagnostic metadata as the cause. FIFO control paths vary with case labels and are part of the process environment, so equal executable-path lengths do not establish equal complete process layout. No unsupported causal or proof-speed inference is made.

The next distinct intervention is fresh byte-exact copies of both controls, with equal-length names and one shared FIFO pathname, followed by relevant frontend counters if available. This can test whether the discrepancy follows original backing or original ELF contents. It is a control diagnostic, not an optimization gate rerun or adoption. Keep the borrowed-queue study's failed gate, its6.81% physical16 improvement over original frozen and its requested-memory saving intact.
