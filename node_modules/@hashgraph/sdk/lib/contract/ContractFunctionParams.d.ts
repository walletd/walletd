import BigNumber from "bignumber.js";
import { ContractFunctionSelector } from "./ContractFunctionSelector";
export declare class ContractFunctionParams {
    private readonly _selector;
    private readonly _arguments;
    constructor();
    addString(value: string): this;
    addStringArray(value: string[]): this;
    addBytes(value: Uint8Array): this;
    addBytes32(value: Uint8Array): this;
    addBytesArray(value: Uint8Array[]): this;
    addBytes32Array(value: Uint8Array[]): this;
    addBool(value: boolean): this;
    addInt8(value: number): this;
    addInt32(value: number): this;
    addInt64(value: BigNumber): this;
    addInt256(value: BigNumber): this;
    addInt8Array(value: number[]): this;
    addInt32Array(value: number[]): this;
    addInt64Array(value: BigNumber[]): this;
    addInt256Array(value: BigNumber[]): this;
    addUint8(value: number): this;
    addUint32(value: number): this;
    addUint64(value: BigNumber): this;
    addUint256(value: BigNumber): this;
    addUint8Array(value: number[]): this;
    addUint32Array(value: number[]): this;
    addUint64Array(value: BigNumber[]): this;
    addUint256Array(value: BigNumber[]): this;
    addAddress(value: string): this;
    addAddressArray(value: string[]): this;
    addFunction(address: string, selector: ContractFunctionSelector): this;
    private _addParam;
    /**
     * NOT A STABLE API
     */
    _build(name: string | null): Uint8Array;
}
