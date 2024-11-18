module.exports = {
  preset: 'ts-jest', // Use ts-jest for TypeScript transformation
  testEnvironment: 'node', // Set the environment for tests
  transform: {
    '^.+\\.tsx?$': 'ts-jest', // Transform TypeScript files using ts-jest
    '^.+\\.js$': 'babel-jest', // Transform JavaScript files using Babel
  },
  transformIgnorePatterns: [
    'node_modules/(?!(some-esm-package)/)', // Include packages using ESM if needed
  ],
  extensionsToTreatAsEsm: ['.ts', '.tsx'], // Treat these extensions as ESM
  moduleNameMapper: {
    '^.+\\.wasm$': '<rootDir>/__mocks__/timelock-wasm-wrapper.js', // Mock WASM files for testing
  },
};
