import { QueryBuilder } from "../QueryBuilder";
import { grpc } from "@improbable-eng/grpc-web";
import { Query } from "../generated/Query_pb";
import { QueryHeader } from "../generated/QueryHeader_pb";
import { Response } from "../generated/Response_pb";
import { ResponseHeader } from "../generated/ResponseHeader_pb";
import { Time } from "../Time";
import { PublicKey } from "../crypto/PublicKey";
import { AccountId } from "../account/AccountId";
import { ConsensusTopicIdLike } from "./ConsensusTopicId";
export interface ConsensusTopicInfo {
    topicMemo: string;
    runningHash: Uint8Array;
    sequenceNumber: number;
    expirationTime: Time;
    adminKey: PublicKey | null;
    submitKey: PublicKey | null;
    autoRenewPeriod: number;
    autoRenewAccount: AccountId | null;
}
export declare class ConsensusTopicInfoQuery extends QueryBuilder<ConsensusTopicInfo> {
    private readonly _builder;
    constructor();
    setTopicId(id: ConsensusTopicIdLike): this;
    protected _doLocalValidate(errors: string[]): void;
    protected _getMethod(): grpc.UnaryMethodDefinition<Query, Response>;
    protected _getHeader(): QueryHeader;
    protected _mapResponseHeader(response: Response): ResponseHeader;
    protected _mapResponse(response: Response): ConsensusTopicInfo;
}
