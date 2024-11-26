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

import './App.css'
import React, { useEffect, useState } from 'react'
import { Timelock, IdealNetworkIdentityBuilder } from '@driemworks/timelock.js'
import hkdf from 'js-crypto-hkdf'

function App() {
  const [timelock, setTimelock] = useState(null)

  useEffect(() => {
    Timelock.build().then((tlock) => {
      setTimelock(tlock)
      console.log('timelock wasm ready')
    })
  }, [])

  const fromHexString = (hexString) =>
    Uint8Array.from(
      hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16))
    )

  const runDemo = async () => {
    // 1. Setup parameters for encryption
    // use an hkdf to generate an ephemeral secret key
    const seed = new TextEncoder().encode('my-secret-seed')
    const hash = 'SHA-256'
    const length = 32
    const esk = await hkdf.compute(seed, hash, length, '')
    // the message to encrypt for the future
    const message = 'Hello, Timelock!'
    const encodedMessage = new TextEncoder().encode(message)
    // A randomness beacon public key (ex: IDN public key)
    // We first get it as hex and then convert to a Uint8Array
    const pkHex =
      'b68da85d953219f84d86c5167481f505edf04ab586f28aefd238475026f5f46ba707f41bd2702f3639a4eddff8cad50041dc53da3d3617a189c85c8cb51a5f4fdfcebda05c50e81595f69e178d240fce3acdafd97b5fd204553e685836393a00b112f5cd78477d79ac8094c608d35bb42bd5091c5bbedd881e2ee0e8492a4361c69bf15250d75aee44035bc5b7553100'
    const pubkey = fromHexString(pkHex)
    // A future round number of the randomness beacon
    const roundNumber = 10

    // 2. Encrypt the message
    let ct = await timelock.encrypt(
      encodedMessage,
      roundNumber,
      IdealNetworkIdentityBuilder,
      pubkey,
      esk.key
    )
    console.log('Timelocked ciphertext: ' + JSON.stringify(ct))

    // 3. Acquire a signature for decryption from he pulse output by the beacon at the given roundNumber
    const sigHex =
      'e6cdf6c9d11c13e013b2c6cfd11dab46d8f1ace226ff845ffff4c7d6f64992892c54fb5d1f0f87dd300ce66f53598e01'
    const sig = fromHexString(sigHex)
    // Decrypt the ciphertext with the signature
    const plaintext = await timelock.decrypt(ct, sig)
    console.log(`Recovered ${String.fromCharCode(...plaintext)}, Expected ${message}`)
  }

  return (
    <div className="App">
      <h1>Timelock Encryption Demo</h1>
      <p>
        Open the developer console (F12) and then click the button below to
        execute the demo.
      </p>
      <button onClick={runDemo}>Run Demo</button>
    </div>
  )
}

export default App
