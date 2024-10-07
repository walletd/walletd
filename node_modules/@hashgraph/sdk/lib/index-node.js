"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !exports.hasOwnProperty(p)) __createBinding(exports, m, p);
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Client = void 0;
const BaseClient_1 = require("./BaseClient");
const fs = require("fs");
const util = require("util");
const grpc = require("@grpc/grpc-js");
const exports_1 = require("./exports");
__exportStar(require("./exports"), exports);
const readFile = util.promisify(fs.readFile);
const testNet = { "0.testnet.hedera.com:50211": { shard: 0, realm: 0, account: 3 } };
const mainnetNodes = {
    "35.237.200.180:50211": "0.0.3",
    "35.186.191.247:50211": "0.0.4",
    "35.192.2.25:50211": "0.0.5",
    "35.199.161.108:50211": "0.0.6",
    "35.203.82.240:50211": "0.0.7",
    "35.236.5.219:50211": "0.0.8",
    "35.197.192.225:50211": "0.0.9",
    "35.242.233.154:50211": "0.0.10",
    "35.240.118.96:50211": "0.0.11",
    "35.204.86.32:50211": "0.0.12"
};
const testnetNodes = {
    "0.testnet.hedera.com:50211": "0.0.3",
    "1.testnet.hedera.com:50211": "0.0.4",
    "2.testnet.hedera.com:50211": "0.0.5",
    "3.testnet.hedera.com:50211": "0.0.6"
};
const previewnetNodes = {
    "0.previewnet.hedera.com:50211": "0.0.3",
    "1.previewnet.hedera.com:50211": "0.0.4",
    "2.previewnet.hedera.com:50211": "0.0.5",
    "3.previewnet.hedera.com:50211": "0.0.6"
};
/**
 * This implementation of `BaseClient` is exported for Node.js usage.
 */
class Client extends BaseClient_1.BaseClient {
    /** If `nodes` is not specified, the Hedera public testnet is assumed. */
    constructor({ network = testNet, operator }) {
        super(network, operator);
        this._nodeClients = Object.keys(network).reduce((prev, url) => (Object.assign({ [url]: new grpc.Client(url, grpc.credentials.createInsecure()) }, prev)), {});
    }
    static forMainnet() {
        return new Client({ network: mainnetNodes });
    }
    static forTestnet() {
        return new Client({ network: testnetNodes });
    }
    static forPreviewnet() {
        return new Client({ network: previewnetNodes });
    }
    static fromFile(filename) {
        return __awaiter(this, void 0, void 0, function* () {
            return Client.fromJson(yield readFile(filename, "utf8"));
        });
    }
    static fromJson(text) {
        return new Client(JSON.parse(text));
    }
    close() {
        for (const client of Object.values(this._nodeClients)) {
            client.close();
        }
    }
    /* eslint-disable-next-line @typescript-eslint/member-naming */
    _unaryCall(url, request, method) {
        return new Promise((resolve, reject) => this._nodeClients[url].makeUnaryRequest(
        // this gRPC client takes the full path
        `/${method.service.serviceName}/${method.methodName}`, (req) => Buffer.from(req.serializeBinary()), (bytes) => method.responseType.deserializeBinary(bytes), request, new grpc.Metadata(), {}, (err, val) => {
            if (err != null) {
                reject(err);
            }
            else {
                resolve(val);
            }
        }));
    }
}
exports.Client = Client;
// Mirror
var MirrorClient_1 = require("./mirror/node/MirrorClient");
Object.defineProperty(exports, "MirrorClient", { enumerable: true, get: function () { return MirrorClient_1.MirrorClient; } });
var MirrorConsensusTopicQuery_1 = require("./mirror/node/MirrorConsensusTopicQuery");
Object.defineProperty(exports, "MirrorConsensusTopicQuery", { enumerable: true, get: function () { return MirrorConsensusTopicQuery_1.MirrorConsensusTopicQuery; } });
// Override console.log output for some classes (to be toString)
for (const cls of [
    exports_1.TransactionReceipt,
    exports_1.AccountId,
    exports_1.FileId,
    exports_1.ConsensusTopicId,
    exports_1.ContractId,
    exports_1.TransactionId,
    exports_1.Ed25519PrivateKey,
    exports_1.Ed25519PublicKey,
    exports_1.Status,
    exports_1.Hbar
]) {
    Object.defineProperty(cls.prototype, util.inspect.custom, {
        enumerable: false,
        writable: false,
        value() {
            return this.toString();
        }
    });
}
