import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { test } from "node:test";
import { keyBytes, keyPath, metadata, requestPath } from "./key.js";

const exec = promisify(execFile);

test(
  "packed package proves from a separate Node consumer and exits",
  { timeout: 90_000 },
  async () => {
    await keyBytes();
    const directory = await mkdtemp(join(tmpdir(), "zopro-consumer-"));
    try {
      const { stdout } = await exec(
        "npm",
        ["pack", "--ignore-scripts", "--json", "--pack-destination", directory],
        { cwd: fileURLToPath(new URL("../", import.meta.url)) },
      );
      const [{ filename }] = JSON.parse(stdout);
      await writeFile(
        join(directory, "package.json"),
        JSON.stringify({ private: true, type: "module" }),
      );
      await exec(
        "npm",
        [
          "install",
          "--offline",
          "--ignore-scripts",
          "--no-audit",
          "--no-fund",
          join(directory, filename),
        ],
        { cwd: directory },
      );
      const code = `
      import { readFile } from 'node:fs/promises';
      import { init } from '@heliuslabs/zopro-spr';
      import { checkProofs } from ${JSON.stringify(new URL("./proof-check.js", import.meta.url).href)};
      console.log(JSON.stringify(await checkProofs(init,
        await readFile(${JSON.stringify(fileURLToPath(keyPath))}),
        await readFile(${JSON.stringify(fileURLToPath(requestPath))}, 'utf8'),
        2, ${JSON.stringify(metadata.key_sha256)})));
    `;
      const proved = await exec(
        process.execPath,
        ["--input-type=module", "--eval", code],
        { cwd: directory, timeout: 60_000 },
      );
      assert.equal(JSON.parse(proved.stdout.trim()).threads, 2);
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
  },
);
