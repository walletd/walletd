import { QueryBuilder } from "./QueryBuilder";
import { grpc } from "@improbable-eng/grpc-web";
import { Query } from "./generated/Query_pb";
import { Response } from "./generated/Response_pb";
import { QueryHeader } from "./generated/QueryHeader_pb";
import { FileId } from "./file/FileId";
import { ContractId } from "./contract/ContractId";
import { AccountId } from "./account/AccountId";
import { ResponseHeader } from "./generated/ResponseHeader_pb";
export declare type EntityId = {
    type: "ACCOUNT";
    accountId: AccountId;
} | {
    type: "CONTRACT";
    contractId: ContractId;
} | {
    type: "FILE";
    fileId: FileId;
};
/**
 * Get the IDs in the format used by transactions, given the ID in the format used by Solidity.
 * If the Solidity ID is for a smart contract instance, then both the ContractID and
 * associated AccountID will be returned.
 */
export declare class GetBySolidityIdQuery extends QueryBuilder<EntityId> {
    private readonly _builder;
    constructor();
    /**
     * The ID in the format used by Solidity.
     */
    setSolidityId(id: string): this;
    protected _doLocalValidate(): void;
    protected _getHeader(): QueryHeader;
    protected _getMethod(): grpc.UnaryMethodDefinition<Query, Response>;
    protected _mapResponseHeader(response: Response): ResponseHeader;
    protected _mapResponse(response: Response): EntityId;
}
