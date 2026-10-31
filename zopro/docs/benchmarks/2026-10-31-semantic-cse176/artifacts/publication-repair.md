# Publication bookkeeping repair

The first publisher invocation exited 101 at its relative-link assertion for
SHA256SUMS. It checked links before writing that file. No test, census, separate
reader, source pin or performance measurement failed or was repeated.

The original publisher source is preserved as publish-attempt-1.rs and its
partial output remains local in publication-attempt-1/. The corrected publisher
writes checksums before checking report links. It also publishes this note and
the original publisher source. Input and measurement artifacts are unchanged.
