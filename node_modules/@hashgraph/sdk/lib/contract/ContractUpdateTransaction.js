"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ContractUpdateTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const SmartContractService_pb_service_1 = require("../generated/SmartContractService_pb_service");
const ContractUpdate_pb_1 = require("../generated/ContractUpdate_pb");
const util_1 = require("../util");
const ContractId_1 = require("./ContractId");
const AccountId_1 = require("../account/AccountId");
const FileId_1 = require("../file/FileId");
const Timestamp_1 = require("../Timestamp");
/**
 * Modify a smart contract instance to have the given parameter values. Any null field is ignored
 * (left unchanged). If only the contractInstanceExpirationTime is being modified, then no signature
 * is needed on this transaction other than for the account paying for the transaction itself.
 * But if any of the other fields are being modified, then it must be signed by the adminKey. The
 * use of adminKey is not currently supported in this API, but in the future will be implemented to
 * allow these fields to be modified, and also to make modifications to the state of the instance.
 * If the contract is created with no admin key, then none of the fields can be changed that need an
 * admin signature, and therefore no admin key can ever be added. So if there is no admin key, then
 * things like the bytecode are immutable. But if there is an admin key, then they can be changed.
 * For example, the admin key might be a threshold key, which requires 3 of 5 binding arbitration
 * judges to agree before the bytecode can be changed. This can be used to add flexibility to the
 * management of smart contract behavior. But this is optional. If the smart contract is created
 * without an admin key, then such a key can never be added, and its bytecode will be immutable.
 */
class ContractUpdateTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        this._body = new ContractUpdate_pb_1.ContractUpdateTransactionBody();
        this._inner.setContractupdateinstance(this._body);
    }
    /**
     * The Contract ID instance to update (this can't be changed).
     */
    setContractId(contractIdLike) {
        this._body.setContractid(new ContractId_1.ContractId(contractIdLike)._toProto());
        return this;
    }
    /**
     * The state of the instance can be modified arbitrarily if this key signs a transaction to
     * modify it. If this is null, then such modifications are not possible, and there is no
     * administrator that can override the normal operation of this smart contract instance.
     */
    setAdminKey(publicKey) {
        this._body.setAdminkey(publicKey._toProtoKey());
        return this;
    }
    /**
     * ID of the account to which this account is proxy staked. If proxyAccountID is null, or is
     * an invalid account, or is an account that isn't a node, then this account is automatically
     * proxy staked to a node chosen by the network, but without earning payments. If the
     * proxyAccountID account refuses to accept proxy staking , or if it is not currently running
     * a node, then it will behave as if proxyAccountID was null.
     */
    setProxyAccountId(proxyAccountId) {
        this._body.setProxyaccountid(new AccountId_1.AccountId(proxyAccountId)._toProto());
        return this;
    }
    /**
     * The file ID of file containing the smart contract byte code. A copy will be made and held
     * by the contract instance, and have the same expiration time as the instance.
     */
    setBytecodeFileId(fileIdLike) {
        this._body.setFileid(new FileId_1.FileId(fileIdLike)._toProto());
        return this;
    }
    /**
     * The instance will charge its account every this many seconds to renew for this long.
     */
    setAutoRenewPeriod(seconds) {
        this._body.setAutorenewperiod(util_1.newDuration(seconds));
        return this;
    }
    /**
     * Extend the expiration of the instance and its account to this time (no effect if it
     * already is this time or later).
     */
    setExpirationTime(date) {
        this._body.setExpirationtime(Timestamp_1.timestampToProto(Timestamp_1.dateToTimestamp(date)));
        return this;
    }
    /**
     * The memo associated with the contract (max 100 bytes)
     */
    setContractMemo(memo) {
        this._body.setMemo(memo);
        return this;
    }
    _doValidate(errors) {
        if (!this._body.hasContractid()) {
            errors.push(".setContractId() required");
        }
    }
    get _method() {
        return SmartContractService_pb_service_1.SmartContractService.updateContract;
    }
}
exports.ContractUpdateTransaction = ContractUpdateTransaction;
