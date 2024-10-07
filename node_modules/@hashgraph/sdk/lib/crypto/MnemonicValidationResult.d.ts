import { MnemonicValidationStatus } from "./MnemonicValidationStatus";
/**
 * The output of `toString()` is not part of the stable API; it only appears in the typedef
 * so that Typescript allows us to define it.
 */
export declare class MnemonicValidationResult {
    readonly status: MnemonicValidationStatus;
    /**
     * If not null, these are the indices in the mnemonic that were not found in the
     * BIP-39 standard English word list.
     *
     * If {@link status} is {@link MnemonicValidationStatus.UnknownWords} then this will be non-null.
     */
    readonly unknownIndices?: number[];
    constructor(status: MnemonicValidationStatus, unknownIndices?: number[]);
    isOk(): boolean;
    toString(): string;
}
