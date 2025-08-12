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

//! C-compatible FFI bindings for the timelock encryption library.
//!
//! This crate provides C-compatible wrapper functions for timelock encryption
//! and decryption operations, enabling integration with C/C++ projects.

// Allow unsafe code for FFI bindings - this is necessary for C interop
#![allow(unsafe_code)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar};
use std::ptr;
use std::slice;
use std::cell::RefCell;

use timelock::{
    block_ciphers::AESGCMBlockCipherProvider,
    engines::{drand::TinyBLS381, EngineBLS},
    ibe::fullident::Identity,
    tlock::{tle, tld, TLECiphertext},
};

// NOTE: Current Implementation - Drand QuickNet (BLS12-381) Only
// This FFI currently supports only TinyBLS381 (Drand QuickNet beacon).
// Future versions will support TinyBLS377 (Ideal Network) when available
// in the core timelock library. The API is designed to be extensible
// for multiple beacon types while maintaining backward compatibility.
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::rngs::OsRng;
use sha2::{Digest, Sha256};

// Constants for C consumers
/// Size of identity buffer in bytes
#[no_mangle]
pub static TIMELOCK_IDENTITY_SIZE: usize = 32;

/// Size of secret key in bytes
#[no_mangle]
pub static TIMELOCK_SECRET_KEY_SIZE: usize = 32;

// Thread-local storage for error messages
thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = RefCell::new(None);
}

/// Set the last error message (internal helper)
fn set_last_error(message: &str) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = CString::new(message).ok();
    });
}

/// Clear the last error message (internal helper)
fn clear_last_error() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
}

/// Result codes for timelock operations
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum TimelockResult {
    /// Operation succeeded
    Success = 0,
    /// Invalid input parameters
    InvalidInput = 1,
    /// Encryption failed
    EncryptionFailed = 2,
    /// Decryption failed
    DecryptionFailed = 3,
    /// Memory allocation failed
    MemoryError = 4,
    /// Serialization/deserialization failed
    SerializationError = 5,
    /// Invalid public key
    InvalidPublicKey = 6,
    /// Invalid signature
    InvalidSignature = 7,
}

/// Opaque handle for encrypted data
#[repr(C)]
pub struct TimelockCiphertext {
    /// Pointer to the encrypted data
    pub data: *mut c_uchar,
    /// Length of the encrypted data
    pub len: usize,
}

/// Free memory allocated for ciphertext
///
/// # Safety
/// - `ciphertext` must be a valid pointer returned by a timelock function
/// - `ciphertext` must not be used after calling this function
#[no_mangle]
pub unsafe extern "C" fn timelock_ciphertext_free(ciphertext: *mut TimelockCiphertext) {
    if !ciphertext.is_null() {
        let ct = Box::from_raw(ciphertext);
        if !ct.data.is_null() {
            let _ = Vec::from_raw_parts(ct.data, ct.len, ct.len);
        }
    }
}

/// Create an identity for a given round number (Drand-style)
///
/// This creates an identity by hashing the round number as used by Drand quicknet.
///
/// # Parameters
/// - `round_number`: The round number for which to create the identity
/// - `identity_out`: Output buffer for the identity (must be at least 32 bytes)
/// - `identity_len`: Length of the output buffer
///
/// # Returns
/// `TimelockResult::Success` on success, error code on failure
///
/// # Safety
/// - `identity_out` must be a valid pointer to a buffer of at least `identity_len` bytes
#[no_mangle]
pub unsafe extern "C" fn timelock_create_drand_identity(
    round_number: u64,
    identity_out: *mut c_uchar,
    identity_len: usize,
) -> TimelockResult {    if identity_out.is_null() || identity_len < 32 {
        set_last_error("Invalid identity buffer: null pointer or insufficient size (need 32 bytes)");
        return TimelockResult::InvalidInput;
    }

    let mut hasher = Sha256::new();
    hasher.update(round_number.to_be_bytes());
    let hash = hasher.finalize();

    let output = slice::from_raw_parts_mut(identity_out, identity_len);
    output[..32].copy_from_slice(&hash);

    clear_last_error();
    TimelockResult::Success
}

/// Encrypt a message using timelock encryption
///
/// # Parameters
/// - `message`: Pointer to the message to encrypt
/// - `message_len`: Length of the message
/// - `identity`: Pointer to the identity (32 bytes)
/// - `identity_len`: Length of the identity (must be 32)
/// - `public_key_hex`: Null-terminated hex string of the public key
/// - `secret_key`: 32-byte secret key for encryption
/// - `ciphertext_out`: Output pointer for the encrypted ciphertext
///
/// # Returns
/// `TimelockResult::Success` on success, error code on failure
///
/// # Safety
/// - All pointer parameters must be valid
/// - `message` must point to `message_len` bytes
/// - `identity` must point to 32 bytes
/// - `secret_key` must point to 32 bytes
/// - `public_key_hex` must be a valid null-terminated C string
/// - `ciphertext_out` will be set to a pointer that must be freed with `timelock_ciphertext_free`
#[no_mangle]
pub unsafe extern "C" fn timelock_encrypt(
    message: *const c_uchar,
    message_len: usize,
    identity: *const c_uchar,
    identity_len: usize,
    public_key_hex: *const c_char,
    secret_key: *const c_uchar,
    ciphertext_out: *mut *mut TimelockCiphertext,
) -> TimelockResult {    // Validate inputs
    if message.is_null()
        || identity.is_null()
        || public_key_hex.is_null()
        || secret_key.is_null()
        || ciphertext_out.is_null()
        || identity_len != 32
    {
        set_last_error("Invalid input parameters: null pointers or incorrect identity length (need 32 bytes)");
        return TimelockResult::InvalidInput;
    }

    // Convert inputs
    let message_slice = slice::from_raw_parts(message, message_len);
    let identity_slice = slice::from_raw_parts(identity, identity_len);
    let secret_key_slice = slice::from_raw_parts(secret_key, 32);    // Convert secret key to array
    let mut secret_key_array = [0u8; 32];
    secret_key_array.copy_from_slice(secret_key_slice);

    // Parse public key hex string
    let public_key_cstr = match CStr::from_ptr(public_key_hex).to_str() {
        Ok(s) => s,        Err(_) => {
            // Zero out sensitive data before returning
            secret_key_array.fill(0);
            set_last_error("Invalid UTF-8 in public key hex string");
            return TimelockResult::InvalidInput;
        }
    };    let public_key_bytes = match hex::decode(public_key_cstr) {
        Ok(bytes) => bytes,
        Err(_) => {
            // Zero out sensitive data before returning
            secret_key_array.fill(0);
            set_last_error("Invalid hex encoding in public key");
            return TimelockResult::InvalidPublicKey;
        }
    };

    let public_key = match <TinyBLS381 as EngineBLS>::PublicKeyGroup::deserialize_compressed(
        &public_key_bytes[..],
    ) {
        Ok(pk) => pk,        Err(_) => {
            // Zero out sensitive data before returning
            secret_key_array.fill(0);
            set_last_error("Failed to deserialize BLS public key");
            return TimelockResult::InvalidPublicKey;
        }
    };

    // Create identity
    let timelock_identity = Identity::new(b"", vec![identity_slice.to_vec()]);

    // Perform encryption
    let ciphertext = match tle::<TinyBLS381, AESGCMBlockCipherProvider, OsRng>(
        public_key,
        secret_key_array,
        message_slice,
        timelock_identity,
        OsRng,
    ) {
        Ok(ct) => ct,        Err(_) => {
            // Zero out sensitive data before returning
            secret_key_array.fill(0);
            set_last_error("Timelock encryption operation failed");
            return TimelockResult::EncryptionFailed;
        }
    };

    // Zero out sensitive data after use
    secret_key_array.fill(0);    // Serialize ciphertext
    let mut serialized = Vec::new();
    if ciphertext.serialize_compressed(&mut serialized).is_err() {
        set_last_error("Failed to serialize ciphertext");
        return TimelockResult::SerializationError;
    }

    // Allocate output
    let data_ptr = serialized.as_mut_ptr();
    let data_len = serialized.len();
    std::mem::forget(serialized); // Transfer ownership to C

    let result = Box::new(TimelockCiphertext {
        data: data_ptr,
        len: data_len,
    });

    *ciphertext_out = Box::into_raw(result);

    clear_last_error();
    TimelockResult::Success
}

/// Estimate the size of the ciphertext for a given message length
///
/// This function provides an estimate of the serialized ciphertext size,
/// which can be useful for C callers to pre-allocate buffers.
///
/// # Parameters
/// - `message_len`: Length of the message to be encrypted
/// - `estimated_size_out`: Output pointer for the estimated size
///
/// # Returns
/// `TimelockResult::Success` on success, error code on failure
///
/// # Safety
/// - `estimated_size_out` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn timelock_estimate_ciphertext_size(
    message_len: usize,
    estimated_size_out: *mut usize,
) -> TimelockResult {
    if estimated_size_out.is_null() {
        set_last_error("Null output pointer for estimated size");
        return TimelockResult::InvalidInput;
    }

    // Rough estimate: message + overhead for encryption structures
    // This is a conservative estimate based on typical ciphertext sizes
    let overhead = 200; // Approximate overhead for BLS elements, AES-GCM auth tag, etc.
    *estimated_size_out = message_len + overhead;

    clear_last_error();
    TimelockResult::Success
}

/// Decrypt a timelock-encrypted ciphertext
///
/// # Parameters
/// - `ciphertext`: Pointer to the encrypted ciphertext
/// - `signature_hex`: Null-terminated hex string of the signature
/// - `plaintext_out`: Output buffer for the decrypted plaintext
/// - `plaintext_len`: Pointer to the length of the output buffer, updated with actual length
///
/// # Returns
/// `TimelockResult::Success` on success, error code on failure
///
/// # Safety
/// - `ciphertext` must be a valid pointer returned by `timelock_encrypt`
/// - `signature_hex` must be a valid null-terminated C string
/// - `plaintext_out` must point to a buffer of at least `*plaintext_len` bytes
/// - `plaintext_len` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn timelock_decrypt(
    ciphertext: *const TimelockCiphertext,
    signature_hex: *const c_char,
    plaintext_out: *mut c_uchar,
    plaintext_len: *mut usize,
) -> TimelockResult {    // Validate inputs
    if ciphertext.is_null()
        || signature_hex.is_null()
        || plaintext_out.is_null()
        || plaintext_len.is_null()
    {
        set_last_error("Invalid input parameters: null pointers not allowed");
        return TimelockResult::InvalidInput;
    }

    let ct = &*ciphertext;
    if ct.data.is_null() {
        set_last_error("Invalid ciphertext: null data pointer");
        return TimelockResult::InvalidInput;
    }    // Parse signature hex string
    let signature_cstr = match CStr::from_ptr(signature_hex).to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("Invalid UTF-8 in signature hex string");
            return TimelockResult::InvalidInput;
        }
    };

    let signature_bytes = match hex::decode(signature_cstr) {
        Ok(bytes) => bytes,
        Err(_) => {
            set_last_error("Invalid hex encoding in signature");
            return TimelockResult::InvalidSignature;
        }
    };

    let signature = match <TinyBLS381 as EngineBLS>::SignatureGroup::deserialize_compressed(
        &signature_bytes[..],
    ) {
        Ok(sig) => sig,
        Err(_) => {
            set_last_error("Failed to deserialize BLS signature");
            return TimelockResult::InvalidSignature;
        }
    };    // Deserialize ciphertext
    let ciphertext_slice = slice::from_raw_parts(ct.data, ct.len);
    let timelock_ciphertext: TLECiphertext<TinyBLS381> =
        match TLECiphertext::deserialize_compressed(&ciphertext_slice[..]) {
            Ok(ct) => ct,
            Err(_) => {
                set_last_error("Failed to deserialize ciphertext");
                return TimelockResult::SerializationError;
            }
        };

    // Perform decryption
    let plaintext_result = match tld::<TinyBLS381, AESGCMBlockCipherProvider>(
        timelock_ciphertext,
        signature,
    ) {
        Ok(plaintext) => plaintext,
        Err(_) => {
            set_last_error("Timelock decryption failed: signature may be invalid, round may be in the future, or ciphertext may be corrupted");
            return TimelockResult::DecryptionFailed;
        }
    };

    // Check if output buffer is large enough
    if *plaintext_len < plaintext_result.len() {
        *plaintext_len = plaintext_result.len();
        return TimelockResult::MemoryError;
    }

    // Copy result to output buffer
    let output = slice::from_raw_parts_mut(plaintext_out, *plaintext_len);
    output[..plaintext_result.len()].copy_from_slice(&plaintext_result);
    *plaintext_len = plaintext_result.len();

    clear_last_error();
    TimelockResult::Success
}

/// Get the last error message (if any)
///
/// # Returns
/// Null-terminated string with the last error message, or null if no error
///
/// # Safety
/// The returned pointer is valid until the next call to any timelock function
#[no_mangle]
pub unsafe extern "C" fn timelock_get_last_error() -> *const c_char {
    LAST_ERROR.with(|e| {
        if let Some(ref cstring) = *e.borrow() {
            cstring.as_ptr()
        } else {
            ptr::null()
        }
    })
}

/// Get the version of the timelock library
///
/// # Returns
/// Null-terminated string with the version (static, no need to free)
#[no_mangle]
pub unsafe extern "C" fn timelock_get_version() -> *const c_char {
    static VERSION: &[u8] = concat!(env!("CARGO_PKG_VERSION"), "\0").as_bytes();
    VERSION.as_ptr() as *const c_char
}

/// Initialize the timelock library
///
/// Call this function before using any other timelock functions.
/// It's safe to call this multiple times.
///
/// # Returns
/// `TimelockResult::Success` on success
#[no_mangle]
pub unsafe extern "C" fn timelock_init() -> TimelockResult {
    // Initialize any global state if needed
    // For now, just clear any existing error state
    clear_last_error();
    TimelockResult::Success
}

/// Clean up the timelock library
///
/// Call this function when you're done using the timelock library.
/// It's safe to call this multiple times.
#[no_mangle]
pub unsafe extern "C" fn timelock_cleanup() {
    // Clean up any global resources
    clear_last_error();
}

#[cfg(test)]
mod tests;
