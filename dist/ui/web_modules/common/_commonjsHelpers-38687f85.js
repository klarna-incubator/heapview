var f=typeof globalThis!=="undefined"?globalThis:typeof window!=="undefined"?window:typeof global!=="undefined"?global:typeof self!=="undefined"?self:{};function g(c,d,a){return a={path:d,exports:{},require:function(e,b){return h(e,b===void 0||b===null?a.path:b)}},c(a,a.exports),a.exports}function h(){throw new Error("Dynamic requires are not currently supported by @rollup/plugin-commonjs")}export{f as a,g as c};