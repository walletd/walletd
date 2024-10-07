"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AccountBalanceQuery = void 0;
const QueryBuilder_1 = require("../QueryBuilder");
const CryptoService_pb_service_1 = require("../generated/CryptoService_pb_service");
const QueryHeader_pb_1 = require("../generated/QueryHeader_pb");
const Hbar_1 = require("../Hbar");
const AccountId_1 = require("./AccountId");
const CryptoGetAccountBalance_pb_1 = require("../generated/CryptoGetAccountBalance_pb");
/**
 * Get the balance of a cryptocurrency account. This returns only the balance, so it is a smaller
 * and faster reply than CryptoGetInfo, which returns the balance plus additional information.
 */
class AccountBalanceQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new CryptoGetAccountBalance_pb_1.CryptoGetAccountBalanceQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setCryptogetaccountbalance(this._builder);
    }
    /**
     * The account ID for which information is requested.
     */
    setAccountId(id) {
        this._builder.setAccountid(new AccountId_1.AccountId(id)._toProto());
        return this;
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasAccountid()) {
            errors.push("`.setAccountId()` required");
        }
    }
    _getMethod() {
        return CryptoService_pb_service_1.CryptoService.cryptoGetBalance;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getCryptogetaccountbalance().getHeader();
    }
    _mapResponse(response) {
        const accountBalance = response.getCryptogetaccountbalance();
        return Hbar_1.Hbar.fromTinybar(accountBalance.getBalance());
    }
    _isPaymentRequired() {
        return false;
    }
}
exports.AccountBalanceQuery = AccountBalanceQuery;
