# Size optimizations:
  
opt-level='z' is smaller than opt-level='s'
<br>
lto=true also made improvements
<br>
wasm-opt halved the size
<br>
wasm-opt -Oz -o output.wasm input.wasm
