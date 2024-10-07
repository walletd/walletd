"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ContractFunctionSelector = exports.ArgumentType = void 0;
const js_sha3_1 = require("js-sha3");
var ArgumentType;
(function (ArgumentType) {
    ArgumentType[ArgumentType["uint8"] = 0] = "uint8";
    ArgumentType[ArgumentType["int8"] = 1] = "int8";
    ArgumentType[ArgumentType["uint16"] = 2] = "uint16";
    ArgumentType[ArgumentType["int16"] = 3] = "int16";
    ArgumentType[ArgumentType["uint32"] = 4] = "uint32";
    ArgumentType[ArgumentType["int32"] = 5] = "int32";
    ArgumentType[ArgumentType["uint64"] = 6] = "uint64";
    ArgumentType[ArgumentType["int64"] = 7] = "int64";
    ArgumentType[ArgumentType["uint256"] = 8] = "uint256";
    ArgumentType[ArgumentType["int256"] = 9] = "int256";
    ArgumentType[ArgumentType["string"] = 10] = "string";
    ArgumentType[ArgumentType["bool"] = 11] = "bool";
    ArgumentType[ArgumentType["bytes"] = 12] = "bytes";
    ArgumentType[ArgumentType["bytes32"] = 13] = "bytes32";
    ArgumentType[ArgumentType["address"] = 14] = "address";
    ArgumentType[ArgumentType["func"] = 15] = "func";
})(ArgumentType = exports.ArgumentType || (exports.ArgumentType = {}));
class ContractFunctionSelector {
    constructor(name = null) {
        this._name = null;
        this._params = "";
        // Unstable
        this._paramTypes = [];
        if (name != null) {
            this._name = name;
        }
    }
    addString() {
        return this._addParam({ ty: ArgumentType.string, array: false });
    }
    addStringArray() {
        return this._addParam({ ty: ArgumentType.string, array: true });
    }
    addBytes() {
        return this._addParam({ ty: ArgumentType.bytes, array: false });
    }
    addBytes32() {
        return this._addParam({ ty: ArgumentType.bytes32, array: false });
    }
    addBytesArray() {
        return this._addParam({ ty: ArgumentType.bytes, array: true });
    }
    addBytes32Array() {
        return this._addParam({ ty: ArgumentType.bytes32, array: true });
    }
    addInt8() {
        return this._addParam({ ty: ArgumentType.int8, array: false });
    }
    addInt32() {
        return this._addParam({ ty: ArgumentType.int32, array: false });
    }
    addInt64() {
        return this._addParam({ ty: ArgumentType.int64, array: false });
    }
    addInt256() {
        return this._addParam({ ty: ArgumentType.int256, array: false });
    }
    addInt8Array() {
        return this._addParam({ ty: ArgumentType.int8, array: true });
    }
    addInt32Array() {
        return this._addParam({ ty: ArgumentType.int32, array: true });
    }
    addInt64Array() {
        return this._addParam({ ty: ArgumentType.int64, array: true });
    }
    addInt256Array() {
        return this._addParam({ ty: ArgumentType.int256, array: true });
    }
    addUint8() {
        return this._addParam({ ty: ArgumentType.uint8, array: false });
    }
    addUint32() {
        return this._addParam({ ty: ArgumentType.uint32, array: false });
    }
    addUint64() {
        return this._addParam({ ty: ArgumentType.uint64, array: false });
    }
    addUint256() {
        return this._addParam({ ty: ArgumentType.uint256, array: false });
    }
    addUint8Array() {
        return this._addParam({ ty: ArgumentType.uint8, array: true });
    }
    addUint32Array() {
        return this._addParam({ ty: ArgumentType.uint32, array: true });
    }
    addUint64Array() {
        return this._addParam({ ty: ArgumentType.uint64, array: true });
    }
    addUint256Array() {
        return this._addParam({ ty: ArgumentType.uint256, array: true });
    }
    addBool() {
        return this._addParam({ ty: ArgumentType.bool, array: false });
    }
    addAddress() {
        return this._addParam({ ty: ArgumentType.address, array: false });
    }
    addAddressArray() {
        return this._addParam({ ty: ArgumentType.address, array: true });
    }
    addFunction() {
        return this._addParam({ ty: ArgumentType.func, array: false });
    }
    _addParam(ty) {
        if (this._paramTypes.length > 0) {
            this._params += ",";
        }
        this._params += solidityTypeToString(ty);
        this._paramTypes.push(ty);
        return this;
    }
    /**
     * NOT A STABLE API
     */
    _build(name) {
        if (name != null) {
            this._name = name;
        }
        else if (this._name == null) {
            throw new Error("`name` required for ContractFunctionSelector");
        }
        return new Uint8Array(js_sha3_1.keccak256.arrayBuffer(this.toString()).slice(0, 4));
    }
    toString() {
        return `${this._name}(${this._params})`;
    }
}
exports.ContractFunctionSelector = ContractFunctionSelector;
function solidityTypeToString(ty) {
    let s = "";
    switch (ty.ty) {
        case ArgumentType.uint8:
            s = "uint8";
            break;
        case ArgumentType.int8:
            s = "int8";
            break;
        case ArgumentType.uint16:
            s = "uint16";
            break;
        case ArgumentType.int16:
            s = "int16";
            break;
        case ArgumentType.uint32:
            s = "uint32";
            break;
        case ArgumentType.int32:
            s = "int32";
            break;
        case ArgumentType.uint64:
            s = "uint64";
            break;
        case ArgumentType.int64:
            s = "int64";
            break;
        case ArgumentType.uint256:
            s = "uint256";
            break;
        case ArgumentType.int256:
            s = "int256";
            break;
        case ArgumentType.string:
            s = "string";
            break;
        case ArgumentType.bool:
            s = "bool";
            break;
        case ArgumentType.bytes:
            s = "bytes";
            break;
        case ArgumentType.bytes32:
            s = "bytes32";
            break;
        case ArgumentType.address:
            s = "address";
            break;
        case ArgumentType.func:
            s = "function";
            break;
        default:
            s = "";
            break;
    }
    if (ty.array) {
        s += "[]";
    }
    return s;
}
