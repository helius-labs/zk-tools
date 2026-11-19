# Scalar composition at equal grouped point counts

The same 6×2 witness and 5×16 witness used in the full-proof intervention supplied these canonical scalar arrays. Each of the six queries repeated identically over three priming proofs and one measured proof. All class sizes were positive and their sums matched the independently recorded nonzero point counts. The 6×2 split reconstructs the exact extra multiplicity from the published intervention. Both new exports verified.

| Shape | Query by base count | Layout | Terms | Values over 128 bits, with multiplicity | Most copies of one value | Nonzero digits, canonical / GLV | Same next bucket, canonical ordinary / striped / expected shuffled |
|---|---|---|---:|---:|---:|---:|---:|
| 6x2 | G1-A | ordinary | 23,576 | 23,552 | 1 | 514,213 / 517,615 | 7,394 / 7,219 / 7,273 |
| 6x2 | G1-A | split | 34,543 | 34,519 | 5 | 753,709 / 758,608 | 246,890 / 10,773 / 10,704 |
| 5x16 | G1-A | ordinary | 34,543 | 34,520 | 1 | 753,630 / 758,638 | 10,726 / 10,747 / 10,668 |
| 6x2 | G1-K | ordinary | 35,425 | 35,373 | 1 | 772,496 / 777,756 | 11,026 / 10,806 / 10,883 |
| 6x2 | G1-K | split | 51,983 | 51,931 | 10 | 1,133,978 / 1,141,512 | 372,508 / 15,952 / 16,029 |
| 5x16 | G1-K | ordinary | 51,983 | 51,936 | 1 | 1,134,009 / 1,141,695 | 15,906 / 16,054 / 15,998 |
| 6x2 | G2 | ordinary | 35,323 | 35,299 | 1 | 770,609 / 775,754 | 10,993 / 10,794 / 10,860 |
| 6x2 | G2 | split | 51,707 | 51,683 | 10 | 1,128,290 / 1,135,672 | 368,674 / 15,868 / 15,944 |
| 5x16 | G2 | ordinary | 51,707 | 51,684 | 1 | 1,128,284 / 1,135,837 | 15,821 / 16,006 / 15,922 |

G1-A denotes the smaller original G1 query and G1-K the larger. The last column sums adjacent equal bucket magnitudes over the nonzero entries of each width-12 window. The alternate traversal takes every 32nd term, then advances the starting offset. The random-order expectation holds each window’s digit multiset fixed. GLV counts treat the two component streams separately. Neither model measures field arithmetic time. [Full counts and both digit models](artifacts/counts.json) · [Reader](artifacts/weights.rs) · [Input and executable digests](artifacts/pins.json) · [Verification](artifacts/verification.json)
