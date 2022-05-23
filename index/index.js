import init from "./pkg/sort.js";

const wasm = await init("./pkg/sort_bg.wasm");

wasm.main();

/*document.getElementById("btn").addEventListener("click", btnClick);
function btnClick() {
    wasm.greet();
}*/

