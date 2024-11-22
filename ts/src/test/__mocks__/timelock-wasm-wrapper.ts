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

const init = jest.fn().mockResolvedValue(undefined)
const build_encoded_commitment = jest.fn().mockReturnValue(new Uint8Array(0))
const tle = jest.fn().mockResolvedValue(new Uint8Array(1))
const tld = jest.fn().mockResolvedValue(new Uint8Array(2))
const decrypt = jest.fn().mockResolvedValue(new Uint8Array(3))

export default init
export { build_encoded_commitment, tle, tld, decrypt }
