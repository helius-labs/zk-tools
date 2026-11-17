import assert from "node:assert/strict";
import { createServer } from "node:http";
import { mkdtemp, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { after, before, test } from "node:test";
import { chromium, firefox, webkit } from "playwright";
import { build } from "vite";
import {
  keyBytes,
  metadata,
  requestPath,
  mergeMetadata,
  mergeRequestPath,
} from "./key.js";

const root = resolve(fileURLToPath(new URL("../", import.meta.url)));
let output;

before(async () => {
  output = await mkdtemp(join(tmpdir(), "zopro-browser-"));
  await build({
    configFile: false,
    root,
    logLevel: "warn",
    worker: { format: "es" },
    build: {
      outDir: output,
      emptyOutDir: true,
      rolldownOptions: { input: resolve(root, "test/browser-page.html") },
    },
  });
});
after(async () => {
  if (output) await rm(output, { recursive: true, force: true });
});

async function server(isolated) {
  const key = await keyBytes();
  const request = await readFile(requestPath);
  const http = createServer(async (req, res) => {
    if (isolated) {
      res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
      res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
    }
    const pathname = new URL(req.url, "http://localhost").pathname;
    if (pathname === "/") {
      res.setHeader("Content-Type", "text/html");
      res.end(await readFile(join(output, "test/browser-page.html")));
      return;
    }
    if (pathname === "/key.zopro") {
      res.end(key);
      return;
    }
    if (pathname === "/request.json") {
      res.end(request);
      return;
    }
    if (pathname === "/merge-key.zopro") {
      res.end(await keyBytes("merge_8_1.key"));
      return;
    }
    if (pathname === "/merge-request.json") {
      res.end(await readFile(mergeRequestPath));
      return;
    }
    const path = resolve(output, `.${decodeURIComponent(pathname)}`);
    if (!path.startsWith(output + sep)) {
      res.writeHead(403).end();
      return;
    }
    try {
      res.setHeader(
        "Content-Type",
        path.endsWith(".wasm") ? "application/wasm" : "text/javascript",
      );
      res.end(await readFile(path));
    } catch {
      res.writeHead(404).end();
    }
  });
  await new Promise((ready) => http.listen(0, "127.0.0.1", ready));
  return {
    url: `http://127.0.0.1:${http.address().port}`,
    close: async () => {
      await new Promise((done) => http.close(done));
    },
  };
}

async function prove(page, threads, noCrypto = false, merge = false) {
  return page.evaluate((options) => globalThis.proveInWorker(options), {
    threads,
    keyDigest: (merge ? mergeMetadata : metadata).key_sha256,
    noCrypto,
    merge,
  });
}

for (const [name, engine] of Object.entries({ chromium, firefox, webkit })) {
  test(`${name}: browser worker proofs`, { timeout: 300_000 }, async (t) => {
    const browser = await engine.launch();
    t.after(() => browser.close());
    for (const isolated of [true, false]) {
      const host = await server(isolated);
      try {
        const page = await browser.newPage();
        await page.goto(host.url);
        assert.equal(await page.evaluate(() => crossOriginIsolated), isolated);
        for (const threads of isolated ? [0, 1, 2, 4] : [0]) {
          await t.test(`${threads} threads; isolated=${isolated}`, async () => {
            const response = await prove(page, threads);
            assert.equal(response.error, undefined);
            assert.equal(response.result.threads, threads);
          });
        }
        if (isolated) {
          await t.test("SDK proveMerge with an 8-input key", async () => {
            const response = await prove(page, 4, false, true);
            assert.equal(response.error, undefined);
            assert.equal(response.result.threads, 4);
          });
        }
        if (!isolated) {
          await t.test(
            "threaded startup refuses missing isolation",
            async () => {
              const response = await prove(page, 2);
              assert.match(response.error, /cross-origin isolation/);
            },
          );
          await t.test(
            "single-thread proving fails without cryptographic randomness",
            async () => {
              const response = await prove(page, 0, true);
              assert.equal(response.result, undefined);
              assert.ok(response.error);
            },
          );
        }
        await page.close();
      } finally {
        await host.close();
      }
    }
  });
}
