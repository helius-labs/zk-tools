import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";

const lock = JSON.parse(
  await readFile(
    new URL("../../../tools/spr/zopro-keys.lock", import.meta.url),
    "utf8",
  ),
);
export const metadata = lock.keys["transfer_confidential_2_2.key"];
export const keyPath = new URL(`./keys/${metadata.file}`, import.meta.url);
export const requestPath = new URL(
  "../../../crates/zopro-spr/tests/fixtures/transfer-2x2.request.json",
  import.meta.url,
);

export async function keyBytes() {
  let bytes;
  try {
    bytes = await readFile(keyPath);
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    const response = await fetch(new URL(metadata.file, lock.baseUrl), {
      signal: AbortSignal.timeout(60_000),
    });
    if (!response.ok)
      throw new Error(`Key download failed (${response.status})`);
    bytes = Buffer.from(await response.arrayBuffer());
  }
  if (
    bytes.length !== metadata.size ||
    createHash("sha256").update(bytes).digest("hex") !== metadata.sha256
  ) {
    throw new Error(
      "Proving key size or SHA-256 does not match zopro-keys.lock",
    );
  }
  await mkdir(new URL("./keys/", import.meta.url), { recursive: true });
  await writeFile(keyPath, bytes);
  return bytes;
}
