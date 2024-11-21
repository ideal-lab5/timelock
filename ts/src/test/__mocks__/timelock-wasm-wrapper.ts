// src\test\__mocks__\timelock-wasm-wrapper.ts
const init = jest.fn().mockResolvedValue(undefined)
const build_encoded_commitment = jest.fn().mockReturnValue('mocked_commitment')
const tle = jest.fn().mockResolvedValue('mocked_encrypted_data')
const tld = jest.fn().mockResolvedValue('mocked_decrypted_data')
const decrypt = jest.fn().mockResolvedValue('mocked_force_decrypted_data')

export default init
export { build_encoded_commitment, tle, tld, decrypt }
