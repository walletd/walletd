// package: com.hedera.mirror.api.proto
// file: MirrorConsensusService.proto

var MirrorConsensusService_pb = require("./MirrorConsensusService_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var ConsensusService = (function () {
  function ConsensusService() {}
  ConsensusService.serviceName = "com.hedera.mirror.api.proto.ConsensusService";
  return ConsensusService;
}());

ConsensusService.subscribeTopic = {
  methodName: "subscribeTopic",
  service: ConsensusService,
  requestStream: false,
  responseStream: true,
  requestType: MirrorConsensusService_pb.ConsensusTopicQuery,
  responseType: MirrorConsensusService_pb.ConsensusTopicResponse
};

exports.ConsensusService = ConsensusService;

function ConsensusServiceClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

ConsensusServiceClient.prototype.subscribeTopic = function subscribeTopic(requestMessage, metadata) {
  var listeners = {
    data: [],
    end: [],
    status: []
  };
  var client = grpc.invoke(ConsensusService.subscribeTopic, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onMessage: function (responseMessage) {
      listeners.data.forEach(function (handler) {
        handler(responseMessage);
      });
    },
    onEnd: function (status, statusMessage, trailers) {
      listeners.status.forEach(function (handler) {
        handler({ code: status, details: statusMessage, metadata: trailers });
      });
      listeners.end.forEach(function (handler) {
        handler({ code: status, details: statusMessage, metadata: trailers });
      });
      listeners = null;
    }
  });
  return {
    on: function (type, handler) {
      listeners[type].push(handler);
      return this;
    },
    cancel: function () {
      listeners = null;
      client.close();
    }
  };
};

exports.ConsensusServiceClient = ConsensusServiceClient;

