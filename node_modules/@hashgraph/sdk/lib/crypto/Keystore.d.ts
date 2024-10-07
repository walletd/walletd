import { RawKeyPair } from "./RawKeyPair";
declare const AES_128_CTR = "aes-128-ctr";
declare const HMAC_SHA256 = "hmac-sha256";
export interface Keystore {
    version: 1;
    crypto: {
        /** hex-encoded ciphertext */
        ciphertext: string;
        /** hex-encoded initialization vector */
        cipherparams: {
            iv: string;
        };
        /** cipher being used */
        cipher: typeof AES_128_CTR;
        /** key derivation function being used */
        kdf: "pbkdf2";
        /** params for key derivation function */
        kdfparams: {
            /** derived key length */
            dkLen: number;
            /** hex-encoded salt */
            salt: string;
            /** iteration count */
            c: number;
            /** hash function */
            prf: typeof HMAC_SHA256;
        };
        /** hex-encoded HMAC-SHA384 */
        mac: string;
    };
}
export declare function createKeystore(privateKey: Uint8Array, passphrase: string): Promise<Uint8Array>;
export declare function loadKeystore(keystoreBytes: Uint8Array, passphrase: string): Promise<RawKeyPair>;
export {};
