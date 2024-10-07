import { SingleTransactionBuilder } from "../TransactionBuilder";
import { Transaction } from "../generated/Transaction_pb";
import { TransactionResponse } from "../generated/TransactionResponse_pb";
import { grpc } from "@improbable-eng/grpc-web";
import UnaryMethodDefinition = grpc.UnaryMethodDefinition;
import { PublicKey } from "../crypto/PublicKey";
import { AccountIdLike } from "../account/AccountId";
export declare class ConsensusTopicCreateTransaction extends SingleTransactionBuilder {
    private _body;
    constructor();
    setAdminKey(key: PublicKey): this;
    /**
     * @deprecated `ConsensusTopicUpdateTransaction.setAutoRenewAccount()`
     * use `ConsensusTopicUpdateTransaction.setAutoRenewAccountId()` instead.
     */
    setAutoRenewAccount(id: AccountIdLike): this;
    setAutoRenewAccountId(id: AccountIdLike): this;
    setAutoRenewPeriod(seconds: number): this;
    setSubmitKey(key: PublicKey): this;
    setTopicMemo(memo: string): this;
    protected get _method(): UnaryMethodDefinition<Transaction, TransactionResponse>;
    protected _doValidate(_: string[]): void;
}
