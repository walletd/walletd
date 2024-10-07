"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.timestampToProto = exports.timestampToMs = exports.timestampToDate = exports.dateToTimestamp = void 0;
const Timestamp_pb_1 = require("./generated/Timestamp_pb");
function dateToTimestamp(dateOrMs) {
    const dateMs = dateOrMs instanceof Date ? dateOrMs.getTime() : dateOrMs;
    return {
        // get whole seconds since the epoch
        seconds: Math.floor(dateMs / 1000),
        // get remainder as nanoseconds
        nanos: Math.floor(dateMs % 1000 * 1000000)
    };
}
exports.dateToTimestamp = dateToTimestamp;
function timestampToDate(timestamp) {
    return new Date(timestampToMs(timestamp));
}
exports.timestampToDate = timestampToDate;
function timestampToMs(timestamp) {
    return timestamp.getSeconds() * 1000 + Math.floor(timestamp.getNanos() / 1000000);
}
exports.timestampToMs = timestampToMs;
/* eslint-disable-next-line max-len */
function timestampToProto({ seconds, nanos }) {
    const timestamp = new Timestamp_pb_1.Timestamp();
    timestamp.setSeconds(seconds);
    timestamp.setNanos(nanos);
    return timestamp;
}
exports.timestampToProto = timestampToProto;
