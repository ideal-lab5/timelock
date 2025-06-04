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
//! This module provides wasm-bindings for the Timelock library that are
//! compatible with Python

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use pyo3::{exceptions::PyValueError, prelude::*, wrap_pyfunction};
use rand_core::OsRng;
use sha2::Digest;
use timelock::{
	block_ciphers::AESGCMBlockCipherProvider,
	curves::drand::TinyBLS381,
	ibe::fullident::Identity,
	tlock::{EngineBLS, TLECiphertext, tld as timelock_decrypt, tle as timelock_encrypt},
};

/// The encrypt wrapper used by the Python bindings to call tlock.rs encrypt
/// function
/// * 'id_py': ID string for which the message will be encrypted
/// * 'message_py': Message which will be encrypted
/// * 'sk_py': secret key passed in from the Python side
/// * 'p_pub_py': public key commitment for the IBE system
#[pyfunction]
fn tle(
	round_number: u64,
	message: Vec<u8>,
	sk_py: Vec<u8>,
	p_pub_py: Vec<u8>,
) -> PyResult<Vec<u8>> {
	let msk_bytes: [u8; 32] = sk_py
		.try_into()
		.map_err(|_| PyErr::new::<PyValueError, _>("Could not convert secret key"))?;

	let pp = <TinyBLS381 as EngineBLS>::PublicKeyGroup::deserialize_compressed(&p_pub_py[..])
		.map_err(|_| {
			PyErr::new::<PyValueError, _>(
				"The public key bytes could not be deserialized to a valid public key.",
			)
		})?;
	let id = {
		let mut hasher = sha2::Sha256::new();
		hasher.update(round_number.to_be_bytes());
		hasher.finalize().to_vec()
	};
	let identity = Identity::new(b"", vec![id]);

	let ciphertext = timelock_encrypt::<TinyBLS381, AESGCMBlockCipherProvider, OsRng>(
		pp, msk_bytes, &message, identity, OsRng,
	)
	.map_err(|_| PyErr::new::<PyValueError, _>("Encryption failed"))?;

	let mut ciphertext_bytes: Vec<u8> = Vec::new();
	ciphertext
		.serialize_compressed(&mut ciphertext_bytes)
		.map_err(|_| PyErr::new::<PyValueError, _>("Ciphertext serialization failed"))?;

	Ok(ciphertext_bytes)
}

/// The decrypt wrapper used by the Python bindings to call the timelock decrypt
/// function
/// * 'ciphertext_bytes': The ciphertext bytes to be decrypted
/// * 'sig_bytes': A signature (output of IBE Extract)
#[pyfunction]
fn tld(ciphertext_bytes: Vec<u8>, sig_bytes: Vec<u8>) -> PyResult<Vec<u8>> {
	let sig_point =
		<TinyBLS381 as EngineBLS>::SignatureGroup::deserialize_compressed(sig_bytes.as_slice())
			.map_err(|_| PyErr::new::<PyValueError, _>("Could not deserialize signature"))?;

	let ciphertext: TLECiphertext<TinyBLS381> =
		TLECiphertext::deserialize_compressed(ciphertext_bytes.as_slice())
			.map_err(|_| PyErr::new::<PyValueError, _>("Could not deserialize ciphertext"))?;

	let result =
		timelock_decrypt::<TinyBLS381, AESGCMBlockCipherProvider>(ciphertext, sig_point)
			.map_err(|e| PyErr::new::<PyValueError, _>(format!("Decryption failed: {:?}", e)))?;

	Ok(result)
}

#[pymodule]
#[pyo3(name = "timelock_wasm_wrapper")]
fn py(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(tle, m)?)?;
	m.add_function(wrap_pyfunction!(tld, m)?)?;
	Ok(())
}
