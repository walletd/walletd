"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getResponseHeader = exports.runValidation = exports.reqDefined = exports.timeoutPromise = exports.setTimeoutAwaitable = exports.getSdkKeys = exports.normalizeEntityId = exports.newDuration = exports.orThrow = void 0;
const Duration_pb_1 = require("./generated/Duration_pb");
const Response_pb_1 = require("./generated/Response_pb");
const LocalValidationError_1 = require("./errors/LocalValidationError");
const Ed25519PublicKey_1 = require("./crypto/Ed25519PublicKey");
function orThrow(val, msg = "value must not be null") {
    return reqDefined(val, msg);
}
exports.orThrow = orThrow;
function newDuration(seconds) {
    if (!Number.isSafeInteger(seconds)) {
        throw new TypeError("duration cannot have fractional seconds");
    }
    const duration = new Duration_pb_1.Duration();
    duration.setSeconds(seconds);
    return duration;
}
exports.newDuration = newDuration;
function normalizeEntityId(kind, entityId) {
    switch (typeof entityId) {
        case "object":
            if (entityId[kind] == null) {
                break;
            }
            return Object.assign({ 
                // defaults overwritten if they exist in `entityId`
                shard: 0, realm: 0 }, entityId);
        case "string": {
            const components = entityId.split(".");
            if (components.length === 1) {
                const id = { [kind]: Number(components[0]) };
                return normalizeEntityId(kind, id);
            }
            else if (components.length === 3) {
                return {
                    shard: Number(components[0]),
                    realm: Number(components[1]),
                    [kind]: Number(components[2])
                };
            }
            break;
        }
        case "number": {
            if (!Number.isInteger(entityId) || entityId < 0) {
                break;
            }
            else if (!Number.isSafeInteger(entityId)) {
                // this isn't really a `TypeError` as we already checked that it is a `number`
                // eslint-disable-next-line unicorn/prefer-type-error
                throw new Error(`${kind} ID outside safe integer range for number: ${entityId}`);
            }
            return normalizeEntityId(kind, { [kind]: entityId });
        }
        default:
    }
    throw new Error(`invalid ${kind} ID: ${entityId}`);
}
exports.normalizeEntityId = normalizeEntityId;
function getSdkKeys(keylist) {
    return keylist.getKeysList().map((key) => Ed25519PublicKey_1.Ed25519PublicKey.fromBytes(key.getEd25519_asU8()));
}
exports.getSdkKeys = getSdkKeys;
function setTimeoutAwaitable(timeoutMs) {
    return new Promise((resolve) => setTimeout(resolve, timeoutMs));
}
exports.setTimeoutAwaitable = setTimeoutAwaitable;
function timeoutPromise(ms, promise, timedOutCallback) {
    const timeout = new Promise((resolve, reject) => {
        setTimeout(() => {
            timedOutCallback(reject);
        }, ms);
    });
    return Promise.race([promise, timeout]);
}
exports.timeoutPromise = timeoutPromise;
function reqDefined(val, msg) {
    if (val == null) {
        throw new Error(msg);
    }
    return val;
}
exports.reqDefined = reqDefined;
function runValidation(instance, doValidate) {
    const errors = [];
    doValidate(errors);
    if (errors.length > 0) {
        throw new LocalValidationError_1.LocalValidationError(instance.constructor.name, errors);
    }
}
exports.runValidation = runValidation;
function getResponseHeader(response) {
    switch (response.getResponseCase()) {
        case Response_pb_1.Response.ResponseCase.RESPONSE_NOT_SET:
            throw new Error(`expected body for query response: ${response.toString()}`);
        case Response_pb_1.Response.ResponseCase.GETBYKEY:
            return response.getGetbykey().getHeader();
        case Response_pb_1.Response.ResponseCase.GETBYSOLIDITYID:
            return response.getGetbysolidityid().getHeader();
        case Response_pb_1.Response.ResponseCase.CONTRACTCALLLOCAL:
            return response.getContractcalllocal().getHeader();
        case Response_pb_1.Response.ResponseCase.CONTRACTGETBYTECODERESPONSE:
            return response.getContractgetbytecoderesponse().getHeader();
        case Response_pb_1.Response.ResponseCase.CONTRACTGETINFO:
            return response.getContractgetinfo().getHeader();
        case Response_pb_1.Response.ResponseCase.CONTRACTGETRECORDSRESPONSE:
            return response.getContractgetrecordsresponse().getHeader();
        case Response_pb_1.Response.ResponseCase.CRYPTOGETACCOUNTBALANCE:
            return response.getCryptogetaccountbalance().getHeader();
        case Response_pb_1.Response.ResponseCase.CRYPTOGETACCOUNTRECORDS:
            return response.getCryptogetaccountrecords().getHeader();
        case Response_pb_1.Response.ResponseCase.CRYPTOGETINFO:
            return response.getCryptogetinfo().getHeader();
        case Response_pb_1.Response.ResponseCase.CRYPTOGETPROXYSTAKERS:
            return response.getCryptogetproxystakers().getHeader();
        case Response_pb_1.Response.ResponseCase.FILEGETCONTENTS:
            return response.getFilegetcontents().getHeader();
        case Response_pb_1.Response.ResponseCase.FILEGETINFO:
            return response.getFilegetinfo().getHeader();
        case Response_pb_1.Response.ResponseCase.TRANSACTIONGETRECEIPT:
            return response.getTransactiongetreceipt().getHeader();
        case Response_pb_1.Response.ResponseCase.TRANSACTIONGETRECORD:
            return response.getTransactiongetrecord().getHeader();
        case Response_pb_1.Response.ResponseCase.TRANSACTIONGETFASTRECORD:
            return response.getTransactiongetfastrecord().getHeader();
        default:
            throw new Error(`unsupported response case ${response.getResponseCase()}`);
    }
}
exports.getResponseHeader = getResponseHeader;
