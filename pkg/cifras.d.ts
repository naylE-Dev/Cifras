/* tslint:disable */
/* eslint-disable */

export class AesResultado {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly ciphertext: Uint8Array;
    readonly nonce: Uint8Array;
}

export function aes_wasm(texto: string, chave: Uint8Array): AesResultado;

export function caesar_wasm(texto: string, chave: number): string;

export function decifrar_aes_wasm(texto: Uint8Array, nonce: Uint8Array, chave: Uint8Array): string;

export function hash_wasm(texto: string): Uint8Array;

export function xor_wasm(texto: Uint8Array, chave: Uint8Array): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_aesresultado_free: (a: number, b: number) => void;
    readonly aes_wasm: (a: number, b: number, c: number, d: number) => number;
    readonly aesresultado_ciphertext: (a: number) => [number, number];
    readonly aesresultado_nonce: (a: number) => [number, number];
    readonly caesar_wasm: (a: number, b: number, c: number) => [number, number];
    readonly decifrar_aes_wasm: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly hash_wasm: (a: number, b: number) => [number, number];
    readonly xor_wasm: (a: number, b: number, c: number, d: number) => [number, number];
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
