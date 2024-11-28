#!/usr/bin/env bash
#
# This shell script compiles the rust library to WebAssembly and creates a JS package using wasm-pack. 
# It also replaces any Node.js specific code with PythonMonkey code using sed.
#
# Note: this script is only compatible with unix.
#

cargo build
# compiles rust code to wasm and creates a package targetting Node.js
wasm-pack build --target nodejs

# replaces calls to require('path') and require('fs') with pythonmonkey non Node.js equivalents
NEW_READ_FILE_SYNC="function pyReadFileSync(filename) {\n  python.exec(\n\\\`\ndef getBytes(filename):\n  file = open(filename, 'rb')\n  return bytearray(file.read())\n\\\`\n  );\n  return python.eval('getBytes')(filename)\n}\n"

find ./pkg/*js -type f -exec sed -i "s/require('path').join(\(.*\));/[\1]\.join('\/');/g" {} \;
find ./pkg/*js -type f -exec sed -i "s/require('fs').readFileSync(\(.*\));/(${NEW_READ_FILE_SYNC})(\1)/g" {} \;