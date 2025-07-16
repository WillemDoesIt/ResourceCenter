/* tslint:disable */
/* eslint-disable */
export function main(): Promise<void>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: () => void;
  readonly rust_sqlite_wasm_shim_localtime_js: (a: bigint, b: number) => void;
  readonly rust_sqlite_wasm_shim_tzset_js: (a: number, b: number, c: number, d: number) => void;
  readonly rust_sqlite_wasm_shim_emscripten_get_now: () => number;
  readonly rust_sqlite_wasm_shim_wasi_random_get: (a: number, b: number) => number;
  readonly rust_sqlite_wasm_shim_exit: (a: number) => void;
  readonly rust_sqlite_wasm_shim_abort_js: () => void;
  readonly rust_sqlite_wasm_shim_malloc: (a: number) => number;
  readonly rust_sqlite_wasm_shim_free: (a: number) => void;
  readonly rust_sqlite_wasm_shim_realloc: (a: number, b: number) => number;
  readonly rust_sqlite_wasm_shim_calloc: (a: number, b: number) => number;
  readonly sqlite3_os_init: () => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_5: WebAssembly.Table;
  readonly _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd619b19651a02435: (a: number, b: number) => void;
  readonly closure586_externref_shim: (a: number, b: number, c: any) => void;
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
