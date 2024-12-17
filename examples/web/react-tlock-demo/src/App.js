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
import { Timelock, IdealNetworkIdentityBuilder, DrandIdentityBuilder, SupportedCurve, u8a } from '@ideallabs/timelock.js'
import hkdf from 'js-crypto-hkdf'

function App() {

  const [timelockDrand, setTimelockDrand] = useState(null)
  const [timelockIdeal, setTimelockIdeal] = useState(null)

  useEffect(() => {
    Timelock.build(SupportedCurve.BLS12_381).then((tlock) => {
      setTimelockDrand(tlock)
    })

    Timelock.build(SupportedCurve.BLS12_377).then((tlock) => {
      setTimelockIdeal(tlock)
    })

  }, [])

  const fromHexString = (hexString) =>
    Uint8Array.from(
      hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16))
    )

  // 83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a
  const runDemoDrand = async () => {
    // 1. Setup parameters for encryption
    // use an hkdf to generate an ephemeral secret key
    const seed = new TextEncoder().encode('my-secret-seed')
    const hash = 'SHA-256'
    const length = 32
    const esk = await hkdf.compute(seed, hash, length, '')
    const key = Array.from(esk.key)
      .map((byte) => byte.toString(16).padStart(2, '0'))
      .join('')
    // the message to encrypt for the future
    const message = 'Hello, Timelock!'
    const encodedMessage = new TextEncoder().encode(message)
    // A randomness beacon public key (ex: IDN public key)
    // We first get it as hex and then convert to a Uint8Array
    const pubkey =
      '83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a'
    // A future round number of the randomness beacon
    const roundNumber = 1000
    // 2. Encrypt the message
    let ct = await timelockDrand.encrypt(
      encodedMessage,
      roundNumber,
      DrandIdentityBuilder,
      pubkey,
      key
    )
    console.log('Timelocked ciphertext: ' + JSON.stringify(ct))

    // 3. Acquire a signature for decryption from he pulse output by the beacon at the given roundNumber
    const sigHex =
      'b44679b9a59af2ec876b1a6b1ad52ea9b1615fc3982b19576350f93447cb1125e342b73a8dd2bacbe47e4b6b63ed5e39'
    // Decrypt the ciphertext with the signature
    const plaintext = await timelockDrand.decrypt(ct, sigHex)
    // console.log(plaintext)
    console.log(`Recovered ${String.fromCharCode(...plaintext)}, Expected ${message}`)
  }

  const runDemoIdeal = async () => {
    // 1. Setup parameters for encryption
    // use an hkdf to generate an ephemeral secret key
    const seed = new TextEncoder().encode('my-secret-seed')
    const hash = 'SHA-256'
    const length = 32
    const esk = await hkdf.compute(seed, hash, length, '')
    const key = Array.from(esk.key)
      .map((byte) => byte.toString(16).padStart(2, '0'))
      .join('')
    // the message to encrypt for the future
    const message = 'Hello, Timelock!'
    const encodedMessage = new TextEncoder().encode(message)
    // A randomness beacon public key (ex: IDN public key)
    // We first get it as hex and then convert to a Uint8Array
    const pubkey =
      '41dc53da3d3617a189c85c8cb51a5f4fdfcebda05c50e81595f69e178d240fce3acdafd97b5fd204553e685836393a00b112f5cd78477d79ac8094c608d35bb42bd5091c5bbedd881e2ee0e8492a4361c69bf15250d75aee44035bc5b7553100'
    // A future round number of the randomness beacon
    const roundNumber = 10

    // 2. Encrypt the message
    let ct = await timelockIdeal.encrypt(
      encodedMessage,
      roundNumber,
      IdealNetworkIdentityBuilder,
      pubkey,
      key
    )

    console.log('Timelocked ciphertext: ' + JSON.stringify(ct))

    // 3. Acquire a signature for decryption from he pulse output by the beacon at the given roundNumber
    const sig =
      'e6cdf6c9d11c13e013b2c6cfd11dab46d8f1ace226ff845ffff4c7d6f64992892c54fb5d1f0f87dd300ce66f53598e01'
    // Decrypt the ciphertext with the signature
    const plaintext = await timelockIdeal.decrypt(ct, sig)
    console.log(`Recovered ${String.fromCharCode(...plaintext)}, Expected ${message}`)
  }

  return (
    <div className="App">
      <h1>Timelock Encryption Demo</h1>
      <p>
        Open the developer console (F12) and then click the button below to
        execute the demo.
      </p>
      <button onClick={runDemoDrand}>Run Demo (Drand)</button>
      <button onClick={runDemoIdeal}>Run Demo (IDN)</button>
    </div>
  )
}

export default App
