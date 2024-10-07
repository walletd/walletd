"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MaxPaymentExceededError = void 0;
// @deprecate This error is no longer in use in the sdk. Use `MaxQueryPaymentExceededError` instead.
class MaxPaymentExceededError extends Error {
    constructor(queryCost, maxQueryCost) {
        console.warn("`MaxPaymentExceededError` has been renamed to `MaxQueryPaymentExceededError`");
        super();
        this.message = `query cost of ${queryCost.value()} HBAR exceeds max set on client: ${maxQueryCost.value()} HBAR`;
        this.name = "MaxPaymentExceededError";
        this.queryCost = queryCost;
    }
}
exports.MaxPaymentExceededError = MaxPaymentExceededError;
