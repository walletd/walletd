"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MirrorClient = void 0;
const grpc = require("@grpc/grpc-js");
class MirrorClient {
    constructor(endpoint) {
        this._client = new grpc.Client(endpoint, grpc.credentials.createInsecure(), { "grpc.keepalive_time_ms": 2000 });
    }
    close() {
        this._client.close();
    }
}
exports.MirrorClient = MirrorClient;
