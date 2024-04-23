var Module = (function() {
    var _scriptDir = typeof document !== 'undefined' && document.currentScript ? document.currentScript.src : undefined;
    if (typeof __filename !== 'undefined') _scriptDir || (_scriptDir = __filename);
    return function() {
        var moduleArg = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : {};
        var Module = moduleArg;
        var readyPromiseResolve, readyPromiseReject;
        Module["ready"] = new Promise(function(resolve, reject) {
            readyPromiseResolve = resolve;
            readyPromiseReject = reject;
        });
        "use strict";
        return moduleArg;
    };
})()();
if (typeof exports === 'object' && typeof module === 'object') module.exports = Module;
else if (typeof define === 'function' && define['amd']) define([], function() {
    return Module;
});
