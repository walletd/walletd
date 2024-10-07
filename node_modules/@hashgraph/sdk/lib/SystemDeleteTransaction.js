"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SystemDeleteTransaction = void 0;
const TransactionBuilder_1 = require("./TransactionBuilder");
const SystemDelete_pb_1 = require("./generated/SystemDelete_pb");
const Timestamp_1 = require("./Timestamp");
const FileId_1 = require("./file/FileId");
const ContractId_1 = require("./contract/ContractId");
const FileService_pb_service_1 = require("./generated/FileService_pb_service");
const util_1 = require("./util");
/**
 * Delete a file or smart contract - can only be done with a Hedera admin multisig. When it is
 * deleted, it immediately disappears from the system as seen by the user, but is still stored
 * internally until the expiration time, at which time it is truly and permanently deleted.
 * Until that time, it can be undeleted by the Hedera admin multisig. When a smart contract is
 * deleted, the cryptocurrency account within it continues to exist, and is not affected
 * by the expiration time here.
 */
class SystemDeleteTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        this._body = new SystemDelete_pb_1.SystemDeleteTransactionBody();
        this.setExpirationTime(Date.now() + 7890000000);
        this._inner.setSystemdelete(this._body);
    }
    /**
     * The timestamp in seconds at which the "deleted" file should truly be permanently deleted.
     */
    setExpirationTime(date) {
        this._body.setExpirationtime(Timestamp_1.timestampToProto(Timestamp_1.dateToTimestamp(date)));
        return this;
    }
    /**
     * @deprecated `.setId` is deprecated. Use `.setFileId` or `.setContractId` instead.
     */
    setId(id) {
        console.warn("`.setId` is deprecated. Use `.setFileId` or `.setContractId` instead");
        try {
            const fileId = util_1.normalizeEntityId("file", id);
            this._body.setFileid(new FileId_1.FileId(fileId)._toProto());
        }
        catch (_a) {
            const contractId = util_1.normalizeEntityId("contract", id);
            this._body.setContractid(new ContractId_1.ContractId(contractId)._toProto());
        }
        return this;
    }
    /**
     * The file ID of the file to delete, in the format used in transactions.
     */
    setFileId(id) {
        this._body.setFileid(new FileId_1.FileId(id)._toProto());
        return this;
    }
    /**
     * The contract ID instance to delete, in the format used in transactions
     */
    setContractId(id) {
        this._body.setContractid(new ContractId_1.ContractId(id)._toProto());
        return this;
    }
    _doValidate(errors) {
        if (!this._body.hasContractid() == null && !this._body.hasFileid()) {
            errors.push("SystemDelete must have an id set. Use `.setFileId()` or `.setContractId()");
        }
    }
    get _method() {
        return FileService_pb_service_1.FileService.systemDelete;
    }
}
exports.SystemDeleteTransaction = SystemDeleteTransaction;
