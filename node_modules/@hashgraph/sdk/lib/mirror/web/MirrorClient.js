"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MirrorClient = void 0;
class MirrorClient {
    constructor(endpoint) {
        this.endpoint = endpoint;
    }
    close() {
        console.warn("Close is not implememented for the web version of `MirrorClient`");
    }
}
exports.MirrorClient = MirrorClient;
