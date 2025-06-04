#
# Copyright 2025 by Ideal Labs, LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
import timelock_wasm_wrapper

# The Timelock class is a thin wrapper around the output from PyO3
# It opinionates the input, expecting public keys to be represented as hex strings
# and encryption messages being strings.
class Timelock: 
    def __init__(self, pk_hex):
        self.public_key = bytearray.fromhex(pk_hex)

    def tle(self, id, message, sk):
        result = timelock_wasm_wrapper.tle(id, str.encode(message), sk, self.public_key)
        return result
    
    def tld(self, ct, sig):
        result = timelock_wasm_wrapper.tld(ct, sig)
        return result
