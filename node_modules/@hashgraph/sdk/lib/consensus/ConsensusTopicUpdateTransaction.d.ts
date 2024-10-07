import { SingleTransactionBuilder } from "../TransactionBuilder";
import { Transaction } from "../generated/Transaction_pb";
import { TransactionResponse } from "../generated/TransactionResponse_pb";
import { grpc } from "@improbable-eng/grpc-web";
import UnaryMethodDefinition = grpc.UnaryMethodDefinition;
import { PublicKey } from "../crypto/PublicKey";
import { Time } from "../Time";
import { AccountIdLike } from "../account/AccountId";
import { ConsensusTopicIdLike } from "./ConsensusTopicId";
export declare class ConsensusTopicUpdateTransaction extends SingleTransactionBuilder {
    private _body;
    constructor();
    clearTopicMemo(): this;
    clearAdminKey(): this;
    clearSubmitKey(): this;
    clearAutoRenewAccount(): this;
    setAdminKey(key: PublicKey): this;
    /**
     * @deprecated `ConsensusTopicCreateTransaction.setAutoRenewAccount()`
     * use `ConsensusTopicCreateTransaction.setAutoRenewAccountId()` instead.
     */
    setAutoRenewAccount(id: AccountIdLike): this;
    setAutoRenewAccountId(id: AccountIdLike): this;
    setAutoRenewPeriod(seconds: number): this;
    setExpirationTime(time: Time): this;
    setSubmitKey(key: PublicKey): this;
    setTopicId(id: ConsensusTopicIdLike): this;
    setTopicMemo(memo: string): this;
    protected get _method(): UnaryMethodDefinition<Transaction, TransactionResponse>;
    protected _doValidate(_: string[]): void;
}
