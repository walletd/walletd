"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FileDeleteTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const FileService_pb_service_1 = require("../generated/FileService_pb_service");
const FileDelete_pb_1 = require("../generated/FileDelete_pb");
const FileId_1 = require("../file/FileId");
/**
 * Delete the given file. After deletion, it will be marked as deleted and will have no contents.
 * But information about it will continue to exist until it expires. A list of keys  was given
 * when the file was created. All the keys on that list must sign transactions to create or modify
 * the file, but any single one of them can be used to delete the file. Each "key" on that list
 * may itself be a threshold key containing other keys (including other threshold keys).
 */
class FileDeleteTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        this._body = new FileDelete_pb_1.FileDeleteTransactionBody();
        this._inner.setFiledelete(this._body);
    }
    /**
     * The file to delete. It will be marked as deleted until it expires. Then it will disappear.
     */
    setFileId(fileIdLike) {
        this._body.setFileid(new FileId_1.FileId(fileIdLike)._toProto());
        return this;
    }
    _doValidate(errors) {
        const fileId = this._body.getFileid();
        if (fileId == null) {
            errors.push("FileDeleteTransaction must have a file id set");
        }
    }
    get _method() {
        return FileService_pb_service_1.FileService.deleteFile;
    }
}
exports.FileDeleteTransaction = FileDeleteTransaction;
