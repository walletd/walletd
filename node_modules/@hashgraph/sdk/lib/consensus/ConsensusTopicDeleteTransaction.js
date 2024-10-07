"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ConsensusTopicDeleteTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const ConsensusDeleteTopic_pb_1 = require("../generated/ConsensusDeleteTopic_pb");
const ConsensusService_pb_service_1 = require("../generated/ConsensusService_pb_service");
const ConsensusTopicId_1 = require("./ConsensusTopicId");
class ConsensusTopicDeleteTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    constructor() {
        super();
        const body = new ConsensusDeleteTopic_pb_1.ConsensusDeleteTopicTransactionBody();
        this._body = body;
        this._inner.setConsensusdeletetopic(body);
    }
    setTopicId(id) {
        this._body.setTopicid(new ConsensusTopicId_1.ConsensusTopicId(id)._toProto());
        return this;
    }
    get _method() {
        return ConsensusService_pb_service_1.ConsensusService.deleteTopic;
    }
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _doValidate(_) {
        // No local validation needed
    }
}
exports.ConsensusTopicDeleteTransaction = ConsensusTopicDeleteTransaction;
