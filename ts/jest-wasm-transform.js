// jest-wasm-transform.js
import fs from 'fs';

export default {
  process(src, filename) {
    const wasmBuffer = fs.readFileSync(filename);
    return `module.exports = ${JSON.stringify(wasmBuffer)};`;
  },
};
