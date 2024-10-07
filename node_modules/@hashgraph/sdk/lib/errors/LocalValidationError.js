"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.LocalValidationError = void 0;
class LocalValidationError extends Error {
    constructor(className, errors) {
        super();
        this.message = `${className} failed validation:\n${errors.join("\n")}`;
        this.name = "ValidationError";
    }
}
exports.LocalValidationError = LocalValidationError;
