// package: proto
// file: ConsensusService.proto

var ConsensusService_pb = require("./ConsensusService_pb");
var Query_pb = require("./Query_pb");
var Response_pb = require("./Response_pb");
var TransactionResponse_pb = require("./TransactionResponse_pb");
var Transaction_pb = require("./Transaction_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var ConsensusService = (function () {
  function ConsensusService() {}
  ConsensusService.serviceName = "proto.ConsensusService";
  return ConsensusService;
}());

ConsensusService.createTopic = {
  methodName: "createTopic",
  service: ConsensusService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

ConsensusService.updateTopic = {
  methodName: "updateTopic",
  service: ConsensusService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

ConsensusService.deleteTopic = {
  methodName: "deleteTopic",
  service: ConsensusService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

ConsensusService.getTopicInfo = {
  methodName: "getTopicInfo",
  service: ConsensusService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

ConsensusService.submitMessage = {
  methodName: "submitMessage",
  service: ConsensusService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

exports.ConsensusService = ConsensusService;

function ConsensusServiceClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

ConsensusServiceClient.prototype.createTopic = function createTopic(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(ConsensusService.createTopic, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

ConsensusServiceClient.prototype.updateTopic = function updateTopic(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(ConsensusService.updateTopic, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

ConsensusServiceClient.prototype.deleteTopic = function deleteTopic(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(ConsensusService.deleteTopic, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

ConsensusServiceClient.prototype.getTopicInfo = function getTopicInfo(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(ConsensusService.getTopicInfo, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

ConsensusServiceClient.prototype.submitMessage = function submitMessage(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(ConsensusService.submitMessage, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

exports.ConsensusServiceClient = ConsensusServiceClient;

