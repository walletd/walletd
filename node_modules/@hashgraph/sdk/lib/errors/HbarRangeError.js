"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HbarRangeError = void 0;
class HbarRangeError extends Error {
    constructor(amount) {
        super();
        this.message = `Hbar amount out of range: ${amount.toString()}`;
        this.name = "HbarRangeError";
        this.amount = amount;
    }
}
exports.HbarRangeError = HbarRangeError;
