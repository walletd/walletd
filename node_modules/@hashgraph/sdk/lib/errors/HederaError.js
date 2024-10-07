"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HederaError = void 0;
const Status_1 = require("../Status");
/**
 * Class of errors for response codes returned from Hedera.
 * @deprecate This error is no longer in use in the sdk. `HederaStatusError` is used instead.
 */
class HederaError extends Error {
    constructor(code) {
        console.warn("`HederaError` has been renamed to `HederaStatusError`");
        const codeName = Status_1.Status._fromCode(code).toString();
        super();
        this.message = `Hedera returned response code: ${codeName} (${code})`;
        this.name = "HederaError";
        this.code = code;
        this.codeName = codeName;
    }
}
exports.HederaError = HederaError;
