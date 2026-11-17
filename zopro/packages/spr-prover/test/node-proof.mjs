import { readFile } from "node:fs/promises";
import { init } from "@heliuslabs/zopro-spr";
import { keyBytes, metadata, requestPath } from "./key.js";
import { checkProofs } from "./proof-check.js";

console.log(
  JSON.stringify(
    await checkProofs(
      init,
      await keyBytes(),
      await readFile(requestPath, "utf8"),
      Number(process.argv[2]),
      metadata.key_sha256,
    ),
  ),
);
