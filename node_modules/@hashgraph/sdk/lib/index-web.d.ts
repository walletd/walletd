import { BaseClient, ClientConfig } from "./BaseClient";
import { grpc } from "@improbable-eng/grpc-web";
import ProtobufMessage = grpc.ProtobufMessage;
export * from "./exports";
/** This implementation of `BaseClient` is exported for browser usage. */
export declare class Client extends BaseClient {
    /**
     * If `network` is not specified, default url is a proxy to 0.testnet.hedera.com:50211 generously
     * hosted by MyHbarWallet.com. Mainnet proxy to come later.
     */
    constructor({ network, operator }: ClientConfig);
    static forMainnet(): Client;
    static forTestnet(): Client;
    static forPreviewnet(): Client;
    static fromFile(): Promise<Client>;
    static fromJson(text: string): Client;
    close(): Promise<void>;
    _unaryCall<Rq extends ProtobufMessage, Rs extends ProtobufMessage>(url: string, request: Rq, method: grpc.UnaryMethodDefinition<Rq, Rs>): Promise<Rs>;
}
export { MirrorClient } from "./mirror/web/MirrorClient";
export { MirrorConsensusTopicQuery } from "./mirror/web/MirrorConsensusTopicQuery";
