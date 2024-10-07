"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AccountRecordsQuery = void 0;
const QueryBuilder_1 = require("../QueryBuilder");
const CryptoService_pb_service_1 = require("../generated/CryptoService_pb_service");
const QueryHeader_pb_1 = require("../generated/QueryHeader_pb");
const AccountId_1 = require("./AccountId");
const CryptoGetAccountRecords_pb_1 = require("../generated/CryptoGetAccountRecords_pb");
const TransactionRecord_1 = require("../TransactionRecord");
/**
 * Get all the records for an account for any transfers into it and out of it, that were above the
 * threshold, during the last 25 hours.
 */
class AccountRecordsQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new CryptoGetAccountRecords_pb_1.CryptoGetAccountRecordsQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setCryptogetaccountrecords(this._builder);
    }
    /**
     * The account that this record is for.
     */
    setAccountId(accountId) {
        this._builder.setAccountid(new AccountId_1.AccountId(accountId)._toProto());
        return this;
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasAccountid()) {
            errors.push("`.setAccountId()` required");
        }
    }
    _getMethod() {
        return CryptoService_pb_service_1.CryptoService.getAccountRecords;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getCryptogetaccountrecords().getHeader();
    }
    _mapResponse(response) {
        const accountInfo = response.getCryptogetaccountrecords();
        return accountInfo.getRecordsList().map(TransactionRecord_1.TransactionRecord._fromProto);
    }
}
exports.AccountRecordsQuery = AccountRecordsQuery;
