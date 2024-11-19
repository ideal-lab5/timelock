module.exports = {
    init: jest.fn().mockResolvedValue(undefined),
    build_encoded_commitment: jest.fn((bn, id) => `commitment-${bn}-${id}`),
    tle: jest.fn((id, message, key, pubKey) => `ciphertext-${id}-${message}`),
    tld: jest.fn((ciphertext, signature) => `plaintext-${ciphertext}-${signature}`),
    decrypt: jest.fn((ciphertext, key) => `plaintext-${ciphertext}-${key}`),
  };
  