function fields(value) {
  if (Array.isArray(value)) return value.map(fields);
  if (value && typeof value === "object")
    return Object.fromEntries(
      Object.entries(value).map(([key, item]) => [key, fields(item)]),
    );
  return typeof value === "string" && value.startsWith("0x")
    ? BigInt(value)
    : value;
}

function rename(value, names) {
  return Object.fromEntries(
    Object.entries(value).map(([key, item]) => [names[key] ?? key, item]),
  );
}

export function sdkInputs(request) {
  const { circuitType, nInputs, nOutputs, ...payload } = fields(
    JSON.parse(request),
  );
  if (circuitType !== "transfer-confidential")
    throw new Error("Expected the confidential transfer fixture");
  return {
    circuit: "transfer",
    payload: {
      ...rename(payload, {
        signerPkHashes: "signerPublicKeyHashes",
        publishedOutputOwnerPkHashes: "publishedOutputOwnerPublicKeyHashes",
      }),
      inputs: payload.inputs.map((input) =>
        rename(input, { utxo: "circuit", ownerPkHash: "ownerPublicKeyHash" }),
      ),
      outputs: payload.outputs.map((output) =>
        rename(output, {
          utxo: "circuit",
          ownerPkHash: "ownerPublicKeyHash",
          nullifierPk: "nullifierPublicKey",
        }),
      ),
    },
  };
}

export function sdkMergeInputs(request) {
  const { circuitType, ...payload } = fields(JSON.parse(request));
  if (circuitType !== "merge") throw new Error("Expected the merge fixture");
  const circuit = (values) => ({
    domain: 0n,
    owner: 0n,
    asset: 0n,
    amount: 0n,
    blinding: 0n,
    dataHash: 0n,
    ringDataHash: 0n,
    ringProgramId: 0n,
    ...values,
  });
  return {
    ...rename(payload, {
      ownerPkHash: "ownerPublicKeyHash",
      userNullifierPk: "userNullifierPublicKey",
    }),
    inputs: payload.inputs.map((input) => ({
      ...input,
      circuit: circuit({
        domain: input.domain,
        amount: input.amount,
        blinding: input.blinding,
        ringDataHash: input.ringDataHash,
      }),
      isDummy: 0n,
      ownerPublicKeyHash: payload.ownerPkHash,
    })),
    output: {
      ...payload.output,
      circuit: circuit({ ringDataHash: payload.output.ringDataHash }),
      isDummy: 0n,
      ownerPublicKeyHash: payload.ownerPkHash,
      nullifierPublicKey: payload.userNullifierPk,
    },
  };
}
