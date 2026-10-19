# The gnark side of zopro

Offline tools over gnark, from zolana's prover at the release the keys come
from (`./fetch-zolana.sh` checks it out into `zolana/`, which `go.mod`
builds against; its aeglos GPU files need a module this does not fetch, so
`go mod tidy -e`).

- `TestZoproDump` writes a gnark proving key's parts for `zopro-bench pack`
  (`ZOPRO_DUMP=<dir> ZOPRO_KEYS=<gnark keys> ZOPRO_SHAPES=2x2,merge-24x1 go
  test -run TestZoproDump .`), with `ZOPRO_REQUESTS=<requests>` the
  request's witness too, and gnark's verdict on a raw proof zopro left in a
  shape's dump (`zopro-prover`'s `tests/prove.rs` leaves one).
- `build-keys.sh <gnark proving-keys.lock> <out>` builds the zopro key of
  every confidential transfer and merge key of a key set and the
  `zopro-keys.lock` that pins them, beside the gnark key each was built
  from. The SDKs copy the lockfile and serve the keys from the key set's
  CDN folder.
