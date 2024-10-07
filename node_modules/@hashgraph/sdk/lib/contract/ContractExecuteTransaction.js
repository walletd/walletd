"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ContractExecuteTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const SmartContractService_pb_service_1 = require("../generated/SmartContractService_pb_service");
const ContractCall_pb_1 = require("../generated/ContractCall_pb");
const ContractId_1 = require("./ContractId");
const ContractFunctionParams_1 = require("./ContractFunctionParams");
const Hbar_1 = require("../Hbar");
/**
 * Call a function of the given smart contract instance, giving it functionParameters as its inputs.
 * It can use the given amount of gas, and any unspent gas will be refunded to the paying account.
 *
 * If this function stores information, it is charged gas to store it. There is a fee in hbars to
 * maintain that storage until the expiration time, and that fee is added as part of the
 * transaction fee.
 */
class ContractExecuteTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        this._body = new ContractCall_pb_1.ContractCallTransactionBody();
        this._inner.setContractcall(this._body);
    }
    /**
     * The maximum amount of gas to use for the call.
     */
    setGas(gas) {
        this._body.setGas(String(gas));
        return this;
    }
    /**
     * Number of tinybars sent (the function must be payable if this is nonzero).
     */
    setPayableAmount(amount) {
        const amountHbar = Hbar_1.hbarFromTinybarOrHbar(amount);
        amountHbar[Hbar_1.hbarCheck]({ allowNegative: false });
        this._body.setAmount(amountHbar[Hbar_1.hbarToProto]());
        return this;
    }
    /**
     * Which function to call, and the parameters to pass to the function.
     */
    setFunction(name, params) {
        this._body.setFunctionparameters((params !== null && params !== void 0 ? params : new ContractFunctionParams_1.ContractFunctionParams())._build(name));
        return this;
    }
    /**
     * The contract instance to call, in the format used in transactions.
     */
    setContractId(contractIdLike) {
        this._body.setContractid(new ContractId_1.ContractId(contractIdLike)._toProto());
        return this;
    }
    _doValidate(errors) {
        if (!this._body.hasContractid()) {
            errors.push(".setContractId() required");
        }
    }
    get _method() {
        return SmartContractService_pb_service_1.SmartContractService.contractCallMethod;
    }
}
exports.ContractExecuteTransaction = ContractExecuteTransaction;
