declare type UnsubscribeCall = () => void;
export declare class MirrorSubscriptionHandle {
    private _call;
    constructor(call?: UnsubscribeCall);
    _setCall(call: UnsubscribeCall): void;
    unsubscribe(): void;
}
export {};
