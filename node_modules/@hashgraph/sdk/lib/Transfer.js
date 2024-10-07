"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.transferFromProto = void 0;
const AccountId_1 = require("./account/AccountId");
const Hbar_1 = require("./Hbar");
function transferFromProto(accountAmount) {
    return {
        accountId: AccountId_1.AccountId._fromProto(accountAmount.getAccountid()),
        amount: Hbar_1.Hbar.fromTinybar(accountAmount.getAmount())
    };
}
exports.transferFromProto = transferFromProto;
