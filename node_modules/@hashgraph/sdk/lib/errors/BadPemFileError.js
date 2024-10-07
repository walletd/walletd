"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BadPemFileError = void 0;
class BadPemFileError extends Error {
    constructor() {
        super();
        this.message = "Failed to parse .pem file";
        this.name = "BadPemFileError";
    }
}
exports.BadPemFileError = BadPemFileError;
