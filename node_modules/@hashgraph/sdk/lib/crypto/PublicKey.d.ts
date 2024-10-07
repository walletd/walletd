import * as pb from "../generated/BasicTypes_pb";
export declare abstract class PublicKey {
    abstract _toProtoKey(): pb.Key;
}
export declare function _fromProtoKey(key: pb.Key): PublicKey;
export declare function _fromProtoKeyList(keys: pb.KeyList): PublicKey[];
