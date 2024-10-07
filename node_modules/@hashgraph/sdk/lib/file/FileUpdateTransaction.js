"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FileUpdateTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const FileService_pb_service_1 = require("../generated/FileService_pb_service");
const BasicTypes_pb_1 = require("../generated/BasicTypes_pb");
const FileUpdate_pb_1 = require("../generated/FileUpdate_pb");
const FileId_1 = require("../file/FileId");
const Timestamp_1 = require("../Timestamp");
const utf8 = require("@stablelib/utf8");
/**
 * Modify some of the metadata for a file. Any null field is ignored (left unchanged). Any field
 * that is null is left unchanged. If contents is non-null, then the file's contents will be
 * replaced with the given bytes. This transaction must be signed by all the keys for that file.
 * If the transaction is modifying the keys field, then it must be signed by all the keys in both
 * the old list and the new list.
 *
 * If a file was created without ANY keys in the keys field, ONLY the expirationTime of the file
 * can be changed using this call. The file contents or its keys cannot be changed.
 */
class FileUpdateTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        this._body = new FileUpdate_pb_1.FileUpdateTransactionBody();
        this._inner.setFileupdate(this._body);
    }
    /**
     * The new time at which it should expire (ignored if not later than the current value).
     */
    setExpirationTime(date) {
        this._body.setExpirationtime(Timestamp_1.timestampToProto(Timestamp_1.dateToTimestamp(date)));
        return this;
    }
    /**
     * The keys that can modify or delete the file.
     */
    addKey(key) {
        const keylist = this._body.getKeys() == null ?
            new BasicTypes_pb_1.KeyList() :
            this._body.getKeys();
        keylist.addKeys(key._toProtoKey());
        this._body.setKeys(keylist);
        return this;
    }
    /**
     * The new file contents. All the bytes in the old contents are discarded.
     */
    setContents(contents) {
        const bytes = contents instanceof Uint8Array ?
            contents :
            utf8.encode(contents);
        this._body.setContents(bytes);
        return this;
    }
    /**
     * The file ID of the file to update.
     */
    setFileId(fileIdLike) {
        this._body.setFileid(new FileId_1.FileId(fileIdLike)._toProto());
        return this;
    }
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _doValidate(errors) {
        // No validation
    }
    get _method() {
        return FileService_pb_service_1.FileService.updateFile;
    }
}
exports.FileUpdateTransaction = FileUpdateTransaction;
