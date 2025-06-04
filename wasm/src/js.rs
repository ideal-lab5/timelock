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

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use codec::Encode;
use rand_core::OsRng;
use sp_consensus_beefy_etf::{known_payloads, Commitment, Payload};
use serde::{Serialize, Deserialize};
use timelock::{
	curves::drand::TinyBLS381,
	ibe::fullident::Identity,
	block_ciphers::{
		AESGCMBlockCipherProvider, AESOutput, BlockCipherProvider,
	},
	tlock::{tld as timelock_decrypt, tle as timelock_encrypt, TLECiphertext},
};

use crate::engines::{drand::TinyBLS381, EngineBLS};
use wasm_bindgen::prelude::*;

/// a helper function to deserialize arkworks elements from bytes
fn convert_from_bytes<E: CanonicalDeserialize, const N: usize>(
	bytes: &[u8; N],
) -> Option<E> {
	E::deserialize_compressed(&bytes[..]).ok()
}

/// Supported Beacon Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportedCurve {
    Bls12_381,
}

/// The encrypt wrapper used by the WASM blob to call tlock.rs encrypt function
/// * `id_js`: ID string for which the message will be encrypted
/// * `message_js`: Message which will be encrypted
/// * `sk_js`: secret key passed in from UI. This should be obtained elsewhere
///   later on.
/// * `p_pub_js`: the public key commitment for the IBE system
/// * `
///
#[wasm_bindgen]
pub fn tle(
	id_js: JsValue,
	message_js: JsValue,
	sk_js: JsValue,
	p_pub_js: JsValue,
	supported_curve_js: JsValue,
) -> Result<JsValue, JsError> {
    let curve: SupportedCurve = serde_wasm_bindgen::from_value(supported_curve_js.clone())
		.map_err(|_| JsError::new("could not decode the curve type"))?;

	match curve {
		SupportedCurve::Bls12_381 => do_tle::<TinyBLS381>(id_js, message_js, sk_js, p_pub_js),
	}
}

pub fn do_tle<E: EngineBLS>(
	id_js: JsValue,
	message_js: JsValue,
	sk_js: JsValue,
	p_pub_js: JsValue,
) -> Result<JsValue, JsError> {
	let msk_bytes: [u8; 32] = serde_wasm_bindgen::from_value(sk_js.clone())
		.map_err(|_| JsError::new("could not decode secret key"))?;
	let p_pub_vec: Vec<u8> =
		serde_wasm_bindgen::from_value(p_pub_js.clone())
			.map_err(|_| JsError::new("could not decode p_pub"))?;
	let pp_bytes: [u8; 96] = p_pub_vec
		.try_into()
		.map_err(|_| JsError::new("could not convert public params"))?;
	let pp = convert_from_bytes::<<E as EngineBLS>::PublicKeyGroup, 96>(
		&pp_bytes.clone(),
	)
	.ok_or(JsError::new("Could not convert secret key"))?;

	let id_bytes: Vec<u8> = serde_wasm_bindgen::from_value(id_js.clone())
		.map_err(|_| JsError::new("could not decode id"))?;
	let identity = Identity::new(b"", vec![id_bytes]);
	let message_bytes: Vec<u8> =
		serde_wasm_bindgen::from_value(message_js.clone())
			.map_err(|_| JsError::new("could not decode message"))?;

	let mut ciphertext_bytes: Vec<u8> = Vec::new();
	let ciphertext: TLECiphertext<E> =
		timelock_encrypt::<E, AESGCMBlockCipherProvider, OsRng>(
			pp,
			msk_bytes,
			&message_bytes,
			identity,
			OsRng,
		)
		.map_err(|_| JsError::new("encryption failed"))?;

	ciphertext
		.serialize_compressed(&mut ciphertext_bytes)
		.map_err(|_| JsError::new("ciphertext serialization has failed"))?;

	serde_wasm_bindgen::to_value(&ciphertext_bytes)
		.map_err(|_| JsError::new("could not convert ciphertext to JsValue"))
}

/// The decrypt wrapper used by the WASM blob to call tlock.rs encrypt function
/// * `ciphertext_js`: The string to be decrypted
/// * `sig_vec_js`: The array of BLS signatures required to rebuild the secret
///   key and decrypt the message
#[wasm_bindgen]
pub fn tld(
	ciphertext_js: JsValue,
	sig_vec_js: JsValue,
	supported_curve_js: JsValue,
) -> Result<JsValue, JsError> {
	let curve: SupportedCurve = serde_wasm_bindgen::from_value(supported_curve_js.clone())
		.map_err(|_| JsError::new("could not decode the curve type"))?;

	match curve {
        SupportedCurve::Bls12_381 => do_tld::<TinyBLS381>(ciphertext_js, sig_vec_js),
    }
}

/// Timelock decryption
fn do_tld<E: EngineBLS>(
	ciphertext_js: JsValue,
	sig_vec_js: JsValue,
) -> Result<JsValue, JsError> {
	let sig_conversion: Vec<u8> =
		serde_wasm_bindgen::from_value(sig_vec_js.clone())
			.map_err(|_| JsError::new("could not decode secret key"))?;
	let sig_bytes = sig_conversion.as_slice();
	let sig_point =
		<E as EngineBLS>::SignatureGroup::deserialize_compressed(sig_bytes)
			.map_err(|_| JsError::new("could not deserialize sig_vec"))?;
	let ciphertext_vec: Vec<u8> =
		serde_wasm_bindgen::from_value(ciphertext_js.clone())
			.map_err(|_| JsError::new("could not decode ciphertext"))?;
	let ciphertext_bytes: &[u8] = ciphertext_vec.as_slice();

	let ciphertext: TLECiphertext<E> =
		TLECiphertext::deserialize_compressed(ciphertext_bytes)
			.map_err(|_| JsError::new("Could not deserialize ciphertext"))?;
	let result: Vec<u8> = timelock_decrypt::<E, AESGCMBlockCipherProvider>(
		ciphertext, sig_point,
	)
	.map_err(|e| JsError::new(&format!("decryption has failed {:?}", e)))?;
	serde_wasm_bindgen::to_value(&result)
		.map_err(|_| JsError::new("plaintext conversion has failed"))
}

#[wasm_bindgen]
pub fn decrypt(
	ciphertext_js: JsValue,
	sk_vec_js: JsValue,
	supported_curve_js: JsValue,
) -> Result<JsValue, JsError> {
	let curve: SupportedCurve = serde_wasm_bindgen::from_value(supported_curve_js.clone())
		.map_err(|_| JsError::new("could not decode the curve type"))?;

	match curve {
		SupportedCurve::Bls12_381 => do_decrypt::<TinyBLS381>(ciphertext_js, sk_vec_js),
	}
}

/// Bypass Tlock by attempting to decrypt the ciphertext with some secret key
/// under the stream cipher only
pub fn do_decrypt<E: EngineBLS>(
	ciphertext_js: JsValue,
	sk_vec_js: JsValue,
) -> Result<JsValue, JsError> {
	let sk_bytes: Vec<u8> =
		serde_wasm_bindgen::from_value(sk_vec_js.clone())
			.map_err(|_| JsError::new("could not decode secret key"))?;

	let secret_key: [u8; 32] = sk_bytes.clone().try_into().map_err(|_| {
		JsError::new(&format!(
			"The secret key should be 32 bytes, but it was {:?}",
			sk_bytes.len()
		))
	})?;

	let ciphertext_vec: Vec<u8> =
		serde_wasm_bindgen::from_value(ciphertext_js.clone())
			.map_err(|_| JsError::new("could not decode ciphertext"))?;
	let ciphertext_bytes: &[u8] = ciphertext_vec.as_slice();
	let ciphertext: TLECiphertext<E> =
		TLECiphertext::deserialize_compressed(ciphertext_bytes)
			.map_err(|_| JsError::new("Could not deserialize ciphertext"))?;

	let aes_ciphertext: AESOutput =
		AESOutput::deserialize_compressed(&mut &ciphertext.body[..]).unwrap();

	let result: Vec<u8> =
		AESGCMBlockCipherProvider::decrypt(aes_ciphertext, secret_key)
			.map_err(|_| JsError::new("Message decryption failed"))?;

	serde_wasm_bindgen::to_value(&result)
		.map_err(|_| JsError::new("plaintext conversion has failed"))
}

/// Logging struct, useful for testing and debugging
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

/// Builds an encoded commitment for use in timelock encryption using the Ideal
/// Network
#[wasm_bindgen]
pub fn build_encoded_commitment(
	block_number_js: JsValue,
	validator_set_id_js: JsValue,
) -> Result<JsValue, JsError> {
	let block_number: u32 = serde_wasm_bindgen::from_value(
		block_number_js.clone(),
	)
	.map_err(|_| JsError::new("could not decode a u32 from the input"))?;
	let validator_set_id: u64 = serde_wasm_bindgen::from_value(
		validator_set_id_js.clone(),
	)
	.map_err(|_| JsError::new("could not decode a u32 from the input"))?;
	let payload =
		Payload::from_single_entry(known_payloads::ETF_SIGNATURE, Vec::new());
	let commitment = Commitment { payload, block_number, validator_set_id };
	let encoded = commitment.encode();
	serde_wasm_bindgen::to_value(&encoded).map_err(|_| {
		JsError::new("could not convert the encoded commitment to json")
	})
}

#[cfg(test)]
mod test {
	use super::*;
	use rand_chacha::ChaCha20Rng;
	use rand_core::SeedableRng;
	use sha2::Digest;
	// use crate::engines::Engine;
	use wasm_bindgen_test::*;

	enum TestStatusReport {
		EncryptSuccess { ciphertext: JsValue },
		DecryptSuccess { plaintext: JsValue },
		EncryptFailure { _error: JsError },
		DecryptFailure { _error: JsError },
	}

	/// This function is used purely for testing purposes.
	/// It takes in a seed and generates a secret key and public params
	fn generate_keys<E: EngineBLS>(seed: JsValue) -> ([u8; 144], [u8; 32]) {
		let seed_vec: Vec<u8> = serde_wasm_bindgen::from_value(seed).unwrap();
		let seed_vec = seed_vec.as_slice();

		let mut hasher = sha2::Sha256::default();
		hasher.update(seed_vec);
		let hash = hasher.finalize();
		let seed_hash: [u8; 32] = hash.into();
		let mut rng: ChaCha20Rng = ChaCha20Rng::from_seed(seed_hash);
		let keypair = w3f_bls::KeypairVT::<E>::generate(&mut rng);
		let sk_gen: <E as EngineBLS>::Scalar = keypair.secret.0;
		let double_public: DoublePublicKey<E> = DoublePublicKey(
			keypair.into_public_key_in_signature_group().0,
			keypair.public.0,
		);
		let mut sk_bytes = Vec::new();
		sk_gen.serialize_compressed(&mut sk_bytes).unwrap();
		let mut double_public_bytes = Vec::new();
		double_public.serialize_compressed(&mut double_public_bytes).unwrap();

		(double_public_bytes.try_into().unwrap(), sk_bytes.try_into().unwrap())
	}

	fn setup_test<E: EngineBLS>(
		identity_vec: Vec<u8>,
		message: Vec<u8>,
		succesful_decrypt: bool,
		standard_tle: bool,
		beacon: &str,
		handler: &dyn Fn(TestStatusReport) -> (),
	) {
		let seed_bytes = "seeeeeeed".as_bytes();
		let seed = serde_wasm_bindgen::to_value(seed_bytes).unwrap();

		let (p_pub, sk) = generate_keys::<E>(seed);
		let mut sk_js: JsValue =
			serde_wasm_bindgen::to_value(sk.as_slice()).unwrap();
		let p_pub_js: JsValue =
			serde_wasm_bindgen::to_value(&p_pub[48..]).unwrap();

		let identity_js: JsValue =
			serde_wasm_bindgen::to_value(&identity_vec).unwrap();
		let message_js: JsValue =
			serde_wasm_bindgen::to_value(&message).unwrap();

		let msk: <E as EngineBLS>::Scalar =
			convert_from_bytes::<<E as EngineBLS>::Scalar, 32>(&sk.clone())
				.unwrap();
		let identity = Identity::new(b"", vec![identity_vec]);

		let sig: E::SignatureGroup = identity.extract::<E>(msk).0;
		let mut sig_bytes: Vec<_> = Vec::new();

		if succesful_decrypt {
			sig.serialize_compressed(&mut sig_bytes).unwrap();
		} else {
			let bad_ident_vec = b"bad_ident".to_vec();
			let bad_ident = Identity::new(b"", vec![bad_ident_vec]);
			let bad_sig: E::SignatureGroup = bad_ident.extract::<E>(msk).0;
			let bad_sig_vec = vec![bad_sig];
			bad_sig_vec.serialize_compressed(&mut sig_bytes).unwrap();

			// this portion (intentionally) corrupts the decryption result for
			// early decryption
			sk_js = serde_wasm_bindgen::to_value([1; 32].as_slice()).unwrap();
		}

		let sig_vec_js: JsValue =
			serde_wasm_bindgen::to_value(&sig_bytes).unwrap();

		if standard_tle {
			match tle(identity_js, message_js, sk_js, p_pub_js, beacon.into()) {
				Ok(ciphertext) => {
					let ciphertext_clone = ciphertext.clone();
					handler(TestStatusReport::EncryptSuccess { ciphertext });
					match tld(ciphertext_clone, sig_vec_js, beacon.into()) {
						Ok(plaintext) => {
							handler(TestStatusReport::DecryptSuccess {
								plaintext,
							})
						},
						Err(error) => {
							handler(TestStatusReport::DecryptFailure {
								_error: error
							})
						},
					}
				},
				Err(error) => {
					handler(TestStatusReport::EncryptFailure { _error: error })
				},
			}
		} else {
			match tle(
				identity_js,
				message_js,
				sk_js.clone(),
				p_pub_js,
				beacon.into(),
			) {
				Ok(ciphertext) => {
					let ciphertext_clone = ciphertext.clone();
					handler(TestStatusReport::EncryptSuccess { ciphertext });
					match decrypt(ciphertext_clone, sk_js, beacon.into()) {
						Ok(plaintext) => {
							handler(TestStatusReport::DecryptSuccess {
								plaintext,
							})
						},
						Err(error) => {
							handler(TestStatusReport::DecryptFailure {
								_error: error
							})
						},
					}
				},
				Err(error) => {
					handler(TestStatusReport::EncryptFailure { _error: error })
				},
			}
		}
	}

	#[wasm_bindgen_test]
	pub fn can_encrypt_decrypt_drand() {
		can_encrypt_decrypt::<TinyBLS381>("drand");
	}

	pub fn can_encrypt_decrypt<E: EngineBLS>(beacon_type: &str) {
		let message: Vec<u8> = b"this is a test message".to_vec();
		let id: Vec<u8> = b"testing purposes".to_vec();
		setup_test::<E>(
			id,
			message.clone(),
			true,
			true,
			beacon_type,
			&|status: TestStatusReport| match status {
				TestStatusReport::EncryptSuccess { ciphertext } => {
					let ciphertext_convert: Vec<u8> =
						serde_wasm_bindgen::from_value(ciphertext.clone())
							.unwrap();
					assert!(ciphertext.is_truthy());
					assert_ne!(ciphertext_convert, message);
				},
				TestStatusReport::DecryptSuccess { plaintext } => {
					let plaintext_convert: Vec<u8> =
						serde_wasm_bindgen::from_value(plaintext.clone())
							.unwrap();
					assert_eq!(plaintext_convert, message);
				},
				_ => panic!("The ciphertext is falsy"),
			},
		)
	}

	#[wasm_bindgen_test]
	pub fn can_encrypt_decrypt_early_ideal() {
		can_encrypt_decrypt_early::<TinyBLS377>("ideal");
	}

	#[wasm_bindgen_test]
	pub fn can_encrypt_decrypt_early_drand() {
		can_encrypt_decrypt_early::<TinyBLS381>("drand");
	}

	pub fn can_encrypt_decrypt_early<E: EngineBLS>(beacon_type: &str) {
		let message: Vec<u8> = b"this is a test message".to_vec();
		let id: Vec<u8> = b"testing purposes".to_vec();
		setup_test::<E>(
			id,
			message.clone(),
			true,
			false,
			beacon_type.into(),
			&|status: TestStatusReport| match status {
				TestStatusReport::EncryptSuccess { ciphertext } => {
					let ciphertext_convert: Vec<u8> =
						serde_wasm_bindgen::from_value(ciphertext.clone())
							.unwrap();
					assert!(ciphertext.is_truthy());
					assert_ne!(ciphertext_convert, message);
				},
				TestStatusReport::DecryptSuccess { plaintext } => {
					let plaintext_convert: Vec<u8> =
						serde_wasm_bindgen::from_value(plaintext.clone())
							.unwrap();
					assert_eq!(plaintext_convert, message);
				},
				_ => panic!("The ciphertext is falsy"),
			},
		)
	}
}
