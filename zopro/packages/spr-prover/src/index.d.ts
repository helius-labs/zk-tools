export interface ProofResult {
  readonly proof: string;
  readonly witnessMs: number;
  readonly proveMs: number;
  readonly verifyMs: number;
}

export interface SprProver {
  name(): string;
  digest(): string;
  inputs(): number;
  outputs(): number;
  prove(request: string): Promise<ProofResult>;
  verify(request: string, proof: string): boolean;
  export_raw(): Uint8Array;
  free(): void;
}

export interface Runtime {
  readonly SprProver: new (key: Uint8Array) => SprProver;
  readonly threads: number;
  /** Free loaded provers before closing. Start a new worker to use another runtime. */
  close(): Promise<void>;
}

/** Once per worker; threads: 0 (default) selects WASM without shared memory. */
export function init(options?: { readonly threads?: number }): Promise<Runtime>;
