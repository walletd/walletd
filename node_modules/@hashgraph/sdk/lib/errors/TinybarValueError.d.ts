import BigNumber from "bignumber.js";
import { Hbar } from "../Hbar";
export declare class TinybarValueError extends Error {
    readonly amount: BigNumber;
    constructor(message: string, amount: number | BigNumber | Hbar);
}
