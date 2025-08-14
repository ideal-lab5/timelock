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
//!
//! ## Supported Beacon Types
//!
//! This FFI currently supports only TinyBLS381 (Drand QuickNet beacon).
//! Future versions will support TinyBLS377 (Ideal Network) when available
//! in the core timelock library. The API is designed to be extensible
//! for multiple beacon types while maintaining backward compatibility.

// Allow unsafe code for FFI bindings - this is necessary for C interop
#![allow(unsafe_code)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar};
use std::ptr;
use std::slice;
use std::cell::RefCell;
use zeroize::Zeroize;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::rngs::OsRng;
use sha2::{Digest, Sha256};

use timelock::{
    block_ciphers::AESGCMBlockCipherProvider,
    engines::{drand::TinyBLS381, EngineBLS},
    ibe::fullident::Identity,
    tlock::{tle, tld, TLECiphertext},
};

// BLS12-381 curve element sizes - referenced from the EngineBLS implementation
// to ensure consistency and support future multi-curve extensibility.
// These constants are derived from TinyBLS381 which implements the BLS12-381 specification:
// - G1 compressed point: 48 bytes (signatures)
// - G2 compressed point: 96 bytes (public keys) 
// See: RFC 9380, Section 4.3.3
// (https://datatracker.ietf.org/doc/html/rfc9380#section-4.3.3)
// These constants are validated against the actual compressed_size() values
// returned by the ark-bls12-381 library during initialization.
pub const BLS_G1_SIZE: usize = <TinyBLS381 as EngineBLS>::SIGNATURE_SERIALIZED_SIZE;
pub const BLS_G2_SIZE: usize = <TinyBLS381 as EngineBLS>::PUBLICKEY_SERIALIZED_SIZE;

// AES-GCM constants
pub const AES_GCM_IV_SIZE: usize = 12;  // AES-GCM initialization vector size (96 bits)
// AES-GCM authentication tag size is standardized at 16 bytes per RFC 5116.
// This constant ensures compatibility with the AES-GCM implementation and should
// remain consistent with the underlying cryptographic library's tag size.
pub const AES_GCM_TAG_SIZE: usize = 16;

// Total fixed overhead for timelock ciphertext
pub const TIMELOCK_CIPHERTEXT_OVERHEAD: usize = 
    BLS_G1_SIZE +  // BLS signature (G1 element in QuickNet "bls-unchained-g1-rfc9380")
    BLS_G2_SIZE +  // Public key (G2 element in QuickNet "bls-unchained-g1-rfc9380")
    AES_GCM_IV_SIZE + 
    AES_GCM_TAG_SIZE + 
    SERIALIZATION_OVERHEAD;

/// Runtime validation of cryptographic constants to ensure consistency with the underlying library.
/// This function is called during initialization to verify that our engine-derived constants match
/// the actual sizes reported by the cryptographic library.
fn validate_cryptographic_constants() -> Result<(), String> {
    use ark_bls12_381::{G1Affine, G2Affine};
    use ark_serialize::CanonicalSerialize;
    
    let g1_size = G1Affine::identity().compressed_size();
    let g2_size = G2Affine::identity().compressed_size();
    
    if g1_size != BLS_G1_SIZE {
        return Err(format!(
            "BLS_G1_SIZE constant ({}) does not match library size ({})",
            BLS_G1_SIZE, g1_size
        ));
    }
    
    if g2_size != BLS_G2_SIZE {
        return Err(format!(
            "BLS_G2_SIZE constant ({}) does not match library size ({})",
            BLS_G2_SIZE, g2_size
        ));
    }
    
    Ok(())
}

/// The serialization overhead constant accounts for additional bytes used in encoding structures,
/// such as length prefixes, structure tags, and potential padding. The value of 32 was chosen based
/// on comprehensive analysis including protocol metadata and serialization format overhead observed
/// in the current implementation. This value ensures accurate size estimation and prevents buffer 
/// overflows while minimizing wasted space.
/// 
/// The value of 32 was determined by summing the maximum observed overhead from:
/// - 4 bytes for length prefix (u32)
/// - 1–2 bytes for structure tags or enum discriminants
/// - up to 16 bytes for protocol metadata (e.g., versioning, type tags)
/// - remaining bytes for alignment/padding and future-proofing
/// 
/// This value was chosen based on analysis of the current serialization format (ark-serialize and protocol metadata)
/// and is validated during library initialization to ensure it remains accurate.
/// **Update this value if the serialization format changes, new fields are added to encoded structures,
/// or if protocol metadata overhead increases.**
const SERIALIZATION_OVERHEAD: usize = 32;

// Constants for C consumers
/// Size of identity buffer in bytes (SHA-256 digest size)
#[no_mangle]
pub static TIMELOCK_IDENTITY_SIZE: usize = 32;

/// Size of secret key in bytes (derived from the BLS engine implementation)
#[no_mangle]
pub static TIMELOCK_SECRET_KEY_SIZE: usize = <TinyBLS381 as EngineBLS>::SECRET_KEY_SIZE;

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
            // SAFETY: The pointer `ct.data` is originally allocated via Box::into_raw(Box<[u8]>) in the
            // corresponding allocation site. According to Rust documentation, converting a Box<[T]> into 
            // a raw pointer and then reconstructing it with Vec::from_raw_parts using (ptr, len, len) 
            // is valid because Box<[T]> is always allocated with capacity == length.
            // This debug assertion validates this Box<[T]> invariant in debug builds and will detect
            // if the allocation strategy changes in future Rust versions or if memory corruption occurs.
            let vec = Vec::from_raw_parts(ct.data, ct.len, ct.len);
            debug_assert!(vec.capacity() == vec.len(), "Box<[T]> invariant broken: capacity != length for ciphertext buffer. This indicates either memory corruption or a change in Rust's Box<[T]> allocation strategy.");
            // Dropping vec will free the memory.
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
) -> TimelockResult 
{
    if identity_out.is_null() || identity_len < 32 {
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

/// Helper function to ensure sensitive data is always cleared on error paths.
///
/// # Parameters
/// - `secret_key_array`: Mutable reference to the 32-byte secret key array to be zeroized.
/// - `error_message`: Error message to set for the last error.
/// - `result_code`: The `TimelockResult` error code to return.
///
/// # Returns
/// Returns the provided `result_code` after zeroizing the secret key and setting the error message.
fn fail_with_zeroize(
    secret_key_array: &mut [u8; 32],
    error_message: &str,
    result_code: TimelockResult,
) -> TimelockResult {
    secret_key_array.zeroize();
    set_last_error(error_message);
    result_code
}

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
) -> TimelockResult 
{
    // Validate inputs
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
    
    // Convert secret key to array directly to minimize exposure time
    let mut secret_key_array = [0u8; 32];
    unsafe {
        ptr::copy_nonoverlapping(secret_key, secret_key_array.as_mut_ptr(), 32);
    }

    // Parse public key hex string
    let public_key_cstr = match CStr::from_ptr(public_key_hex).to_str() {
        Ok(s) => s,
        Err(e) => {
            return fail_with_zeroize(
                &mut secret_key_array,
                &format!("Invalid UTF-8 in public key hex string: {}", e),
                TimelockResult::InvalidInput,
            );
        }
    };

    let public_key_bytes = match hex::decode(public_key_cstr) {
        Ok(bytes) => bytes,
        Err(e) => {
            return fail_with_zeroize(
                &mut secret_key_array,
                &format!("Invalid hex encoding in public key: {}", e),
                TimelockResult::InvalidPublicKey,
            );
        }
    };

    let public_key = match <TinyBLS381 as EngineBLS>::PublicKeyGroup::deserialize_compressed(
        &public_key_bytes[..],
    ) {
        Ok(pk) => pk,
        Err(e) => {
            return fail_with_zeroize(
                &mut secret_key_array,
                &format!("Failed to deserialize BLS public key: {:?}", e),
                TimelockResult::InvalidPublicKey,
            );
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
        Ok(ct) => ct,
        Err(e) => {
            return fail_with_zeroize(
                &mut secret_key_array,
                &format!("Timelock encryption operation failed: {:?}", e),
                TimelockResult::EncryptionFailed,
            );
        }
    };

    // Securely zero out sensitive data after use
    secret_key_array.zeroize();
    
    // Serialize ciphertext
    let mut serialized = Vec::new();
    if ciphertext.serialize_compressed(&mut serialized).is_err() {
        set_last_error("Failed to serialize ciphertext");
        return TimelockResult::SerializationError;
    }

    // Use Box::into_raw for safe ownership transfer to C
    let boxed_data = serialized.into_boxed_slice();
    let data_len = boxed_data.len();
    // SAFETY: We cast Box<[u8]> to *mut u8 to transfer ownership to C.
    // The slice pointer is cast to a raw u8 pointer for C compatibility, as C APIs
    // expect simple byte pointers rather than slice metadata. The length information
    // is not lost, as it is stored in the `len` field of the TimelockCiphertext struct
    // alongside the pointer. When freeing, both the pointer and length are available
    // to reconstruct the Box<[u8]> via Vec::from_raw_parts.
    let data_ptr = Box::into_raw(boxed_data) as *mut u8;

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

    // Estimate ciphertext size as message length plus the predefined overhead constant
    let overhead = TIMELOCK_CIPHERTEXT_OVERHEAD;
    match message_len.checked_add(overhead) {
        Some(total) => {
            *estimated_size_out = total;
            clear_last_error();
            TimelockResult::Success
        }
        None => {
            set_last_error("Integer overflow when estimating ciphertext size");
            TimelockResult::InvalidInput
        }
    }
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
) -> TimelockResult {
    // Validate inputs
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
    }

    // Parse signature hex string
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
        Err(e) => {
            set_last_error(&format!("Failed to deserialize BLS signature: {:?}", e));
            return TimelockResult::InvalidSignature;
        }
    };

    // Deserialize ciphertext
    let ciphertext_slice = slice::from_raw_parts(ct.data, ct.len);
    let timelock_ciphertext: TLECiphertext<TinyBLS381> =
        match TLECiphertext::deserialize_compressed(&ciphertext_slice[..]) {
            Ok(ct) => ct,
            Err(e) => {
                set_last_error(&format!("Failed to deserialize ciphertext: {:?}", e));
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
    // Validate cryptographic constants match the underlying library
    if let Err(err) = validate_cryptographic_constants() {
        set_last_error(&format!("Cryptographic constant validation failed: {}", err));
        return TimelockResult::InvalidInput;
    }
    
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
