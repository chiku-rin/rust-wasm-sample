use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     pub fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn collatz(n: u64, step: u32) -> u32 {
  if n < 1 {
    return 0;
  }
  if n == 1 {
    return step;
  }
  if n % 2 == 1 {
    return collatz(3 * n + 1, step + 1);
  } else {
    return collatz(n / 2, step + 1);
  }
}

#[wasm_bindgen]
pub fn collatz_loop(n: u64) -> u32 {
  let mut step = 0;
  for i in 1..=n {
    step += collatz(i, 1);
    step %= 1000000007;
  }
  return step;
}
