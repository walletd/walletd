import { grpc } from "@improbable-eng/grpc-web";
import { ProtobufMessage } from "@improbable-eng/grpc-web/dist/typings/message";
import { Hbar } from "./Hbar";
import UnaryMethodDefinition = grpc.UnaryMethodDefinition;
import { Ed25519PrivateKey } from "./crypto/Ed25519PrivateKey";
import { Ed25519PublicKey } from "./crypto/Ed25519PublicKey";
import { AccountId, AccountIdLike } from "./account/AccountId";
export declare type TransactionSigner = (msg: Uint8Array) => Uint8Array | Promise<Uint8Array>;
/** If `privateKey` is a string it will be parsed as an `Ed25519PrivateKey` */
export interface PrivateKey {
    privateKey: Ed25519PrivateKey | string;
}
export interface PubKeyAndSigner {
    publicKey: Ed25519PublicKey;
    signer: TransactionSigner;
}
export declare type SigningOpts = PrivateKey | PubKeyAndSigner;
export declare type Operator = {
    accountId: AccountIdLike;
} & SigningOpts;
export declare type Nodes = {
    [url: string]: AccountIdLike;
} | Node[];
/** A URL,AccountID pair identifying a Node */
export interface Node {
    url: string;
    id: AccountId;
}
export interface ClientConfig {
    network?: Nodes;
    operator?: Operator;
}
export declare abstract class BaseClient {
    private _operatorAccount?;
    private _operatorSigner?;
    private _operatorPublicKey?;
    protected _nodes: Node[];
    _maxTransactionFee: Hbar;
    _maxQueryPayment: Hbar;
    protected constructor(network: Nodes, operator?: Operator);
    putNode(id: AccountIdLike, url: string): this;
    /** Set the operator for the client object */
    setOperator(account: AccountIdLike, privateKey: Ed25519PrivateKey | string): this;
    setOperatorWith(account: AccountIdLike, publicKey: Ed25519PublicKey, signer: TransactionSigner): this;
    replaceNodes(network: Nodes): this;
    _getOperatorAccountId(): AccountId | undefined;
    _getOperatorSigner(): TransactionSigner | undefined;
    _getOperatorKey(): Ed25519PublicKey | undefined;
    /** Get the current maximum transaction fee. */
    get maxTransactionFee(): Hbar;
    /** Get the current maximum query payment. */
    get maxQueryPayment(): Hbar | undefined;
    /**
     * Set the default maximum fee for a transaction.
     *
     * This can be overridden for an individual transaction with
     * `TransactionBuilder.setMaxTransactionFee()`.
     *
     * If a transaction's fee will exceed this value, a `HederaStatusError` will be thrown with
     * `ResponseCode.INSUFFICIENT_TX_FEE`.
     *
     * @param maxFee
     */
    setMaxTransactionFee(maxFee: Hbar): this;
    /**
     * Set the max payment that can be automatically attached to a query.
     *
     * If this is not called then by default no payments will be made automatically for queries.
     *
     * If a query will cost more than this amount, a `MaxQueryPaymentExceededError` will be thrown
     * from `QueryBuilder.execute()`.
     *
     * This can be overridden for an individual query with
     * `query.setPaymentDefault(await query.requestCost())`.
     *
     * @param maxPayment the maximum automatic payment for a query
     */
    setMaxQueryPayment(maxPayment: Hbar): this;
    /**
     * Get the current account balance.
     * @deprecated `Client.getAccountBalance()` is deprecated with no replacement. Use the `AccountBalanceQuery` directly instead.
     */
    getAccountBalance(id: AccountIdLike): Promise<Hbar>;
    ping(id: AccountIdLike): Promise<void>;
    _randomNode(): Node;
    _getNode(node: string | AccountId): Node;
    abstract _unaryCall<Rq extends ProtobufMessage, Rs extends ProtobufMessage>(url: string, request: Rq, method: UnaryMethodDefinition<Rq, Rs>): Promise<Rs>;
}
