"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Time = void 0;
const Timestamp_pb_1 = require("./generated/Timestamp_pb");
class Time {
    constructor(seconds, nanos) {
        this.seconds = seconds;
        this.nanos = nanos;
    }
    asDate() {
        return new Date(this.seconds * 1000 + Math.floor(this.nanos / 1000000));
    }
    static fromDate(date) {
        let ms;
        if (typeof date === "number") {
            ms = date;
        }
        else if (date instanceof Date) {
            ms = date.getTime();
        }
        else {
            throw new TypeError(`Invalid type ${JSON.stringify(date)} is not 'number' or 'Date'`);
        }
        const seconds = Math.floor(ms / 1000);
        const nanos = Math.floor(ms % 1000) * 1000000;
        return new Time(seconds, nanos);
    }
    _toProto() {
        const proto = new Timestamp_pb_1.Timestamp();
        proto.setSeconds(this.seconds);
        proto.setNanos(this.nanos);
        return proto;
    }
    static _fromProto(timestamp) {
        return new Time(timestamp.getSeconds(), timestamp.getNanos());
    }
    _increment() {
        if (Math.floor(this.nanos + 1) === 1000000000) {
            return new Time(this.seconds + 1, 0);
        }
        return new Time(this.seconds, this.nanos + 1);
    }
}
exports.Time = Time;
