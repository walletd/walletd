"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FreezeTransaction = void 0;
const TransactionBuilder_1 = require("./TransactionBuilder");
const Freeze_pb_1 = require("./generated/Freeze_pb");
const FreezeService_pb_service_1 = require("./generated/FreezeService_pb_service");
/**
 * Set the freezing period in which the platform will stop creating events and accepting
 * transactions. This is used before safely shut down the platform for maintenance.
 */
class FreezeTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        this._body = new Freeze_pb_1.FreezeTransactionBody();
        this._inner.setFreeze(this._body);
    }
    setStartTime(dateOrHour, maybeMinute) {
        let hour;
        let minute;
        if (typeof dateOrHour === "number" && maybeMinute != null) {
            hour = dateOrHour;
            minute = maybeMinute;
        }
        else {
            console.warn("passing `Date` is deprecated; pass the `hour` and `minute` as separate parameters");
            hour = dateOrHour.getHours();
            minute = dateOrHour.getMinutes();
        }
        this._body.setStarthour(hour);
        this._body.setStartmin(minute);
        return this;
    }
    setEndTime(dateOrHour, maybeMinute) {
        let hour;
        let minute;
        if (typeof dateOrHour === "number" && maybeMinute != null) {
            hour = dateOrHour;
            minute = maybeMinute;
        }
        else {
            console.warn("passing `Date` is deprecated; pass the `hour` and `minute` as separate parameters");
            hour = dateOrHour.getHours();
            minute = dateOrHour.getMinutes();
        }
        this._body.setEndhour(hour);
        this._body.setEndmin(minute);
        return this;
    }
    _doValidate( /* errors: string[] */) {
        // Do nothing
    }
    get _method() {
        return FreezeService_pb_service_1.FreezeService.freeze;
    }
}
exports.FreezeTransaction = FreezeTransaction;
