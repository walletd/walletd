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
Object.defineProperty(exports, "__esModule", { value: true });
exports.Client = void 0;
const BaseClient_1 = require("./BaseClient");
const grpc_web_1 = require("@improbable-eng/grpc-web");
var Code = grpc_web_1.grpc.Code;
__exportStar(require("./exports"), exports);
const mainnetProxy = { "https://grpc-web.myhbarwallet.com": { shard: 0, realm: 0, account: 3 } };
const testnetProxy = { "https://grpc-web.testnet.myhbarwallet.com": { shard: 0, realm: 0, account: 3 } };
const previewnetProxy = { "https://grpc-web.previewnet.myhbarwallet.com": { shard: 0, realm: 0, account: 3 } };
/** This implementation of `BaseClient` is exported for browser usage. */
class Client extends BaseClient_1.BaseClient {
    /**
     * If `network` is not specified, default url is a proxy to 0.testnet.hedera.com:50211 generously
     * hosted by MyHbarWallet.com. Mainnet proxy to come later.
     */
    constructor({ network = testnetProxy, operator }) {
        super(network, operator);
    }
    static forMainnet() {
        return new Client({ network: mainnetProxy });
    }
    static forTestnet() {
        return new Client({ network: testnetProxy });
    }
    static forPreviewnet() {
        return new Client({ network: previewnetProxy });
    }
    static fromFile() {
        throw new Error("Client.fromFile is not supported in the browser");
    }
    static fromJson(text) {
        return new Client(JSON.parse(text));
    }
    close() {
        throw new Error("Client.close is not supported in the browser");
    }
    /* eslint-disable-next-line @typescript-eslint/member-naming */
    _unaryCall(url, request, method) {
        return new Promise((resolve, reject) => grpc_web_1.grpc.unary(method, {
            host: url,
            request,
            onEnd(response) {
                if (response.status === Code.OK && response.message != null) {
                    resolve(response.message);
                }
                else {
                    reject(new Error(response.statusMessage));
                }
            }
        }));
    }
}
exports.Client = Client;
// Mirror
var MirrorClient_1 = require("./mirror/web/MirrorClient");
Object.defineProperty(exports, "MirrorClient", { enumerable: true, get: function () { return MirrorClient_1.MirrorClient; } });
var MirrorConsensusTopicQuery_1 = require("./mirror/web/MirrorConsensusTopicQuery");
Object.defineProperty(exports, "MirrorConsensusTopicQuery", { enumerable: true, get: function () { return MirrorConsensusTopicQuery_1.MirrorConsensusTopicQuery; } });
