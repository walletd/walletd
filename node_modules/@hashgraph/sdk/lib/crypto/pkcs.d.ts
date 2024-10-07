import { AsnType } from "./der";
export declare class AlgorithmIdentifier {
    readonly algIdent: string;
    readonly parameters?: AsnType;
    constructor(asn: AsnType);
    toString(): string;
}
export declare class PrivateKeyInfo {
    readonly version: 0;
    readonly algId: AlgorithmIdentifier;
    readonly privateKey: Uint8Array;
    constructor(asn: AsnType);
    static parse(encoded: Uint8Array): PrivateKeyInfo;
}
export declare class EncryptedPrivateKeyInfo {
    readonly algId: AlgorithmIdentifier;
    readonly data: Uint8Array;
    constructor(asn: AsnType);
    static parse(encoded: Uint8Array): EncryptedPrivateKeyInfo;
    decrypt(passphrase: string): Promise<PrivateKeyInfo>;
}
