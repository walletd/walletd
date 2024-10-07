"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TinybarValueError = void 0;
const bignumber_js_1 = require("bignumber.js");
const Hbar_1 = require("../Hbar");
// @deprecate This error is no longer in use in the sdk. Use `HbarRangeError` instead.
class TinybarValueError extends Error {
    constructor(message, amount) {
        console.warn("`TinybarValueError` has been renamed to `HbarRangeError`");
        let bnAmount;
        if (amount instanceof Hbar_1.Hbar) {
            bnAmount = amount.asTinybar();
        }
        else if (amount instanceof bignumber_js_1.default) {
            bnAmount = amount;
        }
        else {
            bnAmount = new bignumber_js_1.default(amount);
        }
        super();
        this.message = `${message}: ${bnAmount.toString()}`;
        this.name = "TinybarValueError";
        this.amount = bnAmount;
    }
}
exports.TinybarValueError = TinybarValueError;
