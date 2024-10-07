"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MaxQueryPaymentExceededError = void 0;
class MaxQueryPaymentExceededError extends Error {
    constructor(queryCost, maxQueryPayment) {
        super();
        this.message = `query cost of ${queryCost.value()} HBAR exceeds max set on client: ${maxQueryPayment.value()} HBAR`;
        this.name = "MaxQueryPaymentExceededError";
        this.queryCost = queryCost;
        this.maxQueryPayment = maxQueryPayment;
    }
}
exports.MaxQueryPaymentExceededError = MaxQueryPaymentExceededError;
