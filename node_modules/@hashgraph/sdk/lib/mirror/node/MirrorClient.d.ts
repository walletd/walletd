import * as grpc from "@grpc/grpc-js";
export declare class MirrorClient {
    readonly _client: grpc.Client;
    constructor(endpoint: string);
    close(): void;
}
