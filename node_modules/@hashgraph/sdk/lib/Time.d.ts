import { Timestamp as ProtoTimestamp } from "./generated/Timestamp_pb";
export declare class Time {
    readonly seconds: number;
    readonly nanos: number;
    constructor(seconds: number, nanos: number);
    asDate(): Date;
    static fromDate(date: number | Date): Time;
    _toProto(): ProtoTimestamp;
    static _fromProto(timestamp: ProtoTimestamp): Time;
    _increment(): Time;
}
