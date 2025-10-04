// @generated file from wasmbuild -- do not edit
// @ts-nocheck: generated
// deno-lint-ignore-file
// deno-fmt-ignore-file

let wasm;
export function __wbg_set_wasm(val) {
  wasm = val;
}

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
  if (
    cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0
  ) {
    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8ArrayMemory0;
}

const lTextDecoder = typeof TextDecoder === "undefined"
  ? (0, module.require)("util").TextDecoder
  : TextDecoder;

let cachedTextDecoder = new lTextDecoder("utf-8", {
  ignoreBOM: true,
  fatal: true,
});

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
  numBytesDecoded += len;
  if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
    cachedTextDecoder = new lTextDecoder("utf-8", {
      ignoreBOM: true,
      fatal: true,
    });
    cachedTextDecoder.decode();
    numBytesDecoded = len;
  }
  return cachedTextDecoder.decode(
    getUint8ArrayMemory0().subarray(ptr, ptr + len),
  );
}

function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return decodeText(ptr, len);
}
/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add(a, b) {
  const ret = wasm.add(a, b);
  return ret;
}

const BoardFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) => wasm.__wbg_board_free(ptr >>> 0, 1));

export class Board {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    BoardFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_board_free(ptr, 0);
  }
  constructor() {
    const ret = wasm.board_new();
    this.__wbg_ptr = ret >>> 0;
    BoardFinalization.register(this, this.__wbg_ptr, this);
    return this;
  }
  /**
   * @param {number} x
   * @param {number} y
   * @param {number} value
   */
  set(x, y, value) {
    wasm.board_set(this.__wbg_ptr, x, y, value);
  }
  /**
   * @param {number} x
   * @param {number} y
   * @returns {number}
   */
  get(x, y) {
    const ret = wasm.board_get(this.__wbg_ptr, x, y);
    return ret;
  }
  print_board() {
    wasm.board_print_board(this.__wbg_ptr);
  }
}

export function __wbg_wbindgenthrow_681185b504fabc8e(arg0, arg1) {
  throw new Error(getStringFromWasm0(arg0, arg1));
}

export function __wbindgen_init_externref_table() {
  const table = wasm.__wbindgen_export_0;
  const offset = table.grow(4);
  table.set(0, undefined);
  table.set(offset + 0, undefined);
  table.set(offset + 1, null);
  table.set(offset + 2, true);
  table.set(offset + 3, false);
}
