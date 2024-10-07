"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.KeyMismatchError = void 0;
const hex = require("@stablelib/hex");
class KeyMismatchError extends Error {
    constructor(hmac, expectedHmac) {
        super("key mismatch when loading from keystore");
        this.name = "KeyMismatchError";
        this._hmac = hex.encode(hmac, true);
        this._expectedHmac = hex.encode(expectedHmac, true);
    }
}
exports.KeyMismatchError = KeyMismatchError;
