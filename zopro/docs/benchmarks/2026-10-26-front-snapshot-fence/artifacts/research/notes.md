# Ordering sources

Read the portable C11 algorithm and barrier discussion in Figure1/Section5 of
[Lê, Pop, Cohen, Zappa Nardelli (PPoPP2013)](https://fzn.fr/readings/ppopp13.pdf)
and [Rust's atomic fence documentation](https://doc.rust-lang.org/std/sync/atomic/fn.fence.html).
Exact downloaded PDF/HTML hashes are frozen in source-pins.json. The paper PDF
and full documentation HTML remain local; only links/hashes/our review enter Git.
Local pdftotext is unavailable, so the paper was read through the web PDF text.

These sources explain the single-victim front/fence/back requirement and the
role of the owner's barrier. They do not present this bounded multi-victim API.
Using a common explicit fence after all saved fronts is our inference, checked
against actual source and bounded instrumented models. It does not permit
removing the fence or treating all victims as an atomic global snapshot.

History searches of archived README/design/handoff text and the actual10/25
batch-guard implementation distinguish that earlier version: its fronts were
loaded one by one with a separate fence on each subsequent victim. Its original
negative result is preserved. Rejected10/26 retained buffers are not composed.
