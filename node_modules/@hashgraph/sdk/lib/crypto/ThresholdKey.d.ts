import { PublicKey } from "./PublicKey";
import * as proto from "../generated/BasicTypes_pb";
export declare class ThresholdKey extends PublicKey {
    private readonly _threshold;
    private readonly _keys;
    constructor(threshold: number);
    add(key: PublicKey): this;
    addAll(...keys: PublicKey[]): this;
    _toProtoKey(): proto.Key;
}
