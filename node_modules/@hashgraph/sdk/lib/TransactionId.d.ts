import { AccountId, AccountIdLike } from "./account/AccountId";
import { TransactionID } from "./generated/BasicTypes_pb";
import { BaseClient } from "./BaseClient";
import { TransactionReceipt } from "./TransactionReceipt";
import { TransactionRecord } from "./TransactionRecord";
import { Time } from "./Time";
/**
 * Normalized transaction ID returned by various methods in the SDK.
 *
 * The ID for a transaction. This is used for retrieving receipts and records for a transaction,
 * for appending to a file right after creating it, for instantiating a smart contract with
 * bytecode in a file just created, and internally by the network for detecting when duplicate
 * transactions are submitted. A user might get a transaction processed faster by submitting it
 * to N nodes, each with a different node account, but all with the same TransactionID. Then,
 * the transaction will take effect when the first of all those nodes submits the transaction
 * and it reaches consensus. The other transactions will not take effect. So this could make the
 * transaction take effect faster, if any given node might be slow. However, the full transaction
 * fee is charged for each transaction, so the total fee is N times as much if the transaction
 * is sent to N nodes.
 */
export declare class TransactionId {
    /**
     * The Account ID that paid for this transaction.
     */
    readonly accountId: AccountId;
    /**
     * The transaction is invalid if consensusTimestamp < transactionID.transactionStartValid.
     */
    readonly validStart: Time;
    constructor(id: TransactionIdLike | AccountIdLike);
    static withValidStart(id: AccountIdLike, validStart: Time): TransactionId;
    static fromString(id: string): TransactionId;
    toString(): string;
    getReceipt(_: BaseClient): Promise<TransactionReceipt>;
    getRecord(_: BaseClient): Promise<TransactionRecord>;
    static _fromProto(id: TransactionID): TransactionId;
    _toProto(): TransactionID;
}
/**
 * Input type for an ID of a new transaction.
 */
export declare type TransactionIdLike = ({
    account: AccountIdLike;
} & ({
    validStart: Date;
} | {
    validStartSeconds: number;
    validStartNanos: number;
})) | TransactionId;
