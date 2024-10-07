"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ContractBytecodeQuery = void 0;
const QueryBuilder_1 = require("../QueryBuilder");
const QueryHeader_pb_1 = require("../generated/QueryHeader_pb");
const SmartContractService_pb_service_1 = require("../generated/SmartContractService_pb_service");
const ContractGetBytecode_pb_1 = require("../generated/ContractGetBytecode_pb");
const ContractId_1 = require("./ContractId");
/**
 * Get the bytecode for a smart contract instance.
 */
class ContractBytecodeQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new ContractGetBytecode_pb_1.ContractGetBytecodeQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setContractgetbytecode(this._builder);
    }
    /**
     * The contract for which information is requested.
     */
    setContractId(contractIdLike) {
        this._builder.setContractid(new ContractId_1.ContractId(contractIdLike)._toProto());
        return this;
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasContractid()) {
            errors.push(".setContractId() required");
        }
    }
    _getMethod() {
        return SmartContractService_pb_service_1.SmartContractService.ContractGetBytecode;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getContractgetbytecoderesponse().getHeader();
    }
    _mapResponse(response) {
        return response.getContractgetbytecoderesponse().getBytecode_asU8();
    }
}
exports.ContractBytecodeQuery = ContractBytecodeQuery;
