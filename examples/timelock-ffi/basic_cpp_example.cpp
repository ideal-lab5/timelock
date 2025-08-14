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

/**
 * @file basic_cpp_example.cpp
 * @brief Basic C++ example demonstrating timelock encryption FFI
 * 
 * This example demonstrates that the FFI works perfectly with C++:
 * 1. C++ string handling with automatic conversions
 * 2. RAII-style resource management 
 * 3. Exception-safe error handling patterns
 * 4. Modern C++ features alongside C FFI
 *
 * Build instructions:
 * Windows (MSVC): cl /EHsc basic_cpp_example.cpp timelock_ffi.lib /I. /Fe:basic_cpp_example.exe ntdll.lib bcrypt.lib advapi32.lib
 */

#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <iomanip>

// Include the C FFI header - works perfectly in C++!
extern "C" {
#include "timelock.h"
}

// C++ RAII wrapper for automatic cleanup
class TimelockCiphertextPtr {
private:
    TimelockCiphertext* ptr_;
    
public:
    TimelockCiphertextPtr() : ptr_(nullptr) {}
    
    explicit TimelockCiphertextPtr(TimelockCiphertext* ptr) : ptr_(ptr) {}
    
    ~TimelockCiphertextPtr() {
        if (ptr_) {
            timelock_ciphertext_free(ptr_);
        }
    }
    
    // Move semantics
    TimelockCiphertextPtr(TimelockCiphertextPtr&& other) noexcept : ptr_(other.ptr_) {
        other.ptr_ = nullptr;
    }
    
    TimelockCiphertextPtr& operator=(TimelockCiphertextPtr&& other) noexcept {
        if (this != &other) {
            if (ptr_) {
                timelock_ciphertext_free(ptr_);
            }
            ptr_ = other.ptr_;
            other.ptr_ = nullptr;
        }
        return *this;
    }
    
    // No copy semantics (move-only)
    TimelockCiphertextPtr(const TimelockCiphertextPtr&) = delete;
    TimelockCiphertextPtr& operator=(const TimelockCiphertextPtr&) = delete;
    
    TimelockCiphertext* get() const { return ptr_; }
    TimelockCiphertext** put() { return &ptr_; }
    
    bool valid() const { return ptr_ != nullptr; }
};

void print_hex(const std::string& label, const std::vector<uint8_t>& data) {
    std::cout << label << ": ";
    for (auto byte : data) {
        std::cout << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(byte);
    }
    std::cout << std::dec << std::endl;
}

void check_result(TimelockResult result, const std::string& operation) {
    if (result != Success) {
        std::string error_msg = "Unknown error";
        const char* c_error = timelock_get_last_error();
        if (c_error) {
            error_msg = c_error;
        }
        
        throw std::runtime_error("Operation '" + operation + "' failed with error " + 
                                std::to_string(result) + ": " + error_msg);
    }
}

int main() {
    try {
        std::cout << "=== C++ Timelock Encryption Example ===" << std::endl;
        std::cout << "Testing C FFI integration with modern C++\n" << std::endl;
        
        // Initialize library
        check_result(timelock_init(), "library initialization");
        
        // Modern C++ string handling
        const std::string message = "Hello from C++!";
        const uint64_t round_number = 1000;
        
        // Print library version using C++ streams
        const char* version = timelock_get_version();
        if (version) {
            std::cout << "Timelock library version: " << version << std::endl;
        }
        
        std::cout << "\nParameters:" << std::endl;
        std::cout << "  Message: \"" << message << "\"" << std::endl;
        std::cout << "  Round number: " << round_number << std::endl;
        std::cout << "  Message length: " << message.size() << " bytes\n" << std::endl;
        
        // Use std::vector for binary data - much safer than raw arrays
        std::vector<uint8_t> secret_key = {
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
            0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
            0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20
        };
        
        // C++ string literals
        const std::string public_key_hex = "83cf0f2896adee7eb8b5f01fcad3912212c437e0073e911fb90022d3e760183c8c4b450b6a0a6c3ac6a5776a2d1064510d1fec758c921cc22b0e17e63aaf4bcb5ed66304de9cf809bd274ca73bab4af5a6e9c76a4bc09e76eae8991ef5ece45a";
        // NOTE: The following signature is hardcoded for demonstration purposes and corresponds to round 1000 from Drand Quicknet.
        // If you wish to use a different round, you must obtain the correct signature for that round.
        // Drand QuickNet uses the "bls-unchained-g1-rfc9380" scheme, which places BLS signatures
        // on G1 (48 bytes) instead of the typical G2 (96 bytes). This is intentional and matches
        // the QuickNet protocol specification. There is no protocol mismatch here.
        // To obtain a valid signature for a given round, use the Drand HTTP API. For example:
        // curl https://api.drand.sh/52db9ba70e0cc0f6eaf7803dd07447a1f5477735fd3f661792ba94600c84e971/public/1000 | jq .signature
        // See the Drand API documentation for more details: https://drand.love/docs/http-api/
        // Last verified: August 14, 2025
        // WARNING: This is a hardcoded test signature for demo purposes only.
        // In production, fetch the actual signature from the Drand API for your specific round.
        const std::string signature_hex = "b44679b9a59af2ec876b1a6b1ad52ea9b1615fc3982b19576350f93447cb1125e342b73a8dd2bacbe47e4b6b63ed5e39";
        
        std::cout << "Cryptographic Parameters:" << std::endl;
        std::cout << "  Public key: " << public_key_hex << std::endl;
        std::cout << "  Signature: " << signature_hex << std::endl;
        print_hex("  Secret key", secret_key);
        
        // Step 1: Create identity
        std::cout << "\nStep 1: Creating identity for round " << round_number << "..." << std::endl;
        std::vector<uint8_t> identity(32);  // C++ vector instead of raw array
        check_result(
            timelock_create_drand_identity(round_number, identity.data(), identity.size()),
            "identity creation"
        );
        print_hex("[OK] Identity created", identity);
        
        // Step 2: Estimate ciphertext size
        std::cout << "\nStep 2: Estimating ciphertext size..." << std::endl;
        size_t estimated_size = 0;
        check_result(
            timelock_estimate_ciphertext_size(message.size(), &estimated_size),
            "size estimation"
        );
        std::cout << "[OK] Estimated ciphertext size: " << estimated_size << " bytes (overhead: " 
                  << (estimated_size - message.size()) << " bytes)" << std::endl;
        
        // Step 3: Encrypt using RAII wrapper
        std::cout << "\nStep 3: Encrypting message..." << std::endl;
        TimelockCiphertextPtr ciphertext;  // RAII - automatic cleanup!
        check_result(
            timelock_encrypt(
                reinterpret_cast<const uint8_t*>(message.c_str()),
                message.size(),
                identity.data(),
                identity.size(),
                public_key_hex.c_str(),
                secret_key.data(),
                ciphertext.put()
            ),
            "encryption"
        );
        
        std::cout << "[OK] Encryption successful!" << std::endl;
        std::cout << "   Actual ciphertext size: " << ciphertext.get()->len << " bytes" << std::endl;
        std::cout << "   Size estimation accuracy: " 
                  << std::fixed << std::setprecision(1)
                  << (100.0 * estimated_size / ciphertext.get()->len) << "%" << std::endl;
        
        // Step 4: Decrypt
        std::cout << "\nStep 4: Decrypting message..." << std::endl;
        std::vector<uint8_t> plaintext(message.size() + 100);  // C++ vector with margin
        size_t plaintext_len = plaintext.size();
        
        check_result(
            timelock_decrypt(
                ciphertext.get(),
                signature_hex.c_str(),
                plaintext.data(),
                &plaintext_len
            ),
            "decryption"
        );
        
        std::cout << "[OK] Decryption successful!" << std::endl;
        std::cout << "   Decrypted length: " << plaintext_len << " bytes" << std::endl;
        
        // Step 5: Verification using C++ string comparison
        std::cout << "\nStep 5: Verifying results..." << std::endl;
        std::string decrypted_message(reinterpret_cast<char*>(plaintext.data()), plaintext_len);
        
        if (decrypted_message == message) {
            std::cout << "[OK] Message verification passed!" << std::endl;
            std::cout << "   Original:  \"" << message << "\" (" << message.size() << " bytes)" << std::endl;
            std::cout << "   Decrypted: \"" << decrypted_message << "\" (" << plaintext_len << " bytes)" << std::endl;
        } else {
            throw std::runtime_error("Message verification failed!");
        }
        
        // Performance summary with C++ formatting
        std::cout << "\nPerformance Summary:" << std::endl;
        std::cout << "  Message size: " << message.size() << " bytes" << std::endl;
        std::cout << "  Ciphertext size: " << ciphertext.get()->len << " bytes" << std::endl;
        std::cout << "  Overhead ratio: " << std::fixed << std::setprecision(2)
                  << (static_cast<double>(ciphertext.get()->len) / message.size()) << "x" << std::endl;
        
        std::cout << "\n*** C++ integration test completed successfully!" << std::endl;
        std::cout << "   [OK] C FFI header works perfectly in C++" << std::endl;
        std::cout << "   [OK] RAII resource management working" << std::endl;
        std::cout << "   [OK] Modern C++ features integrated seamlessly" << std::endl;
        std::cout << "   [OK] Exception-safe error handling functional" << std::endl;
        
        timelock_cleanup();
        return 0;
        
    } catch (const std::exception& e) {
        std::cerr << "❌ Error: " << e.what() << std::endl;
        timelock_cleanup();
        return 1;
    }
}
