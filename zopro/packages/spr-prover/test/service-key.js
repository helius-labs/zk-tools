import { createHash, randomUUID } from "node:crypto";
import { mkdir, readFile, rename, rm, writeFile } from "node:fs/promises";
import catalog from "../src/proving-keys.json" with { type: "json" };

export const productionKey = "transfer_confidential_1_2.key";
export const productionArtifact = catalog.keys[productionKey];
export const productionUrl = new URL(productionArtifact.file, catalog.baseUrl)
  .href;

export async function productionKeyBytes() {
  const path = new URL(
    `./keys/production-78751f5d1c591777/${productionArtifact.file}`,
    import.meta.url,
  );
  let bytes;
  let downloaded = false;
  try {
    bytes = await readFile(path);
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    const response = await fetch(productionUrl, {
      signal: AbortSignal.timeout(120_000),
    });
    if (!response.ok)
      throw new Error(
        `Could not download production service fixture (${response.status})`,
      );
    bytes = new Uint8Array(await response.arrayBuffer());
    downloaded = true;
  }
  if (
    bytes.length !== productionArtifact.size ||
    createHash("sha256").update(bytes).digest("hex") !==
      productionArtifact.sha256
  ) {
    throw new Error("Production service fixture checksum mismatch");
  }
  if (downloaded) {
    await mkdir(
      new URL("./keys/production-78751f5d1c591777/", import.meta.url),
      { recursive: true },
    );
    const temporary = new URL(`${path.href}.${randomUUID()}.part`);
    try {
      await writeFile(temporary, bytes);
      await rename(temporary, path);
    } finally {
      await rm(temporary, { force: true });
    }
  }
  return bytes;
}
