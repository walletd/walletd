/// <reference types="node" />
import { HashAlgorithm } from "./Hmac";
import * as crypto from "crypto";
export declare const pbkdf2: (arg1: crypto.BinaryLike, arg2: crypto.BinaryLike, arg3: number, arg4: number, arg5: string) => Promise<Buffer>;
export declare class Pbkdf2 {
    static deriveKey(algorithm: HashAlgorithm, password: Uint8Array | string, salt: Uint8Array | string, iterations: number, length: number): Promise<Uint8Array>;
}
