import { HederaStatusError } from "./HederaStatusError";
import { Status } from "../Status";
import { TransactionId } from "../TransactionId";
/**
 * Error returned when precheck fails with a bad status code
 */
export declare class HederaPrecheckStatusError extends HederaStatusError {
    readonly transactionId: TransactionId;
    constructor(status: Status, transactionId: TransactionId);
    static _throwIfError(code: number, transactionId: TransactionId): void;
}
