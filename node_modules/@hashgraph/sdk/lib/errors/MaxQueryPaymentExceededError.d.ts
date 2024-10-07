import { Hbar } from "../Hbar";
export declare class MaxQueryPaymentExceededError extends Error {
    readonly queryCost: Hbar;
    readonly maxQueryPayment: Hbar;
    constructor(queryCost: Hbar, maxQueryPayment: Hbar);
}
