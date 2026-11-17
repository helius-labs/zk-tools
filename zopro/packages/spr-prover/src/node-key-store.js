import { createHash, randomUUID } from "node:crypto";
import { mkdir, readFile, rename, rm, writeFile } from "node:fs/promises";
import { homedir } from "node:os";
import { join, resolve } from "node:path";

export function nodeKeyStore(options, keySet) {
  if (
    options.keyDirectory !== undefined &&
    (typeof options.keyDirectory !== "string" || !options.keyDirectory)
  ) {
    throw new TypeError("keyDirectory must be a nonempty directory path");
  }
  const base =
    process.platform === "darwin"
      ? join(homedir(), "Library", "Caches")
      : process.platform === "win32"
        ? (process.env.LOCALAPPDATA ?? join(homedir(), "AppData", "Local"))
        : (process.env.XDG_CACHE_HOME ?? join(homedir(), ".cache"));
  const directory = resolve(
    options.keyDirectory ?? join(base, "zopro-spr", keySet),
  );
  return {
    async read(artifact) {
      try {
        return await readFile(join(directory, artifact.file));
      } catch (error) {
        if (error.code !== "ENOENT") throw error;
      }
    },
    async write(artifact, bytes) {
      await mkdir(directory, { recursive: true });
      const path = join(directory, artifact.file);
      const temporary = `${path}.${randomUUID()}.part`;
      try {
        await writeFile(temporary, bytes);
        await rename(temporary, path);
      } finally {
        await rm(temporary, { force: true });
      }
    },
    sha256(bytes) {
      return createHash("sha256").update(bytes).digest("hex");
    },
  };
}
