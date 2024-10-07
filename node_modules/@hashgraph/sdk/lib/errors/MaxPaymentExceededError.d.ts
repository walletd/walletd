import { Hbar } from "../Hbar";
export declare class MaxPaymentExceededError extends Error {
    readonly queryCost: Hbar;
    constructor(queryCost: Hbar, maxQueryCost: Hbar);
}
