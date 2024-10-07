export declare type AsnSeq = {
    seq: AsnType[];
};
export declare type AsnInt = {
    int: number;
};
export declare type AsnBytes = {
    bytes: Uint8Array;
};
export declare type AsnIdent = {
    ident: string;
};
export declare type AsnNull = {};
export declare type AsnType = AsnSeq | AsnInt | AsnBytes | AsnIdent | AsnNull;
/**
 * Note: may throw weird errors on malformed input. Catch and rethrow with, e.g. `BadKeyError`.
 */
export declare function decodeDer(derBytes: Uint8Array): AsnType;
