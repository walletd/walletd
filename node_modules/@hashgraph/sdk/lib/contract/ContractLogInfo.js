"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.contractLogInfoListToSdk = void 0;
const ContractId_1 = require("./ContractId");
function contractLogInfoListToSdk(logInfoList) {
    return logInfoList.map((logInfo) => ({
        contractId: ContractId_1.ContractId._fromProto(logInfo.getContractid()),
        bloom: logInfo.getBloom_asU8(),
        topics: logInfo.getTopicList_asU8(),
        data: logInfo.getData_asU8()
    }));
}
exports.contractLogInfoListToSdk = contractLogInfoListToSdk;
