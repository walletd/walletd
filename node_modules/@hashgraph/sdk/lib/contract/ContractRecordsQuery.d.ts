import { QueryBuilder } from "../QueryBuilder";
import { QueryHeader } from "../generated/QueryHeader_pb";
import { Query } from "../generated/Query_pb";
import { grpc } from "@improbable-eng/grpc-web";
import { Response } from "../generated/Response_pb";
import { TransactionRecord } from "../TransactionRecord";
import { ContractIdLike } from "./ContractId";
import { ResponseHeader } from "../generated/ResponseHeader_pb";
/**
 * Get all the records for a smart contract instance, for any function call
 * (or the constructor call) during the last 25 hours, for which a Record was requested.
 */
export declare class ContractRecordsQuery extends QueryBuilder<TransactionRecord[]> {
    private readonly _builder;
    constructor();
    /**
     * The smart contract instance for which the records should be retrieved.
     */
    setContractId(contractIdLike: ContractIdLike): this;
    protected _doLocalValidate(errors: string[]): void;
    protected _getMethod(): grpc.UnaryMethodDefinition<Query, Response>;
    protected _getHeader(): QueryHeader;
    protected _mapResponseHeader(response: Response): ResponseHeader;
    protected _mapResponse(response: Response): TransactionRecord[];
}
