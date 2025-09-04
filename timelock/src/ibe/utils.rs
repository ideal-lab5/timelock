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

use crate::engines::EngineBLS;
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use ark_std::vec::Vec;
use sha2::Digest;

/// sha256 hasher
pub fn sha256(b: &[u8]) -> Vec<u8> {
	let mut hasher = sha2::Sha256::new();
	hasher.update(b);
	hasher.finalize().to_vec()
}

#[inline(always)]
pub fn cross_product_const<const N: usize>(a: &[u8], b: &[u8]) -> [u8; N] {
	let mut result = [0u8; N];

	// Process 8 bytes at a time using u64 for better performance
	let chunks = N / 8;
	let remainder = N % 8;

	// Process full 8-byte chunks
	for i in 0..chunks {
		let start_idx = i * 8;
		let end_idx = start_idx + 8;

		let a_chunk = u64::from_ne_bytes(a[start_idx..end_idx].try_into().unwrap());
		let b_chunk = u64::from_ne_bytes(b[start_idx..end_idx].try_into().unwrap());
		let result_chunk = (a_chunk ^ b_chunk).to_ne_bytes();
		result[start_idx..end_idx].copy_from_slice(&result_chunk);
	}

	// Handle remaining bytes
	let remainder_start = chunks * 8;
	for j in 0..remainder {
		result[remainder_start + j] = a[remainder_start + j] ^ b[remainder_start + j];
	}

	result
}

/// a map from G -> {0, 1}^{32}
pub fn h2<G: CanonicalSerialize>(g: G) -> Vec<u8> {
	// let mut out = Vec::with_capacity(g.compressed_size());
	let mut out = Vec::new();
	g.serialize_compressed(&mut out)
		.expect("Enough space has been allocated in the buffer");
	sha256(&out)
}

// Should add a const to the signature so I can enforce sized inputs?
// right now this works with any size slices
/// H_3: {0,1}^n x {0, 1}^m -> Z_p
pub fn h3<E: EngineBLS>(a: &[u8], b: &[u8]) -> E::Scalar {
	let mut input = Vec::new();
	input.extend_from_slice(a);
	input.extend_from_slice(b);
	let hash = sha256(&input);
	E::Scalar::from_be_bytes_mod_order(&hash)
}

/// H_4: {0, 1}^n -> {0, 1}^n
pub fn h4(a: &[u8]) -> Vec<u8> {
	let o = sha256(a);
	o[..a.len()].to_vec()
}

#[cfg(test)]
mod test {

	use alloc::vec;

	#[test]
	fn utils_can_calc_sha256() {
		let actual = crate::ibe::utils::sha256(b"test");
		let expected = vec![
			159, 134, 208, 129, 136, 76, 125, 101, 154, 47, 234, 160, 197, 90, 208, 21, 163, 191,
			79, 27, 43, 11, 130, 44, 209, 93, 108, 21, 176, 240, 10, 8,
		];
		assert_eq!(actual, expected);
	}
}
