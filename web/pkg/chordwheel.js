let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let cachedFloat64ArrayMemory0 = null;

function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}
/**
 * Radii of the wheel drawing, so the page can tell the rings apart.
 * @returns {Float64Array}
 */
export function ring_radii() {
    const ret = wasm.ring_radii();
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

const WheelFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wheel_free(ptr >>> 0, 1));

export class Wheel {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WheelFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wheel_free(ptr, 0);
    }
    /**
     * @param {number} d
     * @returns {Uint8Array}
     */
    chord_midi(d) {
        const ret = wasm.wheel_chord_midi(this.__wbg_ptr, d);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {string}
     */
    guitar_svg() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wheel_guitar_svg(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} d
     */
    set degree(d) {
        wasm.wheel_set_degree(this.__wbg_ptr, d);
    }
    /**
     * @param {number} i
     */
    set family(i) {
        wasm.wheel_set_family(this.__wbg_ptr, i);
    }
    /**
     * @param {boolean} on
     */
    set fifths(on) {
        wasm.wheel_set_fifths(this.__wbg_ptr, on);
    }
    /**
     * 0 = sharps, 1 = flats, 2 = follow the key.
     * @returns {number}
     */
    get accidentals() {
        const ret = wasm.wheel_accidentals(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {string}
     */
    keyboard_svg() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wheel_keyboard_svg(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {boolean} on
     */
    set sevenths(on) {
        wasm.wheel_set_sevenths(this.__wbg_ptr, on);
    }
    /**
     * @param {number} i
     */
    set accidentals(i) {
        wasm.wheel_set_accidentals(this.__wbg_ptr, i);
    }
    /**
     * Pick a mode while keeping its tonic, e.g. D Dorian to D Phrygian.
     * @param {number} d
     */
    set_mode_keep_tonic(d) {
        wasm.wheel_set_mode_keep_tonic(this.__wbg_ptr, d);
    }
    /**
     * @returns {number}
     */
    get key() {
        const ret = wasm.wheel_key(this.__wbg_ptr);
        return ret;
    }
    constructor() {
        const ret = wasm.wheel_new();
        this.__wbg_ptr = ret >>> 0;
        WheelFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Pitch class of the selected mode's tonic.
     * @returns {number}
     */
    tonic() {
        const ret = wasm.wheel_tonic(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get degree() {
        const ret = wasm.wheel_degree(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * 0 = major, 1 = melodic minor. Switching keeps the selected mode's tonic.
     * @returns {number}
     */
    get family() {
        const ret = wasm.wheel_family(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get fifths() {
        const ret = wasm.wheel_fifths(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Turn the inner wheel by whole slots (positive is clockwise).
     * @param {number} steps
     */
    rotate(steps) {
        wasm.wheel_rotate(this.__wbg_ptr, steps);
    }
    /**
     * @param {number} pc
     * @returns {string}
     */
    name_of(pc) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wheel_name_of(this.__wbg_ptr, pc);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} pc
     */
    set key(pc) {
        wasm.wheel_set_key(this.__wbg_ptr, pc);
    }
    /**
     * @returns {boolean}
     */
    get sevenths() {
        const ret = wasm.wheel_sevenths(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Everything the info panel shows, as JSON.
     * @returns {string}
     */
    info_json() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wheel_info_json(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Turn the wheel so the selected mode starts on pitch class `pc`.
     * @param {number} pc
     */
    set_tonic(pc) {
        wasm.wheel_set_tonic(this.__wbg_ptr, pc);
    }
    /**
     * @returns {string}
     */
    staff_svg() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wheel_staff_svg(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * The wheels as SVG; `drag` turns the inner wheel by extra degrees.
     * @param {number} drag
     * @returns {string}
     */
    wheel_svg(drag) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wheel_wheel_svg(this.__wbg_ptr, drag);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_0;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedFloat64ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('chordwheel_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
