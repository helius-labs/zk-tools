import assert from "node:assert/strict";
import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { test } from "node:test";
import { chromium, firefox, webkit } from "playwright";
import { keyBytes, metadata, requestPath } from "./key.js";

const root = resolve(fileURLToPath(new URL("../", import.meta.url)));

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
      res.end("<!doctype html><title>WASM proving test</title>");
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
    const path = resolve(root, `.${decodeURIComponent(pathname)}`);
    if (!path.startsWith(root + sep)) {
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
    close: () => new Promise((done) => http.close(done)),
  };
}

async function prove(page, threads, noCrypto = false) {
  return page.evaluate(
    ({ threads, keyDigest, noCrypto }) =>
      new Promise((resolve, reject) => {
        const worker = new Worker("/test/browser-proof.js", { type: "module" });
        const timeout = setTimeout(() => {
          worker.terminate();
          reject(new Error("proof timed out"));
        }, 60_000);
        worker.onmessage = ({ data }) => {
          clearTimeout(timeout);
          worker.terminate();
          resolve(data);
        };
        worker.onerror = (event) => {
          clearTimeout(timeout);
          worker.terminate();
          reject(new Error(event.message));
        };
        worker.postMessage({ threads, keyDigest, noCrypto });
      }),
    { threads, keyDigest: metadata.key_sha256, noCrypto },
  );
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
