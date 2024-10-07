import { SingleTransactionBuilder } from "./TransactionBuilder";
import { FileIdLike } from "./file/FileId";
import { ContractIdLike } from "./contract/ContractId";
import { grpc } from "@improbable-eng/grpc-web";
import { Transaction } from "./generated/Transaction_pb";
import { TransactionResponse } from "./generated/TransactionResponse_pb";
/**
 * Undelete a file or smart contract that was deleted by AdminDelete - can only be done with a
 * Hedera admin multisig. When it is deleted, it immediately disappears from the system as seen
 * by the user, but is still stored internally until the expiration time, at which time it is
 * truly and permanently deleted. Until that time, it can be undeleted by the Hedera admin
 * multisig. When a smart contract is deleted, the cryptocurrency account within it continues to
 * exist, and is not affected by the expiration time here.
 */
export declare class SystemUndeleteTransaction extends SingleTransactionBuilder {
    private readonly _body;
    constructor();
    setId(id: FileIdLike | ContractIdLike): this;
    /**
     * The file ID to undelete, in the format used in transactions.
     */
    setFileId(id: FileIdLike): this;
    /**
     * The contract ID instance to undelete, in the format used in transactions
     */
    setContractId(id: ContractIdLike): this;
    protected _doValidate(errors: string[]): void;
    protected get _method(): grpc.UnaryMethodDefinition<Transaction, TransactionResponse>;
}
