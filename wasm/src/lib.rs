use wasm_bindgen::prelude::*;
// use wasm_bindgen_test::*;

#[wasm_bindgen]
pub fn sums(value: i32) -> i32 {
  value + 1
}

// #[wasm_bindgen_test]
// fn pass() {
//   assert_eq!(sums(10), 55);
//   assert_eq!(sums(100), 5050);
// }
