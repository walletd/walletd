"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ConsensusTopicUpdateTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const ConsensusUpdateTopic_pb_1 = require("../generated/ConsensusUpdateTopic_pb");
const ConsensusService_pb_service_1 = require("../generated/ConsensusService_pb_service");
const util_1 = require("../util");
const AccountId_1 = require("../account/AccountId");
const ConsensusTopicId_1 = require("./ConsensusTopicId");
const wrappers_pb_1 = require("google-protobuf/google/protobuf/wrappers_pb");
class ConsensusTopicUpdateTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        const body = new ConsensusUpdateTopic_pb_1.ConsensusUpdateTopicTransactionBody();
        this._body = body;
        this._inner.setConsensusupdatetopic(body);
    }
    clearTopicMemo() {
        this._body.clearMemo();
        return this;
    }
    clearAdminKey() {
        this._body.clearAdminkey();
        return this;
    }
    clearSubmitKey() {
        this._body.clearSubmitkey();
        return this;
    }
    clearAutoRenewAccount() {
        this._body.clearAutorenewaccount();
        return this;
    }
    setAdminKey(key) {
        this._body.setAdminkey(key._toProtoKey());
        return this;
    }
    /**
     * @deprecated `ConsensusTopicCreateTransaction.setAutoRenewAccount()`
     * use `ConsensusTopicCreateTransaction.setAutoRenewAccountId()` instead.
     */
    setAutoRenewAccount(id) {
        console.warn("`ConsensusTopicUpdateTransaction.setAutoRenewAccount()` is deprecated\
use `ConsensusTopicUpdateTransaction.setAutoRenewAccountId()` instead.");
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
    setExpirationTime(time) {
        this._body.setExpirationtime(time._toProto());
        return this;
    }
    setSubmitKey(key) {
        this._body.setSubmitkey(key._toProtoKey());
        return this;
    }
    setTopicId(id) {
        this._body.setTopicid(new ConsensusTopicId_1.ConsensusTopicId(id)._toProto());
        return this;
    }
    setTopicMemo(memo) {
        const value = new wrappers_pb_1.StringValue();
        value.setValue(memo);
        this._body.setMemo(value);
        return this;
    }
    get _method() {
        return ConsensusService_pb_service_1.ConsensusService.updateTopic;
    }
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _doValidate(_) {
        // No local validation needed
    }
}
exports.ConsensusTopicUpdateTransaction = ConsensusTopicUpdateTransaction;
