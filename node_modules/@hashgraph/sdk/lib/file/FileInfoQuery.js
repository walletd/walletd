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
exports.FileInfoQuery = void 0;
const QueryBuilder_1 = require("../QueryBuilder");
const FileService_pb_service_1 = require("../generated/FileService_pb_service");
const FileGetInfo_pb_1 = require("../generated/FileGetInfo_pb");
const QueryHeader_pb_1 = require("../generated/QueryHeader_pb");
const FileId_1 = require("../file/FileId");
const Timestamp_1 = require("../Timestamp");
const Hbar_1 = require("../Hbar");
const PublicKey_1 = require("../crypto/PublicKey");
/**
 * Get all of the information about a file, except for its contents. When a file expires, it no
 * longer exists, and there will be no info about it, and the fileInfo field will be blank.
 * If a transaction or smart contract deletes the file, but it has not yet expired, then the
 * fileInfo field will be non-empty, the deleted field will be true, its size will be 0, and
 * its contents will be empty. Note that each file has a FileID, but does not have a filename.
 */
class FileInfoQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new FileGetInfo_pb_1.FileGetInfoQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setFilegetinfo(this._builder);
    }
    /**
     * The file ID of the file for which information is requested.
     */
    setFileId(fileId) {
        this._builder.setFileid(new FileId_1.FileId(fileId)._toProto());
        return this;
    }
    getCost(client) {
        const _super = Object.create(null, {
            getCost: { get: () => super.getCost }
        });
        return __awaiter(this, void 0, void 0, function* () {
            // deleted files return a COST_ANSWER of zero which triggers `INSUFFICIENT_TX_FEE`
            // if you set that as the query payment; 25 tinybar seems to be the minimum to get
            // `FILE_DELETED` back instead.
            const min = Hbar_1.Hbar.fromTinybar(25);
            const cost = yield _super.getCost.call(this, client);
            return cost.isGreaterThan(min) ? cost : min;
        });
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasFileid()) {
            errors.push(".setFileId() required");
        }
    }
    _getMethod() {
        return FileService_pb_service_1.FileService.getFileInfo;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getFilegetinfo().getHeader();
    }
    _mapResponse(response) {
        const fileInfo = response.getFilegetinfo().getFileinfo();
        return {
            fileId: FileId_1.FileId._fromProto(fileInfo.getFileid()),
            size: fileInfo.getSize(),
            expirationTime: fileInfo.hasExpirationtime() ?
                Timestamp_1.timestampToDate(fileInfo.getExpirationtime()) :
                null,
            isDeleted: fileInfo.getDeleted(),
            keys: PublicKey_1._fromProtoKeyList(fileInfo.getKeys())
        };
    }
}
exports.FileInfoQuery = FileInfoQuery;
