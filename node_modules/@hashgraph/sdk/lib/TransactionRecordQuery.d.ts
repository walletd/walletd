import { QueryBuilder } from "./QueryBuilder";
import { QueryHeader } from "./generated/QueryHeader_pb";
import { TransactionIdLike } from "./TransactionId";
import { grpc } from "@improbable-eng/grpc-web";
import { Query } from "./generated/Query_pb";
import { Response } from "./generated/Response_pb";
import { TransactionRecord } from "./TransactionRecord";
import { ResponseHeader } from "./generated/ResponseHeader_pb";
/**
 * Get the record for a transaction. If the transaction requested a record, then the record lasts
 * for one hour, and a state proof is available for it. If the transaction created an account, file,
 * or smart contract instance, then the record will contain the ID for what it created. If the
 * transaction called a smart contract function, then the record contains the result of that call.
 * If the transaction was a cryptocurrency transfer, then the record includes the TransferList which
 * gives the details of that transfer. If the transaction didn't return anything that should be in
 * the record, then the results field will be set to nothing.
 */
export declare class TransactionRecordQuery extends QueryBuilder<TransactionRecord> {
    private readonly _builder;
    constructor();
    /**
     * The ID of the transaction for which the record is requested.
     */
    setTransactionId(txId: TransactionIdLike): this;
    protected _doLocalValidate(errors: string[]): void;
    protected _getHeader(): QueryHeader;
    protected _getMethod(): grpc.UnaryMethodDefinition<Query, Response>;
    protected _mapResponseHeader(response: Response): ResponseHeader;
    protected _mapResponse(response: Response): TransactionRecord;
}
