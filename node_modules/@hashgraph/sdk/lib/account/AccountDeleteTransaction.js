"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AccountDeleteTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const CryptoService_pb_service_1 = require("../generated/CryptoService_pb_service");
const CryptoDelete_pb_1 = require("../generated/CryptoDelete_pb");
const AccountId_1 = require("./AccountId");
/**
 * Mark an account as deleted, moving all its current hbars to another account. It will remain in
 * the ledger, marked as deleted, until it expires. Transfers into it a deleted account fail. But
 * a deleted account can still have its expiration extended in the normal way.
 */
class AccountDeleteTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        const body = new CryptoDelete_pb_1.CryptoDeleteTransactionBody();
        this._body = body;
        this._inner.setCryptodelete(body);
    }
    /**
     * Sets the account to delete. Note: To successfully delete an account
     * one must also manually set the `TransactionId` to a `TransactionId`
     * constructed from the same `AccountId`
     *
     * The account ID which should be deleted.
     */
    setDeleteAccountId(accountId) {
        this._body.setDeleteaccountid(new AccountId_1.AccountId(accountId)._toProto());
        return this;
    }
    /**
     * The account ID which will receive all remaining hbars.
     */
    setTransferAccountId(accountId) {
        this._body.setTransferaccountid(new AccountId_1.AccountId(accountId)._toProto());
        return this;
    }
    get _method() {
        return CryptoService_pb_service_1.CryptoService.cryptoDelete;
    }
    _doValidate(errors) {
        if (!this._body.hasDeleteaccountid()) {
            errors.push("AccountDeleteTransaction requires .setAccountid()");
        }
    }
}
exports.AccountDeleteTransaction = AccountDeleteTransaction;
