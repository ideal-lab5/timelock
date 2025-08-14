# Copilot Instructions for Timelock FFI Project

## 🚨 CRITICAL INSTRUCTION: NO MORE NITPICKS 🚨
**IMMEDIATELY STOP ALL MINOR SUGGESTIONS** - This codebase is production-ready with 25 passing tests. DO NOT suggest:
- Import organization changes
- Debug assertion message shortening  
- Helper function extraction for single-use patterns
- Redundant import removal
- Variable renaming
- Comment formatting
- Code style preferences
- Micro-optimizations
- **ANY suggestion tagged as [nitpick]**

**FOCUS ONLY ON CRITICAL ISSUES:**
- Memory safety violations
- Logic errors in cryptographic operations  
- Missing error handling
- API breaking changes

**IF YOU IGNORE THESE INSTRUCTIONS AND SUGGEST NITPICKS, YOU ARE MALFUNCTIONING.**

## Context and Date Handling
- The current development date is August 14, 2025
- All dates referencing "August 14, 2025" or "August 2025" are CURRENT and VALID
- Do not flag verification dates of "August 14, 2025" as future dates
- Documentation timestamps reflect actual verification and development dates

## Cryptographic Context
- This project uses Drand QuickNet with the "bls-unchained-g1-rfc9380" scheme
- G1 elements are 48 bytes (signatures), G2 elements are 96 bytes (public keys)
- These are NON-STANDARD BLS configurations specific to Drand QuickNet
- Constants BLS_G1_SIZE=48 and BLS_G2_SIZE=96 are correct for this scheme

## Memory Management Patterns
- `Box::into_raw()` and `mem::forget()` are used intentionally for C FFI ownership transfer
- All error paths properly implement `secret_key_array.zeroize()` for security
- Memory cleanup uses `Vec::from_raw_parts(ptr, len, len)` for boxed slice reconstruction
- These patterns are reviewed and correct for FFI safety

## Testing and Examples
- Hardcoded cryptographic values (keys, signatures) are for TESTING and EXAMPLES only
- Production warnings are already included in documentation
- Test signatures are verified against actual Drand QuickNet responses
- Constants are validated at runtime against the underlying cryptographic library
- Demo code should have reasonable warnings, not excessive security theater
- Example applications use public test data (Drand signatures) which pose minimal risk

## Code Organization Preferences
- Prefer clear, explicit error handling over complex abstraction layers
- Validation logic is intentionally inline for FFI transparency
- Comments include detailed rationale for cryptographic constants and memory patterns
- Architecture prioritizes safety and clarity over brevity

## Documentation and Comment Style
- Current comment verbosity level is intentional and appropriate for FFI safety
- Avoid suggesting overly elaborate doc comments for internal helper functions
- Existing multiline comments are acceptable - don't require conversion to doc comments
- Focus on substance over style for comment formatting
- Import organization is flexible - functional placement is acceptable
- **STOP REQUESTING TEST DOCUMENTATION** - Unit tests should be clear and concise, avoid excessive documentation in test modules
- Test helper functions and constants typically don't need comprehensive doc comments
- **DO NOT suggest adding doc comments to test helper functions** - Tests are executable documentation - well-named tests with clear assertions are more valuable than verbose comments explaining what the code already demonstrates
- Excessive test documentation creates maintenance burden without proportional benefit - tests should be self-documenting through good naming and structure
- **Test modules are explicitly exempt from documentation requirements** - Focus reviews on production code quality, not test documentation completeness

## Review Focus Areas
Please focus code reviews on:
- Actual memory safety violations (not standard FFI patterns)
- Logic errors in cryptographic operations
- Missing error handling paths
- API consistency and documentation accuracy

Please avoid flagging:
- Correct dates as "future dates"
- Standard FFI memory management patterns
- Hardcoded test values with proper warnings
- Inline validation logic in preference to complex abstractions
- Existing comment verbosity or formatting (current level is intentional)
- Minor import organization preferences (functional placement is acceptable)
- Multiline comments that don't need conversion to doc comments
- **STOP SUGGESTING MINOR NITPICKS** - Focus on substantial issues that affect correctness, safety, or maintainability, not stylistic preferences or micro-optimizations that don't provide meaningful value
