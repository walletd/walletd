import { BaseClient, ClientConfig } from "./BaseClient";
import { grpc as grpcWeb } from "@improbable-eng/grpc-web";
import ProtobufMessage = grpcWeb.ProtobufMessage;
import UnaryMethodDefinition = grpcWeb.UnaryMethodDefinition;
export * from "./exports";
/**
 * This implementation of `BaseClient` is exported for Node.js usage.
 */
export declare class Client extends BaseClient {
    private readonly _nodeClients;
    /** If `nodes` is not specified, the Hedera public testnet is assumed. */
    constructor({ network, operator }: ClientConfig);
    static forMainnet(): Client;
    static forTestnet(): Client;
    static forPreviewnet(): Client;
    static fromFile(filename: string): Promise<Client>;
    static fromJson(text: string): Client;
    close(): void;
    _unaryCall<Rq extends ProtobufMessage, Rs extends ProtobufMessage>(url: string, request: Rq, method: UnaryMethodDefinition<Rq, Rs>): Promise<Rs>;
}
export { MirrorClient } from "./mirror/node/MirrorClient";
export { MirrorConsensusTopicQuery } from "./mirror/node/MirrorConsensusTopicQuery";
