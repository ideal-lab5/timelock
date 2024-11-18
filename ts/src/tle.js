/// Timelock Encryption TS Wrapper
/// This lib provides a typescript wrapper around the wasm-pack output of the timelock encryption library
import init, { build_encoded_commitment, tle, tld, decrypt as aesDecrypt } from '../../wasm/pkg';
import hkdf from 'js-crypto-hkdf'; // for npm
const HASH = 'SHA-256';
const HASH_LENGTH = 32;
/**
 * An IdentityBuilder for the Ideal Network
 */
export const IdealNetworkIdentityHandler = {
    build: (bn) => build_encoded_commitment(bn, 0),
};
/**
 * Timelock Encryption: Encrypt the message for the given block
 * The HKDF used satisfies RFC5869
 *
 * @param encodedMessage: The message to encrypt, encoded as a Uint8Array
 * @param roundNumber: The round of the protocol
 * @param identityBuilder: Something that imlement IdentityBuilder (e.g. idealNetworkIdentityHandler)
 * @param beaconPublicKey: The public key of the randomness beacon used
 * @param seed: A seed to derive crypto keys
 * @returns the ciphertext
 */
export async function timelockEncrypt(encodedMessage, roundNumber, identityBuilder, beaconPublicKey, seed) {
    await init();
    // TODO: fine for now but should ultimately query the BABE pallet config instead
    // https://github.com/ideal-lab5/tle/issues/7
    let t = new TextEncoder();
    let masterSecret = t.encode(seed);
    return hkdf.compute(masterSecret, HASH, HASH_LENGTH, '').then((derivedKey) => {
        let id = identityBuilder.build(roundNumber);
        let ct = tle(id, encodedMessage, derivedKey.key, beaconPublicKey);
        return ct;
    });
}
/**
 * Timelock decryption: Decrypt the ciphertext using a pulse from the beacon produced at the given block
 * @param ciphertext: Ciphertext to be decrypted
 * @param blockNumber: Block number that has the signature for decryption
 * @returns: Plaintext of encrypted message
 */
export async function timelockDecrypt(ciphertext, signature) {
    await init();
    return tld(ciphertext, signature);
}
/**
 * Decrypt a ciphertext early if you know the seed
 * @param ciphertext The ciphertext to decrypt
 * @param seed The ciphertext seed
 * @returns The plaintext
 */
export async function decrypt(ciphertext, seed) {
    await init();
    let t = new TextEncoder();
    let masterSecret = t.encode(seed);
    return hkdf.compute(masterSecret, HASH, HASH_LENGTH, '').then((derivedKey) => {
        let pt = aesDecrypt(ciphertext, derivedKey);
        return pt;
    });
}
