import { randomUUID } from "node:crypto";
import { mkdir, readFile, rename, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { digest } from "./records.mjs";

export async function keyBytes(lock, key, directory) {
  const artifact = lock.keys[key];
  const path = join(directory, artifact.file);
  let bytes;
  let downloaded = false;
  try {
    bytes = await readFile(path);
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    const response = await fetch(new URL(artifact.file, lock.baseUrl), {
      signal: AbortSignal.timeout(180_000),
    });
    if (!response.ok)
      throw new Error(
        `Key download failed (${response.status}): ${artifact.file}`,
      );
    bytes = new Uint8Array(await response.arrayBuffer());
    downloaded = true;
  }
  if (bytes.length !== artifact.size || digest(bytes) !== artifact.sha256)
    throw new Error(`Key size or SHA-256 mismatch: ${artifact.file}`);
  if (downloaded) {
    await mkdir(directory, { recursive: true });
    const temporary = `${path}.${randomUUID()}.part`;
    try {
      await writeFile(temporary, bytes);
      await rename(temporary, path);
    } finally {
      await rm(temporary, { force: true });
    }
  }
  return bytes;
}
