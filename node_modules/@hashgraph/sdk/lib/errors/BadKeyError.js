"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BadKeyError = void 0;
class BadKeyError extends Error {
    constructor(msg) {
        super();
        this.message = msg ? msg : "Failed to parse correct key";
        this.name = "BadKeyError";
    }
}
exports.BadKeyError = BadKeyError;
