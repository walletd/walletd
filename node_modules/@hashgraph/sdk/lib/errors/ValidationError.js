"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ValidationError = void 0;
// @deprecate This error is no longer in use in the sdk. Use `LocalValidationError` instead.
class ValidationError extends Error {
    constructor(className, errors) {
        console.warn("`ValidationError` has been renamed to `LocalValidationError`");
        super();
        this.message = `${className} failed validation:\n${errors.join("\n")}`;
        this.name = "ValidationError";
    }
}
exports.ValidationError = ValidationError;
