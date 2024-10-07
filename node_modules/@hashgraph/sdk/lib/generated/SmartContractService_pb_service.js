// package: proto
// file: SmartContractService.proto

var SmartContractService_pb = require("./SmartContractService_pb");
var TransactionResponse_pb = require("./TransactionResponse_pb");
var Query_pb = require("./Query_pb");
var Response_pb = require("./Response_pb");
var Transaction_pb = require("./Transaction_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var SmartContractService = (function () {
  function SmartContractService() {}
  SmartContractService.serviceName = "proto.SmartContractService";
  return SmartContractService;
}());

SmartContractService.createContract = {
  methodName: "createContract",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

SmartContractService.updateContract = {
  methodName: "updateContract",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

SmartContractService.contractCallMethod = {
  methodName: "contractCallMethod",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

SmartContractService.getContractInfo = {
  methodName: "getContractInfo",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

SmartContractService.contractCallLocalMethod = {
  methodName: "contractCallLocalMethod",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

SmartContractService.ContractGetBytecode = {
  methodName: "ContractGetBytecode",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

SmartContractService.getBySolidityID = {
  methodName: "getBySolidityID",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

SmartContractService.getTxRecordByContractID = {
  methodName: "getTxRecordByContractID",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

SmartContractService.deleteContract = {
  methodName: "deleteContract",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

SmartContractService.systemDelete = {
  methodName: "systemDelete",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

SmartContractService.systemUndelete = {
  methodName: "systemUndelete",
  service: SmartContractService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

exports.SmartContractService = SmartContractService;

function SmartContractServiceClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

SmartContractServiceClient.prototype.createContract = function createContract(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.createContract, {
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

SmartContractServiceClient.prototype.updateContract = function updateContract(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.updateContract, {
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

SmartContractServiceClient.prototype.contractCallMethod = function contractCallMethod(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.contractCallMethod, {
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

SmartContractServiceClient.prototype.getContractInfo = function getContractInfo(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.getContractInfo, {
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

SmartContractServiceClient.prototype.contractCallLocalMethod = function contractCallLocalMethod(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.contractCallLocalMethod, {
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

SmartContractServiceClient.prototype.contractGetBytecode = function contractGetBytecode(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.ContractGetBytecode, {
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

SmartContractServiceClient.prototype.getBySolidityID = function getBySolidityID(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.getBySolidityID, {
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

SmartContractServiceClient.prototype.getTxRecordByContractID = function getTxRecordByContractID(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.getTxRecordByContractID, {
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

SmartContractServiceClient.prototype.deleteContract = function deleteContract(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.deleteContract, {
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

SmartContractServiceClient.prototype.systemDelete = function systemDelete(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.systemDelete, {
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

SmartContractServiceClient.prototype.systemUndelete = function systemUndelete(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(SmartContractService.systemUndelete, {
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

exports.SmartContractServiceClient = SmartContractServiceClient;

