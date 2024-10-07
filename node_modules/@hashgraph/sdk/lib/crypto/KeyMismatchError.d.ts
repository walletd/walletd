export declare class KeyMismatchError extends Error {
    private readonly _hmac;
    private readonly _expectedHmac;
    constructor(hmac: Uint8Array, expectedHmac: Uint8Array);
}
