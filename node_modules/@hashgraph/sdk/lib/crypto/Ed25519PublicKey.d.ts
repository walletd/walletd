import { Key } from "../generated/BasicTypes_pb";
import { PublicKey } from "./PublicKey";
export declare class Ed25519PublicKey implements PublicKey {
    private readonly _keyData;
    private _asStringRaw?;
    private constructor();
    static fromBytes(keyData: Uint8Array): Ed25519PublicKey;
    static fromString(keyStr: string): Ed25519PublicKey;
    toBytes(): Uint8Array;
    toString(raw?: boolean): string;
    _toProtoKey(): Key;
    _bytesEqual(bytes: Uint8Array): boolean;
}
