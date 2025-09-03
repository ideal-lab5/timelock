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

use super::utils::{cross_product_const, h2, h3, h4};
use alloc::vec;
use ark_ec::PrimeGroup;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{ops::Mul, rand::Rng, vec::Vec};
use serde::{Deserialize, Serialize};

use crate::engines::EngineBLS;
use crate::{Hash, HASH_LENGTH, Message};

/// Represents a serialized field element of a scalar field
pub type SerializedFieldElement = [u8; 32];

/// Represents a ciphertext in the BF-IBE FullIdent scheme
#[derive(
	Debug, Clone, PartialEq, CanonicalDeserialize, CanonicalSerialize, Serialize, Deserialize,
)]
#[repr(C)] // since we know the exact size at compile time
pub struct Ciphertext<E: EngineBLS> {
	/// U = rP
	pub u: E::PublicKeyGroup,
	/// V = sigma (+) H_2(g_id^r)
	pub v: Hash,
	/// W = message (+) H_4(sigma)
	pub w: Hash,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IbeError {
	DecryptionFailed,
}

#[derive(Debug, PartialEq)]
pub enum InputError {
	InvalidLength,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Input<E: EngineBLS> {
	data: Vec<u8>,
	_phantom: ark_std::marker::PhantomData<E>,
}

impl<E: EngineBLS> Input<E> {
	pub fn new(data: SerializedFieldElement) -> Result<Self, InputError> {
		Ok(Self { data, _phantom: ark_std::marker::PhantomData })
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.data
	}
}

/// A type to represent an IBE identity (for which we will encrypt message)
#[derive(Debug, Clone)]
pub struct Identity(pub Message);

impl Identity {
	/// construct a new identity from a string
	pub fn new(ctx: &[u8], identity: &[u8]) -> Self {
		Self(Message::new(ctx, &identity))
	}

	/// The IBE extract function on a given secret key
	pub fn extract<E: EngineBLS>(&self, sk: E::Scalar) -> IBESecret<E> {
		IBESecret(self.public::<E>() * sk)
	}

	/// Derive the public key for this identity (hash to G1)
	pub fn public<E: EngineBLS>(&self) -> E::SignatureGroup {
		self.0.hash_to_signature_curve::<E>()
	}

	/// BF-IBE encryption
	///
	/// For a message with 32-bytes and a public key (in G2), calculates the
	/// BF-IBE ciphertext
	///
	/// C = <U, V, W> = <rP, sigma (+) H_2(g_{ID}^r, message (+) H_4(sigma))>
	/// where r is randomly selected from the finite field (Z_p) and g_{ID} =
	/// e(Q_ID, P_pub)
	pub fn encrypt<E, R>(
		&self,
		message: &Input<E>,
		p_pub: E::PublicKeyGroup,
		mut rng: R,
	) -> Ciphertext<E>
	where
		E: EngineBLS,
		R: Rng + Sized,
	{
		let bytes = message.as_bytes();
		// sigma <- {0, 1}^d
		let mut sigma = vec![0u8; E::SECRET_KEY_SIZE];
		rng.fill_bytes(&mut sigma);
		// r= H3(sigma, message)
		let r: E::Scalar = h3::<E>(&sigma, bytes);
		let p = E::PublicKeyGroup::generator();
		// U = rP \in \mathbb{G}_1
		let u = p * r;
		// e(P_pub, Q_id)
		let g_id = E::pairing(p_pub.mul(r), self.public::<E>());
		// sigma (+) H2(e(P_pub, Q_id))
		let v_rhs = h2(g_id);
		let v = cross_product_const::<HASH_LENGTH>(&sigma, &v_rhs);
		// message (+) H4(sigma)
		let w_rhs = h4(&sigma);
		let w = cross_product_const::<HASH_LENGTH>(bytes, &w_rhs);
		// (rP, sigma (+) H2(e(Q_id, P_pub)), message (+) H4(sigma))
		Ciphertext::<E> { u, v, w }
	}
}

/// The output of the IBE extract algorithm is a BLS signature
#[derive(Debug, Clone, CanonicalDeserialize, CanonicalSerialize, Serialize, Deserialize)]
pub struct IBESecret<E: EngineBLS>(pub E::SignatureGroup);

impl<E: EngineBLS> IBESecret<E> {
	/// BF-IBE decryption of a
	/// * `ciphertext`: C = <U, V, W>
	///
	/// Attempts to decrypt under the given IBESecret (in G1)
	pub fn decrypt(&self, ciphertext: &Ciphertext<E>) -> Result<Hash, IbeError> {
		// sigma = V (+) H2(e(d_id, U))
		let sigma_rhs = h2(E::pairing(ciphertext.u, self.0));
		let sigma = cross_product_const::<HASH_LENGTH>(&ciphertext.v, &sigma_rhs);
		// m = W (+) H4(sigma)
		let m_rhs = h4(&sigma);
		let m = cross_product_const::<HASH_LENGTH>(&ciphertext.w, &m_rhs);
		// check: U == rP
		let p = E::PublicKeyGroup::generator();
		let r = h3::<E>(&sigma, &m);
		let u_check = p * r;
		if !u_check.eq(&ciphertext.u) {
			return Err(IbeError::DecryptionFailed);
		}

		Ok(m)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::engines::drand::TinyBLS381;
	use alloc::vec;
	use ark_std::{test_rng, UniformRand};

	// this enum represents the conditions or branches that I want to test
	enum TestStatusReport {
		DecryptionResult { data: [u8; 32], verify: Vec<u8> },
		DecryptionFailure { error: IbeError },
	}

	/// Runs a test for the encryption and decryption process using the
	/// specified BLS engine.
	///
	/// This function performs the following steps:
	/// 1. Extracts the master secret key (msk) and secret key (sk) for the
	///    given identity.
	/// 2. Computes the public key `p_pub` using the master secret key.
	/// 3. Creates a `Ciphertext` structure, optionally inserting a bad
	///    ciphertext if specified.
	/// 4. Attempts to decrypt the ciphertext using the secret key.
	/// 5. Calls the provided handler with the result of the decryption attempt.
	fn run_test<EB: EngineBLS>(
		identity: Identity,
		message: [u8; 32],
		derive_bad_sk: bool,
		insert_bad_ciphertext: bool,
		handler: &dyn Fn(TestStatusReport) -> (),
	) {
		let (msk, sk) = extract::<EB>(identity.clone(), derive_bad_sk);

		let p_pub = <<EB as EngineBLS>::PublicKeyGroup as PrimeGroup>::generator() * msk;

		let mut ct = Ciphertext { u: EB::PublicKeyGroup::generator(), v: [0u8; 32], w: [0u8; 32] };

		if !insert_bad_ciphertext {
			ct = identity.encrypt(&Input::new(message).unwrap(), p_pub, &mut test_rng());
		}

		match sk.decrypt(&ct) {
			Ok(data) => {
				handler(TestStatusReport::DecryptionResult { data, verify: message.to_vec() });
			},
			Err(e) => {
				handler(TestStatusReport::DecryptionFailure { error: e });
			},
		}
	}

	fn extract<E: EngineBLS>(identity: Identity, derive_bad_sk: bool) -> (E::Scalar, IBESecret<E>) {
		let msk = <E as EngineBLS>::Scalar::rand(&mut test_rng());
		if derive_bad_sk {
			return (msk, IBESecret(E::SignatureGroup::generator()));
		}

		let sk = identity.extract::<E>(msk);
		(msk, sk)
	}

	#[test]
	pub fn fullident_identity_construction_works() {

		let identity = Identity::new(b"", &[1,2,3]);
		let expected_message = Message::new(b"", &[1,2,3]);
		assert_eq!(identity.0, expected_message);
	}

	#[test]
	pub fn fullident_encrypt_and_decrypt() {
		let identity = Identity::new(b"", &[1,2,3]);
		let message: [u8; 32] = [2; 32];

		run_test::<TinyBLS381>(identity, message, false, false, &|status: TestStatusReport| {
			match status {
				TestStatusReport::DecryptionResult { data, verify } => {
					assert_eq!(data.to_vec(), verify);
				},
				_ => panic!("Decryption should work"),
			}
		});
	}

	#[test]
	pub fn fullident_decryption_fails_with_bad_ciphertext() {
		let identity = Identity::new(b"", &[1,2,3]);
		let message: [u8; 32] = [2; 32];

		run_test::<TinyBLS381>(identity, message, false, true, &|status: TestStatusReport| {
			match status {
				TestStatusReport::DecryptionFailure { error } => {
					assert_eq!(error, IbeError::DecryptionFailed);
				},
				_ => panic!("all other conditions invalid"),
			}
		});
	}

	#[test]
	pub fn fullident_decryption_fails_with_bad_key() {
		let identity = Identity::new(b"", &[1,2,3]);
		let message: [u8; 32] = [2; 32];

		run_test::<TinyBLS381>(identity, message, true, false, &|status: TestStatusReport| {
			match status {
				TestStatusReport::DecryptionFailure { error } => {
					assert_eq!(error, IbeError::DecryptionFailed);
				},
				_ => panic!("all other conditions invalid"),
			}
		});
	}
}
