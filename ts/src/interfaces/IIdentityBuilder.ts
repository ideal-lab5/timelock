import { build_encoded_commitment } from 'timelock-wasm-wrapper'

/**
 * The IdentityBuilder is used to build identities for IBE
 * In relation to the verifiable randomness beacon used, the implementation
 * should correspond to however that beacon constructs messages for signing.
 */
export interface IdentityBuilder<X> {
    /**
     * Build an identity based on the input 'x'
     * @param x : The identity data
     * @returns : The constructed identity
     */
    build: (x: X) => Uint8Array
  }
  
  /**
   * An IdentityBuilder for the Ideal Network
   */
  export const IdealNetworkIdentityHandler: IdentityBuilder<number> = {
    build: (bn) => build_encoded_commitment(bn, 0),
  }