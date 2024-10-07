"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FileContentsQuery = void 0;
const QueryBuilder_1 = require("../QueryBuilder");
const QueryHeader_pb_1 = require("../generated/QueryHeader_pb");
const FileGetContents_pb_1 = require("../generated/FileGetContents_pb");
const FileService_pb_service_1 = require("../generated/FileService_pb_service");
const FileId_1 = require("../file/FileId");
/**
 * Get the contents of a file. The content field is empty (no bytes) if the file is empty.
 */
class FileContentsQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new FileGetContents_pb_1.FileGetContentsQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setFilegetcontents(this._builder);
    }
    /**
     * The file ID of the file whose contents are requested.
     */
    setFileId(fileId) {
        this._builder.setFileid(new FileId_1.FileId(fileId)._toProto());
        return this;
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasFileid()) {
            errors.push(".setFileId() required");
        }
    }
    _getMethod() {
        return FileService_pb_service_1.FileService.getFileContent;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getFilegetcontents().getHeader();
    }
    _mapResponse(response) {
        const fileConents = response.getFilegetcontents().getFilecontents();
        return fileConents.getContents_asU8();
    }
}
exports.FileContentsQuery = FileContentsQuery;
