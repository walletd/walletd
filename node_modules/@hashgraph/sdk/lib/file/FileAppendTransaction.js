"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FileAppendTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const FileService_pb_service_1 = require("../generated/FileService_pb_service");
const FileAppend_pb_1 = require("../generated/FileAppend_pb");
const FileId_1 = require("../file/FileId");
const utf8 = require("@stablelib/utf8");
/**
 * Append the given contents to the end of the file. If a file is too big to create with a single
 * `FileCreateTransaction``, then it can be created with the first part of its contents, and then
 * appended multiple times to create the entire file.
 */
class FileAppendTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        this._body = new FileAppend_pb_1.FileAppendTransactionBody();
        this._inner.setFileappend(this._body);
    }
    /**
     * The file ID of the file to which the bytes are appended to.
     */
    setFileId(fileId) {
        this._body.setFileid(new FileId_1.FileId(fileId)._toProto());
        return this;
    }
    /**
     * The bytes to append to the contents of the file.
     */
    setContents(contents) {
        const bytes = contents instanceof Uint8Array ?
            contents :
            utf8.encode(contents);
        this._body.setContents(bytes);
        return this;
    }
    _doValidate(errors) {
        const file = this._body.getFileid();
        const contents = this._body.getContents();
        if (file == null || contents == null) {
            errors.push("FileAppendTransaction must have a file id and contents set");
        }
    }
    get _method() {
        return FileService_pb_service_1.FileService.appendContent;
    }
}
exports.FileAppendTransaction = FileAppendTransaction;
