(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/shortcode_web.js":
/*!*******************************!*\
  !*** ../pkg/shortcode_web.js ***!
  \*******************************/
/*! exports provided: shortcode, Char, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./shortcode_web_bg.wasm */ \"../pkg/shortcode_web_bg.wasm\");\n/* harmony import */ var _shortcode_web_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./shortcode_web_bg.js */ \"../pkg/shortcode_web_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"shortcode\", function() { return _shortcode_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"shortcode\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Char\", function() { return _shortcode_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Char\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _shortcode_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/shortcode_web.js?");

/***/ }),

/***/ "../pkg/shortcode_web_bg.js":
/*!**********************************!*\
  !*** ../pkg/shortcode_web_bg.js ***!
  \**********************************/
/*! exports provided: shortcode, Char, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"shortcode\", function() { return shortcode; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Char\", function() { return Char; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./shortcode_web_bg.wasm */ \"../pkg/shortcode_web_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n/**\n* @param {string} s\n* @returns {Char}\n*/\nfunction shortcode(s) {\n    var ptr0 = passStringToWasm0(s, _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    var ret = _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"shortcode\"](ptr0, len0);\n    return Char.__wrap(ret);\n}\n\n/**\n*/\nclass Char {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Char.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_char_free\"](ptr);\n    }\n    /**\n    * @returns {boolean}\n    */\n    get found() {\n        var ret = _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_char_found\"](this.ptr);\n        return ret !== 0;\n    }\n    /**\n    * @param {boolean} arg0\n    */\n    set found(arg0) {\n        _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_char_found\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {string | undefined}\n    */\n    get first() {\n        var ret = _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_char_first\"](this.ptr);\n        return ret === 0xFFFFFF ? undefined : String.fromCodePoint(ret);\n    }\n    /**\n    * @param {string | undefined} arg0\n    */\n    set first(arg0) {\n        _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_char_first\"](this.ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0.codePointAt(0));\n    }\n    /**\n    * @returns {string | undefined}\n    */\n    get second() {\n        var ret = _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_char_second\"](this.ptr);\n        return ret === 0xFFFFFF ? undefined : String.fromCodePoint(ret);\n    }\n    /**\n    * @param {string | undefined} arg0\n    */\n    set second(arg0) {\n        _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_char_second\"](this.ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0.codePointAt(0));\n    }\n    /**\n    * @returns {string | undefined}\n    */\n    get third() {\n        var ret = _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_char_third\"](this.ptr);\n        return ret === 0xFFFFFF ? undefined : String.fromCodePoint(ret);\n    }\n    /**\n    * @param {string | undefined} arg0\n    */\n    set third(arg0) {\n        _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_char_third\"](this.ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0.codePointAt(0));\n    }\n}\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/shortcode_web_bg.js?");

/***/ }),

/***/ "../pkg/shortcode_web_bg.wasm":
/*!************************************!*\
  !*** ../pkg/shortcode_web_bg.wasm ***!
  \************************************/
/*! exports provided: memory, __wbg_char_free, __wbg_get_char_found, __wbg_set_char_found, __wbg_get_char_first, __wbg_set_char_first, __wbg_get_char_second, __wbg_set_char_second, __wbg_get_char_third, __wbg_set_char_third, shortcode, __wbindgen_malloc, __wbindgen_realloc */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./shortcode_web_bg.js */ \"../pkg/shortcode_web_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/shortcode_web_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var shortcode_web__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! shortcode-web */ \"../pkg/shortcode_web.js\");\n\n\nconst input = document.getElementById(\"lookup\");\n\ninput.onkeydown = function(e) {\n    if (e.key == \"Enter\") {\n        const ret = shortcode_web__WEBPACK_IMPORTED_MODULE_0__[\"shortcode\"](e.target.value);\n\n        console.log(ret)\n\n        let txt = document.createElement(\"p\");\n        if (!ret.found) {\n            txt.innerText = \"not found\";\n        } else {\n            txt.innerText = [ret.first, ret.second, ret.third].join('');\n        }\n\n        e.target.value = \"\";\n        document.body.appendChild(txt);\n    }\n}\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);