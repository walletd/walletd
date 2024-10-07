import { SingleTransactionBuilder } from "../TransactionBuilder";
import { Transaction } from "../generated/Transaction_pb";
import { TransactionResponse } from "../generated/TransactionResponse_pb";
import { grpc } from "@improbable-eng/grpc-web";
import UnaryMethodDefinition = grpc.UnaryMethodDefinition;
import { ConsensusTopicIdLike } from "./ConsensusTopicId";
export declare class ConsensusTopicDeleteTransaction extends SingleTransactionBuilder {
    private _body;
    constructor();
    setTopicId(id: ConsensusTopicIdLike): this;
    protected get _method(): UnaryMethodDefinition<Transaction, TransactionResponse>;
    protected _doValidate(_: string[]): void;
}
