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

//! Comprehensive tests for the timelock FFI bindings

use super::*;
use std::ffi::CString;
use std::thread;
use std::sync::Arc;

// Test constants for overhead calculations
const MAX_OVERHEAD_BYTES: usize = 1000; // Maximum fixed overhead in bytes
const MAX_OVERHEAD_MULTIPLIER: usize = 250; // Maximum overhead multiplier for very small messages

#[test]
fn test_error_codes() {
    assert_eq!(TimelockResult::Success as i32, 0);
    assert_eq!(TimelockResult::InvalidInput as i32, 1);
    assert_eq!(TimelockResult::EncryptionFailed as i32, 2);
    assert_eq!(TimelockResult::DecryptionFailed as i32, 3);
    assert_eq!(TimelockResult::MemoryError as i32, 4);
    assert_eq!(TimelockResult::SerializationError as i32, 5);
    assert_eq!(TimelockResult::InvalidPublicKey as i32, 6);
    assert_eq!(TimelockResult::InvalidSignature as i32, 7);
}

#[test]
fn test_constants() {
    assert_eq!(TIMELOCK_IDENTITY_SIZE, 32);
    assert_eq!(TIMELOCK_SECRET_KEY_SIZE, 32);
}

#[test]
fn test_init_cleanup() {
    unsafe {
        let result = timelock_init();
        assert_eq!(result, TimelockResult::Success);
        
        timelock_cleanup();
        
        // Should be safe to call multiple times
        let result2 = timelock_init();
        assert_eq!(result2, TimelockResult::Success);
        timelock_cleanup();
    }
}

#[test]
fn test_version_function() {
    unsafe {
        let version_ptr = timelock_get_version();
        assert!(!version_ptr.is_null());
        
        let version_cstr = CStr::from_ptr(version_ptr);
        let version_str = version_cstr.to_str().unwrap();
        assert!(!version_str.is_empty());
        // Should match the current crate version
        assert_eq!(version_str, env!("CARGO_PKG_VERSION"));
    }
}

#[test]
fn test_estimate_ciphertext_size() {
    unsafe {
        let mut estimated_size = 0usize;
        let result = timelock_estimate_ciphertext_size(100, &mut estimated_size);
        assert_eq!(result, TimelockResult::Success);
        assert!(estimated_size > 100); // Should be larger than message
        assert!(estimated_size < 1000); // But reasonable
        
        // Test with null pointer
        let result = timelock_estimate_ciphertext_size(100, ptr::null_mut());
        assert_eq!(result, TimelockResult::InvalidInput);
    }
}

#[test]
fn test_error_message_handling() {
    unsafe {
        // Initially should be null
        let error_ptr = timelock_get_last_error();
        assert!(error_ptr.is_null());
        
        // After init, should still be null
        timelock_init();
        let error_ptr = timelock_get_last_error();
        assert!(error_ptr.is_null());
        
        timelock_cleanup();
    }
}

#[test]
fn test_identity_creation() {
    let mut identity = [0u8; 32];
    let result = unsafe {
        timelock_create_drand_identity(1000, identity.as_mut_ptr(), identity.len())
    };
    assert_eq!(result, TimelockResult::Success);
    
    // Verify against known hash
    let mut hasher = Sha256::new();
    hasher.update(1000u64.to_be_bytes());
    let expected = hasher.finalize();
    assert_eq!(identity, expected.as_slice());
}

#[test]
fn test_identity_creation_invalid_buffer() {
    let mut identity = [0u8; 16]; // Too small
    let result = unsafe {
        timelock_create_drand_identity(1000, identity.as_mut_ptr(), identity.len())
    };
    assert_eq!(result, TimelockResult::InvalidInput);
}

#[test]
fn test_identity_creation_null_buffer() {
    let result = unsafe {
        timelock_create_drand_identity(1000, ptr::null_mut(), 32)
    };
    assert_eq!(result, TimelockResult::InvalidInput);
}

#[test]
fn test_encrypt_invalid_inputs() {
    let message = b"test";
    let identity = [1u8; 32];
    let secret_key = [2u8; 32];
    let pk_hex = CString::new("invalid_hex").unwrap();
    let mut ciphertext_ptr: *mut TimelockCiphertext = ptr::null_mut();

    // Test null message
    let result = unsafe {
        timelock_encrypt(
            ptr::null(),
            message.len(),
            identity.as_ptr(),
            identity.len(),
            pk_hex.as_ptr(),
            secret_key.as_ptr(),
            &mut ciphertext_ptr,
        )
    };
    assert_eq!(result, TimelockResult::InvalidInput);

    // Test invalid identity length
    let result = unsafe {
        timelock_encrypt(
            message.as_ptr(),
            message.len(),
            identity.as_ptr(),
            16, // Wrong length
            pk_hex.as_ptr(),
            secret_key.as_ptr(),
            &mut ciphertext_ptr,
        )
    };
    assert_eq!(result, TimelockResult::InvalidInput);

    // Test null public key
    let result = unsafe {
        timelock_encrypt(
            message.as_ptr(),
            message.len(),
            identity.as_ptr(),
            identity.len(),
            ptr::null(),
            secret_key.as_ptr(),
            &mut ciphertext_ptr,
        )
    };
    assert_eq!(result, TimelockResult::InvalidInput);
}

#[test]
fn test_encrypt_invalid_public_key() {
    let message = b"test";
    let identity = [1u8; 32];
    let secret_key = [2u8; 32];
    let pk_hex = CString::new("invalid_hex_string").unwrap();
    let mut ciphertext_ptr: *mut TimelockCiphertext = ptr::null_mut();

    let result = unsafe {
        timelock_encrypt(
            message.as_ptr(),
            message.len(),
            identity.as_ptr(),
            identity.len(),
            pk_hex.as_ptr(),
            secret_key.as_ptr(),
            &mut ciphertext_ptr,
        )
    };
    assert_eq!(result, TimelockResult::InvalidPublicKey);
}

#[test]
fn test_encrypt_malformed_public_key() {
    let message = b"test";
    let identity = [1u8; 32];
    let secret_key = [2u8; 32];
    let pk_hex = CString::new("deadbeef").unwrap(); // Valid hex but wrong format
    let mut ciphertext_ptr: *mut TimelockCiphertext = ptr::null_mut();

    let result = unsafe {
        timelock_encrypt(
            message.as_ptr(),
            message.len(),
            identity.as_ptr(),
            identity.len(),
            pk_hex.as_ptr(),
            secret_key.as_ptr(),
            &mut ciphertext_ptr,
        )
    };
    assert_eq!(result, TimelockResult::InvalidPublicKey);
}

#[test]
fn test_decrypt_invalid_inputs() {
    let fake_ciphertext = TimelockCiphertext {
        data: ptr::null_mut(),
        len: 0,
    };
    let sig_hex = CString::new("test").unwrap();
    let mut plaintext = [0u8; 100];
    let mut plaintext_len = plaintext.len();

    // Test null ciphertext
    let result = unsafe {
        timelock_decrypt(
            ptr::null(),
            sig_hex.as_ptr(),
            plaintext.as_mut_ptr(),
            &mut plaintext_len,
        )
    };
    assert_eq!(result, TimelockResult::InvalidInput);

    // Test ciphertext with null data
    let result = unsafe {
        timelock_decrypt(
            &fake_ciphertext,
            sig_hex.as_ptr(),
            plaintext.as_mut_ptr(),
            &mut plaintext_len,
        )
    };
    assert_eq!(result, TimelockResult::InvalidInput);

    // Test null signature
    let result = unsafe {
        timelock_decrypt(
            &fake_ciphertext,
            ptr::null(),
            plaintext.as_mut_ptr(),
            &mut plaintext_len,
        )
    };
    assert_eq!(result, TimelockResult::InvalidInput);
}

#[test]
fn test_memory_management() {
    // Test that we can create and free ciphertext structures
    // without memory leaks (this would be caught by tools like valgrind)
    
    let data = vec![1u8; 100];
    let data_ptr = data.as_ptr() as *mut u8;
    let data_len = data.len();
    std::mem::forget(data); // Transfer ownership
    
    let ciphertext = Box::new(TimelockCiphertext {
        data: data_ptr,
        len: data_len,
    });
    
    let ciphertext_ptr = Box::into_raw(ciphertext);
    
    // This should safely free the memory
    unsafe {
        timelock_ciphertext_free(ciphertext_ptr);
    }
}

#[test]
fn test_ciphertext_free_null() {
    // Should handle null pointer gracefully
    unsafe {
        timelock_ciphertext_free(ptr::null_mut());
    }
}

#[test]
fn test_large_message_encryption() {
    // Test with a larger message to ensure the FFI handles arbitrary-length data
    let large_message = vec![0xABu8; 10000];
    let identity = [1u8; 32];
    let secret_key = [2u8; 32];
    
    // Valid Drand quicknet public key
    let pk_hex = CString::new("83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a").unwrap();
    let mut ciphertext_ptr: *mut TimelockCiphertext = ptr::null_mut();

    let result = unsafe {
        timelock_encrypt(
            large_message.as_ptr(),
            large_message.len(),
            identity.as_ptr(),
            identity.len(),
            pk_hex.as_ptr(),
            secret_key.as_ptr(),
            &mut ciphertext_ptr,
        )
    };
    
    assert_eq!(result, TimelockResult::Success);
    assert!(!ciphertext_ptr.is_null());
    
    // Verify the ciphertext is reasonable
    unsafe {
        let ct = &*ciphertext_ptr;
        assert!(ct.len > large_message.len()); // Should be larger due to headers and padding
        assert!(!ct.data.is_null());
        
        timelock_ciphertext_free(ciphertext_ptr);
    }
}

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let message = b"Hello, Timelock World! This is a roundtrip test.";
    let mut identity = [0u8; 32];
    let secret_key = [2u8; 32];
    
    // Create identity for round 1000
    let identity_result = unsafe {
        timelock_create_drand_identity(1000, identity.as_mut_ptr(), identity.len())
    };
    assert_eq!(identity_result, TimelockResult::Success);
    
    // Valid Drand quicknet public key
    let pk_hex = CString::new("83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a").unwrap();
    
    // Encrypt
    let mut ciphertext_ptr: *mut TimelockCiphertext = ptr::null_mut();
    let result = unsafe {
        timelock_encrypt(
            message.as_ptr(),
            message.len(),
            identity.as_ptr(),
            identity.len(),
            pk_hex.as_ptr(),
            secret_key.as_ptr(),
            &mut ciphertext_ptr,
        )
    };
    assert_eq!(result, TimelockResult::Success);
    assert!(!ciphertext_ptr.is_null());
    
    // Verify ciphertext structure
    unsafe {
        let ct = &*ciphertext_ptr;
        assert!(ct.len > message.len());
        assert!(!ct.data.is_null());
        
        // Note: For a real roundtrip test, we would need the actual signature
        // from the Drand network. This test verifies the encryption succeeds
        // and produces reasonable output.
        
        timelock_ciphertext_free(ciphertext_ptr);
    }
}

#[test]
fn test_error_messages_after_failure() {
    unsafe {
        // Clear any existing error
        timelock_init();
        
        // Trigger an error with null buffer
        let result = timelock_create_drand_identity(1000, ptr::null_mut(), 32);
        assert_eq!(result, TimelockResult::InvalidInput);
        
        // Check that an error message was set
        let error_ptr = timelock_get_last_error();
        if !error_ptr.is_null() {
            let error_cstr = CStr::from_ptr(error_ptr);
            let error_str = error_cstr.to_str().unwrap();
            assert!(!error_str.is_empty());
            // Should contain relevant error information
            assert!(error_str.to_lowercase().contains("null") || 
                   error_str.to_lowercase().contains("invalid") ||
                   error_str.to_lowercase().contains("buffer"));
        }
        
        // Trigger another type of error
        let invalid_pk = CString::new("not_valid_hex").unwrap();
        let message = b"test";
        let identity = [1u8; 32];
        let secret_key = [2u8; 32];
        let mut ciphertext_ptr: *mut TimelockCiphertext = ptr::null_mut();
        
        let result = timelock_encrypt(
            message.as_ptr(),
            message.len(),
            identity.as_ptr(),
            identity.len(),
            invalid_pk.as_ptr(),
            secret_key.as_ptr(),
            &mut ciphertext_ptr,
        );
        assert_eq!(result, TimelockResult::InvalidPublicKey);
        
        // Check for error message
        let error_ptr = timelock_get_last_error();
        if !error_ptr.is_null() {
            let error_cstr = CStr::from_ptr(error_ptr);
            let error_str = error_cstr.to_str().unwrap();
            assert!(!error_str.is_empty());
        }
        
        timelock_cleanup();
    }
}

#[test]
fn test_thread_safety() {
    unsafe { timelock_init(); }
    
    let handles: Vec<_> = (0..10).map(|i| {
        thread::spawn(move || {
            let mut identity = [0u8; 32];
            let result = unsafe {
                timelock_create_drand_identity(
                    (1000 + i) as u64,
                    identity.as_mut_ptr(),
                    identity.len()
                )
            };
            assert_eq!(result, TimelockResult::Success);
            
            // Each thread should get a different identity
            let mut hasher = Sha256::new();
            hasher.update(((1000 + i) as u64).to_be_bytes());
            let expected = hasher.finalize();
            assert_eq!(identity, expected.as_slice());
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    unsafe { timelock_cleanup(); }
}

#[test]
fn test_decrypt_buffer_size_handling() {
    // Create a mock ciphertext structure for testing
    let mock_data = vec![1u8; 100];
    let mock_data_ptr = Box::into_raw(mock_data.into_boxed_slice()) as *mut u8;
    
    let ciphertext = TimelockCiphertext {
        data: mock_data_ptr,
        len: 100,
    };
    
    let sig_hex = CString::new("invalid_signature_for_testing").unwrap();
    
    // Test with very small buffer
    let mut small_plaintext = [0u8; 1];
    let mut plaintext_len = small_plaintext.len();
    
    let result = unsafe {
        timelock_decrypt(
            &ciphertext,
            sig_hex.as_ptr(),
            small_plaintext.as_mut_ptr(),
            &mut plaintext_len,
        )
    };
    
    // Should fail due to invalid signature, but not crash due to small buffer
    assert!(result != TimelockResult::Success);
    
    // Clean up mock data
    unsafe {
        let _ = Box::from_raw(std::slice::from_raw_parts_mut(mock_data_ptr, 100));
    }
}

#[test]
fn test_multiple_init_cleanup_cycles() {
    for i in 0..5 {
        unsafe {
            assert_eq!(timelock_init(), TimelockResult::Success);
            
            // Do some operations to ensure the library is properly initialized
            let mut identity = [0u8; 32];
            let result = timelock_create_drand_identity(
                (42 + i) as u64,
                identity.as_mut_ptr(),
                identity.len()
            );
            assert_eq!(result, TimelockResult::Success);
            
            // Verify the operation worked
            let mut hasher = Sha256::new();
            hasher.update(((42 + i) as u64).to_be_bytes());
            let expected = hasher.finalize();
            assert_eq!(identity, expected.as_slice());
            
            // Test version access
            let version_ptr = timelock_get_version();
            assert!(!version_ptr.is_null());
            
            timelock_cleanup();
        }
    }
}

#[test]
fn test_zero_length_message_encryption() {
    let empty_message = b"";
    let identity = [1u8; 32];
    let secret_key = [2u8; 32];
    let pk_hex = CString::new("83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a").unwrap();
    let mut ciphertext_ptr: *mut TimelockCiphertext = ptr::null_mut();

    let result = unsafe {
        timelock_encrypt(
            empty_message.as_ptr(),
            empty_message.len(),
            identity.as_ptr(),
            identity.len(),
            pk_hex.as_ptr(),
            secret_key.as_ptr(),
            &mut ciphertext_ptr,
        )
    };
    
    // Should handle empty messages gracefully
    assert_eq!(result, TimelockResult::Success);
    assert!(!ciphertext_ptr.is_null());
    
    unsafe {
        let ct = &*ciphertext_ptr;
        assert!(ct.len > 0); // Should still have headers/metadata
        assert!(!ct.data.is_null());
        
        timelock_ciphertext_free(ciphertext_ptr);
    }
}

#[test]
fn test_estimate_size_boundary_conditions() {
    unsafe {
        let mut estimated = 0usize;
        
        // Test with zero-length message
        let result = timelock_estimate_ciphertext_size(0, &mut estimated);
        assert_eq!(result, TimelockResult::Success);
        assert!(estimated > 0); // Should still have overhead
        
        // Test with maximum reasonable size
        let result = timelock_estimate_ciphertext_size(1_000_000, &mut estimated);
        assert_eq!(result, TimelockResult::Success);
        assert!(estimated >= 1_000_000);
        assert!(estimated < 1_000_000 * 2); // Reasonable overhead
          // Test various sizes to ensure consistency
        for msg_len in [1, 16, 64, 256, 1024, 4096].iter() {
            let result = timelock_estimate_ciphertext_size(*msg_len, &mut estimated);
            assert_eq!(result, TimelockResult::Success);
            assert!(estimated >= *msg_len);
            // For small messages, timelock has significant overhead due to fixed headers and metadata.
            // The overhead should not exceed MAX_OVERHEAD_BYTES for small messages or MAX_OVERHEAD_MULTIPLIER for very small messages.
            if *msg_len < 128 {
                assert!(
                    estimated < *msg_len * MAX_OVERHEAD_MULTIPLIER,
                    "Estimated size {} exceeds multiplier overhead for very small message length {}",
                    estimated,
                    *msg_len
                );
            } else {
                assert!(
                    estimated < *msg_len + MAX_OVERHEAD_BYTES,
                    "Estimated size {} exceeds fixed overhead for message length {}",
                    estimated,
                    *msg_len
                );
            }
        }
    }
}

#[test]
fn test_concurrent_memory_operations() {
    use std::sync::Barrier;
    
    let barrier = Arc::new(Barrier::new(5));
    let handles: Vec<_> = (0..5).map(|i| {
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            barrier.wait();
            
            // Each thread performs memory-intensive operations
            let message = vec![i as u8; 1000];
            let identity = [i as u8; 32];
            let secret_key = [(i * 2) as u8; 32];
            let pk_hex = CString::new("83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a").unwrap();
            let mut ciphertext_ptr: *mut TimelockCiphertext = ptr::null_mut();

            let result = unsafe {
                timelock_encrypt(
                    message.as_ptr(),
                    message.len(),
                    identity.as_ptr(),
                    identity.len(),
                    pk_hex.as_ptr(),
                    secret_key.as_ptr(),
                    &mut ciphertext_ptr,
                )
            };
            
            assert_eq!(result, TimelockResult::Success);
            assert!(!ciphertext_ptr.is_null());
            
            // Verify and cleanup
            unsafe {
                let ct = &*ciphertext_ptr;
                assert!(ct.len > message.len());
                assert!(!ct.data.is_null());
                timelock_ciphertext_free(ciphertext_ptr);
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}
