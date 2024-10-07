import { BaseClient } from "./BaseClient";
import { TransactionBody } from "./generated/TransactionBody_pb";
import { Transaction } from "./Transaction";
import { Transaction as Transaction_ } from "./generated/Transaction_pb";
import { grpc } from "@improbable-eng/grpc-web";
import { TransactionResponse } from "./generated/TransactionResponse_pb";
import { Hbar, Tinybar } from "./Hbar";
import UnaryMethodDefinition = grpc.UnaryMethodDefinition;
import { AccountId, AccountIdLike } from "./account/AccountId";
import { TransactionId, TransactionIdLike } from "./TransactionId";
export declare abstract class TransactionBuilder<O = Transaction> {
    protected readonly _inner: TransactionBody;
    protected _shouldSetFee: boolean;
    protected _node?: AccountId;
    protected constructor();
    setTransactionId(id: TransactionIdLike): this;
    setTransactionValidDuration(seconds: number): this;
    setMaxTransactionFee(fee: Tinybar | Hbar): this;
    setNodeAccountId(nodeAccountId: AccountIdLike): this;
    setTransactionMemo(memo: string): this;
    setGenerateRecord(generateRecord: boolean): this;
    protected abstract get _method(): UnaryMethodDefinition<Transaction_, TransactionResponse>;
    protected abstract _doValidate(errors: string[]): void;
    validate(): void;
    abstract getCost(client: BaseClient): Promise<Hbar>;
    abstract build(client?: BaseClient): O;
}
export declare class SingleTransactionBuilder extends TransactionBuilder<Transaction> {
    getCost(client: BaseClient): Promise<Hbar>;
    build(client?: BaseClient): Transaction;
    execute(client: BaseClient): Promise<TransactionId>;
    protected get _method(): grpc.UnaryMethodDefinition<Transaction_, TransactionResponse>;
    protected _doValidate(_: string[]): void;
}
