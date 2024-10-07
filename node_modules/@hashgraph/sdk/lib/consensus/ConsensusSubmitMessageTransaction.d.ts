import { SingleTransactionBuilder } from "../TransactionBuilder";
import { Transaction } from "../generated/Transaction_pb";
import { TransactionResponse } from "../generated/TransactionResponse_pb";
import { grpc } from "@improbable-eng/grpc-web";
import UnaryMethodDefinition = grpc.UnaryMethodDefinition;
import { ConsensusTopicIdLike } from "./ConsensusTopicId";
/** @deprecated use `ConsensusMessageSubmitTransaction` instead. */
export declare class ConsensusSubmitMessageTransaction extends SingleTransactionBuilder {
    private _body;
    /** @deprecated use `ConsensusMessageSubmitTransaction` instead. */
    constructor();
    setTopicId(id: ConsensusTopicIdLike): this;
    setMessage(message: Uint8Array | string): this;
    protected get _method(): UnaryMethodDefinition<Transaction, TransactionResponse>;
    protected _doValidate(_: string[]): void;
}
