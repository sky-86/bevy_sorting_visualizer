Size optimizations:
  opt-level='z' is smaller than opt-level='s'
  lto=true also made improvements
  wasm-opt halved the size
    wasm-opt -Oz -o output.wasm input.wasm
