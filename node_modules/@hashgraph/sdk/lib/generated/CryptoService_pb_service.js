// package: proto
// file: CryptoService.proto

var CryptoService_pb = require("./CryptoService_pb");
var Query_pb = require("./Query_pb");
var Response_pb = require("./Response_pb");
var TransactionResponse_pb = require("./TransactionResponse_pb");
var Transaction_pb = require("./Transaction_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var CryptoService = (function () {
  function CryptoService() {}
  CryptoService.serviceName = "proto.CryptoService";
  return CryptoService;
}());

CryptoService.createAccount = {
  methodName: "createAccount",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

CryptoService.updateAccount = {
  methodName: "updateAccount",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

CryptoService.cryptoTransfer = {
  methodName: "cryptoTransfer",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

CryptoService.cryptoDelete = {
  methodName: "cryptoDelete",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

CryptoService.addLiveHash = {
  methodName: "addLiveHash",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

CryptoService.deleteLiveHash = {
  methodName: "deleteLiveHash",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

CryptoService.getLiveHash = {
  methodName: "getLiveHash",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

CryptoService.getAccountRecords = {
  methodName: "getAccountRecords",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

CryptoService.cryptoGetBalance = {
  methodName: "cryptoGetBalance",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

CryptoService.getAccountInfo = {
  methodName: "getAccountInfo",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

CryptoService.getTransactionReceipts = {
  methodName: "getTransactionReceipts",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

CryptoService.getFastTransactionRecord = {
  methodName: "getFastTransactionRecord",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

CryptoService.getTxRecordByTxID = {
  methodName: "getTxRecordByTxID",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

CryptoService.getStakersByAccountID = {
  methodName: "getStakersByAccountID",
  service: CryptoService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

exports.CryptoService = CryptoService;

function CryptoServiceClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

CryptoServiceClient.prototype.createAccount = function createAccount(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.createAccount, {
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

CryptoServiceClient.prototype.updateAccount = function updateAccount(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.updateAccount, {
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

CryptoServiceClient.prototype.cryptoTransfer = function cryptoTransfer(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.cryptoTransfer, {
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

CryptoServiceClient.prototype.cryptoDelete = function cryptoDelete(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.cryptoDelete, {
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

CryptoServiceClient.prototype.addLiveHash = function addLiveHash(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.addLiveHash, {
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

CryptoServiceClient.prototype.deleteLiveHash = function deleteLiveHash(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.deleteLiveHash, {
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

CryptoServiceClient.prototype.getLiveHash = function getLiveHash(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.getLiveHash, {
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

CryptoServiceClient.prototype.getAccountRecords = function getAccountRecords(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.getAccountRecords, {
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

CryptoServiceClient.prototype.cryptoGetBalance = function cryptoGetBalance(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.cryptoGetBalance, {
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

CryptoServiceClient.prototype.getAccountInfo = function getAccountInfo(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.getAccountInfo, {
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

CryptoServiceClient.prototype.getTransactionReceipts = function getTransactionReceipts(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.getTransactionReceipts, {
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

CryptoServiceClient.prototype.getFastTransactionRecord = function getFastTransactionRecord(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.getFastTransactionRecord, {
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

CryptoServiceClient.prototype.getTxRecordByTxID = function getTxRecordByTxID(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.getTxRecordByTxID, {
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

CryptoServiceClient.prototype.getStakersByAccountID = function getStakersByAccountID(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(CryptoService.getStakersByAccountID, {
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

exports.CryptoServiceClient = CryptoServiceClient;

