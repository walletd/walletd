"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HederaStatusError = void 0;
/**
 * Class of errors for response codes returned from Hedera.
 */
class HederaStatusError extends Error {
    constructor(status) {
        super();
        this.message = `Hedera returned response code: ${status.toString()}`;
        this.status = status;
        this.name = "HederaStatusError";
    }
}
exports.HederaStatusError = HederaStatusError;
