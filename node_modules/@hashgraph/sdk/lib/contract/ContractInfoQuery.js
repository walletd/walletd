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
exports.ContractInfoQuery = void 0;
const QueryBuilder_1 = require("../QueryBuilder");
const ContractGetInfo_pb_1 = require("../generated/ContractGetInfo_pb");
const QueryHeader_pb_1 = require("../generated/QueryHeader_pb");
const SmartContractService_pb_service_1 = require("../generated/SmartContractService_pb_service");
const ContractId_1 = require("./ContractId");
const AccountId_1 = require("../account/AccountId");
const Timestamp_1 = require("../Timestamp");
const PublicKey_1 = require("../crypto/PublicKey");
const Hbar_1 = require("../Hbar");
/**
 * Get information about a smart contract instance. This includes the account that it uses, the
 * file containing its bytecode, and the time when it will expire.
 */
class ContractInfoQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new ContractGetInfo_pb_1.ContractGetInfoQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setContractgetinfo(this._builder);
    }
    /**
     * The contract for which information is requested.
     */
    setContractId(contractIdLike) {
        this._builder.setContractid(new ContractId_1.ContractId(contractIdLike)._toProto());
        return this;
    }
    /**
     * Wrapper around `QueryBuilder.getCost()`. This must exist because the cost returned
     * `QueryBuilder.getCost()` and therein the Hedera Network doesn't work for any
     * contracts that have been deleted. In that case we want the minimum
     * cost to be ~25 Tinybar as this seems to succeed most of the time.
     */
    getCost(client) {
        const _super = Object.create(null, {
            getCost: { get: () => super.getCost }
        });
        return __awaiter(this, void 0, void 0, function* () {
            // deleted contracts return a COST_ANSWER of zero which triggers `INSUFFICIENT_TX_FEE`
            // if you set that as the query payment; 25 tinybar seems to be the minimum to get
            // `CONTRACT_DELETED` back instead.
            const min = Hbar_1.Hbar.fromTinybar(25);
            const cost = yield _super.getCost.call(this, client);
            return cost.isGreaterThan(min) ? cost : min;
        });
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasContractid()) {
            errors.push(".setContractId() required");
        }
    }
    _getMethod() {
        return SmartContractService_pb_service_1.SmartContractService.getContractInfo;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getContractgetinfo().getHeader();
    }
    _mapResponse(response) {
        const contractInfo = response.getContractgetinfo().getContractinfo();
        return {
            contractId: ContractId_1.ContractId._fromProto(contractInfo.getContractid()),
            accountId: AccountId_1.AccountId._fromProto(contractInfo.getAccountid()),
            contractAccountId: contractInfo.getContractaccountid(),
            adminKey: contractInfo.hasAdminkey() ?
                PublicKey_1._fromProtoKey(contractInfo.getAdminkey()) :
                null,
            expirationTime: Timestamp_1.timestampToDate(contractInfo.getExpirationtime()),
            autoRenewPeriod: contractInfo.getAutorenewperiod().getSeconds(),
            storage: contractInfo.getStorage(),
            contractMemo: contractInfo.getMemo()
        };
    }
}
exports.ContractInfoQuery = ContractInfoQuery;
