import { QueryBuilder } from "./QueryBuilder";
import { QueryHeader } from "./generated/QueryHeader_pb";
import { TransactionIdLike } from "./TransactionId";
import { grpc } from "@improbable-eng/grpc-web";
import { Query } from "./generated/Query_pb";
import { Response } from "./generated/Response_pb";
import { TransactionReceipt } from "./TransactionReceipt";
import { ResponseHeader } from "./generated/ResponseHeader_pb";
import { Status } from "./Status";
export declare class TransactionReceiptQuery extends QueryBuilder<TransactionReceipt> {
    private readonly _builder;
    constructor();
    setTransactionId(txId: TransactionIdLike): this;
    protected _doLocalValidate(errors: string[]): void;
    protected _getMethod(): grpc.UnaryMethodDefinition<Query, Response>;
    protected _shouldRetry(status: Status, response: Response): boolean;
    protected _getDefaultExecuteTimeout(): number;
    protected _isPaymentRequired(): boolean;
    protected _getHeader(): QueryHeader;
    protected _mapResponseHeader(response: Response): ResponseHeader;
    protected _mapResponse(response: Response): TransactionReceipt;
}
