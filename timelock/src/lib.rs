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

#![no_std]
#![warn(
	unused,
	future_incompatible,
	nonstandard_style,
	rust_2018_idioms,
	rust_2021_compatibility
)]
#![deny(unsafe_code)]

extern crate alloc;

pub mod block_ciphers;
pub mod engines;
pub mod ibe;
pub mod tlock;
use crate::engines::EngineBLS;

// Adapted from: https://github.com/w3f/bls
/// Internal message hash size.  
///
/// We choose 256 bits here so that birthday bound attacks cannot
/// find messages with the same hash.
const MESSAGE_SIZE: usize = 32;

type MessageDigest = [u8; MESSAGE_SIZE];
/// Internal message hash type.  Short for frequent rehashing
/// by `HashMap`, etc.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message(pub MessageDigest, pub alloc::vec::Vec<u8>);

impl Message {
	pub fn new(context: &[u8], message: &[u8]) -> Message {
		use sha3::{
			digest::{ExtendableOutput, Update, XofReader},
			Shake128,
		};
		let mut h = Shake128::default();
		h.update(context);
		let l = message.len() as u64;
		h.update(&l.to_le_bytes());
		h.update(message);
		let mut msg = [0u8; MESSAGE_SIZE];
		h.finalize_xof().read(&mut msg[..]);
		Message(msg, [context, message].concat())
	}

	pub fn hash_to_signature_curve<E: EngineBLS>(&self) -> E::SignatureGroup {
		E::hash_to_signature_curve(&self.1[..])
	}
}

impl From<&[u8]> for Message {
	fn from(x: &[u8]) -> Message {
		Message::new(b"", x)
	}
}
