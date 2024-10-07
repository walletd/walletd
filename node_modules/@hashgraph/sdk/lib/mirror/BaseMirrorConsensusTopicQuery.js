"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BaseMirrorConsensusTopicQuery = void 0;
const MirrorConsensusService_pb_1 = require("../generated/MirrorConsensusService_pb");
const ConsensusTopicId_1 = require("../consensus/ConsensusTopicId");
const Time_1 = require("../Time");
const LocalValidationError_1 = require("../errors/LocalValidationError");
class BaseMirrorConsensusTopicQuery {
    constructor() {
        this._builder = new MirrorConsensusService_pb_1.ConsensusTopicQuery();
    }
    setTopicId(id) {
        this._builder.setTopicid(new ConsensusTopicId_1.ConsensusTopicId(id)._toProto());
        return this;
    }
    setStartTime(start) {
        this._builder.setConsensusstarttime(Time_1.Time.fromDate(start)._toProto());
        return this;
    }
    setEndTime(start) {
        this._builder.setConsensusendtime(Time_1.Time.fromDate(start)._toProto());
        return this;
    }
    setLimit(limit) {
        this._builder.setLimit(limit);
        return this;
    }
    // NOT A STABLE API
    _validate() {
        if (!this._builder.hasTopicid()) {
            throw new LocalValidationError_1.LocalValidationError("MirrorConsensusTopicQuery", ["`.setTopicId()` is required to be called"]);
        }
    }
}
exports.BaseMirrorConsensusTopicQuery = BaseMirrorConsensusTopicQuery;
