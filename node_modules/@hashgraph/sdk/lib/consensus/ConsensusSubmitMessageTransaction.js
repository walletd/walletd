"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ConsensusSubmitMessageTransaction = void 0;
const TransactionBuilder_1 = require("../TransactionBuilder");
const ConsensusSubmitMessage_pb_1 = require("../generated/ConsensusSubmitMessage_pb");
const ConsensusService_pb_service_1 = require("../generated/ConsensusService_pb_service");
const ConsensusTopicId_1 = require("./ConsensusTopicId");
const utf8 = require("@stablelib/utf8");
/** @deprecated use `ConsensusMessageSubmitTransaction` instead. */
class ConsensusSubmitMessageTransaction extends TransactionBuilder_1.SingleTransactionBuilder {
    /** @deprecated use `ConsensusMessageSubmitTransaction` instead. */
    constructor() {
        super();
        console.warn("deprecated: ConsensusSubmitMessageTransaction has been renamed to ConsensusMessageSubmitTransaction");
        const body = new ConsensusSubmitMessage_pb_1.ConsensusSubmitMessageTransactionBody();
        this._body = body;
        this._inner.setConsensussubmitmessage(body);
    }
    setTopicId(id) {
        this._body.setTopicid(new ConsensusTopicId_1.ConsensusTopicId(id)._toProto());
        return this;
    }
    setMessage(message) {
        if (message instanceof Uint8Array) {
            this._body.setMessage(message);
        }
        else {
            this._body.setMessage(utf8.encode(message));
        }
        return this;
    }
    get _method() {
        return ConsensusService_pb_service_1.ConsensusService.submitMessage;
    }
    // eslint-disable-next-line @typescript-eslint/no-empty-function
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _doValidate(_) {
        // No local validation needed
    }
}
exports.ConsensusSubmitMessageTransaction = ConsensusSubmitMessageTransaction;
