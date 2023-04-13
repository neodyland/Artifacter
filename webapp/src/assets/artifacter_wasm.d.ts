/* tslint:disable */
/* eslint-disable */

type Character = {
  cid: number,
  name: string,
  level: number,
  elementName: string,
  imageDataUrl: string
}

type UserProfile = {
  nickname: string,
  signature: string,
  achievement: number,
  level: number,
  worldLevel: number,
  towerFloorIndex: number,
  towerLevelIndex: number,
  namecard: string
}

/**
* Load
* @returns {Promise<any>}
*/
export function w_load(): Promise<any>;

/**
 * [nickname, signature(description), achievement, level, world_level, tower_floor_index, tower_level_index, namecard]
 */
type ArrayProfile = [
  string, // nickname
  string, // signature
  number, // achievement
  number, // level
  number, // world_level
  number, // tower_floor_index
  number, // tower_level_index
  string  // namecard
];

/**
* get user profile
* 
* [nickname, signature(description), achievement, level, world_level, tower_floor_index, tower_level_index, namecard]
* @param {number} uid
* @returns {Promise<ArrayProfile>}
*/
export function get_profile(uid: number): Promise<ArrayProfile>;



/**
 * [cid, name, level, 元素, 画像]
 */
type ArrayCharacter = [number, string, number, string, string]

/**
* [cid, name, level, 元素, 画像][]
* @param {number} uid
* @param {string} lang
* @returns {Promise<ArrayCharacter[]>}
*/
export function get_characters(uid: number, lang: string): Promise<ArrayCharacter[]>;
/**
* generater
* @param {number} uid
* @param {number} cid
* @param {string} lang
* @param {string} format
* @param {string} counter
* @returns {Promise<any>}
*/
export function generate(uid: number, cid: number, lang: string, format: string, counter: string): Promise<string>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly w_load: () => number;
  readonly get_profile: (a: number) => number;
  readonly get_characters: (a: number, b: number, c: number) => number;
  readonly generate: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly BrotliDecoderCreateInstance: (a: number, b: number, c: number) => number;
  readonly BrotliDecoderSetParameter: (a: number, b: number, c: number) => void;
  readonly BrotliDecoderDecompressPrealloc: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly BrotliDecoderDecompressWithReturnInfo: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly BrotliDecoderDecompress: (a: number, b: number, c: number, d: number) => number;
  readonly BrotliDecoderDecompressStream: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly BrotliDecoderDecompressStreaming: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly BrotliDecoderMallocU8: (a: number, b: number) => number;
  readonly BrotliDecoderFreeU8: (a: number, b: number, c: number) => void;
  readonly BrotliDecoderMallocUsize: (a: number, b: number) => number;
  readonly BrotliDecoderFreeUsize: (a: number, b: number, c: number) => void;
  readonly BrotliDecoderDestroyInstance: (a: number) => void;
  readonly BrotliDecoderHasMoreOutput: (a: number) => number;
  readonly BrotliDecoderTakeOutput: (a: number, b: number) => number;
  readonly BrotliDecoderIsUsed: (a: number) => number;
  readonly BrotliDecoderIsFinished: (a: number) => number;
  readonly BrotliDecoderGetErrorCode: (a: number) => number;
  readonly BrotliDecoderGetErrorString: (a: number) => number;
  readonly BrotliDecoderErrorString: (a: number) => number;
  readonly BrotliDecoderVersion: () => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1e9e4ca0e3075a47: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h274a6b6f781367d9: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init(module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
