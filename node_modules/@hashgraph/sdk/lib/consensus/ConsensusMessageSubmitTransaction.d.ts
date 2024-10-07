import { TransactionBuilder } from "../TransactionBuilder";
import { Transaction as ProtoTransaction } from "../generated/Transaction_pb";
import { TransactionResponse } from "../generated/TransactionResponse_pb";
import { grpc } from "@improbable-eng/grpc-web";
import UnaryMethodDefinition = grpc.UnaryMethodDefinition;
import { ConsensusTopicIdLike } from "./ConsensusTopicId";
import { TransactionId } from "../TransactionId";
import { BaseClient } from "../BaseClient";
import { Transaction } from "../Transaction";
import { Hbar } from "../Hbar";
export interface ChunkInfo {
    id: TransactionId;
    number: number;
    total: number;
}
export declare class ConsensusMessageSubmitTransaction extends TransactionBuilder<Transaction[]> {
    private static readonly chunkSize;
    private _maxChunks;
    private topicId;
    private message;
    private chunkInfo;
    constructor();
    setTopicId(id: ConsensusTopicIdLike): this;
    setMessage(message: Uint8Array | string): this;
    setMaxChunks(maxChunks: number): this;
    setChunkInfo(initialId: TransactionId, total: number, num: number): this;
    build(client: BaseClient): Transaction[];
    getCost(): Promise<Hbar>;
    execute(client: BaseClient): Promise<TransactionId>;
    executeAll(client: BaseClient): Promise<TransactionId[]>;
    protected get _method(): UnaryMethodDefinition<ProtoTransaction, TransactionResponse>;
    protected _doValidate(_: string[]): void;
}
