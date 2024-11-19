/*
 * Copyright 2024 by Ideal Labs, LLC
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

module.exports = {
    init: jest.fn().mockResolvedValue(undefined),
    build_encoded_commitment: jest.fn((bn, id) => `commitment-${bn}-${id}`),
    tle: jest.fn((id, message, key, pubKey) => `ciphertext-${id}-${message}`),
    tld: jest.fn((ciphertext, signature) => `plaintext-${ciphertext}-${signature}`),
    decrypt: jest.fn((ciphertext, key) => `plaintext-${ciphertext}-${key}`),
  };
  