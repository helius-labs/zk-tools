# Accepted borrowed G2 control-flow inspection

Exact accepted executable20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f. Symbol address46cdf0,sizecaee. Full objdump outputs remain local and will be hash-indexed.

Entry46cf1a jumps into the norm body46d414 with first index zero. The norm result goes through46defe/46df10 to46cf20 (or the capacity-growth path rejoins46cf20). Range46cf20..46d414 contains100 vpmadd52 sites and completes the prefix product before the loop comparison46d40e. It has no first-group bypass. Initial chain vectors are loaded from Montgomery-one constants at entry.

The reverse loop enters4738a0, gathers rows and chain records, and has no j==0 endpoint bypass. Field products are interleaved with coordinate operations; for example47461f..475b06 contains400IFMA sites, so that mixed region must not be labeled only factor/update multiplication. The final loop test at479256/479259 follows the final product-bearing block and returns at47386a. This supports testing endpoint removal; it is not a dynamic instruction count or exclusive cost measurement.

The source recurrence independently identifies the three proposed removals: one*firstnorm, one*lastinverse factor, and the final unused inverse update. Static inspection confirms retained work and absence of a dedicated shortcut, without assigning timing savings to individual instructions. A new compiled candidate and real-caller timings are required.
