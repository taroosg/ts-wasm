use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sums(value: isize) -> isize {
  (value + 1) * value / 2
}

#[wasm_bindgen]
pub fn f(n: usize) -> usize {
  // match n {
  //   0 | 1 => 1,
  //   _ => f(n - 1) + f(n - 2),
  // }
  let mut now = 1;
  let mut p1 = 1;
  for _ in 2..n {
    let p2 = p1;
    p1 = now;
    now = p1 + p2;
  }
  now
}

#[wasm_bindgen]
pub fn decimal_to_binary(dec_number: u32) -> String {
  fn get_quotient_divided_by_2(number: u32) -> u32 {
    number / 2
  }
  fn get_surplus_divided_by_2(number: u32) -> u32 {
    number % 2
  }
  fn unshift_number_to_array(number: u32, array: Vec<u32>) -> Vec<u32> {
    [number].iter().chain(array.iter()).map(|&x| x).collect()
  }
  fn create_binary_array(number: u32, array: Vec<u32>) -> Vec<u32> {
    match get_quotient_divided_by_2(number) {
      0 => unshift_number_to_array(get_surplus_divided_by_2(number), array),
      _ => create_binary_array(
        get_quotient_divided_by_2(number),
        unshift_number_to_array(get_surplus_divided_by_2(number), array),
      ),
    }
  }
  fn join_array(array: Vec<u32>) -> String {
    array.iter().map(|&x| x.to_string()).collect()
  }
  join_array(create_binary_array(dec_number, vec![]))
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(sums(10), 55);
    assert_eq!(f(1), 1);
    assert_eq!(f(5), 5);
    assert_eq!(f(10), 55);
    assert_eq!(f(90), 2880067194370816120);
    assert_eq!(decimal_to_binary(60), "111100");
    assert_eq!(decimal_to_binary(26), "11010");
    assert_eq!(decimal_to_binary(35), "100011");
    assert_eq!(decimal_to_binary(100), "1100100");
    assert_eq!(decimal_to_binary(505), "111111001");
  }
}
