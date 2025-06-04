/*
 * Copyright 2025 by Ideal Labs, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
  build: (x: X) => Promise<Uint8Array>
}
