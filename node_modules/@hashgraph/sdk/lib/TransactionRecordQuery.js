"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.TransactionRecordQuery = void 0;
const QueryBuilder_1 = require("./QueryBuilder");
const TransactionGetRecord_pb_1 = require("./generated/TransactionGetRecord_pb");
const QueryHeader_pb_1 = require("./generated/QueryHeader_pb");
const TransactionId_1 = require("./TransactionId");
const CryptoService_pb_service_1 = require("./generated/CryptoService_pb_service");
const TransactionRecord_1 = require("./TransactionRecord");
const ResponseHeader_pb_1 = require("./generated/ResponseHeader_pb");
const HederaReceiptStatusError_1 = require("./errors/HederaReceiptStatusError");
const HederaRecordStatusError_1 = require("./errors/HederaRecordStatusError");
/**
 * Get the record for a transaction. If the transaction requested a record, then the record lasts
 * for one hour, and a state proof is available for it. If the transaction created an account, file,
 * or smart contract instance, then the record will contain the ID for what it created. If the
 * transaction called a smart contract function, then the record contains the result of that call.
 * If the transaction was a cryptocurrency transfer, then the record includes the TransferList which
 * gives the details of that transfer. If the transaction didn't return anything that should be in
 * the record, then the results field will be set to nothing.
 */
class TransactionRecordQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new TransactionGetRecord_pb_1.TransactionGetRecordQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setTransactiongetrecord(this._builder);
    }
    /**
     * The ID of the transaction for which the record is requested.
     */
    setTransactionId(txId) {
        this._builder.setTransactionid(new TransactionId_1.TransactionId(txId)._toProto());
        return this;
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasTransactionid()) {
            errors.push("`.setTransactionId()` required");
        }
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _getMethod() {
        return CryptoService_pb_service_1.CryptoService.getTxRecordByTxID;
    }
    _mapResponseHeader(response) {
        const header = response.getTransactiongetrecord().getHeader();
        return header == null ? new ResponseHeader_pb_1.ResponseHeader() : header;
    }
    _mapResponse(response) {
        const receipt = response.getTransactiongetrecord();
        return TransactionRecord_1.TransactionRecord._fromProto(receipt.getTransactionrecord());
    }
}
exports.TransactionRecordQuery = TransactionRecordQuery;
TransactionId_1.TransactionId.prototype.getRecord =
    function (client) {
        return __awaiter(this, void 0, void 0, function* () {
            // Wait for consensus using a free query first
            try {
                yield this.getReceipt(client);
            }
            catch (error) {
                if (!(error instanceof HederaReceiptStatusError_1.HederaReceiptStatusError)) {
                    throw error;
                }
            }
            const record = yield new TransactionRecordQuery()
                .setTransactionId(this)
                .execute(client);
            HederaRecordStatusError_1.HederaRecordStatusError._throwIfError(record.receipt.status.code, record, this);
            return record;
        });
    };
