import { SingleTransactionBuilder } from "../TransactionBuilder";
import { Transaction } from "../generated/Transaction_pb";
import { TransactionResponse } from "../generated/TransactionResponse_pb";
import { grpc } from "@improbable-eng/grpc-web";
import UnaryMethodDefinition = grpc.UnaryMethodDefinition;
import { ConsensusTopicIdLike } from "./ConsensusTopicId";
import { ChunkInfo } from "./ConsensusMessageSubmitTransaction";
export declare class SingleConsensusMessageSubmitTransaction extends SingleTransactionBuilder {
    private _body;
    constructor();
    setTopicId(id: ConsensusTopicIdLike): this;
    setMessage(message: Uint8Array | string): this;
    setChunkInfo(info: ChunkInfo): this;
    protected get _method(): UnaryMethodDefinition<Transaction, TransactionResponse>;
    protected _doValidate(_: string[]): void;
}
