"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.GetBySolidityIdQuery = void 0;
const QueryBuilder_1 = require("./QueryBuilder");
const QueryHeader_pb_1 = require("./generated/QueryHeader_pb");
const FileId_1 = require("./file/FileId");
const ContractId_1 = require("./contract/ContractId");
const AccountId_1 = require("./account/AccountId");
const pb = require("./generated/GetBySolidityID_pb");
const SmartContractService_pb_service_1 = require("./generated/SmartContractService_pb_service");
/**
 * Get the IDs in the format used by transactions, given the ID in the format used by Solidity.
 * If the Solidity ID is for a smart contract instance, then both the ContractID and
 * associated AccountID will be returned.
 */
class GetBySolidityIdQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new pb.GetBySolidityIDQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setGetbysolidityid(this._builder);
    }
    /**
     * The ID in the format used by Solidity.
     */
    setSolidityId(id) {
        this._builder.setSolidityid(id);
        return this;
    }
    _doLocalValidate( /* errors: string[] */) {
        // Nothing
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _getMethod() {
        return SmartContractService_pb_service_1.SmartContractService.getBySolidityID;
    }
    _mapResponseHeader(response) {
        return response.getGetbysolidityid().getHeader();
    }
    _mapResponse(response) {
        const id = response.getGetbysolidityid();
        if (id.hasAccountid()) {
            return {
                type: "ACCOUNT",
                accountId: AccountId_1.AccountId._fromProto(id.getAccountid())
            };
        }
        if (id.hasContractid()) {
            return {
                type: "CONTRACT",
                contractId: ContractId_1.ContractId._fromProto(id.getContractid())
            };
        }
        if (id.hasFileid()) {
            return {
                type: "FILE",
                fileId: FileId_1.FileId._fromProto(id.getFileid())
            };
        }
        throw new Error("unreachable");
    }
}
exports.GetBySolidityIdQuery = GetBySolidityIdQuery;
