import { readFile } from "node:fs/promises";
import { init } from "@heliuslabs/zopro-spr";
import {
  keyBytes,
  metadata,
  requestPath,
  mergeMetadata,
  mergeRequestPath,
} from "./key.js";
import { checkProofs } from "./proof-check.js";

const merge = process.argv[3] === "merge";
console.log(
  JSON.stringify(
    await checkProofs(
      init,
      await keyBytes(merge ? "merge_8_1.key" : "transfer_confidential_2_2.key"),
      await readFile(merge ? mergeRequestPath : requestPath, "utf8"),
      Number(process.argv[2]),
      (merge ? mergeMetadata : metadata).key_sha256,
      merge,
    ),
  ),
);
