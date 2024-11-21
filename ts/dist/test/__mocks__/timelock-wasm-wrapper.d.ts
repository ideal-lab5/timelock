/// <reference types="jest" />
declare const init: jest.Mock<any, any, any>;
declare const build_encoded_commitment: jest.Mock<any, any, any>;
declare const tle: jest.Mock<any, any, any>;
declare const tld: jest.Mock<any, any, any>;
declare const decrypt: jest.Mock<any, any, any>;
export default init;
export { build_encoded_commitment, tle, tld, decrypt };
