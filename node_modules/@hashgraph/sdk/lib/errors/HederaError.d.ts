export declare type ResponseCode = number;
/**
 * Class of errors for response codes returned from Hedera.
 * @deprecate This error is no longer in use in the sdk. `HederaStatusError` is used instead.
 */
export declare class HederaError extends Error {
    /** The numerical code */
    readonly code: ResponseCode;
    /** The name of the code from the protobufs, or 'UNKNOWN STATUS CODE (4120)' */
    readonly codeName: string;
    constructor(code: ResponseCode);
}
