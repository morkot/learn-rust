fn main() {
  // you can use different formats and don't need to convert
  let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
  assert!(v == 1597);

  println!("Success!");

  //floating point
  let x = 0.1+0.2;
  println!("{}", x);
  // returns 0.30000000000000004

  assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
  assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);
}
