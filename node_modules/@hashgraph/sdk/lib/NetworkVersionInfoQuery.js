"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.NetworkVersionInfoQuery = void 0;
const QueryBuilder_1 = require("./QueryBuilder");
const NetworkGetVersionInfo_pb_1 = require("./generated/NetworkGetVersionInfo_pb");
const QueryHeader_pb_1 = require("./generated/QueryHeader_pb");
const NetworkService_pb_service_1 = require("./generated/NetworkService_pb_service");
/**
 * Get the deployed versions of Hedera Services and the HAPI proto in semantic version format.
 */
class NetworkVersionInfoQuery extends QueryBuilder_1.QueryBuilder {
    constructor() {
        super();
        this._builder = new NetworkGetVersionInfo_pb_1.NetworkGetVersionInfoQuery();
        this._builder.setHeader(new QueryHeader_pb_1.QueryHeader());
        this._inner.setNetworkgetversioninfo(this._builder);
    }
    _doLocalValidate() {
        // do nothing
    }
    _getMethod() {
        return NetworkService_pb_service_1.NetworkService.getVersionInfo;
    }
    _getHeader() {
        return this._builder.getHeader();
    }
    _mapResponseHeader(response) {
        return response.getNetworkgetversioninfo().getHeader();
    }
    _mapResponse(response) {
        const res = response.getNetworkgetversioninfo();
        const hapi = res.getHapiprotoversion();
        const hedera = res.getHederaservicesversion();
        return {
            hapiProtoVersion: {
                major: hapi.getMajor(),
                minor: hapi.getMinor(),
                patch: hapi.getPatch()
            },
            hederaServicesVersion: {
                major: hedera.getMajor(),
                minor: hedera.getMinor(),
                patch: hedera.getPatch()
            }
        };
    }
}
exports.NetworkVersionInfoQuery = NetworkVersionInfoQuery;
