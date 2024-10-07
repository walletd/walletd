"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HederaPrecheckStatusError = void 0;
const HederaStatusError_1 = require("./HederaStatusError");
const Status_1 = require("../Status");
/**
 * Error returned when precheck fails with a bad status code
 */
class HederaPrecheckStatusError extends HederaStatusError_1.HederaStatusError {
    constructor(status, transactionId) {
        super(status);
        this.transactionId = transactionId;
        this.name = "HederaPrecheckStatusError";
        this.message = `Transaction ${this.transactionId} failed with status: ${this.status}`;
    }
    static _throwIfError(code, transactionId) {
        const status = Status_1.Status._fromCode(code);
        if (status._isError()) {
            throw new HederaPrecheckStatusError(status, transactionId);
        }
    }
}
exports.HederaPrecheckStatusError = HederaPrecheckStatusError;
