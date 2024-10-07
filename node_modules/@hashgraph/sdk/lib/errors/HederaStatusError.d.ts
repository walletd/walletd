import { Status } from "../Status";
/**
 * Class of errors for response codes returned from Hedera.
 */
export declare class HederaStatusError extends Error {
    readonly status: Status;
    constructor(status: Status);
}
