(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/shortcode_web.js":
/*!*******************************!*\
  !*** ../pkg/shortcode_web.js ***!
  \*******************************/
/*! exports provided: lookup */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./shortcode_web_bg.wasm */ \"../pkg/shortcode_web_bg.wasm\");\n/* harmony import */ var _shortcode_web_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./shortcode_web_bg.js */ \"../pkg/shortcode_web_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"lookup\", function() { return _shortcode_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"lookup\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/shortcode_web.js?");

/***/ }),

/***/ "../pkg/shortcode_web_bg.js":
/*!**********************************!*\
  !*** ../pkg/shortcode_web_bg.js ***!
  \**********************************/
/*! exports provided: lookup */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"lookup\", function() { return lookup; });\n/* harmony import */ var _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./shortcode_web_bg.wasm */ \"../pkg/shortcode_web_bg.wasm\");\n\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {string} s\n* @returns {string | undefined}\n*/\nfunction lookup(s) {\n    try {\n        const retptr = _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n        _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n        var ptr0 = passStringToWasm0(s, _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"lookup\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        let v1;\n        if (r0 !== 0) {\n            v1 = getStringFromWasm0(r0, r1).slice();\n            _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 1);\n        }\n        return v1;\n    } finally {\n        _shortcode_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n    }\n}\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/shortcode_web_bg.js?");

/***/ }),

/***/ "../pkg/shortcode_web_bg.wasm":
/*!************************************!*\
  !*** ../pkg/shortcode_web_bg.wasm ***!
  \************************************/
/*! exports provided: memory, lookup, __wbindgen_export_0, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/shortcode_web_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var shortcode_web__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! shortcode-web */ \"../pkg/shortcode_web.js\");\n\n\nwindow.shortcode_lookup = shortcode_web__WEBPACK_IMPORTED_MODULE_0__[\"lookup\"];\n\nconst input = document.getElementById(\"lookup\");\nconst output = document.getElementById(\"output\");\nconst sample_invalid_input = document.getElementById(\"sample\");\n\n// https://stackoverflow.com/questions/1349404/generate-random-string-characters-in-javascript\nfunction makeid(length) {\n   var result           = '';\n   var characters       = 'abcdefghijklmnopqrstuvwxyz__';\n   var charactersLength = characters.length;\n   for ( var i = 0; i < length; i++ ) {\n      result += characters.charAt(Math.floor(Math.random() * charactersLength));\n   }\n   return result;\n}\n\n// if the hash function didn't have false positives, this would loop forever.\n// good thing it doesn't!\nwhile (true) {\n    const id = makeid(10);\n    const out = shortcode_lookup(id);\n    if (out) {\n        sample_invalid_input.innerHTML = `<code>:${id}:</code> maps to ${out}`\n        break;\n    }\n}\n\n// https://stackoverflow.com/a/26156806\nfunction trimChar(string, charToRemove) {\n    while(string.charAt(0)==charToRemove) {\n        string = string.substring(1);\n    }\n\n    while(string.charAt(string.length-1)==charToRemove) {\n        string = string.substring(0,string.length-1);\n    }\n\n    return string;\n}\n\ninput.onkeydown = function(e) {\n    if (e.key == \"Enter\") {\n        const val = trimChar(e.target.value, ':');\n        const ret = shortcode_web__WEBPACK_IMPORTED_MODULE_0__[\"lookup\"](val);\n\n        let txt = \"\";\n        if (!ret) {\n            txt = `<p><code>:${val}:</code> is not a valid shortcode</p>`;\n        } else {\n            txt = `<p>mapped <code>:${val}:</code> to ${ret}</p>`;\n        }\n\n        e.target.value = \"\";\n        output.innerHTML = txt + output.innerHTML;\n    }\n}\n\n\n//# sourceURL=webpack:///./index.js?");

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