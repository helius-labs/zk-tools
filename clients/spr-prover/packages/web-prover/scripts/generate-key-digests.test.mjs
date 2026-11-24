import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFile, mkdir, mkdtemp, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { test } from "node:test";

const packageRoot = fileURLToPath(new URL("../", import.meta.url));
const clientRoot = fileURLToPath(new URL("../../../", import.meta.url));

test("key generation works inside a pnpm store under node_modules", async () => {
  const temporary = await mkdtemp(join(tmpdir(), "spr-key-generator-"));
  try {
    const manifest = JSON.parse(await readFile(join(packageRoot, "package.json"), "utf8"));
    const command = /^node (\S+)$/u.exec(manifest.scripts["generate:keys"]);
    assert.ok(command, "generate:keys runs a standalone Node script");
    const script = command[1];
    const client = join(temporary, "node_modules/.bin/store/git-checkout/clients/spr-prover");
    const pkg = join(client, "packages/web-prover");
    await mkdir(dirname(join(pkg, script)), { recursive: true });
    await mkdir(join(pkg, "src"));
    await copyFile(join(packageRoot, script), join(pkg, script));
    await copyFile(join(clientRoot, "zopro-keys.lock"), join(client, "zopro-keys.lock"));
    const result = spawnSync(process.execPath, [join(pkg, script)], { encoding: "utf8", cwd: pkg });
    assert.equal(result.status, 0, result.stderr);
    const generated = await readFile(join(pkg, "src/key-digests.ts"), "utf8");
    const lock = JSON.parse(await readFile(join(client, "zopro-keys.lock"), "utf8"));
    assert.ok(generated.includes(`export const KEY_SET = "${lock.prefix.split("/").at(-1)}"`));
    for (const [name, key] of Object.entries(lock.keys)) {
      assert.ok(generated.includes(`"${name}":`), name);
      assert.ok(generated.includes(`keySha256: "${key.key_sha256}"`), name);
    }
  } finally {
    await rm(temporary, { recursive: true, force: true });
  }
});
