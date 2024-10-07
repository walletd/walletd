"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.KeyList = void 0;
const proto = require("../generated/BasicTypes_pb");
class KeyList {
    constructor() {
        this._keys = [];
    }
    add(key) {
        this._keys.push(key._toProtoKey());
        return this;
    }
    addAll(...keys) {
        this._keys.push(...keys.map((key) => key._toProtoKey()));
        return this;
    }
    /* eslint-disable-next-line @typescript-eslint/member-naming */
    _toProtoKey() {
        const keyList = new proto.KeyList();
        keyList.setKeysList(this._keys);
        const protoKey = new proto.Key();
        protoKey.setKeylist(keyList);
        return protoKey;
    }
}
exports.KeyList = KeyList;
