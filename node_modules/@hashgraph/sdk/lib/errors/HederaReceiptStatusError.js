"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HederaReceiptStatusError = void 0;
const HederaStatusError_1 = require("./HederaStatusError");
const Status_1 = require("../Status");
/**
 * Error returned when the `TransactionReceipt` has a bad status code
 */
class HederaReceiptStatusError extends HederaStatusError_1.HederaStatusError {
    constructor(status, receipt, transactionId) {
        super(status);
        this.transactionId = transactionId;
        this.receipt = receipt;
        this.name = "HederaReceiptStatusError";
        this.message = `Received receipt for transaction ${this.transactionId} with exceptional status: ${this.status}`;
    }
    static _throwIfError(code, receipt, transactionId) {
        const status = Status_1.Status._fromCode(code);
        if (status._isError()) {
            throw new HederaReceiptStatusError(status, receipt, transactionId);
        }
    }
}
exports.HederaReceiptStatusError = HederaReceiptStatusError;
