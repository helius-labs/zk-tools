import {
  ClientError,
  mergeProverRequestBody,
  proverRequestBody,
  type ProofService,
  type ProverRequestBody,
} from "@heliuslabs/spr/client";
import {
  composeSignal,
  expectedProvingKey,
  parseCheckedProof,
  requestError,
  type ExpectedProvingKey,
} from "@heliuslabs/spr/internal";
import type { WasmProver } from "./wasm-prover.js";

/** Bridge the pinned SDK's typed proof service directly to the WASM worker. */
export function createLocalProofService(
  prover: Pick<WasmProver, "proveRequest">,
): ProofService {
  async function prove(
    body: ProverRequestBody,
    key: ExpectedProvingKey,
    context: Parameters<ProofService["prove"]>[1],
  ) {
    const operation = composeSignal(context, "prove");
    try {
      operation.signal.throwIfAborted();
      const result = await prover.proveRequest(
        JSON.stringify(body),
        operation.signal,
      );
      operation.signal.throwIfAborted();
      return parseCheckedProof(JSON.parse(result.proof), key);
    } catch (error) {
      if (operation.signal.aborted) throw requestError("prove", operation);
      throw error;
    } finally {
      operation.cleanup();
    }
  }
  return {
    prove(inputs, context) {
      const body = proverRequestBody(inputs);
      if (
        inputs.circuit !== "transfer" ||
        body.circuitType !== "transfer-confidential"
      )
        throw new ClientError("CLIENT_PROVER_INPUT");
      return prove(
        body,
        expectedProvingKey({
          circuit: "transfer-confidential",
          nInputs: inputs.payload.inputs.length,
          nOutputs: inputs.payload.outputs.length,
        }),
        context,
      );
    },
    proveMerge(inputs, context) {
      if (inputs.ringProgramId !== 0n)
        throw new ClientError("CLIENT_PROVER_INPUT");
      return prove(
        mergeProverRequestBody(inputs),
        expectedProvingKey({
          circuit: "merge",
          nInputs: inputs.inputs.length,
        }),
        context,
      );
    },
  };
}
