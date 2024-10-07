import { BaseMirrorConsensusTopicQuery, ErrorHandler, Listener } from "../BaseMirrorConsensusTopicQuery";
import { MirrorSubscriptionHandle } from "../MirrorSubscriptionHandle";
import { MirrorClient } from "./MirrorClient";
export declare class MirrorConsensusTopicQuery extends BaseMirrorConsensusTopicQuery {
    subscribe(client: MirrorClient, listener: Listener, errorHandler?: ErrorHandler): MirrorSubscriptionHandle;
    private _makeServerStreamRequest;
}
