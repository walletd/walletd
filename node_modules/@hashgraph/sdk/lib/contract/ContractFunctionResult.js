"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ContractFunctionResult = void 0;
const ContractLogInfo_1 = require("./ContractLogInfo");
const pb = require("../generated/ContractCallLocal_pb");
const ContractId_1 = require("./ContractId");
const bignumber_js_1 = require("bignumber.js");
const hex = require("@stablelib/hex");
const utf8 = require("@stablelib/utf8");
/**
 * The result returned by a call to a smart contract function. This is part of the response to
 * a ContractCallLocal query, and is in the record for a ContractCall or ContractCreateInstance
 * transaction. The ContractCreateInstance transaction record has the results of the call to
 * the constructor.
 */
class ContractFunctionResult {
    // Constructor isn't part of the stable API
    constructor(result) {
        if (result instanceof pb.ContractFunctionResult) {
            this.bytes = result.getContractcallresult_asU8();
            this.contractId = result.hasContractid() ?
                ContractId_1.ContractId._fromProto(result.getContractid()) :
                null;
            this.errorMessage = result.getErrormessage();
            this.bloom = result.getBloom_asU8();
            this.gasUsed = result.getGasused();
            this.logs = ContractLogInfo_1.contractLogInfoListToSdk(result.getLoginfoList());
        }
        else {
            this.contractId = new ContractId_1.ContractId(0);
            this.bytes = result;
            this.errorMessage = "";
            this.bloom = new Uint8Array();
            this.gasUsed = 0;
            this.logs = [];
        }
    }
    asBytes() {
        return this.bytes;
    }
    getString(index) {
        return utf8.decode(this.getBytes(index));
    }
    getBytes(index) {
        // Len should never be larger than Number.MAX
        // index * 32 is the position of the lenth
        // (index + 1) * 32 onward to (index + 1) * 32 + len will be the elements of the array
        // Arrays in solidity cannot be longer than 1024:
        // https://solidity.readthedocs.io/en/v0.4.21/introduction-to-smart-contracts.html
        const offset = this.getInt32(index);
        const len = new DataView(this.bytes.buffer, this.bytes.byteOffset + offset + 28, 4).getInt32(0);
        return this.bytes.subarray(offset + 32, offset + 32 + len);
    }
    getBytes32(index) {
        return this.bytes.subarray((index !== null && index !== void 0 ? index : 0) * 32, (index !== null && index !== void 0 ? index : 0) * 32 + 32);
    }
    getBool(index) {
        return this.bytes[(index !== null && index !== void 0 ? index : 0) * 32 + 31] !== 0;
    }
    getInt8(index) {
        return this.bytes[(index !== null && index !== void 0 ? index : 0) * 32 + 31];
    }
    getInt32(index) {
        // .getUint32() interprets as big-endian
        // Using DataView instead of Uint32Array because the latter interprets
        // using platform endianness which is little-endian on x86
        return new DataView(this.bytes.buffer, this.bytes.byteOffset + (index !== null && index !== void 0 ? index : 0) * 32 + 28, 4).getInt32(0);
    }
    getInt64(index) {
        return new bignumber_js_1.default(hex.encode(this._getBytes32(index !== null && index !== void 0 ? index : 0).subarray(24, 32), true), 16);
    }
    getInt256(index) {
        return new bignumber_js_1.default(hex.encode(this._getBytes32(index !== null && index !== void 0 ? index : 0), true), 16);
    }
    getUint8(index) {
        return this.bytes[(index !== null && index !== void 0 ? index : 0) * 32 + 31];
    }
    getUint32(index) {
        // .getUint32() interprets as big-endian
        // Using DataView instead of Uint32Array because the latter interprets
        // using platform endianness which is little-endian on x86
        return new DataView(this.bytes.buffer, this.bytes.byteOffset + (index !== null && index !== void 0 ? index : 0) * 32 + 28, 4).getUint32(0);
    }
    getUint64(index) {
        return new bignumber_js_1.default(hex.encode(this._getBytes32(index).subarray(24, 32), true), 16);
    }
    getUint256(index) {
        return new bignumber_js_1.default(hex.encode(this._getBytes32(index), true), 16);
    }
    getAddress(index) {
        return hex.encode(this.bytes.subarray((index !== null && index !== void 0 ? index : 0) * 32 + 12, (index !== null && index !== void 0 ? index : 0) * 32 + 32), true);
    }
    //
    //  NOT A STABLE API
    //
    _getBytes32(index) {
        return this.bytes.subarray((index !== null && index !== void 0 ? index : 0) * 32, (index !== null && index !== void 0 ? index : 0) * 32 + 32);
    }
}
exports.ContractFunctionResult = ContractFunctionResult;
