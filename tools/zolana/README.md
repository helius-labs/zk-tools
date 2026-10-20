# Zolana's zopro keys

Zolana's gnark keys for zopro, from zolana's prover at the release the keys
come from (`./fetch-zolana.sh` checks it out into `zolana/`, which `go.mod`
builds against; its aeglos GPU files need a module this does not fetch, so
`go mod tidy -e`).

- `TestZoproDump` writes a Zolana gnark key's dump with its metadata (shape,
  and the sha256 of the key file, which proofs report)
  (`ZOPRO_DUMP=<dir> ZOPRO_KEYS=<gnark keys> ZOPRO_SHAPES=2x2,merge-24x1 go
  test -run TestZoproDump .`), with `ZOPRO_REQUESTS=<requests>` the
  request's witness too, and gnark's verdict on a raw proof zopro left in a
  shape's dump (`zopro-zolana`'s `tests/prove.rs` leaves one).
- `build-keys.sh <gnark proving-keys.lock> <out>` builds the zopro key of
  every confidential transfer and merge key of a key set and the
  `zopro-keys.lock` that pins them, beside the gnark key each was built
  from. The SDKs copy the lockfile and serve the keys from the key set's
  CDN folder.
- `fixtures/` holds a `/prove` request per shape (`transfer-<n>x2.json`,
  `merge-<n>x1.json`, written by zolana's `TestRequestFixture` with the
  shape changed) and `zopro-keys.lock` the key set they prove with; both
  feed `tools/bench.sh`.
