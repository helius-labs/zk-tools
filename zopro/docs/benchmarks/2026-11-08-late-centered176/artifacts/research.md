# Reading and history scope

Fresh primary reading: Hurchalla's28April2022 positive-inverse REDC article,
https://jeffhurchalla.com/2022/04/28/montgomery-redc-using-the-positive-inverse-mod-r/,
especially alternative algorithm and low-half borrow discussion. Its native-word
advantages do not predict multiprecision IFMA speed. No C/assembly body is ported.
Montgomery1985, three-page primary paper, sections1–2:
https://langevin.univ-tln.fr/cours/MLC/extra/montgomery.pdf.
The new local recipe uses additive negative-inverse cancellation with the last
digit centered. It does not claim to reproduce an external benchmark or novel
Montgomery arithmetic. Full papers/external source bodies are not redistributed.

History read before selection: Winograd16 actual caller just closed16%slower;
common-Z SDK unchanged8msprimary; whole-field census and streaming census;
signed-pair52 model/native (signed product factorization, output below2p);
signed/loose complex square (local differences, not an entire signed affine path);
mul-sub SDK; lazy-Y field/Window; lazy-linear; scaled-R260 screen; handoff confirms
its native and constant variants are already closed20%/9%slower, despite those
older public directories not being present in this checkout. Do not reopen them.
Uniform-prefix scale and coordinate isomorphism are distinct old negatives.

Current full-SDK sampled accepted profile, common-z-sdk176/diagnostic-analysis.json:
G1stream2465/12246samples, G2borrowed1979, Fqinverse865, G2/G1reducers705/634.
These are aggregate CPU samples, not critical-path fractions or predicted savings.
No new capture/decoder is performed. Host key/bucket format remains canonicalR256;
late-centered model also checksR260 only as a field representation cross-check.

Search also revisited Hrabar/Waterloo2026, SRE, RNS/PMNS and existing overlap/
scheduler trials. They are prior work, not new discoveries. No external timing
is transferred, no unchanged search/kernel/observer rerun is selected.
