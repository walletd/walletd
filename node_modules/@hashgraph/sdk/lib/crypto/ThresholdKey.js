"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ThresholdKey = void 0;
const PublicKey_1 = require("./PublicKey");
const proto = require("../generated/BasicTypes_pb");
class ThresholdKey extends PublicKey_1.PublicKey {
    constructor(threshold) {
        super();
        this._keys = [];
        this._threshold = threshold;
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
        if (this._keys.length === 0) {
            throw new Error("ThresholdKey must have at least one key");
        }
        if (this._threshold > this._keys.length) {
            throw new Error("ThresholdKey must have at least as many keys as threshold: " +
                `${this._threshold}; # of keys currently: ${this._keys.length}`);
        }
        const keyList = new proto.KeyList();
        keyList.setKeysList(this._keys);
        const thresholdKey = new proto.ThresholdKey();
        thresholdKey.setThreshold(this._threshold);
        thresholdKey.setKeys(keyList);
        const protoKey = new proto.Key();
        protoKey.setThresholdkey(thresholdKey);
        return protoKey;
    }
}
exports.ThresholdKey = ThresholdKey;
