"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.MirrorSubscriptionHandle = void 0;
class MirrorSubscriptionHandle {
    constructor(call) {
        this._call = null;
        if (call != null) {
            this._call = call;
        }
    }
    // NOT A STABLE API
    _setCall(call) {
        this._call = call;
    }
    unsubscribe() {
        if (this._call != null) {
            this._call();
        }
    }
}
exports.MirrorSubscriptionHandle = MirrorSubscriptionHandle;
