"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TransactionId = void 0;
const AccountId_1 = require("./account/AccountId");
const BasicTypes_pb_1 = require("./generated/BasicTypes_pb");
const util_1 = require("./util");
const Timestamp_pb_1 = require("./generated/Timestamp_pb");
const Timestamp_1 = require("./Timestamp");
const Time_1 = require("./Time");
/**
 * Normalized transaction ID returned by various methods in the SDK.
 *
 * The ID for a transaction. This is used for retrieving receipts and records for a transaction,
 * for appending to a file right after creating it, for instantiating a smart contract with
 * bytecode in a file just created, and internally by the network for detecting when duplicate
 * transactions are submitted. A user might get a transaction processed faster by submitting it
 * to N nodes, each with a different node account, but all with the same TransactionID. Then,
 * the transaction will take effect when the first of all those nodes submits the transaction
 * and it reaches consensus. The other transactions will not take effect. So this could make the
 * transaction take effect faster, if any given node might be slow. However, the full transaction
 * fee is charged for each transaction, so the total fee is N times as much if the transaction
 * is sent to N nodes.
 */
class TransactionId {
    constructor(id) {
        // Cannot use try/catch here because test die horribly
        // eslint-disable-next-line @typescript-eslint/ban-ts-ignore
        // @ts-ignore
        // eslint-disable-next-line dot-notation
        if (!id["validStart"] && !id["validStartSeconds"]) {
            this.accountId = new AccountId_1.AccountId(id);
            const { seconds, nanos } = getIncreasingInstant();
            this.validStart = new Time_1.Time(seconds, nanos);
        }
        else {
            const transactionId = id;
            if (transactionId instanceof TransactionId) {
                this.accountId = transactionId.accountId;
                this.validStart = new Time_1.Time(transactionId.validStart.seconds, transactionId.validStart.nanos);
            }
            else {
                this.accountId = new AccountId_1.AccountId(transactionId.account);
                if ("validStart" in transactionId) {
                    const { seconds, nanos } = Timestamp_1.dateToTimestamp(transactionId.validStart);
                    this.validStart = new Time_1.Time(seconds, nanos);
                }
                else {
                    this.validStart = new Time_1.Time(transactionId.validStartSeconds, transactionId.validStartNanos);
                }
            }
        }
    }
    static withValidStart(id, validStart) {
        return new TransactionId({
            account: id,
            validStartSeconds: validStart.seconds,
            validStartNanos: validStart.nanos
        });
    }
    static fromString(id) {
        const [account, time] = id.split("@");
        const [seconds, nanos] = time.split(".");
        return new TransactionId({
            account: new AccountId_1.AccountId(account),
            validStartSeconds: Number(seconds),
            validStartNanos: Number(nanos)
        });
    }
    toString() {
        return `${this.accountId.toString()}@${this.validStart.seconds}.${this.validStart.nanos}`;
    }
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    getReceipt(_) {
        return new Promise((_, reject) => {
            reject(new Error("(BUG) `TransactionId.getReceipt()` declared, but not overwritten."));
        });
    }
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    getRecord(_) {
        return new Promise((_, reject) => {
            reject(new Error("(BUG) `TransactionId.getRecord()` declared, but not overwritten."));
        });
    }
    // NOT A STABLE API
    static _fromProto(id) {
        const seconds = util_1.orThrow(id.getTransactionvalidstart()).getSeconds();
        const nanos = util_1.orThrow(id.getTransactionvalidstart()).getNanos();
        return new TransactionId({
            account: AccountId_1.AccountId._fromProto(util_1.orThrow(id.getAccountid())),
            validStartSeconds: seconds,
            validStartNanos: nanos
        });
    }
    // NOT A STABLE API
    _toProto() {
        const txnId = new BasicTypes_pb_1.TransactionID();
        txnId.setAccountid(this.accountId._toProto());
        const ts = new Timestamp_pb_1.Timestamp();
        ts.setSeconds(this.validStart.seconds);
        ts.setNanos(this.validStart.nanos);
        txnId.setTransactionvalidstart(ts);
        return txnId;
    }
}
exports.TransactionId = TransactionId;
let lastInstant;
// We need this method to return a timestamp because JS times do not generally
// handle nanoseconds. So if transactions are created too quickly, duplicate timestamps
// could be produced. This method ensures the timestamps are always _increasing_ or monotonic.
function getIncreasingInstant() {
    // Allows the transaction to be accepted as long as the
    // server is not more than 10 seconds behind us
    const instant = Timestamp_1.dateToTimestamp(Date.now() - 10000);
    // ensures every instant is at least always greater than the last
    lastInstant = lastInstant != null && instantLessThanOrEqual(instant, lastInstant) ?
        addNanos(lastInstant, 1) :
        instant;
    return lastInstant;
}
function addNanos(a, n) {
    return {
        seconds: a.seconds,
        nanos: a.nanos + n
    };
}
function instantLessThanOrEqual(a, b) {
    if (a.seconds < b.seconds) {
        return true;
    }
    else if (a.seconds === b.seconds && a.nanos <= b.nanos) {
        return true;
    }
    return false;
}
