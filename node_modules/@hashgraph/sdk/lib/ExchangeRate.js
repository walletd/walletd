"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.exchangeRateSetToSdk = exports.exchangeRateToSdk = void 0;
function exchangeRateToSdk(exchangeRate) {
    return {
        hbarEquiv: exchangeRate.getHbarequiv(),
        centEquiv: exchangeRate.getCentequiv(),
        expirationTime: new Date(exchangeRate.getExpirationtime().getSeconds())
    };
}
exports.exchangeRateToSdk = exchangeRateToSdk;
function exchangeRateSetToSdk(exchangeRateSet) {
    return {
        currentRate: exchangeRateToSdk(exchangeRateSet.getCurrentrate()),
        nextRate: exchangeRateToSdk(exchangeRateSet.getCurrentrate())
    };
}
exports.exchangeRateSetToSdk = exchangeRateSetToSdk;
