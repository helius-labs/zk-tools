import { readFileSync } from "node:fs";
import { gunzipSync } from "node:zlib";
import { expect, it } from "vitest";
import { MERGE_SHAPES, TRANSFER_SHAPES } from "../src/shapes.js";

const root = new URL("../../../", import.meta.url);
const readJson = (path: string) => JSON.parse(readFileSync(new URL(path, root), "utf8"));

it("keeps benchmark fixtures aligned with supported shapes and pinned sources", () => {
  const bundle = JSON.parse(gunzipSync(readFileSync(new URL("fixtures/node-shapes.json.gz", root))).toString());
  expect(bundle.protocolRevision).toBe(readJson("key-catalog-source.json").revision);
  expect(bundle.runtimeRevision).toBe(readJson("runtime.lock.json").zopro.revision);
  const shapes = [
    ...TRANSFER_SHAPES.map((shape) => ({ ...shape, id: shape.label, circuitType: "transfer-confidential" })),
    ...MERGE_SHAPES.map((shape) => ({ ...shape, id: `merge-${shape.inputs}x1`, circuitType: "merge" })),
  ];
  expect(Object.keys(bundle.requests).sort()).toEqual(shapes.map((shape) => shape.id).sort());
  for (const shape of shapes) {
    const request = bundle.requests[shape.id];
    expect(request.circuitType, shape.id).toBe(shape.circuitType);
    expect(request.inputs, shape.id).toHaveLength(shape.inputs);
    if (shape.circuitType === "transfer-confidential") {
      expect(Number(request.nInputs), shape.id).toBe(shape.inputs);
      expect(Number(request.nOutputs), shape.id).toBe(shape.outputs);
      expect(request.outputs, shape.id).toHaveLength(shape.outputs);
    } else {
      expect(request.output, shape.id).toBeDefined();
    }
  }
});
