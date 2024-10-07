"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ConsensusTopicInfoQuery = void 0;
const QueryBuilder_1 = require("../QueryBuilder");
const QueryHeader_pb_1 = require("../generated/QueryHeader_pb");
const ConsensusService_pb_service_1 = require("../generated/ConsensusService_pb_service");
const ConsensusGetTopicInfo_pb_1 = require("../generated/ConsensusGetTopicInfo_pb");
const Time_1 = require("../Time");
const PublicKey_1 = require("../crypto/PublicKey");
const AccountId_1 = require("../account/AccountId");
const ConsensusTopicId_1 = require("./ConsensusTopicId");
class ConsensusTopicInfoQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new ConsensusGetTopicInfo_pb_1.ConsensusGetTopicInfoQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setConsensusgettopicinfo(this._builder);
    }
    setTopicId(id) {
        this._builder.setTopicid(new ConsensusTopicId_1.ConsensusTopicId(id)._toProto());
        return this;
    }
    _doLocalValidate(errors) {
        if (!this._builder.hasTopicid()) {
            errors.push(".setTopicId() required");
        }
    }
    _getMethod() {
        return ConsensusService_pb_service_1.ConsensusService.getTopicInfo;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getConsensusgettopicinfo().getHeader();
    }
    _mapResponse(response) {
        const topicInfo = response.getConsensusgettopicinfo().getTopicinfo();
        return {
            topicMemo: topicInfo.getMemo(),
            runningHash: topicInfo.getRunninghash_asU8(),
            sequenceNumber: topicInfo.getSequencenumber(),
            expirationTime: Time_1.Time._fromProto(topicInfo.getExpirationtime()),
            adminKey: topicInfo.hasAdminkey() ?
                PublicKey_1._fromProtoKey(topicInfo.getAdminkey()) :
                null,
            submitKey: topicInfo.hasSubmitkey() ?
                PublicKey_1._fromProtoKey(topicInfo.getSubmitkey()) :
                null,
            autoRenewPeriod: topicInfo.getAutorenewperiod().getSeconds(),
            autoRenewAccount: topicInfo.hasAutorenewaccount() ?
                AccountId_1.AccountId._fromProto(topicInfo.getAutorenewaccount()) :
                null
        };
    }
}
exports.ConsensusTopicInfoQuery = ConsensusTopicInfoQuery;
