import { PublicKey } from "./PublicKey";
import * as proto from "../generated/BasicTypes_pb";
export declare class KeyList {
    private readonly _keys;
    add(key: PublicKey): this;
    addAll(...keys: PublicKey[]): this;
    _toProtoKey(): proto.Key;
}
