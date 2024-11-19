import { timelockEncrypt, timelockDecrypt, IdealNetworkIdentityHandler } from '../../../dist/timelock.js'
// import * as timelock from '../../../dist/timelock.js'
import './App.css'
import React, { useEffect, useState } from 'react'
function App() {

  useEffect(() => {

  }, [])

  const fromHexString = (hexString) =>
    Uint8Array.from(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));

  async function runDemo() {
    try {
      // await timelock.init();
      // Example parameters
      const exampleMessage = "Hello, Timelock!";
      // The round number of the beacon protocol for which we use a signautre
      const roundNumber = 10;
      // The drand quicknet pubkey
      const beaconPublicKey = Uint8Array.from(fromHexString('aced2f1f580f863d9144844275e76c8efdb255299c8188c7a63292ea5ebd40d3dde2bdf7466f57053e843a1100bf8a80a8cbebf43a11477763e3f02f31a5656aef2231888c71593e8c460e344cf4f6f03d3cbb342718b41b89d51efcfdf91a00082ea50f652239aaf2c6c7c7a87d9e76f6b4337da8f0c4ebc50cd79ae781da0dcfaad6d8b9cdc6f087eac7bffb164101'))
      // Simulated signature (in a real scenario, this comes from the randomness beacon)
      const signature = Uint8Array.from(fromHexString('38552cb0180ae083d73f12ca75e680f2a597f8f8cb40bbb07c0fe57786f14b44ef3c1351d7aab135dfa5e096b3f82181'))
      // A seed passed to the rng within the protcool
      const seed = "my-secure-seed";

      console.log("Original Message:", exampleMessage);

      // Encrypt the message
      const encodedMessage = new TextEncoder().encode(exampleMessage);
      const ciphertext = await timelockEncrypt(
        encodedMessage,
        roundNumber,
        IdealNetworkIdentityHandler,
        beaconPublicKey,
        seed
      );
      console.log("Ciphertext:", ciphertext);
      // Decrypt the message
      const plaintext = await timelockDecrypt(ciphertext, signature);
      console.log("Decrypted Message:", plaintext);
    } catch (error) {
      console.error("Error:", error);
    }
  }

  return (
    <div className="App">
      <div className="header">
        React Timelock Encryption Example
      </div>
      <div className='body'>
        <p>Click the button below to run the demo. Keep the console opened.</p>
        <button onClick={runDemo}>Run</button>
      </div>
    </div>
  )
}

export default App
