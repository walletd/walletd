"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ContractDeleteTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const SmartContractService_pb_service_1 = require("../generated/SmartContractService_pb_service");
const ContractDelete_pb_1 = require("../generated/ContractDelete_pb");
const ContractId_1 = require("./ContractId");
const AccountId_1 = require("../account/AccountId");
/**
 * Modify a smart contract instance to have the given parameter values. Any null field is ignored
 * (left unchanged). If only the contractInstanceExpirationTime is being modified, then no
 * signature is needed on this transaction other than for the account paying for the transaction
 * itself. But if any of the other fields are being modified, then it must be signed by the adminKey.
 * The use of adminKey is not currently supported in this API, but in the future will be implemented
 * to allow these fields to be modified, and also to make modifications to the state of the instance.
 * If the contract is created with no admin key, then none of the fields can be changed that need an
 * admin signature, and therefore no admin key can ever be added. So if there is no admin key, then
 * things like the bytecode are immutable. But if there is an admin key, then they can be changed.
 * For example, the admin key might be a threshold key, which requires 3 of 5 binding arbitration
 * judges to agree before the bytecode can be changed. This can be used to add flexibility to the
 * mangement of smart contract behavior. But this is optional. If the smart contract is created
 * without an admin key, then such a key can never be added, and its bytecode will be immutable.
 */
class ContractDeleteTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        this._body = new ContractDelete_pb_1.ContractDeleteTransactionBody();
        this._inner.setContractdeleteinstance(this._body);
    }
    /**
     * The Contract ID instance to delete (this can't be changed).
     */
    setContractId(contractIdLike) {
        this._body.setContractid(new ContractId_1.ContractId(contractIdLike)._toProto());
        return this;
    }
    /**
     * The account ID which will receive all remaining hbars.
     *
     * Note: Can only transfer to either an `AccountId` *or* `ContractId` not both.
     */
    setTransferAccountId(id) {
        this._body.setTransferaccountid(new AccountId_1.AccountId(id)._toProto());
        return this;
    }
    /**
     * The contract ID which will receive all remaining hbars
     *
     * Note: Can only transfer to either an `AccountId` *or* `ContractId` not both.
     */
    setTransferContractid(id) {
        this._body.setTransfercontractid(new ContractId_1.ContractId(id)._toProto());
        return this;
    }
    _doValidate(errors) {
        if (!this._body.hasContractid()) {
            errors.push(".setContractId() required");
        }
    }
    get _method() {
        return SmartContractService_pb_service_1.SmartContractService.deleteContract;
    }
}
exports.ContractDeleteTransaction = ContractDeleteTransaction;
