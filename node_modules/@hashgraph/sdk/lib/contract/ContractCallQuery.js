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
exports.ContractCallQuery = void 0;
const QueryBuilder_1 = require("../QueryBuilder");
const QueryHeader_pb_1 = require("../generated/QueryHeader_pb");
const SmartContractService_pb_service_1 = require("../generated/SmartContractService_pb_service");
const ContractId_1 = require("./ContractId");
const ContractFunctionResult_1 = require("./ContractFunctionResult");
const ContractCallLocal_pb_1 = require("../generated/ContractCallLocal_pb");
const ContractFunctionParams_1 = require("./ContractFunctionParams");
/**
 * Call a function of the given smart contract instance, giving it functionParameters as its inputs.
 * It will consume the entire given amount of gas.
 *
 * This is performed locally on the particular node that the client is communicating with. It cannot
 * change the state of the contract instance (and so, cannot spend anything from the instance's
 * cryptocurrency account). It will not have a consensus timestamp. It cannot generate a record or a
 * receipt. The response will contain the output returned by the function call.  This is useful for
 * calling getter functions, which purely read the state and don't change it. It is faster and
 * cheaper than a normal call, because it is purely local to a single  node.
 */
class ContractCallQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new ContractCallLocal_pb_1.ContractCallLocalQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setContractcalllocal(this._builder);
    }
    /**
     * The contract instance to call, in the format used in transactions.
     */
    setContractId(contractIdLike) {
        this._builder.setContractid(new ContractId_1.ContractId(contractIdLike)._toProto());
        return this;
    }
    /**
     * Which function to call, and the parameters to pass to the function.
     */
    setFunction(name, params) {
        this._builder.setFunctionparameters((params !== null && params !== void 0 ? params : new ContractFunctionParams_1.ContractFunctionParams())._build(name));
        return this;
    }
    /**
     * The amount of gas to use for the call. All of the gas offered will be charged for.
     */
    setGas(gas) {
        this._builder.setGas(gas);
        return this;
    }
    /**
     * Max number of bytes that the result might include. The run will fail if it would have
     * returned more than this number of bytes.
     */
    setMaxResultSize(size) {
        this._builder.setMaxresultsize(size);
        return this;
    }
    /**
     * Wrapper around `QueryBuilder.getCost()`. This must exist because the cost returned
     * `QueryBuilder.getCost()` and therein the Hedera Network doesn't work for
     * `ContractCallQuery`'s. However, if you multiply the cost by ~1.1 then _most_
     * contracts calls seem to complete fine.
     */
    getCost(client) {
        const _super = Object.create(null, {
            getCost: { get: () => super.getCost }
        });
        return __awaiter(this, void 0, void 0, function* () {
            return (yield _super.getCost.call(this, client)).multipliedBy(1.1);
        });
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasContractid()) {
            errors.push(".setContractId() required");
        }
    }
    _getMethod() {
        return SmartContractService_pb_service_1.SmartContractService.contractCallLocalMethod;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getContractcalllocal().getHeader();
    }
    _mapResponse(response) {
        return new ContractFunctionResult_1.ContractFunctionResult(response.getContractcalllocal().getFunctionresult());
    }
}
exports.ContractCallQuery = ContractCallQuery;
