"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.hbarFromTinybarOrHbar = exports.Hbar = exports.hbarCheck = exports.hbarToProtoValue = exports.hbarToProto = void 0;
const bignumber_js_1 = require("bignumber.js");
const HbarRangeError_1 = require("./errors/HbarRangeError");
const HbarUnit_1 = require("./HbarUnit");
const wrappers_pb_1 = require("google-protobuf/google/protobuf/wrappers_pb");
const hbarTinybar = Symbol("hbarTinybar");
exports.hbarToProto = Symbol("hbarToProto");
exports.hbarToProtoValue = Symbol("hbarToProtoValue");
exports.hbarCheck = Symbol("hbarCheck");
function convertToTinybar(amount, unit) {
    const bnAmount = bignumber_js_1.default.isBigNumber(amount) ? amount : new bignumber_js_1.default(amount);
    return bnAmount.multipliedBy(unit._toTinybarCount());
}
const maxTinybar = new bignumber_js_1.default(2).pow(63).minus(1);
const maxHbar = maxTinybar.dividedBy(HbarUnit_1.HbarUnit.Hbar._toTinybarCount());
const minTinybar = new bignumber_js_1.default(-2).pow(63);
const minHbar = minTinybar.dividedBy(HbarUnit_1.HbarUnit.Hbar._toTinybarCount());
/**
 * Typesafe wrapper for values of HBAR providing foolproof conversions to other denominations.
 */
class Hbar {
    constructor(amount) {
        const bnAmount = amount instanceof bignumber_js_1.default ? amount : new bignumber_js_1.default(amount);
        if (bnAmount.isZero()) {
            this[hbarTinybar] = bnAmount;
        }
        else {
            this[hbarTinybar] = bnAmount.multipliedBy(HbarUnit_1.HbarUnit.Hbar._toTinybarCount());
            this[exports.hbarCheck]({ allowNegative: true });
        }
        // See `Hbar.fromTinybar()` as to why this is done
        if (typeof amount === "number" && amount >= Math.pow(2, 53)) {
            throw new HbarRangeError_1.HbarRangeError(this);
        }
    }
    /**
     * Calculate the HBAR amount given a raw value and a unit.
     */
    static from(amount, unit) {
        const bnAmount = new bignumber_js_1.default(amount);
        const hbar = new Hbar(0);
        hbar[hbarTinybar] = bnAmount.multipliedBy(unit._toTinybarCount());
        return hbar;
    }
    /** Get HBAR from a tinybar amount, may be a string */
    static fromTinybar(amount) {
        const bnAmount = new bignumber_js_1.default(amount);
        const hbar = new Hbar(0);
        hbar[hbarTinybar] = bnAmount;
        // Check if amount is out of range after hbar is constructed
        // Technically we're able to successfully construct Hbar from 2 ** 53,
        // but at that point the number is out of range for a js `number` type
        // so we throw an error to indicate this. If someone wants to use values
        // 2 ** 53 and higher then they shhould wrap the number in BigNumber.
        if (typeof amount === "number" && amount >= Math.pow(2, 53)) {
            throw new HbarRangeError_1.HbarRangeError(hbar);
        }
        return hbar;
    }
    /**
     * Wrap a raw value of HBAR, may be a string.
     * @deprecate Use constructor instead. `new Hbar(amount)`
     */
    static of(amount) {
        console.warn("`Hbar.of` is deprecated. Use `new Hbar(amount)` instead.");
        return new Hbar(amount);
    }
    // Create an Hbar with a value of 0 tinybar; Note that this is a positive signed zero
    //
    // @deprecate `Hbar.zero() is deprecated. If you want to use `Hbar.zero()` for
    // comparisions then use `Hbar.ZERO` static field, otherwise use `new Hbar(0)`.
    static zero() {
        console.warn(`\`Hbar.zero()\` is deprecated. If you want to use \`Hbar.zero()\` for 
comparisions then use \`Hbar.ZERO\` static field, otherwise use \`new Hbar(0)\``);
        return new Hbar(new bignumber_js_1.default(0));
    }
    toString() {
        return this.value().toString();
    }
    value() {
        return this.as(HbarUnit_1.HbarUnit.Hbar);
    }
    asTinybar() {
        return this.as(HbarUnit_1.HbarUnit.Tinybar);
    }
    as(unit) {
        if (unit.toString() === HbarUnit_1.HbarUnit.Tinybar.toString()) {
            return this[hbarTinybar];
        }
        return this[hbarTinybar].dividedBy(unit._toTinybarCount());
    }
    multipliedBy(amount) {
        return new Hbar(this[hbarTinybar].multipliedBy(amount)
            .dividedBy(HbarUnit_1.HbarUnit.Hbar._toTinybarCount()));
    }
    plus(amount, unit) {
        return new Hbar((amount instanceof Hbar ?
            this[hbarTinybar].plus(amount[hbarTinybar]) :
            this[hbarTinybar].plus(convertToTinybar(amount, unit))).dividedBy(HbarUnit_1.HbarUnit.Hbar._toTinybarCount()));
    }
    minus(amount, unit) {
        return new Hbar((amount instanceof Hbar ?
            this[hbarTinybar].minus(amount[hbarTinybar]) :
            this[hbarTinybar].minus(convertToTinybar(amount, unit))).dividedBy(HbarUnit_1.HbarUnit.Hbar._toTinybarCount()));
    }
    isEqualTo(amount, unit) {
        return amount instanceof Hbar ?
            this[hbarTinybar].isEqualTo(amount[hbarTinybar]) :
            this[hbarTinybar].isEqualTo(convertToTinybar(amount, unit));
    }
    isGreaterThan(amount, unit) {
        return amount instanceof Hbar ?
            this[hbarTinybar].isGreaterThan(amount[hbarTinybar]) :
            this[hbarTinybar].isGreaterThan(convertToTinybar(amount, unit));
    }
    isGreaterThanOrEqualTo(amount, unit) {
        return amount instanceof Hbar ?
            this[hbarTinybar].isGreaterThanOrEqualTo(amount[hbarTinybar]) :
            this[hbarTinybar].isGreaterThanOrEqualTo(convertToTinybar(amount, unit));
    }
    isLessThan(amount, unit) {
        return amount instanceof Hbar ?
            this[hbarTinybar].isLessThan(amount[hbarTinybar]) :
            this[hbarTinybar].isLessThan(convertToTinybar(amount, unit));
    }
    isLessThanOrEqualTo(amount, unit) {
        return amount instanceof Hbar ?
            this[hbarTinybar].isLessThanOrEqualTo(amount[hbarTinybar]) :
            this[hbarTinybar].isLessThanOrEqualTo(convertToTinybar(amount, unit));
    }
    comparedTo(amount, unit) {
        return amount instanceof Hbar ?
            this[hbarTinybar].comparedTo(amount[hbarTinybar]) :
            this[hbarTinybar].comparedTo(convertToTinybar(amount, unit));
    }
    isZero() {
        return this[hbarTinybar].isZero();
    }
    negated() {
        return Hbar.fromTinybar(this[hbarTinybar].negated());
    }
    isNegative() {
        return this[hbarTinybar].isNegative();
    }
    isPositive() {
        return this[hbarTinybar].isPositive();
    }
    [exports.hbarCheck]({ allowNegative }) {
        const tinybar = this[hbarTinybar];
        if (tinybar.isNegative() && !allowNegative && tinybar.isLessThan(maxTinybar)) {
            throw new HbarRangeError_1.HbarRangeError(this);
        }
        if (tinybar.isGreaterThan(maxTinybar)) {
            throw new HbarRangeError_1.HbarRangeError(this);
        }
    }
    [exports.hbarToProto]() {
        return String(this[hbarTinybar]);
    }
    [exports.hbarToProtoValue]() {
        const value = new wrappers_pb_1.UInt64Value();
        value.setValue(this[hbarTinybar].toNumber());
        return value;
    }
}
exports.Hbar = Hbar;
Hbar.MAX = new Hbar(maxHbar);
Hbar.MIN = new Hbar(minHbar);
Hbar.ZERO = new Hbar(0);
function hbarFromTinybarOrHbar(number) {
    if (number instanceof Hbar) {
        return number;
    }
    return Hbar.fromTinybar(new bignumber_js_1.default(number));
}
exports.hbarFromTinybarOrHbar = hbarFromTinybarOrHbar;
