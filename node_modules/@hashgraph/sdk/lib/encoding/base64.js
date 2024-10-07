"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.decode = void 0;
function decode(text) {
    if (typeof window !== "undefined") {
        return Uint8Array.from(window.atob(text), (c) => c.charCodeAt(0));
    }
    return Buffer.from(text, "base64");
}
exports.decode = decode;
