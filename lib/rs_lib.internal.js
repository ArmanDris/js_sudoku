// @generated file from wasmbuild -- do not edit
// @ts-nocheck: generated
// deno-lint-ignore-file
// deno-fmt-ignore-file

let wasm;
export function __wbg_set_wasm(val) {
  wasm = val;
}

function addToExternrefTable0(obj) {
  const idx = wasm.__externref_table_alloc();
  wasm.__wbindgen_export_2.set(idx, obj);
  return idx;
}

function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    const idx = addToExternrefTable0(e);
    wasm.__wbindgen_exn_store(idx);
  }
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

function getArrayU8FromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function isLikeNone(x) {
  return x === undefined || x === null;
}

function _assertClass(instance, klass) {
  if (!(instance instanceof klass)) {
    throw new Error(`expected instance of ${klass.name}`);
  }
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
  if (
    cachedDataViewMemory0 === null ||
    cachedDataViewMemory0.buffer.detached === true ||
    (cachedDataViewMemory0.buffer.detached === undefined &&
      cachedDataViewMemory0.buffer !== wasm.memory.buffer)
  ) {
    cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
  }
  return cachedDataViewMemory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  const mem = getDataViewMemory0();
  const result = [];
  for (let i = ptr; i < ptr + 4 * len; i += 4) {
    result.push(wasm.__wbindgen_export_2.get(mem.getUint32(i, true)));
  }
  wasm.__externref_drop_slice(ptr, len);
  return result;
}
/**
 * @param {Board | null} [starting_board]
 * @param {DecisionStrategy | null} [decision_strategy]
 * @param {number | null} [desired_solutions]
 * @returns {Board[]}
 */
export function launch_algorithm_x(
  starting_board,
  decision_strategy,
  desired_solutions,
) {
  let ptr0 = 0;
  if (!isLikeNone(starting_board)) {
    _assertClass(starting_board, Board);
    ptr0 = starting_board.__destroy_into_raw();
  }
  const ret = wasm.launch_algorithm_x(
    ptr0,
    isLikeNone(decision_strategy) ? 2 : decision_strategy,
    isLikeNone(desired_solutions) ? 0x100000001 : desired_solutions >>> 0,
  );
  var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
  wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
  return v2;
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

/**
 * @enum {0 | 1}
 */
export const DecisionStrategy = Object.freeze({
  First: 0,
  "0": "First",
  Random: 1,
  "1": "Random",
});

const BoardFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) => wasm.__wbg_board_free(ptr >>> 0, 1));

export class Board {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(Board.prototype);
    obj.__wbg_ptr = ptr;
    BoardFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

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
   * @param {Board} board
   * @returns {Board}
   */
  static from_board(board) {
    _assertClass(board, Board);
    const ret = wasm.board_from_board(board.__wbg_ptr);
    return Board.__wrap(ret);
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

export function __wbg_board_new(arg0) {
  const ret = Board.__wrap(arg0);
  return ret;
}

export function __wbg_call_52af042a326d9b3a() {
  return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.call(arg1, arg2);
    return ret;
  }, arguments);
}

export function __wbg_call_90bf4b9978d51034() {
  return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
  }, arguments);
}

export function __wbg_crypto_574e78ad8b13b65f(arg0) {
  const ret = arg0.crypto;
  return ret;
}

export function __wbg_getRandomValues_b8f5dbd5f3995a9e() {
  return handleError(function (arg0, arg1) {
    arg0.getRandomValues(arg1);
  }, arguments);
}

export function __wbg_length_537fa63a6103cbdb(arg0) {
  const ret = arg0.length;
  return ret;
}

export function __wbg_msCrypto_a61aeb35a24c1329(arg0) {
  const ret = arg0.msCrypto;
  return ret;
}

export function __wbg_newnoargs_863941679b1933bb(arg0, arg1) {
  const ret = new Function(getStringFromWasm0(arg0, arg1));
  return ret;
}

export function __wbg_newwithlength_79dd8226b146df94(arg0) {
  const ret = new Uint8Array(arg0 >>> 0);
  return ret;
}

export function __wbg_node_905d3e251edff8a2(arg0) {
  const ret = arg0.node;
  return ret;
}

export function __wbg_process_dc0fbacc7c1c06f7(arg0) {
  const ret = arg0.process;
  return ret;
}

export function __wbg_prototypesetcall_a81ac58a5b6e988c(arg0, arg1, arg2) {
  Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
}

export function __wbg_randomFillSync_ac0988aba3254290() {
  return handleError(function (arg0, arg1) {
    arg0.randomFillSync(arg1);
  }, arguments);
}

export function __wbg_require_60cc747a6bc5215a() {
  return handleError(function () {
    const ret = module.require;
    return ret;
  }, arguments);
}

export function __wbg_static_accessor_GLOBAL_656a564fb01c5b63() {
  const ret = typeof global === "undefined" ? null : global;
  return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}

export function __wbg_static_accessor_GLOBAL_THIS_09a6cc4b9571ef65() {
  const ret = typeof globalThis === "undefined" ? null : globalThis;
  return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}

export function __wbg_static_accessor_SELF_36742aea97854d74() {
  const ret = typeof self === "undefined" ? null : self;
  return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}

export function __wbg_static_accessor_WINDOW_0ce0d90b0830e7e6() {
  const ret = typeof window === "undefined" ? null : window;
  return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}

export function __wbg_subarray_07c7c2b284d2102d(arg0, arg1, arg2) {
  const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
  return ret;
}

export function __wbg_versions_c01dfd4722a88165(arg0) {
  const ret = arg0.versions;
  return ret;
}

export function __wbg_wbindgenisfunction_27a5c72d80bbdf07(arg0) {
  const ret = typeof arg0 === "function";
  return ret;
}

export function __wbg_wbindgenisobject_bdb9aa7f2dd707ef(arg0) {
  const val = arg0;
  const ret = typeof val === "object" && val !== null;
  return ret;
}

export function __wbg_wbindgenisstring_55b63daa584dc807(arg0) {
  const ret = typeof arg0 === "string";
  return ret;
}

export function __wbg_wbindgenisundefined_2e902cd900cf5927(arg0) {
  const ret = arg0 === undefined;
  return ret;
}

export function __wbg_wbindgenthrow_681185b504fabc8e(arg0, arg1) {
  throw new Error(getStringFromWasm0(arg0, arg1));
}

export function __wbindgen_cast_2241b6af4c4b2941(arg0, arg1) {
  // Cast intrinsic for `Ref(String) -> Externref`.
  const ret = getStringFromWasm0(arg0, arg1);
  return ret;
}

export function __wbindgen_cast_cb9088102bce6b30(arg0, arg1) {
  // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
  const ret = getArrayU8FromWasm0(arg0, arg1);
  return ret;
}

export function __wbindgen_init_externref_table() {
  const table = wasm.__wbindgen_export_2;
  const offset = table.grow(4);
  table.set(0, undefined);
  table.set(offset + 0, undefined);
  table.set(offset + 1, null);
  table.set(offset + 2, true);
  table.set(offset + 3, false);
}
