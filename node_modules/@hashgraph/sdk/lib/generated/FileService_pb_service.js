// package: proto
// file: FileService.proto

var FileService_pb = require("./FileService_pb");
var Query_pb = require("./Query_pb");
var Response_pb = require("./Response_pb");
var TransactionResponse_pb = require("./TransactionResponse_pb");
var Transaction_pb = require("./Transaction_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var FileService = (function () {
  function FileService() {}
  FileService.serviceName = "proto.FileService";
  return FileService;
}());

FileService.createFile = {
  methodName: "createFile",
  service: FileService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

FileService.updateFile = {
  methodName: "updateFile",
  service: FileService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

FileService.deleteFile = {
  methodName: "deleteFile",
  service: FileService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

FileService.appendContent = {
  methodName: "appendContent",
  service: FileService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

FileService.getFileContent = {
  methodName: "getFileContent",
  service: FileService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

FileService.getFileInfo = {
  methodName: "getFileInfo",
  service: FileService,
  requestStream: false,
  responseStream: false,
  requestType: Query_pb.Query,
  responseType: Response_pb.Response
};

FileService.systemDelete = {
  methodName: "systemDelete",
  service: FileService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

FileService.systemUndelete = {
  methodName: "systemUndelete",
  service: FileService,
  requestStream: false,
  responseStream: false,
  requestType: Transaction_pb.Transaction,
  responseType: TransactionResponse_pb.TransactionResponse
};

exports.FileService = FileService;

function FileServiceClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

FileServiceClient.prototype.createFile = function createFile(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(FileService.createFile, {
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

FileServiceClient.prototype.updateFile = function updateFile(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(FileService.updateFile, {
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

FileServiceClient.prototype.deleteFile = function deleteFile(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(FileService.deleteFile, {
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

FileServiceClient.prototype.appendContent = function appendContent(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(FileService.appendContent, {
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

FileServiceClient.prototype.getFileContent = function getFileContent(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(FileService.getFileContent, {
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

FileServiceClient.prototype.getFileInfo = function getFileInfo(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(FileService.getFileInfo, {
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

FileServiceClient.prototype.systemDelete = function systemDelete(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(FileService.systemDelete, {
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

FileServiceClient.prototype.systemUndelete = function systemUndelete(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(FileService.systemUndelete, {
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

exports.FileServiceClient = FileServiceClient;

