import { ConsensusTopicResponse } from "../generated/MirrorConsensusService_pb";
import { Time } from "../Time";
export declare class ConsensusMessageChunk {
    readonly consensusTimestamp: Time;
    readonly runningHash: Uint8Array;
    readonly sequenceNumber: number;
    readonly contentSize: number;
    constructor(consensusTimestamp: Time, runningHash: Uint8Array, sequenceNumber: number, contentSize: number);
}
export declare class MirrorConsensusTopicResponse {
    /**
     * The time at which the transaction reached consensus
     */
    readonly consensusTimestamp: Time;
    /**
     * The message body originally in the ConsensusSubmitMessageTransactionBody.
     *  Message size will be less than 4K.
     */
    readonly message: Uint8Array;
    /**
     * The running hash (SHA384) of every message.
     */
    readonly runningHash: Uint8Array;
    /**
     * Starts at 1 for first submitted message. Incremented on each submitted message.
     */
    readonly sequenceNumber: number;
    readonly chunks: ConsensusMessageChunk[] | null;
    constructor(message: ConsensusTopicResponse[]);
    constructor(message: ConsensusTopicResponse);
    toString(): string;
}
