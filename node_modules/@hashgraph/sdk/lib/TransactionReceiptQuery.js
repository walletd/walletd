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
exports.TransactionReceiptQuery = void 0;
const QueryBuilder_1 = require("./QueryBuilder");
const TransactionGetReceipt_pb_1 = require("./generated/TransactionGetReceipt_pb");
const QueryHeader_pb_1 = require("./generated/QueryHeader_pb");
const TransactionId_1 = require("./TransactionId");
const TransactionReceipt_1 = require("./TransactionReceipt");
const CryptoService_pb_service_1 = require("./generated/CryptoService_pb_service");
const Status_1 = require("./Status");
const HederaReceiptStatusError_1 = require("./errors/HederaReceiptStatusError");
class TransactionReceiptQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new TransactionGetReceipt_pb_1.TransactionGetReceiptQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setTransactiongetreceipt(this._builder);
    }
    setTransactionId(txId) {
        this._builder.setTransactionid(new TransactionId_1.TransactionId(txId)._toProto());
        return this;
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasTransactionid()) {
            errors.push("`.setTransactionId()` required");
        }
    }
    _getMethod() {
        return CryptoService_pb_service_1.CryptoService.getTransactionReceipts;
    }
    _shouldRetry(status, response) {
        var _a;
        if (super._shouldRetry(status, response))
            return true;
        if ([
            Status_1.Status.Busy.code,
            Status_1.Status.Unknown.code,
            Status_1.Status.ReceiptNotFound.code
        ].includes(status.code)) {
            return true;
        }
        // If there _was_ a receipt fetched, check the status of that
        const receipt = (_a = response.getTransactiongetreceipt()) === null || _a === void 0 ? void 0 : _a.getReceipt();
        const receiptStatus = receipt == null ? null : Status_1.Status._fromCode(receipt.getStatus());
        if (receiptStatus != null) {
            if ([
                // Accepted but has not reached consensus
                Status_1.Status.Ok.code,
                // Queue is full
                Status_1.Status.Busy.code,
                // Still in the node's queue
                Status_1.Status.Unknown.code,
                Status_1.Status.ReceiptNotFound.code
            ].includes(receiptStatus.code)) {
                return true;
            }
        }
        return false;
    }
    _getDefaultExecuteTimeout() {
        return 120000; // ~2 minutes
    }
    _isPaymentRequired() {
        // Receipt queries do not require a payment
        return false;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getTransactiongetreceipt().getHeader();
    }
    _mapResponse(response) {
        const receipt = response.getTransactiongetreceipt();
        return TransactionReceipt_1.TransactionReceipt._fromProto(receipt.getReceipt());
    }
}
exports.TransactionReceiptQuery = TransactionReceiptQuery;
TransactionId_1.TransactionId.prototype.getReceipt =
    function (client) {
        return __awaiter(this, void 0, void 0, function* () {
            const receipt = yield new TransactionReceiptQuery()
                .setTransactionId(this)
                .execute(client);
            // Throw an exception on an invalid receipt status
            HederaReceiptStatusError_1.HederaReceiptStatusError._throwIfError(receipt.status.code, receipt, this);
            return receipt;
        });
    };
