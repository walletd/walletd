import { QueryBuilder } from "./QueryBuilder";
import { QueryHeader } from "./generated/QueryHeader_pb";
import { grpc } from "@improbable-eng/grpc-web";
import { Query } from "./generated/Query_pb";
import { Response } from "./generated/Response_pb";
import { ResponseHeader } from "./generated/ResponseHeader_pb";
export interface SemanticVersion {
    major: number;
    minor: number;
    patch: number;
}
export interface NetworkVersionInfo {
    hapiProtoVersion: SemanticVersion;
    hederaServicesVersion: SemanticVersion;
}
/**
 * Get the deployed versions of Hedera Services and the HAPI proto in semantic version format.
 */
export declare class NetworkVersionInfoQuery extends QueryBuilder<NetworkVersionInfo> {
    private readonly _builder;
    constructor();
    protected _doLocalValidate(): void;
    protected _getMethod(): grpc.UnaryMethodDefinition<Query, Response>;
    protected _getHeader(): QueryHeader;
    protected _mapResponseHeader(response: Response): ResponseHeader;
    protected _mapResponse(response: Response): NetworkVersionInfo;
}
