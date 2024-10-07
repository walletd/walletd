"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HederaRecordStatusError = void 0;
const HederaStatusError_1 = require("./HederaStatusError");
const Status_1 = require("../Status");
/**
 * Error returned when the `TransactionRecord` has a bad status code
 */
class HederaRecordStatusError extends HederaStatusError_1.HederaStatusError {
    constructor(status, record, transactionId) {
        super(status);
        this.transactionId = transactionId;
        this.record = record;
        this.name = "HederaRecordStatusError";
        this.message = `Received record for transaction ${this.transactionId} with exceptional status: ${this.status}`;
    }
    static _throwIfError(code, record, transactionId) {
        const status = Status_1.Status._fromCode(code);
        if (status._isError()) {
            throw new HederaRecordStatusError(status, record, transactionId);
        }
    }
}
exports.HederaRecordStatusError = HederaRecordStatusError;
