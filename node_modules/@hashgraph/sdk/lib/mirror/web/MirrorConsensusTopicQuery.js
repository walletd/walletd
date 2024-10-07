"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MirrorConsensusTopicQuery = void 0;
const grpc_web_1 = require("@improbable-eng/grpc-web");
const MirrorConsensusService_pb_service_1 = require("../../generated/MirrorConsensusService_pb_service");
const TransactionId_1 = require("../../TransactionId");
const BaseMirrorConsensusTopicQuery_1 = require("../BaseMirrorConsensusTopicQuery");
const MirrorConsensusTopicResponse_1 = require("../MirrorConsensusTopicResponse");
const MirrorSubscriptionHandle_1 = require("../MirrorSubscriptionHandle");
class MirrorConsensusTopicQuery extends BaseMirrorConsensusTopicQuery_1.BaseMirrorConsensusTopicQuery {
    subscribe(client, listener, errorHandler) {
        this._validate();
        const handle = new MirrorSubscriptionHandle_1.MirrorSubscriptionHandle();
        this._makeServerStreamRequest(handle, 0, client, listener, errorHandler);
        return handle;
    }
    _makeServerStreamRequest(handle, attempt, client, listener, errorHandler) {
        const list = {};
        const _makeServerStreamRequest = this._makeServerStreamRequest;
        let shouldRetry = true;
        const response = grpc_web_1.grpc.invoke(MirrorConsensusService_pb_service_1.ConsensusService.subscribeTopic, {
            host: client.endpoint,
            request: this._builder,
            onMessage(message) {
                shouldRetry = false;
                if (!message.hasChunkinfo()) {
                    listener(new MirrorConsensusTopicResponse_1.MirrorConsensusTopicResponse(message));
                }
                else {
                    const chunkInfo = message.getChunkinfo();
                    // eslint-disable-next-line max-len
                    const txId = TransactionId_1.TransactionId._fromProto(chunkInfo.getInitialtransactionid()).toString();
                    if (list[txId] == null) {
                        list[txId] = [];
                    }
                    list[txId].push(message);
                    if (list[txId].length === chunkInfo.getTotal()) {
                        const m = list[txId];
                        list[txId] = null;
                        listener(new MirrorConsensusTopicResponse_1.MirrorConsensusTopicResponse(m));
                    }
                }
            },
            onEnd(code, message) {
                if (!shouldRetry || attempt > 10) {
                    if (errorHandler != null) {
                        errorHandler(new Error(`Received status code: ${code} and message: ${message}`));
                    }
                }
                else if (attempt < 10 &&
                    shouldRetry &&
                    (code === grpc_web_1.grpc.Code.NotFound ||
                        code === grpc_web_1.grpc.Code.Unavailable)) {
                    setTimeout(() => {
                        _makeServerStreamRequest(handle, attempt + 1, client, listener, errorHandler);
                    }, 250 * Math.pow(2, attempt));
                }
            }
        });
        handle._setCall(response.close);
    }
}
exports.MirrorConsensusTopicQuery = MirrorConsensusTopicQuery;
