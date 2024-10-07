/**
 * @param {string} algorithm
 * @param {Uint8Array} key
 * @param {Uint8Array} iv
 * @param {Uint8Array} data
 * @returns {Promise<Uint8Array>}
 */
export function createCipheriv(algorithm: string, key: Uint8Array, iv: Uint8Array, data: Uint8Array): Promise<Uint8Array>;
/**
 * @param {string} algorithm
 * @param {Uint8Array} key
 * @param {Uint8Array} iv
 * @param {Uint8Array} data
 * @returns {Promise<Uint8Array>}
 */
export function createDecipheriv(algorithm: string, key: Uint8Array, iv: Uint8Array, data: Uint8Array): Promise<Uint8Array>;
export namespace CipherAlgorithm {
    const Aes128Ctr: string;
    const Aes128Cbc: string;
}
