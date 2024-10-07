import { TopicID } from "../generated/BasicTypes_pb";
export declare class ConsensusTopicId {
    shard: number;
    realm: number;
    topic: number;
    constructor(shard: number, realm: number, topic: number);
    constructor(topicId: ConsensusTopicIdLike);
    static fromString(id: string): ConsensusTopicId;
    static _fromProto(topicId: TopicID): ConsensusTopicId;
    toString(): string;
    static fromSolidityAddress(address: string): ConsensusTopicId;
    toSolidityAddress(): string;
    _toProto(): TopicID;
}
export declare type ConsensusTopicIdLike = {
    shard?: number;
    realm?: number;
    topic: number;
} | string | number | ConsensusTopicId;
