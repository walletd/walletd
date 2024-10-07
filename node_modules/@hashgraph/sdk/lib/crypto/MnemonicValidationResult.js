"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MnemonicValidationResult = void 0;
/**
 * The output of `toString()` is not part of the stable API; it only appears in the typedef
 * so that Typescript allows us to define it.
 */
class MnemonicValidationResult {
    constructor(status, unknownIndices) {
        this.status = status;
        this.unknownIndices = unknownIndices;
    }
    isOk() {
        return this.status === 0 /* Ok */;
    }
    toString() {
        switch (this.status) {
            case 0 /* Ok */:
                return "mnemonic passed validation";
            case 1 /* BadLength */:
                return "mnemonic did not contain exactly 24 words";
            case 2 /* UnknownWords */:
                return "mnemonic contained words that are not in the standard BIP-39 English word list";
            case 3 /* ChecksumMismatch */:
                return "checksum byte in mnemonic did not match the rest of the mnemonic";
            case 4 /* UnknownLegacyWords */:
                return "legacy mnemonic contained words that are not in the legacy word list";
            default:
                throw new Error(`(BUG) missing branch for status: ${this.status}`);
        }
    }
}
exports.MnemonicValidationResult = MnemonicValidationResult;
