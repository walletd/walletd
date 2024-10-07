"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HbarUnit = void 0;
const bignumber_js_1 = require("bignumber.js");
class HbarUnit {
    constructor(unit) {
        this._unit = unit;
    }
    getSymbol() {
        switch (this._unit) {
            case "tinybar": return "tℏ";
            case "microbar": return "μℏ";
            case "millibar": return "mℏ";
            case "hbar": return "ℏ";
            case "kilobar": return "kℏ";
            case "megabar": return "Mℏ";
            case "gigabar": return "Gℏ";
            default: throw new TypeError("HbarUnit was not a valid value");
        }
    }
    _toTinybarCount() {
        switch (this._unit) {
            case "tinybar": return new bignumber_js_1.default(1);
            case "microbar": return new bignumber_js_1.default(100);
            case "millibar": return new bignumber_js_1.default(100000);
            case "hbar": return new bignumber_js_1.default(100000000);
            case "kilobar": return new bignumber_js_1.default(100000000).multipliedBy(1000);
            case "megabar": return new bignumber_js_1.default(100000000).multipliedBy(1000000);
            case "gigabar": return new bignumber_js_1.default(100000000).multipliedBy(1000000000);
            default: throw new TypeError("HbarUnit was not a valid value");
        }
    }
    toString() {
        return this._unit;
    }
}
exports.HbarUnit = HbarUnit;
HbarUnit.Tinybar = new HbarUnit("tinybar");
HbarUnit.Microbar = new HbarUnit("microbar");
HbarUnit.Millibar = new HbarUnit("millibar");
HbarUnit.Hbar = new HbarUnit("hbar");
HbarUnit.Kilobar = new HbarUnit("kilobar");
HbarUnit.Megabar = new HbarUnit("megabar");
HbarUnit.Gigabar = new HbarUnit("gigabar");
