import { HederaStatusError } from "./HederaStatusError";
import { Status } from "../Status";
import { TransactionId } from "../TransactionId";
import { TransactionReceipt } from "../TransactionReceipt";
/**
 * Error returned when the `TransactionReceipt` has a bad status code
 */
export declare class HederaReceiptStatusError extends HederaStatusError {
    readonly transactionId: TransactionId;
    readonly receipt: TransactionReceipt;
    constructor(status: Status, receipt: TransactionReceipt, transactionId: TransactionId);
    static _throwIfError(code: number, receipt: TransactionReceipt, transactionId: TransactionId): void;
}
