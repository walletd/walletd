export declare enum HashAlgorithm {
    Sha256 = "SHA-256",
    Sha384 = "SHA-384",
    Sha512 = "SHA-512"
}
export declare class Hmac {
    static hash(algorithm: HashAlgorithm, secretKey: Uint8Array | string, data: Uint8Array | string): Promise<Uint8Array>;
}
