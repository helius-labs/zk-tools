# Signed G2 component alias screen

**No additional component reuse was found on either retained query.** After contracting full scalars up to sign, every nonzero Phi2 component magnitude is unique within its original point-range partition. The declared necessary novelty gate is false. No native implementation, timing capture or adoption follows from this screen; production remains unchanged.

The question came from Barretenberg's allowance for grouping equal working scalars after GLV. Its [pinned review map](https://github.com/AztecProtocol/aztec-packages/blob/3c7386eb7bbddec89bb6548982583eb65bb563bc/barretenberg/cpp/src/barretenberg/ecc/scalar_multiplication/pippenger_rewrite_review_map.md) explicitly discusses duplicate halves that need not imply duplicate original scalars. This is a structural idea to examine, not a transferred speed claim. No upstream runtime code was copied.

Zopro currently groups full scalars before Phi2. This screen first contracts equal full scalars modulo sign within each of the two original point-range parts, then looks for additional equal component magnitudes. This comparison deliberately gives ordinary scalar grouping more reuse than production may admit. It separates new component reuse from existing full-scalar reuse; it is not an implemented dispatch change or a universal claim about other witnesses.

|Query|Raw rows|Active rows|Signed full-scalar classes, two parts|Additional reusable component classes|Net additional generic adds saved|
|---|---:|---:|---:|---:|---:|
|1x2 B2|30,089|28,489|13,156 + 13,332|0|0|
|2x2 B2|52,274|49,511|20,698 + 23,388|0|0|

The diagnostic signed contraction merges three more full-scalar classes than exact equality on primary and eight more on 2x2. These are full-scalar sign relationships, not new component collisions. They do not establish a useful sign-grouping implementation or a measured latency gain.

For a component class of `m` points active in `w` Booth windows, one shared signed point sum costs `m−1` generic additions and removes `(m−1)w` bucket arrivals. The necessary net saving is `(m−1)(w−1)`. One-window classes are left alone. The fixed screen retains width12, eleven component windows and two original point-range parts, with no cross-part or cross-component grouping. It excludes zero scalars and input identities. [Design](artifacts/design.md), [counts](artifacts/analysis.json), [independent review](artifacts/review.json).

Three new tests pass once: a constructed additional-component collision with opposite signs; zero/unit/partial-size and cancellation cases; and accepted Phi2 reconstruction on scalar edges and changing values. All 82,363 retained raw scalars reconstruct with the accepted Fr arithmetic and lambda. Two small-prime additive token groups independently check every signed weighted bucket before and after component grouping. These are linear-algebra checks, not elliptic-curve or proof oracles. Existing field/core/prover qualification is inherited, not rerun.

A separate reviewer uses manual modulus subtraction for sign normalization, sorted adjacent scalar/component runs and per-bit Booth extraction. It imports no count-model code and reproduces every histogram and count, verifies all192 input/source/tool/library pins, all176 unchanged production files, and the four successful Rust/xtask commands with stream hashes. The copied Phi2 arithmetic is shared with the model and production; it is not a new independent decomposition implementation. No build, test, helper or workflow failure occurred and no successful run was repeated. [Workflow](artifacts/plan.json), [pins](artifacts/pins.json), [sources](artifacts/study.rs), [review source](artifacts/review.rs), [receipts](artifacts/receipts/result.json).

No new latency, CPU profile, allocation, RSS, minimum-RAM, pressure, concurrent-throughput, AWS/Aeglos or large250/4999 result is claimed. A future component-grouping runtime would still need metadata, derived points, reduction scratch, safe borrowed lifetimes and resource admission; this screen does not qualify those costs. There are no new SDK requests, proof exports or Gnark verifications. The canonical unique-proof ledger remains29,953 at the preceding finalization/drain study. Input scalars, point arrays, keys, images, witnesses and executable/library blobs are not included.

The following are the unchanged accepted local32-worker observations against the supplied historical M5 references, not measurements from this screen. Hardware differs. The [latest full-SDK experiment](../2026-11-05-finalize-drain-bundle176/README.md) has a separately reported primary gain and failed full adoption gate; it does not replace these accepted numbers.

|Shape|Accepted CPU ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37|4.667×|24|3.027×|3.7|
|2x2|11.151932|59|5.291×|30|2.690×|5.9|
|4x2|16.086386|109|6.776×|48|2.984×|10.9|
|8x2|24.179453|189|7.817×|74|3.060×|18.9|
|12x2|33.884455|284|8.381×|101|2.981×|28.4|
|16x2|40.077381|335|8.359×|124|3.094×|33.5|
|24x2|65.850777|528|8.018×|176|2.673×|52.8|
|merge24x1|65.060937|507|7.793×|175|2.690×|50.7|

Accepted merge54x1 remains161.883899ms without a supplied M5 reference. The all-eight verified CPU10× goal remains active and unmet,0/8. This closes only the extra component-alias opportunity on these two retained inputs; it does not rule out every grouping algorithm or different scalar distribution. The next experiment must remove other useful work rather than rerun this zero-opportunity case.
