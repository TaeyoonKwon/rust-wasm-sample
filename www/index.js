import * as wasm from "rust-wasm-sample";

wasm.greet("Ryan");

const result = wasm.calculate(2);

alert(`2 * 2 is ${result} from Rust`);
