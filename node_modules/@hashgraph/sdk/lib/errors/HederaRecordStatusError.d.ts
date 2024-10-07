import { HederaStatusError } from "./HederaStatusError";
import { Status } from "../Status";
import { TransactionId } from "../TransactionId";
import { TransactionRecord } from "../TransactionRecord";
/**
 * Error returned when the `TransactionRecord` has a bad status code
 */
export declare class HederaRecordStatusError extends HederaStatusError {
    readonly transactionId: TransactionId;
    readonly record: TransactionRecord;
    constructor(status: Status, record: TransactionRecord, transactionId: TransactionId);
    static _throwIfError(code: number, record: TransactionRecord, transactionId: TransactionId): void;
}
