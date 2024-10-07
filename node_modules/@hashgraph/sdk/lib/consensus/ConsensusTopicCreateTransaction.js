"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ConsensusTopicCreateTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const ConsensusCreateTopic_pb_1 = require("../generated/ConsensusCreateTopic_pb");
const ConsensusService_pb_service_1 = require("../generated/ConsensusService_pb_service");
const util_1 = require("../util");
const AccountId_1 = require("../account/AccountId");
class ConsensusTopicCreateTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        const body = new ConsensusCreateTopic_pb_1.ConsensusCreateTopicTransactionBody();
        this._body = body;
        this._inner.setConsensuscreatetopic(body);
        this.setAutoRenewPeriod(7890000);
    }
    setAdminKey(key) {
        this._body.setAdminkey(key._toProtoKey());
        return this;
    }
    /**
     * @deprecated `ConsensusTopicUpdateTransaction.setAutoRenewAccount()`
     * use `ConsensusTopicUpdateTransaction.setAutoRenewAccountId()` instead.
     */
    setAutoRenewAccount(id) {
        console.warn("`ConsensusTopicCreateTransaction.setAutoRenewAccount()` is deprecated\
use `ConsensusTopicCreateTransaction.setAutoRenewAccountId()` instead.");
        return this.setAutoRenewAccountId(id);
    }
    setAutoRenewAccountId(id) {
        this._body.setAutorenewaccount(new AccountId_1.AccountId(id)._toProto());
        return this;
    }
    setAutoRenewPeriod(seconds) {
        this._body.setAutorenewperiod(util_1.newDuration(seconds));
        return this;
    }
    setSubmitKey(key) {
        this._body.setSubmitkey(key._toProtoKey());
        return this;
    }
    setTopicMemo(memo) {
        this._body.setMemo(memo);
        return this;
    }
    get _method() {
        return ConsensusService_pb_service_1.ConsensusService.createTopic;
    }
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _doValidate(_) {
        // No local validation needed
    }
}
exports.ConsensusTopicCreateTransaction = ConsensusTopicCreateTransaction;
