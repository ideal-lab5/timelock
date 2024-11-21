/**
 * The IdentityBuilder is used to build identities for IBE
 * In relation to the verifiable randomness beacon used, the implementation
 * should correspond to however that beacon constructs messages for signing.
 */
interface IdentityBuilder<X> {
    /**
     * Build an identity based on the input 'x'
     * @param x : The identity data
     * @returns : The constructed identity
     */
    build: (x: X) => any;
}
/**
 * An IdentityBuilder for the Ideal Network
 */
export declare const IdealNetworkIdentityHandler: IdentityBuilder<number>;
/**
 * The Timelock class handles initialization of the WASM modules required to use the Timelock library
 * from web based contexts. It is a thin wrapper around the output of running `wasm-pack --target web` from the wasm directory
 * It gracefully ensures that the WASM is available before attempting to call the respective functions.
 */
export declare class Timelock {
    /**
     * Indicates if the wasm has been initialized or not
     */
    private wasmReady;
    /**
     * A private constructor to enforce usage of `build`
     */
    private constructor();
    /**
     * Loads the wasm and constructs a new Timelock instance
     * @returns
     */
    static build(): Promise<Timelock>;
    /**
     * Timelock Encryption: Encrypt the message for the given block
     * The HKDF used satisfies RFC5869
     *
     * @param encodedMessage: The message to encrypt, encoded as a Uint8Array
     * @param roundNumber: The round of the protocol
     * @param identityBuilder: Something that imlement IdentityBuilder (e.g. IdealNetworkIdentityHandler)
     * @param beaconPublicKey: The public key of the randomness beacon
     * @param ephemeralSecretKey: An ephemeral secret key passed to AES-GCM
     * @returns The Timelocked ciphertext
     */
    encrypt(encodedMessage: Uint8Array, roundNumber: number, identityBuilder: IdentityBuilder<number>, beaconPublicKey: Uint8Array, ephemeralSecretKey: Uint8Array): Promise<any>;
    /**
     * Timelock decryption: Decrypt the ciphertext using a pulse from the beacon produced at the given block
     * @param ciphertext: Ciphertext to be decrypted
     * @param blockNumber: Block number that has the signature for decryption
     * @returns: The decrypted message (if successful), else nothing (is that what I want?)
     */
    decrypt(ciphertext: Uint8Array, signature: Uint8Array): Promise<any>;
    /**
     * Decrypt a ciphertext early using the cipher used when encrypting if you know the seed
     * @param ciphertext The ciphertext to decrypt
     * @param ephemeralSecretKey: An ephemeral secret key passed to AES-GCM
     * @returns The plaintext
     */
    forceDecrypt(ciphertext: Uint8Array, ephemeralSecretKey: Uint8Array): Promise<any>;
    /**
     * Check if the wasm has been initialized.
     * If it hasn't, gracefully load the wasm and continue.
     * Q: In which scenarios would the wasm be unavailable after initially being loaded?
     */
    checkWasm(): Promise<void>;
}
export {};
