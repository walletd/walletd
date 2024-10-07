import { SingleTransactionBuilder } from "./TransactionBuilder";
import { FileIdLike } from "./file/FileId";
import { ContractIdLike } from "./contract/ContractId";
import { grpc } from "@improbable-eng/grpc-web";
import { Transaction } from "./generated/Transaction_pb";
import { TransactionResponse } from "./generated/TransactionResponse_pb";
/**
 * Delete a file or smart contract - can only be done with a Hedera admin multisig. When it is
 * deleted, it immediately disappears from the system as seen by the user, but is still stored
 * internally until the expiration time, at which time it is truly and permanently deleted.
 * Until that time, it can be undeleted by the Hedera admin multisig. When a smart contract is
 * deleted, the cryptocurrency account within it continues to exist, and is not affected
 * by the expiration time here.
 */
export declare class SystemDeleteTransaction extends SingleTransactionBuilder {
    private readonly _body;
    constructor();
    /**
     * The timestamp in seconds at which the "deleted" file should truly be permanently deleted.
     */
    setExpirationTime(date: number | Date): this;
    /**
     * @deprecated `.setId` is deprecated. Use `.setFileId` or `.setContractId` instead.
     */
    setId(id: FileIdLike | ContractIdLike): this;
    /**
     * The file ID of the file to delete, in the format used in transactions.
     */
    setFileId(id: FileIdLike): this;
    /**
     * The contract ID instance to delete, in the format used in transactions
     */
    setContractId(id: ContractIdLike): this;
    protected _doValidate(errors: string[]): void;
    protected get _method(): grpc.UnaryMethodDefinition<Transaction, TransactionResponse>;
}
