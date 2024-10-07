import { ConsensusTopicQuery } from "../generated/MirrorConsensusService_pb";
import { ConsensusTopicIdLike } from "../consensus/ConsensusTopicId";
import { MirrorConsensusTopicResponse } from "./MirrorConsensusTopicResponse";
export declare type Listener = (message: MirrorConsensusTopicResponse) => void;
export declare type ErrorHandler = (error: Error) => void;
export declare class BaseMirrorConsensusTopicQuery {
    protected readonly _builder: ConsensusTopicQuery;
    setTopicId(id: ConsensusTopicIdLike): this;
    setStartTime(start: number | Date): this;
    setEndTime(start: number | Date): this;
    setLimit(limit: number): this;
    _validate(): void;
}
