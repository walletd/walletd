import { SingleTransactionBuilder } from "./TransactionBuilder";
import { grpc } from "@improbable-eng/grpc-web";
import { Transaction } from "./generated/Transaction_pb";
import { TransactionResponse } from "./generated/TransactionResponse_pb";
/**
 * Set the freezing period in which the platform will stop creating events and accepting
 * transactions. This is used before safely shut down the platform for maintenance.
 */
export declare class FreezeTransaction extends SingleTransactionBuilder {
    private readonly _body;
    constructor();
    /**
     * @param hour  The start hour (in UTC time), a value between 0 and 23.
     * @param minute  The start minute (in UTC time), a value between 0 and 59.
     */
    setStartTime(date: number | Date): this;
    setStartTime(hour: number, minute: number): this;
    /**
     * @param hour  The end hour (in UTC time), a value between 0 and 23.
     * @param minute  The end minute (in UTC time), a value between 0 and 59.
     */
    setEndTime(date: number | Date): this;
    setEndTime(hour: number, minute: number): this;
    protected _doValidate(): void;
    protected get _method(): grpc.UnaryMethodDefinition<Transaction, TransactionResponse>;
}
