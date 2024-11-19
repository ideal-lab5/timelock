import { vi } from 'vitest'

vi.mock('timelock-wasm-wrapper', () => ({
    default: vi.fn(),
    build_encoded_commitment: vi.fn().mockReturnValue('commitment'),
    tle: vi.fn().mockReturnValue('tle-ciphertext'),
    tld: vi.fn().mockReturnValue('tld-plaintext'),
    decrypt: vi.fn().mockReturnValue('plaintext'),
  }))
  